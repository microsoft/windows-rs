// __msvc_ranges_to.hpp internal header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// This header provides ranges::to (C++23) and supporting machinery (C++20).

#ifndef __MSVC_RANGES_TO_HPP
#define __MSVC_RANGES_TO_HPP
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR

#if !_HAS_CXX20
#error The contents of <ranges> are only available with C++20. (Also, you should not include this internal header.)
#endif // !_HAS_CXX20

#include <tuple>
#include <xutility>

#if _HAS_CXX23
#include <xmemory>
#endif // _HAS_CXX23

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

_STD_BEGIN
namespace ranges {
    template <class _Ty>
    constexpr bool _Is_initializer_list = _Is_specialization_v<remove_cvref_t<_Ty>, initializer_list>;

    _EXPORT_STD template <class _Rng>
    concept viewable_range = range<_Rng>
                          && ((view<remove_cvref_t<_Rng>> && constructible_from<remove_cvref_t<_Rng>, _Rng>)
                              || (!view<remove_cvref_t<_Rng>>
                                  && (is_lvalue_reference_v<_Rng>
                                      || (movable<remove_reference_t<_Rng>> && !_Is_initializer_list<_Rng>) )));

    namespace _Pipe {
        template <class _Derived>
        struct _Base {};

        template <class _Ty>
        _Ty* _Derived_from_range_adaptor_closure(_Base<_Ty>&); // not defined

        template <class _Ty>
        concept _Range_adaptor_closure_object = !range<remove_cvref_t<_Ty>> && requires(remove_cvref_t<_Ty>& __t) {
            { _Pipe::_Derived_from_range_adaptor_closure(__t) } -> same_as<remove_cvref_t<_Ty>*>;
        };

        template <class _ClosureLeft, class _ClosureRight>
        struct _Pipeline : _Base<_Pipeline<_ClosureLeft, _ClosureRight>> {
            _STL_INTERNAL_STATIC_ASSERT(_Range_adaptor_closure_object<_ClosureLeft>);
            _STL_INTERNAL_STATIC_ASSERT(_Range_adaptor_closure_object<_ClosureRight>);

            /* [[no_unique_address]] */ _ClosureLeft _Left;
            /* [[no_unique_address]] */ _ClosureRight _Right;

            template <class _Ty1, class _Ty2>
            constexpr explicit _Pipeline(_Ty1&& _Val1, _Ty2&& _Val2) noexcept(
                is_nothrow_constructible_v<_Ty1, _ClosureLeft> && is_nothrow_constructible_v<_Ty2, _ClosureRight>)
                : _Left(_STD forward<_Ty1>(_Val1)), _Right(_STD forward<_Ty2>(_Val2)) {}

            void operator()(auto&&) &       = delete;
            void operator()(auto&&) const&  = delete;
            void operator()(auto&&) &&      = delete;
            void operator()(auto&&) const&& = delete;

            template <class _Ty>
            _NODISCARD constexpr decltype(auto) operator()(_Ty&& _Val) & noexcept(
                noexcept(_Right(_Left(_STD forward<_Ty>(_Val)))))
                requires requires { _Right(_Left(_STD forward<_Ty>(_Val))); }
            {
                return _Right(_Left(_STD forward<_Ty>(_Val)));
            }

            template <class _Ty>
            _NODISCARD constexpr decltype(auto) operator()(_Ty&& _Val) const& noexcept(
                noexcept(_Right(_Left(_STD forward<_Ty>(_Val)))))
                requires requires { _Right(_Left(_STD forward<_Ty>(_Val))); }
            {
                return _Right(_Left(_STD forward<_Ty>(_Val)));
            }

            template <class _Ty>
            _NODISCARD constexpr decltype(auto) operator()(_Ty&& _Val) && noexcept(
                noexcept(_STD move(_Right)(_STD move(_Left)(_STD forward<_Ty>(_Val)))))
                requires requires { _STD move(_Right)(_STD move(_Left)(_STD forward<_Ty>(_Val))); }
            {
                return _STD move(_Right)(_STD move(_Left)(_STD forward<_Ty>(_Val)));
            }

            template <class _Ty>
            _NODISCARD constexpr decltype(auto) operator()(_Ty&& _Val) const&& noexcept(
                noexcept(_STD move(_Right)(_STD move(_Left)(_STD forward<_Ty>(_Val)))))
                requires requires { _STD move(_Right)(_STD move(_Left)(_STD forward<_Ty>(_Val))); }
            {
                return _STD move(_Right)(_STD move(_Left)(_STD forward<_Ty>(_Val)));
            }
        };

        template <class _Ty1, class _Ty2>
        _Pipeline(_Ty1, _Ty2) -> _Pipeline<_Ty1, _Ty2>;

        _EXPORT_STD template <class _Left, class _Right>
            requires _Range_adaptor_closure_object<_Left> && _Range_adaptor_closure_object<_Right>
                  && constructible_from<remove_cvref_t<_Left>, _Left>
                  && constructible_from<remove_cvref_t<_Right>, _Right>
        _NODISCARD constexpr auto operator|(_Left&& __l, _Right&& __r)
            noexcept(noexcept(_Pipeline{static_cast<_Left&&>(__l), static_cast<_Right&&>(__r)})) {
            return _Pipeline{static_cast<_Left&&>(__l), static_cast<_Right&&>(__r)};
        }

        _EXPORT_STD template <class _Left, class _Right>
            requires (_Range_adaptor_closure_object<_Right> && range<_Left>)
        _NODISCARD constexpr decltype(auto) operator|(_Left&& __l, _Right&& __r)
            noexcept(noexcept(_STD forward<_Right>(__r)(_STD forward<_Left>(__l))))
            requires requires { static_cast<_Right&&>(__r)(static_cast<_Left&&>(__l)); }
        {
            return _STD forward<_Right>(__r)(_STD forward<_Left>(__l));
        }
    } // namespace _Pipe

    template <class _Ty>
    concept _Valid_movable_box_object =
#if _HAS_CXX23
        move_constructible<_Ty>
#else // ^^^ _HAS_CXX23 / !_HAS_CXX23 vvv
        copy_constructible<_Ty>
#endif // ^^^ !_HAS_CXX23 ^^^
        && _Destructible_object<_Ty>;

    // A simplified optional that augments copy_constructible types with full copyability,
    // and move_constructible types with full movability.
    // In C++20, this corresponds to copyable-box.
    template <_Valid_movable_box_object _Ty>
    class _Movable_box {
    public:
        constexpr _Movable_box() noexcept(is_nothrow_default_constructible_v<_Ty>)
            requires default_initializable<_Ty>
            : _Val(), _Engaged{true} {}

        template <class... _Types>
        constexpr _Movable_box(in_place_t, _Types&&... _Args)
            noexcept(is_nothrow_constructible_v<_Ty, _Types...>) // strengthened
            : _Val(_STD forward<_Types>(_Args)...), _Engaged{true} {}

        constexpr ~_Movable_box() {
            if (_Engaged) {
                _Val.~_Ty();
            }
        }

        ~_Movable_box()
            requires is_trivially_destructible_v<_Ty>
        = default;

        _Movable_box(const _Movable_box&)
            requires copy_constructible<_Ty> && is_trivially_copy_constructible_v<_Ty>
        = default;

        constexpr _Movable_box(const _Movable_box& _That)
            requires copy_constructible<_Ty>
            : _Engaged{_That._Engaged} {
            if (_That._Engaged) {
                _STD _Construct_in_place(_Val, static_cast<const _Ty&>(_That._Val));
            }
        }

        _Movable_box(_Movable_box&&)
            requires is_trivially_move_constructible_v<_Ty>
        = default;

        constexpr _Movable_box(_Movable_box&& _That) : _Engaged{_That._Engaged} {
            if (_That._Engaged) {
                _STD _Construct_in_place(_Val, static_cast<_Ty&&>(_That._Val));
            }
        }

        _Movable_box& operator=(const _Movable_box&) noexcept
            requires copyable<_Ty> && is_trivially_copy_assignable_v<_Ty>
        = default;

        constexpr _Movable_box& operator=(const _Movable_box& _That)
            noexcept(is_nothrow_copy_constructible_v<_Ty> && is_nothrow_copy_assignable_v<_Ty>) // strengthened
            requires copyable<_Ty>
        {
            if (_Engaged) {
                if (_That._Engaged) {
                    static_cast<_Ty&>(_Val) = static_cast<const _Ty&>(_That._Val);
                } else {
                    _Val.~_Ty();
                    _Engaged = false;
                }
            } else {
                if (_That._Engaged) {
                    _STD _Construct_in_place(_Val, static_cast<const _Ty&>(_That._Val));
                    _Engaged = true;
                } else {
                    // nothing to do
                }
            }

            return *this;
        }

        constexpr _Movable_box& operator=(const _Movable_box& _That) noexcept(is_nothrow_copy_constructible_v<_Ty>)
            requires copy_constructible<_Ty>
        {
            if (_STD addressof(_That) == this) {
                return *this;
            }

            if (_Engaged) {
                _Val.~_Ty();
                _Engaged = false;
            }

            if (_That._Engaged) {
                _STD _Construct_in_place(_Val, static_cast<const _Ty&>(_That._Val));
                _Engaged = true;
            }

            return *this;
        }

        _Movable_box& operator=(_Movable_box&&) noexcept
            requires movable<_Ty> && is_trivially_move_assignable_v<_Ty>
        = default;

        constexpr _Movable_box& operator=(_Movable_box&& _That)
            noexcept(is_nothrow_move_constructible_v<_Ty> && is_nothrow_move_assignable_v<_Ty>) // strengthened
            requires movable<_Ty>
        {
            if (_Engaged) {
                if (_That._Engaged) {
                    static_cast<_Ty&>(_Val) = static_cast<_Ty&&>(_That._Val);
                } else {
                    _Val.~_Ty();
                    _Engaged = false;
                }
            } else {
                if (_That._Engaged) {
                    _STD _Construct_in_place(_Val, static_cast<_Ty&&>(_That._Val));
                    _Engaged = true;
                } else {
                    // nothing to do
                }
            }

            return *this;
        }

        constexpr _Movable_box& operator=(_Movable_box&& _That) noexcept(is_nothrow_move_constructible_v<_Ty>) {
            if (_STD addressof(_That) == this) {
                return *this;
            }

            if (_Engaged) {
                _Val.~_Ty();
                _Engaged = false;
            }

            if (_That._Engaged) {
                _STD _Construct_in_place(_Val, static_cast<_Ty&&>(_That._Val));
                _Engaged = true;
            }

            return *this;
        }

        constexpr explicit operator bool() const noexcept {
            return _Engaged;
        }

        _NODISCARD constexpr _Ty& operator*() noexcept {
            _STL_INTERNAL_CHECK(_Engaged);
            return _Val;
        }
        _NODISCARD constexpr const _Ty& operator*() const noexcept {
            _STL_INTERNAL_CHECK(_Engaged);
            return _Val;
        }

    private:
        union {
            remove_cv_t<_Ty> _Val;
        };
        bool _Engaged;
    };

    // [range.move.wrap]
    template <class _Ty>
    concept _Use_simple_movable_box_wrapper =
        (copy_constructible<_Ty>
                // 1. If copy_constructible<T> is true, movable-box<T> should store only a T if either T models
                // copyable, or is_nothrow_move_constructible_v<T> && is_nothrow_copy_constructible_v<T> is true.
                ? copyable<_Ty> || (is_nothrow_move_constructible_v<_Ty> && is_nothrow_copy_constructible_v<_Ty>)
                // 2. Otherwise, movable-box<T> should store only a T if either T models movable or
                // is_nothrow_move_constructible_v<T> is true.
                : movable<_Ty> || is_nothrow_move_constructible_v<_Ty>);

    template <class _Ty>
    concept _Copy_constructible_for_box = is_copy_constructible_v<_Ty>;

    template <_Valid_movable_box_object _Ty>
        requires _Use_simple_movable_box_wrapper<_Ty>
    class _Movable_box<_Ty> { // provide the same API more efficiently when we can avoid the disengaged state
    public:
        _Movable_box()
            requires default_initializable<_Ty>
        = default;

        template <class... _Types>
        constexpr _Movable_box(in_place_t, _Types&&... _Args)
            noexcept(is_nothrow_constructible_v<_Ty, _Types...>) // strengthened
            : _Val(_STD forward<_Types>(_Args)...) {}

        _Movable_box(const _Movable_box&)
            requires _Copy_constructible_for_box<_Ty> && is_trivially_copy_constructible_v<_Ty>
        = default;
        _Movable_box(_Movable_box&&)
            requires is_trivially_move_constructible_v<_Ty>
        = default;

        constexpr _Movable_box(const _Movable_box& _That) noexcept(is_nothrow_copy_constructible_v<_Ty>)
            requires _Copy_constructible_for_box<_Ty>
            : _Val(static_cast<const _Ty&>(_That._Val)) {}

        constexpr _Movable_box(_Movable_box&& _That) noexcept(is_nothrow_move_constructible_v<_Ty>)
            : _Val(static_cast<_Ty&&>(_That._Val)) {}

        _Movable_box& operator=(const _Movable_box&)
            requires copyable<_Ty> && is_trivially_copy_assignable_v<_Ty>
        = default;
        _Movable_box& operator=(_Movable_box&&)
            requires movable<_Ty> && is_trivially_move_assignable_v<_Ty>
        = default;

        constexpr _Movable_box& operator=(const _Movable_box& _That)
            noexcept(is_nothrow_copy_assignable_v<_Ty> || !copyable<_Ty>) // strengthened
            requires copy_constructible<_Ty>
        {
            if constexpr (copyable<_Ty>) {
                static_cast<_Ty&>(_Val) = static_cast<const _Ty&>(_That._Val);
            } else {
                if (_STD addressof(_That) != this) {
                    _Val.~_Ty();
                    _STD _Construct_in_place(_Val, static_cast<const _Ty&>(_That._Val));
                }
            }
            return *this;
        }

        constexpr _Movable_box& operator=(_Movable_box&& _That)
            noexcept(is_nothrow_move_assignable_v<_Ty> || !movable<_Ty>) /* strengthened */ {
            if constexpr (movable<_Ty>) {
                static_cast<_Ty&>(_Val) = static_cast<_Ty&&>(_That._Val);
            } else {
                if (_STD addressof(_That) != this) {
                    _Val.~_Ty();
                    _STD _Construct_in_place(_Val, static_cast<_Ty&&>(_That._Val));
                }
            }
            return *this;
        }

        constexpr explicit operator bool() const noexcept {
            return true;
        }

        _NODISCARD constexpr _Ty& operator*() noexcept {
            return _Val;
        }
        _NODISCARD constexpr const _Ty& operator*() const noexcept {
            return _Val;
        }

    private:
        /* [[no_unique_address]] */ remove_cv_t<_Ty> _Val{};
    };

    template <class _Fn, class... _Types>
    class _Range_closure : public _Pipe::_Base<_Range_closure<_Fn, _Types...>> {
    public:
        // We assume that _Fn is the type of a customization point object. That means
        // 1. The behavior of operator() is independent of cvref qualifiers, so we can use `invocable<_Fn, ` without
        //    loss of generality, and
        // 2. _Fn must be default-constructible and stateless, so we can create instances "on-the-fly" and avoid
        //    storing a copy.

        _STL_INTERNAL_STATIC_ASSERT((same_as<decay_t<_Types>, _Types> && ...));
        _STL_INTERNAL_STATIC_ASSERT(is_empty_v<_Fn>&& is_default_constructible_v<_Fn>);

        template <class... _UTypes>
            requires (same_as<decay_t<_UTypes>, _Types> && ...)
        constexpr explicit _Range_closure(_UTypes&&... _Args)
            noexcept(conjunction_v<is_nothrow_constructible<_Types, _UTypes>...>)
            : _Captures(_STD forward<_UTypes>(_Args)...) {}

        void operator()(auto&&) &       = delete;
        void operator()(auto&&) const&  = delete;
        void operator()(auto&&) &&      = delete;
        void operator()(auto&&) const&& = delete;

        using _Indices = index_sequence_for<_Types...>;

        template <class _Ty>
            requires invocable<_Fn, _Ty, _Types&...>
        constexpr decltype(auto) operator()(_Ty&& _Arg) & noexcept(
            noexcept(_Call(*this, _STD forward<_Ty>(_Arg), _Indices{}))) {
            return _Call(*this, _STD forward<_Ty>(_Arg), _Indices{});
        }

        template <class _Ty>
            requires invocable<_Fn, _Ty, const _Types&...>
        constexpr decltype(auto) operator()(_Ty&& _Arg) const& noexcept(
            noexcept(_Call(*this, _STD forward<_Ty>(_Arg), _Indices{}))) {
            return _Call(*this, _STD forward<_Ty>(_Arg), _Indices{});
        }

        template <class _Ty>
            requires invocable<_Fn, _Ty, _Types...>
        constexpr decltype(auto) operator()(_Ty&& _Arg) && noexcept(
            noexcept(_Call(_STD move(*this), _STD forward<_Ty>(_Arg), _Indices{}))) {
            return _Call(_STD move(*this), _STD forward<_Ty>(_Arg), _Indices{});
        }

        template <class _Ty>
            requires invocable<_Fn, _Ty, const _Types...>
        constexpr decltype(auto) operator()(_Ty&& _Arg) const&& noexcept(
            noexcept(_Call(_STD move(*this), _STD forward<_Ty>(_Arg), _Indices{}))) {
            return _Call(_STD move(*this), _STD forward<_Ty>(_Arg), _Indices{});
        }

    private:
        template <class _SelfTy, class _Ty, size_t... _Idx>
        static constexpr decltype(auto) _Call(_SelfTy&& _Self, _Ty&& _Arg, index_sequence<_Idx...>) noexcept(
            noexcept(_Fn{}(_STD forward<_Ty>(_Arg), _STD get<_Idx>(_STD forward<_SelfTy>(_Self)._Captures)...))) {
            _STL_INTERNAL_STATIC_ASSERT(same_as<index_sequence<_Idx...>, _Indices>);
            return _Fn{}(_STD forward<_Ty>(_Arg), _STD get<_Idx>(_STD forward<_SelfTy>(_Self)._Captures)...);
        }

        tuple<_Types...> _Captures;
    };

    _EXPORT_STD template <range _Rng>
        requires is_object_v<_Rng>
    class ref_view : public view_interface<ref_view<_Rng>> {
    private:
        _Rng* _Range;

        static void _Rvalue_poison(_Rng&);
        static void _Rvalue_poison(_Rng&&) = delete;

    public:
        template <_Different_from<ref_view> _OtherRng>
        constexpr ref_view(_OtherRng&& _Other)
            noexcept(noexcept(static_cast<_Rng&>(_STD forward<_OtherRng>(_Other)))) // strengthened
            requires convertible_to<_OtherRng, _Rng&> && requires { _Rvalue_poison(static_cast<_OtherRng&&>(_Other)); }
            : _Range{_STD addressof(static_cast<_Rng&>(_STD forward<_OtherRng>(_Other)))} {}

        _NODISCARD constexpr _Rng& base() const noexcept /* strengthened */ {
            return *_Range;
        }

        _NODISCARD constexpr iterator_t<_Rng> begin() const
            noexcept(noexcept(_RANGES begin(*_Range))) /* strengthened */ {
            return _RANGES begin(*_Range);
        }

        _NODISCARD constexpr sentinel_t<_Rng> end() const noexcept(noexcept(_RANGES end(*_Range))) /* strengthened */ {
            return _RANGES end(*_Range);
        }

        _NODISCARD constexpr bool empty() const noexcept(noexcept(_RANGES empty(*_Range))) /* strengthened */
            requires _Can_empty<_Rng>
        {
            return _RANGES empty(*_Range);
        }

        _NODISCARD constexpr auto size() const noexcept(noexcept(_RANGES size(*_Range))) /* strengthened */
            requires sized_range<_Rng>
        {
            return _RANGES size(*_Range);
        }

        _NODISCARD constexpr auto data() const noexcept(noexcept(_RANGES data(*_Range))) /* strengthened */
            requires contiguous_range<_Rng>
        {
            return _RANGES data(*_Range);
        }
    };

    template <class _Rng>
    ref_view(_Rng&) -> ref_view<_Rng>;

    template <class _Rng>
    constexpr bool enable_borrowed_range<ref_view<_Rng>> = true;

    _EXPORT_STD template <range _Rng>
        requires (movable<_Rng> && !_Is_initializer_list<_Rng>)
    class owning_view : public view_interface<owning_view<_Rng>> {
    private:
        _Rng _Range{};

    public:
        owning_view()
            requires default_initializable<_Rng>
        = default;

        constexpr owning_view(_Rng&& _Range_) noexcept(is_nothrow_move_constructible_v<_Rng>) // strengthened
            : _Range(_STD move(_Range_)) {}

        owning_view(owning_view&&)            = default;
        owning_view& operator=(owning_view&&) = default;

        _NODISCARD constexpr _Rng& base() & noexcept {
            return _Range;
        }
        _NODISCARD constexpr const _Rng& base() const& noexcept {
            return _Range;
        }
        _NODISCARD constexpr _Rng&& base() && noexcept {
            return _STD move(_Range);
        }
        _NODISCARD constexpr const _Rng&& base() const&& noexcept {
            return _STD move(_Range);
        }

        _NODISCARD constexpr iterator_t<_Rng> begin() noexcept(noexcept(_RANGES begin(_Range))) /* strengthened */ {
            return _RANGES begin(_Range);
        }

        _NODISCARD constexpr sentinel_t<_Rng> end() noexcept(noexcept(_RANGES end(_Range))) /* strengthened */ {
            return _RANGES end(_Range);
        }

        _NODISCARD constexpr auto begin() const noexcept(noexcept(_RANGES begin(_Range))) /* strengthened */
            requires range<const _Rng>
        {
            return _RANGES begin(_Range);
        }

        _NODISCARD constexpr auto end() const noexcept(noexcept(_RANGES end(_Range))) /* strengthened */
            requires range<const _Rng>
        {
            return _RANGES end(_Range);
        }

        _NODISCARD constexpr bool empty() noexcept(noexcept(_RANGES empty(_Range))) /* strengthened */
            requires _Can_empty<_Rng>
        {
            return _RANGES empty(_Range);
        }
        _NODISCARD constexpr bool empty() const noexcept(noexcept(_RANGES empty(_Range))) /* strengthened */
            requires _Can_empty<const _Rng>
        {
            return _RANGES empty(_Range);
        }

        _NODISCARD constexpr auto size() noexcept(noexcept(_RANGES size(_Range))) /* strengthened */
            requires sized_range<_Rng>
        {
            return _RANGES size(_Range);
        }
        _NODISCARD constexpr auto size() const noexcept(noexcept(_RANGES size(_Range))) /* strengthened */
            requires sized_range<const _Rng>
        {
            return _RANGES size(_Range);
        }

        _NODISCARD constexpr auto data() noexcept(noexcept(_RANGES data(_Range))) /* strengthened */
            requires contiguous_range<_Rng>
        {
            return _RANGES data(_Range);
        }
        _NODISCARD constexpr auto data() const noexcept(noexcept(_RANGES data(_Range))) /* strengthened */
            requires contiguous_range<const _Rng>
        {
            return _RANGES data(_Range);
        }
    };

    template <class _Rng>
    constexpr bool enable_borrowed_range<owning_view<_Rng>> = enable_borrowed_range<_Rng>;

    namespace views {
        template <class _Rng>
        concept _Can_ref_view = requires(_Rng&& __r) { ref_view{static_cast<_Rng&&>(__r)}; };

        template <class _Rng>
        concept _Ownable = requires(_Rng&& __r) { owning_view{static_cast<_Rng&&>(__r)}; };

        class _All_fn : public _Pipe::_Base<_All_fn> {
        private:
            enum class _St { _None, _View, _Ref, _Own };

            template <class _Rng>
            _NODISCARD static consteval _Choice_t<_St> _Choose() noexcept {
                if constexpr (view<remove_cvref_t<_Rng>>) {
                    if constexpr (convertible_to<_Rng, remove_cvref_t<_Rng>>) {
                        return {_St::_View, is_nothrow_convertible_v<_Rng, remove_cvref_t<_Rng>>};
                    }
                } else if constexpr (_Can_ref_view<_Rng>) {
                    return {_St::_Ref, noexcept(ref_view{_STD declval<_Rng>()})};
                } else if constexpr (_Ownable<_Rng>) {
                    return {_St::_Own, noexcept(owning_view{_STD declval<_Rng>()})};
                }

                return {_St::_None};
            }

            template <class _Rng>
            static constexpr _Choice_t<_St> _Choice = _Choose<_Rng>();

        public:
            template <viewable_range _Rng>
                requires (_Choice<_Rng>._Strategy != _St::_None)
            _NODISCARD _STATIC_CALL_OPERATOR constexpr auto operator()(_Rng&& _Range) _CONST_CALL_OPERATOR
                noexcept(_Choice<_Rng>._No_throw) {
                constexpr _St _Strat = _Choice<_Rng>._Strategy;

                if constexpr (_Strat == _St::_View) {
                    return _STD forward<_Rng>(_Range);
                } else if constexpr (_Strat == _St::_Ref) {
                    return ref_view{_STD forward<_Rng>(_Range)};
                } else if constexpr (_Strat == _St::_Own) {
                    return owning_view{_STD forward<_Rng>(_Range)};
                } else {
                    _STL_INTERNAL_STATIC_ASSERT(false); // unexpected strategy
                }
            }
        };

        _EXPORT_STD inline constexpr _All_fn all;

        _EXPORT_STD template <viewable_range _Rng>
        using all_t = decltype(all(_STD declval<_Rng>()));
    } // namespace views

    _EXPORT_STD template <input_range _Vw, _Valid_movable_box_object _Fn>
        requires view<_Vw> && regular_invocable<_Fn&, range_reference_t<_Vw>>
              && _Can_reference<invoke_result_t<_Fn&, range_reference_t<_Vw>>>
    class transform_view : public view_interface<transform_view<_Vw, _Fn>> {
    private:
        /* [[no_unique_address]] */ _Vw _Range{};
        /* [[no_unique_address]] */ _Movable_box<_Fn> _Fun{};

        template <bool _Const>
        struct _Category_base {};

        template <bool _Const>
            requires forward_range<_Maybe_const<_Const, _Vw>>
        struct _Category_base<_Const> {
            using _Base = _Maybe_const<_Const, _Vw>;
            using iterator_category =
                conditional_t<is_reference_v<invoke_result_t<_Maybe_const<_Const, _Fn>&, range_reference_t<_Base>>>,
                    conditional_t<derived_from<_Iter_cat_t<iterator_t<_Base>>, contiguous_iterator_tag>,
                        random_access_iterator_tag, _Iter_cat_t<iterator_t<_Base>>>,
                    input_iterator_tag>;
        };

        template <bool _Const>
        class _Iterator : public _Category_base<_Const> {
        private:
            friend transform_view;

            using _Parent_t = _Maybe_const<_Const, transform_view>;
            using _Base     = _Maybe_const<_Const, _Vw>;

            iterator_t<_Base> _Current{};
            _Parent_t* _Parent{};

#if _ITERATOR_DEBUG_LEVEL != 0
            constexpr void _Check_dereference() const noexcept {
                _STL_VERIFY(_Parent != nullptr, "cannot dereference value-initialized transform_view iterator");
                _STL_VERIFY(_Current != _RANGES end(_Parent->_Range), "cannot dereference end transform_view iterator");
            }
#endif // _ITERATOR_DEBUG_LEVEL != 0

#if _ITERATOR_DEBUG_LEVEL != 0
            constexpr void _Same_range(const _Iterator& _Right) const noexcept {
                _STL_VERIFY(_Parent == _Right._Parent, "cannot compare incompatible transform_view iterators");
            }
#endif // _ITERATOR_DEBUG_LEVEL != 0

        public:
            using iterator_concept = conditional_t<random_access_range<_Base>, random_access_iterator_tag,
                conditional_t<bidirectional_range<_Base>, bidirectional_iterator_tag,
                    conditional_t<forward_range<_Base>, forward_iterator_tag, input_iterator_tag>>>;
            using value_type = remove_cvref_t<invoke_result_t<_Maybe_const<_Const, _Fn>&, range_reference_t<_Base>>>;
            using difference_type = range_difference_t<_Base>;

            _Iterator()
                requires default_initializable<iterator_t<_Base>>
            = default;

            constexpr _Iterator(_Parent_t& _Parent_, iterator_t<_Base> _Current_)
                noexcept(is_nothrow_move_constructible_v<iterator_t<_Base>>) // strengthened
                : _Current{_STD move(_Current_)}, _Parent{_STD addressof(_Parent_)} {
#if _ITERATOR_DEBUG_LEVEL != 0
                _STD _Adl_verify_range(_Current, _RANGES end(_Parent_._Range));
                if constexpr (forward_range<_Base>) {
                    _STD _Adl_verify_range(_RANGES begin(_Parent_._Range), _Current);
                }
#endif // _ITERATOR_DEBUG_LEVEL != 0
            }

            constexpr _Iterator(_Iterator<!_Const> _It)
                noexcept(is_nothrow_constructible_v<iterator_t<_Base>, iterator_t<_Vw>>) // strengthened
                requires _Const && convertible_to<iterator_t<_Vw>, iterator_t<_Base>>
                : _Current(_STD move(_It._Current)), _Parent(_It._Parent) {}

            _NODISCARD constexpr const iterator_t<_Base>& base() const& noexcept {
                return _Current;
            }
            _NODISCARD constexpr iterator_t<_Base> base() && noexcept(
                is_nothrow_move_constructible_v<iterator_t<_Base>>) /* strengthened */ {
                return _STD move(_Current);
            }

            _NODISCARD constexpr decltype(auto) operator*() const
                noexcept(noexcept(_STD invoke(*_Parent->_Fun, *_Current))) {
#if _ITERATOR_DEBUG_LEVEL != 0
                _Check_dereference();
                _STL_VERIFY(
                    _Parent->_Fun, "Cannot dereference iterator into transform_view with no transformation function");
#endif // _ITERATOR_DEBUG_LEVEL != 0
                return _STD invoke(*_Parent->_Fun, *_Current);
            }

            constexpr _Iterator& operator++() noexcept(noexcept(++_Current)) /* strengthened */ {
#if _ITERATOR_DEBUG_LEVEL != 0
                _STL_VERIFY(_Parent != nullptr, "Cannot increment value-initialized transform_view iterator");
                _STL_VERIFY(
                    _Current != _RANGES end(_Parent->_Range), "Cannot increment transform_view iterator past end");
#endif // _ITERATOR_DEBUG_LEVEL != 0
                ++_Current;
                return *this;
            }

            constexpr decltype(auto) operator++(int) noexcept(
                noexcept(++_Current)
                && (!forward_range<_Base> || is_nothrow_copy_constructible_v<iterator_t<_Base>>) ) /* strengthened */ {
                if constexpr (forward_range<_Base>) {
                    auto _Tmp = *this;
                    ++*this;
                    return _Tmp;
                } else {
                    ++*this;
                }
            }

            constexpr _Iterator& operator--() noexcept(noexcept(--_Current)) /* strengthened */
                requires bidirectional_range<_Base>
            {
#if _ITERATOR_DEBUG_LEVEL != 0
                _STL_VERIFY(_Parent != nullptr, "Cannot decrement value-initialized transform_view iterator");
                if constexpr (forward_range<_Vw>) {
                    _STL_VERIFY(_Current != _RANGES begin(_Parent->_Range),
                        "Cannot decrement transform_view iterator before begin");
                }
#endif // _ITERATOR_DEBUG_LEVEL != 0
                --_Current;
                return *this;
            }
            constexpr _Iterator operator--(int)
                noexcept(noexcept(--_Current) && is_nothrow_copy_constructible_v<iterator_t<_Base>>) /* strengthened */
                requires bidirectional_range<_Base>
            {
                auto _Tmp = *this;
                --*this;
                return _Tmp;
            }

#if _ITERATOR_DEBUG_LEVEL != 0
            constexpr void _Verify_offset(const difference_type _Off) const noexcept
                requires random_access_range<_Base>
            {
                _STL_VERIFY(_Off == 0 || _Parent, "cannot seek value-initialized transform_view iterator");
                if constexpr (_Offset_verifiable_v<iterator_t<_Base>>) {
                    _Current._Verify_offset(_Off);
                }
            }
#endif // _ITERATOR_DEBUG_LEVEL != 0

            constexpr _Iterator& operator+=(const difference_type _Off)
                noexcept(noexcept(_Current += _Off)) /* strengthened */
                requires random_access_range<_Base>
            {
#if _ITERATOR_DEBUG_LEVEL != 0
                _Verify_offset(_Off);
#endif // _ITERATOR_DEBUG_LEVEL != 0
                _Current += _Off;
                return *this;
            }
            constexpr _Iterator& operator-=(const difference_type _Off)
                noexcept(noexcept(_Current -= _Off)) /* strengthened */
                requires random_access_range<_Base>
            {
#if _ITERATOR_DEBUG_LEVEL != 0
                _STL_VERIFY(_Off != _Min_possible_v<difference_type>, "integer overflow");
                _Verify_offset(-_Off);
#endif // _ITERATOR_DEBUG_LEVEL != 0
                _Current -= _Off;
                return *this;
            }

            _NODISCARD constexpr decltype(auto) operator[](const difference_type _Idx) const
                noexcept(noexcept(_STD invoke(*_Parent->_Fun, _Current[_Idx]))) /* strengthened */
                requires random_access_range<_Base>
            {
#if _ITERATOR_DEBUG_LEVEL != 0
                _Verify_offset(_Idx);
                _STL_VERIFY(
                    _Parent->_Fun, "Cannot dereference iterator into transform_view with no transformation function");
#endif // _ITERATOR_DEBUG_LEVEL != 0
                return _STD invoke(*_Parent->_Fun, _Current[_Idx]);
            }

            _NODISCARD friend constexpr bool operator==(const _Iterator& _Left, const _Iterator& _Right)
                noexcept(noexcept(_Left._Current == _Right._Current)) /* strengthened */
                requires equality_comparable<iterator_t<_Base>>
            {
#if _ITERATOR_DEBUG_LEVEL != 0
                _Left._Same_range(_Right);
#endif // _ITERATOR_DEBUG_LEVEL != 0
                return _Left._Current == _Right._Current;
            }

            _NODISCARD friend constexpr bool operator<(const _Iterator& _Left, const _Iterator& _Right)
                noexcept(noexcept(_Left._Current < _Right._Current)) /* strengthened */
                requires random_access_range<_Base>
            {
#if _ITERATOR_DEBUG_LEVEL != 0
                _Left._Same_range(_Right);
#endif // _ITERATOR_DEBUG_LEVEL != 0
                return _Left._Current < _Right._Current;
            }
            _NODISCARD friend constexpr bool operator>(const _Iterator& _Left, const _Iterator& _Right)
                noexcept(noexcept(_Left._Current < _Right._Current)) /* strengthened */
                requires random_access_range<_Base>
            {
                return _Right < _Left;
            }
            _NODISCARD friend constexpr bool operator<=(const _Iterator& _Left, const _Iterator& _Right)
                noexcept(noexcept(_Left._Current < _Right._Current)) /* strengthened */
                requires random_access_range<_Base>
            {
                return !(_Right < _Left);
            }
            _NODISCARD friend constexpr bool operator>=(const _Iterator& _Left, const _Iterator& _Right)
                noexcept(noexcept(_Left._Current < _Right._Current)) /* strengthened */
                requires random_access_range<_Base>
            {
                return !(_Left < _Right);
            }
            _NODISCARD friend constexpr auto operator<=>(const _Iterator& _Left, const _Iterator& _Right)
                noexcept(noexcept(_Left._Current <=> _Right._Current)) /* strengthened */
                requires random_access_range<_Base> && three_way_comparable<iterator_t<_Base>>
            {
#if _ITERATOR_DEBUG_LEVEL != 0
                _Left._Same_range(_Right);
#endif // _ITERATOR_DEBUG_LEVEL != 0
                return _Left._Current <=> _Right._Current;
            }

            _NODISCARD friend constexpr _Iterator operator+(_Iterator _It, const difference_type _Off)
                noexcept(noexcept(_It._Current += _Off)) /* strengthened */
                requires random_access_range<_Base>
            {
#if _ITERATOR_DEBUG_LEVEL != 0
                _It._Verify_offset(_Off);
#endif // _ITERATOR_DEBUG_LEVEL != 0
                _It._Current += _Off;
                return _It;
            }
            _NODISCARD friend constexpr _Iterator operator+(const difference_type _Off, _Iterator _It)
                noexcept(noexcept(_It._Current += _Off)) /* strengthened */
                requires random_access_range<_Base>
            {
#if _ITERATOR_DEBUG_LEVEL != 0
                _It._Verify_offset(_Off);
#endif // _ITERATOR_DEBUG_LEVEL != 0
                _It._Current += _Off;
                return _It;
            }

            _NODISCARD friend constexpr _Iterator operator-(_Iterator _It, const difference_type _Off)
                noexcept(noexcept(_It._Current -= _Off)) /* strengthened */
                requires random_access_range<_Base>
            {
#if _ITERATOR_DEBUG_LEVEL != 0
                _STL_VERIFY(_Off != _Min_possible_v<difference_type>, "integer overflow");
                _It._Verify_offset(-_Off);
#endif // _ITERATOR_DEBUG_LEVEL != 0
                _It._Current -= _Off;
                return _It;
            }

            _NODISCARD friend constexpr difference_type operator-(const _Iterator& _Left, const _Iterator& _Right)
                noexcept(noexcept(_Left._Current - _Right._Current)) /* strengthened */
                requires sized_sentinel_for<iterator_t<_Base>, iterator_t<_Base>>
            {
#if _ITERATOR_DEBUG_LEVEL != 0
                _Left._Same_range(_Right);
#endif // _ITERATOR_DEBUG_LEVEL != 0
                return _Left._Current - _Right._Current;
            }
        };

        template <bool _Const>
        class _Sentinel {
        private:
            friend transform_view;

            using _Parent_t = _Maybe_const<_Const, transform_view>;
            using _Base     = _Maybe_const<_Const, _Vw>;
            template <bool _OtherConst>
            using _Maybe_const_iter = iterator_t<_Maybe_const<_OtherConst, _Vw>>;

            sentinel_t<_Base> _Last{};

            template <bool _OtherConst>
            _NODISCARD static constexpr const _Maybe_const_iter<_OtherConst>& _Get_current(
                const _Iterator<_OtherConst>& _It) noexcept {
#if _ITERATOR_DEBUG_LEVEL != 0
                _STL_VERIFY(
                    _It._Parent != nullptr, "cannot compare transform_view sentinel with value-initialized iterator");
#endif // _ITERATOR_DEBUG_LEVEL != 0
                return _It._Current;
            }

        public:
            _Sentinel() = default;
            constexpr explicit _Sentinel(sentinel_t<_Base> _Last_)
                noexcept(is_nothrow_move_constructible_v<sentinel_t<_Base>>) // strengthened
                : _Last(_STD move(_Last_)) {}

            constexpr _Sentinel(_Sentinel<!_Const> _Se)
                noexcept(is_nothrow_constructible_v<sentinel_t<_Base>, sentinel_t<_Vw>>) // strengthened
                requires _Const && convertible_to<sentinel_t<_Vw>, sentinel_t<_Base>>
                : _Last(_STD move(_Se._Last)) {}

            _NODISCARD constexpr sentinel_t<_Base> base() const
                noexcept(is_nothrow_copy_constructible_v<sentinel_t<_Base>>) /* strengthened */ {
                return _Last;
            }

            template <bool _OtherConst>
                requires sentinel_for<sentinel_t<_Base>, _Maybe_const_iter<_OtherConst>>
            _NODISCARD friend constexpr bool operator==(const _Iterator<_OtherConst>& _Left, const _Sentinel& _Right)
                noexcept(noexcept(_Get_current(_Left) == _Right._Last)) /* strengthened */ {
                return _Get_current(_Left) == _Right._Last;
            }

            template <bool _OtherConst>
                requires sized_sentinel_for<sentinel_t<_Base>, _Maybe_const_iter<_OtherConst>>
            _NODISCARD friend constexpr range_difference_t<_Maybe_const<_OtherConst, _Vw>> operator-(
                const _Iterator<_OtherConst>& _Left, const _Sentinel& _Right)
                noexcept(noexcept(_Get_current(_Left) - _Right._Last)) /* strengthened */ {
                return _Get_current(_Left) - _Right._Last;
            }

            template <bool _OtherConst>
                requires sized_sentinel_for<sentinel_t<_Base>, _Maybe_const_iter<_OtherConst>>
            _NODISCARD friend constexpr range_difference_t<_Maybe_const<_OtherConst, _Vw>> operator-(
                const _Sentinel& _Left, const _Iterator<_OtherConst>& _Right)
                noexcept(noexcept(_Left._Last - _Get_current(_Right))) /* strengthened */ {
                return _Left._Last - _Get_current(_Right);
            }
        };

    public:
        transform_view()
            requires default_initializable<_Vw> && default_initializable<_Fn>
        = default;

        constexpr explicit transform_view(_Vw _Range_, _Fn _Fun_)
            noexcept(is_nothrow_move_constructible_v<_Vw> && is_nothrow_move_constructible_v<_Fn>) // strengthened
            : _Range(_STD move(_Range_)), _Fun{in_place, _STD move(_Fun_)} {}

        _NODISCARD constexpr _Vw base() const& noexcept(is_nothrow_copy_constructible_v<_Vw>) /* strengthened */
            requires copy_constructible<_Vw>
        {
            return _Range;
        }
        _NODISCARD constexpr _Vw base() && noexcept(is_nothrow_move_constructible_v<_Vw>) /* strengthened */ {
            return _STD move(_Range);
        }

        _NODISCARD constexpr _Iterator<false> begin() noexcept(
            noexcept(_RANGES begin(_Range)) && is_nothrow_move_constructible_v<iterator_t<_Vw>>) /* strengthened */ {
            return _Iterator<false>{*this, _RANGES begin(_Range)};
        }

        _NODISCARD constexpr _Iterator<true> begin() const noexcept(
            noexcept(_RANGES begin(_Range)) && is_nothrow_move_constructible_v<iterator_t<_Vw>>) /* strengthened */
            requires range<const _Vw> && regular_invocable<const _Fn&, range_reference_t<const _Vw>>
        {
            return _Iterator<true>{*this, _RANGES begin(_Range)};
        }

        _NODISCARD constexpr auto end()
            noexcept(noexcept(_RANGES end(_Range))
                     && is_nothrow_move_constructible_v<decltype(_RANGES end(_Range))>) /* strengthened */ {
            if constexpr (common_range<_Vw>) {
                return _Iterator<false>{*this, _RANGES end(_Range)};
            } else {
                return _Sentinel<false>{_RANGES end(_Range)};
            }
        }

        _NODISCARD constexpr auto end() const
            noexcept(noexcept(_RANGES end(_Range))
                     && is_nothrow_move_constructible_v<decltype(_RANGES end(_Range))>) /* strengthened */
            requires range<const _Vw> && regular_invocable<const _Fn&, range_reference_t<const _Vw>>
        {
            if constexpr (common_range<_Vw>) {
                return _Iterator<true>{*this, _RANGES end(_Range)};
            } else {
                return _Sentinel<true>{_RANGES end(_Range)};
            }
        }

        _NODISCARD constexpr auto size() noexcept(noexcept(_RANGES size(_Range))) /* strengthened */
            requires sized_range<_Vw>
        {
            return _RANGES size(_Range);
        }
        _NODISCARD constexpr auto size() const noexcept(noexcept(_RANGES size(_Range))) /* strengthened */
            requires sized_range<const _Vw>
        {
            return _RANGES size(_Range);
        }
    };

    template <class _Rng, class _Fn>
    transform_view(_Rng&&, _Fn) -> transform_view<views::all_t<_Rng>, _Fn>;

    namespace views {
        struct _Transform_fn {
            template <viewable_range _Rng, class _Fn>
            _NODISCARD _STATIC_CALL_OPERATOR constexpr auto operator()(_Rng&& _Range, _Fn _Fun) _CONST_CALL_OPERATOR
                noexcept(noexcept(transform_view(_STD forward<_Rng>(_Range), _STD move(_Fun))))
                requires requires { transform_view(static_cast<_Rng&&>(_Range), _STD move(_Fun)); }
            {
                return transform_view(_STD forward<_Rng>(_Range), _STD move(_Fun));
            }

            template <class _Fn>
                requires constructible_from<decay_t<_Fn>, _Fn>
            _NODISCARD _STATIC_CALL_OPERATOR constexpr auto operator()(_Fn&& _Fun) _CONST_CALL_OPERATOR
                noexcept(is_nothrow_constructible_v<decay_t<_Fn>, _Fn>) {
                return _Range_closure<_Transform_fn, decay_t<_Fn>>{_STD forward<_Fn>(_Fun)};
            }
        };

        _EXPORT_STD inline constexpr _Transform_fn transform;
    } // namespace views

#if _HAS_CXX23
    template <class _Range, class _Container>
    concept _Sized_and_reservable = sized_range<_Range> && sized_range<_Container>
                                 && requires(_Container& _Cont, const range_size_t<_Container> _Count) {
                                        _Cont.reserve(_Count);
                                        { _Cont.capacity() } -> same_as<range_size_t<_Container>>;
                                        { _Cont.max_size() } -> same_as<range_size_t<_Container>>;
                                    };

    template <class _Rng, class _Container>
    concept _Ref_converts =
        !input_range<_Container> || convertible_to<range_reference_t<_Rng>, range_value_t<_Container>>;

    template <class _Rng, class _Container, class... _Types>
    concept _Common_constructible =
        common_range<_Rng> //
        && requires { typename iterator_traits<iterator_t<_Rng>>::iterator_category; }
        && derived_from<typename iterator_traits<iterator_t<_Rng>>::iterator_category, input_iterator_tag>
        && constructible_from<_Container, iterator_t<_Rng>, iterator_t<_Rng>, _Types...>;

    template <class _Container, class _Reference>
    concept _Can_emplace_back = requires(_Container& _Cont) { _Cont.emplace_back(_STD declval<_Reference>()); };

    template <class _Container, class _Reference>
    concept _Can_push_back = requires(_Container& _Cont) { _Cont.push_back(_STD declval<_Reference>()); };

    template <class _Container, class _Reference>
    concept _Can_emplace_end = requires(_Container& _Cont) { _Cont.emplace(_Cont.end(), _STD declval<_Reference>()); };

    template <class _Container, class _Reference>
    concept _Can_insert_end = requires(_Container& _Cont) { _Cont.insert(_Cont.end(), _STD declval<_Reference>()); };

    template <class _Rng, class _Container, class... _Types>
    concept _Constructible_appendable = constructible_from<_Container, _Types...>
                                     && (_Can_emplace_back<_Container, range_reference_t<_Rng>>
                                         || _Can_push_back<_Container, range_reference_t<_Rng>>
                                         || _Can_emplace_end<_Container, range_reference_t<_Rng>>
                                         || _Can_insert_end<_Container, range_reference_t<_Rng>>);

    _EXPORT_STD template <class _Container, input_range _Rng, class... _Types>
        requires (!view<_Container>)
    _NODISCARD constexpr _Container to(_Rng&& _Range, _Types&&... _Args) {
        static_assert(!is_const_v<_Container>, "C must not be const. ([range.utility.conv.to])");
        static_assert(!is_volatile_v<_Container>, "C must not be volatile. ([range.utility.conv.to])");
        static_assert(is_class_v<_Container>, "C must be a class type. ([range.utility.conv.to])");
        if constexpr (_Ref_converts<_Rng, _Container>) {
            if constexpr (constructible_from<_Container, _Rng, _Types...>) {
                return _Container(_STD forward<_Rng>(_Range), _STD forward<_Types>(_Args)...);
            } else if constexpr (constructible_from<_Container, const from_range_t&, _Rng, _Types...>) { // per LWG-3845
                return _Container(from_range, _STD forward<_Rng>(_Range), _STD forward<_Types>(_Args)...);
            } else if constexpr (_Common_constructible<_Rng, _Container, _Types...>) {
                return _Container(_RANGES begin(_Range), _RANGES end(_Range), _STD forward<_Types>(_Args)...);
            } else if constexpr (_Constructible_appendable<_Rng, _Container, _Types...>) {
                _Container _Cont(_STD forward<_Types>(_Args)...);
                if constexpr (_Sized_and_reservable<_Rng, _Container>) {
                    _Cont.reserve(static_cast<range_size_t<_Container>>(_RANGES size(_Range)));
                }

                auto _Iter       = _RANGES begin(_Range);
                const auto _Sent = _RANGES end(_Range);
                for (; _Iter != _Sent; ++_Iter) {
                    auto&& _Elem  = *_Iter;
                    using _ElemTy = decltype(_Elem);
                    if constexpr (_Can_emplace_back<_Container, _ElemTy>) {
                        _Cont.emplace_back(_STD forward<_ElemTy>(_Elem));
                    } else if constexpr (_Can_push_back<_Container, _ElemTy>) {
                        _Cont.push_back(_STD forward<_ElemTy>(_Elem));
                    } else if constexpr (_Can_emplace_end<_Container, _ElemTy>) {
                        _Cont.emplace(_Cont.end(), _STD forward<_ElemTy>(_Elem));
                    } else {
                        _STL_INTERNAL_STATIC_ASSERT(_Can_insert_end<_Container, _ElemTy>);
                        _Cont.insert(_Cont.end(), _STD forward<_ElemTy>(_Elem));
                    }
                }
                return _Cont;
            } else {
                static_assert(false, "ranges::to requires the result to be constructible from the source range, either "
                                     "by using a suitable constructor, or by inserting each element of the range into "
                                     "the default-constructed object. (N4981 [range.utility.conv.to]/2.1.5)");
            }
        } else if constexpr (input_range<range_reference_t<_Rng>>) {
            const auto _Xform = [](auto&& _Elem) _STATIC_LAMBDA {
                return _RANGES to<range_value_t<_Container>>(_STD forward<decltype(_Elem)>(_Elem));
            };
            return _RANGES to<_Container>(views::transform(ref_view{_Range}, _Xform), _STD forward<_Types>(_Args)...);
        } else {
            static_assert(false,
                "ranges::to requires the elements of the source range to be either implicitly convertible to the "
                "elements of the destination container, or be ranges themselves for ranges::to to be applied "
                "recursively. (N4981 [range.utility.conv.to]/2.3)");
        }
    }

    template <class _Container>
    struct _To_class_fn {
        _STL_INTERNAL_STATIC_ASSERT(!is_const_v<_Container>);
        _STL_INTERNAL_STATIC_ASSERT(!is_volatile_v<_Container>);
        _STL_INTERNAL_STATIC_ASSERT(is_class_v<_Container>);
        _STL_INTERNAL_STATIC_ASSERT(!view<_Container>);

        template <input_range _Rng, class... _Types>
        _NODISCARD _STATIC_CALL_OPERATOR constexpr auto operator()(
            _Rng&& _Range, _Types&&... _Args) _CONST_CALL_OPERATOR
            requires requires { _RANGES to<_Container>(_STD forward<_Rng>(_Range), _STD forward<_Types>(_Args)...); }
        {
            return _RANGES to<_Container>(_STD forward<_Rng>(_Range), _STD forward<_Types>(_Args)...);
        }
    };

    _EXPORT_STD template <class _Container, class... _Types>
        requires (!view<_Container>)
    _NODISCARD constexpr auto to(_Types&&... _Args) {
        static_assert(!is_const_v<_Container>, "C must not be const. ([range.utility.conv.adaptors])");
        static_assert(!is_volatile_v<_Container>, "C must not be volatile. ([range.utility.conv.adaptors])");
        static_assert(is_class_v<_Container>, "C must be a class type. ([range.utility.conv.adaptors])");
        return _Range_closure<_To_class_fn<_Container>, decay_t<_Types>...>{_STD forward<_Types>(_Args)...};
    }

    template <input_range _Rng>
    struct _Phony_input_iterator {
        using value_type      = range_value_t<_Rng>;
        using difference_type = ptrdiff_t;

        // These member functions are never defined:
        range_reference_t<_Rng> operator*() const;
        add_pointer_t<range_reference_t<_Rng>> operator->() const;
        _Phony_input_iterator& operator++();
        _Phony_input_iterator operator++(int);
        bool operator==(const _Phony_input_iterator&) const;
    };

    template <template <class...> class _Cnt, class _Rng, class... _Args>
    auto _To_helper() {
        if constexpr (requires { _Cnt(_STD declval<_Rng>(), _STD declval<_Args>()...); }) {
            return static_cast<decltype(_Cnt(_STD declval<_Rng>(), _STD declval<_Args>()...))*>(nullptr);
        } else if constexpr (requires { _Cnt(from_range, _STD declval<_Rng>(), _STD declval<_Args>()...); }) {
            return static_cast<decltype(_Cnt(from_range, _STD declval<_Rng>(), _STD declval<_Args>()...))*>(nullptr);
        } else if constexpr (requires {
                                 _Cnt(_STD declval<_Phony_input_iterator<_Rng>>(),
                                     _STD declval<_Phony_input_iterator<_Rng>>(), _STD declval<_Args>()...);
                             }) {
            return static_cast<decltype(_Cnt(_STD declval<_Phony_input_iterator<_Rng>>(),
                _STD declval<_Phony_input_iterator<_Rng>>(), _STD declval<_Args>()...))*>(nullptr);
        }
    }

    _EXPORT_STD template <template <class...> class _Container, input_range _Rng, class... _Types,
        class _Deduced = remove_pointer_t<decltype(_To_helper<_Container, _Rng, _Types...>())>>
    _NODISCARD constexpr _Deduced to(_Rng&& _Range, _Types&&... _Args) {
        return _RANGES to<_Deduced>(_STD forward<_Rng>(_Range), _STD forward<_Types>(_Args)...);
    }

    template <template <class...> class _Container>
    struct _To_template_fn {
        template <input_range _Rng, class... _Types,
            class _Deduced = remove_pointer_t<decltype(_To_helper<_Container, _Rng, _Types...>())>>
        _NODISCARD _STATIC_CALL_OPERATOR constexpr auto operator()(
            _Rng&& _Range, _Types&&... _Args) _CONST_CALL_OPERATOR {
            return _RANGES to<_Deduced>(_STD forward<_Rng>(_Range), _STD forward<_Types>(_Args)...);
        }
    };

    _EXPORT_STD template <template <class...> class _Container, class... _Types>
    _NODISCARD constexpr auto to(_Types&&... _Args) {
        return _Range_closure<_To_template_fn<_Container>, decay_t<_Types>...>{_STD forward<_Types>(_Args)...};
    }
#endif // _HAS_CXX23
} // namespace ranges
_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // _STL_COMPILER_PREPROCESSOR
#endif // __MSVC_RANGES_TO_HPP
