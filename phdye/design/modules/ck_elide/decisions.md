# Module: ck_elide — Design Decisions

## Decision: Macro-based lock type generation

**Context:**
Lock elision needs to wrap arbitrary lock types.

**Options Considered:**

1. Function pointers at runtime
   - Pro: Single elision implementation
   - Con: Cannot inline lock predicates

2. Macro-generated specialized wrappers
   - Pro: Inline predicates, optimal code
   - Con: Code generation per lock type

**Decision:** Macro-generated wrappers (option 2)

**Rationale:** Lock predicates (L_P, U_P) must be inlined for the RTM check to work correctly. Transaction checks need to be minimal for low abort rates.

**Rationale Source:** CK_ELIDE_PROTOTYPE macro design

**Consequences:**
- One set of elision functions per lock type
- Predicates fully inlined
- Direct fallback calls

---

## Decision: Skip counter for abort adaptation

**Context:**
Repeated aborts indicate elision may not be worthwhile.

**Options Considered:**

1. Always attempt elision
   - Pro: Simple, no state
   - Con: Wasted cycles on hostile workloads

2. Skip elision after repeated failures
   - Pro: Adapts to workload
   - Con: Needs per-thread state

**Decision:** Skip counter with configurable decay (option 2)

**Rationale:** After repeated aborts, temporarily forfeit elision to avoid wasting cycles. skip_busy, skip_conflict, skip_other configure how many acquisitions to skip.

**Rationale Source:** ck_elide_stat.skip field, config skip_* fields

**Consequences:**
- Per-thread stat structure required
- Automatic adaptation to conflict rates
- Configurable skip durations

---

## Decision: Abort-specific retry strategies

**Context:**
Different abort reasons may benefit from different retry strategies.

**Options Considered:**

1. Uniform retry count
   - Pro: Simple
   - Con: Suboptimal for different abort types

2. Abort-specific retry and skip
   - Pro: Tuned for each abort type
   - Con: More configuration

**Decision:** Abort-specific configuration (option 2)

**Rationale:** Conflict aborts are often transient (retry helps). Explicit aborts (lock busy) benefit from spinning. Capacity/debug aborts are persistent (skip many).

**Rationale Source:** retry_busy, retry_conflict, retry_other fields

**Consequences:**
- Conflicts: retry quickly
- Lock busy: spin then retry
- Capacity/nesting: skip many (USHRT_MAX)

---

## Decision: Spin on lock-busy abort

**Context:**
When lock is held, transaction aborts with explicit LOCK_BUSY.

**Options Considered:**

1. Immediately fallback to lock
   - Pro: Simple
   - Con: Loses elision opportunity

2. Spin waiting for lock release, then retry
   - Pro: May still elide when lock releases
   - Con: Spin time

**Decision:** Spin then retry on lock-busy (option 2)

**Rationale:** If the lock holder releases quickly, spinning allows subsequent elision. hint == CK_ELIDE_HINT_SPIN triggers this behavior.

**Rationale Source:** SPIN hint in _ck_elide_fallback

**Consequences:**
- Short critical sections benefit more
- Long critical sections eventually timeout to fallback

---

## Decision: No fences in RTM path

**Context:**
RTM transactions need memory barriers.

**Options Considered:**

1. Include fences for portability
   - Pro: Works on non-TSO
   - Con: Unnecessary on x86

2. Omit fences (x86-TSO only)
   - Pro: Minimal overhead
   - Con: Not portable

**Decision:** Omit fences (option 2)

**Rationale:** RTM is currently only supported on x86 (TSO). The header explicitly notes fences would be needed for non-TSO TM architectures.

**Rationale Source:** Header comment: "fences have been omitted"

**Consequences:**
- Optimal performance on x86
- Would need updates for future non-TSO TM
