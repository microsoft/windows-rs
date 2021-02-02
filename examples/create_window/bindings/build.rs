fn main() {
    windows::build!(
        windows::win32::gdi::ValidateRect,
        windows::win32::menus_and_resources::{LoadCursorA, HMENU},
        windows::win32::system_services::{
            GetModuleHandleA, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, HINSTANCE, IDC_ARROW, LRESULT,
            WM_DESTROY, WM_PAINT, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
        windows::win32::windows_and_messaging::{
            CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, PostQuitMessage,
            RegisterClassA, HWND, LPARAM, MSG, WNDCLASSA, WPARAM,
        },
    );
}
