# Module: ck_cc — Specification

## Operations

### ck_cc_ffs

**Signature:**
```
ck_cc_ffs(v: 32-bit unsigned integer) → 32-bit signed integer
```

**Preconditions:**
- None [SPECIFIED]

**Postconditions:**
- IF v equals 0: returns 0 [SPECIFIED]
- IF v not equals 0: returns 1-indexed position of least significant set bit [SPECIFIED]
- Return value is in range [0, 32] [INFERRED]

**Invariants Preserved:**
- Pure function, no state modified [OBSERVED]

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| None | No error conditions | OBSERVED |

**Concurrency:**
- Thread Safety: Safe (pure function, no shared state) [OBSERVED]
- Memory Ordering: N/A [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_cc_ffsl

**Signature:**
```
ck_cc_ffsl(v: unsigned long) → 32-bit signed integer
```

**Preconditions:**
- None [SPECIFIED]

**Postconditions:**
- IF v equals 0: returns 0 [SPECIFIED]
- IF v not equals 0: returns 1-indexed position of least significant set bit [SPECIFIED]
- Return value is in range [0, sizeof(long)*8] [INFERRED]

**Invariants Preserved:**
- Pure function, no state modified [OBSERVED]

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| None | No error conditions | OBSERVED |

**Concurrency:**
- Thread Safety: Safe (pure function, no shared state) [OBSERVED]
- Memory Ordering: N/A [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_cc_ffsll

**Signature:**
```
ck_cc_ffsll(v: unsigned long long) → 32-bit signed integer
```

**Preconditions:**
- None [SPECIFIED]

**Postconditions:**
- IF v equals 0: returns 0 [SPECIFIED]
- IF v not equals 0: returns 1-indexed position of least significant set bit [SPECIFIED]
- Return value is in range [0, 64] [INFERRED]

**Invariants Preserved:**
- Pure function, no state modified [OBSERVED]

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| None | No error conditions | OBSERVED |

**Concurrency:**
- Thread Safety: Safe (pure function, no shared state) [OBSERVED]
- Memory Ordering: N/A [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_cc_ctz

**Signature:**
```
ck_cc_ctz(x: 32-bit unsigned integer) → 32-bit signed integer
```

**Preconditions:**
- None [OBSERVED]

**Postconditions:**
- IF x equals 0: returns 0 [OBSERVED]
- IF x not equals 0: returns count of trailing zero bits [SPECIFIED]
- Return value is in range [0, 31] for non-zero input [INFERRED]

**Invariants Preserved:**
- Pure function, no state modified [OBSERVED]

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| x equals 0 | Returns 0 (note: differs from some ctz definitions which have undefined behavior for 0) | OBSERVED |

**Concurrency:**
- Thread Safety: Safe (pure function, no shared state) [OBSERVED]
- Memory Ordering: N/A [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_cc_popcount

**Signature:**
```
ck_cc_popcount(x: 32-bit unsigned integer) → 32-bit signed integer
```

**Preconditions:**
- None [SPECIFIED]

**Postconditions:**
- Returns count of set (1) bits in x [SPECIFIED]
- Return value is in range [0, 32] [INFERRED]

**Invariants Preserved:**
- Pure function, no state modified [OBSERVED]

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| None | No error conditions | OBSERVED |

**Concurrency:**
- Thread Safety: Safe (pure function, no shared state) [OBSERVED]
- Memory Ordering: N/A [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

## Data Structure Invariants

No runtime data structures. All definitions are compile-time macros.

## Module-Level Invariants

- All functions are pure (no side effects) [OBSERVED]
- All functions are reentrant [OBSERVED]
- All macro expansions produce valid C99 [OBSERVED]
- GCC-specific code is guarded by `__GNUC__` or `__SUNPRO_C` [SPECIFIED]

## Safety Properties

**No Undefined Behavior:** All functions handle edge cases (zero input) explicitly rather than relying on undefined behavior. [OBSERVED]

## Liveness Properties

**Termination:** All operations terminate in bounded time proportional to input bit width. [OBSERVED]

## Behavioral Ambiguities

### ck_cc_ctz with zero input

**Observed Behavior:** Returns 0 when x equals 0

**Intent:** UNKNOWN - Standard ctz typically has undefined behavior for zero input

**Recommendation:** Reimplementers should preserve the zero-returns-zero behavior as defensive programming, documenting it as CK-specific.

## Discrepancies

No discrepancies detected between sources.
