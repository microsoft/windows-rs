fn main() {
    windows::build!(
        Windows::Foundation::Diagnostics::*,
        Windows::Foundation::Collections::*,
        Windows::Foundation::*,
        Windows::AI::MachineLearning::*,
        Windows::Storage::Streams::{
            DataReader, DataWriter, InMemoryRandomAccessStream, DataWriterStoreOperation,
            DataReaderLoadOperation
        },
        Windows::UI::{Color, Colors},
        Windows::UI::Composition::{Compositor, SpriteVisual, Visual, CompositionColorBrush, VisualCollection},
        Windows::Foundation::Numerics::*,
        TestComponent::*,
        Windows::UI::Xaml::*,
        Windows::Data::Xml::Dom::*,
        Windows::ApplicationModel::Appointments::AppointmentDaysOfWeek,
        Windows::ApplicationModel::Activation::{
            LaunchActivatedEventArgs, IActivatedEventArgs, FileActivatedEventArgs,SearchActivatedEventArgs,
            ShareTargetActivatedEventArgs, FileOpenPickerActivatedEventArgs, FileSavePickerActivatedEventArgs,
            CachedFileUpdaterActivatedEventArgs, BackgroundActivatedEventArgs,
        },
        Windows::Win32::Foundation::{E_POINTER, E_NOINTERFACE},
        Windows::Win32::System::WinRT::{
            CreateDispatcherQueueController, DQTAT_COM_NONE, DQTYPE_THREAD_CURRENT
        },
        Windows::UI::Xaml::Interop::TypeName,
        Windows::UI::Xaml::Data::ICustomProperty,

        // Usage of method named `try` when `ICurrencyIdentifiersStatics` is generated
        // This tests that it is escaped.
        Windows::Globalization::ICurrencyIdentifiersStatics,

        // Test for https://github.com/microsoft/windows-rs/issues/300
        Windows::Devices::WiFiDirect::{WiFiDirectConnectionParameters, WiFiDirectDevice, WiFiDirectDeviceSelectorType},

        // Test for https://github.com/microsoft/windows-rs/issues/280
        Windows::ApplicationModel::Email::EmailAttachment,
        Windows::Storage::Streams::{InMemoryRandomAccessStream, RandomAccessStreamReference},

        // Test for https://github.com/microsoft/windows-rs/issues/361
        Windows::UI::Xaml::{IApplicationOverrides, IApplicationOverrides2},

        // Test for https://github.com/microsoft/windows-rs/issues/386
        Windows::UI::Xaml::Data::ICustomPropertyProvider,

    );
}
