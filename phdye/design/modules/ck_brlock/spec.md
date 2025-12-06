# Module: ck_brlock — Specification

## Operations

### ck_brlock_init

**Signature:**
```
ck_brlock_init(br: Pointer to ck_brlock) → void
```

**Postconditions:**
- br->readers = NULL [SPECIFIED]
- br->writer = false [SPECIFIED]

---

### ck_brlock_read_register

**Signature:**
```
ck_brlock_read_register(br: Pointer to ck_brlock, reader: Pointer to ck_brlock_reader) → void
```

**Preconditions:**
- reader not already registered [INFERRED]

**Postconditions:**
- reader added to br's reader list [SPECIFIED]
- reader->n_readers = 0 [SPECIFIED]

---

### ck_brlock_read_unregister

**Signature:**
```
ck_brlock_read_unregister(br: Pointer to ck_brlock, reader: Pointer to ck_brlock_reader) → void
```

**Preconditions:**
- reader currently registered [INFERRED]
- reader not holding read lock [INFERRED]

**Postconditions:**
- reader removed from br's reader list [SPECIFIED]

---

### ck_brlock_write_lock

**Signature:**
```
ck_brlock_write_lock(br: Pointer to ck_brlock) → void
```

**Postconditions:**
- Caller holds exclusive write access [SPECIFIED]
- All readers have released [SPECIFIED]

---

### ck_brlock_write_unlock

**Signature:**
```
ck_brlock_write_unlock(br: Pointer to ck_brlock) → void
```

**Postconditions:**
- Write lock released [SPECIFIED]

---

### ck_brlock_write_trylock

**Signature:**
```
ck_brlock_write_trylock(br: Pointer to ck_brlock, factor: unsigned int) → bool
```

**Postconditions:**
- Returns true if lock acquired within factor iterations [SPECIFIED]
- Returns false otherwise [SPECIFIED]

---

### ck_brlock_read_lock

**Signature:**
```
ck_brlock_read_lock(br: Pointer to ck_brlock, reader: Pointer to ck_brlock_reader) → void
```

**Preconditions:**
- reader registered with br [SPECIFIED]

**Postconditions:**
- Caller holds shared read access [SPECIFIED]

---

### ck_brlock_read_trylock

**Signature:**
```
ck_brlock_read_trylock(br: Pointer to ck_brlock, reader: Pointer to ck_brlock_reader, factor: unsigned int) → bool
```

**Postconditions:**
- Returns true if lock acquired within factor iterations [SPECIFIED]

---

### ck_brlock_read_unlock

**Signature:**
```
ck_brlock_read_unlock(reader: Pointer to ck_brlock_reader) → void
```

**Postconditions:**
- Read lock released [SPECIFIED]

---

## Safety Properties

**Mutual Exclusion:** Writer has exclusive access. [SPECIFIED]

**Reader Sharing:** Multiple readers hold lock concurrently. [SPECIFIED]

**Recursive Reads:** Same reader can acquire multiple times. [OBSERVED]

## Liveness Properties

**Writer Progress:** Writer eventually acquires after finite readers. [SPECIFIED]

## Discrepancies

No discrepancies detected.
