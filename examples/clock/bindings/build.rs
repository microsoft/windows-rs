fn main() {
    windows::build!(
        windows::foundation::numerics::Matrix3x2,
        windows::win32::gdi::{ValidateRect, BeginPaint, EndPaint},
        windows::win32::menus_and_resources::{LoadCursorA, HMENU},
        windows::win32::system_services::{
            GetModuleHandleA, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, HINSTANCE, IDC_ARROW, LRESULT,
            WM_PAINT, WS_OVERLAPPEDWINDOW, WS_VISIBLE, WM_NCCREATE,GWLP_USERDATA, SIZE_MINIMIZED,
            DXGI_STATUS_OCCLUDED, WM_USER,PM_REMOVE, WM_QUIT, WM_SIZE, WM_DISPLAYCHANGE,WM_ACTIVATE,
            WM_DESTROY
        },
        windows::win32::windows_and_messaging::{
            CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, PostQuitMessage,
            PeekMessageA, RegisterClassA, HWND, LPARAM, MSG, WNDCLASSA, WPARAM, CREATESTRUCTA,
            MINMAXINFO,
        },
        windows::win32::direct2d::{ID2D1Factory7, D2D1CreateFactory, D2D1_DEBUG_LEVEL, D2D1_INTERPOLATION_MODE, D2D1_COMPOSITE_MODE, D2D1_ALPHA_MODE, D2D1_BITMAP_OPTIONS,D2D_SIZE_F,
            D2D1_CAP_STYLE, D2D1_DEVICE_CONTEXT_OPTIONS,D2D1_UNIT_MODE,ID2D1StrokeStyle,ID2D1DeviceContext,ID2D1SolidColorBrush,ID2D1Effect,ID2D1Bitmap,D2D1_ELLIPSE,ID2D1Bitmap1,D2D_SIZE_U,D2D1_BITMAP_PROPERTIES, D2D1_BITMAP_PROPERTIES1,D2D1_PIXEL_FORMAT,
            D2D1_BRUSH_PROPERTIES,D2D1_STROKE_STYLE_PROPERTIES,ID2D1Image, D2D_POINT_2F, D2D_RECT_F, ID2D1Device
        },
        windows::win32::direct3d11::{D3D11CreateDevice, D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION},
        windows::win32::system_services::{DXGI_ERROR_UNSUPPORTED,  DXGI_ERROR_INVALID_CALL},
        windows::win32::dxgi::{
            IDXGIDevice, IDXGIFactory7, CreateDXGIFactory1, DXGI_USAGE_RENDER_TARGET_OUTPUT,
            DXGI_PRESENT_TEST, DXGI_FORMAT, DXGI_RGBA, IDXGISurface, DXGI_SWAP_EFFECT,IDXGISwapChain1, DXGI_SWAP_CHAIN_DESC1,DXGI_SWAP_CHAIN_FULLSCREEN_DESC,IDXGIOutput,
        },
        windows::win32::ui_animation::{IUIAnimationManager, IUIAnimationTransitionLibrary, UIAnimationManager,UIAnimationTransitionLibrary, IUIAnimationVariable,IUIAnimationTransition,UI_ANIMATION_UPDATE_RESULT,},
        windows::win32::windows_programming::{
            QueryPerformanceCounter, QueryPerformanceFrequency, GetLocalTime
        },
    );
}
