//! Modern concurrency primitives and building blocks for high performance applications.
//!
//! CK (Concurrency Kit) provides high-performance, lock-free data structures and
//! synchronization primitives for building scalable concurrent applications.
//!
//! # Module Organization
//!
//! The library is organized into tiers based on dependencies:
//!
//! - **Tier 0 (Foundation)**: `cc`, `malloc` - No internal dependencies
//! - **Tier 1 (Core)**: `pr` - Atomic primitives
//! - **Tier 2+**: Higher-level data structures and synchronization primitives

#![no_std]

// Tier 0: Foundation
pub mod cc;
pub mod malloc;

// Tier 1: Core Primitives
pub mod pr;

// Tier 2: Basic Building Blocks
pub mod backoff;
pub mod sequence;

#[cfg(test)]
mod tests {
    #[test]
    fn library_compiles() {
        // Basic smoke test
    }
}
