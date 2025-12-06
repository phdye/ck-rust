# Module: ck_bitmap — Design Decisions

## Decision: Word-granularity atomic operations

**Context:**
Individual bit operations could be implemented various ways.

**Options Considered:**

1. Lock per bitmap
   - Pro: Simple implementation
   - Con: Contention on unrelated bits

2. Lock per word
   - Pro: Finer granularity than whole bitmap
   - Con: Still some unnecessary contention

3. Atomic word operations (OR/AND)
   - Pro: Lock-free, minimal contention
   - Pro: Bits in different words never contend
   - Con: Bits in same word may briefly contend

**Decision:** Atomic word operations (option 3)

**Rationale:** Using ck_pr_or_uint and ck_pr_and_uint for set/reset provides lock-free operation. Adjacent bits in the same word may contend briefly, but atomic RMW operations ensure correctness.

**Rationale Source:** Implementation using ck_pr_or_uint, ck_pr_and_uint

**Consequences:**
- set/reset are wait-free
- Bits in same word may contend
- No external locking needed for individual bit operations

---

## Decision: Non-linearizable bulk operations

**Context:**
Bulk operations like union and intersection operate on multiple words.

**Options Considered:**

1. Lock entire bitmap for bulk ops
   - Pro: Linearizable
   - Con: Blocks all concurrent access

2. CAS-based word-by-word with retry
   - Pro: More atomic
   - Con: Complex, potential livelock

3. Per-word atomic operations without global atomicity
   - Pro: Simple, efficient
   - Con: Not linearizable as a whole

**Decision:** Per-word atomic operations (option 3)

**Rationale:** Full linearizability for bulk operations would require significant overhead. Most use cases can tolerate per-word atomicity, and users needing stronger guarantees can add external synchronization.

**Rationale Source:** Comment "not a linearized operation" in union/intersection

**Consequences:**
- Each word updated atomically
- Whole bitmap not updated atomically
- Concurrent modifications may interleave at word boundaries
- Users need external sync for atomic bulk updates

---

## Decision: Flexible array member for bit storage

**Context:**
Bitmap size varies at runtime.

**Options Considered:**

1. Fixed maximum size
   - Pro: Simple allocation
   - Con: Wastes memory for small bitmaps
   - Con: Cannot support arbitrary sizes

2. Separate allocation for header and bits
   - Pro: Flexible
   - Con: Two allocations, cache miss potential

3. Flexible array member (C99)
   - Pro: Single allocation
   - Pro: Contiguous memory
   - Pro: Portable (C99 feature)

**Decision:** Flexible array member (option 3)

**Rationale:** C99 flexible array members (unsigned int map[]) allow single allocation for header and bits, with contiguous memory layout.

**Rationale Source:** struct ck_bitmap { unsigned int n_bits; unsigned int map[]; }

**Consequences:**
- User calls ck_bitmap_size() to determine allocation size
- Single malloc/free for entire bitmap
- Good cache locality

---

## Decision: CK_BITMAP_INSTANCE macro for stack allocation

**Context:**
Sometimes bitmap size is known at compile time.

**Options Considered:**

1. Always require dynamic allocation
   - Pro: Uniform interface
   - Con: Heap overhead for small fixed bitmaps

2. Provide stack allocation macro
   - Pro: Zero heap overhead
   - Pro: Known size enables optimization
   - Con: Additional API complexity

**Decision:** Provide CK_BITMAP_INSTANCE macro (option 2)

**Rationale:** For compile-time-known sizes, stack allocation eliminates heap overhead. The macro creates a union that allows both raw access and ck_bitmap pointer access.

**Rationale Source:** CK_BITMAP_INSTANCE(n_entries) macro definition

**Consequences:**
- CK_BITMAP_INSTANCE(100) creates 100-bit bitmap on stack
- Union provides both .content access and .bitmap pointer
- Works with all ck_bitmap_* functions via CK_BITMAP() macro

---

## Decision: Iterator with cached block

**Context:**
Iterating over set bits should be efficient.

**Options Considered:**

1. Scan bit-by-bit
   - Pro: Simple
   - Con: O(n) for sparse bitmaps

2. Find-first-set per word with caching
   - Pro: O(popcount) for sparse bitmaps
   - Pro: Efficient skip over zero words
   - Con: More complex state

**Decision:** Find-first-set with caching (option 2)

**Rationale:** Using ctz (count trailing zeros) to find set bits, and caching the current word, allows O(popcount) iteration. Zero words are skipped entirely.

**Rationale Source:** Iterator using ck_cc_ctz, cache manipulation

**Consequences:**
- Fast iteration over sparse bitmaps
- Iterator state caches current word
- cache &= (cache - 1) clears lowest set bit efficiently

---

## Decision: Limit parameter for count/empty/full

**Context:**
Sometimes only a prefix of the bitmap is relevant.

**Options Considered:**

1. Always operate on entire bitmap
   - Pro: Simpler API
   - Con: Cannot query partial bitmap

2. Limit parameter for partial queries
   - Pro: Flexible
   - Pro: Efficient for prefix queries
   - Con: Slightly more complex

**Decision:** Limit parameter (option 2)

**Rationale:** The limit parameter allows querying a prefix of the bitmap without scanning the entire thing. If limit > n_bits, it's truncated to n_bits.

**Rationale Source:** ck_bitmap_count(bitmap, limit), ck_bitmap_empty(bitmap, limit)

**Consequences:**
- count/empty/full take limit parameter
- limit > n_bits is handled (truncated)
- Efficient partial queries

---

## Decision: Requires specific ck_pr features

**Context:**
Not all platforms support all atomic operations.

**Options Considered:**

1. Provide fallbacks for missing operations
   - Pro: Broader platform support
   - Con: Performance penalty

2. Require specific features, fail at compile time
   - Pro: Guaranteed performance
   - Pro: Clear requirements
   - Con: Not available everywhere

**Decision:** Require specific features (option 2)

**Rationale:** ck_bitmap requires CK_F_PR_LOAD_UINT, CK_F_PR_STORE_UINT, CK_F_PR_AND_UINT, CK_F_PR_OR_UINT, and CK_F_CC_CTZ. Without these, the module cannot provide correct concurrent behavior.

**Rationale Source:** #error directive if features missing

**Consequences:**
- Compile-time error on unsupported platforms
- Users know upfront if bitmap is available
- No runtime performance degradation
