/******************************************************************
*                                                                 *
*  intsafe.h -- This module defines helper functions to prevent   *
*               integer overflow bugs.                            *
*                                                                 *
*  Copyright (c) Microsoft Corp.  All rights reserved.            *
*                                                                 *
******************************************************************/
#ifndef _INTSAFE_H_INCLUDED_
#define _INTSAFE_H_INCLUDED_

#include <winapifamily.h>


#if (_MSC_VER > 1000)
#pragma once
#endif

#pragma region Application Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#include <specstrings.h>    // for _In_, etc.

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4668) // #if not_defined treated as #if 0
#endif

#if !defined(_W64)
#if !defined(__midl) && (defined(_X86_) || defined(_M_IX86) || defined(_ARM_) || defined(_M_ARM)) && (_MSC_VER >= 1300)
#define _W64 __w64
#else
#define _W64
#endif
#endif

//
// typedefs
//
typedef char                CHAR;
typedef signed char         INT8;
typedef unsigned char       UCHAR;
typedef unsigned char       UINT8;
typedef unsigned char       BYTE;
typedef short               SHORT;
typedef signed short        INT16;
typedef unsigned short      USHORT;
typedef unsigned short      UINT16;
typedef unsigned short      WORD;
typedef int                 INT;
typedef signed int          INT32;
typedef unsigned int        UINT;
typedef unsigned int        UINT32;
typedef long                LONG;
typedef unsigned long       ULONG;
typedef unsigned long       DWORD;
typedef __int64             LONGLONG;
typedef __int64             LONG64;
typedef signed __int64      INT64;
typedef unsigned __int64    ULONGLONG;
typedef unsigned __int64    DWORDLONG;
typedef unsigned __int64    ULONG64;
typedef unsigned __int64    DWORD64;
typedef unsigned __int64    UINT64;

#if (__midl > 501)
typedef [public]          __int3264 INT_PTR;
typedef [public] unsigned __int3264 UINT_PTR;
typedef [public]          __int3264 LONG_PTR;
typedef [public] unsigned __int3264 ULONG_PTR;
#else
#ifdef _WIN64
typedef __int64             INT_PTR;
typedef unsigned __int64    UINT_PTR;
typedef __int64             LONG_PTR;
typedef unsigned __int64    ULONG_PTR;
#else
typedef _W64 int            INT_PTR;
typedef _W64 unsigned int   UINT_PTR;
typedef _W64 long           LONG_PTR;
typedef _W64 unsigned long  ULONG_PTR;
#endif // WIN64
#endif // (__midl > 501)

#ifdef _WIN64
typedef __int64             ptrdiff_t;
typedef unsigned __int64    size_t;
#else
typedef _W64 int            ptrdiff_t;
typedef _W64 unsigned int   size_t;
#endif

typedef ULONG_PTR   DWORD_PTR;
typedef LONG_PTR    SSIZE_T;
typedef ULONG_PTR   SIZE_T;

#undef _USE_INTRINSIC_MULTIPLY128

#if !defined(_ARM64_MULT_INTRINS_SUPPORTED)
#define _ARM64_MULT_INTRINS_SUPPORTED 0
#if (defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64))
#if defined(__clang__)
#if __has_builtin(__umulh) && __has_builtin(__mulh)
#undef _ARM64_MULT_INTRINS_SUPPORTED
#define _ARM64_MULT_INTRINS_SUPPORTED 1
#endif // __has_builtin(__umulh) && __has_builtin(__mulh)
#else // defined(__clang__)
#undef _ARM64_MULT_INTRINS_SUPPORTED
#define _ARM64_MULT_INTRINS_SUPPORTED 1
#endif // defined(__clang__)
#endif // (defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64))
#endif // !defined(_ARM64_MULT_INTRINS_SUPPORTED)

#if !defined(_M_CEE) && \
    ((defined(_M_X64) && !defined(_M_ARM64EC)) || \
     (defined(_ARM64_MULT_INTRINS_SUPPORTED) && _ARM64_MULT_INTRINS_SUPPORTED) || \
     (defined(_M_IA64) && (_MSC_VER >= 1400)))
#define _USE_INTRINSIC_MULTIPLY128
#endif

#if defined(_USE_INTRINSIC_MULTIPLY128)
#ifdef __cplusplus
extern "C" {
#endif

#if defined(_M_X64) && !defined(_M_ARM64EC)

#define __UnsignedMultiply128 _umul128

#endif // defined(_M_X64) && !defined(_M_ARM64EC)

ULONG64
__UnsignedMultiply128(
    _In_ ULONG64 Multiplicand,
    _In_ ULONG64 Multiplier,
    _Out_ _Deref_out_range_(==, Multiplicand * Multiplier) ULONG64* HighProduct);

#if defined(_M_X64) && !defined(_M_ARM64EC)
#pragma intrinsic(_umul128)
#endif

#if defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)

ULONG64
__umulh(
    _In_ ULONG64 Multiplicand,
    _In_ ULONG64 Multiplier
    );

#pragma intrinsic(__umulh)

__inline
ULONG64
__UnsignedMultiply128(
    _In_ ULONG64 Multiplicand,
    _In_ ULONG64 Multiplier,
    _Out_ _Deref_out_range_(==, Multiplicand * Multiplier) ULONG64* HighProduct)
{
    *HighProduct = __umulh(Multiplier, Multiplicand);
    return Multiplier * Multiplicand;
}

#endif // defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)

#ifdef __cplusplus
}
#endif
#endif // _USE_INTRINSIC_MULTIPLY128



typedef _Return_type_success_(return >= 0) long HRESULT;

#define SUCCEEDED(hr)   (((HRESULT)(hr)) >= 0)
#define FAILED(hr)      (((HRESULT)(hr)) < 0)

#define S_OK    ((HRESULT)0L)

#define INTSAFE_E_ARITHMETIC_OVERFLOW   ((HRESULT)0x80070216L)  // 0x216 = 534 = ERROR_ARITHMETIC_OVERFLOW
#ifndef SORTPP_PASS 
// compiletime asserts (failure results in error C2118: negative subscript)
#define C_ASSERT(e) typedef char __C_ASSERT__[(e)?1:-1]
#else
#define C_ASSERT(e)
#endif

//
// UInt32x32To64 macro
//
#ifndef UInt32x32To64
#if defined(MIDL_PASS) || defined(RC_INVOKED) || defined(_M_CEE_PURE) \
    || defined(_68K_) || defined(_MPPC_) \
    || defined(_M_IA64) || defined(_M_X64) || defined(_M_ARM) || defined(_M_ARM64) \
    || defined(_M_HYBRID_X86_ARM64)
#define UInt32x32To64(a, b) (((unsigned __int64)((unsigned int)(a))) * ((unsigned __int64)((unsigned int)(b))))
#elif defined(_M_IX86)
#define UInt32x32To64(a, b) ((unsigned __int64)(((unsigned __int64)((unsigned int)(a))) * ((unsigned int)(b))))
#else
#error Must define a target architecture.
#endif
#endif

//
// Min/Max type values
//
#define INT8_MIN        (-127i8 - 1)
#define SHORT_MIN       (-32768)
#define INT16_MIN       (-32767i16 - 1)
#ifndef INT_MIN
#define INT_MIN         (-2147483647 - 1)
#endif
#define INT32_MIN       (-2147483647i32 - 1)
#ifndef LONG_MIN
#define LONG_MIN        (-2147483647L - 1)
#endif
#define LONGLONG_MIN    (-9223372036854775807i64 - 1)
#define LONG64_MIN      (-9223372036854775807i64 - 1)
#define INT64_MIN       (-9223372036854775807i64 - 1)
#define INT128_MIN      (-170141183460469231731687303715884105727i128 - 1)

#ifdef _WIN64
#define INT_PTR_MIN     (-9223372036854775807i64 - 1)
#define LONG_PTR_MIN    (-9223372036854775807i64 - 1)
#define PTRDIFF_T_MIN   (-9223372036854775807i64 - 1)
#define SSIZE_T_MIN     (-9223372036854775807i64 - 1)
#else
#define INT_PTR_MIN     (-2147483647 - 1)
#define LONG_PTR_MIN    (-2147483647L - 1)
#define PTRDIFF_T_MIN   (-2147483647 - 1)
#define SSIZE_T_MIN     (-2147483647L - 1)
#endif

#define INT8_MAX        127i8
#define UINT8_MAX       0xffui8
#define BYTE_MAX        0xff
#define SHORT_MAX       32767
#define INT16_MAX       32767i16
#define USHORT_MAX      0xffff
#define UINT16_MAX      0xffffui16
#define WORD_MAX        0xffff
#ifndef INT_MAX
#define INT_MAX         2147483647
#endif
#define INT32_MAX       2147483647i32
#ifndef UINT_MAX
#define UINT_MAX        0xffffffff
#endif
#define UINT32_MAX      0xffffffffui32
#ifndef LONG_MAX
#define LONG_MAX        2147483647L
#endif
#ifndef ULONG_MAX
#define ULONG_MAX       0xffffffffUL
#endif
#define DWORD_MAX       0xffffffffUL
#define LONGLONG_MAX    9223372036854775807i64
#define LONG64_MAX      9223372036854775807i64
#define INT64_MAX       9223372036854775807i64
#define ULONGLONG_MAX   0xffffffffffffffffui64
#define DWORDLONG_MAX   0xffffffffffffffffui64
#define ULONG64_MAX     0xffffffffffffffffui64
#define DWORD64_MAX     0xffffffffffffffffui64
#define UINT64_MAX      0xffffffffffffffffui64
#define INT128_MAX      170141183460469231731687303715884105727i128
#define UINT128_MAX     0xffffffffffffffffffffffffffffffffui128

#undef SIZE_T_MAX

#ifdef _WIN64
#define INT_PTR_MAX     9223372036854775807i64
#define UINT_PTR_MAX    0xffffffffffffffffui64
#define LONG_PTR_MAX    9223372036854775807i64
#define ULONG_PTR_MAX   0xffffffffffffffffui64
#define DWORD_PTR_MAX   0xffffffffffffffffui64
#define PTRDIFF_T_MAX   9223372036854775807i64
#define SIZE_T_MAX      0xffffffffffffffffui64
#define SSIZE_T_MAX     9223372036854775807i64
#define _SIZE_T_MAX     0xffffffffffffffffui64
#else
#define INT_PTR_MAX     2147483647 
#define UINT_PTR_MAX    0xffffffff
#define LONG_PTR_MAX    2147483647L
#define ULONG_PTR_MAX   0xffffffffUL
#define DWORD_PTR_MAX   0xffffffffUL
#define PTRDIFF_T_MAX   2147483647
#define SIZE_T_MAX      0xffffffff
#define SSIZE_T_MAX     2147483647L
#define _SIZE_T_MAX     0xffffffffUL
#endif


//
// It is common for -1 to be used as an error value
//
#define INT8_ERROR      (-1i8)
#define UINT8_ERROR     0xffui8
#define BYTE_ERROR      0xff
#define SHORT_ERROR     (-1)
#define INT16_ERROR     (-1i16)
#define USHORT_ERROR    0xffff
#define UINT16_ERROR    0xffffui16
#define WORD_ERROR      0xffff
#define INT_ERROR       (-1)
#define INT32_ERROR     (-1i32)
#define UINT_ERROR      0xffffffff
#define UINT32_ERROR    0xffffffffui32
#define LONG_ERROR      (-1L)
#define ULONG_ERROR     0xffffffffUL
#define DWORD_ERROR     0xffffffffUL
#define LONGLONG_ERROR  (-1i64)
#define LONG64_ERROR    (-1i64)
#define INT64_ERROR     (-1i64)
#define ULONGLONG_ERROR 0xffffffffffffffffui64
#define DWORDLONG_ERROR 0xffffffffffffffffui64
#define ULONG64_ERROR   0xffffffffffffffffui64
#define UINT64_ERROR    0xffffffffffffffffui64

#ifdef _WIN64
#define INT_PTR_ERROR   (-1i64)
#define UINT_PTR_ERROR  0xffffffffffffffffui64
#define LONG_PTR_ERROR  (-1i64)
#define ULONG_PTR_ERROR 0xffffffffffffffffui64
#define DWORD_PTR_ERROR 0xffffffffffffffffui64
#define PTRDIFF_T_ERROR (-1i64)
#define SIZE_T_ERROR    0xffffffffffffffffui64
#define SSIZE_T_ERROR   (-1i64)
#define _SIZE_T_ERROR   0xffffffffffffffffui64
#else
#define INT_PTR_ERROR   (-1) 
#define UINT_PTR_ERROR  0xffffffff
#define LONG_PTR_ERROR  (-1L)
#define ULONG_PTR_ERROR 0xffffffffUL
#define DWORD_PTR_ERROR 0xffffffffUL
#define PTRDIFF_T_ERROR (-1)
#define SIZE_T_ERROR    0xffffffff
#define SSIZE_T_ERROR   (-1L)
#define _SIZE_T_ERROR   0xffffffffUL
#endif


//
// We make some assumptions about the sizes of various types. Let's be
// explicit about those assumptions and check them.
//
C_ASSERT(sizeof(USHORT) == 2);
C_ASSERT(sizeof(INT) == 4);
C_ASSERT(sizeof(UINT) == 4);
C_ASSERT(sizeof(LONG) == 4);
C_ASSERT(sizeof(ULONG) == 4);
C_ASSERT(sizeof(UINT_PTR) == sizeof(ULONG_PTR));


//=============================================================================
// Conversion functions
//
// There are three reasons for having conversion functions:
//
// 1. We are converting from a signed type to an unsigned type of the same
//    size, or vice-versa.
//
//    Since we default to only having unsigned math functions,
//    (see ENABLE_INTSAFE_SIGNED_FUNCTIONS below) we prefer people to convert
//    to unsigned, do the math, and then convert back to signed.
//
// 2. We are converting to a smaller type, and we could therefore possibly
//    overflow.
//
// 3. We are converting to a bigger type, and we are signed and the type we are
//    converting to is unsigned.
//
//=============================================================================


//
// INT8 -> UCHAR conversion
//
_Must_inspect_result_
__inline
HRESULT
Int8ToUChar(
    _In_ INT8 i8Operand,
    _Out_ _Deref_out_range_(==, i8Operand) UCHAR* pch)
{
    HRESULT hr;

    if (i8Operand >= 0)
    {
        *pch = (UCHAR)i8Operand;
        hr = S_OK;
    }
    else
    {
        *pch = '\0';
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// INT8 -> UINT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
Int8ToUInt8(
    _In_ INT8 i8Operand,
    _Out_ _Deref_out_range_(==, i8Operand) UINT8* pu8Result)
{
    HRESULT hr;
    
    if (i8Operand >= 0)
    {
        *pu8Result = (UINT8)i8Operand;
        hr = S_OK;
    }
    else
    {
        *pu8Result = UINT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// INT8 -> BYTE conversion
//
#define Int8ToByte  Int8ToUInt8

//
// INT8 -> USHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
Int8ToUShort(
    _In_ INT8 i8Operand,
    _Out_ _Deref_out_range_(==, i8Operand) USHORT* pusResult)
{
    HRESULT hr;
    
    if (i8Operand >= 0)
    {
        *pusResult = (USHORT)i8Operand;
        hr = S_OK;
    }
    else
    {
        *pusResult = USHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// INT8 -> UINT16 conversion
//
#define Int8ToUInt16    Int8ToUShort

//
// INT8 -> WORD conversion
//
#define Int8ToWord  Int8ToUShort

//
// INT8 -> UINT conversion
//
_Must_inspect_result_
__inline
HRESULT
Int8ToUInt(
    _In_ INT8 i8Operand,
    _Out_ _Deref_out_range_(==, i8Operand) UINT* puResult)
{
    HRESULT hr;
    
    if (i8Operand >= 0)
    {
        *puResult = (UINT)i8Operand;
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// INT8 -> UINT32 conversion
//
#define Int8ToUInt32    Int8ToUInt

//
// INT8 -> UINT_PTR conversion
//
_Must_inspect_result_
__inline
HRESULT
Int8ToUIntPtr(
    _In_ INT8 i8Operand,
    _Out_ _Deref_out_range_(==, i8Operand) UINT_PTR* puResult)
{
    HRESULT hr;
    
    if (i8Operand >= 0)
    {
        *puResult = (UINT_PTR)i8Operand;
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// INT8 -> ULONG conversion
//
_Must_inspect_result_
__inline
HRESULT
Int8ToULong(
    _In_ INT8 i8Operand,
    _Out_ _Deref_out_range_(==, i8Operand) ULONG* pulResult)
{
    HRESULT hr;
    
    if (i8Operand >= 0)
    {
        *pulResult = (ULONG)i8Operand;
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// INT8 -> ULONG_PTR conversion
//
_Must_inspect_result_
__inline
HRESULT
Int8ToULongPtr(
    _In_ INT8 i8Operand,
    _Out_ _Deref_out_range_(==, i8Operand) ULONG_PTR* pulResult)
{
    HRESULT hr;
    
    if (i8Operand >= 0)
    {
        *pulResult = (ULONG_PTR)i8Operand;
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// INT8 -> DWORD conversion
//
#define Int8ToDWord Int8ToULong

//
// INT8 -> DWORD_PTR conversion
//
#define Int8ToDWordPtr  Int8ToULongPtr

//
// INT8 -> ULONGLONG conversion
//
_Must_inspect_result_
__inline
HRESULT
Int8ToULongLong(
    _In_ INT8 i8Operand,
    _Out_ _Deref_out_range_(==, i8Operand) ULONGLONG* pullResult)
{
    HRESULT hr;
    
    if (i8Operand >= 0)
    {
        *pullResult = (ULONGLONG)i8Operand;
        hr = S_OK;
    }
    else
    {
        *pullResult = ULONGLONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// INT8 -> DWORDLONG conversion
//
#define Int8ToDWordLong Int8ToULongLong

//
// INT8 -> ULONG64 conversion
//
#define Int8ToULong64   Int8ToULongLong

//
// INT8 -> DWORD64 conversion
//
#define Int8ToDWord64   Int8ToULongLong

//
// INT8 -> UINT64 conversion
//
#define Int8ToUInt64    Int8ToULongLong

//
// INT8 -> size_t conversion
//
#define Int8ToSizeT Int8ToUIntPtr

//
// INT8 -> SIZE_T conversion
//
#define Int8ToSIZET Int8ToULongPtr

//
// UINT8 -> INT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
UInt8ToInt8(
    _In_ UINT8 u8Operand,
    _Out_ _Deref_out_range_(==, u8Operand) INT8* pi8Result)
{
    HRESULT hr;
    
    if (u8Operand <= INT8_MAX)
    {
        *pi8Result = (INT8)u8Operand;
        hr = S_OK;
    }
    else
    {
        *pi8Result = INT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// UINT8 -> CHAR conversion
//
__forceinline
HRESULT
UInt8ToChar(
    _In_ UINT8 u8Operand,
    _Out_ _Deref_out_range_(==, u8Operand) CHAR* pch)
{
#ifdef _CHAR_UNSIGNED
    *pch = (CHAR)u8Operand;
    return S_OK;
#else
    return UInt8ToInt8(u8Operand, (INT8*)pch);
#endif
}

//
// BYTE -> INT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
ByteToInt8(
    _In_ BYTE bOperand,
    _Out_ _Deref_out_range_(==, bOperand) INT8* pi8Result)
{
    HRESULT hr;
    
    if (bOperand <= INT8_MAX)
    {
        *pi8Result = (INT8)bOperand;
        hr = S_OK;
    }
    else
    {
        *pi8Result = INT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// BYTE -> CHAR conversion
//
__forceinline
HRESULT
ByteToChar(
    _In_ BYTE bOperand,
    _Out_ _Deref_out_range_(==, bOperand) CHAR* pch)
{
#ifdef _CHAR_UNSIGNED
    *pch = (CHAR)bOperand;
    return S_OK;
#else
    return ByteToInt8(bOperand, (INT8*)pch);
#endif
}

//
// SHORT -> INT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
ShortToInt8(
    _In_ SHORT sOperand,
    _Out_ _Deref_out_range_(==, sOperand) INT8* pi8Result)
{
    HRESULT hr;

    if ((sOperand >= INT8_MIN) && (sOperand <= INT8_MAX))
    {
        *pi8Result = (INT8)sOperand;
        hr = S_OK;
    }
    else
    {
        *pi8Result = INT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// SHORT -> UCHAR conversion
//
_Must_inspect_result_
__inline
HRESULT
ShortToUChar(
    _In_ SHORT sOperand,
    _Out_ _Deref_out_range_(==, sOperand) UCHAR* pch)
{
    HRESULT hr;

    if ((sOperand >= 0) && (sOperand <= 255))
    {
        *pch = (UCHAR)sOperand;
        hr = S_OK;
    }
    else
    {
        *pch = '\0';
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// SHORT -> CHAR conversion
//
__forceinline
HRESULT
ShortToChar(
    _In_ SHORT sOperand,
    _Out_ _Deref_out_range_(==, sOperand) CHAR* pch)
{
#ifdef _CHAR_UNSIGNED
    return ShortToUChar(sOperand, (UCHAR*)pch);
#else
    return ShortToInt8(sOperand, (INT8*)pch);
#endif // _CHAR_UNSIGNED
}

//
// SHORT -> UINT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
ShortToUInt8(
    _In_ SHORT sOperand,
    _Out_ _Deref_out_range_(==, sOperand) UINT8* pui8Result)
{
    HRESULT hr;

    if ((sOperand >= 0) && (sOperand <= UINT8_MAX))
    {
        *pui8Result = (UINT8)sOperand;
        hr = S_OK;
    }
    else
    {
        *pui8Result = UINT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// SHORT -> BYTE conversion
//
#define ShortToByte  ShortToUInt8

//
// SHORT -> USHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
ShortToUShort(
    _In_ SHORT sOperand,
    _Out_ _Deref_out_range_(==, sOperand) USHORT* pusResult)
{
    HRESULT hr;

    if (sOperand >= 0)
    {
        *pusResult = (USHORT)sOperand;
        hr = S_OK;
    }
    else
    {
        *pusResult = USHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// SHORT -> UINT16 conversion
//
#define ShortToUInt16   ShortToUShort

//
// SHORT -> WORD conversion
//
#define ShortToWord ShortToUShort

//
// SHORT -> UINT conversion
//
_Must_inspect_result_
__inline
HRESULT
ShortToUInt(
    _In_ SHORT sOperand,
    _Out_ _Deref_out_range_(==, sOperand) UINT* puResult)
{
    HRESULT hr;

    if (sOperand >= 0)
    {
        *puResult = (UINT)sOperand;
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// SHORT -> UINT32 conversion
//
#define ShortToUInt32   ShortToUInt

//
// SHORT -> UINT_PTR conversion
//
_Must_inspect_result_
__inline
HRESULT
ShortToUIntPtr(
    _In_ SHORT sOperand,
    _Out_ _Deref_out_range_(==, sOperand) UINT_PTR* puResult)
{
    HRESULT hr;

    if (sOperand >= 0)
    {
        *puResult = (UINT_PTR)sOperand;
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// SHORT -> ULONG conversion
//
_Must_inspect_result_
__inline
HRESULT
ShortToULong(
    _In_ SHORT sOperand,
    _Out_ _Deref_out_range_(==, sOperand) ULONG* pulResult)
{
    HRESULT hr;
    
    if (sOperand >= 0)
    {
        *pulResult = (ULONG)sOperand;
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// SHORT -> ULONG_PTR conversion
//
_Must_inspect_result_
__inline
HRESULT
ShortToULongPtr(
    _In_ SHORT sOperand,
    _Out_ _Deref_out_range_(==, sOperand) ULONG_PTR* pulResult)
{
    HRESULT hr;
    
    if (sOperand >= 0)
    {
        *pulResult = (ULONG_PTR)sOperand;
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// SHORT -> DWORD conversion
//
#define ShortToDWord    ShortToULong

//
// SHORT -> DWORD_PTR conversion
//
_Must_inspect_result_
__inline
HRESULT
ShortToDWordPtr(
    _In_ SHORT sOperand,
    _Out_ _Deref_out_range_(==, sOperand) DWORD_PTR* pdwResult)
{
    HRESULT hr;
    
    if (sOperand >= 0)
    {
        *pdwResult = (DWORD_PTR)sOperand;
        hr = S_OK;
    }
    else
    {
        *pdwResult = DWORD_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// SHORT -> ULONGLONG conversion
//
_Must_inspect_result_
__inline
HRESULT
ShortToULongLong(
    _In_ SHORT sOperand,
    _Out_ _Deref_out_range_(==, sOperand) ULONGLONG* pullResult)
{
    HRESULT hr;

    if (sOperand >= 0)
    {
        *pullResult = (ULONGLONG)sOperand;
        hr = S_OK;
    }
    else
    {
        *pullResult = ULONGLONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// SHORT -> DWORDLONG conversion
//
#define ShortToDWordLong    ShortToULongLong

//
// SHORT -> ULONG64 conversion
//
#define ShortToULong64  ShortToULongLong

//
// SHORT -> DWORD64 conversion
//
#define ShortToDWord64  ShortToULongLong

//
// SHORT -> UINT64 conversion
//
#define ShortToUInt64   ShortToULongLong

//
// SHORT -> size_t conversion
//
#define ShortToSizeT    ShortToUIntPtr

//
// SHORT -> SIZE_T conversion
//
#define ShortToSIZET    ShortToULongPtr

//
// INT16 -> CHAR conversion
//
#define Int16ToChar ShortToChar

//
// INT16 -> INT8 conversion
//
#define Int16ToInt8 ShortToInt8

//
// INT16 -> UCHAR conversion
//
#define Int16ToUChar    ShortToUChar

//
// INT16 -> UINT8 conversion
//
#define Int16ToUInt8    ShortToUInt8

//
// INT16 -> BYTE conversion
//
#define Int16ToByte ShortToUInt8

//
// INT16 -> USHORT conversion
//
#define Int16ToUShort   ShortToUShort

//
// INT16 -> UINT16 conversion
//
#define Int16ToUInt16   ShortToUShort

//
// INT16 -> WORD conversion
//
#define Int16ToWord ShortToUShort

//
// INT16 -> UINT conversion
//
#define Int16ToUInt ShortToUInt

//
// INT16 -> UINT32 conversion
//
#define Int16ToUInt32   ShortToUInt

//
// INT16 -> UINT_PTR conversion
//
#define Int16ToUIntPtr  ShortToUIntPtr

//
// INT16 -> ULONG conversion
//
#define Int16ToULong    ShortToULong

//
// INT16 -> ULONG_PTR conversion
//
#define Int16ToULongPtr ShortToULongPtr

//
// INT16 -> DWORD conversion
//
#define Int16ToDWord    ShortToULong

//
// INT16 -> DWORD_PTR conversion
//
#define Int16ToDWordPtr ShortToULongPtr

//
// INT16 -> ULONGLONG conversion
//
#define Int16ToULongLong    ShortToULongLong

//
// INT16 -> DWORDLONG conversion
//
#define Int16ToDWordLong    ShortToULongLong

//
// INT16 -> ULONG64 conversion
//
#define Int16ToULong64  ShortToULongLong

//
// INT16 -> DWORD64 conversion
//
#define Int16ToDWord64  ShortToULongLong

//
// INT16 -> UINT64 conversion
//
#define Int16ToUInt64   ShortToULongLong

//
// INT16 -> size_t conversion
//
#define Int16ToSizeT    ShortToUIntPtr

//
// INT16 -> SIZE_T conversion
//
#define Int16ToSIZET    ShortToULongPtr

//
// USHORT -> INT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
UShortToInt8(
    _In_ USHORT usOperand,
    _Out_ _Deref_out_range_(==, usOperand) INT8* pi8Result)
{
    HRESULT hr;
    
    if (usOperand <= INT8_MAX)
    {
        *pi8Result = (INT8)usOperand;
        hr = S_OK;
    }
    else
    {
        *pi8Result = INT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// USHORT -> UCHAR conversion
//
_Must_inspect_result_
__inline
HRESULT
UShortToUChar(
    _In_ USHORT usOperand,
    _Out_ _Deref_out_range_(==, usOperand) UCHAR* pch)
{
    HRESULT hr;

    if (usOperand <= 255)
    {
        *pch = (UCHAR)usOperand;
        hr = S_OK;
    }
    else
    {
        *pch = '\0';
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// USHORT -> CHAR conversion
//
__forceinline
HRESULT
UShortToChar(
    _In_ USHORT usOperand,
    _Out_ _Deref_out_range_(==, usOperand) CHAR* pch)
{
#ifdef _CHAR_UNSIGNED
    return UShortToUChar(usOperand, (UCHAR*)pch);
#else
    return UShortToInt8(usOperand, (INT8*)pch);
#endif // _CHAR_UNSIGNED
}

//
// USHORT -> UINT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
UShortToUInt8(
    _In_ USHORT usOperand,
    _Out_ _Deref_out_range_(==, usOperand) UINT8* pui8Result)
{
    HRESULT hr;
    
    if (usOperand <= UINT8_MAX)
    {
        *pui8Result = (UINT8)usOperand;
        hr = S_OK;
    }
    else
    {
        *pui8Result = UINT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// USHORT -> BYTE conversion
//
#define UShortToByte    UShortToUInt8

//
// USHORT -> SHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
UShortToShort(
    _In_ USHORT usOperand,
    _Out_ _Deref_out_range_(==, usOperand) SHORT* psResult)
{
    HRESULT hr;

    if (usOperand <= SHORT_MAX)
    {
        *psResult = (SHORT)usOperand;
        hr = S_OK;
    }
    else
    {
        *psResult = SHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// USHORT -> INT16 conversion
//
#define UShortToInt16   UShortToShort

//
// UINT16 -> CHAR conversion
//
#define UInt16ToChar    UShortToChar

//
// UINT16 -> INT8 conversion
//
#define UInt16ToInt8    UShortToInt8

//
// UINT16 -> UCHAR conversion
//
#define UInt16ToUChar   UShortToUChar

//
// UINT16 -> UINT8 conversion
//
#define UInt16ToUInt8   UShortToUInt8

//
// UINT16 -> BYTE conversion
//
#define UInt16ToByte    UShortToUInt8

//
// UINT16 -> SHORT conversion
//
#define UInt16ToShort   UShortToShort

//
// UINT16 -> INT16 conversion
//
#define UInt16ToInt16   UShortToShort

//
// WORD -> INT8 conversion
//
#define WordToInt8  UShortToInt8

//
// WORD -> CHAR conversion
//
#define WordToChar  UShortToChar

//
// WORD -> UCHAR conversion
//
#define WordToUChar UShortToUChar

//
// WORD -> UINT8 conversion
//
#define WordToUInt8 UShortToUInt8

//
// WORD -> BYTE conversion
//
#define WordToByte  UShortToUInt8

//
// WORD -> SHORT conversion
//
#define WordToShort UShortToShort

//
// WORD -> INT16 conversion
//
#define WordToInt16 UShortToShort

//
// INT -> INT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
IntToInt8(
    _In_ INT iOperand,
    _Out_ _Deref_out_range_(==, iOperand) INT8* pi8Result)
{
    HRESULT hr;
    
    if ((iOperand >= INT8_MIN) && (iOperand <= INT8_MAX))
    {
        *pi8Result = (INT8)iOperand;
        hr = S_OK;
    }
    else
    {
        *pi8Result = INT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// INT -> UCHAR conversion
//
_Must_inspect_result_
__inline
HRESULT
IntToUChar(
    _In_ INT iOperand,
    _Out_ _Deref_out_range_(==, iOperand) UCHAR* pch)
{
    HRESULT hr;

    if ((iOperand >= 0) && (iOperand <= 255))
    {
        *pch = (UCHAR)iOperand;
        hr = S_OK;
    }
    else
    {
        *pch = '\0';
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// INT -> CHAR conversion
//
__forceinline
HRESULT
IntToChar(
    _In_ INT iOperand,
    _Out_ _Deref_out_range_(==, iOperand) CHAR* pch)
{
#ifdef _CHAR_UNSIGNED
    return IntToUChar(iOperand, (UCHAR*)pch);
#else
    return IntToInt8(iOperand, (INT8*)pch);
#endif // _CHAR_UNSIGNED
}

//
// INT -> BYTE conversion
//
#define IntToByte   IntToUInt8

//
// INT -> UINT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
IntToUInt8(
    _In_ INT iOperand,
    _Out_ _Deref_out_range_(==, iOperand) UINT8* pui8Result)
{
    HRESULT hr;
    
    if ((iOperand >= 0) && (iOperand <= UINT8_MAX))
    {
        *pui8Result = (UINT8)iOperand;
        hr = S_OK;
    }
    else
    {
        *pui8Result = UINT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// INT -> SHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
IntToShort(
    _In_ INT iOperand,
    _Out_ _Deref_out_range_(==, iOperand) SHORT* psResult)
{
    HRESULT hr;

    if ((iOperand >= SHORT_MIN) && (iOperand <= SHORT_MAX))
    {
        *psResult = (SHORT)iOperand;
        hr = S_OK;
    }
    else
    {
        *psResult = SHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// INT -> INT16 conversion
//
#define IntToInt16  IntToShort

//
// INT -> USHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
IntToUShort(
    _In_ INT iOperand,
    _Out_ _Deref_out_range_(==, iOperand) USHORT* pusResult)
{
    HRESULT hr;

    if ((iOperand >= 0) && (iOperand <= USHORT_MAX))
    {
        *pusResult = (USHORT)iOperand;
        hr = S_OK;
    }
    else
    {
        *pusResult = USHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// INT -> UINT16 conversion
//
#define IntToUInt16  IntToUShort

//
// INT -> WORD conversion
//
#define IntToWord   IntToUShort

//
// INT -> UINT conversion
//
_Must_inspect_result_
__inline
HRESULT
IntToUInt(
    _In_ INT iOperand,
    _Out_ _Deref_out_range_(==, iOperand) UINT* puResult)
{
    HRESULT hr;

    if (iOperand >= 0)
    {
        *puResult = (UINT)iOperand;
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// INT -> UINT_PTR conversion
//
#ifdef _WIN64
#define IntToUIntPtr    IntToULongLong
#else
#define IntToUIntPtr    IntToUInt
#endif

//
// INT -> ULONG conversion
//
_Must_inspect_result_
__inline
HRESULT
IntToULong(
    _In_ INT iOperand,
    _Out_ _Deref_out_range_(==, iOperand) ULONG* pulResult)
{
    HRESULT hr;

    if (iOperand >= 0)
    {
        *pulResult = (ULONG)iOperand;
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// INT -> ULONG_PTR conversion
//
#ifdef _WIN64
#define IntToULongPtr   IntToULongLong
#else
#define IntToULongPtr   IntToULong
#endif

//
// INT -> DWORD conversion
//
#define IntToDWord  IntToULong

//
// INT -> DWORD_PTR conversion
//
#define IntToDWordPtr   IntToULongPtr

//
// INT -> ULONGLONG conversion
//
_Must_inspect_result_
__inline
HRESULT
IntToULongLong(
    _In_ INT iOperand,
    _Out_ _Deref_out_range_(==, iOperand) ULONGLONG* pullResult)
{
    HRESULT hr;

    if (iOperand >= 0)
    {
        *pullResult = (ULONGLONG)iOperand;
        hr = S_OK;
    }
    else
    {
        *pullResult = ULONGLONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// INT -> DWORDLONG conversion
//
#define IntToDWordLong  IntToULongLong

//
// INT -> ULONG64 conversion
//
#define IntToULong64    IntToULongLong

//
// INT -> DWORD64 conversion
//
#define IntToDWord64    IntToULongLong

//
// INT -> UINT64 conversion
//
#define IntToUInt64 IntToULongLong

//
// INT -> size_t conversion
//
#define IntToSizeT  IntToUIntPtr

//
// INT -> SIZE_T conversion
//
#define IntToSIZET  IntToULongPtr

//
// INT32 -> CHAR conversion
//
#define Int32ToChar IntToChar

//
// INT32 -> INT328 conversion
//
#define Int32ToInt8 IntToInt8

//
// INT32 -> UCHAR conversion
//
#define Int32ToUChar    IntToUChar

//
// INT32 -> BYTE conversion
//
#define Int32ToByte IntToUInt8

//
// INT32 -> UINT8 conversion
//
#define Int32ToUInt8    IntToUInt8

//
// INT32 -> SHORT conversion
//
#define Int32ToShort    IntToShort

//
// INT32 -> INT16 conversion
//
#define Int32ToInt16    IntToShort

//
// INT32 -> USHORT conversion
//
#define Int32ToUShort   IntToUShort

//
// INT32 -> UINT16 conversion
//
#define Int32ToUInt16   IntToUShort

//
// INT32 -> WORD conversion
//
#define Int32ToWord IntToUShort

//
// INT32 -> UINT conversion
//
#define Int32ToUInt IntToUInt

//
// INT32 -> UINT32 conversion
//
#define Int32ToUInt32   IntToUInt

//
// INT32 -> UINT_PTR conversion
//
#define Int32ToUIntPtr  IntToUIntPtr

//
// INT32 -> ULONG conversion
//
#define Int32ToULong    IntToULong

//
// INT32 -> ULONG_PTR conversion
//
#define Int32ToULongPtr IntToULongPtr

//
// INT32 -> DWORD conversion
//
#define Int32ToDWord    IntToULong

//
// INT32 -> DWORD_PTR conversion
//
#define Int32ToDWordPtr IntToULongPtr

//
// INT32 -> ULONGLONG conversion
//
#define Int32ToULongLong    IntToULongLong

//
// INT32 -> DWORDLONG conversion
//
#define Int32ToDWordLong    IntToULongLong

//
// INT32 -> ULONG64 conversion
//
#define Int32ToULong64  IntToULongLong

//
// INT32 -> DWORD64 conversion
//
#define Int32ToDWord64  IntToULongLong

//
// INT32 -> UINT64 conversion
//
#define Int32ToUInt64   IntToULongLong

//
// INT32 -> size_t conversion
//
#define Int32ToSizeT    IntToUIntPtr

//
// INT32 -> SIZE_T conversion
//
#define Int32ToSIZET    IntToULongPtr

//
// INT_PTR -> INT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
IntPtrToInt8(
    _In_ INT_PTR iOperand,
    _Out_ _Deref_out_range_(==, iOperand) INT8* pi8Result)
{
    HRESULT hr;
    
    if ((iOperand >= INT8_MIN) && (iOperand <= INT8_MAX))
    {
        *pi8Result = (INT8)iOperand;
        hr = S_OK;
    }
    else
    {
        *pi8Result = INT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// INT_PTR -> UCHAR conversion
//
_Must_inspect_result_
__inline
HRESULT
IntPtrToUChar(
    _In_ INT_PTR iOperand,
    _Out_ _Deref_out_range_(==, iOperand) UCHAR* pch)
{
    HRESULT hr;

    if ((iOperand >= 0) && (iOperand <= 255))
    {
        *pch = (UCHAR)iOperand;
        hr = S_OK;
    }
    else
    {
        *pch = '\0';
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// INT_PTR -> CHAR conversion
//
__forceinline
HRESULT
IntPtrToChar(
    _In_ INT_PTR iOperand,
    _Out_ _Deref_out_range_(==, iOperand) CHAR* pch)
{
#ifdef _CHAR_UNSIGNED
    return IntPtrToUChar(iOperand, (UCHAR*)pch);
#else
    return IntPtrToInt8(iOperand, (INT8*)pch);
#endif // _CHAR_UNSIGNED
}

//
// INT_PTR -> UINT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
IntPtrToUInt8(
    _In_ INT_PTR iOperand,
    _Out_ _Deref_out_range_(==, iOperand) UINT8* pui8Result)
{
    HRESULT hr;
    
    if ((iOperand >= 0) && (iOperand <= UINT8_MAX))
    {
        *pui8Result = (UINT8)iOperand;
        hr = S_OK;
    }
    else
    {
        *pui8Result = UINT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// INT_PTR -> BYTE conversion
//
#define IntPtrToByte    IntPtrToUInt8

//
// INT_PTR -> SHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
IntPtrToShort(
    _In_ INT_PTR iOperand,
    _Out_ _Deref_out_range_(==, iOperand) SHORT* psResult)
{
    HRESULT hr;

    if ((iOperand >= SHORT_MIN) && (iOperand <= SHORT_MAX))
    {
        *psResult = (SHORT)iOperand;
        hr = S_OK;
    }
    else
    {
        *psResult = SHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// INT_PTR -> INT16 conversion
//
#define IntPtrToInt16   IntPtrToShort

//
// INT_PTR -> USHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
IntPtrToUShort(
    _In_ INT_PTR iOperand,
    _Out_ _Deref_out_range_(==, iOperand) USHORT* pusResult)
{
    HRESULT hr;

    if ((iOperand >= 0) && (iOperand <= USHORT_MAX))
    {
        *pusResult = (USHORT)iOperand;
        hr = S_OK;
    }
    else
    {
        *pusResult = USHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// INT_PTR -> UINT16 conversion
//
#define IntPtrToUInt16  IntPtrToUShort

//
// INT_PTR -> WORD conversion
//
#define IntPtrToWord    IntPtrToUShort

//
// INT_PTR -> INT conversion
//
#ifdef _WIN64
#define IntPtrToInt LongLongToInt
#else
_Must_inspect_result_
__inline
HRESULT
IntPtrToInt(
    _In_ INT_PTR iOperand,
    _Out_ _Deref_out_range_(==, iOperand) INT* piResult)
{
    *piResult = (INT)iOperand;
    return S_OK;
}
#endif

//
// INT_PTR -> INT32 conversion
//
#define IntPtrToInt32   IntPtrToInt

//
// INT_PTR -> UINT conversion
//
#ifdef _WIN64
#define IntPtrToUInt    LongLongToUInt
#else
_Must_inspect_result_
__inline
HRESULT
IntPtrToUInt(
    _In_ INT_PTR iOperand,
    _Out_ _Deref_out_range_(==, iOperand) UINT* puResult)
{
    HRESULT hr;

    if (iOperand >= 0)
    {
        *puResult = (UINT)iOperand;
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}
#endif

//
// INT_PTR -> UINT32 conversion
//
#define IntPtrToUInt32  IntPtrToUInt

//
// INT_PTR -> UINT_PTR conversion
//
#ifdef _WIN64
#define IntPtrToUIntPtr LongLongToULongLong
#else
_Must_inspect_result_
__inline
HRESULT
IntPtrToUIntPtr(
    _In_ INT_PTR iOperand,
    _Out_ _Deref_out_range_(==, iOperand) UINT_PTR* puResult)
{
    HRESULT hr;

    if (iOperand >= 0)
    {
        *puResult = (UINT_PTR)iOperand;
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}
#endif

//
// INT_PTR -> LONG conversion
//
#ifdef _WIN64
#define IntPtrToLong    LongLongToLong
#else
_Must_inspect_result_
__inline
HRESULT
IntPtrToLong(
    _In_ INT_PTR iOperand,
    _Out_ _Deref_out_range_(==, iOperand) LONG* plResult)
{
    *plResult = (LONG)iOperand;
    return S_OK;
}
#endif

//
// INT_PTR -> LONG_PTR conversion
//
_Must_inspect_result_
__inline
HRESULT
IntPtrToLongPtr(
    _In_ INT_PTR iOperand,
    _Out_ _Deref_out_range_(==, iOperand) LONG_PTR* plResult)
{
    *plResult = (LONG_PTR)iOperand;
    return S_OK;
}

//
// INT_PTR -> ULONG conversion
//
#ifdef _WIN64
#define IntPtrToULong   LongLongToULong
#else
_Must_inspect_result_
__inline
HRESULT
IntPtrToULong(
    _In_ INT_PTR iOperand,
    _Out_ _Deref_out_range_(==, iOperand) ULONG* pulResult)
{
    HRESULT hr;

    if (iOperand >= 0)
    {
        *pulResult = (ULONG)iOperand;
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}
#endif

//
// INT_PTR -> ULONG_PTR conversion
//
#ifdef _WIN64
#define IntPtrToULongPtr    LongLongToULongLong
#else
_Must_inspect_result_
__inline
HRESULT
IntPtrToULongPtr(
    _In_ INT_PTR iOperand,
    _Out_ _Deref_out_range_(==, iOperand) ULONG_PTR* pulResult)
{
    HRESULT hr;

    if (iOperand >= 0)
    {
        *pulResult = (ULONG_PTR)iOperand;
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}
#endif

//
// INT_PTR -> DWORD conversion
//
#define IntPtrToDWord   IntPtrToULong

//    
// INT_PTR -> DWORD_PTR conversion
//
#define IntPtrToDWordPtr    IntPtrToULongPtr

//
// INT_PTR -> ULONGLONG conversion
//
#ifdef _WIN64
#define IntPtrToULongLong   LongLongToULongLong
#else
_Must_inspect_result_
__inline
HRESULT
IntPtrToULongLong(
    _In_ INT_PTR iOperand,
    _Out_ _Deref_out_range_(==, iOperand) ULONGLONG* pullResult)
{
    HRESULT hr;

    if (iOperand >= 0)
    {
        *pullResult = (ULONGLONG)iOperand;
        hr = S_OK;
    }
    else
    {
        *pullResult = ULONGLONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}   
#endif

//
// INT_PTR -> DWORDLONG conversion
//
#define IntPtrToDWordLong   IntPtrToULongLong

//
// INT_PTR -> ULONG64 conversion
//
#define IntPtrToULong64 IntPtrToULongLong

//
// INT_PTR -> DWORD64 conversion
//
#define IntPtrToDWord64 IntPtrToULongLong

//
// INT_PTR -> UINT64 conversion
//
#define IntPtrToUInt64  IntPtrToULongLong

//
// INT_PTR -> size_t conversion
//
#define IntPtrToSizeT   IntPtrToUIntPtr

//
// INT_PTR -> SIZE_T conversion
//
#define IntPtrToSIZET   IntPtrToULongPtr

//
// UINT -> INT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntToInt8(
    _In_ UINT uOperand,
    _Out_ _Deref_out_range_(==, uOperand) INT8* pi8Result)
{
    HRESULT hr;
    
    if (uOperand <= INT8_MAX)
    {
        *pi8Result = (INT8)uOperand;
        hr = S_OK;
    }
    else
    {
        *pi8Result = INT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}    

//
// UINT -> UCHAR conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntToUChar(
    _In_ UINT uOperand,
    _Out_ _Deref_out_range_(==, uOperand) UCHAR* pch)
{
    HRESULT hr;

    if (uOperand <= 255)
    {
        *pch = (UCHAR)uOperand;
        hr = S_OK;
    }
    else
    {
        *pch = '\0';
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// UINT -> CHAR conversion
//
__forceinline
HRESULT
UIntToChar(
    _In_ UINT uOperand,
    _Out_ _Deref_out_range_(==, uOperand) CHAR* pch)
{
#ifdef _CHAR_UNSIGNED
    return UIntToUChar(uOperand, (UCHAR*)pch);
#else
    return UIntToInt8(uOperand, (INT8*)pch);
#endif
}

//
// UINT -> UINT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntToUInt8(
    _In_ UINT uOperand,
    _Out_ _Deref_out_range_(==, uOperand) UINT8* pui8Result)
{
    HRESULT hr;
    
    if (uOperand <= UINT8_MAX)
    {
        *pui8Result = (UINT8)uOperand;
        hr = S_OK;
    }
    else
    {
        *pui8Result = UINT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}    
    
//
// UINT -> BYTE conversion
//
#define UIntToByte   UIntToUInt8

//
// UINT -> SHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntToShort(
    _In_ UINT uOperand,
    _Out_ _Deref_out_range_(==, uOperand) SHORT* psResult)
{
    HRESULT hr;

    if (uOperand <= SHORT_MAX)
    {
        *psResult = (SHORT)uOperand;
        hr = S_OK;
    }
    else
    {
        *psResult = SHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// UINT -> INT16 conversion
//
#define UIntToInt16 UIntToShort

//
// UINT -> USHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntToUShort(
    _In_ UINT uOperand,
    _Out_ _Deref_out_range_(==, uOperand) USHORT* pusResult)
{
    HRESULT hr;

    if (uOperand <= USHORT_MAX)
    {
        *pusResult = (USHORT)uOperand;
        hr = S_OK;
    }
    else
    {
        *pusResult = USHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// UINT -> UINT16 conversion
//
#define UIntToUInt16    UIntToUShort

//
// UINT -> WORD conversion
//
#define UIntToWord  UIntToUShort

//
// UINT -> INT conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntToInt(
    _In_ UINT uOperand,
    _Out_ _Deref_out_range_(==, uOperand) INT* piResult)
{
    HRESULT hr;

    if (uOperand <= INT_MAX)
    {
        *piResult = (INT)uOperand;
        hr = S_OK;
    }
    else
    {
        *piResult = INT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// UINT -> INT32 conversion
//
#define UIntToInt32 UIntToInt

//
// UINT -> INT_PTR conversion
//
#ifdef _WIN64
_Must_inspect_result_
__inline
HRESULT
UIntToIntPtr(
    _In_ UINT uOperand,
    _Out_ _Deref_out_range_(==, uOperand) INT_PTR* piResult)
{
    *piResult = uOperand;
    return S_OK;
}
#else
#define UIntToIntPtr    UIntToInt
#endif

//
// UINT -> LONG conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntToLong(
    _In_ UINT uOperand,
    _Out_ _Deref_out_range_(==, uOperand) LONG* plResult)
{
    HRESULT hr;

    if (uOperand <= LONG_MAX)
    {
        *plResult = (LONG)uOperand;
        hr = S_OK;
    }
    else
    {
        *plResult = LONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// UINT -> LONG_PTR conversion
//
#ifdef _WIN64
_Must_inspect_result_
__inline
HRESULT
UIntToLongPtr(
    _In_ UINT uOperand,
    _Out_ _Deref_out_range_(==, uOperand) LONG_PTR* plResult)
{
    *plResult = uOperand;
    return S_OK;
}
#else
#define UIntToLongPtr   UIntToLong
#endif

//
// UINT -> ptrdiff_t conversion
//
#define UIntToPtrdiffT  UIntToIntPtr

//
// UINT -> SSIZE_T conversion
//
#define UIntToSSIZET    UIntToLongPtr

//
// UINT32 -> CHAR conversion
//
#define UInt32ToChar    UIntToChar

//
// UINT32 -> INT8 conversion
//
#define UInt32ToInt8    UIntToInt8

//
// UINT32 -> UCHAR conversion
//
#define UInt32ToUChar   UIntToUChar

//
// UINT32 -> UINT8 conversion
//
#define UInt32ToUInt8   UIntToUInt8

//
// UINT32 -> BYTE conversion
//
#define UInt32ToByte    UInt32ToUInt8

//
// UINT32 -> SHORT conversion
//
#define UInt32ToShort   UIntToShort

//
// UINT32 -> INT16 conversion
//
#define UInt32ToInt16   UIntToShort

//
// UINT32 -> USHORT conversion
//
#define UInt32ToUShort  UIntToUShort

//
// UINT32 -> UINT16 conversion
//
#define UInt32ToUInt16  UIntToUShort

//
// UINT32 -> WORD conversion
//
#define UInt32ToWord    UIntToUShort

//
// UINT32 -> INT conversion
//
#define UInt32ToInt UIntToInt

//
// UINT32 -> INT_PTR conversion
//
#define UInt32ToIntPtr  UIntToIntPtr

//
// UINT32 -> INT32 conversion
//
#define UInt32ToInt32   UIntToInt

//
// UINT32 -> LONG conversion
//
#define UInt32ToLong    UIntToLong

//
// UINT32 -> LONG_PTR conversion
//
#define UInt32ToLongPtr UIntToLongPtr

//
// UINT32 -> ptrdiff_t conversion
//
#define UInt32ToPtrdiffT    UIntToPtrdiffT

//
// UINT32 -> SSIZE_T conversion
//
#define UInt32ToSSIZET  UIntToSSIZET

//
// UINT_PTR -> INT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntPtrToInt8(
    _In_ UINT_PTR uOperand,
    _Out_ _Deref_out_range_(==, uOperand) INT8* pi8Result)
{
    HRESULT hr;
    
    if (uOperand <= INT8_MAX)
    {
        *pi8Result = (INT8)uOperand;
        hr = S_OK;
    }
    else
    {
        *pi8Result = INT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// UINT_PTR -> UCHAR conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntPtrToUChar(
    _In_ UINT_PTR uOperand,
    _Out_ _Deref_out_range_(==, uOperand) UCHAR* pch)
{
    HRESULT hr;

    if (uOperand <= 255)
    {
        *pch = (UCHAR)uOperand;
        hr = S_OK;
    }
    else
    {
        *pch = '\0';
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// UINT_PTR -> CHAR conversion
//
__forceinline
HRESULT
UIntPtrToChar(
    _In_ UINT_PTR uOperand,
    _Out_ _Deref_out_range_(==, uOperand) CHAR* pch)
{
#ifdef _CHAR_UNSIGNED
    return UIntPtrToUChar(uOperand, (UCHAR*)pch);
#else
    return UIntPtrToInt8(uOperand, (INT8*)pch);
#endif
}

//
// UINT_PTR -> UINT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntPtrToUInt8(
    _In_ UINT_PTR uOperand,
    _Out_ _Deref_out_range_(==,uOperand) UINT8* pu8Result)
{
    HRESULT hr;
    
    if (uOperand <= UINT8_MAX)
    {
        *pu8Result = (UINT8)uOperand;
        hr = S_OK;
    }
    else
    {
        *pu8Result = UINT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// UINT_PTR -> BYTE conversion
//
#define UIntPtrToByte   UIntPtrToUInt8

//
// UINT_PTR -> SHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntPtrToShort(
    _In_ UINT_PTR uOperand,
    _Out_ _Deref_out_range_(==, uOperand) SHORT* psResult)
{
    HRESULT hr;

    if (uOperand <= SHORT_MAX)
    {
        *psResult = (SHORT)uOperand;
        hr = S_OK;
    }
    else
    {
        *psResult = SHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr; 
}

//
// UINT_PTR -> INT16 conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntPtrToInt16(
    _In_ UINT_PTR uOperand,
    _Out_ _Deref_out_range_(==, uOperand) INT16* pi16Result)
{
    HRESULT hr;
    
    if (uOperand <= INT16_MAX)
    {
        *pi16Result = (INT16)uOperand;
        hr = S_OK;
    }
    else
    {
        *pi16Result = INT16_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// UINT_PTR -> USHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntPtrToUShort(
    _In_ UINT_PTR uOperand,
    _Out_ _Deref_out_range_(==, uOperand) USHORT* pusResult)
{
    HRESULT hr;

    if (uOperand <= USHORT_MAX)
    {
        *pusResult = (USHORT)uOperand;
        hr = S_OK;
    }
    else
    {
        *pusResult = USHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// UINT_PTR -> UINT16 conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntPtrToUInt16(
    _In_ UINT_PTR uOperand,
    _Out_ _Deref_out_range_(==, uOperand) UINT16* pu16Result)
{
    HRESULT hr;
    
    if (uOperand <= UINT16_MAX)
    {
        *pu16Result = (UINT16)uOperand;
        hr = S_OK;
    }
    else
    {
        *pu16Result = UINT16_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// UINT_PTR -> WORD conversion
//
#define UIntPtrToWord   UIntPtrToUShort

//
// UINT_PTR -> INT conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntPtrToInt(
    _In_ UINT_PTR uOperand,
    _Out_ _Deref_out_range_(==, uOperand) INT* piResult)
{
    HRESULT hr;

    if (uOperand <= INT_MAX)
    {
        *piResult = (INT)uOperand;
        hr = S_OK;
    }
    else
    {
        *piResult = INT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// UINT_PTR -> INT32 conversion
//
#define UIntPtrToInt32  UIntPtrToInt

//
// UINT_PTR -> INT_PTR conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntPtrToIntPtr(
    _In_ UINT_PTR uOperand,
    _Out_ _Deref_out_range_(==, uOperand) INT_PTR* piResult)
{
    HRESULT hr;

    if (uOperand <= INT_PTR_MAX)
    {
        *piResult = (INT_PTR)uOperand;
        hr = S_OK;
    }
    else
    {
        *piResult = INT_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// UINT_PTR -> UINT conversion
//
#ifdef _WIN64
#define UIntPtrToUInt   ULongLongToUInt
#else
_Must_inspect_result_
__inline
HRESULT
UIntPtrToUInt(
    _In_ UINT_PTR uOperand,
    _Out_ _Deref_out_range_(==, uOperand) UINT* puResult)
{
    *puResult = (UINT)uOperand;
    return S_OK;
}
#endif

//
// UINT_PTR -> UINT32 conversion
//
#define UIntPtrToUInt32 UIntPtrToUInt

//
// UINT_PTR -> LONG conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntPtrToLong(
    _In_ UINT_PTR uOperand,
    _Out_ _Deref_out_range_(==, uOperand) LONG* plResult)
{
    HRESULT hr;

    if (uOperand <= LONG_MAX)
    {
        *plResult = (LONG)uOperand;
        hr = S_OK;
    }
    else
    {
        *plResult = LONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// UINT_PTR -> LONG_PTR conversion
//
_Must_inspect_result_
__inline
HRESULT
UIntPtrToLongPtr(
    _In_ UINT_PTR uOperand,
    _Out_ _Deref_out_range_(==, uOperand) LONG_PTR* plResult)
{
    HRESULT hr;

    if (uOperand <= LONG_PTR_MAX)
    {
        *plResult = (LONG_PTR)uOperand;
        hr = S_OK;
    }
    else
    {
        *plResult = LONG_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// UINT_PTR -> ULONG conversion
//
#ifdef _WIN64
#define UIntPtrToULong  ULongLongToULong
#else
_Must_inspect_result_
__inline
HRESULT
UIntPtrToULong(
    _In_ UINT_PTR uOperand,
    _Out_ _Deref_out_range_(==, uOperand) ULONG* pulResult)
{
    *pulResult = (ULONG)uOperand;
    return S_OK;
}
#endif

//
// UINT_PTR -> DWORD conversion
//
#define UIntPtrToDWord  UIntPtrToULong

//
// UINT_PTR -> LONGLONG conversion
//
#ifdef _WIN64
#define UIntPtrToLongLong   ULongLongToLongLong
#else
_Must_inspect_result_
__inline
HRESULT
UIntPtrToLongLong(
    _In_ UINT_PTR uOperand,
    _Out_ _Deref_out_range_(==, uOperand) LONGLONG* pllResult)
{
    *pllResult = (LONGLONG)uOperand;
    return S_OK;
}
#endif

//
// UINT_PTR -> LONG64 conversion
//
#define UIntPtrToLong64 UIntPtrToLongLong

//
// UINT_PTR -> INT64 conversion
//
#define UIntPtrToInt64  UIntPtrToLongLong

//
// UINT_PTR -> ptrdiff_t conversion
//
#define UIntPtrToPtrdiffT   UIntPtrToIntPtr

//
// UINT_PTR -> SSIZE_T conversion
//
#define UIntPtrToSSIZET UIntPtrToLongPtr

//
// LONG -> INT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
LongToInt8(
    _In_ LONG lOperand,
    _Out_ _Deref_out_range_(==, lOperand) INT8* pi8Result)
{
    HRESULT hr;
    
    if ((lOperand >= INT8_MIN) && (lOperand <= INT8_MAX))
    {
        *pi8Result = (INT8)lOperand;
        hr = S_OK;
    }
    else
    {
        *pi8Result = INT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONG -> UCHAR conversion
//
_Must_inspect_result_
__inline
HRESULT
LongToUChar(
    _In_ LONG lOperand,
    _Out_ _Deref_out_range_(==, lOperand) UCHAR* pch)
{
    HRESULT hr;

    if ((lOperand >= 0) && (lOperand <= 255))
    {
        *pch = (UCHAR)lOperand;
        hr = S_OK;
    }
    else
    {
        *pch = '\0';
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// LONG -> CHAR conversion
//
__forceinline
HRESULT
LongToChar(
    _In_ LONG lOperand,
    _Out_ _Deref_out_range_(==, lOperand) CHAR* pch)
{
#ifdef _CHAR_UNSIGNED
    return LongToUChar(lOperand, (UCHAR*)pch);
#else
    return LongToInt8(lOperand, (INT8*)pch);
#endif
}

//
// LONG -> UINT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
LongToUInt8(
    _In_ LONG lOperand,
    _Out_ _Deref_out_range_(==, lOperand) UINT8* pui8Result)
{
    HRESULT hr;
    
    if ((lOperand >= 0) && (lOperand <= UINT8_MAX))
    {
        *pui8Result = (UINT8)lOperand;
        hr = S_OK;
    }
    else
    {
        *pui8Result = UINT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONG -> BYTE conversion
//
#define LongToByte  LongToUInt8

//
// LONG -> SHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
LongToShort(
    _In_ LONG lOperand,
    _Out_ _Deref_out_range_(==, lOperand) SHORT* psResult)
{
    HRESULT hr;
     
    if ((lOperand >= SHORT_MIN) && (lOperand <= SHORT_MAX))
    {
       *psResult = (SHORT)lOperand;
       hr = S_OK;
    }
    else
    {
        *psResult = SHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
     
    return hr;
}

//
// LONG -> INT16 conversion
//
#define LongToInt16 LongToShort

//
// LONG -> USHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
LongToUShort(
    _In_ LONG lOperand,
    _Out_ _Deref_out_range_(==, lOperand) USHORT* pusResult)
{
    HRESULT hr;
    
    if ((lOperand >= 0) && (lOperand <= USHORT_MAX))
    {
        *pusResult = (USHORT)lOperand;
        hr = S_OK;
    }
    else
    {
        *pusResult = USHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONG -> UINT16 conversion
//
#define LongToUInt16    LongToUShort

//   
// LONG -> WORD conversion
//
#define LongToWord  LongToUShort

//
// LONG -> INT conversion
//
_Must_inspect_result_
__inline
HRESULT
LongToInt(
    _In_ LONG lOperand,
    _Out_ _Deref_out_range_(==, lOperand) INT* piResult)
{
    C_ASSERT(sizeof(INT) == sizeof(LONG));
    *piResult = (INT)lOperand;
    return S_OK;
}

//
// LONG -> INT32 conversion
//
#define LongToInt32 LongToInt

//
// LONG -> INT_PTR conversion
//
#ifdef _WIN64
_Must_inspect_result_
__inline
HRESULT
LongToIntPtr(
    _In_ LONG lOperand,
    _Out_ _Deref_out_range_(==, lOperand) INT_PTR* piResult)
{
    *piResult = lOperand;
    return S_OK;
}
#else
#define LongToIntPtr    LongToInt
#endif

//
// LONG -> UINT conversion
//
_Must_inspect_result_
__inline
HRESULT
LongToUInt(
    _In_ LONG lOperand,
    _Out_ _Deref_out_range_(==, lOperand) UINT* puResult)
{
    HRESULT hr;
    
    if (lOperand >= 0)
    {
        *puResult = (UINT)lOperand;
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONG -> UINT32 conversion
//
#define LongToUInt32    LongToUInt

//
// LONG -> UINT_PTR conversion
//
#ifdef _WIN64
_Must_inspect_result_
__inline
HRESULT
LongToUIntPtr(
    _In_ LONG lOperand,
    _Out_ _Deref_out_range_(==, lOperand) UINT_PTR* puResult)
{
    HRESULT hr;
    
    if (lOperand >= 0)
    {
        *puResult = (UINT_PTR)lOperand;
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}
#else
#define LongToUIntPtr   LongToUInt
#endif

//
// LONG -> ULONG conversion
//
_Must_inspect_result_
__inline
HRESULT
LongToULong(
    _In_ LONG lOperand,
    _Out_ _Deref_out_range_(==, lOperand) ULONG* pulResult)
{
    HRESULT hr;
    
    if (lOperand >= 0)
    {
        *pulResult = (ULONG)lOperand;
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONG -> ULONG_PTR conversion
//
#ifdef _WIN64
_Must_inspect_result_
__inline
HRESULT
LongToULongPtr(
    _In_ LONG lOperand,
    _Out_ _Deref_out_range_(==, lOperand) ULONG_PTR* pulResult)
{
    HRESULT hr;
    
    if (lOperand >= 0)
    {
        *pulResult = (ULONG_PTR)lOperand;
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}
#else
#define LongToULongPtr  LongToULong
#endif

//
// LONG -> DWORD conversion
//
#define LongToDWord LongToULong

//
// LONG -> DWORD_PTR conversion
//
#define LongToDWordPtr  LongToULongPtr

//
// LONG -> ULONGLONG conversion
//
_Must_inspect_result_
__inline
HRESULT
LongToULongLong(
    _In_ LONG lOperand,
    _Out_ _Deref_out_range_(==, lOperand) ULONGLONG* pullResult)
{
    HRESULT hr;
    
    if (lOperand >= 0)
    {
        *pullResult = (ULONGLONG)lOperand;
        hr = S_OK;
    }
    else
    {
        *pullResult = ULONGLONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONG -> DWORDLONG conversion
//
#define LongToDWordLong LongToULongLong

//
// LONG -> ULONG64 conversion
//
#define LongToULong64   LongToULongLong

//
// LONG -> DWORD64 conversion
//
#define LongToDWord64   LongToULongLong

//
// LONG -> UINT64 conversion
//
#define LongToUInt64    LongToULongLong

//
// LONG -> ptrdiff_t conversion
//
#define LongToPtrdiffT  LongToIntPtr

//
// LONG -> size_t conversion
//
#define LongToSizeT LongToUIntPtr

//
// LONG -> SIZE_T conversion
//
#define LongToSIZET LongToULongPtr

//
// LONG_PTR -> INT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
LongPtrToInt8(
    _In_ LONG_PTR lOperand,
    _Out_ _Deref_out_range_(==, lOperand) INT8* pi8Result)
{
    HRESULT hr;
    
    if ((lOperand >= INT8_MIN) && (lOperand <= INT8_MAX))
    {
        *pi8Result = (INT8)lOperand;
        hr = S_OK;
    }
    else
    {
        *pi8Result = INT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONG_PTR -> UCHAR conversion
//
_Must_inspect_result_
__inline
HRESULT
LongPtrToUChar(
    _In_ LONG_PTR lOperand,
    _Out_ _Deref_out_range_(==, lOperand) UCHAR* pch)
{
    HRESULT hr;
    
    if ((lOperand >= 0) && (lOperand <= 255))
    {
        *pch = (UCHAR)lOperand;
        hr = S_OK;
    }
    else
    {
        *pch = '\0';
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONG_PTR -> CHAR conversion
//
__forceinline
HRESULT
LongPtrToChar(
    _In_ LONG_PTR lOperand,
    _Out_ _Deref_out_range_(==, lOperand) CHAR* pch)
{
#ifdef _CHAR_UNSIGNED
    return LongPtrToUChar(lOperand, (UCHAR*)pch);
#else
    return LongPtrToInt8(lOperand, (INT8*)pch);
#endif
}

//
// LONG_PTR -> UINT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
LongPtrToUInt8(
    _In_ LONG_PTR lOperand,
    _Out_ _Deref_out_range_(==, lOperand) UINT8* pui8Result)
{
    HRESULT hr;
    
    if ((lOperand >= 0) && (lOperand <= UINT8_MAX))
    {
        *pui8Result = (UINT8)lOperand;
        hr = S_OK;
    }
    else
    {
        *pui8Result = UINT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONG_PTR -> BYTE conversion
//
#define LongPtrToByte   LongPtrToUInt8

//
// LONG_PTR -> SHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
LongPtrToShort(
    _In_ LONG_PTR lOperand,
    _Out_ _Deref_out_range_(==, lOperand) SHORT* psResult)
{
    HRESULT hr;
    
    if ((lOperand >= SHORT_MIN) && (lOperand <= SHORT_MAX))
    {
        *psResult = (SHORT)lOperand;
        hr = S_OK;
    }
    else
    {
        *psResult = SHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}    

//
// LONG_PTR -> INT16 conversion
//
#define LongPtrToInt16  LongPtrToShort

//
// LONG_PTR -> USHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
LongPtrToUShort(
    _In_ LONG_PTR lOperand,
    _Out_ _Deref_out_range_(==, lOperand) USHORT* pusResult)
{
    HRESULT hr;
    
    if ((lOperand >= 0) && (lOperand <= USHORT_MAX))
    {
        *pusResult = (USHORT)lOperand;
        hr = S_OK;
    }
    else
    {
        *pusResult = USHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONG_PTR -> UINT16 conversion
//
#define LongPtrToUInt16 LongPtrToUShort

//
// LONG_PTR -> WORD conversion
//
#define LongPtrToWord   LongPtrToUShort

//
// LONG_PTR -> INT conversion
//
#ifdef _WIN64
#define LongPtrToInt    LongLongToInt
#else
_Must_inspect_result_
__inline
HRESULT
LongPtrToInt(
    _In_ LONG_PTR lOperand,
    _Out_ _Deref_out_range_(==, lOperand) INT* piResult)
{
    C_ASSERT(sizeof(INT) == sizeof(LONG_PTR));
    *piResult = (INT)lOperand;
    return S_OK;
}   
#endif

//
// LONG_PTR -> INT32 conversion
//
#define LongPtrToInt32  LongPtrToInt

//
// LONG_PTR -> INT_PTR conversion
//
_Must_inspect_result_
__inline
HRESULT
LongPtrToIntPtr(
    _In_ LONG_PTR lOperand,
    _Out_ _Deref_out_range_(==, lOperand) INT_PTR* piResult)
{
    C_ASSERT(sizeof(LONG_PTR) == sizeof(INT_PTR));
    *piResult = (INT_PTR)lOperand;
    return S_OK;
}

//
// LONG_PTR -> UINT conversion
//
#ifdef _WIN64
#define LongPtrToUInt   LongLongToUInt
#else
_Must_inspect_result_
__inline
HRESULT
LongPtrToUInt(
    _In_ LONG_PTR lOperand,
    _Out_ _Deref_out_range_(==, lOperand) UINT* puResult)
{
    HRESULT hr;
    
    if (lOperand >= 0)
    {
        *puResult = (UINT)lOperand;
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}
#endif

//
// LONG_PTR -> UINT32 conversion
//
#define LongPtrToUInt32 LongPtrToUInt

//
// LONG_PTR -> UINT_PTR conversion
//
_Must_inspect_result_
__inline
HRESULT
LongPtrToUIntPtr(
    _In_ LONG_PTR lOperand,
    _Out_ _Deref_out_range_(==, lOperand) UINT_PTR* puResult)
{
    HRESULT hr;
    
    if (lOperand >= 0)
    {
        *puResult = (UINT_PTR)lOperand;
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONG_PTR -> LONG conversion
//
#ifdef _WIN64
#define LongPtrToLong   LongLongToLong
#else
_Must_inspect_result_
__inline
HRESULT
LongPtrToLong(
    _In_ LONG_PTR lOperand,
    _Out_ _Deref_out_range_(==, lOperand) LONG* plResult)
{
    *plResult = (LONG)lOperand;
    return S_OK;
}
#endif

//    
// LONG_PTR -> ULONG conversion
//
#ifdef _WIN64
#define LongPtrToULong  LongLongToULong
#else
_Must_inspect_result_
__inline
HRESULT
LongPtrToULong(
    _In_ LONG_PTR lOperand,
    _Out_ _Deref_out_range_(==, lOperand) ULONG* pulResult)
{
    HRESULT hr;
    
    if (lOperand >= 0)
    {
        *pulResult = (ULONG)lOperand;
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}
#endif

//
// LONG_PTR -> ULONG_PTR conversion
//
_Must_inspect_result_
__inline
HRESULT
LongPtrToULongPtr(
    _In_ LONG_PTR lOperand,
    _Out_ _Deref_out_range_(==, lOperand) ULONG_PTR* pulResult)
{
    HRESULT hr;
    
    if (lOperand >= 0)
    {
        *pulResult = (ULONG_PTR)lOperand;
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONG_PTR -> DWORD conversion
//
#define LongPtrToDWord  LongPtrToULong

//
// LONG_PTR -> DWORD_PTR conversion
//
#define LongPtrToDWordPtr   LongPtrToULongPtr 

//
// LONG_PTR -> ULONGLONG conversion
//
_Must_inspect_result_
__inline
HRESULT
LongPtrToULongLong(
    _In_ LONG_PTR lOperand,
    _Out_ _Deref_out_range_(==, lOperand) ULONGLONG* pullResult)
{
    HRESULT hr;
    
    if (lOperand >= 0)
    {
        *pullResult = (ULONGLONG)lOperand;
        hr = S_OK;
    }
    else
    {
        *pullResult = ULONGLONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONG_PTR -> DWORDLONG conversion
//
#define LongPtrToDWordLong  LongPtrToULongLong

//
// LONG_PTR -> ULONG64 conversion
//
#define LongPtrToULong64    LongPtrToULongLong

//
// LONG_PTR -> DWORD64 conversion
//
#define LongPtrToDWord64    LongPtrToULongLong

//
// LONG_PTR -> UINT64 conversion
//
#define LongPtrToUInt64 LongPtrToULongLong

//
// LONG_PTR -> size_t conversion
//
#define LongPtrToSizeT  LongPtrToUIntPtr

//
// LONG_PTR -> SIZE_T conversion
//
#define LongPtrToSIZET  LongPtrToULongPtr

//
// ULONG -> INT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongToInt8(
    _In_ ULONG ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) INT8* pi8Result)
{
    HRESULT hr;
    
    if (ulOperand <= INT8_MAX)
    {
        *pi8Result = (INT8)ulOperand;
        hr = S_OK;
    }
    else
    {
        *pi8Result = INT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONG -> UCHAR conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongToUChar(
    _In_ ULONG ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) UCHAR* pch)
{
    HRESULT hr;

    if (ulOperand <= 255)
    {
        *pch = (UCHAR)ulOperand;
        hr = S_OK;
    }
    else
    {
        *pch = '\0';
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// ULONG -> CHAR conversion
//
__forceinline
HRESULT
ULongToChar(
    _In_ ULONG ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) CHAR* pch)
{
#ifdef _CHAR_UNSIGNED
    return ULongToUChar(ulOperand, (UCHAR*)pch);
#else
    return ULongToInt8(ulOperand, (INT8*)pch);
#endif
}

//
// ULONG -> UINT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongToUInt8(
    _In_ ULONG ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) UINT8* pui8Result)
{
    HRESULT hr;
    
    if (ulOperand <= UINT8_MAX)
    {
        *pui8Result = (UINT8)ulOperand;
        hr = S_OK;
    }
    else
    {
        *pui8Result = UINT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}    

//
// ULONG -> BYTE conversion
//
#define ULongToByte ULongToUInt8

//
// ULONG -> SHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongToShort(
    _In_ ULONG ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) SHORT* psResult)
{
    HRESULT hr;

    if (ulOperand <= SHORT_MAX)
    {
        *psResult = (SHORT)ulOperand;
        hr = S_OK;
    }
    else
    {
        *psResult = SHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// ULONG -> INT16 conversion
//
#define ULongToInt16    ULongToShort

//
// ULONG -> USHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongToUShort(
    _In_ ULONG ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) USHORT* pusResult)
{
    HRESULT hr;

    if (ulOperand <= USHORT_MAX)
    {
        *pusResult = (USHORT)ulOperand;
        hr = S_OK;
    }
    else
    {
        *pusResult = USHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// ULONG -> UINT16 conversion
//
#define ULongToUInt16   ULongToUShort

//
// ULONG -> WORD conversion
//
#define ULongToWord ULongToUShort

//
// ULONG -> INT conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongToInt(
    _In_ ULONG ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) INT* piResult)
{
    HRESULT hr;
    
    if (ulOperand <= INT_MAX)
    {
        *piResult = (INT)ulOperand;
        hr = S_OK;
    }
    else
    {
        *piResult = INT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONG -> INT32 conversion
//
#define ULongToInt32    ULongToInt

//
// ULONG -> INT_PTR conversion
//
#ifdef _WIN64
_Must_inspect_result_
__inline
HRESULT
ULongToIntPtr(
    _In_ ULONG ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) INT_PTR* piResult)
{
    *piResult = (INT_PTR)ulOperand;
    return S_OK;
}
#else
#define ULongToIntPtr   ULongToInt
#endif

//
// ULONG -> UINT conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongToUInt(
    _In_ ULONG ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) UINT* puResult)
{
    C_ASSERT(sizeof(ULONG) == sizeof(UINT));
    *puResult = (UINT)ulOperand;    
    return S_OK;
}

//
// ULONG -> UINT32 conversion
//
#define ULongToUInt32   ULongToUInt

//
// ULONG -> UINT_PTR conversion
//
#ifdef _WIN64
_Must_inspect_result_
__inline
HRESULT
ULongToUIntPtr(
    _In_ ULONG ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) UINT_PTR* puiResult)
{
    C_ASSERT(sizeof(UINT_PTR) > sizeof(ULONG));
    *puiResult = (UINT_PTR)ulOperand;
    return S_OK;
}
#else
#define ULongToUIntPtr  ULongToUInt
#endif

//
// ULONG -> LONG conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongToLong(
    _In_ ULONG ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) LONG* plResult)
{
    HRESULT hr;
    
    if (ulOperand <= LONG_MAX)
    {
        *plResult = (LONG)ulOperand;
        hr = S_OK;
    }
    else
    {
        *plResult = LONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONG -> LONG_PTR conversion
//
#ifdef _WIN64
_Must_inspect_result_
__inline
HRESULT
ULongToLongPtr(
    _In_ ULONG ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) LONG_PTR* plResult)
{
    C_ASSERT(sizeof(LONG_PTR) > sizeof(ULONG));
    *plResult = (LONG_PTR)ulOperand;
    return S_OK;
}
#else
#define ULongToLongPtr  ULongToLong
#endif

//
// ULONG -> ptrdiff_t conversion
//
#define ULongToPtrdiffT ULongToIntPtr

//
// ULONG -> SSIZE_T conversion
//
#define ULongToSSIZET   ULongToLongPtr

//
// ULONG_PTR -> INT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongPtrToInt8(
    _In_ ULONG_PTR ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) INT8* pi8Result)
{
    HRESULT hr;
    
    if (ulOperand <= INT8_MAX)
    {
        *pi8Result = (INT8)ulOperand;
        hr = S_OK;
    }
    else
    {
        *pi8Result = INT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONG_PTR -> UCHAR conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongPtrToUChar(
    _In_ ULONG_PTR ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) UCHAR* pch)
{
    HRESULT hr;

    if (ulOperand <= 255)
    {
        *pch = (UCHAR)ulOperand;
        hr = S_OK;
    }
    else
    {
        *pch = '\0';
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// ULONG_PTR -> CHAR conversion
//
__forceinline
HRESULT
ULongPtrToChar(
    _In_ ULONG_PTR ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) CHAR* pch)
{
#ifdef _CHAR_UNSIGNED
    return ULongPtrToUChar(ulOperand, (UCHAR*)pch);
#else
    return ULongPtrToInt8(ulOperand, (INT8*)pch);
#endif
}

//
// ULONG_PTR -> UINT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongPtrToUInt8(
    _In_ ULONG_PTR ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) UINT8* pui8Result)
{
    HRESULT hr;
    
    if (ulOperand <= UINT8_MAX)
    {
        *pui8Result = (UINT8)ulOperand;
        hr = S_OK;
    }
    else
    {
        *pui8Result = UINT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//    
// ULONG_PTR -> BYTE conversion
//
#define ULongPtrToByte  ULongPtrToUInt8

//
// ULONG_PTR -> SHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongPtrToShort(
    _In_ ULONG_PTR ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) SHORT* psResult)
{
    HRESULT hr;

    if (ulOperand <= SHORT_MAX)
    {
        *psResult = (SHORT)ulOperand;
        hr = S_OK;
    }
    else
    {
        *psResult = SHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr; 
}

//
// ULONG_PTR -> INT16 conversion
//
#define ULongPtrToInt16 ULongPtrToShort

//
// ULONG_PTR -> USHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongPtrToUShort(
    _In_ ULONG_PTR ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) USHORT* pusResult)
{
    HRESULT hr;

    if (ulOperand <= USHORT_MAX)
    {
        *pusResult = (USHORT)ulOperand;
        hr = S_OK;
    }
    else
    {
        *pusResult = USHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// ULONG_PTR -> UINT16 conversion
//
#define ULongPtrToUInt16    ULongPtrToUShort

//
// ULONG_PTR -> WORD conversion
//
#define ULongPtrToWord  ULongPtrToUShort

//
// ULONG_PTR -> INT conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongPtrToInt(
    _In_ ULONG_PTR ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) INT* piResult)
{
    HRESULT hr;
    
    if (ulOperand <= INT_MAX)
    {
        *piResult = (INT)ulOperand;
        hr = S_OK;
    }
    else
    {
        *piResult = INT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONG_PTR -> INT32 conversion
//
#define ULongPtrToInt32 ULongPtrToInt

//
// ULONG_PTR -> INT_PTR conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongPtrToIntPtr(
    _In_ ULONG_PTR ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) INT_PTR* piResult)
{
    HRESULT hr;
    
    if (ulOperand <= INT_PTR_MAX)
    {
        *piResult = (INT_PTR)ulOperand;
        hr = S_OK;
    }
    else
    {
        *piResult = INT_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONG_PTR -> UINT conversion
//
#ifdef _WIN64
#define ULongPtrToUInt  ULongLongToUInt
#else
_Must_inspect_result_
__inline
HRESULT
ULongPtrToUInt(
    _In_ ULONG_PTR ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) UINT* puResult)
{
    C_ASSERT(sizeof(ULONG_PTR) == sizeof(UINT));
    *puResult = (UINT)ulOperand;    
    return S_OK;
}
#endif

//
// ULONG_PTR -> UINT32 conversion
//
#define ULongPtrToUInt32    ULongPtrToUInt

//
// ULONG_PTR -> UINT_PTR conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongPtrToUIntPtr(
    _In_ ULONG_PTR ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) UINT_PTR* puResult)
{
    *puResult = (UINT_PTR)ulOperand;
    return S_OK;
}

//
// ULONG_PTR -> LONG conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongPtrToLong(
    _In_ ULONG_PTR ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) LONG* plResult)
{
    HRESULT hr;
    
    if (ulOperand <= LONG_MAX)
    {
        *plResult = (LONG)ulOperand;
        hr = S_OK;
    }
    else
    {
        *plResult = LONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//        
// ULONG_PTR -> LONG_PTR conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongPtrToLongPtr(
    _In_ ULONG_PTR ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) LONG_PTR* plResult)
{
    HRESULT hr;
    
    if (ulOperand <= LONG_PTR_MAX)
    {
        *plResult = (LONG_PTR)ulOperand;
        hr = S_OK;
    }
    else
    {
        *plResult = LONG_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONG_PTR -> ULONG conversion
//
#ifdef _WIN64
#define ULongPtrToULong ULongLongToULong
#else
_Must_inspect_result_
__inline
HRESULT
ULongPtrToULong(
    _In_ ULONG_PTR ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) ULONG* pulResult)
{
    *pulResult = (ULONG)ulOperand;
    return S_OK;
}
#endif    

//
// ULONG_PTR -> DWORD conversion
//
#define ULongPtrToDWord ULongPtrToULong

//
// ULONG_PTR -> LONGLONG conversion
//
#ifdef _WIN64
#define ULongPtrToLongLong  ULongLongToLongLong
#else
_Must_inspect_result_
__inline
HRESULT
ULongPtrToLongLong(
    _In_ ULONG_PTR ulOperand,
    _Out_ _Deref_out_range_(==, ulOperand) LONGLONG* pllResult)
{
    *pllResult = (LONGLONG)ulOperand;
    return S_OK;
}
#endif

//
// ULONG_PTR -> LONG64 conversion
//
#define ULongPtrToLong64    ULongPtrToLongLong

//
// ULONG_PTR -> INT64
//
#define ULongPtrToInt64 ULongPtrToLongLong

//
// ULONG_PTR -> ptrdiff_t conversion
//
#define ULongPtrToPtrdiffT  ULongPtrToIntPtr

//
// ULONG_PTR -> SSIZE_T conversion
//
#define ULongPtrToSSIZET    ULongPtrToLongPtr

//
// DWORD -> INT8 conversion
//
#define DWordToInt8 ULongToInt8

//
// DWORD -> CHAR conversion
//
#define DWordToChar ULongToChar

//
// DWORD -> UCHAR conversion
//
#define DWordToUChar    ULongToUChar

//
// DWORD -> UINT8 conversion
//
#define DWordToUInt8    ULongToUInt8

//
// DWORD -> BYTE conversion
//
#define DWordToByte ULongToUInt8

//
// DWORD -> SHORT conversion
//
#define DWordToShort    ULongToShort

//
// DWORD -> INT16 conversion
//
#define DWordToInt16    ULongToShort

//
// DWORD -> USHORT conversion
//
#define DWordToUShort   ULongToUShort

//
// DWORD -> UINT16 conversion
//
#define DWordToUInt16   ULongToUShort

//
// DWORD -> WORD conversion
//
#define DWordToWord ULongToUShort

//
// DWORD -> INT conversion
//
#define DWordToInt  ULongToInt

//
// DWORD -> INT32 conversion
//
#define DWordToInt32    ULongToInt

//
// DWORD -> INT_PTR conversion
//
#define DWordToIntPtr   ULongToIntPtr

//
// DWORD -> UINT conversion
//
#define DWordToUInt ULongToUInt

//
// DWORD -> UINT32 conversion
//
#define DWordToUInt32   ULongToUInt

//
// DWORD -> UINT_PTR conversion
//
#define DWordToUIntPtr  ULongToUIntPtr

//
// DWORD -> LONG conversion
//
#define DWordToLong ULongToLong

//
// DWORD -> LONG_PTR conversion
//
#define DWordToLongPtr  ULongToLongPtr

//
// DWORD -> ptrdiff_t conversion
//
#define DWordToPtrdiffT ULongToIntPtr

//
// DWORD -> SSIZE_T conversion
//
#define DWordToSSIZET   ULongToLongPtr

//
// DWORD_PTR -> INT8 conversion
//
#define DWordPtrToInt8  ULongPtrToInt8

//
// DWORD_PTR -> UCHAR conversion
//
#define DWordPtrToUChar ULongPtrToUChar

//
// DWORD_PTR -> CHAR conversion
//
#define DWordPtrToChar  ULongPtrToChar

//
// DWORD_PTR -> UINT8 conversion
//
#define DWordPtrToUInt8 ULongPtrToUInt8

//
// DWORD_PTR -> BYTE conversion
//
#define DWordPtrToByte  ULongPtrToUInt8

//
// DWORD_PTR -> SHORT conversion
//
#define DWordPtrToShort ULongPtrToShort

//
// DWORD_PTR -> INT16 conversion
//
#define DWordPtrToInt16 ULongPtrToShort

//
// DWORD_PTR -> USHORT conversion
//
#define DWordPtrToUShort    ULongPtrToUShort

//
// DWORD_PTR -> UINT16 conversion
//
#define DWordPtrToUInt16    ULongPtrToUShort

//
// DWORD_PTR -> WORD conversion
//
#define DWordPtrToWord  ULongPtrToUShort

//
// DWORD_PTR -> INT conversion
//
#define DWordPtrToInt   ULongPtrToInt

//
// DWORD_PTR -> INT32 conversion
//
#define DWordPtrToInt32 ULongPtrToInt

//
// DWORD_PTR -> INT_PTR conversion
//
#define DWordPtrToIntPtr    ULongPtrToIntPtr

//
// DWORD_PTR -> UINT conversion
//
#define DWordPtrToUInt  ULongPtrToUInt

//
// DWORD_PTR -> UINT32 conversion
//
#define DWordPtrToUInt32    ULongPtrToUInt

//
// DWODR_PTR -> UINT_PTR conversion
//
#define DWordPtrToUIntPtr   ULongPtrToUIntPtr

//
// DWORD_PTR -> LONG conversion
//
#define DWordPtrToLong  ULongPtrToLong

//
// DWORD_PTR -> LONG_PTR conversion
//
#define DWordPtrToLongPtr   ULongPtrToLongPtr

//
// DWORD_PTR -> ULONG conversion
//
#define DWordPtrToULong ULongPtrToULong

//
// DWORD_PTR -> DWORD conversion
//
#define DWordPtrToDWord ULongPtrToULong

//
// DWORD_PTR -> LONGLONG conversion
//
#define DWordPtrToLongLong  ULongPtrToLongLong

//
// DWORD_PTR -> LONG64 conversion
//
#define DWordPtrToLong64    ULongPtrToLongLong

//
// DWORD_PTR -> INT64 conversion
//
#define DWordPtrToInt64 ULongPtrToLongLong

//
// DWORD_PTR -> ptrdiff_t conversion
//
#define DWordPtrToPtrdiffT  ULongPtrToIntPtr

//
// DWORD_PTR -> SSIZE_T conversion
//
#define DWordPtrToSSIZET    ULongPtrToLongPtr

//
// LONGLONG -> INT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
LongLongToInt8(
    _In_ LONGLONG llOperand,
    _Out_ _Deref_out_range_(==, llOperand) INT8* pi8Result)
{
    HRESULT hr;
    
    if ((llOperand >= INT8_MIN) && (llOperand <= INT8_MAX))
    {
        *pi8Result = (INT8)llOperand;
        hr = S_OK;
    }
    else
    {
        *pi8Result = INT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }

    return hr;
}

//
// LONGLONG -> UCHAR conversion
//
_Must_inspect_result_
__inline
HRESULT
LongLongToUChar(
    _In_ LONGLONG llOperand,
    _Out_ _Deref_out_range_(==, llOperand) UCHAR* pch)
{
    HRESULT hr;

    if ((llOperand >= 0) && (llOperand <= 255))
    {
        *pch = (UCHAR)llOperand;
        hr = S_OK;
    }
    else
    {
        *pch = '\0';
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONGLONG -> CHAR conversion
//
__forceinline
HRESULT
LongLongToChar(
    _In_ LONGLONG llOperand,
    _Out_ _Deref_out_range_(==, llOperand) CHAR* pch)
{
#ifdef _CHAR_UNSIGNED
    return LongLongToUChar(llOperand, (UCHAR*)pch);
#else
    return LongLongToInt8(llOperand, (INT8*)pch);
#endif
}

//
// LONGLONG -> UINT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
LongLongToUInt8(
    _In_ LONGLONG llOperand,
    _Out_ _Deref_out_range_(==, llOperand) UINT8* pu8Result)
{
    HRESULT hr;
    
    if ((llOperand >= 0) && (llOperand <= UINT8_MAX))
    {
        *pu8Result = (UINT8)llOperand;
        hr = S_OK;
    }
    else
    {
        *pu8Result = UINT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONGLONG -> BYTE conversion
//
#define LongLongToByte  LongLongToUInt8

//
// LONGLONG -> SHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
LongLongToShort(
    _In_ LONGLONG llOperand,
    _Out_ _Deref_out_range_(==, llOperand) SHORT* psResult)
{
    HRESULT hr;
    
    if ((llOperand >= SHORT_MIN) && (llOperand <= SHORT_MAX))
    {
        *psResult = (SHORT)llOperand;
        hr = S_OK;
    }
    else
    {
        *psResult = SHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONGLONG -> INT16 conversion
//
#define LongLongToInt16 LongLongToShort

//
// LONGLONG -> USHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
LongLongToUShort(
    _In_ LONGLONG llOperand,
    _Out_ _Deref_out_range_(==, llOperand) USHORT* pusResult)
{
    HRESULT hr;
    
    if ((llOperand >= 0) && (llOperand <= USHORT_MAX))
    {
        *pusResult = (USHORT)llOperand;
        hr = S_OK;
    }
    else
    {
        *pusResult = USHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONGLONG -> UINT16 conversion
//
#define LongLongToUInt16    LongLongToUShort

//
// LONGLONG -> WORD conversion
//
#define LongLongToWord  LongLongToUShort

//
// LONGLONG -> INT conversion
//
_Must_inspect_result_
__inline
HRESULT
LongLongToInt(
    _In_ LONGLONG llOperand,
    _Out_ _Deref_out_range_(==, llOperand) INT* piResult)
{
    HRESULT hr;
    
    if ((llOperand >= INT_MIN) && (llOperand <= INT_MAX))
    {
        *piResult = (INT)llOperand;
        hr = S_OK;
    }
    else
    {
        *piResult = INT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// LONGLONG -> INT32 conversion
//
#define LongLongToInt32 LongLongToInt

//
// LONGLONG -> INT_PTR conversion
//
#ifdef _WIN64
_Must_inspect_result_
__inline
HRESULT
LongLongToIntPtr(
    _In_ LONGLONG llOperand,
    _Out_ _Deref_out_range_(==, llOperand) INT_PTR* piResult)
{
    *piResult = llOperand;
    return S_OK;
}
#else
#define LongLongToIntPtr   LongLongToInt
#endif

//
// LONGLONG -> UINT conversion
//
_Must_inspect_result_
__inline
HRESULT
LongLongToUInt(
    _In_ LONGLONG llOperand,
    _Out_ _Deref_out_range_(==, llOperand) UINT* puResult)
{
    HRESULT hr;
    
    if ((llOperand >= 0) && (llOperand <= UINT_MAX))
    {
        *puResult = (UINT)llOperand;
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;    
}

//
// LONGLONG -> UINT32 conversion
//
#define LongLongToUInt32    LongLongToUInt

//
// LONGLONG -> UINT_PTR conversion
//
#ifdef _WIN64
#define LongLongToUIntPtr  LongLongToULongLong
#else
#define LongLongToUIntPtr  LongLongToUInt
#endif

//
// LONGLONG -> LONG conversion
//
_Must_inspect_result_
__inline
HRESULT
LongLongToLong(
    _In_ LONGLONG llOperand,
    _Out_ _Deref_out_range_(==, llOperand) LONG* plResult)
{
    HRESULT hr;
    
    if ((llOperand >= LONG_MIN) && (llOperand <= LONG_MAX))
    {
        *plResult = (LONG)llOperand;
        hr = S_OK;
    }
    else
    {
        *plResult = LONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;    
}

//
// LONGLONG -> LONG_PTR conversion
//
#ifdef _WIN64
_Must_inspect_result_
__inline
HRESULT
LongLongToLongPtr(
    _In_ LONGLONG llOperand,
    _Out_ _Deref_out_range_(==, llOperand) LONG_PTR* plResult)
{
    *plResult = (LONG_PTR)llOperand;
    return S_OK;
}    
#else
#define LongLongToLongPtr  LongLongToLong
#endif

//
// LONGLONG -> ULONG conversion
//
_Must_inspect_result_
__inline
HRESULT
LongLongToULong(
    _In_ LONGLONG llOperand,
    _Out_ _Deref_out_range_(==, llOperand) ULONG* pulResult)
{
    HRESULT hr;
    
    if ((llOperand >= 0) && (llOperand <= ULONG_MAX))
    {
        *pulResult = (ULONG)llOperand;
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;    
}

//
// LONGLONG -> ULONG_PTR conversion
//
#ifdef _WIN64
#define LongLongToULongPtr LongLongToULongLong
#else
#define LongLongToULongPtr LongLongToULong
#endif

//
// LONGLONG -> DWORD conversion
//
#define LongLongToDWord    LongLongToULong

//
// LONGLONG -> DWORD_PTR conversion
//
#define LongLongToDWordPtr LongLongToULongPtr

//
// LONGLONG -> ULONGLONG conversion
//
_Must_inspect_result_
__inline
HRESULT
LongLongToULongLong(
    _In_ LONGLONG llOperand,
    _Out_ _Deref_out_range_(==, llOperand) ULONGLONG* pullResult)
{
    HRESULT hr;
    
    if (llOperand >= 0)
    {
        *pullResult = (ULONGLONG)llOperand;
        hr = S_OK;
    }
    else
    {
        *pullResult = ULONGLONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr; 
}

//
// LONGLONG -> DWORDLONG conversion
//
#define LongLongToDWordLong LongLongToULongLong

//
// LONGLONG -> ULONG64 conversion
//
#define LongLongToULong64   LongLongToULongLong

//
// LONGLONG -> DWORD64 conversion
//
#define LongLongToDWord64   LongLongToULongLong

//
// LONGLONG -> UINT64 conversion
//
#define LongLongToUInt64    LongLongToULongLong

//
// LONGLONG -> ptrdiff_t conversion
//
#define LongLongToPtrdiffT LongLongToIntPtr

//
// LONGLONG -> size_t conversion
//
#define LongLongToSizeT    LongLongToUIntPtr

//
// LONGLONG -> SSIZE_T conversion
//
#define LongLongToSSIZET   LongLongToLongPtr

//
// LONGLONG -> SIZE_T conversion
//
#define LongLongToSIZET    LongLongToULongPtr

//
// LONG64 -> CHAR conversion
//
#define Long64ToChar    LongLongToChar

//
// LONG64 -> INT8 conversion
//
#define Long64ToInt8    LongLongToInt8

//
// LONG64 -> UCHAR conversion
//
#define Long64ToUChar   LongLongToUChar

//
// LONG64 -> UINT8 conversion
//
#define Long64ToUInt8   LongLongToUInt8

//
// LONG64 -> BYTE conversion
//
#define Long64ToByte    LongLongToUInt8

//
// LONG64 -> SHORT conversion
//
#define Long64ToShort   LongLongToShort

//
// LONG64 -> INT16 conversion
//
#define Long64ToInt16   LongLongToShort

//
// LONG64 -> USHORT conversion
//
#define Long64ToUShort  LongLongToUShort

//
// LONG64 -> UINT16 conversion
//
#define Long64ToUInt16  LongLongToUShort

//
// LONG64 -> WORD conversion
//
#define Long64ToWord    LongLongToUShort

//
// LONG64 -> INT conversion
//
#define Long64ToInt LongLongToInt

//
// LONG64 -> INT32 conversion
//
#define Long64ToInt32   LongLongToInt

//
// LONG64 -> INT_PTR conversion
//
#define Long64ToIntPtr  LongLongToIntPtr

//
// LONG64 -> UINT conversion
//
#define Long64ToUInt    LongLongToUInt

//
// LONG64 -> UINT32 conversion
//
#define Long64ToUInt32  LongLongToUInt

//
// LONG64 -> UINT_PTR conversion
//
#define Long64ToUIntPtr LongLongToUIntPtr

//
// LONG64 -> LONG conversion
//
#define Long64ToLong    LongLongToLong

//
// LONG64 -> LONG_PTR conversion
//
#define Long64ToLongPtr LongLongToLongPtr

//
// LONG64 -> ULONG conversion
//
#define Long64ToULong   LongLongToULong

//
// LONG64 -> ULONG_PTR conversion
//
#define Long64ToULongPtr    LongLongToULongPtr

//
// LONG64 -> DWORD conversion
//
#define Long64ToDWord   LongLongToULong

//
// LONG64 -> DWORD_PTR conversion
//
#define Long64ToDWordPtr    LongLongToULongPtr  

//
// LONG64 -> ULONGLONG conversion
//
#define Long64ToULongLong   LongLongToULongLong

//
// LONG64 -> ptrdiff_t conversion
//
#define Long64ToPtrdiffT    LongLongToIntPtr

//
// LONG64 -> size_t conversion
//
#define Long64ToSizeT   LongLongToUIntPtr

//
// LONG64 -> SSIZE_T conversion
//
#define Long64ToSSIZET  LongLongToLongPtr

//
// LONG64 -> SIZE_T conversion
//
#define Long64ToSIZET   LongLongToULongPtr

//
// INT64 -> CHAR conversion
//
#define Int64ToChar LongLongToChar

//
// INT64 -> INT8 conversion
//
#define Int64ToInt8 LongLongToInt8

//
// INT64 -> UCHAR conversion
//
#define Int64ToUChar    LongLongToUChar

//
// INT64 -> UINT8 conversion
//
#define Int64ToUInt8    LongLongToUInt8

//
// INT64 -> BYTE conversion
//
#define Int64ToByte LongLongToUInt8

//
// INT64 -> SHORT conversion
//
#define Int64ToShort    LongLongToShort

//
// INT64 -> INT16 conversion
//
#define Int64ToInt16    LongLongToShort

//
// INT64 -> USHORT conversion
//
#define Int64ToUShort   LongLongToUShort

//
// INT64 -> UINT16 conversion
//
#define Int64ToUInt16   LongLongToUShort

//
// INT64 -> WORD conversion
//
#define Int64ToWord LongLongToUShort

//
// INT64 -> INT conversion
//
#define Int64ToInt  LongLongToInt

//
// INT64 -> INT32 conversion
//
#define Int64ToInt32    LongLongToInt

//
// INT64 -> INT_PTR conversion
//
#define Int64ToIntPtr   LongLongToIntPtr

//
// INT64 -> UINT conversion
//
#define Int64ToUInt LongLongToUInt

//
// INT64 -> UINT32 conversion
//
#define Int64ToUInt32   LongLongToUInt

//
// INT64 -> UINT_PTR conversion
//
#define Int64ToUIntPtr  LongLongToUIntPtr

//
// INT64 -> LONG conversion
//
#define Int64ToLong LongLongToLong

//
// INT64 -> LONG_PTR conversion
//
#define Int64ToLongPtr  LongLongToLongPtr

//
// INT64 -> ULONG conversion
//
#define Int64ToULong    LongLongToULong

//
// INT64 -> ULONG_PTR conversion
//
#define Int64ToULongPtr LongLongToULongPtr

//
// INT64 -> DWORD conversion
//
#define Int64ToDWord    LongLongToULong

//
// INT64 -> DWORD_PTR conversion
//
#define Int64ToDWordPtr LongLongToULongPtr

//
// INT64 -> ULONGLONG conversion
//
#define Int64ToULongLong    LongLongToULongLong

//
// INT64 -> DWORDLONG conversion
//
#define Int64ToDWordLong    LongLongToULongLong

//
// INT64 -> ULONG64 conversion
//
#define Int64ToULong64  LongLongToULongLong

//
// INT64 -> DWORD64 conversion
//
#define Int64ToDWord64  LongLongToULongLong

//
// INT64 -> UINT64 conversion
//
#define Int64ToUInt64   LongLongToULongLong

//
// INT64 -> ptrdiff_t conversion
//
#define Int64ToPtrdiffT LongLongToIntPtr

//
// INT64 -> size_t conversion
//
#define Int64ToSizeT    LongLongToUIntPtr

//
// INT64 -> SSIZE_T conversion
//
#define Int64ToSSIZET   LongLongToLongPtr

//
// INT64 -> SIZE_T conversion
//
#define Int64ToSIZET    LongLongToULongPtr

//
// ULONGLONG -> INT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongLongToInt8(
    _In_ ULONGLONG ullOperand,
    _Out_ _Deref_out_range_(==, ullOperand) INT8* pi8Result)
{
    HRESULT hr;
    
    if (ullOperand <= INT8_MAX)
    {
        *pi8Result = (INT8)ullOperand;
        hr = S_OK;
    }
    else
    {
        *pi8Result = INT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONGLONG -> UCHAR conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongLongToUChar(
    _In_ ULONGLONG ullOperand,
    _Out_ _Deref_out_range_(==, ullOperand) UCHAR* pch)
{
    HRESULT hr;
    
    if (ullOperand <= 255)
    {
        *pch = (UCHAR)ullOperand;
        hr = S_OK;
    }
    else
    {
        *pch = '\0';
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONGLONG -> CHAR conversion
//
__forceinline
HRESULT
ULongLongToChar(
    _In_ ULONGLONG ullOperand,
    _Out_ _Deref_out_range_(==, ullOperand) CHAR* pch)
{
#ifdef _CHAR_UNSIGNED
    return ULongLongToUChar(ullOperand, (UCHAR*)pch);
#else
    return ULongLongToInt8(ullOperand, (INT8*)pch);
#endif
}

//
// ULONGLONG -> UINT8 conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongLongToUInt8(
    _In_ ULONGLONG ullOperand,
    _Out_ _Deref_out_range_(==, ullOperand) UINT8* pu8Result)
{
    HRESULT hr;
    
    if (ullOperand <= UINT8_MAX)
    {
        *pu8Result = (UINT8)ullOperand;
        hr = S_OK;
    }
    else
    {
        *pu8Result = UINT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONGLONG -> BYTE conversion
//
#define ULongLongToByte ULongLongToUInt8

//
// ULONGLONG -> SHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongLongToShort(
    _In_ ULONGLONG ullOperand,
    _Out_ _Deref_out_range_(==, ullOperand) SHORT* psResult)
{
    HRESULT hr;
    
    if (ullOperand <= SHORT_MAX)
    {
        *psResult = (SHORT)ullOperand;
        hr = S_OK;
    }
    else
    {
        *psResult = SHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONGLONG -> INT16 conversion
//
#define ULongLongToInt16    ULongLongToShort

//
// ULONGLONG -> USHORT conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongLongToUShort(
    _In_ ULONGLONG ullOperand,
    _Out_ _Deref_out_range_(==, ullOperand) USHORT* pusResult)
{
    HRESULT hr;
    
    if (ullOperand <= USHORT_MAX)
    {
        *pusResult = (USHORT)ullOperand;
        hr = S_OK;
    }
    else
    {
        *pusResult = USHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONGLONG -> UINT16 conversion
//
#define ULongLongToUInt16   ULongLongToUShort

//
// ULONGLONG -> WORD conversion
//
#define ULongLongToWord ULongLongToUShort

//
// ULONGLONG -> INT conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongLongToInt(
    _In_ ULONGLONG ullOperand,
    _Out_ _Deref_out_range_(==, ullOperand) INT* piResult)
{
    HRESULT hr;
    
    if (ullOperand <= INT_MAX)
    {
        *piResult = (INT)ullOperand;
        hr = S_OK;
    }
    else
    {
        *piResult = INT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONGLONG -> INT32 conversion
//
#define ULongLongToInt32    ULongLongToInt

//
// ULONGLONG -> INT_PTR conversion
//
#ifdef _WIN64
#define ULongLongToIntPtr   ULongLongToLongLong
#else
#define ULongLongToIntPtr   ULongLongToInt
#endif

//
// ULONGLONG -> UINT conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongLongToUInt(
    _In_ ULONGLONG ullOperand,
    _Out_ _Deref_out_range_(==, ullOperand) UINT* puResult)
{
    HRESULT hr;
    
    if (ullOperand <= UINT_MAX)
    {
        *puResult = (UINT)ullOperand;
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONGLONG -> UINT32 conversion
//
#define ULongLongToUInt32   ULongLongToUInt

//
// ULONGLONG -> UINT_PTR conversion
//
#ifdef _WIN64
_Must_inspect_result_
__inline
HRESULT
ULongLongToUIntPtr(
    _In_ ULONGLONG ullOperand,
    _Out_ _Deref_out_range_(==, ullOperand) UINT_PTR* puResult)
{
    *puResult = ullOperand;
    return S_OK;
}
#else    
#define ULongLongToUIntPtr  ULongLongToUInt
#endif

//
// ULONGLONG -> LONG conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongLongToLong(
    _In_ ULONGLONG ullOperand,
    _Out_ _Deref_out_range_(==, ullOperand) LONG* plResult)
{
    HRESULT hr;
    
    if (ullOperand <= LONG_MAX)
    {
        *plResult = (LONG)ullOperand;
        hr = S_OK;
    }
    else
    {
        *plResult = LONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONGLONG -> LONG_PTR conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongLongToLongPtr(
    _In_ ULONGLONG ullOperand,
    _Out_ _Deref_out_range_(==, ullOperand) LONG_PTR* plResult)
{
    HRESULT hr;
    
    if (ullOperand <= LONG_PTR_MAX)
    {
        *plResult = (LONG_PTR)ullOperand;
        hr = S_OK;
    }
    else
    {
        *plResult = LONG_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONGLONG -> ULONG conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongLongToULong(
    _In_ ULONGLONG ullOperand,
    _Out_ _Deref_out_range_(==, ullOperand) ULONG* pulResult)
{
    HRESULT hr;
    
    if (ullOperand <= ULONG_MAX)
    {
        *pulResult = (ULONG)ullOperand;
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONGLONG -> ULONG_PTR conversion
//
#ifdef _WIN64
_Must_inspect_result_
__inline
HRESULT
ULongLongToULongPtr(
    _In_ ULONGLONG ullOperand,
    _Out_ _Deref_out_range_(==, ullOperand) ULONG_PTR* pulResult)
{
    *pulResult = ullOperand;
    return S_OK;
}
#else
#define ULongLongToULongPtr ULongLongToULong
#endif

//
// ULONGLONG -> DWORD conversion
//
#define ULongLongToDWord    ULongLongToULong

//
// ULONGLONG -> DWORD_PTR conversion
//
#define ULongLongToDWordPtr ULongLongToULongPtr

//
// ULONGLONG -> LONGLONG conversion
//
_Must_inspect_result_
__inline
HRESULT
ULongLongToLongLong(
    _In_ ULONGLONG ullOperand,
    _Out_ _Deref_out_range_(==, ullOperand) LONGLONG* pllResult)
{
    HRESULT hr;
    
    if (ullOperand <= LONGLONG_MAX)
    {
        *pllResult = (LONGLONG)ullOperand;
        hr = S_OK;
    }
    else
    {
        *pllResult = LONGLONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONGLONG -> INT64 conversion
//
#define ULongLongToInt64    ULongLongToLongLong

//
// ULONGLONG -> LONG64 conversion
//
#define ULongLongToLong64   ULongLongToLongLong

//
// ULONGLONG -> ptrdiff_t conversion
//
#define ULongLongToPtrdiffT ULongLongToIntPtr

//
// ULONGLONG -> size_t conversion
//
#define ULongLongToSizeT    ULongLongToUIntPtr

//
// ULONGLONG -> SSIZE_T conversion
//
#define ULongLongToSSIZET   ULongLongToLongPtr

//
// ULONGLONG -> SIZE_T conversion
//
#define ULongLongToSIZET    ULongLongToULongPtr

//
// DWORDLONG -> CHAR conversion
//
#define DWordLongToChar ULongLongToChar

//
// DWORDLONG -> INT8 conversion
//
#define DWordLongToInt8 ULongLongToInt8

//
// DWORDLONG -> UCHAR conversion
//
#define DWordLongToUChar    ULongLongToUChar

//
// DWORDLONG -> UINT8 conversion
//
#define DWordLongToUInt8    ULongLongToUInt8

//
// DWORDLONG -> BYTE conversion
//
#define DWordLongToByte ULongLongToUInt8

//
// DWORDLONG -> SHORT conversion
//
#define DWordLongToShort    ULongLongToShort

//
// DWORDLONG -> INT16 conversion
//
#define DWordLongToInt16    ULongLongToShort

//
// DWORDLONG -> USHORT conversion
//
#define DWordLongToUShort   ULongLongToUShort

//
// DWORDLONG -> UINT16 conversion
//
#define DWordLongToUInt16   ULongLongToUShort

//
// DWORDLONG -> WORD conversion
//
#define DWordLongToWord ULongLongToUShort

//
// DWORDLONG -> INT conversion
//
#define DWordLongToInt  ULongLongToInt

//
// DWORDLONG -> INT32 conversion
//
#define DWordLongToInt32    ULongLongToInt

//
// DWORDLONG -> INT_PTR conversion
//
#define DWordLongToIntPtr   ULongLongToIntPtr

//
// DWORDLONG -> UINT conversion
//
#define DWordLongToUInt ULongLongToUInt

//
// DWORDLONG -> UINT32 conversion
//
#define DWordLongToUInt32   ULongLongToUInt

//
// DWORDLONG -> UINT_PTR conversion
//
#define DWordLongToUIntPtr  ULongLongToUIntPtr

//
// DWORDLONG -> LONG conversion
//
#define DWordLongToLong ULongLongToLong

//
// DWORDLONG -> LONG_PTR conversion
//
#define DWordLongToLongPtr  ULongLongToLongPtr

//
// DWORDLONG -> ULONG conversion
//
#define DWordLongToULong    ULongLongToULong

//
// DWORDLONG -> ULONG_PTR conversion
//
#define DWordLongToULongPtr ULongLongToULongPtr

//
// DWORDLONG -> DWORD conversion
//
#define DWordLongToDWord    ULongLongToULong

//
// DWORDLONG -> DWORD_PTR conversion
//
#define DWordLongToDWordPtr ULongLongToULongPtr

//
// DWORDLONG -> LONGLONG conversion
//
#define DWordLongToLongLong ULongLongToLongLong

//
// DWORDLONG -> LONG64 conversion
//
#define DWordLongToLong64   ULongLongToLongLong

//
// DWORDLONG -> INT64 conversion
//
#define DWordLongToInt64    ULongLongToLongLong

//
// DWORDLONG -> ptrdiff_t conversion
//
#define DWordLongToPtrdiffT ULongLongToIntPtr

//
// DWORDLONG -> size_t conversion
//
#define DWordLongToSizeT    ULongLongToUIntPtr

//
// DWORDLONG -> SSIZE_T conversion
//
#define DWordLongToSSIZET   ULongLongToLongPtr

//
// DWORDLONG -> SIZE_T conversion
//
#define DWordLongToSIZET    ULongLongToULongPtr

//
// ULONG64 -> CHAR conversion
//
#define ULong64ToChar   ULongLongToChar

//
// ULONG64 -> INT8 conversion
//
#define ULong64ToInt8   ULongLongToInt8

//
// ULONG64 -> UCHAR conversion
//
#define ULong64ToUChar  ULongLongToUChar

//
// ULONG64 -> UINT8 conversion
//
#define ULong64ToUInt8  ULongLongToUInt8

//
// ULONG64 -> BYTE conversion
//
#define ULong64ToByte   ULongLongToUInt8

//
// ULONG64 -> SHORT conversion
//
#define ULong64ToShort  ULongLongToShort

//
// ULONG64 -> INT16 conversion
//
#define ULong64ToInt16  ULongLongToShort

//
// ULONG64 -> USHORT conversion
//
#define ULong64ToUShort ULongLongToUShort

//
// ULONG64 -> UINT16 conversion
//
#define ULong64ToUInt16 ULongLongToUShort

//
// ULONG64 -> WORD conversion
//
#define ULong64ToWord   ULongLongToUShort

//
// ULONG64 -> INT conversion
//
#define ULong64ToInt    ULongLongToInt

//
// ULONG64 -> INT32 conversion
//
#define ULong64ToInt32  ULongLongToInt

//
// ULONG64 -> INT_PTR conversion
//
#define ULong64ToIntPtr ULongLongToIntPtr

//
// ULONG64 -> UINT conversion
//
#define ULong64ToUInt   ULongLongToUInt

//
// ULONG64 -> UINT32 conversion
//
#define ULong64ToUInt32 ULongLongToUInt

//
// ULONG64 -> UINT_PTR conversion
//
#define ULong64ToUIntPtr    ULongLongToUIntPtr

//
// ULONG64 -> LONG conversion
//
#define ULong64ToLong   ULongLongToLong

//
// ULONG64 -> LONG_PTR conversion
//
#define ULong64ToLongPtr    ULongLongToLongPtr

//
// ULONG64 -> ULONG conversion
//
#define ULong64ToULong  ULongLongToULong

//
// ULONG64 -> ULONG_PTR conversion
//
#define ULong64ToULongPtr   ULongLongToULongPtr

//
// ULONG64 -> DWORD conversion
//
#define ULong64ToDWord  ULongLongToULong

//
// ULONG64 -> DWORD_PTR conversion
//
#define ULong64ToDWordPtr   ULongLongToULongPtr

//
// ULONG64 -> LONGLONG conversion
//
#define ULong64ToLongLong   ULongLongToLongLong

//
// ULONG64 -> LONG64 conversion
//
#define ULong64ToLong64 ULongLongToLongLong

//
// ULONG64 -> INT64 conversion
//
#define ULong64ToInt64  ULongLongToLongLong

//
// ULONG64 -> ptrdiff_t conversion
//
#define ULong64ToPtrdiffT   ULongLongToIntPtr

//
// ULONG64 -> size_t conversion
//
#define ULong64ToSizeT  ULongLongToUIntPtr

//
// ULONG64 -> SSIZE_T conversion
//
#define ULong64ToSSIZET ULongLongToLongPtr

//
// ULONG64 -> SIZE_T conversion
//
#define ULong64ToSIZET  ULongLongToULongPtr

//
// DWORD64 -> CHAR conversion
//
#define DWord64ToChar   ULongLongToChar

//
// DWORD64 -> INT8 conversion
//
#define DWord64ToInt8   ULongLongToInt8

//
// DWORD64 -> UCHAR conversion
//
#define DWord64ToUChar  ULongLongToUChar

//
// DWORD64 -> UINT8 conversion
//
#define DWord64ToUInt8  ULongLongToUInt8

//
// DWORD64 -> BYTE conversion
//
#define DWord64ToByte   ULongLongToUInt8

//
// DWORD64 -> SHORT conversion
//
#define DWord64ToShort  ULongLongToShort

//
// DWORD64 -> INT16 conversion
//
#define DWord64ToInt16  ULongLongToShort

//
// DWORD64 -> USHORT conversion
//
#define DWord64ToUShort ULongLongToUShort

//
// DWORD64 -> UINT16 conversion
//
#define DWord64ToUInt16 ULongLongToUShort

//
// DWORD64 -> WORD conversion
//
#define DWord64ToWord   ULongLongToUShort

//
// DWORD64 -> INT conversion
//
#define DWord64ToInt    ULongLongToInt

//
// DWORD64 -> INT32 conversion
//
#define DWord64ToInt32  ULongLongToInt

//
// DWORD64 -> INT_PTR conversion
//
#define DWord64ToIntPtr ULongLongToIntPtr

//
// DWORD64 -> UINT conversion
//
#define DWord64ToUInt   ULongLongToUInt

//
// DWORD64 -> UINT32 conversion
//
#define DWord64ToUInt32 ULongLongToUInt

//
// DWORD64 -> UINT_PTR conversion
//
#define DWord64ToUIntPtr    ULongLongToUIntPtr

//
// DWORD64 -> LONG conversion
//
#define DWord64ToLong   ULongLongToLong

//
// DWORD64 -> LONG_PTR conversion
//
#define DWord64ToLongPtr    ULongLongToLongPtr

//
// DWORD64 -> ULONG conversion
//
#define DWord64ToULong  ULongLongToULong

//
// DWORD64 -> ULONG_PTR conversion
//
#define DWord64ToULongPtr   ULongLongToULongPtr

//
// DWORD64 -> DWORD conversion
//
#define DWord64ToDWord  ULongLongToULong

//
// DWORD64 -> DWORD_PTR conversion
//
#define DWord64ToDWordPtr   ULongLongToULongPtr

//
// DWORD64 -> LONGLONG conversion
//
#define DWord64ToLongLong   ULongLongToLongLong

//
// DWORD64 -> LONG64 conversion
//
#define DWord64ToLong64 ULongLongToLongLong

//
// DWORD64 -> INT64 conversion
//
#define DWord64ToInt64  ULongLongToLongLong

//
// DWORD64 -> ptrdiff_t conversion
//
#define DWord64ToPtrdiffT   ULongLongToIntPtr

//
// DWORD64 -> size_t conversion
//
#define DWord64ToSizeT  ULongLongToUIntPtr

//
// DWORD64 -> SSIZE_T conversion
//
#define DWord64ToSSIZET ULongLongToLongPtr

//
// DWORD64 -> SIZE_T conversion
//
#define DWord64ToSIZET  ULongLongToULongPtr

//
// UINT64 -> CHAR conversion
//
#define UInt64ToChar    ULongLongToChar

//
// UINT64 -> INT8 conversion
//
#define UInt64ToInt8    ULongLongToInt8

//
// UINT64 -> UCHAR conversion
//
#define UInt64ToUChar   ULongLongToUChar

//
// UINT64 -> UINT8 conversion
//
#define UInt64ToUInt8   ULongLongToUInt8

//
// UINT64 -> BYTE conversion
//
#define UInt64ToByte    ULongLongToUInt8

//
// UINT64 -> SHORT conversion
//
#define UInt64ToShort   ULongLongToShort

//
// UINT64 -> INT16 conversion
//
//
#define UInt64ToInt16   ULongLongToShort

//
// UINT64 -> USHORT conversion
//
#define UInt64ToUShort  ULongLongToUShort

//
// UINT64 -> UINT16 conversion
//
#define UInt64ToUInt16  ULongLongToUShort

//
// UINT64 -> WORD conversion
//
#define UInt64ToWord    ULongLongToUShort

//
// UINT64 -> INT conversion
//
#define UInt64ToInt ULongLongToInt

//
// UINT64 -> INT32 conversion
//
#define UInt64ToInt32   ULongLongToInt

//
// UINT64 -> INT_PTR conversion
//
#define UInt64ToIntPtr  ULongLongToIntPtr

//
// UINT64 -> UINT conversion
//
#define UInt64ToUInt    ULongLongToUInt

//
// UINT64 -> UINT32 conversion
//
#define UInt64ToUInt32  ULongLongToUInt

//
// UINT64 -> UINT_PTR conversion
//
#define UInt64ToUIntPtr ULongLongToUIntPtr

//
// UINT64 -> LONG conversion
//
#define UInt64ToLong    ULongLongToLong

//
// UINT64 -> LONG_PTR conversion
//
#define UInt64ToLongPtr ULongLongToLongPtr

//
// UINT64 -> ULONG conversion
//
#define UInt64ToULong   ULongLongToULong

//
// UINT64 -> ULONG_PTR conversion
//
#define UInt64ToULongPtr    ULongLongToULongPtr

//
// UINT64 -> DWORD conversion
//
#define UInt64ToDWord   ULongLongToULong

//
// UINT64 -> DWORD_PTR conversion
//
#define UInt64ToDWordPtr    ULongLongToULongPtr

//
// UINT64 -> LONGLONG conversion
//
#define UInt64ToLongLong    ULongLongToLongLong

//
// UINT64 -> LONG64 conversion
//
#define UInt64ToLong64  ULongLongToLongLong

//
// UINT64 -> INT64 conversion
//
#define UInt64ToInt64   ULongLongToLongLong

//
// UINT64 -> ptrdiff_t conversion
//
#define UInt64ToPtrdiffT    ULongLongToIntPtr

//
// UINT64 -> size_t conversion
//
#define UInt64ToSizeT   ULongLongToUIntPtr

//
// UINT64 -> SSIZE_T conversion
//
#define UInt64ToSSIZET  ULongLongToLongPtr

//
// UINT64 -> SIZE_T conversion
//
#define UInt64ToSIZET  ULongLongToULongPtr

//
// ptrdiff_t -> CHAR conversion
//
#define PtrdiffTToChar  IntPtrToChar

//
// ptrdiff_t -> INT8 conversion
//
#define PtrdiffTToInt8  IntPtrToInt8

//
// ptrdiff_t -> UCHAR conversion
//
#define PtrdiffTToUChar IntPtrToUChar

//
// ptrdiff_t -> UINT8 conversion
//
#define PtrdiffTToUInt8 IntPtrToUInt8

//
// ptrdiff_t -> BYTE conversion
//
#define PtrdiffTToByte  IntPtrToUInt8

//
// ptrdiff_t -> SHORT conversion
//
#define PtrdiffTToShort IntPtrToShort

//
// ptrdiff_t -> INT16 conversion
//
#define PtrdiffTToInt16 IntPtrToShort

//
// ptrdiff_t -> USHORT conversion
//
#define PtrdiffTToUShort    IntPtrToUShort

//
// ptrdiff_t -> UINT16 conversion
//
#define PtrdiffTToUInt16    IntPtrToUShort

//
// ptrdiff_t -> WORD conversion
//
#define PtrdiffTToWord  IntPtrToUShort

//
// ptrdiff_t -> INT conversion
//
#define PtrdiffTToInt   IntPtrToInt

//
// ptrdiff_t -> INT32 conversion
//
#define PtrdiffTToInt32 IntPtrToInt

//
// ptrdiff_t -> UINT conversion
//
#define PtrdiffTToUInt  IntPtrToUInt

//
// ptrdiff_t -> UINT32 conversion
//
#define PtrdiffTToUInt32    IntPtrToUInt

//
// ptrdiff_t -> UINT_PTR conversion
//
#define PtrdiffTToUIntPtr   IntPtrToUIntPtr

//
// ptrdiff_t -> LONG conversion
//
#define PtrdiffTToLong  IntPtrToLong

//
// ptrdiff_t -> LONG_PTR conversion
//
#define PtrdiffTToLongPtr   IntPtrToLongPtr

//
// ptrdiff_t -> ULONG conversion
//
#define PtrdiffTToULong IntPtrToULong

//
// ptrdiff_t -> ULONG_PTR conversion
//
#define PtrdiffTToULongPtr  IntPtrToULongPtr

//
// ptrdiff_t -> DWORD conversion
//
#define PtrdiffTToDWord IntPtrToULong

//
// ptrdiff_t -> DWORD_PTR conversion
//
#define PtrdiffTToDWordPtr  IntPtrToULongPtr

//
// ptrdiff_t -> ULONGLONG conversion
//
#define PtrdiffTToULongLong IntPtrToULongLong

//
// ptrdiff_t -> DWORDLONG conversion
//
#define PtrdiffTToDWordLong IntPtrToULongLong

//
// ptrdiff_t -> ULONG64 conversion
//
#define PtrdiffTToULong64   IntPtrToULongLong

//
// ptrdiff_t -> DWORD64 conversion
//
#define PtrdiffTToDWord64   IntPtrToULongLong

//
// ptrdiff_t -> UINT64 conversion
//
#define PtrdiffTToUInt64    IntPtrToULongLong

//
// ptrdiff_t -> size_t conversion
//
#define PtrdiffTToSizeT IntPtrToUIntPtr

//
// ptrdiff_t -> SIZE_T conversion
//
#define PtrdiffTToSIZET IntPtrToULongPtr

//
// size_t -> INT8 conversion
//
#define SizeTToInt8 UIntPtrToInt8

//
// size_t -> UCHAR conversion
//
#define SizeTToUChar    UIntPtrToUChar

//
// size_t -> CHAR conversion
//
#define SizeTToChar UIntPtrToChar

//
// size_t -> UINT8 conversion
//
#define SizeTToUInt8    UIntPtrToUInt8

//
// size_t -> BYTE conversion
//
#define SizeTToByte UIntPtrToUInt8

//
// size_t -> SHORT conversion
//
#define SizeTToShort    UIntPtrToShort

//
// size_t -> INT16 conversion
//
#define SizeTToInt16    UIntPtrToShort

//
// size_t -> USHORT conversion
//
#define SizeTToUShort   UIntPtrToUShort

//
// size_t -> UINT16 conversion
//
#define SizeTToUInt16   UIntPtrToUShort

//
// size_t -> WORD
//
#define SizeTToWord UIntPtrToUShort

//
// size_t -> INT conversion
//
#define SizeTToInt  UIntPtrToInt

//
// size_t -> INT32 conversion
//
#define SizeTToInt32    UIntPtrToInt

//
// size_t -> INT_PTR conversion
//
#define SizeTToIntPtr   UIntPtrToIntPtr

//
// size_t -> UINT conversion
//
#define SizeTToUInt UIntPtrToUInt

//
// size_t -> UINT32 conversion
//
#define SizeTToUInt32   UIntPtrToUInt

//
// size_t -> LONG conversion
//
#define SizeTToLong UIntPtrToLong

//
// size_t -> LONG_PTR conversion
//
#define SizeTToLongPtr  UIntPtrToLongPtr

//
// size_t -> ULONG conversion
//
#define SizeTToULong    UIntPtrToULong

//
// size_t -> DWORD conversion
//
#define SizeTToDWord    UIntPtrToULong

//
// size_t -> LONGLONG conversion
//
#define SizeTToLongLong UIntPtrToLongLong

//
// size_t -> LONG64 conversion
//
#define SizeTToLong64   UIntPtrToLongLong

//
// size_t -> INT64
//
#define SizeTToInt64    UIntPtrToLongLong

//   
// size_t -> ptrdiff_t conversion
//
#define SizeTToPtrdiffT UIntPtrToIntPtr

//
// size_t -> SSIZE_T conversion
//
#define SizeTToSSIZET   UIntPtrToLongPtr

//
// SSIZE_T -> INT8 conversion
//
#define SSIZETToInt8    LongPtrToInt8

//
// SSIZE_T -> UCHAR conversion
//
#define SSIZETToUChar   LongPtrToUChar

//
// SSIZE_T -> CHAR conversion
//
#define SSIZETToChar    LongPtrToChar

//
// SSIZE_T -> UINT8 conversion
//
#define SSIZETToUInt8   LongPtrToUInt8

//
// SSIZE_T -> BYTE conversion
//
#define SSIZETToByte    LongPtrToUInt8

//
// SSIZE_T -> SHORT conversion
//
#define SSIZETToShort   LongPtrToShort

//
// SSIZE_T -> INT16 conversion
//
#define SSIZETToInt16   LongPtrToShort

//
// SSIZE_T -> USHORT conversion
//
#define SSIZETToUShort  LongPtrToUShort

//
// SSIZE_T -> UINT16 conversion
//
#define SSIZETToUInt16  LongPtrToUShort

//
// SSIZE_T -> WORD conversion
//
#define SSIZETToWord    LongPtrToUShort

//
// SSIZE_T -> INT conversion
//
#define SSIZETToInt LongPtrToInt

//
// SSIZE_T -> INT32 conversion
//
#define SSIZETToInt32   LongPtrToInt

//
// SSIZE_T -> INT_PTR conversion
//
#define SSIZETToIntPtr  LongPtrToIntPtr

//
// SSIZE_T -> UINT conversion
//
#define SSIZETToUInt    LongPtrToUInt

//
// SSIZE_T -> UINT32 conversion
//
#define SSIZETToUInt32  LongPtrToUInt

//
// SSIZE_T -> UINT_PTR conversion
//
#define SSIZETToUIntPtr LongPtrToUIntPtr

//
// SSIZE_T -> LONG conversion
//
#define SSIZETToLong    LongPtrToLong

//
// SSIZE_T -> ULONG conversion
//
#define SSIZETToULong   LongPtrToULong

//
// SSIZE_T -> ULONG_PTR conversion
//
#define SSIZETToULongPtr    LongPtrToULongPtr

//
// SSIZE_T -> DWORD conversion
//
#define SSIZETToDWord   LongPtrToULong

//
// SSIZE_T -> DWORD_PTR conversion
//
#define SSIZETToDWordPtr    LongPtrToULongPtr

//
// SSIZE_T -> ULONGLONG conversion
//
#define SSIZETToULongLong   LongPtrToULongLong

//
// SSIZE_T -> DWORDLONG conversion
//
#define SSIZETToDWordLong   LongPtrToULongLong

//
// SSIZE_T -> ULONG64 conversion
//
#define SSIZETToULong64 LongPtrToULongLong

//
// SSIZE_T -> DWORD64 conversion
//
#define SSIZETToDWord64 LongPtrToULongLong

//
// SSIZE_T -> UINT64 conversion
//
#define SSIZETToUInt64  LongPtrToULongLong

//
// SSIZE_T -> size_t conversion
//
#define SSIZETToSizeT   LongPtrToUIntPtr

//
// SSIZE_T -> SIZE_T conversion
//
#define SSIZETToSIZET   LongPtrToULongPtr

//
// SIZE_T -> INT8 conversion
//
#define SIZETToInt8 ULongPtrToInt8

//
// SIZE_T -> UCHAR conversion
//
#define SIZETToUChar    ULongPtrToUChar

//
// SIZE_T -> CHAR conversion
//
#define SIZETToChar ULongPtrToChar

//
// SIZE_T -> UINT8 conversion
//
#define SIZETToUInt8    ULongPtrToUInt8

//
// SIZE_T -> BYTE conversion
//
#define SIZETToByte ULongPtrToUInt8

//
// SIZE_T -> SHORT conversion
//
#define SIZETToShort    ULongPtrToShort

//
// SIZE_T -> INT16 conversion
//
#define SIZETToInt16    ULongPtrToShort

//
// SIZE_T -> USHORT conversion
//
#define SIZETToUShort   ULongPtrToUShort

//
// SIZE_T -> UINT16 conversion
//
#define SIZETToUInt16   ULongPtrToUShort

//
// SIZE_T -> WORD
//
#define SIZETToWord ULongPtrToUShort

//
// SIZE_T -> INT conversion
//
#define SIZETToInt  ULongPtrToInt

//
// SIZE_T -> INT32 conversion
//
#define SIZETToInt32    ULongPtrToInt

//
// SIZE_T -> INT_PTR conversion
//
#define SIZETToIntPtr   ULongPtrToIntPtr

//
// SIZE_T -> UINT conversion
//
#define SIZETToUInt ULongPtrToUInt

//
// SIZE_T -> UINT32 conversion
//
#define SIZETToUInt32   ULongPtrToUInt

//
// SIZE_T -> UINT_PTR conversion
//
#define SIZETToUIntPtr  ULongPtrToUIntPtr

//
// SIZE_T -> LONG conversion
//
#define SIZETToLong ULongPtrToLong

//
// SIZE_T -> LONG_PTR conversion
//
#define SIZETToLongPtr  ULongPtrToLongPtr

//
// SIZE_T -> ULONG conversion
//
#define SIZETToULong    ULongPtrToULong

//
// SIZE_T -> DWORD conversion
//
#define SIZETToDWord    ULongPtrToULong

//
// SIZE_T -> LONGLONG conversion
//
#define SIZETToLongLong ULongPtrToLongLong

//
// SIZE_T -> LONG64 conversion
//
#define SIZETToLong64   ULongPtrToLongLong

//
// SIZE_T -> INT64
//
#define SIZETToInt64    ULongPtrToLongLong

//
// SIZE_T -> ptrdiff_t conversion
//
#define SIZETToPtrdiffT ULongPtrToIntPtr

//
// SIZE_T -> SSIZE_T conversion
//
#define SIZETToSSIZET   ULongPtrToLongPtr


//=============================================================================
// Addition functions
//=============================================================================

//
// UINT8 addition
//
_Must_inspect_result_
__inline
HRESULT
UInt8Add(
    _In_ UINT8 u8Augend,
    _In_ UINT8 u8Addend,
    _Out_ _Deref_out_range_(==, u8Augend + u8Addend) UINT8* pu8Result)
{
    HRESULT hr;

    if (((UINT8)(u8Augend + u8Addend)) >= u8Augend)
    {
        *pu8Result = (UINT8)(u8Augend + u8Addend);
        hr = S_OK;
    }
    else
    {
        *pu8Result = UINT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// USHORT addition
//
_Must_inspect_result_
__inline
HRESULT
UShortAdd(
    _In_ USHORT usAugend,
    _In_ USHORT usAddend,
    _Out_ _Deref_out_range_(==, usAugend + usAddend) USHORT* pusResult)
{
    HRESULT hr;

    if (((USHORT)(usAugend + usAddend)) >= usAugend)
    {
        *pusResult = (USHORT)(usAugend + usAddend);
        hr = S_OK;
    }
    else
    {
        *pusResult = USHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// UINT16 addition
//
#define UInt16Add   UShortAdd

//
// WORD addtition
//
#define WordAdd     UShortAdd

//
// UINT addition
//
_Must_inspect_result_
__inline
HRESULT
UIntAdd(
    _In_ UINT uAugend,
    _In_ UINT uAddend,
    _Out_ _Deref_out_range_(==, uAugend + uAddend) UINT* puResult)
{
    HRESULT hr;

    if ((uAugend + uAddend) >= uAugend)
    {
        *puResult = (uAugend + uAddend);
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// UINT32 addition
//
#define UInt32Add   UIntAdd

//
// UINT_PTR addition
//
#ifdef _WIN64
#define UIntPtrAdd      ULongLongAdd
#else
_Must_inspect_result_
__inline
HRESULT
UIntPtrAdd(
    _In_ UINT_PTR uAugend,
    _In_ UINT_PTR uAddend,
    _Out_ _Deref_out_range_(==, uAugend + uAddend) UINT_PTR* puResult)
{
    HRESULT hr;

    if ((uAugend + uAddend) >= uAugend)
    {
        *puResult = (uAugend + uAddend);
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}
#endif // _WIN64

//
// ULONG addition
//
_Must_inspect_result_
__inline
HRESULT
ULongAdd(
    _In_ ULONG ulAugend,
    _In_ ULONG ulAddend,
    _Out_ _Deref_out_range_(==, ulAugend + ulAddend) ULONG* pulResult)
{
    HRESULT hr;

    if ((ulAugend + ulAddend) >= ulAugend)
    {
        *pulResult = (ulAugend + ulAddend);
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONG_PTR addition
//
#ifdef _WIN64
#define ULongPtrAdd     ULongLongAdd
#else
_Must_inspect_result_
__inline
HRESULT
ULongPtrAdd(
    _In_ ULONG_PTR ulAugend,
    _In_ ULONG_PTR ulAddend,
    _Out_ _Deref_out_range_(==, ulAugend + ulAddend) ULONG_PTR* pulResult)
{
    HRESULT hr;

    if ((ulAugend + ulAddend) >= ulAugend)
    {
        *pulResult = (ulAugend + ulAddend);
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}
#endif // _WIN64

//
// DWORD addition
//
#define DWordAdd        ULongAdd

//
// DWORD_PTR addition
//
#ifdef _WIN64
#define DWordPtrAdd     ULongLongAdd
#else
_Must_inspect_result_
__inline
HRESULT
DWordPtrAdd(
    _In_ DWORD_PTR dwAugend,
    _In_ DWORD_PTR dwAddend,
    _Out_ _Deref_out_range_(==, dwAugend + dwAddend) DWORD_PTR* pdwResult)
{
    HRESULT hr;

    if ((dwAugend + dwAddend) >= dwAugend)
    {
        *pdwResult = (dwAugend + dwAddend);
        hr = S_OK;
    }
    else
    {
        *pdwResult = DWORD_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}
#endif // _WIN64

//
// size_t addition
//
_Must_inspect_result_
__inline
HRESULT
SizeTAdd(
    _In_ size_t Augend,
    _In_ size_t Addend,
    _Out_ _Deref_out_range_(==, Augend + Addend) size_t* pResult)
{
    HRESULT hr;

    if ((Augend + Addend) >= Augend)
    {
        *pResult = (Augend + Addend);
        hr = S_OK;
    }
    else
    {
        *pResult = SIZE_T_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// SIZE_T addition
//
#ifdef _WIN64
#define SIZETAdd      ULongLongAdd
#else
_Must_inspect_result_
__inline
HRESULT
SIZETAdd(
    _In_ SIZE_T Augend,
    _In_ SIZE_T Addend,
    _Out_ _Deref_out_range_(==, Augend + Addend) SIZE_T* pResult)
{
    HRESULT hr;

    if ((Augend + Addend) >= Augend)
    {
        *pResult = (Augend + Addend);
        hr = S_OK;
    }
    else
    {
        *pResult = _SIZE_T_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}
#endif // _WIN64

//
// ULONGLONG addition
//
_Must_inspect_result_
__inline
HRESULT
ULongLongAdd(
    _In_ ULONGLONG ullAugend,
    _In_ ULONGLONG ullAddend,
    _Out_ _Deref_out_range_(==, ullAugend + ullAddend) ULONGLONG* pullResult)
{
    HRESULT hr;

    if ((ullAugend + ullAddend) >= ullAugend)
    {
        *pullResult = (ullAugend + ullAddend);
        hr = S_OK;
    }
    else
    {
        *pullResult = ULONGLONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// DWORDLONG addition
//
#define DWordLongAdd    ULongLongAdd

//
// ULONG64 addition
//
#define ULong64Add  ULongLongAdd

//
// DWORD64 addition
//
#define DWord64Add  ULongLongAdd

//
// UINT64 addition
//
#define UInt64Add   ULongLongAdd


//=============================================================================
// Subtraction functions
//=============================================================================

//
// UINT8 subtraction
//
_Must_inspect_result_
__inline
HRESULT
UInt8Sub(
    _In_ UINT8 u8Minuend,
    _In_ UINT8 u8Subtrahend,
    _Out_ _Deref_out_range_(==, u8Minuend - u8Subtrahend) UINT8* pu8Result)
{
    HRESULT hr;
    
    if (u8Minuend >= u8Subtrahend)
    {
        *pu8Result = (UINT8)(u8Minuend - u8Subtrahend);
        hr = S_OK;
    }
    else
    {
        *pu8Result = UINT8_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// USHORT subtraction
//
_Must_inspect_result_
__inline
HRESULT
UShortSub(
    _In_ USHORT usMinuend,
    _In_ USHORT usSubtrahend,
    _Out_ _Deref_out_range_(==, usMinuend - usSubtrahend) USHORT* pusResult)
{
    HRESULT hr;

    if (usMinuend >= usSubtrahend)
    {
        *pusResult = (USHORT)(usMinuend - usSubtrahend);
        hr = S_OK;
    }
    else
    {
        *pusResult = USHORT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// UINT16 subtraction
//
#define UInt16Sub  UShortSub

//
// WORD subtraction
//
#define WordSub    UShortSub


//
// UINT subtraction
//
_Must_inspect_result_
__inline
HRESULT
UIntSub(
    _In_ UINT uMinuend,
    _In_ UINT uSubtrahend,
    _Out_ _Deref_out_range_(==, uMinuend - uSubtrahend) UINT* puResult)
{
    HRESULT hr;

    if (uMinuend >= uSubtrahend)
    {
        *puResult = (uMinuend - uSubtrahend);
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// UINT32 subtraction
//
#define UInt32Sub  UIntSub

//
// UINT_PTR subtraction
//
#ifdef _WIN64
#define UIntPtrSub ULongLongSub
#else
_Must_inspect_result_
__inline
HRESULT
UIntPtrSub(
    _In_ UINT_PTR uMinuend,
    _In_ UINT_PTR uSubtrahend,
    _Out_ _Deref_out_range_(==, uMinuend - uSubtrahend) UINT_PTR* puResult)
{
    HRESULT hr;

    if (uMinuend >= uSubtrahend)
    {
        *puResult = (uMinuend - uSubtrahend);
        hr = S_OK;
    }
    else
    {
        *puResult = UINT_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}
#endif // _WIN64

//
// ULONG subtraction
//
_Must_inspect_result_
__inline
HRESULT
ULongSub(
    _In_ ULONG ulMinuend,
    _In_ ULONG ulSubtrahend,
    _Out_ _Deref_out_range_(==, ulMinuend - ulSubtrahend) ULONG* pulResult)
{
    HRESULT hr;

    if (ulMinuend >= ulSubtrahend)
    {
        *pulResult = (ulMinuend - ulSubtrahend);
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// ULONG_PTR subtraction
//
#ifdef _WIN64
#define ULongPtrSub ULongLongSub
#else
_Must_inspect_result_
__inline
HRESULT
ULongPtrSub(
    _In_ ULONG_PTR ulMinuend,
    _In_ ULONG_PTR ulSubtrahend,
    _Out_ _Deref_out_range_(==, ulMinuend - ulSubtrahend) ULONG_PTR* pulResult)
{
    HRESULT hr;

    if (ulMinuend >= ulSubtrahend)
    {
        *pulResult = (ulMinuend - ulSubtrahend);
        hr = S_OK;
    }
    else
    {
        *pulResult = ULONG_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}
#endif // _WIN64


//
// DWORD subtraction
//
#define DWordSub       ULongSub

//
// DWORD_PTR subtraction
//
#ifdef _WIN64
#define DWordPtrSub    ULongLongSub
#else
_Must_inspect_result_
__inline
HRESULT
DWordPtrSub(
    _In_ DWORD_PTR dwMinuend,
    _In_ DWORD_PTR dwSubtrahend,
    _Out_ _Deref_out_range_(==, dwMinuend - dwSubtrahend) DWORD_PTR* pdwResult)
{
    HRESULT hr;

    if (dwMinuend >= dwSubtrahend)
    {
        *pdwResult = (dwMinuend - dwSubtrahend);
        hr = S_OK;
    }
    else
    {
        *pdwResult = DWORD_PTR_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
} 
#endif // _WIN64

//
// size_t subtraction
//
_Must_inspect_result_
__inline
HRESULT
SizeTSub(
    _In_ size_t Minuend,
    _In_ size_t Subtrahend,
    _Out_ _Deref_out_range_(==, Minuend - Subtrahend) size_t* pResult)
{
    HRESULT hr;

    if (Minuend >= Subtrahend)
    {
        *pResult = (Minuend - Subtrahend);
        hr = S_OK;
    }
    else
    {
        *pResult = SIZE_T_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// SIZE_T subtraction
//
#ifdef _WIN64
#define SIZETSub   ULongLongSub
#else
_Must_inspect_result_
__inline
HRESULT
SIZETSub(
    _In_ SIZE_T Minuend,
    _In_ SIZE_T Subtrahend,
    _Out_ _Deref_out_range_(==, Minuend - Subtrahend) SIZE_T* pResult)
{
    HRESULT hr;

    if (Minuend >= Subtrahend)
    {
        *pResult = (Minuend - Subtrahend);
        hr = S_OK;
    }
    else
    {
        *pResult = _SIZE_T_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}
#endif // _WIN64

//
// ULONGLONG subtraction
//
_Must_inspect_result_
__inline
HRESULT
ULongLongSub(
    _In_ ULONGLONG ullMinuend,
    _In_ ULONGLONG ullSubtrahend,
    _Out_ _Deref_out_range_(==, ullMinuend - ullSubtrahend) ULONGLONG* pullResult)
{
    HRESULT hr;

    if (ullMinuend >= ullSubtrahend)
    {
        *pullResult = (ullMinuend - ullSubtrahend);
        hr = S_OK;
    }
    else
    {
        *pullResult = ULONGLONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    
    return hr;
}

//
// DWORDLONG subtraction
//
#define DWordLongSub   ULongLongSub

//
// ULONG64 subtraction
//
#define ULong64Sub ULongLongSub

//
// DWORD64 subtraction
//
#define DWord64Sub ULongLongSub

//
// UINT64 subtraction
//
#define UInt64Sub  ULongLongSub


//=============================================================================
// Multiplication functions
//=============================================================================

//
// UINT8 multiplication
//
_Must_inspect_result_
__inline
HRESULT
UInt8Mult(
    _In_ UINT8 u8Multiplicand,
    _In_ UINT8 u8Multiplier,
    _Out_ _Deref_out_range_(==, u8Multiplicand * u8Multiplier) UINT8* pu8Result)
{
    UINT uResult = ((UINT)u8Multiplicand) * ((UINT)u8Multiplier);
    
    return UIntToUInt8(uResult, pu8Result);
}    

//
// USHORT multiplication
//
_Must_inspect_result_
__inline
HRESULT
UShortMult(
    _In_ USHORT usMultiplicand,
    _In_ USHORT usMultiplier,
    _Out_ _Deref_out_range_(==, usMultiplicand * usMultiplier) USHORT* pusResult)
{
    ULONG ulResult = ((ULONG)usMultiplicand) * ((ULONG)usMultiplier);
    
    return ULongToUShort(ulResult, pusResult);
}

//
// UINT16 multiplication
//
#define UInt16Mult  UShortMult

//
// WORD multiplication
//
#define WordMult    UShortMult

//
// UINT multiplication
//
_Must_inspect_result_
__inline
HRESULT
UIntMult(
    _In_ UINT uMultiplicand,
    _In_ UINT uMultiplier,
    _Out_ _Deref_out_range_(==, uMultiplicand * uMultiplier) UINT* puResult)
{
    ULONGLONG ull64Result = UInt32x32To64(uMultiplicand, uMultiplier);

    return ULongLongToUInt(ull64Result, puResult);
}

//
// UINT32 multiplication
//
#define UInt32Mult  UIntMult

//
// UINT_PTR multiplication
//
#ifdef _WIN64
#define UIntPtrMult     ULongLongMult
#else
_Must_inspect_result_
__inline
HRESULT
UIntPtrMult(
    _In_ UINT_PTR uMultiplicand,
    _In_ UINT_PTR uMultiplier,
    _Out_ _Deref_out_range_(==, uMultiplicand * uMultiplier) UINT_PTR* puResult)
{
    ULONGLONG ull64Result = UInt32x32To64(uMultiplicand, uMultiplier);

    return ULongLongToUIntPtr(ull64Result, puResult);
}
#endif // _WIN64

//
// ULONG multiplication
//
_Must_inspect_result_
__inline
HRESULT
ULongMult(
    _In_ ULONG ulMultiplicand,
    _In_ ULONG ulMultiplier,
    _Out_ _Deref_out_range_(==, ulMultiplicand * ulMultiplier) ULONG* pulResult)
{
    ULONGLONG ull64Result = UInt32x32To64(ulMultiplicand, ulMultiplier);
    
    return ULongLongToULong(ull64Result, pulResult);
}

//
// ULONG_PTR multiplication
//
#ifdef _WIN64
#define ULongPtrMult    ULongLongMult
#else
_Must_inspect_result_
__inline
HRESULT
ULongPtrMult(
    _In_ ULONG_PTR ulMultiplicand,
    _In_ ULONG_PTR ulMultiplier,
    _Out_ _Deref_out_range_(==, ulMultiplicand * ulMultiplier) ULONG_PTR* pulResult)
{
    ULONGLONG ull64Result = UInt32x32To64(ulMultiplicand, ulMultiplier);
    
    return ULongLongToULongPtr(ull64Result, pulResult);
}
#endif // _WIN64

//
// DWORD multiplication
//
#define DWordMult       ULongMult

//
// DWORD_PTR multiplication
//
#ifdef _WIN64
#define DWordPtrMult    ULongLongMult
#else
_Must_inspect_result_
__inline
HRESULT
DWordPtrMult(
    _In_ DWORD_PTR dwMultiplicand,
    _In_ DWORD_PTR dwMultiplier,
    _Out_ _Deref_out_range_(==, dwMultiplicand * dwMultiplier) DWORD_PTR* pdwResult)
{
    ULONGLONG ull64Result = UInt32x32To64(dwMultiplicand, dwMultiplier);
    
    return ULongLongToDWordPtr(ull64Result, pdwResult);
}
#endif // _WIN64

//
// size_t multiplication
//

#ifdef _WIN64
#define SizeTMult       ULongLongMult
#else
_Must_inspect_result_
__inline
HRESULT
SizeTMult(
    _In_ size_t Multiplicand,
    _In_ size_t Multiplier,
    _Out_ _Deref_out_range_(==, Multiplicand * Multiplier) size_t* pResult)
{
    ULONGLONG ull64Result = UInt32x32To64(Multiplicand, Multiplier);

    return ULongLongToSizeT(ull64Result, pResult);
}
#endif // _WIN64

//
// SIZE_T multiplication
//
#ifdef _WIN64
#define SIZETMult       ULongLongMult
#else
_Must_inspect_result_
__inline
HRESULT
SIZETMult(
    _In_ SIZE_T Multiplicand,
    _In_ SIZE_T Multiplier,
    _Out_ _Deref_out_range_(==, Multiplicand * Multiplier) SIZE_T* pResult)
{
    ULONGLONG ull64Result = UInt32x32To64(Multiplicand, Multiplier);
    
    return ULongLongToSIZET(ull64Result, pResult);
}
#endif // _WIN64

//
// ULONGLONG multiplication
//
_Must_inspect_result_
__inline
HRESULT
ULongLongMult(
    _In_ ULONGLONG ullMultiplicand,
    _In_ ULONGLONG ullMultiplier,
    _Out_ _Deref_out_range_(==, ullMultiplicand * ullMultiplier) ULONGLONG* pullResult)
{
    HRESULT hr;
#if defined(_USE_INTRINSIC_MULTIPLY128)
    ULONGLONG ullResultHigh;
    ULONGLONG ullResultLow;
    
    ullResultLow = __UnsignedMultiply128(ullMultiplicand, ullMultiplier, &ullResultHigh);
    if (ullResultHigh == 0)
    {
        _Analysis_assume_(ullMultiplicand * ullMultiplier == ullResultLow);
        *pullResult = ullResultLow;
        hr = S_OK;
    }
    else
    {
        *pullResult = ULONGLONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
#else
    // 64x64 into 128 is like 32.32 x 32.32.
    //
    // a.b * c.d = a*(c.d) + .b*(c.d) = a*c + a*.d + .b*c + .b*.d
    // back in non-decimal notation where A=a*2^32 and C=c*2^32:  
    // A*C + A*d + b*C + b*d
    // So there are four components to add together.
    //   result = (a*c*2^64) + (a*d*2^32) + (b*c*2^32) + (b*d)
    //
    // a * c must be 0 or there would be bits in the high 64-bits
    // a * d must be less than 2^32 or there would be bits in the high 64-bits
    // b * c must be less than 2^32 or there would be bits in the high 64-bits
    // then there must be no overflow of the resulting values summed up.
    
    ULONG dw_a;
    ULONG dw_b;
    ULONG dw_c;
    ULONG dw_d;
    ULONGLONG ad = 0;
    ULONGLONG bc = 0;
    ULONGLONG bd = 0;
    ULONGLONG ullResult = 0;

    hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    
    dw_a = (ULONG)(ullMultiplicand >> 32);
    dw_c = (ULONG)(ullMultiplier >> 32);

    // common case -- if high dwords are both zero, no chance for overflow
    if ((dw_a == 0) && (dw_c == 0))
    {
        dw_b = (DWORD)ullMultiplicand;
        dw_d = (DWORD)ullMultiplier;

        *pullResult = (((ULONGLONG)dw_b) * (ULONGLONG)dw_d);
        hr = S_OK;
    }
    else
    {
        // a * c must be 0 or there would be bits set in the high 64-bits
        if ((dw_a == 0) ||
            (dw_c == 0))
        {
            dw_d = (DWORD)ullMultiplier;

            // a * d must be less than 2^32 or there would be bits set in the high 64-bits
            ad = (((ULONGLONG)dw_a) * (ULONGLONG)dw_d);
            if ((ad & 0xffffffff00000000) == 0)
            {
                dw_b = (DWORD)ullMultiplicand;

                // b * c must be less than 2^32 or there would be bits set in the high 64-bits
                bc = (((ULONGLONG)dw_b) * (ULONGLONG)dw_c);
                if ((bc & 0xffffffff00000000) == 0)
                {
                    // now sum them all up checking for overflow.
                    // shifting is safe because we already checked for overflow above
                    if (SUCCEEDED(ULongLongAdd(bc << 32, ad << 32, &ullResult)))                        
                    {
                        // b * d
                        bd = (((ULONGLONG)dw_b) * (ULONGLONG)dw_d);
                    
                        if (SUCCEEDED(ULongLongAdd(ullResult, bd, &ullResult)))
                        {
                            *pullResult = ullResult;
                            hr = S_OK;
                        }
                    }
                }
            }
        }
    }

    if (FAILED(hr))
    {
        *pullResult = ULONGLONG_ERROR;
    }
#pragma warning(suppress:26071)
#endif // _USE_INTRINSIC_MULTIPLY128
    return hr;
}

//
// DWORDLONG multiplication
//
#define DWordLongMult   ULongLongMult

//
// ULONG64 multiplication
//
#define ULong64Mult ULongLongMult

//
// DWORD64 multiplication
//
#define DWord64Mult ULongLongMult

//
// UINT64 multiplication
//
#define UInt64Mult  ULongLongMult


/////////////////////////////////////////////////////////////////////////
//
// signed operations
//
// Strongly consider using unsigned numbers.
//
// Signed numbers are often used where unsigned numbers should be used.
// For example file sizes and array indices should always be unsigned.
// (File sizes should be 64bit integers; array indices should be size_t.)
// Subtracting a larger positive signed number from a smaller positive
// signed number with IntSub will succeed, producing a negative number,
// that then must not be used as an array index (but can occasionally be
// used as a pointer index.) Similarly for adding a larger magnitude
// negative number to a smaller magnitude positive number.
//
// intsafe.h does not protect you from such errors. It tells you if your
// integer operations overflowed, not if you are doing the right thing
// with your non-overflowed integers.
//
// Likewise you can overflow a buffer with a non-overflowed unsigned index.
//
#if defined(ENABLE_INTSAFE_SIGNED_FUNCTIONS)

#if defined(_USE_INTRINSIC_MULTIPLY128)
#ifdef __cplusplus
extern "C" {
#endif

#if defined(_M_X64) && !defined(_M_ARM64EC)

#define __Multiply128 _mul128

#endif // defined(_M_X64) && !defined(_M_ARM64EC)

LONG64
__Multiply128(
    _In_ LONG64 Multiplier,
    _In_ LONG64  Multiplicand,
    _Out_ LONG64 *HighProduct
    );

#if defined(_M_X64) && !defined(_M_ARM64EC)

#pragma intrinsic(_mul128)

#endif // defined(_M_X64) && !defined(_M_ARM64EC)

#if defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)

LONG64
__mulh(
   LONG64 Multiplier,
   LONG64 Multiplicand
   );

#pragma intrinsic(__mulh)

__inline
LONG64
__Multiply128(
    _In_ LONG64 Multiplier,
    _In_ LONG64 Multiplicand,
    _Out_ _Deref_out_range_(==, Multiplicand * Multiplier) LONG64* HighProduct)
{
    *HighProduct = __mulh(Multiplier, Multiplicand);
    return Multiplier * Multiplicand;
}

#endif // defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)

#ifdef __cplusplus
}
#endif
#endif // _USE_INTRINSIC_MULTIPLY128


//=============================================================================
// Signed addition functions
//=============================================================================

//
// INT8 Addition
//
_Must_inspect_result_
__inline
HRESULT
Int8Add(
    _In_ INT8 i8Augend,
    _In_ INT8 i8Addend,
    _Out_ _Deref_out_range_(==, i8Augend + i8Addend) INT8* pi8Result
    )
{
    C_ASSERT(sizeof(LONG) > sizeof(INT8));
    return LongToInt8(((LONG)i8Augend) + ((LONG)i8Addend), pi8Result);
}

//
// SHORT Addition
//
_Must_inspect_result_
__inline
HRESULT
ShortAdd(
    _In_ SHORT sAugend,
    _In_ SHORT sAddend,
    _Out_ _Deref_out_range_(==, sAugend + sAddend) SHORT* psResult
    )
{
    C_ASSERT(sizeof(LONG) > sizeof(SHORT));
    return LongToShort(((LONG)sAugend) + ((LONG)sAddend), psResult);
}

//
// INT16 Addition
//
#define Int16Add    ShortAdd

//
// INT Addition
//
_Must_inspect_result_
__inline
HRESULT
IntAdd(
    _In_ INT iAugend,
    _In_ INT iAddend,
    _Out_ _Deref_out_range_(==, iAugend + iAddend) INT* piResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(INT));
    return LongLongToInt(((LONGLONG)iAugend) + ((LONGLONG)iAddend), piResult);
}

//
// INT32 Addition
//
#define Int32Add    IntAdd

//
// INT_PTR addition
//
#ifdef _WIN64
#define IntPtrAdd   LongLongAdd
#else
_Must_inspect_result_
__inline
HRESULT
IntPtrAdd(
    _In_ INT_PTR iAugend,
    _In_ INT_PTR iAddend,
    _Out_ _Deref_out_range_(==, iAugend + iAddend) INT_PTR* piResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(INT_PTR));
    return LongLongToIntPtr(((LONGLONG)iAugend) + ((LONGLONG)iAddend), piResult);
}
#endif

//
// LONG Addition
//
_Must_inspect_result_
__inline
HRESULT
LongAdd(
    _In_ LONG lAugend,
    _In_ LONG lAddend,
    _Out_ _Deref_out_range_(==, lAugend + lAddend) LONG* plResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(LONG));
    return LongLongToLong(((LONGLONG)lAugend) + ((LONGLONG)lAddend), plResult);
}

//
// LONG32 Addition
//
#define Long32Add   IntAdd

//
// LONG_PTR Addition
//
#ifdef _WIN64
#define LongPtrAdd   LongLongAdd
#else
_Must_inspect_result_
__inline
HRESULT
LongPtrAdd(
    _In_ LONG_PTR lAugend,
    _In_ LONG_PTR lAddend,
    _Out_ _Deref_out_range_(==, lAugend + lAddend) LONG_PTR* plResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(LONG_PTR));
    return LongLongToLongPtr(((LONGLONG)lAugend) + ((LONGLONG)lAddend), plResult);
}
#endif

//
// LONGLONG Addition
//
_Must_inspect_result_
__inline
HRESULT
LongLongAdd(
    _In_ LONGLONG llAugend,
    _In_ LONGLONG llAddend,
    _Out_ _Deref_out_range_(==, llAugend + llAddend) LONGLONG* pllResult
    )
{
    HRESULT hr;

    //
    // Signed integer overflow is undefined and must be avoided.
    // Compilers do sometimes propagate undefined.
    // Unsigned integer overflow is well defined to silently wraparound
    // mod 2^n and may be used without problem.
    //
    ULONGLONG ullResult = ((ULONGLONG)llAugend) + (ULONGLONG)llAddend;
    
    //
    // Adding positive to negative never overflows.
    // If you add two positive numbers, you expect a positive result.
    // If you add two negative numbers, you expect a negative result.
    // Overflow if inputs have the same sign and output has the other sign.
    // Unsigned greater than signed maximum, cast to signed, will be negative.
    //
    if (((llAugend < 0) == (llAddend < 0))  &&
        ((llAugend < 0) != (ullResult > (ULONGLONG)LONGLONG_MAX)))
    {
        *pllResult = LONGLONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    else
    {
        *pllResult = (LONGLONG)ullResult;
        hr = S_OK;
    }

    return hr;
}

//
// LONG64 Addition
//
#define Long64Add   LongLongAdd

//
// INT64 Addition
//
#define Int64Add    LongLongAdd

//
// ptrdiff_t Addition
//
#ifdef _WIN64
#define PtrdiffTAdd LongLongAdd
#else
_Must_inspect_result_
__inline
HRESULT
PtrdiffTAdd(
    _In_ ptrdiff_t Augend,
    _In_ ptrdiff_t Addend,
    _Out_ _Deref_out_range_(==, Augend + Addend) ptrdiff_t* pResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(ptrdiff_t));
    return LongLongToPtrdiffT(((LONGLONG)Augend) + ((LONGLONG)Addend), pResult);
}
#endif

//
// SSIZE_T Addition
//
#ifdef _WIN64
#define SSIZETAdd   LongLongAdd
#else
_Must_inspect_result_
__inline
HRESULT
SSIZETAdd(
    _In_ SSIZE_T Augend,
    _In_ SSIZE_T Addend,
    _Out_ _Deref_out_range_(==, Augend + Addend) SSIZE_T* pResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(SSIZE_T));
    return LongLongToSSIZET(((LONGLONG)Augend) + ((LONGLONG)Addend), pResult);
}
#endif


//=============================================================================
// Signed subtraction functions
//=============================================================================

//
// INT8 Subtraction
//
_Must_inspect_result_
__inline
HRESULT
Int8Sub(
    _In_ INT8 i8Minuend,
    _In_ INT8 i8Subtrahend,
    _Out_ _Deref_out_range_(==, i8Minuend - i8Subtrahend) INT8* pi8Result
    )
{
    C_ASSERT(sizeof(LONG) > sizeof(INT8));
    return LongToInt8(((LONG)i8Minuend) - ((LONG)i8Subtrahend), pi8Result);
}

//
// SHORT Subtraction
//
_Must_inspect_result_
__inline
HRESULT
ShortSub(
    _In_ SHORT sMinuend,
    _In_ SHORT sSubtrahend,
    _Out_ _Deref_out_range_(==, sMinuend - sSubtrahend) SHORT* psResult
    )
{
    C_ASSERT(sizeof(LONG) > sizeof(SHORT));
    return LongToShort(((LONG)sMinuend) - ((LONG)sSubtrahend), psResult);
}

//
// INT16 Subtraction
//
#define Int16Sub   ShortSub

//
// INT Subtraction
//
_Must_inspect_result_
__inline
HRESULT
IntSub(
    _In_ INT iMinuend,
    _In_ INT iSubtrahend,
    _Out_ _Deref_out_range_(==, iMinuend - iSubtrahend) INT* piResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(INT));
    return LongLongToInt(((LONGLONG)iMinuend) - ((LONGLONG)iSubtrahend), piResult);
}

//
// INT32 Subtraction
//
#define Int32Sub   IntSub

//
// INT_PTR Subtraction
//
#ifdef _WIN64
#define IntPtrSub  LongLongSub
#else
_Must_inspect_result_
__inline
HRESULT
IntPtrSub(
    _In_ INT_PTR iMinuend,
    _In_ INT_PTR iSubtrahend,
    _Out_ _Deref_out_range_(==, iMinuend - iSubtrahend) INT_PTR* piResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(INT_PTR));
    return LongLongToIntPtr(((LONGLONG)iMinuend) - ((LONGLONG)iSubtrahend), piResult);
}
#endif

//
// LONG Subtraction
//
_Must_inspect_result_
__inline
HRESULT
LongSub(
    _In_ LONG lMinuend,
    _In_ LONG lSubtrahend,
    _Out_ _Deref_out_range_(==, lMinuend - lSubtrahend) LONG* plResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(LONG));
    return LongLongToLong(((LONGLONG)lMinuend) - ((LONGLONG)lSubtrahend), plResult);
}

//
// LONG32 Subtraction
//
#define Long32Sub  IntSub

//
// LONG_PTR Subtraction
//
#ifdef _WIN64
#define LongPtrSub  LongLongSub
#else
_Must_inspect_result_
__inline
HRESULT
LongPtrSub(
    _In_ LONG_PTR lMinuend,
    _In_ LONG_PTR lSubtrahend,
    _Out_ _Deref_out_range_(==, lMinuend - lSubtrahend) LONG_PTR* plResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(LONG_PTR));
    return LongLongToLongPtr(((LONGLONG)lMinuend) - ((LONGLONG)lSubtrahend), plResult);
}
#endif

//
// LongLongSub
//
_Must_inspect_result_
__inline
HRESULT
LongLongSub(
    _In_ LONGLONG llMinuend,
    _In_ LONGLONG llSubtrahend,
    _Out_ _Deref_out_range_(==, llMinuend - llSubtrahend) LONGLONG* pllResult
    )
{
    HRESULT hr;

    //
    // Signed integer overflow is undefined and must be avoided.
    // Compilers do sometimes propagate undefined.
    // Unsigned integer overflow is well defined to silently wraparound
    // mod 2^n and may be used without problem.
    //
    ULONGLONG ullResult = ((ULONGLONG)llMinuend) - (ULONGLONG)llSubtrahend;

    //
    // Subtracting a positive number from a positive number never overflows.
    // Subtracting a negative number from a negative number never overflows.
    // If you subtract a negative number from a positive number, you expect a positive result.
    // If you subtract a positive number from a negative number, you expect a negative result.
    // Overflow if inputs vary in sign and the output does not have the same sign as the first input.
    // Unsigned greater than signed maximum, cast to signed, will be negative.
    //
    if (((llMinuend < 0) != (llSubtrahend < 0)) &&
        ((llMinuend < 0) != (ullResult > (ULONGLONG)LONGLONG_MAX)))
    {
        *pllResult = LONGLONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    else
    {
        *pllResult = (LONGLONG)ullResult;
        hr = S_OK;
    }
    
    return hr;
}

//
// LONG64 Subtraction
//
#define Long64Sub  LongLongSub

//
// INT64 Subtraction
//
#define Int64Sub   LongLongSub

//
// ptrdiff_t Subtraction
//
#ifdef _WIN64
#define PtrdiffTSub LongLongSub
#else
_Must_inspect_result_
__inline
HRESULT
PtrdiffTSub(
    _In_ ptrdiff_t Minuend,
    _In_ ptrdiff_t Subtrahend,
    _Out_ _Deref_out_range_(==, Minuend - Subtrahend) ptrdiff_t* pResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(ptrdiff_t));
    return LongLongToPtrdiffT(((LONGLONG)Minuend) - ((LONGLONG)Subtrahend), pResult);
}
#endif

//
// SSIZE_T Subtraction
//
#ifdef _WIN64
#define SSIZETSub  LongLongSub
#else
_Must_inspect_result_
__inline
HRESULT
SSIZETSub(
    _In_ SSIZE_T Minuend,
    _In_ SSIZE_T Subtrahend,
    _Out_ _Deref_out_range_(==, Minuend - Subtrahend) SSIZE_T* pResult)
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(SSIZE_T));
    return LongLongToSSIZET(((LONGLONG)Minuend) - ((LONGLONG)Subtrahend), pResult);
}
#endif


//=============================================================================
// Signed multiplication functions
//=============================================================================

//
// INT8 multiplication
//
_Must_inspect_result_
__inline
HRESULT
Int8Mult(
    _In_ INT8 i8Multiplicand,
    _In_ INT8 i8Multiplier,
    _Out_ _Deref_out_range_(==, i8Multiplicand * i8Multiplier) INT8* pi8Result
    )
{
    C_ASSERT(sizeof(LONG) > sizeof(INT8));
    return LongToInt8(((LONG)i8Multiplier) * ((LONG)i8Multiplicand), pi8Result);
}

//
// SHORT multiplication
//
_Must_inspect_result_
__inline
HRESULT
ShortMult(
    _In_ SHORT sMultiplicand,
    _In_ SHORT sMultiplier,
    _Out_ _Deref_out_range_(==, sMultiplicand * sMultiplier) SHORT* psResult
    )
{
    C_ASSERT(sizeof(LONG) > sizeof(SHORT));
    return LongToShort(((LONG)sMultiplicand) * ((LONG)sMultiplier), psResult);
}

//
// INT16 multiplication
//
#define Int16Mult   ShortMult

//
// INT multiplication
//
_Must_inspect_result_
__inline
HRESULT
IntMult(
    _In_ INT iMultiplicand,
    _In_ INT iMultiplier,
    _Out_ _Deref_out_range_(==, iMultiplicand * iMultiplier) INT* piResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(INT));
    return LongLongToInt(((LONGLONG)iMultiplicand) * ((LONGLONG)iMultiplier), piResult);
}

//
// INT32 multiplication
//
#define Int32Mult   IntMult

//
// INT_PTR multiplication
//
#ifdef _WIN64
#define IntPtrMult   LongLongMult
#else
_Must_inspect_result_
__inline
HRESULT
IntPtrMult(
    _In_ INT_PTR iMultiplicand,
    _In_ INT_PTR iMultiplier,
    _Out_ _Deref_out_range_(==, iMultiplicand * iMultiplier) INT_PTR* piResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(INT_PTR));
    return LongLongToIntPtr(((LONGLONG)iMultiplicand) * ((LONGLONG)iMultiplier), piResult);
}
#endif

//
// LONG multiplication
//
_Must_inspect_result_
__inline
HRESULT
LongMult(
    _In_ LONG lMultiplicand,
    _In_ LONG lMultiplier,
    _Out_ _Deref_out_range_(==, lMultiplicand * lMultiplier) LONG* plResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(LONG));
    return LongLongToLong(((LONGLONG)lMultiplicand) * ((LONGLONG)lMultiplier), plResult);
}

//
// LONG32 multiplication
//
#define Long32Mult  IntMult

//
// LONG_PTR multiplication
//
#ifdef _WIN64
#define LongPtrMult LongLongMult
#else
_Must_inspect_result_
__inline
HRESULT
LongPtrMult(
    _In_ LONG_PTR lMultiplicand,
    _In_ LONG_PTR lMultiplier,
    _Out_ _Deref_out_range_(==, lMultiplicand * lMultiplier) LONG_PTR* plResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(LONG_PTR));
    return LongLongToLongPtr(((LONGLONG)lMultiplicand) * ((LONGLONG)lMultiplier), plResult);
}
#endif

//
// LONGLONG multiplication
//
_Must_inspect_result_
__inline
HRESULT
LongLongMult(
    _In_ LONGLONG llMultiplicand,
    _In_ LONGLONG llMultiplier,
    _Out_ _Deref_out_range_(==, llMultiplicand * llMultiplier) LONGLONG* pllResult
    )
{
    HRESULT hr;

#if defined(_USE_INTRINSIC_MULTIPLY128)
    LONGLONG llResultHigh;
    LONGLONG llResultLow;

    llResultLow = __Multiply128(llMultiplicand, llMultiplier, &llResultHigh);
    
    if (((llResultLow < 0) && (llResultHigh != -1))    || 
        ((llResultLow >= 0) && (llResultHigh != 0)))
    {
        *pllResult = LONGLONG_ERROR;
        hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
    }
    else
    {
        *pllResult = llResultLow;
        hr = S_OK;
    }
#else // _USE_INTRINSIC_MULTIPLY128
    //
    // Split into sign and magnitude, do unsigned operation, apply sign.
    //
    
    ULONGLONG ullMultiplicand;
    ULONGLONG ullMultiplier;
    ULONGLONG ullResult;
    const ULONGLONG LONGLONG_MIN_MAGNITUDE = ((((ULONGLONG) - (LONGLONG_MIN + 1))) + 1);

    if (llMultiplicand < 0)
    {
        //
        // Avoid negating the most negative number.
        //
        ullMultiplicand = ((ULONGLONG)(- (llMultiplicand + 1))) + 1;
    }
    else
    {
        ullMultiplicand = (ULONGLONG)llMultiplicand;
    }

    if (llMultiplier < 0)
    {
        //
        // Avoid negating the most negative number.
        //
        ullMultiplier = ((ULONGLONG)(- (llMultiplier + 1))) + 1;
    }
    else
    {
        ullMultiplier = (ULONGLONG)llMultiplier;
    }

    hr = ULongLongMult(ullMultiplicand, ullMultiplier, &ullResult);
    if (SUCCEEDED(hr))
    {
        if ((llMultiplicand < 0) != (llMultiplier < 0))
        {
            if (ullResult > LONGLONG_MIN_MAGNITUDE)
            {
                *pllResult = LONGLONG_ERROR;
                hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
            }
            else
            {
                *pllResult = - ((LONGLONG)ullResult);
            }
        }
        else
        {
            if (ullResult > LONGLONG_MAX)
            {
                *pllResult = LONGLONG_ERROR;
                hr = INTSAFE_E_ARITHMETIC_OVERFLOW;
            }
            else
            {
                *pllResult = (LONGLONG)ullResult;
            }
        }
    }
    else
    {
        *pllResult = LONGLONG_ERROR;
    }
#endif // _USE_INTRINSIC_MULTIPLY128

    return hr;
}

//
// LONG64 multiplication
//
#define Long64Mult  LongLongMult

//
// INT64 multiplication
//
#define Int64Mult   LongLongMult

//
// ptrdiff_t multiplication
//
#ifdef _WIN64
#define PtrdiffTMult    LongLongMult
#else
_Must_inspect_result_
__inline
HRESULT
PtrdiffTMult(
    _In_ ptrdiff_t Multiplicand,
    _In_ ptrdiff_t Multiplier,
    _Out_ _Deref_out_range_(==, Multiplicand * Multiplier) ptrdiff_t* pResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(ptrdiff_t));
    return LongLongToPtrdiffT(((LONGLONG)Multiplicand) * ((LONGLONG)Multiplier), pResult);
}
#endif

//
// SSIZE_T multiplication
//
#ifdef _WIN64
#define SSIZETMult  LongLongMult
#else
_Must_inspect_result_
__inline
HRESULT
SSIZETMult(
    _In_ SSIZE_T Multiplicand,
    _In_ SSIZE_T Multiplier,
    _Out_ _Deref_out_range_(==, Multiplicand * Multiplier) SSIZE_T* pResult
    )
{
    C_ASSERT(sizeof(LONGLONG) > sizeof(SSIZE_T));
    return LongLongToSSIZET(((LONGLONG)Multiplicand) * ((LONGLONG)Multiplier), pResult);
}
#endif

#endif // ENABLE_INTSAFE_SIGNED_FUNCTIONS

//
// Macros that are no longer used in this header but which clients may
// depend on being defined here.
//
#ifndef LOWORD
#define LOWORD(l)     ((WORD)(((DWORD_PTR)(l)) & 0xffff))
#endif
#ifndef HIWORD
#define HIWORD(l)     ((WORD)((((DWORD_PTR)(l)) >> 16) & 0xffff))
#endif
#ifndef LODWORD
#define LODWORD(_qw)    ((DWORD)(_qw))
#endif
#ifndef HIDWORD
#define HIDWORD(_qw)    ((DWORD)(((_qw) >> 32) & 0xffffffff))
#endif

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // _INTSAFE_H_INCLUDED_
