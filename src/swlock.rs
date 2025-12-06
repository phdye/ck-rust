//! Single-writer reader-writer lock.
//!
//! Optimized for workloads with a single designated writer and many readers.

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::backoff::Backoff;

/// Single-writer reader-writer lock.
#[repr(C)]
pub struct SwLock<T: ?Sized> {
    readers: AtomicUsize,
    data: UnsafeCell<T>,
}

impl<T> SwLock<T> {
    /// Create a new unlocked single-writer lock.
    #[inline]
    pub const fn new(data: T) -> Self {
        Self {
            readers: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquire a read lock.
    #[inline]
    pub fn read(&self) -> SwLockReadGuard<'_, T> {
        self.readers.fetch_add(1, Ordering::Acquire);
        SwLockReadGuard { lock: self }
    }

    /// Acquire a write lock (single writer only).
    ///
    /// # Safety
    ///
    /// Must only be called from a single designated writer thread.
    #[inline]
    pub unsafe fn write(&self) -> SwLockWriteGuard<'_, T> {
        let mut backoff = Backoff::new();
        while self.readers.load(Ordering::Acquire) != 0 {
            backoff.spin();
        }
        SwLockWriteGuard { lock: self }
    }
}

unsafe impl<T: Send> Send for SwLock<T> {}
unsafe impl<T: Send + Sync> Sync for SwLock<T> {}

/// RAII read guard for SwLock.
pub struct SwLockReadGuard<'a, T: ?Sized> {
    lock: &'a SwLock<T>,
}

impl<T: ?Sized> Deref for SwLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for SwLockReadGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.readers.fetch_sub(1, Ordering::Release);
    }
}

/// RAII write guard for SwLock.
pub struct SwLockWriteGuard<'a, T: ?Sized> {
    lock: &'a SwLock<T>,
}

impl<T: ?Sized> Deref for SwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for SwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for SwLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        crate::pr::fence_release();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let lock = SwLock::new(42);
        let guard = lock.read();
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_write() {
        let lock = SwLock::new(0);
        unsafe {
            let mut guard = lock.write();
            *guard = 42;
        }
        let guard = lock.read();
        assert_eq!(*guard, 42);
    }
}
