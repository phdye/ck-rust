# Module: ck_queue — Specification

## Operations

### CK_SLIST_INIT

**Signature:**
```
CK_SLIST_INIT(head) → void (macro)
```

**Preconditions:**
- head must point to valid CK_SLIST_HEAD structure [INFERRED]

**Postconditions:**
- head->cslh_first == NULL [SPECIFIED]
- Store fence executed [OBSERVED]

**Concurrency:**
- Thread Safety: Not safe to call concurrently with other operations [OBSERVED]
- Memory Ordering: Release semantics [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### CK_SLIST_EMPTY

**Signature:**
```
CK_SLIST_EMPTY(head) → bool (macro)
```

**Preconditions:**
- head must point to valid CK_SLIST_HEAD structure [INFERRED]

**Postconditions:**
- Returns true if list is empty, false otherwise [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe with concurrent readers and writers [SPECIFIED]
- Memory Ordering: Atomic load [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### CK_SLIST_FIRST

**Signature:**
```
CK_SLIST_FIRST(head) → Pointer to first element (macro)
```

**Preconditions:**
- head must point to valid CK_SLIST_HEAD structure [INFERRED]

**Postconditions:**
- Returns pointer to first element or NULL if empty [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe with concurrent modifications [SPECIFIED]
- Memory Ordering: Atomic load [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### CK_SLIST_NEXT

**Signature:**
```
CK_SLIST_NEXT(elm, field) → Pointer to next element (macro)
```

**Preconditions:**
- elm must point to valid list element [INFERRED]

**Postconditions:**
- Returns pointer to next element or NULL [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe with concurrent modifications [SPECIFIED]
- Memory Ordering: Atomic load [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### CK_SLIST_INSERT_HEAD

**Signature:**
```
CK_SLIST_INSERT_HEAD(head, elm, field) → void (macro)
```

**Preconditions:**
- head must point to valid CK_SLIST_HEAD [INFERRED]
- elm must point to valid, unlinked element [INFERRED]

**Postconditions:**
- elm is now first element in list [SPECIFIED]
- elm->next points to former first element [SPECIFIED]

**Concurrency:**
- Thread Safety: Requires external synchronization for writers [SPECIFIED]
- Memory Ordering: Release semantics (fence before store) [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### CK_SLIST_INSERT_AFTER

**Signature:**
```
CK_SLIST_INSERT_AFTER(a, b, field) → void (macro)
```

**Preconditions:**
- a must point to element in list [INFERRED]
- b must point to valid, unlinked element [INFERRED]

**Postconditions:**
- b inserted after a [SPECIFIED]

**Concurrency:**
- Thread Safety: Requires external synchronization for writers [SPECIFIED]
- Memory Ordering: Release semantics [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### CK_SLIST_REMOVE_HEAD

**Signature:**
```
CK_SLIST_REMOVE_HEAD(head, field) → void (macro)
```

**Preconditions:**
- List must not be empty [INFERRED]

**Postconditions:**
- Former first element is unlinked [SPECIFIED]
- head->first now points to second element (or NULL) [SPECIFIED]

**Concurrency:**
- Thread Safety: Requires external synchronization [SPECIFIED]
- Memory Ordering: Atomic store [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### CK_SLIST_REMOVE

**Signature:**
```
CK_SLIST_REMOVE(head, elm, type, field) → void (macro)
```

**Preconditions:**
- elm must be in the list [INFERRED]

**Postconditions:**
- elm is unlinked from list [SPECIFIED]

**Concurrency:**
- Thread Safety: Requires external synchronization [SPECIFIED]
- Progress Guarantee: O(n) traversal [OBSERVED]

---

### CK_STAILQ_INSERT_TAIL

**Signature:**
```
CK_STAILQ_INSERT_TAIL(head, elm, field) → void (macro)
```

**Preconditions:**
- head must point to valid CK_STAILQ_HEAD [INFERRED]
- elm must be unlinked [INFERRED]

**Postconditions:**
- elm is now last element [SPECIFIED]
- head->last updated [SPECIFIED]

**Concurrency:**
- Thread Safety: Requires external synchronization [SPECIFIED]
- Memory Ordering: Release semantics [OBSERVED]
- Progress Guarantee: wait-free [OBSERVED]

---

### CK_LIST_REMOVE

**Signature:**
```
CK_LIST_REMOVE(elm, field) → void (macro)
```

**Preconditions:**
- elm must be in a list [INFERRED]

**Postconditions:**
- elm is unlinked [SPECIFIED]
- Predecessor's next updated [SPECIFIED]
- Successor's prev updated (if exists) [SPECIFIED]

**Concurrency:**
- Thread Safety: Requires external synchronization [SPECIFIED]
- Progress Guarantee: O(1) - no traversal needed [SPECIFIED]

---

### CK_*_FOREACH

**Signature:**
```
CK_SLIST_FOREACH(var, head, field) (macro)
CK_STAILQ_FOREACH(var, head, field) (macro)
CK_LIST_FOREACH(var, head, field) (macro)
```

**Preconditions:**
- head must point to valid head structure [INFERRED]

**Postconditions:**
- Iterates through all elements [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe with concurrent modifications [SPECIFIED]
- Note: May observe elements added after iteration started [OBSERVED]
- Note: May miss elements removed after iteration started [OBSERVED]

---

### CK_*_FOREACH_SAFE

**Signature:**
```
CK_SLIST_FOREACH_SAFE(var, head, field, tvar) (macro)
CK_STAILQ_FOREACH_SAFE(var, head, field, tvar) (macro)
CK_LIST_FOREACH_SAFE(var, head, field, tvar) (macro)
```

**Preconditions:**
- head must point to valid head structure [INFERRED]
- tvar must be same type as var [INFERRED]

**Postconditions:**
- Iterates through elements, safe to remove current element [SPECIFIED]

**Concurrency:**
- Thread Safety: Safe if only current element is removed [SPECIFIED]
- Note: Not safe if other elements are concurrently removed [OBSERVED]

---

## Data Structure Invariants

### SLIST
- Single forward pointer per element [SPECIFIED]
- NULL-terminated [SPECIFIED]

### STAILQ
- Single forward pointer per element [SPECIFIED]
- Tail pointer enables O(1) append [SPECIFIED]
- Empty: last points to &first [SPECIFIED]

### LIST
- Doubly-linked (forward next, backward prev-of-next) [SPECIFIED]
- O(1) removal without traversal [SPECIFIED]

## Module-Level Invariants

- All read operations use atomic loads [OBSERVED]
- All write operations preceded by store fence [OBSERVED]
- Writers require external synchronization [SPECIFIED]

## Safety Properties

**Reader Safety:** Concurrent readers always see a consistent view of list structure (no torn pointers). [SPECIFIED]

**Traversal Safety:** FOREACH iteration will not crash even with concurrent modifications. [SPECIFIED]

## Liveness Properties

**Wait-Free Readers:** All read operations complete in bounded time. [SPECIFIED]

## Behavioral Ambiguities

### Concurrent modification during FOREACH

**Observed Behavior:** Iterator may or may not observe concurrent modifications

**Intent:** SPECIFIED - Iteration sees a snapshot of pointers at read time

**Recommendation:** For precise semantics, use external synchronization or accept relaxed guarantees.

## Discrepancies

No discrepancies detected between sources.
