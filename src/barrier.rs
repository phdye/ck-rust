//! Execution barriers.
//!
//! Barriers allow a set of threads to synchronize at a common point.
//! All threads must reach the barrier before any can proceed.

use core::sync::atomic::{AtomicUsize, Ordering};

use crate::backoff::Backoff;

/// A simple centralized barrier.
pub struct Barrier {
    count: AtomicUsize,
    total: usize,
    generation: AtomicUsize,
}

impl Barrier {
    /// Create a new barrier for the specified number of threads.
    #[inline]
    pub fn new(n: usize) -> Self {
        Self {
            count: AtomicUsize::new(0),
            total: n,
            generation: AtomicUsize::new(0),
        }
    }

    /// Wait at the barrier until all threads arrive.
    ///
    /// Returns `true` for exactly one thread (the "leader").
    pub fn wait(&self) -> bool {
        let mut backoff = Backoff::new();
        let gen = self.generation.load(Ordering::Acquire);

        let arrived = self.count.fetch_add(1, Ordering::AcqRel) + 1;

        if arrived == self.total {
            // Last thread to arrive - reset and advance generation
            self.count.store(0, Ordering::Relaxed);
            self.generation.fetch_add(1, Ordering::Release);
            return true;
        }

        // Wait for generation to change
        while self.generation.load(Ordering::Acquire) == gen {
            backoff.spin();
        }

        false
    }

    /// Reset the barrier for reuse.
    pub fn reset(&self) {
        self.count.store(0, Ordering::Relaxed);
        self.generation.fetch_add(1, Ordering::Release);
    }
}

/// A sense-reversing barrier for better performance.
pub struct SenseBarrier {
    count: AtomicUsize,
    total: usize,
    sense: AtomicUsize,
}

impl SenseBarrier {
    /// Create a new sense-reversing barrier.
    #[inline]
    pub fn new(n: usize) -> Self {
        Self {
            count: AtomicUsize::new(0),
            total: n,
            sense: AtomicUsize::new(0),
        }
    }

    /// Wait at the barrier.
    pub fn wait(&self, local_sense: &mut usize) -> bool {
        *local_sense = 1 - *local_sense;
        let mut backoff = Backoff::new();

        let arrived = self.count.fetch_add(1, Ordering::AcqRel) + 1;

        if arrived == self.total {
            self.count.store(0, Ordering::Relaxed);
            self.sense.store(*local_sense, Ordering::Release);
            return true;
        }

        while self.sense.load(Ordering::Acquire) != *local_sense {
            backoff.spin();
        }

        false
    }
}

/// A combining tree barrier for better scalability.
pub struct CombiningBarrier {
    nodes: alloc::vec::Vec<AtomicUsize>,
    total: usize,
}

impl CombiningBarrier {
    /// Create a new combining barrier.
    pub fn new(n: usize) -> Self {
        let num_nodes = n.next_power_of_two();
        Self {
            nodes: (0..num_nodes).map(|_| AtomicUsize::new(0)).collect(),
            total: n,
        }
    }

    /// Wait at the barrier.
    pub fn wait(&self, _thread_id: usize) -> bool {
        let mut backoff = Backoff::new();

        // Simple implementation - use centralized for now
        let arrived = self.nodes[0].fetch_add(1, Ordering::AcqRel) + 1;

        if arrived == self.total {
            self.nodes[0].store(0, Ordering::Relaxed);
            return true;
        }

        // Spin until done
        while self.nodes[0].load(Ordering::Acquire) != 0 {
            backoff.spin();
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barrier_single_thread() {
        let barrier = Barrier::new(1);
        assert!(barrier.wait()); // Single thread is always leader
    }

    #[test]
    fn test_sense_barrier_single() {
        let barrier = SenseBarrier::new(1);
        let mut sense = 0;
        assert!(barrier.wait(&mut sense));
    }

    #[test]
    fn test_combining_barrier_single() {
        let barrier = CombiningBarrier::new(1);
        assert!(barrier.wait(0));
    }
}
