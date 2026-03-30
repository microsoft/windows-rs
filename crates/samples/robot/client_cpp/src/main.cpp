#include <windows.h>
#include "winrt/Robotics.h"

using namespace winrt::Robotics;

typedef HRESULT (__stdcall *CreateRobotFromHandle)(void* handle, void**) noexcept;

struct __declspec(uuid("ae60832b-0bc8-57b0-8a69-f82ebc1560ed")) IRobotInterop: IUnknown {
    virtual void* __stdcall Handle() noexcept = 0;
};

extern "C" {
    HRESULT __stdcall main_cpp() noexcept try {
        Robot robot;
        robot.Speak(L"Hello from cpp land");

        auto library = LoadLibraryExW(L"robotics.dll", 0, LOAD_LIBRARY_SEARCH_DEFAULT_DIRS);
        if (!library) winrt::throw_last_error();
        auto create = reinterpret_cast<CreateRobotFromHandle>(GetProcAddress(library, "CreateRobotFromHandle"));
        if (!create) winrt::throw_last_error();

        Robot handy_robot;
        winrt::check_hresult(create((void*)456, winrt::put_abi(handy_robot)));

        handy_robot.Speak(L"Hello handy");
        auto interop = handy_robot.as<IRobotInterop>();
        auto handle = interop->Handle();

        printf("interop handle: 0x%Ix\n", reinterpret_cast<uintptr_t>(handle));
        return S_OK;
    }
    catch (...) { return winrt::to_hresult(); }
}
