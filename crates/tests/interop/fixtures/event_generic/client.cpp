#include <winrt/EventGeneric.h>
#include <winrt/Windows.Foundation.h>
#include "check.h"

using namespace winrt;
using namespace EventGeneric;

void test()
{
    Widget widget;
    Widget sender{ nullptr };
    int32_t received = 0;

    event_token token = widget.Changed([&](Widget const& s, int32_t value)
        {
            sender = s;
            received = value;
        });

    widget.Raise(1);
    check(sender == widget);
    check(received == 1);

    widget.Changed(token);
    widget.Raise(2);
    check(received == 1);
}
