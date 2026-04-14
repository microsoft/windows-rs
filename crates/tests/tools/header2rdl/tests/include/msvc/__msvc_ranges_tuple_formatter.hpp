// __msvc_ranges_tuple_formatter.hpp internal header

// Copyright (c) Microsoft Corporation.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// NOTE:
// The contents of this header are derived in part from libfmt under the following license:

// Copyright (c) 2012 - present, Victor Zverovich
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// --- Optional exception to the license ---
//
// As an exception, if, as a result of your compiling your source code, portions
// of this Software are embedded into a machine-executable object form of such
// source code, you may redistribute such embedded portions in such object form
// without including the above copyright and permission notices.

#ifndef __MSVC_RANGES_TUPLE_FORMATTER_HPP
#define __MSVC_RANGES_TUPLE_FORMATTER_HPP
#include <yvals_core.h>
#if _STL_COMPILER_PREPROCESSOR

#if !_HAS_CXX20
#error The contents of <format> are only available with C++20. (Also, you should not include this internal header.)
#endif // !_HAS_CXX20

#include <__msvc_formatter.hpp>
#include <__msvc_string_view.hpp>
#include <bit>
#include <iterator>
#include <stdexcept>
#include <xlocale>

#if _HAS_CXX23
#include <tuple>
#endif // _HAS_CXX23

#pragma pack(push, _CRT_PACKING)
#pragma warning(push, _STL_WARNING_LEVEL)
#pragma warning(disable : _STL_DISABLED_WARNINGS)
_STL_DISABLE_CLANG_WARNINGS
#pragma push_macro("new")
#undef new

_STD_BEGIN
_EXPORT_STD template <class, class>
class vector;

template <class _CharT>
_NODISCARD constexpr const _CharT* _Choose_literal(const char* const _Str, const wchar_t* const _WStr) noexcept {
    if constexpr (is_same_v<_CharT, char>) {
        return _Str;
    } else {
        return _WStr;
    }
}

// _STATICALLY_WIDEN is used by <chrono> since C++20 and by <format> since C++23.
// It's defined here, so that both headers can use this definition.
#define _STATICALLY_WIDEN(_CharT, _Literal) (_Choose_literal<_CharT>(_Literal, L##_Literal))

_EXPORT_STD class _NODISCARD format_error : public runtime_error {
    using runtime_error::runtime_error;
};

[[noreturn]] inline void _Throw_format_error(const char* const _Message) {
    _THROW(format_error{_Message});
}

inline void _You_see_this_error_because_arg_id_is_out_of_range() noexcept {}
inline void _Invalid_arg_type_for_dynamic_width_or_precision() noexcept {}

template <class _CharT>
class _Compile_time_parse_context;

_EXPORT_STD template <class _CharT>
class basic_format_parse_context {
public:
    using char_type      = _CharT;
    using const_iterator = basic_string_view<_CharT>::const_iterator;
    using iterator       = const_iterator;

    constexpr explicit basic_format_parse_context(
        const basic_string_view<_CharT> _Fmt, const size_t _Num_args_ = 0) noexcept
        : _Format_string(_Fmt), _Num_args(_Num_args_) {}

    basic_format_parse_context(const basic_format_parse_context&)            = delete;
    basic_format_parse_context& operator=(const basic_format_parse_context&) = delete;

    _NODISCARD constexpr const_iterator begin() const noexcept {
        return _Format_string.begin();
    }
    _NODISCARD constexpr const_iterator end() const noexcept {
        return _Format_string.end();
    }
    _NODISCARD constexpr const _CharT* _Unchecked_begin() const noexcept {
        return _Format_string._Unchecked_begin();
    }
    _NODISCARD constexpr const _CharT* _Unchecked_end() const noexcept {
        return _Format_string._Unchecked_end();
    }

    constexpr void advance_to(const const_iterator _It) {
        _Adl_verify_range(_It, _Format_string.end());
        _Adl_verify_range(_Format_string.begin(), _It);
        const auto _Diff = static_cast<size_t>(_It._Unwrapped() - _Format_string._Unchecked_begin());
        _Format_string.remove_prefix(_Diff);
    }

    // While the standard presents an exposition-only enum value for
    // the indexing mode (manual, automatic, or unknown) we use _Next_arg_id to indicate it.
    // _Next_arg_id > 0 means automatic
    // _Next_arg_id == 0 means unknown
    // _Next_arg_id < 0 means manual
    _NODISCARD constexpr size_t next_arg_id() {
        if (_Next_arg_id < 0) {
            _Throw_format_error("Can not switch from manual to automatic indexing");
        }

        if (_STD is_constant_evaluated()) {
            if (static_cast<size_t>(_Next_arg_id) >= _Num_args) {
                _You_see_this_error_because_arg_id_is_out_of_range();
            }
        }

        return static_cast<size_t>(_Next_arg_id++);
    }

    constexpr void check_arg_id(const size_t _Id) {
        if (_STD is_constant_evaluated()) {
            if (_Id >= _Num_args) {
                _You_see_this_error_because_arg_id_is_out_of_range();
            }
        }

        if (_Next_arg_id > 0) {
            _Throw_format_error("Can not switch from automatic to manual indexing");
        }
        _Next_arg_id = -1;
    }

    constexpr void _Check_dynamic_spec_integral(const size_t _Idx) noexcept {
        if (_STD is_constant_evaluated()) {
            // This downcast might seem UB-prone, but since it only happens at compile-time,
            // the compiler will produce an error if it is invalid.
            auto& _Ctx = static_cast<_Compile_time_parse_context<_CharT>&>(*this);

            _STL_INTERNAL_CHECK(_Ctx._Arg_type[_Idx] != _Basic_format_arg_type::_None);
            if (_Ctx._Arg_type[_Idx] > _Basic_format_arg_type::_ULong_long_type) {
                _Invalid_arg_type_for_dynamic_width_or_precision();
            }
        }
    }

private:
    basic_string_view<_CharT> _Format_string;
    size_t _Num_args;
    // The standard says this is size_t, however we use ptrdiff_t to save some space
    // by not having to store the indexing mode. Above is a more detailed explanation
    // of how this works.
    ptrdiff_t _Next_arg_id = 0;
};

template <class _CharT>
class _Compile_time_parse_context : public basic_format_parse_context<_CharT> {
    friend basic_format_parse_context<_CharT>;

public:
    constexpr _Compile_time_parse_context(const basic_string_view<_CharT> _Fmt, const size_t _Num_args,
        const _Basic_format_arg_type* const _Arg_type_) noexcept
        : basic_format_parse_context<_CharT>(_Fmt, _Num_args), _Arg_type(_Arg_type_) {}

private:
    const _Basic_format_arg_type* const _Arg_type;
};

_EXPORT_STD using format_parse_context  = basic_format_parse_context<char>;
_EXPORT_STD using wformat_parse_context = basic_format_parse_context<wchar_t>;

template <class _Ty, class _Context, class _Formatter = _Context::template formatter_type<remove_const_t<_Ty>>>
concept _Formattable_with = semiregular<_Formatter>
                         && requires(_Formatter& __f, const _Formatter& __cf, _Ty&& __t, _Context __fc,
                             basic_format_parse_context<typename _Context::char_type> __pc) {
                                { __f.parse(__pc) } -> same_as<typename decltype(__pc)::iterator>;
                                { __cf.format(__t, __fc) } -> same_as<typename _Context::iterator>;
                            };

template <class _Context>
struct _Format_arg_traits {
    using _Char_type = _Context::char_type;

    // Function template _Type_eraser mirrors the type dispatching mechanism in the construction of basic_format_arg
    // (N4950 [format.arg]). They determine the mapping from "raw" to "erased" argument type for _Format_arg_store.
    template <class _Ty>
    static auto _Type_eraser();

    template <class _Ty>
    using _Storage_type = decltype(_Type_eraser<remove_reference_t<_Ty>>());

    template <class _Ty>
    static constexpr size_t _Storage_size = sizeof(_Storage_type<_Ty>);
};

_EXPORT_STD template <class _Context>
class basic_format_args;

_FMT_P2286_BEGIN
template <class _CharT>
struct _Format_handler;
_FMT_P2286_END

_EXPORT_STD template <class _Context>
class basic_format_arg {
public:
    using _CharType = _Context::char_type;

    class handle {
    private:
        const void* _Ptr;
        void(__cdecl* _Format)(basic_format_parse_context<_CharType>& _Parse_ctx, _Context& _Format_ctx, const void*);

        template <class _Ty>
        explicit handle(_Ty& _Val) noexcept
            : _Ptr(_STD addressof(_Val)), _Format([](basic_format_parse_context<_CharType>& _Parse_ctx,
                                                      _Context& _Format_ctx, const void* _Ptr) _STATIC_LAMBDA {
                  using _Td = remove_const_t<_Ty>;
                  // doesn't drop const-qualifier per an unnumbered LWG issue
                  using _Tq = conditional_t<_Formattable_with<const _Ty, _Context>, const _Ty, _Ty>;
                  _STL_INTERNAL_STATIC_ASSERT(_Formattable_with<_Tq, _Context>);

                  typename _Context::template formatter_type<_Td> _Formatter;
                  _Parse_ctx.advance_to(_Formatter.parse(_Parse_ctx));
                  _Format_ctx.advance_to(
                      _Formatter.format(*const_cast<_Tq*>(static_cast<const _Td*>(_Ptr)), _Format_ctx));
              }) {}

    public:
        void format(basic_format_parse_context<_CharType>& _Parse_ctx, _Context& _Format_ctx) const {
            _Format(_Parse_ctx, _Format_ctx, _Ptr);
        }

        template <class _Ty>
        _NODISCARD static handle _Make_from(_Ty& _Val) noexcept {
            return handle{_Val};
        }
    };

#if defined(__clang__) || defined(__EDG__) \
    || defined(__CUDACC__) // TRANSITION, LLVM-81774 (Clang), VSO-1956558 (EDG), VSO-2411436 (needed by CUDA 12.8.1)
    basic_format_arg() noexcept : _Active_state(_Basic_format_arg_type::_None), _No_state() {}
#else // ^^^ workaround / no workaround vvv
    basic_format_arg() noexcept = default;
#endif // ^^^ no workaround ^^^

    explicit operator bool() const noexcept {
        return _Active_state != _Basic_format_arg_type::_None;
    }

    // Function template _Make_from mirrors the exposition-only single-argument constructor template of
    // basic_format_arg (N4950 [format.arg]).
    template <_Formattable_with<_Context> _Ty>
    _NODISCARD static basic_format_arg _Make_from(_Ty& _Val) noexcept {
        using _Erased_type = _Format_arg_traits<_Context>::template _Storage_type<_Ty>;
        if constexpr (is_same_v<remove_const_t<_Ty>, char> && is_same_v<_CharType, wchar_t>) {
            return basic_format_arg(static_cast<_Erased_type>(static_cast<unsigned char>(_Val)));
        }
#if !_HAS_CXX23
        else if constexpr (is_same_v<_Erased_type, basic_string_view<_CharType>>) {
            return basic_format_arg(_Erased_type{_Val.data(), _Val.size()});
        }
#endif // !_HAS_CXX23
        else {
            return basic_format_arg(static_cast<_Erased_type>(_Val));
        }
    }

    template <class _Visitor>
    decltype(auto) _Visit(_Visitor&& _Vis) {
        switch (_Active_state) {
        case _Basic_format_arg_type::_None:
            return _STD forward<_Visitor>(_Vis)(_No_state);
        case _Basic_format_arg_type::_Int_type:
            return _STD forward<_Visitor>(_Vis)(_Int_state);
        case _Basic_format_arg_type::_UInt_type:
            return _STD forward<_Visitor>(_Vis)(_UInt_state);
        case _Basic_format_arg_type::_Long_long_type:
            return _STD forward<_Visitor>(_Vis)(_Long_long_state);
        case _Basic_format_arg_type::_ULong_long_type:
            return _STD forward<_Visitor>(_Vis)(_ULong_long_state);
        case _Basic_format_arg_type::_Bool_type:
            return _STD forward<_Visitor>(_Vis)(_Bool_state);
        case _Basic_format_arg_type::_Char_type:
            return _STD forward<_Visitor>(_Vis)(_Char_state);
        case _Basic_format_arg_type::_Float_type:
            return _STD forward<_Visitor>(_Vis)(_Float_state);
        case _Basic_format_arg_type::_Double_type:
            return _STD forward<_Visitor>(_Vis)(_Double_state);
        case _Basic_format_arg_type::_Long_double_type:
            return _STD forward<_Visitor>(_Vis)(_Long_double_state);
        case _Basic_format_arg_type::_Pointer_type:
            return _STD forward<_Visitor>(_Vis)(_Pointer_state);
        case _Basic_format_arg_type::_CString_type:
            return _STD forward<_Visitor>(_Vis)(_CString_state);
        case _Basic_format_arg_type::_String_type:
            return _STD forward<_Visitor>(_Vis)(_String_state);
        case _Basic_format_arg_type::_Custom_type:
            return _STD forward<_Visitor>(_Vis)(_Custom_state);
        default:
            _STL_REPORT_ERROR("basic_format_arg contains an impossible type");
            int _Dummy{};
            return _STD forward<_Visitor>(_Vis)(_Dummy);
        }
    }

private:
    friend basic_format_args<_Context>;
    friend _Format_handler<_CharType>;
    friend _Format_arg_traits<_Context>;

    explicit basic_format_arg(const int _Val) noexcept
        : _Active_state(_Basic_format_arg_type::_Int_type), _Int_state(_Val) {}
    explicit basic_format_arg(const unsigned int _Val) noexcept
        : _Active_state(_Basic_format_arg_type::_UInt_type), _UInt_state(_Val) {}
    explicit basic_format_arg(const long long _Val) noexcept
        : _Active_state(_Basic_format_arg_type::_Long_long_type), _Long_long_state(_Val) {}
    explicit basic_format_arg(const unsigned long long _Val) noexcept
        : _Active_state(_Basic_format_arg_type::_ULong_long_type), _ULong_long_state(_Val) {}
    explicit basic_format_arg(const bool _Val) noexcept
        : _Active_state(_Basic_format_arg_type::_Bool_type), _Bool_state(_Val) {}
    explicit basic_format_arg(const _CharType _Val) noexcept
        : _Active_state(_Basic_format_arg_type::_Char_type), _Char_state(_Val) {}
    explicit basic_format_arg(const float _Val) noexcept
        : _Active_state(_Basic_format_arg_type::_Float_type), _Float_state(_Val) {}
    explicit basic_format_arg(const double _Val) noexcept
        : _Active_state(_Basic_format_arg_type::_Double_type), _Double_state(_Val) {}
    explicit basic_format_arg(const long double _Val) noexcept
        : _Active_state(_Basic_format_arg_type::_Long_double_type), _Long_double_state(_Val) {}
    explicit basic_format_arg(const void* _Val) noexcept
        : _Active_state(_Basic_format_arg_type::_Pointer_type), _Pointer_state(_Val) {}
    explicit basic_format_arg(const _CharType* _Val) noexcept
        : _Active_state(_Basic_format_arg_type::_CString_type), _CString_state(_Val) {}
    explicit basic_format_arg(const basic_string_view<_CharType> _Val) noexcept
        : _Active_state(_Basic_format_arg_type::_String_type), _String_state(_Val) {}
    explicit basic_format_arg(const handle _Val) noexcept
        : _Active_state(_Basic_format_arg_type::_Custom_type), _Custom_state(_Val) {}

    _Basic_format_arg_type _Active_state = _Basic_format_arg_type::_None;
    union {
        monostate _No_state = monostate{};
        int _Int_state;
        unsigned int _UInt_state;
        long long _Long_long_state;
        unsigned long long _ULong_long_state;
        bool _Bool_state;
        _CharType _Char_state;
        float _Float_state;
        double _Double_state;
        long double _Long_double_state;
        const void* _Pointer_state;
        const _CharType* _CString_state;
        basic_string_view<_CharType> _String_state;
        handle _Custom_state;
    };
};

template <class _Ty, class _CharT>
constexpr bool _Is_basic_string_like_for = false;

template <class _CharT, class _Traits, class _Alloc>
constexpr bool _Is_basic_string_like_for<basic_string<_CharT, _Traits, _Alloc>, _CharT> = true;

template <class _CharT, class _Traits>
constexpr bool _Is_basic_string_like_for<basic_string_view<_CharT, _Traits>, _CharT> = true;

template <class _Context>
template <class _Ty>
auto _Format_arg_traits<_Context>::_Type_eraser() {
    using _Td = remove_const_t<_Ty>;
    // See N4950 [format.arg]/6
    if constexpr (is_same_v<_Td, bool>) {
        return bool{};
    } else if constexpr (is_same_v<_Td, _Char_type>) {
        return _Char_type{};
    } else if constexpr (is_same_v<_Td, char> && is_same_v<_Char_type, wchar_t>) {
        return _Char_type{};
    } else if constexpr (signed_integral<_Td> && sizeof(_Td) <= sizeof(int)) {
        return int{};
    } else if constexpr (unsigned_integral<_Td> && sizeof(_Td) <= sizeof(unsigned int)) {
        return static_cast<unsigned int>(42);
    } else if constexpr (signed_integral<_Td> && sizeof(_Td) <= sizeof(long long)) {
        return static_cast<long long>(42);
    } else if constexpr (unsigned_integral<_Td> && sizeof(_Td) <= sizeof(unsigned long long)) {
        return static_cast<unsigned long long>(42);
    } else if constexpr (is_same_v<_Td, float>) {
        return float{};
    } else if constexpr (is_same_v<_Td, double>) {
        return double{};
    } else if constexpr (is_same_v<_Td, long double>) {
        return static_cast<long double>(42);
    } else if constexpr (_Is_basic_string_like_for<_Td, _Char_type>) {
        return basic_string_view<_Char_type>{};
    } else if constexpr (_Is_any_of_v<decay_t<_Td>, _Char_type*, const _Char_type*>) {
        return static_cast<const _Char_type*>(nullptr);
    } else if constexpr (is_void_v<remove_pointer_t<_Td>> || is_same_v<_Td, nullptr_t>) {
        return static_cast<const void*>(nullptr);
    } else {
        int _Dummy{};
        return basic_format_arg<_Context>::handle::_Make_from(_Dummy);
    }
}

template <class _Context, class _Ty>
_NODISCARD consteval _Basic_format_arg_type _Get_format_arg_type() noexcept {
    using _CharType    = _Context::char_type;
    using _Erased_type = _Format_arg_traits<_Context>::template _Storage_type<_Ty>;

    if constexpr (is_same_v<_Erased_type, bool>) {
        return _Basic_format_arg_type::_Bool_type;
    } else if constexpr (is_same_v<_Erased_type, _CharType>) {
        return _Basic_format_arg_type::_Char_type;
    } else if constexpr (is_same_v<_Erased_type, int>) {
        return _Basic_format_arg_type::_Int_type;
    } else if constexpr (is_same_v<_Erased_type, unsigned int>) {
        return _Basic_format_arg_type::_UInt_type;
    } else if constexpr (is_same_v<_Erased_type, long long>) {
        return _Basic_format_arg_type::_Long_long_type;
    } else if constexpr (is_same_v<_Erased_type, unsigned long long>) {
        return _Basic_format_arg_type::_ULong_long_type;
    } else if constexpr (is_same_v<_Erased_type, float>) {
        return _Basic_format_arg_type::_Float_type;
    } else if constexpr (is_same_v<_Erased_type, double>) {
        return _Basic_format_arg_type::_Double_type;
    } else if constexpr (is_same_v<_Erased_type, long double>) {
        return _Basic_format_arg_type::_Long_double_type;
    } else if constexpr (is_same_v<_Erased_type, const void*>) {
        return _Basic_format_arg_type::_Pointer_type;
    } else if constexpr (is_same_v<_Erased_type, const _CharType*>) {
        return _Basic_format_arg_type::_CString_type;
    } else if constexpr (is_same_v<_Erased_type, basic_string_view<_CharType>>) {
        return _Basic_format_arg_type::_String_type;
    } else {
        _STL_INTERNAL_STATIC_ASSERT(is_same_v<_Erased_type, typename basic_format_arg<_Context>::handle>);
        return _Basic_format_arg_type::_Custom_type;
    }
}

_EXPORT_STD template <class _Visitor, class _Context>
decltype(auto) visit_format_arg(_Visitor&& _Vis, basic_format_arg<_Context> _Arg) {
    return _Arg._Visit(_STD forward<_Visitor>(_Vis));
}

struct _Format_arg_index {
    // TRANSITION, Should be templated on number of arguments for even less storage

    constexpr _Format_arg_index() = default;
    constexpr explicit _Format_arg_index(const size_t _Index_) noexcept : _Index(_Index_) {
        _Type(_Basic_format_arg_type::_None);
    }

    _NODISCARD constexpr _Basic_format_arg_type _Type() const noexcept {
        return static_cast<_Basic_format_arg_type>(_Type_);
    }

    constexpr void _Type(_Basic_format_arg_type _Val) noexcept {
        _Type_ = static_cast<size_t>(_Val);
    }

    size_t _Index : (sizeof(size_t) * 8 - 4){};
    size_t _Type_ : 4 {};
};

template <class _Context, class... _Args>
class _Format_arg_store {
private:
    using _CharType = _Context::char_type;
    using _Traits   = _Format_arg_traits<_Context>;

    friend basic_format_args<_Context>;

    static constexpr size_t _Num_args       = sizeof...(_Args);
    static constexpr size_t _Storage_length = (_Traits::template _Storage_size<_Args> + ...);

    // The actual storage representation: _Num_args offsets into _Storage, followed immediately by the untyped
    // _Storage which holds copies of the object representations of arguments (with no regard for alignment).
    // These must be allocated consecutively, since basic_format_args thinks it can store a pointer to
    // _Index_array and use arithmetic to access the bytes of _Storage.
    _Format_arg_index _Index_array[_Num_args];
    unsigned char _Storage[_Storage_length];

#pragma warning(push)
#pragma warning(disable : 6386) // Buffer overrun while writing to '%s' ...
    template <class _Ty>
    void _Store_impl(
        const size_t _Arg_index, const _Basic_format_arg_type _Arg_type, const type_identity_t<_Ty>& _Val) noexcept {
        _STL_INTERNAL_CHECK(_Arg_index < _Num_args);

        const auto _Store_index = _Index_array[_Arg_index]._Index;

        _CSTD memcpy(_Storage + _Store_index, _STD addressof(_Val), sizeof(_Ty));
        _Index_array[_Arg_index]._Type(_Arg_type);
        if (_Arg_index + 1 < _Num_args) {
            // Set the starting index of the next arg, as that is dynamic, must be called with increasing index
            _Index_array[_Arg_index + 1] = _Format_arg_index{_Store_index + sizeof(_Ty)};
        }
    }
#pragma warning(pop)

    template <class _Ty>
    void _Store(const size_t _Arg_index, _Ty&& _Val) noexcept {
        constexpr _Basic_format_arg_type _Arg_type = _STD _Get_format_arg_type<_Context, _Ty>();
        using _Erased_type                         = _Traits::template _Storage_type<_Ty>;

        if constexpr (is_same_v<remove_const_t<remove_reference_t<_Ty>>, char> && is_same_v<_CharType, wchar_t>) {
            _Store_impl<_Erased_type>(
                _Arg_index, _Arg_type, static_cast<_Erased_type>(static_cast<unsigned char>(_Val)));
        } else if constexpr (is_same_v<_Erased_type, typename basic_format_arg<_Context>::handle>) {
            _Store_impl<_Erased_type>(_Arg_index, _Arg_type, _Erased_type::_Make_from(_Val));
        }
#if !_HAS_CXX23
        // Workaround towards N4950 [format.arg]/6.8 in C++20
        else if constexpr (is_same_v<_Erased_type, basic_string_view<_CharType>>) {
            _Store_impl<_Erased_type>(_Arg_index, _Arg_type, _Erased_type{_Val.data(), _Val.size()});
        }
#endif // !_HAS_CXX23
        else {
            _Store_impl<_Erased_type>(_Arg_index, _Arg_type, static_cast<_Erased_type>(_Val));
        }
    }

public:
    _Format_arg_store(_Args&... _Vals) noexcept {
        _Index_array[0]   = {};
        size_t _Arg_index = 0;
        (_Store(_Arg_index++, _Vals), ...);
    }
};

template <class _Context>
class _Format_arg_store<_Context> {};

_EXPORT_STD template <class _Context>
class basic_format_args {
public:
    basic_format_args(const _Format_arg_store<_Context>&) noexcept {}

    template <class... _Args>
    basic_format_args(const _Format_arg_store<_Context, _Args...>& _Store) noexcept
        : _Num_args(sizeof...(_Args)), _Index_array(_Store._Index_array) {}

    _NODISCARD basic_format_arg<_Context> get(const size_t _Index) const noexcept {
        if (_Index >= _Num_args) {
            return basic_format_arg<_Context>{};
        }

        using _CharType = _Context::char_type;
        // The explanatory comment in _Format_arg_store explains how the following works.
        const auto _Packed_index = _Index_array[_Index];
        const auto _Arg_storage =
            reinterpret_cast<const unsigned char*>(_Index_array + _Num_args) + _Packed_index._Index;

        switch (_Packed_index._Type()) {
        case _Basic_format_arg_type::_None:
        default:
            _STL_ASSERT(false, "Invalid basic_format_arg type");
            return basic_format_arg<_Context>{};
        case _Basic_format_arg_type::_Int_type:
            return basic_format_arg<_Context>{_Get_value_from_memory<int>(_Arg_storage)};
        case _Basic_format_arg_type::_UInt_type:
            return basic_format_arg<_Context>{_Get_value_from_memory<unsigned int>(_Arg_storage)};
        case _Basic_format_arg_type::_Long_long_type:
            return basic_format_arg<_Context>{_Get_value_from_memory<long long>(_Arg_storage)};
        case _Basic_format_arg_type::_ULong_long_type:
            return basic_format_arg<_Context>{_Get_value_from_memory<unsigned long long>(_Arg_storage)};
        case _Basic_format_arg_type::_Bool_type:
            return basic_format_arg<_Context>{_Get_value_from_memory<bool>(_Arg_storage)};
        case _Basic_format_arg_type::_Char_type:
            return basic_format_arg<_Context>{_Get_value_from_memory<_CharType>(_Arg_storage)};
        case _Basic_format_arg_type::_Float_type:
            return basic_format_arg<_Context>{_Get_value_from_memory<float>(_Arg_storage)};
        case _Basic_format_arg_type::_Double_type:
            return basic_format_arg<_Context>{_Get_value_from_memory<double>(_Arg_storage)};
        case _Basic_format_arg_type::_Long_double_type:
            return basic_format_arg<_Context>{_Get_value_from_memory<long double>(_Arg_storage)};
        case _Basic_format_arg_type::_Pointer_type:
            return basic_format_arg<_Context>{_Get_value_from_memory<const void*>(_Arg_storage)};
        case _Basic_format_arg_type::_CString_type:
            return basic_format_arg<_Context>{_Get_value_from_memory<const _CharType*>(_Arg_storage)};
        case _Basic_format_arg_type::_String_type:
            return basic_format_arg<_Context>{_Get_value_from_memory<basic_string_view<_CharType>>(_Arg_storage)};
        case _Basic_format_arg_type::_Custom_type:
            return basic_format_arg<_Context>{
                _Get_value_from_memory<typename basic_format_arg<_Context>::handle>(_Arg_storage)};
        }
    }

    _NODISCARD size_t _Estimate_required_capacity() const noexcept {
        using _CharType = _Context::char_type;
        size_t _Result  = 0;

        for (size_t _Idx = 0; _Idx < _Num_args; ++_Idx) {
            const auto _Packed_index = _Index_array[_Idx];
            const auto _Arg_type     = _Packed_index._Type();
            if (_Arg_type == _Basic_format_arg_type::_String_type) {
                const auto _Arg_storage =
                    reinterpret_cast<const unsigned char*>(_Index_array + _Num_args) + _Packed_index._Index;
                const auto _View = _Get_value_from_memory<basic_string_view<_CharType>>(_Arg_storage);
                _Result += _View.size();
            } else if (_Arg_type == _Basic_format_arg_type::_CString_type) {
                _Result += 32; // estimate for length of null-terminated strings
            } else {
                _Result += 8; // estimate for length of all other arguments
            }
        }
        return _Result;
    }

private:
    template <class _Ty>
    _NODISCARD static auto _Get_value_from_memory(const unsigned char* const _Val) noexcept {
        auto& _Temp = *reinterpret_cast<const unsigned char(*)[sizeof(_Ty)]>(_Val);
        return _STD bit_cast<_Ty>(_Temp);
    }

    size_t _Num_args                      = 0;
    const _Format_arg_index* _Index_array = nullptr;
};

template <class _Context, class... _Args>
basic_format_args(_Format_arg_store<_Context, _Args...>) -> basic_format_args<_Context>;

// _Lazy_locale is used instead of a std::locale so that the locale is only
// constructed when needed, and is never constructed if the format string does not
// contain locale-sensitive format specifiers. Note that this means that a new locale
// will be constructed for _every_ locale-sensitive format specifier in the format string,
// making that case slower than if we had stored a "real" locale in the basic_format_context.
class _Lazy_locale {
private:
    const locale* _Loc = nullptr;

public:
    _Lazy_locale() = default;

    explicit _Lazy_locale(const locale& _Loc_) : _Loc(&_Loc_) {}

    explicit _Lazy_locale(const locale&&) = delete;

    _NODISCARD locale _Get() const {
        if (_Loc) {
            return *_Loc;
        } else {
            return locale{};
        }
    }
};

_EXPORT_STD template <class _Out, class _CharT>
    requires output_iterator<_Out, const _CharT&>
class basic_format_context {
private:
    _Out _OutputIt;
    basic_format_args<basic_format_context> _Args;
    _Lazy_locale _Loc;

    constexpr basic_format_context(
        _Out&& _OutputIt_, const basic_format_args<basic_format_context>& _Ctx_args, const _Lazy_locale& _Loc_)
        : _OutputIt(_STD move(_OutputIt_)), _Args(_Ctx_args), _Loc(_Loc_) {}

    basic_format_context(const basic_format_context&)            = delete;
    basic_format_context& operator=(const basic_format_context&) = delete;

public:
    using iterator  = _Out;
    using char_type = _CharT;

    template <class _Ty>
    using formatter_type = formatter<_Ty, _CharT>;

    _NODISCARD basic_format_arg<basic_format_context> arg(size_t _Id) const noexcept {
        return _Args.get(_Id);
    }
    _NODISCARD locale locale() {
        return _Loc._Get();
    }
    _NODISCARD iterator out() {
        return _STD move(_OutputIt);
    }
    void advance_to(iterator _It) {
        _OutputIt = _STD move(_It);
    }

    _NODISCARD const basic_format_args<basic_format_context>& _Get_args() const noexcept {
        return _Args;
    }
    _NODISCARD _Lazy_locale _Get_lazy_locale() const {
        return _Loc;
    }

    _NODISCARD static constexpr basic_format_context _Make_from(
        _Out _OutputIt_, basic_format_args<basic_format_context> _Ctx_args, const _Lazy_locale& _Loc_) {
        return basic_format_context{_STD move(_OutputIt_), _Ctx_args, _Loc_};
    }
};

template <class _Ty>
class _Fmt_buffer {
private:
    _Ty* _Ptr_        = nullptr;
    size_t _Size_     = 0;
    size_t _Capacity_ = 0;

protected:
    explicit _Fmt_buffer(const size_t _Size) noexcept : _Size_(_Size), _Capacity_(_Size) {}

    ~_Fmt_buffer() = default;

    _Fmt_buffer(_Ty* _Data, const size_t _Size, const size_t _Capacity) noexcept
        : _Ptr_(_Data), _Size_(_Size), _Capacity_(_Capacity) {}

    void _Set(_Ty* _Buf_data, const size_t _Buf_capacity) noexcept {
        _Ptr_      = _Buf_data;
        _Capacity_ = _Buf_capacity;
    }

    virtual void _Grow(size_t _Capacity) = 0;

public:
    using value_type = _Ty;

    _Fmt_buffer(const _Fmt_buffer&)    = delete;
    void operator=(const _Fmt_buffer&) = delete;

    _NODISCARD _Ty* begin() noexcept {
        return _Ptr_;
    }

    _NODISCARD _Ty* end() noexcept {
        return _Ptr_ + _Size_;
    }

    _NODISCARD size_t _Size() const noexcept {
        return _Size_;
    }

    _NODISCARD size_t _Capacity() const noexcept {
        return _Capacity_;
    }

    void _Clear() noexcept {
        _Size_ = 0;
    }

    void _Try_reserve(const size_t _New_capacity) {
        if (_New_capacity > _Capacity_) {
            _Grow(_New_capacity);
        }
    }

    void push_back(const _Ty _Value) {
        _Try_reserve(_Size_ + 1);
        _Ptr_[_Size_++] = _Value;
    }
};

struct _Fmt_buffer_traits {
    explicit _Fmt_buffer_traits(ptrdiff_t) {}

    _NODISCARD size_t _Count() const noexcept {
        return 0;
    }

    _NODISCARD size_t _Limit(const size_t _Size) noexcept {
        return _Size;
    }
};

inline constexpr size_t _Fmt_buffer_size = 256;

template <class _OutputIt>
struct _Fmt_iterator_flush {
    template <class _Ty>
    static _OutputIt _Flush(const _Ty* const _First, const _Ty* const _Last, _OutputIt _Output) {
        return _STD copy(_First, _Last, _STD move(_Output));
    }
};

template <class _Container>
struct _Back_insert_iterator_container_access : back_insert_iterator<_Container> {
    explicit _Back_insert_iterator_container_access(back_insert_iterator<_Container> _Iter)
        : back_insert_iterator<_Container>(_Iter) {}

    using back_insert_iterator<_Container>::container;
};

template <class _Container>
    requires (_Is_specialization_v<_Container, basic_string> || _Is_specialization_v<_Container, vector>)
struct _Fmt_iterator_flush<back_insert_iterator<_Container>> {
    using _OutputIt = back_insert_iterator<_Container>;

    template <class _Ty>
    static _OutputIt _Flush(const _Ty* const _First, const _Ty* const _Last, _OutputIt _Output) {
        _Container& _Cont = *_Back_insert_iterator_container_access<_Container>{_Output}.container;
        _Cont.insert(_Cont.end(), _First, _Last);
        return _Output;
    }
};

template <class _OutputIt, class _Ty, class _Traits = _Fmt_buffer_traits>
class _Fmt_iterator_buffer final : public _Traits, public _Fmt_buffer<_Ty> {
private:
    _OutputIt _Output;
    _Ty _Data[_Fmt_buffer_size];

    void _Grow(size_t) final {
        if (this->_Size() == _Fmt_buffer_size) {
            _Flush();
        }
    }

    void _Flush() {
        auto _Size = this->_Size();
        this->_Clear();
        const auto _End = _Data + this->_Limit(_Size);

        _Output = _Fmt_iterator_flush<_OutputIt>::_Flush(_Data, _End, _STD move(_Output));
    }

public:
    explicit _Fmt_iterator_buffer(_OutputIt _Out, ptrdiff_t _Size = _Fmt_buffer_size)
        : _Traits(_Size), _Fmt_buffer<_Ty>(_Data, 0, _Fmt_buffer_size), _Output(_STD move(_Out)) {}

    ~_Fmt_iterator_buffer() {
        if (this->_Size() != 0) {
            _Flush();
        }
    }

    _NODISCARD _OutputIt _Out() {
        _Flush();
        return _STD move(_Output);
    }

    _NODISCARD ptrdiff_t _Count() const noexcept {
        return static_cast<ptrdiff_t>(_Traits::_Count() + this->_Size());
    }
};

template <class _Ty>
class _Fmt_iterator_buffer<_Ty*, _Ty> final : public _Fmt_buffer<_Ty> {
private:
    void _Grow(size_t) final {}

public:
    explicit _Fmt_iterator_buffer(_Ty* _Out, ptrdiff_t = 0) : _Fmt_buffer<_Ty>(_Out, 0, ~size_t{}) {}

    _NODISCARD _Ty* _Out() noexcept {
        return this->end();
    }
};

#if _HAS_CXX23
template <class _CharT>
struct _Phony_fmt_iter_for {
    using difference_type = ptrdiff_t;

    // These member functions are never defined:
    _CharT& operator*() const;
    _Phony_fmt_iter_for& operator++();
    _Phony_fmt_iter_for operator++(int);
};

_EXPORT_STD template <class _Ty, class _CharT>
concept formattable =
    _Formattable_with<remove_reference_t<_Ty>, basic_format_context<_Phony_fmt_iter_for<_CharT>, _CharT>>;

template <class _CharT>
struct _Range_specs : _Fill_align_and_width_specs<_CharT> {
    bool _No_brackets = false;
    char _Type        = '\0';
};

// TRANSITION, VSO-1236041: Avoid declaring and defining member functions in different headers.
template <class _Ty, class _CharT, class _ParseContext>
_NODISCARD constexpr _ParseContext::iterator _Range_formatter_parse(formatter<_Ty, _CharT>& _Underlying,
    basic_string_view<_CharT>& _Separator, basic_string_view<_CharT>& _Opening_bracket,
    basic_string_view<_CharT>& _Closing_bracket, _Range_specs<_CharT>& _Specs, _ParseContext& _Ctx);

template <class _Ty, class _CharT, _RANGES input_range _Range, class _FormatContext>
_NODISCARD _FormatContext::iterator _Range_formatter_format(const formatter<_Ty, _CharT>& _Underlying,
    basic_string_view<_CharT> _Separator, basic_string_view<_CharT> _Opening_bracket,
    basic_string_view<_CharT> _Closing_bracket, const _Range_specs<_CharT>& _Specs, _Range&& _Rng,
    _FormatContext& _Ctx);

_EXPORT_STD template <class _Ty, class _CharT = char>
    requires same_as<remove_cvref_t<_Ty>, _Ty> && formattable<_Ty, _CharT>
class range_formatter {
private:
    formatter<_Ty, _CharT> _Underlying;
    basic_string_view<_CharT> _Separator       = _STATICALLY_WIDEN(_CharT, ", ");
    basic_string_view<_CharT> _Opening_bracket = _STATICALLY_WIDEN(_CharT, "[");
    basic_string_view<_CharT> _Closing_bracket = _STATICALLY_WIDEN(_CharT, "]");
    _Range_specs<_CharT> _Specs;

public:
    constexpr void set_separator(basic_string_view<_CharT> _Sep) noexcept {
        _Separator = _Sep;
    }

    constexpr void set_brackets(basic_string_view<_CharT> _Opening, basic_string_view<_CharT> _Closing) noexcept {
        _Opening_bracket = _Opening;
        _Closing_bracket = _Closing;
    }

    _NODISCARD constexpr formatter<_Ty, _CharT>& underlying() noexcept {
        return _Underlying;
    }

    _NODISCARD constexpr const formatter<_Ty, _CharT>& underlying() const noexcept {
        return _Underlying;
    }

    template <class _ParseContext>
    constexpr _ParseContext::iterator parse(_ParseContext& _Ctx) {
        return _STD _Range_formatter_parse(_Underlying, _Separator, _Opening_bracket, _Closing_bracket, _Specs, _Ctx);
    }

    template <_RANGES input_range _Range, class _FormatContext>
        requires formattable<_RANGES range_reference_t<_Range>, _CharT>
              && same_as<remove_cvref_t<_RANGES range_reference_t<_Range>>, _Ty>
    _FormatContext::iterator format(_Range&& _Rng, _FormatContext& _Ctx) const {
        return _Format(_STD forward<_Range>(_Rng), _Ctx);
    }

private:
    template <_RANGES input_range _Range, class _FormatContext>
    _FormatContext::iterator _Format(_Range&&, _FormatContext&) const {
        _Throw_format_error("Unsupported 'basic_format_context'.");
    }

    template <_RANGES input_range _Range, class _FormatContext>
        requires _Is_specialization_v<typename _FormatContext::iterator, back_insert_iterator>
              && derived_from<typename _FormatContext::iterator::container_type, _Fmt_buffer<_CharT>>
    _FormatContext::iterator _Format(_Range&& _Rng, _FormatContext& _Ctx) const {
        return _STD _Range_formatter_format(
            _Underlying, _Separator, _Opening_bracket, _Closing_bracket, _Specs, _STD forward<_Range>(_Rng), _Ctx);
    }
};

template <class _Rng, class _CharT>
concept _Const_formattable_range =
    _RANGES input_range<const _Rng> && formattable<_RANGES range_reference_t<const _Rng>, _CharT>;

template <class _Rng, class _CharT>
using _Fmt_maybe_const = conditional_t<_Const_formattable_range<_Rng, _CharT>, const _Rng, _Rng>;

template <range_format _Kind, _RANGES input_range _Rng, class _CharT>
struct _Range_default_formatter;

template <_RANGES input_range _Rng, class _CharT>
struct _Range_default_formatter<range_format::sequence, _Rng, _CharT> {
private:
    using _Range_type = _Fmt_maybe_const<_Rng, _CharT>;

    range_formatter<remove_cvref_t<_RANGES range_reference_t<_Range_type>>, _CharT> _Underlying;

public:
    constexpr void set_separator(const basic_string_view<_CharT> _Sep) noexcept {
        _Underlying.set_separator(_Sep);
    }

    constexpr void set_brackets(
        const basic_string_view<_CharT> _Opening, const basic_string_view<_CharT> _Closing) noexcept {
        _Underlying.set_brackets(_Opening, _Closing);
    }

    template <class _ParseContext>
    constexpr _ParseContext::iterator parse(_ParseContext& _Ctx) {
        return _Underlying.parse(_Ctx);
    }

    template <class _FormatContext>
    _FormatContext::iterator format(_Range_type& _Elems, _FormatContext& _Ctx) const {
        return _Underlying.format(_Elems, _Ctx);
    }
};

template <_RANGES input_range _Rng, class _CharT>
struct _Range_default_formatter<range_format::map, _Rng, _CharT> {
private:
    using _Map_type     = _Fmt_maybe_const<_Rng, _CharT>;
    using _Element_type = remove_cvref_t<_RANGES range_reference_t<_Map_type>>;

    range_formatter<_Element_type, _CharT> _Underlying;

public:
    constexpr _Range_default_formatter()
        noexcept(is_nothrow_default_constructible_v<range_formatter<_Element_type, _CharT>>) /* strengthened */ {
        static_assert(_Is_two_tuple<_Element_type>, "the element type of the formatted range must be either pair<T, U> "
                                                    "or tuple<T, U> (N4981 [format.range.fmtmap]/1)");

        _Underlying.set_brackets(_STATICALLY_WIDEN(_CharT, "{"), _STATICALLY_WIDEN(_CharT, "}"));
        _Underlying.underlying().set_brackets({}, {});
        _Underlying.underlying().set_separator(_STATICALLY_WIDEN(_CharT, ": "));
    }

    template <class _ParseContext>
    constexpr _ParseContext::iterator parse(_ParseContext& _Ctx) {
        return _Underlying.parse(_Ctx);
    }

    template <class _FormatContext>
    _FormatContext::iterator format(_Map_type& _Rx, _FormatContext& _Ctx) const {
        return _Underlying.format(_Rx, _Ctx);
    }
};

template <_RANGES input_range _Rng, class _CharT>
struct _Range_default_formatter<range_format::set, _Rng, _CharT> {
private:
    using _Set_type = _Fmt_maybe_const<_Rng, _CharT>;
    range_formatter<remove_cvref_t<_RANGES range_reference_t<_Set_type>>, _CharT> _Underlying;

public:
    constexpr _Range_default_formatter() noexcept(is_nothrow_default_constructible_v<
        range_formatter<remove_cvref_t<_RANGES range_reference_t<_Set_type>>, _CharT>>) /* strengthened */ {
        _Underlying.set_brackets(_STATICALLY_WIDEN(_CharT, "{"), _STATICALLY_WIDEN(_CharT, "}"));
    }

    template <class _ParseContext>
    constexpr _ParseContext::iterator parse(_ParseContext& _Ctx) {
        return _Underlying.parse(_Ctx);
    }

    template <class _FormatContext>
    _FormatContext::iterator format(_Set_type& _Rx, _FormatContext& _Ctx) const {
        return _Underlying.format(_Rx, _Ctx);
    }
};

template <range_format _Kind, _RANGES input_range _Rng, class _CharT>
    requires (_Kind == range_format::string || _Kind == range_format::debug_string)
struct _Range_default_formatter<_Kind, _Rng, _CharT> {
private:
    static_assert(is_same_v<remove_cvref_t<_RANGES range_reference_t<_Rng>>, _CharT>,
        "the element type of the formatted range must be the character type used in formatting "
        "(N4981 [format.range.fmtstr]/1)");

    using _Range_type = _Maybe_const<_RANGES input_range<const _Rng>, _Rng>;

    formatter<basic_string_view<_CharT>, _CharT> _Underlying; // avoid copying the string if possible

public:
    template <class _ParseContext>
    constexpr _ParseContext::iterator parse(_ParseContext& _Ctx) {
        auto _Iter = _Underlying.parse(_Ctx);
        if constexpr (_Kind == range_format::debug_string) {
            _Underlying.set_debug_format();
        }
        return _Iter;
    }

    template <class _FormatContext>
    _FormatContext::iterator format(_Range_type& _Rx, _FormatContext& _Ctx) const {
        if constexpr (_RANGES contiguous_range<_Range_type>) {
            const auto _Size = _STD _To_unsigned_like(_RANGES distance(_Rx));

            if (!_STD in_range<size_t>(_Size)) {
                _Throw_format_error("Formatted range is too long.");
            }

            const basic_string_view<_CharT> _Str(_STD to_address(_RANGES begin(_Rx)), static_cast<size_t>(_Size));
            return _Underlying.format(_Str, _Ctx);
        } else {
            return _Underlying.format(basic_string<_CharT>{from_range, _Rx}, _Ctx);
        }
    }
};

// the deleted default constructor makes it "disabled" as per N4981 [format.formatter.spec]/5

template <_RANGES input_range _Rng, _Format_supported_charT _CharT>
    requires _Formatting_enabled_range<_Rng>
struct formatter<_Rng, _CharT> {
    formatter()                            = delete;
    formatter(const formatter&)            = delete;
    formatter& operator=(const formatter&) = delete;
};

template <_RANGES input_range _Rng, _Format_supported_charT _CharT>
    requires _Formatting_enabled_range<_Rng> && formattable<_RANGES range_reference_t<_Rng>, _CharT>
struct formatter<_Rng, _CharT> : _Range_default_formatter<format_kind<_Rng>, _Rng, _CharT> {};

// Per LWG-3997, `_CharT` in library-provided `formatter` specializations is
// constrained to character types supported by `format`.
template <class _AdaptorType, _Format_supported_charT _CharT, template <class> class _RefView>
struct _Adaptor_formatter_base {
private:
    using _Container             = _AdaptorType::container_type;
    using _Maybe_const_container = _Fmt_maybe_const<_Container, _CharT>;
    using _Maybe_const_adaptor   = _Maybe_const<is_const_v<_Maybe_const_container>, _AdaptorType>;

    formatter<_RefView<_Maybe_const_container>, _CharT> _Underlying;

public:
    template <class _ParseCtx>
    constexpr _ParseCtx::iterator parse(_ParseCtx& _Ctx) {
        return _Underlying.parse(_Ctx);
    }

    template <class _FmtCtx>
    _FmtCtx::iterator format(_Maybe_const_adaptor& _Adaptor, _FmtCtx& _Ctx) const {
        struct _Container_exposer : _AdaptorType {
            using _AdaptorType::c;

            _Container_exposer(const _Container_exposer&)            = delete;
            _Container_exposer& operator=(const _Container_exposer&) = delete;
            ~_Container_exposer()                                    = delete;
        };

        constexpr auto _Mem_cont_ptr = &_Container_exposer::c;
        return _Underlying.format(_Adaptor.*_Mem_cont_ptr, _Ctx);
    }
};

// TRANSITION, VSO-1236041: Avoid declaring and defining member functions in different headers.
template <class... _Types, class _CharT, class _ParseContext>
_NODISCARD constexpr _ParseContext::iterator _Tuple_formatter_parse(tuple<formatter<_Types, _CharT>...>& _Underlying,
    basic_string_view<_CharT>& _Separator, basic_string_view<_CharT>& _Opening_bracket,
    basic_string_view<_CharT>& _Closing_bracket, _Fill_align_and_width_specs<_CharT>& _Specs, _ParseContext& _Ctx);

template <class... _Types, class _CharT, class _FormatContext, class... _ArgTypes>
_NODISCARD _FormatContext::iterator _Tuple_formatter_format(const tuple<formatter<_Types, _CharT>...>& _Underlying,
    basic_string_view<_CharT> _Separator, basic_string_view<_CharT> _Opening_bracket,
    basic_string_view<_CharT> _Closing_bracket, const _Fill_align_and_width_specs<_CharT>& _Specs,
    _FormatContext& _Fmt_ctx, _ArgTypes&... _Args);

template <class _CharT, formattable<_CharT>... _Types>
class _Tuple_formatter_common_base {
private:
    tuple<formatter<remove_cvref_t<_Types>, _CharT>...> _Underlying;
    basic_string_view<_CharT> _Separator       = _STATICALLY_WIDEN(_CharT, ", ");
    basic_string_view<_CharT> _Opening_bracket = _STATICALLY_WIDEN(_CharT, "(");
    basic_string_view<_CharT> _Closing_bracket = _STATICALLY_WIDEN(_CharT, ")");

    _Fill_align_and_width_specs<_CharT> _Specs;

protected:
    static constexpr bool _Is_const_formattable = (formattable<const _Types, _CharT> && ...);

    template <class _FormatContext, class... _ArgTypes>
    _FormatContext::iterator _Format(_FormatContext& _Fmt_ctx, _ArgTypes&... _Args) const {
        _STL_INTERNAL_STATIC_ASSERT(
            (is_same_v<_ArgTypes, remove_reference_t<_Maybe_const<_Is_const_formattable, _Types>>> && ...));

        return _STD _Tuple_formatter_format(
            _Underlying, _Separator, _Opening_bracket, _Closing_bracket, _Specs, _Fmt_ctx, _Args...);
    }

public:
    constexpr void set_separator(const basic_string_view<_CharT> _Sep) noexcept {
        _Separator = _Sep;
    }

    constexpr void set_brackets(
        const basic_string_view<_CharT> _Opening, const basic_string_view<_CharT> _Closing) noexcept {
        _Opening_bracket = _Opening;
        _Closing_bracket = _Closing;
    }

    template <class _ParseContext>
    constexpr _ParseContext::iterator parse(_ParseContext& _Ctx) {
        return _STD _Tuple_formatter_parse(_Underlying, _Separator, _Opening_bracket, _Closing_bracket, _Specs, _Ctx);
    }
};

// formatter definition for all pairs and tuples, the deleted default constructor
// makes it "disabled" as per N4971 [format.formatter.spec]/5

template <class _TupleOrPair, class _CharT>
struct _Tuple_formatter_base {
    _Tuple_formatter_base()                                        = delete;
    _Tuple_formatter_base(const _Tuple_formatter_base&)            = delete;
    _Tuple_formatter_base& operator=(const _Tuple_formatter_base&) = delete;
};

template <class _CharT, formattable<_CharT> _Ty1, formattable<_CharT> _Ty2>
struct _Tuple_formatter_base<pair<_Ty1, _Ty2>, _CharT> : _Tuple_formatter_common_base<_CharT, _Ty1, _Ty2> {
private:
    using _Base           = _Tuple_formatter_common_base<_CharT, _Ty1, _Ty2>;
    using _Formatted_type = _Maybe_const<_Base::_Is_const_formattable, pair<_Ty1, _Ty2>>;

public:
    template <class _FormatContext>
    _FormatContext::iterator format(_Formatted_type& _Elems, _FormatContext& _Ctx) const {
        return this->_Format(_Ctx, _Elems.first, _Elems.second);
    }
};

template <class _CharT, formattable<_CharT>... _Types>
struct _Tuple_formatter_base<tuple<_Types...>, _CharT> : _Tuple_formatter_common_base<_CharT, _Types...> {
private:
    using _Base           = _Tuple_formatter_common_base<_CharT, _Types...>;
    using _Formatted_type = _Maybe_const<_Base::_Is_const_formattable, tuple<_Types...>>;

public:
    template <class _FormatContext>
    _FormatContext::iterator format(_Formatted_type& _Elems, _FormatContext& _Ctx) const {
        return _STD apply([this, &_Ctx](auto&... _Args) { return this->_Format(_Ctx, _Args...); }, _Elems);
    }
};

// specializations for tuple-like types that are input ranges and not formattable as tuples

template <_Format_supported_charT _CharT, class _Ty1, class _Ty2>
    requires (!formattable<_Ty1, _CharT> || !formattable<_Ty2, _CharT>)
          // TRANSITION, clang-format, () should be redundant
          && (_RANGES input_range<pair<_Ty1, _Ty2>>) && _Formatting_enabled_range<pair<_Ty1, _Ty2>>
          && formattable<_RANGES range_reference_t<pair<_Ty1, _Ty2>>, _CharT>
struct _Tuple_formatter_base<pair<_Ty1, _Ty2>, _CharT>
    : _Range_default_formatter<format_kind<pair<_Ty1, _Ty2>>, pair<_Ty1, _Ty2>, _CharT> {};

template <_Format_supported_charT _CharT, class... _Types>
    requires ((!formattable<_Types, _CharT>) || ...)
          // TRANSITION, clang-format, () should be redundant
          && (_RANGES input_range<tuple<_Types...>>) && _Formatting_enabled_range<tuple<_Types...>>
          && formattable<_RANGES range_reference_t<tuple<_Types...>>, _CharT>
struct _Tuple_formatter_base<tuple<_Types...>, _CharT>
    : _Range_default_formatter<format_kind<tuple<_Types...>>, tuple<_Types...>, _CharT> {};

// Per LWG-3997, `_CharT` in library-provided `formatter` specializations is
// constrained to character types supported by `format`.

template <_Format_supported_charT _CharT, class _Ty1, class _Ty2>
struct formatter<pair<_Ty1, _Ty2>, _CharT> : _Tuple_formatter_base<pair<_Ty1, _Ty2>, _CharT> {};

template <_Format_supported_charT _CharT, class... _Types>
struct formatter<tuple<_Types...>, _CharT> : _Tuple_formatter_base<tuple<_Types...>, _CharT> {};
#endif // _HAS_CXX23
_STD_END

#pragma pop_macro("new")
_STL_RESTORE_CLANG_WARNINGS
#pragma warning(pop)
#pragma pack(pop)

#endif // _STL_COMPILER_PREPROCESSOR
#endif // __MSVC_RANGES_TUPLE_FORMATTER_HPP
