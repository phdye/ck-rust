# Module: ck_hs — Specification

## Operations

### ck_hs_init

**Signature:**
```
ck_hs_init(hs: Pointer to ck_hs, mode: unsigned int, hf: hash_cb, compare: compare_cb, m: allocator, capacity: unsigned long, seed: unsigned long) → bool
```

**Postconditions:**
- Hash set initialized with mode, callbacks, allocator [SPECIFIED]
- Returns true on success [SPECIFIED]

---

### ck_hs_destroy

**Signature:**
```
ck_hs_destroy(hs: Pointer to ck_hs) → void
```

**Postconditions:**
- All resources freed [SPECIFIED]

---

### ck_hs_get

**Signature:**
```
ck_hs_get(hs: Pointer to ck_hs, hash: unsigned long, key: const void*) → void*
```

**Postconditions:**
- Returns matching element or NULL [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Wait-free [OBSERVED]

---

### ck_hs_put

**Signature:**
```
ck_hs_put(hs: Pointer to ck_hs, hash: unsigned long, key: const void*) → bool
```

**Preconditions:**
- Single writer context [SPECIFIED]

**Postconditions:**
- Returns true if inserted [SPECIFIED]
- Returns false if already present [SPECIFIED]

---

### ck_hs_put_unique

**Signature:**
```
ck_hs_put_unique(hs: Pointer to ck_hs, hash: unsigned long, key: const void*) → bool
```

**Preconditions:**
- Key not already present [SPECIFIED]

**Postconditions:**
- Returns true if inserted [SPECIFIED]

---

### ck_hs_set

**Signature:**
```
ck_hs_set(hs: Pointer to ck_hs, hash: unsigned long, key: const void*, previous: void**) → bool
```

**Postconditions:**
- Element inserted or replaced [SPECIFIED]
- previous set to old value or NULL [SPECIFIED]

---

### ck_hs_fas

**Signature:**
```
ck_hs_fas(hs: Pointer to ck_hs, hash: unsigned long, key: const void*, previous: void**) → bool
```

**Postconditions:**
- IF key exists: replaced, previous set to old [SPECIFIED]
- IF key not exists: returns false [SPECIFIED]

---

### ck_hs_remove

**Signature:**
```
ck_hs_remove(hs: Pointer to ck_hs, hash: unsigned long, key: const void*) → void*
```

**Postconditions:**
- Returns removed element or NULL [SPECIFIED]

---

### ck_hs_grow

**Signature:**
```
ck_hs_grow(hs: Pointer to ck_hs, capacity: unsigned long) → bool
```

**Postconditions:**
- Capacity increased to at least given value [SPECIFIED]

---

### ck_hs_rebuild

**Signature:**
```
ck_hs_rebuild(hs: Pointer to ck_hs) → bool
```

**Postconditions:**
- Map rebuilt (clears tombstones) [SPECIFIED]

---

### ck_hs_gc

**Signature:**
```
ck_hs_gc(hs: Pointer to ck_hs, cycles: unsigned long, seed: unsigned long) → bool
```

**Postconditions:**
- Tombstones collected [SPECIFIED]

---

### ck_hs_count

**Signature:**
```
ck_hs_count(hs: Pointer to ck_hs) → unsigned long
```

**Postconditions:**
- Returns number of elements [SPECIFIED]

---

### ck_hs_reset

**Signature:**
```
ck_hs_reset(hs: Pointer to ck_hs) → bool
```

**Postconditions:**
- All elements removed [SPECIFIED]

---

### ck_hs_stat

**Signature:**
```
ck_hs_stat(hs: Pointer to ck_hs, stat: Pointer to ck_hs_stat) → void
```

**Postconditions:**
- stat populated with tombstones, n_entries, probe_maximum [SPECIFIED]

---

### ck_hs_next / ck_hs_next_spmc

**Signature:**
```
ck_hs_next(hs: Pointer to ck_hs, iter: Pointer to ck_hs_iterator, key: void**) → bool
```

**Postconditions:**
- Returns next element or false if done [SPECIFIED]

---

### ck_hs_apply

**Signature:**
```
ck_hs_apply(hs: Pointer to ck_hs, hash: unsigned long, key: const void*, fn: apply_fn, context: void*) → bool
```

**Postconditions:**
- fn called on matching entry [SPECIFIED]
- Entry may be updated/removed by fn [SPECIFIED]

---

## Safety Properties

**Snapshot Consistency:** Readers see consistent state. [SPECIFIED]

## Liveness Properties

**Reader Progress:** Readers never block. [SPECIFIED]

## Discrepancies

No discrepancies detected.
