# Module: ck_hp_stack — Specification

## Operations

### ck_hp_stack_push_mpmc

**Signature:**
```
ck_hp_stack_push_mpmc(target: Pointer to ck_stack, entry: Pointer to ck_stack_entry) → void
```

**Postconditions:**
- entry pushed to top of stack [SPECIFIED]
- Operation always completes [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Lock-free [OBSERVED]

**Note:** Delegates to ck_stack_push_upmc.

---

### ck_hp_stack_trypush_mpmc

**Signature:**
```
ck_hp_stack_trypush_mpmc(target: Pointer to ck_stack, entry: Pointer to ck_stack_entry) → bool
```

**Postconditions:**
- Returns true if push succeeded [SPECIFIED]
- Returns false if CAS failed [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Wait-free (single attempt) [OBSERVED]

---

### ck_hp_stack_pop_mpmc

**Signature:**
```
ck_hp_stack_pop_mpmc(record: Pointer to ck_hp_record_t, target: Pointer to ck_stack) → Pointer to ck_stack_entry
```

**Preconditions:**
- record is valid HP record with at least 1 slot [SPECIFIED]

**Postconditions:**
- Returns entry pointer if stack non-empty [SPECIFIED]
- Returns NULL if stack empty [SPECIFIED]
- Returned entry protected by hazard pointer [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Lock-free [OBSERVED]

---

### ck_hp_stack_trypop_mpmc

**Signature:**
```
ck_hp_stack_trypop_mpmc(record: Pointer to ck_hp_record_t, target: Pointer to ck_stack, r: Pointer to Pointer to ck_stack_entry) → bool
```

**Postconditions:**
- Returns true if pop succeeded [SPECIFIED]
- Returns false if empty or contention [SPECIFIED]
- *r set to popped entry on success [SPECIFIED]
- On failure, hazard pointer cleared [OBSERVED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Wait-free (single attempt) [OBSERVED]

---

## Safety Properties

**LIFO Ordering:** Elements popped in reverse push order. [SPECIFIED]

**Safe Reclamation:** Hazard pointers prevent use-after-free. [SPECIFIED]

**No ABA Problem:** HP protection prevents ABA during pop. [OBSERVED]

## Liveness Properties

**Push Progress:** Push always completes (lock-free). [OBSERVED]

**Pop Progress:** At least one pop completes (lock-free). [OBSERVED]

## Discrepancies

No discrepancies detected.
