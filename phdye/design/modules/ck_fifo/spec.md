# Module: ck_fifo — Specification

## Operations

### ck_fifo_spsc_init

**Signature:**
```
ck_fifo_spsc_init(fifo: Pointer to ck_fifo_spsc, stub: Pointer to ck_fifo_spsc_entry) → void
```

**Postconditions:**
- fifo->head = fifo->tail = stub [OBSERVED]
- stub->next = NULL [OBSERVED]
- Queue is empty [SPECIFIED]

---

### ck_fifo_spsc_deinit

**Signature:**
```
ck_fifo_spsc_deinit(fifo: Pointer to ck_fifo_spsc, garbage: Pointer to Pointer) → void
```

**Postconditions:**
- *garbage points to remaining entries [SPECIFIED]
- Queue invalidated [OBSERVED]

---

### ck_fifo_spsc_enqueue

**Signature:**
```
ck_fifo_spsc_enqueue(fifo: Pointer to ck_fifo_spsc, entry: Pointer to ck_fifo_spsc_entry, value: void*) → void
```

**Preconditions:**
- Called by single producer [SPECIFIED]

**Postconditions:**
- value added to tail of queue [SPECIFIED]
- entry becomes new tail [OBSERVED]

**Concurrency:**
- Thread Safety: Single producer only [SPECIFIED]
- Progress Guarantee: Wait-free [OBSERVED]

---

### ck_fifo_spsc_dequeue

**Signature:**
```
ck_fifo_spsc_dequeue(fifo: Pointer to ck_fifo_spsc, value: Pointer to void*) → bool
```

**Preconditions:**
- Called by single consumer [SPECIFIED]

**Postconditions:**
- Returns true if element dequeued [SPECIFIED]
- Returns false if queue empty [SPECIFIED]
- *value set to dequeued value on success [SPECIFIED]

**Concurrency:**
- Thread Safety: Single consumer only [SPECIFIED]
- Progress Guarantee: Wait-free [OBSERVED]

---

### ck_fifo_spsc_recycle

**Signature:**
```
ck_fifo_spsc_recycle(fifo: Pointer to ck_fifo_spsc) → Pointer to ck_fifo_spsc_entry
```

**Postconditions:**
- Returns recyclable entry if available [SPECIFIED]
- Returns NULL if no entries to recycle [SPECIFIED]

---

### ck_fifo_spsc_isempty

**Signature:**
```
ck_fifo_spsc_isempty(fifo: Pointer to ck_fifo_spsc) → bool
```

**Postconditions:**
- Returns true if queue empty [SPECIFIED]

---

### ck_fifo_mpmc_init

**Signature:**
```
ck_fifo_mpmc_init(fifo: Pointer to ck_fifo_mpmc, stub: Pointer to ck_fifo_mpmc_entry) → void
```

**Postconditions:**
- fifo->head = fifo->tail = stub [OBSERVED]
- Generation counters = NULL (0) [OBSERVED]

---

### ck_fifo_mpmc_deinit

**Signature:**
```
ck_fifo_mpmc_deinit(fifo: Pointer to ck_fifo_mpmc, garbage: Pointer to Pointer) → void
```

**Postconditions:**
- *garbage points to head stub [SPECIFIED]

---

### ck_fifo_mpmc_enqueue

**Signature:**
```
ck_fifo_mpmc_enqueue(fifo: Pointer to ck_fifo_mpmc, entry: Pointer to ck_fifo_mpmc_entry, value: void*) → void
```

**Postconditions:**
- value added to queue [SPECIFIED]
- Operation completes (blocking) [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Lock-free [OBSERVED]

---

### ck_fifo_mpmc_tryenqueue

**Signature:**
```
ck_fifo_mpmc_tryenqueue(fifo: Pointer to ck_fifo_mpmc, entry: Pointer to ck_fifo_mpmc_entry, value: void*) → bool
```

**Postconditions:**
- Returns true if enqueued [SPECIFIED]
- Returns false if contention [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Wait-free (single attempt) [OBSERVED]

---

### ck_fifo_mpmc_dequeue

**Signature:**
```
ck_fifo_mpmc_dequeue(fifo: Pointer to ck_fifo_mpmc, value: Pointer to void*, garbage: Pointer to Pointer) → bool
```

**Postconditions:**
- Returns true if dequeued [SPECIFIED]
- Returns false if empty [SPECIFIED]
- *garbage set to old head for reclamation [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Lock-free [OBSERVED]

---

### ck_fifo_mpmc_trydequeue

**Signature:**
```
ck_fifo_mpmc_trydequeue(fifo: Pointer to ck_fifo_mpmc, value: Pointer to void*, garbage: Pointer to Pointer) → bool
```

**Postconditions:**
- Returns true if dequeued [SPECIFIED]
- Returns false if empty or contention [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Wait-free (single attempt) [OBSERVED]

---

## Safety Properties

**FIFO Ordering:** Elements dequeued in enqueue order. [SPECIFIED]

**Linearizability:** MPMC operations are linearizable. [OBSERVED]

## Liveness Properties

**SPSC Progress:** Single producer and consumer always make progress. [SPECIFIED]

**MPMC Progress:** System-wide progress guaranteed (lock-free). [OBSERVED]

## Discrepancies

No discrepancies detected.
