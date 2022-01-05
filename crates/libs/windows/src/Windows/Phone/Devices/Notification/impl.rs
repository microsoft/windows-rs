#[cfg(feature = "implement_exclusive")]
pub trait IVibrationDeviceImpl: Sized {
    fn Vibrate(&self, duration: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVibrationDeviceStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<VibrationDevice>;
}
