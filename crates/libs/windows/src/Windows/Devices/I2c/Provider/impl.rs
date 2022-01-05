pub trait II2cControllerProviderImpl: Sized {
    fn GetDeviceProvider(&self, settings: &::core::option::Option<ProviderI2cConnectionSettings>) -> ::windows::core::Result<II2cDeviceProvider>;
}
#[cfg(feature = "Foundation")]
pub trait II2cDeviceProviderImpl: Sized + IClosableImpl {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Write(&self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn WritePartial(&self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<ProviderI2cTransferResult>;
    fn Read(&self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn ReadPartial(&self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<ProviderI2cTransferResult>;
    fn WriteRead(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn WriteReadPartial(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<ProviderI2cTransferResult>;
}
pub trait II2cProviderImpl: Sized {
    fn GetControllersAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<II2cControllerProvider>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProviderI2cConnectionSettingsImpl: Sized {
    fn SlaveAddress(&self) -> ::windows::core::Result<i32>;
    fn SetSlaveAddress(&self, value: i32) -> ::windows::core::Result<()>;
    fn BusSpeed(&self) -> ::windows::core::Result<ProviderI2cBusSpeed>;
    fn SetBusSpeed(&self, value: ProviderI2cBusSpeed) -> ::windows::core::Result<()>;
    fn SharingMode(&self) -> ::windows::core::Result<ProviderI2cSharingMode>;
    fn SetSharingMode(&self, value: ProviderI2cSharingMode) -> ::windows::core::Result<()>;
}
