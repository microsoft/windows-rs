#[cfg(feature = "implement_exclusive")]
pub trait ICustomDeviceImpl: Sized {
    fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn SendIOControlAsync(&self, iocontrolcode: &::core::option::Option<IIOControlCode>, inputbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>, outputbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn TrySendIOControlAsync(&self, iocontrolcode: &::core::option::Option<IIOControlCode>, inputbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>, outputbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self, classguid: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING, desiredaccess: DeviceAccessMode, sharingmode: DeviceSharingMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CustomDevice>>;
}
pub trait IIOControlCodeImpl: Sized {
    fn AccessMode(&self) -> ::windows::core::Result<IOControlAccessMode>;
    fn BufferingMethod(&self) -> ::windows::core::Result<IOControlBufferingMethod>;
    fn Function(&self) -> ::windows::core::Result<u16>;
    fn DeviceType(&self) -> ::windows::core::Result<u16>;
    fn ControlCode(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIOControlCodeFactoryImpl: Sized {
    fn CreateIOControlCode(&self, devicetype: u16, function: u16, accessmode: IOControlAccessMode, bufferingmethod: IOControlBufferingMethod) -> ::windows::core::Result<IOControlCode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKnownDeviceTypesStaticsImpl: Sized {
    fn Unknown(&self) -> ::windows::core::Result<u16>;
}
