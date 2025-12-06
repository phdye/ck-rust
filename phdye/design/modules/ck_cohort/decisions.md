# Module: ck_cohort — Design Decisions

## Decision: Macro-based lock type generation

**Context:**
Cohort locks need to wrap arbitrary lock types.

**Options Considered:**

1. Function pointers at runtime
   - Pro: Single cohort type
   - Con: Indirect call overhead

2. Macro-generated specialized types
   - Pro: Direct calls, inlining
   - Con: Code generation complexity

**Decision:** Macro-generated types (option 2)

**Rationale:** Lock operations are performance-critical. Direct calls allow inlining and eliminate function pointer overhead.

**Rationale Source:** CK_COHORT_PROTOTYPE macro design

**Consequences:**
- One cohort type per lock type combination
- Direct inlined lock calls
- More compile-time code, less runtime overhead

---

## Decision: Local pass limit for fairness

**Context:**
Unlimited local passing could starve remote nodes.

**Options Considered:**

1. No limit (maximum local throughput)
   - Pro: Best local performance
   - Con: Remote starvation possible

2. Configurable limit
   - Pro: Balance throughput vs fairness
   - Con: User must choose value

**Decision:** Configurable limit (option 2)

**Rationale:** The local_pass_limit parameter lets users tune the fairness/throughput tradeoff. Default of 10 provides reasonable balance.

**Rationale Source:** local_pass_limit field, DEFAULT_LOCAL_PASS_LIMIT = 10

**Consequences:**
- User-tunable fairness
- Default prevents severe starvation
- Higher limits favor local throughput

---

## Decision: Waiting threads counter

**Context:**
Need to know if local pass is worthwhile.

**Options Considered:**

1. Always try local pass
   - Pro: Simple
   - Con: May hold global unnecessarily

2. Track waiting threads
   - Pro: Release global if no local waiters
   - Con: Extra atomic operations

**Decision:** Track waiting threads (option 2)

**Rationale:** If no threads are waiting locally, holding the global lock wastes resources and delays other nodes.

**Rationale Source:** waiting_threads field with inc/dec around local lock

**Consequences:**
- Two extra atomics per lock (inc/dec)
- Global released when no local waiters
- Better overall system throughput
