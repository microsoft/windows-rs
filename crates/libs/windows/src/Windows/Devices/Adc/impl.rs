#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAdcChannelImpl: Sized + IClosableImpl {
    fn Controller(&self) -> ::windows::core::Result<AdcController>;
    fn ReadValue(&self) -> ::windows::core::Result<i32>;
    fn ReadRatio(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdcControllerImpl: Sized {
    fn ChannelCount(&self) -> ::windows::core::Result<i32>;
    fn ResolutionInBits(&self) -> ::windows::core::Result<i32>;
    fn MinValue(&self) -> ::windows::core::Result<i32>;
    fn MaxValue(&self) -> ::windows::core::Result<i32>;
    fn ChannelMode(&self) -> ::windows::core::Result<AdcChannelMode>;
    fn SetChannelMode(&self, value: AdcChannelMode) -> ::windows::core::Result<()>;
    fn IsChannelModeSupported(&self, channelmode: AdcChannelMode) -> ::windows::core::Result<bool>;
    fn OpenChannel(&self, channelnumber: i32) -> ::windows::core::Result<AdcChannel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdcControllerStaticsImpl: Sized {
    fn GetControllersAsync(&self, provider: &::core::option::Option<Provider::IAdcProvider>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<AdcController>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdcControllerStatics2Impl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AdcController>>;
}
