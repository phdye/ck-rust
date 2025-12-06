# Module: ck_rhs — Specification

## Operations

### ck_rhs_init

**Signature:**
```
ck_rhs_init(rhs: Pointer to ck_rhs, mode: unsigned int, hf: hash_cb, compare: compare_cb, m: allocator, capacity: unsigned long, seed: unsigned long) → bool
```

**Postconditions:**
- Hash set initialized [SPECIFIED]
- Returns true on success [SPECIFIED]

---

### ck_rhs_destroy

**Signature:**
```
ck_rhs_destroy(rhs: Pointer to ck_rhs) → void
```

**Postconditions:**
- All resources freed [SPECIFIED]

---

### ck_rhs_get

**Signature:**
```
ck_rhs_get(rhs: Pointer to ck_rhs, hash: unsigned long, key: const void*) → void*
```

**Postconditions:**
- Returns matching element or NULL [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Wait-free [OBSERVED]

---

### ck_rhs_put

**Signature:**
```
ck_rhs_put(rhs: Pointer to ck_rhs, hash: unsigned long, key: const void*) → bool
```

**Postconditions:**
- Returns true if inserted [SPECIFIED]
- Returns false if duplicate [SPECIFIED]
- Robin Hood displacement applied [OBSERVED]

---

### ck_rhs_put_unique

**Signature:**
```
ck_rhs_put_unique(rhs: Pointer to ck_rhs, hash: unsigned long, key: const void*) → bool
```

**Preconditions:**
- Key not already present [SPECIFIED]

**Postconditions:**
- Returns true on success [SPECIFIED]

---

### ck_rhs_set

**Signature:**
```
ck_rhs_set(rhs: Pointer to ck_rhs, hash: unsigned long, key: const void*, previous: void**) → bool
```

**Postconditions:**
- Element inserted or replaced [SPECIFIED]
- previous set to old value or NULL [SPECIFIED]

---

### ck_rhs_fas

**Signature:**
```
ck_rhs_fas(rhs: Pointer to ck_rhs, hash: unsigned long, key: const void*, previous: void**) → bool
```

**Postconditions:**
- IF exists: replaced, previous = old [SPECIFIED]
- IF not exists: returns false [SPECIFIED]

---

### ck_rhs_remove

**Signature:**
```
ck_rhs_remove(rhs: Pointer to ck_rhs, hash: unsigned long, key: const void*) → void*
```

**Postconditions:**
- Returns removed element or NULL [SPECIFIED]

---

### ck_rhs_grow

**Signature:**
```
ck_rhs_grow(rhs: Pointer to ck_rhs, capacity: unsigned long) → bool
```

**Postconditions:**
- Capacity increased [SPECIFIED]

---

### ck_rhs_rebuild

**Signature:**
```
ck_rhs_rebuild(rhs: Pointer to ck_rhs) → bool
```

**Postconditions:**
- Map rebuilt, tombstones cleared [SPECIFIED]

---

### ck_rhs_gc

**Signature:**
```
ck_rhs_gc(rhs: Pointer to ck_rhs) → bool
```

**Postconditions:**
- Garbage collection performed [SPECIFIED]

---

### ck_rhs_count

**Signature:**
```
ck_rhs_count(rhs: Pointer to ck_rhs) → unsigned long
```

**Postconditions:**
- Returns element count [SPECIFIED]

---

### ck_rhs_reset / ck_rhs_reset_size

**Signature:**
```
ck_rhs_reset(rhs: Pointer to ck_rhs) → bool
ck_rhs_reset_size(rhs: Pointer to ck_rhs, size: unsigned long) → bool
```

**Postconditions:**
- All elements removed [SPECIFIED]

---

### ck_rhs_stat

**Signature:**
```
ck_rhs_stat(rhs: Pointer to ck_rhs, stat: Pointer to ck_rhs_stat) → void
```

**Postconditions:**
- stat populated [SPECIFIED]

---

### ck_rhs_set_load_factor

**Signature:**
```
ck_rhs_set_load_factor(rhs: Pointer to ck_rhs, load_factor: unsigned int) → bool
```

**Postconditions:**
- Load factor updated [SPECIFIED]

---

### ck_rhs_apply

**Signature:**
```
ck_rhs_apply(rhs: Pointer to ck_rhs, hash: unsigned long, key: const void*, fn: apply_fn, context: void*) → bool
```

**Postconditions:**
- fn called on matching entry [SPECIFIED]

---

### ck_rhs_next

**Signature:**
```
ck_rhs_next(rhs: Pointer to ck_rhs, iter: Pointer to ck_rhs_iterator, key: void**) → bool
```

**Postconditions:**
- Returns next element [SPECIFIED]

---

## Safety Properties

**Robin Hood Invariant:** Probe distances increase monotonically. [OBSERVED]

**Snapshot Consistency:** Readers see consistent state. [SPECIFIED]

## Liveness Properties

**Bounded Probing:** Maximum probe length is bounded by load factor. [OBSERVED]

## Discrepancies

No discrepancies detected.
