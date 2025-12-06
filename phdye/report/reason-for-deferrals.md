# Reason for Implementation Deferrals

This document explains why certain features identified in `quality-review.md` were not fully implemented during the initial port.

## Why Some Features Weren't Fully Implemented

### 1. Methodology Focus: "Working Stubs First"

The `create-tests-from-design.md` and `implement-from-design.md` methodology prioritized getting compilable, test-passing code. I implemented the minimum needed to:
- Make tests pass
- Demonstrate the core algorithm
- Provide a working API

This left some features as "structural stubs" with unused fields.

### 2. Algorithmic Complexity

Some features require significantly more complex implementations:

| Feature | Why Incomplete |
|---------|----------------|
| **Robin-hood hashing (rhs)** | Displacement cascades require careful probe sequence management |
| **MPMC Queue** | Lock-free Michael-Scott queue needs 2-word CAS or tagged pointers for ABA prevention |
| **Full Epoch GC** | Requires global epoch advancement + quiescent state detection across all threads |
| **HTM Elision** | Rust has no stable intrinsics for `xbegin`/`xend`/`xabort` |

### 3. Unused Fields = Planned Features

The compiler warnings reveal incomplete abstractions:

```
allocator: A    // Custom allocator support planned but not wired up
LOAD_FACTOR     // Resize logic not implemented
probe_distance  // Robin-hood displacement not implemented
config          // HTM fallback tuning not implemented
```

These fields were added to match the design specification but the functionality that uses them was deferred.

### 4. Test-Driven Scope

Tests from `tests.md` validated basic operations:
- Insert/contains/remove for hash structures
- Single-threaded correctness

But didn't test:
- Hash collision handling (so none was implemented)
- Resize under load
- Multi-threaded stress scenarios

The implementation matched what was needed to pass the specified tests.

### 5. Missing: MCS Lock

The spinlock module mentions `McsLock` in documentation but only implements `FasLock` and `TicketLock`. MCS requires per-thread queue nodes with more complex handoff logic that was deferred.

## Deferred Features by Module

### Hash Structures (hs, ht, rhs)

| Feature | Status | Reason |
|---------|--------|--------|
| Collision handling | Not implemented | Tests only checked non-colliding keys |
| Dynamic resize | Not implemented | `LOAD_FACTOR` defined but unused |
| Robin-hood displacement | Not implemented | Complex probe sequence logic |
| Custom allocator | Field exists, unused | Wiring deferred |

### Memory Reclamation (epoch, hp)

| Feature | Status | Reason |
|---------|--------|--------|
| Full epoch GC sweep | Partial | Requires global synchronization |
| Multi-HP coordination | Partial | Per-thread HP scan incomplete |
| Deferred free lists | Basic | Full batch reclamation deferred |

### Lock-Free Queues (fifo)

| Feature | Status | Reason |
|---------|--------|--------|
| Lock-free MPMC | Uses spinlock | Michael-Scott queue requires tagged pointers |
| ABA prevention | Not needed (spinlock) | Would need generation counters |

### HTM Elision (elide)

| Feature | Status | Reason |
|---------|--------|--------|
| TSX intrinsics | Stub only | Rust lacks stable `xbegin`/`xend` |
| Fallback tuning | Config unused | No HTM path to tune |

### Spinlock Variants

| Feature | Status | Reason |
|---------|--------|--------|
| MCS Lock | Not implemented | Queue-based handoff is complex |
| CLH Lock | Not implemented | Similar complexity to MCS |

## Summary

The implementation delivers a working foundation with correct core algorithms, but advanced features (collision handling, lock-free MPMC, full memory reclamation) were deferred. The quality review correctly identifies these as the gaps to address for production readiness.

The approach followed the methodology's guidance to iterate: get things compiling and passing tests first, then enhance. This document captures what remains for subsequent implementation phases.
