fn main() {
    windows::build!(
        windows::foundation::diagnostics::*,
        windows::foundation::collections::*,
        windows::foundation::*,
        windows::ai::machine_learning::*,
        windows::storage::streams::{DataReader, DataWriter, InMemoryRandomAccessStream, DataWriterStoreOperation, DataReaderLoadOperation},
        windows::ui::{Color, Colors},
        windows::ui::composition::{Compositor, SpriteVisual, Visual, CompositionColorBrush, VisualCollection},
        windows::foundation::numerics::*,
        test_component::*,
        windows::ui::xaml::*,
        windows::data::xml::dom::*,
        windows::application_model::appointments::AppointmentDaysOfWeek,
        windows::application_model::activation::{LaunchActivatedEventArgs, IActivatedEventArgs, FileActivatedEventArgs, SearchActivatedEventArgs, ShareTargetActivatedEventArgs, FileOpenPickerActivatedEventArgs, FileSavePickerActivatedEventArgs, CachedFileUpdaterActivatedEventArgs, BackgroundActivatedEventArgs},
        windows::win32::system_services::CreateDispatcherQueueController,
        windows::ui::xaml::interop::TypeName,
        windows::ui::xaml::data::ICustomProperty,

        // Usage of method named `try` when `ICurrencyIdentifiersStatics` is generated
        // This tests that it is escaped.
        windows::globalization::ICurrencyIdentifiersStatics,

        // Test for https://github.com/microsoft/windows-rs/issues/300
        windows::devices::wifi_direct::{WiFiDirectConnectionParameters, WiFiDirectDevice, WiFiDirectDeviceSelectorType},

        // Test for https://github.com/microsoft/windows-rs/issues/280
        windows::application_model::email::EmailAttachment,
        windows::storage::streams::{InMemoryRandomAccessStream, RandomAccessStreamReference},

        // Test for https://github.com/microsoft/windows-rs/issues/361
        windows::ui::xaml::{IApplicationOverrides, IApplicationOverrides2},

        // Test for https://github.com/microsoft/windows-rs/issues/386
        windows::ui::xaml::data::ICustomPropertyProvider,

    );
}
