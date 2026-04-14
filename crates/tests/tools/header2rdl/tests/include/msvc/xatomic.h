// xatomic.h internal header (core)

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef _XATOMIC_H
#define _XATOMIC_H
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR

#include <type_traits>

#include _STL_INTRIN_HEADER

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

#define _CONCATX(x, y) x##y
#define _CONCAT(x, y)  _CONCATX(x, y)

// Interlocked intrinsic mapping for _nf/_acq/_rel
#if defined(_M_CEE_PURE) || (defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)) \
    || (defined(_M_X64) && !defined(_M_ARM64EC))
#define _INTRIN_RELAXED(x) x
#define _INTRIN_ACQUIRE(x) x
#define _INTRIN_RELEASE(x) x
#define _INTRIN_ACQ_REL(x) x
#ifdef _M_CEE_PURE
#define _YIELD_PROCESSOR()
#else // ^^^ defined(_M_CEE_PURE) / !defined(_M_CEE_PURE) vvv
#define _YIELD_PROCESSOR() _mm_pause()
#endif // ^^^ !defined(_M_CEE_PURE) ^^^

#elif defined(_M_ARM) || defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)
#define _INTRIN_RELAXED(x) _CONCAT(x, _nf)
#define _INTRIN_ACQUIRE(x) _CONCAT(x, _acq)
#define _INTRIN_RELEASE(x) _CONCAT(x, _rel)
// We don't have interlocked intrinsics for acquire-release ordering, even on
// ARM32/ARM64, so fall back to sequentially consistent.
#define _INTRIN_ACQ_REL(x) x
#define _YIELD_PROCESSOR() __yield()

#else // ^^^ ARM32/ARM64/ARM64EC/HYBRID_X86_ARM64 / unsupported hardware vvv
#error Unsupported hardware
#endif // hardware

#define _MT_INCR(x) _INTRIN_RELAXED(_InterlockedIncrement)(reinterpret_cast<volatile long*>(&x))
#define _MT_DECR(x) _INTRIN_ACQ_REL(_InterlockedDecrement)(reinterpret_cast<volatile long*>(&x))

// The following macros are SHARED with vcruntime and any updates should be mirrored.
// Also: if any macros are added they should be #undefed in vcruntime as well.
#define _Compiler_barrier() _STL_DISABLE_DEPRECATED_WARNING _ReadWriteBarrier() _STL_RESTORE_DEPRECATED_WARNING

#if defined(_M_ARM) || defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)
#define _Memory_barrier()             __dmb(0xB) // inner shared data memory barrier
#define _Compiler_or_memory_barrier() _Memory_barrier()
#if defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)
#define _Memory_load_acquire_barrier() __dmb(0x9) // inner shared data memory load barrier
#else // ^^^ ARM64/ARM64EC/HYBRID_X86_ARM64 / ARM32 vvv
#define _Memory_load_acquire_barrier() _Memory_barrier()
#endif // ^^^ ARM32 ^^^
#elif defined(_M_IX86) || defined(_M_X64)
// x86/x64 hardware only emits memory barriers inside _Interlocked intrinsics
#define _Compiler_or_memory_barrier() _Compiler_barrier()
#else // ^^^ x86/x64 / unsupported hardware vvv
#error Unsupported hardware
#endif // hardware

_STD_BEGIN

#if _HAS_CXX20
_EXPORT_STD enum class memory_order : int {
    relaxed,
    consume,
    acquire,
    release,
    acq_rel,
    seq_cst,

    // LWG-3268
    memory_order_relaxed = relaxed,
    memory_order_consume = consume,
    memory_order_acquire = acquire,
    memory_order_release = release,
    memory_order_acq_rel = acq_rel,
    memory_order_seq_cst = seq_cst
};
_EXPORT_STD inline constexpr memory_order memory_order_relaxed = memory_order::relaxed;
_EXPORT_STD inline constexpr memory_order memory_order_consume = memory_order::consume;
_EXPORT_STD inline constexpr memory_order memory_order_acquire = memory_order::acquire;
_EXPORT_STD inline constexpr memory_order memory_order_release = memory_order::release;
_EXPORT_STD inline constexpr memory_order memory_order_acq_rel = memory_order::acq_rel;
_EXPORT_STD inline constexpr memory_order memory_order_seq_cst = memory_order::seq_cst;
#else // ^^^ _HAS_CXX20 / !_HAS_CXX20 vvv
enum memory_order {
    memory_order_relaxed,
    memory_order_consume,
    memory_order_acquire,
    memory_order_release,
    memory_order_acq_rel,
    memory_order_seq_cst
};
#endif // ^^^ !_HAS_CXX20 ^^^

using _Atomic_counter_t = unsigned long;

template <class _Integral, class _Ty>
_NODISCARD volatile _Integral* _Atomic_address_as(_Ty& _Source) noexcept {
    // gets a pointer to the argument as an integral type (to pass to intrinsics)
    static_assert(is_integral_v<_Integral>, "Tried to reinterpret memory as non-integral");
    return &reinterpret_cast<volatile _Integral&>(_Source);
}

template <class _Integral, class _Ty>
_NODISCARD const volatile _Integral* _Atomic_address_as(const _Ty& _Source) noexcept {
    // gets a pointer to the argument as an integral type (to pass to intrinsics)
    static_assert(is_integral_v<_Integral>, "Tried to reinterpret memory as non-integral");
    return &reinterpret_cast<const volatile _Integral&>(_Source);
}

_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // _STL_COMPILER_PREPROCESSOR
#endif // _XATOMIC_H
