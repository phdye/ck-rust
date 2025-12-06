# Module: ck_epoch — Design Decisions

## Decision: Per-record pending lists

**Context:**
Deferred callbacks need storage until safe to dispatch.

**Options Considered:**

1. Global pending list
   - Pro: Simple
   - Con: Contention on list

2. Per-record pending lists
   - Pro: No contention on defer
   - Con: More memory, distributed dispatch

**Decision:** Per-record pending lists (option 2)

**Rationale:** Each thread defers to its own record's pending lists, avoiding cross-thread synchronization on the hot path.

**Rationale Source:** pending[CK_EPOCH_LENGTH] in ck_epoch_record

**Consequences:**
- ck_epoch_call is wait-free for single owner
- Dispatch scans per-record lists
- Memory proportional to thread count

---

## Decision: Epoch bucket rotation

**Context:**
Need to know which callbacks are safe to dispatch.

**Options Considered:**

1. Timestamp per callback
   - Pro: Fine-grained
   - Con: Overhead per callback

2. Bucket by epoch modulo length
   - Pro: O(1) categorization
   - Con: Limited granularity

**Decision:** Bucket rotation (option 2)

**Rationale:** Callbacks are pushed to pending[epoch % CK_EPOCH_LENGTH]. When epoch advances, older buckets become safe. CK_EPOCH_LENGTH of 4 gives enough separation.

**Rationale Source:** offset = e & (CK_EPOCH_LENGTH - 1) in ck_epoch_call

**Consequences:**
- O(1) callback categorization
- Batched dispatch
- CK_EPOCH_LENGTH configurable

---

## Decision: Active nesting counter

**Context:**
Threads may nest epoch sections.

**Options Considered:**

1. Binary active flag
   - Pro: Simple
   - Con: Can't nest

2. Nesting counter
   - Pro: Supports recursive sections
   - Con: Slightly more state

**Decision:** Nesting counter (option 2)

**Rationale:** Real code often has nested read-side critical sections. Counter tracks depth, only updates epoch on outermost begin.

**Rationale Source:** active field increment/decrement

**Consequences:**
- Recursive begin/end supported
- Epoch observed only on outermost begin
- Release fence only on outermost end

---

## Decision: Store-load serialization on begin

**Context:**
Thread must observe current epoch before reading protected data.

**Options Considered:**

1. Plain store + fence
   - Pro: Portable
   - Con: May have stale reads

2. Fetch-and-store on TSO
   - Pro: Single instruction serialization
   - Con: Platform-specific

**Decision:** Platform-optimized serialization (both)

**Rationale:** On TSO (x86), fetch-and-store provides store-load ordering in one instruction. On weaker models, explicit memory fence used.

**Rationale Source:** #ifdef CK_MD_TSO in ck_epoch_begin

**Consequences:**
- Optimal on TSO platforms
- Correct on all platforms
- Readers see current epoch

---

## Decision: Record recycling

**Context:**
Threads may exit and rejoin.

**Options Considered:**

1. Allocate new record each time
   - Pro: Simple
   - Con: Memory growth

2. Recycle unregistered records
   - Pro: Bounded memory
   - Con: More complex lifecycle

**Decision:** Record recycling (option 2)

**Rationale:** Records marked FREE can be recycled by new threads. Prevents unbounded record growth in dynamic thread pools.

**Rationale Source:** ck_epoch_recycle, state field

**Consequences:**
- n_free tracks recyclable records
- recycle scans for FREE records
- Memory bounded by peak thread count

---

## Decision: Section objects for progress

**Context:**
Long-running readers could block epoch advancement.

**Options Considered:**

1. No special handling
   - Pro: Simple
   - Con: Long readers block reclamation

2. Section reference counting
   - Pro: Progress guarantees in long sections
   - Con: Extra API complexity

**Decision:** Optional section tracking (option 2)

**Rationale:** ck_epoch_section allows threads to make progress even during long read sections by tracking sense buckets.

**Rationale Source:** ck_epoch_section parameter, _ck_epoch_addref/_delref

**Consequences:**
- Optional section parameter
- Better progress in long sections
- Additional bookkeeping when used
