// xcharconv.h internal header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef _XCHARCONV_H
#define _XCHARCONV_H
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR

#if !_HAS_CXX17
#error The contents of <charconv> are only available with C++17. (Also, you should not include this internal header.)
#endif // !_HAS_CXX17

#include <cstdint>
#include <type_traits>
#include <xerrc.h>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

_STD_BEGIN
_EXPORT_STD enum class chars_format {
    scientific = 0b001,
    fixed      = 0b010,
    hex        = 0b100,
    general    = fixed | scientific,
};

_BITMASK_OPS(_EXPORT_STD, chars_format)

_EXPORT_STD struct to_chars_result {
    char* ptr;
    errc ec;
#if _HAS_CXX20
    _NODISCARD friend bool operator==(const to_chars_result&, const to_chars_result&) = default;
#endif // _HAS_CXX20
};

_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)

#endif // _STL_COMPILER_PREPROCESSOR
#endif // _XCHARCONV_H
