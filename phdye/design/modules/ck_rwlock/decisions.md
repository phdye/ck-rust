# Module: ck_rwlock — Design Decisions

## Decision: Separate writer flag and reader count

**Context:**
Need to track both writers and readers.

**Options Considered:**

1. Packed single word
   - Pro: Atomic operations
   - Con: Limited range

2. Separate fields
   - Pro: Full range, simpler logic
   - Con: Not atomic together

**Decision:** Separate fields (option 2)

**Rationale:** Writer flag and n_readers as separate unsigned ints. Careful ordering with fences ensures correctness. Simpler code and full count range.

**Rationale Source:** Separate writer and n_readers fields

**Consequences:**
- Simple increment/decrement
- Fence ordering required
- Full 32-bit reader count

---

## Decision: Check writer before incrementing readers

**Context:**
Read lock needs to coordinate with writers.

**Options Considered:**

1. Always increment, then check
   - Pro: Simple
   - Con: Wasted increment if writer present

2. Check first, then increment, then recheck
   - Pro: Avoid increment when writer active
   - Con: More complex

**Decision:** Check-increment-recheck (option 2)

**Rationale:** Spin while writer != 0 before incrementing. After increment, recheck writer and back out if appeared. Reduces unnecessary atomic operations.

**Rationale Source:** read_lock loop structure

**Consequences:**
- Less contention when writer active
- More code but better performance
- Classic reader-writer lock pattern

---

## Decision: Recursive writer via thread ID

**Context:**
Some use cases need recursive writer acquisition.

**Options Considered:**

1. No recursive support
   - Pro: Simple
   - Con: User must track

2. Thread ID-based recursion
   - Pro: Handles recursive case
   - Con: User provides tid

**Decision:** Thread ID-based recursion (option 2)

**Rationale:** Recursive variant stores owner tid in writer field. If caller matches, just increment wc. User must pass consistent tid.

**Rationale Source:** ck_rwlock_recursive_write_lock(rw, tid)

**Consequences:**
- Recursive writer supported
- User responsible for tid consistency
- wc tracks depth

---

## Decision: Write-to-read downgrade

**Context:**
Writer may want to become reader without releasing.

**Options Considered:**

1. Release and reacquire
   - Pro: Simple
   - Con: Other writers may intervene

2. Atomic downgrade
   - Pro: No window for other writers
   - Con: Extra operation

**Decision:** Atomic downgrade (option 2)

**Rationale:** ck_rwlock_write_downgrade increments n_readers before clearing writer. Caller transitions smoothly to reader.

**Rationale Source:** write_downgrade implementation

**Consequences:**
- Seamless write-to-read transition
- Other readers can join
- Other writers still blocked until read unlock

---

## Decision: Lock elision support

**Context:**
Can benefit from hardware transactional memory.

**Options Considered:**

1. No elision
   - Pro: Simple
   - Con: Miss optimization

2. Add elision wrappers
   - Pro: RTM acceleration
   - Con: More code

**Decision:** Add elision wrappers (option 2)

**Rationale:** Both read and write paths have CK_ELIDE_PROTOTYPE wrappers. On RTM-capable hardware, can speculatively execute without lock.

**Rationale Source:** CK_ELIDE_PROTOTYPE macros for read and write

**Consequences:**
- RTM optimization available
- Falls back to regular path
- Separate elision wrappers for read and write
