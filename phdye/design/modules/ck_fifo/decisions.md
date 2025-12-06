# Module: ck_fifo — Design Decisions

## Decision: Stub node design

**Context:**
FIFO queues need to handle empty state.

**Options Considered:**

1. NULL head for empty
   - Pro: No extra node
   - Con: Complex empty handling

2. Stub node always present
   - Pro: Simplified logic, head always valid
   - Con: One extra node

**Decision:** Stub node (option 2)

**Rationale:** Head always points to a stub. head->next is the actual first element. If head->next == NULL, queue is empty. Simplifies concurrent access - no special empty case.

**Rationale Source:** All init functions require stub parameter

**Consequences:**
- Simpler enqueue/dequeue logic
- One node always allocated
- Uniform head handling

---

## Decision: Separate head/tail cachelines for SPSC

**Context:**
SPSC has one producer writing tail, one consumer writing head.

**Options Considered:**

1. Pack head and tail together
   - Pro: Smaller struct
   - Con: False sharing between producer/consumer

2. Cacheline-separated head and tail
   - Pro: No false sharing
   - Con: Larger struct

**Decision:** Cacheline separation (option 2)

**Rationale:** ck_fifo_spsc uses explicit padding between head and tail sections. Producer only touches tail cacheline. Consumer only touches head cacheline. Eliminates false sharing.

**Rationale Source:** pad[] in ck_fifo_spsc struct

**Consequences:**
- Better SPSC performance
- Larger memory footprint
- Independent head/tail operations

---

## Decision: Double-wide CAS for MPMC ABA prevention

**Context:**
MPMC queue suffers from ABA problem on pointer comparisons.

**Options Considered:**

1. Single-wide CAS with epoch/SMR
   - Pro: Standard CAS
   - Con: Requires external reclamation

2. Double-wide CAS with generation counter
   - Pro: Self-contained ABA prevention
   - Con: Requires DWCAS support

**Decision:** Double-wide CAS (option 2)

**Rationale:** ck_fifo_mpmc_pointer packs pointer + generation in 16 bytes. ck_pr_cas_ptr_2 performs atomic 128-bit CAS. Generation increments prevent ABA - same pointer with different generation fails CAS.

**Rationale Source:** ck_fifo_mpmc_pointer struct, CK_F_PR_CAS_PTR_2 requirement

**Consequences:**
- ABA prevention without external SMR
- Requires double-wide CAS support
- 16-byte aligned pointers

---

## Decision: Michael-Scott algorithm for MPMC

**Context:**
Need lock-free MPMC queue algorithm.

**Options Considered:**

1. Lock-based queue
   - Pro: Simple
   - Con: Not lock-free

2. Michael-Scott queue
   - Pro: Well-known, lock-free
   - Con: More complex

**Decision:** Michael-Scott (option 2)

**Rationale:** Classic lock-free queue from "Simple, Fast, and Practical Non-Blocking and Blocking Concurrent Queue Algorithms" (1996). Uses helping mechanism - enqueuers advance lagging tail. Widely validated.

**Rationale Source:** Algorithm structure in enqueue/dequeue

**Consequences:**
- Lock-free progress guarantee
- Helping for liveness
- Well-understood correctness

---

## Decision: Built-in recycling for SPSC

**Context:**
SPSC needs efficient node management.

**Options Considered:**

1. External allocation only
   - Pro: Simple
   - Con: Allocation overhead

2. Built-in recycling
   - Pro: Zero allocation after warmup
   - Con: More complex

**Decision:** Built-in recycling (option 2)

**Rationale:** ck_fifo_spsc_recycle uses head_snapshot and garbage pointers. Dequeued nodes between garbage and head_snapshot can be reused. Based on Vyukov's technique for bounded allocation.

**Rationale Source:** recycle function, head_snapshot and garbage fields

**Consequences:**
- Efficient node reuse
- Steady-state allocation-free
- Producer can recycle consumer's nodes

---

## Decision: Try variants for non-blocking attempts

**Context:**
Applications may want non-blocking single attempts.

**Options Considered:**

1. Only blocking operations
   - Pro: Simpler API
   - Con: No single-attempt option

2. Provide try variants
   - Pro: Non-blocking single attempts
   - Con: More API surface

**Decision:** Provide try variants (option 2)

**Rationale:** tryenqueue and trydequeue make single CAS attempt. Return false on contention without retry. Useful for back-pressure or polling patterns.

**Rationale Source:** tryenqueue/trydequeue functions

**Consequences:**
- More flexible API
- Non-blocking option available
- Single-attempt semantics
