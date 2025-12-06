# Module: ck_rhs

## Overview

The ck_rhs module implements a concurrent hash set using Robin Hood hashing. Robin Hood hashing improves on standard linear probing by keeping probe sequences short: during insertion, entries with shorter probe distances "steal" slots from entries with longer distances. This results in more uniform probe lengths and better cache performance.

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

### struct ck_rhs

**Description:** Hash set handle.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| m | struct ck_malloc* | Platform pointer | Memory allocator |
| map | struct ck_rhs_map* | Platform pointer | Current map |
| mode | unsigned int | 4 bytes | Operation mode flags |
| load_factor | unsigned int | 4 bytes | Load factor percentage |
| seed | unsigned long | Platform word | Hash seed |
| hf | ck_rhs_hash_cb_t* | Platform pointer | Hash function |
| compare | ck_rhs_compare_cb_t* | Platform pointer | Comparison function |

### struct ck_rhs_stat

**Description:** Statistics.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| n_entries | unsigned long | Platform word | Entry count |
| probe_maximum | unsigned int | 4 bytes | Max probe length |

### struct ck_rhs_iterator

**Description:** Iteration state.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| cursor | void** | Platform pointer | Current position |
| offset | unsigned long | Platform word | Offset in map |

## Algorithms

### ck_rhs_init

**Purpose:** Initialize Robin Hood hash set.

**Algorithm:**
1. Allocate map with given capacity
2. Store hash/compare callbacks
3. Store mode, load_factor, seed

**Complexity:** O(capacity)

### ck_rhs_put

**Purpose:** Insert element using Robin Hood strategy.

**Algorithm:**
1. Compute hash, start probe
2. For each slot:
   - IF empty: insert, done
   - IF key match: return false (duplicate)
   - Compute probe distance of current occupant
   - IF our distance > occupant distance:
     - Swap (steal slot)
     - Continue with displaced entry
3. Handle overflow/grow

**Complexity:** O(1) average

### ck_rhs_get

**Purpose:** Look up element.

**Algorithm:**
1. Compute hash
2. Linear probe, track probe distance
3. IF entry probe distance < our distance: not found
4. Compare keys until match or impossible

**Complexity:** O(1) average

### ck_rhs_remove

**Purpose:** Remove element with backward shift.

**Algorithm:**
1. Find element
2. Mark as tombstone or shift entries back
3. MODE_READ_MOSTLY may use tombstones

**Complexity:** O(1) average

### ck_rhs_set_load_factor

**Purpose:** Adjust target load factor.

**Algorithm:**
1. Update load_factor field
2. May trigger grow on next insert

**Complexity:** O(1)

### Robin Hood Invariant

**Property:** For any entry at slot i:
- probe_distance(i) <= probe_distance(i+1) + 1

This ensures entries are roughly sorted by their probe distances, making lookups predictable.

## Concurrency

**Thread Safety:** SPMC (single writer, multiple readers).

**Progress Guarantee:**
- Read operations: Wait-free
- Write operations: Blocking

**Memory Ordering:**
- Reads use atomic loads
- Writes use fences before publishing

## Platform Considerations

**Modes:**
- CK_RHS_MODE_SPMC: Read-optimized
- CK_RHS_MODE_DIRECT: Integer values
- CK_RHS_MODE_OBJECT: Pointer values with packing
- CK_RHS_MODE_READ_MOSTLY: Optimize get over put/delete

**Pointer Packing:** CK_RHS_PP similar to ck_hs.

**Load Factor:** Configurable via ck_rhs_set_load_factor.
