// __msvc_string_view.hpp internal header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// (<string_view> without emitting non-C++17 warnings)

#ifndef __MSVC_STRING_VIEW_HPP
#define __MSVC_STRING_VIEW_HPP
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR
#include <iosfwd>
#include <xutility>

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

#if _USE_STD_VECTOR_ALGORITHMS
extern "C" {
// The "noalias" attribute tells the compiler optimizer that pointers going into these hand-vectorized algorithms
// won't be stored beyond the lifetime of the function, and that the function will only reference arrays denoted by
// those pointers. The optimizer also assumes in that case that a pointer parameter is not returned to the caller via
// the return value, so functions using "noalias" must usually return void. This attribute is valuable because these
// functions are in native code objects that the compiler cannot analyze. In the absence of the noalias attribute, the
// compiler has to assume that the denoted arrays are "globally address taken", and that any later calls to
// unanalyzable routines may modify those arrays.

__declspec(noalias) size_t __stdcall __std_find_first_of_trivial_pos_1(
    const void* _Haystack, size_t _Haystack_length, const void* _Needle, size_t _Needle_length) noexcept;
__declspec(noalias) size_t __stdcall __std_find_first_of_trivial_pos_2(
    const void* _Haystack, size_t _Haystack_length, const void* _Needle, size_t _Needle_length) noexcept;
__declspec(noalias) size_t __stdcall __std_find_first_of_trivial_pos_4(
    const void* _Haystack, size_t _Haystack_length, const void* _Needle, size_t _Needle_length) noexcept;
__declspec(noalias) size_t __stdcall __std_find_first_of_trivial_pos_8(
    const void* _Haystack, size_t _Haystack_length, const void* _Needle, size_t _Needle_length) noexcept;

__declspec(noalias) size_t __stdcall __std_find_last_of_trivial_pos_1(
    const void* _Haystack, size_t _Haystack_length, const void* _Needle, size_t _Needle_length) noexcept;
__declspec(noalias) size_t __stdcall __std_find_last_of_trivial_pos_2(
    const void* _Haystack, size_t _Haystack_length, const void* _Needle, size_t _Needle_length) noexcept;

} // extern "C"

_STD_BEGIN

template <class _Ty1, class _Ty2>
size_t _Find_first_of_pos_vectorized(const _Ty1* const _Haystack, const size_t _Haystack_length,
    const _Ty2* const _Needle, const size_t _Needle_length) noexcept {
    _STL_INTERNAL_STATIC_ASSERT(sizeof(_Ty1) == sizeof(_Ty2));
    if constexpr (sizeof(_Ty1) == 1) {
        return ::__std_find_first_of_trivial_pos_1(_Haystack, _Haystack_length, _Needle, _Needle_length);
    } else if constexpr (sizeof(_Ty1) == 2) {
        return ::__std_find_first_of_trivial_pos_2(_Haystack, _Haystack_length, _Needle, _Needle_length);
    } else if constexpr (sizeof(_Ty1) == 4) {
        return ::__std_find_first_of_trivial_pos_4(_Haystack, _Haystack_length, _Needle, _Needle_length);
    } else if constexpr (sizeof(_Ty1) == 8) {
        return ::__std_find_first_of_trivial_pos_8(_Haystack, _Haystack_length, _Needle, _Needle_length);
    } else {
        _STL_INTERNAL_STATIC_ASSERT(false); // unexpected size
    }
}

template <class _Ty1, class _Ty2>
size_t _Find_last_of_pos_vectorized(const _Ty1* const _Haystack, const size_t _Haystack_length,
    const _Ty2* const _Needle, const size_t _Needle_length) noexcept {
    _STL_INTERNAL_STATIC_ASSERT(sizeof(_Ty1) == sizeof(_Ty2));
    if constexpr (sizeof(_Ty1) == 1) {
        return ::__std_find_last_of_trivial_pos_1(_Haystack, _Haystack_length, _Needle, _Needle_length);
    } else if constexpr (sizeof(_Ty1) == 2) {
        return ::__std_find_last_of_trivial_pos_2(_Haystack, _Haystack_length, _Needle, _Needle_length);
    } else {
        _STL_INTERNAL_STATIC_ASSERT(false); // unexpected size
    }
}

_STD_END

#endif // _USE_STD_VECTOR_ALGORITHMS

_STD_BEGIN
#ifdef __clang__
#define _HAS_MEMCPY_MEMMOVE_INTRINSICS 1
#else // ^^^ use __builtin_memcpy and __builtin_memmove / use workaround vvv
#define _HAS_MEMCPY_MEMMOVE_INTRINSICS 0 // TRANSITION, DevCom-1046483 (MSVC) and VSO-1129974 (EDG)
#endif // ^^^ use workaround ^^^

template <class _Elem, class _Int_type>
struct _Char_traits { // properties of a string or stream element
    using char_type  = _Elem;
    using int_type   = _Int_type;
    using pos_type   = streampos;
    using off_type   = streamoff;
    using state_type = _Mbstatet;
#if _HAS_CXX20
    using comparison_category = strong_ordering;
#endif // _HAS_CXX20

    // For copy/move, we can uniformly call memcpy/memmove (or their builtin versions) for all element types.

    static _CONSTEXPR20 _Elem* copy(_Out_writes_all_(_Count) _Elem* const _First1,
        _In_reads_(_Count) const _Elem* const _First2, const size_t _Count) noexcept /* strengthened */ {
        // copy [_First2, _First2 + _Count) to [_First1, ...)
#if _HAS_MEMCPY_MEMMOVE_INTRINSICS
        __builtin_memcpy(_First1, _First2, _Count * sizeof(_Elem));
#else // ^^^ _HAS_MEMCPY_MEMMOVE_INTRINSICS / !_HAS_MEMCPY_MEMMOVE_INTRINSICS vvv
#if _HAS_CXX20
        if (_STD is_constant_evaluated()) {
            // pre: [_First1, _First1 + _Count) and [_First2, _First2 + _Count) do not overlap
            for (size_t _Idx = 0; _Idx != _Count; ++_Idx) {
                _First1[_Idx] = _First2[_Idx];
            }

            return _First1;
        }
#endif // _HAS_CXX20

        _CSTD memcpy(_First1, _First2, _Count * sizeof(_Elem));
#endif // ^^^ !_HAS_MEMCPY_MEMMOVE_INTRINSICS ^^^

        return _First1;
    }

    _Pre_satisfies_(_Dest_size >= _Count) static _CONSTEXPR20 _Elem* _Copy_s(_Out_writes_all_(_Dest_size)
                                                                                 _Elem* const _First1,
        const size_t _Dest_size, _In_reads_(_Count) const _Elem* const _First2, const size_t _Count) noexcept {
        // copy [_First2, _First2 + _Count) to [_First1, _First1 + _Dest_size)
        _STL_VERIFY(_Count <= _Dest_size, "invalid argument");
        return copy(_First1, _First2, _Count);
    }

    static _CONSTEXPR20 _Elem* move(_Out_writes_all_(_Count) _Elem* const _First1,
        _In_reads_(_Count) const _Elem* const _First2, const size_t _Count) noexcept /* strengthened */ {
        // copy [_First2, _First2 + _Count) to [_First1, ...), allowing overlap
#if _HAS_MEMCPY_MEMMOVE_INTRINSICS
        __builtin_memmove(_First1, _First2, _Count * sizeof(_Elem));
#else // ^^^ _HAS_MEMCPY_MEMMOVE_INTRINSICS / !_HAS_MEMCPY_MEMMOVE_INTRINSICS vvv
#if _HAS_CXX20
        if (_STD is_constant_evaluated()) {
            // dest: [_First1, _First1 + _Count)
            // src: [_First2, _First2 + _Count)
            // We need to handle overlapping ranges.
            // If _First1 is in the src range, we need a backward loop.
            // Otherwise, the forward loop works (even if the back of dest overlaps the front of src).

            // Usually, we would compare pointers with less-than, even though they could belong to different arrays.
            // However, we're not allowed to do that during constant evaluation, so we need a linear scan for equality.
            bool _Loop_forward = true;

            for (const _Elem* _Src = _First2; _Src != _First2 + _Count; ++_Src) {
                if (_First1 == _Src) {
                    _Loop_forward = false;
                    break;
                }
            }

            if (_Loop_forward) {
                for (size_t _Idx = 0; _Idx != _Count; ++_Idx) {
                    _First1[_Idx] = _First2[_Idx];
                }
            } else {
                for (size_t _Idx = _Count; _Idx != 0; --_Idx) {
                    _First1[_Idx - 1] = _First2[_Idx - 1];
                }
            }

            return _First1;
        }
#endif // _HAS_CXX20

        _CSTD memmove(_First1, _First2, _Count * sizeof(_Elem));
#endif // ^^^ !_HAS_MEMCPY_MEMMOVE_INTRINSICS ^^^

        return _First1;
    }

    // For compare/length/find/assign, we can't uniformly call CRT functions (or their builtin versions).
    // 8-bit: memcmp/strlen/memchr/memset; 16-bit: wmemcmp/wcslen/wmemchr/wmemset; 32-bit: nonexistent

    _NODISCARD static _CONSTEXPR17 int compare(_In_reads_(_Count) const _Elem* _First1,
        _In_reads_(_Count) const _Elem* _First2, size_t _Count) noexcept /* strengthened */ {
        // compare [_First1, _First1 + _Count) with [_First2, ...)
        for (; 0 < _Count; --_Count, ++_First1, ++_First2) {
            if (*_First1 != *_First2) {
                return *_First1 < *_First2 ? -1 : +1;
            }
        }

        return 0;
    }

    _NODISCARD static _CONSTEXPR17 size_t length(_In_z_ const _Elem* _First) noexcept /* strengthened */ {
        // find length of null-terminated sequence
        size_t _Count = 0;
        while (*_First != _Elem()) {
            ++_Count;
            ++_First;
        }

        return _Count;
    }

    _NODISCARD static _CONSTEXPR17 const _Elem* find(
        _In_reads_(_Count) const _Elem* _First, size_t _Count, const _Elem& _Ch) noexcept /* strengthened */ {
        // look for _Ch in [_First, _First + _Count)
        for (; 0 < _Count; --_Count, ++_First) {
            if (*_First == _Ch) {
                return _First;
            }
        }

        return nullptr;
    }

    static _CONSTEXPR20 _Elem* assign(
        _Out_writes_all_(_Count) _Elem* const _First, size_t _Count, const _Elem _Ch) noexcept /* strengthened */ {
        // assign _Count * _Ch to [_First, ...)
        for (_Elem* _Next = _First; _Count > 0; --_Count, ++_Next) {
            *_Next = _Ch;
        }

        return _First;
    }

    static _CONSTEXPR17 void assign(_Elem& _Left, const _Elem& _Right) noexcept {
        _Left = _Right;
    }

    _NODISCARD static constexpr bool eq(const _Elem _Left, const _Elem _Right) noexcept {
        return _Left == _Right;
    }

    _NODISCARD static constexpr bool lt(const _Elem _Left, const _Elem _Right) noexcept {
        return _Left < _Right;
    }

    _NODISCARD static constexpr _Elem to_char_type(const int_type _Meta) noexcept {
        return static_cast<_Elem>(_Meta);
    }

    _NODISCARD static constexpr int_type to_int_type(const _Elem _Ch) noexcept {
        return static_cast<int_type>(_Ch);
    }

    _NODISCARD static constexpr bool eq_int_type(const int_type _Left, const int_type _Right) noexcept {
        return _Left == _Right;
    }

    _NODISCARD static constexpr int_type not_eof(const int_type _Meta) noexcept {
        return _Meta != eof() ? _Meta : !eof();
    }

    _NODISCARD static constexpr int_type eof() noexcept {
        return static_cast<int_type>(EOF);
    }
};

template <class _Elem>
struct _WChar_traits : private _Char_traits<_Elem, unsigned short> {
    // char_traits for the char16_t-likes: char16_t, wchar_t, unsigned short
private:
    using _Primary_char_traits = _Char_traits<_Elem, unsigned short>;

public:
    using char_type  = _Elem;
    using int_type   = unsigned short;
    using pos_type   = streampos;
    using off_type   = streamoff;
    using state_type = mbstate_t;
#if _HAS_CXX20
    using comparison_category = strong_ordering;
#endif // _HAS_CXX20

    using _Primary_char_traits::_Copy_s;
    using _Primary_char_traits::copy;
    using _Primary_char_traits::move;

    _NODISCARD static _CONSTEXPR17 int compare(_In_reads_(_Count) const _Elem* const _First1,
        _In_reads_(_Count) const _Elem* const _First2, const size_t _Count) noexcept /* strengthened */ {
        // compare [_First1, _First1 + _Count) with [_First2, ...)
#if _HAS_CXX17
        if (_STD _Is_constant_evaluated()) {
            if constexpr (is_same_v<_Elem, wchar_t>) {
                return __builtin_wmemcmp(_First1, _First2, _Count);
            } else {
                return _Primary_char_traits::compare(_First1, _First2, _Count);
            }
        }
#endif // _HAS_CXX17

        return _CSTD wmemcmp(
            reinterpret_cast<const wchar_t*>(_First1), reinterpret_cast<const wchar_t*>(_First2), _Count);
    }

    _NODISCARD static _CONSTEXPR17 size_t length(_In_z_ const _Elem* _First) noexcept /* strengthened */ {
        // find length of null-terminated sequence
#if _HAS_CXX17
        if (_STD _Is_constant_evaluated()) {
            if constexpr (is_same_v<_Elem, wchar_t>) {
                return __builtin_wcslen(_First);
            } else {
                return _Primary_char_traits::length(_First);
            }
        }
#endif // _HAS_CXX17

        return _CSTD wcslen(reinterpret_cast<const wchar_t*>(_First));
    }

    _NODISCARD static _CONSTEXPR17 const _Elem* find(
        _In_reads_(_Count) const _Elem* _First, const size_t _Count, const _Elem& _Ch) noexcept /* strengthened */ {
        // look for _Ch in [_First, _First + _Count)
#if _HAS_CXX17
        if (_STD _Is_constant_evaluated()) {
            if constexpr (is_same_v<_Elem, wchar_t>) {
                return __builtin_wmemchr(_First, _Ch, _Count);
            } else {
                return _Primary_char_traits::find(_First, _Count, _Ch);
            }
        }
#endif // _HAS_CXX17

        return reinterpret_cast<const _Elem*>(_CSTD wmemchr(reinterpret_cast<const wchar_t*>(_First), _Ch, _Count));
    }

    static _CONSTEXPR20 _Elem* assign(
        _Out_writes_all_(_Count) _Elem* const _First, size_t _Count, const _Elem _Ch) noexcept /* strengthened */ {
        // assign _Count * _Ch to [_First, ...)
#if _HAS_CXX20
        if (_STD is_constant_evaluated()) {
            return _Primary_char_traits::assign(_First, _Count, _Ch);
        }
#endif // _HAS_CXX20

        return reinterpret_cast<_Elem*>(_CSTD wmemset(reinterpret_cast<wchar_t*>(_First), _Ch, _Count));
    }

    static _CONSTEXPR17 void assign(_Elem& _Left, const _Elem& _Right) noexcept {
#if _HAS_CXX20
        if (_STD is_constant_evaluated()) {
            return _Primary_char_traits::assign(_Left, _Right);
        }
#endif // _HAS_CXX20
        _Left = _Right;
    }

    _NODISCARD static constexpr bool eq(const _Elem _Left, const _Elem _Right) noexcept {
        return _Left == _Right;
    }

    _NODISCARD static constexpr bool lt(const _Elem _Left, const _Elem _Right) noexcept {
        return _Left < _Right;
    }

    _NODISCARD static constexpr _Elem to_char_type(const int_type _Meta) noexcept {
        return _Meta;
    }

    _NODISCARD static constexpr int_type to_int_type(const _Elem _Ch) noexcept {
        return _Ch;
    }

    _NODISCARD static constexpr bool eq_int_type(const int_type _Left, const int_type _Right) noexcept {
        return _Left == _Right;
    }

    _NODISCARD static constexpr int_type not_eof(const int_type _Meta) noexcept {
        return _Meta != eof() ? _Meta : static_cast<int_type>(!eof());
    }

    _NODISCARD static constexpr int_type eof() noexcept {
        return WEOF;
    }
};

_EXPORT_STD template <class _Elem>
struct char_traits : _Char_traits<_Elem, long> {}; // properties of a string or stream unknown element

template <>
struct char_traits<char16_t> : _WChar_traits<char16_t> {};

template <>
struct char_traits<char32_t> : _Char_traits<char32_t, unsigned int> {};

template <>
struct char_traits<wchar_t> : _WChar_traits<wchar_t> {};

#ifdef _CRTBLD
template <>
struct char_traits<unsigned short> : _WChar_traits<unsigned short> {};
#endif // defined(_CRTBLD)

// signed char and other unsigned integral types are supported as an extension.
// Use of other arithmetic types and nullptr_t should be rejected.
template <class _Ty>
constexpr bool _Is_implementation_handled_char_like_type = is_arithmetic_v<_Ty> || is_null_pointer_v<_Ty>;

template <class>
constexpr bool _Is_implementation_handled_char_traits = false;
template <class _Elem>
constexpr bool _Is_implementation_handled_char_traits<char_traits<_Elem>> =
    _Is_implementation_handled_char_like_type<_Elem>;

#if defined(__cpp_char8_t) && !defined(__clang__) && !defined(__EDG__)
#define _HAS_U8_INTRINSICS 1
#else // ^^^ Use intrinsics for char8_t / don't use said intrinsics vvv
#define _HAS_U8_INTRINSICS 0
#endif // Detect u8 intrinsics

template <class _Elem, class _Int_type>
struct _Narrow_char_traits : private _Char_traits<_Elem, _Int_type> {
    // Implement char_traits for narrow character types char and char8_t
private:
    using _Primary_char_traits = _Char_traits<_Elem, _Int_type>;

public:
    using char_type  = _Elem;
    using int_type   = _Int_type;
    using pos_type   = streampos;
    using off_type   = streamoff;
    using state_type = mbstate_t;
#if _HAS_CXX20
    using comparison_category = strong_ordering;
#endif // _HAS_CXX20

    using _Primary_char_traits::_Copy_s;
    using _Primary_char_traits::copy;
    using _Primary_char_traits::move;

    _NODISCARD static _CONSTEXPR17 int compare(_In_reads_(_Count) const _Elem* const _First1,
        _In_reads_(_Count) const _Elem* const _First2, const size_t _Count) noexcept /* strengthened */ {
        // compare [_First1, _First1 + _Count) with [_First2, ...)
#if _HAS_CXX17
        return __builtin_memcmp(_First1, _First2, _Count);
#else // ^^^ _HAS_CXX17 / !_HAS_CXX17 vvv
        return _CSTD memcmp(_First1, _First2, _Count);
#endif // ^^^ !_HAS_CXX17 ^^^
    }

    _NODISCARD static _CONSTEXPR17 size_t length(_In_z_ const _Elem* const _First) noexcept /* strengthened */ {
        // find length of null-terminated string
#if _HAS_CXX17
#ifdef __cpp_char8_t
        if constexpr (is_same_v<_Elem, char8_t>) {
#if _HAS_U8_INTRINSICS
            return __builtin_u8strlen(_First);
#else // ^^^ use u8 intrinsics / no u8 intrinsics vvv
            return _Primary_char_traits::length(_First);
#endif // ^^^ no u8 intrinsics ^^^
        } else
#endif // defined(__cpp_char8_t)
        {
            return __builtin_strlen(_First);
        }
#else // ^^^ _HAS_CXX17 / !_HAS_CXX17 vvv
        return _CSTD strlen(reinterpret_cast<const char*>(_First));
#endif // ^^^ !_HAS_CXX17 ^^^
    }

    _NODISCARD static _CONSTEXPR17 const _Elem* find(_In_reads_(_Count) const _Elem* const _First, const size_t _Count,
        const _Elem& _Ch) noexcept /* strengthened */ {
        // look for _Ch in [_First, _First + _Count)
#if _HAS_CXX17
#ifdef __cpp_char8_t
        if constexpr (is_same_v<_Elem, char8_t>) {
#if _HAS_U8_INTRINSICS
            return __builtin_u8memchr(_First, _Ch, _Count);
#else // ^^^ use u8 intrinsics / no u8 intrinsics vvv
            return _Primary_char_traits::find(_First, _Count, _Ch);
#endif // ^^^ no u8 intrinsics ^^^
        } else
#endif // defined(__cpp_char8_t)
        {
            return __builtin_char_memchr(_First, _Ch, _Count);
        }
#else // ^^^ _HAS_CXX17 / !_HAS_CXX17 vvv
        return static_cast<const _Elem*>(_CSTD memchr(_First, _Ch, _Count));
#endif // ^^^ !_HAS_CXX17 ^^^
    }

    static _CONSTEXPR20 _Elem* assign(
        _Out_writes_all_(_Count) _Elem* const _First, size_t _Count, const _Elem _Ch) noexcept /* strengthened */ {
        // assign _Count * _Ch to [_First, ...)
#if _HAS_CXX20
        if (_STD is_constant_evaluated()) {
            return _Primary_char_traits::assign(_First, _Count, _Ch);
        }
#endif // _HAS_CXX20

        return static_cast<_Elem*>(_CSTD memset(_First, _Ch, _Count));
    }

    static _CONSTEXPR17 void assign(_Elem& _Left, const _Elem& _Right) noexcept {
#if _HAS_CXX20
        if (_STD is_constant_evaluated()) {
            return _Primary_char_traits::assign(_Left, _Right);
        }
#endif // _HAS_CXX20
        _Left = _Right;
    }

    _NODISCARD static constexpr bool eq(const _Elem _Left, const _Elem _Right) noexcept {
        return _Left == _Right;
    }

    _NODISCARD static constexpr bool lt(const _Elem _Left, const _Elem _Right) noexcept {
        return static_cast<unsigned char>(_Left) < static_cast<unsigned char>(_Right);
    }

    _NODISCARD static constexpr _Elem to_char_type(const int_type _Meta) noexcept {
        return static_cast<_Elem>(_Meta);
    }

    _NODISCARD static constexpr int_type to_int_type(const _Elem _Ch) noexcept {
        return static_cast<unsigned char>(_Ch);
    }

    _NODISCARD static constexpr bool eq_int_type(const int_type _Left, const int_type _Right) noexcept {
        return _Left == _Right;
    }

    _NODISCARD static constexpr int_type not_eof(const int_type _Meta) noexcept {
        return _Meta != eof() ? _Meta : !eof();
    }

    _NODISCARD static constexpr int_type eof() noexcept {
        return static_cast<int_type>(EOF);
    }
};

#undef _HAS_U8_INTRINSICS
#undef _HAS_MEMCPY_MEMMOVE_INTRINSICS

template <>
struct char_traits<char> : _Narrow_char_traits<char, int> {}; // properties of a string or stream char element

#ifdef __cpp_char8_t
template <>
struct char_traits<char8_t> : _Narrow_char_traits<char8_t, unsigned int> {};
#endif // defined(__cpp_char8_t)

template <class _Elem, class _Traits, class _SizeT>
basic_ostream<_Elem, _Traits>& _Insert_string(
    basic_ostream<_Elem, _Traits>& _Ostr, const _Elem* const _Data, const _SizeT _Size) {
    // insert a character-type sequence into _Ostr as if through a basic_string copy
    using _Ostr_t                    = basic_ostream<_Elem, _Traits>;
    typename _Ostr_t::iostate _State = _Ostr_t::goodbit;

    _SizeT _Pad;
    if (_Ostr.width() <= 0 || static_cast<_SizeT>(_Ostr.width()) <= _Size) {
        _Pad = 0;
    } else {
        _Pad = static_cast<_SizeT>(_Ostr.width()) - _Size;
    }

    const typename _Ostr_t::sentry _Ok(_Ostr);

    if (!_Ok) {
        _State |= _Ostr_t::badbit;
    } else { // state okay, insert characters
        _TRY_IO_BEGIN
        if ((_Ostr.flags() & _Ostr_t::adjustfield) != _Ostr_t::left) {
            for (; 0 < _Pad; --_Pad) { // pad on left
                if (_Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ostr.fill()))) {
                    _State |= _Ostr_t::badbit; // insertion failed, quit
                    break;
                }
            }
        }

        if (_State == _Ostr_t::goodbit
            && _Ostr.rdbuf()->sputn(_Data, static_cast<streamsize>(_Size)) != static_cast<streamsize>(_Size)) {
            _State |= _Ostr_t::badbit;
        } else {
            for (; 0 < _Pad; --_Pad) { // pad on right
                if (_Traits::eq_int_type(_Traits::eof(), _Ostr.rdbuf()->sputc(_Ostr.fill()))) {
                    _State |= _Ostr_t::badbit; // insertion failed, quit
                    break;
                }
            }
        }

        _Ostr.width(0);
        _CATCH_IO_(_Ostr_t, _Ostr)
    }

    _Ostr.setstate(_State);
    return _Ostr;
}

template <class _Traits>
using _Traits_ch_t = typename _Traits::char_type;

template <class _Traits>
using _Traits_ptr_t = const typename _Traits::char_type*;

template <class _Traits>
constexpr bool _Traits_equal(_In_reads_(_Left_size) const _Traits_ptr_t<_Traits> _Left, const size_t _Left_size,
    _In_reads_(_Right_size) const _Traits_ptr_t<_Traits> _Right, const size_t _Right_size) noexcept {
    // compare [_Left, _Left + _Left_size) to [_Right, _Right + _Right_size) for equality using _Traits
    if (_Left_size != _Right_size) {
        return false;
    }

    if (_Left_size == 0u) {
        return true;
    }

    return _Traits::compare(_Left, _Right, _Left_size) == 0;
}

template <class _Traits>
constexpr int _Traits_compare(_In_reads_(_Left_size) const _Traits_ptr_t<_Traits> _Left, const size_t _Left_size,
    _In_reads_(_Right_size) const _Traits_ptr_t<_Traits> _Right, const size_t _Right_size) noexcept {
    // compare [_Left, _Left + _Left_size) to [_Right, _Right + _Right_size) using _Traits
    const int _Ans = _Traits::compare(_Left, _Right, (_STD min)(_Left_size, _Right_size));

    if (_Ans != 0) {
        return _Ans;
    }

    if (_Left_size < _Right_size) {
        return -1;
    }

    if (_Left_size > _Right_size) {
        return 1;
    }

    return 0;
}

template <class _Traits>
constexpr size_t _Traits_find(_In_reads_(_Hay_size) const _Traits_ptr_t<_Traits> _Haystack, const size_t _Hay_size,
    const size_t _Start_at, _In_reads_(_Needle_size) const _Traits_ptr_t<_Traits> _Needle,
    const size_t _Needle_size) noexcept {
    // search [_Haystack, _Haystack + _Hay_size) for [_Needle, _Needle + _Needle_size), at/after _Start_at
    if (_Needle_size > _Hay_size || _Start_at > _Hay_size - _Needle_size) {
        // xpos cannot exist, report failure
        // N4950 [string.view.find]/3 says:
        // 1. _Start_at <= xpos
        // 2. xpos + _Needle_size <= _Hay_size;
        // therefore:
        // 3. _Needle_size <= _Hay_size (by 2) (checked above)
        // 4. _Start_at + _Needle_size <= _Hay_size (substitute 1 into 2)
        // 5. _Start_at <= _Hay_size - _Needle_size (4, move _Needle_size to other side) (also checked above)
        return static_cast<size_t>(-1);
    }

    if (_Needle_size == 0) { // empty string always matches if xpos is possible
        return _Start_at;
    }

#if _USE_STD_VECTOR_ALGORITHMS
    if constexpr (_Is_implementation_handled_char_traits<_Traits> && sizeof(typename _Traits::char_type) <= 2) {
        if (!_STD _Is_constant_evaluated()) {
            const auto _End = _Haystack + _Hay_size;
            const auto _Ptr = _STD _Search_vectorized(_Haystack + _Start_at, _End, _Needle, _Needle_size);

            if (_Ptr != _End) {
                return static_cast<size_t>(_Ptr - _Haystack);
            } else {
                return static_cast<size_t>(-1);
            }
        }
    }
#endif // _USE_STD_VECTOR_ALGORITHMS

    const auto _Possible_matches_end = _Haystack + (_Hay_size - _Needle_size) + 1;
    for (auto _Match_try = _Haystack + _Start_at;; ++_Match_try) {
        _Match_try = _Traits::find(_Match_try, static_cast<size_t>(_Possible_matches_end - _Match_try), *_Needle);
        if (!_Match_try) { // didn't find first character; report failure
            return static_cast<size_t>(-1);
        }

        if (_Traits::compare(_Match_try, _Needle, _Needle_size) == 0) { // found match
            return static_cast<size_t>(_Match_try - _Haystack);
        }
    }
}

template <class _Traits>
constexpr size_t _Traits_find_ch(_In_reads_(_Hay_size) const _Traits_ptr_t<_Traits> _Haystack, const size_t _Hay_size,
    const size_t _Start_at, const _Traits_ch_t<_Traits> _Ch) noexcept {
    // search [_Haystack, _Haystack + _Hay_size) for _Ch, at/after _Start_at
    if (_Start_at >= _Hay_size) {
        return static_cast<size_t>(-1); // (npos) no room for match
    }

#if _USE_STD_VECTOR_ALGORITHMS
    if constexpr (_Is_implementation_handled_char_traits<_Traits>) {
        if (!_STD _Is_constant_evaluated()) {
            const auto _End = _Haystack + _Hay_size;
            const auto _Ptr = _STD _Find_vectorized(_Haystack + _Start_at, _End, _Ch);

            if (_Ptr != _End) {
                return static_cast<size_t>(_Ptr - _Haystack);
            } else {
                return static_cast<size_t>(-1); // (npos) no match
            }
        }
    }
#endif // _USE_STD_VECTOR_ALGORITHMS

    const auto _Found_at = _Traits::find(_Haystack + _Start_at, _Hay_size - _Start_at, _Ch);
    if (_Found_at) {
        return static_cast<size_t>(_Found_at - _Haystack);
    }

    return static_cast<size_t>(-1); // (npos) no match
}

template <class _Traits>
constexpr size_t _Traits_rfind(_In_reads_(_Hay_size) const _Traits_ptr_t<_Traits> _Haystack, const size_t _Hay_size,
    const size_t _Start_at, _In_reads_(_Needle_size) const _Traits_ptr_t<_Traits> _Needle,
    const size_t _Needle_size) noexcept {
    // search [_Haystack, _Haystack + _Hay_size) for [_Needle, _Needle + _Needle_size) beginning before _Start_at
    if (_Needle_size == 0) {
        return (_STD min)(_Start_at, _Hay_size); // empty string always matches
    }

    if (_Needle_size > _Hay_size) { // no room for match
        return static_cast<size_t>(-1);
    }

    const size_t _Actual_start_at = (_STD min)(_Start_at, _Hay_size - _Needle_size);

#if _USE_STD_VECTOR_ALGORITHMS
    if constexpr (_Is_implementation_handled_char_traits<_Traits> && sizeof(typename _Traits::char_type) <= 2) {
        if (!_STD _Is_constant_evaluated()) {
            // _Find_end_vectorized takes into account the needle length when locating the search start.
            // As a potentially earlier start position can be specified, we need to take it into account,
            // and pick between the maximum possible start position and the specified one,
            // and then add _Needle_size, so that it is subtracted back in _Find_end_vectorized.
            const auto _End = _Haystack + _Actual_start_at + _Needle_size;
            const auto _Ptr = _STD _Find_end_vectorized(_Haystack, _End, _Needle, _Needle_size);

            if (_Ptr != _End) {
                return static_cast<size_t>(_Ptr - _Haystack);
            } else {
                return static_cast<size_t>(-1);
            }
        }
    }
#endif // _USE_STD_VECTOR_ALGORITHMS

    for (auto _Match_try = _Haystack + _Actual_start_at;; --_Match_try) {
        if (_Traits::eq(*_Match_try, *_Needle) && _Traits::compare(_Match_try, _Needle, _Needle_size) == 0) {
            return static_cast<size_t>(_Match_try - _Haystack); // found a match
        }

        if (_Match_try == _Haystack) {
            return static_cast<size_t>(-1); // at beginning, no more chance for match
        }
    }
}

template <class _Traits>
constexpr size_t _Traits_rfind_ch(_In_reads_(_Hay_size) const _Traits_ptr_t<_Traits> _Haystack, const size_t _Hay_size,
    const size_t _Start_at, const _Traits_ch_t<_Traits> _Ch) noexcept {
    // search [_Haystack, _Haystack + _Hay_size) for _Ch before _Start_at

    if (_Hay_size == 0) { // no room for match
        return static_cast<size_t>(-1);
    }

    const size_t _Actual_start_at = (_STD min)(_Start_at, _Hay_size - 1);

#if _USE_STD_VECTOR_ALGORITHMS
    if constexpr (_Is_implementation_handled_char_traits<_Traits>) {
        if (!_STD _Is_constant_evaluated()) {
            const auto _End = _Haystack + _Actual_start_at + 1;
            const auto _Ptr = _STD _Find_last_vectorized(_Haystack, _End, _Ch);

            if (_Ptr != _End) {
                return static_cast<size_t>(_Ptr - _Haystack);
            } else {
                return static_cast<size_t>(-1);
            }
        }
    }
#endif // _USE_STD_VECTOR_ALGORITHMS

    for (auto _Match_try = _Haystack + _Actual_start_at;; --_Match_try) {
        if (_Traits::eq(*_Match_try, _Ch)) {
            return static_cast<size_t>(_Match_try - _Haystack); // found a match
        }

        if (_Match_try == _Haystack) {
            return static_cast<size_t>(-1); // at beginning, no more chance for match
        }
    }
}

template <class _Elem, bool = _Is_character<_Elem>::value>
class _String_bitmap { // _String_bitmap for character types
public:
    constexpr bool _Mark(const _Elem* _First, const _Elem* const _Last) noexcept {
        // mark this bitmap such that the characters in [_First, _Last) are intended to match
        // returns whether all inputs can be placed in the bitmap
        for (; _First != _Last; ++_First) {
            _Matches[static_cast<unsigned char>(*_First)] = true;
        }

        return true;
    }

    constexpr bool _Match(const _Elem _Ch) const noexcept { // test if _Ch is in the bitmap
        // CodeQL [SM01954] This index is valid: we cast to unsigned char and the array has 256 elements.
        return _Matches[static_cast<unsigned char>(_Ch)];
    }

private:
    bool _Matches[256] = {};
};

template <class _Elem>
class _String_bitmap<_Elem, false> { // _String_bitmap for wchar_t/unsigned short/char16_t/char32_t/etc. types
public:
    static_assert(is_unsigned_v<_Elem>, "Standard char_traits is only provided for char, wchar_t, char8_t, char16_t, "
                                        "and char32_t. See N4988 [char.traits]. "
                                        "Visual C++ accepts other unsigned integral types as an extension.");

    constexpr bool _Mark(const _Elem* _First, const _Elem* const _Last) noexcept {
        // mark this bitmap such that the characters in [_First, _Last) are intended to match
        // returns whether all inputs can be placed in the bitmap
        for (; _First != _Last; ++_First) {
            const auto _Ch = *_First;
            if (_Ch >= 256U) {
                return false;
            }

            _Matches[static_cast<unsigned char>(_Ch)] = true;
        }

        return true;
    }

    constexpr bool _Match(const _Elem _Ch) const noexcept { // test if _Ch is in the bitmap
        return _Ch < 256U && _Matches[_Ch];
    }

private:
    bool _Matches[256] = {};
};

template <class _Traits>
constexpr size_t _Traits_find_first_of(_In_reads_(_Hay_size) const _Traits_ptr_t<_Traits> _Haystack,
    const size_t _Hay_size, const size_t _Start_at, _In_reads_(_Needle_size) const _Traits_ptr_t<_Traits> _Needle,
    const size_t _Needle_size) noexcept {
    // in [_Haystack, _Haystack + _Hay_size), look for one of [_Needle, _Needle + _Needle_size), at/after _Start_at
    if (_Needle_size == 0 || _Start_at >= _Hay_size) { // no match possible
        return static_cast<size_t>(-1);
    }

    const auto _Hay_start = _Haystack + _Start_at;
    const auto _Hay_end   = _Haystack + _Hay_size;

    if constexpr (_Is_implementation_handled_char_traits<_Traits>) {
#if _USE_STD_VECTOR_ALGORITHMS
        if (!_STD _Is_constant_evaluated()) {
            const size_t _Remaining_size = _Hay_size - _Start_at;
            if (_Remaining_size + _Needle_size >= _Threshold_find_first_of) {
                size_t _Pos = _Find_first_of_pos_vectorized(_Hay_start, _Remaining_size, _Needle, _Needle_size);
                if (_Pos != static_cast<size_t>(-1)) {
                    _Pos += _Start_at;
                }
                return _Pos;
            }
        }
#endif // _USE_STD_VECTOR_ALGORITHMS

        _String_bitmap<typename _Traits::char_type> _Matches;

        if (_Matches._Mark(_Needle, _Needle + _Needle_size)) {
            for (auto _Match_try = _Hay_start; _Match_try < _Hay_end; ++_Match_try) {
                if (_Matches._Match(*_Match_try)) {
                    return static_cast<size_t>(_Match_try - _Haystack); // found a match
                }
            }
            return static_cast<size_t>(-1); // no match
        }

        // couldn't put one of the characters into the bitmap, fall back to serial algorithm
    }

    for (auto _Match_try = _Hay_start; _Match_try < _Hay_end; ++_Match_try) {
        if (_Traits::find(_Needle, _Needle_size, *_Match_try)) {
            return static_cast<size_t>(_Match_try - _Haystack); // found a match
        }
    }

    return static_cast<size_t>(-1); // no match
}

template <class _Traits>
constexpr size_t _Traits_find_last_of(_In_reads_(_Hay_size) const _Traits_ptr_t<_Traits> _Haystack,
    const size_t _Hay_size, const size_t _Start_at, _In_reads_(_Needle_size) const _Traits_ptr_t<_Traits> _Needle,
    const size_t _Needle_size) noexcept {
    // in [_Haystack, _Haystack + _Hay_size), look for last of [_Needle, _Needle + _Needle_size), before _Start_at
    if (_Needle_size == 0 || _Hay_size == 0) { // not worth searching
        return static_cast<size_t>(-1);
    }

    const auto _Hay_start = (_STD min)(_Start_at, _Hay_size - 1);

    if constexpr (_Is_implementation_handled_char_traits<_Traits>) {
        using _Elem = typename _Traits::char_type;
#if _USE_STD_VECTOR_ALGORITHMS
        if constexpr (sizeof(_Elem) <= 2) {
            if (!_STD _Is_constant_evaluated()) {
                const size_t _Remaining_size = _Hay_start + 1;
                if (_Remaining_size + _Needle_size >= _Threshold_find_first_of) { // same threshold for first/last
                    return _Find_last_of_pos_vectorized(_Haystack, _Remaining_size, _Needle, _Needle_size);
                }
            }
        }
#endif // _USE_STD_VECTOR_ALGORITHMS

        _String_bitmap<_Elem> _Matches;
        if (_Matches._Mark(_Needle, _Needle + _Needle_size)) {
            for (auto _Match_try = _Haystack + _Hay_start;; --_Match_try) {
                if (_Matches._Match(*_Match_try)) {
                    return static_cast<size_t>(_Match_try - _Haystack); // found a match
                }

                if (_Match_try == _Haystack) {
                    return static_cast<size_t>(-1); // at beginning, no more chance for match
                }
            }
        }

        // couldn't put one of the characters into the bitmap, fall back to serial algorithm
    }

    for (auto _Match_try = _Haystack + _Hay_start;; --_Match_try) {
        if (_Traits::find(_Needle, _Needle_size, *_Match_try)) {
            return static_cast<size_t>(_Match_try - _Haystack); // found a match
        }

        if (_Match_try == _Haystack) {
            return static_cast<size_t>(-1); // at beginning, no more chance for match
        }
    }
}

template <class _Traits>
constexpr size_t _Traits_find_first_not_of(_In_reads_(_Hay_size) const _Traits_ptr_t<_Traits> _Haystack,
    const size_t _Hay_size, const size_t _Start_at, _In_reads_(_Needle_size) const _Traits_ptr_t<_Traits> _Needle,
    const size_t _Needle_size) noexcept {
    // in [_Haystack, _Haystack + _Hay_size), look for none of [_Needle, _Needle + _Needle_size), at/after _Start_at
    if (_Start_at >= _Hay_size) { // no room for match
        return static_cast<size_t>(-1);
    }

    const auto _Hay_start = _Haystack + _Start_at;
    const auto _Hay_end   = _Haystack + _Hay_size;

    if constexpr (_Is_implementation_handled_char_traits<_Traits>) {
        using _Elem = typename _Traits::char_type;
        _String_bitmap<_Elem> _Matches;
        if (_Matches._Mark(_Needle, _Needle + _Needle_size)) {
            for (auto _Match_try = _Hay_start; _Match_try < _Hay_end; ++_Match_try) {
                if (!_Matches._Match(*_Match_try)) {
                    return static_cast<size_t>(_Match_try - _Haystack); // found a match
                }
            }
            return static_cast<size_t>(-1); // no match
        }

        // couldn't put one of the characters into the bitmap, fall back to the serial algorithm
    }

    for (auto _Match_try = _Hay_start; _Match_try < _Hay_end; ++_Match_try) {
        if (!_Traits::find(_Needle, _Needle_size, *_Match_try)) {
            return static_cast<size_t>(_Match_try - _Haystack); // found a match
        }
    }

    return static_cast<size_t>(-1); // no match
}

template <class _Traits>
constexpr size_t _Traits_find_not_ch(_In_reads_(_Hay_size) const _Traits_ptr_t<_Traits> _Haystack,
    const size_t _Hay_size, const size_t _Start_at, const _Traits_ch_t<_Traits> _Ch) noexcept {
    // search [_Haystack, _Haystack + _Hay_size) for any value other than _Ch, at/after _Start_at
    if (_Start_at < _Hay_size) { // room for match, look for it
        const auto _End = _Haystack + _Hay_size;
        for (auto _Match_try = _Haystack + _Start_at; _Match_try < _End; ++_Match_try) {
            if (!_Traits::eq(*_Match_try, _Ch)) {
                return static_cast<size_t>(_Match_try - _Haystack); // found a match
            }
        }
    }

    return static_cast<size_t>(-1); // no match
}

template <class _Traits>
constexpr size_t _Traits_find_last_not_of(_In_reads_(_Hay_size) const _Traits_ptr_t<_Traits> _Haystack,
    const size_t _Hay_size, const size_t _Start_at, _In_reads_(_Needle_size) const _Traits_ptr_t<_Traits> _Needle,
    const size_t _Needle_size) noexcept {
    // in [_Haystack, _Haystack + _Hay_size), look for none of [_Needle, _Needle + _Needle_size), before _Start_at
    if (_Hay_size == 0) { // no match possible
        return static_cast<size_t>(-1);
    }

    const auto _Hay_start = (_STD min)(_Start_at, _Hay_size - 1);

    if constexpr (_Is_implementation_handled_char_traits<_Traits>) {
        using _Elem = typename _Traits::char_type;
        _String_bitmap<_Elem> _Matches;
        if (_Matches._Mark(_Needle, _Needle + _Needle_size)) {
            for (auto _Match_try = _Haystack + _Hay_start;; --_Match_try) {
                if (!_Matches._Match(*_Match_try)) {
                    return static_cast<size_t>(_Match_try - _Haystack); // found a match
                }

                if (_Match_try == _Haystack) {
                    return static_cast<size_t>(-1); // at beginning, no more chance for match
                }
            }
        }

        // couldn't put one of the characters into the bitmap, fall back to the serial algorithm
    }

    for (auto _Match_try = _Haystack + _Hay_start;; --_Match_try) {
        if (!_Traits::find(_Needle, _Needle_size, *_Match_try)) {
            return static_cast<size_t>(_Match_try - _Haystack); // found a match
        }

        if (_Match_try == _Haystack) {
            return static_cast<size_t>(-1); // at beginning, no more chance for match
        }
    }
}

template <class _Traits>
constexpr size_t _Traits_rfind_not_ch(_In_reads_(_Hay_size) const _Traits_ptr_t<_Traits> _Haystack,
    const size_t _Hay_size, const size_t _Start_at, const _Traits_ch_t<_Traits> _Ch) noexcept {
    // search [_Haystack, _Haystack + _Hay_size) for any value other than _Ch before _Start_at
    if (_Hay_size == 0) { // no room for match
        return static_cast<size_t>(-1);
    }

    for (auto _Match_try = _Haystack + (_STD min)(_Start_at, _Hay_size - 1);; --_Match_try) {
        if (!_Traits::eq(*_Match_try, _Ch)) {
            return static_cast<size_t>(_Match_try - _Haystack); // found a match
        }

        if (_Match_try == _Haystack) {
            return static_cast<size_t>(-1); // at beginning, no more chance for match
        }
    }
}

template <class _Ty>
constexpr bool _Is_EcharT = _Is_any_of_v<_Ty, char, wchar_t,
#ifdef __cpp_char8_t
    char8_t,
#endif // defined(__cpp_char8_t)
    char16_t, char32_t>;

#if _HAS_CXX17
_EXPORT_STD template <class _Elem, class _Traits = char_traits<_Elem>>
class basic_string_view;

template <class _Traits>
class _String_view_iterator {
public:
#if _HAS_CXX20
    using iterator_concept = contiguous_iterator_tag;
#endif // _HAS_CXX20
    using iterator_category = random_access_iterator_tag;
    using value_type        = typename _Traits::char_type;
    using difference_type   = ptrdiff_t;
    using pointer           = const value_type*;
    using reference         = const value_type&;

    constexpr _String_view_iterator() noexcept = default;

private:
    friend basic_string_view<value_type, _Traits>;

#if _ITERATOR_DEBUG_LEVEL >= 1
    constexpr _String_view_iterator(const pointer _Data, const size_t _Size, const size_t _Off) noexcept
        : _Mydata(_Data), _Mysize(_Size), _Myoff(_Off) {}
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
    constexpr explicit _String_view_iterator(const pointer _Ptr) noexcept : _Myptr(_Ptr) {}
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^

public:
    _NODISCARD constexpr reference operator*() const noexcept {
#if _ITERATOR_DEBUG_LEVEL >= 1
        _STL_VERIFY(_Mydata, "cannot dereference value-initialized string_view iterator");
        _STL_VERIFY(_Myoff < _Mysize, "cannot dereference end string_view iterator");
        return _Mydata[_Myoff];
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
        return *_Myptr;
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^
    }

    _NODISCARD constexpr pointer operator->() const noexcept {
#if _ITERATOR_DEBUG_LEVEL >= 1
        _STL_VERIFY(_Mydata, "cannot dereference value-initialized string_view iterator");
        _STL_VERIFY(_Myoff < _Mysize, "cannot dereference end string_view iterator");
        return _Mydata + _Myoff;
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
        return _Myptr;
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^
    }

    constexpr _String_view_iterator& operator++() noexcept {
#if _ITERATOR_DEBUG_LEVEL >= 1
        _STL_VERIFY(_Mydata, "cannot increment value-initialized string_view iterator");
        _STL_VERIFY(_Myoff < _Mysize, "cannot increment string_view iterator past end");
        ++_Myoff;
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
        ++_Myptr;
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^
        return *this;
    }

    constexpr _String_view_iterator operator++(int) noexcept {
        _String_view_iterator _Tmp{*this};
        ++*this;
        return _Tmp;
    }

    constexpr _String_view_iterator& operator--() noexcept {
#if _ITERATOR_DEBUG_LEVEL >= 1
        _STL_VERIFY(_Mydata, "cannot decrement value-initialized string_view iterator");
        _STL_VERIFY(_Myoff != 0, "cannot decrement string_view iterator before begin");
        --_Myoff;
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
        --_Myptr;
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^
        return *this;
    }

    constexpr _String_view_iterator operator--(int) noexcept {
        _String_view_iterator _Tmp{*this};
        --*this;
        return _Tmp;
    }

    constexpr void _Verify_offset(const difference_type _Off) const noexcept {
#if _ITERATOR_DEBUG_LEVEL >= 1
        if (_Off != 0) {
            _STL_VERIFY(_Mydata, "cannot seek value-initialized string_view iterator");
        }

        if (_Off < 0) {
            _STL_VERIFY(
                _Myoff >= size_t{0} - static_cast<size_t>(_Off), "cannot seek string_view iterator before begin");
        }

        if (_Off > 0) {
            _STL_VERIFY(_Mysize - _Myoff >= static_cast<size_t>(_Off), "cannot seek string_view iterator after end");
        }
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
        (void) _Off;
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^
    }

    constexpr _String_view_iterator& operator+=(const difference_type _Off) noexcept {
#if _ITERATOR_DEBUG_LEVEL >= 1
        _Verify_offset(_Off);
        _Myoff += static_cast<size_t>(_Off);
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
        _Myptr += _Off;
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^

        return *this;
    }

    _NODISCARD constexpr _String_view_iterator operator+(const difference_type _Off) const noexcept {
        _String_view_iterator _Tmp{*this};
        _Tmp += _Off;
        return _Tmp;
    }

    _NODISCARD friend constexpr _String_view_iterator operator+(
        const difference_type _Off, _String_view_iterator _Right) noexcept {
        _Right += _Off;
        return _Right;
    }

    constexpr _String_view_iterator& operator-=(const difference_type _Off) noexcept {
#if _ITERATOR_DEBUG_LEVEL >= 1
        if (_Off != 0) {
            _STL_VERIFY(_Mydata, "cannot seek value-initialized string_view iterator");
        }

        if (_Off > 0) {
            _STL_VERIFY(_Myoff >= static_cast<size_t>(_Off), "cannot seek string_view iterator before begin");
        }

        if (_Off < 0) {
            _STL_VERIFY(_Mysize - _Myoff >= size_t{0} - static_cast<size_t>(_Off),
                "cannot seek string_view iterator after end");
        }

        _Myoff -= static_cast<size_t>(_Off);
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
        _Myptr -= _Off;
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^

        return *this;
    }

    _NODISCARD constexpr _String_view_iterator operator-(const difference_type _Off) const noexcept {
        _String_view_iterator _Tmp{*this};
        _Tmp -= _Off;
        return _Tmp;
    }

    _NODISCARD constexpr difference_type operator-(const _String_view_iterator& _Right) const noexcept {
#if _ITERATOR_DEBUG_LEVEL >= 1
        _STL_VERIFY(_Mydata == _Right._Mydata && _Mysize == _Right._Mysize,
            "cannot subtract incompatible string_view iterators");
        return static_cast<difference_type>(_Myoff - _Right._Myoff);
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
        return _Myptr - _Right._Myptr;
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^
    }

    _NODISCARD constexpr reference operator[](const difference_type _Off) const noexcept {
        return *(*this + _Off);
    }

    _NODISCARD constexpr bool operator==(const _String_view_iterator& _Right) const noexcept {
#if _ITERATOR_DEBUG_LEVEL >= 1
        _STL_VERIFY(_Mydata == _Right._Mydata && _Mysize == _Right._Mysize,
            "cannot compare incompatible string_view iterators for equality");
        return _Myoff == _Right._Myoff;
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
        return _Myptr == _Right._Myptr;
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^
    }

#if _HAS_CXX20
    _NODISCARD constexpr strong_ordering operator<=>(const _String_view_iterator& _Right) const noexcept {
#if _ITERATOR_DEBUG_LEVEL >= 1
        _STL_VERIFY(_Mydata == _Right._Mydata && _Mysize == _Right._Mysize,
            "cannot compare incompatible string_view iterators");
        return _Myoff <=> _Right._Myoff;
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
        return _Myptr <=> _Right._Myptr;
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^
    }
#else // ^^^ _HAS_CXX20 / !_HAS_CXX20 vvv
    _NODISCARD constexpr bool operator!=(const _String_view_iterator& _Right) const noexcept {
        return !(*this == _Right);
    }

    _NODISCARD constexpr bool operator<(const _String_view_iterator& _Right) const noexcept {
#if _ITERATOR_DEBUG_LEVEL >= 1
        _STL_VERIFY(_Mydata == _Right._Mydata && _Mysize == _Right._Mysize,
            "cannot compare incompatible string_view iterators");
        return _Myoff < _Right._Myoff;
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
        return _Myptr < _Right._Myptr;
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^
    }

    _NODISCARD constexpr bool operator>(const _String_view_iterator& _Right) const noexcept {
        return _Right < *this;
    }

    _NODISCARD constexpr bool operator<=(const _String_view_iterator& _Right) const noexcept {
        return !(_Right < *this);
    }

    _NODISCARD constexpr bool operator>=(const _String_view_iterator& _Right) const noexcept {
        return !(*this < _Right);
    }
#endif // !_HAS_CXX20

#if _ITERATOR_DEBUG_LEVEL >= 1
    friend constexpr void _Verify_range(const _String_view_iterator& _First, const _String_view_iterator& _Last) {
        _STL_VERIFY(_First._Mydata == _Last._Mydata && _First._Mysize == _Last._Mysize,
            "string_view iterators in range are from different views");
        _STL_VERIFY(_First._Myoff <= _Last._Myoff, "string_view iterator range transposed");
    }
#endif // _ITERATOR_DEBUG_LEVEL >= 1

    using _Prevent_inheriting_unwrap = _String_view_iterator;

    _NODISCARD constexpr pointer _Unwrapped() const noexcept {
#if _ITERATOR_DEBUG_LEVEL >= 1
        return _Mydata + _Myoff;
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
        return _Myptr;
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^
    }

    static constexpr bool _Unwrap_when_unverified = _ITERATOR_DEBUG_LEVEL == 0;

    constexpr void _Seek_to(pointer _It) noexcept {
#if _ITERATOR_DEBUG_LEVEL >= 1
        _Myoff = static_cast<size_t>(_It - _Mydata);
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
        _Myptr = _It;
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^
    }

private:
#if _ITERATOR_DEBUG_LEVEL >= 1
    pointer _Mydata = nullptr;
    size_t _Mysize  = 0;
    size_t _Myoff   = 0;
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
    pointer _Myptr = nullptr;
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^
};

#if _HAS_CXX20
template <class _Traits>
struct pointer_traits<_String_view_iterator<_Traits>> {
    using pointer         = _String_view_iterator<_Traits>;
    using element_type    = const pointer::value_type;
    using difference_type = pointer::difference_type;

    _NODISCARD static constexpr element_type* to_address(const pointer& _Iter) noexcept {
        return _Iter._Unwrapped();
    }
};
#endif // _HAS_CXX20

#pragma warning(push)
// Invalid annotation: 'NullTerminated' property may only be used on buffers whose elements are of integral or pointer
// type
#pragma warning(disable : 6510)

_EXPORT_STD template <class _Elem, class _Traits>
class basic_string_view { // wrapper for any kind of contiguous character buffer
public:
    static_assert(is_same_v<_Elem, typename _Traits::char_type>,
        "Bad char_traits for basic_string_view; N4950 [string.view.template.general]/1 "
        "\"The program is ill-formed if traits::char_type is not the same type as charT.\"");

    static_assert(!is_array_v<_Elem> && is_trivially_copyable_v<_Elem> && is_trivially_default_constructible_v<_Elem>
                      && is_standard_layout_v<_Elem>,
        "The character type of basic_string_view must be a non-array trivially copyable standard-layout type T where "
        "is_trivially_default_constructible_v<T> is true. See N5001 [strings.general]/1.");

    using traits_type            = _Traits;
    using value_type             = _Elem;
    using pointer                = _Elem*;
    using const_pointer          = const _Elem*;
    using reference              = _Elem&;
    using const_reference        = const _Elem&;
    using const_iterator         = _String_view_iterator<_Traits>;
    using iterator               = const_iterator;
    using const_reverse_iterator = _STD reverse_iterator<const_iterator>;
    using reverse_iterator       = const_reverse_iterator;
    using size_type              = size_t;
    using difference_type        = ptrdiff_t;

    static constexpr auto npos{static_cast<size_type>(-1)};

    constexpr basic_string_view() noexcept : _Mydata(), _Mysize(0) {}

    constexpr basic_string_view(const basic_string_view&) noexcept            = default;
    constexpr basic_string_view& operator=(const basic_string_view&) noexcept = default;

    /* implicit */ constexpr basic_string_view(_In_z_ const const_pointer _Ntcts) noexcept // strengthened
        : _Mydata(_Ntcts), _Mysize(_Traits::length(_Ntcts)) {}

#if _HAS_CXX23
    basic_string_view(nullptr_t) = delete;
#endif // _HAS_CXX23

    constexpr basic_string_view(
        _In_reads_(_Count) const const_pointer _Cts, const size_type _Count) noexcept // strengthened
        : _Mydata(_Cts), _Mysize(_Count) {
#if _ITERATOR_DEBUG_LEVEL != 0
        _STL_VERIFY(_Count == 0 || _Cts, "cannot construct a string_view from a null pointer and a non-zero size");
#endif
    }

#if _HAS_CXX20
    template <contiguous_iterator _Iter, sized_sentinel_for<_Iter> _Sent>
        requires (is_same_v<iter_value_t<_Iter>, _Elem> && !is_convertible_v<_Sent, size_type>)
    constexpr basic_string_view(_Iter _First, _Sent _Last) noexcept(noexcept(_Last - _First)) // strengthened
        : _Mydata(_STD to_address(_First)), _Mysize(static_cast<size_type>(_Last - _First)) {}

#if _HAS_CXX23
    template <class _Range>
        requires (!same_as<remove_cvref_t<_Range>, basic_string_view> && _RANGES contiguous_range<_Range>
                     && _RANGES sized_range<_Range> && same_as<_RANGES range_value_t<_Range>, _Elem>
                     && !is_convertible_v<_Range, const _Elem*>
                     && !requires(
                         remove_cvref_t<_Range>& _Rng) { _Rng.operator _STD basic_string_view<_Elem, _Traits>(); })
    constexpr explicit basic_string_view(_Range&& _Rng)
        noexcept(noexcept(_RANGES data(_Rng)) && noexcept(_RANGES size(_Rng))) // strengthened
        : _Mydata(_RANGES data(_Rng)), _Mysize(static_cast<size_t>(_RANGES size(_Rng))) {}
#endif // _HAS_CXX23
#endif // _HAS_CXX20

    _NODISCARD constexpr const_iterator begin() const noexcept {
#if _ITERATOR_DEBUG_LEVEL >= 1
        return const_iterator(_Mydata, _Mysize, 0);
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
        return const_iterator(_Mydata);
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^
    }

    _NODISCARD constexpr const_iterator end() const noexcept {
#if _ITERATOR_DEBUG_LEVEL >= 1
        return const_iterator(_Mydata, _Mysize, _Mysize);
#else // ^^^ _ITERATOR_DEBUG_LEVEL >= 1 / _ITERATOR_DEBUG_LEVEL == 0 vvv
        return const_iterator(_Mydata + _Mysize);
#endif // ^^^ _ITERATOR_DEBUG_LEVEL == 0 ^^^
    }

    _NODISCARD constexpr const_iterator cbegin() const noexcept {
        return begin();
    }

    _NODISCARD constexpr const_iterator cend() const noexcept {
        return end();
    }

    _NODISCARD constexpr const_reverse_iterator rbegin() const noexcept {
        return const_reverse_iterator{end()};
    }

    _NODISCARD constexpr const_reverse_iterator rend() const noexcept {
        return const_reverse_iterator{begin()};
    }

    _NODISCARD constexpr const_reverse_iterator crbegin() const noexcept {
        return rbegin();
    }

    _NODISCARD constexpr const_reverse_iterator crend() const noexcept {
        return rend();
    }

    constexpr const_pointer _Unchecked_begin() const noexcept {
        return _Mydata;
    }

    constexpr const_pointer _Unchecked_end() const noexcept {
        return _Mydata + _Mysize;
    }

    _NODISCARD constexpr size_type size() const noexcept {
        return _Mysize;
    }

    _NODISCARD constexpr size_type length() const noexcept {
        return _Mysize;
    }

    _NODISCARD constexpr bool empty() const noexcept {
        return _Mysize == 0;
    }

    _NODISCARD constexpr const_pointer data() const noexcept {
        return _Mydata;
    }

    _NODISCARD constexpr size_type max_size() const noexcept {
        // bound to PTRDIFF_MAX to make end() - begin() well defined (also makes room for npos)
        // bound to static_cast<size_t>(-1) / sizeof(_Elem) by address space limits
        return (_STD min)(static_cast<size_t>(PTRDIFF_MAX), static_cast<size_t>(-1) / sizeof(_Elem));
    }

    _NODISCARD constexpr const_reference operator[](const size_type _Off) const noexcept /* strengthened */ {
#if _MSVC_STL_HARDENING_BASIC_STRING_VIEW || _ITERATOR_DEBUG_LEVEL != 0
        _STL_VERIFY(_Off < _Mysize, "string_view subscript out of range");
#endif

        // CodeQL [SM01954] This index is optionally validated above.
        return _Mydata[_Off];
    }

    _NODISCARD constexpr const_reference at(const size_type _Off) const {
        // get the character at _Off or throw if that is out of range
        _Check_offset_exclusive(_Off);
        return _Mydata[_Off];
    }

    _NODISCARD constexpr const_reference front() const noexcept /* strengthened */ {
#if _MSVC_STL_HARDENING_BASIC_STRING_VIEW || _ITERATOR_DEBUG_LEVEL != 0
        _STL_VERIFY(_Mysize != 0, "front() called on empty string_view");
#endif

        return _Mydata[0];
    }

    _NODISCARD constexpr const_reference back() const noexcept /* strengthened */ {
#if _MSVC_STL_HARDENING_BASIC_STRING_VIEW || _ITERATOR_DEBUG_LEVEL != 0
        _STL_VERIFY(_Mysize != 0, "back() called on empty string_view");
#endif

        return _Mydata[_Mysize - 1];
    }

    constexpr void remove_prefix(const size_type _Count) noexcept /* strengthened */ {
#if _MSVC_STL_HARDENING_BASIC_STRING_VIEW || _ITERATOR_DEBUG_LEVEL != 0
        _STL_VERIFY(_Mysize >= _Count, "cannot remove_prefix() larger than string_view size");
#endif

        _Mydata += _Count;
        _Mysize -= _Count;
    }

    constexpr void remove_suffix(const size_type _Count) noexcept /* strengthened */ {
#if _MSVC_STL_HARDENING_BASIC_STRING_VIEW || _ITERATOR_DEBUG_LEVEL != 0
        _STL_VERIFY(_Mysize >= _Count, "cannot remove_suffix() larger than string_view size");
#endif

        _Mysize -= _Count;
    }

    constexpr void swap(basic_string_view& _Other) noexcept {
        const basic_string_view _Tmp{_Other}; // note: std::swap is not constexpr before C++20
        _Other = *this;
        *this  = _Tmp;
    }

    _CONSTEXPR20 size_type copy(
        _Out_writes_(_Count) _Elem* const _Ptr, size_type _Count, const size_type _Off = 0) const {
        // copy [_Off, _Off + Count) to [_Ptr, _Ptr + _Count)
        _Check_offset(_Off);
        _Count = _Clamp_suffix_size(_Off, _Count);
        _Traits::copy(_Ptr, _Mydata + _Off, _Count);
        return _Count;
    }

    _Pre_satisfies_(_Dest_size >= _Count) _CONSTEXPR20 size_type
        _Copy_s(_Out_writes_all_(_Dest_size) _Elem* const _Dest, const size_type _Dest_size, size_type _Count,
            const size_type _Off = 0) const {
        // copy [_Off, _Off + _Count) to [_Dest, _Dest + _Count)
        _Check_offset(_Off);
        _Count = _Clamp_suffix_size(_Off, _Count);
        _Traits::_Copy_s(_Dest, _Dest_size, _Mydata + _Off, _Count);
        return _Count;
    }

    _NODISCARD constexpr basic_string_view substr(const size_type _Off = 0, size_type _Count = npos) const {
        // return a new basic_string_view moved forward by _Off and trimmed to _Count elements
        _Check_offset(_Off);
        _Count = _Clamp_suffix_size(_Off, _Count);
        return basic_string_view(_Mydata + _Off, _Count);
    }

    constexpr bool _Equal(const basic_string_view _Right) const noexcept {
        return _Traits_equal<_Traits>(_Mydata, _Mysize, _Right._Mydata, _Right._Mysize);
    }

    _NODISCARD constexpr int compare(const basic_string_view _Right) const noexcept {
        return _Traits_compare<_Traits>(_Mydata, _Mysize, _Right._Mydata, _Right._Mysize);
    }

    _NODISCARD constexpr int compare(const size_type _Off, const size_type _Nx, const basic_string_view _Right) const {
        // compare [_Off, _Off + _Nx) with _Right
        return substr(_Off, _Nx).compare(_Right);
    }

    _NODISCARD constexpr int compare(const size_type _Off, const size_type _Nx, const basic_string_view _Right,
        const size_type _Roff, const size_type _Count) const {
        // compare [_Off, _Off + _Nx) with _Right [_Roff, _Roff + _Count)
        return substr(_Off, _Nx).compare(_Right.substr(_Roff, _Count));
    }

    _NODISCARD constexpr int compare(_In_z_ const _Elem* const _Ptr) const noexcept /* strengthened */ {
        // compare [0, _Mysize) with [_Ptr, <null>)
        return compare(basic_string_view(_Ptr));
    }

    _NODISCARD constexpr int compare(const size_type _Off, const size_type _Nx, _In_z_ const _Elem* const _Ptr) const {
        // compare [_Off, _Off + _Nx) with [_Ptr, <null>)
        return substr(_Off, _Nx).compare(basic_string_view(_Ptr));
    }

    _NODISCARD constexpr int compare(const size_type _Off, const size_type _Nx,
        _In_reads_(_Count) const _Elem* const _Ptr, const size_type _Count) const {
        // compare [_Off, _Off + _Nx) with [_Ptr, _Ptr + _Count)
        return substr(_Off, _Nx).compare(basic_string_view(_Ptr, _Count));
    }

#if _HAS_CXX20
    _NODISCARD constexpr bool starts_with(const basic_string_view _Right) const noexcept {
        const auto _Rightsize = _Right._Mysize;
        if (_Mysize < _Rightsize) {
            return false;
        }
        return _Traits::compare(_Mydata, _Right._Mydata, _Rightsize) == 0;
    }

    _NODISCARD constexpr bool starts_with(const _Elem _Right) const noexcept {
        return !empty() && _Traits::eq(front(), _Right);
    }

    _NODISCARD constexpr bool starts_with(const _Elem* const _Right) const noexcept /* strengthened */ {
        return starts_with(basic_string_view(_Right));
    }

    _NODISCARD constexpr bool ends_with(const basic_string_view _Right) const noexcept {
        const auto _Rightsize = _Right._Mysize;
        if (_Mysize < _Rightsize) {
            return false;
        }
        return _Traits::compare(_Mydata + (_Mysize - _Rightsize), _Right._Mydata, _Rightsize) == 0;
    }

    _NODISCARD constexpr bool ends_with(const _Elem _Right) const noexcept {
        return !empty() && _Traits::eq(back(), _Right);
    }

    _NODISCARD constexpr bool ends_with(const _Elem* const _Right) const noexcept /* strengthened */ {
        return ends_with(basic_string_view(_Right));
    }
#endif // _HAS_CXX20

#if _HAS_CXX23
    _NODISCARD constexpr bool contains(const basic_string_view _Right) const noexcept {
        return find(_Right) != npos;
    }

    _NODISCARD constexpr bool contains(const _Elem _Right) const noexcept {
        return find(_Right) != npos;
    }

    _NODISCARD constexpr bool contains(const _Elem* const _Right) const noexcept /* strengthened */ {
        return find(_Right) != npos;
    }
#endif // _HAS_CXX23

    _NODISCARD constexpr size_type find(const basic_string_view _Right, const size_type _Off = 0) const noexcept {
        // look for _Right beginning at or after _Off
        return _Traits_find<_Traits>(_Mydata, _Mysize, _Off, _Right._Mydata, _Right._Mysize);
    }

    _NODISCARD constexpr size_type find(const _Elem _Ch, const size_type _Off = 0) const noexcept {
        // look for _Ch at or after _Off
        return _Traits_find_ch<_Traits>(_Mydata, _Mysize, _Off, _Ch);
    }

    _NODISCARD constexpr size_type find(_In_reads_(_Count) const _Elem* const _Ptr, const size_type _Off,
        const size_type _Count) const noexcept /* strengthened */ {
        // look for [_Ptr, _Ptr + _Count) beginning at or after _Off
        return _Traits_find<_Traits>(_Mydata, _Mysize, _Off, _Ptr, _Count);
    }

    _NODISCARD constexpr size_type find(_In_z_ const _Elem* const _Ptr, const size_type _Off = 0) const noexcept
    /* strengthened */ {
        // look for [_Ptr, <null>) beginning at or after _Off
        return _Traits_find<_Traits>(_Mydata, _Mysize, _Off, _Ptr, _Traits::length(_Ptr));
    }

    _NODISCARD constexpr size_type rfind(const basic_string_view _Right, const size_type _Off = npos) const noexcept {
        // look for _Right beginning before _Off
        return _Traits_rfind<_Traits>(_Mydata, _Mysize, _Off, _Right._Mydata, _Right._Mysize);
    }

    _NODISCARD constexpr size_type rfind(const _Elem _Ch, const size_type _Off = npos) const noexcept {
        // look for _Ch before _Off
        return _Traits_rfind_ch<_Traits>(_Mydata, _Mysize, _Off, _Ch);
    }

    _NODISCARD constexpr size_type rfind(_In_reads_(_Count) const _Elem* const _Ptr, const size_type _Off,
        const size_type _Count) const noexcept /* strengthened */ {
        // look for [_Ptr, _Ptr + _Count) beginning before _Off
        return _Traits_rfind<_Traits>(_Mydata, _Mysize, _Off, _Ptr, _Count);
    }

    _NODISCARD constexpr size_type rfind(_In_z_ const _Elem* const _Ptr, const size_type _Off = npos) const noexcept
    /* strengthened */ {
        // look for [_Ptr, <null>) beginning before _Off
        return _Traits_rfind<_Traits>(_Mydata, _Mysize, _Off, _Ptr, _Traits::length(_Ptr));
    }

    _NODISCARD constexpr size_type find_first_of(const basic_string_view _Right,
        const size_type _Off = 0) const noexcept { // look for one of _Right at or after _Off
        return _Traits_find_first_of<_Traits>(_Mydata, _Mysize, _Off, _Right._Mydata, _Right._Mysize);
    }

    _NODISCARD constexpr size_type find_first_of(const _Elem _Ch, const size_type _Off = 0) const noexcept {
        // look for _Ch at or after _Off
        return _Traits_find_ch<_Traits>(_Mydata, _Mysize, _Off, _Ch);
    }

    _NODISCARD constexpr size_type find_first_of(_In_reads_(_Count) const _Elem* const _Ptr, const size_type _Off,
        const size_type _Count) const noexcept /* strengthened */ {
        // look for one of [_Ptr, _Ptr + _Count) at or after _Off
        return _Traits_find_first_of<_Traits>(_Mydata, _Mysize, _Off, _Ptr, _Count);
    }

    _NODISCARD constexpr size_type find_first_of(
        _In_z_ const _Elem* const _Ptr, const size_type _Off = 0) const noexcept /* strengthened */ {
        // look for one of [_Ptr, <null>) at or after _Off
        return _Traits_find_first_of<_Traits>(_Mydata, _Mysize, _Off, _Ptr, _Traits::length(_Ptr));
    }

    _NODISCARD constexpr size_type find_last_of(const basic_string_view _Right,
        const size_type _Off = npos) const noexcept { // look for one of _Right before _Off
        return _Traits_find_last_of<_Traits>(_Mydata, _Mysize, _Off, _Right._Mydata, _Right._Mysize);
    }

    _NODISCARD constexpr size_type find_last_of(const _Elem _Ch, const size_type _Off = npos) const noexcept {
        // look for _Ch before _Off
        return _Traits_rfind_ch<_Traits>(_Mydata, _Mysize, _Off, _Ch);
    }

    _NODISCARD constexpr size_type find_last_of(_In_reads_(_Count) const _Elem* const _Ptr, const size_type _Off,
        const size_type _Count) const noexcept /* strengthened */ {
        // look for one of [_Ptr, _Ptr + _Count) before _Off
        return _Traits_find_last_of<_Traits>(_Mydata, _Mysize, _Off, _Ptr, _Count);
    }

    _NODISCARD constexpr size_type find_last_of(
        _In_z_ const _Elem* const _Ptr, const size_type _Off = npos) const noexcept /* strengthened */ {
        // look for one of [_Ptr, <null>) before _Off
        return _Traits_find_last_of<_Traits>(_Mydata, _Mysize, _Off, _Ptr, _Traits::length(_Ptr));
    }

    _NODISCARD constexpr size_type find_first_not_of(const basic_string_view _Right,
        const size_type _Off = 0) const noexcept { // look for none of _Right at or after _Off
        return _Traits_find_first_not_of<_Traits>(_Mydata, _Mysize, _Off, _Right._Mydata, _Right._Mysize);
    }

    _NODISCARD constexpr size_type find_first_not_of(const _Elem _Ch, const size_type _Off = 0) const noexcept {
        // look for any value other than _Ch at or after _Off
        return _Traits_find_not_ch<_Traits>(_Mydata, _Mysize, _Off, _Ch);
    }

    _NODISCARD constexpr size_type find_first_not_of(_In_reads_(_Count) const _Elem* const _Ptr, const size_type _Off,
        const size_type _Count) const noexcept /* strengthened */ {
        // look for none of [_Ptr, _Ptr + _Count) at or after _Off
        return _Traits_find_first_not_of<_Traits>(_Mydata, _Mysize, _Off, _Ptr, _Count);
    }

    _NODISCARD constexpr size_type find_first_not_of(
        _In_z_ const _Elem* const _Ptr, const size_type _Off = 0) const noexcept /* strengthened */ {
        // look for none of [_Ptr, <null>) at or after _Off
        return _Traits_find_first_not_of<_Traits>(_Mydata, _Mysize, _Off, _Ptr, _Traits::length(_Ptr));
    }

    _NODISCARD constexpr size_type find_last_not_of(const basic_string_view _Right,
        const size_type _Off = npos) const noexcept { // look for none of _Right before _Off
        return _Traits_find_last_not_of<_Traits>(_Mydata, _Mysize, _Off, _Right._Mydata, _Right._Mysize);
    }

    _NODISCARD constexpr size_type find_last_not_of(const _Elem _Ch, const size_type _Off = npos) const noexcept {
        // look for any value other than _Ch before _Off
        return _Traits_rfind_not_ch<_Traits>(_Mydata, _Mysize, _Off, _Ch);
    }

    _NODISCARD constexpr size_type find_last_not_of(_In_reads_(_Count) const _Elem* const _Ptr, const size_type _Off,
        const size_type _Count) const noexcept /* strengthened */ {
        // look for none of [_Ptr, _Ptr + _Count) before _Off
        return _Traits_find_last_not_of<_Traits>(_Mydata, _Mysize, _Off, _Ptr, _Count);
    }

    _NODISCARD constexpr size_type find_last_not_of(
        _In_z_ const _Elem* const _Ptr, const size_type _Off = npos) const noexcept /* strengthened */ {
        // look for none of [_Ptr, <null>) before _Off
        return _Traits_find_last_not_of<_Traits>(_Mydata, _Mysize, _Off, _Ptr, _Traits::length(_Ptr));
    }

    _NODISCARD constexpr bool _Starts_with(const basic_string_view _View) const noexcept {
        return _Mysize >= _View._Mysize && _Traits::compare(_Mydata, _View._Mydata, _View._Mysize) == 0;
    }

private:
    constexpr void _Check_offset(const size_type _Off) const { // checks whether _Off is in the bounds of [0, size()]
        if (_Mysize < _Off) {
            _Xran();
        }
    }

    constexpr void _Check_offset_exclusive(const size_type _Off) const {
        // checks whether _Off is in the bounds of [0, size())
        if (_Mysize <= _Off) {
            _Xran();
        }
    }

    constexpr size_type _Clamp_suffix_size(const size_type _Off, const size_type _Size) const noexcept {
        // trims _Size to the longest it can be assuming a string at/after _Off
        return (_STD min)(_Size, _Mysize - _Off);
    }

    [[noreturn]] static void _Xran() {
        _Xout_of_range("invalid string_view position");
    }

    const_pointer _Mydata;
    size_type _Mysize;
};

#pragma warning(pop)

#if _HAS_CXX20
template <contiguous_iterator _Iter, sized_sentinel_for<_Iter> _Sent>
basic_string_view(_Iter, _Sent) -> basic_string_view<iter_value_t<_Iter>>;

#if _HAS_CXX23
template <_RANGES contiguous_range _Range>
basic_string_view(_Range&&) -> basic_string_view<_RANGES range_value_t<_Range>>;
#endif // _HAS_CXX23

namespace ranges {
    template <class _Elem, class _Traits>
    constexpr bool enable_view<basic_string_view<_Elem, _Traits>> = true;
    template <class _Elem, class _Traits>
    constexpr bool enable_borrowed_range<basic_string_view<_Elem, _Traits>> = true;
} // namespace ranges
#endif // _HAS_CXX20

#if _HAS_CXX20
_EXPORT_STD template <class _Elem, class _Traits>
_NODISCARD constexpr bool operator==(const basic_string_view<_Elem, _Traits> _Lhs,
    const type_identity_t<basic_string_view<_Elem, _Traits>> _Rhs) noexcept {
    return _Lhs._Equal(_Rhs);
}

template <class _Traits>
struct _Get_comparison_category {
    using type = weak_ordering;
};

template <class _Traits>
    requires requires { typename _Traits::comparison_category; }
struct _Get_comparison_category<_Traits> {
    using type = _Traits::comparison_category;

    static_assert(_Is_any_of_v<type, partial_ordering, weak_ordering, strong_ordering>,
        "N4971 [string.view.comparison]/4: Mandates: R denotes a comparison category type.");
};

template <class _Traits>
using _Get_comparison_category_t = _Get_comparison_category<_Traits>::type;

_EXPORT_STD template <class _Elem, class _Traits>
_NODISCARD constexpr _Get_comparison_category_t<_Traits> operator<=>(const basic_string_view<_Elem, _Traits> _Lhs,
    const type_identity_t<basic_string_view<_Elem, _Traits>> _Rhs) noexcept {
    return static_cast<_Get_comparison_category_t<_Traits>>(_Lhs.compare(_Rhs) <=> 0);
}
#else // ^^^ _HAS_CXX20 / !_HAS_CXX20 vvv
template <class _Elem, class _Traits>
_NODISCARD constexpr bool operator==(
    const basic_string_view<_Elem, _Traits> _Lhs, const basic_string_view<_Elem, _Traits> _Rhs) noexcept {
    return _Lhs._Equal(_Rhs);
}

template <class _Elem, class _Traits, int = 1> // TRANSITION, VSO-409326
_NODISCARD constexpr bool operator==(
    const _Identity_t<basic_string_view<_Elem, _Traits>> _Lhs, const basic_string_view<_Elem, _Traits> _Rhs) noexcept {
    return _Lhs._Equal(_Rhs);
}

template <class _Elem, class _Traits, int = 2> // TRANSITION, VSO-409326
_NODISCARD constexpr bool operator==(
    const basic_string_view<_Elem, _Traits> _Lhs, const _Identity_t<basic_string_view<_Elem, _Traits>> _Rhs) noexcept {
    return _Lhs._Equal(_Rhs);
}

template <class _Elem, class _Traits>
_NODISCARD constexpr bool operator!=(
    const basic_string_view<_Elem, _Traits> _Lhs, const basic_string_view<_Elem, _Traits> _Rhs) noexcept {
    return !_Lhs._Equal(_Rhs);
}

template <class _Elem, class _Traits, int = 1> // TRANSITION, VSO-409326
_NODISCARD constexpr bool operator!=(
    const _Identity_t<basic_string_view<_Elem, _Traits>> _Lhs, const basic_string_view<_Elem, _Traits> _Rhs) noexcept {
    return !_Lhs._Equal(_Rhs);
}

template <class _Elem, class _Traits, int = 2> // TRANSITION, VSO-409326
_NODISCARD constexpr bool operator!=(
    const basic_string_view<_Elem, _Traits> _Lhs, const _Identity_t<basic_string_view<_Elem, _Traits>> _Rhs) noexcept {
    return !_Lhs._Equal(_Rhs);
}

template <class _Elem, class _Traits>
_NODISCARD constexpr bool operator<(
    const basic_string_view<_Elem, _Traits> _Lhs, const basic_string_view<_Elem, _Traits> _Rhs) noexcept {
    return _Lhs.compare(_Rhs) < 0;
}

template <class _Elem, class _Traits, int = 1> // TRANSITION, VSO-409326
_NODISCARD constexpr bool operator<(
    const _Identity_t<basic_string_view<_Elem, _Traits>> _Lhs, const basic_string_view<_Elem, _Traits> _Rhs) noexcept {
    return _Lhs.compare(_Rhs) < 0;
}

template <class _Elem, class _Traits, int = 2> // TRANSITION, VSO-409326
_NODISCARD constexpr bool operator<(
    const basic_string_view<_Elem, _Traits> _Lhs, const _Identity_t<basic_string_view<_Elem, _Traits>> _Rhs) noexcept {
    return _Lhs.compare(_Rhs) < 0;
}

template <class _Elem, class _Traits>
_NODISCARD constexpr bool operator>(
    const basic_string_view<_Elem, _Traits> _Lhs, const basic_string_view<_Elem, _Traits> _Rhs) noexcept {
    return _Lhs.compare(_Rhs) > 0;
}

template <class _Elem, class _Traits, int = 1> // TRANSITION, VSO-409326
_NODISCARD constexpr bool operator>(
    const _Identity_t<basic_string_view<_Elem, _Traits>> _Lhs, const basic_string_view<_Elem, _Traits> _Rhs) noexcept {
    return _Lhs.compare(_Rhs) > 0;
}

template <class _Elem, class _Traits, int = 2> // TRANSITION, VSO-409326
_NODISCARD constexpr bool operator>(
    const basic_string_view<_Elem, _Traits> _Lhs, const _Identity_t<basic_string_view<_Elem, _Traits>> _Rhs) noexcept {
    return _Lhs.compare(_Rhs) > 0;
}

template <class _Elem, class _Traits>
_NODISCARD constexpr bool operator<=(
    const basic_string_view<_Elem, _Traits> _Lhs, const basic_string_view<_Elem, _Traits> _Rhs) noexcept {
    return _Lhs.compare(_Rhs) <= 0;
}

template <class _Elem, class _Traits, int = 1> // TRANSITION, VSO-409326
_NODISCARD constexpr bool operator<=(
    const _Identity_t<basic_string_view<_Elem, _Traits>> _Lhs, const basic_string_view<_Elem, _Traits> _Rhs) noexcept {
    return _Lhs.compare(_Rhs) <= 0;
}

template <class _Elem, class _Traits, int = 2> // TRANSITION, VSO-409326
_NODISCARD constexpr bool operator<=(
    const basic_string_view<_Elem, _Traits> _Lhs, const _Identity_t<basic_string_view<_Elem, _Traits>> _Rhs) noexcept {
    return _Lhs.compare(_Rhs) <= 0;
}

template <class _Elem, class _Traits>
_NODISCARD constexpr bool operator>=(
    const basic_string_view<_Elem, _Traits> _Lhs, const basic_string_view<_Elem, _Traits> _Rhs) noexcept {
    return _Lhs.compare(_Rhs) >= 0;
}

template <class _Elem, class _Traits, int = 1> // TRANSITION, VSO-409326
_NODISCARD constexpr bool operator>=(
    const _Identity_t<basic_string_view<_Elem, _Traits>> _Lhs, const basic_string_view<_Elem, _Traits> _Rhs) noexcept {
    return _Lhs.compare(_Rhs) >= 0;
}

template <class _Elem, class _Traits, int = 2> // TRANSITION, VSO-409326
_NODISCARD constexpr bool operator>=(
    const basic_string_view<_Elem, _Traits> _Lhs, const _Identity_t<basic_string_view<_Elem, _Traits>> _Rhs) noexcept {
    return _Lhs.compare(_Rhs) >= 0;
}
#endif // ^^^ !_HAS_CXX20 ^^^

_EXPORT_STD using string_view = basic_string_view<char>;
#ifdef __cpp_lib_char8_t
_EXPORT_STD using u8string_view = basic_string_view<char8_t>;
#endif // defined(__cpp_lib_char8_t)
_EXPORT_STD using u16string_view = basic_string_view<char16_t>;
_EXPORT_STD using u32string_view = basic_string_view<char32_t>;
_EXPORT_STD using wstring_view   = basic_string_view<wchar_t>;

template <class _Elem>
struct hash<basic_string_view<_Elem>> : _Conditionally_enabled_hash<basic_string_view<_Elem>, _Is_EcharT<_Elem>> {
    _NODISCARD static size_t _Do_hash(const basic_string_view<_Elem> _Keyval) noexcept {
        return _Hash_array_representation(_Keyval.data(), _Keyval.size());
    }
};

_EXPORT_STD template <class _Elem, class _Traits>
basic_ostream<_Elem, _Traits>& operator<<(
    basic_ostream<_Elem, _Traits>& _Ostr, const basic_string_view<_Elem, _Traits> _Str) {
    return _Insert_string(_Ostr, _Str.data(), _Str.size());
}

inline namespace literals {
    inline namespace string_view_literals {
        _EXPORT_STD _NODISCARD constexpr string_view operator""sv(const char* _Str, size_t _Len) noexcept {
            return string_view(_Str, _Len);
        }

        _EXPORT_STD _NODISCARD constexpr wstring_view operator""sv(const wchar_t* _Str, size_t _Len) noexcept {
            return wstring_view(_Str, _Len);
        }

#ifdef __cpp_char8_t
        _EXPORT_STD _NODISCARD constexpr basic_string_view<char8_t> operator""sv(
            const char8_t* _Str, size_t _Len) noexcept {
            return basic_string_view<char8_t>(_Str, _Len);
        }
#endif // defined(__cpp_char8_t)

        _EXPORT_STD _NODISCARD constexpr u16string_view operator""sv(const char16_t* _Str, size_t _Len) noexcept {
            return u16string_view(_Str, _Len);
        }

        _EXPORT_STD _NODISCARD constexpr u32string_view operator""sv(const char32_t* _Str, size_t _Len) noexcept {
            return u32string_view(_Str, _Len);
        }
    } // namespace string_view_literals
} // namespace literals
#endif // _HAS_CXX17
_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)

#endif // _STL_COMPILER_PREPROCESSOR
#endif // __MSVC_STRING_VIEW_HPP
