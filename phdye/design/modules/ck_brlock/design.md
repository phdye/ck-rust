# Module: ck_brlock

## Overview

The ck_brlock module implements "big reader" locks (also called distributed reader-writer locks). Each reader thread has its own counter, providing cache-local contention-free read lock acquisition. Write lock acquisition requires traversing all reader counters, making it O(n) in the number of readers.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_pr | Internal | Atomic operations |
| ck_stdbool | External | Boolean type |
| ck_stddef | External | NULL definition |

## Data Structures

### struct ck_brlock

**Description:** Global lock state.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| readers | Pointer to ck_brlock_reader | Platform pointer | Head of reader list |
| writer | unsigned int | 4 bytes | Writer flag (0 or 1) |

### struct ck_brlock_reader

**Description:** Per-reader state (one per thread).

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| n_readers | unsigned int | 4 bytes | Reader recursion count |
| previous | Pointer to ck_brlock_reader | Platform pointer | Previous in list |
| next | Pointer to ck_brlock_reader | Platform pointer | Next in list |

## Algorithms

### ck_brlock_init

**Purpose:** Initialize lock

**Algorithm:**
1. Set readers = NULL
2. Set writer = false

**Complexity:** O(1)

### ck_brlock_read_register

**Purpose:** Register reader thread with lock

**Algorithm:**
1. Set reader->n_readers = 0
2. Acquire write lock
3. Insert reader at head of list
4. Release write lock

**Complexity:** O(n) readers

### ck_brlock_read_unregister

**Purpose:** Unregister reader thread

**Algorithm:**
1. Acquire write lock
2. Remove reader from list
3. Release write lock

**Complexity:** O(n) readers

### ck_brlock_write_lock

**Purpose:** Acquire exclusive write access

**Algorithm:**
1. Spin until writer flag acquired (FAS)
2. Fence atomic-load
3. Traverse reader list, wait for each n_readers == 0
4. Fence lock

**Complexity:** O(n) readers

### ck_brlock_write_unlock

**Purpose:** Release write lock

**Algorithm:**
1. Fence unlock
2. Store writer = false

**Complexity:** O(1)

### ck_brlock_read_lock

**Purpose:** Acquire shared read access

**Algorithm:**
1. IF already holding (n_readers >= 1): increment and return
2. Loop:
   a. Wait while writer flag is set
   b. Set n_readers = 1 (with fence)
   c. IF writer still 0: break
   d. Set n_readers = 0, retry
3. Fence lock

**Complexity:** O(1) if no writer

### ck_brlock_read_unlock

**Purpose:** Release read lock

**Algorithm:**
1. Fence unlock
2. Decrement n_readers

**Complexity:** O(1)

## Concurrency

**Thread Safety:** Fully thread-safe. Readers must register before use.

**Progress Guarantee:**
- Read lock: Blocking (waits for writer)
- Write lock: Blocking (waits for all readers)

**Scalability:**
- Read path: O(1), cache-local (no shared writes)
- Write path: O(n) traversal of reader list

## Platform Considerations

- Per-reader structure should be on same cache line as reader's data if possible
- Readers list protected by writer lock
- Platform-specific fence selection (x86 uses FAS, others use store + fence)
- Originally implemented in Linux kernel by Ingo Molnar and David S. Miller (~2000)
