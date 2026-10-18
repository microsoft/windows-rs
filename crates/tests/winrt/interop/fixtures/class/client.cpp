#include <winrt/Test.h>

using namespace winrt;
using namespace Test;

Widget create()
{
    return Widget();
}

int32_t use(Widget const& widget)
{
    widget.Name(L"name");
    hstring name = widget.Name();
    return widget.Add(1, 2) + widget.Count() + static_cast<int32_t>(name.size());
}
