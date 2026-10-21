#include <winrt/BoxedArrays.h>
#include <winrt/Windows.Foundation.h>
#include <algorithm>
#include "check.h"

using namespace winrt;
using namespace BoxedArrays;
using namespace Windows::Foundation;

void test()
{
    Widget widget;
    std::array<int32_t, 3> numbers{ INT32_MIN, 0, INT32_MAX };
    std::array<bool, 3> booleans{ true, false, true };
    std::array<hstring, 3> strings{
        hstring{ std::wstring_view{ L"first\0tail", 10 } }, L"", L"last"
    };

    for (uint32_t count : { 0u, 3u })
    {
        array_view<int32_t const> number_input(numbers.data(), numbers.data() + count);
        auto numbers_boxed = widget.BoxNumbers(number_input);
        check(std::ranges::equal(numbers_boxed.as<IReferenceArray<int32_t>>().Value(), number_input));
        check(std::ranges::equal(widget.Numbers(PropertyValue::CreateInt32Array(number_input)), number_input));

        array_view<bool const> boolean_input(booleans.data(), booleans.data() + count);
        auto booleans_boxed = widget.BoxBooleans(boolean_input);
        check(std::ranges::equal(booleans_boxed.as<IReferenceArray<bool>>().Value(), boolean_input));
        check(std::ranges::equal(widget.Booleans(PropertyValue::CreateBooleanArray(boolean_input)), boolean_input));

        array_view<hstring const> string_input(strings.data(), strings.data() + count);
        auto strings_boxed = widget.BoxStrings(string_input);
        check(std::ranges::equal(strings_boxed.as<IReferenceArray<hstring>>().Value(), string_input));
        check(std::ranges::equal(widget.Strings(PropertyValue::CreateStringArray(string_input)), string_input));
    }

    try
    {
        widget.Numbers(PropertyValue::CreateStringArray({ L"not a number" }));
        check(false);
    }
    catch (hresult_no_interface const&)
    {
    }
}
