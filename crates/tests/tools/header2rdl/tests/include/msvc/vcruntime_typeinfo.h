//
// vcruntime_typeinfo.h
//
//      Copyright (c) Microsoft Corporation. All rights reserved.
//
// <typeinfo> functionality that is implemented in the VCRuntime.
//
#pragma once

#include <vcruntime.h>

#if _VCRT_COMPILER_PREPROCESSOR
#include <vcruntime_exception.h>

#pragma warning(push)
#pragma warning(disable: _VCRUNTIME_DISABLED_WARNINGS)

#pragma pack(push, _CRT_PACKING)

extern "C++" { // attach declarations to the global module, see N4910 [module.unit]/7

#if defined _M_CEE_MIXED && !defined _VCRT_BUILD
    // Provide a fake definition of __type_info_node to suppress linker warning
    // LNK4248: unresolved typeref token for '__type_info_node'; image may not run.
    struct __type_info_node { };
#else
    struct __type_info_node;
#endif

#ifdef _M_CEE_PURE
    extern System::IntPtr __type_info_root_node;
#else
    extern __type_info_node __type_info_root_node;
#endif

} // extern "C++"


_CRT_BEGIN_C_HEADER

struct __std_type_info_data
{
    const char * _UndecoratedName;
    const char   _DecoratedName[1];
    __std_type_info_data() = delete;
    __std_type_info_data(const __std_type_info_data&) = delete;
    __std_type_info_data(__std_type_info_data&&) = delete;

    __std_type_info_data& operator=(const __std_type_info_data&) = delete;
    __std_type_info_data& operator=(__std_type_info_data&&) = delete;
};

_VCRTIMP int __cdecl __std_type_info_compare(
    _In_ const __std_type_info_data* _Lhs,
    _In_ const __std_type_info_data* _Rhs
    );

_VCRTIMP size_t __cdecl __std_type_info_hash(
    _In_ const __std_type_info_data* _Data
    );

_VCRTIMP const char* __cdecl __std_type_info_name(
    _Inout_ __std_type_info_data* _Data,
    _Inout_ __type_info_node*     _RootNode
    );

_CRT_END_C_HEADER


extern "C++" { // attach declarations to the global module, see N4910 [module.unit]/7

#pragma warning(push)
#pragma warning(disable: 4577) // 'noexcept' used with no exception handling mode specified
_VCRT_EXPORT_STD class type_info // Exported because for typeid, MSVC looks for type_info in the global namespace
{
public:

    type_info(const type_info&) = delete;
    type_info& operator=(const type_info&) = delete;

    _NODISCARD size_t hash_code() const noexcept
    {
        return __std_type_info_hash(&_Data);
    }

    _NODISCARD
#if _HAS_CXX23
    constexpr
#endif // _HAS_CXX23
    bool operator==(const type_info& _Other) const noexcept
    {
#if _HAS_CXX23
        if (__builtin_is_constant_evaluated())
        {
            return &_Data == &_Other._Data;
        }
#endif // _HAS_CXX23

        return __std_type_info_compare(&_Data, &_Other._Data) == 0;
    }

#if !_HAS_CXX20
    _NODISCARD bool operator!=(const type_info& _Other) const noexcept
    {
        return __std_type_info_compare(&_Data, &_Other._Data) != 0;
    }
#endif // !_HAS_CXX20

    _NODISCARD bool before(const type_info& _Other) const noexcept
    {
        return __std_type_info_compare(&_Data, &_Other._Data) < 0;
    }

    _NODISCARD const char* name() const noexcept
    {
        #ifdef _M_CEE_PURE
        return __std_type_info_name(&_Data, static_cast<__type_info_node*>(__type_info_root_node.ToPointer()));
        #else
        return __std_type_info_name(&_Data, &__type_info_root_node);
        #endif
    }

    _NODISCARD const char* raw_name() const noexcept
    {
        return _Data._DecoratedName;
    }

    virtual ~type_info() noexcept;

private:

    mutable __std_type_info_data _Data;
};
#pragma warning(pop)

namespace std {
    _VCRT_EXPORT_STD using ::type_info;
}

#if _HAS_EXCEPTIONS

namespace std {

#pragma warning(push)
#pragma warning(disable: 4577) // 'noexcept' used with no exception handling mode specified
_VCRT_EXPORT_STD class _NODISCARD bad_cast
    : public exception
{
public:

    bad_cast() noexcept
        : exception("bad cast", 1)
    {
    }

    static bad_cast __construct_from_string_literal(const char* const _Message) noexcept
    {
        return bad_cast(_Message, 1);
    }

private:

    bad_cast(const char* const _Message, int) noexcept
        : exception(_Message, 1)
    {
    }
};

_VCRT_EXPORT_STD class _NODISCARD bad_typeid
    : public exception
{
public:

    bad_typeid() noexcept
        : exception("bad typeid", 1)
    {
    }

    static bad_typeid __construct_from_string_literal(const char* const _Message) noexcept
    {
        return bad_typeid(_Message, 1);
    }

private:

    friend class __non_rtti_object;

    bad_typeid(const char* const _Message, int) noexcept
        : exception(_Message, 1)
    {
    }
};

class _NODISCARD __non_rtti_object
    : public bad_typeid
{
public:

    static __non_rtti_object __construct_from_string_literal(const char* const _Message) noexcept
    {
        return __non_rtti_object(_Message, 1);
    }

private:

    __non_rtti_object(const char* const _Message, int) noexcept
        : bad_typeid(_Message, 1)
    {
    }
};

#pragma warning(pop)
} // namespace std

#endif // _HAS_EXCEPTIONS

} // extern "C++"

#pragma pack(pop)
#pragma warning(pop) // _VCRUNTIME_DISABLED_WARNINGS
#endif // _VCRT_COMPILER_PREPROCESSOR
