# Module: ck_bytelock

## Overview

The ck_bytelock module implements the TLRW (Ticket Lock Read-Write) lock from Dice and Shavit's 2010 work. It uses per-slot byte indicators for readers, providing fast read-side acquisition when threads have assigned slots. Unslotted threads fall back to a shared counter.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Alignment attributes |
| ck_md | Internal | Cache line size |
| ck_pr | Internal | Atomic operations |
| ck_stdbool | External | Boolean type |
| ck_stddef | External | NULL definition |
| ck_limits | External | UINT_MAX |

## Data Structures

### struct ck_bytelock

**Description:** TLRW lock with per-slot reader bytes.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| owner | unsigned int | 4 bytes | Current writer slot (0 = none) |
| n_readers | unsigned int | 4 bytes | Unslotted reader count |
| readers | uint8_t[] | ~56 bytes | Per-slot reader flags |

**Memory Layout:**
- Total Size: ~64 bytes (one cache line)
- readers array sized to fill cache line
- Supports up to ~56 slotted readers

**Constants:**
- CK_BYTELOCK_UNSLOTTED = UINT_MAX

## Algorithms

### ck_bytelock_init

**Purpose:** Initialize lock

**Algorithm:**
1. Set owner = 0
2. Set n_readers = 0
3. Clear all reader slots

**Complexity:** O(cache line size)

### ck_bytelock_write_lock

**Purpose:** Acquire exclusive write access

**Algorithm:**
1. CAS owner from 0 to slot (spin on failure)
2. IF upgrading from read: clear own reader slot
3. Fence atomic-load
4. Wait for all slotted readers (scan readers[])
5. Wait for n_readers == 0 (unslotted readers)
6. Fence lock

**Complexity:** O(slots) for slotted readers + O(unslotted)

### ck_bytelock_write_unlock

**Purpose:** Release write lock

**Algorithm:**
1. Fence unlock
2. Store owner = 0

**Complexity:** O(1)

### ck_bytelock_read_lock

**Purpose:** Acquire shared read access

**Algorithm:**
1. IF caller is current writer: downgrade (set slot, clear owner)
2. IF slot > readers size: use n_readers counter with retry
3. ELSE: set readers[slot-1] = true, retry if writer arrived

**Complexity:** O(1) for slotted, may retry if contention

### ck_bytelock_read_unlock

**Purpose:** Release read lock

**Algorithm:**
1. Fence unlock
2. IF unslotted: decrement n_readers
3. ELSE: clear readers[slot-1]

**Complexity:** O(1)

## Concurrency

**Thread Safety:** Fully thread-safe.

**Progress Guarantee:**
- Read lock: Blocking (waits for writer)
- Write lock: Blocking (waits for all readers)

**Scalability:**
- Slotted readers: O(1) cache-local byte set
- Unslotted readers: Shared counter contention
- Writer: O(slots) scan

## Platform Considerations

- Requires 64-bit or 32-bit atomic loads (CK_F_PR_LOAD_64 or CK_F_PR_LOAD_32)
- Reader slots sized to cache line (~56 on 64-byte cache line)
- Slots are 1-indexed (slot 0 reserved for "no owner")
- Lock/unlock upgrade path supported

**Correctness Reference:** Dice, D. and Shavit, N. 2010. "TLRW: return of the read-write lock." SPAA '10.
