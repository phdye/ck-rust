//! Hazard pointer memory reclamation.
//!
//! Hazard pointers provide safe memory reclamation for lock-free data
//! structures. Each thread maintains a set of "hazard pointers" that
//! indicate which memory locations it's currently accessing.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::UnsafeCell;
use core::ptr;
use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

/// Maximum number of hazard pointers per thread.
const HP_PER_THREAD: usize = 4;

/// Threshold for triggering garbage collection.
const SCAN_THRESHOLD: usize = 2 * HP_PER_THREAD;

/// A hazard pointer record for a thread.
#[repr(C)]
pub struct HpRecord {
    hazards: [AtomicPtr<()>; HP_PER_THREAD],
    next: AtomicPtr<HpRecord>,
    active: AtomicUsize,
    retire_list: UnsafeCell<Vec<RetiredNode>>,
}

struct RetiredNode {
    ptr: *mut (),
    free_fn: unsafe fn(*mut ()),
}

impl HpRecord {
    fn new() -> Self {
        Self {
            hazards: core::array::from_fn(|_| AtomicPtr::new(ptr::null_mut())),
            next: AtomicPtr::new(ptr::null_mut()),
            active: AtomicUsize::new(1),
            retire_list: UnsafeCell::new(Vec::new()),
        }
    }
}

/// Global hazard pointer state.
pub struct HazardPointers {
    records: AtomicPtr<HpRecord>,
}

impl HazardPointers {
    /// Create a new hazard pointer instance.
    pub const fn new() -> Self {
        Self {
            records: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Register the current thread.
    pub fn register(&self) -> HpGuard<'_> {
        let record = Box::into_raw(Box::new(HpRecord::new()));

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

        HpGuard { hp: self, record }
    }

    /// Collect all active hazard pointers.
    fn collect_hazards(&self) -> Vec<*mut ()> {
        let mut hazards = Vec::new();
        let mut current = self.records.load(Ordering::Acquire);

        while !current.is_null() {
            let record = unsafe { &*current };
            if record.active.load(Ordering::Acquire) != 0 {
                for hp in &record.hazards {
                    let ptr = hp.load(Ordering::Acquire);
                    if !ptr.is_null() {
                        hazards.push(ptr);
                    }
                }
            }
            current = record.next.load(Ordering::Acquire);
        }

        hazards
    }
}

impl Default for HazardPointers {
    fn default() -> Self {
        Self::new()
    }
}

/// A guard for hazard pointer protected access.
pub struct HpGuard<'a> {
    hp: &'a HazardPointers,
    record: *mut HpRecord,
}

impl<'a> HpGuard<'a> {
    /// Protect a pointer with a hazard pointer.
    ///
    /// Returns the hazard pointer slot index.
    pub fn protect<T>(&self, slot: usize, ptr: *const T) -> Option<usize> {
        if slot >= HP_PER_THREAD {
            return None;
        }

        let record = unsafe { &*self.record };
        record.hazards[slot].store(ptr as *mut (), Ordering::Release);
        crate::pr::fence_acquire();

        Some(slot)
    }

    /// Clear a hazard pointer slot.
    pub fn clear(&self, slot: usize) {
        if slot < HP_PER_THREAD {
            let record = unsafe { &*self.record };
            record.hazards[slot].store(ptr::null_mut(), Ordering::Release);
        }
    }

    /// Clear all hazard pointers.
    pub fn clear_all(&self) {
        let record = unsafe { &*self.record };
        for hp in &record.hazards {
            hp.store(ptr::null_mut(), Ordering::Release);
        }
    }

    /// Retire a pointer for later reclamation.
    ///
    /// # Safety
    ///
    /// The pointer must be valid and will be freed when safe.
    pub unsafe fn retire<T>(&self, ptr: *mut T) {
        let record = &*self.record;
        let retire_list = &mut *record.retire_list.get();

        retire_list.push(RetiredNode {
            ptr: ptr as *mut (),
            free_fn: |p| {
                drop(Box::from_raw(p as *mut T));
            },
        });

        if retire_list.len() >= SCAN_THRESHOLD {
            self.scan();
        }
    }

    /// Scan and reclaim retired nodes.
    pub fn scan(&self) {
        let hazards = self.hp.collect_hazards();
        let record = unsafe { &*self.record };
        let retire_list = unsafe { &mut *record.retire_list.get() };

        retire_list.retain(|node| {
            if hazards.contains(&node.ptr) {
                true // Keep - still hazardous
            } else {
                // Safe to free
                unsafe {
                    (node.free_fn)(node.ptr);
                }
                false
            }
        });
    }
}

impl Drop for HpGuard<'_> {
    fn drop(&mut self) {
        self.clear_all();
        let record = unsafe { &*self.record };
        record.active.store(0, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let hp = HazardPointers::new();
        let guard = hp.register();
        guard.clear_all();
    }

    #[test]
    fn test_protect() {
        let hp = HazardPointers::new();
        let guard = hp.register();

        let value = Box::new(42i32);
        let ptr = Box::into_raw(value);

        guard.protect(0, ptr);
        guard.clear(0);

        // Clean up
        unsafe {
            drop(Box::from_raw(ptr));
        }
    }

    #[test]
    fn test_retire() {
        let hp = HazardPointers::new();
        let guard = hp.register();

        let value = Box::new(42i32);
        let ptr = Box::into_raw(value);

        unsafe {
            guard.retire(ptr);
        }

        guard.scan();
    }
}
