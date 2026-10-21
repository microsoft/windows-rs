#include <winrt/ArrayDelegates.h>
#include <algorithm>
#include "check.h"

using namespace winrt;
using namespace ArrayDelegates;

void test()
{
    Widget widget;
    widget.CheckStrings([](array_view<hstring const> input, array_view<hstring> fill, com_array<hstring>& output)
    {
        check(input.size() == fill.size());
        std::reverse_copy(input.begin(), input.end(), fill.begin());
        output = com_array<hstring>(input.begin(), input.end());
        return com_array<hstring>(fill.begin(), fill.end());
    });
    widget.CheckEntries([](array_view<Entry const> input, array_view<Entry> fill, com_array<Entry>& output)
    {
        check(input.size() == fill.size());
        std::reverse_copy(input.begin(), input.end(), fill.begin());
        output = com_array<Entry>(input.begin(), input.end());
        return com_array<Entry>(fill.begin(), fill.end());
    });
}
