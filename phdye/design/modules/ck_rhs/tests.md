# Module: ck_rhs — Test Specification

## Conformance Tests

### TEST-001: init_basic

**Category:** basic

**Action:** Initialize Robin Hood hash set

**Expected Result:** Hash set created

---

### TEST-002: put_get_single

**Category:** basic

**Setup:** Initialize

**Action:** Put element, get same

**Expected Result:** Get returns element

---

### TEST-003: put_duplicate

**Category:** basic

**Setup:** Initialize, put element

**Action:** Put same element again

**Expected Result:** Returns false

---

### TEST-004: put_unique

**Category:** basic

**Setup:** Initialize

**Action:** put_unique with new element

**Expected Result:** Returns true

---

### TEST-005: robin_hood_displacement

**Category:** basic

**Setup:** Initialize, create collision scenario

**Action:** Insert elements that should displace

**Expected Result:** All elements accessible, displacement occurred

---

### TEST-006: set_insert

**Category:** basic

**Setup:** Initialize

**Action:** set with new key

**Expected Result:** previous = NULL, returns true

---

### TEST-007: set_replace

**Category:** basic

**Setup:** Initialize, insert element

**Action:** set with same key

**Expected Result:** previous = old, element replaced

---

### TEST-008: fas_exists

**Category:** basic

**Setup:** Initialize, insert element

**Action:** fas with same key

**Expected Result:** Returns true, previous set

---

### TEST-009: fas_missing

**Category:** basic

**Setup:** Initialize

**Action:** fas with non-existent key

**Expected Result:** Returns false

---

### TEST-010: remove_existing

**Category:** basic

**Setup:** Initialize, insert elements

**Action:** Remove one

**Expected Result:** Returns removed, count decremented

---

### TEST-011: grow_capacity

**Category:** basic

**Setup:** Initialize with small capacity

**Action:** Insert many elements, trigger grow

**Expected Result:** All elements preserved

---

### TEST-012: rebuild_clears_tombstones

**Category:** basic

**Setup:** Insert and remove many

**Action:** rebuild

**Expected Result:** Tombstones cleared

---

### TEST-013: count_accuracy

**Category:** basic

**Setup:** Insert 100, remove 40

**Action:** count

**Expected Result:** Returns 60

---

### TEST-014: iteration

**Category:** basic

**Setup:** Insert known elements

**Action:** Iterate with next

**Expected Result:** All elements visited

---

### TEST-015: load_factor_change

**Category:** basic

**Setup:** Initialize

**Action:** set_load_factor to 50

**Expected Result:** Load factor updated

---

### TEST-016: concurrent_read

**Category:** concurrent

**Setup:** Initialize, populate

**Action:** Multiple readers calling get

**Expected Result:** All reads correct

---

### TEST-017: apply_function

**Category:** basic

**Setup:** Initialize, insert

**Action:** apply with transformation

**Expected Result:** Entry modified

---

### TEST-018: probe_bound_check

**Category:** stress

**Setup:** Insert many elements

**Action:** Check probe_maximum in stats

**Expected Result:** Reasonable bound (Robin Hood property)

---

### TEST-019: read_mostly_mode

**Category:** basic

**Setup:** Initialize with MODE_READ_MOSTLY

**Action:** Insert, lookup, remove

**Expected Result:** Operations work, lookup optimized

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| Init | TEST-001 | Covered |
| Put | TEST-002, TEST-003, TEST-005 | Covered |
| Put unique | TEST-004 | Covered |
| Set | TEST-006, TEST-007 | Covered |
| Fas | TEST-008, TEST-009 | Covered |
| Remove | TEST-010 | Covered |
| Grow | TEST-011 | Covered |
| Rebuild | TEST-012 | Covered |
| Count | TEST-013 | Covered |
| Iteration | TEST-014 | Covered |
| Load factor | TEST-015 | Covered |
| Concurrent | TEST-016 | Covered |
| Apply | TEST-017 | Covered |
| Probe bound | TEST-018 | Covered |
| Read-mostly | TEST-019 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_rhs regression | regressions/ck_rhs/ | TEST-001 through TEST-019 |
