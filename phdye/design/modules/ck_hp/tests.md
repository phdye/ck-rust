# Module: ck_hp — Test Specification

## Conformance Tests

### TEST-001: init_basic

**Category:** basic

**Action:** Initialize ck_hp with degree=2, threshold=100

**Expected Result:** degree=2, threshold=100 set

---

### TEST-002: register_record

**Category:** basic

**Setup:** Initialize ck_hp

**Action:** Register record with pointers array

**Expected Result:** Record linked, all slots NULL

---

### TEST-003: set_hazard

**Category:** basic

**Setup:** Register record

**Action:** ck_hp_set(record, 0, ptr)

**Expected Result:** Slot 0 contains ptr

---

### TEST-004: set_fence_ordering

**Category:** concurrent

**Setup:** Register record, shared data structure

**Action:** set_fence, then read data

**Expected Result:** Data read sees writes before pointer publication

---

### TEST-005: clear_all

**Category:** basic

**Setup:** Register record, set multiple hazards

**Action:** ck_hp_clear

**Expected Result:** All slots NULL

---

### TEST-006: retire_object

**Category:** basic

**Setup:** Register record

**Action:** ck_hp_retire with hazard

**Expected Result:** n_pending incremented, destructor not called

---

### TEST-007: free_below_threshold

**Category:** basic

**Setup:** threshold=100, retire 50 objects

**Action:** Check state

**Expected Result:** n_pending=50, no reclaim triggered

---

### TEST-008: free_triggers_reclaim

**Category:** basic

**Setup:** threshold=10, retire 10 objects, no hazards set

**Action:** ck_hp_free

**Expected Result:** Destructor called for objects

---

### TEST-009: protected_not_freed

**Category:** concurrent

**Setup:** Thread A sets hazard to object, Thread B retires same object

**Action:** Thread B reclaims

**Expected Result:** Object NOT freed (hazard protects it)

---

### TEST-010: reclaim_scan

**Category:** concurrent

**Setup:** Multiple threads, various hazards

**Action:** One thread reclaims

**Expected Result:** Only unprotected objects freed

---

### TEST-011: purge_all

**Category:** basic

**Setup:** Retire objects, clear hazards

**Action:** ck_hp_purge

**Expected Result:** All objects freed

---

### TEST-012: recycle_record

**Category:** basic

**Setup:** Register, unregister record

**Action:** ck_hp_recycle

**Expected Result:** Returns recycled record

---

### TEST-013: threshold_update

**Category:** basic

**Setup:** Initialize with threshold=100

**Action:** ck_hp_set_threshold to 50

**Expected Result:** threshold=50

---

### TEST-014: multi_slot_usage

**Category:** basic

**Setup:** degree=4, register record

**Action:** Set slots 0-3 to different pointers

**Expected Result:** All slots correctly set

---

### TEST-015: stress_retire_reclaim

**Category:** stress

**Setup:** Multiple threads retiring and reclaiming

**Action:** Continuous operation

**Expected Result:** No double-free, no use-after-free

---

### TEST-016: destructor_context

**Category:** basic

**Setup:** Retire with specific data pointer

**Action:** Reclaim

**Expected Result:** Destructor receives correct data pointer

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| Init | TEST-001 | Covered |
| Register | TEST-002 | Covered |
| Set | TEST-003, TEST-014 | Covered |
| Set fence | TEST-004 | Covered |
| Clear | TEST-005 | Covered |
| Retire | TEST-006 | Covered |
| Free/threshold | TEST-007, TEST-008 | Covered |
| Protection | TEST-009, TEST-010 | Covered |
| Purge | TEST-011 | Covered |
| Recycle | TEST-012 | Covered |
| Threshold update | TEST-013 | Covered |
| Stress | TEST-015 | Covered |
| Destructor | TEST-016 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_hp regression | regressions/ck_hp/ | TEST-001 through TEST-016 |
