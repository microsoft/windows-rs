#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGpioChangeCounterImpl: Sized + IClosableImpl {
    fn SetPolarity(&self, value: GpioChangePolarity) -> ::windows::core::Result<()>;
    fn Polarity(&self) -> ::windows::core::Result<GpioChangePolarity>;
    fn IsStarted(&self) -> ::windows::core::Result<bool>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Read(&self) -> ::windows::core::Result<GpioChangeCount>;
    fn Reset(&self) -> ::windows::core::Result<GpioChangeCount>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioChangeCounterFactoryImpl: Sized {
    fn Create(&self, pin: &::core::option::Option<GpioPin>) -> ::windows::core::Result<GpioChangeCounter>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGpioChangeReaderImpl: Sized + IClosableImpl {
    fn Capacity(&self) -> ::windows::core::Result<i32>;
    fn Length(&self) -> ::windows::core::Result<i32>;
    fn IsEmpty(&self) -> ::windows::core::Result<bool>;
    fn IsOverflowed(&self) -> ::windows::core::Result<bool>;
    fn SetPolarity(&self, value: GpioChangePolarity) -> ::windows::core::Result<()>;
    fn Polarity(&self) -> ::windows::core::Result<GpioChangePolarity>;
    fn IsStarted(&self) -> ::windows::core::Result<bool>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn GetNextItem(&self) -> ::windows::core::Result<GpioChangeRecord>;
    fn PeekNextItem(&self) -> ::windows::core::Result<GpioChangeRecord>;
    fn GetAllItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<GpioChangeRecord>>;
    fn WaitForItemsAsync(&self, count: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioChangeReaderFactoryImpl: Sized {
    fn Create(&self, pin: &::core::option::Option<GpioPin>) -> ::windows::core::Result<GpioChangeReader>;
    fn CreateWithCapacity(&self, pin: &::core::option::Option<GpioPin>, mincapacity: i32) -> ::windows::core::Result<GpioChangeReader>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioControllerImpl: Sized {
    fn PinCount(&self) -> ::windows::core::Result<i32>;
    fn OpenPin(&self, pinnumber: i32) -> ::windows::core::Result<GpioPin>;
    fn OpenPinWithSharingMode(&self, pinnumber: i32, sharingmode: GpioSharingMode) -> ::windows::core::Result<GpioPin>;
    fn TryOpenPin(&self, pinnumber: i32, sharingmode: GpioSharingMode, pin: &mut ::core::option::Option<GpioPin>, openstatus: &mut GpioOpenStatus) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioControllerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<GpioController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioControllerStatics2Impl: Sized {
    fn GetControllersAsync(&self, provider: &::core::option::Option<Provider::IGpioProvider>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<GpioController>>>;
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GpioController>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IGpioPinImpl: Sized + IClosableImpl {
    fn ValueChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GpioPin, GpioPinValueChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DebounceTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDebounceTimeout(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn PinNumber(&self) -> ::windows::core::Result<i32>;
    fn SharingMode(&self) -> ::windows::core::Result<GpioSharingMode>;
    fn IsDriveModeSupported(&self, drivemode: GpioPinDriveMode) -> ::windows::core::Result<bool>;
    fn GetDriveMode(&self) -> ::windows::core::Result<GpioPinDriveMode>;
    fn SetDriveMode(&self, value: GpioPinDriveMode) -> ::windows::core::Result<()>;
    fn Write(&self, value: GpioPinValue) -> ::windows::core::Result<()>;
    fn Read(&self) -> ::windows::core::Result<GpioPinValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGpioPinValueChangedEventArgsImpl: Sized {
    fn Edge(&self) -> ::windows::core::Result<GpioPinEdge>;
}
