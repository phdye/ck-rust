# Module: ck_swlock

## Overview

The ck_swlock module implements a reader-writer "swap" lock that packs the writer flag, latch bit, and reader count into a single 32-bit word. It provides both standard read/write operations and a latched write mode that ensures exclusive access without reader interference.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_elide | Internal | Lock elision support |
| ck_limits | External | UINT32_MAX |
| ck_pr | Internal | Atomic operations, fences |
| ck_stdbool | External | bool type |
| ck_stddef | External | NULL definition |

## Data Structures

### struct ck_swlock

**Description:** Packed reader-writer lock.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| value | uint32_t | 4 bytes | Packed state word |

**Bit Layout:**
- Bit 31: WRITER_BIT (writer intent/holding)
- Bit 30: LATCH_BIT (writer latched, no new readers)
- Bits 0-29: READER_MASK (reader count, max ~1 billion)

**Masks:**
- CK_SWLOCK_WRITER_BIT: 1 << 31
- CK_SWLOCK_LATCH_BIT: 1 << 30
- CK_SWLOCK_WRITER_MASK: WRITER_BIT | LATCH_BIT
- CK_SWLOCK_READER_MASK: ~WRITER_MASK

## Algorithms

### ck_swlock_write_lock

**Purpose:** Acquire write access (standard mode).

**Algorithm:**
1. OR in WRITER_BIT (publish intent)
2. Spin while READER_MASK portion != 0
3. Fence lock

**Property:** Readers may increment counter if they see WRITER_BIT but not LATCH_BIT.

### ck_swlock_write_latch

**Purpose:** Acquire write access with reader barrier.

**Algorithm:**
1. OR in WRITER_BIT (publish intent)
2. Spin until value == WRITER_BIT only
3. CAS to set WRITER_MASK (both bits)
4. Fence lock

**Property:** Once latched, new readers cannot enter.

### ck_swlock_write_unlock

**Purpose:** Release standard write lock.

**Algorithm:**
1. Fence unlock
2. AND with READER_MASK (clear writer bits)

**Complexity:** O(1)

### ck_swlock_write_unlatch

**Purpose:** Release latched write lock.

**Algorithm:**
1. Fence unlock
2. Store 0 (clear everything)

**Complexity:** O(1)

### ck_swlock_read_lock

**Purpose:** Acquire shared read access.

**Algorithm:**
1. Loop:
   a. Spin while WRITER_BIT set
   b. FAA increment reader count
   c. Check WRITER_MASK bits
   d. IF only WRITER_BIT (not latched): OK (writer will wait)
   e. IF WRITER_BIT set (maybe latching): decrement, retry
2. Fence lock

**Key Insight:** If latch bit not set, reader increment is visible and writer will wait for us.

### ck_swlock_read_trylock

**Purpose:** Try to acquire read access.

**Algorithm:**
1. Load value
2. IF WRITER_BIT set: return false
3. FAA increment
4. Check WRITER_MASK after increment
5. IF only WRITER_BIT: decrement (writer latching), return false
6. IF clear: fence, return true

### ck_swlock_read_unlock

**Purpose:** Release read access.

**Algorithm:**
1. Fence unlock
2. Decrement reader count

## Concurrency

**Thread Safety:** Fully thread-safe.

**Progress Guarantee:**
- read_lock: Blocking (waits for no writer)
- write_lock: Blocking (waits for readers)
- write_latch: Blocking (waits for readers)

**Memory Ordering:**
- All locks: Acquire semantics
- All unlocks: Release semantics

## Platform Considerations

- Single 32-bit word (smaller than ck_rwlock)
- Lock elision support for both read and write
- Latch mode for stronger writer guarantees
- Reader count limited to ~1 billion
