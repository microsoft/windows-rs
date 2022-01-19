fn main() {
    windows::core::build_legacy! {
        // Test for https://github.com/microsoft/windows-rs/issues/1055
        Windows::Win32::Data::Xml::MsXml::IMXWriter,

        // Test for https://github.com/microsoft/windows-rs/issues/1055
        Windows::Win32::Devices::DeviceQuery::DevCreateObjectQuery,

        // Test for GetRepresentation which has a false-positive for query pattern
        Windows::Win32::Media::MediaFoundation::IMFMediaType,

        // Test for https://github.com/microsoft/windows-rs/issues/1055
        Windows::Win32::Media::Speech::ISpPhoneConverter,

        // Test for https://github.com/microsoft/win32metadata/issues/449
        Windows::Win32::System::DistributedTransactionCoordinator::ITransactionImport,

        // Test for https://github.com/microsoft/windows-rs/issues/924
        Windows::Win32::System::WinRT::Composition::ICompositorInterop,
        Windows::Win32::System::WinRT::ISystemMediaTransportControlsInterop,

        // Test for https://github.com/microsoft/windows-rs/issues/1055
        Windows::Win32::AI::MachineLearning::WinML::IWinMLEvaluationContext,
    };
}
