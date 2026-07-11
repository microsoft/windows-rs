#[inline]
pub unsafe fn WscGetAntiMalwareUri() -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("wscapi.dll" "system" fn WscGetAntiMalwareUri(ppszuri : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WscGetAntiMalwareUri(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WscGetSecurityProviderHealth(providers: u32) -> windows_core::Result<WSC_SECURITY_PROVIDER_HEALTH> {
    windows_core::link!("wscapi.dll" "system" fn WscGetSecurityProviderHealth(providers : u32, phealth : *mut WSC_SECURITY_PROVIDER_HEALTH) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WscGetSecurityProviderHealth(providers, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn WscQueryAntiMalwareUri() -> windows_core::HRESULT {
    windows_core::link!("wscapi.dll" "system" fn WscQueryAntiMalwareUri() -> windows_core::HRESULT);
    unsafe { WscQueryAntiMalwareUri() }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn WscRegisterForChanges(reserved: *mut core::ffi::c_void, phcallbackregistration: *mut super::winnt::HANDLE, lpcallbackaddress: super::minwinbase::LPTHREAD_START_ROUTINE, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("wscapi.dll" "system" fn WscRegisterForChanges(reserved : *mut core::ffi::c_void, phcallbackregistration : *mut super::winnt::HANDLE, lpcallbackaddress : super::minwinbase::LPTHREAD_START_ROUTINE, pcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WscRegisterForChanges(reserved as _, phcallbackregistration as _, lpcallbackaddress, pcontext as _) }
}
#[inline]
pub unsafe fn WscRegisterForUserNotifications() -> windows_core::HRESULT {
    windows_core::link!("wscapi.dll" "system" fn WscRegisterForUserNotifications() -> windows_core::HRESULT);
    unsafe { WscRegisterForUserNotifications() }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WscUnRegisterChanges(hregistrationhandle: super::winnt::HANDLE) -> windows_core::HRESULT {
    windows_core::link!("wscapi.dll" "system" fn WscUnRegisterChanges(hregistrationhandle : super::winnt::HANDLE) -> windows_core::HRESULT);
    unsafe { WscUnRegisterChanges(hregistrationhandle) }
}
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
