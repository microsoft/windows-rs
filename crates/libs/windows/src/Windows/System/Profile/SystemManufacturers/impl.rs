#[cfg(feature = "implement_exclusive")]
pub trait IOemSupportInfoImpl: Sized {
    fn SupportLink(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SupportAppLink(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SupportProvider(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmbiosInformationStaticsImpl: Sized {
    fn SerialNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemSupportDeviceInfoImpl: Sized {
    fn OperatingSystem(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemManufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemProductName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemSku(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemHardwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SystemFirmwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemSupportInfoStaticsImpl: Sized {
    fn LocalSystemEdition(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OemSupportInfo(&self) -> ::windows::core::Result<OemSupportInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemSupportInfoStatics2Impl: Sized {
    fn LocalDeviceInfo(&self) -> ::windows::core::Result<SystemSupportDeviceInfo>;
}
