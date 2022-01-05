pub trait IPwmControllerProviderImpl: Sized {
    fn PinCount(&self) -> ::windows::core::Result<i32>;
    fn ActualFrequency(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredFrequency(&self, frequency: f64) -> ::windows::core::Result<f64>;
    fn MaxFrequency(&self) -> ::windows::core::Result<f64>;
    fn MinFrequency(&self) -> ::windows::core::Result<f64>;
    fn AcquirePin(&self, pin: i32) -> ::windows::core::Result<()>;
    fn ReleasePin(&self, pin: i32) -> ::windows::core::Result<()>;
    fn EnablePin(&self, pin: i32) -> ::windows::core::Result<()>;
    fn DisablePin(&self, pin: i32) -> ::windows::core::Result<()>;
    fn SetPulseParameters(&self, pin: i32, dutycycle: f64, invertpolarity: bool) -> ::windows::core::Result<()>;
}
pub trait IPwmProviderImpl: Sized {
    fn GetControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IPwmControllerProvider>>;
}
