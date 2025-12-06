//! Hardware transactional memory lock elision.
//!
//! This module provides lock elision using hardware transactional memory (HTM)
//! when available. Falls back to regular locking when HTM is not supported
//! or transactions fail.

use core::sync::atomic::{AtomicBool, Ordering};

/// Check if HTM is available on this platform.
#[inline]
pub fn is_available() -> bool {
    // HTM availability depends on CPU features
    // For now, return false as a safe default
    #[cfg(all(target_arch = "x86_64", target_feature = "rtm"))]
    {
        true
    }
    #[cfg(not(all(target_arch = "x86_64", target_feature = "rtm")))]
    {
        false
    }
}

/// Elision configuration.
#[derive(Debug, Clone, Copy)]
pub struct ElideConfig {
    /// Maximum retry attempts before falling back to lock.
    pub max_retries: u32,
    /// Whether elision is enabled.
    pub enabled: bool,
}

impl Default for ElideConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            enabled: is_available(),
        }
    }
}

/// An elide-capable lock wrapper.
///
/// Wraps a lock and attempts to use HTM elision when available.
pub struct ElideLock {
    fallback_locked: AtomicBool,
    config: ElideConfig,
}

impl ElideLock {
    /// Create a new elide lock.
    #[inline]
    pub const fn new() -> Self {
        Self {
            fallback_locked: AtomicBool::new(false),
            config: ElideConfig {
                max_retries: 3,
                enabled: false, // Will be set at runtime
            },
        }
    }

    /// Create a new elide lock with custom configuration.
    #[inline]
    pub const fn with_config(config: ElideConfig) -> Self {
        Self {
            fallback_locked: AtomicBool::new(false),
            config,
        }
    }

    /// Acquire the lock with elision if possible.
    #[inline]
    pub fn lock(&self) -> ElideGuard<'_> {
        // For now, just use fallback locking
        while self.fallback_locked.swap(true, Ordering::Acquire) {
            crate::pr::stall();
        }
        ElideGuard {
            lock: self,
            elided: false,
        }
    }

    /// Try to acquire the lock.
    #[inline]
    pub fn try_lock(&self) -> Option<ElideGuard<'_>> {
        if self.fallback_locked.swap(true, Ordering::Acquire) {
            None
        } else {
            Some(ElideGuard {
                lock: self,
                elided: false,
            })
        }
    }

    /// Check if the lock is held.
    #[inline]
    pub fn is_locked(&self) -> bool {
        self.fallback_locked.load(Ordering::Relaxed)
    }
}

impl Default for ElideLock {
    fn default() -> Self {
        Self::new()
    }
}

/// RAII guard for ElideLock.
pub struct ElideGuard<'a> {
    lock: &'a ElideLock,
    elided: bool,
}

impl Drop for ElideGuard<'_> {
    fn drop(&mut self) {
        if !self.elided {
            self.lock.fallback_locked.store(false, Ordering::Release);
        }
        // If elided, HTM commit will happen automatically
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let lock = ElideLock::new();
        assert!(!lock.is_locked());
    }

    #[test]
    fn test_lock_unlock() {
        let lock = ElideLock::new();
        {
            let _guard = lock.lock();
            assert!(lock.is_locked());
        }
        assert!(!lock.is_locked());
    }

    #[test]
    fn test_try_lock() {
        let lock = ElideLock::new();
        let guard = lock.try_lock();
        assert!(guard.is_some());

        let guard2 = lock.try_lock();
        assert!(guard2.is_none());
    }
}
