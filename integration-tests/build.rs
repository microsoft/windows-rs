fn main() {
    winrt::build!(
        types
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
    );
}
