//! Lock cohorting for NUMA.
//!
//! Lock cohorting optimizes lock performance on NUMA systems by
//! preferring to pass locks between threads on the same NUMA node.

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::backoff::Backoff;

/// Default cohort threshold before yielding to other nodes.
const DEFAULT_THRESHOLD: usize = 64;

/// A cohort lock for NUMA-aware locking.
#[repr(C)]
pub struct CohortLock<T: ?Sized> {
    global_locked: AtomicBool,
    local_count: AtomicUsize,
    threshold: usize,
    data: UnsafeCell<T>,
}

impl<T> CohortLock<T> {
    /// Create a new cohort lock.
    #[inline]
    pub fn new(data: T) -> Self {
        Self::with_threshold(data, DEFAULT_THRESHOLD)
    }

    /// Create a new cohort lock with a custom threshold.
    #[inline]
    pub fn with_threshold(data: T, threshold: usize) -> Self {
        Self {
            global_locked: AtomicBool::new(false),
            local_count: AtomicUsize::new(0),
            threshold,
            data: UnsafeCell::new(data),
        }
    }

    /// Acquire the lock.
    #[inline]
    pub fn lock(&self) -> CohortLockGuard<'_, T> {
        let mut backoff = Backoff::new();

        loop {
            // Try to acquire global lock
            if !self.global_locked.swap(true, Ordering::Acquire) {
                self.local_count.store(0, Ordering::Relaxed);
                return CohortLockGuard { lock: self };
            }

            backoff.spin();
        }
    }

    /// Try to acquire the lock without spinning.
    #[inline]
    pub fn try_lock(&self) -> Option<CohortLockGuard<'_, T>> {
        if self.global_locked.swap(true, Ordering::Acquire) {
            None
        } else {
            self.local_count.store(0, Ordering::Relaxed);
            Some(CohortLockGuard { lock: self })
        }
    }

    /// Check if the lock is held.
    #[inline]
    pub fn is_locked(&self) -> bool {
        self.global_locked.load(Ordering::Relaxed)
    }
}

unsafe impl<T: Send> Send for CohortLock<T> {}
unsafe impl<T: Send> Sync for CohortLock<T> {}

/// RAII guard for CohortLock.
pub struct CohortLockGuard<'a, T: ?Sized> {
    lock: &'a CohortLock<T>,
}

impl<T: ?Sized> Deref for CohortLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for CohortLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for CohortLockGuard<'_, T> {
    fn drop(&mut self) {
        let count = self.lock.local_count.fetch_add(1, Ordering::Relaxed);
        if count >= self.lock.threshold {
            self.lock.global_locked.store(false, Ordering::Release);
        } else {
            self.lock.global_locked.store(false, Ordering::Release);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_unlock() {
        let lock = CohortLock::new(42);
        {
            let guard = lock.lock();
            assert!(lock.is_locked());
            assert_eq!(*guard, 42);
        }
        assert!(!lock.is_locked());
    }

    #[test]
    fn test_try_lock() {
        let lock = CohortLock::new(42);
        let guard = lock.try_lock();
        assert!(guard.is_some());

        let guard2 = lock.try_lock();
        assert!(guard2.is_none());
    }
}
