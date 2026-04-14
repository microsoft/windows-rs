// __msvc_cxx_stdatomic.hpp internal header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef __MSVC_CXX_STDATOMIC_HPP
#define __MSVC_CXX_STDATOMIC_HPP

// see _STL_COMPILER_PREPROCESSOR in yvals_core.h
#if !defined(RC_INVOKED) && !defined(Q_MOC_RUN) && !defined(__midl)

// provide a specific error message for C compilers, before the general error message in yvals_core.h
#ifndef __cplusplus
#error <__msvc_cxx_stdatomic.hpp> is an internal header. It is incompatible with C and should not be directly included.
#endif // !defined(__cplusplus)

#include <yvals.h>

#ifdef _M_CEE_PURE
#error <stdatomic.h> is not supported when compiling with /clr:pure.
#endif // defined(_M_CEE_PURE)

#if !_HAS_CXX23
_EMIT_STL_WARNING(STL4038, "The contents of <stdatomic.h> are available only with C++23 or later.");
#else // ^^^ !_HAS_CXX23 / _HAS_CXX23 vvv

#include <atomic>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

template <class _Ty>
using _Std_atomic = _STD atomic<_Ty>;

#define _Atomic(T) _Std_atomic<T>

using _STD memory_order;
using _STD memory_order_relaxed;
using _STD memory_order_consume;
using _STD memory_order_acquire;
using _STD memory_order_release;
using _STD memory_order_acq_rel;
using _STD memory_order_seq_cst;

using _STD atomic_flag;

using _STD atomic_bool;
using _STD atomic_char;
using _STD atomic_schar;
using _STD atomic_uchar;
using _STD atomic_short;
using _STD atomic_ushort;
using _STD atomic_int;
using _STD atomic_uint;
using _STD atomic_long;
using _STD atomic_ulong;
using _STD atomic_llong;
using _STD atomic_ullong;

#ifdef __cpp_lib_char8_t
using _STD atomic_char8_t;
#endif // defined(__cpp_lib_char8_t)

using _STD atomic_char16_t;
using _STD atomic_char32_t;
using _STD atomic_wchar_t;
using _STD atomic_int8_t;
using _STD atomic_uint8_t;
using _STD atomic_int16_t;
using _STD atomic_uint16_t;
using _STD atomic_int32_t;
using _STD atomic_uint32_t;
using _STD atomic_int64_t;
using _STD atomic_uint64_t;
using _STD atomic_int_least8_t;
using _STD atomic_uint_least8_t;
using _STD atomic_int_least16_t;
using _STD atomic_uint_least16_t;
using _STD atomic_int_least32_t;
using _STD atomic_uint_least32_t;
using _STD atomic_int_least64_t;
using _STD atomic_uint_least64_t;
using _STD atomic_int_fast8_t;
using _STD atomic_uint_fast8_t;
using _STD atomic_int_fast16_t;
using _STD atomic_uint_fast16_t;
using _STD atomic_int_fast32_t;
using _STD atomic_uint_fast32_t;
using _STD atomic_int_fast64_t;
using _STD atomic_uint_fast64_t;
using _STD atomic_intptr_t;
using _STD atomic_uintptr_t;
using _STD atomic_size_t;
using _STD atomic_ptrdiff_t;
using _STD atomic_intmax_t;
using _STD atomic_uintmax_t;

using _STD atomic_is_lock_free;
using _STD atomic_load;
using _STD atomic_load_explicit;
using _STD atomic_store;
using _STD atomic_store_explicit;
using _STD atomic_exchange;
using _STD atomic_exchange_explicit;
using _STD atomic_compare_exchange_strong;
using _STD atomic_compare_exchange_strong_explicit;
using _STD atomic_compare_exchange_weak;
using _STD atomic_compare_exchange_weak_explicit;
using _STD atomic_fetch_add;
using _STD atomic_fetch_add_explicit;
using _STD atomic_fetch_sub;
using _STD atomic_fetch_sub_explicit;
using _STD atomic_fetch_or;
using _STD atomic_fetch_or_explicit;
using _STD atomic_fetch_xor;
using _STD atomic_fetch_xor_explicit;
using _STD atomic_fetch_and;
using _STD atomic_fetch_and_explicit;
using _STD atomic_flag_test_and_set;
using _STD atomic_flag_test_and_set_explicit;
using _STD atomic_flag_clear;
using _STD atomic_flag_clear_explicit;

using _STD atomic_thread_fence;
using _STD atomic_signal_fence;

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // ^^^ _HAS_CXX23 ^^^

#endif // !defined(RC_INVOKED) && !defined(Q_MOC_RUN) && !defined(__midl)
#endif // __MSVC_CXX_STDATOMIC_HPP
