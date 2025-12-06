# Module: ck_pr — Design Decisions

## Decision: Memory model abstraction via compile-time selection

**Context:**
Different architectures have different memory models (TSO, PSO, RMO) requiring different fence instructions.

**Options Considered:**

1. Single strongest fence everywhere
   - Pro: Simple, always correct
   - Con: Massive performance penalty on TSO architectures (unnecessary fences)

2. Runtime memory model detection
   - Pro: Single binary for all platforms
   - Con: Runtime overhead, complex dispatch

3. Compile-time memory model selection via CK_MD_*
   - Pro: Zero runtime overhead
   - Pro: Optimal fences per platform
   - Con: Separate compilation per platform

**Decision:** Compile-time selection (option 3)

**Rationale:** Performance is paramount for concurrency primitives. The small cost of separate compilation is far outweighed by the performance benefit of eliminating unnecessary fences.

**Rationale Source:** Code structure in ck_pr.h showing CK_MD_TSO/PSO/RMO selection

**Consequences:**
- Configure script sets CK_MD_* based on detected platform
- Different fence implementations based on memory model
- Most fences become no-ops on TSO (x86)

---

## Decision: Use volatile + compiler barrier for loads/stores

**Context:**
Atomic loads and stores can be implemented multiple ways.

**Options Considered:**

1. Use C11 atomics (_Atomic)
   - Pro: Standard, portable
   - Con: Not available in C99
   - Con: Less control over generated code

2. Use GCC atomic builtins (__atomic_load_n)
   - Pro: Well-supported
   - Con: May generate stronger ordering than needed

3. Use volatile + compiler barrier
   - Pro: Minimal overhead
   - Pro: Precise control over ordering
   - Con: Relies on platform guarantees for aligned access atomicity

**Decision:** Use volatile + compiler barrier (option 3)

**Rationale:** On all supported platforms, naturally aligned loads and stores are atomic at the hardware level. Using volatile prevents compiler reordering, and explicit fences control hardware ordering. This gives optimal performance.

**Rationale Source:** Implementation in gcc/ck_pr.h

**Consequences:**
- Loads/stores emit no hardware barriers (only compiler barriers)
- Users must add explicit fences for ordering beyond relaxed
- Maximum performance for simple cases

---

## Decision: CAS-based fallbacks for missing hardware operations

**Context:**
Not all operations (e.g., fetch-and-add for all sizes) have direct hardware support on all platforms.

**Options Considered:**

1. Only provide operations with hardware support
   - Pro: Guaranteed performance
   - Con: Non-uniform API across platforms

2. Provide CAS-based fallbacks
   - Pro: Uniform API
   - Pro: Correctness guaranteed
   - Con: Potential performance penalty under contention

**Decision:** Provide CAS-based fallbacks (option 2)

**Rationale:** API uniformity is more important than guaranteed performance. CAS loops are correct and typically fast enough. Feature flags (CK_F_PR_*) allow detection of hardware support.

**Rationale Source:** CK_PR_BIN macro and fallback implementations in ck_pr.h

**Consequences:**
- All operations available on all platforms
- CK_F_PR_* flags indicate hardware support
- Performance may vary by platform for some operations

---

## Decision: ck_pr_stall for spin loops

**Context:**
Spin loops waste CPU resources and can cause performance problems (SMT thread starvation, power consumption).

**Options Considered:**

1. Just spin with no hint
   - Pro: Simple
   - Con: Wastes power, starves sibling threads

2. Use PAUSE instruction or equivalent
   - Pro: Reduces power consumption
   - Pro: Improves SMT performance
   - Con: Platform-specific

**Decision:** Provide ck_pr_stall() with PAUSE or equivalent (option 2)

**Rationale:** Modern CPUs benefit significantly from pause hints in spin loops.

**Rationale Source:** Implementation shows PAUSE on x86, barrier on others

**Consequences:**
- Spin loops should call ck_pr_stall()
- Platform-specific implementation provides best behavior

---

## Decision: Type-specific operation variants

**Context:**
Atomic operations could be generic (void*) or type-specific.

**Options Considered:**

1. Single generic void* interface
   - Pro: Simple API
   - Con: No type safety
   - Con: Cannot optimize for specific sizes

2. Type-specific variants (ck_pr_*_64, ck_pr_*_32, etc.)
   - Pro: Type safety
   - Pro: Can optimize per size
   - Con: API proliferation

**Decision:** Type-specific variants (option 2)

**Rationale:** Type safety catches bugs at compile time. Different sizes may have different hardware support.

**Rationale Source:** API design with _64, _32, _16, _8, _ptr, _int, _uint variants

**Consequences:**
- Large API surface with many variants
- Macros used to generate variants (CK_PR_LOAD, etc.)
- Clear type expectations for users

---

## Decision: Sequential consistency for RMW operations

**Context:**
Read-modify-write operations could have various memory orderings.

**Options Considered:**

1. Relaxed ordering for all operations
   - Pro: Maximum performance
   - Con: Difficult to reason about, error-prone

2. Sequential consistency for all operations
   - Pro: Easiest to reason about
   - Pro: Matches common mental model
   - Con: May be stronger than necessary

3. Per-operation ordering selection
   - Pro: Maximum flexibility
   - Con: Complex API, error-prone

**Decision:** Sequential consistency for RMW operations (option 2)

**Rationale:** Sequential consistency is the safest default. Users who need weaker ordering can use loads/stores with explicit fences.

**Rationale Source:** GCC __sync_* builtins provide sequential consistency

**Consequences:**
- All RMW operations are fully ordered
- Simple mental model for users
- Possible to optimize manually with loads/stores + fences if needed

---

## Decision: Support for static analysis tools

**Context:**
Static analysis tools (Coverity, Clang analyzer, sparse) may not understand inline assembly.

**Options Considered:**

1. Ignore analysis tool compatibility
   - Pro: Simpler code
   - Con: False positives in analysis tools

2. Provide builtin-based alternative for analysis
   - Pro: Better static analysis coverage
   - Con: Code complexity

**Decision:** Provide builtin-based alternative (option 2)

**Rationale:** Static analysis is valuable for finding bugs. The builtin-based code is semantically equivalent and enables analysis.

**Rationale Source:** CK_USE_CC_BUILTINS flag and conditional compilation

**Consequences:**
- CK_USE_CC_BUILTINS auto-enables for known analysis tools
- Code is analyzable when flag is set
- Minor code duplication
