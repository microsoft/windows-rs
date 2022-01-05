#[cfg(feature = "implement_exclusive")]
pub trait IVariablePhotoCapturedEventArgsImpl: Sized {
    fn Frame(&self) -> ::windows::core::Result<super::CapturedFrame>;
    fn CaptureTimeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn UsedFrameControllerIndex(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn CapturedFrameControlValues(&self) -> ::windows::core::Result<super::CapturedFrameControlValues>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVariablePhotoSequenceCaptureImpl: Sized {
    fn StartAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn StopAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn FinishAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn PhotoCaptured(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<VariablePhotoSequenceCapture, VariablePhotoCapturedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePhotoCaptured(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<VariablePhotoSequenceCapture, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVariablePhotoSequenceCapture2Impl: Sized {
    fn UpdateSettingsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
