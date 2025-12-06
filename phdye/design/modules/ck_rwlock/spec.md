# Module: ck_rwlock — Specification

## Operations

### ck_rwlock_init

**Signature:**
```
ck_rwlock_init(rw: Pointer to ck_rwlock) → void
```

**Postconditions:**
- writer = 0 [SPECIFIED]
- n_readers = 0 [SPECIFIED]

---

### ck_rwlock_write_lock

**Signature:**
```
ck_rwlock_write_lock(rw: Pointer to ck_rwlock) → void
```

**Postconditions:**
- Caller holds exclusive write access [SPECIFIED]
- All readers have exited [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Acquire semantics [OBSERVED]
- Progress Guarantee: Blocking [SPECIFIED]

---

### ck_rwlock_write_trylock

**Signature:**
```
ck_rwlock_write_trylock(rw: Pointer to ck_rwlock) → bool
```

**Postconditions:**
- Returns true if write lock acquired [SPECIFIED]
- Returns false if lock unavailable [SPECIFIED]

---

### ck_rwlock_write_unlock

**Signature:**
```
ck_rwlock_write_unlock(rw: Pointer to ck_rwlock) → void
```

**Postconditions:**
- Write lock released [SPECIFIED]

**Concurrency:**
- Memory Ordering: Release semantics [OBSERVED]

---

### ck_rwlock_write_downgrade

**Signature:**
```
ck_rwlock_write_downgrade(rw: Pointer to ck_rwlock) → void
```

**Preconditions:**
- Caller holds write lock [INFERRED]

**Postconditions:**
- Caller holds read lock [SPECIFIED]
- Other readers may now acquire [SPECIFIED]

---

### ck_rwlock_read_lock

**Signature:**
```
ck_rwlock_read_lock(rw: Pointer to ck_rwlock) → void
```

**Postconditions:**
- Caller holds shared read access [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Acquire semantics [OBSERVED]
- Progress Guarantee: Blocking [SPECIFIED]

---

### ck_rwlock_read_trylock

**Signature:**
```
ck_rwlock_read_trylock(rw: Pointer to ck_rwlock) → bool
```

**Postconditions:**
- Returns true if read lock acquired [SPECIFIED]
- Returns false if writer active [SPECIFIED]

---

### ck_rwlock_read_unlock

**Signature:**
```
ck_rwlock_read_unlock(rw: Pointer to ck_rwlock) → void
```

**Postconditions:**
- Read lock released [SPECIFIED]

---

### ck_rwlock_locked

**Signature:**
```
ck_rwlock_locked(rw: Pointer to ck_rwlock) → bool
```

**Postconditions:**
- Returns true if any lock held (reader or writer) [SPECIFIED]

---

### ck_rwlock_locked_writer / ck_rwlock_locked_reader

**Signature:**
```
ck_rwlock_locked_writer(rw) → bool
ck_rwlock_locked_reader(rw) → bool
```

**Postconditions:**
- Returns true if writer/reader(s) hold lock [SPECIFIED]

---

### Recursive Writer Operations

**Signatures:**
```
ck_rwlock_recursive_write_lock(rw, tid) → void
ck_rwlock_recursive_write_trylock(rw, tid) → bool
ck_rwlock_recursive_write_unlock(rw) → void
ck_rwlock_recursive_read_lock(rw) → void
ck_rwlock_recursive_read_trylock(rw) → bool
ck_rwlock_recursive_read_unlock(rw) → void
```

**Postconditions:**
- Writer can re-acquire if already owner [SPECIFIED]
- wc tracks recursion depth [OBSERVED]

---

## Safety Properties

**Mutual Exclusion:** Writer has exclusive access. [SPECIFIED]

**Reader Sharing:** Multiple readers hold lock concurrently. [SPECIFIED]

**No Read-Write Concurrency:** Writer waits for readers to exit. [SPECIFIED]

## Liveness Properties

**Writer Progress:** Writers eventually acquire when readers exit. [OBSERVED]

**Reader Progress:** Readers eventually acquire when no writer. [OBSERVED]

## Discrepancies

No discrepancies detected.
