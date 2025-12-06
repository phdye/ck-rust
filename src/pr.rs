//! Atomic primitives and memory barriers.
//!
//! This module provides portable atomic operations and memory barriers for concurrent
//! programming. It abstracts architecture-specific atomic instructions behind a uniform
//! API, supporting multiple memory orderings.
//!
//! # Memory Ordering
//!
//! All atomic operations in this module use explicit memory orderings:
//!
//! - `Ordering::Relaxed` - No ordering guarantees
//! - `Ordering::Acquire` - Loads acquire; prevents reordering with subsequent operations
//! - `Ordering::Release` - Stores release; prevents reordering with prior operations
//! - `Ordering::AcqRel` - Both acquire and release semantics
//! - `Ordering::SeqCst` - Sequentially consistent; strongest ordering
//!
//! # Operations
//!
//! ## Memory Barriers
//! - `barrier` - Compiler barrier (no hardware fence)
//! - `fence_memory`, `fence_acquire`, `fence_release` - Memory fences
//!
//! ## Atomic Load/Store
//! - `load`, `store` - Generic atomic load/store
//!
//! ## Read-Modify-Write
//! - `cas` - Compare-and-swap
//! - `faa` - Fetch-and-add
//! - `fas` - Fetch-and-store (exchange)
//!
//! # Example
//!
//! ```
//! use std::sync::atomic::{AtomicUsize, Ordering};
//! use concurrencykit::pr;
//!
//! let counter = AtomicUsize::new(0);
//!
//! // Atomic increment
//! let old = counter.fetch_add(1, Ordering::SeqCst);
//! assert_eq!(old, 0);
//!
//! // Compare-and-swap
//! let result = counter.compare_exchange(1, 2, Ordering::SeqCst, Ordering::SeqCst);
//! assert!(result.is_ok());
//! ```

use core::sync::atomic::{
    fence, AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicPtr,
    AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicUsize, Ordering,
};

// =============================================================================
// Memory Barriers
// =============================================================================

/// Compiler barrier - prevents compiler reordering without hardware fence.
///
/// This is a compile-time barrier that prevents the compiler from reordering
/// memory operations across it. It does not emit any hardware instructions.
#[inline]
pub fn barrier() {
    core::sync::atomic::compiler_fence(Ordering::SeqCst);
}

/// Full memory fence - prevents all reordering.
#[inline]
pub fn fence_memory() {
    fence(Ordering::SeqCst);
}

/// Load fence - prevents load-load reordering.
#[inline]
pub fn fence_load() {
    fence(Ordering::Acquire);
}

/// Store fence - prevents store-store reordering.
#[inline]
pub fn fence_store() {
    fence(Ordering::Release);
}

/// Acquire fence - acquire semantics.
#[inline]
pub fn fence_acquire() {
    fence(Ordering::Acquire);
}

/// Release fence - release semantics.
#[inline]
pub fn fence_release() {
    fence(Ordering::Release);
}

/// Acquire-release fence.
#[inline]
pub fn fence_acqrel() {
    fence(Ordering::AcqRel);
}

/// Store-load fence - prevents store-load reordering.
///
/// This is the strongest fence on x86 (requires mfence).
#[inline]
pub fn fence_store_load() {
    fence(Ordering::SeqCst);
}

/// CPU pause/stall hint for spin loops.
///
/// Reduces power consumption and improves performance in spin loops
/// by hinting to the CPU that we're in a busy-wait loop.
#[inline]
pub fn stall() {
    core::hint::spin_loop();
}

// =============================================================================
// Atomic Types (re-exports with CK naming)
// =============================================================================

/// Atomic unsigned 8-bit integer.
pub type AtomicU8Type = AtomicU8;
/// Atomic unsigned 16-bit integer.
pub type AtomicU16Type = AtomicU16;
/// Atomic unsigned 32-bit integer.
pub type AtomicU32Type = AtomicU32;
/// Atomic unsigned 64-bit integer.
pub type AtomicU64Type = AtomicU64;
/// Atomic unsigned pointer-sized integer.
pub type AtomicUsizeType = AtomicUsize;
/// Atomic signed 8-bit integer.
pub type AtomicI8Type = AtomicI8;
/// Atomic signed 16-bit integer.
pub type AtomicI16Type = AtomicI16;
/// Atomic signed 32-bit integer.
pub type AtomicI32Type = AtomicI32;
/// Atomic signed 64-bit integer.
pub type AtomicI64Type = AtomicI64;
/// Atomic signed pointer-sized integer.
pub type AtomicIsizeType = AtomicIsize;
/// Atomic boolean.
pub type AtomicBoolType = AtomicBool;

// =============================================================================
// Generic Atomic Operations
// =============================================================================

/// Trait for types that support atomic operations.
pub trait Atomic: Sized {
    /// The underlying value type.
    type Value: Copy;

    /// Create a new atomic with the given value.
    fn new(v: Self::Value) -> Self;

    /// Load the value with the given ordering.
    fn load(&self, order: Ordering) -> Self::Value;

    /// Store the value with the given ordering.
    fn store(&self, val: Self::Value, order: Ordering);

    /// Exchange (fetch-and-store) with the given ordering.
    fn swap(&self, val: Self::Value, order: Ordering) -> Self::Value;

    /// Compare-and-swap. Returns Ok(old) on success, Err(actual) on failure.
    fn compare_exchange(
        &self,
        current: Self::Value,
        new: Self::Value,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::Value, Self::Value>;

    /// Weak compare-and-swap. May fail spuriously.
    fn compare_exchange_weak(
        &self,
        current: Self::Value,
        new: Self::Value,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::Value, Self::Value>;
}

/// Trait for atomic integers that support arithmetic operations.
pub trait AtomicInt: Atomic {
    /// Fetch-and-add: atomically add delta and return old value.
    fn fetch_add(&self, val: Self::Value, order: Ordering) -> Self::Value;

    /// Fetch-and-subtract: atomically subtract delta and return old value.
    fn fetch_sub(&self, val: Self::Value, order: Ordering) -> Self::Value;

    /// Fetch-and-and: atomically AND mask and return old value.
    fn fetch_and(&self, val: Self::Value, order: Ordering) -> Self::Value;

    /// Fetch-and-or: atomically OR mask and return old value.
    fn fetch_or(&self, val: Self::Value, order: Ordering) -> Self::Value;

    /// Fetch-and-xor: atomically XOR mask and return old value.
    fn fetch_xor(&self, val: Self::Value, order: Ordering) -> Self::Value;
}

// Implement Atomic for all standard atomic types
macro_rules! impl_atomic {
    ($atomic:ty, $value:ty) => {
        impl Atomic for $atomic {
            type Value = $value;

            #[inline]
            fn new(v: Self::Value) -> Self {
                <$atomic>::new(v)
            }

            #[inline]
            fn load(&self, order: Ordering) -> Self::Value {
                <$atomic>::load(self, order)
            }

            #[inline]
            fn store(&self, val: Self::Value, order: Ordering) {
                <$atomic>::store(self, val, order)
            }

            #[inline]
            fn swap(&self, val: Self::Value, order: Ordering) -> Self::Value {
                <$atomic>::swap(self, val, order)
            }

            #[inline]
            fn compare_exchange(
                &self,
                current: Self::Value,
                new: Self::Value,
                success: Ordering,
                failure: Ordering,
            ) -> Result<Self::Value, Self::Value> {
                <$atomic>::compare_exchange(self, current, new, success, failure)
            }

            #[inline]
            fn compare_exchange_weak(
                &self,
                current: Self::Value,
                new: Self::Value,
                success: Ordering,
                failure: Ordering,
            ) -> Result<Self::Value, Self::Value> {
                <$atomic>::compare_exchange_weak(self, current, new, success, failure)
            }
        }
    };
}

macro_rules! impl_atomic_int {
    ($atomic:ty, $value:ty) => {
        impl_atomic!($atomic, $value);

        impl AtomicInt for $atomic {
            #[inline]
            fn fetch_add(&self, val: Self::Value, order: Ordering) -> Self::Value {
                <$atomic>::fetch_add(self, val, order)
            }

            #[inline]
            fn fetch_sub(&self, val: Self::Value, order: Ordering) -> Self::Value {
                <$atomic>::fetch_sub(self, val, order)
            }

            #[inline]
            fn fetch_and(&self, val: Self::Value, order: Ordering) -> Self::Value {
                <$atomic>::fetch_and(self, val, order)
            }

            #[inline]
            fn fetch_or(&self, val: Self::Value, order: Ordering) -> Self::Value {
                <$atomic>::fetch_or(self, val, order)
            }

            #[inline]
            fn fetch_xor(&self, val: Self::Value, order: Ordering) -> Self::Value {
                <$atomic>::fetch_xor(self, val, order)
            }
        }
    };
}

impl_atomic_int!(AtomicU8, u8);
impl_atomic_int!(AtomicU16, u16);
impl_atomic_int!(AtomicU32, u32);
impl_atomic_int!(AtomicU64, u64);
impl_atomic_int!(AtomicUsize, usize);
impl_atomic_int!(AtomicI8, i8);
impl_atomic_int!(AtomicI16, i16);
impl_atomic_int!(AtomicI32, i32);
impl_atomic_int!(AtomicI64, i64);
impl_atomic_int!(AtomicIsize, isize);
impl_atomic!(AtomicBool, bool);

// Implement for AtomicPtr
impl<T> Atomic for AtomicPtr<T> {
    type Value = *mut T;

    #[inline]
    fn new(v: Self::Value) -> Self {
        AtomicPtr::new(v)
    }

    #[inline]
    fn load(&self, order: Ordering) -> Self::Value {
        AtomicPtr::load(self, order)
    }

    #[inline]
    fn store(&self, val: Self::Value, order: Ordering) {
        AtomicPtr::store(self, val, order)
    }

    #[inline]
    fn swap(&self, val: Self::Value, order: Ordering) -> Self::Value {
        AtomicPtr::swap(self, val, order)
    }

    #[inline]
    fn compare_exchange(
        &self,
        current: Self::Value,
        new: Self::Value,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::Value, Self::Value> {
        AtomicPtr::compare_exchange(self, current, new, success, failure)
    }

    #[inline]
    fn compare_exchange_weak(
        &self,
        current: Self::Value,
        new: Self::Value,
        success: Ordering,
        failure: Ordering,
    ) -> Result<Self::Value, Self::Value> {
        AtomicPtr::compare_exchange_weak(self, current, new, success, failure)
    }
}

// =============================================================================
// CK-style convenience functions
// =============================================================================

/// Atomically load a value.
#[inline]
pub fn load<A: Atomic>(target: &A, order: Ordering) -> A::Value {
    target.load(order)
}

/// Atomically store a value.
#[inline]
pub fn store<A: Atomic>(target: &A, value: A::Value, order: Ordering) {
    target.store(value, order);
}

/// Compare-and-swap. Returns true if the swap occurred.
#[inline]
pub fn cas<A: Atomic>(target: &A, compare: A::Value, set: A::Value) -> bool
where
    A::Value: PartialEq,
{
    target
        .compare_exchange(compare, set, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
}

/// Compare-and-swap returning the old value.
#[inline]
pub fn cas_value<A: Atomic>(target: &A, compare: A::Value, set: A::Value) -> (bool, A::Value)
where
    A::Value: PartialEq,
{
    match target.compare_exchange(compare, set, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(old) => (true, old),
        Err(old) => (false, old),
    }
}

/// Fetch-and-add for atomic integers.
#[inline]
pub fn faa<A: AtomicInt>(target: &A, delta: A::Value) -> A::Value {
    target.fetch_add(delta, Ordering::SeqCst)
}

/// Fetch-and-store (exchange).
#[inline]
pub fn fas<A: Atomic>(target: &A, value: A::Value) -> A::Value {
    target.swap(value, Ordering::SeqCst)
}

/// Atomic increment.
#[inline]
pub fn inc<A>(target: &A)
where
    A: AtomicInt,
    A::Value: From<u8>,
{
    target.fetch_add(A::Value::from(1u8), Ordering::SeqCst);
}

/// Atomic decrement.
#[inline]
pub fn dec<A>(target: &A)
where
    A: AtomicInt,
    A::Value: From<u8>,
{
    target.fetch_sub(A::Value::from(1u8), Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barrier() {
        barrier();
        // Just verify it doesn't crash
    }

    #[test]
    fn test_fence_memory() {
        fence_memory();
    }

    #[test]
    fn test_fence_acquire() {
        fence_acquire();
    }

    #[test]
    fn test_fence_release() {
        fence_release();
    }

    #[test]
    fn test_stall() {
        stall();
    }

    #[test]
    fn test_atomic_load_store() {
        let a = AtomicU32::new(42);
        assert_eq!(load(&a, Ordering::SeqCst), 42);

        store(&a, 100, Ordering::SeqCst);
        assert_eq!(load(&a, Ordering::SeqCst), 100);
    }

    #[test]
    fn test_cas_success() {
        let a = AtomicU32::new(42);
        assert!(cas(&a, 42, 100));
        assert_eq!(load(&a, Ordering::SeqCst), 100);
    }

    #[test]
    fn test_cas_failure() {
        let a = AtomicU32::new(42);
        assert!(!cas(&a, 99, 100));
        assert_eq!(load(&a, Ordering::SeqCst), 42);
    }

    #[test]
    fn test_cas_value() {
        let a = AtomicU32::new(42);

        let (success, old) = cas_value(&a, 42, 100);
        assert!(success);
        assert_eq!(old, 42);

        let (success, old) = cas_value(&a, 42, 200);
        assert!(!success);
        assert_eq!(old, 100);
    }

    #[test]
    fn test_faa() {
        let a = AtomicU32::new(10);
        let old = faa(&a, 5);
        assert_eq!(old, 10);
        assert_eq!(load(&a, Ordering::SeqCst), 15);
    }

    #[test]
    fn test_fas() {
        let a = AtomicU32::new(42);
        let old = fas(&a, 100);
        assert_eq!(old, 42);
        assert_eq!(load(&a, Ordering::SeqCst), 100);
    }

    #[test]
    fn test_inc_dec() {
        let a = AtomicU32::new(10);
        inc(&a);
        assert_eq!(load(&a, Ordering::SeqCst), 11);
        dec(&a);
        assert_eq!(load(&a, Ordering::SeqCst), 10);
    }

    #[test]
    fn test_atomic_ptr() {
        let mut value = 42i32;
        let ptr = AtomicPtr::new(&mut value as *mut i32);

        let loaded = load(&ptr, Ordering::SeqCst);
        assert_eq!(loaded, &mut value as *mut i32);

        let mut other = 100i32;
        store(&ptr, &mut other as *mut i32, Ordering::SeqCst);

        let loaded = load(&ptr, Ordering::SeqCst);
        assert_eq!(loaded, &mut other as *mut i32);
    }

    #[test]
    fn test_fetch_and_or_xor() {
        let a = AtomicU32::new(0b1010);

        let old = a.fetch_and(0b1100, Ordering::SeqCst);
        assert_eq!(old, 0b1010);
        assert_eq!(a.load(Ordering::SeqCst), 0b1000);

        let old = a.fetch_or(0b0011, Ordering::SeqCst);
        assert_eq!(old, 0b1000);
        assert_eq!(a.load(Ordering::SeqCst), 0b1011);

        let old = a.fetch_xor(0b1111, Ordering::SeqCst);
        assert_eq!(old, 0b1011);
        assert_eq!(a.load(Ordering::SeqCst), 0b0100);
    }

    #[test]
    fn test_all_atomic_types() {
        // Test that all atomic types work
        let _ = AtomicU8::new(0);
        let _ = AtomicU16::new(0);
        let _ = AtomicU32::new(0);
        let _ = AtomicU64::new(0);
        let _ = AtomicUsize::new(0);
        let _ = AtomicI8::new(0);
        let _ = AtomicI16::new(0);
        let _ = AtomicI32::new(0);
        let _ = AtomicI64::new(0);
        let _ = AtomicIsize::new(0);
        let _ = AtomicBool::new(false);
    }
}
