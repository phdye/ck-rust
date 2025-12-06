# Module: ck_rhs — Design Decisions

## Decision: Robin Hood hashing over standard linear probing

**Context:**
Need efficient hash set with good worst-case behavior.

**Options Considered:**

1. Standard linear probing
   - Pro: Simple
   - Con: Long probe chains

2. Robin Hood hashing
   - Pro: Bounded variance in probe length
   - Con: More complex insertion

**Decision:** Robin Hood hashing (option 2)

**Rationale:** Robin Hood "steals" slots from rich entries (short probe distance) for poor entries (long probe distance). This bounds the maximum probe length and improves lookup consistency.

**Rationale Source:** Implementation follows Robin Hood algorithm

**Consequences:**
- More predictable lookup times
- Slightly more complex insertion
- Better cache behavior

---

## Decision: Configurable load factor

**Context:**
Trade-off between memory usage and probe length.

**Options Considered:**

1. Fixed load factor
   - Pro: Simple
   - Con: Not optimal for all workloads

2. Configurable load factor
   - Pro: Tunable
   - Con: User must choose

**Decision:** Configurable load factor (option 2)

**Rationale:** ck_rhs_set_load_factor allows tuning. Higher load factors use less memory but increase probe lengths. Lower factors waste memory but have faster lookups.

**Rationale Source:** load_factor field, ck_rhs_set_load_factor

**Consequences:**
- User controls memory/speed trade-off
- Can adjust at runtime
- Auto-grow triggered by load factor

---

## Decision: Read-mostly mode

**Context:**
Some workloads are heavily read-biased.

**Options Considered:**

1. Single balanced mode
   - Pro: Simple
   - Con: Not optimal for read-heavy

2. CK_RHS_MODE_READ_MOSTLY flag
   - Pro: Optimized get path
   - Con: Slightly slower writes

**Decision:** Read-mostly mode (option 2)

**Rationale:** MODE_READ_MOSTLY optimizes get at the expense of put/delete. Useful for lookup-heavy workloads like caches.

**Rationale Source:** CK_RHS_MODE_READ_MOSTLY flag

**Consequences:**
- get optimized when flag set
- May use tombstones instead of backward shift
- User selects based on workload

---

## Decision: Separate put and put_unique

**Context:**
Same as ck_hs - uniqueness check may be redundant.

**Options Considered:**

1. Single put function
   - Pro: Simple API
   - Con: Redundant checks

2. Separate functions
   - Pro: Skip check when known unique
   - Con: More API

**Decision:** Separate functions (option 2)

**Rationale:** When caller guarantees uniqueness (e.g., during rebuild), put_unique skips the duplicate check for better performance.

**Rationale Source:** ck_rhs_put vs ck_rhs_put_unique

**Consequences:**
- put: full duplicate check
- put_unique: no check
- Faster bulk operations

---

## Decision: Probe distance termination

**Context:**
How to know when to stop probing during lookup.

**Options Considered:**

1. Probe until empty slot
   - Pro: Simple
   - Con: May scan past where key could be

2. Probe distance comparison
   - Pro: Early termination
   - Con: Track distances

**Decision:** Probe distance comparison (option 2)

**Rationale:** Robin Hood invariant ensures if our probe distance exceeds the entry's probe distance, the key cannot be further. This enables early termination.

**Rationale Source:** Robin Hood algorithm property

**Consequences:**
- Faster negative lookups
- Must track probe distances
- Relies on Robin Hood invariant
