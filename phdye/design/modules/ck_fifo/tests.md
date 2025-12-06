# Module: ck_fifo — Test Specification

## Conformance Tests

### TEST-001: spsc_init

**Category:** basic

**Action:** Initialize SPSC queue with stub

**Expected Result:** head = tail = stub, stub->next = NULL

---

### TEST-002: spsc_enqueue_dequeue

**Category:** basic

**Setup:** Initialize SPSC queue

**Action:** Enqueue value, dequeue value

**Expected Result:** Dequeued value matches enqueued

---

### TEST-003: spsc_empty_dequeue

**Category:** basic

**Setup:** Initialize empty SPSC queue

**Action:** Attempt dequeue

**Expected Result:** Returns false

---

### TEST-004: spsc_fifo_ordering

**Category:** basic

**Setup:** Initialize SPSC queue

**Action:** Enqueue A, B, C; dequeue 3 times

**Expected Result:** Returns A, B, C in order

---

### TEST-005: spsc_recycle

**Category:** basic

**Setup:** Initialize SPSC queue, enqueue/dequeue several entries

**Action:** Call recycle

**Expected Result:** Returns recyclable entry

---

### TEST-006: spsc_isempty

**Category:** basic

**Setup:** Initialize SPSC queue

**Action:** Check isempty before/after enqueue

**Expected Result:** true when empty, false after enqueue

---

### TEST-007: spsc_concurrent

**Category:** concurrent

**Setup:** SPSC queue, 1 producer thread, 1 consumer thread

**Action:** Producer enqueues 10000 items, consumer dequeues

**Expected Result:** All items received in order

---

### TEST-008: mpmc_init

**Category:** basic

**Action:** Initialize MPMC queue with stub

**Expected Result:** head = tail = stub, generations = 0

---

### TEST-009: mpmc_enqueue_dequeue

**Category:** basic

**Setup:** Initialize MPMC queue

**Action:** Enqueue value, dequeue value

**Expected Result:** Dequeued value matches, garbage returned

---

### TEST-010: mpmc_empty_dequeue

**Category:** basic

**Setup:** Initialize empty MPMC queue

**Action:** Attempt dequeue

**Expected Result:** Returns false

---

### TEST-011: mpmc_fifo_ordering

**Category:** basic

**Setup:** Initialize MPMC queue

**Action:** Enqueue A, B, C; dequeue 3 times

**Expected Result:** Returns A, B, C in order

---

### TEST-012: mpmc_tryenqueue_success

**Category:** basic

**Setup:** Initialize MPMC queue

**Action:** tryenqueue on empty queue

**Expected Result:** Returns true

---

### TEST-013: mpmc_trydequeue_empty

**Category:** basic

**Setup:** Initialize empty MPMC queue

**Action:** trydequeue

**Expected Result:** Returns false

---

### TEST-014: mpmc_multi_producer

**Category:** concurrent

**Setup:** MPMC queue, 4 producer threads

**Action:** Each producer enqueues 1000 items

**Expected Result:** All 4000 items dequeued

---

### TEST-015: mpmc_multi_consumer

**Category:** concurrent

**Setup:** MPMC queue, 4 consumer threads

**Action:** Enqueue 4000 items, consumers dequeue

**Expected Result:** All items consumed, no duplicates

---

### TEST-016: mpmc_multi_producer_consumer

**Category:** concurrent

**Setup:** MPMC queue, 4 producers, 4 consumers

**Action:** Producers enqueue, consumers dequeue concurrently

**Expected Result:** All enqueued items dequeued exactly once

---

### TEST-017: stress_spsc

**Category:** stress

**Setup:** SPSC queue, long duration

**Action:** Continuous enqueue/dequeue for 60 seconds

**Expected Result:** No corruption, FIFO ordering maintained

---

### TEST-018: stress_mpmc

**Category:** stress

**Setup:** MPMC queue, many threads

**Action:** Heavy concurrent access

**Expected Result:** Lock-free progress, correctness

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| SPSC init | TEST-001 | Covered |
| SPSC enqueue/dequeue | TEST-002, TEST-003 | Covered |
| SPSC FIFO ordering | TEST-004 | Covered |
| SPSC recycle | TEST-005 | Covered |
| SPSC isempty | TEST-006 | Covered |
| SPSC concurrent | TEST-007 | Covered |
| MPMC init | TEST-008 | Covered |
| MPMC enqueue/dequeue | TEST-009, TEST-010 | Covered |
| MPMC FIFO ordering | TEST-011 | Covered |
| MPMC try operations | TEST-012, TEST-013 | Covered |
| MPMC concurrent | TEST-014, TEST-015, TEST-016 | Covered |
| Stress | TEST-017, TEST-018 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_fifo regression | regressions/ck_fifo/ | TEST-001 through TEST-018 |
