# Module: ck_hs — Design Decisions

## Decision: Open addressing with linear probing

**Context:**
Need collision resolution strategy for hash table.

**Options Considered:**

1. Chaining with linked lists
   - Pro: Simpler deletion
   - Con: Cache unfriendly, extra allocations

2. Open addressing with linear probing
   - Pro: Cache friendly, no extra allocations
   - Con: Tombstones on deletion

**Decision:** Open addressing with linear probing (option 2)

**Rationale:** Linear probing provides excellent cache locality for lookups. Tombstones are managed with gc/rebuild operations.

**Rationale Source:** Linear probe pattern in source

**Consequences:**
- Cache-efficient lookups
- Tombstones accumulate on deletes
- Periodic gc/rebuild needed

---

## Decision: Pointer packing for hash bits

**Context:**
Need to compare hash values efficiently.

**Options Considered:**

1. Store full hash separately
   - Pro: Simple
   - Con: More memory, worse cache

2. Pack hash bits in pointer high bits
   - Pro: Single-word comparison
   - Con: Platform-specific

**Decision:** Pointer packing when available (option 2)

**Rationale:** On 64-bit platforms, virtual addresses don't use all bits. High bits can store hash prefix for fast mismatch detection.

**Rationale Source:** CK_HS_PP, CK_HS_KEY_MASK macros

**Consequences:**
- Faster rejection of mismatches
- Platform-specific optimization
- Falls back to full comparison otherwise

---

## Decision: Separate put and put_unique

**Context:**
Insertion may or may not need uniqueness check.

**Options Considered:**

1. Single put function
   - Pro: Simple API
   - Con: Always checks uniqueness

2. Separate put and put_unique
   - Pro: Optimized path when uniqueness known
   - Con: More API surface

**Decision:** Separate functions (option 2)

**Rationale:** When caller guarantees uniqueness (e.g., during rebuild), skipping the uniqueness check improves performance.

**Rationale Source:** ck_hs_put vs ck_hs_put_unique

**Consequences:**
- put: checks for duplicates
- put_unique: assumes caller's guarantee
- Faster bulk insertion

---

## Decision: Mode flags for behavior customization

**Context:**
Different use cases need different optimizations.

**Options Considered:**

1. Single fixed behavior
   - Pro: Simple
   - Con: Suboptimal for some workloads

2. Mode flags at init
   - Pro: Tunable behavior
   - Con: More complex

**Decision:** Mode flags (option 2)

**Rationale:** SPMC vs MPMC, pointer vs direct values, delete-heavy vs delete-light workloads have different optimal strategies.

**Rationale Source:** CK_HS_MODE_* flags

**Consequences:**
- CK_HS_MODE_SPMC: Read-optimized
- CK_HS_MODE_DIRECT: Integer values
- CK_HS_MODE_OBJECT: Pointer values with packing
- CK_HS_MODE_DELETE: Optimized tombstone handling

---

## Decision: User-provided hash and compare functions

**Context:**
Hash set needs to hash and compare keys.

**Options Considered:**

1. Built-in hash function
   - Pro: Simple
   - Con: Not optimal for all key types

2. User-provided callbacks
   - Pro: Optimal for any key type
   - Con: User must implement

**Decision:** User-provided callbacks (option 2)

**Rationale:** Different applications have different key types (strings, structs, integers). User-provided callbacks enable optimal hashing and comparison.

**Rationale Source:** hf and compare callbacks in ck_hs_init

**Consequences:**
- Flexible key types
- User controls hash quality
- Seed parameter for hash randomization
