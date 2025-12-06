//! Hash set (single-writer, many-reader).
//!
//! A concurrent hash set that supports single-writer updates with
//! many concurrent readers.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::hash::{Hash, Hasher};
use core::ptr;
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

use crate::malloc::Allocator;

/// Default initial capacity.
const DEFAULT_CAPACITY: usize = 16;

/// Load factor threshold for resizing.
const LOAD_FACTOR: f64 = 0.75;

/// A hash set entry.
struct Entry<T> {
    hash: u64,
    value: T,
}

/// A concurrent hash set.
pub struct HashSet<T, A: Allocator = crate::malloc::GlobalAllocator> {
    buckets: AtomicPtr<Vec<AtomicPtr<Entry<T>>>>,
    size: AtomicUsize,
    allocator: A,
}

impl<T: Hash + Eq> HashSet<T, crate::malloc::GlobalAllocator> {
    /// Create a new empty hash set.
    #[inline]
    pub fn new() -> Self {
        Self::with_allocator(crate::malloc::GlobalAllocator)
    }

    /// Create with a specified capacity.
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_allocator(capacity, crate::malloc::GlobalAllocator)
    }
}

impl<T: Hash + Eq, A: Allocator> HashSet<T, A> {
    /// Create a new empty hash set with a custom allocator.
    #[inline]
    pub fn with_allocator(allocator: A) -> Self {
        Self::with_capacity_and_allocator(DEFAULT_CAPACITY, allocator)
    }

    /// Create with specified capacity and allocator.
    pub fn with_capacity_and_allocator(capacity: usize, allocator: A) -> Self {
        let cap = capacity.next_power_of_two().max(DEFAULT_CAPACITY);
        let buckets: Vec<AtomicPtr<Entry<T>>> = (0..cap).map(|_| AtomicPtr::new(ptr::null_mut())).collect();

        Self {
            buckets: AtomicPtr::new(Box::into_raw(Box::new(buckets))),
            size: AtomicUsize::new(0),
            allocator,
        }
    }

    /// Compute hash for a value.
    fn hash_value(value: &T) -> u64 {
        let mut hasher = ahash_fallback::AHasher::default();
        value.hash(&mut hasher);
        hasher.finish()
    }

    /// Get the bucket index for a hash.
    fn bucket_index(&self, hash: u64, cap: usize) -> usize {
        (hash as usize) & (cap - 1)
    }

    /// Check if the set contains a value.
    pub fn contains(&self, value: &T) -> bool {
        let hash = Self::hash_value(value);
        let buckets = unsafe { &*self.buckets.load(Ordering::Acquire) };
        let index = self.bucket_index(hash, buckets.len());

        let entry_ptr = buckets[index].load(Ordering::Acquire);
        if entry_ptr.is_null() {
            return false;
        }

        let entry = unsafe { &*entry_ptr };
        entry.hash == hash && entry.value == *value
    }

    /// Get the number of elements.
    #[inline]
    pub fn len(&self) -> usize {
        self.size.load(Ordering::Acquire)
    }

    /// Check if the set is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Insert a value.
    ///
    /// # Safety
    ///
    /// Must only be called from a single writer thread.
    pub unsafe fn insert(&self, value: T) -> bool {
        let hash = Self::hash_value(&value);
        let buckets = &*self.buckets.load(Ordering::Acquire);
        let index = self.bucket_index(hash, buckets.len());

        // Check if already exists
        let existing = buckets[index].load(Ordering::Acquire);
        if !existing.is_null() {
            let entry = &*existing;
            if entry.hash == hash && entry.value == value {
                return false;
            }
        }

        // Insert new entry
        let entry = Box::into_raw(Box::new(Entry { hash, value }));
        buckets[index].store(entry, Ordering::Release);
        self.size.fetch_add(1, Ordering::Release);

        true
    }

    /// Remove a value.
    ///
    /// # Safety
    ///
    /// Must only be called from a single writer thread.
    pub unsafe fn remove(&self, value: &T) -> bool {
        let hash = Self::hash_value(value);
        let buckets = &*self.buckets.load(Ordering::Acquire);
        let index = self.bucket_index(hash, buckets.len());

        let entry_ptr = buckets[index].load(Ordering::Acquire);
        if entry_ptr.is_null() {
            return false;
        }

        let entry = &*entry_ptr;
        if entry.hash == hash && entry.value == *value {
            buckets[index].store(ptr::null_mut(), Ordering::Release);
            self.size.fetch_sub(1, Ordering::Release);
            // Defer freeing entry_ptr
            true
        } else {
            false
        }
    }
}

impl<T: Hash + Eq> Default for HashSet<T, crate::malloc::GlobalAllocator> {
    fn default() -> Self {
        Self::new()
    }
}

// Simple fallback hasher
mod ahash_fallback {
    use core::hash::Hasher;

    #[derive(Default)]
    pub struct AHasher {
        state: u64,
    }

    impl Hasher for AHasher {
        fn finish(&self) -> u64 {
            self.state
        }

        fn write(&mut self, bytes: &[u8]) {
            for &byte in bytes {
                self.state = self.state.wrapping_mul(31).wrapping_add(byte as u64);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let set: HashSet<i32> = HashSet::new();
        assert!(set.is_empty());
    }

    #[test]
    fn test_insert_contains() {
        let set: HashSet<i32> = HashSet::new();
        unsafe {
            assert!(set.insert(42));
            assert!(set.contains(&42));
            assert!(!set.contains(&99));
        }
    }

    #[test]
    fn test_remove() {
        let set: HashSet<i32> = HashSet::new();
        unsafe {
            set.insert(42);
            assert!(set.remove(&42));
            assert!(!set.contains(&42));
        }
    }
}
