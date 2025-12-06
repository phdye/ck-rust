//! Hash table (key-value pairs).
//!
//! A concurrent hash table that supports single-writer updates with
//! many concurrent readers.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::hash::{Hash, Hasher};
use core::ptr;
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

use crate::malloc::Allocator;

/// Default initial capacity.
const DEFAULT_CAPACITY: usize = 16;

/// A hash table entry.
struct Entry<K, V> {
    hash: u64,
    key: K,
    value: V,
}

/// A concurrent hash table.
pub struct HashTable<K, V, A: Allocator = crate::malloc::GlobalAllocator> {
    buckets: AtomicPtr<Vec<AtomicPtr<Entry<K, V>>>>,
    size: AtomicUsize,
    allocator: A,
}

impl<K: Hash + Eq, V> HashTable<K, V, crate::malloc::GlobalAllocator> {
    /// Create a new empty hash table.
    #[inline]
    pub fn new() -> Self {
        Self::with_allocator(crate::malloc::GlobalAllocator)
    }
}

impl<K: Hash + Eq, V, A: Allocator> HashTable<K, V, A> {
    /// Create a new empty hash table with a custom allocator.
    pub fn with_allocator(allocator: A) -> Self {
        let buckets: Vec<AtomicPtr<Entry<K, V>>> =
            (0..DEFAULT_CAPACITY).map(|_| AtomicPtr::new(ptr::null_mut())).collect();

        Self {
            buckets: AtomicPtr::new(Box::into_raw(Box::new(buckets))),
            size: AtomicUsize::new(0),
            allocator,
        }
    }

    /// Compute hash for a key.
    fn hash_key(key: &K) -> u64 {
        let mut hasher = SimpleHasher::default();
        key.hash(&mut hasher);
        hasher.finish()
    }

    /// Get the bucket index for a hash.
    fn bucket_index(&self, hash: u64, cap: usize) -> usize {
        (hash as usize) & (cap - 1)
    }

    /// Get a value by key.
    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = Self::hash_key(key);
        let buckets = unsafe { &*self.buckets.load(Ordering::Acquire) };
        let index = self.bucket_index(hash, buckets.len());

        let entry_ptr = buckets[index].load(Ordering::Acquire);
        if entry_ptr.is_null() {
            return None;
        }

        let entry = unsafe { &*entry_ptr };
        if entry.hash == hash && entry.key == *key {
            Some(&entry.value)
        } else {
            None
        }
    }

    /// Check if the table contains a key.
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Get the number of entries.
    #[inline]
    pub fn len(&self) -> usize {
        self.size.load(Ordering::Acquire)
    }

    /// Check if the table is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Insert a key-value pair.
    ///
    /// # Safety
    ///
    /// Must only be called from a single writer thread.
    pub unsafe fn insert(&self, key: K, value: V) -> Option<V> {
        let hash = Self::hash_key(&key);
        let buckets = &*self.buckets.load(Ordering::Acquire);
        let index = self.bucket_index(hash, buckets.len());

        let existing = buckets[index].load(Ordering::Acquire);
        let old_value = if !existing.is_null() {
            let old_entry = Box::from_raw(existing);
            if old_entry.hash == hash && old_entry.key == key {
                Some(old_entry.value)
            } else {
                self.size.fetch_add(1, Ordering::Release);
                None
            }
        } else {
            self.size.fetch_add(1, Ordering::Release);
            None
        };

        let entry = Box::into_raw(Box::new(Entry { hash, key, value }));
        buckets[index].store(entry, Ordering::Release);

        old_value
    }

    /// Remove a key.
    ///
    /// # Safety
    ///
    /// Must only be called from a single writer thread.
    pub unsafe fn remove(&self, key: &K) -> Option<V> {
        let hash = Self::hash_key(key);
        let buckets = &*self.buckets.load(Ordering::Acquire);
        let index = self.bucket_index(hash, buckets.len());

        let entry_ptr = buckets[index].load(Ordering::Acquire);
        if entry_ptr.is_null() {
            return None;
        }

        let entry = Box::from_raw(entry_ptr);
        if entry.hash == hash && entry.key == *key {
            buckets[index].store(ptr::null_mut(), Ordering::Release);
            self.size.fetch_sub(1, Ordering::Release);
            Some(entry.value)
        } else {
            // Put it back
            buckets[index].store(Box::into_raw(entry), Ordering::Release);
            None
        }
    }
}

impl<K: Hash + Eq, V> Default for HashTable<K, V, crate::malloc::GlobalAllocator> {
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
        let table: HashTable<i32, i32> = HashTable::new();
        assert!(table.is_empty());
    }

    #[test]
    fn test_insert_get() {
        let table: HashTable<i32, &str> = HashTable::new();
        unsafe {
            table.insert(1, "one");
            assert_eq!(table.get(&1), Some(&"one"));
            assert_eq!(table.get(&2), None);
        }
    }

    #[test]
    fn test_remove() {
        let table: HashTable<i32, i32> = HashTable::new();
        unsafe {
            table.insert(42, 100);
            assert_eq!(table.remove(&42), Some(100));
            assert!(!table.contains_key(&42));
        }
    }
}
