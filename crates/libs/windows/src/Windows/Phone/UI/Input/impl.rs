#[cfg(feature = "implement_exclusive")]
pub trait IBackPressedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IHardwareButtonsStaticsImpl: Sized {
    fn BackPressed(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<BackPressedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveBackPressed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHardwareButtonsStatics2Impl: Sized {
    fn CameraHalfPressed(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<CameraEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraHalfPressed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraPressed(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<CameraEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraPressed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraReleased(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<CameraEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraReleased(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
