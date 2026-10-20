#include <winrt/Errors.h>
#include "check.h"

using namespace winrt;
using namespace Errors;

struct Callback : implements<Callback, ICallback>
{
    void Fail()
    {
        throw hresult_out_of_bounds(L"client");
    }
};

void test()
{
    Widget widget;

    try
    {
        widget.Fail();
        check(false);
    }
    catch (hresult_out_of_bounds const&)
    {
    }

    try
    {
        widget.FailWithMessage(L"component");
        check(false);
    }
    catch (hresult_out_of_bounds const& e)
    {
        check(e.message() == L"component");
    }

    widget.Check(make<Callback>());
}
