//! BSD-style queue macros.
//!
//! This module provides intrusive linked list implementations similar to
//! BSD's `<sys/queue.h>`. These are building blocks for more complex
//! data structures.
//!
//! # Intrusive Lists
//!
//! Unlike standard library collections, intrusive lists embed the link
//! pointers directly in the data structure. This allows for:
//!
//! - Zero-allocation insertion and removal
//! - O(1) removal when you have a pointer to the element
//! - Better cache locality for iteration

use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};
use core::marker::PhantomData;

/// Entry for a singly-linked list (SLIST).
#[repr(C)]
pub struct SlistEntry<T> {
    next: AtomicPtr<T>,
}

impl<T> SlistEntry<T> {
    /// Create a new unlinked entry.
    #[inline]
    pub const fn new() -> Self {
        Self {
            next: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Get the next element.
    #[inline]
    pub fn next(&self) -> *mut T {
        self.next.load(Ordering::Acquire)
    }
}

impl<T> Default for SlistEntry<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Head of a singly-linked list.
#[repr(C)]
pub struct SlistHead<T> {
    first: AtomicPtr<T>,
    _marker: PhantomData<T>,
}

impl<T> Default for SlistHead<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> SlistHead<T> {
    /// Create a new empty list head.
    #[inline]
    pub const fn new() -> Self {
        Self {
            first: AtomicPtr::new(ptr::null_mut()),
            _marker: PhantomData,
        }
    }

    /// Check if the list is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.first.load(Ordering::Acquire).is_null()
    }

    /// Get the first element.
    #[inline]
    pub fn first(&self) -> *mut T {
        self.first.load(Ordering::Acquire)
    }
}

/// Entry for a doubly-linked list (LIST).
#[repr(C)]
pub struct ListEntry<T> {
    next: AtomicPtr<T>,
    prev: AtomicPtr<AtomicPtr<T>>, // Points to previous node's next pointer
}

impl<T> ListEntry<T> {
    /// Create a new unlinked entry.
    #[inline]
    pub const fn new() -> Self {
        Self {
            next: AtomicPtr::new(ptr::null_mut()),
            prev: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Get the next element.
    #[inline]
    pub fn next(&self) -> *mut T {
        self.next.load(Ordering::Acquire)
    }
}

impl<T> Default for ListEntry<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Head of a doubly-linked list.
#[repr(C)]
pub struct ListHead<T> {
    first: AtomicPtr<T>,
    _marker: PhantomData<T>,
}

impl<T> Default for ListHead<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ListHead<T> {
    /// Create a new empty list head.
    #[inline]
    pub const fn new() -> Self {
        Self {
            first: AtomicPtr::new(ptr::null_mut()),
            _marker: PhantomData,
        }
    }

    /// Check if the list is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.first.load(Ordering::Acquire).is_null()
    }

    /// Get the first element.
    #[inline]
    pub fn first(&self) -> *mut T {
        self.first.load(Ordering::Acquire)
    }
}

/// Entry for a tail queue (STAILQ).
#[repr(C)]
pub struct StailqEntry<T> {
    next: AtomicPtr<T>,
}

impl<T> StailqEntry<T> {
    /// Create a new unlinked entry.
    #[inline]
    pub const fn new() -> Self {
        Self {
            next: AtomicPtr::new(ptr::null_mut()),
        }
    }
}

impl<T> Default for StailqEntry<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Head of a singly-linked tail queue.
#[repr(C)]
pub struct StailqHead<T> {
    first: AtomicPtr<T>,
    last: AtomicPtr<AtomicPtr<T>>,
    _marker: PhantomData<T>,
}

impl<T> Default for StailqHead<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> StailqHead<T> {
    /// Create a new empty queue head.
    #[inline]
    pub const fn new() -> Self {
        Self {
            first: AtomicPtr::new(ptr::null_mut()),
            last: AtomicPtr::new(ptr::null_mut()),
            _marker: PhantomData,
        }
    }

    /// Check if the queue is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.first.load(Ordering::Acquire).is_null()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slist_head_new() {
        let head: SlistHead<i32> = SlistHead::new();
        assert!(head.is_empty());
    }

    #[test]
    fn test_list_head_new() {
        let head: ListHead<i32> = ListHead::new();
        assert!(head.is_empty());
    }

    #[test]
    fn test_stailq_head_new() {
        let head: StailqHead<i32> = StailqHead::new();
        assert!(head.is_empty());
    }

    #[test]
    fn test_slist_entry_new() {
        let entry: SlistEntry<i32> = SlistEntry::new();
        assert!(entry.next().is_null());
    }

    #[test]
    fn test_list_entry_new() {
        let entry: ListEntry<i32> = ListEntry::new();
        assert!(entry.next().is_null());
    }
}
