# Module: ck_ec — Test Specification

## Conformance Tests

### TEST-001: init_value_32

**Category:** basic

**Action:** Initialize ck_ec32 with value 42

**Expected Result:** ck_ec32_value returns 42, has_waiters returns false

---

### TEST-002: init_value_64

**Category:** basic

**Condition:** CK_F_EC64 defined

**Action:** Initialize ck_ec64 with value 1000000

**Expected Result:** ck_ec64_value returns 1000000, has_waiters returns false

---

### TEST-003: inc_single_32

**Category:** basic

**Setup:** Initialize ck_ec32 to 0

**Action:** Call ck_ec32_inc 5 times

**Expected Result:** ck_ec32_value returns 5

---

### TEST-004: inc_single_64

**Category:** basic

**Condition:** CK_F_EC64 defined

**Setup:** Initialize ck_ec64 to 0

**Action:** Call ck_ec64_inc 5 times

**Expected Result:** ck_ec64_value returns 5

---

### TEST-005: add_returns_old_value

**Category:** basic

**Setup:** Initialize ck_ec32 to 10

**Action:** Call ck_ec32_add with delta=5

**Expected Result:** Returns 10, ck_ec32_value returns 15

---

### TEST-006: wait_returns_immediately_on_change

**Category:** basic

**Setup:** Initialize ck_ec32 to 5

**Action:** Call ck_ec32_wait with old_value=0

**Expected Result:** Returns 0 immediately (value differs)

---

### TEST-007: wait_timeout_no_change

**Category:** basic

**Setup:** Initialize ck_ec32 to 5, create deadline in past

**Action:** Call ck_ec32_wait with old_value=5, expired deadline

**Expected Result:** Returns -1 (timeout)

---

### TEST-008: wait_nonblocking

**Category:** basic

**Setup:** Initialize ck_ec32 to 5, deadline.tv_sec = 0

**Action:** Call ck_ec32_wait with old_value=5, zero deadline

**Expected Result:** Returns -1 immediately (non-blocking check)

---

### TEST-009: producer_consumer_single

**Category:** concurrent

**Setup:** Initialize ck_ec32 to 0, one producer thread, one consumer thread

**Action:** Consumer waits for value != 0, producer increments after delay

**Expected Result:** Consumer wakes, sees value >= 1

---

### TEST-010: producer_consumer_multi

**Category:** concurrent

**Setup:** Initialize ck_ec32, multiple producer threads, multiple consumers

**Action:** Producers increment concurrently, consumers wait and check

**Expected Result:** All consumers eventually wake, counter reflects all increments

---

### TEST-011: single_producer_mode_32

**Category:** basic

**Condition:** CK_F_EC_SP defined

**Setup:** Initialize ck_ec32, mode.single_producer = true

**Action:** Increment 1000 times

**Expected Result:** Value equals 1000

---

### TEST-012: multi_producer_mode_32

**Category:** concurrent

**Setup:** Initialize ck_ec32, mode.single_producer = false, 4 producer threads

**Action:** Each producer increments 1000 times

**Expected Result:** Final value equals 4000

---

### TEST-013: wait_pred_early_return

**Category:** basic

**Setup:** Initialize ck_ec32 to 5, predicate that returns 42 immediately

**Action:** Call ck_ec32_wait_pred with old_value=5, custom predicate

**Expected Result:** Returns 42 (predicate's return value)

---

### TEST-014: wait_pred_deadline_modify

**Category:** basic

**Setup:** Initialize ck_ec32, predicate that shortens deadline

**Action:** Call ck_ec32_wait_pred with long deadline, deadline-shortening predicate

**Expected Result:** Returns -1 faster than original deadline

---

### TEST-015: has_waiters_flag_set

**Category:** concurrent

**Setup:** Initialize ck_ec32, one waiting thread

**Action:** Thread enters wait slow path (sets flag)

**Expected Result:** ck_ec32_has_waiters returns true while thread waiting

---

### TEST-016: large_add_delta

**Category:** basic

**Setup:** Initialize ck_ec32 to 0

**Action:** Call ck_ec32_add with delta=1000000

**Expected Result:** Value equals 1000000

---

### TEST-017: deadline_computation

**Category:** basic

**Setup:** Get current time, create timeout of 100ms

**Action:** Call ck_ec_deadline

**Expected Result:** new_deadline approximately 100ms in future

---

### TEST-018: wait_64_with_32_futex

**Category:** platform

**Condition:** CK_F_EC64 defined, 32-bit futex only

**Setup:** Initialize ck_ec64, waiter and producer

**Action:** Waiter blocks, producer increments

**Expected Result:** Waiter wakes correctly (flag in low 32 bits)

---

### TEST-019: wraparound_32

**Category:** stress

**Setup:** Initialize ck_ec32 to INT32_MAX - 10

**Action:** Increment 20 times

**Expected Result:** Value wraps correctly, no corruption

---

### TEST-020: stress_inc_mp

**Category:** stress

**Setup:** Initialize ck_ec32, 8 producer threads, multi-producer mode

**Action:** Each thread increments 100000 times

**Expected Result:** Final value equals 800000

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| Init 32-bit | TEST-001 | Covered |
| Init 64-bit | TEST-002 | Covered |
| Inc 32-bit | TEST-003 | Covered |
| Inc 64-bit | TEST-004 | Covered |
| Add returns old | TEST-005 | Covered |
| Wait immediate | TEST-006 | Covered |
| Wait timeout | TEST-007 | Covered |
| Wait non-blocking | TEST-008 | Covered |
| Producer-consumer | TEST-009, TEST-010 | Covered |
| Single producer | TEST-011 | Covered |
| Multi producer | TEST-012, TEST-020 | Covered |
| Wait with predicate | TEST-013, TEST-014 | Covered |
| Has waiters | TEST-015 | Covered |
| Large delta | TEST-016 | Covered |
| Deadline computation | TEST-017 | Covered |
| 64-bit with 32-bit futex | TEST-018 | Covered |
| Wraparound | TEST-019 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_ec regression | regressions/ck_ec/ | TEST-001 through TEST-020 |
