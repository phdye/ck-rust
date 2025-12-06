# Module: ck_sequence — Test Specification

## Conformance Tests

### TEST-001: init_sets_zero

**Category:** basic

**Tests Requirement:** ck_sequence_init sets sequence to 0

**Setup:**
1. Allocate ck_sequence structure

**Action:**
1. ck_sequence_init(&seq)

**Expected Result:**
- seq.sequence == 0

**Cleanup:** None required.

---

### TEST-002: read_returns_even

**Category:** basic

**Tests Requirement:** read_begin returns even value

**Setup:**
1. Initialize sequence

**Action:**
1. version = ck_sequence_read_begin(&seq)

**Expected Result:**
- version & 1 == 0 (even)

**Cleanup:** None required.

---

### TEST-003: retry_false_no_write

**Category:** basic

**Tests Requirement:** read_retry returns false when no write occurred

**Setup:**
1. Initialize sequence

**Action:**
1. version = ck_sequence_read_begin(&seq)
2. // no write
3. result = ck_sequence_read_retry(&seq, version)

**Expected Result:**
- result == false

**Cleanup:** None required.

---

### TEST-004: retry_true_after_write

**Category:** basic

**Tests Requirement:** read_retry returns true after write

**Setup:**
1. Initialize sequence

**Action:**
1. version = ck_sequence_read_begin(&seq)
2. ck_sequence_write_begin(&seq)
3. ck_sequence_write_end(&seq)
4. result = ck_sequence_read_retry(&seq, version)

**Expected Result:**
- result == true
- seq.sequence == version + 2

**Cleanup:** None required.

---

### TEST-005: write_makes_odd_then_even

**Category:** basic

**Tests Requirement:** Write cycle increments sequence by 2

**Setup:**
1. Initialize sequence
2. Note initial value (0)

**Action:**
1. ck_sequence_write_begin(&seq)
2. mid = seq.sequence
3. ck_sequence_write_end(&seq)

**Expected Result:**
- mid is odd (1)
- Final sequence is even (2)

**Cleanup:** None required.

---

### TEST-006: macro_read_pattern

**Category:** basic

**Tests Requirement:** CK_SEQUENCE_READ macro works correctly

**Setup:**
1. Initialize sequence
2. Set test value = 42

**Action:**
1. Use CK_SEQUENCE_READ to read value
2. Perform write changing value to 100
3. Use CK_SEQUENCE_READ again

**Expected Result:**
- First read sees 42
- Second read sees 100
- Both reads complete without hanging

**Cleanup:** None required.

---

### TEST-007: concurrent_read_during_write

**Category:** concurrent

**Tests Requirement:** Readers detect concurrent writes

**Setup:**
1. Initialize sequence
2. Create reader thread
3. Create writer thread

**Action:**
1. Writer: repeatedly write (with mutex)
2. Reader: repeatedly read with retry pattern
3. Track: retry rate

**Expected Result:**
- Reader sometimes retries (when overlapping with write)
- All successful reads are consistent
- No crashes or hangs

**Cleanup:**
1. Join threads

---

### TEST-008: multiple_readers

**Category:** concurrent

**Tests Requirement:** Multiple concurrent readers work

**Setup:**
1. Initialize sequence
2. Initialize protected data
3. Create N reader threads (N = 8)

**Action:**
1. Each reader: continuous reads with retry
2. Occasional writes (with mutex)
3. Verify all reads are consistent

**Expected Result:**
- All reads get consistent data
- Retries occur around writes

**Cleanup:**
1. Join threads

---

## Coverage Matrix

| Requirement (spec.md) | Test IDs | Status |
|-----------------------|----------|--------|
| ck_sequence_init | TEST-001 | Covered |
| ck_sequence_read_begin | TEST-002, TEST-003 | Covered |
| ck_sequence_read_retry | TEST-003, TEST-004 | Covered |
| ck_sequence_write_begin | TEST-005 | Covered |
| ck_sequence_write_end | TEST-005 | Covered |
| CK_SEQUENCE_READ | TEST-006 | Covered |
| Concurrent correctness | TEST-007, TEST-008 | Covered |

## Test Gaps

| Requirement | Reason | Recommendation |
|-------------|--------|----------------|
| Overflow behavior | Rare in practice | Add test with high initial value |
| Reader starvation | Stress test | Add test with continuous writes |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_sequence regression | regressions/ck_sequence/ | TEST-001 through TEST-008 |
