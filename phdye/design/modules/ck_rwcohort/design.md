# Module: ck_rwcohort

## Overview

The ck_rwcohort module implements NUMA-aware reader-writer locks based on lock cohorting. It extends the cohort lock concept to support multiple concurrent readers while maintaining NUMA locality. Three variants are provided: writer-preference (WP), reader-preference (RP), and neutral, each with different fairness characteristics.

## Dependencies

| Dependency | Type | Purpose |
|------------|------|---------|
| ck_cc | Internal | Inline hints |
| ck_pr | Internal | Atomic operations, barriers |
| ck_stddef | External | NULL definition |
| ck_cohort | Internal | Base cohort lock |

## Data Structures

### CK_RWCOHORT_WP_INSTANCE (Writer-Preference)

**Description:** Writer-preference reader-writer cohort lock.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| read_counter | unsigned int | 4 bytes | Active reader count |
| write_barrier | unsigned int | 4 bytes | Waiting writers barrier |
| wait_limit | unsigned int | 4 bytes | Iterations before raising barrier |

**Behavior:** Writers block new readers by raising write_barrier. Readers back off when barrier raised.

### CK_RWCOHORT_RP_INSTANCE (Reader-Preference)

**Description:** Reader-preference reader-writer cohort lock.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| read_counter | unsigned int | 4 bytes | Active reader count |
| read_barrier | unsigned int | 4 bytes | Waiting readers barrier |
| wait_limit | unsigned int | 4 bytes | Iterations before raising barrier |

**Behavior:** Readers block new writers by raising read_barrier. Writers back off when barrier raised.

### CK_RWCOHORT_NEUTRAL_INSTANCE (Neutral)

**Description:** Neutral fairness reader-writer cohort lock.

**Fields:**

| Field | Type | Size | Description |
|-------|------|------|-------------|
| read_counter | unsigned int | 4 bytes | Active reader count |

**Behavior:** Readers acquire cohort lock to increment counter, then release. No preference mechanism.

## Algorithms

### CK_RWCOHORT_WP_PROTOTYPE (Writer-Preference)

**write_lock:**
1. Spin while write_barrier > 0
2. Acquire cohort lock
3. Spin while read_counter > 0

**write_unlock:**
1. Release cohort lock

**read_lock:**
1. Loop:
   a. Increment read_counter
   b. Fence
   c. IF cohort NOT locked: break
   d. Decrement read_counter
   e. Spin while cohort locked
   f. IF wait exceeded: increment write_barrier
2. IF barrier raised: decrement write_barrier
3. Fence

**read_unlock:**
1. Fence
2. Decrement read_counter

### CK_RWCOHORT_RP_PROTOTYPE (Reader-Preference)

**write_lock:**
1. Loop:
   a. Acquire cohort lock
   b. IF read_counter == 0: break
   c. Release cohort lock
   d. Spin while read_counter > 0
   e. IF wait exceeded: increment read_barrier
2. IF barrier raised: decrement read_barrier

**write_unlock:**
1. Release cohort lock

**read_lock:**
1. Spin while read_barrier > 0
2. Increment read_counter
3. Fence
4. Spin while cohort locked

**read_unlock:**
1. Fence
2. Decrement read_counter

### CK_RWCOHORT_NEUTRAL_PROTOTYPE (Neutral)

**write_lock:**
1. Acquire cohort lock
2. Spin while read_counter > 0

**write_unlock:**
1. Release cohort lock

**read_lock:**
1. Acquire cohort lock
2. Increment read_counter
3. Release cohort lock

**read_unlock:**
1. Fence
2. Decrement read_counter

## Concurrency

**Thread Safety:** Fully thread-safe.

**Progress Guarantee:**
- WP: Writers eventually acquire; readers may starve under write pressure
- RP: Readers eventually acquire; writers may starve under read pressure
- Neutral: Fair ordering, lower concurrency

**NUMA Optimization:**
- Inherits from underlying cohort lock
- Local threads pass lock preferentially

## Platform Considerations

- Macro-generated types for different cohort lock types
- Default wait_limit = 1000 iterations
- Barrier mechanism prevents indefinite starvation
- Requires underlying ck_cohort implementation

**Correctness Reference:** Calciu, I.; Dice, D.; Lev, Y.; Luchangco, V.; Marathe, V.; and Shavit, N. 2014. "NUMA-Aware Reader-Writer Locks"
