# Module: ck_ring — Test Specification

## Conformance Tests

### TEST-001: init_creates_empty_ring

**Category:** basic

**Tests Requirement:** ck_ring_init creates empty ring

**Setup:**
1. Allocate ck_ring structure
2. Allocate buffer of size 16

**Action:**
1. ck_ring_init(&ring, 16)

**Expected Result:**
- ck_ring_size(&ring) == 0
- ck_ring_capacity(&ring) == 16

**Cleanup:** None required.

---

### TEST-002: spsc_enqueue_dequeue_single

**Category:** basic

**Tests Requirement:** SPSC enqueue/dequeue works for single element

**Setup:**
1. Initialize ring with size 8
2. Allocate buffer

**Action:**
1. result = ck_ring_enqueue_spsc(&ring, buffer, ptr)
2. success = ck_ring_dequeue_spsc(&ring, buffer, &out)

**Expected Result:**
- enqueue returns true
- dequeue returns true
- out == ptr
- ck_ring_size(&ring) == 0 after dequeue

**Cleanup:** None required.

---

### TEST-003: fifo_order

**Category:** basic

**Tests Requirement:** Ring maintains FIFO order

**Setup:**
1. Initialize ring with size 16
2. Allocate pointers ptr1, ptr2, ptr3

**Action:**
1. Enqueue ptr1, ptr2, ptr3 in order
2. Dequeue three times

**Expected Result:**
- First dequeue returns ptr1
- Second dequeue returns ptr2
- Third dequeue returns ptr3

**Cleanup:** None required.

---

### TEST-004: full_ring_enqueue_fails

**Category:** basic

**Tests Requirement:** Enqueue fails when ring is full

**Setup:**
1. Initialize ring with size 4 (effective capacity 3)

**Action:**
1. Enqueue 3 elements
2. Attempt 4th enqueue

**Expected Result:**
- First 3 enqueues succeed
- 4th enqueue returns false
- Ring size is 3

**Cleanup:** None required.

---

### TEST-005: empty_ring_dequeue_fails

**Category:** basic

**Tests Requirement:** Dequeue fails when ring is empty

**Setup:**
1. Initialize empty ring

**Action:**
1. Attempt dequeue on empty ring

**Expected Result:**
- dequeue returns false
- data pointer unchanged

**Cleanup:** None required.

---

### TEST-006: ring_valid_consistent

**Category:** basic

**Tests Requirement:** ck_ring_valid detects consistent state

**Setup:**
1. Initialize ring with size 16
2. Enqueue 5 elements, dequeue 3

**Action:**
1. Call ck_ring_valid(&ring)

**Expected Result:**
- Returns true
- Ring size is 2

**Cleanup:** None required.

---

### TEST-007: spsc_concurrent

**Category:** concurrent

**Tests Requirement:** SPSC works with concurrent producer/consumer

**Setup:**
1. Initialize ring with size 1024
2. Create producer thread
3. Create consumer thread
4. N = 100000 messages

**Action:**
1. Producer: enqueue N pointers (values 0..N-1), spin on full
2. Consumer: dequeue and sum values
3. Wait for both threads

**Expected Result:**
- All N elements transferred
- Sum of values == N*(N-1)/2
- FIFO order preserved

**Cleanup:**
1. Join threads

---

### TEST-008: mpmc_concurrent

**Category:** concurrent

**Tests Requirement:** MPMC works with multiple producers and consumers

**Setup:**
1. Initialize ring with size 1024
2. Create P producer threads (P = 4)
3. Create C consumer threads (C = 4)
4. Each producer sends M messages (M = 10000)

**Action:**
1. Producers: enqueue M pointers each
2. Consumers: dequeue until all P*M received
3. Track all received values

**Expected Result:**
- All P*M elements received
- No duplicates
- No losses

**Cleanup:**
1. Join all threads

---

### TEST-009: spmc_single_producer_multi_consumer

**Category:** concurrent

**Tests Requirement:** SPMC works correctly

**Setup:**
1. Initialize ring with size 512
2. Create 1 producer thread
3. Create C consumer threads (C = 4)
4. N = 40000 messages

**Action:**
1. Producer: enqueue N messages
2. Consumers: dequeue and count
3. Wait for completion

**Expected Result:**
- Sum of consumer counts == N
- No element received twice

**Cleanup:**
1. Join threads

---

### TEST-010: mpsc_multi_producer_single_consumer

**Category:** concurrent

**Tests Requirement:** MPSC works correctly

**Setup:**
1. Initialize ring with size 512
2. Create P producer threads (P = 4)
3. Create 1 consumer thread
4. Each producer sends M messages (M = 10000)

**Action:**
1. Producers: enqueue M messages each
2. Consumer: dequeue and verify all received
3. Wait for completion

**Expected Result:**
- Consumer receives exactly P*M messages
- Per-producer ordering preserved

**Cleanup:**
1. Join threads

---

### TEST-011: reserve_commit_zero_copy

**Category:** basic

**Tests Requirement:** Reserve/commit enables zero-copy

**Setup:**
1. Initialize ring with size 16
2. Define struct with multiple fields

**Action:**
1. ptr = ck_ring_enqueue_reserve_spsc(&ring, buffer)
2. Write directly to *ptr
3. ck_ring_enqueue_commit_spsc(&ring)
4. Dequeue and verify

**Expected Result:**
- Reserved pointer is valid
- Data written directly is received
- No intermediate copy needed

**Cleanup:** None required.

---

### TEST-012: trydequeue_returns_immediately

**Category:** basic

**Tests Requirement:** trydequeue doesn't loop

**Setup:**
1. Initialize ring with size 8
2. Enqueue 3 elements

**Action:**
1. Spawn multiple threads calling trydequeue concurrently
2. Count successes and failures

**Expected Result:**
- Exactly 3 total successes
- All calls return immediately (timing)
- No blocking

**Cleanup:**
1. Join threads

---

### TEST-013: size_reports_current_count

**Category:** basic

**Tests Requirement:** ck_ring_size returns correct count

**Setup:**
1. Initialize ring with size 16

**Action:**
1. Check size (should be 0)
2. Enqueue 5 elements, check size
3. Dequeue 2 elements, check size

**Expected Result:**
- Initial size: 0
- After 5 enqueues: 5
- After 2 dequeues: 3

**Cleanup:** None required.

---

### TEST-014: non_power_of_2_size

**Category:** edge_case

**Tests Requirement:** Non-power-of-2 size causes problems

**Setup:**
1. Initialize ring with size 10 (not power of 2)

**Action:**
1. Enqueue/dequeue several elements
2. Observe behavior

**Expected Result:**
- ck_ring_valid returns false
- Index wrapping incorrect (undefined behavior)

**Cleanup:** None required.

**Note:** This tests precondition violation; behavior is undefined.

---

### TEST-015: mpmc_high_contention

**Category:** stress

**Tests Requirement:** MPMC survives high contention

**Setup:**
1. Initialize ring with size 64 (small to force contention)
2. Create 8 producer threads
3. Create 8 consumer threads
4. Each producer sends 50000 messages

**Action:**
1. All threads start simultaneously
2. Producers enqueue, spinning on full
3. Consumers dequeue, spinning on empty
4. Track total sent and received

**Expected Result:**
- All 8*50000 = 400000 messages received
- No crashes or hangs
- Progress made despite contention

**Cleanup:**
1. Join all threads

---

### TEST-016: repair_after_interrupted_enqueue

**Category:** edge_case

**Tests Requirement:** ck_ring_repair fixes p_tail != p_head

**Setup:**
1. Initialize ring
2. Simulate interrupted MP enqueue (manually set p_head > p_tail)

**Action:**
1. Call ck_ring_valid (should return false)
2. Call ck_ring_repair
3. Call ck_ring_valid again

**Expected Result:**
- First valid returns false
- repair returns true
- Second valid returns true
- p_tail == p_head after repair

**Cleanup:** None required.

---

## Coverage Matrix

| Requirement (spec.md) | Test IDs | Status |
|-----------------------|----------|--------|
| ck_ring_init | TEST-001 | Covered |
| ck_ring_size | TEST-001, TEST-013 | Covered |
| ck_ring_capacity | TEST-001 | Covered |
| ck_ring_valid | TEST-006, TEST-016 | Covered |
| ck_ring_repair | TEST-016 | Covered |
| ck_ring_enqueue_spsc | TEST-002, TEST-003 | Covered |
| ck_ring_dequeue_spsc | TEST-002, TEST-003 | Covered |
| Full ring behavior | TEST-004 | Covered |
| Empty ring behavior | TEST-005 | Covered |
| SPSC concurrent | TEST-007 | Covered |
| MPMC concurrent | TEST-008, TEST-015 | Covered |
| SPMC concurrent | TEST-009 | Covered |
| MPSC concurrent | TEST-010 | Covered |
| Reserve/commit | TEST-011 | Covered |
| trydequeue | TEST-012 | Covered |
| FIFO order | TEST-003 | Covered |

## Test Gaps

| Requirement | Reason | Recommendation |
|-------------|--------|----------------|
| CK_RING_PROTOTYPE | Macro test | Add test for custom type variant |
| Size variants (_size suffix) | Similar to base | Add basic test |
| Memory ordering | Difficult to test | Use memory model checker |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_ring regression | regressions/ck_ring/ | TEST-001 through TEST-016 |
