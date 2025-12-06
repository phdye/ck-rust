//! Exponential backoff for spin loops.
//!
//! This module provides exponential backoff functionality for spin loops.
//! Exponential backoff reduces contention on shared resources by introducing
//! progressively longer delays between retry attempts, improving overall
//! system throughput under contention.
//!
//! # Usage
//!
//! ```
//! use concurrencykit::backoff::Backoff;
//!
//! let mut backoff = Backoff::new();
//!
//! // In a spin loop:
//! loop {
//!     // Try to acquire resource...
//!     # break; // for doctest
//!     backoff.spin();
//! }
//! ```
//!
//! # Thread Safety
//!
//! This module is NOT thread-safe. Each thread should have its own [`Backoff`]
//! instance. The backoff state must not be shared between threads.

use crate::pr;

/// Default initial backoff value (512 iterations).
pub const BACKOFF_INITIAL: u32 = 1 << 9;

/// Maximum backoff ceiling (about 1 million iterations).
pub const BACKOFF_CEILING: u32 = (1 << 20) - 1;

/// Exponential backoff state.
///
/// Each call to [`spin()`](Backoff::spin) executes a delay proportional to the
/// current value, then doubles the value up to [`BACKOFF_CEILING`].
#[derive(Debug, Clone, Copy)]
pub struct Backoff {
    value: u32,
}

impl Default for Backoff {
    fn default() -> Self {
        Self::new()
    }
}

impl Backoff {
    /// Create a new backoff state with the default initial value.
    #[inline]
    pub const fn new() -> Self {
        Self {
            value: BACKOFF_INITIAL,
        }
    }

    /// Create a new backoff state with a custom initial value.
    ///
    /// Note: Using 0 will result in a stuck state where the backoff never increases.
    #[inline]
    pub const fn with_initial(initial: u32) -> Self {
        Self { value: initial }
    }

    /// Get the current backoff value.
    #[inline]
    pub const fn value(&self) -> u32 {
        self.value
    }

    /// Reset the backoff to the initial value.
    #[inline]
    pub fn reset(&mut self) {
        self.value = BACKOFF_INITIAL;
    }

    /// Execute the exponential backoff delay.
    ///
    /// This method:
    /// 1. Spins for `value` iterations
    /// 2. Doubles `value` (up to [`BACKOFF_CEILING`])
    ///
    /// The delay helps reduce contention in spin loops by giving other threads
    /// a chance to make progress.
    #[inline]
    pub fn spin(&mut self) {
        let ceiling = self.value;

        // Execute delay loop
        for _ in 0..ceiling {
            pr::stall();
        }

        // Double the backoff value, but don't exceed ceiling
        if ceiling < BACKOFF_CEILING {
            self.value = ceiling.saturating_mul(2).min(BACKOFF_CEILING);
        }
    }

    /// Execute a single spin iteration without updating the backoff value.
    ///
    /// This is useful when you want to spin but not increase the backoff.
    #[inline]
    pub fn spin_once(&self) {
        for _ in 0..self.value {
            pr::stall();
        }
    }
}

/// Backoff state type (for C-style API compatibility).
pub type BackoffT = u32;

/// C-style backoff function.
///
/// Executes exponential backoff and updates the ceiling value.
#[inline]
pub fn eb(ceiling: &mut BackoffT) {
    let c = *ceiling;

    // Execute delay loop
    for _ in 0..c {
        pr::stall();
    }

    // Double the backoff value, but don't exceed ceiling
    if c < BACKOFF_CEILING {
        *ceiling = c.saturating_mul(2).min(BACKOFF_CEILING);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST-001: initialization
    #[test]
    fn test_initialization() {
        let backoff = Backoff::new();
        assert_eq!(backoff.value(), 512); // 1 << 9
    }

    // TEST-002: single_backoff
    #[test]
    fn test_single_backoff() {
        let mut backoff = Backoff::new();
        backoff.spin();
        assert_eq!(backoff.value(), 1024); // 512 * 2
    }

    // TEST-003: exponential_growth
    #[test]
    fn test_exponential_growth() {
        let mut backoff = Backoff::new();

        backoff.spin();
        assert_eq!(backoff.value(), 1024);

        backoff.spin();
        assert_eq!(backoff.value(), 2048);

        backoff.spin();
        assert_eq!(backoff.value(), 4096);

        backoff.spin();
        assert_eq!(backoff.value(), 8192);

        backoff.spin();
        assert_eq!(backoff.value(), 16384);
    }

    // TEST-004: ceiling_reached
    #[test]
    fn test_ceiling_reached() {
        let mut backoff = Backoff::with_initial(BACKOFF_CEILING);
        backoff.spin();
        assert_eq!(backoff.value(), BACKOFF_CEILING);
    }

    // TEST-005: ceiling_approach
    #[test]
    fn test_ceiling_approach() {
        let mut backoff = Backoff::new();
        let mut prev_value = backoff.value();

        // Keep spinning until value stops changing
        for _ in 0..30 {
            backoff.spin();
            if backoff.value() == prev_value {
                break;
            }
            prev_value = backoff.value();
        }

        // Should have reached the ceiling
        assert!(backoff.value() >= BACKOFF_CEILING);
    }

    // TEST-006: small_initial_value
    #[test]
    fn test_small_initial_value() {
        let mut backoff = Backoff::with_initial(1);
        backoff.spin();
        assert_eq!(backoff.value(), 2);
    }

    // TEST-007: zero_value_behavior
    #[test]
    fn test_zero_value_behavior() {
        let mut backoff = Backoff::with_initial(0);
        backoff.spin();
        // 0 * 2 = 0, so it stays at 0
        assert_eq!(backoff.value(), 0);
    }

    // TEST-008: delay_occurs - hard to test precisely, but verify it doesn't panic
    #[test]
    fn test_delay_occurs() {
        let mut backoff = Backoff::with_initial(10); // Small value for test speed
        backoff.spin();
        // If we got here without hanging, the test passes
    }

    // Test C-style API
    #[test]
    fn test_eb_function() {
        let mut ceiling: BackoffT = BACKOFF_INITIAL;
        eb(&mut ceiling);
        assert_eq!(ceiling, 1024);
    }

    // Test reset
    #[test]
    fn test_reset() {
        let mut backoff = Backoff::new();
        backoff.spin();
        backoff.spin();
        assert!(backoff.value() > BACKOFF_INITIAL);
        backoff.reset();
        assert_eq!(backoff.value(), BACKOFF_INITIAL);
    }

    // Test default trait
    #[test]
    fn test_default() {
        let backoff: Backoff = Default::default();
        assert_eq!(backoff.value(), BACKOFF_INITIAL);
    }

    // Test spin_once doesn't update value
    #[test]
    fn test_spin_once() {
        let backoff = Backoff::with_initial(10);
        backoff.spin_once();
        assert_eq!(backoff.value(), 10); // unchanged
    }
}
