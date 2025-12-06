# Module: ck_ring — Design Decisions

## Decision: Bounded ring buffer with power-of-2 size

**Context:**
Ring buffer implementation requires efficient index wrapping.

**Options Considered:**

1. Arbitrary size with modulo
   - Pro: Flexible sizing
   - Con: Division/modulo is expensive

2. Power-of-2 size with bitmask
   - Pro: Index wrapping via AND is fast
   - Con: Size must be power of 2, wastes memory if not natural fit

**Decision:** Power-of-2 size with bitmask (option 2)

**Rationale:** Performance is critical for concurrent queues. Replacing modulo with bitwise AND significantly improves throughput.

**Rationale Source:** ring->mask = size - 1, all index operations use & mask

**Consequences:**
- User must allocate power-of-2 sized buffer
- Slight memory overhead if natural size is not power of 2
- Fast index computation

---

## Decision: Separate producer and consumer cache lines

**Context:**
False sharing occurs when producer and consumer indices share a cache line.

**Options Considered:**

1. No padding (compact structure)
   - Pro: Smaller memory footprint
   - Con: Severe false sharing, poor performance

2. Cache line padding between indices
   - Pro: Eliminates false sharing
   - Pro: Each core works on its own cache line
   - Con: Larger structure

**Decision:** Cache line padding (option 2)

**Rationale:** False sharing can reduce performance by 10-100x. The cost of extra padding (typically 64-128 bytes) is negligible compared to the performance benefit.

**Rationale Source:** pad[CK_MD_CACHELINE - sizeof(unsigned int)] between c_head and p_tail

**Consequences:**
- Structure size is ~2 cache lines
- Producer and consumer don't contend on cache
- Near-linear scalability for SPSC

---

## Decision: Separate p_head and p_tail for MP enqueue

**Context:**
Multiple producer enqueue requires coordination.

**Options Considered:**

1. Single producer index with CAS
   - Pro: Simpler structure
   - Con: Cannot support reserve/commit pattern
   - Con: Data copy must happen in CAS loop

2. Separate p_head (reserve) and p_tail (commit)
   - Pro: Enables reserve/commit (zero-copy)
   - Pro: Data copy happens after reservation
   - Con: Slightly more complex

**Decision:** Separate p_head and p_tail (option 2)

**Rationale:** The reserve/commit pattern enables zero-copy enqueue for large data. Separating the indices allows reservation via CAS, then writing data, then committing.

**Rationale Source:** p_head for reservation, p_tail for visibility, commit waits for p_tail

**Consequences:**
- MP enqueue reserves slot first via CAS on p_head
- Data is written to reserved slot
- Commit waits for predecessors (p_tail to catch up)
- Enables reserve/commit API for zero-copy

---

## Decision: Blocking commit for in-order visibility

**Context:**
With multiple producers reserving slots concurrently, commits must be ordered.

**Options Considered:**

1. Allow out-of-order commits
   - Pro: Maximum producer throughput
   - Con: Breaks FIFO guarantee
   - Con: Consumer may see gaps

2. Block commit until predecessors complete
   - Pro: Strict FIFO ordering preserved
   - Pro: Consumer sees complete sequence
   - Con: Producer may wait for slow predecessor

**Decision:** Blocking commit (option 2)

**Rationale:** FIFO ordering is a fundamental ring buffer property. Allowing gaps would complicate consumer logic and potentially lose data.

**Rationale Source:** while (ck_pr_load_uint(&ring->p_tail) != producer) ck_pr_stall()

**Consequences:**
- Producers may briefly wait for predecessors
- Strict FIFO ordering maintained
- Single slow producer can delay others

---

## Decision: Four concurrency variants (SPSC, SPMC, MPSC, MPMC)

**Context:**
Different use cases have different producer/consumer patterns.

**Options Considered:**

1. Single MPMC implementation for all cases
   - Pro: Simple API
   - Con: Unnecessary overhead for simpler patterns

2. Specialized variants per pattern
   - Pro: Optimal performance per use case
   - Pro: SPSC can be wait-free
   - Con: Larger API surface

**Decision:** Specialized variants (option 2)

**Rationale:** SPSC can be completely wait-free (no CAS needed). Using MPMC for SPSC would add unnecessary atomic operations and reduce performance.

**Rationale Source:** Separate _spsc, _spmc, _mpsc, _mpmc function variants

**Consequences:**
- User must choose correct variant for their pattern
- SPSC: wait-free, minimal overhead
- SPMC/MPSC: CAS on one side only
- MPMC: CAS on both sides, most general

---

## Decision: Try-dequeue variant for MPMC

**Context:**
Sometimes caller wants to attempt dequeue without retrying.

**Options Considered:**

1. Only provide blocking dequeue
   - Pro: Simpler API
   - Con: Cannot easily integrate with custom retry logic

2. Provide both dequeue and trydequeue
   - Pro: trydequeue enables custom backoff strategies
   - Pro: Wait-free guarantee for single attempt
   - Con: Slightly larger API

**Decision:** Provide both variants (option 2)

**Rationale:** trydequeue gives users control over retry behavior, useful for implementing custom backoff or time-bounded operations.

**Rationale Source:** ck_ring_trydequeue_mpmc with single CAS attempt

**Consequences:**
- dequeue: loops until success or empty
- trydequeue: single attempt, immediate return
- User can build custom retry logic on trydequeue

---

## Decision: Type-safe variants via CK_RING_PROTOTYPE macro

**Context:**
The base ring buffer stores void pointers. Users may want type-safe variants.

**Options Considered:**

1. Only void pointer interface
   - Pro: Simple implementation
   - Con: No type safety
   - Con: Cannot store structures by value

2. Macro-generated type-safe variants
   - Pro: Type safety
   - Pro: Can store arbitrary-sized data
   - Con: Macro complexity

**Decision:** Macro-generated type-safe variants (option 2)

**Rationale:** CK_RING_PROTOTYPE(name, type) generates type-safe inline functions for any structure type. This provides type safety and enables storing data larger than a pointer.

**Rationale Source:** CK_RING_PROTOTYPE macro definition

**Consequences:**
- User invokes CK_RING_PROTOTYPE once per type
- Generated functions work with that type
- memcpy used for data transfer (not just pointer assignment)

---

## Decision: One slot reserved for full/empty distinction

**Context:**
With only head and tail indices, full and empty conditions look the same.

**Options Considered:**

1. Use separate count variable
   - Pro: Full capacity usable
   - Con: Extra atomic variable, contention

2. Reserve one slot (capacity = size - 1)
   - Pro: No extra variable
   - Pro: Simple full/empty check
   - Con: One slot wasted

**Decision:** Reserve one slot (option 2)

**Rationale:** Avoiding an extra atomic counter reduces contention and simplifies the implementation. The one-slot overhead is typically negligible.

**Rationale Source:** Full check: (delta & mask) == (consumer & mask)

**Consequences:**
- Effective capacity is size - 1
- Empty: producer == consumer
- Full: (producer + 1) & mask == consumer & mask
