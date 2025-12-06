# Module: ck_elide — Test Specification

## Conformance Tests

### TEST-001: stat_init

**Category:** basic

**Action:** Initialize ck_elide_stat

**Expected Result:** n_fallback = 0, n_elide = 0, skip = 0

---

### TEST-002: lock_unlock_basic

**Category:** basic

**Condition:** CK_F_PR_RTM defined

**Setup:** Create lock with elision wrappers

**Action:** ck_elide_N_lock, ck_elide_N_unlock

**Expected Result:** Lock acquired and released (may be elided or fallback)

---

### TEST-003: lock_unlock_no_rtm

**Category:** basic

**Condition:** CK_F_PR_RTM not defined

**Setup:** Create lock with elision wrappers

**Action:** ck_elide_N_lock, ck_elide_N_unlock

**Expected Result:** Fallback path called directly

---

### TEST-004: adaptive_single_thread

**Category:** basic

**Condition:** CK_F_PR_RTM defined

**Setup:** Initialize stat, default config

**Action:** Repeated adaptive lock/unlock without conflicts

**Expected Result:** n_elide increases, n_fallback low or zero

---

### TEST-005: conflict_triggers_fallback

**Category:** concurrent

**Condition:** CK_F_PR_RTM defined

**Setup:** Two threads accessing same data in elided section

**Action:** Both attempt elided lock, cause conflict

**Expected Result:** At least one falls back, n_fallback increases

---

### TEST-006: skip_after_aborts

**Category:** basic

**Condition:** CK_F_PR_RTM defined

**Setup:** Config with small skip values, stat with skip = 0

**Action:** Force abort, check next acquisition

**Expected Result:** skip decremented, fallback used while skip > 0

---

### TEST-007: busy_spin_retry

**Category:** concurrent

**Condition:** CK_F_PR_RTM defined

**Setup:** One thread holds lock, another attempts elided lock

**Action:** Second thread aborts with LOCK_BUSY

**Expected Result:** Spins, then either elides or falls back

---

### TEST-008: trylock_success

**Category:** basic

**Condition:** CK_F_PR_RTM defined

**Setup:** Lock available

**Action:** ck_elide_N_trylock

**Expected Result:** Returns true, in transaction

---

### TEST-009: trylock_failure

**Category:** basic

**Condition:** CK_F_PR_RTM defined

**Setup:** Force RTM begin failure

**Action:** ck_elide_N_trylock

**Expected Result:** Returns false

---

### TEST-010: capacity_abort

**Category:** stress

**Condition:** CK_F_PR_RTM defined

**Setup:** Large working set in elided section

**Action:** Attempt elided lock with large read/write set

**Expected Result:** Capacity abort, fallback used, skip set to USHRT_MAX

---

### TEST-011: stats_accuracy

**Category:** basic

**Setup:** Track manual count of elided vs fallback

**Action:** Multiple lock/unlock cycles

**Expected Result:** n_elide + n_fallback equals total acquisitions

---

### TEST-012: config_tuning

**Category:** stress

**Setup:** Various config values (skip=0, retry=1 through large values)

**Action:** Workload with controlled conflict rate

**Expected Result:** Higher retry values improve elision rate under light conflict

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| Stat init | TEST-001 | Covered |
| Basic lock/unlock | TEST-002, TEST-003 | Covered |
| Adaptive mode | TEST-004 | Covered |
| Conflict handling | TEST-005 | Covered |
| Skip mechanism | TEST-006 | Covered |
| Busy spin | TEST-007 | Covered |
| Trylock | TEST-008, TEST-009 | Covered |
| Capacity abort | TEST-010 | Covered |
| Stats accuracy | TEST-011 | Covered |
| Config tuning | TEST-012 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_elide regression | regressions/ck_elide/ | TEST-001 through TEST-012 |
