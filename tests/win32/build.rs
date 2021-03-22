fn main() {
    windows::build!(
        windows::win32::automation::BSTR,
        windows::win32::security::{
            ACCESS_MODE,
        },
        windows::win32::windows_and_messaging::{
            CHOOSECOLORW,
            PROPENUMPROCA,
            PROPENUMPROCW,
            WM_KEYUP,
        },
        windows::win32::dxgi::{
            DXGI_ADAPTER_FLAG, DXGI_FORMAT, DXGI_MODE_DESC, DXGI_MODE_SCALING,
            DXGI_MODE_SCANLINE_ORDER, DXGI_RATIONAL,IDXGIFactory7, CreateDXGIFactory1, DXGI_ERROR_INVALID_CALL,
        },
        windows::win32::display_devices::{
            RECT,
        },
        windows::win32::system_services::{
            CreateEventW,
            SetEvent,
            WaitForSingleObject,
        },
        windows::win32::direct2d::CLSID_D2D1Shadow,
        windows::win32::direct3d12::{
            D3D12_DEFAULT_BLEND_FACTOR_ALPHA
        },
        windows::win32::windows_accessibility::{
            UIA_ScrollPatternNoScroll
        },
        windows::win32::direct3d_hlsl::{
            D3DCOMPILER_DLL
        },
        windows::win32::windows_programming::{
            CloseHandle
        },
        windows::win32::com::CreateUri,
        windows::win32::structured_storage::{CreateStreamOnHGlobal, STREAM_SEEK},
        windows::win32::ui_animation::{UIAnimationManager, UIAnimationTransitionLibrary},
        windows::win32::ldap::ldapsearch,
        windows::win32::game_mode::HasExpandedResources,
        windows::win32::debug::MiniDumpWriteDump,
        windows::win32::direct3d11::D3DDisassemble11Trace,
        windows::win32::windows_update_agent::IAutomaticUpdates,
        windows::win32::windows_color_system::WhitePoint,
    );
}
