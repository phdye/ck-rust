# Module: ck_backoff — Test Specification

## Conformance Tests

### TEST-001: initialization

**Category:** basic

**Tests Requirement:** CK_BACKOFF_INITIALIZER provides valid initial value

**Setup:**
1. Declare ck_backoff_t variable

**Action:**
1. Initialize: ck_backoff_t backoff = CK_BACKOFF_INITIALIZER

**Expected Result:**
- backoff equals 512 (1 << 9)

**Cleanup:** None required.

---

### TEST-002: single_backoff

**Category:** basic

**Tests Requirement:** Single backoff call executes and doubles value

**Setup:**
1. Initialize: ck_backoff_t backoff = CK_BACKOFF_INITIALIZER

**Action:**
1. Call ck_backoff_eb(&backoff)

**Expected Result:**
- backoff equals 1024 (512 * 2)

**Cleanup:** None required.

---

### TEST-003: exponential_growth

**Category:** basic

**Tests Requirement:** Repeated backoffs grow exponentially

**Setup:**
1. Initialize: ck_backoff_t backoff = CK_BACKOFF_INITIALIZER

**Action:**
1. Call ck_backoff_eb(&backoff) five times
2. Record value after each call

**Expected Result:**
- After 1: 1024
- After 2: 2048
- After 3: 4096
- After 4: 8192
- After 5: 16384

**Cleanup:** None required.

---

### TEST-004: ceiling_reached

**Category:** edge_case

**Tests Requirement:** Backoff stops growing at ceiling

**Setup:**
1. Initialize: ck_backoff_t backoff = CK_BACKOFF_CEILING

**Action:**
1. Call ck_backoff_eb(&backoff)

**Expected Result:**
- backoff still equals CK_BACKOFF_CEILING (not doubled)

**Cleanup:** None required.

---

### TEST-005: ceiling_approach

**Category:** edge_case

**Tests Requirement:** Backoff reaches but does not exceed ceiling

**Setup:**
1. Initialize: ck_backoff_t backoff = CK_BACKOFF_INITIALIZER

**Action:**
1. Call ck_backoff_eb(&backoff) repeatedly until value stops changing
2. Record final value

**Expected Result:**
- Final value equals CK_BACKOFF_CEILING or is at ceiling after shift

**Cleanup:** None required.

---

### TEST-006: small_initial_value

**Category:** edge_case

**Tests Requirement:** Small initial values still work

**Setup:**
1. Initialize: ck_backoff_t backoff = 1

**Action:**
1. Call ck_backoff_eb(&backoff)

**Expected Result:**
- backoff equals 2

**Cleanup:** None required.

---

### TEST-007: zero_value_behavior

**Category:** edge_case

**Tests Requirement:** Zero value behavior (edge case)

**Setup:**
1. Initialize: ck_backoff_t backoff = 0

**Action:**
1. Call ck_backoff_eb(&backoff)

**Expected Result:**
- backoff equals 0 (no growth from zero - stuck state)
- Note: This is a degenerate case; users should not initialize to 0

**Cleanup:** None required.

---

### TEST-008: delay_occurs

**Category:** basic

**Tests Requirement:** Backoff actually introduces delay

**Setup:**
1. Initialize: ck_backoff_t backoff = CK_BACKOFF_CEILING

**Action:**
1. Record start time
2. Call ck_backoff_eb(&backoff)
3. Record end time

**Expected Result:**
- End time > start time (measurable delay occurred)

**Cleanup:** None required.

---

## Coverage Matrix

| Requirement (spec.md) | Test IDs | Status |
|-----------------------|----------|--------|
| CK_BACKOFF_INITIALIZER valid | TEST-001 | Covered |
| Value doubles after call | TEST-002 | Covered |
| Exponential growth | TEST-003 | Covered |
| Ceiling not exceeded | TEST-004, TEST-005 | Covered |
| Delay occurs | TEST-008 | Covered |
| Edge cases | TEST-006, TEST-007 | Covered |

## Test Gaps

| Requirement | Reason | Recommendation |
|-------------|--------|----------------|
| Delay proportional to value | Hard to measure precisely | Could add timing comparison tests |
| Concurrent usage | Module is not thread-safe by design | N/A |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| None found | — | — |

No dedicated regression tests found for ck_backoff. Module is implicitly tested through spinlock usage.
