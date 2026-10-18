#include <winrt/Test.h>

using namespace winrt;
using namespace Test;

void use(Widget const& widget)
{
    event_token token = widget.Changed([](Widget const&, int32_t) {});
    widget.Changed(token);
}
