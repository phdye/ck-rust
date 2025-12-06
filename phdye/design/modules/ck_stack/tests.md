# Module: ck_stack — Test Specification

## Conformance Tests

### TEST-001: init_creates_empty_stack

**Category:** basic

**Tests Requirement:** ck_stack_init creates an empty stack

**Setup:**
1. Allocate ck_stack structure

**Action:**
1. Call ck_stack_init(&stack)

**Expected Result:**
- stack.head == NULL
- stack.generation == NULL

**Cleanup:** None required.

---

### TEST-002: push_pop_spnc_single

**Category:** basic

**Tests Requirement:** Single element push/pop works correctly

**Setup:**
1. Initialize stack
2. Allocate structure containing ck_stack_entry

**Action:**
1. ck_stack_push_spnc(&stack, &entry)
2. result = ck_stack_pop_npsc(&stack)

**Expected Result:**
- result == &entry
- stack.head == NULL after pop

**Cleanup:** None required.

---

### TEST-003: push_pop_lifo_order

**Category:** basic

**Tests Requirement:** Stack maintains LIFO order

**Setup:**
1. Initialize stack
2. Allocate entries[3]

**Action:**
1. Push entries[0], entries[1], entries[2] in order
2. Pop three times

**Expected Result:**
- First pop returns entries[2]
- Second pop returns entries[1]
- Third pop returns entries[0]

**Cleanup:** None required.

---

### TEST-004: pop_empty_returns_null

**Category:** basic

**Tests Requirement:** Pop from empty stack returns NULL

**Setup:**
1. Initialize stack (empty)

**Action:**
1. result = ck_stack_pop_upmc(&stack)

**Expected Result:**
- result == NULL
- stack.head == NULL

**Cleanup:** None required.

---

### TEST-005: batch_pop_returns_all

**Category:** basic

**Tests Requirement:** Batch pop returns all entries

**Setup:**
1. Initialize stack
2. Push entries[0..9]

**Action:**
1. result = ck_stack_batch_pop_upmc(&stack)
2. Count entries by traversing result->next chain

**Expected Result:**
- result != NULL
- Chain contains exactly 10 entries
- stack.head == NULL after batch pop

**Cleanup:** None required.

---

### TEST-006: batch_pop_empty_returns_null

**Category:** basic

**Tests Requirement:** Batch pop from empty stack returns NULL

**Setup:**
1. Initialize stack (empty)

**Action:**
1. result = ck_stack_batch_pop_upmc(&stack)

**Expected Result:**
- result == NULL

**Cleanup:** None required.

---

### TEST-007: upmc_push_concurrent

**Category:** concurrent

**Tests Requirement:** Concurrent UPMC pushes are safe

**Setup:**
1. Initialize stack
2. Create N threads (N = 4)
3. Each thread has M unique entries (M = 1000)

**Action:**
1. Start all threads
2. Each thread: push all M entries using ck_stack_push_upmc
3. Wait for all threads to complete

**Expected Result:**
- Stack contains exactly N × M entries
- All entries are reachable from head
- No entries lost

**Cleanup:**
1. Join all threads
2. Free entries

---

### TEST-008: upmc_pop_concurrent

**Category:** concurrent

**Tests Requirement:** Concurrent UPMC pops are safe

**Setup:**
1. Initialize stack
2. Push N × M entries (N = 4, M = 1000)
3. Create N consumer threads

**Action:**
1. Start all threads
2. Each thread: pop until NULL, count entries
3. Wait for all threads

**Expected Result:**
- Sum of all thread counts == N × M
- Each entry returned exactly once
- Stack is empty after completion

**Cleanup:**
1. Join all threads

---

### TEST-009: upmc_concurrent_push_pop

**Category:** concurrent

**Tests Requirement:** Mixed concurrent push/pop is safe

**Setup:**
1. Initialize stack
2. Create P producer threads (P = 2)
3. Create C consumer threads (C = 2)
4. Each producer has M entries (M = 10000)

**Action:**
1. Start all threads
2. Producers: push all entries, signal done
3. Consumers: pop until empty and all producers done
4. Wait for all threads

**Expected Result:**
- Total popped == P × M
- No entries lost
- Stack eventually empty

**Cleanup:**
1. Join all threads

---

### TEST-010: mpmc_push_pop_with_reuse

**Category:** concurrent

**Tests Requirement:** MPMC allows safe entry reuse

**Setup:**
1. Initialize stack
2. Create N threads (N = 4)
3. Small pool of entries per thread (P = 10)
4. Each thread does M push/pop cycles (M = 10000)

**Action:**
1. Start all threads
2. Each thread:
   - Pop entry (may return NULL)
   - If entry, reuse immediately by pushing back
   - If NULL, push from local pool
3. Repeat M times per thread

**Expected Result:**
- No crashes or corruption
- No ABA-induced incorrect behavior
- All operations complete successfully

**Cleanup:**
1. Join all threads

---

### TEST-011: mpmc_generation_increments

**Category:** basic

**Tests Requirement:** Generation counter increments on MPMC pop

**Setup:**
1. Initialize stack
2. Push 3 entries

**Action:**
1. Record initial generation
2. ck_stack_pop_mpmc(&stack)
3. Record generation after first pop
4. ck_stack_pop_mpmc(&stack)
5. Record generation after second pop

**Expected Result:**
- Generation increases with each pop
- (May not be strictly +1 due to implementation details)

**Cleanup:** None required.

---

### TEST-012: mpnc_multiple_producers

**Category:** concurrent

**Tests Requirement:** MPNC supports concurrent producers

**Setup:**
1. Initialize stack
2. Create N producer threads (N = 4)
3. Each has M entries (M = 1000)
4. No consumers

**Action:**
1. Start all producer threads
2. Each thread: push all entries using ck_stack_push_mpnc
3. Wait for all threads
4. Memory barrier
5. Single-threaded drain of stack

**Expected Result:**
- Stack contains exactly N × M entries
- All entries reachable

**Cleanup:**
1. Join all threads

---

### TEST-013: spnc_push_npsc_pop_single_threaded

**Category:** basic

**Tests Requirement:** SPNC/NPSC variants work single-threaded

**Setup:**
1. Initialize stack
2. Allocate 100 entries

**Action:**
1. Push 100 entries using ck_stack_push_spnc
2. Pop 100 entries using ck_stack_pop_npsc
3. Verify each entry

**Expected Result:**
- All entries returned in LIFO order
- Stack is empty

**Cleanup:** None required.

---

### TEST-014: container_macro

**Category:** basic

**Tests Requirement:** CK_STACK_CONTAINER recovers enclosing structure

**Setup:**
1. Define structure with embedded ck_stack_entry
2. Allocate instance

**Action:**
1. Push &instance->entry
2. Pop to get entry pointer
3. Use CK_STACK_CONTAINER to recover instance pointer

**Expected Result:**
- Recovered pointer == &instance
- Structure fields accessible and correct

**Cleanup:** None required.

---

### TEST-015: batch_pop_preserves_order

**Category:** basic

**Tests Requirement:** Batch pop returns entries in LIFO order

**Setup:**
1. Initialize stack
2. Push entries[0], entries[1], entries[2], entries[3] in order

**Action:**
1. result = ck_stack_batch_pop_upmc(&stack)
2. Traverse result list

**Expected Result:**
- First in list: entries[3]
- Last in list: entries[0]
- Correct LIFO ordering preserved

**Cleanup:** None required.

---

### TEST-016: high_contention_stress

**Category:** stress

**Tests Requirement:** Stack survives high contention

**Setup:**
1. Initialize stack
2. Create N threads (N = 8)
3. M operations per thread (M = 100000)

**Action:**
1. Start all threads
2. Each thread: randomly push or pop
3. Track per-thread push/pop counts
4. Wait for all threads
5. Drain remaining entries

**Expected Result:**
- No crashes
- Total pushes - total pops == remaining entries
- Consistent accounting

**Cleanup:**
1. Join all threads

---

## Coverage Matrix

| Requirement (spec.md) | Test IDs | Status |
|-----------------------|----------|--------|
| ck_stack_init | TEST-001 | Covered |
| ck_stack_push_spnc | TEST-002, TEST-003, TEST-013 | Covered |
| ck_stack_pop_npsc | TEST-002, TEST-003, TEST-013 | Covered |
| ck_stack_push_upmc | TEST-007, TEST-009 | Covered |
| ck_stack_pop_upmc | TEST-004, TEST-008, TEST-009 | Covered |
| ck_stack_push_mpmc | TEST-010 | Covered |
| ck_stack_pop_mpmc | TEST-010, TEST-011 | Covered |
| ck_stack_push_mpnc | TEST-012 | Covered |
| ck_stack_batch_pop_upmc | TEST-005, TEST-006, TEST-015 | Covered |
| LIFO ordering | TEST-003, TEST-015 | Covered |
| Concurrent correctness | TEST-007 through TEST-012 | Covered |
| Container macro | TEST-014 | Covered |

## Test Gaps

| Requirement | Reason | Recommendation |
|-------------|--------|----------------|
| ck_stack_batch_pop_mpmc | Less commonly used | Add test with concurrent producers |
| Platform without double-width CAS | Requires specific platform | Test MPMC unavailability on 32-bit |
| Memory ordering verification | Difficult to test precisely | Use memory model checker (CDSChecker) |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_stack regression | regressions/ck_stack/ | TEST-001 through TEST-016 |
