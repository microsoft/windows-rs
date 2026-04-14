// ymath.h internal header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef _YMATH
#define _YMATH
#include <yvals.h>
#if _STL_COMPILER_PREPROCESSOR
#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

_EXTERN_C_UNLESS_PURE

// macros for _Dtest return (0 => ZERO)
#define _INFCODE 1
#define _NANCODE 2

_CRTIMP2_PURE double __CLRCALL_PURE_OR_CDECL _Cosh(double, double) noexcept;
_CRTIMP2_PURE double __CLRCALL_PURE_OR_CDECL _Sinh(double, double) noexcept;
_CRTIMP2_PURE short __CLRCALL_PURE_OR_CDECL _Exp(double*, double, short) noexcept;

_CRTIMP2_PURE float __CLRCALL_PURE_OR_CDECL _FCosh(float, float) noexcept;
_CRTIMP2_PURE float __CLRCALL_PURE_OR_CDECL _FSinh(float, float) noexcept;
_CRTIMP2_PURE short __CLRCALL_PURE_OR_CDECL _FExp(float*, float, short) noexcept;

_CRTIMP2_PURE long double __CLRCALL_PURE_OR_CDECL _LCosh(long double, long double) noexcept;
_CRTIMP2_PURE short __CLRCALL_PURE_OR_CDECL _LDtest(long double*) noexcept;
_CRTIMP2_PURE long double __CLRCALL_PURE_OR_CDECL _LSinh(long double, long double) noexcept;
_CRTIMP2_PURE short __CLRCALL_PURE_OR_CDECL _LExp(long double*, long double, short) noexcept;

_END_EXTERN_C_UNLESS_PURE

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // _STL_COMPILER_PREPROCESSOR
#endif // _YMATH
