#[cfg(feature = "implement_exclusive")]
pub trait ISpiBusInfoImpl: Sized {
    fn ChipSelectLineCount(&self) -> ::windows::core::Result<i32>;
    fn MinClockFrequency(&self) -> ::windows::core::Result<i32>;
    fn MaxClockFrequency(&self) -> ::windows::core::Result<i32>;
    fn SupportedDataBitLengths(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpiConnectionSettingsImpl: Sized {
    fn ChipSelectLine(&self) -> ::windows::core::Result<i32>;
    fn SetChipSelectLine(&self, value: i32) -> ::windows::core::Result<()>;
    fn Mode(&self) -> ::windows::core::Result<SpiMode>;
    fn SetMode(&self, value: SpiMode) -> ::windows::core::Result<()>;
    fn DataBitLength(&self) -> ::windows::core::Result<i32>;
    fn SetDataBitLength(&self, value: i32) -> ::windows::core::Result<()>;
    fn ClockFrequency(&self) -> ::windows::core::Result<i32>;
    fn SetClockFrequency(&self, value: i32) -> ::windows::core::Result<()>;
    fn SharingMode(&self) -> ::windows::core::Result<SpiSharingMode>;
    fn SetSharingMode(&self, value: SpiSharingMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpiConnectionSettingsFactoryImpl: Sized {
    fn Create(&self, chipselectline: i32) -> ::windows::core::Result<SpiConnectionSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpiControllerImpl: Sized {
    fn GetDevice(&self, settings: &::core::option::Option<SpiConnectionSettings>) -> ::windows::core::Result<SpiDevice>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISpiControllerStaticsImpl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpiController>>;
    fn GetControllersAsync(&self, provider: &::core::option::Option<Provider::ISpiProvider>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<SpiController>>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISpiDeviceImpl: Sized + IClosableImpl {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ConnectionSettings(&self) -> ::windows::core::Result<SpiConnectionSettings>;
    fn Write(&self, buffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn Read(&self, buffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn TransferSequential(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn TransferFullDuplex(&self, writebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType], readbuffer: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
pub trait ISpiDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromFriendlyName(&self, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetBusInfo(&self, busid: &::windows::core::HSTRING) -> ::windows::core::Result<SpiBusInfo>;
    fn FromIdAsync(&self, busid: &::windows::core::HSTRING, settings: &::core::option::Option<SpiConnectionSettings>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SpiDevice>>;
}
