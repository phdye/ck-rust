# Module: ck_pflock — Test Specification

## Conformance Tests

### TEST-001: init_unlocked

**Category:** basic

**Setup:** Allocate ck_pflock

**Action:** ck_pflock_init(&pf)

**Expected Result:** All fields are 0

---

### TEST-002: write_lock_unlock

**Category:** basic

**Setup:** Initialize lock

**Action:** Write lock then unlock

**Expected Result:** Lock acquired and released without error

---

### TEST-003: read_lock_unlock

**Category:** basic

**Setup:** Initialize lock

**Action:** Read lock then unlock

**Expected Result:** Lock acquired and released without error

---

### TEST-004: multiple_readers

**Category:** concurrent

**Setup:** Initialize lock, create N reader threads

**Action:** All threads acquire read lock simultaneously

**Expected Result:** All readers hold lock concurrently, no deadlock

---

### TEST-005: writer_excludes_readers

**Category:** concurrent

**Setup:** Initialize lock, create reader and writer threads

**Action:** Writer holds lock, reader attempts to acquire

**Expected Result:** Reader blocks until writer releases

---

### TEST-006: phase_fairness

**Category:** concurrent

**Setup:** Initialize lock with active readers

**Action:** Writer requests lock while readers active

**Expected Result:**
- Readers present before writer complete
- New readers wait for next phase

---

### TEST-007: writer_fifo

**Category:** concurrent

**Setup:** Multiple writers request lock in sequence

**Action:** Writers 1, 2, 3 request lock

**Expected Result:** Writers acquire in order 1, 2, 3

---

### TEST-008: no_starvation_stress

**Category:** stress

**Setup:** 4 reader threads, 4 writer threads

**Action:** All threads continuously lock/unlock for duration

**Expected Result:**
- All threads make progress
- No starvation detected

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| ck_pflock_init | TEST-001 | Covered |
| ck_pflock_write_lock/unlock | TEST-002, TEST-005 | Covered |
| ck_pflock_read_lock/unlock | TEST-003, TEST-004 | Covered |
| Phase fairness | TEST-006 | Covered |
| Writer FIFO | TEST-007 | Covered |
| No starvation | TEST-008 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_pflock regression | regressions/ck_pflock/ | TEST-001 through TEST-008 |
