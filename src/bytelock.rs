//! Byte-level reader-writer lock.
//!
//! A bytelock uses individual bytes for each reader slot, reducing
//! cache line contention in read-heavy workloads.

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicU8, AtomicBool, Ordering};

use crate::backoff::Backoff;

/// Maximum number of reader slots.
const MAX_SLOTS: usize = 127;

/// Byte lock structure.
#[repr(C)]
pub struct ByteLock<T: ?Sized> {
    writer: AtomicBool,
    readers: [AtomicU8; MAX_SLOTS],
    data: UnsafeCell<T>,
}

impl<T> ByteLock<T> {
    /// Create a new unlocked byte lock.
    pub fn new(data: T) -> Self {
        Self {
            writer: AtomicBool::new(false),
            readers: core::array::from_fn(|_| AtomicU8::new(0)),
            data: UnsafeCell::new(data),
        }
    }

    /// Get a reader slot.
    #[inline]
    fn slot(&self) -> usize {
        let id = core::ptr::from_ref(self) as usize;
        id % MAX_SLOTS
    }

    /// Acquire a read lock.
    #[inline]
    pub fn read(&self) -> ByteLockReadGuard<'_, T> {
        let slot = self.slot();
        let mut backoff = Backoff::new();

        loop {
            self.readers[slot].store(1, Ordering::Release);
            crate::pr::fence_memory();

            if !self.writer.load(Ordering::Acquire) {
                break;
            }

            self.readers[slot].store(0, Ordering::Release);

            while self.writer.load(Ordering::Relaxed) {
                backoff.spin();
            }
        }

        ByteLockReadGuard { lock: self, slot }
    }

    /// Acquire a write lock.
    #[inline]
    pub fn write(&self) -> ByteLockWriteGuard<'_, T> {
        let mut backoff = Backoff::new();

        while self.writer.swap(true, Ordering::Acquire) {
            backoff.spin();
        }

        // Wait for all readers
        for reader in &self.readers {
            while reader.load(Ordering::Acquire) != 0 {
                backoff.spin();
            }
        }

        ByteLockWriteGuard { lock: self }
    }
}

unsafe impl<T: Send> Send for ByteLock<T> {}
unsafe impl<T: Send + Sync> Sync for ByteLock<T> {}

/// RAII read guard for ByteLock.
pub struct ByteLockReadGuard<'a, T: ?Sized> {
    lock: &'a ByteLock<T>,
    slot: usize,
}

impl<T: ?Sized> Deref for ByteLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for ByteLockReadGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.readers[self.slot].store(0, Ordering::Release);
    }
}

/// RAII write guard for ByteLock.
pub struct ByteLockWriteGuard<'a, T: ?Sized> {
    lock: &'a ByteLock<T>,
}

impl<T: ?Sized> Deref for ByteLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for ByteLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for ByteLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.writer.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let lock = ByteLock::new(42);
        let guard = lock.read();
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_write() {
        let lock = ByteLock::new(0);
        {
            let mut guard = lock.write();
            *guard = 42;
        }
        let guard = lock.read();
        assert_eq!(*guard, 42);
    }
}
