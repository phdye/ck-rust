# Module: ck_ring — Specification

## Operations

### ck_ring_init

**Signature:**
```
ck_ring_init(ring: Pointer to ck_ring, size: unsigned int) → void
```

**Preconditions:**
- ring must not be NULL [INFERRED]
- size must be power of 2 [SPECIFIED]
- size must be > 0 [INFERRED]

**Postconditions:**
- ring->size = size [SPECIFIED]
- ring->mask = size - 1 [SPECIFIED]
- ring->c_head = ring->p_tail = ring->p_head = 0 [SPECIFIED]
- Ring is empty [SPECIFIED]

**Invariants Preserved:**
- Power-of-2 size invariant [SPECIFIED]

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| Non-power-of-2 size | Incorrect masking, undefined behavior | SPECIFIED |

**Concurrency:**
- Thread Safety: Not safe during concurrent operations [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_ring_size

**Signature:**
```
ck_ring_size(ring: Pointer to const ck_ring) → unsigned int
```

**Preconditions:**
- ring must not be NULL [INFERRED]

**Postconditions:**
- Returns number of elements currently in ring [SPECIFIED]
- Result accurate at time of call, may be stale immediately [OBSERVED]

**Concurrency:**
- Thread Safety: Safe with concurrent operations [SPECIFIED]
- Memory Ordering: Atomic loads [OBSERVED]
- Progress Guarantee: wait-free [SPECIFIED]

---

### ck_ring_capacity

**Signature:**
```
ck_ring_capacity(ring: Pointer to const ck_ring) → unsigned int
```

**Preconditions:**
- ring must not be NULL [INFERRED]

**Postconditions:**
- Returns ring->size [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe (size is immutable after init) [OBSERVED]
- Progress Guarantee: wait-free [SPECIFIED]

---

### ck_ring_valid

**Signature:**
```
ck_ring_valid(ring: Pointer to const ck_ring) → bool
```

**Preconditions:**
- ring must not be NULL [INFERRED]
- No concurrent operations in progress [SPECIFIED]

**Postconditions:**
- Returns true if ring is in consistent state [SPECIFIED]
- Checks: size is power of 2, c_head <= p_head, p_head - c_head < size [OBSERVED]

**Concurrency:**
- Thread Safety: Only call when quiescent [SPECIFIED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_ring_repair

**Signature:**
```
ck_ring_repair(ring: Pointer to ck_ring) → bool
```

**Preconditions:**
- No concurrent operations [SPECIFIED]

**Postconditions:**
- IF p_tail != p_head: p_tail = p_head, return true [SPECIFIED]
- ELSE: return false [SPECIFIED]

**Note:** For persistent storage recovery after crash during enqueue.

**Concurrency:**
- Thread Safety: Only call when quiescent [SPECIFIED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_ring_enqueue_spsc

**Signature:**
```
ck_ring_enqueue_spsc(ring, buffer, entry) → bool
```

**Preconditions:**
- Single producer calling [SPECIFIED]
- Single consumer may be running concurrently [SPECIFIED]

**Postconditions:**
- IF ring was not full: entry enqueued, return true [SPECIFIED]
- IF ring was full: return false, ring unchanged [SPECIFIED]

**Invariants Preserved:**
- FIFO order [SPECIFIED]
- p_tail incremented by 1 on success [OBSERVED]

**Concurrency:**
- Thread Safety: Safe with single producer, single consumer [SPECIFIED]
- Memory Ordering: Release semantics [OBSERVED]
- Progress Guarantee: wait-free [SPECIFIED]

---

### ck_ring_dequeue_spsc

**Signature:**
```
ck_ring_dequeue_spsc(ring, buffer, data) → bool
```

**Preconditions:**
- Single consumer calling [SPECIFIED]
- Single producer may be running concurrently [SPECIFIED]

**Postconditions:**
- IF ring was not empty: oldest entry returned, return true [SPECIFIED]
- IF ring was empty: return false, data unchanged [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe with single producer, single consumer [SPECIFIED]
- Memory Ordering: Acquire semantics [OBSERVED]
- Progress Guarantee: wait-free [SPECIFIED]

---

### ck_ring_enqueue_mpmc

**Signature:**
```
ck_ring_enqueue_mpmc(ring, buffer, entry) → bool
```

**Preconditions:**
- Any number of producers may call concurrently [SPECIFIED]
- Any number of consumers may be running [SPECIFIED]

**Postconditions:**
- IF ring was not full: entry enqueued, return true [SPECIFIED]
- IF ring was full: return false [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe with multiple producers and consumers [SPECIFIED]
- Memory Ordering: Sequentially consistent [OBSERVED]
- Progress Guarantee: lock-free (system-wide progress) [SPECIFIED]

---

### ck_ring_dequeue_mpmc

**Signature:**
```
ck_ring_dequeue_mpmc(ring, buffer, data) → bool
```

**Preconditions:**
- Any number of consumers may call concurrently [SPECIFIED]

**Postconditions:**
- IF ring was not empty: one entry dequeued, return true [SPECIFIED]
- IF ring was empty: return false [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe with multiple consumers [SPECIFIED]
- Memory Ordering: Acquire semantics [OBSERVED]
- Progress Guarantee: lock-free [SPECIFIED]

---

### ck_ring_trydequeue_mpmc

**Signature:**
```
ck_ring_trydequeue_mpmc(ring, buffer, data) → bool
```

**Preconditions:**
- Same as dequeue_mpmc

**Postconditions:**
- IF ring not empty AND CAS succeeds: dequeue and return true [SPECIFIED]
- Otherwise: return false [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: wait-free (single attempt) [SPECIFIED]

---

### ck_ring_enqueue_reserve_* / ck_ring_enqueue_commit_*

**Signature:**
```
ck_ring_enqueue_reserve_{spsc,spmc,mpsc,mpmc}(ring, buffer, [ticket]) → Pointer
ck_ring_enqueue_commit_{spsc,spmc,mpsc,mpmc}(ring, [ticket]) → void
```

**Preconditions:**
- reserve: ring not full
- commit: must follow successful reserve

**Postconditions:**
- reserve: returns pointer to slot for writing, slot reserved [SPECIFIED]
- commit: slot becomes visible to consumers [SPECIFIED]

**Invariants Preserved:**
- Order of commits matches order of reserves (MP variants) [SPECIFIED]

**Concurrency:**
- MP commit waits for predecessors [OBSERVED]

---

## Data Structure Invariants

### ck_ring

- size is power of 2 [SPECIFIED]
- mask == size - 1 [SPECIFIED]
- c_head <= p_tail <= p_head [OBSERVED]
- p_head - c_head <= size [SPECIFIED]
- p_tail - c_head <= size (elements visible to consumers) [OBSERVED]

## Module-Level Invariants

- FIFO order preserved [SPECIFIED]
- No element lost or duplicated [SPECIFIED]
- Bounded capacity [SPECIFIED]

## Safety Properties

**No Data Loss:** Every successfully enqueued element is eventually dequeueable exactly once. [SPECIFIED]

**No Overflow:** Enqueue fails gracefully when ring is full. [SPECIFIED]

**Capacity Bound:** At most size-1 elements can be in the ring (one slot reserved for full/empty disambiguation). [OBSERVED]

## Liveness Properties

**Lock-Freedom (MPMC):** Under contention, at least one thread makes progress. [SPECIFIED]

**Wait-Freedom (SPSC):** Operations complete in bounded time. [SPECIFIED]

## Behavioral Ambiguities

### Full ring detection

**Observed Behavior:** Ring can hold at most size-1 elements

**Intent:** SPECIFIED - One slot reserved to distinguish full from empty

**Recommendation:** Document effective capacity as size-1.

### MP enqueue ordering

**Observed Behavior:** MP enqueue waits for predecessors to commit

**Intent:** SPECIFIED - Maintains FIFO order with multiple producers

**Recommendation:** Document that MP enqueue may block briefly waiting for prior enqueues.

## Discrepancies

No discrepancies detected between sources.
