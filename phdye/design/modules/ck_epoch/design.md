# Module: ck_epoch

## Overview

The ck_epoch module implements epoch-based safe memory reclamation (SMR) for lock-free data structures. It allows threads to defer memory deallocation until no other thread can be accessing the memory. Based on Keir Fraser's practical lock-freedom work, it uses generation counting to determine when deferred objects can be safely freed.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Inline hints, cache alignment |
| ck_md | Internal | Platform-specific definitions |
| ck_pr | Internal | Atomic operations, fences |
| ck_stack | Internal | Lock-free pending lists |
| ck_stdbool | External | bool type |

## Data Structures

### struct ck_epoch

**Description:** Global epoch state shared by all threads.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| epoch | unsigned int | 4 bytes | Current global epoch counter |
| n_free | unsigned int | 4 bytes | Count of recyclable records |
| records | ck_stack_t | Platform stack | Stack of all registered records |

### struct ck_epoch_record

**Description:** Per-thread epoch participation record.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| record_next | ck_stack_entry_t | Platform size | Link in global records stack |
| global | struct ck_epoch* | Platform pointer | Back-pointer to global state |
| state | unsigned int | 4 bytes | CK_HP_USED or CK_HP_FREE |
| epoch | unsigned int | 4 bytes | Thread's observed epoch |
| active | unsigned int | 4 bytes | Nesting depth of epoch sections |
| local | struct | Cache line | Sense detection buckets |
| n_pending | unsigned int | 4 bytes | Count of deferred callbacks |
| n_peak | unsigned int | 4 bytes | Peak pending count |
| n_dispatch | unsigned int | 4 bytes | Total dispatched callbacks |
| ct | void* | Platform pointer | User context |
| pending | ck_stack_t[] | CK_EPOCH_LENGTH stacks | Deferred callback lists |

**Cache Line Alignment:** Structure is cache-line aligned to prevent false sharing.

### struct ck_epoch_entry

**Description:** Embedded entry for deferred callbacks.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| function | ck_epoch_cb_t* | Platform pointer | Callback function |
| stack_entry | ck_stack_entry_t | Platform size | Link in pending stack |

### struct ck_epoch_section

**Description:** Optional section tracking for forward progress.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| bucket | unsigned int | 4 bytes | Sense bucket index |

## Algorithms

### ck_epoch_init

**Purpose:** Initialize global epoch state.

**Algorithm:**
1. Set epoch = 0
2. Set n_free = 0
3. Initialize records stack

**Complexity:** O(1)

### ck_epoch_register

**Purpose:** Register a thread's record with global state.

**Algorithm:**
1. Set record->global = epoch
2. Set record->state = USED
3. Initialize pending stacks
4. Push record to global records stack

**Complexity:** O(1)

### ck_epoch_begin

**Purpose:** Enter epoch-protected section.

**Algorithm:**
1. IF active == 0 (first nesting level):
   a. Store active = 1 with fence (TSO: fas + atomic fence)
   b. Load global epoch
   c. Store to record->epoch
2. ELSE: increment active
3. IF section provided: add reference

**Complexity:** O(1)

### ck_epoch_end

**Purpose:** Exit epoch-protected section.

**Algorithm:**
1. Release fence
2. Decrement active
3. IF section provided: remove reference

**Complexity:** O(1)

### ck_epoch_call

**Purpose:** Defer callback execution until safe.

**Algorithm:**
1. Load global epoch
2. Compute bucket = epoch % CK_EPOCH_LENGTH
3. Increment n_pending
4. Set entry->function
5. Push entry to pending[bucket]

**Complexity:** O(1)

### ck_epoch_poll

**Purpose:** Attempt to advance epoch and dispatch callbacks.

**Algorithm:**
1. Scan all records
2. IF all observed same or later epoch:
   - Increment global epoch
3. Dispatch callbacks from now-safe buckets
4. Return true if progress made

**Complexity:** O(n_records)

### ck_epoch_synchronize

**Purpose:** Block until all threads in current epoch exit.

**Algorithm:**
1. Enter epoch section
2. Loop until epoch advances twice:
   - Call poll
   - Busy wait if no progress
3. Exit epoch section

**Complexity:** Unbounded (depends on thread activity)

### ck_epoch_barrier

**Purpose:** Block and dispatch all pending callbacks.

**Algorithm:**
1. Synchronize
2. Reclaim all pending entries

**Complexity:** Unbounded

## Concurrency

**Thread Safety:** Fully thread-safe.

**Progress Guarantee:**
- begin/end: Wait-free
- call: Wait-free (UPMC) or Lock-free (MPMC via call_strict)
- poll: Lock-free
- synchronize/barrier: Blocking

**Memory Ordering:**
- begin: Store-load barrier for epoch observation
- end: Release fence before decrementing active
- call: Depends on underlying stack

## Platform Considerations

- CK_EPOCH_LENGTH configurable (default 4)
- CK_EPOCH_SENSE for sense detection (default 2)
- Cache-line aligned records to prevent false sharing
- TSO optimization uses fetch-and-store instead of store + fence

**Correctness Reference:** Fraser, K. 2004. "Practical Lock-Freedom." PhD Thesis, University of Cambridge.
