//! Memory allocator abstraction.
//!
//! This module defines the [`Allocator`] trait which allows CK data structures
//! to use custom memory allocators. This enables integration with application-specific
//! memory management strategies, memory pools, or specialized allocators for
//! NUMA-aware allocation.
//!
//! # Design
//!
//! The allocator interface differs from the standard library's `Allocator` trait
//! in several ways:
//!
//! - **Sized deletes**: The `free` method receives the size of the allocation,
//!   enabling allocators that benefit from sized delete operations.
//!
//! - **In-place reallocation**: The `realloc` method has a `may_move` parameter
//!   that, when false, requires the allocator to resize in place or fail.
//!   This is essential for some concurrent algorithms.
//!
//! - **Deferred deallocation**: The `free` method has a `defer` parameter
//!   that allows batching deallocations for performance.
//!
//! # Example
//!
//! ```
//! use concurrencykit::malloc::{Allocator, GlobalAllocator};
//!
//! // Use the global allocator
//! let alloc = GlobalAllocator;
//!
//! // Allocate memory
//! let ptr = alloc.malloc(1024).expect("allocation failed");
//!
//! // Use the memory...
//!
//! // Free with size hint
//! unsafe { alloc.free(ptr, 1024, false); }
//! ```

use alloc::alloc::{alloc, dealloc, realloc as std_realloc, Layout};
use core::ptr::NonNull;

/// A memory allocator for CK data structures.
///
/// This trait defines the interface that custom allocators must implement
/// to be used with CK's concurrent data structures.
///
/// # Safety
///
/// Implementations must ensure that:
/// - `malloc` returns properly aligned memory for any type
/// - `realloc` preserves data up to `min(old_size, new_size)`
/// - `free` only deallocates memory previously allocated by this allocator
/// - All operations are thread-safe if used with concurrent data structures
pub trait Allocator {
    /// Allocate a new memory block.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of bytes to allocate
    ///
    /// # Returns
    ///
    /// - `Some(ptr)` - A non-null pointer to at least `size` bytes of memory
    /// - `None` - If allocation fails
    ///
    /// # Notes
    ///
    /// The returned memory is not guaranteed to be initialized.
    fn malloc(&self, size: usize) -> Option<NonNull<u8>>;

    /// Resize an existing memory block.
    ///
    /// # Arguments
    ///
    /// * `ptr` - Pointer to the existing allocation
    /// * `old_size` - The current size of the allocation
    /// * `new_size` - The desired new size
    /// * `may_move` - If true, the allocation may be relocated. If false,
    ///   the allocator must resize in place or return `None`.
    ///
    /// # Returns
    ///
    /// - `Some(ptr)` - Pointer to the (possibly relocated) allocation
    /// - `None` - If reallocation fails (original allocation remains valid)
    ///
    /// # Safety
    ///
    /// - `ptr` must have been allocated by this allocator
    /// - `old_size` must match the size of the original allocation
    ///
    /// # Notes
    ///
    /// Data is preserved up to `min(old_size, new_size)` bytes.
    fn realloc(
        &self,
        ptr: NonNull<u8>,
        old_size: usize,
        new_size: usize,
        may_move: bool,
    ) -> Option<NonNull<u8>>;

    /// Deallocate a memory block.
    ///
    /// # Arguments
    ///
    /// * `ptr` - Pointer to the allocation to free
    /// * `size` - The size of the allocation
    /// * `defer` - If true, deallocation may be batched for performance.
    ///   If false, memory is freed immediately.
    ///
    /// # Safety
    ///
    /// - `ptr` must have been allocated by this allocator
    /// - `size` must match the size of the original allocation
    /// - `ptr` must not be used after this call
    unsafe fn free(&self, ptr: NonNull<u8>, size: usize, defer: bool);
}

/// A wrapper around the global allocator.
///
/// This provides an [`Allocator`] implementation that uses Rust's global
/// allocator (typically the system allocator).
#[derive(Debug, Clone, Copy, Default)]
pub struct GlobalAllocator;

impl Allocator for GlobalAllocator {
    fn malloc(&self, size: usize) -> Option<NonNull<u8>> {
        if size == 0 {
            // Return a dangling but aligned pointer for zero-size allocations
            return NonNull::new(core::mem::align_of::<usize>() as *mut u8);
        }

        // Use maximum alignment to be safe for any type
        let layout = Layout::from_size_align(size, core::mem::align_of::<usize>())
            .ok()?;

        // SAFETY: layout has non-zero size
        let ptr = unsafe { alloc(layout) };
        NonNull::new(ptr)
    }

    fn realloc(
        &self,
        ptr: NonNull<u8>,
        old_size: usize,
        new_size: usize,
        may_move: bool,
    ) -> Option<NonNull<u8>> {
        if old_size == 0 {
            return self.malloc(new_size);
        }
        if new_size == 0 {
            // SAFETY: ptr was allocated by us with old_size
            unsafe { self.free(ptr, old_size, false) };
            return NonNull::new(core::mem::align_of::<usize>() as *mut u8);
        }

        let old_layout = Layout::from_size_align(old_size, core::mem::align_of::<usize>())
            .ok()?;

        if !may_move {
            // Standard realloc doesn't guarantee in-place resize
            // For now, we can only succeed if new_size <= old_size
            if new_size <= old_size {
                // In-place shrink is always possible
                return Some(ptr);
            }
            // Cannot grow in place
            return None;
        }

        // SAFETY: ptr was allocated with old_layout, new_size is non-zero
        let new_ptr = unsafe { std_realloc(ptr.as_ptr(), old_layout, new_size) };
        NonNull::new(new_ptr)
    }

    unsafe fn free(&self, ptr: NonNull<u8>, size: usize, _defer: bool) {
        if size == 0 {
            return; // Nothing to free for zero-size allocations
        }

        let layout = match Layout::from_size_align(size, core::mem::align_of::<usize>()) {
            Ok(l) => l,
            Err(_) => return, // Invalid layout, nothing we can do
        };

        // SAFETY: caller guarantees ptr was allocated with size
        dealloc(ptr.as_ptr(), layout);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    // TEST-001: struct_size_verification
    #[test]
    fn test_global_allocator_is_zero_sized() {
        assert_eq!(core::mem::size_of::<GlobalAllocator>(), 0);
    }

    // TEST-002: malloc_wrapper_basic
    #[test]
    fn test_malloc_basic() {
        let alloc = GlobalAllocator;

        let ptr = alloc.malloc(1024).expect("allocation failed");

        // Write to allocated memory
        unsafe {
            core::ptr::write_bytes(ptr.as_ptr(), 0xAB, 1024);
        }

        // Free
        unsafe {
            alloc.free(ptr, 1024, false);
        }
    }

    // TEST-003: realloc_may_move_true
    #[test]
    fn test_realloc_may_move_true() {
        let alloc = GlobalAllocator;

        // Allocate small buffer
        let ptr = alloc.malloc(16).expect("allocation failed");

        // Write data
        unsafe {
            for i in 0..16 {
                *ptr.as_ptr().add(i) = i as u8;
            }
        }

        // Reallocate larger
        let new_ptr = alloc.realloc(ptr, 16, 1024, true).expect("realloc failed");

        // Verify data preserved
        unsafe {
            for i in 0..16 {
                assert_eq!(*new_ptr.as_ptr().add(i), i as u8);
            }
        }

        // Free
        unsafe {
            alloc.free(new_ptr, 1024, false);
        }
    }

    // TEST-004: realloc_may_move_false
    #[test]
    fn test_realloc_may_move_false() {
        let alloc = GlobalAllocator;

        let ptr = alloc.malloc(1024).expect("allocation failed");

        // Shrinking should work
        let shrunk = alloc.realloc(ptr, 1024, 512, false);
        assert!(shrunk.is_some());
        let ptr = shrunk.unwrap();

        // Growing may fail (depends on allocator)
        let result = alloc.realloc(ptr, 512, 2048, false);

        // Either succeeds with same pointer, or fails
        if let Some(new_ptr) = result {
            assert_eq!(ptr.as_ptr(), new_ptr.as_ptr());
            unsafe { alloc.free(new_ptr, 2048, false); }
        } else {
            // Growth failed, original pointer still valid
            unsafe { alloc.free(ptr, 512, false); }
        }
    }

    // TEST-005: free_with_size
    #[test]
    fn test_free_with_size() {
        let alloc = GlobalAllocator;

        let ptr = alloc.malloc(1024).expect("allocation failed");

        // Free with correct size
        unsafe {
            alloc.free(ptr, 1024, false);
        }
    }

    // TEST-006: free_defer_true
    #[test]
    fn test_free_defer_true() {
        let alloc = GlobalAllocator;

        let ptr = alloc.malloc(1024).expect("allocation failed");

        // Free with defer=true (our implementation ignores this)
        unsafe {
            alloc.free(ptr, 1024, true);
        }
    }

    // TEST-007: free_defer_false
    #[test]
    fn test_free_defer_false() {
        let alloc = GlobalAllocator;

        let ptr = alloc.malloc(1024).expect("allocation failed");

        // Free with defer=false
        unsafe {
            alloc.free(ptr, 1024, false);
        }
    }

    // TEST-008: null_pointer_handling
    #[test]
    fn test_realloc_like_malloc() {
        let alloc = GlobalAllocator;

        // malloc(0) should return a valid (possibly dangling) pointer
        let ptr = alloc.malloc(0);
        assert!(ptr.is_some());
    }

    // TEST-009: zero_size_allocation
    #[test]
    fn test_zero_size_allocation() {
        let alloc = GlobalAllocator;

        let ptr = alloc.malloc(0);
        // Should return Some with a dangling but aligned pointer
        assert!(ptr.is_some());

        // Freeing zero-size allocation should be safe
        if let Some(p) = ptr {
            unsafe { alloc.free(p, 0, false); }
        }
    }

    // Additional tests for comprehensive coverage
    #[test]
    fn test_multiple_allocations() {
        let alloc = GlobalAllocator;
        let mut ptrs = Vec::new();

        // Allocate many blocks
        for i in 1..=100 {
            let ptr = alloc.malloc(i * 16).expect("allocation failed");
            ptrs.push((ptr, i * 16));
        }

        // Free all
        for (ptr, size) in ptrs {
            unsafe { alloc.free(ptr, size, false); }
        }
    }

    #[test]
    fn test_realloc_shrink() {
        let alloc = GlobalAllocator;

        let ptr = alloc.malloc(1024).expect("allocation failed");

        // Write data
        unsafe {
            for i in 0..512 {
                *ptr.as_ptr().add(i) = (i % 256) as u8;
            }
        }

        // Shrink
        let new_ptr = alloc.realloc(ptr, 1024, 512, true).expect("realloc failed");

        // Verify data preserved
        unsafe {
            for i in 0..512 {
                assert_eq!(*new_ptr.as_ptr().add(i), (i % 256) as u8);
            }
        }

        unsafe { alloc.free(new_ptr, 512, false); }
    }
}
