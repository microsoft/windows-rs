#include <windows.h>
#include <assert.h>
#include "winrt/test_composable.h"

using namespace winrt;
using namespace test_composable;

void test()
{
    Compositor compositor;

    auto container = compositor.CreateContainerVisual(123);
    assert(container.Children() == 123);
    assert(container.Compositor() == compositor);

    auto sprite = compositor.CreateSpriteVisual(456);
    assert(sprite.Brush() == 456);
    assert(sprite.Children() == 456 * 2);
    assert(sprite.Compositor() == compositor);
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
