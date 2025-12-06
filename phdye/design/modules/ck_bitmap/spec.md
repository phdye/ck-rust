# Module: ck_bitmap — Specification

## Operations

### ck_bitmap_size

**Signature:**
```
ck_bitmap_size(n_bits: unsigned int) → unsigned int
```

**Preconditions:**
- None

**Postconditions:**
- Returns number of bytes needed to store bitmap with n_bits [SPECIFIED]

**Concurrency:**
- Thread Safety: Pure function [SPECIFIED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_bitmap_init

**Signature:**
```
ck_bitmap_init(bitmap: Pointer to ck_bitmap, n_bits: unsigned int, set: bool) → void
```

**Preconditions:**
- bitmap points to memory of at least ck_bitmap_size(n_bits) bytes [INFERRED]

**Postconditions:**
- bitmap->n_bits = n_bits [SPECIFIED]
- IF set: all bits 0..n_bits-1 are set to 1 [SPECIFIED]
- IF !set: all bits are set to 0 [SPECIFIED]
- Trailing bits (if any) are 0 [OBSERVED]

**Concurrency:**
- Thread Safety: Not safe during concurrent access [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_bitmap_set

**Signature:**
```
ck_bitmap_set(bitmap: Pointer to ck_bitmap, n: unsigned int) → void
```

**Preconditions:**
- n < bitmap->n_bits [INFERRED]

**Postconditions:**
- Bit n is set to 1 [SPECIFIED]
- Other bits unchanged [SPECIFIED]

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| n >= n_bits | Out of bounds access | INFERRED |

**Concurrency:**
- Thread Safety: Safe with concurrent set/reset/test [SPECIFIED]
- Memory Ordering: Atomic OR [OBSERVED]
- Progress Guarantee: wait-free [SPECIFIED]

---

### ck_bitmap_bts

**Signature:**
```
ck_bitmap_bts(bitmap: Pointer to ck_bitmap, n: unsigned int) → bool
```

**Preconditions:**
- n < bitmap->n_bits [INFERRED]

**Postconditions:**
- Returns previous value of bit n [SPECIFIED]
- Bit n is now set to 1 [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Atomic bit-test-and-set [OBSERVED]
- Progress Guarantee: wait-free [SPECIFIED]

---

### ck_bitmap_reset

**Signature:**
```
ck_bitmap_reset(bitmap: Pointer to ck_bitmap, n: unsigned int) → void
```

**Preconditions:**
- n < bitmap->n_bits [INFERRED]

**Postconditions:**
- Bit n is set to 0 [SPECIFIED]
- Other bits unchanged [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe with concurrent set/reset/test [SPECIFIED]
- Memory Ordering: Atomic AND [OBSERVED]
- Progress Guarantee: wait-free [SPECIFIED]

---

### ck_bitmap_test

**Signature:**
```
ck_bitmap_test(bitmap: Pointer to const ck_bitmap, n: unsigned int) → bool
```

**Preconditions:**
- n < bitmap->n_bits [INFERRED]

**Postconditions:**
- Returns true if bit n is set, false otherwise [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Atomic load [OBSERVED]
- Progress Guarantee: wait-free [SPECIFIED]

---

### ck_bitmap_union

**Signature:**
```
ck_bitmap_union(dst: Pointer to ck_bitmap, src: Pointer to const ck_bitmap) → void
```

**Preconditions:**
- dst and src point to valid bitmaps [INFERRED]

**Postconditions:**
- For each bit i in range 0..min(dst->n_bits, src->n_bits)-1:
  - dst[i] = dst[i] OR src[i] [SPECIFIED]

**Concurrency:**
- Thread Safety: Individual word operations are atomic [OBSERVED]
- Memory Ordering: Not linearizable as whole operation [SPECIFIED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_bitmap_intersection

**Signature:**
```
ck_bitmap_intersection(dst: Pointer to ck_bitmap, src: Pointer to const ck_bitmap) → void
```

**Preconditions:**
- dst and src point to valid bitmaps [INFERRED]

**Postconditions:**
- For each bit i in range 0..min(dst->n_bits, src->n_bits)-1:
  - dst[i] = dst[i] AND src[i] [SPECIFIED]
- For bits beyond src->n_bits: cleared to 0 [SPECIFIED]

**Concurrency:**
- Thread Safety: Individual word operations are atomic [OBSERVED]
- Memory Ordering: Not linearizable as whole operation [SPECIFIED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_bitmap_intersection_negate

**Signature:**
```
ck_bitmap_intersection_negate(dst: Pointer to ck_bitmap, src: Pointer to const ck_bitmap) → void
```

**Preconditions:**
- dst and src point to valid bitmaps [INFERRED]

**Postconditions:**
- For each bit i in intersection range:
  - dst[i] = dst[i] AND NOT src[i] [SPECIFIED]
- Bits beyond src->n_bits unchanged [SPECIFIED]

**Concurrency:**
- Thread Safety: Individual word operations are atomic [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_bitmap_clear

**Signature:**
```
ck_bitmap_clear(bitmap: Pointer to ck_bitmap) → void
```

**Preconditions:**
- bitmap points to valid bitmap [INFERRED]

**Postconditions:**
- All bits set to 0 [SPECIFIED]

**Concurrency:**
- Thread Safety: Individual word stores are atomic [OBSERVED]
- Memory Ordering: Not linearizable [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_bitmap_empty

**Signature:**
```
ck_bitmap_empty(bitmap: Pointer to const ck_bitmap, limit: unsigned int) → bool
```

**Preconditions:**
- bitmap points to valid bitmap [INFERRED]

**Postconditions:**
- Returns true if all bits 0..min(limit, n_bits)-1 are 0 [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe (atomic loads) [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_bitmap_full

**Signature:**
```
ck_bitmap_full(bitmap: Pointer to const ck_bitmap, limit: unsigned int) → bool
```

**Preconditions:**
- bitmap points to valid bitmap [INFERRED]

**Postconditions:**
- Returns true if all bits 0..min(limit, n_bits)-1 are 1 [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe (atomic loads) [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_bitmap_count

**Signature:**
```
ck_bitmap_count(bitmap: Pointer to const ck_bitmap, limit: unsigned int) → unsigned int
```

**Preconditions:**
- bitmap points to valid bitmap [INFERRED]

**Postconditions:**
- Returns number of set bits in range 0..min(limit, n_bits)-1 [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe (atomic loads) [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_bitmap_iterator_init

**Signature:**
```
ck_bitmap_iterator_init(iter: Pointer to ck_bitmap_iterator, bitmap: Pointer to const ck_bitmap) → void
```

**Preconditions:**
- iter and bitmap point to valid structures [INFERRED]

**Postconditions:**
- Iterator positioned before first set bit [SPECIFIED]

**Concurrency:**
- Thread Safety: Not safe with concurrent modifications [OBSERVED]

---

### ck_bitmap_next

**Signature:**
```
ck_bitmap_next(bitmap: Pointer to const ck_bitmap, iter: Pointer to ck_bitmap_iterator, bit: Pointer to unsigned int) → bool
```

**Preconditions:**
- Iterator previously initialized [INFERRED]

**Postconditions:**
- IF more set bits exist: *bit = next set bit position, return true [SPECIFIED]
- IF no more set bits: return false [SPECIFIED]

**Concurrency:**
- Thread Safety: Not safe with concurrent modifications [OBSERVED]
- Note: May see inconsistent state if bitmap modified during iteration

---

## Data Structure Invariants

### ck_bitmap

- n_bits is immutable after init [OBSERVED]
- map contains ceil(n_bits / CK_BITMAP_BLOCK) words [SPECIFIED]
- Trailing bits (beyond n_bits) are 0 [OBSERVED]

## Module-Level Invariants

- All bit operations use atomic primitives [OBSERVED]
- Bulk operations are not linearizable [SPECIFIED]

## Safety Properties

**Atomic Bit Access:** Individual bit operations are atomic with respect to each other. [SPECIFIED]

**No Torn Reads:** ck_bitmap_test always returns a valid bit value (0 or 1). [OBSERVED]

## Liveness Properties

**Wait-Freedom:** All operations complete in bounded time. [OBSERVED]

## Behavioral Ambiguities

### Out of bounds access

**Observed Behavior:** Accessing bit n >= n_bits is undefined

**Intent:** INFERRED - Bounds checking is caller's responsibility

**Recommendation:** Document that n must be < n_bits. Consider debug assertion.

### Bulk operation atomicity

**Observed Behavior:** Union/intersection process one word at a time

**Intent:** SPECIFIED - Not linearizable as whole bitmap

**Recommendation:** Document that concurrent bulk operations may interleave. Use external synchronization for atomic bulk updates.

## Discrepancies

No discrepancies detected between sources.
