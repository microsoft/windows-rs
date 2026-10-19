#include <winrt/Windows.Foundation.h>
#include <winrt/AsyncMethods.h>
#include "check.h"

using namespace winrt;
using namespace Windows::Foundation;
using namespace AsyncMethods;

IAsyncOperation<int32_t> background()
{
    co_await resume_background();
    co_return 41;
}

void test()
{
    Widget widget;
    widget.Action().get();
    check(widget.Operation(1).get() == 1);
    check(widget.StringOperation(L"a").get() == L"a");

    try
    {
        widget.Fail().get();
        check(false);
    }
    catch (hresult_out_of_bounds const&)
    {
    }

    check(widget.Increment(background()).get() == 42);
}
