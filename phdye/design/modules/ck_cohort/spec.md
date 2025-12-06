# Module: ck_cohort — Specification

## Operations

### CK_COHORT_PROTOTYPE

**Signature:**
```
CK_COHORT_PROTOTYPE(N, GL, GU, GI, LL, LU, LI) (macro)
```

**Preconditions:**
- GL/GU/GI are valid global lock functions [INFERRED]
- LL/LU/LI are valid local lock functions [INFERRED]

**Postconditions:**
- Generates struct ck_cohort_N type [SPECIFIED]
- Generates ck_cohort_N_init, _lock, _unlock, _locked functions [SPECIFIED]

---

### ck_cohort_*_init

**Signature:**
```
ck_cohort_N_init(cohort, global_lock, local_lock, pass_limit) → void
```

**Postconditions:**
- Cohort initialized with provided locks [SPECIFIED]
- release_state = GLOBAL [SPECIFIED]
- local_pass_limit set [SPECIFIED]

---

### ck_cohort_*_lock

**Signature:**
```
ck_cohort_N_lock(cohort, global_context, local_context) → void
```

**Postconditions:**
- Caller holds cohort lock [SPECIFIED]
- Global lock acquired if necessary [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Blocking [SPECIFIED]

---

### ck_cohort_*_unlock

**Signature:**
```
ck_cohort_N_unlock(cohort, global_context, local_context) → void
```

**Postconditions:**
- Lock released [SPECIFIED]
- IF local waiters and under limit: pass locally [SPECIFIED]
- ELSE: release global [SPECIFIED]

---

### ck_cohort_*_locked

**Signature:**
```
ck_cohort_N_locked(cohort, global_context, local_context) → bool
```

**Postconditions:**
- Returns true if lock is held [SPECIFIED]

---

## Safety Properties

**Mutual Exclusion:** Only one thread holds cohort lock. [SPECIFIED]

**Bounded Local Passes:** At most local_pass_limit consecutive local passes. [SPECIFIED]

## Liveness Properties

**Fairness:** Non-local threads eventually acquire after limit reached. [SPECIFIED]

## Discrepancies

No discrepancies detected.
