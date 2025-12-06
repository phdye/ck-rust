# Module Inventory

## Modules

| Module | Description | Source Files | Dependencies |
|--------|-------------|--------------|--------------|
| ck_cc | Compiler compatibility macros and builtins | include/ck_cc.h, include/gcc/ck_cc.h | None |
| ck_pr | Core atomic primitives and memory barriers | include/ck_pr.h, include/gcc/ck_pr.h, include/gcc/ck_f_pr.h, include/gcc/{arch}/ck_pr.h | ck_cc |
| ck_backoff | Exponential backoff for contention | include/ck_backoff.h | ck_cc, ck_pr |
| ck_malloc | Memory allocator abstraction | include/ck_malloc.h | None |
| ck_elide | Hardware transactional memory lock elision | include/ck_elide.h | ck_cc, ck_pr |
| ck_spinlock | Spinlock implementations (FAS, CAS, ticket, MCS, CLH, etc.) | include/ck_spinlock.h, include/spinlock/*.h | ck_cc, ck_pr, ck_backoff, ck_elide |
| ck_rwlock | Centralized reader-writer lock | include/ck_rwlock.h | ck_pr, ck_elide |
| ck_pflock | Phase-fair reader-writer lock | include/ck_pflock.h | ck_cc, ck_pr |
| ck_swlock | Single-writer reader-writer lock | include/ck_swlock.h | ck_pr, ck_elide |
| ck_tflock | Task-fair reader-writer lock | include/ck_tflock.h | ck_cc, ck_pr |
| ck_brlock | Big-reader lock | include/ck_brlock.h | ck_pr |
| ck_bytelock | Byte-level reader-writer lock | include/ck_bytelock.h | ck_cc, ck_pr |
| ck_sequence | Sequence counter (seqlock) | include/ck_sequence.h | ck_cc, ck_pr |
| ck_barrier | Execution barriers (centralized, combining, dissemination, MCS, tournament) | include/ck_barrier.h, src/ck_barrier_*.c | ck_spinlock |
| ck_cohort | Lock cohorting for NUMA | include/ck_cohort.h | ck_cc, ck_pr |
| ck_rwcohort | Reader-writer lock cohorting for NUMA | include/ck_rwcohort.h | ck_cc, ck_pr, ck_cohort |
| ck_stack | Lock-free stack | include/ck_stack.h | ck_cc, ck_pr |
| ck_queue | Concurrent queue macros (BSD-style lists) | include/ck_queue.h | ck_pr |
| ck_fifo | Lock-free FIFO (SPSC and MPMC) | include/ck_fifo.h | ck_cc, ck_pr, ck_spinlock |
| ck_ring | Concurrent ring buffer | include/ck_ring.h | ck_cc, ck_pr |
| ck_array | Concurrently-readable dynamic array | include/ck_array.h, src/ck_array.c | ck_cc, ck_pr, ck_malloc |
| ck_bitmap | Concurrent bitmap | include/ck_bitmap.h | ck_cc, ck_pr |
| ck_hs | Hash set (single-writer, many-reader) | include/ck_hs.h, src/ck_hs.c | ck_cc, ck_pr, ck_malloc |
| ck_ht | Hash table (key-value pairs) | include/ck_ht.h, src/ck_ht.c | ck_cc, ck_pr, ck_malloc |
| ck_rhs | Robin-hood hash set | include/ck_rhs.h, src/ck_rhs.c | ck_cc, ck_pr, ck_malloc |
| ck_epoch | Epoch-based safe memory reclamation | include/ck_epoch.h, src/ck_epoch.c | ck_cc, ck_pr, ck_stack |
| ck_hp | Hazard pointer memory reclamation | include/ck_hp.h, src/ck_hp.c | ck_cc, ck_pr, ck_stack |
| ck_hp_fifo | Hazard pointer protected FIFO | include/ck_hp_fifo.h | ck_cc, ck_pr, ck_hp |
| ck_hp_stack | Hazard pointer protected stack | include/ck_hp_stack.h | ck_cc, ck_pr, ck_hp, ck_stack |
| ck_ec | Event counts (futex-based blocking) | include/ck_ec.h, src/ck_ec.c | ck_cc, ck_pr |

## Dependency Graph

```
                              ┌─────────┐
                              │  ck_cc  │ (Foundation)
                              └────┬────┘
                                   │
                              ┌────▼────┐
                              │  ck_pr  │ (Atomics)
                              └────┬────┘
                                   │
           ┌───────────────────────┼───────────────────────┐
           │                       │                       │
      ┌────▼────┐            ┌─────▼─────┐           ┌─────▼─────┐
      │ck_backoff│            │ ck_malloc │           │ ck_stack  │
      └────┬────┘            └─────┬─────┘           └─────┬─────┘
           │                       │                       │
      ┌────▼────┐                  │                 ┌─────┴─────┐
      │ck_elide │                  │                 │           │
      └────┬────┘                  │           ┌─────▼───┐ ┌─────▼────┐
           │                       │           │ck_epoch │ │  ck_hp   │
      ┌────▼─────┐                 │           └─────────┘ └────┬─────┘
      │ck_spinlock│                │                            │
      └────┬─────┘                 │                      ┌─────┴─────┐
           │                       │                      │           │
      ┌────▼────┐           ┌──────▼──────┐         ┌─────▼────┐┌────▼─────┐
      │ck_barrier│           │ ck_hs/ht/rhs│         │ck_hp_fifo││ck_hp_stack│
      └─────────┘           └─────────────┘         └──────────┘└──────────┘
           │
      ┌────▼────┐
      │ ck_fifo │
      └─────────┘

  Reader-Writer Locks (all depend on ck_pr, some on ck_elide):
  ┌──────────┬──────────┬──────────┬──────────┬──────────┬──────────┐
  │ck_rwlock │ck_pflock │ck_swlock │ck_tflock │ck_brlock │ck_bytelock│
  └──────────┴──────────┴──────────┴──────────┴──────────┴──────────┘

  Data Structures (depend on ck_pr):
  ┌──────────┬──────────┬──────────┬──────────┐
  │ck_queue  │ ck_ring  │ck_bitmap │ ck_array │
  └──────────┴──────────┴──────────┴──────────┘

  NUMA Cohorts:
  ┌───────────┐
  │ck_cohort  │
  └─────┬─────┘
        │
  ┌─────▼─────┐
  │ck_rwcohort│
  └───────────┘
```

## Circular Dependencies

None identified.

All dependencies flow from foundation (ck_cc) through primitives (ck_pr) to higher-level abstractions. The only cross-dependency is ck_rwcohort depending on ck_cohort, which is intentional layering.
