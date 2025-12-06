# Module: ck_stack — Design Decisions

## Decision: Treiber stack algorithm

**Context:**
Lock-free stack implementation is a well-studied problem with multiple solutions.

**Options Considered:**

1. Treiber stack with CAS
   - Pro: Simple, well-understood, efficient
   - Pro: Single atomic operation for push/pop
   - Con: Susceptible to ABA problem

2. Elimination stack
   - Pro: Better scalability under high contention
   - Con: More complex implementation
   - Con: Not always beneficial

3. Flat combining
   - Pro: Can reduce cache contention
   - Con: Not truly lock-free
   - Con: Requires dedicated combiner thread logic

**Decision:** Treiber stack (option 1)

**Rationale:** The Treiber stack is the canonical lock-free stack algorithm. It provides good performance for most workloads and is simple to implement correctly.

**Rationale Source:** Implementation in ck_stack.h and reference to Treiber (1986)

**Consequences:**
- Simple, minimal code
- ABA problem must be addressed separately
- Good baseline performance

---

## Decision: Multiple API variants for different concurrency patterns

**Context:**
Different use cases have different producer/consumer patterns, and optimal implementation varies.

**Options Considered:**

1. Single generic implementation (MPMC)
   - Pro: Simple API
   - Con: Performance penalty when MPMC not needed

2. Multiple specialized variants
   - Pro: Optimal performance per use case
   - Pro: Clear semantics per variant
   - Con: API complexity

**Decision:** Multiple specialized variants (option 2)

**Rationale:** Concurrency primitives are performance-critical. Providing UPMC, MPMC, MPNC, SPNC, NPSC variants lets users select the minimum overhead for their use case.

**Rationale Source:** API design with _upmc, _mpmc, _mpnc, _spnc, _npsc suffixes

**Consequences:**
- Users must understand their concurrency pattern
- Misuse of wrong variant causes undefined behavior
- Maximum performance when used correctly

---

## Decision: Generation counter for MPMC ABA prevention

**Context:**
MPMC pop is susceptible to ABA problem when entries are reused.

**Options Considered:**

1. Require external safe memory reclamation
   - Pro: Simpler stack implementation
   - Con: Pushes complexity to user
   - Con: Same as UPMC

2. Tagged pointer with version number
   - Pro: Solves ABA in common cases
   - Con: Version bits reduce address space
   - Con: Overflow possible

3. Separate generation counter with double-width CAS
   - Pro: Full pointer space preserved
   - Pro: Counter can be pointer-sized
   - Con: Requires hardware double-width CAS

**Decision:** Separate generation counter with double-width CAS (option 3)

**Rationale:** The generation counter approach preserves the full address space and provides robust ABA prevention. Double-width CAS is available on modern platforms (cmpxchg16b on x86-64, CASP on ARM64).

**Rationale Source:** ck_stack structure with separate generation field, CK_F_PR_CAS_PTR_2_VALUE requirement

**Consequences:**
- MPMC pop requires double-width CAS support
- Generation counter increments on each pop
- Stack structure is 16 bytes on 64-bit platforms

---

## Decision: Intrusive design (embedded entry structure)

**Context:**
Stack entries can be allocated separately or embedded in user structures.

**Options Considered:**

1. Stack allocates and manages entry structures
   - Pro: Simpler user API
   - Con: Memory allocation overhead
   - Con: Cache locality issues

2. Intrusive design (user embeds ck_stack_entry)
   - Pro: Zero allocation overhead
   - Pro: User controls memory layout
   - Con: User must understand intrusive patterns

**Decision:** Intrusive design (option 2)

**Rationale:** Intrusive data structures are standard for high-performance C libraries. Zero allocation overhead and user-controlled memory layout are essential for performance-critical code.

**Rationale Source:** ck_stack_entry structure design with CK_STACK_CONTAINER macro

**Consequences:**
- User embeds ck_stack_entry in their structures
- CK_STACK_CONTAINER macro to recover containing structure
- No dynamic allocation within stack operations

---

## Decision: Batch pop operation

**Context:**
Some use cases need to drain entire stack atomically.

**Options Considered:**

1. Only single-element pop
   - Pro: Simpler API
   - Con: Multiple atomic ops to drain

2. Provide batch pop
   - Pro: Single atomic operation to drain
   - Pro: Useful for work-stealing, memory reclamation
   - Con: Slightly larger API

**Decision:** Provide batch pop (option 2)

**Rationale:** Batch pop is a single atomic swap, making it extremely efficient for draining. This pattern is common in work-stealing schedulers and memory reclamation.

**Rationale Source:** ck_stack_batch_pop_upmc implementation using FAS

**Consequences:**
- Caller receives linked list of all entries
- Single atomic operation regardless of stack size
- Useful primitive for higher-level algorithms

---

## Decision: Fence placement in push/pop

**Context:**
Correct memory ordering requires fences, but fence placement affects performance.

**Options Considered:**

1. Full memory barrier on every operation
   - Pro: Obviously correct
   - Con: Performance penalty

2. Minimal fences based on memory model
   - Pro: Optimal performance
   - Pro: Correct ordering preserved
   - Con: More subtle correctness reasoning

**Decision:** Minimal fences (option 2)

**Rationale:** Following standard patterns: release fence before CAS in push (ensures entry->next is visible), acquire fence after load in pop (ensures entry contents are visible after pop returns).

**Rationale Source:** Implementation shows fence_store/fence_load placement

**Consequences:**
- Push has release semantics
- Pop has acquire semantics
- Correct synchronization for typical producer/consumer patterns

---

## Decision: MPNC uses FAS instead of CAS

**Context:**
When no consumers exist, push can be optimized.

**Options Considered:**

1. Use CAS like other variants
   - Pro: Consistent implementation
   - Con: CAS may fail and retry under contention

2. Use FAS (fetch-and-store)
   - Pro: Always succeeds, no retry needed
   - Pro: Single atomic operation
   - Con: Requires post-operation fix-up of next pointer

**Decision:** FAS-based MPNC (option 2)

**Rationale:** When no consumers exist, there's no ABA risk, so FAS (which always succeeds) is more efficient than CAS loop. The next pointer fix-up is safe since no consumer is reading.

**Rationale Source:** ck_stack_push_mpnc implementation using ck_pr_fas_ptr

**Consequences:**
- Wait-free push under MPNC pattern
- Stack temporarily inconsistent during push
- Must ensure no consumers during MPNC usage
