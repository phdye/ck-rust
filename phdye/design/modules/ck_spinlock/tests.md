# Module: ck_spinlock — Test Specification

## Conformance Tests

### TEST-001: fas_init

**Category:** basic

**Action:** Initialize FAS spinlock

**Expected Result:** Lock is unlocked

---

### TEST-002: fas_lock_unlock

**Category:** basic

**Setup:** Initialize FAS spinlock

**Action:** lock, unlock

**Expected Result:** Lock acquired and released

---

### TEST-003: fas_trylock_success

**Category:** basic

**Setup:** Initialize FAS spinlock

**Action:** trylock on unlocked

**Expected Result:** Returns true

---

### TEST-004: fas_trylock_failure

**Category:** concurrent

**Setup:** Another thread holds lock

**Action:** trylock

**Expected Result:** Returns false

---

### TEST-005: fas_lock_eb

**Category:** stress

**Setup:** Initialize FAS spinlock

**Action:** Multiple threads using lock_eb

**Expected Result:** All threads eventually acquire

---

### TEST-006: ticket_init

**Category:** basic

**Action:** Initialize ticket spinlock

**Expected Result:** next=0, position=0

---

### TEST-007: ticket_fifo_ordering

**Category:** concurrent

**Setup:** Initialize ticket lock, multiple threads

**Action:** Threads acquire in rapid succession

**Expected Result:** FIFO ordering observed

---

### TEST-008: ticket_lock_pb

**Category:** stress

**Setup:** Initialize ticket lock

**Action:** Multiple threads with proportional backoff

**Expected Result:** All threads acquire, reduced contention

---

### TEST-009: mcs_init

**Category:** basic

**Action:** Initialize MCS lock

**Expected Result:** queue = NULL

---

### TEST-010: mcs_lock_unlock

**Category:** basic

**Setup:** Initialize MCS lock, allocate node

**Action:** lock, unlock

**Expected Result:** Lock acquired and released

---

### TEST-011: mcs_fairness

**Category:** concurrent

**Setup:** Initialize MCS lock, multiple threads

**Action:** Threads acquire in sequence

**Expected Result:** FIFO ordering

---

### TEST-012: mcs_trylock_success

**Category:** basic

**Setup:** Initialize MCS lock

**Action:** trylock on empty queue

**Expected Result:** Returns true

---

### TEST-013: locked_predicate

**Category:** basic

**Setup:** Initialize various locks

**Action:** Check locked() before and after lock

**Expected Result:** false before, true during, false after

---

### TEST-014: elision_wrapper

**Category:** basic

**Condition:** CK_F_PR_RTM defined

**Action:** Use elision wrappers

**Expected Result:** Lock operations work

---

### TEST-015: stress_contention

**Category:** stress

**Setup:** All spinlock variants

**Action:** High contention workload

**Expected Result:** All threads make progress

---

### TEST-016: ticket_trylock

**Category:** basic

**Condition:** CK_F_SPINLOCK_TICKET_TRYLOCK defined

**Action:** trylock on ticket lock

**Expected Result:** Returns true when available

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| FAS init | TEST-001 | Covered |
| FAS lock/unlock | TEST-002 | Covered |
| FAS trylock | TEST-003, TEST-004 | Covered |
| FAS backoff | TEST-005 | Covered |
| Ticket init | TEST-006 | Covered |
| Ticket FIFO | TEST-007 | Covered |
| Ticket backoff | TEST-008 | Covered |
| Ticket trylock | TEST-016 | Covered |
| MCS init | TEST-009 | Covered |
| MCS lock/unlock | TEST-010 | Covered |
| MCS fairness | TEST-011 | Covered |
| MCS trylock | TEST-012 | Covered |
| Locked predicate | TEST-013 | Covered |
| Elision | TEST-014 | Covered |
| Stress | TEST-015 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_spinlock regression | regressions/ck_spinlock/ | TEST-001 through TEST-016 |
