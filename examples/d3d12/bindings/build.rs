fn main() {
    windows::build! {
        Windows::Win32::Graphics::Direct3D11::{
            D3D_FEATURE_LEVEL_11_0, D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
        },
        Windows::Win32::Graphics::Direct3D12::*,
        Windows::Win32::Graphics::Dxgi::*,
        Windows::Win32::Graphics::Hlsl::*,
        Windows::Win32::System::SystemServices::{GetModuleHandleA, HINSTANCE, PSTR},
        Windows::Win32::System::Threading::{CreateEventA, WaitForSingleObject},
        Windows::Win32::System::WindowsProgramming::INFINITE,
        Windows::Win32::UI::DisplayDevices::RECT,
        Windows::Win32::UI::WindowsAndMessaging::{
            AdjustWindowRect, CreateWindowExA, DefWindowProcA, DispatchMessageA, GetWindowLongA,
            GetWindowLongPtrA, LoadCursorW, PeekMessageA, PostQuitMessage, RegisterClassExA,
            SetWindowLongA, SetWindowLongPtrA, ShowWindow, TranslateMessage, CREATESTRUCTA,
            CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, GWLP_USERDATA, IDC_ARROW, MSG, PM_REMOVE,
            SW_SHOW, WM_CREATE, WM_DESTROY, WM_KEYDOWN, WM_KEYUP, WM_PAINT, WM_QUIT, WNDCLASSEXA,
            WNDCLASS_STYLES, WS_OVERLAPPEDWINDOW,
        },
    };
}
