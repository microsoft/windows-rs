// Copyright (c) Microsoft Corporation. All rights reserved.
//
// C11 version of stdatomic.h
#pragma once

#ifdef __cplusplus
// This header should never be included in C++ mode, C++ has it's own stdatomic.h
#error "vcruntime_c11_stdatomic.h is a C-only header"
#endif

#ifdef __STDC_NO_ATOMICS__ 
#error "C atomic support is not enabled"
#endif

#if __STDC_VERSION__ < 201112L
#error "C atomics require C11 or later"
#endif

#include <stddef.h>
#include <stdint.h>
#include <uchar.h>
#include <vcruntime_c11_atomic_support.h>

typedef enum memory_order {
    memory_order_relaxed = _Atomic_memory_order_relaxed,
    memory_order_consume = _Atomic_memory_order_consume,
    memory_order_acquire = _Atomic_memory_order_acquire,
    memory_order_release = _Atomic_memory_order_release,
    memory_order_acq_rel = _Atomic_memory_order_acq_rel,
    memory_order_seq_cst = _Atomic_memory_order_seq_cst
} memory_order;

#define ATOMIC_BOOL_LOCK_FREE     1
#define ATOMIC_CHAR_LOCK_FREE     1
#define ATOMIC_CHAR16_T_LOCK_FREE 1
#define ATOMIC_CHAR32_T_LOCK_FREE 1
#define ATOMIC_WCHAR_T_LOCK_FREE  1
#define ATOMIC_SHORT_LOCK_FREE    1
#define ATOMIC_INT_LOCK_FREE      1
#define ATOMIC_LONG_LOCK_FREE     1
#define ATOMIC_LLONG_LOCK_FREE    1
#define ATOMIC_POINTER_LOCK_FREE  1

typedef _Atomic(_Bool) atomic_bool;
typedef _Atomic(char) atomic_char;
typedef _Atomic(signed char) atomic_schar;
typedef _Atomic(unsigned char) atomic_uchar;
typedef _Atomic(short) atomic_short;
typedef _Atomic(unsigned short) atomic_ushort;
typedef _Atomic(int) atomic_int;
typedef _Atomic(unsigned int) atomic_uint;
typedef _Atomic(long) atomic_long;
typedef _Atomic(unsigned long) atomic_ulong;
typedef _Atomic(long long) atomic_llong;
typedef _Atomic(unsigned long long) atomic_ullong;
typedef _Atomic(char16_t) atomic_char16_t;
typedef _Atomic(char32_t) atomic_char32_t;
typedef _Atomic(wchar_t) atomic_wchar_t;
typedef _Atomic(int_least8_t) atomic_int_least8_t;
typedef _Atomic(uint_least8_t) atomic_uint_least8_t;
typedef _Atomic(int_least16_t) atomic_int_least16_t;
typedef _Atomic(uint_least16_t) atomic_uint_least16_t;
typedef _Atomic(int_least32_t) atomic_int_least32_t;
typedef _Atomic(uint_least32_t) atomic_uint_least32_t;
typedef _Atomic(int_least64_t) atomic_int_least64_t;
typedef _Atomic(uint_least64_t) atomic_uint_least64_t;
typedef _Atomic(int_fast8_t) atomic_int_fast8_t;
typedef _Atomic(uint_fast8_t) atomic_uint_fast8_t;
typedef _Atomic(int_fast16_t) atomic_int_fast16_t;
typedef _Atomic(uint_fast16_t) atomic_uint_fast16_t;
typedef _Atomic(int_fast32_t) atomic_int_fast32_t;
typedef _Atomic(uint_fast32_t) atomic_uint_fast32_t;
typedef _Atomic(int_fast64_t) atomic_int_fast64_t;
typedef _Atomic(uint_fast64_t) atomic_uint_fast64_t;
typedef _Atomic(intptr_t) atomic_intptr_t;
typedef _Atomic(uintptr_t) atomic_uintptr_t;
typedef _Atomic(size_t) atomic_size_t;
typedef _Atomic(ptrdiff_t) atomic_ptrdiff_t;
typedef _Atomic(intmax_t) atomic_intmax_t;
typedef _Atomic(uintmax_t) atomic_uintmax_t;

#define atomic_init __c11_atomic_init

#define kill_dependency(_Obj) (_Obj)

#define atomic_thread_fence(_Order) _Atomic_thread_fence(_Order)
#define atomic_signal_fence(_Order) _Atomic_signal_fence(_Order)

#define atomic_is_lock_free(_Obj) _Atomic_is_lock_free(sizeof(__typeof_unqual__(*(_Obj))))

#define atomic_store(_Obj, _Desired) __c11_atomic_store(_Obj, _Desired, _Atomic_memory_order_seq_cst)
#define atomic_store_explicit        __c11_atomic_store

#define atomic_load(_Obj)    __c11_atomic_load(_Obj, _Atomic_memory_order_seq_cst)
#define atomic_load_explicit __c11_atomic_load

#define atomic_exchange(_Obj, _Desired) __c11_atomic_exchange(_Obj, _Desired, _Atomic_memory_order_seq_cst)
#define atomic_exchange_explicit        __c11_atomic_exchange

#define atomic_compare_exchange_strong(_Obj, _Expected, _Desired) \
    __c11_atomic_compare_exchange_strong(                         \
        _Obj, _Expected, _Desired, _Atomic_memory_order_seq_cst, _Atomic_memory_order_seq_cst)
#define atomic_compare_exchange_strong_explicit __c11_atomic_compare_exchange_strong

#define atomic_compare_exchange_weak(_Obj, _Expected, _Desired) \
    __c11_atomic_compare_exchange_weak(                         \
        _Obj, _Expected, _Desired, _Atomic_memory_order_seq_cst, _Atomic_memory_order_seq_cst)
#define atomic_compare_exchange_weak_explicit __c11_atomic_compare_exchange_weak

#define atomic_fetch_add(_Obj, _Val) __c11_atomic_fetch_add(_Obj, _Val, _Atomic_memory_order_seq_cst)
#define atomic_fetch_add_explicit    __c11_atomic_fetch_add

#define atomic_fetch_sub(_Obj, _Val) __c11_atomic_fetch_sub(_Obj, _Val, _Atomic_memory_order_seq_cst)
#define atomic_fetch_sub_explicit    __c11_atomic_fetch_sub

#define atomic_fetch_or(_Obj, _Val) __c11_atomic_fetch_or(_Obj, _Val, _Atomic_memory_order_seq_cst)
#define atomic_fetch_or_explicit    __c11_atomic_fetch_or

#define atomic_fetch_xor(_Obj, _Val) __c11_atomic_fetch_xor(_Obj, _Val, _Atomic_memory_order_seq_cst)
#define atomic_fetch_xor_explicit    __c11_atomic_fetch_xor

#define atomic_fetch_and(_Obj, _Val) __c11_atomic_fetch_and(_Obj, _Val, _Atomic_memory_order_seq_cst)
#define atomic_fetch_and_explicit    __c11_atomic_fetch_and

#define ATOMIC_FLAG_INIT \
    { 0 }

typedef struct atomic_flag {
    _Atomic(_Bool) _Val;
} atomic_flag;

#define atomic_flag_test_and_set(_Obj)                  __c11_atomic_exchange(&(_Obj)->_Val, 1, _Atomic_memory_order_seq_cst)
#define atomic_flag_test_and_set_explicit(_Obj, _Order) __c11_atomic_exchange(&(_Obj)->_Val, 1, _Order)

#define atomic_flag_clear(_Obj)                  __c11_atomic_store(&(_Obj)->_Val, 0, _Atomic_memory_order_seq_cst)
#define atomic_flag_clear_explicit(_Obj, _Order) __c11_atomic_store(&(_Obj)->_Val, 0, _Order)
