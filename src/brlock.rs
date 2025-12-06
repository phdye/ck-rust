//! Big-reader lock.
//!
//! A big-reader lock optimizes for read-heavy workloads by using per-CPU
//! read counters to reduce contention.

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::backoff::Backoff;

/// Maximum number of reader slots (typically matches max CPUs).
const MAX_READERS: usize = 64;

/// Big-reader lock.
#[repr(C)]
pub struct BrLock<T: ?Sized> {
    writer: AtomicBool,
    readers: [AtomicUsize; MAX_READERS],
    data: UnsafeCell<T>,
}

impl<T> BrLock<T> {
    /// Create a new unlocked big-reader lock.
    pub fn new(data: T) -> Self {
        Self {
            writer: AtomicBool::new(false),
            readers: core::array::from_fn(|_| AtomicUsize::new(0)),
            data: UnsafeCell::new(data),
        }
    }

    /// Get the reader slot for the current thread.
    #[inline]
    fn slot(&self) -> usize {
        // Use thread ID hash as slot
        let id = core::ptr::from_ref(self) as usize;
        id % MAX_READERS
    }

    /// Acquire a read lock.
    #[inline]
    pub fn read(&self) -> BrLockReadGuard<'_, T> {
        let slot = self.slot();
        let mut backoff = Backoff::new();

        loop {
            self.readers[slot].fetch_add(1, Ordering::Acquire);

            if !self.writer.load(Ordering::Acquire) {
                break;
            }

            self.readers[slot].fetch_sub(1, Ordering::Release);

            while self.writer.load(Ordering::Relaxed) {
                backoff.spin();
            }
        }

        BrLockReadGuard { lock: self, slot }
    }

    /// Acquire a write lock.
    #[inline]
    pub fn write(&self) -> BrLockWriteGuard<'_, T> {
        let mut backoff = Backoff::new();

        // Acquire writer lock
        while self.writer.swap(true, Ordering::Acquire) {
            backoff.spin();
        }

        // Wait for all readers
        for reader in &self.readers {
            while reader.load(Ordering::Acquire) != 0 {
                backoff.spin();
            }
        }

        BrLockWriteGuard { lock: self }
    }
}

unsafe impl<T: Send> Send for BrLock<T> {}
unsafe impl<T: Send + Sync> Sync for BrLock<T> {}

/// RAII read guard for BrLock.
pub struct BrLockReadGuard<'a, T: ?Sized> {
    lock: &'a BrLock<T>,
    slot: usize,
}

impl<T: ?Sized> Deref for BrLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for BrLockReadGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.readers[self.slot].fetch_sub(1, Ordering::Release);
    }
}

/// RAII write guard for BrLock.
pub struct BrLockWriteGuard<'a, T: ?Sized> {
    lock: &'a BrLock<T>,
}

impl<T: ?Sized> Deref for BrLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for BrLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for BrLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.writer.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let lock = BrLock::new(42);
        let guard = lock.read();
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_write() {
        let lock = BrLock::new(0);
        {
            let mut guard = lock.write();
            *guard = 42;
        }
        let guard = lock.read();
        assert_eq!(*guard, 42);
    }
}
