# Module: ck_rwcohort — Test Specification

## Conformance Tests

### TEST-001: wp_init

**Category:** basic

**Action:** Initialize writer-preference rwcohort

**Expected Result:** read_counter=0, write_barrier=0

---

### TEST-002: wp_write_lock_unlock

**Category:** basic

**Setup:** Initialize WP rwcohort and cohort

**Action:** write_lock, write_unlock

**Expected Result:** Lock acquired and released

---

### TEST-003: wp_read_lock_unlock

**Category:** basic

**Setup:** Initialize WP rwcohort and cohort

**Action:** read_lock, read_unlock

**Expected Result:** Lock acquired and released

---

### TEST-004: wp_multiple_readers

**Category:** concurrent

**Setup:** Initialize WP rwcohort

**Action:** Multiple threads acquire read lock

**Expected Result:** All readers hold lock concurrently

---

### TEST-005: wp_writer_blocks_readers

**Category:** concurrent

**Setup:** Initialize WP rwcohort

**Action:** Writer holds lock, readers attempt

**Expected Result:** Readers blocked until writer releases

---

### TEST-006: wp_writer_waits_readers

**Category:** concurrent

**Setup:** Initialize WP rwcohort, readers active

**Action:** Writer attempts lock

**Expected Result:** Writer waits for readers to drain

---

### TEST-007: wp_write_barrier

**Category:** concurrent

**Setup:** Initialize WP rwcohort, writer waiting

**Action:** Writer exceeds wait_limit

**Expected Result:** write_barrier raised, new readers blocked

---

### TEST-008: rp_init

**Category:** basic

**Action:** Initialize reader-preference rwcohort

**Expected Result:** read_counter=0, read_barrier=0

---

### TEST-009: rp_write_lock_unlock

**Category:** basic

**Setup:** Initialize RP rwcohort and cohort

**Action:** write_lock, write_unlock

**Expected Result:** Lock acquired and released

---

### TEST-010: rp_read_lock_unlock

**Category:** basic

**Setup:** Initialize RP rwcohort and cohort

**Action:** read_lock, read_unlock

**Expected Result:** Lock acquired and released

---

### TEST-011: rp_read_barrier

**Category:** concurrent

**Setup:** Initialize RP rwcohort, readers waiting

**Action:** Readers exceed wait_limit

**Expected Result:** read_barrier raised, new writers blocked

---

### TEST-012: neutral_init

**Category:** basic

**Action:** Initialize neutral rwcohort

**Expected Result:** read_counter=0

---

### TEST-013: neutral_write_lock_unlock

**Category:** basic

**Setup:** Initialize neutral rwcohort

**Action:** write_lock, write_unlock

**Expected Result:** Lock acquired and released

---

### TEST-014: neutral_read_lock_unlock

**Category:** basic

**Setup:** Initialize neutral rwcohort

**Action:** read_lock, read_unlock

**Expected Result:** Lock acquired and released

---

### TEST-015: neutral_fair_ordering

**Category:** concurrent

**Setup:** Initialize neutral rwcohort

**Action:** Interleaved readers and writers

**Expected Result:** Approximately FIFO ordering

---

### TEST-016: stress_wp

**Category:** stress

**Setup:** Initialize WP rwcohort, many threads

**Action:** Mixed read/write workload

**Expected Result:** No deadlock, all threads make progress

---

### TEST-017: stress_rp

**Category:** stress

**Setup:** Initialize RP rwcohort, many threads

**Action:** Mixed read/write workload

**Expected Result:** No deadlock, all threads make progress

---

### TEST-018: numa_locality

**Category:** stress

**Setup:** Simulated NUMA nodes

**Action:** Local threads contend

**Expected Result:** Local threads pass lock preferentially

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| WP init | TEST-001 | Covered |
| WP write | TEST-002, TEST-005, TEST-006 | Covered |
| WP read | TEST-003, TEST-004 | Covered |
| WP barrier | TEST-007 | Covered |
| RP init | TEST-008 | Covered |
| RP write | TEST-009 | Covered |
| RP read | TEST-010 | Covered |
| RP barrier | TEST-011 | Covered |
| Neutral init | TEST-012 | Covered |
| Neutral write | TEST-013 | Covered |
| Neutral read | TEST-014 | Covered |
| Neutral fairness | TEST-015 | Covered |
| Stress | TEST-016, TEST-017 | Covered |
| NUMA | TEST-018 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_rwcohort regression | regressions/ck_rwcohort/ | TEST-001 through TEST-018 |
