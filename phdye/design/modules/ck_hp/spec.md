# Module: ck_hp — Specification

## Operations

### ck_hp_init

**Signature:**
```
ck_hp_init(hp: Pointer to ck_hp, degree: unsigned int, threshold: unsigned int, destroy: ck_hp_destructor_t) → void
```

**Postconditions:**
- hp->degree = degree [SPECIFIED]
- hp->threshold = threshold [SPECIFIED]
- hp->destroy = destroy [SPECIFIED]

---

### ck_hp_set_threshold

**Signature:**
```
ck_hp_set_threshold(hp: Pointer to ck_hp, threshold: unsigned int) → void
```

**Postconditions:**
- hp->threshold updated [SPECIFIED]

---

### ck_hp_register

**Signature:**
```
ck_hp_register(hp: Pointer to ck_hp, record: Pointer to ck_hp_record, pointers: void**) → void
```

**Preconditions:**
- pointers array has degree elements [INFERRED]

**Postconditions:**
- Record linked to global state [SPECIFIED]
- All hazard slots cleared [OBSERVED]

---

### ck_hp_unregister

**Signature:**
```
ck_hp_unregister(record: Pointer to ck_hp_record) → void
```

**Postconditions:**
- Record marked FREE for recycling [SPECIFIED]

---

### ck_hp_recycle

**Signature:**
```
ck_hp_recycle(hp: Pointer to ck_hp) → ck_hp_record*
```

**Postconditions:**
- Returns recycled record or NULL [SPECIFIED]

---

### ck_hp_set

**Signature:**
```
ck_hp_set(record: Pointer to ck_hp_record, index: unsigned int, pointer: void*) → void
```

**Preconditions:**
- index < degree [INFERRED]

**Postconditions:**
- Hazard slot index set to pointer [SPECIFIED]

**Concurrency:**
- Thread Safety: Record owner only [OBSERVED]
- Progress Guarantee: Wait-free [OBSERVED]

---

### ck_hp_set_fence

**Signature:**
```
ck_hp_set_fence(record: Pointer to ck_hp_record, index: unsigned int, pointer: void*) → void
```

**Postconditions:**
- Same as ck_hp_set [SPECIFIED]
- Memory ordering guaranteed [SPECIFIED]

**Concurrency:**
- Thread Safety: Record owner only [OBSERVED]
- Memory Ordering: Store-load serialization [SPECIFIED]
- Progress Guarantee: Wait-free [OBSERVED]

---

### ck_hp_clear

**Signature:**
```
ck_hp_clear(record: Pointer to ck_hp_record) → void
```

**Postconditions:**
- All hazard slots set to NULL [SPECIFIED]

---

### ck_hp_retire

**Signature:**
```
ck_hp_retire(record: Pointer to ck_hp_record, hazard: Pointer to ck_hp_hazard, pointer: void*, data: void*) → void
```

**Postconditions:**
- Object added to pending list [SPECIFIED]
- n_pending incremented [OBSERVED]

---

### ck_hp_free

**Signature:**
```
ck_hp_free(record: Pointer to ck_hp_record, hazard: Pointer to ck_hp_hazard, pointer: void*, data: void*) → void
```

**Postconditions:**
- Object retired [SPECIFIED]
- IF n_pending >= threshold: reclaim attempted [OBSERVED]

---

### ck_hp_reclaim

**Signature:**
```
ck_hp_reclaim(record: Pointer to ck_hp_record) → void
```

**Postconditions:**
- Objects not in any hazard slot are freed [SPECIFIED]
- Destructor called for freed objects [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Lock-free [OBSERVED]

---

### ck_hp_purge

**Signature:**
```
ck_hp_purge(record: Pointer to ck_hp_record) → void
```

**Postconditions:**
- All reclaimable objects freed [SPECIFIED]

---

## Safety Properties

**No Use-After-Free:** Objects not freed while hazard pointer points to them. [SPECIFIED]

**Bounded Memory:** At most O(n_threads * threshold) pending objects. [SPECIFIED]

## Liveness Properties

**Reclamation Progress:** Objects eventually freed if not continuously protected. [SPECIFIED]

## Discrepancies

No discrepancies detected.
