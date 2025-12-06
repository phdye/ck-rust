# Module: ck_pr — Test Specification

## Conformance Tests

### TEST-001: load_store_single_thread

**Category:** basic

**Tests Requirement:** Atomic load and store work correctly in single-threaded context

**Setup:**
1. Allocate aligned 64-bit variable, initialize to 0

**Action:**
1. Store value 0x123456789ABCDEF0 using ck_pr_store_64
2. Load value using ck_pr_load_64

**Expected Result:**
- Loaded value equals 0x123456789ABCDEF0

**Cleanup:** None required.

---

### TEST-002: cas_success

**Category:** basic

**Tests Requirement:** CAS succeeds when current value matches compare value

**Setup:**
1. Allocate aligned 64-bit variable, initialize to 42

**Action:**
1. Call ck_pr_cas_64(&var, 42, 100)

**Expected Result:**
- Returns true
- var now equals 100

**Cleanup:** None required.

---

### TEST-003: cas_failure

**Category:** basic

**Tests Requirement:** CAS fails when current value does not match compare value

**Setup:**
1. Allocate aligned 64-bit variable, initialize to 42

**Action:**
1. Call ck_pr_cas_64(&var, 99, 100)

**Expected Result:**
- Returns false
- var still equals 42

**Cleanup:** None required.

---

### TEST-004: cas_value_returns_old

**Category:** basic

**Tests Requirement:** CAS value variant returns previous value

**Setup:**
1. Allocate aligned 64-bit variable, initialize to 42
2. Allocate 64-bit variable for old value

**Action:**
1. Call ck_pr_cas_64_value(&var, 99, 100, &old)

**Expected Result:**
- Returns false
- old equals 42
- var equals 42

**Cleanup:** None required.

---

### TEST-005: faa_basic

**Category:** basic

**Tests Requirement:** Fetch-and-add returns previous value and updates

**Setup:**
1. Allocate aligned 64-bit variable, initialize to 100

**Action:**
1. result = ck_pr_faa_64(&var, 50)

**Expected Result:**
- result equals 100
- var equals 150

**Cleanup:** None required.

---

### TEST-006: faa_negative

**Category:** basic

**Tests Requirement:** Fetch-and-add works with negative delta (subtraction)

**Setup:**
1. Allocate aligned 64-bit variable, initialize to 100

**Action:**
1. result = ck_pr_faa_64(&var, (uint64_t)-30)

**Expected Result:**
- result equals 100
- var equals 70

**Cleanup:** None required.

---

### TEST-007: fas_basic

**Category:** basic

**Tests Requirement:** Fetch-and-store returns previous value and updates

**Setup:**
1. Allocate aligned 64-bit variable, initialize to 100

**Action:**
1. result = ck_pr_fas_64(&var, 999)

**Expected Result:**
- result equals 100
- var equals 999

**Cleanup:** None required.

---

### TEST-008: inc_dec_basic

**Category:** basic

**Tests Requirement:** Increment and decrement work correctly

**Setup:**
1. Allocate aligned 32-bit variable, initialize to 100

**Action:**
1. ck_pr_inc_32(&var)
2. ck_pr_inc_32(&var)
3. ck_pr_dec_32(&var)

**Expected Result:**
- var equals 101

**Cleanup:** None required.

---

### TEST-009: binary_ops

**Category:** basic

**Tests Requirement:** Binary operations (and, or, xor) work correctly

**Setup:**
1. Allocate aligned 32-bit variable, initialize to 0xFF00FF00

**Action:**
1. ck_pr_and_32(&var, 0xF0F0F0F0)
2. ck_pr_or_32(&var, 0x0000000F)
3. ck_pr_xor_32(&var, 0x000000F0)

**Expected Result:**
- After and: 0xF000F000
- After or: 0xF000F00F
- After xor: 0xF000F0FF

**Cleanup:** None required.

---

### TEST-010: bts_btr_btc

**Category:** basic

**Tests Requirement:** Bit test operations work correctly

**Setup:**
1. Allocate aligned 32-bit variable, initialize to 0

**Action:**
1. old1 = ck_pr_bts_32(&var, 5)  // set bit 5
2. old2 = ck_pr_bts_32(&var, 5)  // test already set
3. old3 = ck_pr_btr_32(&var, 5)  // reset bit 5
4. old4 = ck_pr_btc_32(&var, 3)  // complement bit 3

**Expected Result:**
- old1 equals false (bit was 0)
- old2 equals true (bit was 1)
- old3 equals true (bit was 1)
- old4 equals false (bit was 0)
- var equals 8 (bit 3 set)

**Cleanup:** None required.

---

### TEST-011: overflow_behavior

**Category:** edge_case

**Tests Requirement:** Arithmetic overflow wraps correctly

**Setup:**
1. Allocate aligned 32-bit variable, initialize to UINT32_MAX

**Action:**
1. ck_pr_inc_32(&var)

**Expected Result:**
- var equals 0 (wrapped)

**Cleanup:** None required.

---

### TEST-012: dec_is_zero

**Category:** basic

**Tests Requirement:** Decrement-is-zero detects zero correctly

**Setup:**
1. Allocate aligned 32-bit variable, initialize to 2

**Action:**
1. result1 = ck_pr_dec_32_is_zero(&var)  // 2 -> 1
2. result2 = ck_pr_dec_32_is_zero(&var)  // 1 -> 0

**Expected Result:**
- result1 equals false
- result2 equals true
- var equals 0

**Cleanup:** None required.

---

### TEST-013: concurrent_increment

**Category:** concurrent

**Tests Requirement:** Concurrent increments are correct

**Setup:**
1. Allocate aligned 64-bit variable, initialize to 0
2. Create N threads (N = 4)
3. Each thread will increment M times (M = 100000)

**Action:**
1. Start all threads
2. Each thread: for i in 0..M: ck_pr_inc_64(&var)
3. Wait for all threads to complete

**Expected Result:**
- var equals N * M (400000)

**Cleanup:**
1. Join all threads

---

### TEST-014: concurrent_cas_contention

**Category:** concurrent

**Tests Requirement:** CAS operations are correct under contention

**Setup:**
1. Allocate aligned 64-bit variable, initialize to 0
2. Create N threads (N = 4)
3. Each thread will attempt M increments via CAS (M = 100000)

**Action:**
1. Start all threads
2. Each thread:
   ```
   for i in 0..M:
     do:
       old = ck_pr_load_64(&var)
     while !ck_pr_cas_64(&var, old, old+1)
   ```
3. Wait for all threads to complete

**Expected Result:**
- var equals N * M (400000)
- No lost updates

**Cleanup:**
1. Join all threads

---

### TEST-015: fence_ordering

**Category:** concurrent

**Tests Requirement:** Fences enforce ordering (message passing pattern)

**Setup:**
1. Allocate aligned data and flag variables, initialize to 0
2. Create reader and writer threads

**Action:**
1. Writer thread:
   - ck_pr_store_64(&data, 42)
   - ck_pr_fence_release()
   - ck_pr_store_64(&flag, 1)
2. Reader thread:
   - spin while ck_pr_load_64(&flag) == 0
   - ck_pr_fence_acquire()
   - read = ck_pr_load_64(&data)

**Expected Result:**
- Reader observes read == 42

**Cleanup:**
1. Join all threads

---

### TEST-016: pointer_operations

**Category:** basic

**Tests Requirement:** Pointer type operations work correctly

**Setup:**
1. Allocate pointer variable, initialize to NULL
2. Allocate test objects obj1, obj2

**Action:**
1. ck_pr_store_ptr(&ptr, obj1)
2. old = ck_pr_fas_ptr(&ptr, obj2)

**Expected Result:**
- old equals obj1
- ptr equals obj2

**Cleanup:** None required.

---

### TEST-017: all_sizes

**Category:** basic

**Tests Requirement:** All size variants work correctly

**Setup:**
1. Allocate aligned variables: uint8_t, uint16_t, uint32_t, uint64_t

**Action:**
1. For each size: store, load, verify

**Expected Result:**
- All loads return stored values

**Cleanup:** None required.

---

## Coverage Matrix

| Requirement (spec.md) | Test IDs | Status |
|-----------------------|----------|--------|
| ck_pr_load_{type} | TEST-001, TEST-017 | Covered |
| ck_pr_store_{type} | TEST-001, TEST-017 | Covered |
| ck_pr_cas_{type} success | TEST-002 | Covered |
| ck_pr_cas_{type} failure | TEST-003 | Covered |
| ck_pr_cas_{type}_value | TEST-004 | Covered |
| ck_pr_faa_{type} | TEST-005, TEST-006 | Covered |
| ck_pr_fas_{type} | TEST-007 | Covered |
| ck_pr_inc/dec | TEST-008, TEST-012 | Covered |
| ck_pr_and/or/xor | TEST-009 | Covered |
| ck_pr_bts/btr/btc | TEST-010 | Covered |
| Overflow wrap | TEST-011 | Covered |
| Concurrent correctness | TEST-013, TEST-014 | Covered |
| Fence ordering | TEST-015 | Covered |
| Pointer operations | TEST-016 | Covered |

## Test Gaps

| Requirement | Reason | Recommendation |
|-------------|--------|----------------|
| ck_pr_neg, ck_pr_not | Less commonly used | Add tests for negate and bitwise not |
| All fence variants | Difficult to test precisely | Add tests for each fence type |
| Platform-specific behavior | Requires multiple platforms | Test on each supported architecture |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_pr regression | regressions/ck_pr/ | TEST-001 through TEST-017 |
