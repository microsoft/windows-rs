#include <winrt/Windows.Foundation.h>
#include <winrt/Boxing.h>
#include "check.h"

using namespace winrt;
using namespace Boxing;

void test()
{
    Widget widget;
    check(widget.Describe(box_value(1)) == L"i32 1");
    check(widget.Describe(box_value(L"a")) == L"String a");
    check(widget.Describe(nullptr) == L"null");

    check(unbox_value<int32_t>(widget.BoxInt32(2)) == 2);
    check(unbox_value<hstring>(widget.BoxString(L"b")) == L"b");

    check(widget.Optional() == nullptr);
    widget.Optional(3);
    check(widget.Optional().Value() == 3);
    widget.Optional(nullptr);
    check(widget.Optional() == nullptr);
}
