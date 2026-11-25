#include <winrt/Collections.h>
#include <winrt/Windows.Foundation.Collections.h>
#include "check.h"

using namespace winrt;
using namespace Collections;
using namespace Windows::Foundation::Collections;

#include "read_only.h"

void test()
{
    Widget widget;

    IVector<int32_t> numbers = widget.Numbers();
    check(numbers.Size() == 3);
    numbers.Append(4);
    check(numbers.Size() == 4);
    check(numbers.GetAt(3) == 4);
    check(widget.Sum(numbers) == 10);
    check(widget.Sum(single_threaded_vector<int32_t>({ 1, 2 })) == 3);

    IMap<hstring, int32_t> map = single_threaded_map<hstring, int32_t>();
    map.Insert(L"one", 1);
    check(widget.Lookup(map.GetView(), L"one") == 1);

    std::vector<hstring> values;
    for (hstring const& value : widget.Values())
    {
        values.push_back(value);
    }
    check((values == std::vector<hstring>{ L"one", L"two" }));

    IMap<hstring, int32_t> index = widget.Index(single_threaded_vector(std::move(values)).GetView());
    check(index.Size() == 2);
    check(index.Lookup(L"two") == 1);

    read_only_contract(widget);
    read_only_marshaling_failure(widget);
    vector_storage_contract([&](array_view<Inspectable const> values) {
        return widget.ReadOnly(values);
    });
    vector_storage_contract([&](array_view<Inspectable const> values) {
        return widget.MarshaledVector(values);
    });
    vector_storage_contract([&](array_view<Inspectable const> values) {
        return widget.MarshaledObservableVector(values).as<Vector>();
    });
    map_resolution_failure([&](array_view<Inspectable const> values) {
        return widget.MarshaledMap(values);
    });
    map_resolution_failure([&](array_view<Inspectable const> values) {
        return widget.MarshaledObservableMap(values).as<IMap<uint32_t, Inspectable>>();
    });
}
