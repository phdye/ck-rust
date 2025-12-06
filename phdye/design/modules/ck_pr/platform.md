# Module: ck_pr — Platform Specification

## Platform Variants

| Platform | Implementation | Memory Model | Notes |
|----------|----------------|--------------|-------|
| x86_64 | include/gcc/x86_64/ck_pr.h | TSO | Most fences are no-ops |
| x86 (32-bit) | include/gcc/x86/ck_pr.h | TSO | Limited 64-bit atomic support |
| AArch64 | include/gcc/aarch64/ck_pr.h | RMO | Full fence support, LSE extensions optional |
| ARM (32-bit) | include/gcc/arm/ck_pr.h | RMO | DMB barriers |
| PowerPC64 | include/gcc/ppc64/ck_pr.h | RMO | lwsync/sync barriers |
| PowerPC | include/gcc/ppc/ck_pr.h | RMO | Limited 64-bit support |
| SPARC v9 | include/gcc/sparcv9/ck_pr.h | PSO | membar instructions |
| s390x | include/gcc/s390x/ck_pr.h | TSO | Similar to x86 |
| RISC-V 64 | include/gcc/riscv64/ck_pr.h | RMO | fence instructions |
| Generic | include/gcc/ck_pr.h | Configurable | Uses GCC builtins |

## OS Dependencies

No OS-specific dependencies.

## Hardware Requirements

| Requirement | Mandatory | Notes |
|-------------|-----------|-------|
| Compare-and-swap | Yes | CMPXCHG (x86), LDREX/STREX (ARM), etc. |
| Atomic load/store | Yes | Natural alignment required |
| Memory barriers | Yes | Platform-specific instructions |

## Architecture-Specific Behavior

### x86 / x86_64

**Memory Model:** TSO (Total Store Order)
- Loads are not reordered with other loads
- Stores are not reordered with other stores
- Stores are not reordered with older loads
- Loads MAY be reordered with older stores to different locations

**Fence Implementation:**
- Most fences (load, store, acquire, release) are no-ops
- store_load fence requires MFENCE instruction
- memory fence uses MFENCE

**Atomic Instructions:**
- LOCK CMPXCHG for CAS
- LOCK XADD for fetch-and-add
- LOCK XCHG for fetch-and-store
- LOCK ADD/SUB/AND/OR/XOR for binary operations

**Special Features:**
- 64-bit CAS available (CMPXCHG8B on 32-bit, native on 64-bit)
- 128-bit CAS available (CMPXCHG16B on 64-bit, requires alignment)
- PAUSE instruction for spin loops
- RTM (Restricted Transactional Memory) on some CPUs

### AArch64

**Memory Model:** RMO (Relaxed Memory Order)
- All reorderings possible without explicit barriers

**Fence Implementation:**
- DMB (Data Memory Barrier) for all fences
- DMB LD for load fence
- DMB ST for store fence
- DMB ISH for full fence

**Atomic Instructions:**
- LDXR/STXR (Load-Exclusive/Store-Exclusive) for CAS
- LDADD (atomic add) with LSE extension
- LDCLR, LDSET, LDEOR for bitwise operations with LSE

**Variants:**
- ck_pr_llsc.h: Load-linked/Store-conditional implementation
- ck_pr_lse.h: Large System Extensions (faster atomics)

### ARM (32-bit)

**Memory Model:** RMO

**Fence Implementation:**
- DMB for data barriers
- DSB for full synchronization

**Atomic Instructions:**
- LDREX/STREX for load-exclusive/store-exclusive
- No native 64-bit atomics on most 32-bit ARM

### PowerPC / PowerPC64

**Memory Model:** RMO (very relaxed)

**Fence Implementation:**
- lwsync (lightweight sync) for most barriers
- sync for full barrier
- isync for instruction synchronization

**Atomic Instructions:**
- lwarx/stwcx (load-word-and-reserve/store-conditional)
- ldarx/stdcx for 64-bit (PPC64 only)

### SPARC v9

**Memory Model:** PSO (Partial Store Order)
- Stores may be reordered with other stores
- Loads are ordered

**Fence Implementation:**
- membar instructions with various options

### RISC-V 64

**Memory Model:** RMO (RVWMO - RISC-V Weak Memory Ordering)

**Fence Implementation:**
- fence instruction with various orderings
- fence.i for instruction synchronization

**Atomic Instructions:**
- LR/SC (Load-Reserved/Store-Conditional)
- AMO instructions (AMOADD, AMOSWAP, etc.)

## Memory Model Notes

| Architecture | Memory Model | Impact on Fences |
|--------------|--------------|------------------|
| x86/x86_64 | TSO | Most fences compile to no-ops; only store_load needs mfence |
| ARM/AArch64 | RMO | All fences emit DMB or equivalent |
| PowerPC | RMO | All fences emit lwsync or sync |
| SPARC v9 | PSO | Store fences needed, load fences often no-ops |
| s390x | TSO | Similar to x86 |

## Alignment Requirements

| Type | Required Alignment | Notes |
|------|-------------------|-------|
| 8-bit | 1 byte | Always atomic |
| 16-bit | 2 bytes | Natural alignment |
| 32-bit | 4 bytes | Natural alignment |
| 64-bit | 8 bytes | Natural alignment; may be emulated on 32-bit |
| 128-bit (DCAS) | 16 bytes | Only on x86_64 with CMPXCHG16B |
| Pointer | sizeof(void*) | Platform pointer size |

## Feature Detection Flags

| Flag | Meaning | Detection |
|------|---------|-----------|
| CK_F_PR_CAS_PTR_2 | 128-bit (double-width) CAS available | Platform detection |
| CK_F_PR_LOAD_64 | 64-bit atomic load available | Platform detection |
| CK_F_PR_STORE_64 | 64-bit atomic store available | Platform detection |
| CK_F_PR_FAA_* | Hardware fetch-and-add for type | Platform detection |
| CK_F_PR_FAS_* | Hardware fetch-and-store for type | Platform detection |
| CK_MD_TSO | Platform has TSO memory model | Configure detection |
| CK_MD_PSO | Platform has PSO memory model | Configure detection |
| CK_MD_RMO | Platform has RMO memory model | Configure detection |

## Configuration Options

| Option | Purpose |
|--------|---------|
| CK_USE_CC_BUILTINS | Force use of GCC builtins instead of inline asm |
| CK_PR_DISABLE_DOUBLE | Disable double-precision floating point atomics |
| CK_MD_CC_BUILTIN_DISABLE | Disable optimized compiler builtins |
