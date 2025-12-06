//! Hazard pointer protected FIFO.
//!
//! A lock-free FIFO queue with hazard pointer based safe memory reclamation.

use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::hp::{HazardPointers, HpGuard};

/// A node in the HP-protected FIFO.
#[repr(C)]
pub struct FifoEntry<T> {
    next: AtomicPtr<FifoEntry<T>>,
    data: T,
}

impl<T> FifoEntry<T> {
    /// Create a new entry.
    #[inline]
    pub fn new(data: T) -> Self {
        Self {
            next: AtomicPtr::new(ptr::null_mut()),
            data,
        }
    }
}

/// A hazard pointer protected FIFO queue.
pub struct HpFifo<T> {
    head: AtomicPtr<FifoEntry<T>>,
    tail: AtomicPtr<FifoEntry<T>>,
    hp: HazardPointers,
}

impl<T> HpFifo<T> {
    /// Create a new empty FIFO.
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
            tail: AtomicPtr::new(ptr::null_mut()),
            hp: HazardPointers::new(),
        }
    }

    /// Register a thread for HP-protected access.
    pub fn register(&self) -> HpFifoGuard<'_, T> {
        HpFifoGuard {
            fifo: self,
            hp_guard: self.hp.register(),
        }
    }

    /// Check if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }
}

impl<T> Default for HpFifo<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A guard for HP-protected FIFO operations.
pub struct HpFifoGuard<'a, T> {
    fifo: &'a HpFifo<T>,
    hp_guard: HpGuard<'a>,
}

impl<'a, T> HpFifoGuard<'a, T> {
    /// Enqueue an entry.
    ///
    /// # Safety
    ///
    /// The entry must be valid and not already in any queue.
    pub unsafe fn enqueue(&self, entry: *mut FifoEntry<T>) {
        (*entry).next.store(ptr::null_mut(), Ordering::Relaxed);

        loop {
            let tail = self.fifo.tail.load(Ordering::Acquire);

            if tail.is_null() {
                // Queue is empty
                if self
                    .fifo
                    .head
                    .compare_exchange(ptr::null_mut(), entry, Ordering::Release, Ordering::Relaxed)
                    .is_ok()
                {
                    self.fifo.tail.store(entry, Ordering::Release);
                    return;
                }
            } else {
                // Try to link at tail
                self.hp_guard.protect(0, tail);

                // Verify tail hasn't changed
                if self.fifo.tail.load(Ordering::Acquire) != tail {
                    continue;
                }

                let next = (*tail).next.load(Ordering::Acquire);
                if next.is_null() {
                    if (*tail)
                        .next
                        .compare_exchange(ptr::null_mut(), entry, Ordering::Release, Ordering::Relaxed)
                        .is_ok()
                    {
                        let _ = self.fifo.tail.compare_exchange(
                            tail,
                            entry,
                            Ordering::Release,
                            Ordering::Relaxed,
                        );
                        return;
                    }
                } else {
                    // Help advance tail
                    let _ = self.fifo.tail.compare_exchange(
                        tail,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed,
                    );
                }
            }

            crate::pr::stall();
        }
    }

    /// Dequeue an entry.
    ///
    /// Returns `None` if the queue is empty.
    pub unsafe fn dequeue(&self) -> Option<*mut FifoEntry<T>> {
        loop {
            let head = self.fifo.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }

            self.hp_guard.protect(0, head);

            // Verify head hasn't changed
            if self.fifo.head.load(Ordering::Acquire) != head {
                continue;
            }

            let next = (*head).next.load(Ordering::Acquire);

            if self
                .fifo
                .head
                .compare_exchange(head, next, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                if next.is_null() {
                    let _ = self.fifo.tail.compare_exchange(
                        head,
                        ptr::null_mut(),
                        Ordering::Release,
                        Ordering::Relaxed,
                    );
                }

                self.hp_guard.clear(0);
                return Some(head);
            }

            crate::pr::stall();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;

    #[test]
    fn test_new() {
        let fifo: HpFifo<i32> = HpFifo::new();
        assert!(fifo.is_empty());
    }

    #[test]
    fn test_enqueue_dequeue() {
        let fifo: HpFifo<i32> = HpFifo::new();
        let guard = fifo.register();

        let entry = Box::into_raw(Box::new(FifoEntry::new(42)));

        unsafe {
            guard.enqueue(entry);
            assert!(!fifo.is_empty());

            let dequeued = guard.dequeue().unwrap();
            assert_eq!((*dequeued).data, 42);
            assert!(fifo.is_empty());

            drop(Box::from_raw(dequeued));
        }
    }
}
