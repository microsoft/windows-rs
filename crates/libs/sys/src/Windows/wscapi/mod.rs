windows_link::link!("wscapi.dll" "system" fn WscGetAntiMalwareUri(ppszuri : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("wscapi.dll" "system" fn WscGetSecurityProviderHealth(providers : u32, phealth : *mut WSC_SECURITY_PROVIDER_HEALTH) -> windows_sys::core::HRESULT);
windows_link::link!("wscapi.dll" "system" fn WscQueryAntiMalwareUri() -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("wscapi.dll" "system" fn WscRegisterForChanges(reserved : *mut core::ffi::c_void, phcallbackregistration : *mut super::winnt::HANDLE, lpcallbackaddress : super::minwinbase::LPTHREAD_START_ROUTINE, pcontext : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("wscapi.dll" "system" fn WscRegisterForUserNotifications() -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("wscapi.dll" "system" fn WscUnRegisterChanges(hregistrationhandle : super::winnt::HANDLE) -> windows_sys::core::HRESULT);
pub type PWSC_SECURITY_PROVIDER = *mut WSC_SECURITY_PROVIDER;
pub type PWSC_SECURITY_PROVIDER_HEALTH = *mut WSC_SECURITY_PROVIDER_HEALTH;
pub type WSC_SECURITY_PROVIDER = i32;
pub const WSC_SECURITY_PROVIDER_ALL: WSC_SECURITY_PROVIDER = 127;
pub const WSC_SECURITY_PROVIDER_ANTISPYWARE: WSC_SECURITY_PROVIDER = 8;
pub const WSC_SECURITY_PROVIDER_ANTIVIRUS: WSC_SECURITY_PROVIDER = 4;
pub const WSC_SECURITY_PROVIDER_AUTOUPDATE_SETTINGS: WSC_SECURITY_PROVIDER = 2;
pub const WSC_SECURITY_PROVIDER_FIREWALL: WSC_SECURITY_PROVIDER = 1;
pub type WSC_SECURITY_PROVIDER_HEALTH = i32;
pub const WSC_SECURITY_PROVIDER_HEALTH_GOOD: WSC_SECURITY_PROVIDER_HEALTH = 0;
pub const WSC_SECURITY_PROVIDER_HEALTH_NOTMONITORED: WSC_SECURITY_PROVIDER_HEALTH = 1;
pub const WSC_SECURITY_PROVIDER_HEALTH_POOR: WSC_SECURITY_PROVIDER_HEALTH = 2;
pub const WSC_SECURITY_PROVIDER_HEALTH_SNOOZE: WSC_SECURITY_PROVIDER_HEALTH = 3;
pub const WSC_SECURITY_PROVIDER_INTERNET_SETTINGS: WSC_SECURITY_PROVIDER = 16;
pub const WSC_SECURITY_PROVIDER_NONE: WSC_SECURITY_PROVIDER = 0;
pub const WSC_SECURITY_PROVIDER_SERVICE: WSC_SECURITY_PROVIDER = 64;
pub const WSC_SECURITY_PROVIDER_USER_ACCOUNT_CONTROL: WSC_SECURITY_PROVIDER = 32;
