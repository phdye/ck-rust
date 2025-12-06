# Module: ck_malloc

## Overview

The ck_malloc module defines an abstract interface for memory allocation that allows CK data structures to use custom allocators. This enables integration with application-specific memory management strategies, memory pools, or specialized allocators for NUMA-aware allocation.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_stdbool.h | Internal | Boolean type for realloc/free parameters |
| sys/types.h | External | size_t type definition |

## Data Structures

### struct ck_malloc

**Description:** Function pointer table for memory allocation operations.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| malloc | Pointer to function(size_t) → Pointer | Platform pointer size | Allocate new memory block |
| realloc | Pointer to function(Pointer, size_t, size_t, Boolean) → Pointer | Platform pointer size | Resize existing memory block |
| free | Pointer to function(Pointer, size_t, Boolean) → void | Platform pointer size | Deallocate memory block |

**Invariants:**
- All function pointers must be non-NULL when passed to CK functions that require allocation [INFERRED]
- Functions must be thread-safe if used with concurrent data structures [INFERRED]

**Memory Layout:**
- Total Size: 3 × sizeof(void*) bytes (24 bytes on 64-bit, 12 bytes on 32-bit)
- Alignment: Platform pointer alignment

## Algorithms

This module defines no algorithms. It provides only a data structure for function pointers.

## Function Pointer Semantics

### malloc

**Purpose:** Allocate a new memory block

**Signature:**
```
malloc(size: size_t) → Pointer to void
```

**Expected Behavior:**
- Allocate at least `size` bytes of memory
- Return pointer to allocated memory on success
- Return NULL on failure
- Returned memory need not be initialized

### realloc

**Purpose:** Resize an existing memory block

**Signature:**
```
realloc(ptr: Pointer to void, old_size: size_t, new_size: size_t, may_move: Boolean) → Pointer to void
```

**Expected Behavior:**
- Resize the block pointed to by `ptr` from `old_size` to `new_size` bytes
- IF `may_move` is true: may relocate the block to a new address
- IF `may_move` is false: must resize in place or fail
- Preserve contents up to min(old_size, new_size)
- Return pointer to (possibly relocated) memory on success
- Return NULL on failure (original block remains valid)

**Note:** The `may_move` parameter enables resize-in-place semantics required for some concurrent algorithms.

### free

**Purpose:** Deallocate a memory block

**Signature:**
```
free(ptr: Pointer to void, size: size_t, defer: Boolean) → void
```

**Expected Behavior:**
- Deallocate the memory block at `ptr` of size `size`
- IF `defer` is true: deallocation may be deferred (e.g., for batch processing)
- IF `defer` is false: deallocate immediately

**Note:** The `size` parameter enables sized-delete optimizations. The `defer` parameter enables integration with deferred reclamation schemes.

## Concurrency

This module is not thread-safe. External synchronization required if the same ck_malloc instance is accessed by multiple threads.

However, the functions pointed to by the structure should be thread-safe if used with concurrent CK data structures.

## Platform Considerations

No platform-specific behavior identified.
