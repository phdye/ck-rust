# Module: ck_epoch — Test Specification

## Conformance Tests

### TEST-001: init_global

**Category:** basic

**Action:** Initialize ck_epoch

**Expected Result:** epoch = 0, records stack empty

---

### TEST-002: register_record

**Category:** basic

**Setup:** Initialize global epoch

**Action:** Register record with context

**Expected Result:** record->global set, context accessible

---

### TEST-003: begin_end_single

**Category:** basic

**Setup:** Initialize epoch, register record

**Action:** begin, end

**Expected Result:** active returns to 0

---

### TEST-004: begin_end_nested

**Category:** basic

**Setup:** Initialize epoch, register record

**Action:** begin, begin, end, end

**Expected Result:** active correctly tracks nesting

---

### TEST-005: call_defer

**Category:** basic

**Setup:** Initialize epoch, register record

**Action:** Call ck_epoch_call with callback

**Expected Result:** n_pending incremented, callback not yet invoked

---

### TEST-006: poll_no_progress

**Category:** basic

**Setup:** One thread in active section

**Action:** Another thread calls poll

**Expected Result:** Returns false, epoch not advanced

---

### TEST-007: poll_progress

**Category:** concurrent

**Setup:** Register records, no active sections

**Action:** Call poll

**Expected Result:** Returns true if epoch advanced

---

### TEST-008: synchronize_blocks

**Category:** concurrent

**Setup:** Thread A in epoch section, Thread B calls synchronize

**Action:** Thread A exits, Thread B observes

**Expected Result:** Thread B unblocks after A exits

---

### TEST-009: callback_dispatch

**Category:** concurrent

**Setup:** Defer callback, exit all sections

**Action:** Poll or barrier

**Expected Result:** Callback invoked

---

### TEST-010: barrier_all_callbacks

**Category:** basic

**Setup:** Defer multiple callbacks

**Action:** Call barrier

**Expected Result:** All callbacks dispatched

---

### TEST-011: recycle_record

**Category:** basic

**Setup:** Register record, unregister

**Action:** Call recycle

**Expected Result:** Returns recycled record

---

### TEST-012: call_strict_concurrent

**Category:** concurrent

**Setup:** Multiple threads sharing record

**Action:** All call call_strict

**Expected Result:** All callbacks properly queued

---

### TEST-013: value_read

**Category:** basic

**Setup:** Initialize epoch

**Action:** Call ck_epoch_value

**Expected Result:** Returns current epoch with ordering

---

### TEST-014: section_progress

**Category:** concurrent

**Setup:** Long-running reader with section

**Action:** Multiple synchronize calls

**Expected Result:** Progress made despite long reader

---

### TEST-015: stress_dispatch

**Category:** stress

**Setup:** Many threads, many callbacks

**Action:** Continuous defer and poll

**Expected Result:** All callbacks eventually dispatched

---

### TEST-016: unregister_safe

**Category:** basic

**Setup:** Register record, exit all sections

**Action:** Unregister

**Expected Result:** Record marked free, no crash

---

### TEST-017: epoch_value_ordering

**Category:** concurrent

**Setup:** Multiple readers and writers

**Action:** Value reads interleaved with epoch advances

**Expected Result:** Values monotonically increase

---

## Coverage Matrix

| Requirement | Test IDs | Status |
|-------------|----------|--------|
| Init | TEST-001 | Covered |
| Register | TEST-002 | Covered |
| Begin/end | TEST-003, TEST-004 | Covered |
| Call | TEST-005 | Covered |
| Poll | TEST-006, TEST-007 | Covered |
| Synchronize | TEST-008 | Covered |
| Dispatch | TEST-009, TEST-010 | Covered |
| Recycle | TEST-011 | Covered |
| Call strict | TEST-012 | Covered |
| Value | TEST-013, TEST-017 | Covered |
| Section | TEST-014 | Covered |
| Stress | TEST-015 | Covered |
| Unregister | TEST-016 | Covered |

## Existing Test Mapping

| Existing Test | Location | Maps To |
|---------------|----------|---------|
| ck_epoch regression | regressions/ck_epoch/ | TEST-001 through TEST-017 |
