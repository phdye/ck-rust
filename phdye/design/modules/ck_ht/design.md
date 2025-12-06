# Module: ck_ht

## Overview

The ck_ht module implements a concurrent hash table (key-value map) with single-writer, multiple-reader (SPMC) semantics. Unlike ck_hs (set), ck_ht stores key-value pairs. It uses open addressing and supports both direct integer keys and byte-string keys with optional pointer packing.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_pr | Internal | Atomic operations |
| ck_cc | Internal | Inline hints |
| ck_malloc | Internal | Memory allocation interface |
| ck_md | Internal | Platform-specific definitions |
| ck_stdint | External | Fixed-width integers |
| ck_stdbool | External | bool type |
| ck_stddef | External | NULL definition |

## Data Structures

### struct ck_ht_hash

**Description:** Hash value wrapper.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| value | uint64_t | 8 bytes | Computed hash value |

### struct ck_ht_entry

**Description:** Hash table entry (key-value pair).

**Fields (with pointer packing):**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| key | uintptr_t | Platform word | Key (possibly packed) |
| value | uintptr_t | Platform word | Value (possibly packed) |

**Fields (without pointer packing):**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| key | uintptr_t | Platform word | Key pointer |
| value | uintptr_t | Platform word | Value pointer |
| key_length | CK_HT_TYPE | 4/8 bytes | Key length |
| hash | CK_HT_TYPE | 4/8 bytes | Stored hash |

**Alignment:** 16 bytes (PP) or 32 bytes (non-PP)

**Special Values:**
- CK_HT_KEY_EMPTY: Slot is empty
- CK_HT_KEY_TOMBSTONE: Slot was deleted

### struct ck_ht

**Description:** Hash table handle.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| m | struct ck_malloc* | Platform pointer | Memory allocator |
| map | struct ck_ht_map* | Platform pointer | Current map |
| mode | unsigned int | 4 bytes | Operation mode |
| seed | uint64_t | 8 bytes | Hash seed |
| h | ck_ht_hash_cb_t* | Platform pointer | Hash callback |

### struct ck_ht_stat

**Description:** Statistics.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| probe_maximum | uint64_t | 8 bytes | Max probe length |
| n_entries | uint64_t | 8 bytes | Entry count |

## Algorithms

### ck_ht_init

**Purpose:** Initialize hash table.

**Algorithm:**
1. Allocate map with given capacity
2. Store mode, seed, hash callback, allocator

**Complexity:** O(capacity)

### ck_ht_put_spmc

**Purpose:** Insert entry (fail if exists).

**Algorithm:**
1. Compute hash
2. Linear probe for empty or tombstone slot
3. IF key match found: return false
4. Insert entry at slot

**Complexity:** O(1) average

### ck_ht_set_spmc

**Purpose:** Insert or replace entry.

**Algorithm:**
1. Compute hash
2. Linear probe
3. IF match: replace value
4. ELSE: insert new entry

**Complexity:** O(1) average

### ck_ht_get_spmc

**Purpose:** Look up entry.

**Algorithm:**
1. Compute hash from entry key
2. Linear probe
3. Compare key/hash at each slot
4. Return true if found, entry populated

**Complexity:** O(1) average

### ck_ht_remove_spmc

**Purpose:** Remove entry.

**Algorithm:**
1. Find entry
2. Mark slot as tombstone
3. Populate entry with removed key-value

**Complexity:** O(1) average

### ck_ht_grow_spmc

**Purpose:** Increase capacity.

**Algorithm:**
1. Allocate larger map
2. Rehash all entries
3. Atomic swap
4. Free old map

**Complexity:** O(n)

### ck_ht_hash / ck_ht_hash_direct

**Purpose:** Compute hash for key.

**Algorithm:**
1. Call user hash callback with key, length, seed
2. Store result in ck_ht_hash struct

**Complexity:** O(key_length) or O(1) for direct

## Concurrency

**Thread Safety:** SPMC (single writer, multiple readers).

**Progress Guarantee:**
- get_spmc: Wait-free
- put/set/remove_spmc: Blocking
- grow_spmc: Blocking

**Memory Ordering:**
- Readers use atomic loads
- Writers use fences before publishing

## Platform Considerations

**Modes:**
- CK_HT_MODE_DIRECT: Integer keys
- CK_HT_MODE_BYTESTRING: Byte-string keys
- CK_HT_WORKLOAD_DELETE: Delete-heavy optimization

**Pointer Packing:** CK_HT_PP packs key length and hash bits into pointer high bits on supported platforms.

**CK_HT_TYPE:** Uses 64-bit counters if available, else 32-bit.
