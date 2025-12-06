# Module: ck_malloc — Design Decisions

## Decision: Custom allocator interface via function pointers

**Context:**
CK data structures need to allocate and free memory. The library must work with different allocation strategies (standard malloc, memory pools, NUMA-aware allocators, etc.).

**Options Considered:**

1. Use standard malloc/free directly
   - Pro: Simple, no abstraction layer
   - Con: Cannot integrate with custom allocators
   - Con: No sized-delete support

2. Use preprocessor macros for allocation
   - Pro: Zero runtime overhead
   - Con: Cannot change allocator at runtime
   - Con: Requires recompilation

3. Use function pointer structure
   - Pro: Runtime allocator selection
   - Pro: Supports custom allocators
   - Pro: Clean interface
   - Con: Indirect call overhead

**Decision:** Use function pointer structure (option 3)

**Rationale:** Flexibility is more important than the minor overhead of indirect calls. Many applications have specific memory management requirements that a library cannot anticipate.

**Rationale Source:** Common pattern in C libraries; explicit design choice visible in code

**Consequences:**
- All data structures that allocate memory accept a `struct ck_malloc *` parameter
- Users must provide allocator implementation
- Enables memory pool integration, NUMA-aware allocation, etc.

---

## Decision: Extended realloc signature with may_move parameter

**Context:**
Standard realloc may return a different pointer than the input. Some concurrent algorithms require resize-in-place semantics to maintain pointer validity.

**Options Considered:**

1. Use standard realloc signature
   - Pro: Familiar API
   - Con: Cannot specify in-place requirement

2. Add may_move parameter
   - Pro: Allows caller to require in-place resize
   - Pro: Enables concurrent resize operations
   - Con: Non-standard signature

**Decision:** Add may_move parameter (option 2)

**Rationale:** Some concurrent hash table algorithms can safely resize if the buffer doesn't move. This is a critical optimization for certain use cases.

**Rationale Source:** Code comment, observed usage in ck_hs, ck_ht, ck_rhs

**Consequences:**
- Custom allocators must handle may_move=false case
- Standard realloc cannot be used directly (wrapper required)
- Enables more efficient concurrent resize operations

---

## Decision: Extended free signature with size and defer parameters

**Context:**
Standard free takes only a pointer. Additional information can enable optimizations.

**Options Considered:**

1. Use standard free signature
   - Pro: Familiar API
   - Con: Cannot support sized-delete
   - Con: Cannot support deferred reclamation

2. Add size and defer parameters
   - Pro: Enables sized-delete optimization
   - Pro: Enables deferred batch reclamation
   - Con: Non-standard signature

**Decision:** Add size and defer parameters (option 2)

**Rationale:**
- Size enables allocators with sized delete pools (common in high-performance allocators)
- Defer enables integration with epoch-based reclamation and batch deallocation

**Rationale Source:** UNKNOWN - inferred from parameter usage patterns

**Possible Reasons:**
- Performance optimization for allocators that track size
- Integration with safe memory reclamation (ck_epoch, ck_hp)

**Sources Checked:**
- [x] Code comments: not found
- [x] Commit history: not found
- [x] Documentation: not found
- [x] Mailing list: not found

**Recommendation:** Preserve the interface; it enables important optimizations

---

## Decision: Separate old_size and new_size in realloc

**Context:**
Standard realloc only takes new_size. Some allocators benefit from knowing the old size.

**Options Considered:**

1. Use standard realloc(ptr, new_size)
   - Pro: Familiar API
   - Con: Allocator must track size internally

2. Provide both sizes: realloc(ptr, old_size, new_size, may_move)
   - Pro: Allocator doesn't need to track size
   - Pro: Enables sized-pool allocators
   - Con: Caller must track size

**Decision:** Provide both sizes (option 2)

**Rationale:** Consistent with the sized-delete philosophy in free(). The caller (CK data structures) always knows the current size, so passing it is trivial.

**Rationale Source:** Code structure, consistency with free signature

**Consequences:**
- CK data structures must track allocation sizes
- Enables more allocator implementations
