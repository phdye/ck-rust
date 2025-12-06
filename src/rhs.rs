//! Robin-hood hash set.
//!
//! A hash set using robin-hood hashing for better cache performance.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::hash::{Hash, Hasher};
use core::ptr;
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

use crate::malloc::Allocator;

const DEFAULT_CAPACITY: usize = 16;

struct Entry<T> {
    hash: u64,
    probe_distance: usize,
    value: T,
}

/// A robin-hood hash set.
pub struct RobinHoodSet<T, A: Allocator = crate::malloc::GlobalAllocator> {
    buckets: AtomicPtr<Vec<AtomicPtr<Entry<T>>>>,
    size: AtomicUsize,
    allocator: A,
}

impl<T: Hash + Eq> RobinHoodSet<T, crate::malloc::GlobalAllocator> {
    /// Create a new empty set.
    #[inline]
    pub fn new() -> Self {
        Self::with_allocator(crate::malloc::GlobalAllocator)
    }
}

impl<T: Hash + Eq, A: Allocator> RobinHoodSet<T, A> {
    /// Create with custom allocator.
    pub fn with_allocator(allocator: A) -> Self {
        let buckets: Vec<AtomicPtr<Entry<T>>> =
            (0..DEFAULT_CAPACITY).map(|_| AtomicPtr::new(ptr::null_mut())).collect();

        Self {
            buckets: AtomicPtr::new(Box::into_raw(Box::new(buckets))),
            size: AtomicUsize::new(0),
            allocator,
        }
    }

    fn hash_value(value: &T) -> u64 {
        let mut hasher = SimpleHasher::default();
        value.hash(&mut hasher);
        hasher.finish()
    }

    /// Check if contains value.
    pub fn contains(&self, value: &T) -> bool {
        let hash = Self::hash_value(value);
        let buckets = unsafe { &*self.buckets.load(Ordering::Acquire) };
        let cap = buckets.len();
        let mut index = (hash as usize) & (cap - 1);

        for _ in 0..cap {
            let entry_ptr = buckets[index].load(Ordering::Acquire);
            if entry_ptr.is_null() {
                return false;
            }

            let entry = unsafe { &*entry_ptr };
            if entry.hash == hash && entry.value == *value {
                return true;
            }

            index = (index + 1) & (cap - 1);
        }

        false
    }

    /// Get length.
    #[inline]
    pub fn len(&self) -> usize {
        self.size.load(Ordering::Acquire)
    }

    /// Check if empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Insert value.
    ///
    /// # Safety
    ///
    /// Must be called from single writer thread.
    pub unsafe fn insert(&self, value: T) -> bool {
        if self.contains(&value) {
            return false;
        }

        let hash = Self::hash_value(&value);
        let buckets = &*self.buckets.load(Ordering::Acquire);
        let cap = buckets.len();
        let mut index = (hash as usize) & (cap - 1);

        let entry = Box::into_raw(Box::new(Entry {
            hash,
            probe_distance: 0,
            value,
        }));

        // Find empty slot
        for _ in 0..cap {
            if buckets[index].load(Ordering::Acquire).is_null() {
                buckets[index].store(entry, Ordering::Release);
                self.size.fetch_add(1, Ordering::Release);
                return true;
            }
            index = (index + 1) & (cap - 1);
        }

        false
    }

    /// Remove value.
    ///
    /// # Safety
    ///
    /// Must be called from single writer thread.
    pub unsafe fn remove(&self, value: &T) -> bool {
        let hash = Self::hash_value(value);
        let buckets = &*self.buckets.load(Ordering::Acquire);
        let cap = buckets.len();
        let mut index = (hash as usize) & (cap - 1);

        for _ in 0..cap {
            let entry_ptr = buckets[index].load(Ordering::Acquire);
            if entry_ptr.is_null() {
                return false;
            }

            let entry = &*entry_ptr;
            if entry.hash == hash && entry.value == *value {
                buckets[index].store(ptr::null_mut(), Ordering::Release);
                self.size.fetch_sub(1, Ordering::Release);
                return true;
            }

            index = (index + 1) & (cap - 1);
        }

        false
    }
}

impl<T: Hash + Eq> Default for RobinHoodSet<T, crate::malloc::GlobalAllocator> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
struct SimpleHasher {
    state: u64,
}

impl Hasher for SimpleHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = self.state.wrapping_mul(31).wrapping_add(byte as u64);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let set: RobinHoodSet<i32> = RobinHoodSet::new();
        assert!(set.is_empty());
    }

    #[test]
    fn test_insert_contains() {
        let set: RobinHoodSet<i32> = RobinHoodSet::new();
        unsafe {
            assert!(set.insert(42));
            assert!(set.contains(&42));
        }
    }

    #[test]
    fn test_remove() {
        let set: RobinHoodSet<i32> = RobinHoodSet::new();
        unsafe {
            set.insert(42);
            assert!(set.remove(&42));
            assert!(!set.contains(&42));
        }
    }
}
