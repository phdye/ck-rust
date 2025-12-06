# Module: ck_ring

## Overview

The ck_ring module provides a bounded FIFO ring buffer (circular queue) with multiple concurrency variants. It supports SPSC (single producer, single consumer), SPMC (single producer, multiple consumers), MPSC (multiple producers, single consumer), and MPMC (multiple producers, multiple consumers) configurations. The implementation uses separate producer and consumer indices with cache line padding to avoid false sharing.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Inline hints, likely/unlikely macros |
| ck_md | Internal | Cache line size for padding |
| ck_pr | Internal | Atomic operations |
| ck_stdbool | External | Boolean type |
| ck_string | External | memcpy for slot operations |

## Data Structures

### struct ck_ring

**Description:** Ring buffer control structure with cache-aligned counters.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| c_head | unsigned int | 4 bytes | Consumer head index |
| pad | char[] | CK_MD_CACHELINE - 4 | Padding to cache line |
| p_tail | unsigned int | 4 bytes | Producer tail (committed) |
| p_head | unsigned int | 4 bytes | Producer head (reserved) |
| _pad | char[] | CK_MD_CACHELINE - 8 | Padding to cache line |
| size | unsigned int | 4 bytes | Ring capacity (power of 2) |
| mask | unsigned int | 4 bytes | size - 1 for index wrapping |

**Invariants:**
- size is power of 2 [SPECIFIED]
- c_head <= p_head [OBSERVED]
- p_head - c_head <= size [SPECIFIED]
- p_tail <= p_head [OBSERVED]

**Memory Layout:**
- Total Size: 2 × CK_MD_CACHELINE + 8 bytes typically
- Alignment: Cache line aligned fields prevent false sharing

### struct ck_ring_buffer

**Description:** Buffer slot for pointer storage.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| value | void* | Platform pointer size | Stored value |

## Algorithms

### ck_ring_init

**Purpose:** Initialize ring buffer to empty state

**Signature:**
```
ck_ring_init(ring: Pointer to ck_ring, size: unsigned int) → void
```

**Algorithm:**
1. Set ring->size = size
2. Set ring->mask = size - 1
3. Set p_tail = p_head = c_head = 0

**Precondition:** size must be power of 2

**Complexity:** O(1)

### ck_ring_size

**Purpose:** Return current number of elements in ring

**Signature:**
```
ck_ring_size(ring: Pointer to const ck_ring) → unsigned int
```

**Algorithm:**
1. Load c_head atomically
2. Load p_tail atomically
3. Return (p_tail - c_head) & mask

**Complexity:** O(1)

**Note:** Result may be stale immediately after return.

### ck_ring_enqueue_spsc

**Purpose:** Enqueue element with single producer, single consumer

**Signature:**
```
ck_ring_enqueue_spsc(ring, buffer, entry) → bool
```

**Algorithm:**
1. Load consumer head (atomic)
2. producer = ring->p_tail (local read, single producer)
3. delta = producer + 1
4. IF (delta & mask) == (consumer & mask): return false (full)
5. Copy entry to buffer[producer & mask]
6. Store fence
7. Store delta to p_tail (atomic)
8. Return true

**Complexity:** O(1)

### ck_ring_dequeue_spsc

**Purpose:** Dequeue element with single producer, single consumer

**Signature:**
```
ck_ring_dequeue_spsc(ring, buffer, data) → bool
```

**Algorithm:**
1. consumer = ring->c_head (local read, single consumer)
2. Load producer tail (atomic)
3. IF consumer == producer: return false (empty)
4. Load fence
5. Copy buffer[consumer & mask] to data
6. Store fence
7. Store consumer + 1 to c_head (atomic)
8. Return true

**Complexity:** O(1)

### ck_ring_enqueue_mpmc

**Purpose:** Enqueue with multiple producers, multiple consumers

**Signature:**
```
ck_ring_enqueue_mpmc(ring, buffer, entry) → bool
```

**Algorithm:**
1. Load producer head (p_head)
2. Loop:
   a. Load fence
   b. Load consumer head
   c. delta = producer + 1
   d. IF (producer - consumer) < mask:
      - CAS p_head from producer to delta
      - IF success: break
   e. ELSE (ring appears full):
      - Load fence
      - Re-read p_head
      - IF unchanged: return false (truly full)
      - Update producer = new p_head
3. Copy entry to buffer[producer & mask]
4. Wait for p_tail to reach producer (ordering)
5. Store fence
6. Store delta to p_tail
7. Return true

**Complexity:** O(1) expected, O(∞) worst case under contention

### ck_ring_dequeue_mpmc

**Purpose:** Dequeue with multiple producers, multiple consumers

**Signature:**
```
ck_ring_dequeue_mpmc(ring, buffer, data) → bool
```

**Algorithm:**
1. Load consumer head (c_head)
2. Loop:
   a. Load fence
   b. Load producer tail
   c. IF consumer == producer: return false (empty)
   d. Load fence
   e. Copy buffer[consumer & mask] to data
   f. Store-atomic fence
   g. CAS c_head from consumer to consumer + 1
   h. IF success: return true
   i. Update consumer from CAS result

**Complexity:** O(1) expected, O(∞) worst case

### ck_ring_trydequeue_mpmc

**Purpose:** Non-blocking dequeue attempt

**Algorithm:**
Same as dequeue_mpmc but returns after single CAS attempt (no loop).

**Complexity:** O(1) always

### Reserve/Commit Pattern

**Purpose:** Two-phase enqueue for zero-copy operations

**ck_ring_enqueue_reserve_*:**
1. Reserve slot by advancing producer index
2. Return pointer to slot for caller to write directly

**ck_ring_enqueue_commit_*:**
1. Wait for predecessors (MP variants)
2. Commit by advancing p_tail

**Use Case:** Large data structures, avoiding double copy

## Concurrency

**Thread Safety:** Depends on variant:
- `_spsc`: Single producer, single consumer
- `_spmc`: Single producer, multiple consumers
- `_mpsc`: Multiple producers, single consumer
- `_mpmc`: Multiple producers, multiple consumers

**Memory Ordering:**
- Enqueue: Release semantics (fence before p_tail update)
- Dequeue: Acquire semantics (fence before data read)
- MP enqueue: Additional ordering to commit in-order

**Progress Guarantee:**
- SPSC: wait-free
- SPMC dequeue: lock-free (CAS loop)
- MPSC enqueue: blocking (waits for p_tail)
- MPMC: lock-free

**False Sharing Prevention:**
- c_head isolated in own cache line
- p_tail and p_head in separate cache line
- Producers don't write consumer's cache line and vice versa

## Platform Considerations

- Ring size must be power of 2 for efficient modulo via masking
- Cache line padding assumes CK_MD_CACHELINE correctly set
- CK_RING_PROTOTYPE macro generates type-safe variants for custom types
