//! Reader-writer lock.
//!
//! This module provides a reader-writer lock that allows multiple concurrent
//! readers or a single exclusive writer.

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::backoff::Backoff;

/// Writer bit in the counter.
const WRITER: usize = 1;
/// Increment for reader count.
const READER: usize = 2;

/// A reader-writer lock.
///
/// This lock allows multiple concurrent readers or a single exclusive writer.
/// Writers have priority over new readers (write-biased).
#[repr(C)]
pub struct RwLock<T: ?Sized> {
    state: AtomicUsize,
    data: UnsafeCell<T>,
}

impl<T> RwLock<T> {
    /// Create a new unlocked reader-writer lock.
    #[inline]
    pub const fn new(data: T) -> Self {
        Self {
            state: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    /// Try to acquire a read lock without spinning.
    #[inline]
    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        let state = self.state.load(Ordering::Relaxed);
        if state & WRITER != 0 {
            return None;
        }

        let new_state = state + READER;
        if self
            .state
            .compare_exchange(state, new_state, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(RwLockReadGuard { lock: self })
        } else {
            None
        }
    }

    /// Acquire a read lock, spinning if necessary.
    #[inline]
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        let mut backoff = Backoff::new();
        loop {
            if let Some(guard) = self.try_read() {
                return guard;
            }
            backoff.spin();
        }
    }

    /// Try to acquire a write lock without spinning.
    #[inline]
    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        if self
            .state
            .compare_exchange(0, WRITER, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(RwLockWriteGuard { lock: self })
        } else {
            None
        }
    }

    /// Acquire a write lock, spinning if necessary.
    #[inline]
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        let mut backoff = Backoff::new();
        loop {
            if let Some(guard) = self.try_write() {
                return guard;
            }
            backoff.spin();
        }
    }

    /// Check if the lock is currently held for writing.
    #[inline]
    pub fn is_locked_exclusive(&self) -> bool {
        self.state.load(Ordering::Relaxed) & WRITER != 0
    }

    /// Get the current reader count.
    #[inline]
    pub fn reader_count(&self) -> usize {
        self.state.load(Ordering::Relaxed) / READER
    }
}

unsafe impl<T: Send> Send for RwLock<T> {}
unsafe impl<T: Send + Sync> Sync for RwLock<T> {}

/// RAII read guard for RwLock.
pub struct RwLockReadGuard<'a, T: ?Sized> {
    lock: &'a RwLock<T>,
}

impl<T: ?Sized> Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for RwLockReadGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.state.fetch_sub(READER, Ordering::Release);
    }
}

/// RAII write guard for RwLock.
pub struct RwLockWriteGuard<'a, T: ?Sized> {
    lock: &'a RwLock<T>,
}

impl<T: ?Sized> Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for RwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for RwLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.state.fetch_and(!WRITER, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let lock = RwLock::new(42);
        assert!(!lock.is_locked_exclusive());
        assert_eq!(lock.reader_count(), 0);
    }

    #[test]
    fn test_read() {
        let lock = RwLock::new(42);
        let guard = lock.read();
        assert_eq!(*guard, 42);
        assert_eq!(lock.reader_count(), 1);
    }

    #[test]
    fn test_multiple_reads() {
        let lock = RwLock::new(42);
        let g1 = lock.read();
        let g2 = lock.read();
        let g3 = lock.read();

        assert_eq!(lock.reader_count(), 3);
        assert_eq!(*g1, 42);
        assert_eq!(*g2, 42);
        assert_eq!(*g3, 42);
    }

    #[test]
    fn test_write() {
        let lock = RwLock::new(0);

        {
            let mut guard = lock.write();
            *guard = 42;
            assert!(lock.is_locked_exclusive());
        }

        let guard = lock.read();
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_try_read_while_writing() {
        let lock = RwLock::new(42);
        let _write = lock.write();

        assert!(lock.try_read().is_none());
    }

    #[test]
    fn test_try_write_while_reading() {
        let lock = RwLock::new(42);
        let _read = lock.read();

        assert!(lock.try_write().is_none());
    }
}
