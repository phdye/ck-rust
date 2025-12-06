# Concurrency Kit for Rust

A Rust port of [Concurrency Kit (CK)](https://github.com/concurrencykit/ck) - high-performance concurrency primitives for lock-free data structures and synchronization.

## Overview

This library provides:

- **Lock-free data structures** - Stacks, queues, FIFOs, and hash tables
- **Spinlock implementations** - FAS, CAS, ticket, and MCS variants
- **Reader-writer locks** - Phase-fair, task-fair, big-reader, byte-level
- **Memory reclamation** - Epoch-based and hazard pointer schemes
- **Atomic primitives** - Cross-platform atomic operations and memory barriers

## Features

- `#![no_std]` compatible with `alloc`
- Safe abstractions over unsafe concurrent operations
- Comprehensive test suite (147 unit tests)
- Based on well-researched algorithms (Treiber stack, MCS lock, etc.)

## Modules

### Foundation
- `cc` - Compiler compatibility (bit operations, branch hints)
- `malloc` - Memory allocator abstraction
- `pr` - Atomic primitives and memory barriers

### Synchronization Primitives
- `spinlock` - Various spinlock implementations
- `rwlock` - Reader-writer lock
- `sequence` - Sequence lock (seqlock)
- `pflock` - Phase-fair reader-writer lock
- `tflock` - Task-fair reader-writer lock
- `brlock` - Big-reader lock
- `bytelock` - Byte-level reader-writer lock
- `cohort` - NUMA-aware lock cohorting
- `rwcohort` - Reader-writer lock cohorting
- `elide` - Hardware transactional memory lock elision
- `barrier` - Thread execution barriers

### Data Structures
- `stack` - Lock-free Treiber stack
- `fifo` - SPSC and MPMC FIFO queues
- `ring` - Single-producer/single-consumer ring buffer
- `queue` - BSD-style queue macros
- `array` - Concurrently-readable dynamic array
- `bitmap` - Concurrent bitmap operations
- `hs` - Hash set (single-writer, many-reader)
- `ht` - Hash table
- `rhs` - Robin-hood hash set

### Memory Reclamation
- `epoch` - Epoch-based safe memory reclamation
- `hp` - Hazard pointer memory reclamation
- `hp_fifo` - HP-protected FIFO
- `hp_stack` - HP-protected stack

### Utilities
- `backoff` - Exponential backoff for contention
- `ec` - Event counts

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
concurrencykit = "0.1"
```

### Example: Lock-free Stack

```rust
use concurrencykit::stack::{Stack, StackEntry};

let stack: Stack<i32> = Stack::new();
let entry = Box::into_raw(Box::new(StackEntry::new(42)));

unsafe {
    stack.push(entry);

    if let Some(popped) = stack.pop() {
        println!("Popped: {}", (*popped).data());
        drop(Box::from_raw(popped));
    }
}
```

### Example: Spinlock

```rust
use concurrencykit::spinlock::FasLock;

let lock = FasLock::new(0);

{
    let mut guard = lock.lock();
    *guard += 1;
}
```

### Example: Atomic Primitives

```rust
use concurrencykit::pr::{AtomicU64, fence_acquire, fence_release};
use core::sync::atomic::Ordering;

let counter = AtomicU64::new(0);
counter.fetch_add(1, Ordering::SeqCst);
fence_release();
```

## Building

```bash
cargo build
```

## Testing

```bash
cargo test
```

## Documentation

Generate API documentation:

```bash
cargo doc --open
```

## Safety

This library uses `unsafe` code for lock-free operations. Users must ensure:

- Proper memory reclamation for popped/removed elements
- Single-threaded access for single-producer variants
- Correct usage of atomic orderings

## License

See LICENSE file.

## References

- [Concurrency Kit](https://github.com/concurrencykit/ck)
- Treiber, R. K. (1986). "Systems Programming: Coping with Parallelism"
- Michael, M. M. (2004). "Hazard Pointers: Safe Memory Reclamation for Lock-Free Objects"
- Fraser, K. (2004). "Practical Lock-Freedom"
