# Implementation Order

## Overview

This document specifies the order in which modules must be implemented.
Each tier contains modules that can be implemented in parallel once all
modules in previous tiers are complete.

## Implementation Tiers

### Tier 0: Foundation (No Internal Dependencies)

| Module | External Dependencies | Rationale |
|--------|----------------------|-----------|
| ck_cc | C99 compiler | Compiler compatibility layer, no CK dependencies |
| ck_malloc | C99 stdlib | Memory allocator interface, pure abstraction |

### Tier 1: Core Primitives (Depends on Tier 0 Only)

| Module | Dependencies | Rationale |
|--------|--------------|-----------|
| ck_pr | ck_cc | Atomic primitives, depends only on compiler layer |

### Tier 2: Basic Building Blocks (Depends on Tiers 0-1)

| Module | Dependencies | Rationale |
|--------|--------------|-----------|
| ck_backoff | ck_cc, ck_pr | Simple exponential backoff using atomics |
| ck_stack | ck_cc, ck_pr | Lock-free stack, fundamental data structure |
| ck_queue | ck_pr | BSD-style queue macros |
| ck_ring | ck_cc, ck_pr | Ring buffer, no complex dependencies |
| ck_bitmap | ck_cc, ck_pr | Bitmap operations |
| ck_sequence | ck_cc, ck_pr | Sequence counter |
| ck_pflock | ck_cc, ck_pr | Phase-fair lock (no elide) |
| ck_tflock | ck_cc, ck_pr | Task-fair lock (no elide) |
| ck_brlock | ck_pr | Big-reader lock |
| ck_bytelock | ck_cc, ck_pr | Byte lock |
| ck_cohort | ck_cc, ck_pr | Lock cohort (template) |
| ck_ec | ck_cc, ck_pr | Event counts |

### Tier 3: Extended Primitives (Depends on Tiers 0-2)

| Module | Dependencies | Rationale |
|--------|--------------|-----------|
| ck_elide | ck_cc, ck_pr | HTM elision, depends on atomics |
| ck_array | ck_cc, ck_pr, ck_malloc | Dynamic array needs allocator |
| ck_hs | ck_cc, ck_pr, ck_malloc | Hash set needs allocator |
| ck_ht | ck_cc, ck_pr, ck_malloc | Hash table needs allocator |
| ck_rhs | ck_cc, ck_pr, ck_malloc | Robin-hood hash set needs allocator |
| ck_epoch | ck_cc, ck_pr, ck_stack | Epoch reclamation uses stack |
| ck_hp | ck_cc, ck_pr, ck_stack | Hazard pointers use stack |
| ck_rwcohort | ck_cc, ck_pr, ck_cohort | RW cohort extends cohort |

### Tier 4: Composite Locks (Depends on Tiers 0-3)

| Module | Dependencies | Rationale |
|--------|--------------|-----------|
| ck_spinlock | ck_cc, ck_pr, ck_backoff, ck_elide | Spinlock variants use backoff and elide |
| ck_rwlock | ck_pr, ck_elide | RW lock with elision support |
| ck_swlock | ck_pr, ck_elide | SW lock with elision support |

### Tier 5: High-Level Structures (Depends on Tiers 0-4)

| Module | Dependencies | Rationale |
|--------|--------------|-----------|
| ck_barrier | ck_spinlock | Barriers use spinlocks internally |
| ck_fifo | ck_cc, ck_pr, ck_spinlock | FIFO uses spinlocks for MPMC |
| ck_hp_fifo | ck_cc, ck_pr, ck_hp | HP-protected FIFO |
| ck_hp_stack | ck_cc, ck_pr, ck_hp, ck_stack | HP-protected stack |

## Circular Dependency Resolution

No circular dependencies to resolve.

## Capture Order

Based on the tiers above, capture modules in this sequence:

1. ck_cc (Tier 0)
2. ck_malloc (Tier 0)
3. ck_pr (Tier 1)
4. ck_backoff (Tier 2)
5. ck_stack (Tier 2)
6. ck_queue (Tier 2)
7. ck_ring (Tier 2)
8. ck_bitmap (Tier 2)
9. ck_sequence (Tier 2)
10. ck_pflock (Tier 2)
11. ck_tflock (Tier 2)
12. ck_brlock (Tier 2)
13. ck_bytelock (Tier 2)
14. ck_cohort (Tier 2)
15. ck_ec (Tier 2)
16. ck_elide (Tier 3)
17. ck_array (Tier 3)
18. ck_hs (Tier 3)
19. ck_ht (Tier 3)
20. ck_rhs (Tier 3)
21. ck_epoch (Tier 3)
22. ck_hp (Tier 3)
23. ck_rwcohort (Tier 3)
24. ck_spinlock (Tier 4)
25. ck_rwlock (Tier 4)
26. ck_swlock (Tier 4)
27. ck_barrier (Tier 5)
28. ck_fifo (Tier 5)
29. ck_hp_fifo (Tier 5)
30. ck_hp_stack (Tier 5)

Note: Modules within the same tier may be captured in any order.
