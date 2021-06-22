fn main() {
    windows::build! {
        Windows::{
            Foundation::Numerics::Matrix3x2,
            Win32::{
                Foundation::{DXGI_STATUS_OCCLUDED, HINSTANCE, HWND, LPARAM, LRESULT, PSTR, WPARAM},
                Graphics::{
                    Direct2D::{
                        CLSID_D2D1Shadow, D2D1CreateFactory, ID2D1Bitmap1, ID2D1Device,
                        ID2D1DeviceContext, ID2D1Effect, ID2D1Factory1, ID2D1SolidColorBrush,
                    },
                    Direct3D11::{D3D11CreateDevice, ID3D11Device, D3D11_SDK_VERSION},
                    Dxgi::{
                        CreateDXGIFactory1, IDXGIAdapter, IDXGIDevice, IDXGIFactory2, IDXGISwapChain1,
                        DXGI_ERROR_UNSUPPORTED, DXGI_FORMAT, DXGI_PRESENT_TEST,
                        DXGI_USAGE_RENDER_TARGET_OUTPUT,
                    },
                    Gdi::{BeginPaint, EndPaint, PAINTSTRUCT},
                },
                System::{
                    LibraryLoader::GetModuleHandleA,
                    Performance::{QueryPerformanceCounter, QueryPerformanceFrequency},
                    SystemInformation::GetLocalTime,
                },
                UI::{
                    Animation::{
                        IUIAnimationManager, IUIAnimationTransitionLibrary, IUIAnimationVariable,
                        UIAnimationManager, UIAnimationTransitionLibrary,
                    },
                    WindowsAndMessaging::{
                        CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, GetWindowLongA,
                        GetWindowLongPtrA, LoadCursorW, PeekMessageA, PostQuitMessage, RegisterClassA,
                        SetWindowLongA, SetWindowLongPtrA, CREATESTRUCTA, CW_USEDEFAULT, IDC_HAND, MSG,
                        SIZE_MINIMIZED, WINDOW_LONG_PTR_INDEX, WM_ACTIVATE, WM_DESTROY,
                        WM_DISPLAYCHANGE, WM_NCCREATE, WM_PAINT, WM_QUIT, WM_SIZE, WM_USER, WNDCLASSA,
                    },
                },
            },
        },
    };
}
