#pragma once

#include <Unknwn.h>
#include <stdint.h>

extern "C"
{
    struct DECLSPEC_UUID("b3544ab4-e61f-4a2d-8f24-a051e92fbd25") DECLSPEC_NOVTABLE
        IMethodNames : IUnknown
    {
        virtual HRESULT __stdcall Method() noexcept = 0;
        virtual HRESULT __stdcall get_Property(_Outptr_ int32_t* value) noexcept = 0;
        virtual HRESULT __stdcall put_Property(int32_t value) noexcept = 0;
    };
}
