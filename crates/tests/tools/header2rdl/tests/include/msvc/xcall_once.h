// xcall_once.h internal header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef _XCALL_ONCE_H
#define _XCALL_ONCE_H
#include <yvals.h>
#if _STL_COMPILER_PREPROCESSOR

#include <cstdlib>
#include <type_traits>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

_STD_BEGIN
_EXPORT_STD struct once_flag { // opaque data structure for call_once()
    constexpr once_flag() noexcept : _Opaque(nullptr) {}

    once_flag(const once_flag&)            = delete;
    once_flag& operator=(const once_flag&) = delete;

    void* _Opaque;
};

template <class _Ty>
union _Immortalizer_impl { // constructs _Ty, never destroys
    constexpr _Immortalizer_impl() noexcept : _Storage{} {}
    _Immortalizer_impl(const _Immortalizer_impl&)            = delete;
    _Immortalizer_impl& operator=(const _Immortalizer_impl&) = delete;
    ~_Immortalizer_impl() {
        // do nothing
    }

    _Ty _Storage;
};

#if defined(_M_CEE) || defined(_M_ARM64EC) || defined(_M_HYBRID) \
    || defined(__clang__) // TRANSITION, avoid /ALTERNATENAME for Clang, see GH-5224
#define _WINDOWS_API              __stdcall
#define _RENAME_WINDOWS_API(_Api) _Api##_clr
#else // ^^^ use forwarders / use /ALTERNATENAME vvv
#define _WINDOWS_API              __declspec(dllimport) __stdcall
#define _RENAME_WINDOWS_API(_Api) _Api
#endif // ^^^ use /ALTERNATENAME ^^^

// WINBASEAPI
// BOOL
// WINAPI
// InitOnceBeginInitialize(
//     _Inout_ LPINIT_ONCE lpInitOnce,
//     _In_ DWORD dwFlags,
//     _Out_ PBOOL fPending,
//     _Outptr_opt_result_maybenull_ LPVOID* lpContext
//     );
extern "C" _NODISCARD int _WINDOWS_API _RENAME_WINDOWS_API(__std_init_once_begin_initialize)(
    void** _LpInitOnce, unsigned long _DwFlags, int* _FPending, void** _LpContext) noexcept;

// WINBASEAPI
// BOOL
// WINAPI
// InitOnceComplete(
//     _Inout_ LPINIT_ONCE lpInitOnce,
//     _In_ DWORD dwFlags,
//     _In_opt_ LPVOID lpContext
//     );
extern "C" _NODISCARD int _WINDOWS_API _RENAME_WINDOWS_API(__std_init_once_complete)(
    void** _LpInitOnce, unsigned long _DwFlags, void* _LpContext) noexcept;

extern "C" [[noreturn]] void __stdcall __std_init_once_link_alternate_names_and_abort() noexcept;

// #define RTL_RUN_ONCE_INIT_FAILED    0x00000004UL
// #define INIT_ONCE_INIT_FAILED       RTL_RUN_ONCE_INIT_FAILED
_INLINE_VAR constexpr unsigned long _Init_once_init_failed = 0x4UL;

struct _Init_once_completer {
    once_flag& _Once;
    unsigned long _DwFlags;
    ~_Init_once_completer() {
        if (!_RENAME_WINDOWS_API(__std_init_once_complete)(&_Once._Opaque, _DwFlags, nullptr)) {
            __std_init_once_link_alternate_names_and_abort();
        }
    }
};

_EXPORT_STD template <class _Fn, class... _Args>
void(call_once)(once_flag& _Once, _Fn&& _Fx, _Args&&... _Ax)
    noexcept(noexcept(_STD invoke(_STD forward<_Fn>(_Fx), _STD forward<_Args>(_Ax)...))) /* strengthened */ {
    // call _Fx(_Ax...) once
    // parentheses against common "#define call_once(flag,func) pthread_once(flag,func)"
    int _Pending;
    if (!_RENAME_WINDOWS_API(__std_init_once_begin_initialize)(&_Once._Opaque, 0, &_Pending, nullptr)) {
        _CSTD abort();
    }

    if (_Pending != 0) {
        _Init_once_completer _Op{_Once, _Init_once_init_failed};
        _STD invoke(_STD forward<_Fn>(_Fx), _STD forward<_Args>(_Ax)...);
        _Op._DwFlags = 0;
    }
}

#undef _WINDOWS_API
#undef _RENAME_WINDOWS_API
_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // _STL_COMPILER_PREPROCESSOR
#endif // _XCALL_ONCE_H
