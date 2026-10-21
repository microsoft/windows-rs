#include <winrt/Windows.Foundation.h>
#include <winrt/MappedTypes.h>
#include "check.h"

using namespace winrt;
using namespace Windows::Foundation;
using namespace MappedTypes;

// 2000-01-01 00:00:00 UTC.
constexpr int64_t y2k = 125'911'584'000'000'000;

void test()
{
    Widget widget;
    DateTime time{ TimeSpan{ y2k } };
    TimeSpan span = std::chrono::minutes(90);
    check(widget.Ticks(time) == y2k);
    check(widget.Duration(span) == span.count());
    check(widget.Add(time, span) == time + span);

    check(widget.Host(Uri(L"https://example.com/path")) == L"example.com");
    check(widget.CreateUri(L"https://example.com/path").Host() == L"example.com");

    check(widget.ToString() == L"widget");
    int32_t closed = Widget::Closed();
    widget.Close();
    check(Widget::Closed() == closed + 1);
}