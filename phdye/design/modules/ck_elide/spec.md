# Module: ck_elide — Specification

## Operations

### CK_ELIDE_PROTOTYPE

**Signature:**
```
CK_ELIDE_PROTOTYPE(N, T, L_P, L, U_P, U) (macro)
```

**Preconditions:**
- L_P is a predicate returning true if lock is held [SPECIFIED]
- L is a valid lock function [SPECIFIED]
- U_P is a predicate returning true if unlock needed [SPECIFIED]
- U is a valid unlock function [SPECIFIED]

**Postconditions:**
- Generates ck_elide_N_lock, _unlock, _lock_adaptive, _unlock_adaptive [SPECIFIED]

---

### ck_elide_N_lock

**Signature:**
```
ck_elide_N_lock(lock: Pointer to T) → void
```

**Postconditions:**
- Caller holds lock or is in transaction [SPECIFIED]

**Concurrency:**
- Thread Safety: Depends on underlying lock [SPECIFIED]
- Progress Guarantee: Blocking (fallback) [OBSERVED]

---

### ck_elide_N_unlock

**Signature:**
```
ck_elide_N_unlock(lock: Pointer to T) → void
```

**Postconditions:**
- Transaction ended or lock released [SPECIFIED]

---

### ck_elide_N_lock_adaptive

**Signature:**
```
ck_elide_N_lock_adaptive(lock: Pointer to T, stat: Pointer to ck_elide_stat, config: Pointer to ck_elide_config) → void
```

**Postconditions:**
- Caller holds lock or is in transaction [SPECIFIED]
- Statistics updated [SPECIFIED]

**Concurrency:**
- Thread Safety: stat must be per-thread [SPECIFIED]
- Progress Guarantee: Blocking (eventually acquires) [OBSERVED]

---

### ck_elide_N_unlock_adaptive

**Signature:**
```
ck_elide_N_unlock_adaptive(stat: Pointer to ck_elide_stat, lock: Pointer to T) → void
```

**Postconditions:**
- Transaction ended or lock released [SPECIFIED]
- n_elide incremented if elided successfully [OBSERVED]
- skip reset to 0 on successful elision [OBSERVED]

---

### CK_ELIDE_TRYLOCK_PROTOTYPE

**Signature:**
```
CK_ELIDE_TRYLOCK_PROTOTYPE(N, T, TL_P, TL) (macro)
```

**Postconditions:**
- Generates ck_elide_N_trylock [SPECIFIED]

---

### ck_elide_N_trylock

**Signature:**
```
ck_elide_N_trylock(lock: Pointer to T) → bool
```

**Postconditions:**
- Returns true if in transaction [SPECIFIED]
- Returns false if transaction could not start [SPECIFIED]

---

### ck_elide_stat_init

**Signature:**
```
ck_elide_stat_init(stat: Pointer to ck_elide_stat) → void
```

**Postconditions:**
- n_fallback = 0 [SPECIFIED]
- n_elide = 0 [SPECIFIED]
- skip = 0 [SPECIFIED]

---

## Safety Properties

**Mutual Exclusion:** If fallback acquired, exclusive access guaranteed. [SPECIFIED]

**Transaction Isolation:** If elided, transaction provides isolation. [SPECIFIED]

## Liveness Properties

**Progress:** Adaptive version eventually acquires lock or succeeds. [OBSERVED]

**Elision Rate:** Depends on conflict rate and config parameters. [OBSERVED]

## Discrepancies

No discrepancies detected.
