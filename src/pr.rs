//! Atomic primitives and memory barriers.
//!
//! This module provides portable atomic operations and memory barriers for
//! concurrent programming. It wraps Rust's `core::sync::atomic` types to
//! provide a CK-compatible API.
//!
//! # Memory Ordering
//!
//! The module provides several fence operations with different ordering guarantees:
//! - [`barrier`]: Compiler barrier only (no hardware fence)
//! - [`fence_acquire`]: Acquire fence
//! - [`fence_release`]: Release fence
//! - [`fence_acqrel`]: Acquire-release fence
//! - [`fence_memory`]: Full memory fence (sequentially consistent)
//!
//! # Thread Safety
//!
//! All operations in this module are atomic and thread-safe by definition.
//! CAS-based operations are lock-free, while load/store operations are wait-free.
//!
//! # Progress Guarantees
//!
//! - **Wait-free**: load, store, fence operations
//! - **Lock-free**: CAS, FAA, FAS, and other read-modify-write operations

use core::sync::atomic::{
    AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize, AtomicPtr,
    Ordering, fence, compiler_fence,
};

// ============================================================================
// Memory Fences
// ============================================================================

/// Compiler barrier - prevents compiler reordering, no hardware fence.
///
/// This is useful when you need to prevent the compiler from reordering
/// memory operations, but don't need a hardware fence.
#[inline(always)]
pub fn barrier() {
    compiler_fence(Ordering::SeqCst);
}

/// Acquire fence - prevents reordering of reads before this fence.
///
/// Operations before the fence cannot be reordered after it.
#[inline(always)]
pub fn fence_acquire() {
    fence(Ordering::Acquire);
}

/// Release fence - prevents reordering of writes after this fence.
///
/// Operations after the fence cannot be reordered before it.
#[inline(always)]
pub fn fence_release() {
    fence(Ordering::Release);
}

/// Acquire-release fence - combines acquire and release semantics.
#[inline(always)]
pub fn fence_acqrel() {
    fence(Ordering::AcqRel);
}

/// Full memory fence - sequentially consistent ordering.
///
/// This is the strongest memory ordering guarantee.
#[inline(always)]
pub fn fence_memory() {
    fence(Ordering::SeqCst);
}

/// Load fence - prevents load-load reordering.
#[inline(always)]
pub fn fence_load() {
    fence(Ordering::Acquire);
}

/// Store fence - prevents store-store reordering.
#[inline(always)]
pub fn fence_store() {
    fence(Ordering::Release);
}

/// Store-load fence - prevents store-load reordering.
///
/// This is the most expensive fence on x86 (requires mfence).
#[inline(always)]
pub fn fence_store_load() {
    fence(Ordering::SeqCst);
}

/// Load-store fence - prevents load-store reordering.
#[inline(always)]
pub fn fence_load_store() {
    fence(Ordering::AcqRel);
}

/// Fence for atomic operations.
#[inline(always)]
pub fn fence_atomic() {
    fence(Ordering::SeqCst);
}

/// Fence for lock acquisition.
#[inline(always)]
pub fn fence_lock() {
    fence(Ordering::Acquire);
}

/// Fence for lock release.
#[inline(always)]
pub fn fence_unlock() {
    fence(Ordering::Release);
}

/// CPU stall/pause hint for spin loops.
///
/// This reduces power consumption and improves performance in spin-wait loops.
#[inline(always)]
pub fn stall() {
    core::hint::spin_loop();
}

// ============================================================================
// Atomic Operations Macro
// ============================================================================

/// Macro to generate atomic operations for a specific type.
macro_rules! atomic_ops {
    ($mod_name:ident, $atomic_ty:ty, $val_ty:ty) => {
        pub mod $mod_name {
            use super::*;

            /// Atomic load with acquire semantics.
            #[inline]
            pub fn load(target: &$atomic_ty) -> $val_ty {
                target.load(Ordering::Acquire)
            }

            /// Atomic load with relaxed semantics.
            #[inline]
            pub fn load_relaxed(target: &$atomic_ty) -> $val_ty {
                target.load(Ordering::Relaxed)
            }

            /// Atomic store with release semantics.
            #[inline]
            pub fn store(target: &$atomic_ty, value: $val_ty) {
                target.store(value, Ordering::Release);
            }

            /// Atomic store with relaxed semantics.
            #[inline]
            pub fn store_relaxed(target: &$atomic_ty, value: $val_ty) {
                target.store(value, Ordering::Relaxed);
            }

            /// Compare-and-swap. Returns true if the swap was performed.
            ///
            /// Atomically: if `*target == compare`, then `*target = set` and return true.
            /// Otherwise return false.
            #[inline]
            pub fn cas(target: &$atomic_ty, compare: $val_ty, set: $val_ty) -> bool {
                target.compare_exchange(compare, set, Ordering::SeqCst, Ordering::SeqCst).is_ok()
            }

            /// Weak compare-and-swap. May fail spuriously.
            #[inline]
            pub fn cas_weak(target: &$atomic_ty, compare: $val_ty, set: $val_ty) -> bool {
                target.compare_exchange_weak(compare, set, Ordering::SeqCst, Ordering::SeqCst).is_ok()
            }

            /// Compare-and-swap returning the old value.
            ///
            /// Returns `(success, old_value)` where `success` indicates if the swap occurred.
            #[inline]
            pub fn cas_value(target: &$atomic_ty, compare: $val_ty, set: $val_ty) -> (bool, $val_ty) {
                match target.compare_exchange(compare, set, Ordering::SeqCst, Ordering::SeqCst) {
                    Ok(old) => (true, old),
                    Err(old) => (false, old),
                }
            }

            /// Fetch-and-add. Returns previous value.
            #[inline]
            pub fn faa(target: &$atomic_ty, delta: $val_ty) -> $val_ty {
                target.fetch_add(delta, Ordering::SeqCst)
            }

            /// Fetch-and-subtract. Returns previous value.
            #[inline]
            pub fn fas_sub(target: &$atomic_ty, delta: $val_ty) -> $val_ty {
                target.fetch_sub(delta, Ordering::SeqCst)
            }

            /// Fetch-and-store (exchange). Returns previous value.
            #[inline]
            pub fn fas(target: &$atomic_ty, value: $val_ty) -> $val_ty {
                target.swap(value, Ordering::SeqCst)
            }

            /// Atomic add (no return value).
            #[inline]
            pub fn add(target: &$atomic_ty, delta: $val_ty) {
                target.fetch_add(delta, Ordering::SeqCst);
            }

            /// Atomic subtract (no return value).
            #[inline]
            pub fn sub(target: &$atomic_ty, delta: $val_ty) {
                target.fetch_sub(delta, Ordering::SeqCst);
            }

            /// Atomic increment.
            #[inline]
            pub fn inc(target: &$atomic_ty) {
                target.fetch_add(1, Ordering::SeqCst);
            }

            /// Atomic decrement.
            #[inline]
            pub fn dec(target: &$atomic_ty) {
                target.fetch_sub(1, Ordering::SeqCst);
            }

            /// Atomic increment, returns true if result is zero (wrapped).
            #[inline]
            pub fn inc_is_zero(target: &$atomic_ty) -> bool {
                target.fetch_add(1, Ordering::SeqCst) == <$val_ty>::MAX
            }

            /// Atomic decrement, returns true if result is zero.
            #[inline]
            pub fn dec_is_zero(target: &$atomic_ty) -> bool {
                target.fetch_sub(1, Ordering::SeqCst) == 1
            }

            /// Atomic bitwise AND.
            #[inline]
            pub fn and(target: &$atomic_ty, mask: $val_ty) {
                target.fetch_and(mask, Ordering::SeqCst);
            }

            /// Atomic bitwise OR.
            #[inline]
            pub fn or(target: &$atomic_ty, mask: $val_ty) {
                target.fetch_or(mask, Ordering::SeqCst);
            }

            /// Atomic bitwise XOR.
            #[inline]
            pub fn xor(target: &$atomic_ty, mask: $val_ty) {
                target.fetch_xor(mask, Ordering::SeqCst);
            }

            /// Fetch-and-AND. Returns previous value.
            #[inline]
            pub fn faa_and(target: &$atomic_ty, mask: $val_ty) -> $val_ty {
                target.fetch_and(mask, Ordering::SeqCst)
            }

            /// Fetch-and-OR. Returns previous value.
            #[inline]
            pub fn faa_or(target: &$atomic_ty, mask: $val_ty) -> $val_ty {
                target.fetch_or(mask, Ordering::SeqCst)
            }

            /// Fetch-and-XOR. Returns previous value.
            #[inline]
            pub fn faa_xor(target: &$atomic_ty, mask: $val_ty) -> $val_ty {
                target.fetch_xor(mask, Ordering::SeqCst)
            }

            /// Atomic bitwise NOT.
            #[inline]
            pub fn not(target: &$atomic_ty) {
                let mut old = target.load(Ordering::Relaxed);
                loop {
                    match target.compare_exchange_weak(old, !old, Ordering::SeqCst, Ordering::Relaxed) {
                        Ok(_) => break,
                        Err(x) => old = x,
                    }
                }
            }

            /// Atomic negation.
            #[inline]
            pub fn neg(target: &$atomic_ty) {
                let mut old = target.load(Ordering::Relaxed);
                loop {
                    let new = (old as i64).wrapping_neg() as $val_ty;
                    match target.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
                        Ok(_) => break,
                        Err(x) => old = x,
                    }
                }
            }

            /// Bit test and set. Returns previous state of bit.
            #[inline]
            pub fn bts(target: &$atomic_ty, offset: u32) -> bool {
                let mask = 1 << offset;
                (target.fetch_or(mask, Ordering::SeqCst) & mask) != 0
            }

            /// Bit test and reset. Returns previous state of bit.
            #[inline]
            pub fn btr(target: &$atomic_ty, offset: u32) -> bool {
                let mask = 1 << offset;
                (target.fetch_and(!mask, Ordering::SeqCst) & mask) != 0
            }

            /// Bit test and complement. Returns previous state of bit.
            #[inline]
            pub fn btc(target: &$atomic_ty, offset: u32) -> bool {
                let mask = 1 << offset;
                (target.fetch_xor(mask, Ordering::SeqCst) & mask) != 0
            }
        }
    };
}

// Generate atomic operations for each type
atomic_ops!(u8_ops, AtomicU8, u8);
atomic_ops!(u16_ops, AtomicU16, u16);
atomic_ops!(u32_ops, AtomicU32, u32);
atomic_ops!(u64_ops, AtomicU64, u64);
atomic_ops!(usize_ops, AtomicUsize, usize);

/// Pointer atomic operations.
pub mod ptr_ops {
    use super::*;

    /// Atomic load of pointer with acquire semantics.
    #[inline]
    pub fn load<T>(target: &AtomicPtr<T>) -> *mut T {
        target.load(Ordering::Acquire)
    }

    /// Atomic load of pointer with relaxed semantics.
    #[inline]
    pub fn load_relaxed<T>(target: &AtomicPtr<T>) -> *mut T {
        target.load(Ordering::Relaxed)
    }

    /// Atomic store of pointer with release semantics.
    #[inline]
    pub fn store<T>(target: &AtomicPtr<T>, value: *mut T) {
        target.store(value, Ordering::Release);
    }

    /// Atomic store of pointer with relaxed semantics.
    #[inline]
    pub fn store_relaxed<T>(target: &AtomicPtr<T>, value: *mut T) {
        target.store(value, Ordering::Relaxed);
    }

    /// Compare-and-swap pointer. Returns true if swap occurred.
    #[inline]
    pub fn cas<T>(target: &AtomicPtr<T>, compare: *mut T, set: *mut T) -> bool {
        target.compare_exchange(compare, set, Ordering::SeqCst, Ordering::SeqCst).is_ok()
    }

    /// Weak compare-and-swap pointer.
    #[inline]
    pub fn cas_weak<T>(target: &AtomicPtr<T>, compare: *mut T, set: *mut T) -> bool {
        target.compare_exchange_weak(compare, set, Ordering::SeqCst, Ordering::SeqCst).is_ok()
    }

    /// Compare-and-swap pointer returning old value.
    #[inline]
    pub fn cas_value<T>(target: &AtomicPtr<T>, compare: *mut T, set: *mut T) -> (bool, *mut T) {
        match target.compare_exchange(compare, set, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(old) => (true, old),
            Err(old) => (false, old),
        }
    }

    /// Fetch-and-store (exchange) pointer. Returns previous value.
    #[inline]
    pub fn fas<T>(target: &AtomicPtr<T>, value: *mut T) -> *mut T {
        target.swap(value, Ordering::SeqCst)
    }
}

// Re-export atomic types for convenience (already imported at top of file)

#[cfg(test)]
mod tests {
    use super::*;

    // TEST-001: load_store_single_thread
    #[test]
    fn test_load_store_single_thread() {
        let var = AtomicU64::new(0);
        u64_ops::store(&var, 0x123456789ABCDEF0);
        let loaded = u64_ops::load(&var);
        assert_eq!(loaded, 0x123456789ABCDEF0);
    }

    // TEST-002: cas_success
    #[test]
    fn test_cas_success() {
        let var = AtomicU64::new(42);
        let result = u64_ops::cas(&var, 42, 100);
        assert!(result);
        assert_eq!(u64_ops::load(&var), 100);
    }

    // TEST-003: cas_failure
    #[test]
    fn test_cas_failure() {
        let var = AtomicU64::new(42);
        let result = u64_ops::cas(&var, 99, 100);
        assert!(!result);
        assert_eq!(u64_ops::load(&var), 42);
    }

    // TEST-004: cas_value_returns_old
    #[test]
    fn test_cas_value_returns_old() {
        let var = AtomicU64::new(42);
        let (success, old) = u64_ops::cas_value(&var, 99, 100);
        assert!(!success);
        assert_eq!(old, 42);
        assert_eq!(u64_ops::load(&var), 42);
    }

    #[test]
    fn test_cas_value_success() {
        let var = AtomicU64::new(42);
        let (success, old) = u64_ops::cas_value(&var, 42, 100);
        assert!(success);
        assert_eq!(old, 42);
        assert_eq!(u64_ops::load(&var), 100);
    }

    // TEST-005: faa_basic
    #[test]
    fn test_faa_basic() {
        let var = AtomicU64::new(100);
        let result = u64_ops::faa(&var, 50);
        assert_eq!(result, 100);
        assert_eq!(u64_ops::load(&var), 150);
    }

    // TEST-006: faa_negative (subtraction via wrapping)
    #[test]
    fn test_faa_negative() {
        let var = AtomicU64::new(100);
        let result = u64_ops::faa(&var, (-30i64) as u64);
        assert_eq!(result, 100);
        assert_eq!(u64_ops::load(&var), 70);
    }

    // TEST-007: fas_basic
    #[test]
    fn test_fas_basic() {
        let var = AtomicU64::new(100);
        let result = u64_ops::fas(&var, 999);
        assert_eq!(result, 100);
        assert_eq!(u64_ops::load(&var), 999);
    }

    // TEST-008: inc_dec_basic
    #[test]
    fn test_inc_dec_basic() {
        let var = AtomicU32::new(100);
        u32_ops::inc(&var);
        u32_ops::inc(&var);
        u32_ops::dec(&var);
        assert_eq!(u32_ops::load(&var), 101);
    }

    // TEST-009: binary_ops
    #[test]
    fn test_binary_ops() {
        let var = AtomicU32::new(0xFF00FF00);
        u32_ops::and(&var, 0xF0F0F0F0);
        assert_eq!(u32_ops::load(&var), 0xF000F000);
        u32_ops::or(&var, 0x0000000F);
        assert_eq!(u32_ops::load(&var), 0xF000F00F);
        u32_ops::xor(&var, 0x000000F0);
        assert_eq!(u32_ops::load(&var), 0xF000F0FF);
    }

    // TEST-010: bts_btr_btc
    #[test]
    fn test_bts_btr_btc() {
        let var = AtomicU32::new(0);

        // Set bit 5
        let old1 = u32_ops::bts(&var, 5);
        assert!(!old1); // bit was 0
        assert_eq!(u32_ops::load(&var), 32); // 1 << 5 = 32

        // Test already set bit
        let old2 = u32_ops::bts(&var, 5);
        assert!(old2); // bit was 1

        // Reset bit 5
        let old3 = u32_ops::btr(&var, 5);
        assert!(old3); // bit was 1
        assert_eq!(u32_ops::load(&var), 0);

        // Complement bit 3
        let old4 = u32_ops::btc(&var, 3);
        assert!(!old4); // bit was 0
        assert_eq!(u32_ops::load(&var), 8); // 1 << 3 = 8
    }

    // TEST-011: overflow_behavior
    #[test]
    fn test_overflow_behavior() {
        let var = AtomicU32::new(u32::MAX);
        u32_ops::inc(&var);
        assert_eq!(u32_ops::load(&var), 0); // wrapped
    }

    // TEST-012: dec_is_zero
    #[test]
    fn test_dec_is_zero() {
        let var = AtomicU32::new(2);
        let result1 = u32_ops::dec_is_zero(&var); // 2 -> 1
        assert!(!result1);
        let result2 = u32_ops::dec_is_zero(&var); // 1 -> 0
        assert!(result2);
        assert_eq!(u32_ops::load(&var), 0);
    }

    // TEST-016: pointer_operations
    #[test]
    fn test_pointer_operations() {
        let mut obj1: u64 = 1;
        let mut obj2: u64 = 2;
        let ptr = AtomicPtr::new(core::ptr::null_mut());

        ptr_ops::store(&ptr, &mut obj1 as *mut u64);
        let old = ptr_ops::fas(&ptr, &mut obj2 as *mut u64);

        assert_eq!(old, &mut obj1 as *mut u64);
        assert_eq!(ptr_ops::load(&ptr), &mut obj2 as *mut u64);
    }

    // TEST-017: all_sizes
    #[test]
    fn test_all_sizes() {
        let v8 = AtomicU8::new(0);
        u8_ops::store(&v8, 0xAB);
        assert_eq!(u8_ops::load(&v8), 0xAB);

        let v16 = AtomicU16::new(0);
        u16_ops::store(&v16, 0xABCD);
        assert_eq!(u16_ops::load(&v16), 0xABCD);

        let v32 = AtomicU32::new(0);
        u32_ops::store(&v32, 0xABCDEF01);
        assert_eq!(u32_ops::load(&v32), 0xABCDEF01);

        let v64 = AtomicU64::new(0);
        u64_ops::store(&v64, 0xABCDEF0123456789);
        assert_eq!(u64_ops::load(&v64), 0xABCDEF0123456789);
    }

    // Additional tests
    #[test]
    fn test_stall() {
        // Just verify it doesn't panic
        stall();
    }

    #[test]
    fn test_fences() {
        // Just verify they don't panic
        barrier();
        fence_acquire();
        fence_release();
        fence_acqrel();
        fence_memory();
        fence_load();
        fence_store();
        fence_store_load();
        fence_load_store();
        fence_atomic();
        fence_lock();
        fence_unlock();
    }

    #[test]
    fn test_not() {
        let var = AtomicU32::new(0xFF00FF00);
        u32_ops::not(&var);
        assert_eq!(u32_ops::load(&var), 0x00FF00FF);
    }

    #[test]
    fn test_inc_is_zero() {
        let var = AtomicU32::new(u32::MAX);
        let result = u32_ops::inc_is_zero(&var);
        assert!(result); // MAX + 1 wraps to 0
        assert_eq!(u32_ops::load(&var), 0);
    }

    #[test]
    fn test_cas_weak() {
        let var = AtomicU64::new(42);
        // Weak CAS may fail spuriously, so we try in a loop
        let mut attempts = 0;
        while !u64_ops::cas_weak(&var, 42, 100) {
            attempts += 1;
            if attempts > 100 {
                panic!("Weak CAS failed too many times");
            }
        }
        assert_eq!(u64_ops::load(&var), 100);
    }

    #[test]
    fn test_ptr_cas() {
        let mut obj1: u64 = 1;
        let mut obj2: u64 = 2;
        let ptr = AtomicPtr::new(&mut obj1 as *mut u64);

        // Successful CAS
        let success = ptr_ops::cas(&ptr, &mut obj1 as *mut u64, &mut obj2 as *mut u64);
        assert!(success);
        assert_eq!(ptr_ops::load(&ptr), &mut obj2 as *mut u64);

        // Failed CAS
        let success = ptr_ops::cas(&ptr, &mut obj1 as *mut u64, core::ptr::null_mut());
        assert!(!success);
        assert_eq!(ptr_ops::load(&ptr), &mut obj2 as *mut u64);
    }
}
