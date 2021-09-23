#pragma once

#include <Unknwn.h>
#include <stdint.h>

extern "C"
{
    struct ReturnType
    {
        int64_t a;
        int64_t b;
        int64_t c;
    };

    struct DECLSPEC_UUID("17dacbc7-146b-4194-ae95-705e550b9e70") DECLSPEC_NOVTABLE
        IReturn : IUnknown
    {
        virtual int64_t __stdcall ReturnValue() noexcept = 0;
        virtual ReturnType __stdcall ReturnStruct() noexcept = 0;
    };

    int64_t __stdcall ReturnValue() noexcept;
    ReturnType __stdcall ReturnStruct() noexcept;
    HRESULT __stdcall CreateReturn(IReturn** object) noexcept;
}
