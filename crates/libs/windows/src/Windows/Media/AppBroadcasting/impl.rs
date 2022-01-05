#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastingMonitorImpl: Sized {
    fn IsCurrentAppBroadcasting(&self) -> ::windows::core::Result<bool>;
    fn IsCurrentAppBroadcastingChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppBroadcastingMonitor, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsCurrentAppBroadcastingChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastingStatusImpl: Sized {
    fn CanStartBroadcast(&self) -> ::windows::core::Result<bool>;
    fn Details(&self) -> ::windows::core::Result<AppBroadcastingStatusDetails>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastingStatusDetailsImpl: Sized {
    fn IsAnyAppBroadcasting(&self) -> ::windows::core::Result<bool>;
    fn IsCaptureResourceUnavailable(&self) -> ::windows::core::Result<bool>;
    fn IsGameStreamInProgress(&self) -> ::windows::core::Result<bool>;
    fn IsGpuConstrained(&self) -> ::windows::core::Result<bool>;
    fn IsAppInactive(&self) -> ::windows::core::Result<bool>;
    fn IsBlockedForApp(&self) -> ::windows::core::Result<bool>;
    fn IsDisabledByUser(&self) -> ::windows::core::Result<bool>;
    fn IsDisabledBySystem(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastingUIImpl: Sized {
    fn GetStatus(&self) -> ::windows::core::Result<AppBroadcastingStatus>;
    fn ShowBroadcastUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastingUIStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<AppBroadcastingUI>;
    fn GetForUser(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<AppBroadcastingUI>;
}
