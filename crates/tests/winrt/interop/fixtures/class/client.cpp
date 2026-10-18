#include <winrt/Class.h>
#include "check.h"

using namespace winrt;
using namespace Class;

void test()
{
    Widget widget;
    check(widget.Add(1, 2) == 3);
    check(widget.Name().empty());
    check(widget.Count() == 0);

    widget.Name(L"name");
    check(widget.Name() == L"name");
    check(widget.Count() == 1);
}
