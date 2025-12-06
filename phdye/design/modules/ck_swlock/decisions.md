# Module: ck_swlock — Design Decisions

## Decision: Packed single-word representation

**Context:**
Need efficient reader-writer lock state storage.

**Options Considered:**

1. Separate fields (like ck_rwlock)
   - Pro: Simple, full range
   - Con: Larger, two cache lines possible

2. Packed single word
   - Pro: Smaller, single atomic access
   - Con: Limited reader count

**Decision:** Packed single word (option 2)

**Rationale:** Single 32-bit word fits in one cache line. Bit 31 for writer, bit 30 for latch, bits 0-29 for readers (~1 billion max).

**Rationale Source:** CK_SWLOCK_*_BIT and _MASK definitions

**Consequences:**
- 4 bytes total (vs 8 for ck_rwlock)
- Reader limit ~1 billion
- Single atomic operations possible

---

## Decision: Separate latch bit

**Context:**
Writer may need stronger exclusion guarantee.

**Options Considered:**

1. Single writer bit
   - Pro: Simple
   - Con: Readers can still increment during writer spin

2. Latch bit for barrier
   - Pro: Blocks new reader entry
   - Con: More complex protocol

**Decision:** Latch bit (option 2)

**Rationale:** LATCH_BIT creates a reader barrier. Once set, read_lock backs out if it sees latch. Guarantees no reader traffic during latched critical section.

**Rationale Source:** CK_SWLOCK_LATCH_BIT, write_latch/unlatch

**Consequences:**
- write_latch for exclusive access
- write_lock allows reader increment (writer waits)
- Different unlock for each mode

---

## Decision: Reader backs out if latching detected

**Context:**
Reader increments count, then discovers writer latching.

**Options Considered:**

1. Keep increment, let writer wait
   - Pro: Simple for reader
   - Con: Defeats latch purpose

2. Back out if writer latching
   - Pro: Respects latch intent
   - Con: Reader retry

**Decision:** Back out (option 2)

**Rationale:** In read_lock, after increment, check WRITER_MASK. If WRITER_BIT set (possible latching), decrement and retry. Only proceed if clear.

**Rationale Source:** read_lock loop logic

**Consequences:**
- Readers respect latch
- May need retry
- Writer latching guaranteed to drain readers

---

## Decision: OR for write intent, AND for clear

**Context:**
Setting and clearing writer bits.

**Options Considered:**

1. CAS for both
   - Pro: Atomic read-modify-write
   - Con: May fail spuriously

2. OR to set, AND to clear
   - Pro: Always succeeds
   - Con: Not read-modify-write

**Decision:** OR/AND operations (option 2)

**Rationale:** ck_pr_or_32 sets WRITER_BIT, ck_pr_and_32 clears it. These are atomic and always succeed, avoiding CAS retry loops.

**Rationale Source:** write_lock uses or_32, write_unlock uses and_32

**Consequences:**
- No CAS failures
- Simple implementation
- Works with reader increments

---

## Decision: Zero-store for unlatch

**Context:**
Releasing latched lock.

**Options Considered:**

1. AND with READER_MASK
   - Pro: Consistent with write_unlock
   - Con: Extra operation

2. Store 0
   - Pro: Simple, fast
   - Con: Assumes no readers (correct for latch)

**Decision:** Store 0 (option 2)

**Rationale:** After latched critical section, reader count is guaranteed zero. Store 0 is simpler and faster than AND.

**Rationale Source:** write_unlatch stores 0

**Consequences:**
- Fast unlatch
- Correct only after latch (precondition)
- Simpler than AND
