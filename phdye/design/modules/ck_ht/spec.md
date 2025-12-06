# Module: ck_ht — Specification

## Operations

### ck_ht_init

**Signature:**
```
ck_ht_init(ht: Pointer to ck_ht, mode: unsigned int, h: hash_cb, m: allocator, capacity: CK_HT_TYPE, seed: uint64_t) → bool
```

**Postconditions:**
- Hash table initialized [SPECIFIED]
- Returns true on success [SPECIFIED]

---

### ck_ht_destroy

**Signature:**
```
ck_ht_destroy(ht: Pointer to ck_ht) → void
```

**Postconditions:**
- All resources freed [SPECIFIED]

---

### ck_ht_hash

**Signature:**
```
ck_ht_hash(h: Pointer to ck_ht_hash, ht: Pointer to ck_ht, key: const void*, key_length: uint16_t) → void
```

**Postconditions:**
- h->value contains hash [SPECIFIED]

---

### ck_ht_hash_direct

**Signature:**
```
ck_ht_hash_direct(h: Pointer to ck_ht_hash, ht: Pointer to ck_ht, key: uintptr_t) → void
```

**Postconditions:**
- h->value contains hash for direct key [SPECIFIED]

---

### ck_ht_get_spmc

**Signature:**
```
ck_ht_get_spmc(ht: Pointer to ck_ht, h: ck_ht_hash, entry: Pointer to ck_ht_entry) → bool
```

**Preconditions:**
- entry->key set to lookup key [SPECIFIED]

**Postconditions:**
- IF found: returns true, entry populated with value [SPECIFIED]
- IF not found: returns false [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Wait-free [OBSERVED]

---

### ck_ht_put_spmc

**Signature:**
```
ck_ht_put_spmc(ht: Pointer to ck_ht, h: ck_ht_hash, entry: Pointer to ck_ht_entry) → bool
```

**Preconditions:**
- Single writer context [SPECIFIED]

**Postconditions:**
- Returns true if inserted [SPECIFIED]
- Returns false if key exists [SPECIFIED]

---

### ck_ht_set_spmc

**Signature:**
```
ck_ht_set_spmc(ht: Pointer to ck_ht, h: ck_ht_hash, entry: Pointer to ck_ht_entry) → bool
```

**Postconditions:**
- Entry inserted or value replaced [SPECIFIED]
- Returns true on success [SPECIFIED]

---

### ck_ht_remove_spmc

**Signature:**
```
ck_ht_remove_spmc(ht: Pointer to ck_ht, h: ck_ht_hash, entry: Pointer to ck_ht_entry) → bool
```

**Postconditions:**
- IF found: returns true, entry has removed key-value [SPECIFIED]
- IF not found: returns false [SPECIFIED]

---

### ck_ht_grow_spmc

**Signature:**
```
ck_ht_grow_spmc(ht: Pointer to ck_ht, capacity: CK_HT_TYPE) → bool
```

**Postconditions:**
- Capacity increased [SPECIFIED]
- All entries preserved [SPECIFIED]

---

### ck_ht_reset_spmc / ck_ht_reset_size_spmc

**Signature:**
```
ck_ht_reset_spmc(ht: Pointer to ck_ht) → bool
ck_ht_reset_size_spmc(ht: Pointer to ck_ht, size: CK_HT_TYPE) → bool
```

**Postconditions:**
- All entries removed [SPECIFIED]
- Optionally resized [SPECIFIED]

---

### ck_ht_gc

**Signature:**
```
ck_ht_gc(ht: Pointer to ck_ht, cycles: unsigned long, seed: unsigned long) → bool
```

**Postconditions:**
- Tombstones collected [SPECIFIED]

---

### ck_ht_count

**Signature:**
```
ck_ht_count(ht: Pointer to ck_ht) → CK_HT_TYPE
```

**Postconditions:**
- Returns number of entries [SPECIFIED]

---

### ck_ht_stat

**Signature:**
```
ck_ht_stat(ht: Pointer to ck_ht, stat: Pointer to ck_ht_stat) → void
```

**Postconditions:**
- stat populated with statistics [SPECIFIED]

---

### ck_ht_next

**Signature:**
```
ck_ht_next(ht: Pointer to ck_ht, iter: Pointer to ck_ht_iterator, entry: Pointer to ck_ht_entry*) → bool
```

**Preconditions:**
- No concurrent mutations [SPECIFIED]

**Postconditions:**
- Returns next entry or false if done [SPECIFIED]

---

### Entry Helper Functions

**Signatures:**
```
ck_ht_entry_empty(entry) → bool
ck_ht_entry_key(entry) → void*
ck_ht_entry_key_direct(entry) → uintptr_t
ck_ht_entry_key_length(entry) → uint16_t
ck_ht_entry_value(entry) → void*
ck_ht_entry_value_direct(entry) → uintptr_t
ck_ht_entry_set(entry, h, key, key_length, value) → void
ck_ht_entry_set_direct(entry, h, key, value) → void
ck_ht_entry_key_set(entry, key, key_length) → void
ck_ht_entry_key_set_direct(entry, key) → void
```

---

## Safety Properties

**Snapshot Consistency:** Readers see consistent entries. [SPECIFIED]

## Liveness Properties

**Reader Progress:** Readers never block. [SPECIFIED]

## Discrepancies

No discrepancies detected.
