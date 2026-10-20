#include "Composition/Base.g.h"
#include <winrt/Windows.Foundation.h>

namespace winrt::Composition::implementation {
    static std::atomic<int32_t> instances;

    struct Base : BaseT<Base> {
        int32_t m_value;

        Base(int32_t value = 0) : m_value(value) {
            ++instances;
        }

        ~Base() {
            --instances;
        }

        static int32_t Instances() {
            return instances;
        }

        int32_t Value() {
            return m_value;
        }

        // Dispatches to a derived class's override, if any.
        hstring Describe() {
            return overridable().Name() + L"(" + to_hstring(m_value) + L")";
        }

        hstring Stringify() {
            if (auto stringable = get_strong().try_as<Windows::Foundation::IStringable>()) {
                return stringable.ToString();
            }

            return L"";
        }

        hstring Name() {
            return L"Base";
        }

        int32_t Doubled() {
            return m_value * 2;
        }
    };
}

namespace winrt::Composition::factory_implementation {
    struct Base : BaseT<Base, implementation::Base> {};
}

void* __stdcall winrt_get_activation_factory(std::wstring_view const& name) {
    if (name == L"Composition.Base") {
        return winrt::detach_abi(winrt::make<winrt::Composition::factory_implementation::Base>());
    }

    return nullptr;
}
