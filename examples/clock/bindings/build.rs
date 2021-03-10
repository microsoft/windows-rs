fn main() {
    windows::build!(
        windows::foundation::numerics::Matrix3x2,
        windows::win32::direct2d::{
            D2D1CreateFactory, ID2D1Bitmap, ID2D1Bitmap1, ID2D1Device, ID2D1DeviceContext, ID2D1Effect,
            ID2D1Factory1, ID2D1Factory7, ID2D1Image, ID2D1SolidColorBrush, ID2D1StrokeStyle,
            D2D1_ALPHA_MODE, D2D1_BITMAP_OPTIONS, D2D1_BITMAP_PROPERTIES, D2D1_BITMAP_PROPERTIES1,
            D2D1_BRUSH_PROPERTIES, D2D1_CAP_STYLE, D2D1_COMPOSITE_MODE, D2D1_DEBUG_LEVEL,
            D2D1_DEVICE_CONTEXT_OPTIONS, D2D1_ELLIPSE, D2D1_FACTORY_OPTIONS, D2D1_FACTORY_TYPE,
            D2D1_INTERPOLATION_MODE, D2D1_PIXEL_FORMAT, D2D1_STROKE_STYLE_PROPERTIES, D2D1_UNIT_MODE,
            D2D_POINT_2F, D2D_RECT_F, D2D_SIZE_F, D2D_SIZE_U,
        },
        windows::win32::direct3d11::{
            D3D11CreateDevice, ID3D11Device, D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION,
            D3D_DRIVER_TYPE,
        },
        windows::win32::dxgi::{
            CreateDXGIFactory1, IDXGIDevice, IDXGIFactory2, IDXGIFactory7, IDXGIOutput, IDXGISurface,
            IDXGISwapChain1, DXGI_FORMAT, DXGI_PRESENT_TEST, DXGI_RGBA, DXGI_SAMPLE_DESC,
            DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_CHAIN_FULLSCREEN_DESC, DXGI_SWAP_EFFECT,
            DXGI_USAGE_RENDER_TARGET_OUTPUT,
        },
        windows::win32::gdi::{BeginPaint, EndPaint, ValidateRect, PAINTSTRUCT},
        windows::win32::menus_and_resources::{LoadCursorA, HMENU},
        windows::win32::system_services::{
            GetModuleHandleA, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, DXGI_ERROR_INVALID_CALL,
            DXGI_ERROR_UNSUPPORTED, DXGI_STATUS_OCCLUDED, GWLP_USERDATA, HINSTANCE, IDC_ARROW, LRESULT,
            PM_REMOVE, PSTR, SIZE_MINIMIZED, WM_ACTIVATE, WM_DESTROY, WM_DISPLAYCHANGE, WM_NCCREATE,
            WM_PAINT, WM_QUIT, WM_SIZE, WM_USER, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
        windows::win32::ui_animation::{
            IUIAnimationManager, IUIAnimationTransition, IUIAnimationTransitionLibrary,
            IUIAnimationVariable, UIAnimationManager, UIAnimationTransitionLibrary,
            UI_ANIMATION_UPDATE_RESULT,
        },
        windows::win32::windows_and_messaging::{
            CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, PeekMessageA,
            PostQuitMessage, RegisterClassA, CREATESTRUCTA, HWND, LPARAM, MINMAXINFO, MSG, WNDCLASSA,
            WPARAM,
        },
        windows::win32::windows_programming::{
            GetLocalTime, QueryPerformanceCounter, QueryPerformanceFrequency,
        },
    );
}
