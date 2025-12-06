# Module: ck_queue — Design Decisions

## Decision: BSD queue.h-compatible API

**Context:**
Choosing the API design for concurrent-safe linked lists.

**Options Considered:**

1. Custom new API
   - Pro: Could optimize for concurrency from scratch
   - Con: Learning curve for users familiar with BSD queues

2. BSD queue.h-compatible with atomic modifications
   - Pro: Familiar API for BSD developers
   - Pro: Drop-in replacement for many use cases
   - Con: Some operations may not map well

**Decision:** BSD queue.h-compatible API (option 2)

**Rationale:** The BSD queue.h macros are widely used and well-understood. Adding concurrent safety while maintaining API compatibility minimizes migration effort.

**Rationale Source:** Header comment referencing FreeBSD sys/queue.h, API naming conventions

**Consequences:**
- CK_SLIST, CK_LIST, CK_STAILQ mirror BSD SLIST, LIST, STAILQ
- Developers familiar with BSD can migrate easily
- Some BSD operations (_SWAP) marked as non-atomic

---

## Decision: Atomic loads for all reads

**Context:**
How to handle concurrent reads while modifications occur.

**Options Considered:**

1. No special handling (rely on volatile or compiler barriers)
   - Pro: Simpler implementation
   - Con: Not portable, may not work on all architectures

2. Atomic loads via ck_pr
   - Pro: Portable concurrent safety
   - Pro: Integrates with ck_pr memory model
   - Con: Slightly more overhead

**Decision:** Atomic loads via ck_pr (option 2)

**Rationale:** Using ck_pr_load_ptr ensures that pointer reads are atomic and properly ordered across all supported platforms.

**Rationale Source:** All FIRST/NEXT/EMPTY macros use ck_pr_load_ptr

**Consequences:**
- Readers see consistent pointer values
- No torn reads even on platforms with weak memory models
- Slight overhead compared to plain reads

---

## Decision: Store fence before linking

**Context:**
Ensuring newly inserted elements' data is visible before the element becomes reachable.

**Options Considered:**

1. No fence (rely on caller)
   - Pro: Maximum performance
   - Con: Easy to misuse, data races

2. Store fence in insertion macros
   - Pro: Automatic safety
   - Pro: Data always visible before element is linked
   - Con: Fence cost even when not needed

**Decision:** Store fence in insertion macros (option 2)

**Rationale:** The pattern "write data, fence, link" is the standard safe publication idiom. Building it into the macros prevents bugs.

**Rationale Source:** ck_pr_fence_store() before ck_pr_store_ptr in all INSERT macros

**Consequences:**
- Readers always see valid data in newly linked elements
- Automatic safety without caller effort
- Small performance cost for fence

---

## Decision: Writer-side synchronization required

**Context:**
Concurrent modifications to linked lists are complex and error-prone.

**Options Considered:**

1. Lock-free write operations
   - Pro: Maximum concurrency
   - Con: Complex, requires CAS loops
   - Con: Some operations difficult (e.g., removal)

2. Require external writer synchronization
   - Pro: Simpler, correct implementation
   - Pro: Writers can use appropriate locking strategy
   - Con: Less concurrent write throughput

**Decision:** Require external writer synchronization (option 2)

**Rationale:** Lock-free linked list modifications are complex (especially for doubly-linked lists) and often require memory reclamation. Requiring writer synchronization simplifies the implementation while still providing significant value for read-heavy workloads.

**Rationale Source:** Documentation stating writers must synchronize

**Consequences:**
- Writers must use locks or other synchronization
- Readers can proceed without blocking
- Suitable for read-mostly workloads

---

## Decision: FOREACH_SAFE for removal during iteration

**Context:**
Removing the current element during iteration would break the iterator.

**Options Considered:**

1. Document that removal during iteration is unsafe
   - Pro: Simpler implementation
   - Con: Common pattern unsupported

2. Provide FOREACH_SAFE variant
   - Pro: Supports common removal-during-iteration pattern
   - Con: Slightly more complex macro

**Decision:** Provide FOREACH_SAFE variant (option 2)

**Rationale:** Removing elements during iteration is a common pattern. The _SAFE variant caches the next pointer before the loop body, allowing safe removal of the current element.

**Rationale Source:** CK_SLIST_FOREACH_SAFE and similar macros

**Consequences:**
- Two variants: regular FOREACH and FOREACH_SAFE
- _SAFE variant requires extra variable (tvar)
- Safe removal of current element only (not arbitrary elements)

---

## Decision: Three list variants (SLIST, STAILQ, LIST)

**Context:**
Different use cases have different requirements for list operations.

**Options Considered:**

1. Single general-purpose list
   - Pro: Simple API
   - Con: Cannot optimize for specific use cases

2. Multiple specialized variants
   - Pro: Optimal data structure per use case
   - Pro: Matches BSD queue.h variants
   - Con: Larger API surface

**Decision:** Multiple specialized variants (option 2)

**Rationale:** Each variant optimizes for different access patterns:
- SLIST: Minimal memory (one pointer), stack-like (LIFO)
- STAILQ: One pointer + tail, queue-like (FIFO) with O(1) append
- LIST: Two pointers, O(1) arbitrary removal

**Rationale Source:** Three separate sets of macros in ck_queue.h

**Consequences:**
- Users choose appropriate variant for their use case
- SLIST for minimal overhead when only head access needed
- STAILQ for FIFO queues
- LIST when O(1) removal is required

---

## Decision: No support for Alpha architecture

**Context:**
Alpha architecture has unique memory model requirements.

**Options Considered:**

1. Support Alpha with additional fences
   - Pro: Broader platform support
   - Con: Alpha largely obsolete
   - Con: Requires load-depend fences

2. Document unsupported
   - Pro: Simpler implementation
   - Con: Excludes Alpha users

**Decision:** Document unsupported (option 2)

**Rationale:** Alpha architecture is largely obsolete and its unique requirement for load-depend memory fences would complicate the implementation for minimal benefit.

**Rationale Source:** Comment in header stating "unsupported on architectures such as the Alpha which require load-depend memory fences"

**Consequences:**
- ck_queue not available on Alpha
- Simpler implementation for current platforms
