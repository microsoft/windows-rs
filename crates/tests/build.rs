fn main() {
    windows::build!(
        windows::foundation::diagnostics::*,
        windows::foundation::*,
        windows::ai::machine_learning::*,
        windows::storage::streams::{DataReader, DataWriter, InMemoryRandomAccessStream},
        windows::ui::{Color, Colors},
        windows::ui::composition::{Compositor, SpriteVisual, Visual},
        windows::foundation::numerics::*,
        test_component::*,
        windows::ui::xaml::*,
        windows::data::xml::dom::*,
        windows::application_model::appointments::AppointmentDaysOfWeek,

        // Usage of method named `try` when `ICurrencyIdentifiersStatics` is generated
        // This tests that it is escaped.
        windows::globalization::ICurrencyIdentifiersStatics,

        // Test for https://github.com/microsoft/windows-rs/issues/300
        windows::devices::wifi_direct::WiFiDirectDevice,

        // Test for https://github.com/microsoft/windows-rs/issues/280
        windows::application_model::email::EmailAttachment,
        windows::storage::streams::{InMemoryRandomAccessStream, RandomAccessStreamReference},

        // Test for https://github.com/microsoft/windows-rs/issues/361
        windows::ui::xaml::{IApplicationOverrides, IApplicationOverrides2},

        // Test for https://github.com/microsoft/windows-rs/issues/386
        windows::ui::xaml::data::ICustomPropertyProvider,

        windows::win32::security::{
            ACCESS_MODE,
        },
        windows::win32::windows_and_messaging::{
            CHOOSECOLORW,
            PROPENUMPROCA,
            PROPENUMPROCW,
        },
        windows::win32::dxgi::{
            DXGI_ADAPTER_FLAG, DXGI_FORMAT, DXGI_MODE_DESC, DXGI_MODE_SCALING,
            DXGI_MODE_SCANLINE_ORDER, DXGI_RATIONAL,IDXGIFactory7, CreateDXGIFactory1
        },
        windows::win32::display_devices::{
            RECT,
        },
        windows::win32::system_services::{
            CreateEventW,
            SetEvent,
            WaitForSingleObject,
            WM_KEYUP,
            DXGI_ERROR_INVALID_CALL,
        },
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
        windows::win32::upnp::UIAnimationTransitionLibrary,
        windows::win32::ldap::ldapsearch,
        windows::win32::upnp::UIAnimationManager,
        windows::win32::game_mode::HasExpandedResources,
        windows::win32::debug::MiniDumpWriteDump,
        windows::win32::direct3d11::D3DDisassemble11Trace,
        windows::win32::windows_update_agent::IAutomaticUpdates,
    );
}
