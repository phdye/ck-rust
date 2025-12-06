# Module: ck_barrier — Design Decisions

## Decision: Multiple barrier implementations

**Context:**
Different barrier algorithms have different performance characteristics.

**Options Considered:**

1. Single generic barrier
   - Pro: Simple API
   - Con: Suboptimal for many scenarios

2. Multiple specialized implementations
   - Pro: Optimal for different thread counts and topologies
   - Con: More complex API

**Decision:** Multiple implementations (option 2)

**Rationale:** Centralized is simple and fast for small n. Combining is good for NUMA. Dissemination has no central bottleneck. Tournament minimizes memory traffic. MCS scales well with low contention.

**Rationale Source:** Five distinct barrier types provided

**Consequences:**
- Users choose appropriate barrier
- Flexibility for different scenarios
- More API surface

---

## Decision: Sense reversal for reuse

**Context:**
Barriers must be reusable across multiple synchronization rounds.

**Options Considered:**

1. Reset counter explicitly
   - Pro: Simple
   - Con: Race conditions possible

2. Sense reversal
   - Pro: Safe, no reset needed
   - Con: Extra state per thread

**Decision:** Sense reversal (option 2)

**Rationale:** Each thread maintains local sense flag. Global sense toggles when barrier completes. Threads compare local vs global to detect completion. Alternating sense prevents early wake from previous round.

**Rationale Source:** All barriers use sense field

**Consequences:**
- Safe reuse without explicit reset
- Per-thread state required
- No race conditions between rounds

---

## Decision: Cacheline alignment for tree nodes

**Context:**
Tree barriers have multiple threads accessing different nodes.

**Options Considered:**

1. Packed nodes
   - Pro: Less memory
   - Con: False sharing between nodes

2. Cacheline-aligned nodes
   - Pro: No false sharing
   - Con: More memory

**Decision:** Cacheline alignment (option 2)

**Rationale:** ck_barrier_combining_group uses CK_CC_CACHELINE alignment to ensure each node is on its own cacheline, preventing false sharing during concurrent access.

**Rationale Source:** CK_CC_CACHELINE on combining_group struct

**Consequences:**
- Better performance under contention
- Higher memory usage
- Predictable cacheline behavior

---

## Decision: External memory allocation

**Context:**
Barriers need per-thread and global state.

**Options Considered:**

1. Internal allocation
   - Pro: Simpler API
   - Con: Requires allocator dependency

2. External allocation
   - Pro: Caller controls memory
   - Con: More complex setup

**Decision:** External allocation (option 2)

**Rationale:** All barrier_init functions take pre-allocated arrays. Dissemination and tournament provide _size functions to compute needed allocation. Keeps module dependency-free.

**Rationale Source:** All *_init signatures take allocated memory

**Consequences:**
- No allocator dependency
- Caller manages memory
- Size functions help allocation

---

## Decision: 4-ary tree for MCS barrier

**Context:**
MCS barrier uses tree structure for scalability.

**Options Considered:**

1. Binary tree
   - Pro: Simple
   - Con: More levels, higher latency

2. 4-ary tree
   - Pro: Fewer levels
   - Con: More children per node

**Decision:** 4-ary tree (option 2)

**Rationale:** MCS barrier uses 4 children per node (childnotready[4], havechild[4]). Reduces tree height from log2(n) to log4(n). Balances parallelism with tree depth.

**Rationale Source:** Array sizes in ck_barrier_mcs struct

**Consequences:**
- Fewer synchronization rounds
- Lower overall latency
- Good balance for typical core counts

---

## Decision: Parity flags for dissemination

**Context:**
Dissemination barrier requires flag management.

**Options Considered:**

1. Reset flags each round
   - Pro: Simple
   - Con: Race with readers

2. Dual flag arrays with parity
   - Pro: No reset needed
   - Con: Double memory

**Decision:** Dual arrays with parity (option 2)

**Rationale:** flags[2] array with parity bit alternates between flag sets. Writers never race with readers from previous round. State parity field tracks current set.

**Rationale Source:** flags[2] in dissemination struct, parity in state

**Consequences:**
- Safe concurrent access
- Double flag memory
- Robust without reset
