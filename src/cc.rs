//! Compiler compatibility utilities.
//!
//! This module provides bit manipulation operations and branch prediction hints
//! that abstract over platform-specific optimizations while providing portable
//! fallback implementations.
//!
//! # Bit Operations
//!
//! - [`ffs`] - Find first set bit (1-indexed)
//! - [`ffsl`] - Find first set bit in usize
//! - [`ffsll`] - Find first set bit in u64
//! - [`ctz`] - Count trailing zeros
//! - [`popcount`] - Population count (number of set bits)
//!
//! # Branch Hints
//!
//! - `likely` - Hint that condition is likely true
//! - `unlikely` - Hint that condition is likely false
//!
//! # Example
//!
//! ```
//! use concurrencykit::cc::{ffs, ctz, popcount};
//!
//! assert_eq!(ffs(0), 0);      // No bits set
//! assert_eq!(ffs(1), 1);      // Bit 0 set, 1-indexed = 1
//! assert_eq!(ffs(0b1000), 4); // Bit 3 set, 1-indexed = 4
//!
//! assert_eq!(ctz(0b1000), 3); // 3 trailing zeros
//! assert_eq!(popcount(0b1010), 2); // 2 bits set
//! ```

/// Find first set bit in a 32-bit unsigned integer.
///
/// Returns the 1-indexed position of the least significant set bit,
/// or 0 if no bits are set.
///
/// # Examples
///
/// ```
/// use concurrencykit::cc::ffs;
///
/// assert_eq!(ffs(0), 0);           // No bits set
/// assert_eq!(ffs(1), 1);           // Bit 0 is set
/// assert_eq!(ffs(2), 2);           // Bit 1 is set
/// assert_eq!(ffs(0x80000000), 32); // Bit 31 is set
/// assert_eq!(ffs(0b11110000), 5);  // Bit 4 is first set bit
/// ```
#[inline]
pub const fn ffs(v: u32) -> i32 {
    if v == 0 {
        0
    } else {
        (v.trailing_zeros() + 1) as i32
    }
}

/// Find first set bit in a pointer-sized unsigned integer.
///
/// Returns the 1-indexed position of the least significant set bit,
/// or 0 if no bits are set.
///
/// # Examples
///
/// ```
/// use concurrencykit::cc::ffsl;
///
/// assert_eq!(ffsl(0), 0);
/// assert_eq!(ffsl(1), 1);
/// ```
#[inline]
pub const fn ffsl(v: usize) -> i32 {
    if v == 0 {
        0
    } else {
        (v.trailing_zeros() + 1) as i32
    }
}

/// Find first set bit in a 64-bit unsigned integer.
///
/// Returns the 1-indexed position of the least significant set bit,
/// or 0 if no bits are set.
///
/// # Examples
///
/// ```
/// use concurrencykit::cc::ffsll;
///
/// assert_eq!(ffsll(0), 0);
/// assert_eq!(ffsll(1), 1);
/// assert_eq!(ffsll(0x8000000000000000), 64); // Bit 63 is set
/// ```
#[inline]
pub const fn ffsll(v: u64) -> i32 {
    if v == 0 {
        0
    } else {
        (v.trailing_zeros() + 1) as i32
    }
}

/// Count trailing zeros in a 32-bit unsigned integer.
///
/// Returns the number of zero bits following the least significant set bit.
/// Returns 0 if the input is 0 (CK-specific behavior for safety).
///
/// # Examples
///
/// ```
/// use concurrencykit::cc::ctz;
///
/// assert_eq!(ctz(0), 0);          // CK-specific: returns 0 for safety
/// assert_eq!(ctz(1), 0);          // No trailing zeros
/// assert_eq!(ctz(0b1000), 3);     // 3 trailing zeros
/// assert_eq!(ctz(0x80000000), 31); // 31 trailing zeros
/// ```
#[inline]
pub const fn ctz(x: u32) -> i32 {
    if x == 0 {
        0
    } else {
        x.trailing_zeros() as i32
    }
}

/// Count the number of set bits in a 32-bit unsigned integer.
///
/// Also known as population count or Hamming weight.
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
pub const fn popcount(x: u32) -> i32 {
    x.count_ones() as i32
}

/// Hint that a condition is likely to be true.
///
/// This is a hint to the compiler for branch prediction optimization.
/// The actual boolean value is preserved.
///
/// # Examples
///
/// ```
/// use concurrencykit::cc::likely;
///
/// let condition = true;
/// if likely(condition) {
///     // This branch is expected to be taken most of the time
/// }
/// ```
#[inline]
#[must_use]
pub fn likely(b: bool) -> bool {
    // Branch prediction hint - the cold function hints to the compiler
    // that the false branch is unlikely
    #[cold]
    #[inline(never)]
    fn cold_false() {}

    if !b {
        cold_false();
    }
    b
}

/// Hint that a condition is unlikely to be true.
///
/// This is a hint to the compiler for branch prediction optimization.
/// The actual boolean value is preserved.
///
/// # Examples
///
/// ```
/// use concurrencykit::cc::unlikely;
///
/// let error_occurred = false;
/// if unlikely(error_occurred) {
///     // This branch is expected to be taken rarely
/// }
/// ```
#[inline]
#[must_use]
pub fn unlikely(b: bool) -> bool {
    // Branch prediction hint - the cold function hints to the compiler
    // that the true branch is unlikely
    #[cold]
    #[inline(never)]
    fn cold_true() {}

    if b {
        cold_true();
    }
    b
}

/// Cache line size in bytes.
///
/// This is the typical cache line size on modern x86-64 and ARM processors.
/// Used for padding structures to avoid false sharing.
pub const CACHELINE: usize = 64;

/// Compute the offset of a field within a struct.
///
/// This is equivalent to the C `offsetof` macro.
///
/// # Safety
///
/// This macro is safe to use and produces a compile-time constant.
#[macro_export]
macro_rules! offset_of {
    ($type:ty, $field:ident) => {{
        // Use MaybeUninit to avoid creating an actual instance
        let uninit = core::mem::MaybeUninit::<$type>::uninit();
        let base_ptr = uninit.as_ptr();
        // SAFETY: We're only computing the offset, not dereferencing
        let field_ptr = unsafe { core::ptr::addr_of!((*base_ptr).$field) };
        (field_ptr as usize) - (base_ptr as usize)
    }};
}

/// Get a pointer to the containing structure from a pointer to a field.
///
/// This is equivalent to the Linux kernel's `container_of` macro.
///
/// # Safety
///
/// The caller must ensure that:
/// - `ptr` points to a valid field within a valid instance of `$type`
/// - The resulting pointer is valid for the lifetime of the containing structure
#[macro_export]
macro_rules! container_of {
    ($ptr:expr, $type:ty, $field:ident) => {{
        let ptr = $ptr as *const _ as *const u8;
        let offset = $crate::offset_of!($type, $field);
        // SAFETY: Caller guarantees ptr points to a valid field
        (ptr.sub(offset)) as *const $type
    }};
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
        assert_eq!(ffs(0b11110000), 5); // Bit 4 is first set, 1-indexed = 5
    }

    // TEST-006: ck_cc_ffsl_zero
    #[test]
    fn test_ffsl_zero() {
        assert_eq!(ffsl(0), 0);
    }

    // TEST-007: ck_cc_ffsl_high_bit_64
    #[test]
    #[cfg(target_pointer_width = "64")]
    fn test_ffsl_high_bit_64() {
        assert_eq!(ffsl(0x8000000000000000usize), 64);
    }

    // TEST-008: ck_cc_ffsll_zero
    #[test]
    fn test_ffsll_zero() {
        assert_eq!(ffsll(0), 0);
    }

    // TEST-009: ck_cc_ffsll_high_bit
    #[test]
    fn test_ffsll_high_bit() {
        assert_eq!(ffsll(0x8000000000000000u64), 64);
    }

    // TEST-010: ck_cc_ctz_zero
    #[test]
    fn test_ctz_zero() {
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

    // TEST-020: ck_cc_container_offset
    #[test]
    fn test_container_offset() {
        struct Test {
            a: i32,
            member: i32,
            c: i32,
        }

        let t = Test { a: 1, member: 2, c: 3 };
        let member_ptr = &t.member as *const i32;

        // Use container_of to recover pointer to t
        let recovered = unsafe { &*container_of!(member_ptr, Test, member) };

        assert_eq!(recovered.a, 1);
        assert_eq!(recovered.member, 2);
        assert_eq!(recovered.c, 3);
    }

    // Additional comprehensive tests
    #[test]
    fn test_ffs_all_single_bits() {
        for i in 0..32 {
            assert_eq!(ffs(1u32 << i), (i + 1) as i32);
        }
    }

    #[test]
    fn test_ctz_all_single_bits() {
        for i in 0..32 {
            assert_eq!(ctz(1u32 << i), i as i32);
        }
    }

    #[test]
    fn test_popcount_powers_of_two() {
        for i in 0..32 {
            assert_eq!(popcount(1u32 << i), 1);
        }
    }

    #[test]
    fn test_offset_of_macro() {
        #[repr(C)]
        struct Foo {
            a: u8,
            b: u32,
            c: u8,
        }

        let offset_a = offset_of!(Foo, a);
        let offset_b = offset_of!(Foo, b);
        let offset_c = offset_of!(Foo, c);

        assert_eq!(offset_a, 0);
        // b is aligned to 4 bytes
        assert!(offset_b >= 1);
        assert!(offset_c > offset_b);
    }
}
