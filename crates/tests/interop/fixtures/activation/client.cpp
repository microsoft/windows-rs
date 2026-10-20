#include <winrt/Activation.h>
#include <winrt/Activation.Nested.h>
#include "check.h"

using namespace winrt;
using namespace Activation;

void test()
{
    Nested::Widget widget;
    check(widget.Value() == 123);

    check(Settings::Version() == 456);

    try
    {
        Missing missing;
        check(false);
    }
    catch (hresult_class_not_registered const&)
    {
    }
}
