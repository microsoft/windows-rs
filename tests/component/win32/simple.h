#pragma once

#include <Unknwn.h>
#include <stdint.h>

extern "C"
{
    // {7FE2C3B5-4EF1-4B35-889D-03BA46CDD1EF}
    GUID const SimpleConstant{ 0x7fe2c3b5, 0x4ef1, 0x4b35, 0x88, 0x9d, 0x3, 0xba, 0x46, 0xcd, 0xd1, 0xef };

    void __stdcall SimpleFunction() noexcept;

    struct DECLSPEC_UUID("D42F6D29-1773-47DD-AA08-7EBCBDA907B1") DECLSPEC_NOVTABLE
    ISimpleInterface : IUnknown
    {
        virtual HRESULT __stdcall Method() = 0;
    };

    struct SimpleStruct
    {
        int32_t First;
        int32_t Second;
    };

    typedef void(__stdcall* SimpleCallback)() noexcept;

    enum SimpleEnum
    {
        First,
        Second
    };
}
