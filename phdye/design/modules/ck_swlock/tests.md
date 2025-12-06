# Module: ck_swlock — Test Specification

## Conformance Tests

### TEST-001: init_basic

**Category:** basic

**Action:** Initialize ck_swlock

**Expected Result:** value = 0

---

### TEST-002: write_lock_unlock

**Category:** basic

**Setup:** Initialize swlock

**Action:** write_lock, write_unlock

**Expected Result:** Lock acquired and released

---

### TEST-003: write_latch_unlatch

**Category:** basic

**Setup:** Initialize swlock

**Action:** write_latch, write_unlatch

**Expected Result:** Lock acquired with latch, released

---

### TEST-004: read_lock_unlock

**Category:** basic

**Setup:** Initialize swlock

**Action:** read_lock, read_unlock

**Expected Result:** Lock acquired and released

---

### TEST-005: multiple_readers

**Category:** concurrent

**Setup:** Initialize swlock

**Action:** Multiple threads acquire read lock

**Expected Result:** All readers hold lock

---

### TEST-006: writer_blocks_readers

**Category:** concurrent

**Setup:** Initialize swlock

**Action:** Writer holds lock, readers attempt

**Expected Result:** Readers blocked

---

### TEST-007: latch_blocks_new_readers

**Category:** concurrent

**Setup:** Writer latches lock

**Action:** New readers attempt

**Expected Result:** Readers cannot enter

---

### TEST-008: write_trylock_success

**Category:** basic

**Setup:** Initialize swlock

**Action:** write_trylock on unlocked

**Expected Result:** Returns true, value = WRITER_BIT

---

### TEST-009: write_trylock_failure_reader

**Category:** concurrent

**Setup:** Reader holds lock

**Action:** write_trylock

**Expected Result:** Returns false

---

### TEST-010: read_trylock_success

**Category:** basic

**Setup:** Initialize swlock

**Action:** read_trylock on unlocked

**Expected Result:** Returns true

---

### TEST-011: read_trylock_writer_present

**Category:** concurrent

**Setup:** Writer holds lock

**Action:** read_trylock

**Expected Result:** Returns false

---

### TEST-012: write_downgrade

**Category:** basic

**Setup:** Acquire write lock

**Action:** write_downgrade

**Expected Result:** Holds read lock

---

### TEST-013: locked_predicates

**Category:** basic

**Setup:** Various states

**Action:** Check locked, locked_writer, locked_reader

**Expected Result:** Correct for each state

---

### TEST-014: reader_backout_on_latch

**Category:** concurrent

**Setup:** Writer beginning latch

**Action:** Reader increments then backs out

**Expected Result:** Reader count returns to 0

---

### TEST-015: stress_mixed

**Category:** stress

**Setup:** Many threads

**Action:** Mixed read/write/latch operations

**Expected Result:** Correctness maintained

---

### TEST-016: latch_vs_lock_semantics

**Category:** concurrent

**Setup:** Compare write_lock vs write_latch

**Action:** Readers during writer spin

**Expected Result:** write_lock allows reader increment, latch doesn't

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| Init | TEST-001 | Covered |
| Write lock/unlock | TEST-002 | Covered |
| Write latch/unlatch | TEST-003 | Covered |
| Read lock/unlock | TEST-004 | Covered |
| Multiple readers | TEST-005 | Covered |
| Writer blocks | TEST-006 | Covered |
| Latch blocks | TEST-007 | Covered |
| Write trylock | TEST-008, TEST-009 | Covered |
| Read trylock | TEST-010, TEST-011 | Covered |
| Downgrade | TEST-012 | Covered |
| Predicates | TEST-013 | Covered |
| Backout | TEST-014 | Covered |
| Stress | TEST-015 | Covered |
| Latch semantics | TEST-016 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_swlock regression | regressions/ck_swlock/ | TEST-001 through TEST-016 |
