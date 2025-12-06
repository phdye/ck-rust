# Module: ck_backoff — Design Decisions

## Decision: Use compiler barrier for delay loop

**Context:**
Backoff needs to introduce a delay without being optimized away by the compiler.

**Options Considered:**

1. Use actual time-based delay (usleep, nanosleep)
   - Pro: Predictable wall-clock delay
   - Con: System call overhead
   - Con: Minimum granularity may be too coarse

2. Use busy-wait with volatile variable
   - Pro: Simple
   - Con: May still be optimized

3. Use compiler barrier in loop
   - Pro: Cannot be optimized away
   - Pro: No system call overhead
   - Pro: Minimal overhead per iteration
   - Con: Delay varies with CPU speed

**Decision:** Use compiler barrier (option 3)

**Rationale:** For spinlock backoff, we want minimal overhead and fine-grained control. Compiler barriers ensure the loop executes the expected number of times without heavy overhead.

**Rationale Source:** Implementation uses ck_pr_barrier() in loop

**Consequences:**
- Delay duration varies with CPU speed
- Very low overhead per iteration
- Cannot be used for wall-clock timing guarantees

---

## Decision: Exponential growth with ceiling

**Context:**
Backoff delay must grow to reduce contention but cannot grow unboundedly.

**Options Considered:**

1. Linear backoff
   - Pro: Simple, predictable
   - Con: Slow to reach effective delay under high contention

2. Exponential backoff without ceiling
   - Pro: Fast convergence
   - Con: Can grow to extremely long delays

3. Exponential backoff with ceiling
   - Pro: Fast convergence
   - Pro: Bounded worst-case delay
   - Con: May not be optimal for all workloads

**Decision:** Exponential with ceiling (option 3)

**Rationale:** Exponential backoff is well-established for contention management. The ceiling prevents pathological cases.

**Rationale Source:** Standard practice; implementation shows ceiling check

**Consequences:**
- Fast adaptation to contention
- Bounded maximum delay
- May need tuning of ceiling for specific workloads

---

## Decision: Default ceiling of ~1 million iterations

**Context:**
CK_BACKOFF_CEILING must be chosen to provide good default behavior.

**Options Considered:**

1. Small ceiling (e.g., 1024)
   - Pro: Bounded delay
   - Con: May not be enough for high contention

2. Large ceiling (~1 million)
   - Pro: Handles high contention well
   - Con: Maximum delay can be significant

3. User-configurable at compile time
   - Pro: Flexibility
   - Con: Requires user tuning

**Decision:** Default ~1 million, user-configurable (option 2+3)

**Rationale:** A million barrier iterations is still fast (microseconds range) but provides good backoff for most scenarios. Users can override via define.

**Rationale Source:** CK_BACKOFF_CEILING = (1 << 20) - 1

**Consequences:**
- Good defaults for most use cases
- Can be tuned via compile-time define
- Maximum delay is bounded but may be noticeable

---

## Decision: Initial value of 512

**Context:**
Starting backoff value affects initial behavior.

**Options Considered:**

1. Start at 1
   - Pro: Minimal initial delay
   - Con: Many doublings needed under contention

2. Start at 512
   - Pro: Reasonable initial delay
   - Pro: Quick to reach effective backoff
   - Con: May be too high for very light contention

**Decision:** Start at 512 (option 2)

**Rationale:** 512 iterations is nearly instantaneous but provides meaningful delay from the start.

**Rationale Source:** CK_BACKOFF_INITIALIZER = 1 << 9

**Consequences:**
- Immediate backoff is noticeable but small
- Fewer iterations to reach effective delay
