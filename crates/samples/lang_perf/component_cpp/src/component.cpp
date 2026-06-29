#include <windows.h>
#include <vector>
#include "winrt/LangPerf.h"
#include "winrt/Windows.Foundation.h"
#include "winrt/Windows.Foundation.Collections.h"

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

    winrt::event_token Event(LangPerf::Handler const& handler) {
        return m_event.add(handler);
    }

    void Event(winrt::event_token const& token) noexcept {
        m_event.remove(token);
    }

    void Raise() {
        m_event(*this, 0);
    }

    Windows::Foundation::Collections::IVector<int32_t> Items(uint32_t count) const {
        std::vector<int32_t> values;
        values.reserve(count);
        for (uint32_t i = 0; i < count; i++) {
            values.push_back(static_cast<int32_t>(i));
        }
        return multi_threaded_vector<int32_t>(std::move(values));
    }

    Windows::Foundation::Collections::IMap<hstring, int32_t> Map(uint32_t count) const {
        auto map = multi_threaded_map<hstring, int32_t>();
        for (uint32_t i = 0; i < count; i++) {
            map.Insert(to_hstring(i), static_cast<int32_t>(i));
        }
        return map;
    }

    Windows::Foundation::IAsyncOperation<int32_t> Operation() const {
        co_return 0;
    }

    Windows::Foundation::IReference<int32_t> Reference() const noexcept {
        return winrt::box_value(0).as<Windows::Foundation::IReference<int32_t>>();
    }

    int32_t Value() const noexcept {
        return 0;
    }

private:
    winrt::event<LangPerf::Handler> m_event;
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
