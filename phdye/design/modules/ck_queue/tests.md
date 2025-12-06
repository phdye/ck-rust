# Module: ck_queue — Test Specification

## Conformance Tests

### TEST-001: slist_init_empty

**Category:** basic

**Tests Requirement:** CK_SLIST_INIT creates empty list

**Setup:**
1. Declare CK_SLIST_HEAD structure

**Action:**
1. CK_SLIST_INIT(&head)

**Expected Result:**
- CK_SLIST_EMPTY(&head) returns true
- CK_SLIST_FIRST(&head) returns NULL

**Cleanup:** None required.

---

### TEST-002: slist_insert_head

**Category:** basic

**Tests Requirement:** CK_SLIST_INSERT_HEAD adds element at front

**Setup:**
1. Initialize SLIST
2. Create element with CK_SLIST_ENTRY

**Action:**
1. CK_SLIST_INSERT_HEAD(&head, &elm, entry)

**Expected Result:**
- CK_SLIST_FIRST(&head) returns &elm
- CK_SLIST_EMPTY(&head) returns false

**Cleanup:** None required.

---

### TEST-003: slist_lifo_order

**Category:** basic

**Tests Requirement:** SLIST maintains LIFO order

**Setup:**
1. Initialize SLIST

**Action:**
1. Insert elm1, elm2, elm3 at head (in that order)
2. Traverse with FOREACH

**Expected Result:**
- Order of traversal: elm3, elm2, elm1

**Cleanup:** None required.

---

### TEST-004: slist_insert_after

**Category:** basic

**Tests Requirement:** CK_SLIST_INSERT_AFTER inserts correctly

**Setup:**
1. Initialize SLIST with elm1

**Action:**
1. CK_SLIST_INSERT_AFTER(&elm1, &elm2, entry)

**Expected Result:**
- CK_SLIST_NEXT(&elm1, entry) returns &elm2

**Cleanup:** None required.

---

### TEST-005: slist_remove_head

**Category:** basic

**Tests Requirement:** CK_SLIST_REMOVE_HEAD removes first element

**Setup:**
1. Initialize SLIST with elm1, elm2 (elm2 at head)

**Action:**
1. CK_SLIST_REMOVE_HEAD(&head, entry)

**Expected Result:**
- CK_SLIST_FIRST(&head) returns &elm1
- List has one element

**Cleanup:** None required.

---

### TEST-006: slist_remove_arbitrary

**Category:** basic

**Tests Requirement:** CK_SLIST_REMOVE removes specific element

**Setup:**
1. Initialize SLIST with elm1, elm2, elm3 (elm3 at head)

**Action:**
1. CK_SLIST_REMOVE(&head, &elm2, type, entry)

**Expected Result:**
- Traversal: elm3, elm1
- elm2 is no longer in list

**Cleanup:** None required.

---

### TEST-007: stailq_init_empty

**Category:** basic

**Tests Requirement:** CK_STAILQ_INIT creates empty queue

**Setup:**
1. Declare CK_STAILQ_HEAD structure

**Action:**
1. CK_STAILQ_INIT(&head)

**Expected Result:**
- CK_STAILQ_EMPTY(&head) returns true
- head.cstqh_last == &head.cstqh_first

**Cleanup:** None required.

---

### TEST-008: stailq_insert_tail_fifo

**Category:** basic

**Tests Requirement:** CK_STAILQ_INSERT_TAIL provides FIFO order

**Setup:**
1. Initialize STAILQ

**Action:**
1. Insert elm1, elm2, elm3 at tail (in that order)
2. Traverse with FOREACH

**Expected Result:**
- Order of traversal: elm1, elm2, elm3 (FIFO)

**Cleanup:** None required.

---

### TEST-009: stailq_concat

**Category:** basic

**Tests Requirement:** CK_STAILQ_CONCAT appends one queue to another

**Setup:**
1. Initialize queue1 with elm1, elm2
2. Initialize queue2 with elm3, elm4

**Action:**
1. CK_STAILQ_CONCAT(&queue1, &queue2)

**Expected Result:**
- queue1 contains elm1, elm2, elm3, elm4 in order
- queue2 is empty

**Cleanup:** None required.

---

### TEST-010: list_init_empty

**Category:** basic

**Tests Requirement:** CK_LIST_INIT creates empty list

**Setup:**
1. Declare CK_LIST_HEAD structure

**Action:**
1. CK_LIST_INIT(&head)

**Expected Result:**
- CK_LIST_EMPTY(&head) returns true
- CK_LIST_FIRST(&head) returns NULL

**Cleanup:** None required.

---

### TEST-011: list_remove_o1

**Category:** basic

**Tests Requirement:** CK_LIST_REMOVE is O(1)

**Setup:**
1. Initialize LIST with elm1, elm2, elm3

**Action:**
1. CK_LIST_REMOVE(&elm2, entry) (no traversal)

**Expected Result:**
- Traversal: elm3, elm1 (or elm1, elm3 depending on order)
- elm2 removed without searching

**Cleanup:** None required.

---

### TEST-012: list_insert_before

**Category:** basic

**Tests Requirement:** CK_LIST_INSERT_BEFORE inserts correctly

**Setup:**
1. Initialize LIST with elm1

**Action:**
1. CK_LIST_INSERT_BEFORE(&elm1, &elm2, entry)

**Expected Result:**
- Traversal order: elm2, elm1
- elm1->prev points to elm2's next field

**Cleanup:** None required.

---

### TEST-013: foreach_safe_removal

**Category:** basic

**Tests Requirement:** FOREACH_SAFE allows removal during iteration

**Setup:**
1. Initialize SLIST with elm1, elm2, elm3

**Action:**
1. Iterate with CK_SLIST_FOREACH_SAFE
2. Remove elm2 when encountered

**Expected Result:**
- Iteration completes without crash
- After iteration: list contains elm3, elm1

**Cleanup:** None required.

---

### TEST-014: concurrent_readers

**Category:** concurrent

**Tests Requirement:** Multiple concurrent readers are safe

**Setup:**
1. Initialize SLIST with 100 elements
2. Create N reader threads (N = 4)

**Action:**
1. Start all threads
2. Each thread: traverse entire list 1000 times with FOREACH
3. Count elements in each traversal

**Expected Result:**
- No crashes
- Each traversal sees consistent element pointers (no torn reads)

**Cleanup:**
1. Join all threads

---

### TEST-015: concurrent_read_write

**Category:** concurrent

**Tests Requirement:** Readers safe during writer modifications

**Setup:**
1. Initialize SLIST
2. Create writer thread and N reader threads (N = 4)
3. Writer will insert/remove elements with lock

**Action:**
1. Start all threads
2. Writer: lock, modify, unlock (repeat 10000 times)
3. Readers: continuously traverse without locking

**Expected Result:**
- No crashes
- Readers see valid list state (may be stale)

**Cleanup:**
1. Join all threads

---

### TEST-016: stailq_remove_head

**Category:** basic

**Tests Requirement:** CK_STAILQ_REMOVE_HEAD works correctly

**Setup:**
1. Initialize STAILQ with elm1, elm2

**Action:**
1. CK_STAILQ_REMOVE_HEAD(&head, entry)

**Expected Result:**
- CK_STAILQ_FIRST(&head) returns &elm2
- Tail pointer remains valid

**Cleanup:** None required.

---

### TEST-017: stailq_remove_to_empty

**Category:** edge_case

**Tests Requirement:** Removing last element updates tail correctly

**Setup:**
1. Initialize STAILQ with single element

**Action:**
1. CK_STAILQ_REMOVE_HEAD(&head, entry)

**Expected Result:**
- CK_STAILQ_EMPTY(&head) returns true
- head.cstqh_last == &head.cstqh_first

**Cleanup:** None required.

---

## Coverage Matrix

| Requirement (spec.md) | Test IDs | Status |
|-----------------------|----------|--------|
| CK_SLIST_INIT | TEST-001 | Covered |
| CK_SLIST_EMPTY | TEST-001, TEST-002 | Covered |
| CK_SLIST_FIRST | TEST-001, TEST-002 | Covered |
| CK_SLIST_NEXT | TEST-003 | Covered |
| CK_SLIST_INSERT_HEAD | TEST-002, TEST-003 | Covered |
| CK_SLIST_INSERT_AFTER | TEST-004 | Covered |
| CK_SLIST_REMOVE_HEAD | TEST-005 | Covered |
| CK_SLIST_REMOVE | TEST-006 | Covered |
| CK_STAILQ_INIT | TEST-007 | Covered |
| CK_STAILQ_INSERT_TAIL | TEST-008 | Covered |
| CK_STAILQ_CONCAT | TEST-009 | Covered |
| CK_STAILQ_REMOVE_HEAD | TEST-016, TEST-017 | Covered |
| CK_LIST_INIT | TEST-010 | Covered |
| CK_LIST_REMOVE | TEST-011 | Covered |
| CK_LIST_INSERT_BEFORE | TEST-012 | Covered |
| FOREACH_SAFE | TEST-013 | Covered |
| Concurrent readers | TEST-014 | Covered |
| Concurrent read/write | TEST-015 | Covered |

## Test Gaps

| Requirement | Reason | Recommendation |
|-------------|--------|----------------|
| CK_*_SWAP | Non-atomic, edge case | Add test verifying non-concurrent swap |
| CK_*_MOVE | Less commonly used | Add basic test |
| Memory ordering verification | Difficult to test | Use memory model checker |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_queue regression | regressions/ck_queue/ | TEST-001 through TEST-017 |
