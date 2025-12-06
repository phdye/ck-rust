# Module: ck_barrier

## Overview

The ck_barrier module provides multiple barrier synchronization implementations for parallel threads. A barrier ensures all participating threads reach a synchronization point before any proceed. The module offers five different barrier algorithms with varying complexity and scalability characteristics: centralized, combining tree, dissemination, tournament, and MCS tree barriers.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_spinlock | Internal | Mutex for combining barrier |
| ck_cc | Internal | Cacheline alignment |
| ck_pr | Internal | Atomic operations |

## Data Structures

### struct ck_barrier_centralized

**Description:** Simple counter-based barrier.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| value | unsigned int | 4 bytes | Thread arrival counter |
| sense | unsigned int | 4 bytes | Global sense flag |

**Constants:**
- CK_BARRIER_CENTRALIZED_INITIALIZER: {0, 0}

### struct ck_barrier_centralized_state

**Description:** Per-thread state for centralized barrier.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| sense | unsigned int | 4 bytes | Local sense flag |

### struct ck_barrier_combining_group

**Description:** Node in combining tree barrier.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| k | unsigned int | 4 bytes | Fan-in degree |
| count | unsigned int | 4 bytes | Arrival counter |
| sense | unsigned int | 4 bytes | Node sense flag |
| parent | pointer | 8 bytes | Parent node |
| left | pointer | 8 bytes | Left child |
| right | pointer | 8 bytes | Right child |
| next | pointer | 8 bytes | Next sibling |

**Alignment:** CK_CC_CACHELINE (avoids false sharing)

### struct ck_barrier_combining

**Description:** Combining tree barrier root.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| root | pointer | 8 bytes | Tree root node |
| mutex | ck_spinlock_fas_t | varies | Lock for tree construction |

### struct ck_barrier_dissemination

**Description:** Dissemination barrier state.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| nthr | unsigned int | 4 bytes | Number of threads |
| size | unsigned int | 4 bytes | Number of rounds |
| tid | unsigned int | 4 bytes | Next thread ID |
| flags | array[2] | 16 bytes | Flag arrays (parity) |

### struct ck_barrier_tournament

**Description:** Tournament tree barrier.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| tid | unsigned int | 4 bytes | Next thread ID |
| size | unsigned int | 4 bytes | Number of rounds |
| rounds | pointer | 8 bytes | Round array per thread |

### struct ck_barrier_mcs

**Description:** MCS tree barrier node.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| tid | unsigned int | 4 bytes | Thread ID |
| children | array[2] | 16 bytes | Child notification pointers |
| childnotready | array[4] | 16 bytes | Child arrival flags |
| havechild | array[4] | 16 bytes | Child existence flags |
| parent | pointer | 8 bytes | Parent notification pointer |
| parentsense | unsigned int | 4 bytes | Parent sense value |

## Algorithms

### ck_barrier_centralized

**Purpose:** Simple centralized counting barrier.

**Algorithm:**
1. Increment global counter atomically
2. If last thread (counter == n):
   - Reset counter to 0
   - Toggle global sense (release others)
3. Else:
   - Spin until global sense matches local
4. Toggle local sense for next round

**Complexity:** O(n) time, O(1) space per thread

### ck_barrier_combining

**Purpose:** Tree-based combining barrier for scalability.

**Algorithm:**
1. Each thread assigned to leaf node
2. Traverse up tree, incrementing counts
3. Last arrival at node propagates up
4. Root toggles sense, propagates down
5. Threads detect sense change, proceed

**Complexity:** O(log n) time, O(n) space

### ck_barrier_dissemination

**Purpose:** All-to-all dissemination pattern.

**Algorithm:**
1. For round r = 0 to ceil(log n)-1:
   - Signal partner at distance 2^r
   - Wait for signal from partner
2. Use parity to avoid write-after-read races

**Complexity:** O(log n) rounds, O(n log n) messages

### ck_barrier_tournament

**Purpose:** Tournament elimination tree.

**Algorithm:**
1. Threads assigned roles: winner, loser, champion, dropout, bye
2. In each round:
   - Losers signal and wait
   - Winners collect signals, advance
3. Champion at root toggles sense
4. Winners release losers on descent

**Complexity:** O(log n) time

### ck_barrier_mcs

**Purpose:** 4-ary tree barrier (Mellor-Crummey & Scott).

**Algorithm:**
1. Each node waits for 4 children
2. Signals parent when all children arrived
3. Root wakes children recursively
4. Uses separate arrival/release flags

**Complexity:** O(log_4 n) time, low contention

## Concurrency

**Thread Safety:** All barriers are thread-safe when used correctly.

**Progress Guarantee:** Blocking (all threads must participate).

**Memory Ordering:**
- All operations use appropriate fences
- Sense toggling provides release-acquire synchronization

## Platform Considerations

- Centralized: Simple, good for small thread counts
- Combining: Good for NUMA (locality)
- Dissemination: No central bottleneck
- Tournament: Low memory traffic
- MCS: Excellent scalability, low contention
