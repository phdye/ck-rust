# Module: ck_swlock — Specification

## Operations

### ck_swlock_init

**Signature:**
```
ck_swlock_init(rw: Pointer to ck_swlock) → void
```

**Postconditions:**
- value = 0 [SPECIFIED]

---

### ck_swlock_write_lock

**Signature:**
```
ck_swlock_write_lock(rw: Pointer to ck_swlock) → void
```

**Postconditions:**
- Caller holds write access [SPECIFIED]
- WRITER_BIT set [OBSERVED]
- Readers drained [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Acquire semantics [OBSERVED]
- Progress Guarantee: Blocking [SPECIFIED]

---

### ck_swlock_write_latch

**Signature:**
```
ck_swlock_write_latch(rw: Pointer to ck_swlock) → void
```

**Postconditions:**
- Caller holds exclusive write access [SPECIFIED]
- Both WRITER_BIT and LATCH_BIT set [SPECIFIED]
- No new readers can enter [SPECIFIED]

---

### ck_swlock_write_trylock

**Signature:**
```
ck_swlock_write_trylock(rw: Pointer to ck_swlock) → bool
```

**Postconditions:**
- Returns true if lock acquired and no readers [SPECIFIED]
- Returns false if busy [SPECIFIED]

---

### ck_swlock_write_unlock

**Signature:**
```
ck_swlock_write_unlock(rw: Pointer to ck_swlock) → void
```

**Postconditions:**
- WRITER_BIT and LATCH_BIT cleared [SPECIFIED]
- Readers may enter [SPECIFIED]

---

### ck_swlock_write_unlatch

**Signature:**
```
ck_swlock_write_unlatch(rw: Pointer to ck_swlock) → void
```

**Postconditions:**
- value = 0 [OBSERVED]
- Complete reset [SPECIFIED]

---

### ck_swlock_write_downgrade

**Signature:**
```
ck_swlock_write_downgrade(rw: Pointer to ck_swlock) → void
```

**Postconditions:**
- Writer becomes reader [SPECIFIED]
- Reader count = 1 after downgrade [OBSERVED]

---

### ck_swlock_read_lock

**Signature:**
```
ck_swlock_read_lock(rw: Pointer to ck_swlock) → void
```

**Postconditions:**
- Caller holds shared read access [SPECIFIED]
- Reader count incremented [OBSERVED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Acquire semantics [OBSERVED]
- Progress Guarantee: Blocking [SPECIFIED]

---

### ck_swlock_read_trylock

**Signature:**
```
ck_swlock_read_trylock(rw: Pointer to ck_swlock) → bool
```

**Postconditions:**
- Returns true if read lock acquired [SPECIFIED]
- Returns false if writer present [SPECIFIED]

---

### ck_swlock_read_unlock

**Signature:**
```
ck_swlock_read_unlock(rw: Pointer to ck_swlock) → void
```

**Postconditions:**
- Reader count decremented [SPECIFIED]

---

### ck_swlock_locked / locked_writer / locked_reader

**Signatures:**
```
ck_swlock_locked(rw) → bool
ck_swlock_locked_writer(rw) → bool
ck_swlock_locked_reader(rw) → bool
```

**Postconditions:**
- Return appropriate status [SPECIFIED]

---

## Safety Properties

**Mutual Exclusion:** Writer has exclusive access. [SPECIFIED]

**Reader Sharing:** Multiple readers can hold lock. [SPECIFIED]

**Latch Isolation:** Latched writer blocks new readers. [SPECIFIED]

## Liveness Properties

**Writer Progress:** Writers eventually acquire. [OBSERVED]

**Reader Progress:** Readers eventually acquire when no writer. [OBSERVED]

## Discrepancies

No discrepancies detected.
