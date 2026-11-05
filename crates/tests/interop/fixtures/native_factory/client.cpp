#include <windows.h>
#include <winrt/NativeFactory.h>
#include "check.h"

using namespace winrt;
using namespace NativeFactory;

struct __declspec(uuid("53c9217e-8e64-4bd7-8b95-bf821db7a198")) IWidgetInterop : ::IUnknown
{
    virtual void* __stdcall Handle() noexcept = 0;
};

void test()
{
    check(Widget().HandleValue() == 0);

    auto library = GetModuleHandleW(L"NativeFactory.dll");
    check(library != nullptr);
    using Create = HRESULT (__stdcall *)(void*, void**) noexcept;
    auto create = reinterpret_cast<Create>(GetProcAddress(library, "CreateWidgetFromHandle"));
    check(create != nullptr);

    auto handle = reinterpret_cast<void*>(0x1c8);
    Widget widget{ nullptr };
    check_hresult(create(handle, put_abi(widget)));
    check(widget.HandleValue() == 0x1c8);
    auto interop = widget.as<IWidgetInterop>();
    check(interop->Handle() == handle);
}
