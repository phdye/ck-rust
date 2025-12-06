//! Hazard pointer protected stack.
//!
//! A lock-free stack with hazard pointer based safe memory reclamation.

use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::hp::{HazardPointers, HpGuard};
use crate::stack::StackEntry;

/// A hazard pointer protected stack.
pub struct HpStack<T> {
    head: AtomicPtr<StackEntry<T>>,
    hp: HazardPointers,
}

impl<T> HpStack<T> {
    /// Create a new empty stack.
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
            hp: HazardPointers::new(),
        }
    }

    /// Register a thread for HP-protected access.
    pub fn register(&self) -> HpStackGuard<'_, T> {
        HpStackGuard {
            stack: self,
            hp_guard: self.hp.register(),
        }
    }

    /// Check if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }
}

impl<T> Default for HpStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A guard for HP-protected stack operations.
pub struct HpStackGuard<'a, T> {
    stack: &'a HpStack<T>,
    hp_guard: HpGuard<'a>,
}

impl<'a, T> HpStackGuard<'a, T> {
    /// Push an entry onto the stack.
    ///
    /// # Safety
    ///
    /// The entry must be valid and not already on any stack.
    pub unsafe fn push(&self, entry: *mut StackEntry<T>) {
        loop {
            let head = self.stack.head.load(Ordering::Relaxed);
            (*entry).next.store(head, Ordering::Relaxed);

            if self
                .stack
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
    pub unsafe fn pop(&self) -> Option<*mut StackEntry<T>> {
        loop {
            let head = self.stack.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }

            // Protect the head with hazard pointer
            self.hp_guard.protect(0, head);

            // Verify head hasn't changed
            if self.stack.head.load(Ordering::Acquire) != head {
                continue;
            }

            let next = (*head).next.load(Ordering::Relaxed);

            if self
                .stack
                .head
                .compare_exchange_weak(head, next, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                self.hp_guard.clear(0);
                return Some(head);
            }

            crate::pr::stall();
        }
    }

    /// Safely retire a popped entry.
    pub unsafe fn retire(&self, entry: *mut StackEntry<T>) {
        self.hp_guard.retire(entry);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;

    #[test]
    fn test_new() {
        let stack: HpStack<i32> = HpStack::new();
        assert!(stack.is_empty());
    }

    #[test]
    fn test_push_pop() {
        let stack: HpStack<i32> = HpStack::new();
        let guard = stack.register();

        let entry = Box::into_raw(Box::new(StackEntry::new(42)));

        unsafe {
            guard.push(entry);
            assert!(!stack.is_empty());

            let popped = guard.pop().unwrap();
            assert_eq!(*(*popped).data(), 42);
            assert!(stack.is_empty());

            drop(Box::from_raw(popped));
        }
    }
}
