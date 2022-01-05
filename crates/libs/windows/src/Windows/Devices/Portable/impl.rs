#[cfg(feature = "implement_exclusive")]
pub trait IServiceDeviceStaticsImpl: Sized {
    fn GetDeviceSelector(&self, servicetype: ServiceDeviceType) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorFromServiceId(&self, serviceid: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageDeviceStaticsImpl: Sized {
    fn FromId(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Storage::StorageFolder>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
