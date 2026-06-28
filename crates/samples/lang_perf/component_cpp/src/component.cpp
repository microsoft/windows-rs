#include <windows.h>
#include "winrt/LangPerf.h"

using namespace winrt;

// A no-op WinRT component authored in C++/WinRT, mirroring the Rust component in
// `../component`. Every method ignores its arguments and returns a fixed value so the
// benchmark measures projection/ABI cost rather than any work in the component.
struct Class : implements<Class, LangPerf::IClass, LangPerf::INonDefault> {
    int32_t Int32Property() const noexcept {
        return 0;
    }

    void Int32Property(int32_t) const noexcept {}

    hstring StringProperty() const noexcept {
        return {};
    }

    void StringProperty(hstring const&) const noexcept {}

    Windows::Foundation::IInspectable ObjectProperty() const noexcept {
        return *this;
    }

    void ObjectProperty(Windows::Foundation::IInspectable const&) const noexcept {}

    int32_t Next() const {
        throw hresult_out_of_bounds();
    }

    hstring Lang() const noexcept {
        return L"C++";
    }

    int32_t Value() const noexcept {
        return 0;
    }
};

struct ClassFactory : implements<ClassFactory, Windows::Foundation::IActivationFactory> {
    auto ActivateInstance() const {
        return make<Class>();
    }
};

extern "C" {
    HRESULT __stdcall DllGetActivationFactory(void* type_handle, void** factory) noexcept {
        *factory = nullptr;
        std::wstring_view const type_name = *reinterpret_cast<hstring*>(&type_handle);

        if (type_name != L"LangPerf.Class") {
            return CLASS_E_CLASSNOTAVAILABLE;
        }

        *factory = detach_abi(make<ClassFactory>());
        return S_OK;
    }
}
