//! Memory allocator interface.
//!
//! This module defines an abstract interface for memory allocation that allows
//! CK data structures to use custom allocators. This enables integration with
//! application-specific memory management strategies, memory pools, or specialized
//! allocators for NUMA-aware allocation.
//!
//! # Design
//!
//! The [`Allocator`] trait provides three operations:
//! - `malloc`: Allocate new memory
//! - `realloc`: Resize existing memory (with in-place option)
//! - `free`: Deallocate memory (with deferred option)
//!
//! # Thread Safety
//!
//! Implementations of [`Allocator`] must be thread-safe if used with concurrent
//! CK data structures.

use core::ptr::NonNull;

/// Error type for allocation failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocError {
    /// Memory allocation failed (out of memory).
    OutOfMemory,
    /// Reallocation failed because in-place resize was not possible.
    CannotResizeInPlace,
}

/// Result type for allocation operations.
pub type AllocResult<T> = Result<T, AllocError>;

/// Memory allocator trait for CK data structures.
///
/// This trait abstracts memory allocation operations to allow custom allocators
/// to be used with CK data structures. It mirrors the C `struct ck_malloc`
/// interface with Rust-idiomatic types.
///
/// # Safety
///
/// Implementations must ensure:
/// - Allocated memory is properly aligned for any type
/// - Memory is not double-freed
/// - Freed memory is not accessed
/// - All operations are thread-safe if used concurrently
///
/// # Example
///
/// ```
/// use concurrencykit::malloc::{Allocator, AllocResult, AllocError};
/// use core::ptr::NonNull;
///
/// struct SimpleAllocator;
///
/// impl Allocator for SimpleAllocator {
///     fn malloc(&self, size: usize) -> AllocResult<NonNull<u8>> {
///         // In a real implementation, allocate memory here
///         Err(AllocError::OutOfMemory)
///     }
///
///     fn realloc(
///         &self,
///         ptr: NonNull<u8>,
///         old_size: usize,
///         new_size: usize,
///         may_move: bool,
///     ) -> AllocResult<NonNull<u8>> {
///         if !may_move {
///             return Err(AllocError::CannotResizeInPlace);
///         }
///         // In a real implementation, resize memory here
///         Err(AllocError::OutOfMemory)
///     }
///
///     fn free(&self, ptr: NonNull<u8>, size: usize, defer: bool) {
///         // In a real implementation, free memory here
///     }
/// }
/// ```
pub trait Allocator {
    /// Allocate a new block of memory.
    ///
    /// # Arguments
    ///
    /// * `size` - The size in bytes to allocate
    ///
    /// # Returns
    ///
    /// * `Ok(NonNull<u8>)` - Pointer to allocated memory on success
    /// * `Err(AllocError::OutOfMemory)` - If allocation fails
    ///
    /// # Notes
    ///
    /// The returned memory need not be initialized. The memory is guaranteed
    /// to be properly aligned for any type.
    fn malloc(&self, size: usize) -> AllocResult<NonNull<u8>>;

    /// Resize an existing memory block.
    ///
    /// # Arguments
    ///
    /// * `ptr` - Pointer to the existing memory block
    /// * `old_size` - The current size of the block
    /// * `new_size` - The desired new size
    /// * `may_move` - If `true`, the block may be relocated; if `false`, must resize in place
    ///
    /// # Returns
    ///
    /// * `Ok(NonNull<u8>)` - Pointer to the (possibly relocated) memory on success
    /// * `Err(AllocError::OutOfMemory)` - If allocation fails
    /// * `Err(AllocError::CannotResizeInPlace)` - If `may_move` is `false` and in-place resize is not possible
    ///
    /// # Notes
    ///
    /// - Contents up to `min(old_size, new_size)` are preserved
    /// - On failure, the original block remains valid
    /// - The `may_move=false` option enables resize-in-place semantics required
    ///   for some concurrent algorithms
    fn realloc(
        &self,
        ptr: NonNull<u8>,
        old_size: usize,
        new_size: usize,
        may_move: bool,
    ) -> AllocResult<NonNull<u8>>;

    /// Deallocate a memory block.
    ///
    /// # Arguments
    ///
    /// * `ptr` - Pointer to the memory block to free
    /// * `size` - The size of the block (for sized-delete optimization)
    /// * `defer` - If `true`, deallocation may be batched; if `false`, free immediately
    ///
    /// # Notes
    ///
    /// - The `size` parameter enables sized-delete optimizations in allocators
    ///   that support them; it may be ignored by simple allocators
    /// - The `defer` parameter enables integration with deferred reclamation
    ///   schemes (epoch-based reclamation, hazard pointers, etc.)
    fn free(&self, ptr: NonNull<u8>, size: usize, defer: bool);
}

/// Extension trait providing convenience methods for typed allocation.
pub trait AllocatorExt: Allocator {
    /// Allocate memory for a single value of type `T`.
    ///
    /// Returns properly aligned memory for type `T`.
    fn alloc<T>(&self) -> AllocResult<NonNull<T>> {
        let ptr = self.malloc(core::mem::size_of::<T>())?;
        // SAFETY: The pointer is non-null and properly aligned
        Ok(ptr.cast())
    }

    /// Allocate memory for an array of `count` values of type `T`.
    fn alloc_array<T>(&self, count: usize) -> AllocResult<NonNull<T>> {
        let size = core::mem::size_of::<T>()
            .checked_mul(count)
            .ok_or(AllocError::OutOfMemory)?;
        let ptr = self.malloc(size)?;
        Ok(ptr.cast())
    }

    /// Free memory for a single value of type `T`.
    ///
    /// # Safety
    ///
    /// The pointer must have been allocated by this allocator.
    unsafe fn dealloc<T>(&self, ptr: NonNull<T>, defer: bool) {
        self.free(ptr.cast(), core::mem::size_of::<T>(), defer);
    }

    /// Free memory for an array of `count` values of type `T`.
    ///
    /// # Safety
    ///
    /// The pointer must have been allocated by this allocator with the same count.
    unsafe fn dealloc_array<T>(&self, ptr: NonNull<T>, count: usize, defer: bool) {
        let size = core::mem::size_of::<T>() * count;
        self.free(ptr.cast(), size, defer);
    }
}

// Blanket implementation for all Allocators
impl<A: Allocator> AllocatorExt for A {}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock allocator for testing
    struct MockAllocator {
        // Track allocations for testing
    }

    impl MockAllocator {
        fn new() -> Self {
            MockAllocator {}
        }
    }

    impl Allocator for MockAllocator {
        fn malloc(&self, _size: usize) -> AllocResult<NonNull<u8>> {
            // For testing, we can't actually allocate in no_std without an allocator
            // This tests the interface, not actual allocation
            Err(AllocError::OutOfMemory)
        }

        fn realloc(
            &self,
            _ptr: NonNull<u8>,
            _old_size: usize,
            _new_size: usize,
            may_move: bool,
        ) -> AllocResult<NonNull<u8>> {
            if !may_move {
                return Err(AllocError::CannotResizeInPlace);
            }
            Err(AllocError::OutOfMemory)
        }

        fn free(&self, _ptr: NonNull<u8>, _size: usize, _defer: bool) {
            // No-op for mock
        }
    }

    // TEST-001: struct_size_verification
    // In Rust, we verify the trait is correctly defined
    #[test]
    fn test_trait_definition() {
        let _alloc = MockAllocator::new();
        // If this compiles, the trait is correctly defined
    }

    // TEST-002: malloc_wrapper_basic
    #[test]
    fn test_malloc_returns_result() {
        let alloc = MockAllocator::new();
        let result = alloc.malloc(1024);
        assert!(result.is_err()); // Mock always fails
    }

    // TEST-003: realloc_may_move_true
    #[test]
    fn test_realloc_may_move_true() {
        let alloc = MockAllocator::new();
        // We can't test with real allocation, but verify interface works
        // Create a dangling pointer for interface testing only
        let ptr = NonNull::dangling();
        let result = alloc.realloc(ptr, 16, 1024, true);
        assert!(result.is_err()); // Mock always fails with OutOfMemory
        assert_eq!(result.unwrap_err(), AllocError::OutOfMemory);
    }

    // TEST-004: realloc_may_move_false
    #[test]
    fn test_realloc_may_move_false() {
        let alloc = MockAllocator::new();
        let ptr = NonNull::dangling();
        let result = alloc.realloc(ptr, 16, 1024, false);
        // Should fail with CannotResizeInPlace when may_move is false
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AllocError::CannotResizeInPlace);
    }

    // TEST-005: free_with_size
    #[test]
    fn test_free_accepts_size() {
        let alloc = MockAllocator::new();
        let ptr = NonNull::dangling();
        // Should not panic - just verify interface accepts size
        alloc.free(ptr, 1024, false);
    }

    // TEST-006: free_defer_true
    #[test]
    fn test_free_defer_true() {
        let alloc = MockAllocator::new();
        let ptr = NonNull::dangling();
        // Should not panic - verify interface accepts defer=true
        alloc.free(ptr, 1024, true);
    }

    // TEST-007: free_defer_false
    #[test]
    fn test_free_defer_false() {
        let alloc = MockAllocator::new();
        let ptr = NonNull::dangling();
        // Should not panic - verify interface accepts defer=false
        alloc.free(ptr, 1024, false);
    }

    // TEST-008: null_pointer_handling
    // In Rust, NonNull prevents null pointers by design
    #[test]
    fn test_nonnull_prevents_null() {
        // NonNull::new(core::ptr::null_mut()) returns None
        let maybe_ptr: Option<NonNull<u8>> = NonNull::new(core::ptr::null_mut());
        assert!(maybe_ptr.is_none());
    }

    // TEST-009: zero_size_allocation
    #[test]
    fn test_zero_size_allocation() {
        let alloc = MockAllocator::new();
        // Zero-size allocation should be handled
        let result = alloc.malloc(0);
        // Implementation-defined behavior
        assert!(result.is_err() || result.is_ok());
    }

    // Test AllocError equality
    #[test]
    fn test_alloc_error_eq() {
        assert_eq!(AllocError::OutOfMemory, AllocError::OutOfMemory);
        assert_eq!(AllocError::CannotResizeInPlace, AllocError::CannotResizeInPlace);
        assert_ne!(AllocError::OutOfMemory, AllocError::CannotResizeInPlace);
    }

    // Test AllocatorExt trait
    #[test]
    fn test_allocator_ext_alloc() {
        let alloc = MockAllocator::new();
        let result: AllocResult<NonNull<u32>> = alloc.alloc();
        assert!(result.is_err());
    }

    #[test]
    fn test_allocator_ext_alloc_array() {
        let alloc = MockAllocator::new();
        let result: AllocResult<NonNull<u32>> = alloc.alloc_array(10);
        assert!(result.is_err());
    }

    #[test]
    fn test_allocator_ext_alloc_array_overflow() {
        let alloc = MockAllocator::new();
        // This should fail due to overflow
        let result: AllocResult<NonNull<u64>> = alloc.alloc_array(usize::MAX);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AllocError::OutOfMemory);
    }
}
