# Module: ck_brlock — Design Decisions

## Decision: Per-reader counter for cache locality

**Context:**
Reader-writer locks often have cache contention on shared counters.

**Options Considered:**

1. Shared reader counter
   - Pro: Simple implementation
   - Con: Cache line bouncing on reader lock/unlock

2. Per-reader distributed counters
   - Pro: Cache-local read operations
   - Con: O(n) write lock acquisition

**Decision:** Per-reader counters (option 2)

**Rationale:** For read-heavy workloads, eliminating cache contention on the read path significantly improves performance. The O(n) write cost is acceptable when writes are infrequent.

**Rationale Source:** Header comments explaining Linux kernel origin (Ingo Molnar, David S. Miller)

**Consequences:**
- Readers must register before use
- Read lock/unlock only touch reader's own cache line
- Writer traverses all readers

---

## Decision: Thread-agnostic design with linked list

**Context:**
Some implementations use thread ID to array mapping.

**Options Considered:**

1. Array indexed by thread ID
   - Pro: O(1) lookup
   - Con: Requires thread ID API
   - Con: Fixed maximum threads

2. Linked list of reader structures
   - Pro: Thread-agnostic
   - Pro: Dynamic number of readers
   - Con: List traversal overhead
   - Con: Larger per-reader structure

**Decision:** Linked list (option 2)

**Rationale:** Being thread-agnostic improves portability and flexibility. The overhead of linked list traversal only affects writers.

**Rationale Source:** ck_brlock_reader with previous/next pointers

**Consequences:**
- Works without thread ID mechanism
- Per-reader structure is larger (2 extra pointers)
- Reader registration modifies the list

---

## Decision: Recursive read locking

**Context:**
Same thread may try to acquire read lock multiple times.

**Options Considered:**

1. Forbid recursive locking
   - Pro: Simpler
   - Con: User must track

2. Support recursive locking
   - Pro: More flexible
   - Con: Extra check on lock

**Decision:** Support recursive locking (option 2)

**Rationale:** Checking n_readers >= 1 allows fast recursive acquisition without full lock protocol.

**Rationale Source:** if (reader->n_readers >= 1) { increment and return }

**Consequences:**
- Same reader can lock multiple times
- Unlock must be called matching number of times
- Fast path for recursive case
