// xcharconv_tables.h internal header (core)

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef _XCHARCONV_TABLES_H
#define _XCHARCONV_TABLES_H
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR

#if !_HAS_CXX17
#error The contents of <charconv> are only available with C++17. (Also, you should not include this internal header.)
#endif // !_HAS_CXX17

#include <cstdint>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

// For general precision, we can use lookup tables to avoid performing trial formatting.

// For a simple example, imagine counting the number of digits D in an integer, and needing to know
// whether D is less than 3, equal to 3/4/5/6, or greater than 6. We could use a lookup table:
// D | Largest integer with D digits
// 2 |      99
// 3 |     999
// 4 |   9'999
// 5 |  99'999
// 6 | 999'999
// 7 | table end
// Looking up an integer in this table with lower_bound() will work:
// * Too-small integers, like 7, 70, and 99, will cause lower_bound() to return the D == 2 row. (If all we care
//   about is whether D is less than 3, then it's okay to smash the D == 1 and D == 2 cases together.)
// * Integers in [100, 999] will cause lower_bound() to return the D == 3 row, and so forth.
// * Too-large integers, like 1'000'000 and above, will cause lower_bound() to return the end of the table. If we
//   compute D from that index, this will be considered D == 7, which will activate any "greater than 6" logic.

// Floating-point is slightly more complicated.

// The ordinary lookup tables are for X within [-5, 38] for float, and [-5, 308] for double.
// (-5 absorbs too-negative exponents, outside the P > X >= -4 criterion. 38 and 308 are the maximum exponents.)
// Due to the P > X condition, we can use a subset of the table for X within [-5, P - 1], suitably clamped.

// When P is small, rounding can affect X. For example:
// For P == 1, the largest double with X == 0 is: 9.4999999999999982236431605997495353221893310546875
// For P == 2, the largest double with X == 0 is: 9.949999999999999289457264239899814128875732421875
// For P == 3, the largest double with X == 0 is: 9.9949999999999992184029906638897955417633056640625

// Exponent adjustment is a concern for P within [1, 7] for float, and [1, 15] for double (determined via
// brute force). While larger values of P still perform rounding, they can't trigger exponent adjustment.
// This is because only values with repeated '9' digits can undergo exponent adjustment during rounding,
// and floating-point granularity limits the number of consecutive '9' digits that can appear.

// So, we need special lookup tables for small values of P.
// These tables have varying lengths due to the P > X >= -4 criterion. For example:
// For P == 1, need table entries for X: -5, -4, -3, -2, -1, 0
// For P == 2, need table entries for X: -5, -4, -3, -2, -1, 0, 1
// For P == 3, need table entries for X: -5, -4, -3, -2, -1, 0, 1, 2
// For P == 4, need table entries for X: -5, -4, -3, -2, -1, 0, 1, 2, 3

// We can concatenate these tables for compact storage, using triangular numbers to access them.
// The table for P begins at index (P - 1) * (P + 10) / 2 with length P + 5.

// For both the ordinary and special lookup tables, after an index I is returned by lower_bound(), X is I - 5.

// We need to special-case the floating-point value 0.0, which is considered to have X == 0.
// Otherwise, the lookup tables would consider it to have a highly negative X.

// Finally, because we're working with positive floating-point values,
// representation comparisons behave identically to floating-point comparisons.

// The generator is in /tools/scripts/charconv_tables_generate.cpp

_STD_BEGIN

template <class _Floating>
struct _General_precision_tables_2;

template <>
struct _General_precision_tables_2<float> {
    static constexpr int _Max_special_P = 7;

    static const uint32_t _Special_X_table[63];

    static constexpr int _Max_P = 39;

    static const uint32_t _Ordinary_X_table[44];
};

template <>
struct _General_precision_tables_2<double> {
    static constexpr int _Max_special_P = 15;

    static const uint64_t _Special_X_table[195];

    static constexpr int _Max_P = 309;

    static const uint64_t _Ordinary_X_table[314];
};

_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)

#endif // _STL_COMPILER_PREPROCESSOR
#endif // _XCHARCONV_TABLES_H
