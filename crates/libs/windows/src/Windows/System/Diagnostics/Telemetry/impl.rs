#[cfg(feature = "implement_exclusive")]
pub trait IPlatformTelemetryClientStaticsImpl: Sized {
    fn Register(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<PlatformTelemetryRegistrationResult>;
    fn RegisterWithSettings(&self, id: &::windows::core::HSTRING, settings: &::core::option::Option<PlatformTelemetryRegistrationSettings>) -> ::windows::core::Result<PlatformTelemetryRegistrationResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlatformTelemetryRegistrationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<PlatformTelemetryRegistrationStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlatformTelemetryRegistrationSettingsImpl: Sized {
    fn StorageSize(&self) -> ::windows::core::Result<u32>;
    fn SetStorageSize(&self, value: u32) -> ::windows::core::Result<()>;
    fn UploadQuotaSize(&self) -> ::windows::core::Result<u32>;
    fn SetUploadQuotaSize(&self, value: u32) -> ::windows::core::Result<()>;
}
