# Module: ck_ec — Design Decisions

## Decision: Flag bit position differs between 32-bit and 64-bit

**Context:**
Event counts need a flag bit to signal that waiters need explicit wakeup.

**Options Considered:**

1. Consistent position (e.g., always MSB)
   - Pro: Simpler mental model
   - Con: 64-bit futex compatibility issues

2. Different positions (MSB for 32-bit, LSB for 64-bit)
   - Pro: 64-bit works with 32-bit futexes
   - Con: Different extraction logic

**Decision:** MSB for 32-bit (bit 31), LSB for 64-bit (bit 0)

**Rationale:** Futexes typically operate on 32-bit words. For 64-bit counters, placing the flag in the LSB means the low 32 bits contain the flag, allowing wait64 to use the low half of the address with 32-bit futexes on little-endian systems.

**Rationale Source:** Header comment: "the futex int lies in the first 4 (bottom) bytes"

**Consequences:**
- 32-bit: value = counter & ~(1<<31)
- 64-bit: value = counter >> 1
- 64-bit works with 32-bit OS primitives on little-endian

---

## Decision: Non-atomic RMW for single-producer mode

**Context:**
Single-producer updates could use atomic or non-atomic instructions.

**Options Considered:**

1. Always use atomic instructions
   - Pro: Portable, straightforward
   - Con: Higher overhead

2. Non-atomic RMW on x86 (inc mem, xadd)
   - Pro: Faster, single instruction
   - Con: x86-specific, requires careful correctness argument

**Decision:** Non-atomic RMW for single producer on x86 (option 2)

**Rationale:** x86-TSO guarantees reads see most recent local store or memory value. A non-atomic RMW is a single instruction that cannot be split by preemption, making it safe even when mixing with atomic flag-setting by waiters.

**Rationale Source:** Extensive implementation notes in header citing x86-TSO semantics

**Consequences:**
- Faster single-producer path on x86/x86_64
- Falls back to atomic path on other architectures
- Requires CK_F_EC_SP feature flag

---

## Decision: Exponential backoff with eventual infinite timeout

**Context:**
Waiters need to handle the case where their flag-set was overwritten by a non-atomic producer update.

**Options Considered:**

1. Simple futex wait with fixed timeout
   - Pro: Simple
   - Con: May miss wakeups or waste CPU

2. Exponential backoff, then infinite
   - Pro: Handles rare flag-overwrite, avoids busy-wait
   - Con: Complex timeout management

**Decision:** Exponential backoff transitioning to infinite timeout (option 2)

**Rationale:** After 1 second, any x86 instruction will have completed (interrupts/preemption force architectural state commit). Beyond that point, the flag is guaranteed visible to the producer, so infinite wait is safe.

**Rationale Source:** Header: "Eventually, more than one second will have elapsed since the flag flip, and the sleep timeout becomes infinite"

**Consequences:**
- Initial backoff: initial_wait_ns (default 2ms)
- Scale: (wait * scale_factor) >> shift_count
- After 1 second: infinite deadline
- Configurable via ck_ec_ops fields

---

## Decision: Predicate-based waiting

**Context:**
Consumers may need to wake on conditions other than counter change.

**Options Considered:**

1. Only wait on counter value
   - Pro: Simple API
   - Con: Can't handle multiple wake conditions

2. User-provided predicate checked during wait
   - Pro: Flexible, can check multiple conditions
   - Con: More complex API

**Decision:** Support predicate-based waiting (option 2)

**Rationale:** Real-world use cases need to wait on multiple conditions (e.g., timeout, secondary event count, application state). Predicate receives wait state and can modify iteration deadline.

**Rationale Source:** ck_ec_wait_pred API and header comment about "optimistically looking at other waking conditions"

**Consequences:**
- ck_ec_wait_pred accepts optional predicate function
- Predicate called before each futex_wait
- Predicate can cause early return or adjust deadline
- NULL predicate behaves as ck_ec_wait

---

## Decision: Spin before blocking

**Context:**
Transitioning to OS-level blocking has latency cost.

**Options Considered:**

1. Block immediately
   - Pro: CPU efficient for long waits
   - Con: High latency for short waits

2. Spin then block
   - Pro: Low latency for short waits
   - Con: Wastes CPU if producer is slow

**Decision:** Spin loop before blocking (option 2)

**Rationale:** For single-producer mode, spinning ~100 iterations gives time for in-flight store queue entries to drain. Most producer updates complete quickly, avoiding syscall overhead.

**Rationale Source:** busy_loop_iter field, default 100

**Consequences:**
- Default 100 iterations of spinning
- Tunable via ck_ec_ops.busy_loop_iter
- Reduces syscall overhead for responsive producers

---

## Decision: Ops struct for platform abstraction

**Context:**
Event counts need OS-specific time and futex operations.

**Options Considered:**

1. Compile-time platform selection
   - Pro: No runtime overhead
   - Con: Less flexible, harder to test

2. Runtime ops function pointers
   - Pro: Flexible, testable
   - Con: Indirect call overhead

**Decision:** Runtime ops struct (option 2)

**Rationale:** Allows different ops for testing, different clock sources, or custom blocking primitives. Ops pointer is passed through ck_ec_mode, enabling per-use customization.

**Rationale Source:** struct ck_ec_ops with function pointers for gettime, wait32/64, wake32/64

**Consequences:**
- Platform provides const ck_ec_ops struct
- ck_ec_mode wraps ops + single_producer flag
- Indirect calls only on slow path (wait/wake)
- Fast path (inc/add/value) stays inline
