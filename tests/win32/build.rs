fn main() {
    windows::build! {
        Windows::Win32::Gaming::HasExpandedResources,
        Windows::Win32::Graphics::Direct2D::CLSID_D2D1Shadow,
        Windows::Win32::Graphics::Direct3D11::D3DDisassemble11Trace,
        Windows::Win32::Graphics::Direct3D12::D3D12_DEFAULT_BLEND_FACTOR_ALPHA,
        Windows::Win32::Graphics::Dxgi::{
            CreateDXGIFactory1, IDXGIFactory7, DXGI_ADAPTER_FLAG, DXGI_ADAPTER_FLAG_REMOTE,
            DXGI_ADAPTER_FLAG_SOFTWARE, DXGI_ERROR_INVALID_CALL, DXGI_FORMAT,
            DXGI_FORMAT_R32_TYPELESS, DXGI_MODE_DESC, DXGI_MODE_SCALING,
            DXGI_MODE_SCALING_CENTERED, DXGI_MODE_SCANLINE_ORDER,
            DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE, DXGI_RATIONAL,
        },
        Windows::Win32::Graphics::Hlsl::D3DCOMPILER_DLL,
        Windows::Win32::Networking::Ldap::ldapsearch,
        Windows::Win32::Security::REVOKE_ACCESS,
        Windows::Win32::Storage::StructuredStorage::{CreateStreamOnHGlobal, STREAM_SEEK_SET},
        Windows::Win32::System::Com::CreateUri,
        Windows::Win32::System::Diagnostics::Debug::{MiniDumpNormal, MiniDumpWriteDump},
        Windows::Win32::System::OleAutomation::BSTR,
        Windows::Win32::System::Threading::{
            CreateEventW, SetEvent, WaitForSingleObject, WAIT_OBJECT_0,
        },
        Windows::Win32::System::UpdateAgent::IAutomaticUpdates,
        Windows::Win32::System::WindowsProgramming::CloseHandle,
        Windows::Win32::UI::Accessibility::UIA_ScrollPatternNoScroll,
        Windows::Win32::UI::Animation::{UIAnimationManager, UIAnimationTransitionLibrary},
        Windows::Win32::UI::ColorSystem::WhitePoint,
        Windows::Win32::UI::DisplayDevices::RECT,
        Windows::Win32::UI::WindowsAndMessaging::{
            CHOOSECOLORW, PROPENUMPROCA, PROPENUMPROCW, WM_KEYUP,
        },
    };
}
