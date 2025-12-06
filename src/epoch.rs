//! Epoch-based safe memory reclamation.
//!
//! Epoch-based reclamation (EBR) is a technique for safely reclaiming memory
//! in lock-free data structures. It tracks which threads are in critical
//! sections and defers memory reclamation until all threads have passed
//! through a grace period.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::UnsafeCell;
use core::ptr;
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

/// The global epoch counter.
static GLOBAL_EPOCH: AtomicUsize = AtomicUsize::new(0);

/// Number of epochs before memory can be freed.
const EPOCH_GRACE: usize = 2;

/// A record for a registered thread.
#[repr(C)]
pub struct EpochRecord {
    epoch: AtomicUsize,
    active: AtomicUsize,
    next: AtomicPtr<EpochRecord>,
    garbage: [UnsafeCell<Vec<DeferredFree>>; 3],
}

struct DeferredFree {
    ptr: *mut u8,
    free_fn: unsafe fn(*mut u8),
}

impl EpochRecord {
    /// Create a new epoch record.
    fn new() -> Self {
        Self {
            epoch: AtomicUsize::new(0),
            active: AtomicUsize::new(0),
            next: AtomicPtr::new(ptr::null_mut()),
            garbage: [
                UnsafeCell::new(Vec::new()),
                UnsafeCell::new(Vec::new()),
                UnsafeCell::new(Vec::new()),
            ],
        }
    }
}

/// Global epoch state.
pub struct Epoch {
    records: AtomicPtr<EpochRecord>,
}

impl Epoch {
    /// Create a new epoch instance.
    pub const fn new() -> Self {
        Self {
            records: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Register the current thread.
    pub fn register(&self) -> Guard<'_> {
        let record = Box::into_raw(Box::new(EpochRecord::new()));

        // Add to linked list
        loop {
            let head = self.records.load(Ordering::Relaxed);
            unsafe {
                (*record).next.store(head, Ordering::Relaxed);
            }
            if self
                .records
                .compare_exchange_weak(head, record, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }
        }

        Guard {
            epoch: self,
            record,
            _marker: core::marker::PhantomData,
        }
    }

    /// Try to advance the global epoch.
    pub fn try_advance(&self) -> bool {
        let global = GLOBAL_EPOCH.load(Ordering::Acquire);
        let new_epoch = global.wrapping_add(1);

        // Check if all threads have caught up
        let mut current = self.records.load(Ordering::Acquire);
        while !current.is_null() {
            let record = unsafe { &*current };
            if record.active.load(Ordering::Acquire) != 0 {
                let thread_epoch = record.epoch.load(Ordering::Acquire);
                if thread_epoch != global {
                    return false;
                }
            }
            current = record.next.load(Ordering::Acquire);
        }

        GLOBAL_EPOCH
            .compare_exchange(global, new_epoch, Ordering::Release, Ordering::Relaxed)
            .is_ok()
    }
}

impl Default for Epoch {
    fn default() -> Self {
        Self::new()
    }
}

/// A guard for epoch-protected access.
pub struct Guard<'a> {
    epoch: &'a Epoch,
    record: *mut EpochRecord,
    // Make Guard !Send and !Sync by including a non-Send/Sync type
    _marker: core::marker::PhantomData<*mut ()>,
}

impl<'a> Guard<'a> {
    /// Enter a critical section.
    pub fn enter(&self) {
        let record = unsafe { &*self.record };
        let global = GLOBAL_EPOCH.load(Ordering::Acquire);
        record.epoch.store(global, Ordering::Relaxed);
        record.active.fetch_add(1, Ordering::Release);
        crate::pr::fence_acquire();
    }

    /// Leave a critical section.
    pub fn leave(&self) {
        let record = unsafe { &*self.record };
        crate::pr::fence_release();
        record.active.fetch_sub(1, Ordering::Release);
    }

    /// Defer freeing a pointer until it's safe.
    ///
    /// # Safety
    ///
    /// The pointer must be valid and must have been allocated by the
    /// corresponding allocation function.
    pub unsafe fn defer_free<T>(&self, ptr: *mut T) {
        let record = &*self.record;
        let epoch = record.epoch.load(Ordering::Relaxed) % 3;
        let garbage = &mut *record.garbage[epoch].get();
        garbage.push(DeferredFree {
            ptr: ptr as *mut u8,
            free_fn: |p| {
                drop(Box::from_raw(p as *mut T));
            },
        });
    }

    /// Try to reclaim garbage from old epochs.
    pub fn try_reclaim(&self) {
        let record = unsafe { &*self.record };
        let epoch = record.epoch.load(Ordering::Relaxed);
        let old_epoch = epoch.wrapping_sub(EPOCH_GRACE) % 3;

        let garbage = unsafe { &mut *record.garbage[old_epoch].get() };
        for item in garbage.drain(..) {
            unsafe {
                (item.free_fn)(item.ptr);
            }
        }
    }
}

impl Drop for Guard<'_> {
    fn drop(&mut self) {
        // Ensure we're not in a critical section
        let record = unsafe { &*self.record };
        if record.active.load(Ordering::Relaxed) > 0 {
            self.leave();
        }
    }
}

// Guard is !Send and !Sync because record is a raw pointer

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let epoch = Epoch::new();
        let guard = epoch.register();
        guard.enter();
        guard.leave();
    }

    #[test]
    fn test_defer_free() {
        let epoch = Epoch::new();
        let guard = epoch.register();
        guard.enter();

        let ptr = Box::into_raw(Box::new(42i32));
        unsafe {
            guard.defer_free(ptr);
        }

        guard.leave();
    }
}
