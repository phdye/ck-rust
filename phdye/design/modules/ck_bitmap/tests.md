# Module: ck_bitmap — Test Specification

## Conformance Tests

### TEST-001: size_computation

**Category:** basic

**Tests Requirement:** ck_bitmap_size returns correct allocation size

**Setup:**
1. Test various n_bits values

**Action:**
1. Call ck_bitmap_size(0), ck_bitmap_size(1), ck_bitmap_size(32), ck_bitmap_size(33), ck_bitmap_size(100)

**Expected Result:**
- size(0) = header size
- size(1) = header + 1 word
- size(32) = header + 1 word
- size(33) = header + 2 words
- Sizes account for word granularity

**Cleanup:** None required.

---

### TEST-002: init_all_zeros

**Category:** basic

**Tests Requirement:** ck_bitmap_init with set=false creates all-zero bitmap

**Setup:**
1. Allocate bitmap for 100 bits

**Action:**
1. ck_bitmap_init(&bitmap, 100, false)
2. Test each bit 0..99

**Expected Result:**
- All bits return false from ck_bitmap_test
- ck_bitmap_empty(&bitmap, 100) returns true

**Cleanup:** Free bitmap.

---

### TEST-003: init_all_ones

**Category:** basic

**Tests Requirement:** ck_bitmap_init with set=true creates all-one bitmap

**Setup:**
1. Allocate bitmap for 100 bits

**Action:**
1. ck_bitmap_init(&bitmap, 100, true)
2. Test each bit 0..99

**Expected Result:**
- All bits return true from ck_bitmap_test
- ck_bitmap_full(&bitmap, 100) returns true

**Cleanup:** Free bitmap.

---

### TEST-004: set_single_bit

**Category:** basic

**Tests Requirement:** ck_bitmap_set sets individual bit

**Setup:**
1. Init all-zero bitmap with 64 bits

**Action:**
1. ck_bitmap_set(&bitmap, 42)

**Expected Result:**
- ck_bitmap_test(&bitmap, 42) returns true
- All other bits remain false

**Cleanup:** Free bitmap.

---

### TEST-005: reset_single_bit

**Category:** basic

**Tests Requirement:** ck_bitmap_reset clears individual bit

**Setup:**
1. Init all-one bitmap with 64 bits

**Action:**
1. ck_bitmap_reset(&bitmap, 42)

**Expected Result:**
- ck_bitmap_test(&bitmap, 42) returns false
- All other bits remain true

**Cleanup:** Free bitmap.

---

### TEST-006: bts_returns_previous

**Category:** basic

**Tests Requirement:** ck_bitmap_bts returns previous value

**Setup:**
1. Init all-zero bitmap with 64 bits

**Action:**
1. result1 = ck_bitmap_bts(&bitmap, 10)  // 0 -> 1
2. result2 = ck_bitmap_bts(&bitmap, 10)  // 1 -> 1

**Expected Result:**
- result1 == false (was 0)
- result2 == true (was 1)
- Bit 10 is set after both calls

**Cleanup:** Free bitmap.

---

### TEST-007: count_bits

**Category:** basic

**Tests Requirement:** ck_bitmap_count returns correct popcount

**Setup:**
1. Init all-zero bitmap with 100 bits
2. Set bits at positions 0, 10, 20, 30, 99

**Action:**
1. count = ck_bitmap_count(&bitmap, 100)

**Expected Result:**
- count == 5

**Cleanup:** Free bitmap.

---

### TEST-008: count_with_limit

**Category:** basic

**Tests Requirement:** ck_bitmap_count respects limit

**Setup:**
1. Init all-zero bitmap with 100 bits
2. Set bits at 10, 20, 30, 50, 90

**Action:**
1. count1 = ck_bitmap_count(&bitmap, 25)  // Only 10, 20
2. count2 = ck_bitmap_count(&bitmap, 100) // All 5

**Expected Result:**
- count1 == 2
- count2 == 5

**Cleanup:** Free bitmap.

---

### TEST-009: union_operation

**Category:** basic

**Tests Requirement:** ck_bitmap_union performs bitwise OR

**Setup:**
1. Init bitmap1 with bits 0, 2, 4
2. Init bitmap2 with bits 1, 2, 3

**Action:**
1. ck_bitmap_union(&bitmap1, &bitmap2)

**Expected Result:**
- bitmap1 has bits 0, 1, 2, 3, 4 set

**Cleanup:** Free bitmaps.

---

### TEST-010: intersection_operation

**Category:** basic

**Tests Requirement:** ck_bitmap_intersection performs bitwise AND

**Setup:**
1. Init bitmap1 with bits 0, 2, 4, 6
2. Init bitmap2 with bits 2, 4, 8

**Action:**
1. ck_bitmap_intersection(&bitmap1, &bitmap2)

**Expected Result:**
- bitmap1 has only bits 2, 4 set

**Cleanup:** Free bitmaps.

---

### TEST-011: intersection_negate

**Category:** basic

**Tests Requirement:** ck_bitmap_intersection_negate performs AND NOT

**Setup:**
1. Init bitmap1 with bits 0, 1, 2, 3, 4
2. Init bitmap2 with bits 1, 3

**Action:**
1. ck_bitmap_intersection_negate(&bitmap1, &bitmap2)

**Expected Result:**
- bitmap1 has bits 0, 2, 4 set (original minus bitmap2)

**Cleanup:** Free bitmaps.

---

### TEST-012: clear_all

**Category:** basic

**Tests Requirement:** ck_bitmap_clear sets all bits to zero

**Setup:**
1. Init all-one bitmap with 100 bits

**Action:**
1. ck_bitmap_clear(&bitmap)

**Expected Result:**
- ck_bitmap_empty(&bitmap, 100) returns true
- All bits test as false

**Cleanup:** Free bitmap.

---

### TEST-013: iterator_basic

**Category:** basic

**Tests Requirement:** Iterator visits all set bits

**Setup:**
1. Init bitmap with bits 5, 10, 15, 20, 63, 64, 65

**Action:**
1. Initialize iterator
2. Collect all bits returned by ck_bitmap_next

**Expected Result:**
- Exactly 7 bits returned
- Bits are 5, 10, 15, 20, 63, 64, 65 (in order)

**Cleanup:** Free bitmap.

---

### TEST-014: iterator_empty_bitmap

**Category:** edge_case

**Tests Requirement:** Iterator handles empty bitmap

**Setup:**
1. Init all-zero bitmap with 100 bits

**Action:**
1. Initialize iterator
2. Call ck_bitmap_next

**Expected Result:**
- ck_bitmap_next returns false immediately

**Cleanup:** Free bitmap.

---

### TEST-015: concurrent_set_different_bits

**Category:** concurrent

**Tests Requirement:** Concurrent set on different bits is safe

**Setup:**
1. Init all-zero bitmap with 256 bits
2. Create N threads (N = 8)
3. Each thread sets bits i*32..(i+1)*32-1

**Action:**
1. Start all threads
2. Each thread sets its range of bits
3. Wait for all threads

**Expected Result:**
- All 256 bits are set
- No crashes or data races

**Cleanup:**
1. Join threads
2. Free bitmap

---

### TEST-016: concurrent_set_same_bit

**Category:** concurrent

**Tests Requirement:** Concurrent set on same bit is safe

**Setup:**
1. Init all-zero bitmap with 64 bits
2. Create N threads (N = 8)

**Action:**
1. All threads set bit 0 simultaneously (1000 times each)

**Expected Result:**
- Bit 0 is set
- No crashes

**Cleanup:**
1. Join threads
2. Free bitmap

---

### TEST-017: concurrent_bts_count

**Category:** concurrent

**Tests Requirement:** bts can be used for atomic counting

**Setup:**
1. Init all-zero bitmap with 1000 bits
2. Create N threads (N = 8)
3. Counter = 0

**Action:**
1. Each thread loops 1000 times:
   - Pick random bit 0..999
   - IF bts returns false: increment local counter
2. Sum local counters

**Expected Result:**
- Sum equals ck_bitmap_count(&bitmap, 1000)
- Each bit counted exactly once

**Cleanup:**
1. Join threads
2. Free bitmap

---

### TEST-018: instance_macro

**Category:** basic

**Tests Requirement:** CK_BITMAP_INSTANCE creates valid stack bitmap

**Setup:**
1. Declare CK_BITMAP_INSTANCE(100) on stack

**Action:**
1. CK_BITMAP_INIT(&instance, 100, false)
2. CK_BITMAP_SET(&instance, 50)
3. result = CK_BITMAP_TEST(&instance, 50)

**Expected Result:**
- result == true
- Works without heap allocation

**Cleanup:** None (stack allocated).

---

## Coverage Matrix

| Requirement (spec.md) | Test IDs | Status |
|-----------------------|----------|--------|
| ck_bitmap_size | TEST-001 | Covered |
| ck_bitmap_init (zeros) | TEST-002 | Covered |
| ck_bitmap_init (ones) | TEST-003 | Covered |
| ck_bitmap_set | TEST-004 | Covered |
| ck_bitmap_reset | TEST-005 | Covered |
| ck_bitmap_bts | TEST-006 | Covered |
| ck_bitmap_test | TEST-002, TEST-003 | Covered |
| ck_bitmap_count | TEST-007, TEST-008 | Covered |
| ck_bitmap_union | TEST-009 | Covered |
| ck_bitmap_intersection | TEST-010 | Covered |
| ck_bitmap_intersection_negate | TEST-011 | Covered |
| ck_bitmap_clear | TEST-012 | Covered |
| ck_bitmap_empty | TEST-002, TEST-012 | Covered |
| ck_bitmap_full | TEST-003 | Covered |
| Iterator | TEST-013, TEST-014 | Covered |
| Concurrent safety | TEST-015, TEST-016, TEST-017 | Covered |
| CK_BITMAP_INSTANCE | TEST-018 | Covered |

## Test Gaps

| Requirement | Reason | Recommendation |
|-------------|--------|----------------|
| ck_bitmap_count_intersect | Less commonly used | Add basic test |
| Mismatched size bitmaps | Edge case | Add test for union/intersection with different sizes |
| Platform without required ops | Compile-time failure | Document platform requirements |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_bitmap regression | regressions/ck_bitmap/ | TEST-001 through TEST-018 |
