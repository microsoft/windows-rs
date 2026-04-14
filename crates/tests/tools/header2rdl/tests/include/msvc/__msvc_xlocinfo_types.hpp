// __msvc_xlocinfo_types.hpp internal header (core)

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef __MSVC_XLOCINFO_TYPES_HPP
#define __MSVC_XLOCINFO_TYPES_HPP
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

_EXTERN_C_UNLESS_PURE

struct _Collvec { // stuff needed by _Strcoll, etc.
    unsigned int _Page; // UINT
    wchar_t* _LocaleName;
};

struct _Ctypevec { // stuff needed by _Tolower, etc.
    unsigned int _Page; // UINT
    const short* _Table;
    int _Delfl;
    wchar_t* _LocaleName;
};

struct _Cvtvec { // stuff needed by _Mbrtowc, etc.
    unsigned int _Page; // UINT
    unsigned int _Mbcurmax;
    int _Isclocale; // LCID == _CLOCALEHANDLE
    unsigned char _Isleadbyte[32]; // 256 bits
};

_END_EXTERN_C_UNLESS_PURE

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // _STL_COMPILER_PREPROCESSOR
#endif // __MSVC_XLOCINFO_TYPES_HPP
