//
// limits.h
//
//      Copyright (c) Microsoft Corporation. All rights reserved.
//
// The C Standard Library <limits.h> header.
//
#pragma once
#define _INC_LIMITS

#include <vcruntime.h>

#pragma warning(push)
#pragma warning(disable: _VCRUNTIME_DISABLED_WARNINGS)

_CRT_BEGIN_C_HEADER

#define CHAR_BIT      8
#define SCHAR_MIN   (-128)
#define SCHAR_MAX     127
#define UCHAR_MAX     0xff

#ifndef _CHAR_UNSIGNED
    #define CHAR_MIN    SCHAR_MIN
    #define CHAR_MAX    SCHAR_MAX
#else
    #define CHAR_MIN    0
    #define CHAR_MAX    UCHAR_MAX
#endif

#define MB_LEN_MAX    5
#define SHRT_MIN    (-32768)
#define SHRT_MAX      32767
#define USHRT_MAX     0xffff
#define INT_MIN     (-2147483647 - 1)
#define INT_MAX       2147483647
#define UINT_MAX      0xffffffff
#define LONG_MIN    (-2147483647L - 1)
#define LONG_MAX      2147483647L
#define ULONG_MAX     0xffffffffUL
#define LLONG_MAX     9223372036854775807i64
#define LLONG_MIN   (-9223372036854775807i64 - 1)
#define ULLONG_MAX    0xffffffffffffffffui64

#define _I8_MIN     (-127i8 - 1)
#define _I8_MAX       127i8
#define _UI8_MAX      0xffui8

#define _I16_MIN    (-32767i16 - 1)
#define _I16_MAX      32767i16
#define _UI16_MAX     0xffffui16

#define _I32_MIN    (-2147483647i32 - 1)
#define _I32_MAX      2147483647i32
#define _UI32_MAX     0xffffffffui32

#define _I64_MIN    (-9223372036854775807i64 - 1)
#define _I64_MAX      9223372036854775807i64
#define _UI64_MAX     0xffffffffffffffffui64

#ifndef SIZE_MAX
    // SIZE_MAX definition must match exactly with stdint.h for modules support.
    #ifdef _WIN64
        #define SIZE_MAX 0xffffffffffffffffui64
    #else
        #define SIZE_MAX 0xffffffffui32
    #endif
#endif

#ifndef RSIZE_MAX
    #define RSIZE_MAX (SIZE_MAX >> 1)
#endif

_CRT_END_C_HEADER

#pragma warning(pop) // _VCRUNTIME_DISABLED_WARNINGS
