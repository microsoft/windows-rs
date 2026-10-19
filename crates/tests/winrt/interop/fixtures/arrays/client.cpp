#include <winrt/Arrays.h>
#include "check.h"

using namespace winrt;
using namespace Arrays;

void test()
{
    Widget widget;
    check(widget.Sum({ 1, 2, 3 }) == 6);
    check(widget.Sum({}) == 0);
    check(widget.Join({ L"a", L"b" }) == L"a,b");

    std::array<int32_t, 3> values{};
    widget.Fill(values);
    check(values == std::array<int32_t, 3>{ 1, 2, 3 });
    widget.Fill({});

    check(widget.Range(3) == com_array<int32_t>{ 1, 2, 3 });
    check(widget.Range(0).empty());
    check(widget.Split(L"a,b") == com_array<hstring>{ L"a", L"b" });

    com_array<int32_t> out;
    widget.RangeOut(2, out);
    check(out == com_array<int32_t>{ 1, 2 });
}
