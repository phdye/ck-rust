# Module: ck_ec — Specification

## Operations

### ck_ec_init

**Signature:**
```
ck_ec32_init(ec: Pointer to ck_ec32, value: uint32_t) → void
ck_ec64_init(ec: Pointer to ck_ec64, value: uint64_t) → void
```

**Preconditions:**
- value <= INT32_MAX (32-bit) or value <= INT64_MAX (64-bit) [SPECIFIED]

**Postconditions:**
- Event count initialized to value [SPECIFIED]
- Flag bit cleared [OBSERVED]

---

### ck_ec_value

**Signature:**
```
ck_ec32_value(ec: Pointer to const ck_ec32) → uint32_t
ck_ec64_value(ec: Pointer to const ck_ec64) → uint64_t
```

**Postconditions:**
- Returns current counter value (without flag bit) [SPECIFIED]
- Value <= INT32_MAX (32-bit) or INT64_MAX (64-bit) [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Acquire semantics [SPECIFIED]
- Progress Guarantee: Wait-free [OBSERVED]

---

### ck_ec_has_waiters

**Signature:**
```
ck_ec32_has_waiters(ec: Pointer to const ck_ec32) → bool
ck_ec64_has_waiters(ec: Pointer to const ck_ec64) → bool
```

**Postconditions:**
- Returns true if flag bit is set (waiters need wake) [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [OBSERVED]
- Progress Guarantee: Wait-free [OBSERVED]

---

### ck_ec_inc

**Signature:**
```
ck_ec32_inc(ec: Pointer to ck_ec32, mode: Pointer to const ck_ec_mode) → void
ck_ec64_inc(ec: Pointer to ck_ec64, mode: Pointer to const ck_ec_mode) → void
```

**Postconditions:**
- Counter value incremented by 1 [SPECIFIED]
- Waiters woken if flag was set [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Write barrier before increment [SPECIFIED]
- Progress Guarantee: Wait-free (SP mode on x86), Lock-free (MP mode) [OBSERVED]

---

### ck_ec_add

**Signature:**
```
ck_ec32_add(ec: Pointer to ck_ec32, mode: Pointer to const ck_ec_mode, delta: uint32_t) → uint32_t
ck_ec64_add(ec: Pointer to ck_ec64, mode: Pointer to const ck_ec_mode, delta: uint64_t) → uint64_t
```

**Preconditions:**
- delta < INT32_MAX (32-bit) or delta < INT64_MAX (64-bit) [INFERRED]

**Postconditions:**
- Counter value incremented by delta [SPECIFIED]
- Returns previous counter value [SPECIFIED]
- Waiters woken if flag was set [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Write barrier before increment [SPECIFIED]
- Progress Guarantee: Wait-free (SP mode on x86), Lock-free (MP mode) [OBSERVED]

---

### ck_ec_deadline

**Signature:**
```
ck_ec_deadline(new_deadline: Pointer to timespec, mode: Pointer to const ck_ec_mode, timeout: Pointer to const timespec) → int
```

**Postconditions:**
- new_deadline = current_time + timeout [SPECIFIED]
- IF timeout is NULL: new_deadline = infinite future [SPECIFIED]
- Returns 0 on success [SPECIFIED]
- Returns -1 if gettime failed [SPECIFIED]

---

### ck_ec_wait

**Signature:**
```
ck_ec32_wait(ec: Pointer to ck_ec32, mode: Pointer to const ck_ec_mode, old_value: uint32_t, deadline: Pointer to const timespec) → int
ck_ec64_wait(ec: Pointer to ck_ec64, mode: Pointer to const ck_ec_mode, old_value: uint64_t, deadline: Pointer to const timespec) → int
```

**Postconditions:**
- IF counter value != old_value: returns 0 [SPECIFIED]
- IF deadline reached: returns -1 [SPECIFIED]
- IF deadline is NULL: waits indefinitely [SPECIFIED]
- IF deadline->tv_sec == 0: non-blocking check [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Acquire semantics [SPECIFIED]
- Progress Guarantee: Blocking [SPECIFIED]

---

### ck_ec_wait_pred

**Signature:**
```
ck_ec32_wait_pred(ec: Pointer to ck_ec32, mode: Pointer to const ck_ec_mode, old_value: uint32_t, pred: Predicate function, data: void*, deadline: Pointer to const timespec) → int
ck_ec64_wait_pred(ec: Pointer to ck_ec64, mode: Pointer to const ck_ec_mode, old_value: uint64_t, pred: Predicate function, data: void*, deadline: Pointer to const timespec) → int
```

**Postconditions:**
- IF counter value != old_value: returns 0 [SPECIFIED]
- IF pred returns non-zero: returns pred's return value [SPECIFIED]
- IF deadline reached: returns -1 [SPECIFIED]
- IF pred is NULL: behaves as ck_ec_wait [SPECIFIED]

**Predicate Signature:**
```
pred(state: Pointer to const ck_ec_wait_state, deadline: Pointer to timespec) → int
```
- pred may modify iteration deadline [SPECIFIED]
- pred receives wait state with start time, current time, ops, user data [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Memory Ordering: Acquire semantics [SPECIFIED]
- Progress Guarantee: Blocking [SPECIFIED]

---

## Safety Properties

**Counter Monotonicity:** Counter value only increases (modulo wraparound). [SPECIFIED]

**Flag Visibility:** After flag set and >1 second elapsed, producer guaranteed to see flag on next update (x86-TSO). [SPECIFIED]

**No Lost Wakes:** If waiter sets flag before producer increments, producer will wake. [SPECIFIED]

## Liveness Properties

**Producer Progress:** inc/add always completes in bounded steps. [OBSERVED]

**Waiter Progress:** Waiter either returns on value change, predicate, timeout, or deadline. [SPECIFIED]

## Discrepancies

No discrepancies detected.
