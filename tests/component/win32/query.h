#pragma once

#include <Unknwn.h>
#include <stdint.h>

extern "C"
{
    HRESULT __stdcall Query(GUID const& iid, _COM_Outptr_ void** object) noexcept;
    HRESULT __stdcall QueryWithValue(int32_t value, GUID const& iid, _COM_Outptr_ void** object) noexcept;
    HRESULT __stdcall QueryOptionalWithValue(int32_t value, GUID const& iid, _COM_Outptr_opt_ void** object) noexcept;
    HRESULT __stdcall QueryOptionalWithConvertible(IUnknown* value, GUID const& iid, _COM_Outptr_opt_ void** object) noexcept;

    struct DECLSPEC_UUID("02a527b9-1c51-44a0-8390-51b0324a1c88") DECLSPEC_NOVTABLE
        IQueryInt32 : IUnknown
    {
        virtual int32_t __stdcall GetInt32() noexcept = 0;

        virtual HRESULT __stdcall Query(GUID const& iid, _COM_Outptr_ void** object) noexcept = 0;
        virtual HRESULT __stdcall QueryWithValue(int32_t value, GUID const& iid, _COM_Outptr_ void** object) noexcept = 0;
        virtual HRESULT __stdcall QueryOptionalWithValue(int32_t value, GUID const& iid, _COM_Outptr_opt_ void** object) noexcept = 0;
        virtual HRESULT __stdcall QueryOptionalWithConvertible(IUnknown* value, GUID const& iid, _COM_Outptr_opt_ void** object) noexcept = 0;
    };

    struct DECLSPEC_UUID("c8fd2084-5150-4de5-afba-6c9a04544b32") DECLSPEC_NOVTABLE
        IQuerySingle : IUnknown
    {
        virtual float __stdcall GetSingle() noexcept = 0;
    };
}
