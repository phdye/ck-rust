# Module: ck_malloc — Test Specification

## Conformance Tests

### TEST-001: struct_size_verification

**Category:** basic

**Tests Requirement:** struct ck_malloc has expected size

**Setup:**
1. None required

**Action:**
1. Check sizeof(struct ck_malloc)

**Expected Result:**
- On 64-bit: sizeof(struct ck_malloc) == 24 (3 pointers × 8 bytes)
- On 32-bit: sizeof(struct ck_malloc) == 12 (3 pointers × 4 bytes)

**Cleanup:** None required.

---

### TEST-002: malloc_wrapper_basic

**Category:** basic

**Tests Requirement:** malloc function pointer can be called

**Setup:**
1. Create test allocator wrapping standard malloc
2. Initialize struct ck_malloc with test functions

**Action:**
1. Call allocator.malloc(1024)
2. Verify returned pointer is not NULL
3. Write to allocated memory
4. Call allocator.free with allocated pointer

**Expected Result:**
- malloc returns non-NULL pointer
- Memory is usable
- No crash on free

**Cleanup:**
1. Free allocated memory if test fails mid-way

---

### TEST-003: realloc_may_move_true

**Category:** basic

**Tests Requirement:** realloc with may_move=true can return different pointer

**Setup:**
1. Create test allocator
2. Initialize struct ck_malloc

**Action:**
1. Allocate small buffer (e.g., 16 bytes)
2. Call realloc with larger size (e.g., 16MB) and may_move=true
3. Check if data is preserved

**Expected Result:**
- realloc returns non-NULL pointer
- Original data is preserved at new location
- Pointer may differ from original

**Cleanup:**
1. Free final pointer

---

### TEST-004: realloc_may_move_false

**Category:** basic

**Tests Requirement:** realloc with may_move=false either resizes in place or fails

**Setup:**
1. Create test allocator that tracks whether realloc moved
2. Initialize struct ck_malloc

**Action:**
1. Allocate buffer
2. Call realloc with larger size and may_move=false
3. Check result

**Expected Result:**
- IF realloc returns non-NULL: pointer equals original pointer
- IF realloc returns NULL: original memory is still valid

**Cleanup:**
1. Free memory

---

### TEST-005: free_with_size

**Category:** basic

**Tests Requirement:** free receives correct size parameter

**Setup:**
1. Create test allocator that verifies size parameter
2. Initialize struct ck_malloc

**Action:**
1. Allocate 1024 bytes
2. Call free with size=1024

**Expected Result:**
- Free receives size=1024

**Cleanup:** None required (memory freed by test).

---

### TEST-006: free_defer_true

**Category:** basic

**Tests Requirement:** free with defer=true allows deferred deallocation

**Setup:**
1. Create test allocator that tracks deferred frees
2. Initialize struct ck_malloc

**Action:**
1. Allocate memory
2. Call free with defer=true
3. Check allocator state

**Expected Result:**
- Allocator records deferred free request
- Actual deallocation may be delayed

**Cleanup:**
1. Ensure allocator flushes deferred frees

---

### TEST-007: free_defer_false

**Category:** basic

**Tests Requirement:** free with defer=false deallocates immediately

**Setup:**
1. Create test allocator that tracks immediate frees
2. Initialize struct ck_malloc

**Action:**
1. Allocate memory
2. Call free with defer=false
3. Check allocator state

**Expected Result:**
- Memory is deallocated immediately

**Cleanup:** None required.

---

### TEST-008: null_pointer_handling

**Category:** edge_case

**Tests Requirement:** Behavior with NULL pointers is defined by allocator

**Setup:**
1. Create test allocator with defined NULL handling

**Action:**
1. Call realloc(NULL, 0, 1024, true) — should behave like malloc
2. Call free(NULL, 0, false) — should be no-op

**Expected Result:**
- realloc(NULL, ...) allocates new memory
- free(NULL, ...) is a no-op

**Cleanup:**
1. Free allocated memory

---

### TEST-009: zero_size_allocation

**Category:** edge_case

**Tests Requirement:** Zero-size allocation behavior

**Setup:**
1. Create test allocator

**Action:**
1. Call malloc(0)

**Expected Result:**
- Returns either NULL or a unique pointer that can be freed
- Behavior is implementation-defined per C standard

**Cleanup:**
1. Free if non-NULL returned

---

## Coverage Matrix

| Requirement (spec.md) | Test IDs | Status |
|-----------------------|----------|--------|
| struct ck_malloc size/layout | TEST-001 | Covered |
| malloc function pointer works | TEST-002 | Covered |
| realloc with may_move=true | TEST-003 | Covered |
| realloc with may_move=false | TEST-004 | Covered |
| free receives size parameter | TEST-005 | Covered |
| free defer=true semantics | TEST-006 | Covered |
| free defer=false semantics | TEST-007 | Covered |
| NULL pointer handling | TEST-008 | Covered |
| Zero-size allocation | TEST-009 | Covered |

## Test Gaps

| Requirement | Reason | Recommendation |
|-------------|--------|----------------|
| Thread safety of allocator functions | Module defines interface, not implementation | Add concurrent stress tests in data structure tests |
| realloc shrink behavior | Not explicitly specified | Add test for shrinking allocations |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| None found | — | — |

No dedicated regression tests found for ck_malloc. The interface is implicitly tested through data structures that use it (ck_hs, ck_ht, ck_rhs, ck_array).
