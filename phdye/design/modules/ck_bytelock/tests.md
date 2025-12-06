# Module: ck_bytelock — Test Specification

## Conformance Tests

### TEST-001: init_unlocked

**Category:** basic

**Action:** Initialize lock

**Expected Result:** owner = 0, n_readers = 0, all slots cleared

---

### TEST-002: write_lock_unlock

**Category:** basic

**Action:** Write lock with slot 1, then unlock

**Expected Result:** Lock acquired and released

---

### TEST-003: read_lock_unlock_slotted

**Category:** basic

**Action:** Read lock with slot 1, then unlock

**Expected Result:** Lock acquired and released

---

### TEST-004: read_lock_unlock_unslotted

**Category:** basic

**Action:** Read lock with CK_BYTELOCK_UNSLOTTED

**Expected Result:** Uses n_readers counter, lock acquired and released

---

### TEST-005: multiple_slotted_readers

**Category:** concurrent

**Setup:** Threads with slots 1, 2, 3, 4

**Action:** All acquire read lock simultaneously

**Expected Result:** All readers hold lock concurrently

---

### TEST-006: writer_waits_for_readers

**Category:** concurrent

**Setup:** Reader holds lock

**Action:** Writer attempts lock

**Expected Result:** Writer blocks until reader releases

---

### TEST-007: write_to_read_downgrade

**Category:** basic

**Setup:** Acquire write lock with slot 1

**Action:** Acquire read lock (same slot)

**Expected Result:** Downgrade succeeds atomically

---

### TEST-008: mixed_slotted_unslotted

**Category:** concurrent

**Setup:** Some threads slotted, some unslotted

**Action:** All acquire read locks

**Expected Result:** All coexist, writer waits for all

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| ck_bytelock_init | TEST-001 | Covered |
| Write lock/unlock | TEST-002 | Covered |
| Slotted read | TEST-003, TEST-005 | Covered |
| Unslotted read | TEST-004 | Covered |
| Writer exclusion | TEST-006 | Covered |
| Downgrade | TEST-007 | Covered |
| Mixed mode | TEST-008 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_bytelock regression | regressions/ck_bytelock/ | TEST-001 through TEST-008 |
