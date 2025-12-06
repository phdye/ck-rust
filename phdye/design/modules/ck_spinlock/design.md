# Module: ck_spinlock

## Overview

The ck_spinlock module provides a collection of spinlock implementations with different performance characteristics. Each variant offers different trade-offs between fairness, scalability, and fast-path latency. The default spinlock type aliases to fetch-and-store (FAS), which showed lowest latency on tested platforms.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_backoff | Internal | Exponential backoff |
| ck_cc | Internal | Inline hints |
| ck_elide | Internal | Lock elision support |
| ck_pr | Internal | Atomic operations, fences |
| ck_stdbool | External | bool type |

## Spinlock Variants

### ck_spinlock_fas (Fetch-And-Store)

**Description:** Test-and-set lock using fetch-and-store.

**Structure:**
- value: unsigned int (locked state)

**Algorithm:**
- Lock: FAS(value, true), spin on load while true
- Unlock: Store false

**Properties:**
- Fastest fast-path on x86
- Not fair (possible starvation)
- Size: 4 bytes

### ck_spinlock_cas (Compare-And-Swap)

**Description:** Similar to FAS but uses CAS.

**Properties:**
- Slightly higher latency than FAS
- Not fair
- Compatible with more architectures

### ck_spinlock_ticket

**Description:** FIFO-fair ticket lock.

**Structure (portable):**
- next: unsigned int (next ticket number)
- position: unsigned int (current serving)

**Structure (optimized):**
- value: 32/64-bit packed (next in high, position in low)

**Algorithm:**
- Lock: Atomically fetch-and-add next, spin until position == ticket
- Unlock: Increment position

**Properties:**
- FIFO fairness
- Higher latency than FAS
- Proportional backoff variant (lock_pb)

### ck_spinlock_mcs (Mellor-Crummey Scott)

**Description:** Queue-based scalable lock.

**Structure:**
- queue: Pointer to tail node
- node (per-thread): locked flag + next pointer

**Algorithm:**
- Lock: Swap into queue, spin on local node's locked flag
- Unlock: Notify next node or CAS queue to NULL

**Properties:**
- Local spinning (cache-friendly)
- FIFO fairness
- Requires per-thread node allocation

### ck_spinlock_clh (Craig, Landin, Hagersten)

**Description:** Queue-based lock spinning on predecessor.

**Properties:**
- Local spinning
- FIFO fairness
- Node recycled via swap

### ck_spinlock_hclh (Hierarchical CLH)

**Description:** NUMA-aware hierarchical CLH lock.

**Properties:**
- Local and global queues
- NUMA-friendly

### ck_spinlock_anderson

**Description:** Array-based queue lock.

**Properties:**
- FIFO fairness
- Fixed-size array
- Local spinning

### ck_spinlock_dec (Decrement)

**Description:** Counter-based lock.

**Properties:**
- Simple implementation
- Higher contention

## Default Lock Type

The module provides generic aliases:
- `ck_spinlock_t` → `ck_spinlock_fas_t`
- `ck_spinlock_lock()` → `ck_spinlock_fas_lock()`

Rationale: FAS proved lowest latency on x86, x86_64, PPC64, SPARC64.

## Concurrency

**Thread Safety:** All variants fully thread-safe.

**Progress Guarantee:**
- FAS, CAS, DEC: Not fair (no progress guarantee per thread)
- Ticket, MCS, CLH, HCLH, Anderson: FIFO fair

**Memory Ordering:**
- lock: Acquire semantics (fence_lock after acquisition)
- unlock: Release semantics (fence_unlock before release)

## Platform Considerations

- All variants support lock elision (CK_ELIDE_PROTOTYPE)
- Exponential backoff variants (_eb, _pb) available
- Some variants require per-thread context (MCS, CLH)
- Ticket lock has platform-specific optimized path
