# Module: ck_tflock

## Overview

The ck_tflock module implements task-fair reader-writer locks based on Mellor-Crummey and Scott's 1991 work. Task-fair locks process requests in strict FIFO order regardless of whether they are reads or writes, providing maximum fairness at the cost of reduced reader parallelism.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Inline hints |
| ck_pr | Internal | Atomic operations |

## Data Structures

### struct ck_tflock_ticket

**Description:** Task-fair reader-writer lock using ticket approach.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| request | uint32_t | 4 bytes | Next ticket to issue (read upper 16, write lower 16) |
| completion | uint32_t | 4 bytes | Last completed ticket |

**Memory Layout:**
- Total Size: 8 bytes
- Alignment: 4-byte aligned

**Bit Layout:**
- Upper 16 bits: Reader ticket counter
- Lower 16 bits: Writer ticket counter

## Algorithms

### ck_tflock_ticket_init

**Purpose:** Initialize lock to unlocked state

**Algorithm:**
1. Set request = completion = 0

**Complexity:** O(1)

### ck_tflock_ticket_write_lock

**Purpose:** Acquire write lock

**Algorithm:**
1. Atomically fetch-clear-add: get ticket from request, clear overflow bit, add 1
2. Wait until completion == ticket

**Complexity:** O(pending requests)

### ck_tflock_ticket_write_unlock

**Purpose:** Release write lock

**Algorithm:**
1. Fence unlock
2. Atomically increment completion (writer portion)

**Complexity:** O(1)

### ck_tflock_ticket_read_lock

**Purpose:** Acquire read lock

**Algorithm:**
1. Atomically fetch-clear-add on request (reader bits)
2. Extract writer ticket from result
3. Wait until completion's writer portion matches

**Complexity:** O(pending writers)

### ck_tflock_ticket_read_unlock

**Purpose:** Release read lock

**Algorithm:**
1. Fence unlock
2. Atomically increment completion (reader portion)

**Complexity:** O(1)

### ck_tflock_ticket_fca_32 (Helper)

**Purpose:** Fetch-clear-add: atomically fetch, clear overflow bit, add delta

**Algorithm:**
1. CAS loop: load, compute (value & ~mask) + delta, CAS

**Complexity:** O(1) expected

## Concurrency

**Thread Safety:** Fully thread-safe for multiple readers and writers.

**Progress Guarantee:**
- All operations: Blocking (wait for prior requests)

**Fairness:**
- Strict FIFO: Requests processed in arrival order
- A writer after N readers waits for all N readers
- Readers after a writer wait for the writer

## Platform Considerations

- Upper/lower 16 bits limit to 32K concurrent readers or writers
- Overflow handling via clear masks (CK_TFLOCK_TICKET_WC_TOPMSK, RC_TOPMSK)

**Correctness Reference:** Mellor-Crummey, J. and Scott, M.L. 1991. "Scalable reader-writer synchronization for shared-memory multiprocessors"
