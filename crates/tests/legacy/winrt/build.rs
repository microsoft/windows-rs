fn main() {
    windows::core::build! {
        Windows::ApplicationModel::Activation::{BackgroundActivatedEventArgs, CachedFileUpdaterActivatedEventArgs, FileActivatedEventArgs, FileOpenPickerActivatedEventArgs, FileSavePickerActivatedEventArgs, IActivatedEventArgs, LaunchActivatedEventArgs, SearchActivatedEventArgs, ShareTargetActivatedEventArgs},
        Windows::ApplicationModel::Appointments::AppointmentDaysOfWeek,
        // Test for https://github.com/microsoft/windows-rs/issues/280
        Windows::ApplicationModel::Email::EmailAttachment,
        Windows::Data::Xml::Dom::*,
        // Test for https://github.com/microsoft/windows-rs/issues/300
        Windows::Devices::WiFiDirect::{WiFiDirectConnectionParameters, WiFiDirectDevice, WiFiDirectDeviceSelectorType},

        Windows::Foundation::Collections::*,
        Windows::Foundation::Diagnostics::*,
        Windows::Foundation::Numerics::*,
        Windows::Foundation::*,
        // Usage of method named `try` when `ICurrencyIdentifiersStatics` is generated
        // This tests that it is escaped.
        Windows::Globalization::ICurrencyIdentifiersStatics,

        Windows::Storage::Streams::{DataReader, DataReaderLoadOperation, DataWriter, DataWriterStoreOperation, InMemoryRandomAccessStream, RandomAccessStreamReference},

        Windows::Win32::Foundation::E_NOINTERFACE,
        Windows::Win32::System::WinRT::CreateDispatcherQueueController,
        Windows::AI::MachineLearning::*,
        Windows::UI::Composition::{CompositionColorBrush, Compositor, SpriteVisual, Visual, VisualCollection},
        Windows::UI::Xaml::Data::ICustomProperty,

        // Test for https://github.com/microsoft/windows-rs/issues/386
        Windows::UI::Xaml::Data::ICustomPropertyProvider,
        Windows::UI::Xaml::Interop::TypeName,
        Windows::UI::Xaml::*,
        // Test for https://github.com/microsoft/windows-rs/issues/361
        Windows::UI::Xaml::{IApplicationOverrides, IApplicationOverrides2},

        Windows::UI::{Color, Colors},
    };
}
