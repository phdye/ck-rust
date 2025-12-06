# Module: ck_bytelock — Specification

## Operations

### ck_bytelock_init

**Signature:**
```
ck_bytelock_init(bytelock: Pointer to ck_bytelock) → void
```

**Postconditions:**
- owner = 0 [SPECIFIED]
- n_readers = 0 [SPECIFIED]
- All reader slots cleared [SPECIFIED]

---

### ck_bytelock_write_lock

**Signature:**
```
ck_bytelock_write_lock(bytelock: Pointer to ck_bytelock, slot: unsigned int) → void
```

**Preconditions:**
- slot must be valid (1 to readers size, or UNSLOTTED) [INFERRED]

**Postconditions:**
- Caller holds exclusive write access [SPECIFIED]
- All readers have drained [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Acquire semantics [OBSERVED]
- Progress Guarantee: Blocking [SPECIFIED]

---

### ck_bytelock_write_unlock

**Signature:**
```
ck_bytelock_write_unlock(bytelock: Pointer to ck_bytelock) → void
```

**Postconditions:**
- Write lock released [SPECIFIED]

---

### ck_bytelock_read_lock

**Signature:**
```
ck_bytelock_read_lock(bytelock: Pointer to ck_bytelock, slot: unsigned int) → void
```

**Preconditions:**
- slot must be valid [INFERRED]

**Postconditions:**
- Caller holds shared read access [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Acquire semantics [OBSERVED]
- Progress Guarantee: Blocking [SPECIFIED]

---

### ck_bytelock_read_unlock

**Signature:**
```
ck_bytelock_read_unlock(bytelock: Pointer to ck_bytelock, slot: unsigned int) → void
```

**Postconditions:**
- Read lock released [SPECIFIED]

---

## Safety Properties

**Mutual Exclusion:** Writer has exclusive access. [SPECIFIED]

**Reader Sharing:** Multiple readers hold lock concurrently. [SPECIFIED]

**Upgrade Support:** Writer can downgrade to reader. [OBSERVED]

## Liveness Properties

**Writer Progress:** Writer eventually acquires after finite readers. [SPECIFIED]

## Discrepancies

No discrepancies detected.
