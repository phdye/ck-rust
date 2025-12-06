# Module: ck_hs

## Overview

The ck_hs module implements a concurrent hash set with single-writer, multiple-reader (SPMC) semantics. It uses open addressing with linear probing and supports both pointer values and direct integer values. The hash set is optimized for read-heavy workloads with optional pointer packing for space efficiency.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Inline hints |
| ck_malloc | Internal | Memory allocation interface |
| ck_md | Internal | Platform-specific definitions |
| ck_pr | Internal | Atomic operations |
| ck_stdint | External | Fixed-width integers |
| ck_stdbool | External | bool type |
| ck_stddef | External | NULL definition |

## Data Structures

### struct ck_hs

**Description:** Hash set handle.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| m | struct ck_malloc* | Platform pointer | Memory allocator |
| map | struct ck_hs_map* | Platform pointer | Current hash map |
| mode | unsigned int | 4 bytes | Operation mode flags |
| seed | unsigned long | Platform word | Hash seed |
| hf | ck_hs_hash_cb_t* | Platform pointer | Hash function |
| compare | ck_hs_compare_cb_t* | Platform pointer | Comparison function |

### struct ck_hs_map (Internal)

**Description:** Internal hash table storage.

**Fields:**
- Capacity, probe bound, entry count
- Array of entries (slots)

### struct ck_hs_stat

**Description:** Hash set statistics.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| tombstones | unsigned long | Platform word | Deleted slot count |
| n_entries | unsigned long | Platform word | Live entry count |
| probe_maximum | unsigned int | 4 bytes | Maximum probe length |

### struct ck_hs_iterator

**Description:** Iteration state.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| cursor | void** | Platform pointer | Current position |
| offset | unsigned long | Platform word | Offset in map |
| map | struct ck_hs_map* | Platform pointer | Map snapshot |

## Algorithms

### ck_hs_init

**Purpose:** Initialize hash set.

**Algorithm:**
1. Allocate initial map with given capacity
2. Store hash function, compare function, mode
3. Store allocator and seed

**Complexity:** O(capacity)

### ck_hs_put

**Purpose:** Insert element (fail if exists).

**Algorithm:**
1. Compute hash
2. Linear probe for empty slot or match
3. IF match found: return false
4. Insert at empty slot
5. Update count

**Complexity:** O(1) average, O(n) worst case

### ck_hs_put_unique

**Purpose:** Insert element (caller guarantees uniqueness).

**Algorithm:**
1. Compute hash
2. Linear probe for empty slot only
3. Insert at empty slot

**Complexity:** O(1) average

### ck_hs_set

**Purpose:** Insert or replace element.

**Algorithm:**
1. Compute hash
2. Linear probe
3. IF match: replace, return old value
4. ELSE: insert, return NULL

**Complexity:** O(1) average

### ck_hs_get

**Purpose:** Look up element.

**Algorithm:**
1. Compute hash
2. Linear probe
3. Compare with each probed slot
4. Return match or NULL

**Complexity:** O(1) average

### ck_hs_remove

**Purpose:** Remove element.

**Algorithm:**
1. Compute hash
2. Find element
3. Mark slot as tombstone
4. Decrement count

**Complexity:** O(1) average

### ck_hs_grow

**Purpose:** Increase capacity.

**Algorithm:**
1. Allocate new map with larger capacity
2. Rehash all entries
3. Atomic swap maps
4. Free old map

**Complexity:** O(n)

### ck_hs_gc

**Purpose:** Garbage collect tombstones.

**Algorithm:**
1. Rebuild map without tombstones
2. Optionally resize

**Complexity:** O(n)

## Concurrency

**Thread Safety:** SPMC (single writer, multiple readers).

**Progress Guarantee:**
- Read operations: Wait-free
- Write operations: Blocking

**Memory Ordering:**
- Reads use atomic loads with appropriate ordering
- Writes use store fences before publishing

## Platform Considerations

**Modes:**
- CK_HS_MODE_SPMC: Single-writer, multiple-reader
- CK_HS_MODE_DIRECT: Store integer values directly
- CK_HS_MODE_OBJECT: Store pointers (allows packing)
- CK_HS_MODE_DELETE: Optimize for delete-heavy workloads

**Pointer Packing:** On platforms with CK_HS_PP, high bits of pointers can store hash bits for faster comparison.
