#include <windows.h>
#include "winrt/Robotics.h"

using namespace winrt;

struct __declspec(uuid("ae60832b-0bc8-57b0-8a69-f82ebc1560ed")) IRobotInterop: IUnknown {
    virtual HANDLE __stdcall Handle() noexcept = 0;
};

struct Robot : implements<Robot, Robotics::IRobot, IRobotInterop> {
    HANDLE m_handle {};

    Robot(HANDLE handle) : m_handle(handle) {}

    void Speak(hstring const& message) {
        if (m_handle == nullptr) {
            printf("%ls\n", message.c_str());
        } else {
            printf("%ls (0x%Ix)\n", message.c_str(), reinterpret_cast<uintptr_t>(m_handle));
        }
    }

    HANDLE __stdcall Handle() noexcept override {
        return m_handle;
    }
};

struct RobotFactory: implements<RobotFactory, Windows::Foundation::IActivationFactory> {
    auto ActivateInstance() const {
        return make<Robot>(nullptr);
    }
};

extern "C" {
    HRESULT __stdcall DllGetActivationFactory(void* type_handle, void** factory) noexcept {
        *factory = nullptr;
        std::wstring_view const type_name = *reinterpret_cast<hstring*>(&type_handle);

        if (type_name != L"Robotics.Robot") {
            return CLASS_E_CLASSNOTAVAILABLE;
        }

        *factory = detach_abi(make<RobotFactory>());
        return S_OK;
    }

    HRESULT __stdcall CreateRobotFromHandle(HANDLE handle, void** robot) noexcept {
        *robot = detach_abi(make<Robot>(handle));
        return S_OK;
    }
}
