#include <winrt/Collections.h>
#include <winrt/Windows.Foundation.Collections.h>
#include "check.h"

using namespace winrt;
using namespace Collections;
using namespace Windows::Foundation::Collections;

void test()
{
    Widget widget;

    IVector<int32_t> numbers = widget.Numbers();
    check(numbers.Size() == 3);
    numbers.Append(4);
    check(numbers.Size() == 4);
    check(numbers.GetAt(3) == 4);

    IMap<hstring, int32_t> map = single_threaded_map<hstring, int32_t>();
    map.Insert(L"one", 1);
    check(widget.Lookup(map.GetView(), L"one") == 1);

    std::vector<hstring> values;
    for (hstring const& value : widget.Values())
    {
        values.push_back(value);
    }
    check((values == std::vector<hstring>{ L"one", L"two" }));
}
