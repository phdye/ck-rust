# Capture Scope

## System Identification

| Field | Value |
|-------|-------|
| System Name | Concurrency Kit (CK) |
| Repository | https://github.com/concurrencykit/ck |
| Version/Commit | 7c357ea9e006f95e569fff4c7fd4148c02a2fa74 |
| Capture Date | 2025-12-06 |

## In Scope

| Component | Justification |
|-----------|---------------|
| ck_pr | Core concurrency primitives - atomic operations, memory barriers, fences |
| ck_cc | Compiler abstraction layer - required by all other modules |
| ck_backoff | Exponential backoff - used by spinlocks and contention management |
| ck_malloc | Memory allocator abstraction - required by data structures |
| ck_spinlock | Basic spinlock implementations (FAS, CAS, ticket, MCS, CLH, etc.) |
| ck_rwlock | Reader-writer lock - centralized write-biased implementation |
| ck_pflock | Phase-fair reader-writer lock |
| ck_swlock | Single-writer reader-writer lock |
| ck_tflock | Task-fair reader-writer lock |
| ck_brlock | Big-reader lock |
| ck_bytelock | Byte-level reader-writer lock |
| ck_sequence | Seqlock/sequence counter |
| ck_barrier | Execution barriers (centralized, combining, dissemination, MCS, tournament) |
| ck_cohort | Lock cohorting for NUMA |
| ck_rwcohort | Reader-writer lock cohorting for NUMA |
| ck_elide | Hardware transactional memory (HTM) lock elision |
| ck_stack | Lock-free stack |
| ck_queue | Concurrent queue (BSD-style linked lists) |
| ck_fifo | Lock-free FIFO (SPSC and MPMC variants) |
| ck_ring | Concurrent ring buffer |
| ck_array | Concurrently-readable array |
| ck_bitmap | Concurrent bitmap |
| ck_hs | Hash set (single-writer, many-reader) |
| ck_ht | Hash table (key-value variant of ck_hs) |
| ck_rhs | Robin-hood hash set |
| ck_epoch | Epoch-based safe memory reclamation |
| ck_hp | Hazard pointer safe memory reclamation |
| ck_hp_fifo | Hazard pointer protected FIFO |
| ck_hp_stack | Hazard pointer protected stack |
| ck_ec | Event counts (futex-based blocking) |

## Out of Scope

| Component | Justification |
|-----------|---------------|
| Build system (configure, Makefile.in) | Build infrastructure, not functional code |
| Regression tests | Test infrastructure, captured separately in tests.md per module |
| Documentation (doc/) | Generated documentation, not source of truth |
| Platform-specific implementations (gcc/*, spinlock/*) | Implementation details of in-scope modules, documented within module platform.md |
| ck_limits.h, ck_stdbool.h, ck_stddef.h, ck_stdint.h, ck_stdlib.h, ck_string.h | Standard library wrappers for portability, trivial |

## Boundary Dependencies

These external components are used but NOT captured. Only their assumed contracts are documented.

| Dependency | Type | Assumed Contract |
|------------|------|------------------|
| C99 Standard Library | OS API | Standard C99 functions (malloc, free, memcpy, etc.) |
| POSIX Threads (pthread) | OS API | Used only in regression tests, not in library itself |
| GCC/Clang builtins | Compiler | Atomic builtins (__atomic_*, __sync_*), branch hints, memory barriers |
| CPU atomic instructions | Hardware | CAS, fetch-and-add, load-linked/store-conditional, memory fences |
| Cache line size | Hardware | Assumed 64 bytes (CK_MD_CACHELINE typically 64) |
| Memory model | Hardware | Platform-specific (TSO for x86, relaxed for ARM) |
