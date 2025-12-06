# Module: ck_array — Specification

## Operations

### ck_array_init

**Signature:**
```
ck_array_init(array: Pointer to ck_array, mode: unsigned int, allocator: Pointer to ck_malloc, capacity: unsigned int) → bool
```

**Preconditions:**
- allocator must be valid [INFERRED]

**Postconditions:**
- Array initialized with given capacity [SPECIFIED]
- Returns true on success, false on allocation failure [SPECIFIED]

---

### ck_array_put

**Signature:**
```
ck_array_put(array: Pointer to ck_array, value: void*) → bool
```

**Preconditions:**
- Single writer context [SPECIFIED]

**Postconditions:**
- Value added to pending transaction [SPECIFIED]
- Returns true on success [SPECIFIED]

**Concurrency:**
- Thread Safety: Single writer only [SPECIFIED]
- Progress Guarantee: Blocking [OBSERVED]

---

### ck_array_put_unique

**Signature:**
```
ck_array_put_unique(array: Pointer to ck_array, value: void*) → int
```

**Postconditions:**
- Returns 0 if added successfully [SPECIFIED]
- Returns 1 if already present [SPECIFIED]
- Returns -1 on failure [SPECIFIED]

---

### ck_array_remove

**Signature:**
```
ck_array_remove(array: Pointer to ck_array, value: void*) → bool
```

**Preconditions:**
- Single writer context [SPECIFIED]

**Postconditions:**
- Value removed from pending transaction [SPECIFIED]
- Returns true if found and removed [SPECIFIED]

---

### ck_array_commit

**Signature:**
```
ck_array_commit(array: Pointer to ck_array) → bool
```

**Preconditions:**
- Single writer context [SPECIFIED]

**Postconditions:**
- Pending modifications become visible to readers [SPECIFIED]
- Returns true on success [SPECIFIED]

**Concurrency:**
- Thread Safety: Single writer only [SPECIFIED]
- Memory Ordering: Release semantics [OBSERVED]

---

### ck_array_deinit

**Signature:**
```
ck_array_deinit(array: Pointer to ck_array, defer: bool) → void
```

**Postconditions:**
- Array resources freed [SPECIFIED]
- If defer=true, memory freed via deferred reclamation [INFERRED]

---

### ck_array_length

**Signature:**
```
ck_array_length(array: Pointer to ck_array) → unsigned int
```

**Postconditions:**
- Returns number of committed elements [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Acquire semantics [OBSERVED]
- Progress Guarantee: Wait-free [OBSERVED]

---

### ck_array_buffer

**Signature:**
```
ck_array_buffer(array: Pointer to ck_array, length: Pointer to unsigned int) → void*
```

**Postconditions:**
- Returns pointer to values array [SPECIFIED]
- length populated with committed count [SPECIFIED]

---

### ck_array_initialized

**Signature:**
```
ck_array_initialized(array: Pointer to ck_array) → bool
```

**Postconditions:**
- Returns true if array is initialized [SPECIFIED]

---

### CK_ARRAY_FOREACH

**Signature:**
```
CK_ARRAY_FOREACH(array, iterator, element_ptr) (macro)
```

**Postconditions:**
- Iterates over committed elements [SPECIFIED]
- element_ptr receives each value [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe for readers [SPECIFIED]
- Progress Guarantee: Wait-free [OBSERVED]

---

## Safety Properties

**Snapshot Consistency:** Readers see consistent committed state. [SPECIFIED]

**Isolation:** Uncommitted modifications invisible to readers. [SPECIFIED]

## Liveness Properties

**Reader Progress:** Readers never block on writer. [SPECIFIED]

## Discrepancies

No discrepancies detected.
