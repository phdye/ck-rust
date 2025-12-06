# Module: ck_hp_fifo

## Overview

The ck_hp_fifo module implements a concurrent MPMC FIFO queue with integrated hazard pointer protection for safe memory reclamation. Unlike ck_fifo_mpmc which uses double-wide CAS with generation counters, this implementation uses standard single-wide CAS operations with hazard pointers to prevent ABA problems and use-after-free. It is based on the Michael-Scott queue algorithm combined with hazard pointer protection.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Inline hints |
| ck_hp | Internal | Hazard pointer infrastructure |
| ck_pr | Internal | Atomic operations |
| ck_stddef | External | NULL definition |

## Data Structures

### struct ck_hp_fifo_entry

**Description:** Entry node with embedded hazard pointer state.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| value | void* | 8 bytes | User data pointer |
| hazard | ck_hp_hazard_t | varies | Hazard pointer state for reclamation |
| next | pointer | 8 bytes | Next entry in queue |

**Note:** Hazard pointer state embedded in entry for deferred reclamation.

### struct ck_hp_fifo

**Description:** MPMC queue with hazard pointer protection.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| head | pointer | 8 bytes | Dequeue position |
| tail | pointer | 8 bytes | Enqueue position |

### Constants

- CK_HP_FIFO_SLOTS_COUNT: 2 (hazard slots needed per operation)
- CK_HP_FIFO_SLOTS_SIZE: sizeof(void*) * 2

## Algorithms

### ck_hp_fifo_enqueue_mpmc

**Purpose:** Lock-free enqueue with hazard pointer protection.

**Algorithm:**
1. Set entry->value = value, entry->next = NULL
2. fence_store_atomic
3. Loop:
   a. Load tail pointer
   b. Set hazard pointer (slot 0) to tail
   c. Verify tail unchanged (double-check pattern)
   d. Load tail->next
   e. If tail changed, retry
   f. If next != NULL, help advance tail via CAS, continue
   g. CAS tail->next from NULL to entry
   h. If success, break
4. fence_atomic
5. CAS to advance tail (may fail, OK)

**Key Insight:** Hazard pointer protects tail while reading tail->next.

### ck_hp_fifo_tryenqueue_mpmc

**Purpose:** Non-blocking single-attempt enqueue.

**Algorithm:**
1. Prepare entry
2. Load tail, set hazard, verify
3. If tail changed, return false
4. If next != NULL, help advance, return false
5. CAS tail->next to entry
6. If fail, return false
7. Advance tail, return true

**Complexity:** O(1) single attempt

### ck_hp_fifo_dequeue_mpmc

**Purpose:** Lock-free dequeue with hazard pointer protection.

**Algorithm:**
1. Loop:
   a. Load head
   b. Set hazard (slot 0) to head
   c. Verify head unchanged
   d. Load tail
   e. Load head->next
   f. Set hazard (slot 1) to next
   g. Verify head unchanged
   h. If next == NULL: clear hazards, return NULL (empty)
   i. If head == tail: help advance tail, continue
   j. CAS head to next
   k. If success, break
2. Store value from next
3. Return old head (caller reclaims)

**Key Insight:** Two hazard slots protect head and next concurrently.

### ck_hp_fifo_trydequeue_mpmc

**Purpose:** Non-blocking single-attempt dequeue.

**Algorithm:**
1. Load head, set hazard, verify
2. Load tail, load next
3. Set hazard on next, verify
4. If empty or CAS fails, return NULL
5. Return head with value

**Complexity:** O(1) single attempt

## Concurrency

**Thread Safety:** Fully thread-safe for multiple producers and consumers.

**Progress Guarantee:** Lock-free (enqueue and dequeue).

**Memory Ordering:**
- Uses ck_pr_fence_* for visibility
- Hazard pointers provide safe reclamation
- ck_hp_set_fence combines HP set with fence

**Memory Reclamation:**
- Returned entries must be reclaimed via ck_hp_free
- Two hazard slots required per thread
- Entries contain embedded hazard state

## Platform Considerations

- No double-wide CAS required (unlike ck_fifo_mpmc)
- Requires hazard pointer infrastructure (ck_hp)
- Each thread needs ck_hp_record with 2 slots
- Embedded ck_hp_hazard_t enables deferred free
- More portable than generation counter approach
