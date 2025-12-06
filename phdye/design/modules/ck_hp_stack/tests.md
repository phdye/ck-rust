# Module: ck_hp_stack — Test Specification

## Conformance Tests

### TEST-001: push_pop_single

**Category:** basic

**Setup:** Initialize stack and HP infrastructure

**Action:** Push value, pop value

**Expected Result:** Popped value matches pushed

---

### TEST-002: pop_empty

**Category:** basic

**Setup:** Initialize empty stack

**Action:** Attempt pop

**Expected Result:** Returns NULL

---

### TEST-003: lifo_ordering

**Category:** basic

**Setup:** Initialize stack

**Action:** Push A, B, C; pop 3 times

**Expected Result:** Returns C, B, A (LIFO order)

---

### TEST-004: trypush_success

**Category:** basic

**Setup:** Initialize stack

**Action:** trypush on empty stack

**Expected Result:** Returns true

---

### TEST-005: trypop_empty

**Category:** basic

**Setup:** Initialize empty stack

**Action:** trypop

**Expected Result:** Returns false

---

### TEST-006: trypop_success

**Category:** basic

**Setup:** Initialize stack with entries

**Action:** trypop

**Expected Result:** Returns true, entry returned

---

### TEST-007: multi_pusher

**Category:** concurrent

**Setup:** Stack, HP records for 4 threads

**Action:** Each thread pushes 1000 items

**Expected Result:** All 4000 items poppable

---

### TEST-008: multi_popper

**Category:** concurrent

**Setup:** Stack with 4000 items, 4 poppers

**Action:** Each thread pops until empty

**Expected Result:** All items popped, no duplicates

---

### TEST-009: multi_push_pop

**Category:** concurrent

**Setup:** Stack, 4 pushers, 4 poppers

**Action:** Concurrent push/pop

**Expected Result:** No corruption, no use-after-free

---

### TEST-010: hazard_protection

**Category:** concurrent

**Setup:** Stack with high contention

**Action:** Pop while other threads modify

**Expected Result:** No use-after-free (sanitizer clean)

---

### TEST-011: trypop_contention

**Category:** concurrent

**Setup:** Stack, multiple trypop callers

**Action:** Concurrent trypop

**Expected Result:** Some succeed, some fail, no corruption

---

### TEST-012: hp_cleared_on_failure

**Category:** basic

**Setup:** HP record, trypop failure

**Action:** Verify HP state after failed trypop

**Expected Result:** HP slot cleared

---

### TEST-013: stress_push_pop

**Category:** stress

**Setup:** Many threads, long duration

**Action:** Continuous push/pop

**Expected Result:** Lock-free progress, correctness

---

### TEST-014: stress_reclamation

**Category:** stress

**Setup:** Stack with continuous operation

**Action:** Pop entries, reclaim via HP

**Expected Result:** Memory bounded, no leaks

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| Push/pop basic | TEST-001, TEST-002 | Covered |
| LIFO ordering | TEST-003 | Covered |
| Try operations | TEST-004, TEST-005, TEST-006 | Covered |
| Multi-producer | TEST-007 | Covered |
| Multi-consumer | TEST-008 | Covered |
| MPMC | TEST-009 | Covered |
| Hazard protection | TEST-010 | Covered |
| Trypop contention | TEST-011 | Covered |
| HP cleanup | TEST-012 | Covered |
| Stress | TEST-013, TEST-014 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_hp_stack regression | regressions/ck_hp/ck_hp_stack/ | TEST-001 through TEST-014 |
