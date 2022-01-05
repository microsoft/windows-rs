#[cfg(feature = "implement_exclusive")]
pub trait IPwmControllerImpl: Sized {
    fn PinCount(&self) -> ::windows::core::Result<i32>;
    fn ActualFrequency(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredFrequency(&self, desiredfrequency: f64) -> ::windows::core::Result<f64>;
    fn MinFrequency(&self) -> ::windows::core::Result<f64>;
    fn MaxFrequency(&self) -> ::windows::core::Result<f64>;
    fn OpenPin(&self, pinnumber: i32) -> ::windows::core::Result<PwmPin>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPwmControllerStaticsImpl: Sized {
    fn GetControllersAsync(&self, provider: &::core::option::Option<Provider::IPwmProvider>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PwmController>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPwmControllerStatics2Impl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PwmController>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPwmControllerStatics3Impl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromFriendlyName(&self, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PwmController>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPwmPinImpl: Sized + IClosableImpl {
    fn Controller(&self) -> ::windows::core::Result<PwmController>;
    fn GetActiveDutyCyclePercentage(&self) -> ::windows::core::Result<f64>;
    fn SetActiveDutyCyclePercentage(&self, dutycyclepercentage: f64) -> ::windows::core::Result<()>;
    fn Polarity(&self) -> ::windows::core::Result<PwmPulsePolarity>;
    fn SetPolarity(&self, value: PwmPulsePolarity) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn IsStarted(&self) -> ::windows::core::Result<bool>;
}
