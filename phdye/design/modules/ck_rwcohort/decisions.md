# Module: ck_rwcohort — Design Decisions

## Decision: Three fairness variants

**Context:**
Different workloads benefit from different reader/writer priority.

**Options Considered:**

1. Single balanced implementation
   - Pro: Simple
   - Con: Not optimal for biased workloads

2. Multiple variants with different fairness
   - Pro: User selects best for workload
   - Con: More code

**Decision:** Three variants (option 2)

**Rationale:** Writer-preference (WP) for write-heavy, Reader-preference (RP) for read-heavy, Neutral for balanced. User selects based on workload characteristics.

**Rationale Source:** WP, RP, NEUTRAL prototype macros

**Consequences:**
- WP: writers not starved
- RP: readers not starved
- Neutral: fair but lower concurrency

---

## Decision: Barrier mechanism for starvation prevention

**Context:**
Preference mechanisms can cause starvation.

**Options Considered:**

1. Pure preference (potential starvation)
   - Pro: Simple
   - Con: Indefinite starvation

2. Barrier after wait_limit
   - Pro: Bounds starvation
   - Con: More complex

**Decision:** Barrier mechanism (option 2)

**Rationale:** After wait_limit iterations, waiting thread raises barrier (write_barrier or read_barrier). Barrier blocks new acquisitions of the non-preferred type, allowing waiters to proceed.

**Rationale Source:** write_barrier, read_barrier fields, wait_limit checks

**Consequences:**
- Starvation bounded
- wait_limit tunable
- Barrier raised/lowered atomically

---

## Decision: Build on ck_cohort

**Context:**
Need NUMA-aware reader-writer lock.

**Options Considered:**

1. Standalone RW lock implementation
   - Pro: Self-contained
   - Con: Duplicates cohort logic

2. Layer on ck_cohort
   - Pro: Reuse NUMA optimization
   - Con: Dependency

**Decision:** Build on ck_cohort (option 2)

**Rationale:** ck_cohort already provides NUMA-aware mutual exclusion. Reader-writer semantics layer cleanly on top with reader counters and barriers.

**Rationale Source:** CK_COHORT_LOCK/UNLOCK used internally

**Consequences:**
- Inherits NUMA locality
- Inherits cohort lock type flexibility
- Requires cohort to be generated first

---

## Decision: Neutral uses cohort for read_lock

**Context:**
Neutral variant needs fair ordering.

**Options Considered:**

1. Complex fairness tracking
   - Pro: High reader concurrency
   - Con: Complex

2. Acquire cohort for read, increment, release
   - Pro: Simple, fair
   - Con: Serializes read acquisition

**Decision:** Cohort for read (option 2)

**Rationale:** Neutral read_lock acquires cohort, increments counter, releases cohort. This serializes reader entry but provides fair ordering with writers.

**Rationale Source:** Neutral read_lock algorithm

**Consequences:**
- Simple implementation
- Fair ordering
- Lower reader concurrency than WP/RP

---

## Decision: Separate read_unlock takes only rwcohort

**Context:**
API consistency for read_unlock.

**Options Considered:**

1. Same parameters as read_lock
   - Pro: Consistent
   - Con: Unused parameters

2. Minimal parameters
   - Pro: Clear what's needed
   - Con: Inconsistent with write_unlock

**Decision:** Minimal parameters (option 2)

**Rationale:** read_unlock only needs to decrement read_counter. No cohort interaction needed. Cleaner API reflects actual requirements.

**Rationale Source:** read_unlock signature takes only rwcohort

**Consequences:**
- Simpler call site
- Clear that cohort not used
- Different from write_unlock signature
