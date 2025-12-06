# Module: ck_tflock — Specification

## Operations

### ck_tflock_ticket_init

**Signature:**
```
ck_tflock_ticket_init(lock: Pointer to ck_tflock_ticket) → void
```

**Preconditions:**
- lock must not be NULL [INFERRED]

**Postconditions:**
- request = completion = 0 [SPECIFIED]
- Lock is unlocked [SPECIFIED]

---

### ck_tflock_ticket_write_lock

**Signature:**
```
ck_tflock_ticket_write_lock(lock: Pointer to ck_tflock_ticket) → void
```

**Preconditions:**
- lock must not be NULL [INFERRED]

**Postconditions:**
- Caller holds exclusive write access [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Acquire semantics [OBSERVED]
- Progress Guarantee: Blocking [SPECIFIED]

---

### ck_tflock_ticket_write_unlock

**Signature:**
```
ck_tflock_ticket_write_unlock(lock: Pointer to ck_tflock_ticket) → void
```

**Preconditions:**
- Caller must hold write lock [INFERRED]

**Postconditions:**
- Lock released [SPECIFIED]

---

### ck_tflock_ticket_read_lock

**Signature:**
```
ck_tflock_ticket_read_lock(lock: Pointer to ck_tflock_ticket) → void
```

**Preconditions:**
- lock must not be NULL [INFERRED]

**Postconditions:**
- Caller holds shared read access [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Blocking [SPECIFIED]

---

### ck_tflock_ticket_read_unlock

**Signature:**
```
ck_tflock_ticket_read_unlock(lock: Pointer to ck_tflock_ticket) → void
```

**Preconditions:**
- Caller must hold read lock [INFERRED]

**Postconditions:**
- Read lock released [SPECIFIED]

---

## Safety Properties

**Mutual Exclusion:** Writer has exclusive access. [SPECIFIED]

**FIFO Ordering:** Requests served in arrival order. [SPECIFIED]

## Liveness Properties

**No Starvation:** All requests eventually complete. [SPECIFIED]

**Task Fairness:** Order of acquisition matches order of request. [SPECIFIED]

## Discrepancies

No discrepancies detected.
