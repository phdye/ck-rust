# Module: ck_array

## Overview

The ck_array module implements a concurrent dynamic array with single-writer, multiple-reader (SPMC) semantics. It uses a copy-on-write approach where mutations build a new array that becomes visible atomically on commit. Readers can safely iterate without locks while a single writer performs modifications.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Inline hints |
| ck_malloc | Internal | Memory allocation interface |
| ck_pr | Internal | Atomic operations, fences |
| ck_stdbool | External | bool type |
| ck_stddef | External | NULL definition |

## Data Structures

### struct _ck_array (Internal)

**Description:** Internal array buffer with committed element count.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| n_committed | unsigned int | 4 bytes | Number of visible elements |
| length | unsigned int | 4 bytes | Total capacity |
| values | void*[] | Variable | Flexible array of element pointers |

### struct ck_array

**Description:** Public array handle with transaction support.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| allocator | struct ck_malloc* | Platform pointer | Memory allocator |
| active | struct _ck_array* | Platform pointer | Current visible array |
| n_entries | unsigned int | 4 bytes | Total entries (including uncommitted) |
| transaction | struct _ck_array* | Platform pointer | Pending modifications |

### struct ck_array_iterator

**Description:** Iterator for safe traversal.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| snapshot | struct _ck_array* | Platform pointer | Snapshot of active at iteration start |

## Algorithms

### ck_array_init

**Purpose:** Initialize array with initial capacity.

**Algorithm:**
1. Allocate initial _ck_array with given capacity
2. Set n_committed = 0, length = capacity
3. Set active = allocated array
4. Set transaction = NULL

**Complexity:** O(1)

### ck_array_put

**Purpose:** Add element to array (writer only).

**Algorithm:**
1. IF no transaction: allocate transaction as copy of active
2. IF transaction full: grow and copy
3. Add element to transaction
4. Increment n_entries

**Complexity:** O(n) if grow needed, O(1) amortized

### ck_array_put_unique

**Purpose:** Add element only if not present.

**Algorithm:**
1. Search active array for element
2. IF found: return 1 (exists)
3. IF not found: call ck_array_put
4. Return 0 on success, -1 on failure

**Complexity:** O(n) search + put cost

### ck_array_remove

**Purpose:** Remove element from array (writer only).

**Algorithm:**
1. IF no transaction: allocate transaction as copy of active
2. Find element in transaction
3. Swap with last element, decrement count
4. Decrement n_entries

**Complexity:** O(n) search

### ck_array_commit

**Purpose:** Atomically publish pending modifications.

**Algorithm:**
1. IF no transaction: return true (nothing to commit)
2. Copy n_entries to transaction->n_committed
3. Store fence
4. Atomic store transaction to active
5. Free old active (after safe reclamation)
6. Clear transaction pointer

**Complexity:** O(1) for commit, deferred free

### ck_array_length

**Purpose:** Get current committed element count.

**Algorithm:**
1. Atomic load active pointer
2. Load fence
3. Atomic load n_committed

**Complexity:** O(1)

### CK_ARRAY_FOREACH

**Purpose:** Safely iterate over array elements.

**Algorithm:**
1. Snapshot active pointer
2. Load fence
3. Iterate from 0 to n_committed

**Complexity:** O(n)

## Concurrency

**Thread Safety:** SPMC (single writer, multiple readers).

**Progress Guarantee:**
- Read operations: Wait-free
- Write operations: Blocking (allocation)

**Memory Ordering:**
- Commit uses store fence before publishing
- Length/iterator use load fence after snapshot

## Platform Considerations

- MPMC mode declared but unsupported
- Relies on ck_malloc for allocation
- Safe memory reclamation needed for freed arrays
