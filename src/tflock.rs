//! Task-fair reader-writer lock.
//!
//! A task-fair lock provides FIFO ordering for all lock acquisitions.

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::backoff::Backoff;

/// Task-fair reader-writer lock.
#[repr(C)]
pub struct TfLock<T: ?Sized> {
    ticket: AtomicUsize,
    now_serving: AtomicUsize,
    readers: AtomicUsize,
    data: UnsafeCell<T>,
}

impl<T> TfLock<T> {
    /// Create a new unlocked task-fair lock.
    #[inline]
    pub const fn new(data: T) -> Self {
        Self {
            ticket: AtomicUsize::new(0),
            now_serving: AtomicUsize::new(0),
            readers: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquire a read lock.
    #[inline]
    pub fn read(&self) -> TfLockReadGuard<'_, T> {
        let mut backoff = Backoff::new();
        let ticket = self.ticket.fetch_add(1, Ordering::Relaxed);

        while self.now_serving.load(Ordering::Acquire) != ticket {
            backoff.spin();
        }

        self.readers.fetch_add(1, Ordering::Relaxed);
        self.now_serving.fetch_add(1, Ordering::Release);

        TfLockReadGuard { lock: self }
    }

    /// Acquire a write lock.
    #[inline]
    pub fn write(&self) -> TfLockWriteGuard<'_, T> {
        let mut backoff = Backoff::new();
        let ticket = self.ticket.fetch_add(1, Ordering::Relaxed);

        while self.now_serving.load(Ordering::Acquire) != ticket {
            backoff.spin();
        }

        // Wait for readers to drain
        while self.readers.load(Ordering::Acquire) != 0 {
            backoff.spin();
        }

        TfLockWriteGuard { lock: self }
    }
}

unsafe impl<T: Send> Send for TfLock<T> {}
unsafe impl<T: Send + Sync> Sync for TfLock<T> {}

/// RAII read guard for TfLock.
pub struct TfLockReadGuard<'a, T: ?Sized> {
    lock: &'a TfLock<T>,
}

impl<T: ?Sized> Deref for TfLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for TfLockReadGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.readers.fetch_sub(1, Ordering::Release);
    }
}

/// RAII write guard for TfLock.
pub struct TfLockWriteGuard<'a, T: ?Sized> {
    lock: &'a TfLock<T>,
}

impl<T: ?Sized> Deref for TfLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for TfLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for TfLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.now_serving.fetch_add(1, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let lock = TfLock::new(42);
        let guard = lock.read();
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_write() {
        let lock = TfLock::new(0);
        {
            let mut guard = lock.write();
            *guard = 42;
        }
        let guard = lock.read();
        assert_eq!(*guard, 42);
    }
}
