#[cfg(feature = "implement_exclusive")]
pub trait IDeviceLockdownProfileInformationImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceLockdownProfileStaticsImpl: Sized {
    fn GetSupportedLockdownProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::GUID>>;
    fn GetCurrentLockdownProfile(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ApplyLockdownProfileAsync(&self, profileid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetLockdownProfileInformation(&self, profileid: &::windows::core::GUID) -> ::windows::core::Result<DeviceLockdownProfileInformation>;
}
