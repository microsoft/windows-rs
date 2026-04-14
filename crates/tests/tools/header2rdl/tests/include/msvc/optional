// optional standard header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef _OPTIONAL_
#define _OPTIONAL_
#include <yvals.h>
#if _STL_COMPILER_PREPROCESSOR
#if !_HAS_CXX17
_EMIT_STL_WARNING(STL4038, "The contents of <optional> are available only with C++17 or later.");
#else // ^^^ !_HAS_CXX17 / _HAS_CXX17 vvv
#if _HAS_CXX20
#include <compare>
#endif // _HAS_CXX20
#include <exception>
#include <initializer_list>
#include <type_traits>
#include <utility>
#include <xsmf_control.h>
#include <xutility>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

_STD_BEGIN

_EXPORT_STD struct nullopt_t { // no-value state indicator
    struct _Tag {};
    constexpr explicit nullopt_t(_Tag) {}
};
_EXPORT_STD inline constexpr nullopt_t nullopt{nullopt_t::_Tag{}};

_EXPORT_STD class _NODISCARD bad_optional_access : public exception {
public:
    _NODISCARD const char* __CLR_OR_THIS_CALL what() const noexcept override {
        return "Bad optional access";
    }

#if !_HAS_EXCEPTIONS
protected:
    void _Doraise() const override { // perform class-specific exception handling
        _RAISE(*this);
    }
#endif // !_HAS_EXCEPTIONS
};

[[noreturn]] inline void _Throw_bad_optional_access() {
    _THROW(bad_optional_access{});
}

struct _Nontrivial_dummy_type {
    constexpr _Nontrivial_dummy_type() noexcept {
        // This default constructor is user-provided to avoid zero-initialization when objects are value-initialized.
    }
};
_STL_INTERNAL_STATIC_ASSERT(!is_trivially_default_constructible_v<_Nontrivial_dummy_type>);

#if _HAS_CXX23
struct _Construct_from_invoke_result_tag {
    explicit _Construct_from_invoke_result_tag() = default;
};
#endif // _HAS_CXX23

template <class _Ty, bool = is_trivially_destructible_v<_Ty>>
struct _Optional_destruct_base { // either contains a value of _Ty or is empty (trivial destructor)
    union {
        _Nontrivial_dummy_type _Dummy;
        remove_cv_t<_Ty> _Value;
    };
    bool _Has_value;

    constexpr _Optional_destruct_base() noexcept : _Dummy{}, _Has_value{false} {} // initialize an empty optional

    template <class... _Types>
    constexpr explicit _Optional_destruct_base(in_place_t, _Types&&... _Args)
        noexcept(is_nothrow_constructible_v<_Ty, _Types...>)
        : _Value(_STD forward<_Types>(_Args)...), _Has_value{true} {} // initialize contained value with _Args...

#if _HAS_CXX23
    template <class _Fn, class _Ux>
    constexpr _Optional_destruct_base(_Construct_from_invoke_result_tag, _Fn&& _Func, _Ux&& _Arg)
        noexcept(noexcept(static_cast<_Ty>(_STD invoke(_STD forward<_Fn>(_Func), _STD forward<_Ux>(_Arg)))))
        : _Value(_STD invoke(_STD forward<_Fn>(_Func), _STD forward<_Ux>(_Arg))), _Has_value{true} {}
#endif // _HAS_CXX23

    // For the trivially destructible case, we can't add a destructor for _MSVC_STL_DESTRUCTOR_TOMBSTONES, due to
    // N5001 [optional.dtor]/2: "Remarks: If is_trivially_destructible_v<T> is true, then this destructor is trivial."

    _CONSTEXPR20 void reset() noexcept {
        _Has_value = false;
    }
};

template <class _Ty>
struct _Optional_destruct_base<_Ty, false> { // either contains a value of _Ty or is empty (non-trivial destructor)
    union {
        _Nontrivial_dummy_type _Dummy;
        remove_cv_t<_Ty> _Value;
    };
    bool _Has_value;

    _CONSTEXPR20 ~_Optional_destruct_base() noexcept {
        if (_Has_value) {
            _Value.~_Ty();

#if _MSVC_STL_DESTRUCTOR_TOMBSTONES
            // For the non-trivially destructible case, we can set the optional to be empty.
            // We don't attempt to scribble over the bytes of the object's storage because that could be expensive
            // and we don't know whether the object has an invalid representation, much less what it could be.
            _Has_value = false;
#endif
        }
    }

    constexpr _Optional_destruct_base() noexcept : _Dummy{}, _Has_value{false} {} // initialize an empty optional

    template <class... _Types>
    constexpr explicit _Optional_destruct_base(in_place_t, _Types&&... _Args)
        noexcept(is_nothrow_constructible_v<_Ty, _Types...>)
        : _Value(_STD forward<_Types>(_Args)...), _Has_value{true} {} // initialize contained value with _Args...

#if _HAS_CXX23
    template <class _Fn, class _Ux>
    constexpr _Optional_destruct_base(_Construct_from_invoke_result_tag, _Fn&& _Func, _Ux&& _Arg)
        noexcept(noexcept(static_cast<_Ty>(_STD invoke(_STD forward<_Fn>(_Func), _STD forward<_Ux>(_Arg)))))
        : _Value(_STD invoke(_STD forward<_Fn>(_Func), _STD forward<_Ux>(_Arg))), _Has_value{true} {}
#endif // _HAS_CXX23

    _Optional_destruct_base(const _Optional_destruct_base&)            = default;
    _Optional_destruct_base(_Optional_destruct_base&&)                 = default;
    _Optional_destruct_base& operator=(const _Optional_destruct_base&) = default;
    _Optional_destruct_base& operator=(_Optional_destruct_base&&)      = default;

    _CONSTEXPR20 void reset() noexcept {
        if (_Has_value) {
            _Value.~_Ty();
            _Has_value = false;
        }
    }
};

template <class _Ty>
struct _Optional_construct_base : _Optional_destruct_base<_Ty> {
    // Provide non-trivial SMF implementations for the _SMF_control machinery
    using _Optional_destruct_base<_Ty>::_Optional_destruct_base;

    template <class... _Types>
    _CONSTEXPR20 _Ty& _Construct(_Types&&... _Args) noexcept(is_nothrow_constructible_v<_Ty, _Types...>) {
        // transition from the empty to the value-containing state
        _STL_INTERNAL_CHECK(!this->_Has_value);
        _STD _Construct_in_place(this->_Value, _STD forward<_Types>(_Args)...);
        this->_Has_value = true;
        return this->_Value;
    }

    template <class _Ty2>
    _CONSTEXPR20 void _Assign(_Ty2&& _Right)
        noexcept(is_nothrow_assignable_v<_Ty&, _Ty2> && is_nothrow_constructible_v<_Ty, _Ty2>) {
        // assign / initialize the contained value from _Right
        if (this->_Has_value) {
            static_cast<_Ty&>(this->_Value) = _STD forward<_Ty2>(_Right);
        } else {
            _Construct(_STD forward<_Ty2>(_Right));
        }
    }

    template <class _Self>
    _CONSTEXPR20 void _Construct_from(_Self&& _Right)
        noexcept(is_nothrow_constructible_v<_Ty, decltype(*_STD forward<_Self>(_Right))>) {
        // initialize contained value from _Right iff it contains a value
        if (_Right._Has_value) {
            _Construct(*_STD forward<_Self>(_Right));
        }
    }

    template <class _Self>
    _CONSTEXPR20 void _Assign_from(_Self&& _Right)
        noexcept(is_nothrow_constructible_v<_Ty, decltype(*_STD forward<_Self>(_Right))>
                 && is_nothrow_assignable_v<_Ty&, decltype(*_STD forward<_Self>(_Right))>) {
        // assign/initialize/destroy contained value from _Right
        if (_Right._Has_value) {
            _Assign(*_STD forward<_Self>(_Right));
        } else {
            this->reset();
        }
    }

    _NODISCARD constexpr _Ty& operator*() & noexcept {
#if _MSVC_STL_HARDENING_OPTIONAL || _ITERATOR_DEBUG_LEVEL != 0
        _STL_VERIFY(this->_Has_value, "operator*() called on empty optional");
#endif

        return this->_Value;
    }

    _NODISCARD constexpr const _Ty& operator*() const& noexcept {
#if _MSVC_STL_HARDENING_OPTIONAL || _ITERATOR_DEBUG_LEVEL != 0
        _STL_VERIFY(this->_Has_value, "operator*() called on empty optional");
#endif

        return this->_Value;
    }

    _NODISCARD constexpr _Ty&& operator*() && noexcept {
#if _MSVC_STL_HARDENING_OPTIONAL || _ITERATOR_DEBUG_LEVEL != 0
        _STL_VERIFY(this->_Has_value, "operator*() called on empty optional");
#endif

        return _STD move(this->_Value);
    }

    _NODISCARD constexpr const _Ty&& operator*() const&& noexcept {
#if _MSVC_STL_HARDENING_OPTIONAL || _ITERATOR_DEBUG_LEVEL != 0
        _STL_VERIFY(this->_Has_value, "operator*() called on empty optional");
#endif

        return _STD move(this->_Value);
    }
};

_EXPORT_STD template <class _Ty>
class optional : private _SMF_control<_Optional_construct_base<_Ty>, _Ty> {
private:
    using _Mybase = _SMF_control<_Optional_construct_base<_Ty>, _Ty>;

    template <class>
    friend class optional;

#if _HAS_CXX23
    template <class _Fn, class _Ux>
    constexpr optional(_Construct_from_invoke_result_tag _Tag, _Fn&& _Func, _Ux&& _Arg)
        noexcept(noexcept(static_cast<_Ty>(_STD invoke(_STD forward<_Fn>(_Func), _STD forward<_Ux>(_Arg)))))
        : _Mybase(_Tag, _STD forward<_Fn>(_Func), _STD forward<_Ux>(_Arg)) {}
#endif // _HAS_CXX23

public:
    static_assert(!_Is_any_of_v<remove_cv_t<_Ty>, nullopt_t, in_place_t>,
        "T in optional<T> must be a type other than nullopt_t or in_place_t (N4950 [optional.optional.general]/3).");
    static_assert(is_object_v<_Ty> && is_destructible_v<_Ty> && !is_array_v<_Ty>,
        "T in optional<T> must meet the Cpp17Destructible requirements (N4950 [optional.optional.general]/3).");

    using value_type = _Ty;

    constexpr optional() noexcept {}
    constexpr optional(nullopt_t) noexcept {}

    template <class... _Types, enable_if_t<is_constructible_v<_Ty, _Types...>, int> = 0>
    constexpr explicit optional(in_place_t, _Types&&... _Args)
        noexcept(is_nothrow_constructible_v<_Ty, _Types...>) // strengthened
        : _Mybase(in_place, _STD forward<_Types>(_Args)...) {}

    template <class _Elem, class... _Types,
        enable_if_t<is_constructible_v<_Ty, initializer_list<_Elem>&, _Types...>, int> = 0>
    constexpr explicit optional(in_place_t, initializer_list<_Elem> _Ilist, _Types&&... _Args)
        noexcept(is_nothrow_constructible_v<_Ty, initializer_list<_Elem>&, _Types...>) // strengthened
        : _Mybase(in_place, _Ilist, _STD forward<_Types>(_Args)...) {}

    template <class _Ty2>
    using _AllowDirectConversion = bool_constant<conjunction_v<negation<is_same<_Remove_cvref_t<_Ty2>, optional>>,
        negation<is_same<_Remove_cvref_t<_Ty2>, in_place_t>>,
        negation<conjunction<is_same<remove_cv_t<_Ty>, bool>, _Is_specialization<_Remove_cvref_t<_Ty2>, optional>>>,
        is_constructible<_Ty, _Ty2>>>;

    template <class _Ty2 = remove_cv_t<_Ty>, enable_if_t<_AllowDirectConversion<_Ty2>::value, int> = 0>
    constexpr explicit(!is_convertible_v<_Ty2, _Ty>) optional(_Ty2&& _Right)
        noexcept(is_nothrow_constructible_v<_Ty, _Ty2>) // strengthened
        : _Mybase(in_place, _STD forward<_Ty2>(_Right)) {}

    template <class _Ty2>
    struct _AllowUnwrapping
        : bool_constant<disjunction_v<is_same<remove_cv_t<_Ty>, bool>,
              negation<disjunction<is_same<_Ty, _Ty2>, is_constructible<_Ty, optional<_Ty2>&>,
                  is_constructible<_Ty, const optional<_Ty2>&>, is_constructible<_Ty, const optional<_Ty2>>,
                  is_constructible<_Ty, optional<_Ty2>>, is_convertible<optional<_Ty2>&, _Ty>,
                  is_convertible<const optional<_Ty2>&, _Ty>, is_convertible<const optional<_Ty2>, _Ty>,
                  is_convertible<optional<_Ty2>, _Ty>>>>> {};

    template <class _Ty2,
        enable_if_t<conjunction_v<_AllowUnwrapping<_Ty2>, is_constructible<_Ty, const _Ty2&>>, int> = 0>
    _CONSTEXPR20 explicit(!is_convertible_v<const _Ty2&, _Ty>) optional(const optional<_Ty2>& _Right)
        noexcept(is_nothrow_constructible_v<_Ty, const _Ty2&>) /* strengthened */ {
        if (_Right) {
            this->_Construct(*_Right);
        }
    }

    template <class _Ty2, enable_if_t<conjunction_v<_AllowUnwrapping<_Ty2>, is_constructible<_Ty, _Ty2>>, int> = 0>
    _CONSTEXPR20 explicit(!is_convertible_v<_Ty2, _Ty>) optional(optional<_Ty2>&& _Right)
        noexcept(is_nothrow_constructible_v<_Ty, _Ty2>) /* strengthened */ {
        if (_Right) {
            this->_Construct(_STD move(*_Right));
        }
    }

    _CONSTEXPR20 optional& operator=(nullopt_t) noexcept {
        reset();
        return *this;
    }

    template <class _Ty2 = remove_cv_t<_Ty>,
        enable_if_t<conjunction_v<negation<is_same<optional, _Remove_cvref_t<_Ty2>>>,
                        negation<conjunction<is_scalar<_Ty>, is_same<_Ty, decay_t<_Ty2>>>>, is_constructible<_Ty, _Ty2>,
                        is_assignable<_Ty&, _Ty2>>,
            int>         = 0>
    _CONSTEXPR20 optional& operator=(_Ty2&& _Right)
        noexcept(is_nothrow_assignable_v<_Ty&, _Ty2> && is_nothrow_constructible_v<_Ty, _Ty2>) /* strengthened */ {
        this->_Assign(_STD forward<_Ty2>(_Right));
        return *this;
    }

    template <class _Ty2>
    struct _AllowUnwrappingAssignment
        : bool_constant<!disjunction_v<is_same<_Ty, _Ty2>, is_assignable<_Ty&, optional<_Ty2>&>,
              is_assignable<_Ty&, const optional<_Ty2>&>, is_assignable<_Ty&, const optional<_Ty2>>,
              is_assignable<_Ty&, optional<_Ty2>>>> {};

    template <class _Ty2, enable_if_t<conjunction_v<_AllowUnwrappingAssignment<_Ty2>,
                                          is_constructible<_Ty, const _Ty2&>, is_assignable<_Ty&, const _Ty2&>>,
                              int> = 0>
    _CONSTEXPR20 optional& operator=(const optional<_Ty2>& _Right) noexcept(
        is_nothrow_assignable_v<_Ty&, const _Ty2&> && is_nothrow_constructible_v<_Ty, const _Ty2&>) /* strengthened */ {
        if (_Right) {
            this->_Assign(*_Right);
        } else {
            reset();
        }

        return *this;
    }

    template <class _Ty2, enable_if_t<conjunction_v<_AllowUnwrappingAssignment<_Ty2>, is_constructible<_Ty, _Ty2>,
                                          is_assignable<_Ty&, _Ty2>>,
                              int> = 0>
    _CONSTEXPR20 optional& operator=(optional<_Ty2>&& _Right)
        noexcept(is_nothrow_assignable_v<_Ty&, _Ty2> && is_nothrow_constructible_v<_Ty, _Ty2>) /* strengthened */ {
        if (_Right) {
            this->_Assign(_STD move(*_Right));
        } else {
            reset();
        }

        return *this;
    }

    template <class... _Types>
    _CONSTEXPR20 _Ty& emplace(_Types&&... _Args)
        noexcept(is_nothrow_constructible_v<_Ty, _Types...>) /* strengthened */ {
        reset();
        return this->_Construct(_STD forward<_Types>(_Args)...);
    }

    template <class _Elem, class... _Types,
        enable_if_t<is_constructible_v<_Ty, initializer_list<_Elem>&, _Types...>, int> = 0>
    _CONSTEXPR20 _Ty& emplace(initializer_list<_Elem> _Ilist, _Types&&... _Args)
        noexcept(is_nothrow_constructible_v<_Ty, initializer_list<_Elem>&, _Types...>) /* strengthened */ {
        reset();
        return this->_Construct(_Ilist, _STD forward<_Types>(_Args)...);
    }

    _CONSTEXPR20 void swap(optional& _Right)
        noexcept(is_nothrow_move_constructible_v<_Ty> && is_nothrow_swappable_v<_Ty>) {
        if constexpr (is_move_constructible_v<_Ty>) {
            static_assert(
                is_swappable_v<_Ty>, "optional<T>::swap requires T to be swappable (N4993 [optional.swap]/2).");
        } else {
            static_assert(false, "optional<T>::swap requires T to be move constructible (N4993 [optional.swap]/1).");
        }
        using _STD swap;
        if constexpr (_Is_trivially_swappable_v<_Ty>) {
            using _TrivialBaseTy = _Optional_destruct_base<_Ty>;
            _STD swap(static_cast<_TrivialBaseTy&>(*this), static_cast<_TrivialBaseTy&>(_Right));
        } else {
            const bool _Engaged = this->_Has_value;
            if (_Engaged == _Right._Has_value) {
                if (_Engaged) {
                    swap(**this, *_Right); // intentional ADL
                }
            } else {
                optional& _Source = _Engaged ? *this : _Right;
                optional& _Target = _Engaged ? _Right : *this;
                _Target._Construct(_STD move(*_Source));
                _Source.reset();
            }
        }
    }

    _NODISCARD constexpr const _Ty* operator->() const noexcept {
#if _MSVC_STL_HARDENING_OPTIONAL || _ITERATOR_DEBUG_LEVEL != 0
        _STL_VERIFY(this->_Has_value, "operator->() called on empty optional");
#endif

        return _STD addressof(this->_Value);
    }
    _NODISCARD constexpr _Ty* operator->() noexcept {
#if _MSVC_STL_HARDENING_OPTIONAL || _ITERATOR_DEBUG_LEVEL != 0
        _STL_VERIFY(this->_Has_value, "operator->() called on empty optional");
#endif

        return _STD addressof(this->_Value);
    }

    using _Mybase::operator*;

    constexpr explicit operator bool() const noexcept {
        return this->_Has_value;
    }
    _NODISCARD constexpr bool has_value() const noexcept {
        return this->_Has_value;
    }

    _NODISCARD constexpr const _Ty& value() const& {
        if (!this->_Has_value) {
            _Throw_bad_optional_access();
        }

        return this->_Value;
    }
    _NODISCARD constexpr _Ty& value() & {
        if (!this->_Has_value) {
            _Throw_bad_optional_access();
        }

        return this->_Value;
    }
    _NODISCARD constexpr _Ty&& value() && {
        if (!this->_Has_value) {
            _Throw_bad_optional_access();
        }

        return _STD move(this->_Value);
    }
    _NODISCARD constexpr const _Ty&& value() const&& {
        if (!this->_Has_value) {
            _Throw_bad_optional_access();
        }

        return _STD move(this->_Value);
    }

    template <class _Ty2 = remove_cv_t<_Ty>>
    _NODISCARD constexpr remove_cv_t<_Ty> value_or(_Ty2&& _Right) const& {
        static_assert(is_convertible_v<const _Ty&, remove_cv_t<_Ty>>,
            "The const overload of optional<T>::value_or requires const T& to be convertible to remove_cv_t<T> "
            "(N4950 [optional.observe]/15 as modified by LWG-3424).");
        static_assert(is_convertible_v<_Ty2, remove_cv_t<_Ty>>,
            "optional<T>::value_or(U) requires U to be convertible to remove_cv_t<T> "
            "(N4950 [optional.observe]/15 as modified by LWG-3424).");

        if (this->_Has_value) {
            return static_cast<const _Ty&>(this->_Value);
        }

        return static_cast<remove_cv_t<_Ty>>(_STD forward<_Ty2>(_Right));
    }
    template <class _Ty2 = remove_cv_t<_Ty>>
    _NODISCARD constexpr remove_cv_t<_Ty> value_or(_Ty2&& _Right) && {
        static_assert(is_convertible_v<_Ty, remove_cv_t<_Ty>>,
            "The rvalue overload of optional<T>::value_or requires T to be convertible to remove_cv_t<T> "
            "(N4950 [optional.observe]/17 as modified by LWG-3424).");
        static_assert(is_convertible_v<_Ty2, remove_cv_t<_Ty>>,
            "optional<T>::value_or(U) requires U to be convertible to remove_cv_t<T> "
            "(N4950 [optional.observe]/17 as modified by LWG-3424).");

        if (this->_Has_value) {
            return static_cast<_Ty&&>(this->_Value);
        }

        return static_cast<remove_cv_t<_Ty>>(_STD forward<_Ty2>(_Right));
    }

#if _HAS_CXX23
    template <class _Fn>
    constexpr auto and_then(_Fn&& _Func) & {
        using _Uty = invoke_result_t<_Fn, _Ty&>;

        static_assert(_Is_specialization_v<remove_cvref_t<_Uty>, optional>,
            "optional<T>::and_then(F) requires the return type of F to be a specialization of optional "
            "(N4950 [optional.monadic]/2).");

        if (this->_Has_value) {
            return _STD invoke(_STD forward<_Fn>(_Func), static_cast<_Ty&>(this->_Value));
        } else {
            return remove_cvref_t<_Uty>{};
        }
    }

    template <class _Fn>
    constexpr auto and_then(_Fn&& _Func) const& {
        using _Uty = invoke_result_t<_Fn, const _Ty&>;

        static_assert(_Is_specialization_v<remove_cvref_t<_Uty>, optional>,
            "optional<T>::and_then(F) requires the return type of F to be a specialization of optional "
            "(N4950 [optional.monadic]/2).");

        if (this->_Has_value) {
            return _STD invoke(_STD forward<_Fn>(_Func), static_cast<const _Ty&>(this->_Value));
        } else {
            return remove_cvref_t<_Uty>{};
        }
    }

    template <class _Fn>
    constexpr auto and_then(_Fn&& _Func) && {
        using _Uty = invoke_result_t<_Fn, _Ty>;

        static_assert(_Is_specialization_v<remove_cvref_t<_Uty>, optional>,
            "optional<T>::and_then(F) requires the return type of F to be a specialization of optional "
            "(N4950 [optional.monadic]/5).");

        if (this->_Has_value) {
            return _STD invoke(_STD forward<_Fn>(_Func), static_cast<_Ty&&>(this->_Value));
        } else {
            return remove_cvref_t<_Uty>{};
        }
    }

    template <class _Fn>
    constexpr auto and_then(_Fn&& _Func) const&& {
        using _Uty = invoke_result_t<_Fn, const _Ty>;

        static_assert(_Is_specialization_v<remove_cvref_t<_Uty>, optional>,
            "optional<T>::and_then(F) requires the return type of F to be a specialization of optional "
            "(N4950 [optional.monadic]/5).");

        if (this->_Has_value) {
            return _STD invoke(_STD forward<_Fn>(_Func), static_cast<const _Ty&&>(this->_Value));
        } else {
            return remove_cvref_t<_Uty>{};
        }
    }

    template <class _Fn>
    constexpr auto transform(_Fn&& _Func) & {
        using _Uty = remove_cv_t<invoke_result_t<_Fn, _Ty&>>;

        static_assert(!_Is_any_of_v<_Uty, nullopt_t, in_place_t>,
            "optional<T>::transform(F) requires the return type of F to be a type other than nullopt_t or in_place_t "
            "(N4950 [optional.monadic]/8).");
        static_assert(is_object_v<_Uty> && !is_array_v<_Uty>,
            "optional<T>::transform(F) requires the return type of F to be a non-array object type "
            "(N4950 [optional.monadic]/8).");

        if (this->_Has_value) {
            return optional<_Uty>{
                _Construct_from_invoke_result_tag{}, _STD forward<_Fn>(_Func), static_cast<_Ty&>(this->_Value)};
        } else {
            return optional<_Uty>{};
        }
    }

    template <class _Fn>
    constexpr auto transform(_Fn&& _Func) const& {
        using _Uty = remove_cv_t<invoke_result_t<_Fn, const _Ty&>>;

        static_assert(!_Is_any_of_v<_Uty, nullopt_t, in_place_t>,
            "optional<T>::transform(F) requires the return type of F to be a type other than nullopt_t or in_place_t "
            "(N4950 [optional.monadic]/8).");
        static_assert(is_object_v<_Uty> && !is_array_v<_Uty>,
            "optional<T>::transform(F) requires the return type of F to be a non-array object type "
            "(N4950 [optional.monadic]/8).");

        if (this->_Has_value) {
            return optional<_Uty>{
                _Construct_from_invoke_result_tag{}, _STD forward<_Fn>(_Func), static_cast<const _Ty&>(this->_Value)};
        } else {
            return optional<_Uty>{};
        }
    }

    template <class _Fn>
    constexpr auto transform(_Fn&& _Func) && {
        using _Uty = remove_cv_t<invoke_result_t<_Fn, _Ty>>;

        static_assert(!_Is_any_of_v<_Uty, nullopt_t, in_place_t>,
            "optional<T>::transform(F) requires the return type of F to be a type other than nullopt_t or in_place_t "
            "(N4950 [optional.monadic]/11).");
        static_assert(is_object_v<_Uty> && !is_array_v<_Uty>,
            "optional<T>::transform(F) requires the return type of F to be a non-array object type "
            "(N4950 [optional.monadic]/11).");

        if (this->_Has_value) {
            return optional<_Uty>{
                _Construct_from_invoke_result_tag{}, _STD forward<_Fn>(_Func), static_cast<_Ty&&>(this->_Value)};
        } else {
            return optional<_Uty>{};
        }
    }

    template <class _Fn>
    constexpr auto transform(_Fn&& _Func) const&& {
        using _Uty = remove_cv_t<invoke_result_t<_Fn, const _Ty>>;

        static_assert(!_Is_any_of_v<_Uty, nullopt_t, in_place_t>,
            "optional<T>::transform(F) requires the return type of F to be a type other than nullopt_t or in_place_t "
            "(N4950 [optional.monadic]/11).");
        static_assert(is_object_v<_Uty> && !is_array_v<_Uty>,
            "optional<T>::transform(F) requires the return type of F to be a non-array object type "
            "(N4950 [optional.monadic]/11).");

        if (this->_Has_value) {
            return optional<_Uty>{
                _Construct_from_invoke_result_tag{}, _STD forward<_Fn>(_Func), static_cast<const _Ty&&>(this->_Value)};
        } else {
            return optional<_Uty>{};
        }
    }

    template <invocable<> _Fn>
        requires copy_constructible<_Ty>
    constexpr optional or_else(_Fn&& _Func) const& {
        static_assert(is_same_v<remove_cvref_t<invoke_result_t<_Fn>>, optional>,
            "optional<T>::or_else(F) requires F to return an optional<T> (N4950 [optional.monadic]/14).");

        if (this->_Has_value) {
            return *this;
        } else {
            return _STD forward<_Fn>(_Func)();
        }
    }

    template <invocable<> _Fn>
        requires move_constructible<_Ty>
    constexpr optional or_else(_Fn&& _Func) && {
        static_assert(is_same_v<remove_cvref_t<invoke_result_t<_Fn>>, optional>,
            "optional<T>::or_else(F) requires F to return an optional<T> (N4950 [optional.monadic]/17).");

        if (this->_Has_value) {
            return _STD move(*this);
        } else {
            return _STD forward<_Fn>(_Func)();
        }
    }
#endif // _HAS_CXX23

    using _Mybase::reset;
};

template <class _Ty>
optional(_Ty) -> optional<_Ty>;

_EXPORT_STD template <class _Ty1, class _Ty2>
_NODISCARD constexpr bool operator==(const optional<_Ty1>& _Left, const optional<_Ty2>& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(*_Left == *_Right))) /* strengthened */
#if _HAS_CXX20
    requires requires {
        { *_Left == *_Right } -> _Implicitly_convertible_to<bool>;
    }
#endif // _HAS_CXX20
{
    const bool _Left_has_value  = _Left.has_value();
    const bool _Right_has_value = _Right.has_value();
    if (_Left_has_value && _Right_has_value) {
        return *_Left == *_Right;
    }
    return _Left_has_value == _Right_has_value;
}

_EXPORT_STD template <class _Ty1, class _Ty2>
_NODISCARD constexpr bool operator!=(const optional<_Ty1>& _Left, const optional<_Ty2>& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(*_Left != *_Right))) /* strengthened */
#if _HAS_CXX20
    requires requires {
        { *_Left != *_Right } -> _Implicitly_convertible_to<bool>;
    }
#endif // _HAS_CXX20
{
    const bool _Left_has_value  = _Left.has_value();
    const bool _Right_has_value = _Right.has_value();
    if (_Left_has_value && _Right_has_value) {
        return *_Left != *_Right;
    }
    return _Left_has_value != _Right_has_value;
}

_EXPORT_STD template <class _Ty1, class _Ty2>
_NODISCARD constexpr bool operator<(const optional<_Ty1>& _Left, const optional<_Ty2>& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(*_Left < *_Right))) /* strengthened */
#if _HAS_CXX20
    requires requires {
        { *_Left < *_Right } -> _Implicitly_convertible_to<bool>;
    }
#endif // _HAS_CXX20
{
    const bool _Left_has_value  = _Left.has_value();
    const bool _Right_has_value = _Right.has_value();
    if (_Left_has_value && _Right_has_value) {
        return *_Left < *_Right;
    }
    return _Left_has_value < _Right_has_value;
}

_EXPORT_STD template <class _Ty1, class _Ty2>
_NODISCARD constexpr bool operator>(const optional<_Ty1>& _Left, const optional<_Ty2>& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(*_Left > *_Right))) /* strengthened */
#if _HAS_CXX20
    requires requires {
        { *_Left > *_Right } -> _Implicitly_convertible_to<bool>;
    }
#endif // _HAS_CXX20
{
    const bool _Left_has_value  = _Left.has_value();
    const bool _Right_has_value = _Right.has_value();
    if (_Left_has_value && _Right_has_value) {
        return *_Left > *_Right;
    }
    return _Left_has_value > _Right_has_value;
}

_EXPORT_STD template <class _Ty1, class _Ty2>
_NODISCARD constexpr bool operator<=(const optional<_Ty1>& _Left, const optional<_Ty2>& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(*_Left <= *_Right))) /* strengthened */
#if _HAS_CXX20
    requires requires {
        { *_Left <= *_Right } -> _Implicitly_convertible_to<bool>;
    }
#endif // _HAS_CXX20
{
    const bool _Left_has_value  = _Left.has_value();
    const bool _Right_has_value = _Right.has_value();
    if (_Left_has_value && _Right_has_value) {
        return *_Left <= *_Right;
    }
    return _Left_has_value <= _Right_has_value;
}

_EXPORT_STD template <class _Ty1, class _Ty2>
_NODISCARD constexpr bool operator>=(const optional<_Ty1>& _Left, const optional<_Ty2>& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(*_Left >= *_Right))) /* strengthened */
#if _HAS_CXX20
    requires requires {
        { *_Left >= *_Right } -> _Implicitly_convertible_to<bool>;
    }
#endif // _HAS_CXX20
{
    const bool _Left_has_value  = _Left.has_value();
    const bool _Right_has_value = _Right.has_value();
    if (_Left_has_value && _Right_has_value) {
        return *_Left >= *_Right;
    }
    return _Left_has_value >= _Right_has_value;
}

#if _HAS_CXX20
_EXPORT_STD template <class _Ty1, three_way_comparable_with<_Ty1> _Ty2>
_NODISCARD constexpr compare_three_way_result_t<_Ty1, _Ty2> operator<=>(const optional<_Ty1>& _Left,
    const optional<_Ty2>& _Right) noexcept(noexcept(*_Left <=> *_Right)) /* strengthened */ {
    const bool _Left_has_value  = _Left.has_value();
    const bool _Right_has_value = _Right.has_value();
    if (_Left_has_value && _Right_has_value) {
        return *_Left <=> *_Right;
    }

    return _Left_has_value <=> _Right_has_value;
}
#endif // _HAS_CXX20

_EXPORT_STD template <class _Ty>
_NODISCARD constexpr bool operator==(const optional<_Ty>& _Left, nullopt_t) noexcept {
    return !_Left.has_value();
}

#if _HAS_CXX20
_EXPORT_STD template <class _Ty>
_NODISCARD constexpr strong_ordering operator<=>(const optional<_Ty>& _Left, nullopt_t) noexcept {
    return _Left.has_value() <=> false;
}
#else // ^^^ _HAS_CXX20 / !_HAS_CXX20 vvv
template <class _Ty>
_NODISCARD constexpr bool operator==(nullopt_t, const optional<_Ty>& _Right) noexcept {
    return !_Right.has_value();
}

template <class _Ty>
_NODISCARD constexpr bool operator!=(const optional<_Ty>& _Left, nullopt_t) noexcept {
    return _Left.has_value();
}
template <class _Ty>
_NODISCARD constexpr bool operator!=(nullopt_t, const optional<_Ty>& _Right) noexcept {
    return _Right.has_value();
}

template <class _Ty>
_NODISCARD constexpr bool operator<(const optional<_Ty>&, nullopt_t) noexcept {
    return false;
}
template <class _Ty>
_NODISCARD constexpr bool operator<(nullopt_t, const optional<_Ty>& _Right) noexcept {
    return _Right.has_value();
}

template <class _Ty>
_NODISCARD constexpr bool operator>(const optional<_Ty>& _Left, nullopt_t) noexcept {
    return _Left.has_value();
}
template <class _Ty>
_NODISCARD constexpr bool operator>(nullopt_t, const optional<_Ty>&) noexcept {
    return false;
}

template <class _Ty>
_NODISCARD constexpr bool operator<=(const optional<_Ty>& _Left, nullopt_t) noexcept {
    return !_Left.has_value();
}
template <class _Ty>
_NODISCARD constexpr bool operator<=(nullopt_t, const optional<_Ty>&) noexcept {
    return true;
}

template <class _Ty>
_NODISCARD constexpr bool operator>=(const optional<_Ty>&, nullopt_t) noexcept {
    return true;
}
template <class _Ty>
_NODISCARD constexpr bool operator>=(nullopt_t, const optional<_Ty>& _Right) noexcept {
    return !_Right.has_value();
}
#endif // ^^^ !_HAS_CXX20 ^^^

template <class _Ty>
using _Enable_if_bool_convertible = enable_if_t<is_convertible_v<_Ty, bool>, int>;

template <class _Lhs, class _Rhs>
using _Enable_if_comparable_with_equal =
    _Enable_if_bool_convertible<decltype(_STD declval<const _Lhs&>() == _STD declval<const _Rhs&>())>;

template <class _Lhs, class _Rhs>
using _Enable_if_comparable_with_not_equal =
    _Enable_if_bool_convertible<decltype(_STD declval<const _Lhs&>() != _STD declval<const _Rhs&>())>;

template <class _Lhs, class _Rhs>
using _Enable_if_comparable_with_less =
    _Enable_if_bool_convertible<decltype(_STD declval<const _Lhs&>() < _STD declval<const _Rhs&>())>;

template <class _Lhs, class _Rhs>
using _Enable_if_comparable_with_greater =
    _Enable_if_bool_convertible<decltype(_STD declval<const _Lhs&>() > _STD declval<const _Rhs&>())>;

template <class _Lhs, class _Rhs>
using _Enable_if_comparable_with_less_equal =
    _Enable_if_bool_convertible<decltype(_STD declval<const _Lhs&>() <= _STD declval<const _Rhs&>())>;

template <class _Lhs, class _Rhs>
using _Enable_if_comparable_with_greater_equal =
    _Enable_if_bool_convertible<decltype(_STD declval<const _Lhs&>() >= _STD declval<const _Rhs&>())>;

_EXPORT_STD template <class _Ty1, class _Ty2, _Enable_if_comparable_with_equal<_Ty1, _Ty2> = 0>
_NODISCARD constexpr bool operator==(const optional<_Ty1>& _Left, const _Ty2& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(*_Left == _Right))) /* strengthened */ {
    if (_Left) {
        return *_Left == _Right;
    }
    return false;
}

_EXPORT_STD template <class _Ty1, class _Ty2, _Enable_if_comparable_with_equal<_Ty1, _Ty2> = 0>
_NODISCARD constexpr bool operator==(const _Ty1& _Left, const optional<_Ty2>& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(_Left == *_Right))) /* strengthened */ {
    if (_Right) {
        return _Left == *_Right;
    }
    return false;
}

_EXPORT_STD template <class _Ty1, class _Ty2, _Enable_if_comparable_with_not_equal<_Ty1, _Ty2> = 0>
_NODISCARD constexpr bool operator!=(const optional<_Ty1>& _Left, const _Ty2& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(*_Left != _Right))) /* strengthened */ {
    if (_Left) {
        return *_Left != _Right;
    }
    return true;
}
_EXPORT_STD template <class _Ty1, class _Ty2, _Enable_if_comparable_with_not_equal<_Ty1, _Ty2> = 0>
_NODISCARD constexpr bool operator!=(const _Ty1& _Left, const optional<_Ty2>& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(_Left != *_Right))) /* strengthened */ {
    if (_Right) {
        return _Left != *_Right;
    }
    return true;
}

_EXPORT_STD template <class _Ty1, class _Ty2, _Enable_if_comparable_with_less<_Ty1, _Ty2> = 0>
_NODISCARD constexpr bool operator<(const optional<_Ty1>& _Left, const _Ty2& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(*_Left < _Right))) /* strengthened */ {
    if (_Left) {
        return *_Left < _Right;
    }
    return true;
}
_EXPORT_STD template <class _Ty1, class _Ty2, _Enable_if_comparable_with_less<_Ty1, _Ty2> = 0>
_NODISCARD constexpr bool operator<(const _Ty1& _Left, const optional<_Ty2>& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(_Left < *_Right))) /* strengthened */ {
    if (_Right) {
        return _Left < *_Right;
    }
    return false;
}

_EXPORT_STD template <class _Ty1, class _Ty2, _Enable_if_comparable_with_greater<_Ty1, _Ty2> = 0>
_NODISCARD constexpr bool operator>(const optional<_Ty1>& _Left, const _Ty2& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(*_Left > _Right))) /* strengthened */ {
    if (_Left) {
        return *_Left > _Right;
    }
    return false;
}
_EXPORT_STD template <class _Ty1, class _Ty2, _Enable_if_comparable_with_greater<_Ty1, _Ty2> = 0>
_NODISCARD constexpr bool operator>(const _Ty1& _Left, const optional<_Ty2>& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(_Left > *_Right))) /* strengthened */ {
    if (_Right) {
        return _Left > *_Right;
    }
    return true;
}

_EXPORT_STD template <class _Ty1, class _Ty2, _Enable_if_comparable_with_less_equal<_Ty1, _Ty2> = 0>
_NODISCARD constexpr bool operator<=(const optional<_Ty1>& _Left, const _Ty2& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(*_Left <= _Right))) /* strengthened */ {
    if (_Left) {
        return *_Left <= _Right;
    }
    return true;
}
_EXPORT_STD template <class _Ty1, class _Ty2, _Enable_if_comparable_with_less_equal<_Ty1, _Ty2> = 0>
_NODISCARD constexpr bool operator<=(const _Ty1& _Left, const optional<_Ty2>& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(_Left <= *_Right))) /* strengthened */ {
    if (_Right) {
        return _Left <= *_Right;
    }
    return false;
}

_EXPORT_STD template <class _Ty1, class _Ty2, _Enable_if_comparable_with_greater_equal<_Ty1, _Ty2> = 0>
_NODISCARD constexpr bool operator>=(const optional<_Ty1>& _Left, const _Ty2& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(*_Left >= _Right))) /* strengthened */ {
    if (_Left) {
        return *_Left >= _Right;
    }
    return false;
}
_EXPORT_STD template <class _Ty1, class _Ty2, _Enable_if_comparable_with_greater_equal<_Ty1, _Ty2> = 0>
_NODISCARD constexpr bool operator>=(const _Ty1& _Left, const optional<_Ty2>& _Right)
    noexcept(noexcept(_STD _Fake_copy_init<bool>(_Left >= *_Right))) /* strengthened */ {
    if (_Right) {
        return _Left >= *_Right;
    }
    return true;
}

#if _HAS_CXX20
_EXPORT_STD template <class _Ty1, class _Ty2>
    requires (!_Derived_from_specialization_of<_Ty2, optional>) && three_way_comparable_with<_Ty1, _Ty2>
_NODISCARD constexpr compare_three_way_result_t<_Ty1, _Ty2> operator<=>(const optional<_Ty1>& _Left, const _Ty2& _Right)
    noexcept(noexcept(*_Left <=> _Right)) /* strengthened */ {
    if (_Left) {
        return *_Left <=> _Right;
    }

    return strong_ordering::less;
}
#endif // _HAS_CXX20

_EXPORT_STD template <class _Ty, enable_if_t<is_move_constructible_v<_Ty> && is_swappable_v<_Ty>, int> = 0>
_CONSTEXPR20 void swap(optional<_Ty>& _Left, optional<_Ty>& _Right) noexcept(noexcept(_Left.swap(_Right))) {
    _Left.swap(_Right);
}

_EXPORT_STD template <class _Ty, enable_if_t<is_constructible_v<decay_t<_Ty>, _Ty>, int> = 0> // LWG-3627
_NODISCARD constexpr optional<decay_t<_Ty>> make_optional(_Ty&& _Value)
    noexcept(noexcept(optional<decay_t<_Ty>>{_STD forward<_Ty>(_Value)})) /* strengthened */ {
    return optional<decay_t<_Ty>>{_STD forward<_Ty>(_Value)};
}
_EXPORT_STD template <class _Ty, class... _Types, enable_if_t<is_constructible_v<_Ty, _Types...>, int> = 0>
_NODISCARD constexpr optional<_Ty> make_optional(_Types&&... _Args)
    noexcept(noexcept(optional<_Ty>{in_place, _STD forward<_Types>(_Args)...})) /* strengthened */ {
    return optional<_Ty>{in_place, _STD forward<_Types>(_Args)...};
}
_EXPORT_STD template <class _Ty, class _Elem, class... _Types,
    enable_if_t<is_constructible_v<_Ty, initializer_list<_Elem>&, _Types...>, int> = 0>
_NODISCARD constexpr optional<_Ty> make_optional(initializer_list<_Elem> _Ilist, _Types&&... _Args)
    noexcept(noexcept(optional<_Ty>{in_place, _Ilist, _STD forward<_Types>(_Args)...})) /* strengthened */ {
    return optional<_Ty>{in_place, _Ilist, _STD forward<_Types>(_Args)...};
}

template <class _Ty>
struct hash<optional<_Ty>>
    : _Conditionally_enabled_hash<optional<_Ty>, is_default_constructible_v<hash<remove_const_t<_Ty>>>> {
    static size_t _Do_hash(const optional<_Ty>& _Opt) noexcept(_Is_nothrow_hashable<remove_const_t<_Ty>>::value) {
        constexpr size_t _Unspecified_value = 0;
        if (_Opt) {
            return hash<remove_const_t<_Ty>>{}(*_Opt);
        }

        return _Unspecified_value;
    }
};

_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // ^^^ _HAS_CXX17 ^^^
#endif // _STL_COMPILER_PREPROCESSOR
#endif // _OPTIONAL_
