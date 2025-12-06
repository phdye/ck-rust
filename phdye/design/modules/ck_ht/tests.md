# Module: ck_ht — Test Specification

## Conformance Tests

### TEST-001: init_basic

**Category:** basic

**Action:** Initialize hash table

**Expected Result:** Table created successfully

---

### TEST-002: put_get_direct

**Category:** basic

**Setup:** Initialize with MODE_DIRECT

**Action:** Put direct key-value, get same key

**Expected Result:** Get returns correct value

---

### TEST-003: put_get_bytestring

**Category:** basic

**Setup:** Initialize with MODE_BYTESTRING

**Action:** Put string key-value, get same key

**Expected Result:** Get returns correct value

---

### TEST-004: put_duplicate

**Category:** basic

**Setup:** Initialize, put entry

**Action:** Put entry with same key

**Expected Result:** Returns false

---

### TEST-005: set_insert

**Category:** basic

**Setup:** Initialize

**Action:** Set with new key

**Expected Result:** Returns true, entry inserted

---

### TEST-006: set_replace

**Category:** basic

**Setup:** Initialize, put entry

**Action:** Set same key with different value

**Expected Result:** Returns true, value updated

---

### TEST-007: remove_existing

**Category:** basic

**Setup:** Initialize, put entry

**Action:** Remove entry

**Expected Result:** Returns true, entry populated

---

### TEST-008: remove_missing

**Category:** basic

**Setup:** Initialize

**Action:** Remove non-existent key

**Expected Result:** Returns false

---

### TEST-009: grow_preserves

**Category:** basic

**Setup:** Initialize, insert 100 entries

**Action:** grow_spmc to larger capacity

**Expected Result:** All entries still accessible

---

### TEST-010: reset_clears

**Category:** basic

**Setup:** Initialize, insert entries

**Action:** reset_spmc

**Expected Result:** count = 0

---

### TEST-011: gc_tombstones

**Category:** basic

**Setup:** Insert and remove many entries

**Action:** gc

**Expected Result:** Tombstones cleaned

---

### TEST-012: iteration

**Category:** basic

**Setup:** Insert known entries

**Action:** Iterate with next

**Expected Result:** All entries visited

---

### TEST-013: concurrent_read

**Category:** concurrent

**Setup:** Initialize, populate

**Action:** Multiple readers calling get_spmc

**Expected Result:** All reads correct

---

### TEST-014: count_accuracy

**Category:** basic

**Setup:** Insert 50, remove 20

**Action:** count

**Expected Result:** Returns 30

---

### TEST-015: entry_empty_check

**Category:** basic

**Setup:** Initialize

**Action:** Check ck_ht_entry_empty on empty slot

**Expected Result:** Returns true

---

### TEST-016: entry_key_extraction

**Category:** basic

**Setup:** Set entry with known key/value

**Action:** Extract with entry_key, entry_value

**Expected Result:** Correct extraction

---

### TEST-017: hash_consistency

**Category:** basic

**Setup:** Compute hash for same key twice

**Action:** Compare hash values

**Expected Result:** Identical hashes

---

### TEST-018: stress_insert_lookup

**Category:** stress

**Setup:** Multiple threads

**Action:** Writer inserts, readers lookup

**Expected Result:** No crashes, correct values

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| Init | TEST-001 | Covered |
| Direct mode | TEST-002 | Covered |
| Bytestring mode | TEST-003 | Covered |
| Put | TEST-004 | Covered |
| Set | TEST-005, TEST-006 | Covered |
| Remove | TEST-007, TEST-008 | Covered |
| Grow | TEST-009 | Covered |
| Reset | TEST-010 | Covered |
| GC | TEST-011 | Covered |
| Iteration | TEST-012 | Covered |
| Concurrent | TEST-013, TEST-018 | Covered |
| Count | TEST-014 | Covered |
| Entry helpers | TEST-015, TEST-016 | Covered |
| Hash | TEST-017 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_ht regression | regressions/ck_ht/ | TEST-001 through TEST-018 |
