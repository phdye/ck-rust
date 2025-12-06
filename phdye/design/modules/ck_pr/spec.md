# Module: ck_pr — Specification

## Operations

### Fence Operations

#### ck_pr_barrier

**Signature:**
```
ck_pr_barrier() → void
```

**Preconditions:**
- None [SPECIFIED]

**Postconditions:**
- Compiler will not reorder memory operations across this barrier [SPECIFIED]
- No hardware fence is emitted [SPECIFIED]

**Invariants Preserved:**
- N/A

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| None | No error conditions | SPECIFIED |

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Compiler barrier only [SPECIFIED]
- Progress Guarantee: wait-free [SPECIFIED]

---

#### ck_pr_fence_{type}

**Signature:**
```
ck_pr_fence_memory() → void
ck_pr_fence_load() → void
ck_pr_fence_store() → void
ck_pr_fence_load_store() → void
ck_pr_fence_store_load() → void
ck_pr_fence_acquire() → void
ck_pr_fence_release() → void
ck_pr_fence_acqrel() → void
```

**Preconditions:**
- None [SPECIFIED]

**Postconditions:**
- Memory operations before fence are visible before operations after fence (according to fence type) [SPECIFIED]
- Fence strength depends on memory model (TSO/PSO/RMO) [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: As specified by fence type [SPECIFIED]
- Progress Guarantee: wait-free [SPECIFIED]

**Fence Semantics by Memory Model:**

| Fence Type | TSO (x86) | RMO (ARM) | Purpose |
|------------|-----------|-----------|---------|
| memory | mfence | dmb | Full barrier |
| load | no-op | dmb ld | Load-load ordering |
| store | no-op | dmb st | Store-store ordering |
| store_load | mfence | dmb | Prevents store-load reorder |
| acquire | no-op | dmb | Acquire semantics |
| release | no-op | dmb | Release semantics |
| acqrel | no-op | dmb | Acquire-release |

---

### Atomic Load Operations

#### ck_pr_load_{type}

**Signature:**
```
ck_pr_load_ptr(target: Pointer to const Pointer) → Pointer
ck_pr_load_64(target: Pointer to const 64-bit unsigned) → 64-bit unsigned
ck_pr_load_32(target: Pointer to const 32-bit unsigned) → 32-bit unsigned
ck_pr_load_16(target: Pointer to const 16-bit unsigned) → 16-bit unsigned
ck_pr_load_8(target: Pointer to const 8-bit unsigned) → 8-bit unsigned
```

**Preconditions:**
- target must point to naturally aligned memory [SPECIFIED]
- target must not be NULL [INFERRED]

**Postconditions:**
- Returns value at target at some point during the call [SPECIFIED]
- No tearing (partial read) occurs for aligned access [SPECIFIED]

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| Unaligned access | Undefined behavior (platform dependent) | SPECIFIED |
| NULL target | Undefined behavior | INFERRED |

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: At least relaxed; combine with fence for acquire [OBSERVED]
- Progress Guarantee: wait-free [SPECIFIED]

---

### Atomic Store Operations

#### ck_pr_store_{type}

**Signature:**
```
ck_pr_store_ptr(target: Pointer to Pointer, value: Pointer) → void
ck_pr_store_64(target: Pointer to 64-bit unsigned, value: 64-bit unsigned) → void
ck_pr_store_32(target: Pointer to 32-bit unsigned, value: 32-bit unsigned) → void
ck_pr_store_16(target: Pointer to 16-bit unsigned, value: 16-bit unsigned) → void
ck_pr_store_8(target: Pointer to 8-bit unsigned, value: 8-bit unsigned) → void
```

**Preconditions:**
- target must point to naturally aligned memory [SPECIFIED]
- target must not be NULL [INFERRED]

**Postconditions:**
- value is written to target atomically (no tearing) [SPECIFIED]

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| Unaligned access | Undefined behavior (platform dependent) | SPECIFIED |
| NULL target | Undefined behavior | INFERRED |

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: At least relaxed; combine with fence for release [OBSERVED]
- Progress Guarantee: wait-free [SPECIFIED]

---

### Compare-and-Swap Operations

#### ck_pr_cas_{type}

**Signature:**
```
ck_pr_cas_ptr(target: Pointer to Pointer, compare: Pointer, set: Pointer) → Boolean
ck_pr_cas_64(target: Pointer to 64-bit, compare: 64-bit, set: 64-bit) → Boolean
ck_pr_cas_32(target: Pointer to 32-bit, compare: 32-bit, set: 32-bit) → Boolean
```

**Preconditions:**
- target must point to naturally aligned memory [SPECIFIED]
- target must not be NULL [INFERRED]

**Postconditions:**
- IF *target == compare: *target = set, return true [SPECIFIED]
- IF *target != compare: return false, *target unchanged [SPECIFIED]
- Operation is atomic with respect to other atomic operations [SPECIFIED]

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| Spurious failure | May return false even when *target == compare (weak CAS) | OBSERVED |

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Sequentially consistent [SPECIFIED]
- Progress Guarantee: lock-free [SPECIFIED]

---

#### ck_pr_cas_{type}_value

**Signature:**
```
ck_pr_cas_ptr_value(target: Pointer, compare: Pointer, set: Pointer, old: Pointer to Pointer) → Boolean
ck_pr_cas_64_value(target: Pointer, compare: 64-bit, set: 64-bit, old: Pointer to 64-bit) → Boolean
```

**Preconditions:**
- target must point to naturally aligned memory [SPECIFIED]
- target and old must not be NULL [INFERRED]
- old must point to writable memory [INFERRED]

**Postconditions:**
- *old = previous value of *target [SPECIFIED]
- IF previous == compare: *target = set, return true [SPECIFIED]
- IF previous != compare: return false [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Sequentially consistent [SPECIFIED]
- Progress Guarantee: lock-free [SPECIFIED]

---

### Fetch-and-Add Operations

#### ck_pr_faa_{type}

**Signature:**
```
ck_pr_faa_64(target: Pointer to 64-bit, delta: 64-bit) → 64-bit
ck_pr_faa_32(target: Pointer to 32-bit, delta: 32-bit) → 32-bit
```

**Preconditions:**
- target must point to naturally aligned memory [SPECIFIED]
- target must not be NULL [INFERRED]

**Postconditions:**
- Returns previous value of *target [SPECIFIED]
- *target = previous + delta (with wraparound) [SPECIFIED]
- Operation is atomic [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Sequentially consistent [SPECIFIED]
- Progress Guarantee: lock-free (wait-free with hardware support) [SPECIFIED]

---

### Fetch-and-Store Operations

#### ck_pr_fas_{type}

**Signature:**
```
ck_pr_fas_64(target: Pointer to 64-bit, update: 64-bit) → 64-bit
ck_pr_fas_32(target: Pointer to 32-bit, update: 32-bit) → 32-bit
```

**Preconditions:**
- target must point to naturally aligned memory [SPECIFIED]
- target must not be NULL [INFERRED]

**Postconditions:**
- Returns previous value of *target [SPECIFIED]
- *target = update [SPECIFIED]
- Operation is atomic [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Sequentially consistent [SPECIFIED]
- Progress Guarantee: lock-free [SPECIFIED]

---

## Data Structure Invariants

No data structures defined.

## Module-Level Invariants

- All operations on aligned data are atomic (no tearing) [SPECIFIED]
- CAS operations are linearizable [SPECIFIED]
- Fence operations enforce the specified ordering [SPECIFIED]

## Safety Properties

**No Data Races:** Properly synchronized accesses using ck_pr operations do not exhibit undefined behavior due to data races. [SPECIFIED]

**Atomicity:** All read-modify-write operations complete atomically with respect to other ck_pr operations. [SPECIFIED]

## Liveness Properties

**Lock-Freedom:** CAS-based operations (cas, faa, fas) are lock-free; they guarantee system-wide progress. [SPECIFIED]

**Wait-Freedom:** Load, store, and fence operations are wait-free; individual operations complete in bounded time. [SPECIFIED]

## Behavioral Ambiguities

### Weak vs Strong CAS

**Observed Behavior:** CAS may fail spuriously on some platforms (ARM, POWER)

**Intent:** SPECIFIED - Weak CAS is acceptable; callers should use CAS in loops

**Recommendation:** Always use CAS in loops; do not assume single CAS will succeed when compare matches.

### Unaligned Access Behavior

**Observed Behavior:** Behavior of unaligned atomic access is platform-dependent

**Intent:** SPECIFIED - Only aligned accesses are supported

**Recommendation:** Ensure all atomic targets are naturally aligned. Use CK_CC_ALIGN for structures.

## Discrepancies

No discrepancies detected between sources.
