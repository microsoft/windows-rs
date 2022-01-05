pub trait IGpioControllerProviderImpl: Sized {
    fn PinCount(&self) -> ::windows::core::Result<i32>;
    fn OpenPinProvider(&self, pin: i32, sharingmode: ProviderGpioSharingMode) -> ::windows::core::Result<IGpioPinProvider>;
}
pub trait IGpioPinProviderImpl: Sized {
    fn ValueChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<IGpioPinProvider, GpioPinProviderValueChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DebounceTimeout(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDebounceTimeout(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn PinNumber(&self) -> ::windows::core::Result<i32>;
    fn SharingMode(&self) -> ::windows::core::Result<ProviderGpioSharingMode>;
    fn IsDriveModeSupported(&self, drivemode: ProviderGpioPinDriveMode) -> ::windows::core::Result<bool>;
    fn GetDriveMode(&self) -> ::windows::core::Result<ProviderGpioPinDriveMode>;
    fn SetDriveMode(&self, value: ProviderGpioPinDriveMode) -> ::windows::core::Result<()>;
    fn Write(&self, value: ProviderGpioPinValue) -> ::windows::core::Result<()>;
    fn Read(&self) -> ::windows::core::Result<ProviderGpioPinValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioPinProviderValueChangedEventArgsImpl: Sized {
    fn Edge(&self) -> ::windows::core::Result<ProviderGpioPinEdge>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioPinProviderValueChangedEventArgsFactoryImpl: Sized {
    fn Create(&self, edge: ProviderGpioPinEdge) -> ::windows::core::Result<GpioPinProviderValueChangedEventArgs>;
}
pub trait IGpioProviderImpl: Sized {
    fn GetControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<IGpioControllerProvider>>;
}
