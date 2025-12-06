# Module: ck_hp_fifo — Specification

## Operations

### ck_hp_fifo_init

**Signature:**
```
ck_hp_fifo_init(fifo: Pointer to ck_hp_fifo, stub: Pointer to ck_hp_fifo_entry) → void
```

**Postconditions:**
- fifo->head = fifo->tail = stub [OBSERVED]
- stub->next = NULL [OBSERVED]

---

### ck_hp_fifo_deinit

**Signature:**
```
ck_hp_fifo_deinit(fifo: Pointer to ck_hp_fifo, stub: Pointer to Pointer) → void
```

**Postconditions:**
- *stub set to head entry [SPECIFIED]
- fifo->head = fifo->tail = NULL [OBSERVED]

---

### ck_hp_fifo_enqueue_mpmc

**Signature:**
```
ck_hp_fifo_enqueue_mpmc(record: Pointer to ck_hp_record_t, fifo: Pointer to ck_hp_fifo, entry: Pointer to ck_hp_fifo_entry, value: void*) → void
```

**Preconditions:**
- record is valid HP record with at least 2 slots [SPECIFIED]
- entry is unlinked [SPECIFIED]

**Postconditions:**
- value enqueued at tail [SPECIFIED]
- Operation always completes [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Lock-free [OBSERVED]

---

### ck_hp_fifo_tryenqueue_mpmc

**Signature:**
```
ck_hp_fifo_tryenqueue_mpmc(record: Pointer to ck_hp_record_t, fifo: Pointer to ck_hp_fifo, entry: Pointer to ck_hp_fifo_entry, value: void*) → bool
```

**Postconditions:**
- Returns true if enqueue succeeded [SPECIFIED]
- Returns false if contention detected [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Wait-free (single attempt) [OBSERVED]

---

### ck_hp_fifo_dequeue_mpmc

**Signature:**
```
ck_hp_fifo_dequeue_mpmc(record: Pointer to ck_hp_record_t, fifo: Pointer to ck_hp_fifo, value: Pointer to void*) → Pointer to ck_hp_fifo_entry
```

**Postconditions:**
- Returns entry pointer if successful [SPECIFIED]
- Returns NULL if queue empty [SPECIFIED]
- *value set to dequeued value on success [SPECIFIED]
- Returned entry must be reclaimed via hazard pointers [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Lock-free [OBSERVED]

---

### ck_hp_fifo_trydequeue_mpmc

**Signature:**
```
ck_hp_fifo_trydequeue_mpmc(record: Pointer to ck_hp_record_t, fifo: Pointer to ck_hp_fifo, value: Pointer to void*) → Pointer to ck_hp_fifo_entry
```

**Postconditions:**
- Returns entry pointer if successful [SPECIFIED]
- Returns NULL if empty or contention [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Wait-free (single attempt) [OBSERVED]

---

## Macros

### CK_HP_FIFO_ISEMPTY

**Signature:**
```
CK_HP_FIFO_ISEMPTY(f) → bool
```

**Postconditions:**
- Returns true if head->next == NULL [SPECIFIED]

### CK_HP_FIFO_FIRST / CK_HP_FIFO_NEXT

**Purpose:** Iteration support.

### CK_HP_FIFO_FOREACH / CK_HP_FIFO_FOREACH_SAFE

**Purpose:** Safe iteration over entries.

---

## Safety Properties

**FIFO Ordering:** Elements dequeued in enqueue order. [SPECIFIED]

**Safe Reclamation:** Hazard pointers prevent use-after-free. [SPECIFIED]

**Linearizability:** Operations are linearizable. [OBSERVED]

## Liveness Properties

**Lock-Freedom:** At least one operation completes in finite steps. [OBSERVED]

**No ABA:** Hazard pointers prevent ABA problem. [SPECIFIED]

## Discrepancies

No discrepancies detected.
