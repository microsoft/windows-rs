// __msvc_int128.hpp internal header (core)

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef __MSVC_INT128_HPP
#define __MSVC_INT128_HPP

#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR
#include <__msvc_bit_utils.hpp>
#include <cstdint>
#include <limits>
#include <type_traits>

#include _STL_INTRIN_HEADER

#if _HAS_CXX20
#include <compare>
#include <concepts>
#define _TEMPLATE_CLASS_INTEGRAL(type) template <integral type>
#define _ZERO_OR_NO_INIT
#else // ^^^ _HAS_CXX20 / !_HAS_CXX20 vvv
#define _TEMPLATE_CLASS_INTEGRAL(type) template <class type, enable_if_t<is_integral_v<type>, int> = 0>
#define _ZERO_OR_NO_INIT \
    {                    \
    } // Trivial default initialization is not allowed in constexpr functions before C++20.
#endif // ^^^ !_HAS_CXX20 ^^^

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

_STD_BEGIN

#if defined(_M_X64) && !defined(_M_ARM64EC) && !defined(_M_CEE_PURE) && !defined(__CUDACC__)
#define _STL_128_INTRINSICS 1
#ifdef __clang__ // clang doesn't have _udiv128 / _div128
#define _STL_128_DIV_INTRINSICS 0
#else // ^^^ Clang / other vvv
#define _STL_128_DIV_INTRINSICS 1
#endif // ^^^ detect _udiv128 / _div128 ^^^
#else // ^^^ intrinsics available / intrinsics unavailable vvv
#define _STL_128_INTRINSICS     0
#define _STL_128_DIV_INTRINSICS 0
#endif // ^^^ intrinsics unavailable ^^^

template <class _Ty>
_NODISCARD constexpr int _Countl_zero_internal(const _Ty _Val) noexcept {
    _STL_INTERNAL_STATIC_ASSERT(_Is_standard_unsigned_integer<_Ty>);
#if _HAS_COUNTL_ZERO_INTRINSICS
#if (defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)) || (defined(_M_X64) && !defined(_M_ARM64EC))
    if (!_Is_constant_evaluated()) {
        return _Checked_x86_x64_countl_zero(_Val);
    }
#elif defined(_M_ARM) || defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)
    if (!_Is_constant_evaluated()) {
        return _Checked_arm_arm64_countl_zero(_Val);
    }
#endif // defined(_M_ARM) || defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)
#endif // _HAS_COUNTL_ZERO_INTRINSICS

    return _Countl_zero_fallback(_Val);
}

struct
#ifndef _M_ARM
    alignas(16)
#endif
        _Base128 {
    uint64_t _Word[2];

    constexpr void _Left_shift(const unsigned char _Count) noexcept {
        // _STL_INTERNAL_CHECK(_Count < 128);
        if (_Count == 0) {
            return;
        }

        if (_Count >= 64) {
            _Word[1] = _Word[0] << (_Count % 64);
            _Word[0] = 0;
            return;
        }

#if _STL_128_INTRINSICS
        if (!_Is_constant_evaluated()) {
            _Word[1] = __shiftleft128(_Word[0], _Word[1], _Count);
        } else
#endif // _STL_128_INTRINSICS
        {
            _Word[1] = (_Word[1] << _Count) | (_Word[0] >> (64 - _Count));
        }

        _Word[0] <<= _Count;
    }

    constexpr void _Unsigned_right_shift(const unsigned char _Count) noexcept {
        // _STL_INTERNAL_CHECK(_Count < 128);
        if (_Count == 0) {
            return;
        }

        if (_Count >= 64) {
            _Word[0] = _Word[1] >> (_Count % 64);
            _Word[1] = 0;
            return;
        }

#if _STL_128_INTRINSICS
        if (!_Is_constant_evaluated()) {
            _Word[0] = __shiftright128(_Word[0], _Word[1], _Count);
        } else
#endif // _STL_128_INTRINSICS
        {
            _Word[0] = (_Word[0] >> _Count) | (_Word[1] << (64 - _Count));
        }

        _Word[1] >>= _Count;
    }

    static constexpr unsigned char _AddCarry64(
        unsigned char _Carry, uint64_t _Left, uint64_t _Right, uint64_t& _Result) noexcept {
        // _STL_INTERNAL_CHECK(_Carry < 2);
#if _STL_128_INTRINSICS
        if (!_Is_constant_evaluated()) {
            return _addcarry_u64(_Carry, _Left, _Right, &_Result);
        }
#endif // _STL_128_INTRINSICS

        const uint64_t _Sum = _Left + _Right + _Carry;
        _Result             = _Sum;
        return _Carry ? _Sum <= _Left : _Sum < _Left;
    }

    static constexpr unsigned char _SubBorrow64(
        unsigned char _Carry, uint64_t _Left, uint64_t _Right, uint64_t& _Result) noexcept {
        // _STL_INTERNAL_CHECK(_Carry < 2);
#if _STL_128_INTRINSICS
        if (!_Is_constant_evaluated()) {
            return _subborrow_u64(_Carry, _Left, _Right, &_Result);
        }
#endif // _STL_128_INTRINSICS

        const auto _Difference = _Left - _Right - _Carry;
        _Result                = _Difference;
        return _Carry ? _Difference >= _Left : _Difference > _Left;
    }

    template <size_t __m, size_t __n>
    static constexpr void _Knuth_4_3_1_M(
        const uint32_t (&__u)[__m], const uint32_t (&__v)[__n], uint32_t (&__w)[__n + __m]) noexcept {
#ifdef _ENABLE_STL_INTERNAL_CHECK
        constexpr auto _Int_max = static_cast<size_t>(INT_MAX);
        _STL_INTERNAL_STATIC_ASSERT(__m <= _Int_max);
        _STL_INTERNAL_STATIC_ASSERT(__n <= _Int_max);
#endif // defined(_ENABLE_STL_INTERNAL_CHECK)

        for (auto& _Elem : __w) {
            _Elem = 0;
        }

        for (int __j = 0; __j < static_cast<int>(__n); ++__j) {
            // stash Knuth's `k` in the lower 32 bits of __t
            uint64_t __t = 0;
            for (int __i = 0; __i < static_cast<int>(__m); ++__i) {
                __t += static_cast<uint64_t>(__u[__i]) * __v[__j] + __w[__i + __j];
                __w[__i + __j] = static_cast<uint32_t>(__t);
                __t >>= 32;
            }
            __w[__j + __m] = static_cast<uint32_t>(__t);
        }
    }

    _NODISCARD static constexpr uint64_t _UMul128(
        const uint64_t _Left, const uint64_t _Right, uint64_t& _High_result) noexcept {
#if _STL_128_INTRINSICS
        if (!_Is_constant_evaluated()) {
            return _umul128(_Left, _Right, &_High_result);
        }
#endif // _STL_128_INTRINSICS

        const uint32_t __u[2] = {
            static_cast<uint32_t>(_Left),
            static_cast<uint32_t>(_Left >> 32),
        };
        const uint32_t __v[2] = {
            static_cast<uint32_t>(_Right),
            static_cast<uint32_t>(_Right >> 32),
        };
        uint32_t __w[4] _ZERO_OR_NO_INIT;

        // multiply 2-digit numbers with 4-digit result in base 2^32
        _Knuth_4_3_1_M(__u, __v, __w);

        _High_result = (static_cast<uint64_t>(__w[3]) << 32) | __w[2];
        return (static_cast<uint64_t>(__w[1]) << 32) | __w[0];
    }

    static constexpr void _Knuth_4_3_1_D(uint32_t* const __u, const size_t __u_size, const uint32_t* const __v,
        const size_t __v_size, uint32_t* const __q) noexcept {
        // Pre: __u + [0, __u_size), __v + [0, __v_size), and __q + [0, __u_size - __v_size) are all valid ranges
        // constexpr auto _Int_max = static_cast<size_t>(INT_MAX);
        // _STL_INTERNAL_CHECK(__v_size <= _Int_max);
        const int __n = static_cast<int>(__v_size);
        // _STL_INTERNAL_CHECK(__u_size > __v_size);
        // _STL_INTERNAL_CHECK(__u_size <= _Int_max);
        const int __m = static_cast<int>(__u_size - __v_size - 1);
        // _STL_INTERNAL_CHECK(__v[__n - 1] >> 31 != 0); // Arguments are already normalized

        for (int __j = __m; __j >= 0; --__j) {
            const auto _Two_digits = (static_cast<uint64_t>(__u[__j + __n]) << 32) | __u[__j + __n - 1];
            auto __qhat            = _Two_digits / __v[__n - 1];
            auto __rhat            = _Two_digits % __v[__n - 1];

            while ((__qhat >> 32) != 0
                   || static_cast<uint32_t>(__qhat) * static_cast<uint64_t>(__v[__n - 2])
                          > ((__rhat << 32) | __u[__j + __n - 2])) {
                --__qhat;
                __rhat += __v[__n - 1];
                if ((__rhat >> 32) != 0) {
                    break;
                }
            }

            int64_t __k = 0;
            int64_t __t _ZERO_OR_NO_INIT;
            for (int __i = 0; __i < __n; ++__i) {
                const auto _Prod = static_cast<uint32_t>(__qhat) * static_cast<uint64_t>(__v[__i]);
                __t              = __u[__i + __j] - __k - static_cast<uint32_t>(_Prod);
                __u[__i + __j]   = static_cast<uint32_t>(__t);
                __k              = static_cast<int64_t>(_Prod >> 32) - (__t >> 32);
            }
            __t            = __u[__j + __n] - __k;
            __u[__j + __n] = static_cast<uint32_t>(__t);

            __q[__j] = static_cast<uint32_t>(__qhat);
            if (__t < 0) {
                --__q[__j];
                __k = 0;
                for (int __i = 0; __i < __n; ++__i) {
                    __t            = __u[__i + __j] + __k + __v[__i];
                    __u[__i + __j] = static_cast<uint32_t>(__t);
                    __k            = __t >> 32;
                }
                __u[__j + __n] += static_cast<int32_t>(__k);
            }
        }

        // quotient is in __q, normalized remainder is in __u
    }

    _NODISCARD static constexpr uint64_t _UDiv128(
        uint64_t _High, uint64_t _Low, uint64_t _Div, uint64_t& _Remainder) noexcept {
        // _STL_INTERNAL_CHECK(_High < _Div);

#if _STL_128_DIV_INTRINSICS
        if (!_Is_constant_evaluated()) {
            return _udiv128(_High, _Low, _Div, &_Remainder);
        }
#endif // _STL_128_DIV_INTRINSICS

        const auto __d = _Countl_zero_internal(static_cast<uint32_t>(_Div >> 32));
        if (__d >= 32) { // _Div < 2^32
            auto _Rem    = (_High << 32) | (_Low >> 32);
            auto _Result = _Rem / static_cast<uint32_t>(_Div);
            _Rem         = ((_Rem % static_cast<uint32_t>(_Div)) << 32) | static_cast<uint32_t>(_Low);
            _Result      = (_Result << 32) | (_Rem / static_cast<uint32_t>(_Div));
            _Remainder   = _Rem % static_cast<uint32_t>(_Div);
            return _Result;
        }

        uint32_t __u[5] = {
            static_cast<uint32_t>(_Low << __d),
            static_cast<uint32_t>(_Low >> (32 - __d)),
            static_cast<uint32_t>(_High << __d),
            static_cast<uint32_t>(_High >> (32 - __d)),
            0,
        };
        if (__d != 0) {
            __u[2] |= static_cast<uint32_t>(_Low >> (64 - __d));
            __u[4] |= static_cast<uint32_t>(_High >> (64 - __d));
        }

        uint32_t __v[2] = {
            static_cast<uint32_t>(_Div << __d),
            static_cast<uint32_t>(_Div >> (32 - __d)),
        };
        uint32_t __q[3] _ZERO_OR_NO_INIT;

        _Knuth_4_3_1_D(__u, 5, __v, 2, __q);
        // _STL_INTERNAL_CHECK(__u[4] == 0);
        // _STL_INTERNAL_CHECK(__u[3] == 0);
        // _STL_INTERNAL_CHECK(__u[2] == 0);
        _Remainder = (static_cast<uint64_t>(__u[1]) << (32 - __d)) | (__u[0] >> __d);

        // _STL_INTERNAL_CHECK(__q[2] == 0);
        return (static_cast<uint64_t>(__q[1]) << 32) | __q[0];
    }

    constexpr _Base128() noexcept : _Word{} {}

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    constexpr _Base128(const _Ty _Val) noexcept : _Word{static_cast<uint64_t>(_Val)} {
#if _HAS_CXX20
        if constexpr (signed_integral<_Ty>)
#else
        if constexpr (is_signed_v<_Ty>)
#endif
        {
            if (_Val < 0) {
                _Word[1] = ~0ull;
            }
        }
    }

    constexpr explicit _Base128(const uint64_t _Low, const uint64_t _High) noexcept : _Word{_Low, _High} {}

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    _NODISCARD constexpr explicit operator _Ty() const noexcept {
        return static_cast<_Ty>(_Word[0]);
    }

    _NODISCARD constexpr explicit operator bool() const noexcept {
        return (_Word[0] | _Word[1]) != 0;
    }

#if _HAS_CXX20
    _NODISCARD friend constexpr bool operator==(const _Base128&, const _Base128&) noexcept = default;
#else // ^^^ _HAS_CXX20 / !_HAS_CXX20 vvv
    _NODISCARD friend constexpr bool operator==(const _Base128& _Left, const _Base128& _Right) noexcept {
        return _Left._Word[0] == _Right._Word[0] && _Left._Word[1] == _Right._Word[1];
    }

    _NODISCARD friend constexpr bool operator!=(const _Base128& _Left, const _Base128& _Right) noexcept {
        return !(_Left == _Right);
    }
#endif // ^^^ !_HAS_CXX20 ^^^

    _NODISCARD friend constexpr bool operator<(const _Base128& _Left, const _Base128& _Right) noexcept {
        if (_Left._Word[1] < _Right._Word[1]) {
            return true;
        }

        if (_Left._Word[1] > _Right._Word[1]) {
            return false;
        }
        return _Left._Word[0] < _Right._Word[0];
    }
    _NODISCARD friend constexpr bool operator>(const _Base128& _Left, const _Base128& _Right) noexcept {
        return _Right < _Left;
    }
    _NODISCARD friend constexpr bool operator<=(const _Base128& _Left, const _Base128& _Right) noexcept {
        return !(_Right < _Left);
    }
    _NODISCARD friend constexpr bool operator>=(const _Base128& _Left, const _Base128& _Right) noexcept {
        return !(_Left < _Right);
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    _NODISCARD friend constexpr _Ty operator<<(const _Ty _Left, const _Base128& _Right) noexcept {
        return _Left << _Right._Word[0];
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    _NODISCARD friend constexpr _Ty operator>>(const _Ty _Left, const _Base128& _Right) noexcept {
        return _Left >> _Right._Word[0];
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    constexpr _Base128& operator<<=(const _Ty _Count) noexcept {
        _Left_shift(static_cast<unsigned char>(_Count));
        return *this;
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    friend constexpr _Ty& operator<<=(_Ty& _Left, const _Base128& _Right) noexcept {
        _Left <<= _Right._Word[0];
        return _Left;
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    constexpr _Base128& operator>>=(const _Ty _Count) noexcept {
        _Unsigned_right_shift(static_cast<unsigned char>(_Count));
        return *this;
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    friend constexpr _Ty& operator>>=(_Ty& _Left, const _Base128& _Right) noexcept {
        _Left >>= _Right._Word[0];
        return _Left;
    }

    constexpr _Base128& operator++() noexcept {
        if (++_Word[0] == 0) {
            ++_Word[1];
        }
        return *this;
    }
    constexpr _Base128 operator++(int) noexcept {
        auto _Tmp = *this;
        ++*this;
        return _Tmp;
    }

    constexpr _Base128& operator--() noexcept {
        if (_Word[0]-- == 0) {
            --_Word[1];
        }
        return *this;
    }
    constexpr _Base128 operator--(int) noexcept {
        auto _Tmp = *this;
        --*this;
        return _Tmp;
    }

    _NODISCARD static constexpr _Base128 _Multiply(const _Base128& _Left, const _Base128& _Right) noexcept {
        _Base128 _Result;
        _Result._Word[0] = _UMul128(_Left._Word[0], _Right._Word[0], _Result._Word[1]);
        _Result._Word[1] += _Left._Word[0] * _Right._Word[1];
        _Result._Word[1] += _Left._Word[1] * _Right._Word[0];
        return _Result;
    }

#if !_STL_128_DIV_INTRINSICS
    _NODISCARD static constexpr _Base128 _Divide(const _Base128& _Num, const uint32_t _Den) noexcept {
        _Base128 _Result;
        _Result._Word[1] = _Num._Word[1] / _Den;
        uint64_t _Rem    = ((_Num._Word[1] % _Den) << 32) | (_Num._Word[0] >> 32);
        _Result._Word[0] = (_Rem / _Den) << 32;
        _Rem             = ((_Rem % _Den) << 32) | static_cast<uint32_t>(_Num._Word[0]);
        _Result._Word[0] |= static_cast<uint32_t>(_Rem / _Den);
        return _Result;
    }
#endif // !_STL_128_DIV_INTRINSICS
    _NODISCARD static constexpr _Base128 _Divide(const _Base128& _Num, const uint64_t _Den) noexcept {
        _Base128 _Result;
        _Result._Word[1] = _Num._Word[1] / _Den;
        uint64_t _Rem    = _Num._Word[1] % _Den;
        _Result._Word[0] = _UDiv128(_Rem, _Num._Word[0], _Den, _Rem);
        return _Result;
    }
    _NODISCARD static constexpr _Base128 _Divide(_Base128 _Num, _Base128 _Den) noexcept {
        // establish _Den < _Num and _Num._Word[1] > 0
        if (_Den._Word[1] >= _Num._Word[1]) {
            if (_Den._Word[1] > _Num._Word[1]) {
                return 0;
            }

            return _Num._Word[1] == 0 ? _Num._Word[0] / _Den._Word[0] : _Num._Word[0] >= _Den._Word[0];
        }

        // establish _Den has more than 1 non-zero "digit"
        if (_Den._Word[1] == 0) {
#if !_STL_128_DIV_INTRINSICS
            if (_Den._Word[0] < (1ull << 32)) {
                return _Divide(_Num, static_cast<uint32_t>(_Den._Word[0]));
            } else
#endif // !_STL_128_DIV_INTRINSICS
            {
                return _Divide(_Num, _Den._Word[0]);
            }
        }

#if _STL_128_INTRINSICS
        // Knuth 4.3.1D, 2-digit by 2-digit divide in base 2^64
        // _STL_INTERNAL_CHECK(_Den._Word[1] != 0);
        // _STL_INTERNAL_CHECK(_Num._Word[1] > _Den._Word[1]);
        // Normalize by shifting both left until _Den's high bit is set (So _Den's high digit is >= b / 2)
        const auto __d = _Countl_zero_internal(_Den._Word[1]);
        _Den <<= __d;
        auto _High_digit = __d == 0 ? 0 : _Num._Word[1] >> (64 - __d); // This creates a third digit for _Num
        _Num <<= __d;

        _Base128 __qhat;
        __qhat._Word[1] = _High_digit >= _Den._Word[1];
        uint64_t __rhat _ZERO_OR_NO_INIT;
        __qhat._Word[0] = _UDiv128(_High_digit >= _Den._Word[1] ? _High_digit - _Den._Word[1] : _High_digit,
            _Num._Word[1], _Den._Word[1], __rhat);

        for (;;) {
            if (__qhat._Word[1] > 0) {
                --__qhat;
            } else {
                _Base128 _Prod;
                _Prod._Word[0] = _UMul128(__qhat._Word[0], _Den._Word[0], _Prod._Word[1]);
                if (_Prod <= _Base128{_Num._Word[0], __rhat}) {
                    break;
                }
                --__qhat._Word[0];
            }

            const auto _Sum = __rhat + _Den._Word[1];
            if (__rhat > _Sum) {
                break;
            }
            __rhat = _Sum;
        }
        // _STL_INTERNAL_CHECK(__qhat._Word[1] == 0);

        // [_High_digit | _Num] -= __qhat * _Den [Since __qhat < b, this is 3-digit - 1-digit * 2-digit]
        uint64_t _Prod0_hi _ZERO_OR_NO_INIT;
        uint64_t _Prod_lo = _UMul128(__qhat._Word[0], _Den._Word[0], _Prod0_hi);
        auto _Borrow      = _SubBorrow64(0, _Num._Word[0], _Prod_lo, _Num._Word[0]);
        uint64_t _Prod1_hi _ZERO_OR_NO_INIT;
        _Prod_lo = _UMul128(__qhat._Word[0], _Den._Word[1], _Prod1_hi);
        _Prod1_hi += _AddCarry64(0, _Prod_lo, _Prod0_hi, _Prod_lo);
        _Borrow = _SubBorrow64(_Borrow, _Num._Word[1], _Prod_lo, _Num._Word[1]);
        _Borrow = _SubBorrow64(_Borrow, _High_digit, _Prod1_hi, _High_digit);
        if (_Borrow) {
            --__qhat._Word[0];
        }
        return __qhat;
#else // ^^^ 128-bit intrinsics / no such intrinsics vvv
        auto __d                   = _Countl_zero_internal(_Den._Word[1]);
        const bool _Three_word_den = __d >= 32;
        __d &= 31;
        uint32_t __u[5]{
            static_cast<uint32_t>(_Num._Word[0] << __d),
            static_cast<uint32_t>(_Num._Word[0] >> (32 - __d)),
            static_cast<uint32_t>(_Num._Word[1] << __d),
            static_cast<uint32_t>(_Num._Word[1] >> (32 - __d)),
            0,
        };
        uint32_t __v[4] = {
            static_cast<uint32_t>(_Den._Word[0] << __d),
            static_cast<uint32_t>(_Den._Word[0] >> (32 - __d)),
            static_cast<uint32_t>(_Den._Word[1] << __d),
            static_cast<uint32_t>(_Den._Word[1] >> (32 - __d)),
        };
        if (__d != 0) {
            __u[2] |= _Num._Word[0] >> (64 - __d);
            __u[4] |= _Num._Word[1] >> (64 - __d);
            __v[2] |= _Den._Word[0] >> (64 - __d);
        }

        uint32_t __q[2] _ZERO_OR_NO_INIT;
        if (_Three_word_den) {
            // 4-digit by 3-digit base 2^32 division
            _Knuth_4_3_1_D(__u, 5, __v, 3, __q);
        } else {
            // 4-digit by 4-digit base 2^32 division
            _Knuth_4_3_1_D(__u, 5, __v, 4, __q);
            __q[1] = 0;
        }

        return (static_cast<uint64_t>(__q[1]) << 32) | __q[0];
#endif // _STL_128_INTRINSICS
    }

#if !_STL_128_DIV_INTRINSICS
    _NODISCARD static constexpr _Base128 _Modulo(const _Base128& _Num, const uint32_t _Den) noexcept {
        uint64_t _Rem = _Num._Word[1];
        _Rem          = ((_Rem % _Den) << 32) | (_Num._Word[0] >> 32);
        _Rem          = ((_Rem % _Den) << 32) | static_cast<uint32_t>(_Num._Word[0]);
        return _Rem % _Den;
    }
#endif // !_STL_128_DIV_INTRINSICS
    _NODISCARD static constexpr _Base128 _Modulo(const _Base128& _Num, const uint64_t _Den) noexcept {
        uint64_t _Rem _ZERO_OR_NO_INIT;
        (void) _UDiv128(_Num._Word[1] % _Den, _Num._Word[0], _Den, _Rem);
        return _Rem;
    }
    _NODISCARD static constexpr _Base128 _Modulo(_Base128 _Num, _Base128 _Den) noexcept {
        // establish _Den < _Num and _Num._Word[1] > 0
        if (_Den._Word[1] >= _Num._Word[1]) {
            if (_Den._Word[1] > _Num._Word[1]) {
                return _Num;
            }

            if (_Den._Word[0] <= _Num._Word[0]) {
                return _Num._Word[1] == 0 ? _Num._Word[0] % _Den._Word[0] : _Num._Word[0] - _Den._Word[0];
            }

            return _Num;
        }

        // establish _Den has more than 1 non-zero "digit"
        if (_Den._Word[1] == 0) {
#if !_STL_128_DIV_INTRINSICS
            if (_Den._Word[0] < (1ull << 32)) {
                return _Modulo(_Num, static_cast<uint32_t>(_Den._Word[0]));
            } else
#endif // !_STL_128_DIV_INTRINSICS
            {
                return _Modulo(_Num, _Den._Word[0]);
            }
        }

#if _STL_128_INTRINSICS
        // Knuth 4.3.1D, 2-digit by 2-digit divide in base 2^64
        // _STL_INTERNAL_CHECK(_Den._Word[1] != 0);
        // _STL_INTERNAL_CHECK(_Num._Word[1] > _Den._Word[1]);
        // Normalize by shifting both left until _Den's high bit is set (So _Den's high digit is >= b / 2)
        const auto __d = _Countl_zero_internal(_Den._Word[1]);
        _Den <<= __d;
        auto _High_digit = __d == 0 ? 0 : _Num._Word[1] >> (64 - __d); // This creates a third digit for _Num
        _Num <<= __d;

        uint64_t __qhat_high = _High_digit >= _Den._Word[1];
        uint64_t __rhat _ZERO_OR_NO_INIT;
        uint64_t __qhat = _UDiv128(_High_digit >= _Den._Word[1] ? _High_digit - _Den._Word[1] : _High_digit,
            _Num._Word[1], _Den._Word[1], __rhat);

        for (;;) {
            if (__qhat_high > 0) {
                if (__qhat-- == 0) {
                    --__qhat_high;
                }
            } else {
                _Base128 _Prod;
                _Prod._Word[0] = _UMul128(__qhat, _Den._Word[0], _Prod._Word[1]);
                if (_Prod <= _Base128{_Num._Word[0], __rhat}) {
                    break;
                }
                --__qhat;
            }

            const auto _Sum = __rhat + _Den._Word[1];
            if (__rhat > _Sum) {
                break;
            }
            __rhat = _Sum;
            // The addition didn't overflow, so `__rhat < b` holds
        }
        // _STL_INTERNAL_CHECK(__qhat_high == 0);

        // [_High_digit | _Num] -= __qhat * _Den [3-digit - 1-digit * 2-digit]
        uint64_t _Prod0_hi _ZERO_OR_NO_INIT;
        uint64_t _Prod_lo = _UMul128(__qhat, _Den._Word[0], _Prod0_hi);
        auto _Borrow      = _SubBorrow64(0, _Num._Word[0], _Prod_lo, _Num._Word[0]);
        uint64_t _Prod1_hi _ZERO_OR_NO_INIT;
        _Prod_lo = _UMul128(__qhat, _Den._Word[1], _Prod1_hi);
        _Prod1_hi += _AddCarry64(0, _Prod_lo, _Prod0_hi, _Prod_lo);
        _Borrow = _SubBorrow64(_Borrow, _Num._Word[1], _Prod_lo, _Num._Word[1]);
        _Borrow = _SubBorrow64(_Borrow, _High_digit, _Prod1_hi, _High_digit);
        if (_Borrow) {
            auto _Carry = _AddCarry64(0, _Num._Word[0], _Den._Word[0], _Num._Word[0]);
            (void) _AddCarry64(_Carry, _Num._Word[1], _Den._Word[1], _Num._Word[1]);
        }
#else // ^^^ 128-bit intrinsics / no such intrinsics vvv
        auto __d                   = _Countl_zero_internal(_Den._Word[1]);
        const bool _Three_word_den = __d >= 32;
        __d &= 31;
        uint32_t __u[5]{
            static_cast<uint32_t>(_Num._Word[0] << __d),
            static_cast<uint32_t>(_Num._Word[0] >> (32 - __d)),
            static_cast<uint32_t>(_Num._Word[1] << __d),
            static_cast<uint32_t>(_Num._Word[1] >> (32 - __d)),
            0,
        };
        uint32_t __v[4] = {
            static_cast<uint32_t>(_Den._Word[0] << __d),
            static_cast<uint32_t>(_Den._Word[0] >> (32 - __d)),
            static_cast<uint32_t>(_Den._Word[1] << __d),
            static_cast<uint32_t>(_Den._Word[1] >> (32 - __d)),
        };
        if (__d != 0) {
            __u[2] |= _Num._Word[0] >> (64 - __d);
            __u[4] |= _Num._Word[1] >> (64 - __d);
            __v[2] |= _Den._Word[0] >> (64 - __d);
        }

        uint32_t __q[2] _ZERO_OR_NO_INIT;
        if (_Three_word_den) {
            // 4-digit by 3-digit base 2^32 division
            _Knuth_4_3_1_D(__u, 5, __v, 3, __q);
            // _STL_INTERNAL_CHECK(__u[3] == 0);
        } else {
            // 4-digit by 4-digit base 2^32 division
            _Knuth_4_3_1_D(__u, 5, __v, 4, __q);
        }
        // _STL_INTERNAL_CHECK(__u[4] == 0);

        _Num._Word[0] = (static_cast<uint64_t>(__u[1]) << 32) | __u[0];
        _Num._Word[1] = (static_cast<uint64_t>(__u[3]) << 32) | __u[2];
#endif // _STL_128_INTRINSICS
        _Num >>= __d;
        return _Num;
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    friend constexpr _Ty& operator&=(_Ty& _Left, const _Base128& _Right) noexcept {
        _Left &= _Right._Word[0];
        return _Left;
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    friend constexpr _Ty& operator^=(_Ty& _Left, const _Base128& _Right) noexcept {
        _Left ^= _Right._Word[0];
        return _Left;
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    friend constexpr _Ty& operator|=(_Ty& _Left, const _Base128& _Right) noexcept {
        _Left |= _Right._Word[0];
        return _Left;
    }
};

struct _Signed128;

struct _Unsigned128 : _Base128 {
    using _Signed_type   = _Signed128;
    using _Unsigned_type = _Unsigned128;

#if !_HAS_CXX17 || (_HAS_CXX20 && !defined(__clang__) && !defined(__EDG__)) // TRANSITION, DevCom-10729775
    constexpr _Unsigned128() noexcept : _Base128{} {}
#endif // ^^^ workaround for C++20 MSVC modules and header units; should be guarded for !_HAS_CXX17 only ^^^

    using _Base128::_Base128;
    constexpr explicit _Unsigned128(const _Base128& _That) noexcept : _Base128{_That} {}

    constexpr _Unsigned128& operator=(const _Base128& _That) noexcept {
        _Base128::operator=(_That);
        return *this;
    }

#if _HAS_CXX20
    _NODISCARD friend constexpr strong_ordering operator<=>(
        const _Unsigned128& _Left, const _Unsigned128& _Right) noexcept {
        strong_ordering _Ord = _Left._Word[1] <=> _Right._Word[1];
        if (_Ord == strong_ordering::equal) {
            _Ord = _Left._Word[0] <=> _Right._Word[0];
        }
        return _Ord;
    }
#else // ^^^ _HAS_CXX20 / !_HAS_CXX20 vvv
    _NODISCARD friend constexpr bool operator<(const _Unsigned128& _Left, const _Unsigned128& _Right) noexcept {
        if (_Left._Word[1] < _Right._Word[1]) {
            return true;
        }

        if (_Right._Word[1] < _Left._Word[1]) {
            return false;
        }

        return _Left._Word[0] < _Right._Word[0];
    }

    _NODISCARD friend constexpr bool operator>(const _Unsigned128& _Left, const _Unsigned128& _Right) noexcept {
        return _Right < _Left;
    }

    _NODISCARD friend constexpr bool operator<=(const _Unsigned128& _Left, const _Unsigned128& _Right) noexcept {
        return !(_Right < _Left);
    }

    _NODISCARD friend constexpr bool operator>=(const _Unsigned128& _Left, const _Unsigned128& _Right) noexcept {
        return !(_Left < _Right);
    }
#endif // ^^^ !_HAS_CXX20 ^^^

    _NODISCARD friend constexpr _Unsigned128 operator<<(const _Unsigned128& _Left, const _Base128& _Right) noexcept {
        auto _Tmp{_Left};
        _Tmp._Left_shift(static_cast<unsigned char>(_Right._Word[0]));
        return _Tmp;
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    constexpr _Unsigned128& operator<<=(const _Ty _Count) noexcept {
        _Left_shift(static_cast<unsigned char>(_Count));
        return *this;
    }
    constexpr _Unsigned128& operator<<=(const _Base128& _Count) noexcept {
        _Left_shift(static_cast<unsigned char>(_Count._Word[0]));
        return *this;
    }

    _NODISCARD friend constexpr _Unsigned128 operator>>(const _Unsigned128& _Left, const _Base128& _Right) noexcept {
        auto _Tmp{_Left};
        _Tmp._Unsigned_right_shift(static_cast<unsigned char>(_Right._Word[0]));
        return _Tmp;
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    constexpr _Unsigned128& operator>>=(const _Ty _Count) noexcept {
        _Unsigned_right_shift(static_cast<unsigned char>(_Count));
        return *this;
    }
    constexpr _Unsigned128& operator>>=(const _Base128& _Count) noexcept {
        _Unsigned_right_shift(static_cast<unsigned char>(_Count._Word[0]));
        return *this;
    }

    constexpr _Unsigned128& operator++() noexcept {
        if (++_Word[0] == 0) {
            ++_Word[1];
        }
        return *this;
    }
    constexpr _Unsigned128 operator++(int) noexcept {
        auto _Tmp = *this;
        ++*this;
        return _Tmp;
    }

    constexpr _Unsigned128& operator--() noexcept {
        if (_Word[0]-- == 0) {
            --_Word[1];
        }
        return *this;
    }
    constexpr _Unsigned128 operator--(int) noexcept {
        auto _Tmp = *this;
        --*this;
        return _Tmp;
    }

    _NODISCARD constexpr _Unsigned128 operator+() const noexcept {
        return *this;
    }

    _NODISCARD constexpr _Unsigned128 operator-() const noexcept {
        return _Unsigned128{} - *this;
    }

    _NODISCARD constexpr _Unsigned128 operator~() const noexcept {
        return _Unsigned128{~_Word[0], ~_Word[1]};
    }

    _NODISCARD friend constexpr _Unsigned128 operator+(const _Base128& _Left, const _Base128& _Right) noexcept {
        _Unsigned128 _Result;
        const auto _Carry = _AddCarry64(0, _Left._Word[0], _Right._Word[0], _Result._Word[0]);
        _AddCarry64(_Carry, _Left._Word[1], _Right._Word[1], _Result._Word[1]);
        return _Result;
    }

    constexpr _Unsigned128& operator+=(const _Base128& _That) noexcept {
        const auto _Carry = _AddCarry64(0, _Word[0], _That._Word[0], _Word[0]);
        _AddCarry64(_Carry, _Word[1], _That._Word[1], _Word[1]);
        return *this;
    }
    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    friend constexpr _Ty& operator+=(_Ty& _Left, const _Unsigned128& _Right) noexcept {
        _Left += _Right._Word[0];
        return _Left;
    }

    _NODISCARD friend constexpr _Unsigned128 operator-(const _Base128& _Left, const _Base128& _Right) noexcept {
        _Unsigned128 _Result;
        const auto _Borrow = _SubBorrow64(0, _Left._Word[0], _Right._Word[0], _Result._Word[0]);
        _SubBorrow64(_Borrow, _Left._Word[1], _Right._Word[1], _Result._Word[1]);
        return _Result;
    }

    constexpr _Unsigned128& operator-=(const _Base128& _That) noexcept {
        const auto _Borrow = _SubBorrow64(0, _Word[0], _That._Word[0], _Word[0]);
        _SubBorrow64(_Borrow, _Word[1], _That._Word[1], _Word[1]);
        return *this;
    }
    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    friend constexpr _Ty& operator-=(_Ty& _Left, const _Unsigned128& _Right) noexcept {
        _Left -= _Right._Word[0];
        return _Left;
    }

    _NODISCARD friend constexpr _Unsigned128 operator*(const _Base128& _Left, const _Base128& _Right) noexcept {
        return _Unsigned128{_Base128::_Multiply(_Left, _Right)};
    }

    constexpr _Unsigned128& operator*=(const _Base128& _That) noexcept {
        *this = *this * _That;
        return *this;
    }
    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    friend constexpr _Ty& operator*=(_Ty& _Left, const _Unsigned128& _Right) noexcept {
        _Left *= _Right._Word[0];
        return _Left;
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    _NODISCARD friend constexpr _Unsigned128 operator/(const _Unsigned128& _Num, const _Ty _Den) noexcept {
#if !_STL_128_DIV_INTRINSICS
        if constexpr (sizeof(_Ty) <= 4) {
            return _Unsigned128{_Base128::_Divide(_Num, static_cast<uint32_t>(_Den))};
        } else
#endif // !_STL_128_DIV_INTRINSICS
        {
            return _Unsigned128{_Base128::_Divide(_Num, static_cast<uint64_t>(_Den))};
        }
    }
    _NODISCARD friend constexpr _Unsigned128 operator/(const _Base128& _Num, const _Base128& _Den) noexcept {
        return _Unsigned128{_Base128::_Divide(_Num, _Den)};
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    constexpr _Unsigned128& operator/=(const _Ty _That) noexcept {
#if !_STL_128_DIV_INTRINSICS
        if constexpr (sizeof(_Ty) <= 4) {
            *this = _Unsigned128{_Base128::_Divide(*this, static_cast<uint32_t>(_That))};
        } else
#endif // !_STL_128_DIV_INTRINSICS
        {
            *this = _Unsigned128{_Base128::_Divide(*this, static_cast<uint64_t>(_That))};
        }
        return *this;
    }
    constexpr _Unsigned128& operator/=(const _Base128& _That) noexcept {
        *this = _Unsigned128{_Base128::_Divide(*this, _That)};
        return *this;
    }
    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    friend constexpr _Ty& operator/=(_Ty& _Left, const _Unsigned128& _Right) noexcept {
        if (_Right._Word[1] != 0) {
            _Left = 0;
        } else {
            _Left /= _Right._Word[0];
        }
        return _Left;
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    _NODISCARD friend constexpr _Unsigned128 operator%(const _Base128& _Num, const _Ty _Den) noexcept {
#if !_STL_128_DIV_INTRINSICS
        if constexpr (sizeof(_Ty) <= 4) {
            return _Unsigned128{_Base128::_Modulo(_Num, static_cast<uint32_t>(_Den))};
        } else
#endif // !_STL_128_DIV_INTRINSICS
        {
            return _Unsigned128{_Base128::_Modulo(_Num, static_cast<uint64_t>(_Den))};
        }
    }
    _NODISCARD friend constexpr _Unsigned128 operator%(const _Base128& _Num, const _Base128& _Den) noexcept {
        return _Unsigned128{_Base128::_Modulo(_Num, _Den)};
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    constexpr _Unsigned128& operator%=(const _Ty _Den) noexcept {
        *this = *this % _Den;
        return *this;
    }
    constexpr _Unsigned128& operator%=(const _Base128& _Den) noexcept {
        *this = *this % _Den;
        return *this;
    }
    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    friend constexpr _Ty& operator%=(_Ty& _Left, const _Unsigned128& _Right) noexcept {
        if (_Right._Word[1] == 0) {
            _Left %= _Right._Word[0];
        }
        return _Left;
    }

    _NODISCARD friend constexpr _Unsigned128 operator&(const _Base128& _Left, const _Base128& _Right) noexcept {
        return _Unsigned128{_Left._Word[0] & _Right._Word[0], _Left._Word[1] & _Right._Word[1]};
    }

    constexpr _Unsigned128& operator&=(const _Base128& _That) noexcept {
        _Word[0] &= _That._Word[0];
        _Word[1] &= _That._Word[1];
        return *this;
    }

    _NODISCARD friend constexpr _Unsigned128 operator^(const _Base128& _Left, const _Base128& _Right) noexcept {
        return _Unsigned128{_Left._Word[0] ^ _Right._Word[0], _Left._Word[1] ^ _Right._Word[1]};
    }

    constexpr _Unsigned128& operator^=(const _Base128& _That) noexcept {
        _Word[0] ^= _That._Word[0];
        _Word[1] ^= _That._Word[1];
        return *this;
    }

    _NODISCARD friend constexpr _Unsigned128 operator|(const _Base128& _Left, const _Base128& _Right) noexcept {
        return _Unsigned128{_Left._Word[0] | _Right._Word[0], _Left._Word[1] | _Right._Word[1]};
    }

    constexpr _Unsigned128& operator|=(const _Base128& _That) noexcept {
        _Word[0] |= _That._Word[0];
        _Word[1] |= _That._Word[1];
        return *this;
    }
};

template <>
class numeric_limits<_Unsigned128> : public _Num_int_base {
public:
    _NODISCARD static constexpr _Unsigned128(min)() noexcept {
        return 0;
    }

    _NODISCARD static constexpr _Unsigned128(max)() noexcept {
        return _Unsigned128{~0ull, ~0ull};
    }

    _NODISCARD static constexpr _Unsigned128 lowest() noexcept {
        return (min) ();
    }

    _NODISCARD static constexpr _Unsigned128 epsilon() noexcept {
        return 0;
    }

    _NODISCARD static constexpr _Unsigned128 round_error() noexcept {
        return 0;
    }

    _NODISCARD static constexpr _Unsigned128 denorm_min() noexcept {
        return 0;
    }

    _NODISCARD static constexpr _Unsigned128 infinity() noexcept {
        return 0;
    }

    _NODISCARD static constexpr _Unsigned128 quiet_NaN() noexcept {
        return 0;
    }

    _NODISCARD static constexpr _Unsigned128 signaling_NaN() noexcept {
        return 0;
    }

    static constexpr bool is_modulo = true;
    static constexpr int digits     = 128;
    static constexpr int digits10   = 38;
};

struct _Signed128 : _Base128 {
    using _Signed_type   = _Signed128;
    using _Unsigned_type = _Unsigned128;

#if !_HAS_CXX17 || (_HAS_CXX20 && !defined(__clang__) && !defined(__EDG__)) // TRANSITION, DevCom-10729775
    constexpr _Signed128() noexcept : _Base128{} {}
#endif // ^^^ workaround for C++20 MSVC modules and header units; should be guarded for !_HAS_CXX17 only ^^^

    using _Base128::_Base128;
    constexpr explicit _Signed128(const _Base128& _That) noexcept : _Base128{_That} {}

    constexpr _Signed128& operator=(const _Base128& _That) noexcept {
        _Base128::operator=(_That);
        return *this;
    }

#if _HAS_CXX20
    _NODISCARD friend constexpr strong_ordering operator<=>(
        const _Signed128& _Left, const _Signed128& _Right) noexcept {
        strong_ordering _Ord = static_cast<int64_t>(_Left._Word[1]) <=> static_cast<int64_t>(_Right._Word[1]);
        if (_Ord == strong_ordering::equal) {
            _Ord = _Left._Word[0] <=> _Right._Word[0];
        }
        return _Ord;
    }
#else // ^^^ _HAS_CXX20 / !_HAS_CXX20 vvv
    _NODISCARD friend constexpr bool operator<(const _Signed128& _Left, const _Signed128& _Right) noexcept {
        if (static_cast<int64_t>(_Left._Word[1]) < static_cast<int64_t>(_Right._Word[1])) {
            return true;
        }

        if (static_cast<int64_t>(_Right._Word[1]) < static_cast<int64_t>(_Left._Word[1])) {
            return false;
        }

        return _Left._Word[0] < _Right._Word[0];
    }

    _NODISCARD friend constexpr bool operator>(const _Signed128& _Left, const _Signed128& _Right) noexcept {
        return _Right < _Left;
    }

    _NODISCARD friend constexpr bool operator<=(const _Signed128& _Left, const _Signed128& _Right) noexcept {
        return !(_Right < _Left);
    }

    _NODISCARD friend constexpr bool operator>=(const _Signed128& _Left, const _Signed128& _Right) noexcept {
        return !(_Left < _Right);
    }
#endif // ^^^ !_HAS_CXX20 ^^^

    _NODISCARD friend constexpr _Signed128 operator<<(const _Signed128& _Left, const _Base128& _Right) noexcept {
        auto _Tmp{_Left};
        _Tmp._Left_shift(static_cast<unsigned char>(_Right._Word[0]));
        return _Tmp;
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    constexpr _Signed128& operator<<=(const _Ty _Count) noexcept {
        _Left_shift(static_cast<unsigned char>(_Count));
        return *this;
    }
    constexpr _Signed128& operator<<=(const _Base128& _Count) noexcept {
        _Left_shift(static_cast<unsigned char>(_Count._Word[0]));
        return *this;
    }

    constexpr void _Signed_right_shift(const unsigned char _Count) noexcept {
        if (_Count == 0) {
            return;
        }

        if (_Count >= 64) {
            _Word[0] = static_cast<uint64_t>(static_cast<int64_t>(_Word[1]) >> (_Count % 64));
            _Word[1] = (_Word[1] & (1ull << 63)) == 0 ? 0 : ~0ull;
            return;
        }

#if _STL_128_INTRINSICS
        if (!_Is_constant_evaluated()) {
            _Word[0] = __shiftright128(_Word[0], _Word[1], _Count);
        } else
#endif // _STL_128_INTRINSICS
        {
            _Word[0] = (_Word[0] >> _Count) | (_Word[1] << (64 - _Count));
        }

        _Word[1] = static_cast<uint64_t>(static_cast<int64_t>(_Word[1]) >> _Count);
    }

    _NODISCARD friend constexpr _Signed128 operator>>(const _Signed128& _Left, const _Base128& _Right) noexcept {
        auto _Tmp{_Left};
        _Tmp._Signed_right_shift(static_cast<unsigned char>(_Right._Word[0]));
        return _Tmp;
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    constexpr _Signed128& operator>>=(const _Ty _Count) noexcept {
        _Signed_right_shift(static_cast<unsigned char>(_Count));
        return *this;
    }
    constexpr _Signed128& operator>>=(const _Base128& _Count) noexcept {
        _Signed_right_shift(static_cast<unsigned char>(_Count._Word[0]));
        return *this;
    }

    constexpr _Signed128& operator++() noexcept {
        if (++_Word[0] == 0) {
            ++_Word[1];
        }
        return *this;
    }
    constexpr _Signed128 operator++(int) noexcept {
        auto _Tmp = *this;
        ++*this;
        return _Tmp;
    }

    constexpr _Signed128& operator--() noexcept {
        if (_Word[0]-- == 0) {
            --_Word[1];
        }
        return *this;
    }
    constexpr _Signed128 operator--(int) noexcept {
        auto _Tmp = *this;
        --*this;
        return _Tmp;
    }

    _NODISCARD constexpr _Signed128 operator+() const noexcept {
        return *this;
    }

    _NODISCARD constexpr _Signed128 operator-() const noexcept {
        return _Signed128{} - *this;
    }

    _NODISCARD constexpr _Signed128 operator~() const noexcept {
        return _Signed128{~_Word[0], ~_Word[1]};
    }

    _NODISCARD friend constexpr _Signed128 operator+(const _Signed128& _Left, const _Signed128& _Right) noexcept {
        _Signed128 _Result;
        const auto _Carry = _AddCarry64(0, _Left._Word[0], _Right._Word[0], _Result._Word[0]);
        _AddCarry64(_Carry, _Left._Word[1], _Right._Word[1], _Result._Word[1]);
        return _Result;
    }

    constexpr _Signed128& operator+=(const _Base128& _That) noexcept {
        const auto _Carry = _AddCarry64(0, _Word[0], _That._Word[0], _Word[0]);
        _AddCarry64(_Carry, _Word[1], _That._Word[1], _Word[1]);
        return *this;
    }
    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    friend constexpr _Ty& operator+=(_Ty& _Left, const _Signed128& _Right) noexcept {
        _Left = static_cast<_Ty>(_Signed128{_Left} + _Right);
        return _Left;
    }

    _NODISCARD friend constexpr _Signed128 operator-(const _Signed128& _Left, const _Signed128& _Right) noexcept {
        _Signed128 _Result;
        const auto _Borrow = _SubBorrow64(0, _Left._Word[0], _Right._Word[0], _Result._Word[0]);
        _SubBorrow64(_Borrow, _Left._Word[1], _Right._Word[1], _Result._Word[1]);
        return _Result;
    }

    constexpr _Signed128& operator-=(const _Base128& _That) noexcept {
        const auto _Borrow = _SubBorrow64(0, _Word[0], _That._Word[0], _Word[0]);
        _SubBorrow64(_Borrow, _Word[1], _That._Word[1], _Word[1]);
        return *this;
    }
    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    friend constexpr _Ty& operator-=(_Ty& _Left, const _Signed128& _Right) noexcept {
        _Left = static_cast<_Ty>(_Signed128{_Left} - _Right);
        return _Left;
    }

    constexpr void _Strip_negative(bool& _Flip) noexcept {
        if ((_Word[1] & (1ull << 63)) != 0) {
            *this = -*this;
            _Flip = !_Flip;
        }
    }

    _NODISCARD friend constexpr _Signed128 operator*(_Signed128 _Left, _Signed128 _Right) noexcept {
        bool _Negative = false;
        _Left._Strip_negative(_Negative);
        _Right._Strip_negative(_Negative);
        _Signed128 _Result{_Base128::_Multiply(_Left, _Right)};
        if (_Negative) {
            _Result = -_Result;
        }
        return _Result;
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    constexpr _Signed128& operator*=(const _Ty _That) noexcept {
        *this = *this * _That;
        return *this;
    }
    constexpr _Signed128& operator*=(const _Signed128& _That) noexcept {
        *this = *this * _That;
        return *this;
    }
    constexpr _Signed128& operator*=(const _Unsigned128& _That) noexcept {
        *this = _Signed128{static_cast<const _Base128&>(*this) * _That};
        return *this;
    }
    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    friend constexpr _Ty& operator*=(_Ty& _Left, const _Signed128& _Right) noexcept {
        _Left = static_cast<_Ty>(_Signed128{_Left} * _Right);
        return _Left;
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    _NODISCARD friend constexpr _Signed128 operator/(_Signed128 _Num, _Ty _Den) noexcept {
        bool _Negative = false;
        _Num._Strip_negative(_Negative);
        if constexpr (is_signed_v<_Ty>) {
            if (_Den < 0) {
                _Den      = -_Den;
                _Negative = !_Negative;
            }
        }

        _Signed128 _Result;
#if !_STL_128_DIV_INTRINSICS
        if constexpr (sizeof(_Ty) <= 4) {
            _Result = _Signed128{_Base128::_Divide(_Num, static_cast<uint32_t>(_Den))};
        } else
#endif // !_STL_128_DIV_INTRINSICS
        {
            _Result = _Signed128{_Base128::_Divide(_Num, static_cast<uint64_t>(_Den))};
        }

        if (_Negative) {
            _Result = -_Result;
        }
        return _Result;
    }
    _NODISCARD friend constexpr _Signed128 operator/(_Signed128 _Num, _Signed128 _Den) noexcept {
        bool _Negative = false;
        _Num._Strip_negative(_Negative);
        _Den._Strip_negative(_Negative);
        _Signed128 _Result{_Base128::_Divide(_Num, _Den)};
        if (_Negative) {
            _Result = -_Result;
        }
        return _Result;
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    constexpr _Signed128& operator/=(const _Ty _That) noexcept {
        *this = *this / _That;
        return *this;
    }
    constexpr _Signed128& operator/=(const _Signed128& _That) noexcept {
        *this = *this / _That;
        return *this;
    }
    constexpr _Signed128& operator/=(const _Unsigned128& _That) noexcept {
        *this = _Signed128{static_cast<_Base128&>(*this) / _That};
        return *this;
    }
    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    friend constexpr _Ty& operator/=(_Ty& _Left, const _Signed128& _Right) noexcept {
        _Left = static_cast<_Ty>(_Signed128{_Left} / _Right);
        return _Left;
    }

    _NODISCARD friend constexpr _Signed128 operator%(_Signed128 _Left, _Signed128 _Right) noexcept {
        bool _Negative = false;
        _Left._Strip_negative(_Negative);

        if ((_Right._Word[1] & (1ull << 63)) != 0) {
            _Right = -_Right;
            // intentionally not flipping _Negative
        }

        _Unsigned128 _Result{_Base128::_Modulo(_Left, _Right)};
        if (_Negative) {
            _Result = -_Result;
        }
        return _Signed128{_Result};
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    _NODISCARD friend constexpr _Signed128 operator%(_Signed128 _Left, const _Ty _Right) noexcept {
        return _Left % _Signed128{_Right};
    }

    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    constexpr _Signed128& operator%=(const _Ty _That) noexcept {
        *this = *this % _That;
        return *this;
    }
    constexpr _Signed128& operator%=(const _Signed128& _That) noexcept {
        *this = *this % _That;
        return *this;
    }
    constexpr _Signed128& operator%=(const _Unsigned128& _That) noexcept {
        *this = static_cast<const _Base128&>(*this) % _That;
        return *this;
    }
    _TEMPLATE_CLASS_INTEGRAL(_Ty)
    friend constexpr _Ty& operator%=(_Ty& _Left, const _Signed128& _Right) noexcept {
        _Left = static_cast<_Ty>(_Signed128{_Left} % _Right);
        return _Left;
    }

    _NODISCARD friend constexpr _Signed128 operator&(const _Signed128& _Left, const _Signed128& _Right) noexcept {
        return _Signed128{_Left._Word[0] & _Right._Word[0], _Left._Word[1] & _Right._Word[1]};
    }

    constexpr _Signed128& operator&=(const _Base128& _That) noexcept {
        _Word[0] &= _That._Word[0];
        _Word[1] &= _That._Word[1];
        return *this;
    }

    _NODISCARD friend constexpr _Signed128 operator^(const _Signed128& _Left, const _Signed128& _Right) noexcept {
        return _Signed128{_Left._Word[0] ^ _Right._Word[0], _Left._Word[1] ^ _Right._Word[1]};
    }

    constexpr _Signed128& operator^=(const _Base128& _That) noexcept {
        _Word[0] ^= _That._Word[0];
        _Word[1] ^= _That._Word[1];
        return *this;
    }

    _NODISCARD friend constexpr _Signed128 operator|(const _Signed128& _Left, const _Signed128& _Right) noexcept {
        return _Signed128{_Left._Word[0] | _Right._Word[0], _Left._Word[1] | _Right._Word[1]};
    }

    constexpr _Signed128& operator|=(const _Base128& _That) noexcept {
        _Word[0] |= _That._Word[0];
        _Word[1] |= _That._Word[1];
        return *this;
    }
};

template <>
class numeric_limits<_Signed128> : public _Num_int_base {
public:
    _NODISCARD static constexpr _Signed128(min)() noexcept {
        return _Signed128{0ull, 1ull << 63};
    }

    _NODISCARD static constexpr _Signed128(max)() noexcept {
        return _Signed128{~0ull, ~0ull >> 1};
    }

    _NODISCARD static constexpr _Signed128 lowest() noexcept {
        return (min) ();
    }

    _NODISCARD static constexpr _Signed128 epsilon() noexcept {
        return 0;
    }

    _NODISCARD static constexpr _Signed128 round_error() noexcept {
        return 0;
    }

    _NODISCARD static constexpr _Signed128 denorm_min() noexcept {
        return 0;
    }

    _NODISCARD static constexpr _Signed128 infinity() noexcept {
        return 0;
    }

    _NODISCARD static constexpr _Signed128 quiet_NaN() noexcept {
        return 0;
    }

    _NODISCARD static constexpr _Signed128 signaling_NaN() noexcept {
        return 0;
    }

    static constexpr int digits     = 127;
    static constexpr int digits10   = 38;
    static constexpr bool is_signed = true;
};

template <>
struct common_type<_Signed128, _Unsigned128> {
    using type = _Unsigned128;
};
template <>
struct common_type<_Unsigned128, _Signed128> {
    using type = _Unsigned128;
};

template <class _Ty>
_NODISCARD constexpr _Ty _Min_limit() noexcept;

template <>
_NODISCARD constexpr _Unsigned128 _Min_limit<_Unsigned128>() noexcept {
    return 0;
}

template <>
_NODISCARD constexpr _Signed128 _Min_limit<_Signed128>() noexcept {
    return _Signed128{0ull, 1ull << 63};
}

template <class _Ty>
_NODISCARD constexpr _Ty _Max_limit() noexcept;

template <>
_NODISCARD constexpr _Unsigned128 _Max_limit<_Unsigned128>() noexcept {
    return _Unsigned128{~0ull, ~0ull};
}

template <>
_NODISCARD constexpr _Signed128 _Max_limit<_Signed128>() noexcept {
    return _Signed128{~0ull, ~0ull >> 1};
}

#undef _STL_128_INTRINSICS
#undef _STL_128_DIV_INTRINSICS

_STD_END

#undef _TEMPLATE_CLASS_INTEGRAL
#undef _ZERO_OR_NO_INIT

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // _STL_COMPILER_PREPROCESSOR
#endif // __MSVC_INT128_HPP
