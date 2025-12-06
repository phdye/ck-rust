//! Sequence lock (seqlock) for optimistic read-copy consistency.
//!
//! This module provides a sequence lock implementation for optimistic read
//! synchronization. Readers speculatively read data without acquiring a lock,
//! then verify the data wasn't modified during the read. Writers increment
//! a sequence counter before and after modifications, allowing readers to
//! detect concurrent writes.
//!
//! # Usage
//!
//! ```
//! use concurrencykit::sequence::SeqLock;
//!
//! let seq = SeqLock::new();
//!
//! // Reader pattern
//! loop {
//!     let version = seq.read_begin();
//!     // ... read protected data ...
//!     if !seq.read_retry(version) {
//!         break; // Read was consistent
//!     }
//! }
//!
//! // Writer pattern (must hold external mutex)
//! seq.write_begin();
//! // ... modify protected data ...
//! seq.write_end();
//! ```
//!
//! # Thread Safety
//!
//! - **Readers**: Multiple concurrent readers are safe
//! - **Writers**: Must use external mutex for mutual exclusion
//! - **Reader vs Writer**: Readers detect concurrent writes and retry

use core::sync::atomic::{AtomicU32, Ordering};
use crate::pr;

/// Sequence lock for optimistic read synchronization.
///
/// The sequence counter uses the following protocol:
/// - Even value: No write in progress (stable state)
/// - Odd value: Write in progress
///
/// Writers increment the counter before and after modifications,
/// allowing readers to detect concurrent writes by comparing
/// the version before and after reading.
#[repr(C)]
pub struct SeqLock {
    sequence: AtomicU32,
}

impl SeqLock {
    /// Create a new sequence lock in the stable (readable) state.
    #[inline]
    pub const fn new() -> Self {
        Self {
            sequence: AtomicU32::new(0),
        }
    }

    /// Initialize the sequence lock to the stable state.
    ///
    /// This is primarily for compatibility with C-style initialization.
    #[inline]
    pub fn init(&self) {
        self.sequence.store(0, Ordering::Release);
    }

    /// Get the raw sequence value (for testing/debugging).
    #[inline]
    pub fn raw_sequence(&self) -> u32 {
        self.sequence.load(Ordering::Acquire)
    }

    /// Begin an optimistic read.
    ///
    /// Returns a version number that must be passed to [`read_retry`](Self::read_retry)
    /// after reading the protected data.
    ///
    /// If a write is in progress, this function will spin until the write completes.
    #[inline]
    pub fn read_begin(&self) -> u32 {
        loop {
            let seq = self.sequence.load(Ordering::Acquire);
            if seq & 1 == 0 {
                // Stable state (even), no write in progress
                return seq;
            }
            // Writer active, spin
            pr::stall();
        }
    }

    /// Check if a read should be retried due to a concurrent write.
    ///
    /// Returns `true` if the read was interrupted by a write and must be retried.
    /// Returns `false` if the read was consistent.
    #[inline]
    pub fn read_retry(&self, version: u32) -> bool {
        // Load fence to ensure all reads complete before checking
        core::sync::atomic::fence(Ordering::Acquire);
        self.sequence.load(Ordering::Relaxed) != version
    }

    /// Begin a write phase.
    ///
    /// **Important**: The caller must hold an external mutex to prevent
    /// concurrent writers. This function only handles reader synchronization.
    ///
    /// After calling this, the sequence will be odd (indicating write in progress).
    #[inline]
    pub fn write_begin(&self) {
        let seq = self.sequence.load(Ordering::Relaxed);
        self.sequence.store(seq.wrapping_add(1), Ordering::Release);
        // Fence to ensure the increment is visible before any writes
        core::sync::atomic::fence(Ordering::Release);
    }

    /// End a write phase.
    ///
    /// After calling this, the sequence will be even (stable state).
    /// Readers that started before the write will detect the change.
    #[inline]
    pub fn write_end(&self) {
        // Fence to ensure all writes complete before the increment
        core::sync::atomic::fence(Ordering::Release);
        let seq = self.sequence.load(Ordering::Relaxed);
        self.sequence.store(seq.wrapping_add(1), Ordering::Release);
    }

    /// Perform a read with automatic retry.
    ///
    /// The provided closure is called repeatedly until a consistent read
    /// is achieved (no concurrent write detected).
    ///
    /// # Example
    ///
    /// ```
    /// use concurrencykit::sequence::SeqLock;
    ///
    /// let seq = SeqLock::new();
    /// let data = 42u64; // Protected data
    ///
    /// let value = seq.read(|| {
    ///     // Read protected data
    ///     data
    /// });
    /// ```
    #[inline]
    pub fn read<T, F: FnMut() -> T>(&self, mut f: F) -> T {
        loop {
            let version = self.read_begin();
            let result = f();
            if !self.read_retry(version) {
                return result;
            }
        }
    }

    /// Perform a write with automatic begin/end.
    ///
    /// **Important**: The caller must hold an external mutex.
    ///
    /// # Example
    ///
    /// ```
    /// use concurrencykit::sequence::SeqLock;
    ///
    /// let seq = SeqLock::new();
    ///
    /// // (Caller holds mutex)
    /// seq.write(|| {
    ///     // Modify protected data
    /// });
    /// ```
    #[inline]
    pub fn write<F: FnOnce()>(&self, f: F) {
        self.write_begin();
        f();
        self.write_end();
    }
}

impl Default for SeqLock {
    fn default() -> Self {
        Self::new()
    }
}

// Safety: SeqLock is thread-safe for reads; writes require external synchronization
unsafe impl Sync for SeqLock {}
unsafe impl Send for SeqLock {}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST-001: init_sets_zero
    #[test]
    fn test_init_sets_zero() {
        let seq = SeqLock::new();
        assert_eq!(seq.raw_sequence(), 0);
    }

    #[test]
    fn test_init_method() {
        let seq = SeqLock::new();
        seq.write_begin();
        seq.write_end();
        assert_eq!(seq.raw_sequence(), 2);
        seq.init();
        assert_eq!(seq.raw_sequence(), 0);
    }

    // TEST-002: read_returns_even
    #[test]
    fn test_read_returns_even() {
        let seq = SeqLock::new();
        let version = seq.read_begin();
        assert_eq!(version & 1, 0); // Even
    }

    // TEST-003: retry_false_no_write
    #[test]
    fn test_retry_false_no_write() {
        let seq = SeqLock::new();
        let version = seq.read_begin();
        // No write occurred
        assert!(!seq.read_retry(version));
    }

    // TEST-004: retry_true_after_write
    #[test]
    fn test_retry_true_after_write() {
        let seq = SeqLock::new();
        let version = seq.read_begin();

        // Perform a write
        seq.write_begin();
        seq.write_end();

        // Should need to retry
        assert!(seq.read_retry(version));
        assert_eq!(seq.raw_sequence(), version + 2);
    }

    // TEST-005: write_makes_odd_then_even
    #[test]
    fn test_write_makes_odd_then_even() {
        let seq = SeqLock::new();
        assert_eq!(seq.raw_sequence(), 0);

        seq.write_begin();
        let mid = seq.raw_sequence();
        assert_eq!(mid, 1); // Odd during write

        seq.write_end();
        assert_eq!(seq.raw_sequence(), 2); // Even after write
    }

    // TEST-006: read closure pattern
    #[test]
    fn test_read_closure() {
        let seq = SeqLock::new();
        let data = 42u64;

        let value = seq.read(|| data);
        assert_eq!(value, 42);
    }

    // Test write closure pattern
    #[test]
    fn test_write_closure() {
        let seq = SeqLock::new();
        let mut data = 0;

        seq.write(|| {
            data = 100;
        });

        assert_eq!(data, 100);
        assert_eq!(seq.raw_sequence(), 2);
    }

    // Test multiple writes
    #[test]
    fn test_multiple_writes() {
        let seq = SeqLock::new();

        for i in 0..10 {
            seq.write_begin();
            seq.write_end();
            assert_eq!(seq.raw_sequence(), (i + 1) * 2);
        }
    }

    // Test default trait
    #[test]
    fn test_default() {
        let seq: SeqLock = Default::default();
        assert_eq!(seq.raw_sequence(), 0);
    }

    // Test read during stable state
    #[test]
    fn test_read_stable() {
        let seq = SeqLock::new();

        // Multiple reads should all succeed immediately
        for _ in 0..100 {
            let version = seq.read_begin();
            assert!(!seq.read_retry(version));
        }
    }

    // Test version increases correctly
    #[test]
    fn test_version_increases() {
        let seq = SeqLock::new();

        let v1 = seq.read_begin();
        seq.write_begin();
        seq.write_end();
        let v2 = seq.read_begin();

        assert_eq!(v2, v1 + 2);
    }
}
