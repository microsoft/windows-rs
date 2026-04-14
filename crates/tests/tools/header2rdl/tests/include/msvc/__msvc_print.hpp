// __msvc_print.hpp internal header (core)

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef __MSVC_PRINT_HPP
#define __MSVC_PRINT_HPP
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR

#if !_HAS_CXX20 // note: <format> includes this header in C++20 mode
#error The contents of <print> are available only with C++23. (Also, you should not include this internal header.)
#endif // ^^^ !_HAS_CXX20 ^^^

#include <cstdio>
#include <xfilesystem_abi.h>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

extern "C" {

enum class __std_unicode_console_handle : intptr_t { _Invalid = -1 };

struct __std_unicode_console_retrieval_result {
    __std_unicode_console_handle _Console_handle;

    // For this, we have a few potential return values:
    //
    //   - __std_win_error::_Success: The operation completed successfully. This is the only value for which the
    //     _Console_handle field has a well-defined value.
    //
    //   - __std_win_error::_File_not_found: The FILE* provided is valid, but it is determined to not be associated
    //     with a unicode console. In this case, printing should fall back to vprint_nonunicode().
    //
    //   - __std_win_error::_Not_supported: The FILE* provided does not actually have an associated output stream. In
    //     this case, the entire print can safely be elided, thanks to the "as-if" rule.
    //     (We haven't observed this happening in practice. Console applications with stdout redirected to NUL
    //     and Windows applications both appear to activate the __std_win_error::_File_not_found "valid, but
    //     not a unicode console" codepath.)
    //
    //   - __std_win_error::_Invalid_parameter: The FILE* provided is invalid. A std::system_error exception should be
    //     thrown if this value is returned within the FILE* overload of vprint_unicode().
    __std_win_error _Error;
};

_NODISCARD _Success_(return._Error == __std_win_error::_Success) __std_unicode_console_retrieval_result
    __stdcall __std_get_unicode_console_handle_from_file_stream(_In_ FILE* _Stream) noexcept;

_NODISCARD _Success_(return == __std_win_error::_Success) __std_win_error
    __stdcall __std_print_to_unicode_console(_In_ __std_unicode_console_handle _Console_handle,
        _In_reads_(_Str_size) const char* _Str, _In_ size_t _Str_size) noexcept;

_NODISCARD _Success_(return == __std_win_error::_Success) __std_win_error
    __stdcall __std_print_newline_only_to_unicode_console(_In_ __std_unicode_console_handle _Console_handle) noexcept;

} // extern "C"

_STD_BEGIN

_NODISCARD consteval bool _Is_ordinary_literal_encoding_utf8() {
    // See: https://learn.microsoft.com/en-us/windows/win32/intl/code-page-identifiers
#if defined(_MSVC_EXECUTION_CHARACTER_SET) && _MSVC_EXECUTION_CHARACTER_SET == 65001 // Unicode (UTF-8) == 65001
    return true;
#else
    return false;
#endif
}

_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // _STL_COMPILER_PREPROCESSOR
#endif // __MSVC_PRINT_HPP
