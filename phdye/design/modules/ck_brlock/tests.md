# Module: ck_brlock — Test Specification

## Conformance Tests

### TEST-001: init_unlocked

**Category:** basic

**Action:** Initialize lock

**Expected Result:** readers = NULL, writer = false

---

### TEST-002: register_unregister

**Category:** basic

**Action:** Register reader, then unregister

**Expected Result:** Reader successfully added and removed from list

---

### TEST-003: read_lock_unlock

**Category:** basic

**Setup:** Register reader

**Action:** Read lock then unlock

**Expected Result:** Lock acquired and released

---

### TEST-004: write_lock_unlock

**Category:** basic

**Action:** Write lock then unlock

**Expected Result:** Lock acquired and released

---

### TEST-005: recursive_read_lock

**Category:** basic

**Setup:** Register reader

**Action:** Read lock twice, unlock twice

**Expected Result:** Both locks acquired and released correctly

---

### TEST-006: multiple_readers

**Category:** concurrent

**Setup:** Register N readers

**Action:** All readers lock simultaneously

**Expected Result:** All readers hold lock concurrently

---

### TEST-007: writer_waits_for_readers

**Category:** concurrent

**Setup:** Reader holds lock

**Action:** Writer attempts lock

**Expected Result:** Writer blocks until reader releases

---

### TEST-008: cache_local_performance

**Category:** performance

**Setup:** Multiple reader threads, each registered

**Action:** High-frequency read lock/unlock

**Expected Result:** No cache line contention between readers

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| ck_brlock_init | TEST-001 | Covered |
| Read register/unregister | TEST-002 | Covered |
| Read lock/unlock | TEST-003 | Covered |
| Write lock/unlock | TEST-004 | Covered |
| Recursive reads | TEST-005 | Covered |
| Multiple readers | TEST-006 | Covered |
| Writer exclusion | TEST-007 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_brlock regression | regressions/ck_brlock/ | TEST-001 through TEST-008 |
