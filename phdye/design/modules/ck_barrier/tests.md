# Module: ck_barrier — Test Specification

## Conformance Tests

### TEST-001: centralized_basic

**Category:** basic

**Setup:** Initialize centralized barrier, 4 threads

**Action:** All threads call barrier

**Expected Result:** All threads proceed after synchronization

---

### TEST-002: centralized_reuse

**Category:** basic

**Setup:** Initialize centralized barrier

**Action:** Multiple barrier rounds

**Expected Result:** Sense reversal enables reuse

---

### TEST-003: combining_init

**Category:** basic

**Action:** Initialize combining barrier with root

**Expected Result:** Tree structure initialized

---

### TEST-004: combining_group_add

**Category:** basic

**Setup:** Initialize combining barrier

**Action:** Add multiple groups

**Expected Result:** Tree built correctly

---

### TEST-005: combining_sync

**Category:** concurrent

**Setup:** Initialize combining tree, multiple threads

**Action:** All threads call barrier

**Expected Result:** All threads synchronized

---

### TEST-006: dissemination_size

**Category:** basic

**Action:** Call dissemination_size for various n

**Expected Result:** Returns ceil(log2(n)) * n

---

### TEST-007: dissemination_subscribe

**Category:** basic

**Setup:** Initialize dissemination barrier

**Action:** Multiple threads subscribe

**Expected Result:** Each gets unique ID

---

### TEST-008: dissemination_sync

**Category:** concurrent

**Setup:** Initialize dissemination, 8 threads

**Action:** All threads call barrier

**Expected Result:** All threads synchronized

---

### TEST-009: tournament_init

**Category:** basic

**Action:** Initialize tournament barrier

**Expected Result:** Rounds allocated correctly

---

### TEST-010: tournament_sync

**Category:** concurrent

**Setup:** Tournament barrier, multiple threads

**Action:** All threads call barrier

**Expected Result:** All threads synchronized

---

### TEST-011: mcs_init

**Category:** basic

**Action:** Initialize MCS barrier for n threads

**Expected Result:** Tree structure initialized

---

### TEST-012: mcs_sync

**Category:** concurrent

**Setup:** MCS barrier, multiple threads

**Action:** All threads call barrier

**Expected Result:** All threads synchronized

---

### TEST-013: stress_centralized

**Category:** stress

**Setup:** Many threads, centralized barrier

**Action:** 1000 barrier rounds

**Expected Result:** All rounds complete correctly

---

### TEST-014: stress_dissemination

**Category:** stress

**Setup:** Many threads, dissemination barrier

**Action:** 1000 barrier rounds

**Expected Result:** All rounds complete correctly

---

### TEST-015: stress_mcs

**Category:** stress

**Setup:** Many threads, MCS barrier

**Action:** 1000 barrier rounds

**Expected Result:** All rounds complete correctly

---

### TEST-016: scalability_comparison

**Category:** performance

**Setup:** Various thread counts

**Action:** Measure latency of each barrier type

**Expected Result:** Performance characteristics documented

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| Centralized init | TEST-001 | Covered |
| Centralized reuse | TEST-002 | Covered |
| Combining init | TEST-003, TEST-004 | Covered |
| Combining sync | TEST-005 | Covered |
| Dissemination size | TEST-006 | Covered |
| Dissemination subscribe | TEST-007 | Covered |
| Dissemination sync | TEST-008 | Covered |
| Tournament init | TEST-009 | Covered |
| Tournament sync | TEST-010 | Covered |
| MCS init | TEST-011 | Covered |
| MCS sync | TEST-012 | Covered |
| Stress | TEST-013, TEST-014, TEST-015 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_barrier regression | regressions/ck_barrier/ | TEST-001 through TEST-016 |
