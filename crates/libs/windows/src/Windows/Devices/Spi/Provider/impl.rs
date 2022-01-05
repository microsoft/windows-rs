#[cfg(feature = "implement_exclusive")]
pub trait IProviderSpiConnectionSettingsImpl: Sized {
    fn ChipSelectLine(&self) -> ::windows::core::Result<i32>;
    fn SetChipSelectLine(&self, value: i32) -> ::windows::core::Result<()>;
    fn Mode(&self) -> ::windows::core::Result<ProviderSpiMode>;
    fn SetMode(&self, value: ProviderSpiMode) -> ::windows::core::Result<()>;
    fn DataBitLength(&self) -> ::windows::core::Result<i32>;
    fn SetDataBitLength(&self, value: i32) -> ::windows::core::Result<()>;
    fn ClockFrequency(&self) -> ::windows::core::Result<i32>;
    fn SetClockFrequency(&self, value: i32) -> ::windows::core::Result<()>;
    fn SharingMode(&self) -> ::windows::core::Result<ProviderSpiSharingMode>;
    fn SetSharingMode(&self, value: ProviderSpiSharingMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProviderSpiConnectionSettingsFactoryImpl: Sized {
    fn Create(&self, chipselectline: i32) -> ::windows::core::Result<ProviderSpiConnectionSettings>;
}
pub trait ISpiControllerProviderImpl: Sized {
    fn GetDeviceProvider(&self, settings: &::core::option::Option<ProviderSpiConnectionSettings>) -> ::windows::core::Result<ISpiDeviceProvider>;
}
#[cfg(feature = "Foundation")]
pub trait ISpiDeviceProviderImpl: Sized + IClosableImpl {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionSettings(&self) -> ::windows::core::Result<ProviderSpiConnectionSettings>;
    fn Write(&self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn Read(&self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn TransferSequential(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn TransferFullDuplex(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
pub trait ISpiProviderImpl: Sized {
    fn GetControllersAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<ISpiControllerProvider>>>;
}
