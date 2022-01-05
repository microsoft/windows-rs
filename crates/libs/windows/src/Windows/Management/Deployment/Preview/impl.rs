#[cfg(feature = "implement_exclusive")]
pub trait IClassicAppManagerStaticsImpl: Sized {
    fn FindInstalledApp(&self, appuninstallkey: &::windows::core::HSTRING) -> ::windows::core::Result<InstalledClassicAppInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInstalledClassicAppInfoImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
