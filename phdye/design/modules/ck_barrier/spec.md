# Module: ck_barrier — Specification

## Operations

### ck_barrier_centralized

**Signature:**
```
ck_barrier_centralized(barrier: Pointer to ck_barrier_centralized_t, state: Pointer to ck_barrier_centralized_state_t, n: unsigned int) → void
```

**Preconditions:**
- n is the total number of participating threads [SPECIFIED]
- Each thread has unique state object [SPECIFIED]

**Postconditions:**
- All n threads have arrived [SPECIFIED]
- All threads released simultaneously [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe [SPECIFIED]
- Progress Guarantee: Blocking [SPECIFIED]

---

### ck_barrier_combining_init

**Signature:**
```
ck_barrier_combining_init(barrier: Pointer to ck_barrier_combining_t, root: Pointer to ck_barrier_combining_group_t) → void
```

**Postconditions:**
- Barrier initialized with given root [SPECIFIED]
- Mutex initialized [OBSERVED]

---

### ck_barrier_combining_group_init

**Signature:**
```
ck_barrier_combining_group_init(barrier: Pointer to ck_barrier_combining_t, group: Pointer to ck_barrier_combining_group_t, k: unsigned int) → void
```

**Postconditions:**
- Group added to barrier tree [SPECIFIED]
- k specifies fan-in degree [SPECIFIED]

---

### ck_barrier_combining

**Signature:**
```
ck_barrier_combining(barrier: Pointer to ck_barrier_combining_t, group: Pointer to ck_barrier_combining_group_t, state: Pointer to ck_barrier_combining_state_t) → void
```

**Postconditions:**
- All threads in tree have arrived [SPECIFIED]
- All threads released [SPECIFIED]

---

### ck_barrier_dissemination_init

**Signature:**
```
ck_barrier_dissemination_init(barrier: Pointer to ck_barrier_dissemination_t, flags: Pointer to array of ck_barrier_dissemination_flag_t, n: unsigned int) → void
```

**Postconditions:**
- Barrier initialized for n threads [SPECIFIED]
- Flag arrays allocated by caller [SPECIFIED]

---

### ck_barrier_dissemination_subscribe

**Signature:**
```
ck_barrier_dissemination_subscribe(barrier: Pointer to ck_barrier_dissemination_t, state: Pointer to ck_barrier_dissemination_state_t) → void
```

**Postconditions:**
- Thread assigned unique ID [SPECIFIED]
- State initialized for barrier participation [SPECIFIED]

---

### ck_barrier_dissemination_size

**Signature:**
```
ck_barrier_dissemination_size(n: unsigned int) → unsigned int
```

**Postconditions:**
- Returns number of flags needed for n threads [SPECIFIED]
- Value is ceil(log2(n)) * n [OBSERVED]

---

### ck_barrier_dissemination

**Signature:**
```
ck_barrier_dissemination(barrier: Pointer to ck_barrier_dissemination_t, state: Pointer to ck_barrier_dissemination_state_t) → void
```

**Postconditions:**
- All subscribed threads synchronized [SPECIFIED]

---

### ck_barrier_tournament_init

**Signature:**
```
ck_barrier_tournament_init(barrier: Pointer to ck_barrier_tournament_t, rounds: Pointer to array of ck_barrier_tournament_round_t, n: unsigned int) → void
```

**Postconditions:**
- Barrier initialized for n threads [SPECIFIED]
- Round arrays allocated by caller [SPECIFIED]

---

### ck_barrier_tournament_subscribe

**Signature:**
```
ck_barrier_tournament_subscribe(barrier: Pointer to ck_barrier_tournament_t, state: Pointer to ck_barrier_tournament_state_t) → void
```

**Postconditions:**
- Thread assigned unique VPID [SPECIFIED]

---

### ck_barrier_tournament_size

**Signature:**
```
ck_barrier_tournament_size(n: unsigned int) → unsigned int
```

**Postconditions:**
- Returns rounds needed [SPECIFIED]

---

### ck_barrier_tournament

**Signature:**
```
ck_barrier_tournament(barrier: Pointer to ck_barrier_tournament_t, state: Pointer to ck_barrier_tournament_state_t) → void
```

**Postconditions:**
- All subscribed threads synchronized [SPECIFIED]

---

### ck_barrier_mcs_init

**Signature:**
```
ck_barrier_mcs_init(barrier: Pointer to ck_barrier_mcs_t, n: unsigned int) → void
```

**Postconditions:**
- Barrier array initialized for n threads [SPECIFIED]

---

### ck_barrier_mcs_subscribe

**Signature:**
```
ck_barrier_mcs_subscribe(barrier: Pointer to ck_barrier_mcs_t, state: Pointer to ck_barrier_mcs_state_t) → void
```

**Postconditions:**
- Thread assigned to barrier slot [SPECIFIED]

---

### ck_barrier_mcs

**Signature:**
```
ck_barrier_mcs(barrier: Pointer to ck_barrier_mcs_t, state: Pointer to ck_barrier_mcs_state_t) → void
```

**Postconditions:**
- All subscribed threads synchronized [SPECIFIED]

---

## Safety Properties

**Barrier Synchronization:** All participating threads must arrive before any proceed. [SPECIFIED]

**Sense Reversal:** Barriers use sense reversal to enable reuse. [OBSERVED]

## Liveness Properties

**Progress:** If all threads call barrier, all eventually proceed. [SPECIFIED]

**Deadlock Freedom:** No deadlock if all threads participate. [SPECIFIED]

## Discrepancies

No discrepancies detected.
