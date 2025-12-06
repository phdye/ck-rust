# Module: ck_rwcohort — Specification

## Operations

### CK_RWCOHORT_WP_PROTOTYPE (Writer-Preference)

**Signature:**
```
CK_RWCOHORT_WP_PROTOTYPE(N) (macro)
```

**Postconditions:**
- Generates ck_rwcohort_wp_N type and functions [SPECIFIED]

---

### ck_rwcohort_wp_N_init

**Signature:**
```
ck_rwcohort_wp_N_init(rwcohort: Pointer to instance, wait_limit: unsigned int) → void
```

**Postconditions:**
- read_counter = 0 [SPECIFIED]
- write_barrier = 0 [SPECIFIED]
- wait_limit set [SPECIFIED]

---

### ck_rwcohort_wp_N_write_lock

**Signature:**
```
ck_rwcohort_wp_N_write_lock(rwcohort: Pointer, cohort: Pointer, global_ctx: void*, local_ctx: void*) → void
```

**Postconditions:**
- Caller holds exclusive write access [SPECIFIED]
- All readers have drained [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Blocking [SPECIFIED]

---

### ck_rwcohort_wp_N_write_unlock

**Signature:**
```
ck_rwcohort_wp_N_write_unlock(rwcohort: Pointer, cohort: Pointer, global_ctx: void*, local_ctx: void*) → void
```

**Postconditions:**
- Write lock released [SPECIFIED]

---

### ck_rwcohort_wp_N_read_lock

**Signature:**
```
ck_rwcohort_wp_N_read_lock(rwcohort: Pointer, cohort: Pointer, global_ctx: void*, local_ctx: void*) → void
```

**Postconditions:**
- Caller holds shared read access [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Blocking (may starve under write pressure) [OBSERVED]

---

### ck_rwcohort_wp_N_read_unlock

**Signature:**
```
ck_rwcohort_wp_N_read_unlock(rwcohort: Pointer) → void
```

**Postconditions:**
- Read lock released [SPECIFIED]

---

### CK_RWCOHORT_RP_PROTOTYPE (Reader-Preference)

**Signature:**
```
CK_RWCOHORT_RP_PROTOTYPE(N) (macro)
```

**Postconditions:**
- Generates ck_rwcohort_rp_N type and functions [SPECIFIED]

---

### ck_rwcohort_rp_N_init

**Signature:**
```
ck_rwcohort_rp_N_init(rwcohort: Pointer to instance, wait_limit: unsigned int) → void
```

**Postconditions:**
- read_counter = 0, read_barrier = 0, wait_limit set [SPECIFIED]

---

### ck_rwcohort_rp_N_write_lock / write_unlock

**Postconditions:**
- Same as WP variant [SPECIFIED]
- Writers may starve under read pressure [OBSERVED]

---

### ck_rwcohort_rp_N_read_lock / read_unlock

**Postconditions:**
- Same as WP variant [SPECIFIED]
- Readers preferred [OBSERVED]

---

### CK_RWCOHORT_NEUTRAL_PROTOTYPE

**Signature:**
```
CK_RWCOHORT_NEUTRAL_PROTOTYPE(N) (macro)
```

**Postconditions:**
- Generates ck_rwcohort_neutral_N type and functions [SPECIFIED]

---

### ck_rwcohort_neutral_N_init

**Signature:**
```
ck_rwcohort_neutral_N_init(rwcohort: Pointer to instance) → void
```

**Postconditions:**
- read_counter = 0 [SPECIFIED]

---

### ck_rwcohort_neutral_N_write_lock / write_unlock / read_lock / read_unlock

**Postconditions:**
- Fair ordering via cohort lock [SPECIFIED]
- Lower reader concurrency (cohort acquired for read) [OBSERVED]

---

## Safety Properties

**Mutual Exclusion:** Writers have exclusive access. [SPECIFIED]

**Reader Sharing:** Multiple readers hold lock concurrently. [SPECIFIED]

**No Read-Write Concurrency:** Writers wait for readers to drain. [SPECIFIED]

## Liveness Properties

**WP Progress:** Writers eventually acquire; readers may wait long. [SPECIFIED]

**RP Progress:** Readers eventually acquire; writers may wait long. [SPECIFIED]

**Neutral Progress:** FIFO-ish ordering through cohort. [OBSERVED]

## Discrepancies

No discrepancies detected.
