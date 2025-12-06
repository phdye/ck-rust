# Module: ck_array — Design Decisions

## Decision: Copy-on-write for mutations

**Context:**
Need to allow concurrent reads during modifications.

**Options Considered:**

1. In-place mutation with locks
   - Pro: Memory efficient
   - Con: Readers blocked during mutation

2. Copy-on-write with atomic swap
   - Pro: Readers never blocked
   - Con: Higher memory usage during mutations

**Decision:** Copy-on-write (option 2)

**Rationale:** Read performance is prioritized. Writers create a transaction copy, modify it, then atomically swap the active pointer. Readers see a consistent snapshot.

**Rationale Source:** transaction field, commit swaps active pointer

**Consequences:**
- Readers always see consistent state
- Writers allocate during modifications
- Old array needs safe reclamation

---

## Decision: Deferred commit model

**Context:**
Multiple modifications may need to be batched.

**Options Considered:**

1. Auto-commit on each modification
   - Pro: Simple API
   - Con: Many allocations for batched changes

2. Explicit commit
   - Pro: Batch modifications efficiently
   - Con: More complex API

**Decision:** Explicit commit (option 2)

**Rationale:** Multiple put/remove operations can share a single transaction allocation. Only one atomic swap needed for batch.

**Rationale Source:** Separate put/remove vs commit functions

**Consequences:**
- Writer batches modifications
- Single commit publishes all changes
- Must remember to commit

---

## Decision: Swap-with-last for removal

**Context:**
Array doesn't need to preserve order.

**Options Considered:**

1. Shift elements on removal
   - Pro: Preserves order
   - Con: O(n) removal

2. Swap with last element
   - Pro: O(1) removal
   - Con: Order not preserved

**Decision:** Swap with last (option 2)

**Rationale:** For many use cases (e.g., callback lists), order doesn't matter. Swap-with-last gives O(1) removal at the cost of ordering.

**Rationale Source:** Common concurrent array implementation pattern

**Consequences:**
- Fast removal
- Iteration order may change after removal
- Not suitable for ordered sequences

---

## Decision: SPMC only, MPMC unsupported

**Context:**
Could support multiple concurrent writers.

**Options Considered:**

1. MPMC with synchronization
   - Pro: More flexible
   - Con: Complex, overhead

2. SPMC only
   - Pro: Simple, efficient
   - Con: Single writer constraint

**Decision:** SPMC only (option 2)

**Rationale:** Most use cases have a designated writer. SPMC avoids synchronization overhead in the write path.

**Rationale Source:** CK_ARRAY_MODE_MPMC marked as unsupported

**Consequences:**
- Single writer required
- No writer synchronization
- Simple implementation

---

## Decision: Flexible array member for values

**Context:**
Need variable-size array storage.

**Options Considered:**

1. Separate allocation for values
   - Pro: Standard C
   - Con: Extra indirection, allocation

2. Flexible array member
   - Pro: Single allocation, cache locality
   - Con: C99 feature

**Decision:** Flexible array member (option 2)

**Rationale:** Single allocation for header + values array improves cache locality and reduces allocation overhead.

**Rationale Source:** void *values[] in struct _ck_array

**Consequences:**
- Single allocation per array
- Good cache performance
- Requires C99
