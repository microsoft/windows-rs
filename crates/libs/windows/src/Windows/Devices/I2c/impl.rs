#[cfg(feature = "implement_exclusive")]
pub trait II2cConnectionSettingsImpl: Sized {
    fn SlaveAddress(&self) -> ::windows::core::Result<i32>;
    fn SetSlaveAddress(&self, value: i32) -> ::windows::core::Result<()>;
    fn BusSpeed(&self) -> ::windows::core::Result<I2cBusSpeed>;
    fn SetBusSpeed(&self, value: I2cBusSpeed) -> ::windows::core::Result<()>;
    fn SharingMode(&self) -> ::windows::core::Result<I2cSharingMode>;
    fn SetSharingMode(&self, value: I2cSharingMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait II2cConnectionSettingsFactoryImpl: Sized {
    fn Create(&self, slaveaddress: i32) -> ::windows::core::Result<I2cConnectionSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait II2cControllerImpl: Sized {
    fn GetDevice(&self, settings: &::core::option::Option<I2cConnectionSettings>) -> ::windows::core::Result<I2cDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait II2cControllerStaticsImpl: Sized {
    fn GetControllersAsync(&self, provider: &::core::option::Option<Provider::II2cProvider>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<I2cController>>>;
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<I2cController>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait II2cDeviceImpl: Sized + IClosableImpl {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionSettings(&self) -> ::windows::core::Result<I2cConnectionSettings>;
    fn Write(&self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn WritePartial(&self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<I2cTransferResult>;
    fn Read(&self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn ReadPartial(&self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<I2cTransferResult>;
    fn WriteRead(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn WriteReadPartial(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<I2cTransferResult>;
}
pub trait II2cDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromFriendlyName(&self, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING, settings: &::core::option::Option<I2cConnectionSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<I2cDevice>>;
}
