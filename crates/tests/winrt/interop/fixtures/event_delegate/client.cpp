#include <winrt/EventDelegate.h>
#include "check.h"

using namespace winrt;
using namespace EventDelegate;

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

    {
        Widget::Changed_revoker revoker = widget.Changed(auto_revoke, ChangedHandler([&](auto&&, int32_t value) { received = value; }));
        widget.Raise(3);
        check(received == 3);
    }

    widget.Raise(4);
    check(received == 3);
}
