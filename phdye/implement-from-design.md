# Implement from Captured Design

## Purpose

This document provides explicit instructions for implementing a system from a captured design. The implementation proceeds module by module in dependency order, with each module's tests passing before proceeding to the next.

## Prerequisites

- Completed captured-design (per design-capture.md)
- Generated test suite (per create-tests-from-design.md)
- All stubs compile
- All tests compile and fail (red phase confirmed)
- Target language and toolchain configured

## Document Organization

| Part | Sections | Description |
|------|----------|-------------|
| **I. Preparation** | §101-§104 | Verify inputs, configure environment |
| **II. Module Implementation** | §201-§210 | Implement single module |
| **III. Integration** | §301-§305 | Connect modules, integration testing |
| **IV. Completion** | §401-§404 | Final validation, documentation |

---

# PART I: PREPARATION

---

## I.A. Input Verification

### I.A.1. Verify Captured Design Complete (§101)

#### PURPOSE
Confirm all required design artifacts exist.

#### INPUT
- Path to captured-design directory

#### ACTION
Verify these exist:

```
captured-design/
├── scope.md
├── modules.md
├── implementation-order.md
└── modules/{each module}/
    ├── design.md
    ├── spec.md
    ├── decisions.md
    └── tests.md
```

#### VALIDATION
```
[ ] scope.md exists
[ ] modules.md exists with complete module list
[ ] implementation-order.md exists with tier assignments
[ ] Every module in modules.md has design.md
[ ] Every module in modules.md has spec.md
[ ] Every module in modules.md has tests.md
[ ] No PENDING-USER discrepancies remain unresolved
```

#### NEXT
- IF any check fails: STOP. Complete design capture first.
- IF all pass: Proceed to §102.

---

### I.A.2. Verify Test Suite Generated (§102)

#### PURPOSE
Confirm test suite exists and is in proper red phase.

#### INPUT
- Path to implementation project

#### ACTION
Verify:

```
{project}/
├── src/                        ← Stubs exist
│   └── {module}/
└── tests/
    ├── conformance/{module}/   ← Tests exist
    └── comprehensive/{module}/ ← Tests exist
```

Run compilation check (no execution):
```
compile --check-only
```

Run one sample test to confirm failure:
```
test --run-one {any test}
→ Expected: Fails with "unimplemented" / todo! / abort
```

#### VALIDATION
```
[ ] Stub files exist for all modules
[ ] Test files exist for all modules
[ ] Project compiles without errors
[ ] Sample test fails with "unimplemented"
```

#### NEXT
- IF any check fails: STOP. Run create-tests-from-design.md first.
- IF all pass: Proceed to §103.

---

## I.B. Environment Setup

### I.B.1. Configure Implementation Environment (§103)

#### PURPOSE
Set up the development environment for implementation.

#### INPUT
- Target language
- Test framework

#### ACTION
Configure:

1. **Build system** — Ensure incremental compilation works
2. **Test runner** — Configure to run tests by module/tier
3. **Linter/formatter** — Configure for consistent style
4. **Debug tooling** — Debugger, logging, assertions enabled
5. **CI (optional)** — Automated test runs on commit

Create implementation tracking file:

```markdown
# Implementation Progress

## Current State
- Current Tier: 0
- Current Module: (none started)
- Status: Ready to begin

## Tier 0 Modules
| Module | Status | Tests Passing | Notes |
|--------|--------|---------------|-------|
| {mod1} | Not Started | 0/N | |
| {mod2} | Not Started | 0/N | |

## Tier 1 Modules
| Module | Status | Tests Passing | Notes |
|--------|--------|---------------|-------|
...
```

#### OUTPUT
- Development environment configured
- Implementation tracking file created

#### VALIDATION
```
[ ] Build system works (can compile)
[ ] Test runner works (can run tests)
[ ] Tracking file created
```

#### NEXT
Proceed to §104.

---

### I.B.2. Load Implementation Order (§104)

#### PURPOSE
Determine which module to implement first.

#### INPUT
- implementation-order.md from captured-design

#### ACTION
Parse implementation order:

```
Tier 0: {module_a}, {module_b}, ...  ← No dependencies, implement first
Tier 1: {module_c}, {module_d}, ...  ← Depend on Tier 0
Tier 2: {module_e}, {module_f}, ...  ← Depend on Tier 0 and 1
...
```

Select first module:
- Start with Tier 0
- Within tier, order doesn't matter (all have same dependencies)
- Pick any unimplemented Tier 0 module

#### OUTPUT
- Current tier: 0
- Current module: {first module}

#### VALIDATION
```
[ ] Implementation order loaded
[ ] First module selected
[ ] Module has no unimplemented dependencies
```

#### NEXT
Proceed to §201.

---

# PART II: MODULE IMPLEMENTATION

This part implements a single module. Repeat for each module.

---

## II.A. Module Preparation

### II.A.1. Read Module Design (§201)

#### PURPOSE
Understand what needs to be implemented.

#### INPUT
- Current module name
- captured-design/modules/{module}/

#### ACTION
Read and internalize:

**1. design.md**
- Data structures (fields, types, invariants)
- Algorithms (steps, complexity, edge cases)
- Internal architecture

**2. spec.md**
- Preconditions (what must be true before operation)
- Postconditions (what must be true after operation)
- Error conditions (when and what errors)
- Invariants (always true)
- Certainty markers ([SPECIFIED], [OBSERVED], [INFERRED])

**3. decisions.md**
- Why certain choices were made
- Constraints that led to the design
- Trade-offs accepted

**4. tests.md**
- What behaviors are tested
- What coverage exists
- What gaps remain

**Note particularly:**
- `[OBSERVED]` behaviors — May be accidental, implement carefully
- `[INFERRED]` behaviors — May be wrong, verify understanding
- UNKNOWN rationales — May need judgment calls

#### OUTPUT
- Mental model of what module does
- List of operations to implement
- List of concerns/questions

#### VALIDATION
```
[ ] design.md read and understood
[ ] spec.md read, operations listed
[ ] decisions.md read, constraints noted
[ ] tests.md read, coverage understood
```

#### NEXT
Proceed to §202.

---

### II.A.2. Review Module Tests (§202)

#### PURPOSE
Understand how the module will be validated.

#### INPUT
- tests/conformance/{module}/
- tests/comprehensive/{module}/

#### ACTION
Review tests to understand:

1. **What's tested first** — Simple cases, happy path
2. **What's tested thoroughly** — Core operations
3. **What edge cases exist** — Boundaries, errors
4. **What concurrent scenarios exist** — Thread safety

Order implementation to make tests pass incrementally:
```
Suggested implementation order for {module}:
1. {data structure} — Needed by all operations
2. {simple operation} — Makes TEST-001..005 pass
3. {complex operation} — Makes TEST-006..020 pass
4. {error handling} — Makes TEST-021..030 pass
5. {concurrent support} — Makes CONC-001..010 pass
```

#### OUTPUT
- Test-informed implementation order within module

#### VALIDATION
```
[ ] Tests reviewed
[ ] Implementation order within module determined
```

#### NEXT
Proceed to §203.

---

## II.B. Implementation

### II.B.1. Implement Data Structures (§203)

#### PURPOSE
Implement the module's data structures.

#### INPUT
- design.md Data Structures section
- Stub file for module

#### ACTION
Replace stub data structures with real implementations:

**For each data structure:**

1. **Fields** — Implement all fields with correct types
2. **Invariants** — Add debug assertions for invariants
3. **Initialization** — Implement constructors
4. **Drop/cleanup** — Implement destructors if needed

**Translation from design.md:**

| design.md | Implementation |
|-----------|----------------|
| "32-bit unsigned integer" | Target language equivalent |
| "Array of X, length N" | Fixed array or equivalent |
| "Pointer to X" | Pointer/reference type |
| "Atomic X" | Atomic type with correct semantics |

**Invariant enforcement:**
```
// From spec.md: "count <= capacity always"
debug_assert!(self.count <= self.capacity);
```

#### OUTPUT
- Data structures implemented
- Invariant assertions in place

#### VALIDATION
```
[ ] All data structures from design.md implemented
[ ] All fields present with correct types
[ ] Invariant assertions added
[ ] Compiles successfully
```

#### NEXT
Proceed to §204.

---

### II.B.2. Implement Operations (§204)

#### PURPOSE
Implement the module's operations one by one.

#### INPUT
- design.md Algorithms section
- spec.md operation specifications
- Test-informed implementation order from §202

#### ACTION
For each operation, in order:

**Step 1: Understand the specification**
```
From spec.md:
- Preconditions: {what must be true}
- Postconditions: {what will be true}
- Error conditions: {what can fail}
```

**Step 2: Implement precondition checks**
```
// Check preconditions, return error if violated
if !precondition_1 {
    return Error::PreconditionViolated;
}
```

**Step 3: Implement core algorithm**
```
// From design.md algorithm steps:
// 1. {step 1}
// 2. {step 2}
// ...
```

**Step 4: Implement postcondition assertions**
```
// Debug assertion for postconditions
debug_assert!(postcondition_holds);
```

**Step 5: Run operation's tests**
```
test --filter {operation_name}
```

**Step 6: Fix until tests pass**

Repeat for each operation.

#### IMPLEMENTATION ORDER WITHIN MODULE

```
1. Constructors / initialization
2. Simple accessors (getters)
3. Simple mutators (setters)
4. Core operations (main functionality)
5. Complex operations (build on core)
6. Error paths
7. Cleanup / destruction
```

#### OUTPUT
- Operations implemented incrementally
- Tests passing incrementally

#### VALIDATION
```
[ ] Each operation implemented
[ ] Each operation's tests run
[ ] All operation tests passing
```

#### NEXT
Proceed to §205.

---

### II.B.3. Handle Memory Ordering (§205)

#### PURPOSE
Implement correct memory ordering for concurrent operations.

#### INPUT
- spec.md memory ordering requirements
- design.md atomic operations

#### ACTION

**IF module has no atomic/concurrent operations:**
```
Skip this section.
Document: "Module has no concurrency requirements."
```

**IF module has atomic/concurrent operations:**

For each atomic operation:

1. **Read spec.md ordering requirement**
   ```
   Operation: atomic_load
   Minimum ordering: Acquire
   ```

2. **Implement with correct ordering**
   ```
   // spec.md requires Acquire for this load
   let value = self.data.load(Ordering::Acquire);
   ```

3. **Document ordering choice**
   ```
   // Ordering: Acquire (per spec.md §3.2)
   // Rationale: Must see all writes before flag was set
   ```

**Common patterns:**

| Pattern | Implementation |
|---------|----------------|
| Publish data, then flag | Store data (any), store flag (Release) |
| Check flag, then read | Load flag (Acquire), load data (any) |
| Read-modify-write | Use appropriate RMW ordering |
| Sequentially consistent | SeqCst (only if spec requires) |

#### OUTPUT
- Atomic operations have correct orderings
- Orderings documented in code

#### VALIDATION
```
[ ] All atomic operations have explicit ordering
[ ] Orderings match spec.md requirements
[ ] Each ordering has comment explaining why
```

#### NEXT
Proceed to §206.

---

### II.B.4. Implement Error Handling (§206)

#### PURPOSE
Implement all error conditions from spec.md.

#### INPUT
- spec.md error conditions
- tests.md error handling tests

#### ACTION
For each error condition in spec.md:

1. **Identify trigger**
   ```
   Error: BufferFull
   Trigger: push() called when count == capacity
   ```

2. **Implement detection**
   ```
   if self.count == self.capacity {
       return Err(Error::BufferFull);
   }
   ```

3. **Verify error test passes**
   ```
   test --filter error_buffer_full
   ```

**Error handling patterns:**

| spec.md says | Implementation |
|--------------|----------------|
| "Returns error X when Y" | Return error type/code |
| "Behavior undefined when Y" | Debug assertion + document |
| "Panics when Y" | Panic/abort |
| "No-op when Y" | Early return, no error |

#### OUTPUT
- All error conditions implemented
- Error tests passing

#### VALIDATION
```
[ ] Every error condition from spec.md implemented
[ ] Every error handling test passes
[ ] Error types/codes match spec
```

#### NEXT
Proceed to §207.

---

## II.C. Module Validation

### II.C.1. Run All Module Tests (§207)

#### PURPOSE
Verify all tests for this module pass.

#### INPUT
- Implemented module
- All tests for module

#### ACTION
Run complete test suite for module:

```
# Conformance tests
test tests/conformance/{module}/

# Comprehensive tests
test tests/comprehensive/{module}/unit/
test tests/comprehensive/{module}/property/
test tests/comprehensive/{module}/concurrent/  # if applicable
test tests/comprehensive/{module}/stress/      # if applicable
```

All tests must pass.

**IF any test fails:**
1. Identify which test
2. Read test to understand expected behavior
3. Read spec.md for that behavior
4. Fix implementation
5. Re-run tests
6. Repeat until all pass

#### OUTPUT
- All module tests passing

#### VALIDATION
```
[ ] All conformance tests pass
[ ] All unit tests pass
[ ] All property tests pass
[ ] All concurrent tests pass (if applicable)
[ ] All stress tests pass (if applicable)
```

#### NEXT
Proceed to §208.

---

### II.C.2. Run Memory Safety Checks (§208)

#### PURPOSE
Verify module has no memory safety issues.

#### INPUT
- Implemented module
- Memory safety tooling (Miri, sanitizers, Valgrind)

#### ACTION

**For Rust:**
```
cargo miri test --filter {module}
```

**For C/C++:**
```
# AddressSanitizer
compile -fsanitize=address && run_tests

# ThreadSanitizer (if concurrent)
compile -fsanitize=thread && run_tests

# Valgrind
valgrind --leak-check=full run_tests
```

**For other languages:**
Use equivalent memory safety tooling.

**IF issues found:**
1. Fix the issue
2. Re-run safety checks
3. Re-run all tests (ensure fix didn't break anything)

#### OUTPUT
- Module passes memory safety checks

#### VALIDATION
```
[ ] Miri/sanitizers/Valgrind pass
[ ] No memory leaks detected
[ ] No data races detected (if concurrent)
[ ] No undefined behavior detected
```

#### NEXT
Proceed to §209.

---

### II.C.3. Run Model Checking (§209)

#### PURPOSE
For concurrent modules, verify correctness under all interleavings.

#### INPUT
- Implemented concurrent module
- Loom tests (or equivalent)

#### ACTION

**IF module has no concurrency:**
```
Skip this section.
Document: "Module has no concurrency, model checking not applicable."
```

**IF module has concurrency:**

**For Rust (Loom):**
```
RUSTFLAGS="--cfg loom" cargo test --release --filter {module}_loom
```

**For other languages:**
Use equivalent model checking (CDSChecker, SPIN, etc.)

**IF model checking finds issues:**
1. Analyze the counterexample trace
2. Identify the race/deadlock/violation
3. Fix the implementation
4. Re-run model checking
5. Re-run all other tests

#### OUTPUT
- Concurrent module passes model checking

#### VALIDATION
```
[ ] Loom tests pass (or equivalent)
[ ] No deadlocks found
[ ] No data races found
[ ] All safety properties verified
```

#### NEXT
Proceed to §210.

---

### II.C.4. Update Progress and Select Next Module (§210)

#### PURPOSE
Record completion and determine next module.

#### INPUT
- Implementation tracking file
- implementation-order.md

#### ACTION

**Update tracking file:**
```markdown
## Tier 0 Modules
| Module | Status | Tests Passing | Notes |
|--------|--------|---------------|-------|
| {completed_module} | ✅ Complete | 47/47 | Completed {date} |
| {next_module} | Not Started | 0/N | |
```

**Select next module:**
```
IF current tier has more unimplemented modules:
    Select any unimplemented module in current tier
ELSE IF higher tiers exist:
    Move to next tier
    Select any module in new tier
ELSE:
    All modules complete
```

**Verify next module's dependencies:**
```
For each dependency of next module:
    Assert: dependency is marked Complete
```

#### OUTPUT
- Tracking file updated
- Next module selected (or all complete)

#### VALIDATION
```
[ ] Completed module marked in tracking file
[ ] Next module selected
[ ] Next module's dependencies all complete
```

#### NEXT
- IF more modules remain: Return to §201 for next module
- IF all modules complete: Proceed to §301

---

# PART III: INTEGRATION

---

## III.A. Module Integration

### III.A.1. Verify Module Interfaces (§301)

#### PURPOSE
Verify modules connect correctly at their interfaces.

#### INPUT
- All implemented modules
- modules.md (dependency graph)

#### ACTION
For each dependency relationship (A depends on B):

1. **Verify A uses B correctly**
   - A calls B's operations with valid arguments
   - A handles B's errors appropriately
   - A respects B's preconditions

2. **Verify no circular runtime dependencies**
   - Initialization order is valid
   - No deadlocks in module initialization

3. **Run cross-module tests**
   ```
   test tests/integration/  # if exists
   ```

#### OUTPUT
- All module interfaces verified

#### VALIDATION
```
[ ] Each module uses dependencies correctly
[ ] No circular initialization dependencies
[ ] Integration tests pass (if any)
```

#### NEXT
Proceed to §302.

---

### III.A.2. Run Full Test Suite (§302)

#### PURPOSE
Verify entire system works together.

#### INPUT
- All implemented modules
- Complete test suite

#### ACTION
Run all tests:

```
# All conformance tests
test tests/conformance/

# All comprehensive tests
test tests/comprehensive/

# All integration tests
test tests/integration/
```

All tests must pass.

#### OUTPUT
- Complete test suite passes

#### VALIDATION
```
[ ] All conformance tests pass
[ ] All comprehensive tests pass
[ ] All integration tests pass
[ ] Total test count matches expected
```

#### NEXT
Proceed to §303.

---

### III.A.3. Run System-Wide Safety Checks (§303)

#### PURPOSE
Verify no system-wide memory safety or concurrency issues.

#### INPUT
- Complete implementation
- Safety tooling

#### ACTION
Run safety checks on entire system:

```
# Memory safety
miri/sanitizers on full test suite

# Thread safety  
thread sanitizer on concurrent tests

# Leak detection
leak checker on full test suite
```

#### OUTPUT
- System passes all safety checks

#### VALIDATION
```
[ ] No memory safety issues system-wide
[ ] No thread safety issues system-wide
[ ] No leaks detected
```

#### NEXT
Proceed to §304.

---

## III.B. Performance Validation

### III.B.1. Run Performance Tests (§304)

#### PURPOSE
Verify implementation meets performance requirements.

#### INPUT
- perf.md (if exists in captured-design)
- Benchmark suite

#### ACTION

**IF no performance requirements (no perf.md):**
```
Run benchmarks for baseline documentation only.
No pass/fail criteria.
```

**IF performance requirements exist:**
```
For each requirement in perf.md:
    Run relevant benchmark
    Compare against requirement
    
    IF requirement not met:
        Profile to identify bottleneck
        Optimize (without breaking tests)
        Re-benchmark
```

Document results:
```markdown
## Performance Results

| Operation | Requirement | Measured | Status |
|-----------|-------------|----------|--------|
| push | < 100ns | 45ns | ✅ Pass |
| pop | < 100ns | 52ns | ✅ Pass |
```

#### OUTPUT
- Performance documented
- Requirements met (if any)

#### VALIDATION
```
[ ] Benchmarks run
[ ] Results documented
[ ] Requirements met (if any exist)
```

#### NEXT
Proceed to §305.

---

### III.B.2. Performance Regression Baseline (§305)

#### PURPOSE
Establish baseline for future performance regression detection.

#### INPUT
- Benchmark results from §304

#### ACTION
Create performance baseline file:

```markdown
# Performance Baseline

Generated: {date}
Platform: {OS, CPU, memory}
Compiler: {version, flags}

## Benchmarks

| Benchmark | Median | P99 | Min | Max |
|-----------|--------|-----|-----|-----|
| {bench1} | 45ns | 52ns | 42ns | 89ns |
| {bench2} | 120ns | 145ns | 115ns | 203ns |

## Notes
- {any relevant observations}
```

Configure CI (if applicable) to detect regressions:
```
IF new_median > baseline_median * 1.1:  # 10% regression threshold
    WARN: Performance regression detected
```

#### OUTPUT
- Performance baseline file
- Regression detection configured (optional)

#### VALIDATION
```
[ ] Baseline file created
[ ] Platform/compiler documented
[ ] All benchmarks recorded
```

#### NEXT
Proceed to §401.

---

# PART IV: COMPLETION

---

## IV.A. Final Validation

### IV.A.1. Validate Against Captured Design (§401)

#### PURPOSE
Verify implementation matches the captured design.

#### INPUT
- Complete implementation
- captured-design artifacts

#### ACTION
Review each module:

**For each module:**

1. **Check spec.md coverage**
   - Every operation in spec.md is implemented
   - Every precondition is checked (or documented why not)
   - Every postcondition holds (verified by tests)
   - Every error condition is handled

2. **Check design.md compliance**
   - Data structures match design
   - Algorithms match design (or deviations documented)

3. **Check decisions.md compliance**
   - Constraints respected
   - Trade-offs maintained

4. **Document any deviations**
   ```markdown
   ## Deviations from Captured Design
   
   ### {module}: {deviation title}
   
   **Specified:** {what spec.md said}
   **Implemented:** {what we did}
   **Reason:** {why we deviated}
   **Approved:** {yes/no, by whom}
   ```

#### OUTPUT
- Implementation validated against design
- Deviations documented

#### VALIDATION
```
[ ] Every module checked against spec.md
[ ] Every module checked against design.md
[ ] Deviations documented with rationale
[ ] Deviations approved (if any)
```

#### NEXT
Proceed to §402.

---

### IV.A.2. Generate Coverage Report (§402)

#### PURPOSE
Document test coverage achieved.

#### INPUT
- Test suite
- Coverage tooling

#### ACTION
Generate coverage report:

```
# Line coverage
coverage run tests/
coverage report

# Branch coverage
coverage run --branch tests/
coverage report
```

Document results:

```markdown
# Coverage Report

## Summary
- Line coverage: 94.2%
- Branch coverage: 87.5%
- Function coverage: 100%

## Per-Module Coverage

| Module | Lines | Branches | Functions |
|--------|-------|----------|-----------|
| {mod1} | 96% | 91% | 100% |
| {mod2} | 92% | 84% | 100% |

## Uncovered Code

| Location | Reason |
|----------|--------|
| {file}:{line} | Error path difficult to trigger |
| {file}:{line} | Platform-specific code |
```

#### OUTPUT
- Coverage report generated

#### VALIDATION
```
[ ] Coverage report generated
[ ] Coverage meets project standards (if defined)
[ ] Uncovered code documented
```

#### NEXT
Proceed to §403.

---

## IV.B. Documentation

### IV.B.1. Update Implementation Documentation (§403)

#### PURPOSE
Document the implementation for future maintainers.

#### INPUT
- Complete implementation
- captured-design
- Deviations document

#### ACTION
Create/update implementation documentation:

```markdown
# Implementation Notes

## Overview

This implementation of {system} was created from captured-design
following implement-from-design.md methodology.

## Key Implementation Decisions

### {Decision 1}
- **Context:** {situation}
- **Choice:** {what we did}
- **Rationale:** {why}

## Deviations from Captured Design

{Copy from §401 deviations document}

## Platform-Specific Notes

### {Platform 1}
- {notes}

## Known Limitations

- {limitation 1}
- {limitation 2}

## Performance Characteristics

{Summary from §304/§305}

## Maintenance Notes

- {guidance for future maintainers}
```

#### OUTPUT
- Implementation documentation complete

#### VALIDATION
```
[ ] Implementation notes created
[ ] Key decisions documented
[ ] Deviations documented
[ ] Platform notes included
[ ] Known limitations listed
```

#### NEXT
Proceed to §404.

---

### IV.B.2. Final Completion Checklist (§404)

#### PURPOSE
Confirm implementation is complete and ready for use.

#### INPUT
- Everything from previous sections

#### ACTION
Complete final checklist:

```
IMPLEMENTATION COMPLETE CHECKLIST

Prerequisites:
[ ] captured-design complete
[ ] Test suite generated

Implementation:
[ ] All Tier 0 modules implemented and tested
[ ] All Tier 1 modules implemented and tested
[ ] All Tier N modules implemented and tested
[ ] All modules pass all tests

Safety:
[ ] Memory safety verified (Miri/sanitizers)
[ ] Thread safety verified (if concurrent)
[ ] Model checking passed (if concurrent)

Integration:
[ ] All integration tests pass
[ ] Full test suite passes
[ ] System-wide safety checks pass

Performance:
[ ] Benchmarks run
[ ] Performance requirements met (if any)
[ ] Baseline established

Documentation:
[ ] Implementation notes complete
[ ] Deviations documented
[ ] Coverage report generated

SIGN-OFF

Implementation complete: {date}
Implemented by: {who}
Reviewed by: {who}
```

#### COMPLETION
```
============================================================
IMPLEMENTATION COMPLETE

Project: {project name}
Date: {completion date}
Target Language: {language}

Summary:
- Modules implemented: {count}
- Tests passing: {count}/{count}
- Line coverage: {percent}
- Deviations from design: {count}

Implementation is ready for deployment/use.
============================================================
```

---

# APPENDICES

---

## Appendix A: Common Implementation Patterns

### Pattern: Initialize-Once

**spec.md:** "Module must be initialized before use"

**Implementation:**
```
static INITIALIZED: AtomicBool = AtomicBool::new(false);

fn init() {
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        return; // Already initialized
    }
    // Actual initialization...
}
```

### Pattern: Resource Cleanup

**spec.md:** "Resources must be released on destruction"

**Implementation:**
```
impl Drop for Resource {
    fn drop(&mut self) {
        // Release resources
        self.cleanup();
    }
}
```

### Pattern: Precondition Check

**spec.md:** "Precondition: buffer not full"

**Implementation:**
```
fn push(&mut self, value: T) -> Result<(), Error> {
    // Precondition check
    if self.is_full() {
        return Err(Error::BufferFull);
    }
    // ... rest of operation
}
```

### Pattern: Postcondition Assertion

**spec.md:** "Postcondition: size increases by 1"

**Implementation:**
```
fn push(&mut self, value: T) -> Result<(), Error> {
    let old_size = self.len();
    
    // ... implementation ...
    
    debug_assert_eq!(self.len(), old_size + 1); // Postcondition
    Ok(())
}
```

---

## Appendix B: Troubleshooting

### Test Fails But Implementation Looks Correct

1. Re-read spec.md — Is your understanding correct?
2. Re-read test — Is the test correct?
3. Check certainty markers — Is behavior `[OBSERVED]` or `[INFERRED]`?
4. Check for discrepancies — Was this resolved correctly?

### Memory Safety Tool Reports Issue

1. Identify the exact location
2. Check if it's in library code vs your code
3. Review the operation's memory model
4. Check for use-after-free, double-free, uninitialized access

### Model Checker Finds Counterexample

1. Read the trace carefully
2. Identify the interleaving that causes failure
3. Check memory orderings — Are they strong enough?
4. Check for missing synchronization

### Performance Requirement Not Met

1. Profile to find bottleneck
2. Check algorithm complexity — Does it match design.md?
3. Check memory allocations — Unnecessary allocations?
4. Consider algorithmic improvements (within spec constraints)

---

## Appendix C: Section Reference

| Section | Title | Purpose |
|---------|-------|---------|
| §101 | Verify Captured Design Complete | Confirm inputs |
| §102 | Verify Test Suite Generated | Confirm tests exist |
| §103 | Configure Implementation Environment | Setup |
| §104 | Load Implementation Order | Determine first module |
| §201 | Read Module Design | Understand module |
| §202 | Review Module Tests | Plan implementation |
| §203 | Implement Data Structures | Create types |
| §204 | Implement Operations | Create functions |
| §205 | Handle Memory Ordering | Correct atomics |
| §206 | Implement Error Handling | Error paths |
| §207 | Run All Module Tests | Validate module |
| §208 | Run Memory Safety Checks | Safety verification |
| §209 | Run Model Checking | Concurrency verification |
| §210 | Update Progress and Select Next | Iterate |
| §301 | Verify Module Interfaces | Integration |
| §302 | Run Full Test Suite | System validation |
| §303 | Run System-Wide Safety Checks | System safety |
| §304 | Run Performance Tests | Performance validation |
| §305 | Performance Regression Baseline | Future tracking |
| §401 | Validate Against Captured Design | Design compliance |
| §402 | Generate Coverage Report | Coverage documentation |
| §403 | Update Implementation Documentation | Documentation |
| §404 | Final Completion Checklist | Sign-off |

---

## Document Information

| Field | Value |
|-------|-------|
| Version | 1.0 |
| Created | 2025-12-06 |
| Purpose | Implement system from captured design |
| Depends On | design-capture.md, create-tests-from-design.md |

