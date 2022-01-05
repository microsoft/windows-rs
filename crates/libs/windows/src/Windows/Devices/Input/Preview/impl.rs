#[cfg(feature = "implement_exclusive")]
pub trait IGazeDevicePreviewImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn CanTrackEyes(&self) -> ::windows::core::Result<bool>;
    fn CanTrackHead(&self) -> ::windows::core::Result<bool>;
    fn ConfigurationState(&self) -> ::windows::core::Result<GazeDeviceConfigurationStatePreview>;
    fn RequestCalibrationAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn GetNumericControlDescriptions(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidNumericControlDescription>>;
    fn GetBooleanControlDescriptions(&self, usagepage: u16, usageid: u16) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::HumanInterfaceDevice::HidBooleanControlDescription>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGazeDeviceWatcherAddedPreviewEventArgsImpl: Sized {
    fn Device(&self) -> ::windows::core::Result<GazeDevicePreview>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGazeDeviceWatcherPreviewImpl: Sized {
    fn Added(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherAddedPreviewEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherRemovedPreviewEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, GazeDeviceWatcherUpdatedPreviewEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GazeDeviceWatcherPreview, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGazeDeviceWatcherRemovedPreviewEventArgsImpl: Sized {
    fn Device(&self) -> ::windows::core::Result<GazeDevicePreview>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGazeDeviceWatcherUpdatedPreviewEventArgsImpl: Sized {
    fn Device(&self) -> ::windows::core::Result<GazeDevicePreview>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGazeEnteredPreviewEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CurrentPoint(&self) -> ::windows::core::Result<GazePointPreview>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGazeExitedPreviewEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CurrentPoint(&self) -> ::windows::core::Result<GazePointPreview>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGazeInputSourcePreviewImpl: Sized {
    fn GazeMoved(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeMovedPreviewEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGazeMoved(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GazeEntered(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeEnteredPreviewEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGazeEntered(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GazeExited(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GazeInputSourcePreview, GazeExitedPreviewEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGazeExited(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGazeInputSourcePreviewStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<GazeInputSourcePreview>;
    fn CreateWatcher(&self) -> ::windows::core::Result<GazeDeviceWatcherPreview>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGazeMovedPreviewEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CurrentPoint(&self) -> ::windows::core::Result<GazePointPreview>;
    fn GetIntermediatePoints(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<GazePointPreview>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGazePointPreviewImpl: Sized {
    fn SourceDevice(&self) -> ::windows::core::Result<GazeDevicePreview>;
    fn EyeGazePosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>>;
    fn HeadGazePosition(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::Point>>;
    fn Timestamp(&self) -> ::windows::core::Result<u64>;
    fn HidInputReport(&self) -> ::windows::core::Result<super::super::HumanInterfaceDevice::HidInputReport>;
}
