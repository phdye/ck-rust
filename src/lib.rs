//! # Concurrency Kit (CK) - Rust Implementation
//!
//! Modern concurrency primitives and building blocks for high performance applications.
//!
//! This library provides a comprehensive set of lock-free and concurrent data structures,
//! synchronization primitives, and memory reclamation schemes ported from the
//! [Concurrency Kit](http://concurrencykit.org/) C library.
//!
//! ## Module Organization
//!
//! The library is organized into the following categories:
//!
//! ### Foundation
//! - [`cc`] - Compiler compatibility utilities (bit operations, branch hints)
//! - [`malloc`] - Memory allocator abstraction
//!
//! ### Core Primitives
//! - [`pr`] - Atomic primitives and memory barriers
//!
//! ### Basic Building Blocks
//! - [`mod@backoff`] - Exponential backoff for contention management
//! - [`stack`] - Lock-free stack
//! - [`queue`] - BSD-style queue macros
//! - [`ring`] - Concurrent ring buffer
//! - [`bitmap`] - Concurrent bitmap operations
//! - [`sequence`] - Sequence counter (seqlock)
//!
//! ### Locks
//! - [`spinlock`] - Various spinlock implementations
//! - [`rwlock`] - Reader-writer lock
//! - [`pflock`] - Phase-fair reader-writer lock
//! - [`swlock`] - Single-writer reader-writer lock
//! - [`tflock`] - Task-fair reader-writer lock
//! - [`brlock`] - Big-reader lock
//! - [`bytelock`] - Byte-level reader-writer lock
//! - [`cohort`] - Lock cohorting for NUMA
//! - [`rwcohort`] - Reader-writer lock cohorting for NUMA
//!
//! ### Extended Primitives
//! - [`elide`] - Hardware transactional memory lock elision
//! - [`mod@array`] - Concurrently-readable dynamic array
//! - [`hs`] - Hash set (single-writer, many-reader)
//! - [`ht`] - Hash table (key-value pairs)
//! - [`rhs`] - Robin-hood hash set
//!
//! ### Memory Reclamation
//! - [`epoch`] - Epoch-based safe memory reclamation
//! - [`hp`] - Hazard pointer memory reclamation
//! - [`hp_fifo`] - Hazard pointer protected FIFO
//! - [`hp_stack`] - Hazard pointer protected stack
//!
//! ### Synchronization
//! - [`barrier`] - Execution barriers
//! - [`fifo`] - Lock-free FIFO queues
//! - [`ec`] - Event counts

#![no_std]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![allow(clippy::module_inception)]

extern crate alloc;

// =============================================================================
// Tier 0: Foundation (No Internal Dependencies)
// =============================================================================

/// Compiler compatibility utilities.
///
/// Provides bit manipulation operations (ffs, ctz, popcount) and branch prediction hints.
pub mod cc;

/// Memory allocator abstraction.
///
/// Defines the [`Allocator`](malloc::Allocator) trait for custom memory allocation.
pub mod malloc;

// =============================================================================
// Tier 1: Core Primitives (Depends on Tier 0)
// =============================================================================

/// Atomic primitives and memory barriers.
///
/// Provides atomic operations with various memory orderings.
pub mod pr;

// =============================================================================
// Tier 2: Basic Building Blocks (Depends on Tiers 0-1)
// =============================================================================

/// Exponential backoff for contention management.
pub mod backoff;

/// Lock-free stack.
pub mod stack;

/// BSD-style queue macros.
pub mod queue;

/// Concurrent ring buffer.
pub mod ring;

/// Concurrent bitmap operations.
pub mod bitmap;

/// Sequence counter (seqlock).
pub mod sequence;

/// Phase-fair reader-writer lock.
pub mod pflock;

/// Task-fair reader-writer lock.
pub mod tflock;

/// Big-reader lock.
pub mod brlock;

/// Byte-level reader-writer lock.
pub mod bytelock;

/// Lock cohorting for NUMA.
pub mod cohort;

/// Event counts for blocking synchronization.
pub mod ec;

// =============================================================================
// Tier 3: Extended Primitives (Depends on Tiers 0-2)
// =============================================================================

/// Hardware transactional memory lock elision.
pub mod elide;

/// Concurrently-readable dynamic array.
pub mod array;

/// Hash set (single-writer, many-reader).
pub mod hs;

/// Hash table (key-value pairs).
pub mod ht;

/// Robin-hood hash set.
pub mod rhs;

/// Epoch-based safe memory reclamation.
pub mod epoch;

/// Hazard pointer memory reclamation.
pub mod hp;

/// Reader-writer lock cohorting for NUMA.
pub mod rwcohort;

// =============================================================================
// Tier 4: Composite Locks (Depends on Tiers 0-3)
// =============================================================================

/// Various spinlock implementations.
pub mod spinlock;

/// Reader-writer lock.
pub mod rwlock;

/// Single-writer reader-writer lock.
pub mod swlock;

// =============================================================================
// Tier 5: High-Level Structures (Depends on Tiers 0-4)
// =============================================================================

/// Execution barriers.
pub mod barrier;

/// Lock-free FIFO queues.
pub mod fifo;

/// Hazard pointer protected FIFO.
pub mod hp_fifo;

/// Hazard pointer protected stack.
pub mod hp_stack;

// =============================================================================
// Re-exports for convenience
// =============================================================================

pub use cc::{ctz, ffs, ffsl, ffsll, popcount};
pub use malloc::Allocator;
