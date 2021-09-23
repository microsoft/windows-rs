#pragma once

#include <Unknwn.h>
#include <stdint.h>
#include <bcrypt.h>

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
        virtual void __stdcall ReturnVoid(int64_t* check) noexcept = 0;
        virtual HRESULT __stdcall ReturnHresult(uint32_t code) noexcept = 0;
        virtual NTSTATUS __stdcall ReturnNtstatus(uint32_t code) noexcept = 0;
    };

    int64_t __stdcall ReturnValue() noexcept;
    ReturnType __stdcall ReturnStruct() noexcept;
    HRESULT __stdcall CreateReturn(IReturn** object) noexcept;
    void __stdcall ReturnVoid(int64_t* check) noexcept;
    HRESULT __stdcall ReturnHresult(uint32_t code) noexcept;
    NTSTATUS __stdcall ReturnNtstatus(uint32_t code) noexcept;
}
