#[cfg(feature = "implement_exclusive")]
pub trait IDeviceServicingDetailsImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExpectedDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceUseDetailsImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
