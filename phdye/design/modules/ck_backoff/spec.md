# Module: ck_backoff — Specification

## Operations

### ck_backoff_eb

**Signature:**
```
ck_backoff_eb(c: Pointer to 32-bit unsigned integer) → void
```

**Preconditions:**
- c must not be NULL [INFERRED]
- *c should be initialized (e.g., to CK_BACKOFF_INITIALIZER) [OBSERVED]

**Postconditions:**
- Delay of approximately *c iterations has occurred [SPECIFIED]
- IF *c < CK_BACKOFF_CEILING: *c is doubled [SPECIFIED]
- IF *c >= CK_BACKOFF_CEILING: *c is unchanged [SPECIFIED]

**Invariants Preserved:**
- *c never exceeds CK_BACKOFF_CEILING after operation [SPECIFIED]

**Error Conditions:**

| Condition | Behavior | Certainty |
|-----------|----------|-----------|
| c is NULL | Undefined behavior | INFERRED |
| *c is 0 | No delay, *c becomes 0 (stuck) | OBSERVED |

**Concurrency:**
- Thread Safety: Not thread-safe; each thread needs own state [SPECIFIED]
- Memory Ordering: N/A (local variable) [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

## Data Structure Invariants

### ck_backoff_t

- Value should be initialized before use [OBSERVED]
- Value grows exponentially until ceiling [SPECIFIED]

## Module-Level Invariants

- CK_BACKOFF_CEILING is a compile-time constant [SPECIFIED]
- CK_BACKOFF_INITIALIZER provides reasonable starting point [OBSERVED]

## Safety Properties

**Bounded Delay:** Maximum delay is bounded by CK_BACKOFF_CEILING iterations. [SPECIFIED]

**No Overflow:** Backoff value cannot exceed ceiling due to conditional shift. [SPECIFIED]

## Liveness Properties

**Termination:** ck_backoff_eb always terminates in bounded time. [SPECIFIED]

## Behavioral Ambiguities

### Zero initialization behavior

**Observed Behavior:** If *c is 0, no delay occurs and *c remains 0

**Intent:** UNKNOWN - zero is not a valid state

**Recommendation:** Always initialize with CK_BACKOFF_INITIALIZER. Document that zero is invalid.

### Ceiling behavior

**Observed Behavior:** Once ceiling is reached, backoff stays at ceiling

**Intent:** SPECIFIED - prevents unbounded growth

**Recommendation:** For very long waits, caller should reset backoff or yield to scheduler.

## Discrepancies

No discrepancies detected between sources.
