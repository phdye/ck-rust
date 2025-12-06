//! Compiler compatibility and bit manipulation primitives.
//!
//! This module provides foundational bit manipulation operations that are used
//! throughout the CK library. In Rust, these leverage the language's built-in
//! intrinsics for optimal performance.
//!
//! # Operations
//!
//! - [`ffs`] / [`ffs_u64`]: Find first set bit (1-indexed)
//! - [`ctz`] / [`ctz_u64`]: Count trailing zeros
//! - [`popcount`] / [`popcount_u64`]: Population count (number of set bits)
//!
//! # Thread Safety
//!
//! All functions in this module are pure functions with no side effects,
//! making them inherently thread-safe and wait-free.

/// Find first set bit in a 32-bit value.
///
/// Returns the 1-indexed position of the least significant set bit,
/// or 0 if no bits are set.
///
/// # Examples
///
/// ```
/// use concurrencykit::cc::ffs;
///
/// assert_eq!(ffs(0), 0);
/// assert_eq!(ffs(1), 1);       // bit 0 set -> position 1
/// assert_eq!(ffs(2), 2);       // bit 1 set -> position 2
/// assert_eq!(ffs(0b11110000), 5); // bit 4 set -> position 5
/// ```
#[inline]
pub const fn ffs(v: u32) -> u32 {
    if v == 0 {
        0
    } else {
        v.trailing_zeros() + 1
    }
}

/// Find first set bit in a 64-bit value.
///
/// Returns the 1-indexed position of the least significant set bit,
/// or 0 if no bits are set. This is equivalent to the C `ffsl`/`ffsll` functions.
///
/// # Examples
///
/// ```
/// use concurrencykit::cc::ffs_u64;
///
/// assert_eq!(ffs_u64(0), 0);
/// assert_eq!(ffs_u64(1), 1);
/// assert_eq!(ffs_u64(0x8000000000000000), 64);
/// ```
#[inline]
pub const fn ffs_u64(v: u64) -> u32 {
    if v == 0 {
        0
    } else {
        v.trailing_zeros() + 1
    }
}

/// Count trailing zeros in a 32-bit value.
///
/// Returns the number of trailing zero bits in the value.
/// For zero input, returns 0 (CK-specific behavior for safety).
///
/// # Examples
///
/// ```
/// use concurrencykit::cc::ctz;
///
/// assert_eq!(ctz(0), 0);       // CK-specific: returns 0 for zero input
/// assert_eq!(ctz(1), 0);       // no trailing zeros
/// assert_eq!(ctz(0b11110000), 4); // 4 trailing zeros
/// assert_eq!(ctz(0x80000000), 31);
/// ```
#[inline]
pub const fn ctz(v: u32) -> u32 {
    if v == 0 {
        0
    } else {
        v.trailing_zeros()
    }
}

/// Count trailing zeros in a 64-bit value.
///
/// Returns the number of trailing zero bits in the value.
/// For zero input, returns 0 (CK-specific behavior for safety).
#[inline]
pub const fn ctz_u64(v: u64) -> u32 {
    if v == 0 {
        0
    } else {
        v.trailing_zeros()
    }
}

/// Count the number of set bits (population count) in a 32-bit value.
///
/// Returns the Hamming weight of the value.
///
/// # Examples
///
/// ```
/// use concurrencykit::cc::popcount;
///
/// assert_eq!(popcount(0), 0);
/// assert_eq!(popcount(1), 1);
/// assert_eq!(popcount(0b10101010), 4);
/// assert_eq!(popcount(0xFFFFFFFF), 32);
/// ```
#[inline]
pub const fn popcount(v: u32) -> u32 {
    v.count_ones()
}

/// Count the number of set bits (population count) in a 64-bit value.
///
/// Returns the Hamming weight of the value.
#[inline]
pub const fn popcount_u64(v: u64) -> u32 {
    v.count_ones()
}

/// Branch prediction hint: likely to be true.
///
/// In Rust, this is a no-op as the compiler handles branch prediction.
/// Provided for API compatibility with C CK.
#[inline(always)]
pub const fn likely(v: bool) -> bool {
    v
}

/// Branch prediction hint: unlikely to be true.
///
/// In Rust, this is a no-op as the compiler handles branch prediction.
/// Provided for API compatibility with C CK.
#[inline(always)]
pub const fn unlikely(v: bool) -> bool {
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST-001: ck_cc_ffs_zero
    #[test]
    fn test_ffs_zero() {
        assert_eq!(ffs(0), 0);
    }

    // TEST-002: ck_cc_ffs_bit_zero
    #[test]
    fn test_ffs_bit_zero() {
        assert_eq!(ffs(1), 1);
    }

    // TEST-003: ck_cc_ffs_bit_one
    #[test]
    fn test_ffs_bit_one() {
        assert_eq!(ffs(2), 2);
    }

    // TEST-004: ck_cc_ffs_high_bit
    #[test]
    fn test_ffs_high_bit() {
        assert_eq!(ffs(0x80000000), 32);
    }

    // TEST-005: ck_cc_ffs_multiple_bits
    #[test]
    fn test_ffs_multiple_bits() {
        // 0b11110000 = 240, first set bit is at position 4 (0-indexed), so 1-indexed = 5
        assert_eq!(ffs(0b11110000), 5);
    }

    // TEST-006: ck_cc_ffsl_zero (using ffs_u64)
    #[test]
    fn test_ffs_u64_zero() {
        assert_eq!(ffs_u64(0), 0);
    }

    // TEST-007: ck_cc_ffsl_high_bit_64
    #[test]
    fn test_ffs_u64_high_bit() {
        assert_eq!(ffs_u64(0x8000000000000000), 64);
    }

    // TEST-008: ck_cc_ffsll_zero (same as TEST-006 in Rust)
    #[test]
    fn test_ffsll_zero() {
        assert_eq!(ffs_u64(0), 0);
    }

    // TEST-009: ck_cc_ffsll_high_bit
    #[test]
    fn test_ffsll_high_bit() {
        assert_eq!(ffs_u64(0x8000000000000000), 64);
    }

    // TEST-010: ck_cc_ctz_zero
    #[test]
    fn test_ctz_zero() {
        // CK-specific behavior: returns 0 for zero input
        assert_eq!(ctz(0), 0);
    }

    // TEST-011: ck_cc_ctz_bit_zero
    #[test]
    fn test_ctz_bit_zero() {
        assert_eq!(ctz(1), 0);
    }

    // TEST-012: ck_cc_ctz_high_bit
    #[test]
    fn test_ctz_high_bit() {
        assert_eq!(ctz(0x80000000), 31);
    }

    // TEST-013: ck_cc_ctz_trailing_zeros
    #[test]
    fn test_ctz_trailing_zeros() {
        assert_eq!(ctz(0b11110000), 4);
    }

    // TEST-014: ck_cc_popcount_zero
    #[test]
    fn test_popcount_zero() {
        assert_eq!(popcount(0), 0);
    }

    // TEST-015: ck_cc_popcount_one
    #[test]
    fn test_popcount_one() {
        assert_eq!(popcount(1), 1);
    }

    // TEST-016: ck_cc_popcount_all_bits
    #[test]
    fn test_popcount_all_bits() {
        assert_eq!(popcount(0xFFFFFFFF), 32);
    }

    // TEST-017: ck_cc_popcount_pattern
    #[test]
    fn test_popcount_pattern() {
        assert_eq!(popcount(0b10101010), 4);
    }

    // TEST-018: ck_cc_likely_identity
    #[test]
    fn test_likely_identity() {
        assert!(likely(true));
        assert!(!likely(false));
    }

    // TEST-019: ck_cc_unlikely_identity
    #[test]
    fn test_unlikely_identity() {
        assert!(unlikely(true));
        assert!(!unlikely(false));
    }

    // Additional edge case tests
    #[test]
    fn test_ffs_all_bits_set() {
        assert_eq!(ffs(0xFFFFFFFF), 1);
    }

    #[test]
    fn test_ctz_u64_zero() {
        assert_eq!(ctz_u64(0), 0);
    }

    #[test]
    fn test_ctz_u64_high_bit() {
        assert_eq!(ctz_u64(0x8000000000000000), 63);
    }

    #[test]
    fn test_popcount_u64_all_bits() {
        assert_eq!(popcount_u64(0xFFFFFFFFFFFFFFFF), 64);
    }
}
