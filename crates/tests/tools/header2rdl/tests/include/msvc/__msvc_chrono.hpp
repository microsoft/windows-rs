// __msvc_chrono.hpp internal header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef __MSVC_CHRONO_HPP
#define __MSVC_CHRONO_HPP
#include <yvals.h>
#if _STL_COMPILER_PREPROCESSOR
#include <ctime>
#include <limits>
#include <ratio>
#include <type_traits>
#include <utility>
#include <xtimec.h>

#if _HAS_CXX20
#include <compare>
#endif

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

_STD_BEGIN
namespace chrono {
    _EXPORT_STD template <class _Rep>
    struct treat_as_floating_point : is_floating_point<_Rep> {}; // tests for floating-point type

    _EXPORT_STD template <class _Rep>
    constexpr bool treat_as_floating_point_v = treat_as_floating_point<_Rep>::value;

    _EXPORT_STD template <class _Rep>
    struct duration_values { // gets arithmetic properties of a type
        _NODISCARD static constexpr _Rep zero() noexcept {
            // get zero value
            return _Rep(0);
        }

        _NODISCARD static constexpr _Rep(min)() noexcept {
            // get smallest value
            return numeric_limits<_Rep>::lowest();
        }

        _NODISCARD static constexpr _Rep(max)() noexcept {
            // get largest value
            return (numeric_limits<_Rep>::max)();
        }
    };

#if _HAS_CXX20
    _EXPORT_STD template <class _Clock>
    constexpr bool is_clock_v = requires {
        typename _Clock::rep;
        typename _Clock::period;
        typename _Clock::duration;
        typename _Clock::time_point;
        _Clock::is_steady;
        _Clock::now();
    };
    _EXPORT_STD template <class _Clock>
    struct is_clock : bool_constant<is_clock_v<_Clock>> {};

    template <class _Clock>
    constexpr bool _Is_clock_v = is_clock_v<_Clock>;
#else // ^^^ _HAS_CXX20 / !_HAS_CXX20 vvv
    template <class _Clock, class = void>
    constexpr bool _Is_clock_v = false;

    template <class _Clock>
    constexpr bool
        _Is_clock_v<_Clock, void_t<typename _Clock::rep, typename _Clock::period, typename _Clock::duration,
                                typename _Clock::time_point, decltype(_Clock::is_steady), decltype(_Clock::now())>> =
            true;
#endif // ^^^ !_HAS_CXX20 ^^^

    _EXPORT_STD template <class _Rep, class _Period = ratio<1>>
    class duration;

    template <class _Ty>
    constexpr bool _Is_duration_v = _Is_specialization_v<_Ty, duration>;

    _EXPORT_STD template <class _To, class _Rep, class _Period, enable_if_t<_Is_duration_v<_To>, int> = 0>
    constexpr _To duration_cast(const duration<_Rep, _Period>&)
        noexcept(is_arithmetic_v<_Rep> && is_arithmetic_v<typename _To::rep>); // strengthened

    _EXPORT_STD template <class _Rep, class _Period>
    class duration { // represents a time duration
    public:
        using rep    = _Rep;
        using period = typename _Period::type;

        static_assert(!_Is_duration_v<_Rep>, "duration can't have duration as first template argument");
        static_assert(_Is_ratio_v<_Period>, "period not an instance of std::ratio");
        static_assert(0 < _Period::num, "period negative or zero");

        constexpr duration() = default;

        template <class _Rep2,
            enable_if_t<is_convertible_v<const _Rep2&, _Rep>
                            && (treat_as_floating_point_v<_Rep> || !treat_as_floating_point_v<_Rep2>),
                int> = 0>
        constexpr explicit duration(const _Rep2& _Val)
            noexcept(is_arithmetic_v<_Rep> && is_arithmetic_v<_Rep2>) // strengthened
            : _MyRep(static_cast<_Rep>(_Val)) {}

        template <class _Rep2, class _Period2,
            enable_if_t<treat_as_floating_point_v<_Rep>
                            || (_Ratio_divide_sfinae<_Period2, _Period>::den == 1 && !treat_as_floating_point_v<_Rep2>),
                int> = 0>
        constexpr duration(const duration<_Rep2, _Period2>& _Dur)
            noexcept(is_arithmetic_v<_Rep> && is_arithmetic_v<_Rep2>) // strengthened
            : _MyRep(_CHRONO duration_cast<duration>(_Dur).count()) {}

        _NODISCARD constexpr _Rep count() const noexcept(is_arithmetic_v<_Rep>) /* strengthened */ {
            return _MyRep;
        }

        _NODISCARD constexpr common_type_t<duration> operator+() const
            noexcept(is_arithmetic_v<_Rep>) /* strengthened */ {
            return common_type_t<duration>(*this);
        }

        _NODISCARD constexpr common_type_t<duration> operator-() const
            noexcept(is_arithmetic_v<_Rep>) /* strengthened */ {
            return common_type_t<duration>(-_MyRep);
        }

        _CONSTEXPR17 duration& operator++() noexcept(is_arithmetic_v<_Rep>) /* strengthened */ {
            ++_MyRep;
            return *this;
        }

        _CONSTEXPR17 duration operator++(int) noexcept(is_arithmetic_v<_Rep>) /* strengthened */ {
            return duration(_MyRep++);
        }

        _CONSTEXPR17 duration& operator--() noexcept(is_arithmetic_v<_Rep>) /* strengthened */ {
            --_MyRep;
            return *this;
        }

        _CONSTEXPR17 duration operator--(int) noexcept(is_arithmetic_v<_Rep>) /* strengthened */ {
            return duration(_MyRep--);
        }

        _CONSTEXPR17 duration& operator+=(const duration& _Right) noexcept(is_arithmetic_v<_Rep>) /* strengthened */ {
            _MyRep += _Right._MyRep;
            return *this;
        }

        _CONSTEXPR17 duration& operator-=(const duration& _Right) noexcept(is_arithmetic_v<_Rep>) /* strengthened */ {
            _MyRep -= _Right._MyRep;
            return *this;
        }

        _CONSTEXPR17 duration& operator*=(const _Rep& _Right) noexcept(is_arithmetic_v<_Rep>) /* strengthened */ {
            _MyRep *= _Right;
            return *this;
        }

        _CONSTEXPR17 duration& operator/=(const _Rep& _Right) noexcept(is_arithmetic_v<_Rep>) /* strengthened */ {
            _MyRep /= _Right;
            return *this;
        }

        _CONSTEXPR17 duration& operator%=(const _Rep& _Right) noexcept(is_arithmetic_v<_Rep>) /* strengthened */ {
            _MyRep %= _Right;
            return *this;
        }

        _CONSTEXPR17 duration& operator%=(const duration& _Right) noexcept(is_arithmetic_v<_Rep>) /* strengthened */ {
            _MyRep %= _Right.count();
            return *this;
        }

        _NODISCARD static constexpr duration zero() noexcept {
            // get zero value
            return duration(duration_values<_Rep>::zero());
        }

        _NODISCARD static constexpr duration(min)() noexcept {
            // get minimum value
            return duration((duration_values<_Rep>::min)());
        }

        _NODISCARD static constexpr duration(max)() noexcept {
            // get maximum value
            return duration((duration_values<_Rep>::max)());
        }

    private:
        _Rep _MyRep; // the stored rep
    };

    _EXPORT_STD template <class _Clock, class _Duration = typename _Clock::duration>
    class time_point { // represents a point in time
    public:
        using clock    = _Clock;
        using duration = _Duration;
        using rep      = typename _Duration::rep;
        using period   = typename _Duration::period;

        static_assert(_Is_duration_v<_Duration>,
            "N4950 [time.point.general]/1 mandates Duration to be a specialization of chrono::duration.");

        constexpr time_point() = default;

        constexpr explicit time_point(const _Duration& _Other) noexcept(is_arithmetic_v<rep>) // strengthened
            : _MyDur(_Other) {}

        template <class _Duration2, enable_if_t<is_convertible_v<_Duration2, _Duration>, int> = 0>
        constexpr time_point(const time_point<_Clock, _Duration2>& _Tp)
            noexcept(is_arithmetic_v<rep> && is_arithmetic_v<typename _Duration2::rep>) // strengthened
            : _MyDur(_Tp.time_since_epoch()) {}

        _NODISCARD constexpr _Duration time_since_epoch() const noexcept(is_arithmetic_v<rep>) /* strengthened */ {
            return _MyDur;
        }

#if _HAS_CXX20
        constexpr time_point& operator++() noexcept(is_arithmetic_v<rep>) /* strengthened */ {
            ++_MyDur;
            return *this;
        }
        constexpr time_point operator++(int) noexcept(is_arithmetic_v<rep>) /* strengthened */ {
            return time_point{_MyDur++};
        }
        constexpr time_point& operator--() noexcept(is_arithmetic_v<rep>) /* strengthened */ {
            --_MyDur;
            return *this;
        }
        constexpr time_point operator--(int) noexcept(is_arithmetic_v<rep>) /* strengthened */ {
            return time_point{_MyDur--};
        }
#endif // _HAS_CXX20

        _CONSTEXPR17 time_point& operator+=(const _Duration& _Dur) noexcept(is_arithmetic_v<rep>) /* strengthened */ {
            _MyDur += _Dur;
            return *this;
        }

        _CONSTEXPR17 time_point& operator-=(const _Duration& _Dur) noexcept(is_arithmetic_v<rep>) /* strengthened */ {
            _MyDur -= _Dur;
            return *this;
        }

        _NODISCARD static constexpr time_point(min)() noexcept {
            return time_point((_Duration::min)());
        }

        _NODISCARD static constexpr time_point(max)() noexcept {
            return time_point((_Duration::max)());
        }

    private:
        _Duration _MyDur{duration::zero()}; // duration since the epoch
    };
} // namespace chrono

template <class _Rep, class _Period>
constexpr bool _Is_trivially_swappable_v<chrono::duration<_Rep, _Period>> = _Is_trivially_swappable_v<_Rep>;

template <class _Clock, class _Duration>
constexpr bool _Is_trivially_swappable_v<chrono::time_point<_Clock, _Duration>> = _Is_trivially_swappable_v<_Duration>;

_NODISCARD constexpr intmax_t _Lcm(const intmax_t _Ax, const intmax_t _Bx) noexcept {
    return (_Ax / _Gcd(_Ax, _Bx)) * _Bx;
}

template <class _Rep1, class _Period1, class _Rep2, class _Period2>
struct common_type<_CHRONO duration<_Rep1, _Period1>, _CHRONO duration<_Rep2, _Period2>> {
    using type = _CHRONO duration<common_type_t<_Rep1, _Rep2>,
        ratio<_Gcd(_Period1::num, _Period2::num), _Lcm(_Period1::den, _Period2::den)>>;
};

template <class _Clock, class _Duration1, class _Duration2>
struct common_type<_CHRONO time_point<_Clock, _Duration1>,
    _CHRONO time_point<_Clock, _Duration2>> { // common type of two time points
    using type = _CHRONO time_point<_Clock, common_type_t<_Duration1, _Duration2>>;
};

namespace chrono {
    _EXPORT_STD template <class _Rep1, class _Period1, class _Rep2, class _Period2>
    _NODISCARD constexpr common_type_t<duration<_Rep1, _Period1>, duration<_Rep2, _Period2>> operator+(
        const duration<_Rep1, _Period1>& _Left, const duration<_Rep2, _Period2>& _Right)
        noexcept(is_arithmetic_v<_Rep1> && is_arithmetic_v<_Rep2>) /* strengthened */ {
        using _CD = common_type_t<duration<_Rep1, _Period1>, duration<_Rep2, _Period2>>;
        return _CD(_CD(_Left).count() + _CD(_Right).count());
    }

    _EXPORT_STD template <class _Rep1, class _Period1, class _Rep2, class _Period2>
    _NODISCARD constexpr common_type_t<duration<_Rep1, _Period1>, duration<_Rep2, _Period2>> operator-(
        const duration<_Rep1, _Period1>& _Left, const duration<_Rep2, _Period2>& _Right)
        noexcept(is_arithmetic_v<_Rep1> && is_arithmetic_v<_Rep2>) /* strengthened */ {
        using _CD = common_type_t<duration<_Rep1, _Period1>, duration<_Rep2, _Period2>>;
        return _CD(_CD(_Left).count() - _CD(_Right).count());
    }

    _EXPORT_STD template <class _Rep1, class _Period1, class _Rep2,
        enable_if_t<is_convertible_v<const _Rep2&, common_type_t<_Rep1, _Rep2>>, int> = 0>
    _NODISCARD constexpr duration<common_type_t<_Rep1, _Rep2>, _Period1> operator*(
        const duration<_Rep1, _Period1>& _Left, const _Rep2& _Right)
        noexcept(is_arithmetic_v<_Rep1> && is_arithmetic_v<_Rep2>) /* strengthened */ {
        using _CR = common_type_t<_Rep1, _Rep2>;
        using _CD = duration<_CR, _Period1>;
        return _CD(_CD(_Left).count() * _Right);
    }

    _EXPORT_STD template <class _Rep1, class _Rep2, class _Period2,
        enable_if_t<is_convertible_v<const _Rep1&, common_type_t<_Rep1, _Rep2>>, int> = 0>
    _NODISCARD constexpr duration<common_type_t<_Rep1, _Rep2>, _Period2> operator*(
        const _Rep1& _Left, const duration<_Rep2, _Period2>& _Right)
        noexcept(is_arithmetic_v<_Rep1> && is_arithmetic_v<_Rep2>) /* strengthened */ {
        return _Right * _Left;
    }

    template <class _CR, class _Period1, class _Rep2, bool = is_convertible_v<const _Rep2&, _CR>>
    struct _Duration_div_mod1 { // return type for duration / rep and duration % rep
        using type = duration<_CR, _Period1>;
    };

    template <class _CR, class _Period1, class _Rep2>
    struct _Duration_div_mod1<_CR, _Period1, _Rep2, false> {}; // no return type

    template <class _CR, class _Period1, class _Rep2, bool = _Is_duration_v<_Rep2>>
    struct _Duration_div_mod {}; // no return type

    template <class _CR, class _Period1, class _Rep2>
    struct _Duration_div_mod<_CR, _Period1, _Rep2, false> : _Duration_div_mod1<_CR, _Period1, _Rep2> {
        // return type for duration / rep and duration % rep
    };

    _EXPORT_STD template <class _Rep1, class _Period1, class _Rep2>
    _NODISCARD constexpr typename _Duration_div_mod<common_type_t<_Rep1, _Rep2>, _Period1, _Rep2>::type operator/(
        const duration<_Rep1, _Period1>& _Left, const _Rep2& _Right)
        noexcept(is_arithmetic_v<_Rep1> && is_arithmetic_v<_Rep2>) /* strengthened */ {
        using _CR = common_type_t<_Rep1, _Rep2>;
        using _CD = duration<_CR, _Period1>;
        return _CD(_CD(_Left).count() / _Right);
    }

    _EXPORT_STD template <class _Rep1, class _Period1, class _Rep2, class _Period2>
    _NODISCARD constexpr common_type_t<_Rep1, _Rep2> operator/(
        const duration<_Rep1, _Period1>& _Left, const duration<_Rep2, _Period2>& _Right)
        noexcept(is_arithmetic_v<_Rep1> && is_arithmetic_v<_Rep2>) /* strengthened */ {
        using _CD = common_type_t<duration<_Rep1, _Period1>, duration<_Rep2, _Period2>>;
        return _CD(_Left).count() / _CD(_Right).count();
    }

    _EXPORT_STD template <class _Rep1, class _Period1, class _Rep2>
    _NODISCARD constexpr typename _Duration_div_mod<common_type_t<_Rep1, _Rep2>, _Period1, _Rep2>::type operator%(
        const duration<_Rep1, _Period1>& _Left, const _Rep2& _Right)
        noexcept(is_arithmetic_v<_Rep1> && is_arithmetic_v<_Rep2>) /* strengthened */ {
        using _CR = common_type_t<_Rep1, _Rep2>;
        using _CD = duration<_CR, _Period1>;
        return _CD(_CD(_Left).count() % _Right);
    }

    _EXPORT_STD template <class _Rep1, class _Period1, class _Rep2, class _Period2>
    _NODISCARD constexpr common_type_t<duration<_Rep1, _Period1>, duration<_Rep2, _Period2>> operator%(
        const duration<_Rep1, _Period1>& _Left, const duration<_Rep2, _Period2>& _Right)
        noexcept(is_arithmetic_v<_Rep1> && is_arithmetic_v<_Rep2>) /* strengthened */ {
        using _CD = common_type_t<duration<_Rep1, _Period1>, duration<_Rep2, _Period2>>;
        return _CD(_CD(_Left).count() % _CD(_Right).count());
    }

    _EXPORT_STD template <class _Rep1, class _Period1, class _Rep2, class _Period2>
    _NODISCARD constexpr bool operator==(
        const duration<_Rep1, _Period1>& _Left, const duration<_Rep2, _Period2>& _Right)
        noexcept(is_arithmetic_v<_Rep1> && is_arithmetic_v<_Rep2>) /* strengthened */ {
        using _CT = common_type_t<duration<_Rep1, _Period1>, duration<_Rep2, _Period2>>;
        return _CT(_Left).count() == _CT(_Right).count();
    }

#if !_HAS_CXX20
    template <class _Rep1, class _Period1, class _Rep2, class _Period2>
    _NODISCARD constexpr bool operator!=(
        const duration<_Rep1, _Period1>& _Left, const duration<_Rep2, _Period2>& _Right)
        noexcept(is_arithmetic_v<_Rep1> && is_arithmetic_v<_Rep2>) /* strengthened */ {
        return !(_Left == _Right);
    }
#endif // !_HAS_CXX20

    _EXPORT_STD template <class _Rep1, class _Period1, class _Rep2, class _Period2>
    _NODISCARD constexpr bool operator<(const duration<_Rep1, _Period1>& _Left, const duration<_Rep2, _Period2>& _Right)
        noexcept(is_arithmetic_v<_Rep1> && is_arithmetic_v<_Rep2>) /* strengthened */ {
        using _CT = common_type_t<duration<_Rep1, _Period1>, duration<_Rep2, _Period2>>;
        return _CT(_Left).count() < _CT(_Right).count();
    }

    _EXPORT_STD template <class _Rep1, class _Period1, class _Rep2, class _Period2>
    _NODISCARD constexpr bool operator<=(
        const duration<_Rep1, _Period1>& _Left, const duration<_Rep2, _Period2>& _Right)
        noexcept(is_arithmetic_v<_Rep1> && is_arithmetic_v<_Rep2>) /* strengthened */ {
        return !(_Right < _Left);
    }

    _EXPORT_STD template <class _Rep1, class _Period1, class _Rep2, class _Period2>
    _NODISCARD constexpr bool operator>(const duration<_Rep1, _Period1>& _Left, const duration<_Rep2, _Period2>& _Right)
        noexcept(is_arithmetic_v<_Rep1> && is_arithmetic_v<_Rep2>) /* strengthened */ {
        return _Right < _Left;
    }

    _EXPORT_STD template <class _Rep1, class _Period1, class _Rep2, class _Period2>
    _NODISCARD constexpr bool operator>=(
        const duration<_Rep1, _Period1>& _Left, const duration<_Rep2, _Period2>& _Right)
        noexcept(is_arithmetic_v<_Rep1> && is_arithmetic_v<_Rep2>) /* strengthened */ {
        return !(_Left < _Right);
    }

#if _HAS_CXX20
    _EXPORT_STD template <class _Rep1, class _Period1, class _Rep2, class _Period2>
        requires three_way_comparable<typename common_type_t<duration<_Rep1, _Period1>, duration<_Rep2, _Period2>>::rep>
    _NODISCARD constexpr auto operator<=>(
        const duration<_Rep1, _Period1>& _Left, const duration<_Rep2, _Period2>& _Right)
        noexcept(is_arithmetic_v<_Rep1> && is_arithmetic_v<_Rep2>) /* strengthened */ {
        using _CT = common_type_t<duration<_Rep1, _Period1>, duration<_Rep2, _Period2>>;
        return _CT(_Left).count() <=> _CT(_Right).count();
    }
#endif // _HAS_CXX20

    _EXPORT_STD template <class _To, class _Rep, class _Period, enable_if_t<_Is_duration_v<_To>, int> /* = 0 */>
    _NODISCARD constexpr _To duration_cast(const duration<_Rep, _Period>& _Dur)
        noexcept(is_arithmetic_v<_Rep> && is_arithmetic_v<typename _To::rep>) /* strengthened */ {
        // convert duration to another duration; truncate
        using _CF = ratio_divide<_Period, typename _To::period>;

        using _ToRep = typename _To::rep;
        using _CR    = common_type_t<_ToRep, _Rep, intmax_t>;

        constexpr bool _Num_is_one = _CF::num == 1;
        constexpr bool _Den_is_one = _CF::den == 1;

        if constexpr (_Den_is_one) {
            if constexpr (_Num_is_one) {
                return static_cast<_To>(static_cast<_ToRep>(_Dur.count()));
            } else {
                return static_cast<_To>(
                    static_cast<_ToRep>(static_cast<_CR>(_Dur.count()) * static_cast<_CR>(_CF::num)));
            }
        } else {
            if constexpr (_Num_is_one) {
                return static_cast<_To>(
                    static_cast<_ToRep>(static_cast<_CR>(_Dur.count()) / static_cast<_CR>(_CF::den)));
            } else {
                return static_cast<_To>(static_cast<_ToRep>(
                    static_cast<_CR>(_Dur.count()) * static_cast<_CR>(_CF::num) / static_cast<_CR>(_CF::den)));
            }
        }
    }

    _EXPORT_STD template <class _To, class _Rep, class _Period, enable_if_t<_Is_duration_v<_To>, int> = 0>
    _NODISCARD constexpr _To floor(const duration<_Rep, _Period>& _Dur)
        noexcept(is_arithmetic_v<_Rep> && is_arithmetic_v<typename _To::rep>) /* strengthened */ {
        // convert duration to another duration; round towards negative infinity
        // i.e. the greatest integral result such that the result <= _Dur
        const _To _Casted{_CHRONO duration_cast<_To>(_Dur)};
        if (_Casted > _Dur) {
            return _To{_Casted.count() - static_cast<typename _To::rep>(1)};
        }

        return _Casted;
    }

    _EXPORT_STD template <class _To, class _Rep, class _Period, enable_if_t<_Is_duration_v<_To>, int> = 0>
    _NODISCARD constexpr _To ceil(const duration<_Rep, _Period>& _Dur)
        noexcept(is_arithmetic_v<_Rep> && is_arithmetic_v<typename _To::rep>) /* strengthened */ {
        // convert duration to another duration; round towards positive infinity
        // i.e. the least integral result such that _Dur <= the result
        const _To _Casted{_CHRONO duration_cast<_To>(_Dur)};
        if (_Casted < _Dur) {
            return _To{_Casted.count() + static_cast<typename _To::rep>(1)};
        }

        return _Casted;
    }

    template <class _Rep>
    constexpr bool _Is_even(_Rep _Val) noexcept(is_arithmetic_v<_Rep>) /* strengthened */ {
        // Tests whether _Val is even
        return _Val % 2 == 0;
    }

    _EXPORT_STD template <class _To, class _Rep, class _Period,
        enable_if_t<_Is_duration_v<_To> && !treat_as_floating_point_v<typename _To::rep>, int> = 0>
    _NODISCARD constexpr _To round(const duration<_Rep, _Period>& _Dur)
        noexcept(is_arithmetic_v<_Rep> && is_arithmetic_v<typename _To::rep>) /* strengthened */ {
        // convert duration to another duration, round to nearest, ties to even
        const _To _Floored{_CHRONO floor<_To>(_Dur)};
        const _To _Ceiled{_Floored + _To{1}};
        const auto _Floor_adjustment = _Dur - _Floored;
        const auto _Ceil_adjustment  = _Ceiled - _Dur;
        if (_Floor_adjustment < _Ceil_adjustment
            || (_Floor_adjustment == _Ceil_adjustment && _Is_even(_Floored.count()))) {
            return _Floored;
        }

        return _Ceiled;
    }

    _EXPORT_STD template <class _Rep, class _Period, enable_if_t<numeric_limits<_Rep>::is_signed, int> = 0>
    _NODISCARD constexpr duration<_Rep, _Period> abs(const duration<_Rep, _Period> _Dur)
        noexcept(is_arithmetic_v<_Rep>) /* strengthened */ {
        // create a duration whose count() is the absolute value of _Dur.count()
        if (_Dur < duration<_Rep, _Period>::zero()) {
            return -_Dur;
        } else {
            return _Dur;
        }
    }

    _EXPORT_STD using nanoseconds  = duration<long long, nano>;
    _EXPORT_STD using microseconds = duration<long long, micro>;
    _EXPORT_STD using milliseconds = duration<long long, milli>;
    _EXPORT_STD using seconds      = duration<long long>;
    _EXPORT_STD using minutes      = duration<int, ratio<60>>;
    _EXPORT_STD using hours        = duration<int, ratio<3600>>;
#if _HAS_CXX20
    _EXPORT_STD using days   = duration<int, ratio_multiply<ratio<24>, hours::period>>;
    _EXPORT_STD using weeks  = duration<int, ratio_multiply<ratio<7>, days::period>>;
    _EXPORT_STD using years  = duration<int, ratio_multiply<ratio<146097, 400>, days::period>>;
    _EXPORT_STD using months = duration<int, ratio_divide<years::period, ratio<12>>>;
#endif // _HAS_CXX20

    _EXPORT_STD template <class _Clock, class _Duration, class _Rep, class _Period>
    _NODISCARD constexpr time_point<_Clock, common_type_t<_Duration, duration<_Rep, _Period>>> operator+(
        const time_point<_Clock, _Duration>& _Left, const duration<_Rep, _Period>& _Right)
        noexcept(is_arithmetic_v<typename _Duration::rep> && is_arithmetic_v<_Rep>) /* strengthened */ {
        using _RT = time_point<_Clock, common_type_t<_Duration, duration<_Rep, _Period>>>;
        return _RT(_Left.time_since_epoch() + _Right);
    }

    _EXPORT_STD template <class _Rep, class _Period, class _Clock, class _Duration>
    _NODISCARD constexpr time_point<_Clock, common_type_t<duration<_Rep, _Period>, _Duration>> operator+(
        const duration<_Rep, _Period>& _Left, const time_point<_Clock, _Duration>& _Right)
        noexcept(is_arithmetic_v<_Rep> && is_arithmetic_v<typename _Duration::rep>) /* strengthened */ {
        return _Right + _Left;
    }

    _EXPORT_STD template <class _Clock, class _Duration, class _Rep, class _Period>
    _NODISCARD constexpr time_point<_Clock, common_type_t<_Duration, duration<_Rep, _Period>>> operator-(
        const time_point<_Clock, _Duration>& _Left, const duration<_Rep, _Period>& _Right)
        noexcept(is_arithmetic_v<typename _Duration::rep> && is_arithmetic_v<_Rep>) /* strengthened */ {
        using _RT = time_point<_Clock, common_type_t<_Duration, duration<_Rep, _Period>>>;
        return _RT(_Left.time_since_epoch() - _Right);
    }

    _EXPORT_STD template <class _Clock, class _Duration1, class _Duration2>
    _NODISCARD constexpr common_type_t<_Duration1, _Duration2> operator-(
        const time_point<_Clock, _Duration1>& _Left, const time_point<_Clock, _Duration2>& _Right)
        noexcept(
            is_arithmetic_v<typename _Duration1::rep> && is_arithmetic_v<typename _Duration2::rep>) /* strengthened */ {
        return _Left.time_since_epoch() - _Right.time_since_epoch();
    }

    _EXPORT_STD template <class _Clock, class _Duration1, class _Duration2>
    _NODISCARD constexpr bool operator==(
        const time_point<_Clock, _Duration1>& _Left, const time_point<_Clock, _Duration2>& _Right)
        noexcept(
            is_arithmetic_v<typename _Duration1::rep> && is_arithmetic_v<typename _Duration2::rep>) /* strengthened */ {
        return _Left.time_since_epoch() == _Right.time_since_epoch();
    }

#if !_HAS_CXX20
    template <class _Clock, class _Duration1, class _Duration2>
    _NODISCARD constexpr bool operator!=(
        const time_point<_Clock, _Duration1>& _Left, const time_point<_Clock, _Duration2>& _Right)
        noexcept(
            is_arithmetic_v<typename _Duration1::rep> && is_arithmetic_v<typename _Duration2::rep>) /* strengthened */ {
        return !(_Left == _Right);
    }
#endif // !_HAS_CXX20

    _EXPORT_STD template <class _Clock, class _Duration1, class _Duration2>
    _NODISCARD constexpr bool operator<(
        const time_point<_Clock, _Duration1>& _Left, const time_point<_Clock, _Duration2>& _Right)
        noexcept(
            is_arithmetic_v<typename _Duration1::rep> && is_arithmetic_v<typename _Duration2::rep>) /* strengthened */ {
        return _Left.time_since_epoch() < _Right.time_since_epoch();
    }

    _EXPORT_STD template <class _Clock, class _Duration1, class _Duration2>
    _NODISCARD constexpr bool operator<=(
        const time_point<_Clock, _Duration1>& _Left, const time_point<_Clock, _Duration2>& _Right)
        noexcept(
            is_arithmetic_v<typename _Duration1::rep> && is_arithmetic_v<typename _Duration2::rep>) /* strengthened */ {
        return !(_Right < _Left);
    }

    _EXPORT_STD template <class _Clock, class _Duration1, class _Duration2>
    _NODISCARD constexpr bool operator>(
        const time_point<_Clock, _Duration1>& _Left, const time_point<_Clock, _Duration2>& _Right)
        noexcept(
            is_arithmetic_v<typename _Duration1::rep> && is_arithmetic_v<typename _Duration2::rep>) /* strengthened */ {
        return _Right < _Left;
    }

    _EXPORT_STD template <class _Clock, class _Duration1, class _Duration2>
    _NODISCARD constexpr bool operator>=(
        const time_point<_Clock, _Duration1>& _Left, const time_point<_Clock, _Duration2>& _Right)
        noexcept(
            is_arithmetic_v<typename _Duration1::rep> && is_arithmetic_v<typename _Duration2::rep>) /* strengthened */ {
        return !(_Left < _Right);
    }

#if _HAS_CXX20
    _EXPORT_STD template <class _Clock, class _Duration1, three_way_comparable_with<_Duration1> _Duration2>
    _NODISCARD constexpr auto operator<=>(
        const time_point<_Clock, _Duration1>& _Left, const time_point<_Clock, _Duration2>& _Right)
        noexcept(
            is_arithmetic_v<typename _Duration1::rep> && is_arithmetic_v<typename _Duration2::rep>) /* strengthened */ {
        return _Left.time_since_epoch() <=> _Right.time_since_epoch();
    }
#endif // _HAS_CXX20

    _EXPORT_STD template <class _To, class _Clock, class _Duration, enable_if_t<_Is_duration_v<_To>, int> = 0>
    _NODISCARD constexpr time_point<_Clock, _To> time_point_cast(const time_point<_Clock, _Duration>& _Time)
        noexcept(is_arithmetic_v<typename _Duration::rep> && is_arithmetic_v<typename _To::rep>) /* strengthened */ {
        // change the duration type of a time_point; truncate
        return time_point<_Clock, _To>(_CHRONO duration_cast<_To>(_Time.time_since_epoch()));
    }

    _EXPORT_STD template <class _To, class _Clock, class _Duration, enable_if_t<_Is_duration_v<_To>, int> = 0>
    _NODISCARD constexpr time_point<_Clock, _To> floor(const time_point<_Clock, _Duration>& _Time)
        noexcept(is_arithmetic_v<typename _Duration::rep> && is_arithmetic_v<typename _To::rep>) /* strengthened */ {
        // change the duration type of a time_point; round towards negative infinity
        return time_point<_Clock, _To>(_CHRONO floor<_To>(_Time.time_since_epoch()));
    }

    _EXPORT_STD template <class _To, class _Clock, class _Duration, enable_if_t<_Is_duration_v<_To>, int> = 0>
    _NODISCARD constexpr time_point<_Clock, _To> ceil(const time_point<_Clock, _Duration>& _Time)
        noexcept(is_arithmetic_v<typename _Duration::rep> && is_arithmetic_v<typename _To::rep>) /* strengthened */ {
        // change the duration type of a time_point; round towards positive infinity
        return time_point<_Clock, _To>(_CHRONO ceil<_To>(_Time.time_since_epoch()));
    }

    _EXPORT_STD template <class _To, class _Clock, class _Duration,
        enable_if_t<_Is_duration_v<_To> && !treat_as_floating_point_v<typename _To::rep>, int> = 0>
    _NODISCARD constexpr time_point<_Clock, _To> round(const time_point<_Clock, _Duration>& _Time)
        noexcept(is_arithmetic_v<typename _Duration::rep> && is_arithmetic_v<typename _To::rep>) /* strengthened */ {
        // change the duration type of a time_point; round to nearest, ties to even
        return time_point<_Clock, _To>(_CHRONO round<_To>(_Time.time_since_epoch()));
    }

    _EXPORT_STD struct steady_clock { // wraps QueryPerformanceCounter
        using rep                       = long long;
        using period                    = nano;
        using duration                  = nanoseconds;
        using time_point                = _CHRONO time_point<steady_clock>;
        static constexpr bool is_steady = true;

        _NODISCARD static time_point now() noexcept { // get current time
            const long long _Freq = _Query_perf_frequency(); // doesn't change after system boot
            const long long _Ctr  = _Query_perf_counter();
            static_assert(period::num == 1, "This assumes period::num == 1.");
            // The compiler recognizes the constants for frequency and time period and uses shifts and
            // multiplies instead of divides to calculate the nanosecond value.
            constexpr long long _TenMHz        = 10'000'000;
            constexpr long long _TwentyFourMHz = 24'000'000;
            if (_Freq == _TenMHz) {
                // 10 MHz is a very common QPC frequency on modern x86/x64 PCs. Optimizing for
                // this specific frequency can double the performance of this function by
                // avoiding the expensive frequency conversion path.
                static_assert(period::den % _TenMHz == 0, "It should never fail.");
                constexpr long long _Multiplier = period::den / _TenMHz;
                return time_point(duration(_Ctr * _Multiplier));
            } else if (_Freq == _TwentyFourMHz) {
                // 24 MHz is a common frequency on ARM/ARM64, including cases where it emulates x86/x64.
                const long long _Whole = (_Ctr / _TwentyFourMHz) * period::den;
                const long long _Part  = (_Ctr % _TwentyFourMHz) * period::den / _TwentyFourMHz;
                return time_point(duration(_Whole + _Part));
            } else {
                // Instead of just having "(_Ctr * period::den) / _Freq",
                // the algorithm below prevents overflow when _Ctr is sufficiently large.
                // It assumes that _Freq * period::den does not overflow, which is currently true for nano period.
                // It is not realistic for _Ctr to accumulate to large values from zero with this assumption,
                // but the initial value of _Ctr could be large.
                const long long _Whole = (_Ctr / _Freq) * period::den;
                const long long _Part  = (_Ctr % _Freq) * period::den / _Freq;
                return time_point(duration(_Whole + _Part));
            }
        }
    };
} // namespace chrono
_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // _STL_COMPILER_PREPROCESSOR
#endif // __MSVC_CHRONO_HPP
