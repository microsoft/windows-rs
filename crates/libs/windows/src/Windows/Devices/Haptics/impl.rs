#[cfg(feature = "implement_exclusive")]
pub trait IKnownSimpleHapticsControllerWaveformsStaticsImpl: Sized {
    fn Click(&self) -> ::windows::core::Result<u16>;
    fn BuzzContinuous(&self) -> ::windows::core::Result<u16>;
    fn RumbleContinuous(&self) -> ::windows::core::Result<u16>;
    fn Press(&self) -> ::windows::core::Result<u16>;
    fn Release(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownSimpleHapticsControllerWaveformsStatics2Impl: Sized {
    fn BrushContinuous(&self) -> ::windows::core::Result<u16>;
    fn ChiselMarkerContinuous(&self) -> ::windows::core::Result<u16>;
    fn EraserContinuous(&self) -> ::windows::core::Result<u16>;
    fn Error(&self) -> ::windows::core::Result<u16>;
    fn GalaxyPenContinuous(&self) -> ::windows::core::Result<u16>;
    fn Hover(&self) -> ::windows::core::Result<u16>;
    fn InkContinuous(&self) -> ::windows::core::Result<u16>;
    fn MarkerContinuous(&self) -> ::windows::core::Result<u16>;
    fn PencilContinuous(&self) -> ::windows::core::Result<u16>;
    fn Success(&self) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISimpleHapticsControllerImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedFeedback(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SimpleHapticsControllerFeedback>>;
    fn IsIntensitySupported(&self) -> ::windows::core::Result<bool>;
    fn IsPlayCountSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPlayDurationSupported(&self) -> ::windows::core::Result<bool>;
    fn IsReplayPauseIntervalSupported(&self) -> ::windows::core::Result<bool>;
    fn StopFeedback(&self) -> ::windows::core::Result<()>;
    fn SendHapticFeedback(&self, feedback: &::core::option::Option<SimpleHapticsControllerFeedback>) -> ::windows::core::Result<()>;
    fn SendHapticFeedbackWithIntensity(&self, feedback: &::core::option::Option<SimpleHapticsControllerFeedback>, intensity: f64) -> ::windows::core::Result<()>;
    fn SendHapticFeedbackForDuration(&self, feedback: &::core::option::Option<SimpleHapticsControllerFeedback>, intensity: f64, playduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SendHapticFeedbackForPlayCount(&self, feedback: &::core::option::Option<SimpleHapticsControllerFeedback>, intensity: f64, playcount: i32, replaypauseinterval: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISimpleHapticsControllerFeedbackImpl: Sized {
    fn Waveform(&self) -> ::windows::core::Result<u16>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVibrationDeviceImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SimpleHapticsController(&self) -> ::windows::core::Result<SimpleHapticsController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVibrationDeviceStaticsImpl: Sized {
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VibrationAccessStatus>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VibrationDevice>>;
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VibrationDevice>>;
    fn FindAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<VibrationDevice>>>;
}
