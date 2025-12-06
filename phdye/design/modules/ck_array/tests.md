# Module: ck_array — Test Specification

## Conformance Tests

### TEST-001: init_basic

**Category:** basic

**Action:** Initialize array with capacity 16

**Expected Result:** initialized() returns true, length() returns 0

---

### TEST-002: put_single

**Category:** basic

**Setup:** Initialize array

**Action:** Put one element, commit

**Expected Result:** length() returns 1

---

### TEST-003: put_multiple

**Category:** basic

**Setup:** Initialize array

**Action:** Put 10 elements, commit

**Expected Result:** length() returns 10

---

### TEST-004: put_unique_new

**Category:** basic

**Setup:** Initialize array with elements

**Action:** put_unique with new element

**Expected Result:** Returns 0, length increases

---

### TEST-005: put_unique_exists

**Category:** basic

**Setup:** Initialize array with elements

**Action:** put_unique with existing element

**Expected Result:** Returns 1, length unchanged

---

### TEST-006: remove_element

**Category:** basic

**Setup:** Initialize array with 5 elements

**Action:** Remove middle element, commit

**Expected Result:** length() returns 4, element not in array

---

### TEST-007: commit_empty

**Category:** basic

**Setup:** Initialize array, no modifications

**Action:** Call commit

**Expected Result:** Returns true (no-op)

---

### TEST-008: batch_commit

**Category:** basic

**Setup:** Initialize array

**Action:** Put 5 elements, remove 2, put 3 more, commit

**Expected Result:** length() = 6, all expected elements present

---

### TEST-009: foreach_iteration

**Category:** basic

**Setup:** Initialize array with 10 elements

**Action:** Iterate with CK_ARRAY_FOREACH

**Expected Result:** All 10 elements visited

---

### TEST-010: concurrent_read_write

**Category:** concurrent

**Setup:** Initialize array, reader and writer threads

**Action:** Writer continuously puts/removes/commits, reader iterates

**Expected Result:** Reader always sees consistent state

---

### TEST-011: grow_capacity

**Category:** basic

**Setup:** Initialize array with capacity 2

**Action:** Put 10 elements, commit

**Expected Result:** length() = 10, all elements present

---

### TEST-012: length_before_commit

**Category:** basic

**Setup:** Initialize array

**Action:** Put elements, check length before commit

**Expected Result:** length() returns 0 (uncommitted)

---

### TEST-013: buffer_access

**Category:** basic

**Setup:** Initialize array with elements, commit

**Action:** Call buffer(), access elements directly

**Expected Result:** Correct elements at indices 0..length-1

---

### TEST-014: deinit_cleanup

**Category:** basic

**Setup:** Initialize array with elements

**Action:** Call deinit

**Expected Result:** Resources freed, no memory leak

---

### TEST-015: reader_snapshot_consistency

**Category:** concurrent

**Setup:** Initialize array, spawn readers and writer

**Action:** Writer modifies during reader iteration

**Expected Result:** Each reader sees consistent count throughout iteration

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| Init | TEST-001 | Covered |
| Put | TEST-002, TEST-003 | Covered |
| Put unique | TEST-004, TEST-005 | Covered |
| Remove | TEST-006 | Covered |
| Commit | TEST-007, TEST-008 | Covered |
| Foreach | TEST-009 | Covered |
| Concurrent | TEST-010, TEST-015 | Covered |
| Growth | TEST-011 | Covered |
| Uncommitted | TEST-012 | Covered |
| Buffer | TEST-013 | Covered |
| Deinit | TEST-014 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_array regression | regressions/ck_array/ | TEST-001 through TEST-015 |
