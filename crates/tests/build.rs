fn main() {
    winrt::build!(
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

        // Test for https://github.com/microsoft/winrt-rs/issues/300
        windows::devices::wifi_direct::WiFiDirectDevice

        // Test for https://github.com/microsoft/winrt-rs/issues/280
        windows::application_model::email::EmailAttachment
        windows::storage::streams::{InMemoryRandomAccessStream, RandomAccessStreamReference}

        // Test for https://github.com/microsoft/winrt-rs/issues/361
        windows::ui::xaml::{IApplicationOverrides, IApplicationOverrides2}

        // Used to test Win32 support
        windows::win32::{
            ACCESS_MODE, CHOOSECOLORW, DXGI_ADAPTER_FLAG, DXGI_FORMAT, DXGI_MODE_DESC, DXGI_MODE_SCALING,
            DXGI_MODE_SCANLINE_ORDER, DXGI_RATIONAL, RECT,
        }
    );
}
