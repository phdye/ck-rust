# Module: ck_cc — Test Specification

## Conformance Tests

### TEST-001: ck_cc_ffs_zero

**Category:** edge_case

**Tests Requirement:** ck_cc_ffs returns 0 for zero input

**Setup:**
1. None required

**Action:**
1. Call ck_cc_ffs(0)

**Expected Result:**
- Returns 0

**Cleanup:** None required.

---

### TEST-002: ck_cc_ffs_bit_zero

**Category:** basic

**Tests Requirement:** ck_cc_ffs returns 1 for least significant bit set

**Setup:**
1. None required

**Action:**
1. Call ck_cc_ffs(1)

**Expected Result:**
- Returns 1

**Cleanup:** None required.

---

### TEST-003: ck_cc_ffs_bit_one

**Category:** basic

**Tests Requirement:** ck_cc_ffs returns correct position for bit 1

**Setup:**
1. None required

**Action:**
1. Call ck_cc_ffs(2)

**Expected Result:**
- Returns 2

**Cleanup:** None required.

---

### TEST-004: ck_cc_ffs_high_bit

**Category:** edge_case

**Tests Requirement:** ck_cc_ffs handles high bit correctly

**Setup:**
1. None required

**Action:**
1. Call ck_cc_ffs(0x80000000)

**Expected Result:**
- Returns 32

**Cleanup:** None required.

---

### TEST-005: ck_cc_ffs_multiple_bits

**Category:** basic

**Tests Requirement:** ck_cc_ffs returns position of least significant set bit

**Setup:**
1. None required

**Action:**
1. Call ck_cc_ffs(0b11110000)  // binary: 11110000 = 240

**Expected Result:**
- Returns 5 (bit 4 is first set bit, 1-indexed = 5)

**Cleanup:** None required.

---

### TEST-006: ck_cc_ffsl_zero

**Category:** edge_case

**Tests Requirement:** ck_cc_ffsl returns 0 for zero input

**Setup:**
1. None required

**Action:**
1. Call ck_cc_ffsl(0)

**Expected Result:**
- Returns 0

**Cleanup:** None required.

---

### TEST-007: ck_cc_ffsl_high_bit_64

**Category:** edge_case

**Tests Requirement:** ck_cc_ffsl handles 64-bit high bit correctly (on 64-bit systems)

**Setup:**
1. Skip if sizeof(long) < 8

**Action:**
1. Call ck_cc_ffsl(0x8000000000000000UL)

**Expected Result:**
- Returns 64

**Cleanup:** None required.

---

### TEST-008: ck_cc_ffsll_zero

**Category:** edge_case

**Tests Requirement:** ck_cc_ffsll returns 0 for zero input

**Setup:**
1. None required

**Action:**
1. Call ck_cc_ffsll(0)

**Expected Result:**
- Returns 0

**Cleanup:** None required.

---

### TEST-009: ck_cc_ffsll_high_bit

**Category:** edge_case

**Tests Requirement:** ck_cc_ffsll handles 64-bit high bit correctly

**Setup:**
1. None required

**Action:**
1. Call ck_cc_ffsll(0x8000000000000000ULL)

**Expected Result:**
- Returns 64

**Cleanup:** None required.

---

### TEST-010: ck_cc_ctz_zero

**Category:** edge_case

**Tests Requirement:** ck_cc_ctz returns 0 for zero input (CK-specific behavior)

**Setup:**
1. None required

**Action:**
1. Call ck_cc_ctz(0)

**Expected Result:**
- Returns 0

**Cleanup:** None required.

---

### TEST-011: ck_cc_ctz_bit_zero

**Category:** basic

**Tests Requirement:** ck_cc_ctz returns 0 when bit 0 is set

**Setup:**
1. None required

**Action:**
1. Call ck_cc_ctz(1)

**Expected Result:**
- Returns 0

**Cleanup:** None required.

---

### TEST-012: ck_cc_ctz_high_bit

**Category:** edge_case

**Tests Requirement:** ck_cc_ctz returns 31 for only high bit set

**Setup:**
1. None required

**Action:**
1. Call ck_cc_ctz(0x80000000)

**Expected Result:**
- Returns 31

**Cleanup:** None required.

---

### TEST-013: ck_cc_ctz_trailing_zeros

**Category:** basic

**Tests Requirement:** ck_cc_ctz counts trailing zeros correctly

**Setup:**
1. None required

**Action:**
1. Call ck_cc_ctz(0b11110000)  // 4 trailing zeros

**Expected Result:**
- Returns 4

**Cleanup:** None required.

---

### TEST-014: ck_cc_popcount_zero

**Category:** edge_case

**Tests Requirement:** ck_cc_popcount returns 0 for zero input

**Setup:**
1. None required

**Action:**
1. Call ck_cc_popcount(0)

**Expected Result:**
- Returns 0

**Cleanup:** None required.

---

### TEST-015: ck_cc_popcount_one

**Category:** basic

**Tests Requirement:** ck_cc_popcount returns 1 for single bit set

**Setup:**
1. None required

**Action:**
1. Call ck_cc_popcount(1)

**Expected Result:**
- Returns 1

**Cleanup:** None required.

---

### TEST-016: ck_cc_popcount_all_bits

**Category:** edge_case

**Tests Requirement:** ck_cc_popcount returns 32 for all bits set

**Setup:**
1. None required

**Action:**
1. Call ck_cc_popcount(0xFFFFFFFF)

**Expected Result:**
- Returns 32

**Cleanup:** None required.

---

### TEST-017: ck_cc_popcount_pattern

**Category:** basic

**Tests Requirement:** ck_cc_popcount counts arbitrary bit patterns

**Setup:**
1. None required

**Action:**
1. Call ck_cc_popcount(0b10101010)  // 4 bits set

**Expected Result:**
- Returns 4

**Cleanup:** None required.

---

### TEST-018: ck_cc_likely_identity

**Category:** basic

**Tests Requirement:** CK_CC_LIKELY preserves boolean value

**Setup:**
1. None required

**Action:**
1. Evaluate CK_CC_LIKELY(1)
2. Evaluate CK_CC_LIKELY(0)

**Expected Result:**
- CK_CC_LIKELY(1) evaluates to non-zero (true)
- CK_CC_LIKELY(0) evaluates to zero (false)

**Cleanup:** None required.

---

### TEST-019: ck_cc_unlikely_identity

**Category:** basic

**Tests Requirement:** CK_CC_UNLIKELY preserves boolean value

**Setup:**
1. None required

**Action:**
1. Evaluate CK_CC_UNLIKELY(1)
2. Evaluate CK_CC_UNLIKELY(0)

**Expected Result:**
- CK_CC_UNLIKELY(1) evaluates to non-zero (true)
- CK_CC_UNLIKELY(0) evaluates to zero (false)

**Cleanup:** None required.

---

### TEST-020: ck_cc_container_offset

**Category:** basic

**Tests Requirement:** CK_CC_CONTAINER correctly computes containing structure pointer

**Setup:**
1. Define struct: `struct test { int a; int member; int c; }`
2. Instantiate: `struct test t = {1, 2, 3};`
3. Define container function using CK_CC_CONTAINER

**Action:**
1. Get pointer to t.member
2. Use container function to recover pointer to t

**Expected Result:**
- Recovered pointer equals &t
- Accessing recovered->a returns 1
- Accessing recovered->c returns 3

**Cleanup:** None required.

---

## Coverage Matrix

| Requirement (spec.md) | Test IDs | Status |
|-----------------------|----------|--------|
| ck_cc_ffs: returns 0 for zero | TEST-001 | Covered |
| ck_cc_ffs: returns 1-indexed position | TEST-002, TEST-003, TEST-004, TEST-005 | Covered |
| ck_cc_ffsl: returns 0 for zero | TEST-006 | Covered |
| ck_cc_ffsl: returns 1-indexed position | TEST-007 | Covered |
| ck_cc_ffsll: returns 0 for zero | TEST-008 | Covered |
| ck_cc_ffsll: returns 1-indexed position | TEST-009 | Covered |
| ck_cc_ctz: returns 0 for zero | TEST-010 | Covered |
| ck_cc_ctz: returns trailing zero count | TEST-011, TEST-012, TEST-013 | Covered |
| ck_cc_popcount: returns 0 for zero | TEST-014 | Covered |
| ck_cc_popcount: returns bit count | TEST-015, TEST-016, TEST-017 | Covered |
| CK_CC_LIKELY: preserves value | TEST-018 | Covered |
| CK_CC_UNLIKELY: preserves value | TEST-019 | Covered |
| CK_CC_CONTAINER: computes offset | TEST-020 | Covered |

## Test Gaps

| Requirement | Reason | Recommendation |
|-------------|--------|----------------|
| CK_CC_ALIGN alignment | Alignment verification is platform-specific | Add platform-specific alignment verification tests |
| CK_CC_CACHELINE alignment | Cache line size varies by platform | Verify alignment to CK_MD_CACHELINE value |
| CK_CC_PACKED struct size | Packing verification is struct-specific | Add struct size verification tests |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| None found | — | — |

No dedicated regression tests found for ck_cc module in the source repository. The module is implicitly tested through usage by other modules.
