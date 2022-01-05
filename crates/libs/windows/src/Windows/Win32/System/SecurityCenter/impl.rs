#[cfg(feature = "Win32_System_Com")]
pub trait IWSCDefaultProductImpl: Sized + IDispatchImpl {
    fn SetDefaultProduct();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWSCProductListImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Count();
    fn Item();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWscProductImpl: Sized + IDispatchImpl {
    fn ProductName();
    fn ProductState();
    fn SignatureStatus();
    fn RemediationPath();
    fn ProductStateTimestamp();
    fn ProductGuid();
    fn ProductIsDefault();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWscProduct2Impl: Sized + IWscProductImpl + IDispatchImpl {
    fn AntivirusScanSubstatus();
    fn AntivirusSettingsSubstatus();
    fn AntivirusProtectionUpdateSubstatus();
    fn FirewallDomainProfileSubstatus();
    fn FirewallPrivateProfileSubstatus();
    fn FirewallPublicProfileSubstatus();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWscProduct3Impl: Sized + IWscProduct2Impl + IWscProductImpl + IDispatchImpl {
    fn AntivirusDaysUntilExpired();
}
