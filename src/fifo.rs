//! Lock-free FIFO queues.
//!
//! This module provides FIFO queue implementations:
//! - SPSC: Single-producer, single-consumer
//! - MPMC: Multi-producer, multi-consumer

use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::spinlock::SpinLock;

/// A node in the FIFO.
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
}

/// Single-producer, single-consumer FIFO.
pub struct SpscFifo<T> {
    head: AtomicPtr<FifoEntry<T>>,
    tail: AtomicPtr<FifoEntry<T>>,
}

impl<T> SpscFifo<T> {
    /// Create a new empty SPSC FIFO.
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
            tail: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Check if the queue is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }

    /// Enqueue an entry (producer only).
    ///
    /// # Safety
    ///
    /// Must only be called from the producer thread.
    pub unsafe fn enqueue(&self, entry: *mut FifoEntry<T>) {
        (*entry).next.store(ptr::null_mut(), Ordering::Relaxed);

        let tail = self.tail.load(Ordering::Relaxed);
        if tail.is_null() {
            self.head.store(entry, Ordering::Release);
        } else {
            (*tail).next.store(entry, Ordering::Release);
        }
        self.tail.store(entry, Ordering::Release);
    }

    /// Dequeue an entry (consumer only).
    ///
    /// # Safety
    ///
    /// Must only be called from the consumer thread.
    pub unsafe fn dequeue(&self) -> Option<*mut FifoEntry<T>> {
        let head = self.head.load(Ordering::Acquire);
        if head.is_null() {
            return None;
        }

        let next = (*head).next.load(Ordering::Acquire);
        self.head.store(next, Ordering::Release);

        if next.is_null() {
            self.tail.store(ptr::null_mut(), Ordering::Release);
        }

        Some(head)
    }
}

impl<T> Default for SpscFifo<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Multi-producer, multi-consumer FIFO.
pub struct MpmcFifo<T> {
    head: SpinLock<*mut FifoEntry<T>>,
    tail: SpinLock<*mut FifoEntry<T>>,
}

impl<T> MpmcFifo<T> {
    /// Create a new empty MPMC FIFO.
    pub fn new() -> Self {
        Self {
            head: SpinLock::new(ptr::null_mut()),
            tail: SpinLock::new(ptr::null_mut()),
        }
    }

    /// Check if the queue is empty.
    pub fn is_empty(&self) -> bool {
        let head = self.head.lock();
        head.is_null()
    }

    /// Enqueue an entry.
    ///
    /// # Safety
    ///
    /// The entry must be valid and not in any queue.
    pub unsafe fn enqueue(&self, entry: *mut FifoEntry<T>) {
        (*entry).next.store(ptr::null_mut(), Ordering::Relaxed);

        let mut tail = self.tail.lock();
        if (*tail).is_null() {
            let mut head = self.head.lock();
            *head = entry;
        } else {
            (**tail).next.store(entry, Ordering::Release);
        }
        *tail = entry;
    }

    /// Dequeue an entry.
    pub unsafe fn dequeue(&self) -> Option<*mut FifoEntry<T>> {
        let mut head = self.head.lock();
        let head_ptr = *head;

        if head_ptr.is_null() {
            return None;
        }

        let next = (*head_ptr).next.load(Ordering::Acquire);
        *head = next;

        if next.is_null() {
            let mut tail = self.tail.lock();
            *tail = ptr::null_mut();
        }

        Some(head_ptr)
    }
}

impl<T> Default for MpmcFifo<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;

    #[test]
    fn test_spsc_new() {
        let fifo: SpscFifo<i32> = SpscFifo::new();
        assert!(fifo.is_empty());
    }

    #[test]
    fn test_spsc_enqueue_dequeue() {
        let fifo: SpscFifo<i32> = SpscFifo::new();

        let e1 = Box::into_raw(Box::new(FifoEntry::new(1)));
        let e2 = Box::into_raw(Box::new(FifoEntry::new(2)));

        unsafe {
            fifo.enqueue(e1);
            fifo.enqueue(e2);

            assert!(!fifo.is_empty());

            assert_eq!((*fifo.dequeue().unwrap()).data, 1);
            assert_eq!((*fifo.dequeue().unwrap()).data, 2);
            assert!(fifo.dequeue().is_none());

            drop(Box::from_raw(e1));
            drop(Box::from_raw(e2));
        }
    }

    #[test]
    fn test_mpmc_new() {
        let fifo: MpmcFifo<i32> = MpmcFifo::new();
        assert!(fifo.is_empty());
    }

    #[test]
    fn test_mpmc_enqueue_dequeue() {
        let fifo: MpmcFifo<i32> = MpmcFifo::new();

        let entry = Box::into_raw(Box::new(FifoEntry::new(42)));

        unsafe {
            fifo.enqueue(entry);
            assert!(!fifo.is_empty());

            let dequeued = fifo.dequeue().unwrap();
            assert_eq!((*dequeued).data, 42);

            drop(Box::from_raw(dequeued));
        }
    }
}
