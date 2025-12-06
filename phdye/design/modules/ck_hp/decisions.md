# Module: ck_hp — Design Decisions

## Decision: Fixed degree per domain

**Context:**
Need to know how many hazard pointers each thread can use.

**Options Considered:**

1. Dynamic degree per thread
   - Pro: Flexible
   - Con: Complex scan, variable memory

2. Fixed degree at init
   - Pro: Simple scan, predictable memory
   - Con: Must know max upfront

**Decision:** Fixed degree at init (option 2)

**Rationale:** Lock-free algorithms typically have a known maximum number of concurrent pointer accesses. Fixed degree simplifies the scan algorithm and memory allocation.

**Rationale Source:** degree parameter in ck_hp_init

**Consequences:**
- Degree fixed for lifetime
- All records have same array size
- Simple iteration during scan

---

## Decision: Threshold-triggered reclamation

**Context:**
When to scan and reclaim pending objects?

**Options Considered:**

1. Immediate scan on each retire
   - Pro: Low pending count
   - Con: Expensive, frequent scans

2. Batch on threshold
   - Pro: Amortized scan cost
   - Con: Higher pending count

**Decision:** Threshold-triggered (option 2)

**Rationale:** Scanning all hazard pointers is O(n_subscribers * degree). Batching reclamation amortizes this cost over many retires.

**Rationale Source:** threshold parameter, check in ck_hp_free

**Consequences:**
- Configurable threshold
- Memory bounded by threshold * threads
- Efficient amortized cost

---

## Decision: Local scan cache

**Context:**
Building set of active hazard pointers for reclaim.

**Options Considered:**

1. Hash set for lookup
   - Pro: O(1) lookup
   - Con: Complex, allocation

2. Sorted array for binary search
   - Pro: O(log n) lookup
   - Con: Sorting overhead

3. Unsorted cache array
   - Pro: Simple, no allocation
   - Con: O(n) lookup

**Decision:** Unsorted cache array (option 3)

**Rationale:** CK_HP_CACHE provides a fixed-size local cache. For typical degree and subscriber counts, linear scan is fast enough and avoids dynamic allocation.

**Rationale Source:** cache[CK_HP_CACHE] in ck_hp_record

**Consequences:**
- No allocation during reclaim
- Simple implementation
- CK_HP_CACHE bounds scan size

---

## Decision: Two-pointer hazard entry

**Context:**
Need to track retired object and its destructor context.

**Options Considered:**

1. Single pointer (destructor from global)
   - Pro: Smaller
   - Con: Less flexible

2. Pointer + data fields
   - Pro: Per-object context
   - Con: Larger hazard entry

**Decision:** Pointer + data (option 2)

**Rationale:** Different objects may need different context for destruction. The data field passed to destructor enables flexible cleanup.

**Rationale Source:** pointer and data fields in ck_hp_hazard

**Consequences:**
- Per-object destructor context
- Larger hazard entries
- Flexible cleanup patterns

---

## Decision: Record recycling

**Context:**
Threads may exit and rejoin.

**Options Considered:**

1. One record per thread lifetime
   - Pro: Simple
   - Con: Memory growth

2. Recycle unregistered records
   - Pro: Bounded memory
   - Con: More complex

**Decision:** Record recycling (option 2)

**Rationale:** Same as epoch-based reclamation. Prevents unbounded growth in dynamic thread pools.

**Rationale Source:** ck_hp_recycle function, n_free counter

**Consequences:**
- Records can be reused
- Memory bounded by peak threads
- recycle scans for FREE records

---

## Decision: Store-load ordering via set_fence

**Context:**
Hazard pointer must be visible before reading protected data.

**Options Considered:**

1. Always fence
   - Pro: Simple
   - Con: Overhead when not needed

2. Separate set and set_fence
   - Pro: Choose based on need
   - Con: User must know when to fence

**Decision:** Separate functions (option 2)

**Rationale:** Some uses can batch multiple sets before fencing. set_fence uses TSO-optimized fetch-and-store on x86.

**Rationale Source:** ck_hp_set vs ck_hp_set_fence

**Consequences:**
- User controls fence placement
- TSO optimization possible
- Correct ordering requires set_fence
