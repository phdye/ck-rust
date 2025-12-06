# Module: ck_rwlock — Test Specification

## Conformance Tests

### TEST-001: init_basic

**Category:** basic

**Action:** Initialize ck_rwlock

**Expected Result:** writer=0, n_readers=0

---

### TEST-002: write_lock_unlock

**Category:** basic

**Setup:** Initialize rwlock

**Action:** write_lock, write_unlock

**Expected Result:** Lock acquired and released

---

### TEST-003: read_lock_unlock

**Category:** basic

**Setup:** Initialize rwlock

**Action:** read_lock, read_unlock

**Expected Result:** Lock acquired and released

---

### TEST-004: multiple_readers

**Category:** concurrent

**Setup:** Initialize rwlock

**Action:** Multiple threads acquire read lock

**Expected Result:** All readers hold lock concurrently

---

### TEST-005: writer_blocks_readers

**Category:** concurrent

**Setup:** Initialize rwlock

**Action:** Writer holds lock, readers attempt

**Expected Result:** Readers blocked

---

### TEST-006: writer_waits_readers

**Category:** concurrent

**Setup:** Initialize rwlock, readers active

**Action:** Writer attempts lock

**Expected Result:** Writer waits for readers to exit

---

### TEST-007: write_trylock_success

**Category:** basic

**Setup:** Initialize rwlock

**Action:** write_trylock on unlocked

**Expected Result:** Returns true

---

### TEST-008: write_trylock_failure

**Category:** concurrent

**Setup:** Reader holds lock

**Action:** write_trylock

**Expected Result:** Returns false

---

### TEST-009: read_trylock_success

**Category:** basic

**Setup:** Initialize rwlock

**Action:** read_trylock on unlocked

**Expected Result:** Returns true

---

### TEST-010: read_trylock_writer_present

**Category:** concurrent

**Setup:** Writer holds lock

**Action:** read_trylock

**Expected Result:** Returns false

---

### TEST-011: write_downgrade

**Category:** basic

**Setup:** Acquire write lock

**Action:** write_downgrade

**Expected Result:** Holds read lock, n_readers=1

---

### TEST-012: locked_predicates

**Category:** basic

**Setup:** Initialize rwlock

**Action:** Check locked, locked_writer, locked_reader

**Expected Result:** Correct values for each state

---

### TEST-013: recursive_write_lock

**Category:** basic

**Setup:** Initialize recursive rwlock

**Action:** write_lock twice with same tid

**Expected Result:** wc=2, lock held

---

### TEST-014: recursive_write_unlock

**Category:** basic

**Setup:** Recursive lock held with wc=2

**Action:** write_unlock twice

**Expected Result:** First decrements wc, second releases

---

### TEST-015: recursive_trylock_reentry

**Category:** basic

**Setup:** Recursive lock held

**Action:** write_trylock with same tid

**Expected Result:** Returns true, wc incremented

---

### TEST-016: recursive_read_operations

**Category:** basic

**Setup:** Initialize recursive rwlock

**Action:** read_lock, read_unlock

**Expected Result:** Delegates to base rwlock

---

### TEST-017: stress_mixed

**Category:** stress

**Setup:** Initialize rwlock, many threads

**Action:** Mixed read/write workload

**Expected Result:** No deadlock, correctness maintained

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| Init | TEST-001 | Covered |
| Write lock/unlock | TEST-002 | Covered |
| Read lock/unlock | TEST-003 | Covered |
| Multiple readers | TEST-004 | Covered |
| Writer blocks | TEST-005 | Covered |
| Writer waits | TEST-006 | Covered |
| Write trylock | TEST-007, TEST-008 | Covered |
| Read trylock | TEST-009, TEST-010 | Covered |
| Downgrade | TEST-011 | Covered |
| Predicates | TEST-012 | Covered |
| Recursive write | TEST-013, TEST-014, TEST-015 | Covered |
| Recursive read | TEST-016 | Covered |
| Stress | TEST-017 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_rwlock regression | regressions/ck_rwlock/ | TEST-001 through TEST-017 |
