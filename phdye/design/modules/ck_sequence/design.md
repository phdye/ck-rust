# Module: ck_sequence

## Overview

The ck_sequence module provides a sequence lock (seqlock) implementation for optimistic read-copy consistency. Readers speculatively read data without acquiring a lock, then verify the data wasn't modified during the read. Writers increment a sequence counter before and after modifications, allowing readers to detect concurrent writes.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Inline hints, likely/unlikely macros |
| ck_pr | Internal | Atomic operations, memory barriers |
| ck_stdbool | External | Boolean type |

## Data Structures

### struct ck_sequence

**Description:** Sequence counter for optimistic read synchronization.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| sequence | unsigned int | 4 bytes | Version counter (even = stable, odd = updating) |

**Invariants:**
- Sequence is even when no write in progress [SPECIFIED]
- Sequence is odd during write [SPECIFIED]
- Sequence increases monotonically [OBSERVED]

## Algorithms

### ck_sequence_init

**Purpose:** Initialize sequence to stable state

**Signature:**
```
ck_sequence_init(sq: Pointer to ck_sequence) → void
```

**Algorithm:**
1. Atomic store 0 to sq->sequence

**Complexity:** O(1)

### ck_sequence_read_begin

**Purpose:** Begin optimistic read, get version for later verification

**Signature:**
```
ck_sequence_read_begin(sq: Pointer to const ck_sequence) → unsigned int
```

**Algorithm:**
1. Loop:
   a. Load sequence atomically
   b. IF (sequence & 1) == 0: break (stable state)
   c. Stall and retry (writer active)
2. Fence load (acquire semantics)
3. Return sequence

**Complexity:** O(1) expected, may spin if writer active

### ck_sequence_read_retry

**Purpose:** Check if read should be retried due to concurrent write

**Signature:**
```
ck_sequence_read_retry(sq: Pointer to const ck_sequence, version: unsigned int) → bool
```

**Algorithm:**
1. Fence load
2. Return sq->sequence != version

**Returns:** true if read must be retried, false if valid

**Complexity:** O(1)

### ck_sequence_write_begin

**Purpose:** Begin write phase (caller must hold mutex)

**Signature:**
```
ck_sequence_write_begin(sq: Pointer to ck_sequence) → void
```

**Algorithm:**
1. Store sequence + 1 (makes odd)
2. Fence store (release semantics)

**Precondition:** Caller holds mutex protecting the data

**Complexity:** O(1)

### ck_sequence_write_end

**Purpose:** End write phase

**Signature:**
```
ck_sequence_write_end(sq: Pointer to ck_sequence) → void
```

**Algorithm:**
1. Fence store
2. Store sequence + 1 (makes even again)

**Postcondition:** Readers can now see consistent data

**Complexity:** O(1)

### CK_SEQUENCE_READ Macro

**Purpose:** Convenient loop for retry pattern

**Usage:**
```c
unsigned int version;
CK_SEQUENCE_READ(&seqlock, &version) {
    // read protected data
}
// version == 0 means read succeeded
```

## Concurrency

**Thread Safety:**
- Readers: Multiple concurrent readers safe [SPECIFIED]
- Writers: Must use external mutex [SPECIFIED]
- Reader vs Writer: Readers detect concurrent writes [SPECIFIED]

**Memory Ordering:**
- read_begin: Acquire semantics after successful load
- read_retry: Acquire fence before check
- write_begin: Release semantics after increment
- write_end: Release semantics before increment

**Progress Guarantee:**
- Readers: Blocking (wait for writer to complete) [OBSERVED]
- Writers: Wait-free (with external mutex held) [OBSERVED]

## Platform Considerations

- Sequence counter overflow is benign (modular arithmetic)
- Works on all platforms with ck_pr support
