fn main() {
    windows::build!(
        Windows::Win32::Automation::BSTR,
        Windows::Win32::Security::{
            ACCESS_MODE,
        },
        Windows::Win32::WindowsAndMessaging::{
            CHOOSECOLORW,
            PROPENUMPROCA,
            PROPENUMPROCW,
            WM_KEYUP,
        },
        Windows::Win32::Dxgi::{
            DXGI_ADAPTER_FLAG, DXGI_FORMAT, DXGI_MODE_DESC, DXGI_MODE_SCALING,
            DXGI_MODE_SCANLINE_ORDER, DXGI_RATIONAL,IDXGIFactory7, CreateDXGIFactory1, DXGI_ERROR_INVALID_CALL,
        },
        Windows::Win32::DisplayDevices::{
            RECT,
        },
        Windows::Win32::SystemServices::{
            CreateEventW,
            SetEvent,
            WaitForSingleObject,
        },
        Windows::Win32::Direct2D::CLSID_D2D1Shadow,
        Windows::Win32::Direct3D12::{
            D3D12_DEFAULT_BLEND_FACTOR_ALPHA
        },
        Windows::Win32::WindowsAccessibility::{
            UIA_ScrollPatternNoScroll
        },
        Windows::Win32::Direct3DHlsl::{
            D3DCOMPILER_DLL
        },
        Windows::Win32::WindowsProgramming::{
            CloseHandle
        },
        Windows::Win32::Com::CreateUri,
        Windows::Win32::StructuredStorage::{CreateStreamOnHGlobal, STREAM_SEEK},
        Windows::Win32::UIAnimation::{UIAnimationManager, UIAnimationTransitionLibrary},
        Windows::Win32::Ldap::ldapsearch,
        Windows::Win32::GameMode::HasExpandedResources,
        Windows::Win32::Debug::MiniDumpWriteDump,
        Windows::Win32::Direct3D11::D3DDisassemble11Trace,
        Windows::Win32::WindowsUpdateAgent::IAutomaticUpdates,
        Windows::Win32::WindowsColorSystem::WhitePoint,
    );
}
