//! Event counts for blocking synchronization.
//!
//! Event counts provide efficient blocking synchronization by allowing
//! threads to wait for a condition with low overhead when not contended.

use core::sync::atomic::{AtomicU64, Ordering};

use crate::backoff::Backoff;

/// Bits used for waiter count.
const WAITER_BITS: u32 = 32;
/// Mask for waiter count.
const WAITER_MASK: u64 = (1u64 << WAITER_BITS) - 1;

/// An event count for blocking synchronization.
///
/// Event counts combine a counter with a waiter count, allowing efficient
/// wake-up of waiting threads when conditions change.
#[repr(C)]
pub struct EventCount {
    state: AtomicU64,
}

impl EventCount {
    /// Create a new event count.
    #[inline]
    pub const fn new() -> Self {
        Self {
            state: AtomicU64::new(0),
        }
    }

    /// Get the current event count (epoch).
    #[inline]
    pub fn get(&self) -> u64 {
        self.state.load(Ordering::Acquire) >> WAITER_BITS
    }

    /// Prepare to wait (increment waiter count).
    ///
    /// Returns the current epoch for use with `wait`.
    #[inline]
    pub fn prepare_wait(&self) -> u64 {
        let state = self.state.fetch_add(1, Ordering::AcqRel);
        state >> WAITER_BITS
    }

    /// Cancel a prepared wait (decrement waiter count).
    #[inline]
    pub fn cancel_wait(&self) {
        self.state.fetch_sub(1, Ordering::Release);
    }

    /// Wait for the epoch to change from the given value.
    ///
    /// This spins until the epoch changes. In a real implementation,
    /// this would use futex or similar for blocking.
    pub fn wait(&self, epoch: u64) {
        let mut backoff = Backoff::new();

        // Decrement waiter count
        self.state.fetch_sub(1, Ordering::Release);

        // Spin until epoch changes
        while self.get() == epoch {
            backoff.spin();
        }
    }

    /// Signal one waiting thread.
    ///
    /// Increments the epoch and wakes at least one waiter.
    pub fn notify_one(&self) {
        self.advance();
        // In a real implementation, wake one waiter via futex
    }

    /// Signal all waiting threads.
    ///
    /// Increments the epoch and wakes all waiters.
    pub fn notify_all(&self) {
        self.advance();
        // In a real implementation, wake all waiters via futex
    }

    /// Advance the epoch.
    #[inline]
    fn advance(&self) {
        self.state.fetch_add(1u64 << WAITER_BITS, Ordering::Release);
    }

    /// Get the number of waiting threads.
    #[inline]
    pub fn waiters(&self) -> u32 {
        (self.state.load(Ordering::Acquire) & WAITER_MASK) as u32
    }
}

impl Default for EventCount {
    fn default() -> Self {
        Self::new()
    }
}

/// A condition variable built on event counts.
pub struct Condvar {
    ec: EventCount,
}

impl Condvar {
    /// Create a new condition variable.
    #[inline]
    pub const fn new() -> Self {
        Self {
            ec: EventCount::new(),
        }
    }

    /// Wait on the condition variable.
    ///
    /// The caller should hold a lock and check a condition in a loop.
    pub fn wait<F>(&self, mut condition: F)
    where
        F: FnMut() -> bool,
    {
        loop {
            let epoch = self.ec.prepare_wait();

            if condition() {
                self.ec.cancel_wait();
                return;
            }

            self.ec.wait(epoch);
        }
    }

    /// Notify one waiting thread.
    #[inline]
    pub fn notify_one(&self) {
        self.ec.notify_one();
    }

    /// Notify all waiting threads.
    #[inline]
    pub fn notify_all(&self) {
        self.ec.notify_all();
    }
}

impl Default for Condvar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_count_new() {
        let ec = EventCount::new();
        assert_eq!(ec.get(), 0);
        assert_eq!(ec.waiters(), 0);
    }

    #[test]
    fn test_notify() {
        let ec = EventCount::new();
        let epoch1 = ec.get();
        ec.notify_one();
        let epoch2 = ec.get();
        assert!(epoch2 > epoch1);
    }

    #[test]
    fn test_prepare_cancel() {
        let ec = EventCount::new();

        let epoch = ec.prepare_wait();
        assert_eq!(ec.waiters(), 1);

        ec.cancel_wait();
        assert_eq!(ec.waiters(), 0);
        assert_eq!(ec.get(), epoch);
    }

    #[test]
    fn test_condvar() {
        let cv = Condvar::new();
        let mut done = false;

        cv.wait(|| {
            done = true;
            done
        });

        assert!(done);
    }
}
