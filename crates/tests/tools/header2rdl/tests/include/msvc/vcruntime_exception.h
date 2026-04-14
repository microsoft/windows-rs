//
// vcruntime_exception.h
//
//      Copyright (c) Microsoft Corporation. All rights reserved.
//
// <exception> functionality that is implemented in the VCRuntime.
//
#pragma once

#include <eh.h>

#ifdef _M_CEE_PURE
    #include <vcruntime_new.h>
#endif

#if _VCRT_COMPILER_PREPROCESSOR && _HAS_EXCEPTIONS

#pragma warning(push)
#pragma warning(disable: _VCRUNTIME_DISABLED_WARNINGS)
#pragma warning(disable: 4577) // 'noexcept' used with no exception handling mode specified
#pragma warning(disable: 4643) // Forward declaring 'meow' in namespace std is not permitted by the C++ Standard.

#pragma pack(push, _CRT_PACKING)

_CRT_BEGIN_C_HEADER

struct __std_exception_data
{
    char const* _What;
    bool        _DoFree;
};

_VCRTIMP void __cdecl __std_exception_copy(
    _In_    __std_exception_data const* _From,
    _Inout_ __std_exception_data*       _To
    );

_VCRTIMP void __cdecl __std_exception_destroy(
    _Inout_ __std_exception_data* _Data
    );

_CRT_END_C_HEADER


extern "C++" {

namespace std {

_VCRT_EXPORT_STD class _NODISCARD exception
{
public:

    exception() noexcept
        : _Data()
    {
    }

    explicit exception(char const* const _Message) noexcept
        : _Data()
    {
        __std_exception_data _InitData = { _Message, true };
        __std_exception_copy(&_InitData, &_Data);
    }

    exception(char const* const _Message, int) noexcept
        : _Data()
    {
        _Data._What = _Message;
    }

    exception(exception const& _Other) noexcept
        : _Data()
    {
        __std_exception_copy(&_Other._Data, &_Data);
    }

    exception& operator=(exception const& _Other) noexcept
    {
        if (this == &_Other)
        {
            return *this;
        }

        __std_exception_destroy(&_Data);
        __std_exception_copy(&_Other._Data, &_Data);
        return *this;
    }

    virtual ~exception() noexcept
    {
        __std_exception_destroy(&_Data);
    }

    _NODISCARD virtual char const* what() const
    {
        return _Data._What ? _Data._What : "Unknown exception";
    }

private:

    __std_exception_data _Data;
};

_VCRT_EXPORT_STD class _NODISCARD bad_exception
    : public exception
{
public:

    bad_exception() noexcept
        : exception("bad exception", 1)
    {
    }
};

_VCRT_EXPORT_STD class _NODISCARD bad_array_new_length;

_VCRT_EXPORT_STD class _NODISCARD bad_alloc
    : public exception
{
public:

    bad_alloc() noexcept
        : exception("bad allocation", 1)
    {
    }

private:

    friend bad_array_new_length;

    bad_alloc(char const* const _Message) noexcept
        : exception(_Message, 1)
    {
    }
};

_VCRT_EXPORT_STD class _NODISCARD bad_array_new_length
    : public bad_alloc
{
public:

    bad_array_new_length() noexcept
        : bad_alloc("bad array new length")
    {
    }
};

} // namespace std

} // extern "C++"

#pragma pack(pop)

#pragma warning(pop) // _VCRUNTIME_DISABLED_WARNINGS
#endif // _VCRT_COMPILER_PREPROCESSOR && _HAS_EXCEPTIONS
