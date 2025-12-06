# Module: ck_fifo

## Overview

The ck_fifo module provides FIFO (First-In-First-Out) queue implementations for concurrent programming. It offers two variants: SPSC (Single-Producer-Single-Consumer) and MPMC (Multi-Producer-Multi-Consumer). Both use linked-list structures with stub nodes for simplicity. The MPMC variant uses the Michael-Scott lock-free queue algorithm with double-wide CAS for ABA prevention.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Compiler hints, alignment |
| ck_md | Internal | Cacheline size |
| ck_pr | Internal | Atomic operations |
| ck_spinlock | Internal | Optional external locking for SPSC |
| ck_stddef | External | NULL definition |

## Data Structures

### struct ck_fifo_spsc_entry

**Description:** Entry node for SPSC queue.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| value | void* | 8 bytes | User data pointer |
| next | pointer | 8 bytes | Next entry in queue |

### struct ck_fifo_spsc

**Description:** Single-producer-single-consumer queue.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| m_head | ck_spinlock_t | varies | Optional head lock |
| head | pointer | 8 bytes | Dequeue position |
| pad | char[] | varies | Cacheline padding |
| m_tail | ck_spinlock_t | varies | Optional tail lock |
| tail | pointer | 8 bytes | Enqueue position |
| head_snapshot | pointer | 8 bytes | Snapshot for recycle |
| garbage | pointer | 8 bytes | Recyclable node pointer |

**Layout:** Head and tail on separate cachelines to avoid false sharing.

### struct ck_fifo_mpmc_pointer

**Description:** Tagged pointer for ABA prevention.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| pointer | pointer | 8 bytes | Entry pointer |
| generation | char* | 8 bytes | Generation counter |

**Alignment:** 16 bytes (for double-wide CAS)

### struct ck_fifo_mpmc_entry

**Description:** Entry node for MPMC queue.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| value | void* | 8 bytes | User data pointer |
| next | ck_fifo_mpmc_pointer | 16 bytes | Tagged next pointer |

### struct ck_fifo_mpmc

**Description:** Multi-producer-multi-consumer queue.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| head | ck_fifo_mpmc_pointer | 16 bytes | Dequeue position |
| pad | char[] | varies | Cacheline padding |
| tail | ck_fifo_mpmc_pointer | 16 bytes | Enqueue position |

## Algorithms

### ck_fifo_spsc_enqueue

**Purpose:** Add element to SPSC queue.

**Algorithm:**
1. Set entry->value = value
2. Set entry->next = NULL
3. fence_store (ensure entry visible before link)
4. Store tail->next = entry
5. Update tail = entry

**Complexity:** O(1)

**Property:** Wait-free for single producer.

### ck_fifo_spsc_dequeue

**Purpose:** Remove element from SPSC queue.

**Algorithm:**
1. Load head->next atomically
2. If NULL, return false (empty)
3. Store value from entry
4. fence_store
5. Update head = entry
6. Return true

**Complexity:** O(1)

**Property:** Wait-free for single consumer.

### ck_fifo_spsc_recycle

**Purpose:** Reclaim dequeued nodes for reuse.

**Algorithm:**
1. Compare head_snapshot with garbage
2. If equal, update snapshot from head
3. If still equal, no nodes to recycle
4. Otherwise, advance garbage, return old node

**Note:** Based on Dmitriy Vyukov's technique.

### ck_fifo_mpmc_enqueue

**Purpose:** Lock-free enqueue for MPMC queue.

**Algorithm:**
1. Prepare entry (value, next = NULL)
2. fence_store_atomic
3. Loop:
   a. Load tail with generation
   b. Load tail->next
   c. If tail outdated, retry
   d. If next != NULL, help advance tail, retry
   e. CAS tail->next from NULL to entry
   f. If success, break
4. fence_atomic
5. CAS to advance tail (may fail, OK)

**Complexity:** O(1) amortized

**Property:** Lock-free (Michael-Scott algorithm).

### ck_fifo_mpmc_dequeue

**Purpose:** Lock-free dequeue for MPMC queue.

**Algorithm:**
1. Loop:
   a. Load head, tail, head->next with generations
   b. If head == tail and next == NULL, empty
   c. If head == tail, help advance tail, retry
   d. If next == NULL, retry (stale)
   e. Save value from next
   f. CAS head to next
   g. If success, break
2. Return old head as garbage

**Complexity:** O(1) amortized

**Property:** Lock-free.

## Concurrency

**Thread Safety:**
- SPSC: Safe for one producer, one consumer
- MPMC: Safe for multiple producers and consumers

**Progress Guarantee:**
- SPSC: Wait-free
- MPMC: Lock-free

**Memory Ordering:**
- Enqueue: Store-release semantics
- Dequeue: Load-acquire semantics
- MPMC uses ck_pr_cas_ptr_2 for double-wide CAS

## Platform Considerations

- SPSC uses cacheline padding to separate head/tail
- MPMC requires CK_F_PR_CAS_PTR_2 (double-wide CAS)
- MPMC uses 16-byte aligned tagged pointers
- Garbage collection is caller's responsibility
- SPSC provides built-in node recycling
