fn main() {
    windows::build!(
        Windows::Win32::System::OleAutomation::BSTR,
        Windows::Win32::Security::{
            ACCESS_MODE,
        },
        Windows::Win32::UI::WindowsAndMessaging::{
            CHOOSECOLORW,
            PROPENUMPROCA,
            PROPENUMPROCW,
            WM_KEYUP,
        },
        Windows::Win32::Graphics::Dxgi::{
            DXGI_ADAPTER_FLAG, DXGI_FORMAT, DXGI_MODE_DESC, DXGI_MODE_SCALING,
            DXGI_MODE_SCANLINE_ORDER, DXGI_RATIONAL,IDXGIFactory7, CreateDXGIFactory1, DXGI_ERROR_INVALID_CALL,
        },
        Windows::Win32::UI::DisplayDevices::{
            RECT,
        },
        Windows::Win32::System::Threading::{
            CreateEventW,
            SetEvent,
            WaitForSingleObject,
        },
        Windows::Win32::Graphics::Direct2D::CLSID_D2D1Shadow,
        Windows::Win32::Graphics::Direct3D12::{
            D3D12_DEFAULT_BLEND_FACTOR_ALPHA
        },
        Windows::Win32::UI::Accessibility::{
            UIA_ScrollPatternNoScroll
        },
        Windows::Win32::Graphics::Hlsl::{
            D3DCOMPILER_DLL
        },
        Windows::Win32::System::WindowsProgramming::{
            CloseHandle
        },
        Windows::Win32::Gaming::HasExpandedResources,
        Windows::Win32::Graphics::Direct3D11::D3DDisassemble11Trace,
        Windows::Win32::Networking::Ldap::ldapsearch,
        Windows::Win32::Storage::StructuredStorage::{CreateStreamOnHGlobal, STREAM_SEEK},
        Windows::Win32::System::Com::CreateUri,
        Windows::Win32::System::Diagnostics::Debug::MiniDumpWriteDump,
        Windows::Win32::System::UpdateAgent::IAutomaticUpdates,
        Windows::Win32::UI::Animation::{UIAnimationManager, UIAnimationTransitionLibrary},
        Windows::Win32::UI::ColorSystem::WhitePoint,
    );
}
