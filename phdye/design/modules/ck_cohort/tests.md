# Module: ck_cohort — Test Specification

## Conformance Tests

### TEST-001: init_correct

**Category:** basic

**Setup:** Create global and local spinlocks

**Action:** Initialize cohort with limit 5

**Expected Result:** release_state = GLOBAL, limit = 5

---

### TEST-002: lock_unlock_single

**Category:** basic

**Action:** Lock and unlock cohort once

**Expected Result:** Lock acquired and released

---

### TEST-003: local_passing

**Category:** concurrent

**Setup:** Two threads on same "node"

**Action:** Thread A locks, unlocks; Thread B waiting

**Expected Result:** B acquires without global re-acquisition

---

### TEST-004: limit_triggers_global_release

**Category:** basic

**Setup:** Limit = 3

**Action:** Lock/unlock 4 times with local waiters

**Expected Result:** Global released after 3rd unlock

---

### TEST-005: no_waiters_releases_global

**Category:** basic

**Setup:** Single thread

**Action:** Lock, unlock (no other waiters)

**Expected Result:** Global released immediately

---

### TEST-006: numa_stress

**Category:** stress

**Setup:** Multiple threads across simulated NUMA nodes

**Action:** Continuous lock/unlock

**Expected Result:** All threads make progress

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| Init | TEST-001 | Covered |
| Lock/unlock | TEST-002 | Covered |
| Local passing | TEST-003 | Covered |
| Pass limit | TEST-004 | Covered |
| No waiters | TEST-005 | Covered |
| NUMA performance | TEST-006 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_cohort regression | regressions/ck_cohort/ | TEST-001 through TEST-006 |
