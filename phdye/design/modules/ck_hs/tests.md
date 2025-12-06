# Module: ck_hs — Test Specification

## Conformance Tests

### TEST-001: init_basic

**Category:** basic

**Action:** Initialize hash set with capacity 16

**Expected Result:** Hash set created successfully

---

### TEST-002: put_get_single

**Category:** basic

**Setup:** Initialize hash set

**Action:** Put key, get same key

**Expected Result:** Get returns inserted value

---

### TEST-003: put_duplicate

**Category:** basic

**Setup:** Initialize hash set, put key

**Action:** Put same key again

**Expected Result:** Returns false (duplicate)

---

### TEST-004: put_unique

**Category:** basic

**Setup:** Initialize hash set

**Action:** put_unique with new key

**Expected Result:** Returns true

---

### TEST-005: set_insert

**Category:** basic

**Setup:** Initialize hash set

**Action:** set with new key

**Expected Result:** Returns true, previous = NULL

---

### TEST-006: set_replace

**Category:** basic

**Setup:** Initialize hash set, insert key

**Action:** set with same key, different value

**Expected Result:** Returns true, previous = old value

---

### TEST-007: fas_exists

**Category:** basic

**Setup:** Initialize hash set, insert key

**Action:** fas with same key

**Expected Result:** Returns true, previous = old value

---

### TEST-008: fas_missing

**Category:** basic

**Setup:** Initialize hash set

**Action:** fas with non-existent key

**Expected Result:** Returns false

---

### TEST-009: remove_existing

**Category:** basic

**Setup:** Initialize hash set, insert key

**Action:** Remove key

**Expected Result:** Returns removed value, get returns NULL

---

### TEST-010: remove_missing

**Category:** basic

**Setup:** Initialize hash set

**Action:** Remove non-existent key

**Expected Result:** Returns NULL

---

### TEST-011: grow_capacity

**Category:** basic

**Setup:** Initialize with small capacity, insert many keys

**Action:** Call grow or auto-grow

**Expected Result:** All keys still accessible

---

### TEST-012: gc_tombstones

**Category:** basic

**Setup:** Insert and remove many keys

**Action:** Call gc

**Expected Result:** Tombstone count reduced

---

### TEST-013: count_accuracy

**Category:** basic

**Setup:** Insert 100 keys, remove 30

**Action:** Call count

**Expected Result:** Returns 70

---

### TEST-014: iteration

**Category:** basic

**Setup:** Insert known keys

**Action:** Iterate with ck_hs_next

**Expected Result:** All keys visited exactly once

---

### TEST-015: concurrent_read_write

**Category:** concurrent

**Setup:** Initialize hash set, reader and writer threads

**Action:** Writer inserts/removes, readers lookup

**Expected Result:** Readers see consistent state

---

### TEST-016: apply_function

**Category:** basic

**Setup:** Initialize hash set, insert keys

**Action:** Call apply with transformation function

**Expected Result:** Entry modified as expected

---

### TEST-017: reset_clears_all

**Category:** basic

**Setup:** Initialize hash set, insert keys

**Action:** Call reset

**Expected Result:** count = 0, all get return NULL

---

### TEST-018: stat_accuracy

**Category:** basic

**Setup:** Insert and remove keys

**Action:** Call stat

**Expected Result:** Correct tombstones, n_entries

---

### TEST-019: direct_mode

**Category:** basic

**Setup:** Initialize with CK_HS_MODE_DIRECT

**Action:** Insert integer keys

**Expected Result:** Direct integer storage works

---

### TEST-020: probe_bound

**Category:** stress

**Setup:** Insert many keys causing collisions

**Action:** Check probe_maximum in stats

**Expected Result:** Probe bound reasonable

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| Init | TEST-001 | Covered |
| Put | TEST-002, TEST-003 | Covered |
| Put unique | TEST-004 | Covered |
| Set | TEST-005, TEST-006 | Covered |
| Fas | TEST-007, TEST-008 | Covered |
| Remove | TEST-009, TEST-010 | Covered |
| Grow | TEST-011 | Covered |
| GC | TEST-012 | Covered |
| Count | TEST-013 | Covered |
| Iteration | TEST-014 | Covered |
| Concurrent | TEST-015 | Covered |
| Apply | TEST-016 | Covered |
| Reset | TEST-017 | Covered |
| Stat | TEST-018 | Covered |
| Direct mode | TEST-019 | Covered |
| Stress | TEST-020 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_hs regression | regressions/ck_hs/ | TEST-001 through TEST-020 |
