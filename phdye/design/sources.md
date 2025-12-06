# Source Materials

## Primary Sources

| Source | Location | Description |
|--------|----------|-------------|
| Source code | /home/user/ck | Definitive implementation of all modules |
| README.md | /home/user/ck/README.md | Overview of features and supported architectures |
| Man pages | /home/user/ck/doc/*.3 | API documentation for each module |

## Academic Papers

| Citation | Location | Relevance |
|----------|----------|-----------|
| Mellor-Crummey & Scott (1991). "Algorithms for Scalable Synchronization on Shared-Memory Multiprocessors" | DOI: 10.1145/103727.103729 | MCS lock algorithm in spinlock/mcs.h |
| Craig (1993). "Building FIFO and priority-queuing spin locks from atomic swap" | TR 93-02-02, U. Washington | CLH lock algorithm in spinlock/clh.h |
| Anderson (1990). "The Performance of Spin Lock Alternatives for Shared-Memory Multiprocessors" | IEEE TPDS | Anderson's array-based lock in spinlock/anderson.h |
| Michael & Scott (1996). "Simple, Fast, and Practical Non-Blocking and Blocking Concurrent Queue Algorithms" | PODC '96 | MPMC FIFO queue algorithm in ck_fifo.h |
| Treiber (1986). "Systems Programming: Coping with Parallelism" | IBM Research Report RJ 5118 | Lock-free stack in ck_stack.h |
| Michael (2002). "Safe Memory Reclamation for Dynamic Lock-Free Objects Using Atomic Reads and Writes" | PODC '02 | Hazard pointers in ck_hp.h |
| Fraser (2004). "Practical Lock-Freedom" | PhD Thesis, Cambridge | Epoch-based reclamation concepts in ck_epoch.h |
| Dice, Hendler & Shavit (2006). "Flat Combining and the Synchronization-Parallelism Tradeoff" | SPAA '10 | Lock cohorting concepts in ck_cohort.h |
| Brandenburg & Anderson (2010). "Spin-Based Reader-Writer Synchronization for Multiprocessor Real-Time Systems" | RTSS '10 | Phase-fair lock in ck_pflock.h, task-fair lock in ck_tflock.h |
| Celis (1986). "Robin Hood Hashing" | PhD Thesis, U. Waterloo | Robin-hood hashing in ck_rhs.h |
| Herlihy & Shavit (2008). "The Art of Multiprocessor Programming" | Morgan Kaufmann | General concurrent data structure principles |

## Documentation

| Document | Location | Relevance |
|----------|----------|-----------|
| ck_array(3) | doc/ck_array | API reference for concurrent array |
| ck_bitmap(3) | doc/ck_bitmap | API reference for concurrent bitmap |
| ck_brlock(3) | doc/ck_brlock | API reference for big-reader lock |
| ck_bytelock(3) | doc/ck_bytelock | API reference for byte lock |
| ck_epoch(3) | doc/ck_epoch | API reference for epoch-based reclamation |
| ck_hs(3) | doc/ck_hs | API reference for hash set |
| ck_ht(3) | doc/ck_ht | API reference for hash table |
| ck_rhs(3) | doc/ck_rhs | API reference for robin-hood hash set |
| ck_pr(3) | doc/ck_pr | API reference for atomic primitives |
| ck_ring(3) | doc/ck_ring | API reference for ring buffer |
| ck_rwlock(3) | doc/ck_rwlock | API reference for reader-writer lock |
| ck_sequence(3) | doc/ck_sequence | API reference for sequence counter |
| ck_spinlock(3) | doc/ck_spinlock | API reference for spinlocks |
| ck_stack(3) | doc/ck_stack | API reference for lock-free stack |

## Design Discussions

| Thread/Issue | Date | Relevance |
|--------------|------|-----------|
| concurrencykit.org | Ongoing | Official project website with design philosophy |
| GitHub Issues | Various | Bug reports and feature discussions |

Note: CK is a well-established library with limited public design discussions. Most design rationale is embedded in code comments and the academic papers that inspired each algorithm.

## Excluded Sources

| Source | Reason for Exclusion |
|--------|----------------------|
| Linux kernel documentation | Different implementation context, may cause confusion |
| Generic concurrency textbooks | Too general, not specific to CK's implementation choices |
