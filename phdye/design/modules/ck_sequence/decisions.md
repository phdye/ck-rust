# Module: ck_sequence — Design Decisions

## Decision: Odd/even sequence convention

**Context:**
Need to indicate whether a write is in progress.

**Options Considered:**

1. Separate flag variable
   - Pro: Clear semantics
   - Con: Additional memory, potential cache issues

2. Odd = writing, even = stable
   - Pro: Single variable
   - Pro: Increment-only operations
   - Con: Requires convention understanding

**Decision:** Odd/even convention (option 2)

**Rationale:** Using the LSB of the sequence as a write-in-progress flag is efficient and well-established in seqlock implementations.

**Rationale Source:** Code comments explaining odd = update in progress

**Consequences:**
- Single atomic variable for both version and state
- Readers spin while sequence is odd
- Writers increment twice per update

---

## Decision: Require external mutex for writers

**Context:**
Multiple concurrent writers need coordination.

**Options Considered:**

1. Lock-free writer coordination
   - Pro: No external mutex needed
   - Con: Complex, potential for high contention

2. Require external mutex
   - Pro: Simple implementation
   - Pro: Flexible mutex choice
   - Con: User must manage mutex

**Decision:** Require external mutex (option 2)

**Rationale:** Seqlocks are designed for read-mostly workloads. Writer coordination via external mutex is simple and allows users to choose appropriate locking strategy.

**Rationale Source:** Comments stating "must be called after a successful mutex acquisition"

**Consequences:**
- User provides mutex for writer exclusion
- Seqlock only handles reader/writer visibility
- Simple, composable design

---

## Decision: Blocking readers during write

**Context:**
What should readers do when sequence is odd?

**Options Considered:**

1. Return odd sequence, let caller handle
   - Pro: Non-blocking
   - Con: Complex caller logic

2. Spin until even
   - Pro: Simple caller pattern
   - Pro: Guarantees usable version
   - Con: May spin under contention

**Decision:** Spin until even (option 2)

**Rationale:** Returning immediately with an odd sequence would require callers to handle another case. Spinning until even simplifies the read pattern.

**Rationale Source:** Loop in read_begin waiting for even sequence

**Consequences:**
- read_begin blocks during active writes
- Returned version is always even
- Readers may briefly spin

---

## Decision: CK_SEQUENCE_READ convenience macro

**Context:**
The read-then-retry pattern is verbose.

**Options Considered:**

1. Only provide primitives
   - Pro: Simpler implementation
   - Con: Verbose usage pattern

2. Provide macro for common pattern
   - Pro: Concise usage
   - Pro: Harder to misuse
   - Con: Macro complexity

**Decision:** Provide macro (option 2)

**Rationale:** The CK_SEQUENCE_READ macro encapsulates the standard retry loop, making correct usage easier.

**Rationale Source:** CK_SEQUENCE_READ macro definition

**Consequences:**
- Common pattern is one line
- Version variable indicates success (0) or must continue
- Still possible to use primitives directly
