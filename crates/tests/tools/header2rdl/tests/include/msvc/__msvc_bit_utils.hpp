// __msvc_bit_utils.hpp internal header (core)

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef __MSVC_BIT_UTILS_HPP
#define __MSVC_BIT_UTILS_HPP
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR

#include <climits>
#include <xtr1common>

#include _STL_INTRIN_HEADER

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

_STD_BEGIN
extern "C" {
extern int __isa_available;
}

_INLINE_VAR constexpr int _Stl_isa_available_sse42 = 2; // equal to __ISA_AVAILABLE_SSE42
_INLINE_VAR constexpr int _Stl_isa_available_avx2  = 5; // equal to __ISA_AVAILABLE_AVX2

template <class _UInt>
constexpr int _Unsigned_integer_digits = sizeof(_UInt) * CHAR_BIT;

// Implementation of countl_zero without using specialized CPU instructions.
// Used at compile time and when said instructions are not supported.
// see "Hacker's Delight" section 5-3
template <class _Ty>
_NODISCARD constexpr int _Countl_zero_fallback(_Ty _Val) noexcept {
    _Ty _Yx = 0;

    unsigned int _Nx = _Unsigned_integer_digits<_Ty>;
    unsigned int _Cx = _Unsigned_integer_digits<_Ty> / 2;
    do {
        _Yx = static_cast<_Ty>(_Val >> _Cx);
        if (_Yx != 0) {
            _Nx -= _Cx;
            _Val = _Yx;
        }
        _Cx >>= 1;
    } while (_Cx != 0);
    return static_cast<int>(_Nx) - static_cast<int>(_Val);
}

#if !defined(_M_CEE_PURE) && !defined(__CUDACC__)
#define _HAS_COUNTL_ZERO_INTRINSICS 1
#else // ^^^ intrinsics available / intrinsics unavailable vvv
#define _HAS_COUNTL_ZERO_INTRINSICS 0
#endif // ^^^ intrinsics unavailable ^^^

#if _HAS_COUNTL_ZERO_INTRINSICS
#if (defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)) || (defined(_M_X64) && !defined(_M_ARM64EC))
template <class _Ty>
_NODISCARD int _Countl_zero_lzcnt(const _Ty _Val) noexcept {
    constexpr int _Digits = _Unsigned_integer_digits<_Ty>;

    if constexpr (_Digits <= 16) {
        return static_cast<int>(__lzcnt16(_Val) - (16 - _Digits));
    } else if constexpr (_Digits == 32) {
        return static_cast<int>(__lzcnt(_Val));
    } else {
#ifdef _M_IX86
        const unsigned int _High = _Val >> 32;
        const auto _Low          = static_cast<unsigned int>(_Val);
        if (_High == 0) {
            return 32 + _Countl_zero_lzcnt(_Low);
        } else {
            return _Countl_zero_lzcnt(_High);
        }
#else // ^^^ defined(_M_IX86) / !defined(_M_IX86) vvv
        return static_cast<int>(__lzcnt64(_Val));
#endif // ^^^ !defined(_M_IX86) ^^^
    }
}

template <class _Ty>
_NODISCARD int _Countl_zero_bsr(const _Ty _Val) noexcept {
    constexpr int _Digits = _Unsigned_integer_digits<_Ty>;

    unsigned long _Result;
    if constexpr (_Digits <= 32) {
        if (!_BitScanReverse(&_Result, _Val)) {
            return _Digits;
        }
    } else {
#ifdef _M_IX86
        const unsigned int _High = _Val >> 32;
        if (_BitScanReverse(&_Result, _High)) {
            return static_cast<int>(31 - _Result);
        }

        const auto _Low = static_cast<unsigned int>(_Val);
        if (!_BitScanReverse(&_Result, _Low)) {
            return _Digits;
        }
#else // ^^^ defined(_M_IX86) / !defined(_M_IX86) vvv
        if (!_BitScanReverse64(&_Result, _Val)) {
            return _Digits;
        }
#endif // ^^^ !defined(_M_IX86) ^^^
    }
    return static_cast<int>(_Digits - 1 - _Result);
}

template <class _Ty>
_NODISCARD int _Checked_x86_x64_countl_zero(const _Ty _Val) noexcept {
#ifdef __AVX2__
    return _Countl_zero_lzcnt(_Val);
#else // ^^^ defined(__AVX2__) / !defined(__AVX2__) vvv
    const bool _Definitely_have_lzcnt = __isa_available >= _Stl_isa_available_avx2;
    if (_Definitely_have_lzcnt) {
        return _Countl_zero_lzcnt(_Val);
    } else {
        return _Countl_zero_bsr(_Val);
    }
#endif // ^^^ !defined(__AVX2__) ^^^
}
#endif // (defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)) || (defined(_M_X64) && !defined(_M_ARM64EC))

#if defined(_M_ARM) || defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)
template <class _Ty>
_NODISCARD int _Checked_arm_arm64_countl_zero(const _Ty _Val) noexcept {
    constexpr int _Digits = _Unsigned_integer_digits<_Ty>;
    if (_Val == 0) {
        return _Digits;
    }

    if constexpr (_Digits <= 32) {
        return static_cast<int>(_CountLeadingZeros(_Val)) - (_Unsigned_integer_digits<unsigned long> - _Digits);
    } else {
        return static_cast<int>(_CountLeadingZeros64(_Val));
    }
}
#endif // defined(_M_ARM) || defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)
#endif // _HAS_COUNTL_ZERO_INTRINSICS

// Implementation of countr_zero without using specialized CPU instructions.
// Used at compile time and when said instructions are not supported.
// see "Hacker's Delight" section 5-4
template <class _Ty>
_NODISCARD constexpr int _Countr_zero_fallback(const _Ty _Val) noexcept {
    constexpr int _Digits = _Unsigned_integer_digits<_Ty>;
    return _Digits - _Countl_zero_fallback(static_cast<_Ty>(static_cast<_Ty>(~_Val) & static_cast<_Ty>(_Val - 1)));
}

// Implementation of popcount without using specialized CPU instructions.
// Used at compile time and when said instructions are not supported.
template <class _Ty>
_NODISCARD constexpr int _Popcount_fallback(_Ty _Val) noexcept {
    constexpr int _Digits = _Unsigned_integer_digits<_Ty>;
#if (defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)) || defined(_M_ARM)
    if constexpr (_Digits == 64) {
        // 64-bit bit operations on architectures without 64-bit registers are less efficient,
        // hence we split the value so that it fits in 32-bit registers
        return _Popcount_fallback(static_cast<unsigned long>(_Val))
             + _Popcount_fallback(static_cast<unsigned long>(_Val >> 32));
    } else
#endif // (defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)) || defined(_M_ARM)
    {
        // we static_cast these bit patterns in order to truncate them to the correct size
        _Val = static_cast<_Ty>(_Val - ((_Val >> 1) & static_cast<_Ty>(0x5555'5555'5555'5555ull)));
        _Val = static_cast<_Ty>((_Val & static_cast<_Ty>(0x3333'3333'3333'3333ull))
                                + ((_Val >> 2) & static_cast<_Ty>(0x3333'3333'3333'3333ull)));
        _Val = static_cast<_Ty>((_Val + (_Val >> 4)) & static_cast<_Ty>(0x0F0F'0F0F'0F0F'0F0Full));
        // Multiply by one in each byte, so that it will have the sum of all source bytes in the highest byte
        _Val = static_cast<_Ty>(_Val * static_cast<_Ty>(0x0101'0101'0101'0101ull));
        // Extract highest byte
        return static_cast<int>(_Val >> (_Digits - 8));
    }
}

#if ((defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)) || (defined(_M_X64) && !defined(_M_ARM64EC))) \
    && !defined(_M_CEE_PURE) && !defined(__CUDACC__)
#define _HAS_TZCNT_BSF_INTRINSICS 1
#else // ^^^ intrinsics available / intrinsics unavailable vvv
#define _HAS_TZCNT_BSF_INTRINSICS 0
#endif // ^^^ intrinsics unavailable ^^^

#if _HAS_TZCNT_BSF_INTRINSICS
#ifdef __clang__
#define _TZCNT_U32 __builtin_ia32_tzcnt_u32
#define _TZCNT_U64 __builtin_ia32_tzcnt_u64
#else // ^^^ __clang__ / !__clang__ vvv
#define _TZCNT_U32 _tzcnt_u32
#define _TZCNT_U64 _tzcnt_u64
#endif // __clang__

template <class _Ty>
_NODISCARD int _Countr_zero_tzcnt(const _Ty _Val) noexcept {
    constexpr int _Digits = _Unsigned_integer_digits<_Ty>;
    constexpr _Ty _Max    = static_cast<_Ty>(-1); // equal to (numeric_limits<_Ty>::max)()

    if constexpr (_Digits <= 32) {
        // Intended widening to int. This operation means that a narrow 0 will widen
        // to 0xFFFF....FFFF0... instead of 0. We need this to avoid counting all the zeros
        // of the wider type.
        return static_cast<int>(_TZCNT_U32(static_cast<unsigned int>(~_Max | _Val)));
    } else {
#ifdef _M_IX86
        const auto _Low = static_cast<unsigned int>(_Val);
        if (_Low == 0) {
            const unsigned int _High = _Val >> 32;
            return static_cast<int>(32 + _TZCNT_U32(_High));
        } else {
            return static_cast<int>(_TZCNT_U32(_Low));
        }
#else // ^^^ defined(_M_IX86) / !defined(_M_IX86) vvv
        return static_cast<int>(_TZCNT_U64(_Val));
#endif // ^^^ !defined(_M_IX86) ^^^
    }
}

#undef _TZCNT_U32
#undef _TZCNT_U64

template <class _Ty>
_NODISCARD int _Countr_zero_bsf(const _Ty _Val) noexcept {
    constexpr int _Digits = _Unsigned_integer_digits<_Ty>;
    constexpr _Ty _Max    = static_cast<_Ty>(-1); // equal to (numeric_limits<_Ty>::max)()

    unsigned long _Result;
    if constexpr (_Digits <= 32) {
        // Intended widening to int. This operation means that a narrow 0 will widen
        // to 0xFFFF....FFFF0... instead of 0. We need this to avoid counting all the zeros
        // of the wider type.
        if (!_BitScanForward(&_Result, static_cast<unsigned int>(~_Max | _Val))) {
            return _Digits;
        }
    } else {
#ifdef _M_IX86
        const auto _Low = static_cast<unsigned int>(_Val);
        if (_BitScanForward(&_Result, _Low)) {
            return static_cast<int>(_Result);
        }

        const unsigned int _High = _Val >> 32;
        if (!_BitScanForward(&_Result, _High)) {
            return _Digits;
        } else {
            return static_cast<int>(_Result + 32);
        }
#else // ^^^ defined(_M_IX86) / !defined(_M_IX86) vvv
        if (!_BitScanForward64(&_Result, _Val)) {
            return _Digits;
        }
#endif // ^^^ !defined(_M_IX86) ^^^
    }
    return static_cast<int>(_Result);
}

template <class _Ty>
_NODISCARD int _Checked_x86_x64_countr_zero(const _Ty _Val) noexcept {
#ifdef __AVX2__
    return _Countr_zero_tzcnt(_Val);
#else // ^^^ defined(__AVX2__) / !defined(__AVX2__) vvv
    const bool _Definitely_have_tzcnt = __isa_available >= _Stl_isa_available_avx2;
    if (_Definitely_have_tzcnt) {
        return _Countr_zero_tzcnt(_Val);
    } else {
        return _Countr_zero_bsf(_Val);
    }
#endif // ^^^ !defined(__AVX2__) ^^^
}

#endif // _HAS_TZCNT_BSF_INTRINSICS

#if (defined(_M_IX86) || defined(_M_X64) || defined(_M_ARM64)) && !defined(_M_CEE_PURE) && !defined(__CUDACC__)
#define _HAS_POPCNT_INTRINSICS 1
#if defined(__AVX__) || defined(_M_ARM64) || defined(_M_ARM64EC)
#define _POPCNT_INTRINSICS_ALWAYS_AVAILABLE 1
#else // ^^^ intrinsics always available / intrinsics not always available vvv
#define _POPCNT_INTRINSICS_ALWAYS_AVAILABLE 0
#endif // ^^^ intrinsics not always available ^^^
#else // ^^^ intrinsics available / intrinsics unavailable vvv
#define _HAS_POPCNT_INTRINSICS 0
#endif // ^^^ intrinsics unavailable ^^^

#if _HAS_POPCNT_INTRINSICS
template <class _Ty>
_NODISCARD int _Unchecked_popcount(const _Ty _Val) noexcept {
    constexpr int _Digits = _Unsigned_integer_digits<_Ty>;
    if constexpr (_Digits <= 16) {
        return static_cast<int>(__popcnt16(_Val));
    } else if constexpr (_Digits == 32) {
        return static_cast<int>(__popcnt(_Val));
    } else {
#ifdef _M_IX86
        return static_cast<int>(__popcnt(_Val >> 32) + __popcnt(static_cast<unsigned int>(_Val)));
#else // ^^^ defined(_M_IX86) / !defined(_M_IX86) vvv
        return static_cast<int>(__popcnt64(_Val));
#endif // ^^^ !defined(_M_IX86) ^^^
    }
}

template <class _Ty>
_NODISCARD int _Checked_popcount(const _Ty _Val) noexcept {
#if !_POPCNT_INTRINSICS_ALWAYS_AVAILABLE
    const bool _Definitely_have_popcnt = __isa_available >= _Stl_isa_available_sse42;
    if (!_Definitely_have_popcnt) {
        return _Popcount_fallback(_Val);
    }
#endif // ^^^ !_POPCNT_INTRINSICS_ALWAYS_AVAILABLE ^^^
    return _Unchecked_popcount(_Val);
}
#endif // ^^^ _HAS_POPCNT_INTRINSICS ^^^

template <class _Ty>
constexpr bool _Is_standard_unsigned_integer =
    _Is_any_of_v<remove_cv_t<_Ty>, unsigned char, unsigned short, unsigned int, unsigned long, unsigned long long>;

template <class _Ty, enable_if_t<_Is_standard_unsigned_integer<_Ty>, int> = 0>
_NODISCARD _CONSTEXPR20 int _Countr_zero(const _Ty _Val) noexcept {
#if _HAS_TZCNT_BSF_INTRINSICS
#if _HAS_CXX20
    if (!_STD is_constant_evaluated())
#endif // _HAS_CXX20
    {
        return _Checked_x86_x64_countr_zero(_Val);
    }
#endif // _HAS_TZCNT_BSF_INTRINSICS
    return _Countr_zero_fallback(_Val);
}

template <class _Ty, class _Fn>
constexpr decltype(auto) _Select_countr_zero_impl(_Fn _Callback) {
    // TRANSITION, DevCom-1527995: Lambdas in this function ensure inlining
#if _HAS_TZCNT_BSF_INTRINSICS && _HAS_CXX20
    if (!_STD is_constant_evaluated()) {
#ifdef __AVX2__
        return _Callback([](_Ty _Val) _STATIC_LAMBDA { return _Countr_zero_tzcnt(_Val); });
#else // ^^^ AVX2 / not AVX2 vvv
        const bool _Definitely_have_tzcnt = __isa_available >= _Stl_isa_available_avx2;
        if (_Definitely_have_tzcnt) {
            return _Callback([](_Ty _Val) _STATIC_LAMBDA { return _Countr_zero_tzcnt(_Val); });
        } else {
            return _Callback([](_Ty _Val) _STATIC_LAMBDA { return _Countr_zero_bsf(_Val); });
        }
#endif // ^^^ not AVX2 ^^^
    }
#endif // ^^^ _HAS_TZCNT_BSF_INTRINSICS && _HAS_CXX20 ^^^
    // C++17 constexpr gcd() calls this function, so it should be constexpr unless we detect runtime evaluation.
    return _Callback([](_Ty _Val) _STATIC_LAMBDA { return _Countr_zero_fallback(_Val); });
}

template <class _Ty, enable_if_t<_Is_standard_unsigned_integer<_Ty>, int> = 0>
_NODISCARD _CONSTEXPR20 int _Popcount(const _Ty _Val) noexcept {
#if _HAS_POPCNT_INTRINSICS
#if _HAS_CXX20
    if (!_STD is_constant_evaluated())
#endif // _HAS_CXX20
    {
        return _Checked_popcount(_Val);
    }
#endif // ^^^ _HAS_POPCNT_INTRINSICS ^^^
    return _Popcount_fallback(_Val);
}

template <class _Ty, class _Fn>
_CONSTEXPR20 decltype(auto) _Select_popcount_impl(_Fn _Callback) {
    // TRANSITION, DevCom-1527995: Lambdas in this function ensure inlining
#if _HAS_POPCNT_INTRINSICS
#if _HAS_CXX20
    if (!_STD is_constant_evaluated())
#endif // _HAS_CXX20
    {
#if !_POPCNT_INTRINSICS_ALWAYS_AVAILABLE
        const bool _Definitely_have_popcnt = __isa_available >= _Stl_isa_available_sse42;
        if (!_Definitely_have_popcnt) {
            return _Callback([](_Ty _Val) _STATIC_LAMBDA { return _Popcount_fallback(_Val); });
        }
#endif // ^^^ !_POPCNT_INTRINSICS_ALWAYS_AVAILABLE ^^^
        return _Callback([](_Ty _Val) _STATIC_LAMBDA { return _Unchecked_popcount(_Val); });
    }
#endif // ^^^ _HAS_POPCNT_INTRINSICS ^^^
    return _Callback([](_Ty _Val) _STATIC_LAMBDA { return _Popcount_fallback(_Val); });
}

#undef _HAS_POPCNT_INTRINSICS
#undef _HAS_TZCNT_BSF_INTRINSICS
#undef _POPCNT_INTRINSICS_ALWAYS_AVAILABLE

_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // _STL_COMPILER_PREPROCESSOR
#endif // __MSVC_BIT_UTILS_HPP
