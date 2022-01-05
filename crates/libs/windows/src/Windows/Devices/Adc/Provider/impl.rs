pub trait IAdcControllerProviderImpl: Sized {
    fn ChannelCount(&self) -> ::windows::core::Result<i32>;
    fn ResolutionInBits(&self) -> ::windows::core::Result<i32>;
    fn MinValue(&self) -> ::windows::core::Result<i32>;
    fn MaxValue(&self) -> ::windows::core::Result<i32>;
    fn ChannelMode(&self) -> ::windows::core::Result<ProviderAdcChannelMode>;
    fn SetChannelMode(&self, value: ProviderAdcChannelMode) -> ::windows::core::Result<()>;
    fn IsChannelModeSupported(&self, channelmode: ProviderAdcChannelMode) -> ::windows::core::Result<bool>;
    fn AcquireChannel(&self, channel: i32) -> ::windows::core::Result<()>;
    fn ReleaseChannel(&self, channel: i32) -> ::windows::core::Result<()>;
    fn ReadValue(&self, channelnumber: i32) -> ::windows::core::Result<i32>;
}
pub trait IAdcProviderImpl: Sized {
    fn GetControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IAdcControllerProvider>>;
}
