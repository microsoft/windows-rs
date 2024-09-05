#include <windows.h>
#include <assert.h>
#include "winrt/test_constructors.h"

using namespace winrt;
using namespace test_constructors;

void test()
{
    Activatable activatable;
    assert(activatable.Property() == 0);

    Activatable activatable2(123);
    assert(activatable2.Property() == 123);

    Composable composable;
    assert(composable.Property() == 0);

    Composable composable2(456);
    assert(composable2.Property() == 456);
}

extern "C"
{
    HRESULT __stdcall interop() noexcept
    try
    {
        test();
        return S_OK;
    }
    catch (...)
    {
        return to_hresult();
    }
}
