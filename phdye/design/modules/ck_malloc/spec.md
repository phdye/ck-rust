# Module: ck_malloc — Specification

## Operations

This module defines no operations. It provides only a data structure definition.

## Data Structure Invariants

### struct ck_malloc

- `malloc` function pointer must not be NULL when structure is used [INFERRED]
- `realloc` function pointer must not be NULL when structure is used [INFERRED]
- `free` function pointer must not be NULL when structure is used [INFERRED]
- All function pointers must point to valid, callable functions [INFERRED]

## Module-Level Invariants

- The structure is a pure data definition with no associated state [OBSERVED]
- The structure does not own the functions it points to [OBSERVED]

## Safety Properties

**No Hidden State:** The module has no hidden state or side effects. [OBSERVED]

## Liveness Properties

No liveness properties identified. [OBSERVED]

## Behavioral Ambiguities

### realloc may_move semantics

**Observed Behavior:** The `may_move` parameter indicates whether realloc may return a different pointer.

**Intent:** SPECIFIED - Comments in code using this interface indicate may_move=false requires in-place resize

**Recommendation:** Reimplementers must honor may_move=false by either resizing in place or returning NULL.

### free defer semantics

**Observed Behavior:** The `defer` parameter indicates deallocation may be batched.

**Intent:** UNKNOWN - No explicit documentation of expected behavior

**Recommendation:** Reimplementers should support defer=true by allowing delayed deallocation (e.g., return immediately and free later). For defer=false, free immediately.

### size parameter in free

**Observed Behavior:** The `size` parameter is passed to free.

**Intent:** INFERRED - Enables sized-delete optimizations in allocators that support them

**Recommendation:** Reimplementers may ignore the size parameter if their allocator does not support sized deletes.

## Discrepancies

No discrepancies detected between sources.
