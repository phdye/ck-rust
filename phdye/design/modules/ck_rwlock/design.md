# Module: ck_rwlock

## Overview

The ck_rwlock module implements a simple reader-writer spinlock. Multiple readers can hold the lock concurrently, but writers require exclusive access. The module also provides a recursive writer variant for cases where writers may need to re-acquire the lock.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_elide | Internal | Lock elision support |
| ck_pr | Internal | Atomic operations, fences |
| ck_stdbool | External | bool type |
| ck_stddef | External | NULL definition |

## Data Structures

### struct ck_rwlock

**Description:** Basic reader-writer lock.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| writer | unsigned int | 4 bytes | Writer flag (0 or 1) |
| n_readers | unsigned int | 4 bytes | Active reader count |

### struct ck_rwlock_recursive

**Description:** Recursive writer reader-writer lock.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| rw | struct ck_rwlock | 8 bytes | Embedded basic rwlock |
| wc | unsigned int | 4 bytes | Writer recursion count |

## Algorithms

### ck_rwlock_write_lock

**Purpose:** Acquire exclusive write access.

**Algorithm:**
1. FAS writer flag to 1, spin until successful
2. Fence atomic-load
3. Spin until n_readers == 0
4. Fence lock

**Complexity:** O(1) + wait for readers

### ck_rwlock_write_trylock

**Purpose:** Try to acquire write access without blocking.

**Algorithm:**
1. FAS writer flag to 1
2. IF was already 1: return false
3. Fence atomic-load
4. IF n_readers != 0: unlock, return false
5. Fence lock, return true

**Complexity:** O(1)

### ck_rwlock_write_unlock

**Purpose:** Release write access.

**Algorithm:**
1. Fence unlock
2. Store 0 to writer

**Complexity:** O(1)

### ck_rwlock_write_downgrade

**Purpose:** Convert write lock to read lock.

**Algorithm:**
1. Increment n_readers
2. Call write_unlock

**Complexity:** O(1)

### ck_rwlock_read_lock

**Purpose:** Acquire shared read access.

**Algorithm:**
1. Loop:
   a. Spin while writer != 0
   b. Increment n_readers
   c. Fence atomic-load
   d. IF writer == 0: break
   e. Decrement n_readers (writer appeared)
2. Fence load

**Complexity:** O(1) + wait for writer

### ck_rwlock_read_trylock

**Purpose:** Try to acquire read access without blocking.

**Algorithm:**
1. IF writer != 0: return false
2. Increment n_readers
3. Fence atomic-load
4. IF writer == 0: fence lock, return true
5. Decrement n_readers, return false

**Complexity:** O(1)

### ck_rwlock_read_unlock

**Purpose:** Release read access.

**Algorithm:**
1. Fence load-atomic
2. Decrement n_readers

**Complexity:** O(1)

### Recursive Writer Operations

**write_lock:** If already owner, increment wc. Else acquire and set wc=1.

**write_unlock:** Decrement wc. If wc==0, release lock.

**Note:** Owner identity passed as tid parameter.

## Concurrency

**Thread Safety:** Fully thread-safe.

**Progress Guarantee:**
- read_lock: Blocking (waits for writer)
- write_lock: Blocking (waits for readers and writers)
- Both trylocks: Lock-free

**Memory Ordering:**
- Read lock: Acquire semantics
- Write lock: Acquire semantics
- Both unlocks: Release semantics

## Platform Considerations

- Lock elision support for both read and write paths
- Writer flag checked before incrementing readers (avoid unnecessary increment)
- Fence placement ensures correct ordering
