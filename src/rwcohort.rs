//! Reader-writer lock cohorting for NUMA.
//!
//! Extends cohort locking to reader-writer locks for NUMA optimization.

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::backoff::Backoff;

const WRITER: usize = 1;
const READER: usize = 2;

/// A reader-writer cohort lock.
#[repr(C)]
pub struct RwCohortLock<T: ?Sized> {
    state: AtomicUsize,
    data: UnsafeCell<T>,
}

impl<T> RwCohortLock<T> {
    /// Create a new reader-writer cohort lock.
    #[inline]
    pub const fn new(data: T) -> Self {
        Self {
            state: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquire a read lock.
    #[inline]
    pub fn read(&self) -> RwCohortReadGuard<'_, T> {
        let mut backoff = Backoff::new();

        loop {
            let state = self.state.load(Ordering::Relaxed);
            if state & WRITER == 0 {
                let new_state = state + READER;
                if self
                    .state
                    .compare_exchange_weak(state, new_state, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
                {
                    return RwCohortReadGuard { lock: self };
                }
            }
            backoff.spin();
        }
    }

    /// Acquire a write lock.
    #[inline]
    pub fn write(&self) -> RwCohortWriteGuard<'_, T> {
        let mut backoff = Backoff::new();

        loop {
            if self
                .state
                .compare_exchange_weak(0, WRITER, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                return RwCohortWriteGuard { lock: self };
            }
            backoff.spin();
        }
    }
}

unsafe impl<T: Send> Send for RwCohortLock<T> {}
unsafe impl<T: Send + Sync> Sync for RwCohortLock<T> {}

/// RAII read guard for RwCohortLock.
pub struct RwCohortReadGuard<'a, T: ?Sized> {
    lock: &'a RwCohortLock<T>,
}

impl<T: ?Sized> Deref for RwCohortReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for RwCohortReadGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.state.fetch_sub(READER, Ordering::Release);
    }
}

/// RAII write guard for RwCohortLock.
pub struct RwCohortWriteGuard<'a, T: ?Sized> {
    lock: &'a RwCohortLock<T>,
}

impl<T: ?Sized> Deref for RwCohortWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for RwCohortWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for RwCohortWriteGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.state.fetch_and(!WRITER, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let lock = RwCohortLock::new(42);
        let guard = lock.read();
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_write() {
        let lock = RwCohortLock::new(0);
        {
            let mut guard = lock.write();
            *guard = 42;
        }
        let guard = lock.read();
        assert_eq!(*guard, 42);
    }
}
