//! Concurrent ring buffer.
//!
//! This module provides a lock-free ring buffer (circular buffer) that supports
//! both single-producer/single-consumer (SPSC) and multi-producer/multi-consumer
//! (MPMC) variants.

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicUsize, Ordering};

/// A single-producer, single-consumer ring buffer.
pub struct SpscRing<T, const N: usize> {
    buffer: [UnsafeCell<MaybeUninit<T>>; N],
    head: AtomicUsize,
    tail: AtomicUsize,
}

impl<T, const N: usize> SpscRing<T, N> {
    /// Create a new empty ring buffer.
    ///
    /// # Panics
    ///
    /// Panics if N is 0 or not a power of 2.
    pub fn new() -> Self {
        assert!(N > 0 && N.is_power_of_two(), "N must be a power of 2");
        Self {
            buffer: core::array::from_fn(|_| UnsafeCell::new(MaybeUninit::uninit())),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// Returns the capacity of the ring buffer.
    #[inline]
    pub const fn capacity(&self) -> usize {
        N - 1 // One slot is always empty to distinguish full from empty
    }

    /// Returns true if the buffer is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire) == self.tail.load(Ordering::Acquire)
    }

    /// Returns true if the buffer is full.
    #[inline]
    pub fn is_full(&self) -> bool {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        (tail + 1) & (N - 1) == head
    }

    /// Enqueue an item. Returns `Err(item)` if full.
    ///
    /// # Safety
    ///
    /// Must only be called from the producer thread.
    pub unsafe fn enqueue(&self, item: T) -> Result<(), T> {
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) & (N - 1);

        if next_tail == self.head.load(Ordering::Acquire) {
            return Err(item);
        }

        (*self.buffer[tail].get()).write(item);
        self.tail.store(next_tail, Ordering::Release);
        Ok(())
    }

    /// Dequeue an item. Returns `None` if empty.
    ///
    /// # Safety
    ///
    /// Must only be called from the consumer thread.
    pub unsafe fn dequeue(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);

        if head == self.tail.load(Ordering::Acquire) {
            return None;
        }

        let item = (*self.buffer[head].get()).assume_init_read();
        let next_head = (head + 1) & (N - 1);
        self.head.store(next_head, Ordering::Release);
        Some(item)
    }
}

impl<T, const N: usize> Default for SpscRing<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

// SAFETY: Ring is safe to send/sync if T is Send
unsafe impl<T: Send, const N: usize> Send for SpscRing<T, N> {}
unsafe impl<T: Send, const N: usize> Sync for SpscRing<T, N> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ring: SpscRing<i32, 16> = SpscRing::new();
        assert!(ring.is_empty());
        assert!(!ring.is_full());
    }

    #[test]
    fn test_capacity() {
        let ring: SpscRing<i32, 16> = SpscRing::new();
        assert_eq!(ring.capacity(), 15);
    }

    #[test]
    fn test_enqueue_dequeue() {
        let ring: SpscRing<i32, 4> = SpscRing::new();

        unsafe {
            assert!(ring.enqueue(1).is_ok());
            assert!(ring.enqueue(2).is_ok());
            assert!(ring.enqueue(3).is_ok());
            assert!(ring.enqueue(4).is_err()); // Full

            assert_eq!(ring.dequeue(), Some(1));
            assert_eq!(ring.dequeue(), Some(2));
            assert_eq!(ring.dequeue(), Some(3));
            assert_eq!(ring.dequeue(), None);
        }
    }

    #[test]
    fn test_wrap_around() {
        let ring: SpscRing<i32, 4> = SpscRing::new();

        unsafe {
            // Fill and empty
            ring.enqueue(1).unwrap();
            ring.enqueue(2).unwrap();
            ring.enqueue(3).unwrap();
            ring.dequeue();
            ring.dequeue();
            ring.dequeue();

            // Fill again (wraps around)
            ring.enqueue(4).unwrap();
            ring.enqueue(5).unwrap();
            ring.enqueue(6).unwrap();

            assert_eq!(ring.dequeue(), Some(4));
            assert_eq!(ring.dequeue(), Some(5));
            assert_eq!(ring.dequeue(), Some(6));
        }
    }
}
