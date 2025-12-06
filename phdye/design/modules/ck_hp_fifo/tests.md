# Module: ck_hp_fifo — Test Specification

## Conformance Tests

### TEST-001: init_basic

**Category:** basic

**Action:** Initialize HP FIFO with stub

**Expected Result:** head = tail = stub, stub->next = NULL

---

### TEST-002: enqueue_dequeue_single

**Category:** basic

**Setup:** Initialize HP FIFO

**Action:** Enqueue value, dequeue value

**Expected Result:** Dequeued value matches, entry returned

---

### TEST-003: dequeue_empty

**Category:** basic

**Setup:** Initialize empty HP FIFO

**Action:** Attempt dequeue

**Expected Result:** Returns NULL

---

### TEST-004: fifo_ordering

**Category:** basic

**Setup:** Initialize HP FIFO

**Action:** Enqueue A, B, C; dequeue 3 times

**Expected Result:** Returns A, B, C in order

---

### TEST-005: tryenqueue_success

**Category:** basic

**Setup:** Initialize HP FIFO

**Action:** tryenqueue on empty queue

**Expected Result:** Returns true

---

### TEST-006: trydequeue_empty

**Category:** basic

**Setup:** Initialize empty HP FIFO

**Action:** trydequeue

**Expected Result:** Returns NULL

---

### TEST-007: multi_producer

**Category:** concurrent

**Setup:** HP FIFO, HP records for 4 producers

**Action:** Each producer enqueues 1000 items

**Expected Result:** All 4000 items dequeued

---

### TEST-008: multi_consumer

**Category:** concurrent

**Setup:** HP FIFO, HP records for 4 consumers

**Action:** Enqueue 4000 items, consumers dequeue

**Expected Result:** All items consumed exactly once

---

### TEST-009: multi_producer_consumer

**Category:** concurrent

**Setup:** HP FIFO, 4 producers, 4 consumers

**Action:** Concurrent enqueue/dequeue

**Expected Result:** All enqueued items dequeued

---

### TEST-010: hazard_protection

**Category:** concurrent

**Setup:** HP FIFO with contention

**Action:** Dequeue while other threads operate

**Expected Result:** No use-after-free (sanitizer clean)

---

### TEST-011: entry_reclamation

**Category:** basic

**Setup:** HP FIFO, dequeue entries

**Action:** Reclaim via ck_hp_free

**Expected Result:** Entries reclaimed after grace period

---

### TEST-012: deinit

**Category:** basic

**Setup:** HP FIFO with entries

**Action:** deinit

**Expected Result:** Returns stub, queue invalidated

---

### TEST-013: stress_lock_freedom

**Category:** stress

**Setup:** HP FIFO, many threads

**Action:** Heavy concurrent access

**Expected Result:** Progress always made (no deadlock)

---

### TEST-014: stress_reclamation

**Category:** stress

**Setup:** HP FIFO, continuous operation

**Action:** Long duration with continuous reclamation

**Expected Result:** Memory bounded, no leaks

---

### TEST-015: iteration_foreach

**Category:** basic

**Setup:** HP FIFO with entries

**Action:** Use CK_HP_FIFO_FOREACH

**Expected Result:** Iterates all entries in order

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| Init/deinit | TEST-001, TEST-012 | Covered |
| Enqueue/dequeue | TEST-002, TEST-003 | Covered |
| FIFO ordering | TEST-004 | Covered |
| Try operations | TEST-005, TEST-006 | Covered |
| Multi-producer | TEST-007 | Covered |
| Multi-consumer | TEST-008 | Covered |
| MPMC | TEST-009 | Covered |
| Hazard protection | TEST-010 | Covered |
| Reclamation | TEST-011, TEST-014 | Covered |
| Stress | TEST-013, TEST-014 | Covered |
| Iteration | TEST-015 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_hp_fifo regression | regressions/ck_hp/ck_hp_fifo/ | TEST-001 through TEST-015 |
