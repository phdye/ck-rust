# Module: ck_cohort

## Overview

The ck_cohort module implements lock cohorting, a technique for building NUMA-aware locks from any two lock types. A cohort lock uses a global lock and per-node local locks to minimize cross-NUMA traffic. Multiple threads on the same NUMA node can pass the global lock between themselves via the local lock, avoiding expensive inter-node synchronization.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Inline hints |
| ck_pr | Internal | Atomic operations, barriers |
| ck_stddef | External | NULL definition |

## Data Structures

### struct ck_cohort_* (Macro-generated)

**Description:** Cohort lock instance combining global and local locks.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| global_lock | void* | Platform pointer | Pointer to global lock |
| local_lock | void* | Platform pointer | Pointer to local lock |
| release_state | ck_cohort_state | 4 bytes | Global vs local release mode |
| waiting_threads | unsigned int | 4 bytes | Threads waiting on local lock |
| acquire_count | unsigned int | 4 bytes | Consecutive local acquisitions |
| local_pass_limit | unsigned int | 4 bytes | Max local passes before global release |

**States:**
- CK_COHORT_STATE_GLOBAL: Next acquire needs global lock
- CK_COHORT_STATE_LOCAL: Global lock held, pass locally

## Algorithms

### CK_COHORT_PROTOTYPE Macro

**Purpose:** Generate cohort lock type for specific lock primitives

**Parameters:**
- N: Name suffix for generated type
- GL/GU/GI: Global lock/unlock/is_locked
- LL/LU/LI: Local lock/unlock/is_locked

### ck_cohort_*_init

**Purpose:** Initialize cohort lock

**Algorithm:**
1. Set global_lock, local_lock pointers
2. Set release_state = GLOBAL
3. Set waiting_threads = 0, acquire_count = 0
4. Set local_pass_limit

**Complexity:** O(1)

### ck_cohort_*_lock

**Purpose:** Acquire cohort lock

**Algorithm:**
1. Atomically increment waiting_threads
2. Acquire local lock
3. Atomically decrement waiting_threads
4. IF release_state == GLOBAL:
   - Acquire global lock
5. Increment acquire_count

**Complexity:** O(local lock) + O(global lock if needed)

### ck_cohort_*_unlock

**Purpose:** Release cohort lock

**Algorithm:**
1. IF waiting_threads > 0 AND acquire_count < limit:
   - Set release_state = LOCAL (pass to local waiter)
2. ELSE:
   - Release global lock
   - Set release_state = GLOBAL
   - Reset acquire_count = 0
3. Release fence
4. Release local lock

**Complexity:** O(1) for local pass, O(global unlock) for global release

### CK_COHORT_TRYLOCK_PROTOTYPE

**Purpose:** Add trylock support to cohort type

**Algorithm:**
1. Increment waiting_threads
2. Try local lock
3. IF failed: return false
4. IF release_state == GLOBAL: try global lock
5. IF global failed: release local, return false
6. Return true

## Concurrency

**Thread Safety:** Fully thread-safe.

**Progress Guarantee:**
- Lock: Blocking (depends on underlying locks)
- Unlock: Wait-free

**NUMA Optimization:**
- Threads on same node pass lock locally
- Reduces cross-NUMA coherence traffic
- local_pass_limit bounds fairness loss

## Platform Considerations

- Macro-based to work with any lock types
- User provides lock/unlock/is_locked functions
- Default local_pass_limit = 10

**Correctness Reference:** Dice, D.; Marathe, V.; and Shavit, N. 2012. "Lock Cohorting: A General Technique for Designing NUMA Locks"
