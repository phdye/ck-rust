# Module: ck_tflock — Design Decisions

## Decision: Task-fair (strict FIFO) ordering

**Context:**
How to order read and write requests.

**Options Considered:**

1. Reader-preference
   - Pro: Maximum reader throughput
   - Con: Writer starvation

2. Phase-fair (group readers and writers)
   - Pro: Good throughput with fairness
   - Con: Not strict FIFO

3. Task-fair (strict FIFO)
   - Pro: Maximum fairness
   - Pro: Predictable wait times
   - Con: Reduced reader parallelism

**Decision:** Task-fair (option 3)

**Rationale:** For real-time systems and predictable behavior, strict FIFO ordering ensures no request waits longer than necessary.

**Rationale Source:** Reference to Mellor-Crummey and Scott 1991 paper

**Consequences:**
- Readers after a writer must wait
- Predictable, bounded wait times
- Lower aggregate throughput than phase-fair

---

## Decision: Split ticket counters in single word

**Context:**
Need to track reader and writer tickets.

**Options Considered:**

1. Separate 32-bit counters
   - Pro: Simple
   - Con: Coordination between counters

2. Split single 32-bit word
   - Pro: Single atomic for coordination
   - Con: Limited to 16 bits each

**Decision:** Split single word (option 2)

**Rationale:** Using upper 16 bits for readers and lower 16 for writers allows atomic read of both in single load.

**Rationale Source:** RC_INCR = 0x10000, WC_INCR = 0x1, W_MASK = 0xFFFF

**Consequences:**
- Maximum 65535 concurrent readers or writers
- Single-word atomic operations
- Overflow handled by clearing top bit

---

## Decision: Fetch-clear-add helper function

**Context:**
Overflow must be prevented in ticket counters.

**Options Considered:**

1. Ignore overflow (undefined behavior)
   - Con: Breaks with high usage

2. Handle overflow via clear mask
   - Pro: Safe operation
   - Con: Slightly more complex

**Decision:** Handle overflow (option 2)

**Rationale:** The fca_32 helper clears potential overflow bit before adding, ensuring counter stays in valid range.

**Rationale Source:** ck_tflock_ticket_fca_32 with mask parameter

**Consequences:**
- Counters cannot overflow
- CAS loop for atomic operation
- Safe for long-running systems
