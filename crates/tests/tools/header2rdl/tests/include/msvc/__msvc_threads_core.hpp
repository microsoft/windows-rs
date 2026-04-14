// __msvc_threads_core.hpp internal header (core)

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef __MSVC_THREADS_CORE_HPP
#define __MSVC_THREADS_CORE_HPP
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR
#include <type_traits>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

extern "C" {
using _Thrd_id_t = unsigned int;
struct _Thrd_t { // thread identifier for Win32
    void* _Hnd; // Win32 HANDLE
    _Thrd_id_t _Id;
};

using _Smtx_t = void*;

enum class _Thrd_result : int { _Success, _Nomem, _Timedout, _Busy, _Error };

struct _Stl_critical_section {
    void* _Unused       = nullptr; // TRANSITION, ABI: was the vptr
    _Smtx_t _M_srw_lock = nullptr;
};

struct _Mtx_internal_imp_t {
// TRANSITION, ABI: We should directly store _M_srw_lock above.
#ifdef _WIN64
    static constexpr size_t _Critical_section_size = 64;
#else // ^^^ 64-bit / 32-bit vvv
    static constexpr size_t _Critical_section_size = 36;
#endif // ^^^ 32-bit ^^^

    int _Type{};
    union {
        _Stl_critical_section _Critical_section{};
        _STD _Aligned_storage_t<_Critical_section_size, alignof(void*)> _Cs_storage;
    };
    long _Thread_id{};
    int _Count{};
};

using _Mtx_t = _Mtx_internal_imp_t*;

struct _Stl_condition_variable {
    void* _Unused = nullptr; // TRANSITION, ABI: was the vptr
    void* _Win_cv = nullptr;
};

#pragma warning(push)
#pragma warning(disable : 26495) // Variable 'meow' is uninitialized. Always initialize a member variable (type.6).
struct _Cnd_internal_imp_t {
// TRANSITION, ABI: We should directly store _Win_cv above.
#ifdef _WIN64
    static constexpr size_t _Cnd_internal_imp_size = 72;
#else // ^^^ 64-bit / 32-bit vvv
    static constexpr size_t _Cnd_internal_imp_size = 40;
#endif // ^^^ 32-bit ^^^

    union {
        _Stl_condition_variable _Stl_cv{};
        _STD _Aligned_storage_t<_Cnd_internal_imp_size, alignof(void*)> _Cv_storage;
    };
};
#pragma warning(pop)

using _Cnd_t = _Cnd_internal_imp_t*;
} // extern "C"

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // _STL_COMPILER_PREPROCESSOR
#endif // __MSVC_THREADS_CORE_HPP

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
