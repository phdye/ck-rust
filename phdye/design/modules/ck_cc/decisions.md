# Module: ck_cc — Design Decisions

## Decision: Use compiler builtins when available

**Context:**
Bit manipulation operations (ffs, ctz, popcount) can be implemented either as portable C loops or using compiler-specific builtins that map to single hardware instructions.

**Options Considered:**

1. Always use portable C implementations
   - Pro: Maximum portability, no compiler dependencies
   - Con: Much slower (O(n) vs O(1) for hardware instructions)

2. Use compiler builtins when available, fall back to portable
   - Pro: Optimal performance on supported compilers
   - Pro: Maintains portability as fallback
   - Con: Code duplication between implementations

**Decision:** Use compiler builtins when available (option 2)

**Rationale:** In a concurrency library, these operations are often on hot paths (e.g., lock acquisition, bitmap scanning). The performance difference between a hardware instruction and a loop is significant.

**Rationale Source:** Code structure in include/ck_cc.h and include/gcc/ck_cc.h

**Consequences:**
- Dual implementation path must be maintained
- Feature flags (CK_F_CC_*) indicate which implementation is in use
- CK_MD_CC_BUILTIN_DISABLE allows forcing portable implementations

---

## Decision: 1-indexed return for ffs functions

**Context:**
Different systems have different conventions for find-first-set: some return 0-indexed positions, others return 1-indexed.

**Options Considered:**

1. Return 0-indexed position (0 = bit 0)
   - Pro: Natural for C programmers
   - Con: Cannot distinguish "no bits set" from "bit 0 set"

2. Return 1-indexed position (1 = bit 0), 0 for no bits set
   - Pro: Matches POSIX ffs() semantics
   - Pro: Zero is unambiguous "not found" value
   - Con: Requires subtract-1 for direct bit indexing

**Decision:** Use 1-indexed return values (option 2)

**Rationale:** Matches POSIX ffs() semantics. The return value can be used directly as a boolean (non-zero = found).

**Rationale Source:** POSIX specification, code implementation

**Consequences:** Users must subtract 1 from return value to get bit index

---

## Decision: Define behavior for zero input to ctz

**Context:**
The count-trailing-zeros operation has undefined behavior on zero input in many implementations (including __builtin_ctz).

**Options Considered:**

1. Leave undefined (follow __builtin_ctz behavior)
   - Pro: Matches compiler builtin
   - Con: Risk of undefined behavior propagation

2. Define as returning 0 for zero input
   - Pro: Predictable behavior
   - Pro: Defensive programming
   - Con: Masks potential bugs (caller might not expect this)

**Decision:** Return 0 for zero input (option 2)

**Rationale:** UNKNOWN

**Possible Reasons:**
- Defensive programming to avoid undefined behavior
- Consistent with ffs behavior (zero returns zero)
- May indicate zero trailing zeros is a reasonable interpretation

**Sources Checked:**
- [x] Code comments: not found
- [x] Commit history: not found
- [x] Documentation: not found
- [x] Mailing list: not found

**Recommendation:** Preserve behavior; document as CK-specific extension

---

## Decision: Use GCC attributes for optimization hints

**Context:**
Compiler attributes provide hints for optimization (inlining, alignment, branch prediction) but reduce portability.

**Options Considered:**

1. Use standard C99 only
   - Pro: Maximum portability
   - Con: Lose significant optimization opportunities

2. Use compiler attributes with fallback macros
   - Pro: Best performance on supported compilers
   - Pro: Graceful degradation on other compilers
   - Con: Complexity of dual-path macros

**Decision:** Use GCC attributes with empty fallbacks (option 2)

**Rationale:** Performance is critical for a concurrency library. The abstraction layer (ck_cc) exists precisely to hide these compiler differences.

**Rationale Source:** Design of the module itself

**Consequences:**
- All attributes wrapped in CK_CC_* macros
- Fallback definitions provided in include/ck_cc.h
- Platform-specific definitions in include/gcc/ck_cc.h

---

## Decision: Provide CK_CC_CONTAINER macro

**Context:**
Need to recover pointer to containing structure from pointer to member (Linux kernel container_of pattern).

**Options Considered:**

1. Require users to implement their own container_of
   - Pro: No dependency on implementation details
   - Con: Error-prone, every user reimplements

2. Provide CK_CC_CONTAINER macro
   - Pro: Standard pattern available
   - Pro: Can use __builtin_offsetof when available
   - Con: Relies on implementation-defined behavior (negative pointer arithmetic)

**Decision:** Provide CK_CC_CONTAINER (option 2)

**Rationale:** The container-of pattern is essential for intrusive data structures, which are common in concurrent code to avoid separate allocations.

**Rationale Source:** Code comment "This relies on (compiler) implementation-defined behavior"

**Consequences:**
- Uses __builtin_offsetof on GCC for safety
- Falls back to (T*)0->M trick on other compilers
- Users should be aware this is technically implementation-defined
