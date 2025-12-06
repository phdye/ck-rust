# Module: ck_cc — Platform Specification

## Platform Variants

| Platform | Implementation | Notes |
|----------|----------------|-------|
| GCC/Clang | include/gcc/ck_cc.h | Uses __attribute__ and __builtin_* functions |
| Sun Studio | include/gcc/ck_cc.h (partial) | Limited attribute support, some builtins disabled |
| Generic | include/ck_cc.h | Portable C99 fallbacks for all operations |

## OS Dependencies

No OS-specific dependencies.

## Hardware Requirements

| Requirement | Mandatory | Notes |
|-------------|-----------|-------|
| None | — | Pure C99 implementations work on all platforms |

Optional hardware support for faster bit operations:
- x86/x86_64: BSF instruction (bit scan forward) via __builtin_ffs
- x86/x86_64: TZCNT instruction (trailing zero count) via __builtin_ctz
- x86/x86_64: POPCNT instruction via __builtin_popcount

## Architecture-Specific Behavior

### x86 / x86_64

Uses optimized inline assembly constraints:
- `CK_CC_IMM_U32 = "Z"` - 32-bit unsigned immediate
- `CK_CC_IMM_S32 = "e"` - 32-bit signed immediate

These enable better code generation for inline assembly in other modules.

### Other Architectures

Uses generic immediate constraint:
- `CK_CC_IMM_U32 = "i"` - generic immediate

## Memory Model Notes

| Architecture | Memory Model | Impact on This Module |
|--------------|--------------|----------------------|
| All | N/A | This module has no memory ordering concerns - all operations are pure functions |

## Compiler Feature Detection

The following feature flags indicate implementation:

| Flag | Meaning |
|------|---------|
| CK_F_CC_FFS | ck_cc_ffs is defined |
| CK_F_CC_FFSL | ck_cc_ffsl is defined |
| CK_F_CC_FFSLL | ck_cc_ffsll is defined |
| CK_F_CC_CTZ | ck_cc_ctz is defined |
| CK_F_CC_POPCOUNT | ck_cc_popcount is defined |

## Configuration Options

| Option | Purpose |
|--------|---------|
| CK_MD_CC_BUILTIN_DISABLE | Force portable implementations instead of compiler builtins |
| __OPTIMIZE__ | GCC: enables inlining when optimization is on |
| __freestanding__ | GCC: poisons malloc/free to prevent accidental use |

## Compiler-Specific Attributes Summary

| Macro | GCC/Clang | Sun Studio | Generic |
|-------|-----------|------------|---------|
| CK_CC_UNUSED | `__attribute__((unused))` | (empty) | (empty) |
| CK_CC_USED | `__attribute__((used))` | (empty) | (empty) |
| CK_CC_INLINE | `inline __attribute__((unused))` | `inline` | `inline` |
| CK_CC_FORCE_INLINE | `inline __attribute__((always_inline))` | `inline` | `inline` |
| CK_CC_RESTRICT | `__restrict__` | `__restrict__` | (empty) |
| CK_CC_PACKED | `__attribute__((packed))` | (empty) | (empty) |
| CK_CC_ALIGN(B) | `__attribute__((aligned(B)))` | (empty) | (empty) |
| CK_CC_CACHELINE | `__attribute__((aligned(CK_MD_CACHELINE)))` | (empty) | (empty) |
| CK_CC_LIKELY(x) | `__builtin_expect(!!(x), 1)` | `x` | `x` |
| CK_CC_UNLIKELY(x) | `__builtin_expect(!!(x), 0)` | `x` | `x` |
| CK_CC_ALIASED | `__attribute__((__may_alias__))` | (empty) | (empty) |
| CK_CC_TYPEOF(X, D) | `__typeof__(X)` | `D` | `D` |
| CK_CC_WEAKREF | `__attribute__((weakref))` | (empty) | (empty) |
