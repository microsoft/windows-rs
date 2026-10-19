#include <winrt/Interfaces.h>
#include "check.h"

using namespace winrt;
using namespace Interfaces;

struct Shape : implements<Shape, IShape>
{
    int32_t Area()
    {
        return 2;
    }
};

void test()
{
    Widget widget;
    check(widget.Area() == 1);

    IShape shape = widget;
    check(shape.Area() == 1);
    check(widget.Echo(shape) == widget);

    shape = make<Shape>();
    check(widget.Echo(shape) == shape);
}
