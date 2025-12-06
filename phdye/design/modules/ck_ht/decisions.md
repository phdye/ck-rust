# Module: ck_ht — Design Decisions

## Decision: Entry-based API

**Context:**
Hash table needs to pass key-value pairs efficiently.

**Options Considered:**

1. Separate key and value parameters
   - Pro: Simple calls
   - Con: Many parameters, can't return both

2. Entry structure for key-value
   - Pro: Single parameter, returns both
   - Con: Entry setup required

**Decision:** Entry-based API (option 2)

**Rationale:** ck_ht_entry holds key, value, and hash metadata. Single entry parameter for put/get/remove simplifies the API and allows returning removed key-value pairs.

**Rationale Source:** ck_ht_entry used in all operations

**Consequences:**
- Caller manages entry struct
- get populates entry with found value
- remove returns key-value in entry

---

## Decision: Pointer packing with alignment

**Context:**
Entries need key, value, length, and hash.

**Options Considered:**

1. Always store all fields
   - Pro: Simple, portable
   - Con: Larger entries (32 bytes)

2. Pack fields into pointers when possible
   - Pro: Smaller entries (16 bytes)
   - Con: Platform-specific

**Decision:** Conditional pointer packing (option 2)

**Rationale:** On 64-bit platforms with limited VMA bits, high bits are unused. Pack key_length and hash into those bits for 16-byte entries.

**Rationale Source:** CK_HT_PP, CK_CC_ALIGN macros

**Consequences:**
- 16-byte entries on supported platforms
- 32-byte entries otherwise
- Better cache efficiency when packed

---

## Decision: Separate hash and hash_direct

**Context:**
Different key types need different hashing.

**Options Considered:**

1. Single hash function
   - Pro: Simple API
   - Con: Overhead for integer keys

2. Separate hash and hash_direct
   - Pro: Optimal for each key type
   - Con: More functions

**Decision:** Separate functions (option 2)

**Rationale:** Direct integer keys don't need length-based hashing. hash_direct is optimized for uintptr_t keys.

**Rationale Source:** ck_ht_hash vs ck_ht_hash_direct

**Consequences:**
- hash: for byte-string keys
- hash_direct: for integer keys
- Matching entry_set and entry_key functions

---

## Decision: Inline entry helpers

**Context:**
Entry field access needs to handle packing.

**Options Considered:**

1. Direct field access
   - Pro: Simple
   - Con: Exposes packing details

2. Inline helper functions
   - Pro: Hides packing
   - Con: More API surface

**Decision:** Inline helper functions (option 2)

**Rationale:** Helpers like ck_ht_entry_key, ck_ht_entry_value abstract away pointer packing. User code doesn't change based on platform.

**Rationale Source:** Inline functions for entry access

**Consequences:**
- Portable user code
- Zero overhead (inlined)
- Consistent API

---

## Decision: _spmc suffix for concurrent operations

**Context:**
Need to distinguish concurrent vs single-threaded ops.

**Options Considered:**

1. All ops thread-safe
   - Pro: Simple naming
   - Con: Overhead when not needed

2. Explicit _spmc suffix
   - Pro: Clear which ops are concurrent
   - Con: Longer names

**Decision:** _spmc suffix (option 2)

**Rationale:** Makes it explicit which operations are safe for concurrent readers. Non-suffixed operations (next) require exclusive access.

**Rationale Source:** get_spmc, put_spmc, etc.

**Consequences:**
- Clear concurrency semantics
- next requires no concurrent mutations
- Matches ck_hs pattern
