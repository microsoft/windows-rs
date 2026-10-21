#include <winrt/Strings.h>
#include "check.h"

using namespace winrt;
using namespace Strings;

void test()
{
    Widget widget;

    hstring const empty;
    check(widget.Length(empty) == 0);
    check(widget.Echo(empty).empty());

    hstring const embedded{ L"a\0b", 3 };
    check(widget.Length(embedded) == 3);
    check(widget.Echo(embedded) == embedded);

    hstring const pair{ L"\U0001F600" };
    check(widget.Length(pair) == 2);
    check(widget.Echo(pair) == pair);

    wchar_t const high = 0xd800;
    hstring const unpaired{ &high, 1 };
    check(widget.Length(unpaired) == 1);
    check(widget.Echo(unpaired) == unpaired);

    hstring const large = widget.Repeat(L"ab", 100'000);
    check(large.size() == 200'000);
    std::wstring expected;
    for (uint32_t i = 0; i < 100'000; ++i)
    {
        expected += L"ab";
    }
    check(large == expected);
    check(widget.Echo(large) == large);

    hstring const greeting = widget.Greeting();
    check(greeting == L"h\u00e9llo \U0001F600");
    check(greeting.size() == 8);

    check(widget.Text().empty());
    widget.Text(embedded);
    check(widget.Text() == embedded);
    widget.Text(empty);
    check(widget.Text().empty());
}