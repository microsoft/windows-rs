// __msvc_iter_core.hpp internal header (core)

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef __MSVC_ITER_CORE_HPP
#define __MSVC_ITER_CORE_HPP
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR
#include <utility>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

_STD_BEGIN
template <class _Ty, class _Alloc, class = void>
struct _Has_allocator_type : false_type {}; // tests for suitable _Ty::allocator_type

template <class _Ty, class _Alloc>
struct _Has_allocator_type<_Ty, _Alloc, void_t<typename _Ty::allocator_type>>
    : is_convertible<_Alloc, typename _Ty::allocator_type>::type {};

_EXPORT_STD struct allocator_arg_t { // tag type for added allocator argument
    explicit allocator_arg_t() = default;
};

_EXPORT_STD _INLINE_VAR constexpr allocator_arg_t allocator_arg{};

_EXPORT_STD template <class _Ty, class _Alloc>
struct uses_allocator : _Has_allocator_type<_Ty, _Alloc>::type {};

_EXPORT_STD template <class _Ty, class _Alloc>
constexpr bool uses_allocator_v = uses_allocator<_Ty, _Alloc>::value;

// from <iterator>
_EXPORT_STD struct input_iterator_tag {};

_EXPORT_STD struct output_iterator_tag {};

_EXPORT_STD struct forward_iterator_tag : input_iterator_tag {};

_EXPORT_STD struct bidirectional_iterator_tag : forward_iterator_tag {};

_EXPORT_STD struct random_access_iterator_tag : bidirectional_iterator_tag {};

#if _HAS_CXX20
_EXPORT_STD struct contiguous_iterator_tag : random_access_iterator_tag {};

template <class _Ty>
using _With_reference = _Ty&;

template <class _Ty>
concept _Can_reference = requires { typename _With_reference<_Ty>; };

template <class _Ty>
concept _Dereferenceable = requires(_Ty& __t) {
    { *__t } -> _Can_reference;
};

template <class _Ty>
concept _Has_member_iterator_concept = requires { typename _Ty::iterator_concept; };

template <class _Ty>
concept _Has_member_iterator_category = requires { typename _Ty::iterator_category; };

template <class _Ty>
concept _Has_member_value_type = requires { typename _Ty::value_type; };

template <class _Ty>
concept _Has_member_element_type = requires { typename _Ty::element_type; };

template <class _Ty>
concept _Has_member_difference_type = requires { typename _Ty::difference_type; };

template <class _Ty>
concept _Has_member_pointer = requires { typename _Ty::pointer; };

template <class _Ty>
concept _Has_member_reference = requires { typename _Ty::reference; };

_EXPORT_STD template <class>
struct incrementable_traits {};

template <class _Ty>
    requires is_object_v<_Ty>
struct incrementable_traits<_Ty*> {
    using difference_type = ptrdiff_t;
};

template <class _Ty>
struct incrementable_traits<const _Ty> : incrementable_traits<_Ty> {};

template <_Has_member_difference_type _Ty>
struct incrementable_traits<_Ty> {
    using difference_type = _Ty::difference_type;
};

template <class _Ty>
concept _Can_difference = requires(const _Ty& __a, const _Ty& __b) {
    { __a - __b } -> integral;
};

template <class _Ty>
    requires (!_Has_member_difference_type<_Ty> && _Can_difference<_Ty>)
struct incrementable_traits<_Ty> {
    using difference_type = make_signed_t<decltype(_STD declval<_Ty>() - _STD declval<_Ty>())>;
};

template <class _Ty>
concept _Is_from_primary = _Same_impl<typename _Ty::_From_primary, _Ty>;

_EXPORT_STD template <class>
struct iterator_traits;

_EXPORT_STD template <class _Ty>
using iter_difference_t = conditional_t<_Is_from_primary<iterator_traits<remove_cvref_t<_Ty>>>,
    incrementable_traits<remove_cvref_t<_Ty>>, iterator_traits<remove_cvref_t<_Ty>>>::difference_type;

template <class>
struct _Cond_value_type {};

template <class _Ty>
    requires is_object_v<_Ty>
struct _Cond_value_type<_Ty> {
    using value_type = remove_cv_t<_Ty>;
};

_EXPORT_STD template <class>
struct indirectly_readable_traits {};

template <class _Ty>
struct indirectly_readable_traits<_Ty*> : _Cond_value_type<_Ty> {};

template <class _Ty>
    requires is_array_v<_Ty>
struct indirectly_readable_traits<_Ty> {
    using value_type = remove_cv_t<remove_extent_t<_Ty>>;
};

template <class _Ty>
struct indirectly_readable_traits<const _Ty> : indirectly_readable_traits<_Ty> {};

template <_Has_member_value_type _Ty>
struct indirectly_readable_traits<_Ty> : _Cond_value_type<typename _Ty::value_type> {};

template <_Has_member_element_type _Ty>
struct indirectly_readable_traits<_Ty> : _Cond_value_type<typename _Ty::element_type> {};

template <_Has_member_value_type _Ty>
    requires _Has_member_element_type<_Ty>
struct indirectly_readable_traits<_Ty> {};

template <_Has_member_value_type _Ty>
    requires _Has_member_element_type<_Ty>
          && same_as<remove_cv_t<typename _Ty::value_type>, remove_cv_t<typename _Ty::element_type>>
struct indirectly_readable_traits<_Ty> : _Cond_value_type<typename _Ty::value_type> {};

_EXPORT_STD template <class _Ty>
using iter_value_t = conditional_t<_Is_from_primary<iterator_traits<remove_cvref_t<_Ty>>>,
    indirectly_readable_traits<remove_cvref_t<_Ty>>, iterator_traits<remove_cvref_t<_Ty>>>::value_type;

_EXPORT_STD template <_Dereferenceable _Ty>
using iter_reference_t = decltype(*_STD declval<_Ty&>());

template <class>
struct _Iterator_traits_base {};

template <class _It>
concept _Has_iter_types = _Has_member_difference_type<_It> && _Has_member_value_type<_It> && _Has_member_reference<_It>
                       && _Has_member_iterator_category<_It>;

template <bool _Has_member_typedef>
struct _Old_iter_traits_pointer {
    template <class _It>
    using _Apply = _It::pointer;
};

template <>
struct _Old_iter_traits_pointer<false> {
    template <class>
    using _Apply = void;
};

template <_Has_iter_types _It>
struct _Iterator_traits_base<_It> {
    using iterator_category = _It::iterator_category;
    using value_type        = _It::value_type;
    using difference_type   = _It::difference_type;
    using pointer           = _Old_iter_traits_pointer<_Has_member_pointer<_It>>::template _Apply<_It>;
    using reference         = _It::reference;
};

template <bool _Has_member_typedef>
struct _Iter_traits_difference {
    template <class _It>
    using _Apply = incrementable_traits<_It>::difference_type;
};

template <>
struct _Iter_traits_difference<false> {
    template <class>
    using _Apply = void;
};

template <class _It>
concept _Cpp17_iterator = requires(_It __i) {
    { *__i } -> _Can_reference;
    { ++__i } -> same_as<_It&>;
    { *__i++ } -> _Can_reference;
} && copyable<_It>;

template <class _It>
concept _Cpp17_input_iterator =
    _Cpp17_iterator<_It> && equality_comparable<_It> && _Has_member_difference_type<incrementable_traits<_It>>
    && _Has_member_value_type<indirectly_readable_traits<_It>> && requires(_It __i) {
           typename common_reference_t<iter_reference_t<_It>&&, typename indirectly_readable_traits<_It>::value_type&>;
           typename common_reference_t<decltype(*__i++)&&, typename indirectly_readable_traits<_It>::value_type&>;
           requires signed_integral<typename incrementable_traits<_It>::difference_type>;
       };

template <class _It>
    requires (!_Has_iter_types<_It> && _Cpp17_iterator<_It> && !_Cpp17_input_iterator<_It>)
struct _Iterator_traits_base<_It> {
    using iterator_category = output_iterator_tag;
    using value_type        = void;
    using difference_type =
        _Iter_traits_difference<_Has_member_difference_type<incrementable_traits<_It>>>::template _Apply<_It>;
    using pointer   = void;
    using reference = void;
};

enum class _Itraits_pointer_strategy { _Use_void, _Use_member, _Use_decltype };

template <_Itraits_pointer_strategy>
struct _Iter_traits_pointer;

template <>
struct _Iter_traits_pointer<_Itraits_pointer_strategy::_Use_void> {
    template <class>
    using _Apply = void;
};

template <>
struct _Iter_traits_pointer<_Itraits_pointer_strategy::_Use_member> {
    template <class _It>
    using _Apply = _It::pointer;
};

template <>
struct _Iter_traits_pointer<_Itraits_pointer_strategy::_Use_decltype> {
    template <class _It>
    using _Apply = decltype(_STD declval<_It&>().operator->());
};

template <class _Ty>
concept _Has_member_arrow = requires(_Ty&& __t) { static_cast<_Ty&&>(__t).operator->(); };

template <bool _Has_member_typedef>
struct _Iter_traits_reference {
    template <class _It>
    using _Apply = _It::reference;
};

template <>
struct _Iter_traits_reference<false> {
    template <class _It>
    using _Apply = iter_reference_t<_It>;
};

template <bool _Is_random>
struct _Iter_traits_category4 {
    using type = random_access_iterator_tag;
};

template <>
struct _Iter_traits_category4<false> {
    using type = bidirectional_iterator_tag;
};

template <class _It>
concept _Cpp17_random_delta =
#if defined(__CUDACC__) && !defined(__clang__) // TRANSITION, fixed in CUDA 12.5
    totally_ordered<_It> && requires(_It __i, typename incrementable_traits<_It>::difference_type __n) {
#else // ^^^ workaround / no workaround vvv
    totally_ordered<_It> && requires(_It __i, incrementable_traits<_It>::difference_type __n) {
#endif // ^^^ no workaround ^^^
        { __i += __n } -> same_as<_It&>;
        { __i -= __n } -> same_as<_It&>;
        { __i + __n } -> same_as<_It>;
        { __n + __i } -> same_as<_It>;
        { __i - __n } -> same_as<_It>;
        { __i - __i } -> same_as<decltype(__n)>;
        { __i[__n] } -> convertible_to<iter_reference_t<_It>>;
    };

template <bool _Is_bidi>
struct _Iter_traits_category3 {
    template <class _It>
    using _Apply = _Iter_traits_category4<_Cpp17_random_delta<_It>>::type;
};

template <>
struct _Iter_traits_category3<false> {
    template <class>
    using _Apply = forward_iterator_tag;
};

template <class _It>
concept _Cpp17_bidi_delta = requires(_It __i) {
    { --__i } -> same_as<_It&>;
    { __i-- } -> convertible_to<const _It&>;
    requires same_as<decltype(*__i--), iter_reference_t<_It>>;
};

template <bool _Is_forward>
struct _Iter_traits_category2 {
    template <class _It>
    using _Apply = _Iter_traits_category3<_Cpp17_bidi_delta<_It>>::template _Apply<_It>;
};

template <>
struct _Iter_traits_category2<false> {
    template <class>
    using _Apply = input_iterator_tag;
};

template <class _It>
concept _Cpp17_forward_delta =
    constructible_from<_It> && is_reference_v<iter_reference_t<_It>>
    && same_as<remove_cvref_t<iter_reference_t<_It>>, typename indirectly_readable_traits<_It>::value_type>
    && requires(_It __i) {
           { __i++ } -> convertible_to<const _It&>;
           requires same_as<decltype(*__i++), iter_reference_t<_It>>;
       };

template <bool _Has_member_typedef>
struct _Iter_traits_category {
    template <class _It>
    using _Apply = _It::iterator_category;
};

template <>
struct _Iter_traits_category<false> {
    template <class _It>
    using _Apply = _Iter_traits_category2<_Cpp17_forward_delta<_It>>::template _Apply<_It>;
};

template <class _It>
    requires (!_Has_iter_types<_It> && _Cpp17_input_iterator<_It>)
struct _Iterator_traits_base<_It> {
    using iterator_category = _Iter_traits_category<_Has_member_iterator_category<_It>>::template _Apply<_It>;
    using value_type        = indirectly_readable_traits<_It>::value_type;
    using difference_type   = incrementable_traits<_It>::difference_type;
    using pointer =
        _Iter_traits_pointer<(_Has_member_pointer<_It>  ? _Itraits_pointer_strategy::_Use_member
                              : _Has_member_arrow<_It&> ? _Itraits_pointer_strategy::_Use_decltype
                                                        : _Itraits_pointer_strategy::_Use_void)>::template _Apply<_It>;
    using reference = _Iter_traits_reference<_Has_member_reference<_It>>::template _Apply<_It>;
};

_EXPORT_STD template <class _Ty>
struct iterator_traits : _Iterator_traits_base<_Ty> {
    using _From_primary = iterator_traits;
};

template <class _Ty>
    requires is_object_v<_Ty>
struct iterator_traits<_Ty*> {
    using iterator_concept  = contiguous_iterator_tag;
    using iterator_category = random_access_iterator_tag;
    using value_type        = remove_cv_t<_Ty>;
    using difference_type   = ptrdiff_t;
    using pointer           = _Ty*;
    using reference         = _Ty&;
};

template <class _Ty>
constexpr bool _Integer_class = requires {
    typename _Ty::_Signed_type;
    typename _Ty::_Unsigned_type;
};

template <class _Ty>
concept _Integer_like = _Is_nonbool_integral<_Ty> || _Integer_class<_Ty>;

template <class _Ty>
concept _Signed_integer_like = _Integer_like<_Ty> && static_cast<_Ty>(-1) < static_cast<_Ty>(0);

_EXPORT_STD template <class _Ty>
concept weakly_incrementable = movable<_Ty> && requires(_Ty __i) {
    typename iter_difference_t<_Ty>;
    requires _Signed_integer_like<iter_difference_t<_Ty>>;
    { ++__i } -> same_as<_Ty&>;
    __i++;
};

_EXPORT_STD template <class _It>
concept input_or_output_iterator = requires(_It __i) {
    { *__i } -> _Can_reference;
} && weakly_incrementable<_It>;

_EXPORT_STD template <class _Se, class _It>
concept sentinel_for = semiregular<_Se> && input_or_output_iterator<_It> && _Weakly_equality_comparable_with<_Se, _It>;

_EXPORT_STD template <class _Se, class _It>
constexpr bool disable_sized_sentinel_for = false;

_EXPORT_STD template <class _Se, class _It>
concept sized_sentinel_for = sentinel_for<_Se, _It> && !disable_sized_sentinel_for<remove_cv_t<_Se>, remove_cv_t<_It>>
                          && requires(const _It& __i, const _Se& __s) {
                                 { __s - __i } -> same_as<iter_difference_t<_It>>;
                                 { __i - __s } -> same_as<iter_difference_t<_It>>;
                             };

_EXPORT_STD struct default_sentinel_t {};

_EXPORT_STD inline constexpr default_sentinel_t default_sentinel{};

namespace ranges {
    _EXPORT_STD enum class subrange_kind : bool { unsized, sized };

    _EXPORT_STD template <input_or_output_iterator _It, sentinel_for<_It> _Se = _It,
        subrange_kind _Ki = sized_sentinel_for<_Se, _It> ? subrange_kind::sized : subrange_kind::unsized>
        requires (_Ki == subrange_kind::sized || !sized_sentinel_for<_Se, _It>)
    class subrange;

    _EXPORT_STD template <size_t _Idx, class _It, class _Se, subrange_kind _Ki>
        requires ((_Idx == 0 && copyable<_It>) || _Idx == 1)
    _NODISCARD constexpr auto get(const subrange<_It, _Se, _Ki>& _Val);

    _EXPORT_STD template <size_t _Idx, class _It, class _Se, subrange_kind _Ki>
        requires (_Idx < 2)
    _NODISCARD constexpr auto get(subrange<_It, _Se, _Ki>&& _Val);
} // namespace ranges

_EXPORT_STD using ranges::get;

template <class _It, class _Se, ranges::subrange_kind _Ki>
constexpr bool _Is_subrange_v<ranges::subrange<_It, _Se, _Ki>> = true;

template <class _It, class _Se, ranges::subrange_kind _Ki>
struct tuple_size<ranges::subrange<_It, _Se, _Ki>> : integral_constant<size_t, 2> {};

template <class _It, class _Se, ranges::subrange_kind _Ki>
struct tuple_element<0, ranges::subrange<_It, _Se, _Ki>> {
    using type = _It;
};

template <class _It, class _Se, ranges::subrange_kind _Ki>
struct tuple_element<1, ranges::subrange<_It, _Se, _Ki>> {
    using type = _Se;
};

template <class _It, class _Se, ranges::subrange_kind _Ki>
struct tuple_element<0, const ranges::subrange<_It, _Se, _Ki>> {
    using type = _It;
};

template <class _It, class _Se, ranges::subrange_kind _Ki>
struct tuple_element<1, const ranges::subrange<_It, _Se, _Ki>> {
    using type = _Se;
};
#else // ^^^ _HAS_CXX20 / !_HAS_CXX20 vvv
template <class, class = void>
struct _Iterator_traits_base {}; // empty for non-iterators

template <class _Iter>
struct _Iterator_traits_base<_Iter,
    void_t<typename _Iter::iterator_category, typename _Iter::value_type, typename _Iter::difference_type,
        typename _Iter::pointer, typename _Iter::reference>> {
    // defined if _Iter::* types exist
    using iterator_category = typename _Iter::iterator_category;
    using value_type        = typename _Iter::value_type;
    using difference_type   = typename _Iter::difference_type;
    using pointer           = typename _Iter::pointer;
    using reference         = typename _Iter::reference;
};

template <class _Ty, bool = is_object_v<_Ty>>
struct _Iterator_traits_pointer_base { // iterator properties for pointers to object
    using iterator_category = random_access_iterator_tag;
    using value_type        = remove_cv_t<_Ty>;
    using difference_type   = ptrdiff_t;
    using pointer           = _Ty*;
    using reference         = _Ty&;
};

template <class _Ty>
struct _Iterator_traits_pointer_base<_Ty, false> {}; // iterator non-properties for pointers to non-object

template <class _Iter>
struct iterator_traits : _Iterator_traits_base<_Iter> {}; // get traits from iterator _Iter, if possible

template <class _Ty>
struct iterator_traits<_Ty*> : _Iterator_traits_pointer_base<_Ty> {}; // get traits from pointer, if possible

template <class _Ty>
constexpr bool _Integer_like = _Is_nonbool_integral<_Ty>;
#endif // ^^^ !_HAS_CXX20 ^^^

_INLINE_VAR constexpr auto _Meta_npos = ~size_t{0};

constexpr size_t _Meta_find_index_i_(const bool* const _Ptr, const size_t _Count, size_t _Idx = 0) {
    // return the index of the first true in the _Count bools at _Ptr, or _Meta_npos if all are false
    for (; _Idx < _Count; ++_Idx) {
        if (_Ptr[_Idx]) {
            return _Idx;
        }
    }

    return _Meta_npos;
}

template <class _List, class _Ty>
struct _Meta_find_unique_index_ {
    using type = integral_constant<size_t, _Meta_npos>;
};
template <class _List, class _Ty>
using _Meta_find_unique_index =
    // The index of _Ty in _List if it occurs exactly once, otherwise _Meta_npos
    typename _Meta_find_unique_index_<_List, _Ty>::type;

constexpr size_t _Meta_find_unique_index_i_2(const bool* const _Ptr, const size_t _Count, const size_t _First) {
    // return _First if there is no _First < j < _Count such that _Ptr[j] is true, otherwise _Meta_npos
    return _First != _Meta_npos && _STD _Meta_find_index_i_(_Ptr, _Count, _First + 1) == _Meta_npos ? _First
                                                                                                    : _Meta_npos;
}

constexpr size_t _Meta_find_unique_index_i_(const bool* const _Ptr, const size_t _Count) {
    // Pass the smallest i such that _Ptr[i] is true to _Meta_find_unique_index_i_2
    return _STD _Meta_find_unique_index_i_2(_Ptr, _Count, _STD _Meta_find_index_i_(_Ptr, _Count));
}

template <template <class...> class _List, class _First, class... _Rest, class _Ty>
struct _Meta_find_unique_index_<_List<_First, _Rest...>, _Ty> {
    static constexpr bool _Bools[] = {is_same_v<_First, _Ty>, is_same_v<_Rest, _Ty>...};
    using type = integral_constant<size_t, _STD _Meta_find_unique_index_i_(_Bools, 1 + sizeof...(_Rest))>;
};
_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)
#endif // _STL_COMPILER_PREPROCESSOR
#endif // __MSVC_ITER_CORE_HPP
