# Module: ck_hp

## Overview

The ck_hp module implements Maged Michael's hazard pointers for safe memory reclamation in lock-free data structures. Each thread publishes pointers it's currently accessing to hazard pointer slots. Before freeing memory, threads scan all hazard pointers to ensure no other thread holds a reference. This provides bounded memory overhead per thread.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Inline hints, cache alignment |
| ck_md | Internal | Platform-specific definitions |
| ck_pr | Internal | Atomic operations |
| ck_stack | Internal | Lock-free pending lists |

## Data Structures

### struct ck_hp

**Description:** Global hazard pointer state.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| subscribers | ck_stack_t | Platform stack | Stack of registered records |
| n_subscribers | unsigned int | 4 bytes | Count of registered threads |
| n_free | unsigned int | 4 bytes | Count of recyclable records |
| threshold | unsigned int | 4 bytes | Pending count trigger for scan |
| degree | unsigned int | 4 bytes | Hazard pointers per thread |
| destroy | ck_hp_destructor_t | Platform pointer | Destructor callback |

### struct ck_hp_record

**Description:** Per-thread hazard pointer record.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| state | int | 4 bytes | CK_HP_USED or CK_HP_FREE |
| pointers | void** | Platform pointer | Array of degree hazard slots |
| cache | void*[] | CK_HP_CACHE pointers | Local scan cache |
| global | struct ck_hp* | Platform pointer | Back-pointer to global |
| pending | ck_stack_t | Platform stack | Objects awaiting reclamation |
| n_pending | unsigned int | 4 bytes | Count of pending objects |
| global_entry | ck_stack_entry_t | Platform size | Link in subscribers list |
| n_peak | unsigned int | 4 bytes | Peak pending count |
| n_reclamations | uint64_t | 8 bytes | Total reclamations |

**Cache Line Alignment:** Structure is cache-line aligned.

### struct ck_hp_hazard

**Description:** Metadata for objects awaiting reclamation.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| pointer | void* | Platform pointer | Actual data pointer |
| data | void* | Platform pointer | User context for destructor |
| pending_entry | ck_stack_entry_t | Platform size | Link in pending list |

## Algorithms

### ck_hp_init

**Purpose:** Initialize global hazard pointer state.

**Algorithm:**
1. Set degree (hazard pointers per thread)
2. Set threshold (when to trigger scan)
3. Set destroy callback
4. Initialize subscribers stack

**Complexity:** O(1)

### ck_hp_register

**Purpose:** Register thread with hazard pointer domain.

**Algorithm:**
1. Allocate pointers array (degree slots)
2. Clear all slots to NULL
3. Set record->global
4. Push record to subscribers

**Complexity:** O(degree)

### ck_hp_set

**Purpose:** Publish hazard pointer (no fence).

**Algorithm:**
1. Atomic store pointer to slot

**Complexity:** O(1)

### ck_hp_set_fence

**Purpose:** Publish hazard pointer with ordering.

**Algorithm:**
1. On TSO: fetch-and-store
2. On non-TSO: store + memory fence

**Complexity:** O(1)

### ck_hp_clear

**Purpose:** Clear all hazard pointers.

**Algorithm:**
1. Set all degree slots to NULL

**Complexity:** O(degree)

### ck_hp_retire

**Purpose:** Defer object reclamation.

**Algorithm:**
1. Set hazard->pointer, hazard->data
2. Push to record->pending
3. Increment n_pending

**Complexity:** O(1)

### ck_hp_free

**Purpose:** Retire object and attempt reclaim if threshold reached.

**Algorithm:**
1. Call retire
2. IF n_pending >= threshold:
   - Call reclaim

**Complexity:** O(1) or O(n_subscribers * degree) if reclaim

### ck_hp_reclaim

**Purpose:** Scan hazard pointers and free safe objects.

**Algorithm:**
1. Build cache of all active hazard pointers:
   - Scan all records
   - Collect non-NULL pointers
2. For each pending object:
   - IF not in cache: call destructor
   - ELSE: re-add to pending
3. Update n_pending

**Complexity:** O(n_subscribers * degree + n_pending)

### ck_hp_purge

**Purpose:** Forcibly reclaim all safe objects.

**Algorithm:**
1. Repeatedly call reclaim until no progress

**Complexity:** Unbounded

## Concurrency

**Thread Safety:** Fully thread-safe.

**Progress Guarantee:**
- set/clear: Wait-free
- retire: Wait-free
- reclaim: Lock-free
- purge: Blocking

**Memory Ordering:**
- set_fence: Acquire-like semantics
- Publication pattern: set_fence then read data

## Platform Considerations

- CK_HP_CACHE size configurable (default 512)
- Cache-line aligned records
- TSO optimization for set_fence
- Degree set at init time
