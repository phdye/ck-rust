# Module: ck_spinlock — Design Decisions

## Decision: Multiple spinlock implementations

**Context:**
Different use cases have different requirements.

**Options Considered:**

1. Single spinlock type
   - Pro: Simple API
   - Con: Can't optimize for specific needs

2. Multiple implementations
   - Pro: Choose based on requirements
   - Con: More code, user must decide

**Decision:** Multiple implementations (option 2)

**Rationale:** Different scenarios need different trade-offs. FAS is fastest for uncontended case. Ticket/MCS provide fairness. HCLH is NUMA-aware. User selects based on needs.

**Rationale Source:** Separate headers for each variant

**Consequences:**
- FAS for low-latency
- Ticket for fairness
- MCS for scalability
- HCLH for NUMA

---

## Decision: FAS as default

**Context:**
Need a default spinlock type for generic use.

**Options Considered:**

1. Ticket (fair, predictable)
   - Pro: Fair
   - Con: Higher latency

2. FAS (lowest latency)
   - Pro: Fastest fast-path
   - Con: Unfair

**Decision:** FAS as default (option 2)

**Rationale:** Testing on x86, x86_64, PPC64, SPARC64 showed FAS had lowest latency in fast path or negligible degradation.

**Rationale Source:** Comment in ck_spinlock.h

**Consequences:**
- ck_spinlock_t aliases ck_spinlock_fas_t
- Best uncontended performance
- May starve under high contention

---

## Decision: Local spinning for MCS/CLH

**Context:**
How should waiting threads spin?

**Options Considered:**

1. Spin on global variable
   - Pro: Simple
   - Con: Cache-line bouncing

2. Spin on local node
   - Pro: Cache-friendly
   - Con: More complex

**Decision:** Local spinning (option 2)

**Rationale:** MCS and CLH threads spin on their own node's flag. Only the unlocking thread modifies it, avoiding cache invalidation storms.

**Rationale Source:** MCS/CLH algorithm design

**Consequences:**
- Better scalability
- Per-thread node required
- Lower cache traffic

---

## Decision: Test-and-test-and-set pattern

**Context:**
How should FAS lock spin?

**Options Considered:**

1. Continuous FAS
   - Pro: Simple
   - Con: Bus traffic

2. Test-and-test-and-set (TTAS)
   - Pro: Reduced bus traffic
   - Con: Slightly more code

**Decision:** TTAS pattern (option 2)

**Rationale:** Spin on load while locked, only attempt FAS when lock appears free. Reduces unnecessary atomic operations.

**Rationale Source:** FAS lock inner loop: load while true, then try FAS

**Consequences:**
- Reduced cache traffic
- Better behavior under contention
- Standard optimization

---

## Decision: Proportional backoff for ticket lock

**Context:**
Ticket lock waiters know their position in queue.

**Options Considered:**

1. Fixed backoff
   - Pro: Simple
   - Con: Not optimal

2. Proportional to queue position
   - Pro: Back off based on expected wait
   - Con: More complex

**Decision:** Proportional backoff (option 2)

**Rationale:** lock_pb calculates backoff as (request - position) << c. Threads further from front back off more, reducing contention.

**Rationale Source:** ck_spinlock_ticket_lock_pb

**Consequences:**
- Smarter backoff
- Reduced contention
- User provides scale factor c
