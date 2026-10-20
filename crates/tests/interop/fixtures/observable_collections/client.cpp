#include <winrt/ObservableCollections.h>
#include <winrt/Windows.Foundation.Collections.h>
#include "check.h"

using namespace winrt;
using namespace ObservableCollections;
using namespace Windows::Foundation::Collections;

static void change_numbers(IObservableVector<int32_t> const& numbers)
{
    check(numbers.Size() == 0);
    std::vector<std::tuple<bool, CollectionChange, uint32_t>> changes;

    auto revoker = numbers.VectorChanged(auto_revoke, [&](IObservableVector<int32_t> const& sender, IVectorChangedEventArgs const& args)
        {
            CollectionChange change = args.CollectionChange();
            uint32_t index = change == CollectionChange::Reset ? 0 : args.Index();
            changes.emplace_back(sender == numbers, change, index);
        });

    numbers.Append(1);
    numbers.Append(3);
    numbers.InsertAt(1, 2);
    numbers.SetAt(0, 10);
    numbers.RemoveAt(1);
    numbers.RemoveAtEnd();
    numbers.Clear();
    revoker.revoke();
    numbers.Append(4);

    check((changes == std::vector<std::tuple<bool, CollectionChange, uint32_t>>{
        { true, CollectionChange::ItemInserted, 0 },
        { true, CollectionChange::ItemInserted, 1 },
        { true, CollectionChange::ItemInserted, 1 },
        { true, CollectionChange::ItemChanged, 0 },
        { true, CollectionChange::ItemRemoved, 1 },
        { true, CollectionChange::ItemRemoved, 1 },
        { true, CollectionChange::Reset, 0 },
    }));

    check(numbers.Size() == 1);
    check(numbers.GetAt(0) == 4);
}

static void change_names(IObservableMap<hstring, int32_t> const& names)
{
    check(names.Size() == 0);
    std::vector<std::tuple<bool, CollectionChange, hstring>> changes;

    auto revoker = names.MapChanged(auto_revoke, [&](IObservableMap<hstring, int32_t> const& sender, IMapChangedEventArgs<hstring> const& args)
        {
            CollectionChange change = args.CollectionChange();
            hstring key = change == CollectionChange::Reset ? hstring{} : args.Key();
            changes.emplace_back(sender == names, change, key);
        });

    check(!names.Insert(L"one", 1));
    check(!names.Insert(L"two", 2));
    check(names.Insert(L"one", 10));
    names.Remove(L"two");
    names.Clear();
    revoker.revoke();
    check(!names.Insert(L"three", 3));

    check((changes == std::vector<std::tuple<bool, CollectionChange, hstring>>{
        { true, CollectionChange::ItemInserted, L"one" },
        { true, CollectionChange::ItemInserted, L"two" },
        { true, CollectionChange::ItemChanged, L"one" },
        { true, CollectionChange::ItemRemoved, L"two" },
        { true, CollectionChange::Reset, L"" },
    }));

    check(names.Size() == 1);
    check(names.Lookup(L"three") == 3);
}

void test()
{
    Widget widget;
    change_numbers(widget.Numbers());
    change_names(widget.Names());
    widget.ChangeNumbers(single_threaded_observable_vector<int32_t>());
    widget.ChangeNames(single_threaded_observable_map<hstring, int32_t>());
}