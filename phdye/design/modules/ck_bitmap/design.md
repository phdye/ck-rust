# Module: ck_bitmap

## Overview

The ck_bitmap module provides a concurrent bitmap (bit array) implementation with atomic bit operations. It supports individual bit set/reset/test, bulk operations (union, intersection), population count, and iteration over set bits. The implementation uses word-sized atomic operations for thread safety.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Inline hints, popcount, ctz |
| ck_limits | External | CHAR_BIT constant |
| ck_pr | Internal | Atomic operations |
| ck_stdint | External | Fixed-width integer types |
| ck_stdbool | External | Boolean type |
| ck_stddef | External | NULL, size definitions |
| ck_string | External | memset for initialization |

## Data Structures

### struct ck_bitmap

**Description:** Bitmap with flexible array member for bits.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| n_bits | unsigned int | 4 bytes | Number of bits in bitmap |
| map | unsigned int[] | Variable | Bit storage array (flexible member) |

**Invariants:**
- n_bits is immutable after init [OBSERVED]
- map has ceil(n_bits / CK_BITMAP_BLOCK) words [OBSERVED]
- Trailing bits (beyond n_bits) are cleared [OBSERVED]

**Memory Layout:**
- Header: 4 bytes
- Bit array: ceil(n_bits / 32) × 4 bytes
- Use ck_bitmap_size(n_bits) to compute total size

### struct ck_bitmap_iterator

**Description:** State for iterating over set bits.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| cache | unsigned int | 4 bytes | Cached word being scanned |
| n_block | unsigned int | 4 bytes | Current block index |
| n_limit | unsigned int | 4 bytes | Total number of blocks |

## Algorithms

### ck_bitmap_size

**Purpose:** Compute bytes needed for bitmap with n_bits

**Signature:**
```
ck_bitmap_size(n_bits: unsigned int) → unsigned int
```

**Algorithm:**
1. blocks = ceil(n_bits / CK_BITMAP_BLOCK)
2. Return sizeof(ck_bitmap) + blocks × sizeof(unsigned int)

**Complexity:** O(1)

### ck_bitmap_init

**Purpose:** Initialize bitmap to all-zeros or all-ones

**Signature:**
```
ck_bitmap_init(bitmap: Pointer to ck_bitmap, n_bits: unsigned int, set: bool) → void
```

**Algorithm:**
1. Set bitmap->n_bits = n_bits
2. memset map to 0x00 (if !set) or 0xFF (if set)
3. IF set: Clear trailing bits beyond n_bits

**Complexity:** O(n_bits / 32)

### ck_bitmap_set

**Purpose:** Set bit at specified position

**Signature:**
```
ck_bitmap_set(bitmap: Pointer to ck_bitmap, n: unsigned int) → void
```

**Algorithm:**
1. word_ptr = bitmap->map + (n / CK_BITMAP_BLOCK)
2. bit_mask = 1 << (n % CK_BITMAP_BLOCK)
3. Atomic OR: ck_pr_or_uint(word_ptr, bit_mask)

**Complexity:** O(1)

### ck_bitmap_bts (Bit Test and Set)

**Purpose:** Set bit and return previous value

**Signature:**
```
ck_bitmap_bts(bitmap: Pointer to ck_bitmap, n: unsigned int) → bool
```

**Algorithm:**
1. word_ptr = bitmap->map + (n / CK_BITMAP_BLOCK)
2. bit_offset = n % CK_BITMAP_BLOCK
3. Return ck_pr_bts_uint(word_ptr, bit_offset)

**Complexity:** O(1)

### ck_bitmap_reset

**Purpose:** Clear bit at specified position

**Signature:**
```
ck_bitmap_reset(bitmap: Pointer to ck_bitmap, n: unsigned int) → void
```

**Algorithm:**
1. word_ptr = bitmap->map + (n / CK_BITMAP_BLOCK)
2. bit_mask = ~(1 << (n % CK_BITMAP_BLOCK))
3. Atomic AND: ck_pr_and_uint(word_ptr, bit_mask)

**Complexity:** O(1)

### ck_bitmap_test

**Purpose:** Test if bit is set

**Signature:**
```
ck_bitmap_test(bitmap: Pointer to const ck_bitmap, n: unsigned int) → bool
```

**Algorithm:**
1. word = atomic_load(bitmap->map + (n / CK_BITMAP_BLOCK))
2. Return (word & (1 << (n % CK_BITMAP_BLOCK))) != 0

**Complexity:** O(1)

### ck_bitmap_union

**Purpose:** dst |= src (bitwise OR)

**Signature:**
```
ck_bitmap_union(dst: Pointer to ck_bitmap, src: Pointer to const ck_bitmap) → void
```

**Algorithm:**
1. n_buckets = min(dst->n_bits, src->n_bits) / CK_BITMAP_BLOCK
2. FOR each bucket i in 0..n_buckets:
   a. Atomic OR: dst->map[i] |= atomic_load(src->map[i])

**Complexity:** O(min(dst_bits, src_bits) / 32)

**Note:** Not linearizable as a whole bitmap operation.

### ck_bitmap_intersection

**Purpose:** dst &= src (bitwise AND)

**Signature:**
```
ck_bitmap_intersection(dst: Pointer to ck_bitmap, src: Pointer to const ck_bitmap) → void
```

**Algorithm:**
1. n_intersect = min(dst->n_bits, src->n_bits) blocks
2. FOR each bucket i in 0..n_intersect:
   a. Atomic AND: dst->map[i] &= atomic_load(src->map[i])
3. FOR remaining buckets:
   a. Clear to zero

**Complexity:** O(dst_bits / 32)

### ck_bitmap_clear

**Purpose:** Set all bits to zero

**Signature:**
```
ck_bitmap_clear(bitmap: Pointer to ck_bitmap) → void
```

**Algorithm:**
1. FOR each bucket i:
   a. Atomic store: bitmap->map[i] = 0

**Complexity:** O(n_bits / 32)

### ck_bitmap_count

**Purpose:** Count number of set bits

**Signature:**
```
ck_bitmap_count(bitmap: Pointer to const ck_bitmap, limit: unsigned int) → unsigned int
```

**Algorithm:**
1. IF limit > n_bits: limit = n_bits
2. count = 0
3. FOR each full word:
   a. count += popcount(atomic_load(word))
4. Handle partial final word with mask
5. Return count

**Complexity:** O(limit / 32)

### ck_bitmap_empty / ck_bitmap_full

**Purpose:** Test if bitmap is all-zeros or all-ones

**Algorithm:**
1. Check each word for 0 (empty) or all-1s (full)
2. Handle partial final word

**Complexity:** O(n_bits / 32)

### Iterator Operations

**ck_bitmap_iterator_init:**
1. Set n_block = 0
2. Set n_limit = number of blocks
3. Cache first block

**ck_bitmap_next:**
1. IF cache == 0: advance to next non-zero block
2. IF no more blocks: return false
3. bit = n_block × BLOCK_SIZE + ctz(cache)
4. cache &= (cache - 1) // Clear lowest set bit
5. Return bit position

**Complexity:** O(1) amortized per set bit

## Concurrency

**Thread Safety:**
- Individual bit operations (set/reset/test/bts): Thread-safe [SPECIFIED]
- Bulk operations (union/intersection/clear): Not linearizable [SPECIFIED]
- Iterator: Not safe with concurrent modifications [OBSERVED]

**Memory Ordering:**
- set/reset: Atomic OR/AND operations
- test: Atomic load
- bts: Atomic bit-test-and-set

**Progress Guarantee:**
- All operations: wait-free [OBSERVED]

## Platform Considerations

- Requires CK_F_PR_LOAD_UINT, CK_F_PR_STORE_UINT, CK_F_PR_AND_UINT, CK_F_PR_OR_UINT, CK_F_CC_CTZ
- Block size is sizeof(unsigned int) × CHAR_BIT (typically 32 bits)
- CK_BITMAP_INSTANCE macro creates stack-allocated bitmap with known size
