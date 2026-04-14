// xthreads.h internal header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef _THR_XTHREADS_H
#define _THR_XTHREADS_H
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR
#include <__msvc_threads_core.hpp>
#include <climits>
#include <xtimec.h>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

extern "C" {
// threads
_CRTIMP2_PURE _Thrd_result __cdecl _Thrd_detach(_Thrd_t) noexcept;
_CRTIMP2_PURE _Thrd_result __cdecl _Thrd_join(_Thrd_t, int*) noexcept;
_CRTIMP2_PURE void __cdecl _Thrd_yield() noexcept;
_CRTIMP2_PURE unsigned int __cdecl _Thrd_hardware_concurrency() noexcept;
_CRTIMP2_PURE _Thrd_id_t __cdecl _Thrd_id() noexcept;
void __stdcall _Thrd_sleep_for(unsigned long /*ms*/) noexcept;

// mutexes
enum { // mutex types
    _Mtx_plain     = 0x01,
    _Mtx_try       = 0x02,
    _Mtx_timed     = 0x04,
    _Mtx_recursive = 0x100
};

#if defined(_CRTBLD) || defined(_DISABLE_CONSTEXPR_MUTEX_CONSTRUCTOR)
_CRTIMP2_PURE void __cdecl _Mtx_init_in_situ(_Mtx_t, int) noexcept;
#endif // ^^^ defined(_CRTBLD) || defined(_DISABLE_CONSTEXPR_MUTEX_CONSTRUCTOR) ^^^
_CRTIMP2_PURE int __cdecl _Mtx_current_owns(_Mtx_t) noexcept;
_CRTIMP2_PURE _Thrd_result __cdecl _Mtx_lock(_Mtx_t) noexcept;
_CRTIMP2_PURE _Thrd_result __cdecl _Mtx_trylock(_Mtx_t) noexcept;
_CRTIMP2_PURE _Thrd_result __cdecl _Mtx_unlock(_Mtx_t) noexcept; // TRANSITION, ABI: Always succeeds

// shared mutex
// these declarations must be in sync with those in sharedmutex.cpp
void __cdecl _Smtx_lock_exclusive(_Smtx_t*) noexcept;
void __cdecl _Smtx_lock_shared(_Smtx_t*) noexcept;
int __cdecl _Smtx_try_lock_exclusive(_Smtx_t*) noexcept;
int __cdecl _Smtx_try_lock_shared(_Smtx_t*) noexcept;
void __cdecl _Smtx_unlock_exclusive(_Smtx_t*) noexcept;
void __cdecl _Smtx_unlock_shared(_Smtx_t*) noexcept;

// condition variables
#ifdef _DISABLE_CONSTEXPR_MUTEX_CONSTRUCTOR
_CRTIMP2_PURE void __cdecl _Cnd_init_in_situ(_Cnd_t) noexcept;
#endif // ^^^ defined(_DISABLE_CONSTEXPR_MUTEX_CONSTRUCTOR) ^^^
_CRTIMP2_PURE _Thrd_result __cdecl _Cnd_wait(_Cnd_t, _Mtx_t) noexcept; // TRANSITION, ABI: Always succeeds
_CRTIMP2_PURE _Thrd_result __cdecl _Cnd_broadcast(_Cnd_t) noexcept; // TRANSITION, ABI: Always succeeds
_CRTIMP2_PURE _Thrd_result __cdecl _Cnd_signal(_Cnd_t) noexcept; // TRANSITION, ABI: Always succeeds
_CRTIMP2_PURE void __cdecl _Cnd_register_at_thread_exit(_Cnd_t, _Mtx_t, int*) noexcept;
_CRTIMP2_PURE void __cdecl _Cnd_unregister_at_thread_exit(_Mtx_t) noexcept;
_CRTIMP2_PURE void __cdecl _Cnd_do_broadcast_at_thread_exit() noexcept;
// '_unchecked' means it is not checked against the 'steady_clock', so may report timeout prematurely
_Thrd_result __stdcall _Cnd_timedwait_for_unchecked(_Cnd_t, _Mtx_t, unsigned int) noexcept;
} // extern "C"

_STD_BEGIN
enum { // constants for error codes
    _DEVICE_OR_RESOURCE_BUSY,
    _INVALID_ARGUMENT,
    _NO_SUCH_PROCESS,
    _NOT_ENOUGH_MEMORY,
    _OPERATION_NOT_PERMITTED,
    _RESOURCE_DEADLOCK_WOULD_OCCUR,
    _RESOURCE_UNAVAILABLE_TRY_AGAIN
};

extern "C++" [[noreturn]] _CRTIMP2_PURE void __cdecl _Throw_Cpp_error(int _Code);
_STD_END
#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // _STL_COMPILER_PREPROCESSOR
#endif // _THR_XTHREADS_H

/*
 * This file is derived from software bearing the following
 * restrictions:
 *
 * (c) Copyright William E. Kempf 2001
 *
 * Permission to use, copy, modify, distribute and sell this
 * software and its documentation for any purpose is hereby
 * granted without fee, provided that the above copyright
 * notice appear in all copies and that both that copyright
 * notice and this permission notice appear in supporting
 * documentation. William E. Kempf makes no representations
 * about the suitability of this software for any purpose.
 * It is provided "as is" without express or implied warranty.
 */
