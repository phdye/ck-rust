# Module: ck_backoff

## Overview

The ck_backoff module provides exponential backoff functionality for spin loops. Exponential backoff reduces contention on shared resources by introducing progressively longer delays between retry attempts, improving overall system throughput under contention.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Inline function hints |
| ck_pr | Internal | ck_pr_barrier() for delay loop |

## Data Structures

### ck_backoff_t

**Description:** Backoff state counter.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| (value) | 32-bit unsigned integer | 4 bytes | Current backoff iteration count |

**Invariants:**
- Value is always a power of 2 (after initialization) [OBSERVED]
- Value is bounded by CK_BACKOFF_CEILING [SPECIFIED]

**Memory Layout:**
- Total Size: 4 bytes
- Alignment: 4 bytes

## Algorithms

### ck_backoff_eb

**Purpose:** Execute exponential backoff delay and update state

**Signature:**
```
ck_backoff_eb(c: Pointer to 32-bit unsigned integer) → void
```

**Algorithm:**
1. Read current ceiling from *c
2. Execute `ceiling` iterations of ck_pr_barrier()
3. IF ceiling < CK_BACKOFF_CEILING:
   - *c = ceiling * 2 (left shift by 1)
4. ELSE:
   - *c = ceiling (unchanged, at maximum)
5. Return

**Complexity:**
- Time: O(n) where n is current backoff value
- Space: O(1)

**Correctness Reference:** Implementation-derived; standard exponential backoff pattern

## Concurrency

This module is not thread-safe. Each thread should have its own ck_backoff_t variable. The backoff state must not be shared between threads.

## Platform Considerations

No platform-specific behavior identified. Uses ck_pr_barrier() which is portable.

## Constants

| Constant | Default Value | Description |
|----------|---------------|-------------|
| CK_BACKOFF_CEILING | (1 << 20) - 1 = 1048575 | Maximum backoff iterations |
| CK_BACKOFF_INITIALIZER | 1 << 9 = 512 | Initial backoff value |
