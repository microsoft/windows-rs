#include <winrt/ErrorRelay.h>
#include "check.h"

using namespace winrt;
using namespace ErrorRelay;

struct Callback : implements<Callback, ICallback>
{
    void Fail(int32_t code, hstring const& message)
    {
        throw hresult_error(hresult{ code }, message);
    }
};

void test()
{
    Widget widget;
    auto callback = make<Callback>();
    auto verify = [&](hresult code, hstring const& message)
    {
        try
        {
            widget.Relay(callback, code, message);
            check(false);
        }
        catch (hresult_error const& error)
        {
            check(error.code() == code);
            check(error.message() == message);
        }
    };
    verify(hresult{ static_cast<int32_t>(0x8000000B) }, L"relay-bounds");
    verify(hresult{ static_cast<int32_t>(0x8004A123) }, L"relay-custom");
}
