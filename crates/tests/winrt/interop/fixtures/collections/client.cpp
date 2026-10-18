#include <winrt/Test.h>
#include <winrt/Windows.Foundation.Collections.h>

using namespace winrt;
using namespace Test;
using namespace Windows::Foundation::Collections;

int32_t use(Widget const& widget)
{
    IVector<int32_t> numbers = widget.Numbers();
    numbers.Append(1);

    int32_t total = widget.Lookup(single_threaded_map<hstring, int32_t>().GetView());

    for (hstring const& value : widget.Values())
    {
        total += static_cast<int32_t>(value.size());
    }

    return total + static_cast<int32_t>(numbers.Size());
}
