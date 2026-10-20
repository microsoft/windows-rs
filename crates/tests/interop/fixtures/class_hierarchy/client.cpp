#include <winrt/ClassHierarchy.h>
#include "check.h"

using namespace winrt;
using namespace ClassHierarchy;

void test()
{
    Compositor compositor;

    auto container = compositor.CreateContainerVisual(123);
    check(container.Children() == 123);
    check(container.Compositor() == compositor);

    auto sprite = compositor.CreateSpriteVisual(456);
    check(sprite.Brush() == 456);
    check(sprite.Children() == 912);
    check(sprite.Compositor() == compositor);

    auto visual = compositor.Echo(sprite);
    check(visual == sprite);
    check(visual.as<SpriteVisual>().Brush() == 456);
    check(!compositor.Echo(container).try_as<SpriteVisual>());
}
