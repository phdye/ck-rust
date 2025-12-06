# Module: ck_pflock

## Overview

The ck_pflock module implements phase-fair reader-writer locks based on Brandenburg and Anderson's 2010 work. Phase-fair locks provide fairness by alternating between read phases and write phases, preventing starvation of either readers or writers.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Inline hints |
| ck_pr | Internal | Atomic operations |

## Data Structures

### struct ck_pflock

**Description:** Phase-fair reader-writer lock.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| rin | uint32_t | 4 bytes | Reader in counter + writer bits |
| rout | uint32_t | 4 bytes | Reader out counter |
| win | uint32_t | 4 bytes | Writer in ticket |
| wout | uint32_t | 4 bytes | Writer out ticket |

**Memory Layout:**
- Total Size: 16 bytes
- Alignment: 4-byte aligned

**Invariants:**
- rout <= rin (reader counter never exceeds in) [OBSERVED]
- wout <= win (writer ticket ordering) [OBSERVED]

## Algorithms

### ck_pflock_init

**Purpose:** Initialize lock to unlocked state

**Algorithm:**
1. Set rin = rout = win = wout = 0

**Complexity:** O(1)

### ck_pflock_write_lock

**Purpose:** Acquire write lock

**Algorithm:**
1. Atomically increment win (ticket = FAA(win, 1))
2. Wait until wout == ticket (writer's turn)
3. Atomically update rin with phase bit and presence flag
4. Wait until rout matches reader ticket (all readers drained)
5. Fence lock

**Complexity:** O(readers) for draining, O(writers) for ticket wait

### ck_pflock_write_unlock

**Purpose:** Release write lock

**Algorithm:**
1. Fence unlock
2. Clear writer bits from rin (migrate to read phase)
3. Atomically increment wout (allow next writer)

**Complexity:** O(1)

### ck_pflock_read_lock

**Purpose:** Acquire read lock

**Algorithm:**
1. Atomically increment rin by RINC, capture writer bits
2. IF no writer present: done
3. ELSE: wait for writer bits to change (write phase complete)
4. Fence lock

**Complexity:** O(1) if no writer, O(write duration) otherwise

### ck_pflock_read_unlock

**Purpose:** Release read lock

**Algorithm:**
1. Fence unlock
2. Atomically increment rout by RINC

**Complexity:** O(1)

## Concurrency

**Thread Safety:** Fully thread-safe for multiple readers and writers.

**Progress Guarantee:**
- Writers: Blocking (must wait for readers and prior writers)
- Readers: Blocking (must wait during write phase)

**Fairness:**
- Phase-fair: Readers present at write start complete before writer enters
- Readers arriving during write wait for next read phase
- Prevents starvation of both readers and writers

## Platform Considerations

- Requires 32-bit atomic FAA
- Reader increment uses upper bits, writer bits use lower 2 bits
- Phase ID bit toggles between read phases

**Correctness Reference:** Brandenburg, B. and Anderson, J. 2010. "Spin-Based Reader-Writer Synchronization for Multiprocessor Real-Time Systems"
