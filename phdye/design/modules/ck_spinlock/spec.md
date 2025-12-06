# Module: ck_spinlock — Specification

## Operations

### ck_spinlock_*_init

**Signature:**
```
ck_spinlock_fas_init(lock: Pointer to ck_spinlock_fas) → void
ck_spinlock_ticket_init(lock: Pointer to ck_spinlock_ticket) → void
ck_spinlock_mcs_init(queue: Pointer to ck_spinlock_mcs*) → void
```

**Postconditions:**
- Lock initialized to unlocked state [SPECIFIED]

---

### ck_spinlock_*_lock

**Signature:**
```
ck_spinlock_fas_lock(lock: Pointer to ck_spinlock_fas) → void
ck_spinlock_ticket_lock(lock: Pointer to ck_spinlock_ticket) → void
ck_spinlock_mcs_lock(queue: Pointer to ck_spinlock_mcs*, node: Pointer to context) → void
```

**Postconditions:**
- Caller holds exclusive lock [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Acquire semantics [SPECIFIED]
- Progress Guarantee: Blocking [SPECIFIED]

---

### ck_spinlock_*_lock_eb / _lock_pb

**Signature:**
```
ck_spinlock_fas_lock_eb(lock) → void
ck_spinlock_ticket_lock_pb(lock, c) → void
```

**Postconditions:**
- Same as lock, with exponential/proportional backoff [SPECIFIED]

---

### ck_spinlock_*_trylock

**Signature:**
```
ck_spinlock_fas_trylock(lock: Pointer to ck_spinlock_fas) → bool
ck_spinlock_ticket_trylock(lock: Pointer to ck_spinlock_ticket) → bool
ck_spinlock_mcs_trylock(queue, node) → bool
```

**Postconditions:**
- Returns true if lock acquired [SPECIFIED]
- Returns false if lock held by another [SPECIFIED]

---

### ck_spinlock_*_unlock

**Signature:**
```
ck_spinlock_fas_unlock(lock: Pointer to ck_spinlock_fas) → void
ck_spinlock_ticket_unlock(lock: Pointer to ck_spinlock_ticket) → void
ck_spinlock_mcs_unlock(queue, node) → void
```

**Postconditions:**
- Lock released [SPECIFIED]
- Next waiter (if any) can proceed [SPECIFIED]

**Concurrency:**
- Memory Ordering: Release semantics [SPECIFIED]

---

### ck_spinlock_*_locked

**Signature:**
```
ck_spinlock_fas_locked(lock: Pointer to ck_spinlock_fas) → bool
ck_spinlock_ticket_locked(lock: Pointer to ck_spinlock_ticket) → bool
ck_spinlock_mcs_locked(queue) → bool
```

**Postconditions:**
- Returns true if lock is held [SPECIFIED]

---

## Safety Properties

**Mutual Exclusion:** At most one thread holds lock. [SPECIFIED]

## Liveness Properties

**FAS/CAS/DEC:** No fairness guarantee. [OBSERVED]

**Ticket/MCS/CLH/Anderson:** FIFO fairness. [SPECIFIED]

## Discrepancies

No discrepancies detected.
