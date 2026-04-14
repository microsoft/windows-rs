// Copyright (c) Microsoft Corporation. All rights reserved.
//
// C11 atomic support routines
#pragma once

#ifdef __cplusplus
// this header should never be included in c++ mode, but if it is
// we need to catch it because the content of this header is provided by
// the STL's <atomic> header in C++
#error "vcruntime_c11_atomic_support.h is a C-only header"
#endif // __cplusplus

#include <crtdbg.h>
#include <intrin0.h>
#include <stdint.h>
#include <vcruntime_string.h>

// code from xatomic.h
#define _CONCATX(x, y) x##y
#define _CONCAT(x, y)  _CONCATX(x, y)

// Interlocked intrinsic mapping for _nf/_acq/_rel
#if defined(_M_CEE_PURE) || (defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)) || (defined(_M_X64) && !defined(_M_ARM64EC))
#define _INTRIN_RELAXED(x) x
#define _INTRIN_ACQUIRE(x) x
#define _INTRIN_RELEASE(x) x
#define _INTRIN_ACQ_REL(x) x
#ifdef _M_CEE_PURE
#define _YIELD_PROCESSOR()
#else // ^^^ _M_CEE_PURE / !_M_CEE_PURE vvv
#define _YIELD_PROCESSOR() _mm_pause()
#endif // ^^^ !_M_CEE_PURE ^^^

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
// end code from xatomic.h


// The following is modified from the _CRT_SECURE_INVALID_PARAMETER macro in
// corecrt.h. We need to do this because this header must be C, not C++, but we
// still want to report invalid parameters in the same way as C++ does. The
// macro in the CRT expands to C++ code because it contains global namespace
// qualification. This can be fixed in the ucrt by using a mechanism that
// defines something like _GLOBAL_NAMESPACE to :: in c++ mode and nothing in C
// mode.
#ifndef _ATOMIC_INVALID_PARAMETER
#ifdef _DEBUG
#define _ATOMIC_INVALID_PARAMETER(expr) _invalid_parameter(_CRT_WIDE(#expr), L"", __FILEW__, __LINE__, 0)
#else
// By default, _ATOMIC_INVALID_PARAMETER in retail invokes
// _invalid_parameter_noinfo_noreturn(), which is marked
// __declspec(noreturn) and does not return control to the application.
// Even if _set_invalid_parameter_handler() is used to set a new invalid
// parameter handler which does return control to the application,
// _invalid_parameter_noinfo_noreturn() will terminate the application
// and invoke Watson. You can overwrite the definition of
// _ATOMIC_INVALID_PARAMETER if you need.
#define _ATOMIC_INVALID_PARAMETER(expr) _invalid_parameter_noinfo_noreturn()
#endif
#endif

// The following code is SHARED between the STL's <atomic> header and vcruntime's
// vcruntime_c11_atomic_support.h header. Any updates should be mirrored.
// Also: if any macros are added they should be #undefed in both headers

#if defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)
#define _STD_ATOMIC_USE_ARM64_LDAR_STLR 1
#ifdef __clang__
#define __LOAD_ACQUIRE_ARM64(_Width, _Ptr) \
    (__int##_Width)(__atomic_load_n((const volatile unsigned __int##_Width*)(_Ptr), 2))
#define __STORE_RELEASE(_Width, _Ptr, _Desired) \
    _Compiler_barrier();                        \
    __atomic_store_n((volatile unsigned __int##_Width*)(_Ptr), (unsigned __int##_Width)(_Desired), 3)
#else // ^^^ Clang / MSVC vvv
#define __LOAD_ACQUIRE_ARM64(_Width, _Ptr) \
    (__int##_Width)(__load_acquire##_Width((const volatile unsigned __int##_Width*)(_Ptr)))
#define __STORE_RELEASE(_Width, _Ptr, _Desired) \
    _Compiler_barrier();                        \
    __stlr##_Width(                             \
        (volatile unsigned __int##_Width*)(_Ptr), (unsigned __int##_Width)(_Desired))
#endif // ^^^ MSVC ^^^
#else // ^^^ ARM64/ARM64EC/HYBRID_X86_ARM64 / Other architectures vvv
#define _STD_ATOMIC_USE_ARM64_LDAR_STLR 0
#define __STORE_RELEASE(_Width, _Ptr, _Desired) \
    _Compiler_or_memory_barrier();              \
    __iso_volatile_store##_Width((_Ptr), (_Desired))
#endif // ^^^ Other architectures ^^^

enum {
    _Atomic_memory_order_relaxed,
    _Atomic_memory_order_consume,
    _Atomic_memory_order_acquire,
    _Atomic_memory_order_release,
    _Atomic_memory_order_acq_rel,
    _Atomic_memory_order_seq_cst,
};

#ifndef _INVALID_MEMORY_ORDER
#ifdef _DEBUG
#define _INVALID_MEMORY_ORDER                              \
    do {                                                   \
        _RPTF0(_CRT_ASSERT, "Invalid memory order");       \
        _ATOMIC_INVALID_PARAMETER("Invalid memory order"); \
    } while (0)
#else // ^^^ _DEBUG / !_DEBUG vvv
#define _INVALID_MEMORY_ORDER
#endif // _DEBUG
#endif // _INVALID_MEMORY_ORDER

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

inline void _Check_memory_order(const unsigned int _Order) {
    if (_Order > _Atomic_memory_order_seq_cst) {
        _INVALID_MEMORY_ORDER;
    }
}

// this is different from the STL
// we are the MSVC runtime so we need not support clang here
#define _Compiler_barrier()                                                                   \
    _Pragma("warning(push)") _Pragma("warning(disable : 4996)") /* was declared deprecated */ \
        _ReadWriteBarrier() _Pragma("warning(pop)")

// note: these macros are _not_ always safe to use with a trailing semicolon,
// we avoid wrapping them in do {} while (0) because MSVC generates code for such loops
// in debug mode.

#if (defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)) || (defined(_M_X64) && !defined(_M_ARM64EC))
#define _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _Intrinsic, ...) \
    _Check_memory_order(_Order);                                   \
    _Result = _Intrinsic(__VA_ARGS__)
#elif defined(_M_ARM) || defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)
#define _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _Intrinsic, ...) \
    switch (_Order) {                                              \
    case _Atomic_memory_order_relaxed:                             \
        _Result = _INTRIN_RELAXED(_Intrinsic)(__VA_ARGS__);        \
        break;                                                     \
    case _Atomic_memory_order_consume:                             \
    case _Atomic_memory_order_acquire:                             \
        _Result = _INTRIN_ACQUIRE(_Intrinsic)(__VA_ARGS__);        \
        break;                                                     \
    case _Atomic_memory_order_release:                             \
        _Result = _INTRIN_RELEASE(_Intrinsic)(__VA_ARGS__);        \
        break;                                                     \
    default:                                                       \
        _INVALID_MEMORY_ORDER;                                     \
        /* [[fallthrough]]; */                                     \
    case _Atomic_memory_order_acq_rel:                             \
    case _Atomic_memory_order_seq_cst:                             \
        _Result = _Intrinsic(__VA_ARGS__);                         \
        break;                                                     \
    }
#endif // hardware

#if _STD_ATOMIC_USE_ARM64_LDAR_STLR == 1

#define _ATOMIC_LOAD_ARM64(_Result, _Width, _Ptr, _Order_var) \
    switch (_Order_var) {                                     \
    case _Atomic_memory_order_relaxed:                        \
        _Result = __iso_volatile_load##_Width(_Ptr);          \
        break;                                                \
    case _Atomic_memory_order_consume:                        \
    case _Atomic_memory_order_acquire:                        \
    case _Atomic_memory_order_seq_cst:                        \
        _Result = __LOAD_ACQUIRE_ARM64(_Width, _Ptr);         \
        _Compiler_barrier();                                  \
        break;                                                \
    case _Atomic_memory_order_release:                        \
    case _Atomic_memory_order_acq_rel:                        \
    default:                                                  \
        _Result = __iso_volatile_load##_Width(_Ptr);          \
        _INVALID_MEMORY_ORDER;                                \
        break;                                                \
    }

#endif // _STD_ATOMIC_USE_ARM64_LDAR_STLR == 1

#define _ATOMIC_POST_LOAD_BARRIER_AS_NEEDED(_Order_var) \
    switch (_Order_var) {                               \
    case _Atomic_memory_order_relaxed:                  \
        break;                                          \
    case _Atomic_memory_order_consume:                  \
    case _Atomic_memory_order_acquire:                  \
    case _Atomic_memory_order_seq_cst:                  \
        _Compiler_or_memory_barrier();                  \
        break;                                          \
    case _Atomic_memory_order_release:                  \
    case _Atomic_memory_order_acq_rel:                  \
    default:                                            \
        _INVALID_MEMORY_ORDER;                          \
        break;                                          \
    }

#define _ATOMIC_STORE_PREFIX(_Width, _Ptr, _Desired)      \
    case _Atomic_memory_order_relaxed:                    \
        __iso_volatile_store##_Width((_Ptr), (_Desired)); \
        return;                                           \
    case _Atomic_memory_order_release:                    \
        __STORE_RELEASE(_Width, _Ptr, _Desired);          \
        return;                                           \
    default:                                              \
    case _Atomic_memory_order_consume:                    \
    case _Atomic_memory_order_acquire:                    \
    case _Atomic_memory_order_acq_rel:                    \
        _INVALID_MEMORY_ORDER;                            \
        /* [[fallthrough]]; */

#define _ATOMIC_STORE_SEQ_CST_ARM(_Width, _Ptr, _Desired) \
    _Memory_barrier();                                    \
    __iso_volatile_store##_Width((_Ptr), (_Desired));     \
    _Memory_barrier();

#define _ATOMIC_STORE_SEQ_CST_ARM64(_Width, _Ptr, _Desired) \
    __STORE_RELEASE(_Width, _Ptr, _Desired);                \
    _Memory_barrier();

#define _ATOMIC_STORE_SEQ_CST_X86_X64(_Width, _Ptr, _Desired) (void) _InterlockedExchange##_Width((_Ptr), (_Desired));
#define _ATOMIC_STORE_32_SEQ_CST_X86_X64(_Ptr, _Desired) \
    (void) _InterlockedExchange((volatile long*)(_Ptr), (long)(_Desired));

#define _ATOMIC_STORE_64_SEQ_CST_IX86(_Ptr, _Desired) \
    _Compiler_barrier();                              \
    __iso_volatile_store64((_Ptr), (_Desired));       \
    _Atomic_thread_fence(_Atomic_memory_order_seq_cst);

#if defined(_M_ARM)
#define _ATOMIC_STORE_SEQ_CST(_Width, _Ptr, _Desired) _ATOMIC_STORE_SEQ_CST_ARM(_Width, (_Ptr), (_Desired))
#define _ATOMIC_STORE_32_SEQ_CST(_Ptr, _Desired)      _ATOMIC_STORE_SEQ_CST_ARM(32, (_Ptr), (_Desired))
#define _ATOMIC_STORE_64_SEQ_CST(_Ptr, _Desired)      _ATOMIC_STORE_SEQ_CST_ARM(64, (_Ptr), (_Desired))
#elif defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64) // ^^^ ARM32 / ARM64/ARM64EC/HYBRID_X86_ARM64 vvv
#define _ATOMIC_STORE_SEQ_CST(_Width, _Ptr, _Desired) _ATOMIC_STORE_SEQ_CST_ARM64(_Width, (_Ptr), (_Desired))
#define _ATOMIC_STORE_32_SEQ_CST(_Ptr, _Desired)      _ATOMIC_STORE_SEQ_CST_ARM64(32, (_Ptr), (_Desired))
#define _ATOMIC_STORE_64_SEQ_CST(_Ptr, _Desired)      _ATOMIC_STORE_SEQ_CST_ARM64(64, (_Ptr), (_Desired))
#elif defined(_M_IX86) || defined(_M_X64) // ^^^ ARM64/ARM64EC/HYBRID_X86_ARM64 / x86/x64 vvv
#define _ATOMIC_STORE_SEQ_CST(_Width, _Ptr, _Desired) _ATOMIC_STORE_SEQ_CST_X86_X64(_Width, (_Ptr), (_Desired))
#define _ATOMIC_STORE_32_SEQ_CST(_Ptr, _Desired)      _ATOMIC_STORE_32_SEQ_CST_X86_X64((_Ptr), (_Desired))
#ifdef _M_IX86
#define _ATOMIC_STORE_64_SEQ_CST(_Ptr, _Desired) _ATOMIC_STORE_64_SEQ_CST_IX86((_Ptr), (_Desired))
#else // ^^^ x86 / x64 vvv
#define _ATOMIC_STORE_64_SEQ_CST(_Ptr, _Desired) _ATOMIC_STORE_SEQ_CST_X86_X64(64, (_Ptr), (_Desired))
#endif // ^^^ x64 ^^^
#else // ^^^ x86/x64 / Unsupported hardware vvv
#error "Unsupported hardware"
#endif

#pragma warning(push)
#pragma warning(disable : 6001) // "Using uninitialized memory '_Guard'"
#pragma warning(disable : 28113) // "Accessing a local variable _Guard via an Interlocked function: This is an unusual
                                 // usage which could be reconsidered."
inline void _Atomic_thread_fence(const unsigned int _Order) {
    if (_Order == _Atomic_memory_order_relaxed) {
        return;
    }

#if (defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)) || (defined(_M_X64) && !defined(_M_ARM64EC))
    _Compiler_barrier();
    if (_Order == _Atomic_memory_order_seq_cst) {
        volatile long _Guard; // Not initialized to avoid an unnecessary operation; the value does not matter

        // _mm_mfence could have been used, but it is not supported on older x86 CPUs and is slower on some recent CPUs.
        // The memory fence provided by interlocked operations has some exceptions, but this is fine:
        // std::atomic_thread_fence works with respect to other atomics only; it may not be a full fence for all ops.
        (void) _InterlockedIncrement(&_Guard);
        _Compiler_barrier();
    }
#elif defined(_M_ARM) || defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)
    if (_Order == _Atomic_memory_order_acquire || _Order == _Atomic_memory_order_consume) {
        _Memory_load_acquire_barrier();
    } else {
        _Memory_barrier();
    }
#else // ^^^ ARM32/ARM64/ARM64EC/HYBRID_X86_ARM64 / unsupported hardware vvv
#error Unsupported hardware
#endif // unsupported hardware
}
#pragma warning(pop)

// End of code shared with STL <atomic>

inline void _Atomic_lock_acquire(volatile long* _Spinlock) {
#if (defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)) || (defined(_M_X64) && !defined(_M_ARM64EC))
    // Algorithm from Intel(R) 64 and IA-32 Architectures Optimization Reference Manual, May 2020
    // Example 2-4. Contended Locks with Increasing Back-off Example - Improved Version, page 2-22
    // The code in mentioned manual is covered by the 0BSD license.
    int _Current_backoff   = 1;
    const int _Max_backoff = 64;
    while (_InterlockedExchange(_Spinlock, 1) != 0) {
        while (__iso_volatile_load32((int*) _Spinlock) != 0) {
            for (int _Count_down = _Current_backoff; _Count_down != 0; --_Count_down) {
                _mm_pause();
            }
            _Current_backoff = _Current_backoff < _Max_backoff ? _Current_backoff << 1 : _Max_backoff;
        }
    }
#elif defined(_M_ARM) || defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)
    while (_InterlockedExchange_acq(_Spinlock, 1) != 0) {
        while (__iso_volatile_load32((int*) _Spinlock) != 0) {
            __yield();
        }
    }
#else // ^^^ defined(_M_ARM) || defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64) ^^^
#error Unsupported hardware
#endif
}

inline void _Atomic_lock_release(volatile long* _Spinlock) {
    __STORE_RELEASE(32, (int*) _Spinlock, 0);
}

// End of code shared with vcruntime

inline void _Atomic_signal_fence(int _Order) {
    if (_Order != _Atomic_memory_order_relaxed) {
        _Compiler_barrier();
    }
}

inline _Bool _Atomic_is_lock_free(size_t _Sz) {
    return _Sz <= 8 && (_Sz & _Sz - 1) == 0;
}

inline void _Atomic_store8(volatile char* _Ptr, char _Desired, int _Order) {
    switch (_Order) {
        _ATOMIC_STORE_PREFIX(8, _Ptr, _Desired)
    case _Atomic_memory_order_seq_cst:
        _ATOMIC_STORE_SEQ_CST(8, _Ptr, _Desired)
        return;
    }
}

inline void _Atomic_store16(volatile short* _Ptr, short _Desired, int _Order) {
    switch (_Order) {
        _ATOMIC_STORE_PREFIX(16, _Ptr, _Desired)
    case _Atomic_memory_order_seq_cst:
        _ATOMIC_STORE_SEQ_CST(16, _Ptr, _Desired)
        return;
    }
}

inline void _Atomic_store32(volatile int* _Ptr, int _Desired, int _Order) {
    switch (_Order) {
        _ATOMIC_STORE_PREFIX(32, _Ptr, _Desired)
    case _Atomic_memory_order_seq_cst:
        _ATOMIC_STORE_32_SEQ_CST(_Ptr, _Desired)
        return;
    }
}

inline void _Atomic_store64(volatile long long* _Ptr, long long _Desired, int _Order) {
    switch (_Order) {
        _ATOMIC_STORE_PREFIX(64, _Ptr, _Desired)
    case _Atomic_memory_order_seq_cst:
        _ATOMIC_STORE_64_SEQ_CST(_Ptr, _Desired)
        return;
    }
}

inline void _Atomic_storef(volatile float* _Ptr, float _Desired, int _Order) {
    _Atomic_store32((volatile int*)_Ptr, *(int*)&_Desired, _Order);
}

inline void _Atomic_stored(volatile double* _Ptr, double _Desired, int _Order) {
    _Atomic_store64((volatile long long*)_Ptr, *(long long*)&_Desired, _Order);
}

inline char _Atomic_load8(const volatile char* _Ptr, int _Order) {
    char _As_bytes;
#if _STD_ATOMIC_USE_ARM64_LDAR_STLR == 1
    _ATOMIC_LOAD_ARM64(_As_bytes, 8, _Ptr, _Order)
#else
    _As_bytes = __iso_volatile_load8(_Ptr);
    _ATOMIC_POST_LOAD_BARRIER_AS_NEEDED(_Order)
#endif
    return _As_bytes;
}
inline short _Atomic_load16(const volatile short* _Ptr, int _Order) {
    short _As_bytes;
#if _STD_ATOMIC_USE_ARM64_LDAR_STLR == 1
    _ATOMIC_LOAD_ARM64(_As_bytes, 16, _Ptr, _Order)
#else
    _As_bytes = __iso_volatile_load16(_Ptr);
    _ATOMIC_POST_LOAD_BARRIER_AS_NEEDED(_Order)
#endif
    return _As_bytes;
}
inline int _Atomic_load32(const volatile int* _Ptr, int _Order) {
    int _As_bytes;
#if _STD_ATOMIC_USE_ARM64_LDAR_STLR == 1
    _ATOMIC_LOAD_ARM64(_As_bytes, 32, _Ptr, _Order)
#else
    _As_bytes = __iso_volatile_load32(_Ptr);
    _ATOMIC_POST_LOAD_BARRIER_AS_NEEDED(_Order)
#endif
    return _As_bytes;
}
inline long long _Atomic_load64(const volatile long long* _Ptr, int _Order) {
    long long _As_bytes;
#if _STD_ATOMIC_USE_ARM64_LDAR_STLR == 1
    _ATOMIC_LOAD_ARM64(_As_bytes, 64, _Ptr, _Order)
#else // ^^^ _STD_ATOMIC_USE_ARM64_LDAR_STLR == 1 / _STD_ATOMIC_USE_ARM64_LDAR_STLR != 1 vvv

#ifdef _M_ARM
    _As_bytes = __ldrexd(_Ptr);
#else
    _As_bytes = __iso_volatile_load64(_Ptr);
#endif
    _ATOMIC_POST_LOAD_BARRIER_AS_NEEDED(_Order);
#endif // _STD_ATOMIC_USE_ARM64_LDAR_STLR == 1
    return _As_bytes;
}
inline float _Atomic_loadf(const volatile float* _Ptr, int _Order) {
    int _As_bytes = _Atomic_load32((const volatile int*)_Ptr, _Order);
    return *(float*)&_As_bytes;
}
inline double _Atomic_loadd(const volatile double* _Ptr, int _Order) {
    long long _As_bytes = _Atomic_load64((const volatile long long*)_Ptr, _Order);
    return *(double*)&_As_bytes;
}

inline _Bool _Atomic_compare_exchange_strong8(volatile char* _Ptr, char* _Expected, char _Desired, int _Order) {
    char _Prev_bytes;
    char _Expected_bytes = *_Expected;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Prev_bytes, _InterlockedCompareExchange8, _Ptr, _Desired, _Expected_bytes);
    if (_Prev_bytes == _Expected_bytes) {
        return 1;
    }
    *_Expected = _Prev_bytes;
    return 0;
}
inline _Bool _Atomic_compare_exchange_strong16(volatile short* _Ptr, short* _Expected, short _Desired, int _Order) {
    short _Prev_bytes;
    short _Expected_bytes = *_Expected;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Prev_bytes, _InterlockedCompareExchange16, _Ptr, _Desired, _Expected_bytes);
    if (_Prev_bytes == _Expected_bytes) {
        return 1;
    }
    *_Expected = _Prev_bytes;
    return 0;
}
inline _Bool _Atomic_compare_exchange_strong32(volatile int* _Ptr, int* _Expected, int _Desired, int _Order) {
    int _Prev_bytes;
    int _Expected_bytes = *_Expected;
    _ATOMIC_CHOOSE_INTRINSIC(
        _Order, _Prev_bytes, _InterlockedCompareExchange, (volatile long*) _Ptr, _Desired, _Expected_bytes);
    if (_Prev_bytes == _Expected_bytes) {
        return 1;
    }
    *_Expected = _Prev_bytes;
    return 0;
}
inline _Bool _Atomic_compare_exchange_strong64(
    volatile long long* _Ptr, long long* _Expected, long long _Desired, int _Order) {
    long long _Prev_bytes;
    long long _Expected_bytes = *_Expected;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Prev_bytes, _InterlockedCompareExchange64, _Ptr, _Desired, _Expected_bytes);
    if (_Prev_bytes == _Expected_bytes) {
        return 1;
    }
    *_Expected = _Prev_bytes;
    return 0;
}
inline _Bool _Atomic_compare_exchange_strongf(volatile float* _Ptr, float* _Expected, float _Desired, int _Order) {
    return _Atomic_compare_exchange_strong32((volatile int*)_Ptr, (int*)_Expected, *(int*)&_Desired, _Order);
}
inline _Bool _Atomic_compare_exchange_strongd(volatile double* _Ptr, double* _Expected, double _Desired, int _Order) {
    return _Atomic_compare_exchange_strong64((volatile long long*)_Ptr, (long long*)_Expected, *(long long*)&_Desired, _Order);
}

inline char _Atomic_exchange8(volatile char* _Ptr, int _Desired, int _Order) {
    char _As_bytes;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _As_bytes, _InterlockedExchange8, _Ptr, (char) _Desired);
    return _As_bytes;
}
inline short _Atomic_exchange16(volatile short* _Ptr, int _Desired, int _Order) {
    short _As_bytes;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _As_bytes, _InterlockedExchange16, _Ptr, (short) _Desired);
    return _As_bytes;
}
inline int _Atomic_exchange32(volatile int* _Ptr, int _Desired, int _Order) {
    long _As_bytes;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _As_bytes, _InterlockedExchange, (volatile long*) _Ptr, (long) _Desired);
    return (int) _As_bytes;
}
inline long long _Atomic_exchange64(volatile long long* _Ptr, long long _Desired, int _Order) {
#if defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
    long long _As_bytes = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_As_bytes, _Desired, _Order)) {
    }
    return _As_bytes;
#else // ^^^ defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64) / !defined(_M_IX86) || defined(_M_HYBRID_X86_ARM64) vvv
    long long _As_bytes;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _As_bytes, _InterlockedExchange64, _Ptr, _Desired);
    return _As_bytes;
#endif // defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
}
inline float _Atomic_exchangef(volatile float* _Ptr, float _Desired, int _Order) {
    long _As_bytes = _Atomic_exchange32((volatile int*)_Ptr, *(int*)&_Desired, _Order);
    return *(float*) &_As_bytes;
}
inline double _Atomic_exchanged(volatile double* _Ptr, double _Desired, int _Order) {
    long long _As_bytes = _Atomic_exchange64((volatile long long*)_Ptr, *(long long*)&_Desired, _Order);
    return *(double*)&_As_bytes;
}

inline char _Atomic_fetch_add8(volatile char* _Ptr, int _Val, int _Order) {
    char _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedExchangeAdd8, _Ptr, (char) _Val);
    return _Result;
}
inline short _Atomic_fetch_add16(volatile short* _Ptr, int _Val, int _Order) {
    short _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedExchangeAdd16, _Ptr, (short) _Val);
    return _Result;
}
inline int _Atomic_fetch_add32(volatile int* _Ptr, int _Val, int _Order) {
    int _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedExchangeAdd, (volatile long*) _Ptr, _Val);
    return _Result;
}
inline long long _Atomic_fetch_add64(volatile long long* _Ptr, long long _Val, int _Order) {
#if defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, _Result + _Val, _Order)) {
    }
    return _Result;
#else // ^^^ defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64) / !defined(_M_IX86) || defined(_M_HYBRID_X86_ARM64) vvvv
    long long _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedExchangeAdd64, _Ptr, _Val);
    return _Result;
#endif // defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
}
inline float _Atomic_fetch_addf(volatile float* _Ptr, float _Val, int _Order) {
    float _Result = _Atomic_loadf(_Ptr, _Atomic_memory_order_relaxed);
    while (!_Atomic_compare_exchange_strongf(_Ptr, &_Result, _Result + _Val, _Order)) {
    }
    return _Result;
}
inline double _Atomic_fetch_addd(volatile double* _Ptr, double _Val, int _Order) {
    double _Result = _Atomic_loadd(_Ptr, _Atomic_memory_order_relaxed);
    while (!_Atomic_compare_exchange_strongd(_Ptr, &_Result, _Result + _Val, _Order)) {
    }
    return _Result;
}

inline char _Atomic_add_fetch8(volatile char* _Ptr, int _Val, int _Order) {
    char _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedExchangeAdd8, _Ptr, (char) _Val);
    return (char) (_Result + (char) _Val);
}
inline short _Atomic_add_fetch16(volatile short* _Ptr, int _Val, int _Order) {
    short _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedExchangeAdd16, _Ptr, (short) _Val);
    return (short) (_Result + (short) _Val);
}
inline int _Atomic_add_fetch32(volatile int* _Ptr, int _Val, int _Order) {
    int _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedExchangeAdd, (volatile long*) _Ptr, _Val);
    return _Result + _Val;
}
inline long long _Atomic_add_fetch64(volatile long long* _Ptr, long long _Val, int _Order) {
#if defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, _Result + _Val, _Order)) {
    }
    return _Result + _Val;
#else // ^^^ defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64) / !defined(_M_IX86) || defined(_M_HYBRID_X86_ARM64) vvvv
    long long _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedExchangeAdd64, _Ptr, _Val);
    return _Result + _Val;
#endif // defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
}
inline signed char _Atomic_add_fetchs8f(volatile signed char* _Ptr, float _Val, int _Order) {
    signed char _Result = (signed char)_Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (signed char) (_Result + _Val), _Order)) {
    }
    return (signed char) (_Result + _Val);
}
inline short _Atomic_add_fetchs16f(volatile short* _Ptr, float _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result + _Val), _Order)) {
    }
    return (short) (_Result + _Val);
}
inline int _Atomic_add_fetchs32f(volatile int* _Ptr, float _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, (int) (_Result + _Val), _Order)) {
    }
    return (int) (_Result + _Val);
}
inline long long _Atomic_add_fetchs64f(volatile long long* _Ptr, float _Val, int _Order) {
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, (long long) (_Result + _Val), _Order)) {
    }
    return (long long) (_Result + _Val);
}
inline unsigned char _Atomic_add_fetchu8f(volatile unsigned char* _Ptr, float _Val, int _Order) {
    unsigned char _Result = (unsigned char)_Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (unsigned char) (_Result + _Val), _Order)) {
    }
    return (unsigned char) (_Result + _Val);
}
inline unsigned short _Atomic_add_fetchu16f(volatile unsigned short* _Ptr, float _Val, int _Order) {
    unsigned short _Result = (unsigned short)_Atomic_load16((volatile short*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16((volatile short*)_Ptr, (short*)&_Result, (unsigned short) (_Result + _Val), _Order)) {
    }
    return (unsigned short) (_Result + _Val);
}
inline unsigned int _Atomic_add_fetchu32f(volatile unsigned int* _Ptr, float _Val, int _Order) {
    unsigned int _Result = (unsigned int)_Atomic_load32((volatile int*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*)_Ptr, (int*)&_Result, (unsigned int) (_Result + _Val), _Order)) {
    }
    return (unsigned int) (_Result + _Val);
}
inline unsigned long long _Atomic_add_fetchu64f(volatile unsigned long long* _Ptr, float _Val, int _Order) {
    unsigned long long _Result = (unsigned long long)_Atomic_load64((volatile long long*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64((volatile long long*)_Ptr, (long long*)&_Result, (unsigned long long) (_Result + _Val), _Order)) {
    }
    return (unsigned long long) (_Result + _Val);
}
inline signed char _Atomic_add_fetchs8d(volatile signed char* _Ptr, double _Val, int _Order) {
    signed char _Result = (signed char)_Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (signed char) (_Result + _Val), _Order)) {
    }
    return (signed char) (_Result + _Val);
}
inline short _Atomic_add_fetchs16d(volatile short* _Ptr, double _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result + _Val), _Order)) {
    }
    return (short) (_Result + _Val);
}
inline int _Atomic_add_fetchs32d(volatile int* _Ptr, double _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, (int) (_Result + _Val), _Order)) {
    }
    return (int) (_Result + _Val);
}
inline long long _Atomic_add_fetchs64d(volatile long long* _Ptr, double _Val, int _Order) {
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, (long long) (_Result + _Val), _Order)) {
    }
    return (long long) (_Result + _Val);
}
inline unsigned char _Atomic_add_fetchu8d(volatile unsigned char* _Ptr, double _Val, int _Order) {
    unsigned char _Result = (unsigned char)_Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (unsigned char) (_Result + _Val), _Order)) {
    }
    return (unsigned char) (_Result + _Val);
}
inline unsigned short _Atomic_add_fetchu16d(volatile unsigned short* _Ptr, double _Val, int _Order) {
    unsigned short _Result = (unsigned short)_Atomic_load16((volatile short*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16((volatile short*)_Ptr, (short*)&_Result, (unsigned short) (_Result + _Val), _Order)) {
    }
    return (unsigned short) (_Result + _Val);
}
inline unsigned int _Atomic_add_fetchu32d(volatile unsigned int* _Ptr, double _Val, int _Order) {
    unsigned int _Result = (unsigned int)_Atomic_load32((volatile int*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*)_Ptr, (int*)&_Result, (unsigned int) (_Result + _Val), _Order)) {
    }
    return (unsigned int) (_Result + _Val);
}
inline unsigned long long _Atomic_add_fetchu64d(volatile unsigned long long* _Ptr, double _Val, int _Order) {
    unsigned long long _Result = (unsigned long long)_Atomic_load64((volatile long long*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64((volatile long long*)_Ptr, (long long*)&_Result, (unsigned long long) (_Result + _Val), _Order)) {
    }
    return (unsigned long long) (_Result + _Val);
}
inline float _Atomic_add_fetchf(volatile float* _Ptr, float _Val, int _Order) {
    float _Result = _Atomic_loadf(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strongf(_Ptr, &_Result, _Result + _Val, _Order)) {
    }
    return _Result + _Val;
}
inline double _Atomic_add_fetchd(volatile double* _Ptr, double _Val, int _Order) {
    double _Result = _Atomic_loadd(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strongd(_Ptr, &_Result, _Result + _Val, _Order)) {
    }
    return _Result + _Val;
}
inline float _Atomic_add_fetchfd(volatile float* _Ptr, double _Val, int _Order) {
    float _Result = _Atomic_loadf(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strongf(_Ptr, &_Result, (float) (_Result + _Val), _Order)) {
    }
    return (float) (_Result + _Val);
}

inline char _Atomic_fetch_sub8(volatile char* _Ptr, int _Val, int _Order) {
    char _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedExchangeAdd8, _Ptr, -(char) _Val);
    return _Result;
}
inline short _Atomic_fetch_sub16(volatile short* _Ptr, int _Val, int _Order) {
    short _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedExchangeAdd16, _Ptr, -(short) _Val);
    return _Result;
}
inline int _Atomic_fetch_sub32(volatile int* _Ptr, int _Val, int _Order) {
    int _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedExchangeAdd, (volatile long*) _Ptr, -_Val);
    return _Result;
}
inline long long _Atomic_fetch_sub64(volatile long long* _Ptr, long long _Val, int _Order) {
#if defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, _Result - _Val, _Order)) {
    }
    return _Result;
#else // ^^^ defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64) / !defined(_M_IX86) || defined(_M_HYBRID_X86_ARM64) vvvv
    long long _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedExchangeAdd64, _Ptr, -_Val);
    return _Result;
#endif // defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
}
inline float _Atomic_fetch_subf(volatile float* _Ptr, float _Val, int _Order) {
    float _Result = _Atomic_loadf(_Ptr, _Atomic_memory_order_relaxed);
    while (!_Atomic_compare_exchange_strongf(_Ptr, &_Result, _Result - _Val, _Order)) {
    }
    return _Result;
}
inline double _Atomic_fetch_subd(volatile double* _Ptr, double _Val, int _Order) {
    double _Result = _Atomic_loadd(_Ptr, _Atomic_memory_order_relaxed);
    while (!_Atomic_compare_exchange_strongd(_Ptr, &_Result, _Result - _Val, _Order)) {
    }
    return _Result;
}

inline char _Atomic_sub_fetch8(volatile char* _Ptr, int _Val, int _Order) {
    char _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedExchangeAdd8, _Ptr, -(char) _Val);
    return (char) (_Result - (char) _Val);
}
inline short _Atomic_sub_fetch16(volatile short* _Ptr, int _Val, int _Order) {
    short _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedExchangeAdd16, _Ptr, -(short) _Val);
    return (short) (_Result - (short) _Val);
}
inline int _Atomic_sub_fetch32(volatile int* _Ptr, int _Val, int _Order) {
    int _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedExchangeAdd, (volatile long*) _Ptr, -_Val);
    return _Result - _Val;
}
inline long long _Atomic_sub_fetch64(volatile long long* _Ptr, long long _Val, int _Order) {
#if defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, _Result - _Val, _Order)) {
    }
    return _Result - _Val;
#else // ^^^ defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64) / !defined(_M_IX86) || defined(_M_HYBRID_X86_ARM64) vvvv
    long long _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedExchangeAdd64, _Ptr, -_Val);
    return _Result - _Val;
#endif // defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
}
inline signed char _Atomic_sub_fetchs8f(volatile signed char* _Ptr, float _Val, int _Order) {
    signed char _Result = (signed char)_Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (signed char) (_Result - _Val), _Order)) {
    }
    return (signed char) (_Result - _Val);
}
inline short _Atomic_sub_fetchs16f(volatile short* _Ptr, float _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result - _Val), _Order)) {
    }
    return (short) (_Result - _Val);
}
inline int _Atomic_sub_fetchs32f(volatile int* _Ptr, float _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, (int) (_Result - _Val), _Order)) {
    }
    return (int) (_Result - _Val);
}
inline long long _Atomic_sub_fetchs64f(volatile long long* _Ptr, float _Val, int _Order) {
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, (long long) (_Result - _Val), _Order)) {
    }
    return (long long) (_Result - _Val);
}
inline unsigned char _Atomic_sub_fetchu8f(volatile unsigned char* _Ptr, float _Val, int _Order) {
    unsigned char _Result = (unsigned char)_Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (unsigned char) (_Result - _Val), _Order)) {
    }
    return (unsigned char) (_Result - _Val);
}
inline unsigned short _Atomic_sub_fetchu16f(volatile unsigned short* _Ptr, float _Val, int _Order) {
    unsigned short _Result = (unsigned short)_Atomic_load16((volatile short*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16((volatile short*)_Ptr, (short*)&_Result, (unsigned short) (_Result - _Val), _Order)) {
    }
    return (unsigned short) (_Result - _Val);
}
inline unsigned int _Atomic_sub_fetchu32f(volatile unsigned int* _Ptr, float _Val, int _Order) {
    unsigned int _Result = (unsigned int)_Atomic_load32((volatile int*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*)_Ptr, (int*)&_Result, (unsigned int) (_Result - _Val), _Order)) {
    }
    return (unsigned int) (_Result - _Val);
}
inline unsigned long long _Atomic_sub_fetchu64f(volatile unsigned long long* _Ptr, float _Val, int _Order) {
    unsigned long long _Result = (unsigned long long)_Atomic_load64((volatile long long*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64((volatile long long*)_Ptr, (long long*)&_Result, (unsigned long long) (_Result - _Val), _Order)) {
    }
    return (unsigned long long) (_Result - _Val);
}
inline signed char _Atomic_sub_fetchs8d(volatile signed char* _Ptr, double _Val, int _Order) {
    signed char _Result = (signed char)_Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (signed char) (_Result - _Val), _Order)) {
    }
    return (signed char) (_Result - _Val);
}
inline short _Atomic_sub_fetchs16d(volatile short* _Ptr, double _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result - _Val), _Order)) {
    }
    return (short) (_Result - _Val);
}
inline int _Atomic_sub_fetchs32d(volatile int* _Ptr, double _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, (int) (_Result - _Val), _Order)) {
    }
    return (int) (_Result - _Val);
}
inline long long _Atomic_sub_fetchs64d(volatile long long* _Ptr, double _Val, int _Order) {
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, (long long) (_Result - _Val), _Order)) {
    }
    return (long long) (_Result - _Val);
}
inline unsigned char _Atomic_sub_fetchu8d(volatile unsigned char* _Ptr, double _Val, int _Order) {
    unsigned char _Result = (unsigned char)_Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (unsigned char) (_Result - _Val), _Order)) {
    }
    return (unsigned char) (_Result - _Val);
}
inline unsigned short _Atomic_sub_fetchu16d(volatile unsigned short* _Ptr, double _Val, int _Order) {
    unsigned short _Result = (unsigned short)_Atomic_load16((volatile short*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16((volatile short*)_Ptr, (short*)&_Result, (unsigned short) (_Result - _Val), _Order)) {
    }
    return (unsigned short) (_Result - _Val);
}
inline unsigned int _Atomic_sub_fetchu32d(volatile unsigned int* _Ptr, double _Val, int _Order) {
    unsigned int _Result = (unsigned int)_Atomic_load32((volatile int*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*)_Ptr, (int*)&_Result, (unsigned int) (_Result - _Val), _Order)) {
    }
    return (unsigned int) (_Result - _Val);
}
inline unsigned long long _Atomic_sub_fetchu64d(volatile unsigned long long* _Ptr, double _Val, int _Order) {
    unsigned long long _Result = (unsigned long long)_Atomic_load64((volatile long long*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64((volatile long long*)_Ptr, (long long*)&_Result, (unsigned long long) (_Result - _Val), _Order)) {
    }
    return (unsigned long long) (_Result - _Val);
}
inline float _Atomic_sub_fetchf(volatile float* _Ptr, float _Val, int _Order) {
    float _Result = _Atomic_loadf(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strongf(_Ptr, &_Result, _Result - _Val, _Order)) {
    }
    return _Result - _Val;
}
inline double _Atomic_sub_fetchd(volatile double* _Ptr, double _Val, int _Order) {
    double _Result = _Atomic_loadd(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strongd(_Ptr, &_Result, _Result - _Val, _Order)) {
    }
    return _Result - _Val;
}
inline float _Atomic_sub_fetchfd(volatile float* _Ptr, double _Val, int _Order) {
    float _Result = _Atomic_loadf(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strongf(_Ptr, &_Result, (float) (_Result - _Val), _Order)) {
    }
    return (float) (_Result - _Val);
}

inline char _Atomic_fetch_and8(volatile char* _Ptr, int _Val, int _Order) {
    char _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedAnd8, _Ptr, (char) _Val);
    return _Result;
}
inline short _Atomic_fetch_and16(volatile short* _Ptr, int _Val, int _Order) {
    short _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedAnd16, _Ptr, (short) _Val);
    return _Result;
}
inline int _Atomic_fetch_and32(volatile int* _Ptr, int _Val, int _Order) {
    int _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedAnd, (volatile long*) _Ptr, _Val);
    return _Result;
}
inline long long _Atomic_fetch_and64(volatile long long* _Ptr, long long _Val, int _Order) {
#if defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, _Result & _Val, _Order)) {
    }
    return _Result;
#else // ^^^ defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64) / !defined(_M_IX86) || defined(_M_HYBRID_X86_ARM64) vvvv
    long long _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedAnd64, _Ptr, _Val);
    return _Result;
#endif // defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
}

inline char _Atomic_and_fetch8(volatile char* _Ptr, int _Val, int _Order) {
    char _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedAnd8, _Ptr, (char) _Val);
    return (char) (_Result & (char) _Val);
}
inline short _Atomic_and_fetch16(volatile short* _Ptr, int _Val, int _Order) {
    short _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedAnd16, _Ptr, (short) _Val);
    return (short) (_Result & (short) _Val);
}
inline int _Atomic_and_fetch32(volatile int* _Ptr, int _Val, int _Order) {
    int _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedAnd, (volatile long*) _Ptr, _Val);
    return (int) (_Result & _Val);
}
inline long long _Atomic_and_fetch64(volatile long long* _Ptr, long long _Val, int _Order) {
#if defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, _Result & _Val, _Order)) {
    }
    return (long long) (_Result & _Val);
#else // ^^^ defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64) / !defined(_M_IX86) || defined(_M_HYBRID_X86_ARM64) vvvv
    long long _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedAnd64, _Ptr, _Val);
    return (long long) (_Result & _Val);
#endif // defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
}

inline char _Atomic_fetch_or8(volatile char* _Ptr, int _Val, int _Order) {
    char _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedOr8, _Ptr, (char) _Val);
    return _Result;
}
inline short _Atomic_fetch_or16(volatile short* _Ptr, int _Val, int _Order) {
    short _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedOr16, _Ptr, (short) _Val);
    return _Result;
}
inline int _Atomic_fetch_or32(volatile int* _Ptr, int _Val, int _Order) {
    int _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedOr, (volatile long*) _Ptr, _Val);
    return _Result;
}
inline long long _Atomic_fetch_or64(volatile long long* _Ptr, long long _Val, int _Order) {
#if defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, _Result | _Val, _Order)) {
    }
    return _Result;
#else // ^^^ defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64) / !defined(_M_IX86) || defined(_M_HYBRID_X86_ARM64) vvvv
    long long _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedOr64, _Ptr, _Val);
    return _Result;
#endif // defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
}

inline char _Atomic_or_fetch8(volatile char* _Ptr, int _Val, int _Order) {
    char _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedOr8, _Ptr, (char) _Val);
    return (char) (_Result | (char) _Val);
}
inline short _Atomic_or_fetch16(volatile short* _Ptr, int _Val, int _Order) {
    short _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedOr16, _Ptr, (short) _Val);
    return (short) (_Result | (short) _Val);
}
inline int _Atomic_or_fetch32(volatile int* _Ptr, int _Val, int _Order) {
    int _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedOr, (volatile long*) _Ptr, _Val);
    return (int) (_Result | _Val);
}
inline long long _Atomic_or_fetch64(volatile long long* _Ptr, long long _Val, int _Order) {
#if defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, _Result | _Val, _Order)) {
    }
    return (long long) (_Result | _Val);
#else // ^^^ defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64) / !defined(_M_IX86) || defined(_M_HYBRID_X86_ARM64) vvvv
    long long _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedOr64, _Ptr, _Val);
    return (long long) (_Result | _Val);
#endif // defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
}

inline char _Atomic_fetch_xor8(volatile char* _Ptr, int _Val, int _Order) {
    char _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedXor8, _Ptr, (char) _Val);
    return _Result;
}
inline short _Atomic_fetch_xor16(volatile short* _Ptr, int _Val, int _Order) {
    short _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedXor16, _Ptr, (short) _Val);
    return _Result;
}
inline int _Atomic_fetch_xor32(volatile int* _Ptr, int _Val, int _Order) {
    int _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedXor, (volatile long*) _Ptr, _Val);
    return _Result;
}
inline long long _Atomic_fetch_xor64(volatile long long* _Ptr, long long _Val, int _Order) {
#if defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, _Result ^ _Val, _Order)) {
    }
    return _Result;
#else // ^^^ defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64) / !defined(_M_IX86) || defined(_M_HYBRID_X86_ARM64) vvvv
    long long _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedXor64, _Ptr, _Val);
    return _Result;
#endif // defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
}

inline char _Atomic_xor_fetch8(volatile char* _Ptr, int _Val, int _Order) {
    char _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedXor8, _Ptr, (char) _Val);
    return (char) (_Result ^ (char) _Val);
}
inline short _Atomic_xor_fetch16(volatile short* _Ptr, int _Val, int _Order) {
    short _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedXor16, _Ptr, (short) _Val);
    return (short) (_Result ^ (short) _Val);
}
inline int _Atomic_xor_fetch32(volatile int* _Ptr, int _Val, int _Order) {
    int _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedXor, (volatile long*) _Ptr, _Val);
    return (int) (_Result ^ _Val);
}
inline long long _Atomic_xor_fetch64(volatile long long* _Ptr, long long _Val, int _Order) {
#if defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, _Result ^ _Val, _Order)) {
    }
    return (long long) (_Result ^ _Val);
#else // ^^^ defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64) / !defined(_M_IX86) || defined(_M_HYBRID_X86_ARM64) vvvv
    long long _Result;
    _ATOMIC_CHOOSE_INTRINSIC(_Order, _Result, _InterlockedXor64, _Ptr, _Val);
    return (long long) (_Result ^ _Val);
#endif // defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)
}

inline char _Atomic_mult_fetch8(volatile char* _Ptr, int _Val, int _Order) {
    char _Result = _Atomic_load8(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8(_Ptr, &_Result, (char) (_Result * (char) _Val), _Order)) {
    }
    return (char) (_Result * (char) _Val);
}
inline short _Atomic_mult_fetch16(volatile short* _Ptr, int _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result * (short) _Val), _Order)) {
    }
    return (short) (_Result * (short) _Val);
}
inline int _Atomic_mult_fetch32(volatile int* _Ptr, int _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, _Result * _Val, _Order)) {
    }
    return (int) (_Result * _Val);
}
inline long long _Atomic_mult_fetch64(volatile long long* _Ptr, long long _Val, int _Order) {
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, _Result * _Val, _Order)) {
    }
    return (long long) (_Result * _Val);
}
inline signed char _Atomic_mult_fetchs8f(volatile signed char* _Ptr, float _Val, int _Order) {
    signed char _Result = (signed char)_Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (signed char) (_Result * _Val), _Order)) {
    }
    return (signed char) (_Result * _Val);
}
inline short _Atomic_mult_fetchs16f(volatile short* _Ptr, float _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result * _Val), _Order)) {
    }
    return (short) (_Result * _Val);
}
inline int _Atomic_mult_fetchs32f(volatile int* _Ptr, float _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, (int) (_Result * _Val), _Order)) {
    }
    return (int) (_Result * _Val);
}
inline long long _Atomic_mult_fetchs64f(volatile long long* _Ptr, float _Val, int _Order) {
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, (long long) (_Result * _Val), _Order)) {
    }
    return (long long) (_Result * _Val);
}
inline unsigned char _Atomic_mult_fetchu8f(volatile unsigned char* _Ptr, float _Val, int _Order) {
    unsigned char _Result = (unsigned char)_Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (unsigned char) (_Result * _Val), _Order)) {
    }
    return (unsigned char) (_Result * _Val);
}
inline unsigned short _Atomic_mult_fetchu16f(volatile unsigned short* _Ptr, float _Val, int _Order) {
    unsigned short _Result = (unsigned short)_Atomic_load16((volatile short*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16((volatile short*)_Ptr, (short*)&_Result, (unsigned short) (_Result * _Val), _Order)) {
    }
    return (unsigned short) (_Result * _Val);
}
inline unsigned int _Atomic_mult_fetchu32f(volatile unsigned int* _Ptr, float _Val, int _Order) {
    unsigned int _Result = (unsigned int)_Atomic_load32((volatile int*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*)_Ptr, (int*)&_Result, (unsigned int) (_Result * _Val), _Order)) {
    }
    return (unsigned int) (_Result * _Val);
}
inline unsigned long long _Atomic_mult_fetchu64f(volatile unsigned long long* _Ptr, float _Val, int _Order) {
    unsigned long long _Result = (unsigned long long)_Atomic_load64((volatile long long*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64((volatile long long*)_Ptr, (long long*)&_Result, (unsigned long long) (_Result * _Val), _Order)) {
    }
    return (unsigned long long) (_Result * _Val);
}
inline signed char _Atomic_mult_fetchs8d(volatile signed char* _Ptr, double _Val, int _Order) {
    signed char _Result = (signed char)_Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (signed char) (_Result * _Val), _Order)) {
    }
    return (signed char) (_Result * _Val);
}
inline short _Atomic_mult_fetchs16d(volatile short* _Ptr, double _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result * _Val), _Order)) {
    }
    return (short) (_Result * _Val);
}
inline int _Atomic_mult_fetchs32d(volatile int* _Ptr, double _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, (int) (_Result * _Val), _Order)) {
    }
    return (int) (_Result * _Val);
}
inline long long _Atomic_mult_fetchs64d(volatile long long* _Ptr, double _Val, int _Order) {
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, (long long) (_Result * _Val), _Order)) {
    }
    return (long long) (_Result * _Val);
}
inline unsigned char _Atomic_mult_fetchu8d(volatile unsigned char* _Ptr, double _Val, int _Order) {
    unsigned char _Result = (unsigned char)_Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (unsigned char) (_Result * _Val), _Order)) {
    }
    return (unsigned char) (_Result * _Val);
}
inline unsigned short _Atomic_mult_fetchu16d(volatile unsigned short* _Ptr, double _Val, int _Order) {
    unsigned short _Result = (unsigned short)_Atomic_load16((volatile short*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16((volatile short*)_Ptr, (short*)&_Result, (unsigned short) (_Result * _Val), _Order)) {
    }
    return (unsigned short) (_Result * _Val);
}
inline unsigned int _Atomic_mult_fetchu32d(volatile unsigned int* _Ptr, double _Val, int _Order) {
    unsigned int _Result = (unsigned int)_Atomic_load32((volatile int*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*)_Ptr, (int*)&_Result, (unsigned int) (_Result * _Val), _Order)) {
    }
    return (unsigned int) (_Result * _Val);
}
inline unsigned long long _Atomic_mult_fetchu64d(volatile unsigned long long* _Ptr, double _Val, int _Order) {
    unsigned long long _Result = (unsigned long long)_Atomic_load64((volatile long long*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64((volatile long long*)_Ptr, (long long*)&_Result, (unsigned long long) (_Result * _Val), _Order)) {
    }
    return (unsigned long long) (_Result * _Val);
}
inline float _Atomic_mult_fetchf(volatile float* _Ptr, float _Val, int _Order) {
    float _Result = _Atomic_loadf(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strongf(_Ptr, &_Result, _Result * _Val, _Order)) {
    }
    return _Result * _Val;
}
inline double _Atomic_mult_fetchd(volatile double* _Ptr, double _Val, int _Order) {
    double _Result = _Atomic_loadd(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strongd(_Ptr, &_Result, _Result * _Val, _Order)) {
    }
    return _Result * _Val;
}
inline float _Atomic_mult_fetchfd(volatile float* _Ptr, double _Val, int _Order) {
    float _Result = _Atomic_loadf(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strongf(_Ptr, &_Result, (float) (_Result * _Val), _Order)) {
    }
    return (float) (_Result * _Val);
}

inline unsigned char _Atomic_div_fetch8(volatile unsigned char* _Ptr, unsigned int _Val, int _Order) {
    unsigned char _Result = (unsigned char) _Atomic_load8((volatile char*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8(
        (volatile char*) _Ptr, (char*) &_Result, (unsigned char) (_Result / _Val), _Order)) {
    }
    return (unsigned char) (_Result / _Val);
}
inline unsigned short _Atomic_div_fetch16(volatile unsigned short* _Ptr, unsigned int _Val, int _Order) {
    unsigned short _Result = (unsigned short) _Atomic_load16((volatile short*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(
        (volatile short*) _Ptr, (short*) &_Result, (unsigned short) (_Result / _Val), _Order)) {
    }
    return (unsigned short) (_Result / _Val);
}
inline unsigned int _Atomic_div_fetch32(volatile unsigned int* _Ptr, unsigned int _Val, int _Order) {
    unsigned int _Result = (unsigned int) _Atomic_load32((volatile int*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*) _Ptr, (int*) &_Result, (unsigned int) (_Result / _Val), _Order)) {
    }
    return (unsigned int) (_Result / _Val);
}
inline unsigned long long _Atomic_div_fetch64(volatile unsigned long long* _Ptr, unsigned long long _Val, int _Order) {
    unsigned long long _Result =
        (unsigned long long) _Atomic_load64((volatile long long*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(
        (volatile long long*) _Ptr, (long long*) &_Result, (unsigned long long) (_Result / _Val), _Order)) {
    }
    return (unsigned long long) (_Result / _Val);
}
inline unsigned char _Atomic_div_fetch8_64(volatile unsigned char* _Ptr, unsigned long long _Val, int _Order) {
    unsigned char _Result = (unsigned char) _Atomic_load8((volatile char*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8(
        (volatile char*) _Ptr, (char*) &_Result, (unsigned char) (_Result / _Val), _Order)) {
    }
    return (unsigned char) (_Result / _Val);
}
inline unsigned short _Atomic_div_fetch16_64(volatile unsigned short* _Ptr, unsigned long long _Val, int _Order) {
    unsigned short _Result = (unsigned short) _Atomic_load16((volatile short*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(
        (volatile short*) _Ptr, (short*) &_Result, (unsigned short) (_Result / _Val), _Order)) {
    }
    return (unsigned short) (_Result / _Val);
}
inline unsigned int _Atomic_div_fetch32_64(volatile unsigned int* _Ptr, unsigned long long _Val, int _Order) {
    unsigned int _Result = (unsigned int) _Atomic_load32((volatile int*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*) _Ptr, (int*) &_Result, (unsigned int) (_Result / _Val), _Order)) {
    }
    return (unsigned int) (_Result / _Val);
}
inline signed char _Atomic_div_fetchs8f(volatile signed char* _Ptr, float _Val, int _Order) {
    signed char _Result = (signed char)_Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (signed char) (_Result / _Val), _Order)) {
    }
    return (signed char) (_Result / _Val);
}
inline short _Atomic_div_fetchs16f(volatile short* _Ptr, float _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result / _Val), _Order)) {
    }
    return (short) (_Result / _Val);
}
inline int _Atomic_div_fetchs32f(volatile int* _Ptr, float _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, (int) (_Result / _Val), _Order)) {
    }
    return (int) (_Result / _Val);
}
inline long long _Atomic_div_fetchs64f(volatile long long* _Ptr, float _Val, int _Order) {
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, (long long) (_Result / _Val), _Order)) {
    }
    return (long long) (_Result / _Val);
}
inline unsigned char _Atomic_div_fetchu8f(volatile unsigned char* _Ptr, float _Val, int _Order) {
    unsigned char _Result = (unsigned char)_Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (unsigned char) (_Result / _Val), _Order)) {
    }
    return (unsigned char) (_Result / _Val);
}
inline unsigned short _Atomic_div_fetchu16f(volatile unsigned short* _Ptr, float _Val, int _Order) {
    unsigned short _Result = (unsigned short)_Atomic_load16((volatile short*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16((volatile short*)_Ptr, (short*)&_Result, (unsigned short) (_Result / _Val), _Order)) {
    }
    return (unsigned short) (_Result / _Val);
}
inline unsigned int _Atomic_div_fetchu32f(volatile unsigned int* _Ptr, float _Val, int _Order) {
    unsigned int _Result = (unsigned int)_Atomic_load32((volatile int*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*)_Ptr, (int*)&_Result, (unsigned int) (_Result / _Val), _Order)) {
    }
    return (unsigned int) (_Result / _Val);
}
inline unsigned long long _Atomic_div_fetchu64f(volatile unsigned long long* _Ptr, float _Val, int _Order) {
    unsigned long long _Result = (unsigned long long)_Atomic_load64((volatile long long*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64((volatile long long*)_Ptr, (long long*)&_Result, (unsigned long long) (_Result / _Val), _Order)) {
    }
    return (unsigned long long) (_Result / _Val);
}
inline signed char _Atomic_div_fetchs8d(volatile signed char* _Ptr, double _Val, int _Order) {
    signed char _Result = (signed char)_Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (signed char) (_Result / _Val), _Order)) {
    }
    return (signed char) (_Result / _Val);
}
inline short _Atomic_div_fetchs16d(volatile short* _Ptr, double _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result / _Val), _Order)) {
    }
    return (short) (_Result / _Val);
}
inline int _Atomic_div_fetchs32d(volatile int* _Ptr, double _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, (int) (_Result / _Val), _Order)) {
    }
    return (int) (_Result / _Val);
}
inline long long _Atomic_div_fetchs64d(volatile long long* _Ptr, double _Val, int _Order) {
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, (long long) (_Result / _Val), _Order)) {
    }
    return (long long) (_Result / _Val);
}
inline unsigned char _Atomic_div_fetchu8d(volatile unsigned char* _Ptr, double _Val, int _Order) {
    unsigned char _Result = (unsigned char)_Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (unsigned char) (_Result / _Val), _Order)) {
    }
    return (unsigned char) (_Result / _Val);
}
inline unsigned short _Atomic_div_fetchu16d(volatile unsigned short* _Ptr, double _Val, int _Order) {
    unsigned short _Result = (unsigned short)_Atomic_load16((volatile short*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16((volatile short*)_Ptr, (short*)&_Result, (unsigned short) (_Result / _Val), _Order)) {
    }
    return (unsigned short) (_Result / _Val);
}
inline unsigned int _Atomic_div_fetchu32d(volatile unsigned int* _Ptr, double _Val, int _Order) {
    unsigned int _Result = (unsigned int)_Atomic_load32((volatile int*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*)_Ptr, (int*)&_Result, (unsigned int) (_Result / _Val), _Order)) {
    }
    return (unsigned int) (_Result / _Val);
}
inline unsigned long long _Atomic_div_fetchu64d(volatile unsigned long long* _Ptr, double _Val, int _Order) {
    unsigned long long _Result = (unsigned long long)_Atomic_load64((volatile long long*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64((volatile long long*)_Ptr, (long long*)&_Result, (unsigned long long) (_Result / _Val), _Order)) {
    }
    return (unsigned long long) (_Result / _Val);
}
inline float _Atomic_div_fetchf(volatile float* _Ptr, float _Val, int _Order) {
    float _Result = _Atomic_loadf(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strongf(_Ptr, &_Result, _Result / _Val, _Order)) {
    }
    return _Result / _Val;
}
inline double _Atomic_div_fetchd(volatile double* _Ptr, double _Val, int _Order) {
    double _Result = _Atomic_loadd(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strongd(_Ptr, &_Result, _Result / _Val, _Order)) {
    }
    return _Result / _Val;
}
inline float _Atomic_div_fetchfd(volatile float* _Ptr, double _Val, int _Order) {
    float _Result = _Atomic_loadf(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strongf(_Ptr, &_Result, (float) (_Result / _Val), _Order)) {
    }
    return (float) (_Result / _Val);
}

inline signed char _Atomic_divs_fetch8(volatile signed char* _Ptr, unsigned int _Val, int _Order) {
    signed char _Result = (signed char) _Atomic_load8((volatile char*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8(
        (volatile char*) _Ptr, (char*) &_Result, (signed char) (_Result / _Val), _Order)) {
    }
    return (signed char) (_Result / _Val);
}
inline short _Atomic_divs_fetch16(volatile short* _Ptr, unsigned int _Val, int _Order) {
    short _Result = _Atomic_load16( _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result / _Val), _Order)) {
    }
    return (short) (_Result / _Val);
}
inline int _Atomic_divs_fetch32(volatile int* _Ptr, unsigned int _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, (int) (_Result / _Val), _Order)) {
    }
    return (int) (_Result / _Val);
}
inline long long _Atomic_divs_fetch64(volatile long long* _Ptr, unsigned long long _Val, int _Order) {
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, (long long) (_Result / _Val), _Order)) {
    }
    return (long long) (_Result / _Val);
}
inline signed char _Atomic_divs_fetch8_64(volatile signed char* _Ptr, unsigned long long _Val, int _Order) {
    signed char _Result = (signed char) _Atomic_load8((volatile char*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8(
        (volatile char*) _Ptr, (char*) &_Result, (signed char) (_Result / _Val), _Order)) {
    }
    return (signed char) (_Result / _Val);
}
inline short _Atomic_divs_fetch16_64(volatile short* _Ptr, unsigned long long _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result / _Val), _Order)) {
    }
    return (short) (_Result / _Val);
}
inline int _Atomic_divs_fetch32_64(volatile int* _Ptr, unsigned long long _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, (int) (_Result / _Val), _Order)) {
    }
    return (int) (_Result / _Val);
}

inline signed char _Atomic_idiv_fetch8(volatile signed char* _Ptr, int _Val, int _Order) {
    signed char _Result = (signed char) _Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*) &_Result, (signed char) (_Result / _Val), _Order)) {
    }
    return (signed char) (_Result / _Val);
}
inline short _Atomic_idiv_fetch16(volatile short* _Ptr, int _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result / _Val), _Order)) {
    }
    return (short) (_Result / _Val);
}
inline int _Atomic_idiv_fetch32(volatile int* _Ptr, int _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, _Result / _Val, _Order)) {
    }
    return _Result / _Val;
}
inline long long _Atomic_idiv_fetch64(volatile long long* _Ptr, long long _Val, int _Order) {
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, _Result / _Val, _Order)) {
    }
    return _Result / _Val;
}
inline signed char _Atomic_idiv_fetch8_64(volatile signed char* _Ptr, long long _Val, int _Order) {
    signed char _Result = (signed char) _Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*) &_Result, (signed char) (_Result / _Val), _Order)) {
    }
    return (signed char) (_Result / _Val);
}
inline short _Atomic_idiv_fetch16_64(volatile short* _Ptr, long long _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result / _Val), _Order)) {
    }
    return (short) (_Result / _Val);
}
inline int _Atomic_idiv_fetch32_64(volatile int* _Ptr, long long _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, (int) (_Result / _Val), _Order)) {
    }
    return (int) (_Result / _Val);
}

inline unsigned char _Atomic_idivu_fetch8(volatile unsigned char* _Ptr, int _Val, int _Order) {
    unsigned char _Result = (unsigned char) _Atomic_load8((volatile char*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*) _Ptr, (char*) &_Result, (unsigned char) (_Result / _Val), _Order)) {
    }
    return (unsigned char) (_Result / _Val);
}
inline unsigned short _Atomic_idivu_fetch16(volatile unsigned short* _Ptr, int _Val, int _Order) {
    unsigned short _Result = _Atomic_load16((volatile short*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16((volatile short*) _Ptr, (short*) &_Result, (unsigned short) (_Result / _Val), _Order)) {
    }
    return (unsigned short) (_Result / _Val);
}
inline unsigned int _Atomic_idivu_fetch32(volatile unsigned int* _Ptr, int _Val, int _Order) {
    unsigned int _Result = _Atomic_load32((volatile int*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*) _Ptr, (int*)&_Result, (unsigned int) (_Result / _Val), _Order)) {
    }
    return (unsigned int) (_Result / _Val);
}
inline unsigned long long _Atomic_idivu_fetch64(volatile unsigned long long* _Ptr, long long _Val, int _Order) {
    unsigned long long _Result = _Atomic_load64((volatile long long*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64((volatile long long*) _Ptr, (long long*)&_Result, (unsigned long long) (_Result / _Val), _Order)) {
    }
    return (unsigned long long) (_Result / _Val);
}
inline unsigned char _Atomic_idivu_fetch8_64(volatile unsigned char* _Ptr, long long _Val, int _Order) {
    unsigned char _Result = (unsigned char) _Atomic_load8((volatile char*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*) _Ptr, (char*) &_Result, (unsigned char) (_Result / _Val), _Order)) {
    }
    return (unsigned char) (_Result / _Val);
}
inline unsigned short _Atomic_idivu_fetch16_64(volatile unsigned short* _Ptr, long long _Val, int _Order) {
    unsigned short _Result = _Atomic_load16((volatile short*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16((volatile short*) _Ptr, (short*)&_Result, (unsigned short) (_Result / _Val), _Order)) {
    }
    return (unsigned short) (_Result / _Val);
}
inline unsigned int _Atomic_idivu_fetch32_64(volatile unsigned int* _Ptr, long long _Val, int _Order) {
    unsigned int _Result = _Atomic_load32((volatile int*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*) _Ptr, (int*)&_Result, (unsigned int) (_Result / _Val), _Order)) {
    }
    return (unsigned int) (_Result / _Val);
}

inline char _Atomic_shl_fetch8(volatile char* _Ptr, int _Val, int _Order) {
    char _Result = _Atomic_load8(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8(_Ptr, &_Result, (char) (_Result << _Val), _Order)) {
    }
    return (char) (_Result << _Val);
}
inline short _Atomic_shl_fetch16(volatile short* _Ptr, int _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result << _Val), _Order)) {
    }
    return (short) (_Result << _Val);
}
inline int _Atomic_shl_fetch32(volatile int* _Ptr, int _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, _Result << _Val, _Order)) {
    }
    return _Result << _Val;
}
inline long long _Atomic_shl_fetch64(volatile long long* _Ptr, long long _Val, int _Order) {
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, _Result << _Val, _Order)) {
    }
    return _Result << _Val;
}

inline signed char _Atomic_sar_fetch8(volatile signed char* _Ptr, int _Val, int _Order) {
    signed char _Result = _Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*)&_Result, (signed char) (_Result >> _Val), _Order)) {
    }
    return (signed char) (_Result >> _Val);
}
inline short _Atomic_sar_fetch16(volatile short* _Ptr, int _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result >> _Val), _Order)) {
    }
    return (short) (_Result >> _Val);
}
inline int _Atomic_sar_fetch32(volatile int* _Ptr, int _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, _Result >> _Val, _Order)) {
    }
    return _Result >> _Val;
}
inline long long _Atomic_sar_fetch64(volatile long long* _Ptr, int _Val, int _Order) {
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, (long long) (_Result >> _Val), _Order)) {
    }
    return (long long) (_Result >> _Val);
}

inline unsigned char _Atomic_shr2_fetch8(volatile unsigned char* _Ptr, int _Val, int _Order) {
    unsigned char _Result = _Atomic_load8((volatile char*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*) _Ptr, (char*)&_Result, (unsigned char) (_Result >> _Val), _Order)) {
    }
    return (unsigned char) (_Result >> _Val);
}
inline unsigned short _Atomic_shr2_fetch16(volatile unsigned short* _Ptr, int _Val, int _Order) {
    unsigned short _Result = _Atomic_load16((volatile short*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16((volatile short*) _Ptr, (short*)&_Result, (unsigned short) (_Result >> _Val), _Order)) {
    }
    return (unsigned short) (_Result >> _Val);
}
inline unsigned int _Atomic_shr2_fetch32(volatile unsigned int* _Ptr, int _Val, int _Order) {
    unsigned int _Result = _Atomic_load32((volatile int*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*) _Ptr, (int*)&_Result, (unsigned int) (_Result >> _Val), _Order)) {
    }
    return (unsigned int) (_Result >> _Val);
}
inline unsigned long long _Atomic_shr2_fetch64(volatile unsigned long long* _Ptr, int _Val, int _Order) {
    unsigned long long _Result = _Atomic_load64((volatile long long*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64((volatile long long*) _Ptr, (long long*)&_Result, (unsigned long long) (_Result >> _Val), _Order)) {
    }
    return (unsigned long long) (_Result >> _Val);
}

inline signed char _Atomic_imod_fetch8(volatile signed char* _Ptr, int _Val, int _Order) {
    signed char _Result = (signed char) _Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*) &_Result, (signed char) (_Result % _Val), _Order)) {
    }
    return (signed char) (_Result % _Val);
}
inline short _Atomic_imod_fetch16(volatile short* _Ptr, int _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result % _Val), _Order)) {
    }
    return (short) (_Result % _Val);
}
inline int _Atomic_imod_fetch32(volatile int* _Ptr, int _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, _Result % _Val, _Order)) {
    }
    return _Result % _Val;
}
inline long long _Atomic_imod_fetch64(volatile long long* _Ptr, long long _Val, int _Order) {
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, _Result % _Val, _Order)) {
    }
    return _Result % _Val;
}

inline signed char _Atomic_imod_fetch8_64(volatile signed char* _Ptr, long long _Val, int _Order) {
    signed char _Result = (signed char) _Atomic_load8((volatile char*)_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*)_Ptr, (char*) &_Result, (signed char) (_Result % _Val), _Order)) {
    }
    return (signed char) (_Result % _Val);
}
inline short _Atomic_imod_fetch16_64(volatile short* _Ptr, long long _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result % _Val), _Order)) {
    }
    return (short) (_Result % _Val);
}
inline int _Atomic_imod_fetch32_64(volatile int* _Ptr, long long _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, (int) (_Result % _Val), _Order)) {
    }
    return (int) (_Result % _Val);
}

inline unsigned char _Atomic_imodu_fetch8(volatile unsigned char* _Ptr, int _Val, int _Order) {
    unsigned char _Result = (unsigned char) _Atomic_load8((volatile char*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*) _Ptr, (char*) &_Result, (unsigned char) (_Result % _Val), _Order)) {
    }
    return (unsigned char) (_Result % _Val);
}
inline unsigned short _Atomic_imodu_fetch16(volatile unsigned short* _Ptr, int _Val, int _Order) {
    unsigned short _Result = _Atomic_load16((volatile short*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16((volatile short*) _Ptr, (short*)&_Result, (unsigned short) (_Result % _Val), _Order)) {
    }
    return (unsigned short) (_Result % _Val);
}
inline unsigned int _Atomic_imodu_fetch32(volatile unsigned int* _Ptr, int _Val, int _Order) {
    unsigned int _Result = _Atomic_load32((volatile int*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*) _Ptr, (int*)&_Result, (unsigned int) (_Result % _Val), _Order)) {
    }
    return (unsigned int) (_Result % _Val);
}
inline unsigned long long _Atomic_imodu_fetch64(volatile unsigned long long* _Ptr, long long _Val, int _Order) {
    unsigned long long _Result = _Atomic_load64((volatile long long*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64((volatile long long*) _Ptr, (long long*)&_Result, (unsigned long long) (_Result % _Val), _Order)) {
    }
    return (unsigned long long) (_Result % _Val);
}

inline unsigned char _Atomic_imodu_fetch8_64(volatile unsigned char* _Ptr, long long _Val, int _Order) {
    unsigned char _Result = (unsigned char) _Atomic_load8((volatile char*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8((volatile char*) _Ptr, (char*) &_Result, (unsigned char) (_Result % _Val), _Order)) {
    }
    return (unsigned char) (_Result % _Val);
}
inline unsigned short _Atomic_imodu_fetch16_64(volatile unsigned short* _Ptr, long long _Val, int _Order) {
    unsigned short _Result = _Atomic_load16((volatile short*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16((volatile short*) _Ptr, (short*)&_Result, (unsigned short) (_Result % _Val), _Order)) {
    }
    return (unsigned short) (_Result % _Val);
}
inline unsigned int _Atomic_imodu_fetch32_64(volatile unsigned int* _Ptr, long long _Val, int _Order) {
    unsigned int _Result = _Atomic_load32((volatile int*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*) _Ptr, (int*)&_Result, (unsigned int) (_Result % _Val), _Order)) {
    }
    return (unsigned int) (_Result % _Val);
}

inline unsigned char _Atomic_mod_fetch8(volatile unsigned char* _Ptr, unsigned int _Val, int _Order) {
    unsigned char _Result = (unsigned char) _Atomic_load8((volatile char*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8(
        (volatile char*) _Ptr, (char*) &_Result, (unsigned char) (_Result % _Val), _Order)) {
    }
    return (unsigned char) (_Result % _Val);
}
inline unsigned short _Atomic_mod_fetch16(volatile unsigned short* _Ptr, unsigned int _Val, int _Order) {
    unsigned short _Result = (unsigned short) _Atomic_load16((volatile short*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(
        (volatile short*) _Ptr, (short*) &_Result, (unsigned short) (_Result % _Val), _Order)) {
    }
    return (unsigned short) (_Result % _Val);
}
inline unsigned int _Atomic_mod_fetch32(volatile unsigned int* _Ptr, unsigned int _Val, int _Order) {
    unsigned int _Result = (unsigned int) _Atomic_load32((volatile int*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*) _Ptr, (int*) &_Result, (unsigned int) (_Result % _Val), _Order)) {
    }
    return (unsigned int) (_Result % _Val);
}
inline unsigned long long _Atomic_mod_fetch64(volatile unsigned long long* _Ptr, unsigned long long _Val, int _Order) {
    unsigned long long _Result =
        (unsigned long long) _Atomic_load64((volatile long long*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(
        (volatile long long*) _Ptr, (long long*) &_Result, (unsigned long long) (_Result % _Val), _Order)) {
    }
    return (unsigned long long) (_Result % _Val);
}
inline unsigned char _Atomic_mod_fetch8_64(volatile unsigned char* _Ptr, unsigned long long _Val, int _Order) {
    unsigned char _Result = (unsigned char) _Atomic_load8((volatile char*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8(
        (volatile char*) _Ptr, (char*) &_Result, (unsigned char) (_Result % _Val), _Order)) {
    }
    return (unsigned char) (_Result % _Val);
}
inline unsigned short _Atomic_mod_fetch16_64(volatile unsigned short* _Ptr, unsigned long long _Val, int _Order) {
    unsigned short _Result = (unsigned short) _Atomic_load16((volatile short*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(
        (volatile short*) _Ptr, (short*) &_Result, (unsigned short) (_Result % _Val), _Order)) {
    }
    return (unsigned short) (_Result % _Val);
}
inline unsigned int _Atomic_mod_fetch32_64(volatile unsigned int* _Ptr, unsigned long long _Val, int _Order) {
    unsigned int _Result = (unsigned int) _Atomic_load32((volatile int*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32((volatile int*) _Ptr, (int*) &_Result, (unsigned int) (_Result % _Val), _Order)) {
    }
    return (unsigned int) (_Result % _Val);
}

inline signed char _Atomic_mods_fetch8(volatile signed char* _Ptr, unsigned int _Val, int _Order) {
    signed char _Result = (signed char) _Atomic_load8((volatile char*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8(
        (volatile char*) _Ptr, (char*) &_Result, (signed char) (_Result % _Val), _Order)) {
    }
    return (signed char) (_Result % _Val);
}
inline short _Atomic_mods_fetch16(volatile short* _Ptr, unsigned int _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result % _Val), _Order)) {
    }
    return (short) (_Result % _Val);
}
inline int _Atomic_mods_fetch32(volatile int* _Ptr, unsigned int _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, (int) (_Result % _Val), _Order)) {
    }
    return (int) (_Result % _Val);
}
inline long long _Atomic_mods_fetch64(volatile long long* _Ptr, unsigned long long _Val, int _Order) {
    long long _Result = _Atomic_load64(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong64(_Ptr, &_Result, (long long) (_Result % _Val), _Order)) {
    }
    return (long long) (_Result % _Val);
}
inline signed char _Atomic_mods_fetch8_64(volatile signed char* _Ptr, unsigned long long _Val, int _Order) {
    signed char _Result = (signed char) _Atomic_load8((volatile char*) _Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong8(
        (volatile char*) _Ptr, (char*) &_Result, (signed char) (_Result % _Val), _Order)) {
    }
    return (signed char) (_Result % _Val);
}
inline short _Atomic_mods_fetch16_64(volatile short* _Ptr, unsigned long long _Val, int _Order) {
    short _Result = _Atomic_load16(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong16(_Ptr, &_Result, (short) (_Result % _Val), _Order)) {
    }
    return (short) (_Result % _Val);
}
inline int _Atomic_mods_fetch32_64(volatile int* _Ptr, unsigned long long _Val, int _Order) {
    int _Result = _Atomic_load32(_Ptr, _Atomic_memory_order_seq_cst);
    while (!_Atomic_compare_exchange_strong32(_Ptr, &_Result, (int) (_Result % _Val), _Order)) {
    }
    return (int) (_Result % _Val);
}

inline void _Atomic_lock_and_store(volatile void* _Obj, const void* _Desired, int _Offset, size_t _Size) {
    _Atomic_lock_acquire(_Obj);
    memmove((char*) _Obj + _Offset, _Desired, _Size);
    _Atomic_lock_release(_Obj);
}

inline void _Atomic_lock_and_load(volatile void* _Obj, void* _Dest, int _Offset, size_t _Size) {
    _Atomic_lock_acquire(_Obj);
    memmove(_Dest, (char*) _Obj + _Offset, _Size);
    _Atomic_lock_release(_Obj);
}

inline void _Atomic_lock_and_exchange(
    volatile void* _Obj, const void* _Desired, void* _Dest, int _Offset, size_t _Size) {
    _Atomic_lock_acquire(_Obj);
    memmove(_Dest, (char*) _Obj + _Offset, _Size);
    memmove((char*) _Obj + _Offset, _Desired, _Size);
    _Atomic_lock_release(_Obj);
}

inline _Bool _Atomic_lock_and_compare_exchange_strong(
    volatile void* _Obj, void* _Expected, const void* _Desired, int _Offset, size_t _Size) {
    _Bool _Result;
    _Atomic_lock_acquire(_Obj);
    _Result = memcmp((char*) _Obj + _Offset, _Expected, _Size) == 0;
    if (_Result) {
        memmove((char*) _Obj + _Offset, _Desired, _Size);
    } else {
        memmove(_Expected, (char*) _Obj + _Offset, _Size);
    }
    _Atomic_lock_release(_Obj);
    return _Result;
}

#undef _ATOMIC_CHOOSE_INTRINSIC
#undef _ATOMIC_POST_LOAD_BARRIER_AS_NEEDED
#undef _ATOMIC_STORE_PREFIX
#undef _ATOMIC_STORE_SEQ_CST_ARM
#undef _ATOMIC_STORE_SEQ_CST_X86_X64
#undef _ATOMIC_STORE_32_SEQ_CST_X86_X64
#undef _ATOMIC_STORE_SEQ_CST
#undef _ATOMIC_STORE_32_SEQ_CST
#undef _ATOMIC_STORE_64_SEQ_CST
#undef _ATOMIC_STORE_64_SEQ_CST_IX86
#undef _ATOMIC_INVALID_PARAMETER
#undef _ATOMIC_STORE_SEQ_CST_ARM64
#undef __LOAD_ACQUIRE_ARM64
#undef _ATOMIC_LOAD_ARM64
#undef __STORE_RELEASE
#undef _STD_ATOMIC_USE_ARM64_LDAR_STLR

#undef _STD_COMPARE_EXCHANGE_128
#undef _INVALID_MEMORY_ORDER
#undef _Compiler_or_memory_barrier
#undef _Memory_barrier
#undef _Memory_load_acquire_barrier
#undef _Compiler_barrier

#undef _CONCATX
#undef _CONCAT
#undef _INTRIN_RELAXED
#undef _INTRIN_ACQUIRE
#undef _INTRIN_RELEASE
#undef _INTRIN_ACQ_REL
#undef _YIELD_PROCESSOR
