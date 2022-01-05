#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOriginImpl: Sized {
    fn Category(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCategory(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CategoryDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCategoryDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Location(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocation(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOrigin2Impl: Sized + IPhoneCallOriginImpl {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOrigin3Impl: Sized + IPhoneCallOriginImpl + IPhoneCallOrigin2Impl {
    fn DisplayPicture(&self) -> ::windows::core::Result<super::super::super::Storage::StorageFile>;
    fn SetDisplayPicture(&self, value: &::core::option::Option<super::super::super::Storage::StorageFile>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOriginManagerStaticsImpl: Sized {
    fn IsCurrentAppActiveCallOriginApp(&self) -> ::windows::core::Result<bool>;
    fn ShowPhoneCallOriginSettingsUI(&self) -> ::windows::core::Result<()>;
    fn SetCallOrigin(&self, requestid: &::windows::core::GUID, callorigin: &::core::option::Option<PhoneCallOrigin>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOriginManagerStatics2Impl: Sized + IPhoneCallOriginManagerStaticsImpl {
    fn RequestSetAsActiveCallOriginAppAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneCallOriginManagerStatics3Impl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
