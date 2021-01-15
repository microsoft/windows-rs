fn main() {
    windows::build!(
        windows::foundation::diagnostics::*
        windows::foundation::*
        windows::ai::machine_learning::*
        windows::storage::streams::{DataReader, DataWriter, InMemoryRandomAccessStream}
        windows::ui::{Color, Colors}
        windows::ui::composition::{Compositor, SpriteVisual, Visual}
        windows::foundation::numerics::*
        test_component::*
        windows::ui::xaml::*
        windows::data::xml::dom::*
        windows::application_model::appointments::AppointmentDaysOfWeek

        // Usage of method named `try` when `ICurrencyIdentifiersStatics` is generated
        // This tests that it is escaped.
        windows::globalization::ICurrencyIdentifiersStatics

        // Test for https://github.com/microsoft/windows-rs/issues/300
        windows::devices::wifi_direct::WiFiDirectDevice

        // Test for https://github.com/microsoft/windows-rs/issues/280
        windows::application_model::email::EmailAttachment
        windows::storage::streams::{InMemoryRandomAccessStream, RandomAccessStreamReference}

        // Test for https://github.com/microsoft/windows-rs/issues/361
        windows::ui::xaml::{IApplicationOverrides, IApplicationOverrides2}

        // Test for https://github.com/microsoft/windows-rs/issues/386
        windows::ui::xaml::data::ICustomPropertyProvider

        windows::win32::security::{
            ACCESS_MODE,
        }
        windows::win32::dlg_box::{
            CHOOSECOLORW,
        }
        windows::win32::direct3d_dxgi::{
            DXGI_ADAPTER_FLAG, DXGI_FORMAT, DXGI_MODE_DESC, DXGI_MODE_SCALING,
            DXGI_MODE_SCANLINE_ORDER, DXGI_RATIONAL,
        }
        windows::win32::backup::{
            RECT,
            CreateEventW,
            SetEvent,
            WaitForSingleObject,
        }
        windows::win32::base::{
            WM_KEYUP
        }
        windows::win32::direct3d12::{
            D3D12_DEFAULT_BLEND_FACTOR_ALPHA
        }
        windows::win32::win_auto::{
            UIA_ScrollPatternNoScroll
        }
        windows::win32::direct3d_hlsl::{
            D3DCOMPILER_DLL
        }
        windows::win32::win_prog::{
            CloseHandle
        }
        windows::win32::com::{IUri, CreateUri}
        windows::win32::menu_rc::{PROPENUMPROCA, PROPENUMPROCW}
    );
}
