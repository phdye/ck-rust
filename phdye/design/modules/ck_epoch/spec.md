# Module: ck_epoch — Specification

## Operations

### ck_epoch_init

**Signature:**
```
ck_epoch_init(epoch: Pointer to ck_epoch) → void
```

**Postconditions:**
- epoch->epoch = 0 [SPECIFIED]
- Records stack initialized [SPECIFIED]

---

### ck_epoch_register

**Signature:**
```
ck_epoch_register(epoch: Pointer to ck_epoch, record: Pointer to ck_epoch_record, context: void*) → void
```

**Postconditions:**
- Record linked to global state [SPECIFIED]
- record->ct = context [SPECIFIED]
- record->state = USED [OBSERVED]

---

### ck_epoch_unregister

**Signature:**
```
ck_epoch_unregister(record: Pointer to ck_epoch_record) → void
```

**Preconditions:**
- Record must have no pending callbacks [INFERRED]

**Postconditions:**
- Record marked as FREE for recycling [SPECIFIED]

---

### ck_epoch_recycle

**Signature:**
```
ck_epoch_recycle(epoch: Pointer to ck_epoch, context: void*) → ck_epoch_record*
```

**Postconditions:**
- Returns recycled record or NULL [SPECIFIED]
- IF returned: record->ct = context [SPECIFIED]

---

### ck_epoch_begin

**Signature:**
```
ck_epoch_begin(record: Pointer to ck_epoch_record, section: Pointer to ck_epoch_section) → void
```

**Postconditions:**
- Thread enters epoch-protected section [SPECIFIED]
- record->active incremented [SPECIFIED]
- IF first entry: record->epoch updated [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Store-load serialization [SPECIFIED]
- Progress Guarantee: Wait-free [OBSERVED]

---

### ck_epoch_end

**Signature:**
```
ck_epoch_end(record: Pointer to ck_epoch_record, section: Pointer to ck_epoch_section) → bool
```

**Postconditions:**
- record->active decremented [SPECIFIED]
- Returns true if no more active sections [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Release fence [OBSERVED]
- Progress Guarantee: Wait-free [OBSERVED]

---

### ck_epoch_call

**Signature:**
```
ck_epoch_call(record: Pointer to ck_epoch_record, entry: Pointer to ck_epoch_entry, function: ck_epoch_cb_t*) → void
```

**Preconditions:**
- entry must be embedded in object to free [SPECIFIED]

**Postconditions:**
- Callback deferred until safe [SPECIFIED]
- record->n_pending incremented [OBSERVED]

**Concurrency:**
- Thread Safety: Single record owner [OBSERVED]
- Progress Guarantee: Wait-free [OBSERVED]

---

### ck_epoch_call_strict

**Signature:**
```
ck_epoch_call_strict(record: Pointer to ck_epoch_record, entry: Pointer to ck_epoch_entry, function: ck_epoch_cb_t*) → void
```

**Postconditions:**
- Same as ck_epoch_call [SPECIFIED]
- Safe for shared records [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe for shared records [SPECIFIED]
- Progress Guarantee: Lock-free [OBSERVED]

---

### ck_epoch_poll

**Signature:**
```
ck_epoch_poll(record: Pointer to ck_epoch_record) → bool
```

**Postconditions:**
- Attempts to advance epoch [SPECIFIED]
- Dispatches safe callbacks [SPECIFIED]
- Returns true if dispatch occurred [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Lock-free [OBSERVED]

---

### ck_epoch_synchronize

**Signature:**
```
ck_epoch_synchronize(record: Pointer to ck_epoch_record) → void
```

**Postconditions:**
- Blocks until epoch advances [SPECIFIED]
- All readers at call time have exited [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Blocking [SPECIFIED]

---

### ck_epoch_barrier

**Signature:**
```
ck_epoch_barrier(record: Pointer to ck_epoch_record) → void
```

**Postconditions:**
- All pending callbacks dispatched [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Blocking [SPECIFIED]

---

### ck_epoch_value

**Signature:**
```
ck_epoch_value(epoch: Pointer to const ck_epoch) → unsigned int
```

**Postconditions:**
- Returns current global epoch [SPECIFIED]

**Concurrency:**
- Memory Ordering: Load fence [OBSERVED]
- Progress Guarantee: Wait-free [OBSERVED]

---

## Safety Properties

**Deferred Safety:** Callbacks not invoked until all threads observing old epoch exit. [SPECIFIED]

**Memory Safety:** Objects freed only when unreachable. [SPECIFIED]

## Liveness Properties

**Epoch Progress:** Epoch advances when all threads participate. [SPECIFIED]

**Callback Dispatch:** Callbacks eventually dispatched if threads poll. [SPECIFIED]

## Discrepancies

No discrepancies detected.
