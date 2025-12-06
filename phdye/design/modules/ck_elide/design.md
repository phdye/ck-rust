# Module: ck_elide

## Overview

The ck_elide module provides hardware lock elision wrappers using Intel's Restricted Transactional Memory (RTM). Lock elision allows critical sections to execute speculatively without acquiring the lock, improving throughput when conflicts are rare. If the transaction aborts (due to conflicts, capacity, or other reasons), the code falls back to traditional locking.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Inline hints |
| ck_pr | Internal | RTM intrinsics (ck_pr_rtm_begin, etc.) |
| ck_string | External | memset |

## Data Structures

### struct ck_elide_config

**Description:** Per-lock elision retry and skip configuration.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| skip_busy | unsigned short | 2 bytes | Consecutive forfeits after busy abort |
| retry_busy | short | 2 bytes | Retry attempts on busy |
| skip_other | unsigned short | 2 bytes | Consecutive forfeits after other abort |
| retry_other | short | 2 bytes | Retry attempts on other |
| skip_conflict | unsigned short | 2 bytes | Consecutive forfeits after conflict |
| retry_conflict | short | 2 bytes | Retry attempts on conflict |

**Default Values:**
- skip_busy = 5, retry_busy = 256
- skip_other = 3, retry_other = 3
- skip_conflict = 2, retry_conflict = 5

### struct ck_elide_stat

**Description:** Per-thread elision statistics.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| n_fallback | unsigned int | 4 bytes | Count of fallback acquisitions |
| n_elide | unsigned int | 4 bytes | Count of successful elisions |
| skip | unsigned short | 2 bytes | Remaining forfeits |

## Algorithms

### CK_ELIDE_PROTOTYPE Macro

**Purpose:** Generate lock elision wrappers for a lock type.

**Parameters:**
- N: Name suffix
- T: Lock type
- L_P: Lock predicate (returns true if locked)
- L: Lock acquire function
- U_P: Unlock predicate (returns true if not elided)
- U: Unlock release function

**Generated Functions:**
- ck_elide_N_lock: Best-effort elision lock
- ck_elide_N_unlock: Best-effort elision unlock
- ck_elide_N_lock_adaptive: Adaptive elision with retry
- ck_elide_N_unlock_adaptive: Adaptive unlock with stats

### ck_elide_N_lock (Best-Effort)

**Purpose:** Attempt elided lock acquisition without adaptation.

**Algorithm:**
1. Call ck_pr_rtm_begin()
2. IF started: check if lock appears held
   - IF held: abort with LOCK_BUSY
   - ELSE: return (in transaction)
3. ELSE: call fallback lock function

**Complexity:** O(1) for elision, O(lock) for fallback

### ck_elide_N_lock_adaptive

**Purpose:** Adaptive elision with configurable retry and skip.

**Algorithm:**
1. IF skip > 0: decrement skip, goto fallback
2. Set retry = retry_conflict
3. Loop:
   a. Call ck_pr_rtm_begin()
   b. IF started and lock not held: return
   c. IF started and lock held: abort LOCK_BUSY
   d. On abort, call _ck_elide_fallback():
      - Analyze status (explicit/conflict/capacity/etc.)
      - Return RETRY, SPIN, or STOP hint
   e. On RETRY: continue loop
   f. On SPIN: busy-wait for lock release, continue
   g. On STOP: break to fallback
4. Fallback: call lock function

**Complexity:** O(retry * spin) worst case

### ck_elide_N_unlock

**Purpose:** End transaction or release lock.

**Algorithm:**
1. IF U_P returns false (was elided):
   - Call ck_pr_rtm_end()
2. ELSE: call unlock function

**Complexity:** O(1)

## Concurrency

**Thread Safety:** Depends on underlying lock.

**Progress Guarantee:**
- Elision: Optimistic (may abort)
- Fallback: Inherits from underlying lock

**RTM Status Codes:**
- CK_PR_RTM_STARTED: Transaction active
- CK_PR_RTM_EXPLICIT: Explicit abort (e.g., LOCK_BUSY)
- CK_PR_RTM_CONFLICT: Data conflict
- CK_PR_RTM_RETRY: Transient, retry may succeed

## Platform Considerations

**RTM Support:** Only x86/x86-64 with TSX (CK_F_PR_RTM).

**Fallback:** On platforms without RTM, macros generate direct calls to lock/unlock (no elision overhead, but stat storage cost remains).

**TSO Dependency:** Fences omitted because RTM only exists on TSO architectures.
