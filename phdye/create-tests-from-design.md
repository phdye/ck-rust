# Test Suite Generation from Captured Design

## Purpose

This document provides explicit instructions for generating a complete, executable test suite from a captured design. The generated test suite enables test-driven development (TDD): all tests exist before implementation begins, and tests are executed incrementally as each module is implemented.

## Completion Criteria

**This methodology is complete when:**
- Tests have been generated for every module in every tier listed in `<design-base>/design/implementation-order.md`
- Commit after each module completes

## Guidance

**Tiers are methodology scaffolding, not target project architecture.**
- Do NOT include "tier" in generated test code or comments
- Do NOT reference `implementation-order.md` in target project files
- The target project may use "tier" for entirely different purposes

**Follow target project conventions.**
- Match existing test organization, naming, and structure
- Use idiomatic test patterns for the target language
- Follow the chosen test framework's conventions

## Prerequisites

- Completed design capture (per design-capture.md)
- Target language selected
- Test framework selected for target language

## Document Organization

| Part | Sections | Description |
|------|----------|-------------|
| **I. Preparation** | §101-§103 | Verify inputs, setup test project |
| **II. Stub Generation** | §201-§205 | Generate minimal compilable declarations |
| **III. Test Generation** | §301-§308 | Generate all tests from captured design |
| **IV. Validation** | §401-§403 | Verify test suite completeness |

Section numbers (§) provide unique references across the document.

---

# PART I: PREPARATION

---

## I.A. Input Verification

### I.A.1. Verify Captured Design (§101)

#### PURPOSE
Confirm the captured design is complete and ready for test generation.

#### INPUT
- Path to captured-design directory

#### ACTION
Verify the following exist and are complete:

If <design-base> is not assigned, <design-base> = '.'

```
<design-base>/design/
├── scope.md                    ← Capture boundaries
├── modules.md                  ← Module inventory
├── sources.md                  ← Bibliography
├── implementation-order.md     ← Module ordering
└── modules/
    └── {each module}/
        ├── design.md           ← Data structures, algorithms
        ├── spec.md             ← Behavioral contracts
        ├── decisions.md        ← Design rationale
        └── tests.md            ← Conformance test specs
```

#### VALIDATION
```
[ ] scope.md exists
[ ] modules.md exists and lists all modules
[ ] implementation-order.md exists with tier assignments
[ ] Every module has design.md
[ ] Every module has spec.md
[ ] Every module has tests.md
[ ] No PENDING-USER discrepancies remain unresolved in any spec.md
```

#### NEXT
- IF any check fails: STOP. Complete design capture first.
- IF all checks pass: Proceed to §102.

---

### I.A.2. Select Target Language (§102)

#### PURPOSE
Document the target language and its implications for test generation.

#### INPUT
- User-specified target language

#### ACTION
Record the target language configuration:

```
Target Language: {language}
Test Framework: {framework}
Stub Placeholder: {how to mark unimplemented code}
Type Mapping: {reference to type translation}
```

#### LANGUAGE CONFIGURATION EXAMPLES

| Language | Test Framework | Stub Placeholder |
|----------|---------------|------------------|
| Rust | `#[cfg(test)]` + cargo test | `todo!()` or `unimplemented!()` |
| Zig | builtin test | `@panic("unimplemented")` |
| C | Unity, Check, or custom | `abort()` or `assert(0)` |
| C++ | Google Test, Catch2 | `throw std::logic_error("unimplemented")` |

#### OUTPUT
- Target language configuration documented

#### VALIDATION
```
[ ] Target language specified
[ ] Test framework selected
[ ] Stub placeholder syntax known
```

#### NEXT
Proceed to §103.

---

## I.B. Project Setup

### I.B.1. Create Test Project Structure (§103)

#### PURPOSE
Create the directory structure for the generated test suite.

#### INPUT
- Target language configuration from §102
- Module list from implementation-order.md

#### ACTION
Create directory structure:

```
{project}/
├── src/                        ← Stubs (later: implementation)
│   └── {module}/
│       └── {stub files}
├── tests/
│   ├── conformance/            ← From tests.md
│   │   └── {module}/
│   ├── comprehensive/          ← From dimension expansion
│   │   └── {module}/
│   │       ├── unit/
│   │       ├── property/
│   │       ├── concurrent/
│   │       └── stress/
│   └── common/                 ← Shared test utilities
├── test-matrix.md              ← Coverage tracking
└── deferred-tests.md           ← Tests awaiting implementation
```

#### OUTPUT
- Empty directory structure exists

#### VALIDATION
```
[ ] src/ directory exists
[ ] tests/conformance/ directory exists
[ ] tests/comprehensive/ directory exists
[ ] Directory per module in src/
[ ] Directory per module in tests/conformance/
[ ] Directory per module in tests/comprehensive/
```

#### NEXT
Proceed to §201.

---

# PART II: STUB GENERATION

This part generates minimal declarations needed for tests to compile.

---

## II.A. Type Stubs

### II.A.1. Extract Type Definitions (§201)

#### PURPOSE
Extract all type definitions from design.md files.

#### INPUT
- All design.md files from captured-design

#### ACTION
For each module's design.md, extract:

**Data Structures:**
- Name
- Fields (name, type, size)
- Generic parameters (if any)
- Visibility (public/private)

**Enumerations:**
- Name
- Variants
- Associated data (if any)

**Type Aliases:**
- Name
- Target type

Record in working notes, organized by module.

#### OUTPUT
- Complete list of types per module

#### VALIDATION
```
[ ] Every data structure from every design.md extracted
[ ] Every enumeration extracted
[ ] Every type alias extracted
[ ] Types organized by module
```

#### NEXT
Proceed to §202.

---

### II.A.2. Generate Type Stubs (§202)

#### PURPOSE
Generate compilable type definitions in target language.

#### INPUT
- Type definitions from §201
- Target language configuration from §102

#### ACTION
For each type, generate target language declaration:

**Translation Rules (Language-Agnostic to Concrete):**

| design.md | Description | Notes |
|-----------|-------------|-------|
| 8-bit unsigned integer | `u8`, `uint8_t`, etc. | Per target language |
| 32-bit signed integer | `i32`, `int32_t`, etc. | Per target language |
| Pointer to X | `*mut X`, `X*`, `*X` | Per target language |
| Array of X, length N | Fixed-size array | Per target language |
| Array of X, variable | Dynamic array/slice | Per target language |
| Optional X | `Option<X>`, `X*`, etc. | Per target language |
| Boolean | `bool` | Per target language |

**Structure Generation:**
```
For each structure in design.md:
  1. Create structure declaration with all fields
  2. Apply correct types per translation rules
  3. Mark visibility (public for API types)
  4. DO NOT generate methods yet (see §203)
```

**Enumeration Generation:**
```
For each enumeration in design.md:
  1. Create enum declaration with all variants
  2. Include associated data if any
  3. Mark visibility
```

#### OUTPUT
- Type stub files in src/{module}/

#### VALIDATION
```
[ ] Every extracted type has a stub
[ ] Stub files compile (syntax check only)
[ ] All types are accessible from test code
```

#### NEXT
Proceed to §203.

---

## II.B. Function Stubs

### II.B.1. Extract Function Signatures (§203)

#### PURPOSE
Extract all function/operation signatures from design.md files.

#### INPUT
- All design.md files from captured-design

#### ACTION
For each module's design.md, extract from Algorithms section:

**Per Function:**
- Name
- Parameters (name, type)
- Return type
- Associated type (if method)
- Visibility (public/private)

**Per Module:**
- Initialization functions
- Cleanup/destruction functions
- All public API functions
- Internal helpers (if documented)

Record in working notes, organized by module.

#### OUTPUT
- Complete list of function signatures per module

#### VALIDATION
```
[ ] Every operation from every design.md extracted
[ ] Parameters fully typed
[ ] Return types specified
[ ] Functions organized by module
```

#### NEXT
Proceed to §204.

---

### II.B.2. Generate Function Stubs (§204)

#### PURPOSE
Generate compilable function stubs with placeholder bodies.

#### INPUT
- Function signatures from §203
- Target language configuration from §102
- Type stubs from §202

#### ACTION
For each function, generate stub:

**Stub Structure:**
```
function_name(parameters) -> return_type {
    <placeholder>
}
```

Where `<placeholder>` is the language-appropriate unimplemented marker:
- Rust: `todo!()`
- Zig: `@panic("unimplemented")`
- C: `abort();` or `assert(0 && "unimplemented");`
- C++: `throw std::logic_error("unimplemented");`

**Method Generation:**
```
For methods (functions associated with a type):
  1. Generate as method/member function of the type
  2. Include self/this parameter appropriately
  3. Placeholder body
```

**Free Function Generation:**
```
For free functions:
  1. Generate as module-level function
  2. Full parameter list
  3. Placeholder body
```

#### OUTPUT
- Function stubs added to src/{module}/ files

#### VALIDATION
```
[ ] Every extracted function has a stub
[ ] All stubs have placeholder bodies
[ ] Stub files compile
```

#### NEXT
Proceed to §205.

---

## II.C. Stub Validation

### II.C.1. Verify Stubs Compile (§205)

#### PURPOSE
Verify all generated stubs compile successfully.

#### INPUT
- All stub files from §202 and §204

#### ACTION
1. Attempt to compile all stub files
2. Fix any compilation errors
3. Do NOT run any code (stubs will panic/abort)

**Common Issues:**
- Missing type imports
- Circular dependencies
- Visibility errors
- Generic parameter issues

#### OUTPUT
- All stubs compile without errors

#### VALIDATION
```
[ ] Compilation succeeds with no errors
[ ] Warnings reviewed (fix or document)
[ ] Each module's stubs compile independently
[ ] All modules compile together
```

#### NEXT
Proceed to §301.

---

# PART III: TEST GENERATION

This part generates all tests from the captured design.

---

## III.A. Conformance Tests

### III.A.1. Translate Conformance Tests (§301)

#### PURPOSE
Translate tests.md specifications into executable tests.

#### INPUT
- All tests.md files from captured-design
- Stubs from Part II

#### ACTION
For each module's tests.md, translate each TEST-{NNN} entry:

**tests.md format:**
```
### TEST-001: {test_name}

Category: {category}
Tests Requirement: {spec.md reference}

Setup:
1. {setup step}

Action:
1. {action step}

Expected Result:
- {expected outcome}

Cleanup:
1. {cleanup step}
```

**Translated test format (language-appropriate):**
```
test "{test_name}" {
    // Setup
    {translated setup steps}
    
    // Action
    {translated action steps}
    
    // Assert
    {translated expected results as assertions}
    
    // Cleanup
    {translated cleanup steps}
}
```

**Translation Rules:**

| tests.md | Translation |
|----------|-------------|
| "Create empty X" | Constructor/initialization call |
| "Call operation Y with Z" | Function call with arguments |
| "X contains Y" | Assertion checking state |
| "X equals Y" | Equality assertion |
| "X throws/returns error" | Error assertion (language-specific) |
| "X is unchanged" | Assert state equals prior state |

#### OUTPUT
- Test files in tests/conformance/{module}/

#### VALIDATION
```
[ ] Every TEST-{NNN} from tests.md has corresponding test
[ ] Test file compiles
[ ] Test references correct stub functions
```

#### NEXT
Proceed to §302.

---

### III.A.2. Map Existing Reference Tests (§301a)

#### PURPOSE
Ensure reference test mappings from tests.md are tracked.

#### INPUT
- "Existing Test Mapping" section from tests.md
- Translated conformance tests from §301

#### ACTION
Create mapping file `tests/conformance/{module}/reference-mapping.md`:

```markdown
# Reference Test Mapping: {module}

| Original Test | Original Location | Our Test | Status |
|---------------|-------------------|----------|--------|
| {name} | {path in reference repo} | TEST-{NNN} | Mapped |
```

This enables:
- Verification that we cover what reference tested
- Traceability back to original test intent
- Gap identification

#### OUTPUT
- reference-mapping.md per module

#### VALIDATION
```
[ ] Every "Existing Test Mapping" entry from tests.md is recorded
[ ] Our test IDs match tests.md TEST-{NNN} IDs
```

#### NEXT
Proceed to §302.

---

## III.B. Comprehensive Tests

### III.B.1. Analyze Test Dimensions (§302)

#### PURPOSE
Identify all dimensions requiring systematic test coverage.

#### INPUT
- design.md (data types, operations)
- spec.md (behavioral contracts)
- comprehensive-testing.md methodology (reference)

#### ACTION
For each module, enumerate dimensions:

**1. Data Type Dimensions:**
```
IF module has parametric/generic operations:
  List all types the operation must support
  Example: u8, u16, u32, u64, u128, usize for atomic operations
```

**2. Operation Dimensions:**
```
List all operations from design.md
Each operation × each applicable type = test needed
```

**3. Memory Ordering Dimensions (if concurrent):**
```
IF module uses atomic operations:
  List all orderings: Relaxed, Acquire, Release, AcqRel, SeqCst
  Each atomic operation × each ordering = test needed
```

**4. State Dimensions:**
```
List valid states from spec.md
- Initial state
- Intermediate states
- Terminal states
- Error states
```

**5. Thread Configuration Dimensions (if concurrent):**
```
IF module has concurrency:
  List thread counts: 1, 2, 4, 8, max_cores
  List contention patterns: none, low, high
```

**6. Edge Case Dimensions:**
```
For each parameter:
- Zero/empty
- One/single
- Maximum value
- Boundary values
- Null/none (if applicable)
```

#### OUTPUT
- Dimension analysis document per module

#### VALIDATION
```
[ ] All parametric types enumerated
[ ] All operations enumerated
[ ] All memory orderings enumerated (if applicable)
[ ] All states enumerated
[ ] All edge cases identified
```

#### NEXT
Proceed to §303.

---

### III.B.2. Calculate Test Matrix (§303)

#### PURPOSE
Calculate the full test matrix from dimension analysis.

#### INPUT
- Dimension analysis from §302

#### ACTION
Calculate required tests:

**Formula:**
```
Minimum tests = Operations × Types × Orderings × States × Configurations
               (where each dimension applies)
```

**Example:**
```
Module: atomic_operations
Operations: load, store, swap, cas = 4
Types: u8, u16, u32, u64, usize = 5
Orderings: Relaxed, Acquire, Release, AcqRel, SeqCst = 5
(States: N/A for stateless operations)

Minimum: 4 × 5 × 5 = 100 tests for basic coverage
```

**Document as test matrix:**

```markdown
# Test Matrix: {module}

## Dimensions

| Dimension | Values | Count |
|-----------|--------|-------|
| Operations | load, store, swap, cas | 4 |
| Types | u8, u16, u32, u64, usize | 5 |
| Orderings | Relaxed, Acquire, Release, AcqRel, SeqCst | 5 |

## Coverage Requirement

Minimum tests: 100
With edge cases: ~150
With concurrency scenarios: ~200

## Matrix

| Operation | Type | Ordering | Test ID |
|-----------|------|----------|---------|
| load | u8 | Relaxed | COMP-001 |
| load | u8 | Acquire | COMP-002 |
... (all combinations)
```

#### OUTPUT
- Test matrix document per module
- Total test count calculated

#### VALIDATION
```
[ ] All dimension combinations accounted for
[ ] Test IDs assigned to each combination
[ ] No gaps in matrix
```

#### NEXT
Proceed to §304.

---

### III.B.3. Generate Unit Tests (§304)

#### PURPOSE
Generate comprehensive unit tests from test matrix.

#### INPUT
- Test matrix from §303
- Stubs from Part II

#### ACTION
For each cell in test matrix, generate test:

**Naming Convention:**
```
test_{operation}_{type}_{ordering}_{scenario}
```

**Test Structure:**
```
test "COMP-{NNN}: {operation} with {type}, {ordering}" {
    // Setup: Create instance with specific type
    // Action: Perform operation
    // Assert: Verify postconditions from spec.md
}
```

**Generation by Category:**

| Category | What to Generate |
|----------|------------------|
| Basic operation | One test per operation × type × ordering |
| Return value | Verify correct return |
| State mutation | Verify state changes correctly |
| Error conditions | Verify errors raised appropriately |

#### OUTPUT
- Unit test files in tests/comprehensive/{module}/unit/

#### VALIDATION
```
[ ] Tests compile against stubs
[ ] Test count matches matrix
[ ] Each test has unique ID (COMP-{NNN})
```

#### NEXT
Proceed to §305.

---

### III.B.4. Generate Property-Based Tests (§305)

#### PURPOSE
Generate property-based test specifications from spec.md invariants.

#### INPUT
- spec.md (invariants, properties)
- Stubs from Part II

#### ACTION
For each invariant in spec.md, generate property test:

**Invariants to Properties:**

| spec.md Invariant | Property Test |
|-------------------|---------------|
| "Stack size ≥ 0" | `∀ ops: size(apply(ops, empty)) ≥ 0` |
| "LIFO ordering" | `∀ x,y: push(y, push(x, s)) then pop twice = y, x` |
| "CAS succeeds only if current = expected" | `∀ cur,exp,new: cas(exp,new) succeeds ⟺ current=exp` |

**Test Structure:**
```
property_test "{invariant name}" {
    // Generate random inputs
    inputs = generate(...)
    
    // Apply operations
    result = apply(inputs)
    
    // Check property holds
    assert(property(result))
}
```

**Properties to Extract:**
- Data structure invariants (from spec.md "Data Structure Invariants")
- Module-level invariants (from spec.md "Module-Level Invariants")
- Safety properties (from spec.md "Safety Properties")
- Liveness properties (from spec.md "Liveness Properties")

#### OUTPUT
- Property test files in tests/comprehensive/{module}/property/

#### VALIDATION
```
[ ] Every invariant from spec.md has property test
[ ] Property tests compile
[ ] Properties correctly encode invariants
```

#### NEXT
Proceed to §306.

---

### III.B.5. Generate Concurrent Tests (§306)

#### PURPOSE
Generate concurrent test scenarios for thread-safe modules.

#### INPUT
- spec.md (concurrency guarantees)
- design.md (atomic operations, synchronization)
- Stubs from Part II

#### ACTION

**IF module has no concurrency guarantees:**
```
Skip this section. Document: "Module is not thread-safe, no concurrent tests."
```

**IF module has concurrency guarantees:**

Generate tests for:

**1. Basic Thread Safety:**
```
test "concurrent_{operation}_thread_safety" {
    // Multiple threads perform same operation
    // Verify no data corruption
}
```

**2. Memory Ordering Verification:**
```
test "concurrent_{operation}_ordering_{ordering}" {
    // Thread 1: Write with Release
    // Thread 2: Read with Acquire
    // Verify ordering guarantees hold
}
```

**3. Progress Guarantees (if lock-free/wait-free):**
```
test "concurrent_{operation}_progress" {
    // Verify operation completes even under contention
}
```

**4. Linearizability (if claimed):**
```
test "concurrent_{operation}_linearizable" {
    // Record history of operations
    // Verify history is linearizable
}
```

**Thread Configurations:**
- 2 threads (minimal concurrency)
- 4 threads (moderate)
- num_cpus threads (realistic)
- 2× num_cpus threads (oversubscribed)

**For Model Checking (Loom):**
```
loom_test "concurrent_{scenario}" {
    // Loom will explore all interleavings
    // Test must be deterministic given interleaving
}
```

#### OUTPUT
- Concurrent test files in tests/comprehensive/{module}/concurrent/

#### VALIDATION
```
[ ] Thread safety tests for every concurrent operation
[ ] Memory ordering tests for every ordering guarantee
[ ] Progress tests for every progress guarantee
[ ] Loom test skeletons (if Rust)
```

#### NEXT
Proceed to §307.

---

### III.B.6. Generate Stress Tests (§307)

#### PURPOSE
Generate stress test specifications for high-load scenarios.

#### INPUT
- design.md (operations)
- perf.md (if exists - performance requirements)
- Stubs from Part II

#### ACTION
Generate stress tests for:

**1. Sustained Load:**
```
stress_test "sustained_{operation}" {
    duration = 60 seconds (or configurable)
    threads = num_cpus
    
    // Continuously perform operation
    // Verify no degradation
    // Verify no resource leaks
}
```

**2. Burst Load:**
```
stress_test "burst_{operation}" {
    // Sudden spike in operations
    // Verify system handles without failure
}
```

**3. Memory Pressure:**
```
stress_test "memory_pressure_{operation}" {
    // Many allocations/deallocations
    // Verify no leaks
    // Verify bounded memory growth
}
```

**4. Contention:**
```
stress_test "high_contention_{operation}" {
    threads = 4× num_cpus
    // All threads contending on same resource
    // Verify correctness under extreme contention
}
```

#### OUTPUT
- Stress test files in tests/comprehensive/{module}/stress/

#### VALIDATION
```
[ ] Sustained load test per major operation
[ ] Memory pressure test if module allocates
[ ] Contention test if module is concurrent
```

#### NEXT
Proceed to §308.

---

## III.C. Test Organization

### III.C.1. Organize by Implementation Order (§308)

#### PURPOSE
Ensure tests are organized to match implementation order.

#### INPUT
- implementation-order.md from captured-design
- All generated tests from §301-§307

#### ACTION
Create test execution plan:

```markdown
# Test Execution Plan

## Tier 0 (Foundation)

| Module | Conformance Tests | Comprehensive Tests | Status |
|--------|-------------------|---------------------|--------|
| {module} | tests/conformance/{module}/ | tests/comprehensive/{module}/ | Ready |

## Tier 1 (Depends on Tier 0)

| Module | Dependencies | Tests | Status |
|--------|--------------|-------|--------|
| {module} | {tier 0 deps} | {paths} | Ready |

... (continue for all tiers)

## Test Execution Order

1. Run Tier 0 tests (all must pass before Tier 1 implementation)
2. Run Tier 1 tests (all must pass before Tier 2 implementation)
... etc
```

Create test runner configuration that respects tiers:
```
// Run only tier 0 tests
test --tier 0

// Run tiers 0-1 tests
test --tier 1

// Run all tests
test --all
```

#### OUTPUT
- Test execution plan document
- Test runner configuration (if applicable)

#### VALIDATION
```
[ ] Every module appears in execution plan
[ ] Tier assignments match implementation-order.md
[ ] Test paths are correct
```

#### NEXT
Proceed to §401.

---

# PART IV: VALIDATION

---

## IV.A. Completeness Check

### IV.A.1. Verify Test Coverage (§401)

#### PURPOSE
Verify all requirements from captured design have tests.

#### INPUT
- All spec.md files
- All generated tests

#### ACTION
Create coverage matrix:

```markdown
# Test Coverage Matrix

## Module: {module}

### Operations

| Operation | Conformance | Unit | Property | Concurrent | Stress |
|-----------|-------------|------|----------|------------|--------|
| {op1} | TEST-001 | COMP-001..010 | PROP-001 | CONC-001 | STR-001 |
| {op2} | TEST-002 | COMP-011..020 | PROP-002 | CONC-002 | — |

### Invariants

| Invariant | Property Test |
|-----------|---------------|
| {inv1} | PROP-001 |
| {inv2} | PROP-002 |

### Error Conditions

| Condition | Test |
|-----------|------|
| {err1} | TEST-003, COMP-015 |

### Gaps

| Requirement | Reason | Plan |
|-------------|--------|------|
| (none if complete) | | |
```

#### OUTPUT
- Coverage matrix per module
- Global coverage summary

#### VALIDATION
```
[ ] Every operation has at least one conformance test
[ ] Every operation has comprehensive unit tests
[ ] Every invariant has property test
[ ] Every error condition has test
[ ] Gaps documented with reasons
```

#### NEXT
Proceed to §402.

---

### IV.A.2. Verify Tests Compile (§402)

#### PURPOSE
Verify entire test suite compiles against stubs.

#### INPUT
- All stub files
- All test files

#### ACTION
1. Compile all stubs
2. Compile all tests (without running)
3. Fix any compilation errors
4. Document any tests that cannot compile until implementation exists

**Expected Outcome:**
- All tests compile
- No test runs successfully (stubs are unimplemented)
- Running any test produces "unimplemented" error

#### OUTPUT
- Clean compilation of entire test suite

#### VALIDATION
```
[ ] All stubs compile
[ ] All tests compile
[ ] No runtime execution attempted
```

#### NEXT
Proceed to §403.

---

## IV.B. Final Validation

### IV.B.1. Verify Red Phase (§403)

#### PURPOSE
Confirm tests are in proper TDD "red" state.

#### INPUT
- Compiled test suite
- Compiled stubs

#### ACTION
Attempt to run a sample of tests to verify they fail:

```
For one test per module:
  1. Run the test
  2. Verify it fails with "unimplemented" / todo! / abort
  3. This confirms:
     - Test is actually testing the stub
     - Stub is properly unimplemented
     - Test will turn green when implemented
```

**DO NOT:**
- Run entire test suite (wastes time, all will fail)
- Try to make any test pass (that's implementation phase)

#### OUTPUT
- Confirmation that tests properly fail against stubs

#### VALIDATION
```
[ ] Sample test from each tier executed
[ ] Each sample test fails with "unimplemented"
[ ] No test accidentally passes (would indicate stub is wrong or test is wrong)
```

#### COMPLETION
```
============================================================
TEST SUITE GENERATION COMPLETE

Project: {project name}
Date: {completion date}
Target Language: {language}

Summary:
- Modules: {count}
- Stubs generated: {count} types, {count} functions
- Conformance tests: {count}
- Comprehensive tests: {count}
- Total tests: {count}

All tests compile against stubs.
All tests fail (red phase confirmed).

Ready for implementation per implementation-order.md.
============================================================
```

---

# APPENDICES

---

## Appendix A: Type Translation Reference

| design.md Type | Rust | Zig | C | C++ |
|----------------|------|-----|---|-----|
| 8-bit unsigned | `u8` | `u8` | `uint8_t` | `uint8_t` |
| 16-bit unsigned | `u16` | `u16` | `uint16_t` | `uint16_t` |
| 32-bit unsigned | `u32` | `u32` | `uint32_t` | `uint32_t` |
| 64-bit unsigned | `u64` | `u64` | `uint64_t` | `uint64_t` |
| 8-bit signed | `i8` | `i8` | `int8_t` | `int8_t` |
| 16-bit signed | `i16` | `i16` | `int16_t` | `int16_t` |
| 32-bit signed | `i32` | `i32` | `int32_t` | `int32_t` |
| 64-bit signed | `i64` | `i64` | `int64_t` | `int64_t` |
| Pointer size unsigned | `usize` | `usize` | `size_t` | `size_t` |
| Pointer size signed | `isize` | `isize` | `ssize_t` | `ssize_t` |
| Boolean | `bool` | `bool` | `bool` | `bool` |
| Pointer to X | `*mut X` / `*const X` | `*X` / `[*]X` | `X*` | `X*` |
| Optional X | `Option<X>` | `?X` | `X*` (nullable) | `std::optional<X>` |
| Array of X, length N | `[X; N]` | `[N]X` | `X[N]` | `std::array<X, N>` |
| Dynamic array of X | `Vec<X>` | `[]X` (slice) | `X*` + length | `std::vector<X>` |

---

## Appendix B: Stub Placeholder Reference

| Language | Placeholder | Behavior |
|----------|-------------|----------|
| Rust | `todo!()` | Panics with "not yet implemented" |
| Rust | `unimplemented!()` | Panics with "not implemented" |
| Zig | `@panic("unimplemented")` | Panics |
| Zig | `unreachable` | Safety-checked unreachable |
| C | `abort()` | Terminates program |
| C | `assert(0 && "unimplemented")` | Assertion failure |
| C++ | `throw std::logic_error("unimplemented")` | Exception |
| C++ | `std::abort()` | Terminates program |

---

## Appendix C: Test Framework Reference

| Language | Framework | Test Declaration | Assertion |
|----------|-----------|------------------|-----------|
| Rust | builtin | `#[test] fn name()` | `assert!`, `assert_eq!` |
| Rust | proptest | `proptest! { \|x\| ... }` | `prop_assert!` |
| Rust | loom | `#[test] fn name() { loom::model(...) }` | standard |
| Zig | builtin | `test "name" { }` | `try std.testing.expect()` |
| C | Unity | `void test_name(void)` | `TEST_ASSERT()` |
| C | Check | `START_TEST(name) ... END_TEST` | `ck_assert()` |
| C++ | Google Test | `TEST(Suite, Name)` | `EXPECT_EQ()`, `ASSERT_EQ()` |
| C++ | Catch2 | `TEST_CASE("name")` | `REQUIRE()`, `CHECK()` |

---

## Appendix D: Section Reference

| Section | Title | Purpose |
|---------|-------|---------|
| §101 | Verify Captured Design | Confirm inputs complete |
| §102 | Select Target Language | Document language config |
| §103 | Create Test Project Structure | Setup directories |
| §201 | Extract Type Definitions | Gather types from design |
| §202 | Generate Type Stubs | Create compilable types |
| §203 | Extract Function Signatures | Gather functions from design |
| §204 | Generate Function Stubs | Create compilable stubs |
| §205 | Verify Stubs Compile | Compilation check |
| §301 | Translate Conformance Tests | tests.md → executable |
| §301a | Map Existing Reference Tests | Traceability |
| §302 | Analyze Test Dimensions | Identify coverage needs |
| §303 | Calculate Test Matrix | Compute required tests |
| §304 | Generate Unit Tests | Comprehensive unit tests |
| §305 | Generate Property-Based Tests | Invariant tests |
| §306 | Generate Concurrent Tests | Thread safety tests |
| §307 | Generate Stress Tests | Load tests |
| §308 | Organize by Implementation Order | Match tiers |
| §401 | Verify Test Coverage | Coverage matrix |
| §402 | Verify Tests Compile | Compilation check |
| §403 | Verify Red Phase | TDD red state |

---

## Appendix E: Glossary

| Term | Definition |
|------|------------|
| **Conformance Test** | Test derived from tests.md verifying reference behavior |
| **Comprehensive Test** | Test derived from systematic dimension analysis |
| **Dimension** | Axis of variation requiring test coverage (type, ordering, state) |
| **Property Test** | Test verifying an invariant holds across random inputs |
| **Red Phase** | TDD state where tests exist but fail (no implementation) |
| **Stub** | Minimal declaration allowing compilation without implementation |
| **Test Matrix** | Cartesian product of dimensions defining required tests |
| **Tier** | Group of modules at same dependency level |

---

# Document Information

| Field | Value |
|-------|-------|
| Version | 1.0 |
| Created | 2025-12-06 |
| Author | Claude (Anthropic) |
| Purpose | Generate TDD test suite from captured design |
| Depends On | design-capture.md, comprehensive-testing.md |

