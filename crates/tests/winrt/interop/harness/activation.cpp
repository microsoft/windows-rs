// Exports the activation factories of a fixture's C++/WinRT `component.cpp`, which defines
// `winrt_get_activation_factory` and returns null for a class it does not implement.

#include <winrt/base.h>

void* __stdcall winrt_get_activation_factory(std::wstring_view const& name);

extern "C" int32_t __stdcall DllGetActivationFactory(void* class_id, void** factory) noexcept try {
    std::wstring_view const name{*reinterpret_cast<winrt::hstring*>(&class_id)};
    *factory = winrt_get_activation_factory(name);

    if (*factory) {
        return 0;
    }

    return winrt::hresult_class_not_available(name).to_abi();
} catch (...) {
    return winrt::to_hresult();
}