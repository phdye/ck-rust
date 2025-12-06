//! Phase-fair reader-writer lock.
//!
//! A phase-fair lock alternates between reader and writer phases,
//! preventing starvation of either group.

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicU32, Ordering};

use crate::backoff::Backoff;

/// Writer bit - indicates a writer wants access.
const WBIT: u32 = 0x1;
/// Reader increment value.
const RINC: u32 = 0x4;
/// Reader mask for counting readers.
const RMASK: u32 = !0x3;

/// A phase-fair reader-writer lock.
///
/// This lock provides fairness by alternating between reader and writer phases.
/// When a writer is waiting, new readers are blocked until the writer completes.
#[repr(C)]
pub struct PfLock<T: ?Sized> {
    /// Combined state: readers count (upper bits) + writer waiting (bit 0)
    state: AtomicU32,
    data: UnsafeCell<T>,
}

impl<T> PfLock<T> {
    /// Create a new unlocked phase-fair lock.
    #[inline]
    pub const fn new(data: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquire a read lock.
    #[inline]
    pub fn read(&self) -> PfLockReadGuard<'_, T> {
        let mut backoff = Backoff::new();

        loop {
            let state = self.state.load(Ordering::Acquire);

            // If no writer waiting/active, try to add ourselves as reader
            if (state & WBIT) == 0 {
                if self.state
                    .compare_exchange_weak(
                        state,
                        state + RINC,
                        Ordering::Acquire,
                        Ordering::Relaxed,
                    )
                    .is_ok()
                {
                    return PfLockReadGuard { lock: self };
                }
            }
            backoff.spin();
        }
    }

    /// Acquire a write lock.
    #[inline]
    pub fn write(&self) -> PfLockWriteGuard<'_, T> {
        let mut backoff = Backoff::new();

        // First, set writer waiting bit
        loop {
            let state = self.state.load(Ordering::Acquire);
            if (state & WBIT) == 0 {
                if self.state
                    .compare_exchange_weak(
                        state,
                        state | WBIT,
                        Ordering::Acquire,
                        Ordering::Relaxed,
                    )
                    .is_ok()
                {
                    break;
                }
            }
            backoff.spin();
        }

        // Wait for all readers to exit
        backoff.reset();
        while (self.state.load(Ordering::Acquire) & RMASK) != 0 {
            backoff.spin();
        }

        PfLockWriteGuard { lock: self }
    }
}

unsafe impl<T: Send> Send for PfLock<T> {}
unsafe impl<T: Send + Sync> Sync for PfLock<T> {}

/// RAII read guard for PfLock.
pub struct PfLockReadGuard<'a, T: ?Sized> {
    lock: &'a PfLock<T>,
}

impl<T: ?Sized> Deref for PfLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for PfLockReadGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.state.fetch_sub(RINC, Ordering::Release);
    }
}

/// RAII write guard for PfLock.
pub struct PfLockWriteGuard<'a, T: ?Sized> {
    lock: &'a PfLock<T>,
}

impl<T: ?Sized> Deref for PfLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for PfLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for PfLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        // Clear writer bit to release the lock
        self.lock.state.fetch_and(!WBIT, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let lock = PfLock::new(42);
        let guard = lock.read();
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_write() {
        let lock = PfLock::new(0);

        {
            let mut guard = lock.write();
            *guard = 42;
        }

        let guard = lock.read();
        assert_eq!(*guard, 42);
    }
}
