# Module: ck_hp_stack — Design Decisions

## Decision: Wrapper over ck_stack

**Context:**
Need HP-protected stack operations.

**Options Considered:**

1. Standalone implementation
   - Pro: Optimized for HP
   - Con: Code duplication

2. Wrapper over ck_stack
   - Pro: Reuses existing code
   - Con: Possible overhead

**Decision:** Wrapper (option 2)

**Rationale:** Push operations delegate directly to ck_stack_push_upmc and ck_stack_trypush_upmc. Pop adds HP protection around existing CAS logic. Avoids duplicating well-tested stack code.

**Rationale Source:** Push functions directly call ck_stack_*_upmc

**Consequences:**
- No code duplication
- Reuses ck_stack correctness
- Minimal overhead

---

## Decision: Single hazard slot for pop

**Context:**
Pop needs protection while reading entry->next.

**Options Considered:**

1. No hazard pointer (rely on external SMR)
   - Pro: Simpler
   - Con: Unsafe without external coordination

2. Single hazard slot
   - Pro: Protects head during next read
   - Con: HP overhead

**Decision:** Single slot (option 2)

**Rationale:** CK_HP_STACK_SLOTS_COUNT = 1. Pop only needs to protect the head entry while reading head->next. Unlike FIFO dequeue, no second pointer needs protection (next isn't dereferenced further).

**Rationale Source:** CK_HP_STACK_SLOTS_COUNT = 1

**Consequences:**
- Safe pop operation
- Lower HP overhead than FIFO
- Simple HP record requirement

---

## Decision: Push doesn't use hazard pointers

**Context:**
Push operation doesn't read memory that might be freed.

**Options Considered:**

1. Use HP for consistency
   - Pro: Uniform API
   - Con: Unnecessary overhead

2. Skip HP for push
   - Pro: No overhead
   - Con: Asymmetric API

**Decision:** Skip HP for push (option 2)

**Rationale:** Push only writes to entry and CASes head. Never reads from potentially-freed memory. No HP needed - delegates directly to ck_stack functions without HP record parameter.

**Rationale Source:** push functions don't take ck_hp_record_t

**Consequences:**
- Optimal push performance
- Asymmetric API (push vs pop)
- Correct without HP for writes

---

## Decision: cas_ptr_value for efficient retry

**Context:**
Pop loop needs current head after failed CAS.

**Options Considered:**

1. Separate load after CAS failure
   - Pro: Simple
   - Con: Extra memory access

2. Use CAS that returns current value
   - Pro: Efficient single operation
   - Con: Requires special primitive

**Decision:** Use cas_ptr_value (option 2)

**Rationale:** ck_pr_cas_ptr_value returns both success/failure and current value. On failure, loop uses returned value as new head instead of extra load. Reduces memory traffic in contended case.

**Rationale Source:** ck_pr_cas_ptr_value in pop loop

**Consequences:**
- More efficient retry
- Fewer memory operations
- Tighter retry loop

---

## Decision: Clear HP on trypop failure

**Context:**
trypop may fail, leaving hazard pointer set.

**Options Considered:**

1. Leave HP set
   - Pro: Simpler
   - Con: Prevents reclamation

2. Clear HP on failure
   - Pro: Correct reclamation
   - Con: Extra operation

**Decision:** Clear HP on failure (option 2)

**Rationale:** On early return (goto leave), code calls ck_hp_set(record, 0, NULL). Ensures failed attempts don't block reclamation of entries that were only transiently referenced.

**Rationale Source:** goto leave path clears HP

**Consequences:**
- Correct reclamation behavior
- No dangling HP references
- Slightly more code
