#include <winrt/NontrivialArrays.h>
#include <winrt/Windows.Foundation.h>
#include "check.h"

using namespace winrt;
using namespace NontrivialArrays;
using namespace Windows::Foundation;

struct Value : implements<Value, IValue>
{
    int32_t Number() { return 42; }
};

void check_entry(Entry const& actual, Entry const& expected)
{
    check(actual.Text == expected.Text);
    check(actual.Code == expected.Code && actual.Enabled == expected.Enabled);
    check(static_cast<bool>(actual.Number) == static_cast<bool>(expected.Number));
    if (expected.Number)
    {
        check(actual.Number.Value() == expected.Number.Value());
    }
}

void test()
{
    Widget widget;
    hstring text{ std::wstring_view{ L"first\0tail", 10 } };

    for (uint32_t count : { 0u, 1u, 3u })
    {
        com_array<hstring> strings_fill(count), strings_out, strings_returned;
        {
            std::array<hstring, 3> input{ text, L"", L"last" };
            strings_returned = widget.Strings({ input.data(), input.data() + count }, strings_fill, strings_out);
        }
        std::array<hstring, 3> strings{ text, L"", L"last" };
        check(strings_out.size() == count && strings_returned.size() == count);
        for (uint32_t i = 0; i < count; ++i)
        {
            check(strings_out[i] == strings[i]);
            check(strings_fill[i] == strings[count - i - 1]);
            check(strings_returned[i] == strings[count - i - 1]);
        }

        com_array<Entry> entries_fill(count), entries_out, entries_returned;
        {
            std::array<Entry, 3> input{
                Entry{ text, L'\x03a9', true, IReference<int32_t>{ -7 } },
                Entry{},
                Entry{ L"last", L'\xffff', false, IReference<int32_t>{ 42 } }
            };
            entries_returned = widget.Entries({ input.data(), input.data() + count }, entries_fill, entries_out);
        }
        std::array<Entry, 3> entries{
            Entry{ text, L'\x03a9', true, IReference<int32_t>{ -7 } },
            Entry{},
            Entry{ L"last", L'\xffff', false, IReference<int32_t>{ 42 } }
        };
        check(entries_out.size() == count && entries_returned.size() == count);
        for (uint32_t i = 0; i < count; ++i)
        {
            check_entry(entries_out[i], entries[i]);
            check_entry(entries_fill[i], entries[count - i - 1]);
            check_entry(entries_returned[i], entries[count - i - 1]);
        }

        com_array<IValue> values_fill(count), values_out, values_returned;
        {
            std::array<IValue, 3> input{ widget.CreateValue(7), nullptr, make<Value>() };
            values_returned = widget.Values({ input.data(), input.data() + count }, values_fill, values_out);
        }
        check(values_out.size() == count && values_returned.size() == count);
        std::array<int32_t, 3> numbers{ 7, 0, 42 };
        for (uint32_t i = 0; i < count; ++i)
        {
            auto check_value = [&](IValue const& value, uint32_t index)
            {
                check(static_cast<bool>(value) == (index != 1));
                if (value)
                {
                    check(value.Number() == numbers[index]);
                }
            };
            check_value(values_out[i], i);
            check_value(values_fill[i], count - i - 1);
            check_value(values_returned[i], count - i - 1);
        }
    }
}
