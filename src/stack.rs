//! Lock-free stack.
//!
//! This module provides a lock-free stack implementation based on
//! Treiber's algorithm. The stack supports concurrent push and pop
//! operations without locks.
//!
//! # Safety
//!
//! The stack itself is lock-free, but memory reclamation for popped nodes
//! is the caller's responsibility. Use epoch-based reclamation ([`crate::epoch`])
//! or hazard pointers ([`crate::hp`]) for safe memory management.

use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

/// A node in the lock-free stack.
#[repr(C)]
pub struct StackEntry<T> {
    /// Pointer to the next entry in the stack.
    pub next: AtomicPtr<StackEntry<T>>,
    data: T,
}

impl<T> StackEntry<T> {
    /// Create a new stack entry.
    #[inline]
    pub fn new(data: T) -> Self {
        Self {
            next: AtomicPtr::new(ptr::null_mut()),
            data,
        }
    }

    /// Get a reference to the data.
    #[inline]
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Get a mutable reference to the data.
    #[inline]
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    /// Consume the entry and return the data.
    #[inline]
    pub fn into_data(self) -> T {
        self.data
    }
}

/// A lock-free stack (LIFO).
///
/// This is a Treiber stack implementation that uses compare-and-swap
/// for thread-safe push and pop operations.
#[repr(C)]
pub struct Stack<T> {
    head: AtomicPtr<StackEntry<T>>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stack<T> {
    /// Create a new empty stack.
    #[inline]
    pub const fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Check if the stack is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }

    /// Push an entry onto the stack.
    ///
    /// # Safety
    ///
    /// The entry must be valid and must not be already on any stack.
    #[inline]
    pub unsafe fn push(&self, entry: *mut StackEntry<T>) {
        debug_assert!(!entry.is_null());

        loop {
            let head = self.head.load(Ordering::Relaxed);
            (*entry).next.store(head, Ordering::Relaxed);

            if self
                .head
                .compare_exchange_weak(head, entry, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                return;
            }

            crate::pr::stall();
        }
    }

    /// Pop an entry from the stack.
    ///
    /// Returns `None` if the stack is empty.
    ///
    /// # Safety
    ///
    /// The returned entry's memory must not be freed until it's safe
    /// to do so (use epoch-based reclamation or hazard pointers).
    #[inline]
    pub unsafe fn pop(&self) -> Option<*mut StackEntry<T>> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }

            let next = (*head).next.load(Ordering::Relaxed);

            if self
                .head
                .compare_exchange_weak(head, next, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                return Some(head);
            }

            crate::pr::stall();
        }
    }

    /// Pop all entries from the stack.
    ///
    /// Returns the head of the removed chain, or `None` if empty.
    #[inline]
    pub unsafe fn pop_all(&self) -> Option<*mut StackEntry<T>> {
        let head = self.head.swap(ptr::null_mut(), Ordering::AcqRel);
        if head.is_null() {
            None
        } else {
            Some(head)
        }
    }
}

// Stack is Send + Sync if T is Send
unsafe impl<T: Send> Send for Stack<T> {}
unsafe impl<T: Send> Sync for Stack<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;

    #[test]
    fn test_new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_push_pop() {
        let stack: Stack<i32> = Stack::new();

        let entry = Box::into_raw(Box::new(StackEntry::new(42)));

        unsafe {
            stack.push(entry);
            assert!(!stack.is_empty());

            let popped = stack.pop().unwrap();
            assert_eq!((*popped).data, 42);
            assert!(stack.is_empty());

            // Clean up
            drop(Box::from_raw(popped));
        }
    }

    #[test]
    fn test_lifo_order() {
        let stack: Stack<i32> = Stack::new();

        let e1 = Box::into_raw(Box::new(StackEntry::new(1)));
        let e2 = Box::into_raw(Box::new(StackEntry::new(2)));
        let e3 = Box::into_raw(Box::new(StackEntry::new(3)));

        unsafe {
            stack.push(e1);
            stack.push(e2);
            stack.push(e3);

            // Should come out in reverse order
            assert_eq!((*stack.pop().unwrap()).data, 3);
            assert_eq!((*stack.pop().unwrap()).data, 2);
            assert_eq!((*stack.pop().unwrap()).data, 1);
            assert!(stack.pop().is_none());

            // Clean up
            drop(Box::from_raw(e1));
            drop(Box::from_raw(e2));
            drop(Box::from_raw(e3));
        }
    }

    #[test]
    fn test_pop_empty() {
        let stack: Stack<i32> = Stack::new();
        unsafe {
            assert!(stack.pop().is_none());
        }
    }

    #[test]
    fn test_pop_all() {
        let stack: Stack<i32> = Stack::new();

        let e1 = Box::into_raw(Box::new(StackEntry::new(1)));
        let e2 = Box::into_raw(Box::new(StackEntry::new(2)));

        unsafe {
            stack.push(e1);
            stack.push(e2);

            let chain = stack.pop_all().unwrap();
            assert!(stack.is_empty());

            // Chain should be e2 -> e1
            assert_eq!((*chain).data, 2);
            let next = (*chain).next.load(Ordering::Relaxed);
            assert_eq!((*next).data, 1);

            drop(Box::from_raw(e1));
            drop(Box::from_raw(e2));
        }
    }
}
