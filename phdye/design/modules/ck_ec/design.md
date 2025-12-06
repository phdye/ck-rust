# Module: ck_ec

## Overview

The ck_ec module implements event counts for integrating OS-level blocking (futexes) with lock-free protocols. Event counts allow waiters to block conditionally when a counter value hasn't changed, enabling efficient producer-consumer patterns without busy-waiting. The module provides 32-bit and 64-bit variants with single-producer and multiple-producer modes.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Inline hints, compiler intrinsics |
| ck_pr | Internal | Atomic operations, fences |
| ck_stdbool | External | bool type |
| ck_stdint | External | uint32_t, uint64_t |
| ck_stddef | External | NULL definition |
| sys/time.h | External | struct timespec |

## Data Structures

### struct ck_ec_ops

**Description:** Platform-specific operations for time, waiting, and waking.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| gettime | function pointer | Platform pointer | Get current monotonic time |
| wait32 | function pointer | Platform pointer | Wait on 32-bit address |
| wait64 | function pointer | Platform pointer | Wait on 64-bit address |
| wake32 | function pointer | Platform pointer | Wake threads on 32-bit address |
| wake64 | function pointer | Platform pointer | Wake threads on 64-bit address |
| busy_loop_iter | uint32_t | 4 bytes | Spin iterations before blocking (default: 100) |
| initial_wait_ns | uint32_t | 4 bytes | Initial backoff delay (default: 2ms) |
| wait_scale_factor | uint32_t | 4 bytes | Exponential scale (default: 8x) |
| wait_shift_count | uint32_t | 4 bytes | Right shift for backoff scaling |

### struct ck_ec_mode

**Description:** Encapsulates ops and producer mode for API calls.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| ops | const struct ck_ec_ops* | Platform pointer | Platform operations |
| single_producer | bool | 1 byte | True for single-producer optimization |

### struct ck_ec32

**Description:** 32-bit event count (31-bit counter + 1 flag bit).

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| counter | uint32_t | 4 bytes | Value in bits 0:30, flag in bit 31 |

**Layout:** Flag bit is the sign bit (bit 31). Counter value in lower 31 bits.

### struct ck_ec64

**Description:** 64-bit event count (63-bit counter + 1 flag bit).

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| counter | uint64_t | 8 bytes | Flag in bit 0, value in bits 1:63 |

**Layout:** Flag bit is LSB (bit 0). Counter value in upper 63 bits (shifted left by 1).

### struct ck_ec_wait_state

**Description:** State passed to wait predicates.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| start | struct timespec | 16 bytes | Time when wait began |
| now | struct timespec | 16 bytes | Current time |
| ops | const struct ck_ec_ops* | Platform pointer | Operations table |
| data | void* | Platform pointer | User-provided opaque data |

## Algorithms

### ck_ec_init

**Purpose:** Initialize event count to a value.

**Algorithm:**
1. Clear flag bit
2. Store value (masked for 32-bit, shifted for 64-bit)

**Complexity:** O(1)

### ck_ec_value

**Purpose:** Read current counter value with acquire semantics.

**Algorithm:**
1. Atomic load of counter word
2. Extract value (mask flag for 32-bit, shift for 64-bit)
3. Acquire fence

**Complexity:** O(1)

### ck_ec_inc / ck_ec_add (Multiple Producer)

**Purpose:** Increment counter and wake waiters.

**Algorithm:**
1. Store-atomic fence
2. Fetch-and-add delta to counter
3. IF old value had flag set: call wake

**Complexity:** O(1) + wake overhead

### ck_ec_inc / ck_ec_add (Single Producer, x86)

**Purpose:** Optimized increment using non-atomic RMW.

**Algorithm:**
1. Store fence
2. Non-atomic increment via `inc mem` or `xadd`
3. Check condition flags for overflow/flag state
4. IF flag was set: call wake

**Optimization:** Exploits x86-TSO guarantee that reads see most recent local store or memory value. Non-atomic RMW is a single instruction, immune to preemption splitting.

**Complexity:** O(1) + wake overhead

### ck_ec_wait

**Purpose:** Block until counter changes or deadline expires.

**Algorithm:**
1. Read counter value
2. IF value != old_value: return 0 (success)
3. Enter slow path:
   a. Spin loop for busy_loop_iter iterations
   b. Set flag bit via CAS
   c. Re-check value
   d. Exponential backoff with futex_wait:
      - Initial wait = initial_wait_ns
      - Scale: wait = (wait * scale_factor) >> shift_count
      - After 1 second: infinite deadline (flag guaranteed visible)
4. Return 0 on change, -1 on timeout

**Complexity:** O(busy_loop_iter) best case, unbounded for blocking

### ck_ec_wait_pred

**Purpose:** Block with user-provided early-exit predicate.

**Algorithm:**
1. Same as ck_ec_wait, but:
2. Before each futex_wait, call pred(state, &iteration_deadline)
3. IF pred returns non-zero: return that value
4. pred may modify iteration_deadline

**Use Case:** Wait on multiple conditions, optimistic checking.

## Concurrency

**Thread Safety:** Fully thread-safe.

**Progress Guarantee:**
- inc/add: Wait-free (single producer), Lock-free (multiple producer)
- value: Wait-free
- wait: Blocking (depends on OS futex)

**Memory Ordering:**
- value: Acquire semantics
- inc/add: Release semantics (via fence before update)
- wait: Acquire semantics on return

## Platform Considerations

**32-bit variant:** Always available.

**64-bit variant:** Requires CK_F_PR_FAA_64 (64-bit fetch-and-add support).

**Single-producer optimization:** Requires GCC extended inline assembly on x86/x86_64. Uses non-atomic RMW instructions (`inc mem`, `xadd`) which are safe under x86-TSO.

**Futex compatibility:** For 64-bit counters on systems with 32-bit futexes, wait64 should use the low 32 bits of the counter address. Works because flag is LSB.

**C11 generics:** Type-generic macros (ck_ec_init, etc.) available with C11 or later.

**Correctness Reference:** Implementation notes in header explain x86-TSO dependency for single-producer mode.
