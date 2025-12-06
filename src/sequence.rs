//! Sequence counter (seqlock).
//!
//! This module provides a sequence counter for implementing seqlocks,
//! which allow readers to detect when a write has occurred during their read.
//!
//! # Usage
//!
//! Writers increment the sequence at the start and end of a write.
//! Readers check that the sequence is even and unchanged after reading.
//!
//! ```
//! use concurrencykit::sequence::Sequence;
//!
//! let seq = Sequence::new();
//!
//! // Writer
//! seq.write_begin();
//! // ... modify shared data ...
//! seq.write_end();
//!
//! // Reader
//! loop {
//!     let s1 = seq.read_begin();
//!     // ... read shared data ...
//!     if seq.read_retry(s1) {
//!         continue; // A write occurred, retry
//!     }
//!     break;
//! }
//! ```

use core::sync::atomic::{AtomicUsize, Ordering};

/// A sequence counter for seqlock implementation.
#[repr(C)]
pub struct Sequence {
    counter: AtomicUsize,
}

impl Default for Sequence {
    fn default() -> Self {
        Self::new()
    }
}

impl Sequence {
    /// Create a new sequence counter initialized to 0.
    #[inline]
    pub const fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
        }
    }

    /// Get the current sequence value.
    #[inline]
    pub fn read(&self) -> usize {
        self.counter.load(Ordering::Acquire)
    }

    /// Begin a read-side critical section.
    ///
    /// Returns the current sequence number. If it's odd, a write is in progress.
    #[inline]
    pub fn read_begin(&self) -> usize {
        loop {
            let seq = self.counter.load(Ordering::Acquire);
            if seq & 1 == 0 {
                return seq;
            }
            crate::pr::stall();
        }
    }

    /// Check if a retry is needed after reading.
    ///
    /// Returns `true` if the sequence changed (write occurred), meaning
    /// the read should be retried.
    #[inline]
    pub fn read_retry(&self, start: usize) -> bool {
        crate::pr::fence_acquire();
        self.counter.load(Ordering::Relaxed) != start
    }

    /// Begin a write-side critical section.
    ///
    /// Increments the sequence to an odd number, indicating a write is in progress.
    #[inline]
    pub fn write_begin(&self) {
        let seq = self.counter.fetch_add(1, Ordering::Release);
        debug_assert!(seq & 1 == 0, "nested write_begin");
    }

    /// End a write-side critical section.
    ///
    /// Increments the sequence to an even number, indicating the write is complete.
    #[inline]
    pub fn write_end(&self) {
        let seq = self.counter.fetch_add(1, Ordering::Release);
        debug_assert!(seq & 1 == 1, "write_end without write_begin");
    }

    /// Perform a complete write sequence.
    ///
    /// Convenience method that calls `write_begin`, executes the closure,
    /// then calls `write_end`.
    #[inline]
    pub fn write<F, R>(&self, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        self.write_begin();
        let result = f();
        self.write_end();
        result
    }

    /// Perform a read with automatic retry.
    ///
    /// Calls the closure repeatedly until a consistent read is achieved.
    #[inline]
    pub fn read_with<F, R>(&self, mut f: F) -> R
    where
        F: FnMut() -> R,
        R: Copy,
    {
        loop {
            let seq = self.read_begin();
            let result = f();
            if !self.read_retry(seq) {
                return result;
            }
        }
    }
}

// Sequence is Send + Sync
unsafe impl Send for Sequence {}
unsafe impl Sync for Sequence {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let seq = Sequence::new();
        assert_eq!(seq.read(), 0);
    }

    #[test]
    fn test_write_sequence() {
        let seq = Sequence::new();

        seq.write_begin();
        assert_eq!(seq.read(), 1); // Odd during write

        seq.write_end();
        assert_eq!(seq.read(), 2); // Even after write
    }

    #[test]
    fn test_read_begin_waits_for_write() {
        let seq = Sequence::new();

        // Complete write should work
        seq.write_begin();
        seq.write_end();

        let s = seq.read_begin();
        assert_eq!(s, 2);
        assert!(!seq.read_retry(s));
    }

    #[test]
    fn test_read_retry_detects_write() {
        let seq = Sequence::new();

        let s1 = seq.read_begin();
        seq.write_begin();
        seq.write_end();

        assert!(seq.read_retry(s1));
    }

    #[test]
    fn test_write_closure() {
        let seq = Sequence::new();
        let result = seq.write(|| {
            assert_eq!(seq.read() & 1, 1); // Odd during write
            42
        });
        assert_eq!(result, 42);
        assert_eq!(seq.read(), 2);
    }

    #[test]
    fn test_read_with() {
        let seq = Sequence::new();
        let counter = core::sync::atomic::AtomicUsize::new(0);

        let result = seq.read_with(|| {
            counter.fetch_add(1, Ordering::Relaxed);
            42
        });

        assert_eq!(result, 42);
        assert_eq!(counter.load(Ordering::Relaxed), 1);
    }
}
