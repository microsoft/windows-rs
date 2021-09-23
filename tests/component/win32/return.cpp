#include "pch.h"
#include "return.h"

namespace
{
    struct Class : winrt::implements<Class, IReturn>
    {
        int64_t __stdcall ReturnValue() noexcept
        {
            return 123;
        }

        ReturnType __stdcall ReturnStruct() noexcept
        {
            return
            {
                123,
                456,
                789
            };
        }

        void __stdcall ReturnVoid(int64_t* check) noexcept
        {
            *check = 123;
        }

        HRESULT __stdcall ReturnHresult(uint32_t code) noexcept
        {
            return code;
        }

        NTSTATUS __stdcall ReturnNtstatus(uint32_t code) noexcept
        {
            return code;
        }

        HRESULT __stdcall ReturnOutValue(_Outptr_ int64_t* value) noexcept
        {
            *value = 123;
            return S_OK;
        }
    };
}

int64_t __stdcall ReturnValue() noexcept
{
    return 123;
}

ReturnType __stdcall ReturnStruct() noexcept
{
    return
    {
        123,
        456,
        789
    };
}

HRESULT __stdcall CreateReturn(IReturn** object) noexcept
{
    *object = winrt::make<Class>().detach();
    return S_OK;
}

void __stdcall ReturnVoid(int64_t* check) noexcept
{
    *check = 123;
}

HRESULT __stdcall ReturnHresult(uint32_t code) noexcept
{
    return code;
}

NTSTATUS __stdcall ReturnNtstatus(uint32_t code) noexcept
{
    return code;
}

HRESULT __stdcall ReturnOutValue(_Outptr_ int64_t* value) noexcept
{
    *value = 123;
    return S_OK;
}
