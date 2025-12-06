# Module: ck_stack

## Overview

The ck_stack module provides a lock-free intrusive stack implementation based on the Treiber stack algorithm. It supports multiple concurrency patterns with specialized variants for different producer/consumer configurations, allowing users to select the most efficient implementation for their use case.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Container macros, inline hints |
| ck_pr | Internal | Atomic operations, memory barriers |
| ck_stdbool | External | Boolean type |
| ck_stddef | External | NULL definition |

## Data Structures

### struct ck_stack_entry

**Description:** Intrusive list node embedded in user structures.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| next | Pointer to ck_stack_entry | Platform pointer size | Next entry in stack |

**Invariants:**
- next points to valid entry or NULL [OBSERVED]

**Memory Layout:**
- Total Size: sizeof(void*) bytes
- Alignment: Platform pointer alignment

### struct ck_stack

**Description:** Stack head structure.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| head | Pointer to ck_stack_entry | Platform pointer size | Top of stack |
| generation | Pointer to char (packed) | Platform pointer size | ABA counter for MPMC |

**Invariants:**
- head is NULL or points to valid entry [OBSERVED]
- generation increments on each pop (MPMC only) [SPECIFIED]

**Memory Layout:**
- Total Size: 2 × sizeof(void*) bytes (16 bytes on 64-bit)
- Alignment: Platform pointer alignment
- Note: Packed to enable double-width CAS on generation+head pair

## Algorithms

### ck_stack_init

**Purpose:** Initialize stack to empty state

**Signature:**
```
ck_stack_init(stack: Pointer to ck_stack) → void
```

**Algorithm:**
1. Set stack->head = NULL
2. Set stack->generation = NULL

**Complexity:** O(1)

### ck_stack_push_upmc (Unique Producers, Multiple Consumers)

**Purpose:** Push entry onto stack, lock-free, for unique producer nodes

**Signature:**
```
ck_stack_push_upmc(target: Pointer to ck_stack, entry: Pointer to ck_stack_entry) → void
```

**Algorithm:**
1. Load current stack head
2. Set entry->next = head
3. Fence store (ensure next is visible before CAS)
4. CAS loop: attempt to set head = entry
5. On failure: update entry->next and retry

**Complexity:** O(1) expected, O(∞) worst case under contention

**Correctness Reference:** Treiber (1986). "Systems Programming: Coping with Parallelism"

### ck_stack_pop_upmc

**Purpose:** Pop entry from stack, lock-free, for unique producer nodes

**Signature:**
```
ck_stack_pop_upmc(target: Pointer to ck_stack) → Pointer to ck_stack_entry (or NULL)
```

**Algorithm:**
1. Load current stack head
2. IF head is NULL: return NULL
3. Fence load (ensure head read before next)
4. Load head->next
5. CAS loop: attempt to set head = next
6. On failure: reload head and next, retry
7. Return original head

**Complexity:** O(1) expected, O(∞) worst case

**Note:** Safe only when entries are unique (not reused without safe memory reclamation).

### ck_stack_push_mpmc / ck_stack_pop_mpmc (Multiple Producers, Multiple Consumers)

**Purpose:** Push/pop with ABA problem prevention

**Signature:**
```
ck_stack_push_mpmc(target: Pointer to ck_stack, entry: Pointer to ck_stack_entry) → void
ck_stack_pop_mpmc(target: Pointer to ck_stack) → Pointer to ck_stack_entry (or NULL)
```

**Algorithm (pop):**
1. Load generation and head atomically
2. IF head is NULL: return NULL
3. Compute: new_generation = generation + 1, new_head = head->next
4. Double-width CAS: attempt to update (generation, head) pair
5. On failure: retry with new values
6. Return original head

**Complexity:** O(1) expected

**Note:** Requires CK_F_PR_CAS_PTR_2_VALUE (128-bit CAS on 64-bit systems).

### ck_stack_push_mpnc (Multiple Producers, No Consumers)

**Purpose:** Push optimized for append-only usage

**Signature:**
```
ck_stack_push_mpnc(target: Pointer to ck_stack, entry: Pointer to ck_stack_entry) → void
```

**Algorithm:**
1. Set entry->next = NULL
2. Fence store-atomic
3. Atomically swap: old_head = FAS(target->head, entry)
4. Set entry->next = old_head
5. Fence store

**Complexity:** O(1)

**Note:** Only safe when no concurrent consumers exist.

### ck_stack_push_spnc / ck_stack_pop_npsc

**Purpose:** Non-concurrent single-threaded operations

**Signature:**
```
ck_stack_push_spnc(target: Pointer to ck_stack, entry: Pointer to ck_stack_entry) → void
ck_stack_pop_npsc(target: Pointer to ck_stack) → Pointer to ck_stack_entry (or NULL)
```

**Algorithm:** Direct pointer manipulation without atomics.

**Complexity:** O(1)

### ck_stack_batch_pop_upmc

**Purpose:** Pop all entries atomically

**Signature:**
```
ck_stack_batch_pop_upmc(target: Pointer to ck_stack) → Pointer to ck_stack_entry (or NULL)
```

**Algorithm:**
1. Atomically swap: entries = FAS(target->head, NULL)
2. Fence load
3. Return entries (linked list of all former stack contents)

**Complexity:** O(1)

## Concurrency

**Thread Safety:** Depends on variant:
- `_upmc`: Multiple unique producers, multiple consumers
- `_mpmc`: Multiple producers (with reuse), multiple consumers
- `_mpnc`: Multiple producers, no consumers
- `_spnc`: Single producer, no consumers
- `_npsc`: No producers, single consumer

**Memory Ordering:**
- Push: release semantics (fence_store before CAS)
- Pop: acquire semantics (fence_load after head read)

**Progress Guarantee:** Lock-free for all concurrent variants.

**ABA Problem:**
- UPMC variants: Caller must ensure entries are not reused (or use safe memory reclamation)
- MPMC variants: Use generation counter to prevent ABA

## Platform Considerations

- MPMC pop requires double-width CAS (CK_F_PR_CAS_PTR_2_VALUE)
- On platforms without double-width CAS, MPMC pop is not available
- Stack structure is packed to enable double-width CAS on (generation, head) pair
