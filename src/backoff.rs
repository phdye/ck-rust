//! Exponential backoff for contention management.
//!
//! This module provides exponential backoff utilities for reducing contention
//! in spin loops. When multiple threads are competing for a resource, backing
//! off exponentially reduces wasted CPU cycles and improves overall throughput.
//!
//! # Example
//!
//! ```
//! use concurrencykit::backoff::Backoff;
//!
//! let mut backoff = Backoff::new();
//!
//! loop {
//!     if try_acquire_lock() {
//!         break;
//!     }
//!     backoff.spin();
//! }
//! # fn try_acquire_lock() -> bool { true }
//! ```

use crate::pr;

/// Default initial backoff value.
const DEFAULT_CEILING: u32 = 128;

/// Maximum backoff ceiling.
const MAX_CEILING: u32 = 65536;

/// Exponential backoff state.
#[derive(Debug, Clone)]
pub struct Backoff {
    current: u32,
    ceiling: u32,
}

impl Default for Backoff {
    fn default() -> Self {
        Self::new()
    }
}

impl Backoff {
    /// Create a new backoff state with default parameters.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            current: 1,
            ceiling: DEFAULT_CEILING,
        }
    }

    /// Create a new backoff state with a custom ceiling.
    #[inline]
    #[must_use]
    pub const fn with_ceiling(ceiling: u32) -> Self {
        Self {
            current: 1,
            ceiling: if ceiling > MAX_CEILING {
                MAX_CEILING
            } else {
                ceiling
            },
        }
    }

    /// Reset the backoff state.
    #[inline]
    pub fn reset(&mut self) {
        self.current = 1;
    }

    /// Perform a spin-wait with exponential backoff.
    ///
    /// This will spin for an increasing number of iterations, doubling
    /// each time until the ceiling is reached.
    #[inline]
    pub fn spin(&mut self) {
        for _ in 0..self.current {
            pr::stall();
        }

        if self.current < self.ceiling {
            self.current = self.current.saturating_mul(2);
        }
    }

    /// Spin once without updating state.
    ///
    /// Useful for a quick pause without affecting the backoff progression.
    #[inline]
    pub fn snooze(&self) {
        pr::stall();
    }

    /// Check if we've reached the maximum backoff.
    #[inline]
    #[must_use]
    pub fn is_maxed(&self) -> bool {
        self.current >= self.ceiling
    }

    /// Get the current backoff value.
    #[inline]
    #[must_use]
    pub fn current(&self) -> u32 {
        self.current
    }
}

/// Simple inline backoff macro for use in tight loops.
#[macro_export]
macro_rules! backoff {
    () => {
        $crate::pr::stall()
    };
    ($count:expr) => {
        for _ in 0..$count {
            $crate::pr::stall();
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let b = Backoff::new();
        assert_eq!(b.current(), 1);
    }

    #[test]
    fn test_with_ceiling() {
        let b = Backoff::with_ceiling(64);
        assert_eq!(b.ceiling, 64);
    }

    #[test]
    fn test_spin_increases() {
        let mut b = Backoff::new();
        assert_eq!(b.current(), 1);

        b.spin();
        assert_eq!(b.current(), 2);

        b.spin();
        assert_eq!(b.current(), 4);

        b.spin();
        assert_eq!(b.current(), 8);
    }

    #[test]
    fn test_ceiling() {
        let mut b = Backoff::with_ceiling(4);

        b.spin(); // 1 -> 2
        b.spin(); // 2 -> 4
        b.spin(); // 4 -> 4 (capped)
        b.spin(); // 4 -> 4 (capped)

        assert_eq!(b.current(), 4);
        assert!(b.is_maxed());
    }

    #[test]
    fn test_reset() {
        let mut b = Backoff::new();
        b.spin();
        b.spin();
        assert!(b.current() > 1);

        b.reset();
        assert_eq!(b.current(), 1);
    }

    #[test]
    fn test_snooze() {
        let b = Backoff::new();
        let before = b.current();
        b.snooze();
        assert_eq!(b.current(), before); // Should not change
    }

    #[test]
    fn test_max_ceiling() {
        let b = Backoff::with_ceiling(u32::MAX);
        assert_eq!(b.ceiling, MAX_CEILING);
    }
}
