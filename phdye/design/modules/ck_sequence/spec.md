# Module: ck_sequence — Specification

## Operations

### ck_sequence_init

**Signature:**
```
ck_sequence_init(sq: Pointer to ck_sequence) → void
```

**Preconditions:**
- sq must not be NULL [INFERRED]

**Postconditions:**
- sq->sequence = 0 [SPECIFIED]

**Concurrency:**
- Thread Safety: Not safe during concurrent access [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_sequence_read_begin

**Signature:**
```
ck_sequence_read_begin(sq: Pointer to const ck_sequence) → unsigned int
```

**Preconditions:**
- sq must not be NULL [INFERRED]

**Postconditions:**
- Returns even sequence value [SPECIFIED]
- Acquire fence executed [OBSERVED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Acquire semantics [OBSERVED]
- Progress Guarantee: Blocking (waits for writers) [OBSERVED]

---

### ck_sequence_read_retry

**Signature:**
```
ck_sequence_read_retry(sq: Pointer to const ck_sequence, version: unsigned int) → bool
```

**Preconditions:**
- version from prior read_begin [INFERRED]

**Postconditions:**
- Returns true if sequence changed since read_begin [SPECIFIED]
- Returns false if read was consistent [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Acquire fence before check [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_sequence_write_begin

**Signature:**
```
ck_sequence_write_begin(sq: Pointer to ck_sequence) → void
```

**Preconditions:**
- Caller must hold mutex protecting the data [SPECIFIED]

**Postconditions:**
- sq->sequence is odd [SPECIFIED]
- Release fence executed [OBSERVED]

**Concurrency:**
- Thread Safety: Requires external mutex [SPECIFIED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_sequence_write_end

**Signature:**
```
ck_sequence_write_end(sq: Pointer to ck_sequence) → void
```

**Preconditions:**
- Must follow successful write_begin [INFERRED]
- Caller must still hold mutex [SPECIFIED]

**Postconditions:**
- sq->sequence is even [SPECIFIED]
- Sequence is 2 greater than before write_begin [OBSERVED]

**Concurrency:**
- Thread Safety: Requires external mutex [SPECIFIED]
- Progress Guarantee: wait-free [OBSERVED]

---

## Data Structure Invariants

### ck_sequence

- Even sequence indicates stable state [SPECIFIED]
- Odd sequence indicates write in progress [SPECIFIED]

## Safety Properties

**Consistency Detection:** If a write occurs during a read, read_retry returns true. [SPECIFIED]

**No False Negatives:** If read_retry returns false, the read was consistent. [SPECIFIED]

## Liveness Properties

**Writer Progress:** Writers complete in bounded time (with mutex held). [OBSERVED]

**Reader Progress:** Readers eventually complete if writers are finite. [OBSERVED]

## Behavioral Ambiguities

### Starvation under heavy writes

**Observed Behavior:** Readers may starve if writers continuously hold the lock

**Intent:** OBSERVED - seqlocks favor writers

**Recommendation:** Use for read-heavy workloads. Consider other locks for write-heavy cases.

## Discrepancies

No discrepancies detected between sources.
