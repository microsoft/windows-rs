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