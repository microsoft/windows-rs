#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ITwoPanelHingedDevicePosturePreviewImpl: Sized {
    fn GetCurrentPostureAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreviewReading>>;
    fn PostureChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<TwoPanelHingedDevicePosturePreview, TwoPanelHingedDevicePosturePreviewReadingChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePostureChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ITwoPanelHingedDevicePosturePreviewReadingImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn HingeState(&self) -> ::windows::core::Result<HingeState>;
    fn Panel1Orientation(&self) -> ::windows::core::Result<super::super::Devices::Sensors::SimpleOrientation>;
    fn Panel1Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Panel2Orientation(&self) -> ::windows::core::Result<super::super::Devices::Sensors::SimpleOrientation>;
    fn Panel2Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ITwoPanelHingedDevicePosturePreviewReadingChangedEventArgsImpl: Sized {
    fn Reading(&self) -> ::windows::core::Result<TwoPanelHingedDevicePosturePreviewReading>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ITwoPanelHingedDevicePosturePreviewStaticsImpl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<TwoPanelHingedDevicePosturePreview>>;
}
