# Module: ck_hp_fifo — Design Decisions

## Decision: Hazard pointers instead of generation counters

**Context:**
MPMC queue needs ABA prevention for correctness.

**Options Considered:**

1. Double-wide CAS with generation counter (ck_fifo_mpmc)
   - Pro: Self-contained
   - Con: Requires DWCAS hardware support

2. Hazard pointers for protection
   - Pro: Single-wide CAS, more portable
   - Con: Requires HP infrastructure

**Decision:** Hazard pointers (option 2)

**Rationale:** Not all platforms support efficient 128-bit CAS. Hazard pointers use standard pointer-width CAS. HP also provides safe memory reclamation as part of the solution.

**Rationale Source:** Uses ck_hp dependency, single-wide CAS

**Consequences:**
- More portable across platforms
- Requires ck_hp initialization
- Integrated memory reclamation

---

## Decision: Two hazard slots per operation

**Context:**
Dequeue needs to protect multiple pointers.

**Options Considered:**

1. Single hazard slot
   - Pro: Less overhead
   - Con: Cannot protect both head and next

2. Two hazard slots
   - Pro: Protects head while reading next
   - Con: More HP overhead

**Decision:** Two slots (option 2)

**Rationale:** During dequeue, must protect head while loading head->next. After loading next, must protect next before CAS. Two slots (CK_HP_FIFO_SLOTS_COUNT = 2) enable this pattern safely.

**Rationale Source:** CK_HP_FIFO_SLOTS_COUNT = 2, dequeue logic

**Consequences:**
- Safe dequeue operation
- Two HP records per thread
- Higher HP overhead than single-slot

---

## Decision: Embedded hazard state in entry

**Context:**
Dequeued entries need reclamation.

**Options Considered:**

1. Separate hazard state
   - Pro: Smaller entries
   - Con: Two allocations per entry

2. Embedded ck_hp_hazard_t in entry
   - Pro: Single allocation
   - Con: Larger entry, potential cache issues

**Decision:** Embedded (option 2)

**Rationale:** ck_hp_fifo_entry includes ck_hp_hazard_t field. Enables ck_hp_free to handle reclamation directly. Comment notes potential cache line bouncing but simpler allocation model wins.

**Rationale Source:** hazard field in ck_hp_fifo_entry, code comment

**Consequences:**
- Single allocation per entry
- Integrated with ck_hp_free
- May cause cache invalidation on retire

---

## Decision: Double-check pattern after HP set

**Context:**
Hazard pointer must be visible before pointer is dereferenced.

**Options Considered:**

1. Set HP, assume protection
   - Pro: Simpler
   - Con: Race if pointer changed

2. Set HP, fence, re-verify pointer
   - Pro: Correct synchronization
   - Con: Extra load

**Decision:** Double-check (option 2)

**Rationale:** After ck_hp_set_fence(record, slot, ptr), code verifies ptr == current value. If changed, HP may not protect new value. Retry loop ensures HP protects current pointer.

**Rationale Source:** Pattern in enqueue/dequeue loops

**Consequences:**
- Correct HP protection
- Extra verification overhead
- Standard HP usage pattern

---

## Decision: Return entry pointer from dequeue

**Context:**
Caller needs reference to dequeued entry for reclamation.

**Options Considered:**

1. Return only value, internal reclamation
   - Pro: Simpler API
   - Con: Cannot batch reclamation

2. Return entry pointer
   - Pro: Caller controls reclamation timing
   - Con: More complex API

**Decision:** Return entry (option 2)

**Rationale:** Dequeue returns ck_hp_fifo_entry pointer. Caller passes to ck_hp_free when safe. Enables batched reclamation and custom policies.

**Rationale Source:** dequeue returns entry pointer

**Consequences:**
- Flexible reclamation
- Caller responsible for ck_hp_free
- Can batch for efficiency

---

## Decision: Try variants for single attempts

**Context:**
Non-blocking single-attempt operations useful for some patterns.

**Options Considered:**

1. Only blocking operations
   - Pro: Simpler
   - Con: No single-attempt option

2. Provide try variants
   - Pro: More flexible
   - Con: More API

**Decision:** Provide try variants (option 2)

**Rationale:** tryenqueue_mpmc and trydequeue_mpmc make single attempt. Return false/NULL on contention. Useful for back-pressure, polling, or combining with other operations.

**Rationale Source:** tryenqueue_mpmc, trydequeue_mpmc functions

**Consequences:**
- More control for caller
- Wait-free single attempts
- Larger API surface
