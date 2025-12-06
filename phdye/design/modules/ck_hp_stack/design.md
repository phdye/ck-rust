# Module: ck_hp_stack

## Overview

The ck_hp_stack module provides hazard pointer-protected stack operations built on top of the ck_stack infrastructure. It wraps the unbounded MPMC (UPMC) stack operations with hazard pointer protection to enable safe memory reclamation of popped entries. This module is a thin wrapper that combines existing ck_stack functionality with ck_hp for ABA prevention and safe reclamation.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Inline hints |
| ck_hp | Internal | Hazard pointer infrastructure |
| ck_pr | Internal | Atomic operations |
| ck_stack | Internal | Underlying stack implementation |
| ck_stddef | External | NULL definition |

## Data Structures

This module reuses ck_stack and ck_stack_entry from the ck_stack module. No new data structures are defined.

### Constants

- CK_HP_STACK_SLOTS_COUNT: 1 (one hazard slot needed)
- CK_HP_STACK_SLOTS_SIZE: sizeof(void*)

## Algorithms

### ck_hp_stack_push_mpmc

**Purpose:** Push entry onto stack with hazard pointer safety.

**Algorithm:**
1. Delegate to ck_stack_push_upmc

**Complexity:** O(1)

**Note:** Push doesn't need HP protection - it only writes, doesn't read freed memory.

### ck_hp_stack_trypush_mpmc

**Purpose:** Non-blocking push attempt.

**Algorithm:**
1. Delegate to ck_stack_trypush_upmc

**Complexity:** O(1) single attempt

### ck_hp_stack_pop_mpmc

**Purpose:** Pop entry with hazard pointer protection.

**Algorithm:**
1. Load head pointer
2. If NULL, return NULL
3. Set hazard pointer (slot 0) to head
4. Verify head unchanged (double-check)
5. Loop:
   a. CAS head to entry->next
   b. If success, return entry
   c. If head became NULL, return NULL
   d. Set hazard to new head
   e. Spin-load until stable
6. Return popped entry

**Key Insight:** Hazard pointer protects head while reading head->next for CAS.

### ck_hp_stack_trypop_mpmc

**Purpose:** Non-blocking pop attempt.

**Algorithm:**
1. Load head
2. If NULL, return false
3. Set hazard pointer to head
4. Verify head unchanged
5. If changed, clear HP, return false
6. CAS head to entry->next
7. If fail, clear HP, return false
8. Return true with entry

**Complexity:** O(1) single attempt

## Concurrency

**Thread Safety:** All operations are thread-safe for multiple threads.

**Progress Guarantee:**
- push: Lock-free (via ck_stack_push_upmc)
- pop: Lock-free
- trypush/trypop: Wait-free (single attempt)

**Memory Ordering:**
- Hazard pointer set with fence (ck_hp_set_fence)
- CAS provides synchronization
- Entry visible before access

**Memory Reclamation:**
- Popped entries protected by caller's hazard pointer
- Caller responsible for reclamation via ck_hp infrastructure
- Only one hazard slot needed per pop operation

## Platform Considerations

- Builds on existing ck_stack (no code duplication)
- Single hazard slot (simpler than FIFO which needs 2)
- Uses ck_pr_cas_ptr_value for efficient CAS with result
- Push operations don't need HP (write-only path)
- Pop returns entry for caller-controlled reclamation
