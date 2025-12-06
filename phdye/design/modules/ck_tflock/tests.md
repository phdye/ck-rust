# Module: ck_tflock — Test Specification

## Conformance Tests

### TEST-001: init_unlocked

**Category:** basic

**Action:** Initialize lock

**Expected Result:** request = completion = 0

---

### TEST-002: write_lock_unlock

**Category:** basic

**Action:** Write lock then unlock

**Expected Result:** Lock acquired and released

---

### TEST-003: read_lock_unlock

**Category:** basic

**Action:** Read lock then unlock

**Expected Result:** Lock acquired and released

---

### TEST-004: task_fifo_ordering

**Category:** concurrent

**Setup:** Create threads: R1, W1, R2, R3

**Action:** Threads request lock in order R1, W1, R2, R3

**Expected Result:** Acquisition order is R1, W1, R2, R3 (strict FIFO)

---

### TEST-005: readers_after_writer_wait

**Category:** concurrent

**Setup:** Writer holds lock, readers arrive

**Action:** Readers request while writer active

**Expected Result:** Readers wait until writer completes

---

### TEST-006: no_starvation_stress

**Category:** stress

**Setup:** Multiple reader and writer threads

**Action:** Continuous lock/unlock operations

**Expected Result:** All threads make progress

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| ck_tflock_ticket_init | TEST-001 | Covered |
| Write operations | TEST-002 | Covered |
| Read operations | TEST-003 | Covered |
| Task-fair ordering | TEST-004, TEST-005 | Covered |
| No starvation | TEST-006 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_tflock regression | regressions/ck_tflock/ | TEST-001 through TEST-006 |
