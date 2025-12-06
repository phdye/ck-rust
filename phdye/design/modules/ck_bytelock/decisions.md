# Module: ck_bytelock — Design Decisions

## Decision: Per-slot byte indicator array

**Context:**
Need efficient reader tracking with minimal contention.

**Options Considered:**

1. Shared reader counter
   - Pro: Simple
   - Con: Cache contention

2. Bitmap for readers
   - Pro: Compact
   - Con: Bit manipulation overhead

3. Byte array (one byte per slot)
   - Pro: Simple byte writes
   - Pro: No read-modify-write for lock
   - Con: More memory than bitmap

**Decision:** Byte array (option 3)

**Rationale:** Using a full byte per reader slot allows simple store operations without read-modify-write, improving performance on most architectures.

**Rationale Source:** readers[] array of uint8_t

**Consequences:**
- ~56 slots per cache line
- Simple store for lock, store for unlock
- Writer scans bytes, can use wide loads

---

## Decision: Slot-based thread identification

**Context:**
Need to identify which reader is which.

**Options Considered:**

1. Automatic slot assignment
   - Pro: Transparent to user
   - Con: Requires thread ID mechanism

2. Caller-provided slot number
   - Pro: Flexible, no TLS dependency
   - Con: User must manage slots

**Decision:** Caller-provided slot (option 2)

**Rationale:** Letting callers provide their slot number keeps the implementation simple and avoids thread-local storage dependencies.

**Rationale Source:** slot parameter in all lock functions

**Consequences:**
- User assigns slots (e.g., thread ID mod slots)
- CK_BYTELOCK_UNSLOTTED for overflow
- Flexible slot assignment strategies

---

## Decision: Fallback counter for unslotted threads

**Context:**
More threads than slots may need read access.

**Options Considered:**

1. Reject unslotted readers
   - Pro: Simple
   - Con: Limits thread count

2. Shared counter fallback
   - Pro: Unlimited readers
   - Con: Counter contention

**Decision:** Shared counter fallback (option 2)

**Rationale:** The n_readers counter allows any number of threads to acquire read locks, with degraded performance for threads without assigned slots.

**Rationale Source:** n_readers field and slot > sizeof(readers) check

**Consequences:**
- Slotted readers: O(1) no contention
- Unslotted readers: shared counter contention
- Graceful degradation

---

## Decision: Lock upgrade/downgrade support

**Context:**
Holder of write lock may want to downgrade to read.

**Options Considered:**

1. No upgrade/downgrade
   - Pro: Simpler
   - Con: Must release and reacquire

2. Support downgrade
   - Pro: Atomic transition
   - Con: More complex logic

**Decision:** Support downgrade (option 2)

**Rationale:** A writer can atomically transition to reader by setting its slot and clearing owner, avoiding window where lock is unprotected.

**Rationale Source:** read_lock checks if caller is current owner

**Consequences:**
- Write → Read is atomic
- No gap in protection during downgrade
- Useful for read-after-write patterns
