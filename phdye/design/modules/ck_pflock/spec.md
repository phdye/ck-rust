# Module: ck_pflock — Specification

## Operations

### ck_pflock_init

**Signature:**
```
ck_pflock_init(pf: Pointer to ck_pflock) → void
```

**Preconditions:**
- pf must not be NULL [INFERRED]

**Postconditions:**
- All fields set to 0 [SPECIFIED]
- Lock is in unlocked state [SPECIFIED]

**Concurrency:**
- Thread Safety: Not safe during concurrent access [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_pflock_write_lock

**Signature:**
```
ck_pflock_write_lock(pf: Pointer to ck_pflock) → void
```

**Preconditions:**
- pf must not be NULL [INFERRED]
- Caller must not already hold the lock [INFERRED]

**Postconditions:**
- Caller holds exclusive write access [SPECIFIED]
- All readers have drained [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Acquire semantics [OBSERVED]
- Progress Guarantee: Blocking [SPECIFIED]

---

### ck_pflock_write_unlock

**Signature:**
```
ck_pflock_write_unlock(pf: Pointer to ck_pflock) → void
```

**Preconditions:**
- Caller must hold write lock [INFERRED]

**Postconditions:**
- Lock is released [SPECIFIED]
- Next writer or readers may proceed [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Release semantics [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_pflock_read_lock

**Signature:**
```
ck_pflock_read_lock(pf: Pointer to ck_pflock) → void
```

**Preconditions:**
- pf must not be NULL [INFERRED]

**Postconditions:**
- Caller holds shared read access [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Acquire semantics [OBSERVED]
- Progress Guarantee: Blocking if writer active [SPECIFIED]

---

### ck_pflock_read_unlock

**Signature:**
```
ck_pflock_read_unlock(pf: Pointer to ck_pflock) → void
```

**Preconditions:**
- Caller must hold read lock [INFERRED]

**Postconditions:**
- Read lock released [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Release semantics [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

## Data Structure Invariants

- Reader count in rin balanced by rout [SPECIFIED]
- Writer tickets win and wout maintain FIFO order [OBSERVED]

## Safety Properties

**Mutual Exclusion:** Writer has exclusive access. [SPECIFIED]

**Reader Sharing:** Multiple readers may hold read lock concurrently. [SPECIFIED]

## Liveness Properties

**No Starvation:** Both readers and writers eventually acquire lock. [SPECIFIED]

**Phase Fairness:** Readers present before write request complete first. [SPECIFIED]

## Discrepancies

No discrepancies detected.
