fn main() {
    windows::build!(
        windows::win32::gdi::{ValidateRect, BeginPaint, EndPaint},
        windows::win32::menus_and_resources::{LoadCursorA, HMENU},
        windows::win32::system_services::{
            GetModuleHandleA, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, HINSTANCE, IDC_ARROW, LRESULT,
            WM_PAINT, WS_OVERLAPPEDWINDOW, WS_VISIBLE, WM_NCCREATE,GWLP_USERDATA, SIZE_MINIMIZED,
            DXGI_STATUS_OCCLUDED, WM_USER,PM_REMOVE, WM_QUIT, WM_SIZE, WM_DISPLAYCHANGE,WM_ACTIVATE,
            WM_DESTROY,VBS_BASIC_ENCLAVE_SYSCALL_PAGE
        },
        windows::win32::windows_and_messaging::{
            CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, PostQuitMessage,
            PeekMessageA, RegisterClassA, HWND, LPARAM, MSG, WNDCLASSA, WPARAM, CREATESTRUCTA,
            MINMAXINFO,
        },
        windows::win32::direct2d::{ID2D1Factory7, D2D1CreateFactory, D2D1_DEBUG_LEVEL},
        windows::win32::direct3d11::{D3D11CreateDevice, D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION},
        windows::win32::system_services::{DXGI_ERROR_UNSUPPORTED,  DXGI_ERROR_INVALID_CALL},
        windows::win32::dxgi::{
            IDXGIDevice, IDXGIFactory7, CreateDXGIFactory1, DXGI_USAGE_RENDER_TARGET_OUTPUT,
            DXGI_PRESENT_TEST, DXGI_FORMAT, DXGI_RGBA,
        },
        windows::win32::ui_animation::{IUIAnimationManager, IUIAnimationTransitionLibrary},
        windows::win32::upnp::{UIAnimationManager,UIAnimationTransitionLibrary},
        windows::win32::windows_programming::{
            QueryPerformanceCounter, QueryPerformanceFrequency, GetLocalTime
        },
    );
}
