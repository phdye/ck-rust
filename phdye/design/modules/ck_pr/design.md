# Module: ck_pr

## Overview

The ck_pr module provides portable atomic operations and memory barriers for concurrent programming. It abstracts architecture-specific atomic instructions behind a uniform API, supporting multiple memory models (TSO, PSO, RMO) and generating appropriate fence operations for each platform. This is the foundational concurrency primitive layer upon which all other CK modules are built.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Compiler compatibility macros, inline hints |
| ck_md | External (generated) | Memory model selection (TSO/PSO/RMO), cache line size |
| ck_limits | External | Integer limits for overflow detection |
| ck_stdint | External | Fixed-width integer types |
| ck_stdbool | External | Boolean type |

## Data Structures

This module defines no runtime data structures. All operations are performed on user-provided memory locations.

## Algorithms

### Memory Barriers / Fences

Memory fences enforce ordering constraints between memory operations.

#### ck_pr_barrier

**Purpose:** Compiler barrier - prevent compiler reordering, no hardware fence

**Signature:**
```
ck_pr_barrier() → void
```

**Algorithm:**
1. Emit empty inline assembly with memory clobber

**Complexity:** O(1), Time: ~0 cycles (compile-time only)

#### ck_pr_fence_{type}

**Purpose:** Memory fences with various ordering guarantees

**Signature:**
```
ck_pr_fence_load() → void
ck_pr_fence_store() → void
ck_pr_fence_load_store() → void
ck_pr_fence_store_load() → void
ck_pr_fence_memory() → void
ck_pr_fence_acquire() → void
ck_pr_fence_release() → void
ck_pr_fence_acqrel() → void
ck_pr_fence_atomic() → void
ck_pr_fence_lock() → void
ck_pr_fence_unlock() → void
```

**Algorithm (memory-model dependent):**
- **TSO (x86):** Most fences become no-ops; only store_load requires mfence
- **PSO:** Store-related fences emit barriers; load fences are no-ops
- **RMO (ARM, POWER):** All fences emit full barriers via `__sync_synchronize()`

**Complexity:** O(1), Time: 0-100+ cycles depending on fence type and architecture

### Atomic Loads and Stores

#### ck_pr_load_{type}

**Purpose:** Atomically load a value with acquire-like semantics

**Signature:**
```
ck_pr_load_ptr(target: Pointer to Pointer) → Pointer
ck_pr_load_64(target: Pointer to 64-bit unsigned) → 64-bit unsigned
ck_pr_load_32(target: Pointer to 32-bit unsigned) → 32-bit unsigned
ck_pr_load_16(target: Pointer to 16-bit unsigned) → 16-bit unsigned
ck_pr_load_8(target: Pointer to 8-bit unsigned) → 8-bit unsigned
ck_pr_load_int(target: Pointer to int) → int
ck_pr_load_uint(target: Pointer to unsigned int) → unsigned int
ck_pr_load_char(target: Pointer to char) → char
ck_pr_load_double(target: Pointer to double) → double
```

**Algorithm:**
1. Emit compiler barrier
2. Perform volatile read from target
3. Emit compiler barrier
4. Return value

**Complexity:** O(1)

#### ck_pr_store_{type}

**Purpose:** Atomically store a value with release-like semantics

**Signature:**
```
ck_pr_store_ptr(target: Pointer to Pointer, value: Pointer) → void
ck_pr_store_64(target: Pointer to 64-bit unsigned, value: 64-bit unsigned) → void
... (similar for all types)
```

**Algorithm:**
1. Emit compiler barrier
2. Perform volatile write to target
3. Emit compiler barrier

**Complexity:** O(1)

### Compare-and-Swap (CAS)

#### ck_pr_cas_{type}

**Purpose:** Atomically compare and swap if equal

**Signature:**
```
ck_pr_cas_ptr(target: Pointer to Pointer, compare: Pointer, set: Pointer) → Boolean
ck_pr_cas_64(target: Pointer to 64-bit, compare: 64-bit, set: 64-bit) → Boolean
... (similar for all types)
```

**Algorithm:**
1. Atomically: IF *target equals compare THEN *target = set
2. Return true if swap occurred, false otherwise

**Implementation:** Uses `__sync_bool_compare_and_swap()` on GCC/Clang

**Complexity:** O(1), but may spin internally on some architectures

#### ck_pr_cas_{type}_value

**Purpose:** Compare and swap, returning old value

**Signature:**
```
ck_pr_cas_ptr_value(target: Pointer, compare: Pointer, set: Pointer, old: Pointer to Pointer) → Boolean
ck_pr_cas_64_value(target: Pointer, compare: 64-bit, set: 64-bit, old: Pointer to 64-bit) → Boolean
... (similar for all types)
```

**Algorithm:**
1. old_value = atomically swap if *target equals compare
2. Store old_value in *old
3. Return (old_value == compare)

**Implementation:** Uses `__sync_val_compare_and_swap()`

**Complexity:** O(1)

### Fetch-and-Add (FAA)

#### ck_pr_faa_{type}

**Purpose:** Atomically add to value, return previous value

**Signature:**
```
ck_pr_faa_ptr(target: Pointer to Pointer, delta: intptr_t) → Pointer
ck_pr_faa_64(target: Pointer to 64-bit, delta: 64-bit) → 64-bit
... (similar for all types)
```

**Algorithm:**
1. Atomically: old = *target; *target = old + delta
2. Return old

**Implementation:** Uses `__sync_fetch_and_add()` when available, else CAS loop

**Complexity:** O(1) with hardware support, O(∞) worst case with CAS loop under contention

### Fetch-and-Store (FAS)

#### ck_pr_fas_{type}

**Purpose:** Atomically exchange value, return previous value

**Signature:**
```
ck_pr_fas_ptr(target: Pointer to Pointer, update: Pointer) → Pointer
ck_pr_fas_64(target: Pointer to 64-bit, update: 64-bit) → 64-bit
... (similar for all types)
```

**Algorithm:**
1. Load current value
2. CAS loop until successful swap
3. Return previous value

**Complexity:** O(1) expected, O(∞) worst case under contention

### Binary Atomic Operations

#### ck_pr_{add,sub,and,or,xor}_{type}

**Purpose:** Atomically perform binary operation

**Signature:**
```
ck_pr_add_64(target: Pointer to 64-bit, delta: 64-bit) → void
ck_pr_sub_64(target: Pointer to 64-bit, delta: 64-bit) → void
ck_pr_and_64(target: Pointer to 64-bit, mask: 64-bit) → void
ck_pr_or_64(target: Pointer to 64-bit, mask: 64-bit) → void
ck_pr_xor_64(target: Pointer to 64-bit, mask: 64-bit) → void
... (similar for all types)
```

**Algorithm:**
1. Atomically: *target = *target OP value

**Implementation:** Uses `__sync_fetch_and_{add,sub,and,or,xor}()` when available

**Complexity:** O(1)

### Unary Atomic Operations

#### ck_pr_{inc,dec}_{type}

**Purpose:** Atomically increment/decrement

**Signature:**
```
ck_pr_inc_64(target: Pointer to 64-bit) → void
ck_pr_dec_64(target: Pointer to 64-bit) → void
... (similar for all types)
```

**Algorithm:**
1. ck_pr_add_{type}(target, 1) or ck_pr_sub_{type}(target, 1)

**Complexity:** O(1)

#### ck_pr_{inc,dec}_{type}_is_zero

**Purpose:** Atomically increment/decrement and check if result is zero

**Signature:**
```
ck_pr_inc_64_is_zero(target: Pointer to 64-bit) → Boolean
ck_pr_dec_64_is_zero(target: Pointer to 64-bit) → Boolean
```

**Algorithm:**
1. CAS loop: load old, compute new, compare-and-swap
2. Return (old == overflow_value) for inc, (old == 1) for dec

**Complexity:** O(1) expected

#### ck_pr_{not,neg}_{type}

**Purpose:** Atomically bitwise NOT or arithmetic negation

**Signature:**
```
ck_pr_not_64(target: Pointer to 64-bit) → void
ck_pr_neg_64(target: Pointer to 64-bit) → void
```

**Algorithm:**
1. CAS loop: load old, compute ~old or -old, compare-and-swap

**Complexity:** O(1) expected

### Bit Test Operations

#### ck_pr_{bts,btr,btc}_{type}

**Purpose:** Atomically test and set/reset/complement bit

**Signature:**
```
ck_pr_bts_64(target: Pointer to 64-bit, offset: unsigned int) → Boolean  // test and set
ck_pr_btr_64(target: Pointer to 64-bit, offset: unsigned int) → Boolean  // test and reset
ck_pr_btc_64(target: Pointer to 64-bit, offset: unsigned int) → Boolean  // test and complement
```

**Algorithm:**
1. CAS loop: load old, compute new with bit modified, compare-and-swap
2. Return previous state of bit at offset

**Complexity:** O(1) expected

### Utility Operations

#### ck_pr_stall

**Purpose:** CPU stall/pause hint for spin loops

**Signature:**
```
ck_pr_stall() → void
```

**Algorithm:**
1. Emit PAUSE instruction (x86) or equivalent
2. Reduces power consumption and improves performance in spin loops

**Complexity:** O(1), ~10-100 cycles

#### ck_pr_rfo

**Purpose:** Read-for-ownership hint (prefetch exclusive)

**Signature:**
```
ck_pr_rfo(target: Pointer to const void) → void
```

**Algorithm:**
1. Hint to CPU to bring cache line into exclusive state
2. Platform-specific or no-op

**Complexity:** O(1)

## Concurrency

**Thread Safety:** All operations are atomic and thread-safe by definition.

**Memory Model:** CK abstracts three memory models:
- **TSO (Total Store Order):** x86/x86_64 - stores are ordered, only store→load reordering
- **PSO (Partial Store Order):** SPARC - stores may reorder with each other
- **RMO (Relaxed Memory Order):** ARM, POWER - any reordering possible

**Memory Ordering:** Atomic RMW operations provide sequentially consistent ordering. Loads and stores provide at least acquire/release semantics when combined with appropriate fences.

## Platform Considerations

Specialized implementations exist for:
- x86 (32-bit)
- x86_64
- ARM (32-bit)
- AArch64
- PowerPC (32-bit)
- PowerPC64
- SPARC v9
- s390x
- RISC-V 64

Generic fallback uses GCC `__sync_*` builtins.
