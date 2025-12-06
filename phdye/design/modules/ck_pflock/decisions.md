# Module: ck_pflock — Design Decisions

## Decision: Phase-fair algorithm

**Context:**
Reader-writer lock algorithms trade off between throughput and fairness.

**Options Considered:**

1. Reader-preference (readers never wait for pending writers)
   - Pro: Maximum reader throughput
   - Con: Writer starvation possible

2. Writer-preference (writers block new readers)
   - Pro: Writers don't starve
   - Con: Reader starvation possible

3. Phase-fair (FIFO by phases)
   - Pro: No starvation for either
   - Pro: Bounded wait times
   - Con: Slightly lower throughput

**Decision:** Phase-fair (option 3)

**Rationale:** For real-time and general-purpose use, preventing starvation is important. Phase-fair locks provide fairness guarantees suitable for multiprocessor real-time systems.

**Rationale Source:** Reference to Brandenburg and Anderson 2010 paper

**Consequences:**
- Bounded waiting for both readers and writers
- Readers in current phase complete before writer enters
- New readers during write wait for next phase

---

## Decision: Ticket-based writer ordering

**Context:**
Multiple writers need fair ordering.

**Options Considered:**

1. CAS-based contention
   - Pro: Simple
   - Con: Unfair under contention

2. Ticket lock style (win/wout)
   - Pro: FIFO fairness
   - Pro: Bounded wait
   - Con: Slightly more complex

**Decision:** Ticket lock style (option 2)

**Rationale:** Ticket-based ordering ensures writers are served in arrival order, preventing writer starvation.

**Rationale Source:** win/wout fields implementing ticket pattern

**Consequences:**
- Writers acquire in FIFO order
- Each writer waits for wout to match its ticket
- Predictable, fair behavior

---

## Decision: Reader count in upper bits

**Context:**
Reader count and writer state share the rin variable.

**Options Considered:**

1. Separate variables
   - Pro: Simpler semantics
   - Con: Multiple atomics for coordination

2. Pack into single word
   - Pro: Atomic read-modify-write of both
   - Con: Bit manipulation complexity

**Decision:** Pack into single word (option 2)

**Rationale:** Using upper bits for reader count and lower bits for writer state allows atomic coordination between readers and writers in a single FAA operation.

**Rationale Source:** RINC = 0x100, WBITS = 0x3

**Consequences:**
- Reader increment doesn't affect writer bits
- Writer can atomically set presence while reading count
- Efficient single-variable coordination
