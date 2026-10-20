#include <winrt/Windows.Foundation.h>
#include <winrt/Statics.h>
#include "check.h"

using namespace winrt;
using namespace Windows::Foundation;
using namespace Statics;

void test()
{
    check(Widget::Add(1, 2) == 3);

    int32_t changed = 0;
    auto token = Widget::Changed([&](IInspectable const& sender, int32_t value)
    {
        check(!sender);
        changed = value;
    });

    Widget::Value(5);
    check(Widget::Value() == 5);
    check(changed == 5);

    Widget::Changed(token);
    Widget::Value(6);
    check(Widget::Value() == 6);
    check(changed == 5);
}