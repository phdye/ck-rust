# Module: ck_stack — Specification

## Operations

### ck_stack_init

**Signature:**
```
ck_stack_init(stack: Pointer to ck_stack) → void
```

**Preconditions:**
- stack must not be NULL [INFERRED]

**Postconditions:**
- stack->head = NULL [SPECIFIED]
- stack->generation = NULL [OBSERVED]

**Invariants Preserved:**
- Stack is in valid empty state [SPECIFIED]

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| stack is NULL | Undefined behavior | INFERRED |

**Concurrency:**
- Thread Safety: Not safe to call concurrently with other operations on same stack [OBSERVED]
- Memory Ordering: N/A [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_stack_push_upmc

**Signature:**
```
ck_stack_push_upmc(target: Pointer to ck_stack, entry: Pointer to ck_stack_entry) → void
```

**Preconditions:**
- target must not be NULL [INFERRED]
- entry must not be NULL [INFERRED]
- entry must not already be in any stack [SPECIFIED]
- entry must be unique (not reused without safe memory reclamation) [SPECIFIED]

**Postconditions:**
- entry is now the head of the stack [SPECIFIED]
- Previous head is accessible via entry->next [SPECIFIED]

**Invariants Preserved:**
- Stack is a valid linked list [SPECIFIED]
- All entries are reachable from head [SPECIFIED]

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| entry already in stack | Corrupts stack | SPECIFIED |
| entry reused without reclamation | ABA problem possible | SPECIFIED |

**Concurrency:**
- Thread Safety: Safe with multiple unique producers and multiple consumers [SPECIFIED]
- Memory Ordering: Release semantics (fence_store before CAS) [OBSERVED]
- Progress Guarantee: lock-free [SPECIFIED]

---

### ck_stack_pop_upmc

**Signature:**
```
ck_stack_pop_upmc(target: Pointer to ck_stack) → Pointer to ck_stack_entry (or NULL)
```

**Preconditions:**
- target must not be NULL [INFERRED]

**Postconditions:**
- IF stack was non-empty: returns former head, stack head updated to head->next [SPECIFIED]
- IF stack was empty: returns NULL, stack unchanged [SPECIFIED]

**Invariants Preserved:**
- Stack remains valid linked list [SPECIFIED]

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| Concurrent reuse of entry | ABA problem, may return wrong entry | SPECIFIED |

**Concurrency:**
- Thread Safety: Safe with UPMC constraints [SPECIFIED]
- Memory Ordering: Acquire semantics (fence_load after head read) [OBSERVED]
- Progress Guarantee: lock-free [SPECIFIED]

---

### ck_stack_push_mpmc

**Signature:**
```
ck_stack_push_mpmc(target: Pointer to ck_stack, entry: Pointer to ck_stack_entry) → void
```

**Preconditions:**
- target must not be NULL [INFERRED]
- entry must not be NULL [INFERRED]
- Platform must support CK_F_PR_CAS_PTR_2_VALUE [SPECIFIED]

**Postconditions:**
- entry is now the head of the stack [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe with multiple producers (with reuse), multiple consumers [SPECIFIED]
- Memory Ordering: Release semantics [OBSERVED]
- Progress Guarantee: lock-free [SPECIFIED]

---

### ck_stack_pop_mpmc

**Signature:**
```
ck_stack_pop_mpmc(target: Pointer to ck_stack) → Pointer to ck_stack_entry (or NULL)
```

**Preconditions:**
- target must not be NULL [INFERRED]
- Platform must support CK_F_PR_CAS_PTR_2_VALUE [SPECIFIED]

**Postconditions:**
- IF stack was non-empty: returns former head [SPECIFIED]
- IF stack was empty: returns NULL [SPECIFIED]
- Generation counter is incremented on success [SPECIFIED]

**Invariants Preserved:**
- ABA prevention via generation counter [SPECIFIED]

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| Platform lacks double-width CAS | Compilation error or runtime failure | SPECIFIED |

**Concurrency:**
- Thread Safety: Safe with multiple producers and consumers, entries may be reused [SPECIFIED]
- Memory Ordering: Sequentially consistent (double-width CAS) [OBSERVED]
- Progress Guarantee: lock-free [SPECIFIED]

---

### ck_stack_push_mpnc

**Signature:**
```
ck_stack_push_mpnc(target: Pointer to ck_stack, entry: Pointer to ck_stack_entry) → void
```

**Preconditions:**
- target must not be NULL [INFERRED]
- entry must not be NULL [INFERRED]
- No concurrent consumers on this stack [SPECIFIED]

**Postconditions:**
- entry is added to stack [SPECIFIED]
- Stack traversal order is not guaranteed until all pushes complete [OBSERVED]

**Invariants Preserved:**
- Stack eventually consistent after pushes drain [OBSERVED]

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| Concurrent consumer | Undefined behavior, corrupted reads | SPECIFIED |

**Concurrency:**
- Thread Safety: Multiple producers only, no consumers [SPECIFIED]
- Memory Ordering: FAS with release fence [OBSERVED]
- Progress Guarantee: wait-free [SPECIFIED]

---

### ck_stack_push_spnc

**Signature:**
```
ck_stack_push_spnc(target: Pointer to ck_stack, entry: Pointer to ck_stack_entry) → void
```

**Preconditions:**
- target must not be NULL [INFERRED]
- entry must not be NULL [INFERRED]
- Single producer, no concurrent consumers [SPECIFIED]

**Postconditions:**
- entry is now head of stack [SPECIFIED]

**Concurrency:**
- Thread Safety: Single producer only [SPECIFIED]
- Memory Ordering: None (direct pointer manipulation) [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_stack_pop_npsc

**Signature:**
```
ck_stack_pop_npsc(target: Pointer to ck_stack) → Pointer to ck_stack_entry (or NULL)
```

**Preconditions:**
- target must not be NULL [INFERRED]
- No concurrent producers [SPECIFIED]
- Single consumer only [SPECIFIED]

**Postconditions:**
- Returns former head or NULL if empty [SPECIFIED]

**Concurrency:**
- Thread Safety: No producers, single consumer [SPECIFIED]
- Memory Ordering: None (direct pointer manipulation) [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### ck_stack_batch_pop_upmc

**Signature:**
```
ck_stack_batch_pop_upmc(target: Pointer to ck_stack) → Pointer to ck_stack_entry (or NULL)
```

**Preconditions:**
- target must not be NULL [INFERRED]

**Postconditions:**
- Returns linked list of all former entries [SPECIFIED]
- Stack is now empty [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe with UPMC constraints [SPECIFIED]
- Memory Ordering: Acquire semantics [OBSERVED]
- Progress Guarantee: wait-free (single atomic swap) [SPECIFIED]

---

### ck_stack_batch_pop_mpmc

**Signature:**
```
ck_stack_batch_pop_mpmc(target: Pointer to ck_stack) → Pointer to ck_stack_entry (or NULL)
```

**Preconditions:**
- target must not be NULL [INFERRED]
- Platform must support double-width CAS [SPECIFIED]

**Postconditions:**
- Returns linked list of all former entries [SPECIFIED]
- Stack is now empty [SPECIFIED]
- Generation counter incremented [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe with MPMC [SPECIFIED]
- Progress Guarantee: lock-free [SPECIFIED]

---

## Data Structure Invariants

### ck_stack_entry

- next points to valid entry or NULL [OBSERVED]
- Entry appears in at most one stack at a time [SPECIFIED]

### ck_stack

- head is NULL or points to valid entry [OBSERVED]
- All entries reachable from head form a proper linked list [SPECIFIED]
- For MPMC: generation monotonically increases with pops [SPECIFIED]

## Module-Level Invariants

- LIFO ordering is preserved within single-threaded access [SPECIFIED]
- Under concurrent access, linearizable ordering [SPECIFIED]

## Safety Properties

**No Lost Entries:** A successfully pushed entry will be returned by exactly one pop operation (or batch_pop). [SPECIFIED]

**No Double Returns:** An entry is never returned by pop more than once without being re-pushed. [SPECIFIED]

## Liveness Properties

**Lock-Freedom:** Under contention, at least one thread makes progress. [SPECIFIED]

**Wait-Freedom:** SPNC push and NPSC pop complete in bounded time regardless of other threads. [SPECIFIED]

## Behavioral Ambiguities

### Entry reuse timing for UPMC

**Observed Behavior:** After pop returns entry, caller must not immediately reuse without safe reclamation

**Intent:** SPECIFIED - Use hazard pointers or epoch-based reclamation

**Recommendation:** Document clearly that UPMC pops require SMR before entry reuse. MPMC variant handles this with generation counter.

### Stack traversal during MPNC

**Observed Behavior:** During concurrent MPNC pushes, stack may be temporarily broken for traversal

**Intent:** OBSERVED - Stack is consistent only after all producers synchronize

**Recommendation:** Document that MPNC is append-only; readers must wait for producers to complete.

## Discrepancies

No discrepancies detected between sources.
