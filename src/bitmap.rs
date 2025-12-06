//! Concurrent bitmap operations.
//!
//! This module provides atomic bitmap operations for efficiently tracking
//! sets of boolean flags or managing resource allocation.

use core::sync::atomic::{AtomicUsize, Ordering};

/// Number of bits per word.
const BITS_PER_WORD: usize = core::mem::size_of::<usize>() * 8;

/// A concurrent bitmap.
///
/// Supports atomic test-and-set, test-and-clear, and iteration operations.
pub struct Bitmap<const N: usize> {
    words: [AtomicUsize; N],
}

impl<const N: usize> Bitmap<N> {
    /// Create a new bitmap with all bits cleared.
    pub const fn new() -> Self {
        // Use array::from_fn at runtime, const initialization requires this
        Self {
            words: unsafe {
                // SAFETY: AtomicUsize has the same layout as usize
                let mut arr: [AtomicUsize; N] = core::mem::zeroed();
                let mut i = 0;
                while i < N {
                    core::ptr::write(&mut arr[i], AtomicUsize::new(0));
                    i += 1;
                }
                arr
            },
        }
    }

    /// Total number of bits in the bitmap.
    #[inline]
    pub const fn bits(&self) -> usize {
        N * BITS_PER_WORD
    }

    /// Get the value of a bit.
    #[inline]
    pub fn get(&self, index: usize) -> bool {
        let word = index / BITS_PER_WORD;
        let bit = index % BITS_PER_WORD;

        if word >= N {
            return false;
        }

        (self.words[word].load(Ordering::Acquire) & (1 << bit)) != 0
    }

    /// Set a bit to 1. Returns the previous value.
    #[inline]
    pub fn set(&self, index: usize) -> bool {
        let word = index / BITS_PER_WORD;
        let bit = index % BITS_PER_WORD;

        if word >= N {
            return false;
        }

        let mask = 1usize << bit;
        let old = self.words[word].fetch_or(mask, Ordering::AcqRel);
        (old & mask) != 0
    }

    /// Clear a bit to 0. Returns the previous value.
    #[inline]
    pub fn clear(&self, index: usize) -> bool {
        let word = index / BITS_PER_WORD;
        let bit = index % BITS_PER_WORD;

        if word >= N {
            return false;
        }

        let mask = 1usize << bit;
        let old = self.words[word].fetch_and(!mask, Ordering::AcqRel);
        (old & mask) != 0
    }

    /// Toggle a bit. Returns the previous value.
    #[inline]
    pub fn toggle(&self, index: usize) -> bool {
        let word = index / BITS_PER_WORD;
        let bit = index % BITS_PER_WORD;

        if word >= N {
            return false;
        }

        let mask = 1usize << bit;
        let old = self.words[word].fetch_xor(mask, Ordering::AcqRel);
        (old & mask) != 0
    }

    /// Test and set: set bit to 1, return previous value.
    #[inline]
    pub fn test_and_set(&self, index: usize) -> bool {
        self.set(index)
    }

    /// Test and clear: set bit to 0, return previous value.
    #[inline]
    pub fn test_and_clear(&self, index: usize) -> bool {
        self.clear(index)
    }

    /// Count the number of set bits.
    #[inline]
    pub fn popcount(&self) -> usize {
        self.words
            .iter()
            .map(|w| w.load(Ordering::Acquire).count_ones() as usize)
            .sum()
    }

    /// Find the first set bit. Returns `None` if all bits are clear.
    pub fn find_first_set(&self) -> Option<usize> {
        for (i, word) in self.words.iter().enumerate() {
            let w = word.load(Ordering::Acquire);
            if w != 0 {
                let bit = w.trailing_zeros() as usize;
                return Some(i * BITS_PER_WORD + bit);
            }
        }
        None
    }

    /// Find the first clear bit. Returns `None` if all bits are set.
    pub fn find_first_clear(&self) -> Option<usize> {
        for (i, word) in self.words.iter().enumerate() {
            let w = word.load(Ordering::Acquire);
            if w != usize::MAX {
                let bit = (!w).trailing_zeros() as usize;
                let index = i * BITS_PER_WORD + bit;
                if index < self.bits() {
                    return Some(index);
                }
            }
        }
        None
    }

    /// Clear all bits.
    pub fn clear_all(&self) {
        for word in &self.words {
            word.store(0, Ordering::Release);
        }
    }

    /// Set all bits.
    pub fn set_all(&self) {
        for word in &self.words {
            word.store(usize::MAX, Ordering::Release);
        }
    }
}

impl<const N: usize> Default for Bitmap<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bm: Bitmap<2> = Bitmap::new();
        assert_eq!(bm.bits(), 2 * BITS_PER_WORD);
    }

    #[test]
    fn test_get_set_clear() {
        let bm: Bitmap<2> = Bitmap::new();

        assert!(!bm.get(0));
        assert!(!bm.set(0)); // Returns previous value (false)
        assert!(bm.get(0));
        assert!(bm.clear(0)); // Returns previous value (true)
        assert!(!bm.get(0));
    }

    #[test]
    fn test_toggle() {
        let bm: Bitmap<1> = Bitmap::new();

        assert!(!bm.toggle(5)); // Was 0, now 1
        assert!(bm.get(5));
        assert!(bm.toggle(5)); // Was 1, now 0
        assert!(!bm.get(5));
    }

    #[test]
    fn test_popcount() {
        let bm: Bitmap<1> = Bitmap::new();

        assert_eq!(bm.popcount(), 0);
        bm.set(0);
        bm.set(5);
        bm.set(10);
        assert_eq!(bm.popcount(), 3);
    }

    #[test]
    fn test_find_first_set() {
        let bm: Bitmap<2> = Bitmap::new();

        assert!(bm.find_first_set().is_none());
        bm.set(42);
        assert_eq!(bm.find_first_set(), Some(42));
        bm.set(10);
        assert_eq!(bm.find_first_set(), Some(10));
    }

    #[test]
    fn test_find_first_clear() {
        let bm: Bitmap<1> = Bitmap::new();
        bm.set_all();

        assert!(bm.find_first_clear().is_none());
        bm.clear(42);
        assert_eq!(bm.find_first_clear(), Some(42));
    }

    #[test]
    fn test_clear_set_all() {
        let bm: Bitmap<1> = Bitmap::new();

        bm.set_all();
        assert_eq!(bm.popcount(), BITS_PER_WORD);

        bm.clear_all();
        assert_eq!(bm.popcount(), 0);
    }
}
