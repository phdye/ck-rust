//! Concurrently-readable dynamic array.
//!
//! A dynamic array that supports concurrent read access with single-writer
//! updates. Uses copy-on-write for safe concurrent access.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::malloc::Allocator;

/// A concurrently-readable dynamic array.
pub struct Array<T, A: Allocator = crate::malloc::GlobalAllocator> {
    current: AtomicPtr<Vec<T>>,
    allocator: A,
}

impl<T: Clone> Array<T, crate::malloc::GlobalAllocator> {
    /// Create a new empty array.
    #[inline]
    pub fn new() -> Self {
        Self::with_allocator(crate::malloc::GlobalAllocator)
    }
}

impl<T: Clone, A: Allocator> Array<T, A> {
    /// Create a new empty array with a custom allocator.
    #[inline]
    pub fn with_allocator(allocator: A) -> Self {
        Self {
            current: AtomicPtr::new(Box::into_raw(Box::new(Vec::new()))),
            allocator,
        }
    }

    /// Get a snapshot of the current array.
    ///
    /// The returned reference is valid until the next mutation.
    #[inline]
    pub fn snapshot(&self) -> ArraySnapshot<'_, T> {
        let ptr = self.current.load(Ordering::Acquire);
        ArraySnapshot {
            data: unsafe { &*ptr },
            _marker: core::marker::PhantomData,
        }
    }

    /// Get the length of the array.
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { (*self.current.load(Ordering::Acquire)).len() }
    }

    /// Check if the array is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Push an element to the array.
    ///
    /// # Safety
    ///
    /// Must only be called from a single writer thread.
    pub unsafe fn push(&self, value: T) {
        let old_ptr = self.current.load(Ordering::Acquire);
        let mut new_vec = (*old_ptr).clone();
        new_vec.push(value);

        let new_ptr = Box::into_raw(Box::new(new_vec));
        self.current.store(new_ptr, Ordering::Release);

        // Old vec will be freed when readers are done
        // In a real implementation, use epoch-based reclamation
    }

    /// Remove the last element.
    ///
    /// # Safety
    ///
    /// Must only be called from a single writer thread.
    pub unsafe fn pop(&self) -> Option<T> {
        let old_ptr = self.current.load(Ordering::Acquire);
        let mut new_vec = (*old_ptr).clone();
        let result = new_vec.pop();

        if result.is_some() {
            let new_ptr = Box::into_raw(Box::new(new_vec));
            self.current.store(new_ptr, Ordering::Release);
        }

        result
    }
}

impl<T: Clone> Default for Array<T, crate::malloc::GlobalAllocator> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A: Allocator> Drop for Array<T, A> {
    fn drop(&mut self) {
        let ptr = self.current.load(Ordering::Relaxed);
        if !ptr.is_null() {
            unsafe {
                drop(Box::from_raw(ptr));
            }
        }
    }
}

/// A read-only snapshot of an Array.
pub struct ArraySnapshot<'a, T> {
    data: &'a Vec<T>,
    _marker: core::marker::PhantomData<&'a T>,
}

impl<'a, T> ArraySnapshot<'a, T> {
    /// Get a slice of the data.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    /// Get the length.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Get an element by index.
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let arr: Array<i32> = Array::new();
        assert!(arr.is_empty());
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_push() {
        let arr: Array<i32> = Array::new();
        unsafe {
            arr.push(1);
            arr.push(2);
            arr.push(3);
        }
        assert_eq!(arr.len(), 3);

        let snap = arr.snapshot();
        assert_eq!(snap.get(0), Some(&1));
        assert_eq!(snap.get(1), Some(&2));
        assert_eq!(snap.get(2), Some(&3));
    }

    #[test]
    fn test_pop() {
        let arr: Array<i32> = Array::new();
        unsafe {
            arr.push(1);
            arr.push(2);

            assert_eq!(arr.pop(), Some(2));
            assert_eq!(arr.pop(), Some(1));
            assert_eq!(arr.pop(), None);
        }
    }
}
