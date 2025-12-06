# Module: ck_queue

## Overview

The ck_queue module provides BSD-style intrusive linked list implementations with concurrent reader support. It includes singly-linked lists (SLIST), singly-linked tail queues (STAILQ), and doubly-linked lists (LIST). All variants support safe concurrent iteration while mutations require external synchronization.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_pr | Internal | Atomic loads and stores for concurrent access |

## Data Structures

### CK_SLIST_HEAD

**Description:** Head structure for singly-linked list.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| cslh_first | Pointer to type | Platform pointer size | First element in list |

**Invariants:**
- cslh_first is NULL or points to valid element [OBSERVED]

### CK_SLIST_ENTRY

**Description:** Entry structure embedded in user types for SLIST.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| csle_next | Pointer to type | Platform pointer size | Next element in list |

### CK_STAILQ_HEAD

**Description:** Head structure for singly-linked tail queue.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| cstqh_first | Pointer to type | Platform pointer size | First element in queue |
| cstqh_last | Pointer to pointer | Platform pointer size | Address of last element's next pointer |

**Invariants:**
- cstqh_first is NULL or points to valid element [OBSERVED]
- cstqh_last points to last element's next field or &cstqh_first [OBSERVED]

### CK_STAILQ_ENTRY

**Description:** Entry structure embedded in user types for STAILQ.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| cstqe_next | Pointer to type | Platform pointer size | Next element in queue |

### CK_LIST_HEAD

**Description:** Head structure for doubly-linked list.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| clh_first | Pointer to type | Platform pointer size | First element in list |

### CK_LIST_ENTRY

**Description:** Entry structure embedded in user types for LIST (doubly-linked).

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| cle_next | Pointer to type | Platform pointer size | Next element |
| cle_prev | Pointer to pointer | Platform pointer size | Address of previous next pointer |

**Invariants:**
- Doubly-linked invariant maintained: *(elm->cle_prev) == elm [OBSERVED]

## Algorithms

### CK_SLIST_INIT

**Purpose:** Initialize singly-linked list to empty state

**Algorithm:**
1. Atomic store NULL to head->cslh_first
2. Store fence

**Complexity:** O(1)

### CK_SLIST_INSERT_HEAD

**Purpose:** Insert element at head of list

**Algorithm:**
1. Set elm->next = head->first
2. Store fence (ensures next visible before head update)
3. Atomic store elm to head->first

**Complexity:** O(1)

### CK_SLIST_INSERT_AFTER

**Purpose:** Insert element after specified element

**Algorithm:**
1. Set new->next = after->next
2. Store fence
3. Atomic store new to after->next

**Complexity:** O(1)

### CK_SLIST_REMOVE_HEAD

**Purpose:** Remove first element from list

**Algorithm:**
1. Atomic store head->first->next to head->first

**Complexity:** O(1)

### CK_SLIST_REMOVE

**Purpose:** Remove specified element from list

**Algorithm:**
1. IF head->first == elm: REMOVE_HEAD
2. ELSE: Traverse to find predecessor, then REMOVE_AFTER

**Complexity:** O(n) - must traverse to find element

### CK_STAILQ_INIT

**Purpose:** Initialize tail queue to empty state

**Algorithm:**
1. Atomic store NULL to head->first
2. Store fence
3. Set head->last = &head->first

**Complexity:** O(1)

### CK_STAILQ_INSERT_TAIL

**Purpose:** Insert element at tail of queue (FIFO enqueue)

**Algorithm:**
1. Set elm->next = NULL
2. Store fence
3. Atomic store elm to *head->last
4. Update head->last = &elm->next

**Complexity:** O(1)

### CK_STAILQ_INSERT_HEAD

**Purpose:** Insert element at head of queue

**Algorithm:**
1. Set elm->next = head->first
2. Store fence
3. Atomic store elm to head->first
4. IF was empty: update head->last

**Complexity:** O(1)

### CK_LIST_INSERT_HEAD

**Purpose:** Insert element at head of doubly-linked list

**Algorithm:**
1. Set elm->next = head->first
2. Store fence
3. IF first not NULL: update first->prev
4. Atomic store elm to head->first
5. Set elm->prev = &head->first

**Complexity:** O(1)

### CK_LIST_REMOVE

**Purpose:** Remove element from doubly-linked list

**Algorithm:**
1. Atomic store elm->next to *elm->prev
2. IF elm->next not NULL: update elm->next->prev

**Complexity:** O(1) - no traversal needed

### Iteration Macros

**Purpose:** Safe concurrent iteration

**CK_SLIST_FOREACH:**
```
for (var = CK_SLIST_FIRST(head); var; var = CK_SLIST_NEXT(var, field))
```

**CK_SLIST_FOREACH_SAFE:**
- Caches next pointer before loop body
- Safe if current element is removed during iteration

**Note:** All FIRST/NEXT operations use atomic loads for concurrent safety.

## Concurrency

**Thread Safety:**
- Readers: Multiple concurrent readers are safe (use atomic loads)
- Writers: Require external synchronization
- FOREACH_SAFE: Safe if current element removed, not if arbitrary elements removed

**Memory Ordering:**
- Insertions: Store fence before linking (ensures data visible before reachable)
- Reads: Atomic loads for head and next pointers
- Not linearizable as a whole structure

**Progress Guarantee:**
- Readers: Wait-free
- Writers: Wait-free (but require exclusive access)

**Note:** The _SWAP operations are not atomic and require exclusive access.

## Platform Considerations

- Not supported on Alpha architecture (requires load-depend memory fences)
- Derived from BSD sys/queue.h with atomic operation additions
