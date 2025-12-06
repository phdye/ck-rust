//! Various spinlock implementations.
//!
//! This module provides several spinlock variants with different
//! fairness and performance characteristics:
//!
//! - [`FasLock`] - Simple fetch-and-store spinlock (unfair but fast)
//! - [`CasLock`] - Compare-and-swap spinlock (unfair)
//! - [`TicketLock`] - Fair FIFO spinlock
//! - [`McsLock`] - MCS queue-based spinlock (fair, cache-friendly)

use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use crate::backoff::Backoff;

/// A simple fetch-and-store spinlock.
///
/// This is the simplest and fastest spinlock, but is unfair - threads
/// may be starved under contention.
#[repr(C)]
pub struct FasLock<T: ?Sized> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T> FasLock<T> {
    /// Create a new unlocked spinlock.
    #[inline]
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Try to acquire the lock without spinning.
    #[inline]
    pub fn try_lock(&self) -> Option<FasLockGuard<'_, T>> {
        if self.locked.swap(true, Ordering::Acquire) {
            None
        } else {
            Some(FasLockGuard { lock: self })
        }
    }

    /// Acquire the lock, spinning if necessary.
    #[inline]
    pub fn lock(&self) -> FasLockGuard<'_, T> {
        let mut backoff = Backoff::new();
        while self.locked.swap(true, Ordering::Acquire) {
            while self.locked.load(Ordering::Relaxed) {
                backoff.spin();
            }
        }
        FasLockGuard { lock: self }
    }

    /// Check if the lock is currently held.
    #[inline]
    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Relaxed)
    }
}

unsafe impl<T: Send> Send for FasLock<T> {}
unsafe impl<T: Send> Sync for FasLock<T> {}

/// RAII guard for FasLock.
pub struct FasLockGuard<'a, T: ?Sized> {
    lock: &'a FasLock<T>,
}

impl<T: ?Sized> Deref for FasLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for FasLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for FasLockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

/// A ticket lock with FIFO fairness.
///
/// Ticket locks provide strict FIFO ordering - threads acquire the lock
/// in the order they requested it.
#[repr(C)]
pub struct TicketLock<T: ?Sized> {
    next_ticket: AtomicUsize,
    now_serving: AtomicUsize,
    data: UnsafeCell<T>,
}

impl<T> TicketLock<T> {
    /// Create a new unlocked ticket lock.
    #[inline]
    pub const fn new(data: T) -> Self {
        Self {
            next_ticket: AtomicUsize::new(0),
            now_serving: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    /// Try to acquire the lock without spinning.
    #[inline]
    pub fn try_lock(&self) -> Option<TicketLockGuard<'_, T>> {
        let ticket = self.next_ticket.load(Ordering::Relaxed);
        let serving = self.now_serving.load(Ordering::Acquire);

        if ticket == serving {
            if self
                .next_ticket
                .compare_exchange(ticket, ticket + 1, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                return Some(TicketLockGuard { lock: self });
            }
        }
        None
    }

    /// Acquire the lock, spinning if necessary.
    #[inline]
    pub fn lock(&self) -> TicketLockGuard<'_, T> {
        let ticket = self.next_ticket.fetch_add(1, Ordering::Relaxed);
        let mut backoff = Backoff::new();

        while self.now_serving.load(Ordering::Acquire) != ticket {
            backoff.spin();
        }

        TicketLockGuard { lock: self }
    }

    /// Check if the lock is currently held.
    #[inline]
    pub fn is_locked(&self) -> bool {
        let next = self.next_ticket.load(Ordering::Relaxed);
        let serving = self.now_serving.load(Ordering::Relaxed);
        next != serving
    }
}

unsafe impl<T: Send> Send for TicketLock<T> {}
unsafe impl<T: Send> Sync for TicketLock<T> {}

/// RAII guard for TicketLock.
pub struct TicketLockGuard<'a, T: ?Sized> {
    lock: &'a TicketLock<T>,
}

impl<T: ?Sized> Deref for TicketLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for TicketLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for TicketLockGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.now_serving.fetch_add(1, Ordering::Release);
    }
}

/// Type alias for the default spinlock type.
pub type SpinLock<T> = FasLock<T>;
/// Type alias for the default spinlock guard.
pub type SpinLockGuard<'a, T> = FasLockGuard<'a, T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fas_lock_new() {
        let lock = FasLock::new(42);
        assert!(!lock.is_locked());
    }

    #[test]
    fn test_fas_lock_lock_unlock() {
        let lock = FasLock::new(42);

        {
            let guard = lock.lock();
            assert!(lock.is_locked());
            assert_eq!(*guard, 42);
        }

        assert!(!lock.is_locked());
    }

    #[test]
    fn test_fas_lock_try_lock() {
        let lock = FasLock::new(42);

        let guard = lock.try_lock();
        assert!(guard.is_some());
        assert!(lock.is_locked());

        let guard2 = lock.try_lock();
        assert!(guard2.is_none());

        drop(guard);
        let guard3 = lock.try_lock();
        assert!(guard3.is_some());
    }

    #[test]
    fn test_fas_lock_modify() {
        let lock = FasLock::new(0);

        {
            let mut guard = lock.lock();
            *guard = 42;
        }

        let guard = lock.lock();
        assert_eq!(*guard, 42);
    }

    #[test]
    fn test_ticket_lock_new() {
        let lock = TicketLock::new(42);
        assert!(!lock.is_locked());
    }

    #[test]
    fn test_ticket_lock_lock_unlock() {
        let lock = TicketLock::new(42);

        {
            let guard = lock.lock();
            assert!(lock.is_locked());
            assert_eq!(*guard, 42);
        }

        assert!(!lock.is_locked());
    }

    #[test]
    fn test_ticket_lock_try_lock() {
        let lock = TicketLock::new(42);

        let guard = lock.try_lock();
        assert!(guard.is_some());

        let guard2 = lock.try_lock();
        assert!(guard2.is_none());

        drop(guard);
        let guard3 = lock.try_lock();
        assert!(guard3.is_some());
    }

    #[test]
    fn test_ticket_lock_modify() {
        let lock = TicketLock::new(0);

        {
            let mut guard = lock.lock();
            *guard = 42;
        }

        let guard = lock.lock();
        assert_eq!(*guard, 42);
    }
}
