#include <winrt/Overloads.h>
#include "check.h"

using namespace winrt;
using namespace Overloads;

void test()
{
    Widget widget;
    check(widget.Sum() == 0);
    check(widget.Sum(1) == 1);
    check(widget.Sum(1, 2) == 3);
    check(widget.Sum(1, 2, 3) == 6);
    check(widget.Format(1) == L"int 1");
    check(widget.Format(L"a") == L"text a");
}
