#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CallNtPowerInformation(informationlevel: super::POWER_INFORMATION_LEVEL, inputbuffer: Option<super::PVOID>, inputbufferlength: super::ULONG, outputbuffer: Option<super::PVOID>, outputbufferlength: super::ULONG) -> super::LONG {
    windows_core::link!("powrprof.dll" "system" fn CallNtPowerInformation(informationlevel : super::POWER_INFORMATION_LEVEL, inputbuffer : super::PVOID, inputbufferlength : super::ULONG, outputbuffer : super::PVOID, outputbufferlength : super::ULONG) -> super::LONG);
    unsafe { CallNtPowerInformation(informationlevel, inputbuffer.unwrap_or(core::mem::zeroed()) as _, inputbufferlength, outputbuffer.unwrap_or(core::mem::zeroed()) as _, outputbufferlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetPwrCapabilities(lpspc: super::PSYSTEM_POWER_CAPABILITIES) -> super::BOOLEAN {
    windows_core::link!("powrprof.dll" "system" fn GetPwrCapabilities(lpspc : super::PSYSTEM_POWER_CAPABILITIES) -> super::BOOLEAN);
    unsafe { GetPwrCapabilities(lpspc as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PowerDeterminePlatformRoleEx(version: super::ULONG) -> super::POWER_PLATFORM_ROLE {
    windows_core::link!("powrprof.dll" "system" fn PowerDeterminePlatformRoleEx(version : super::ULONG) -> super::POWER_PLATFORM_ROLE);
    unsafe { PowerDeterminePlatformRoleEx(version) }
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "winuser"))]
#[inline]
pub unsafe fn PowerRegisterSuspendResumeNotification(flags: super::DWORD, recipient: super::HANDLE, registrationhandle: super::PHPOWERNOTIFY) -> super::DWORD {
    windows_core::link!("powrprof.dll" "system" fn PowerRegisterSuspendResumeNotification(flags : super::DWORD, recipient : super::HANDLE, registrationhandle : super::PHPOWERNOTIFY) -> super::DWORD);
    unsafe { PowerRegisterSuspendResumeNotification(flags, recipient, registrationhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winuser"))]
#[inline]
pub unsafe fn PowerUnregisterSuspendResumeNotification(registrationhandle: super::HPOWERNOTIFY) -> super::DWORD {
    windows_core::link!("powrprof.dll" "system" fn PowerUnregisterSuspendResumeNotification(registrationhandle : super::HPOWERNOTIFY) -> super::DWORD);
    unsafe { PowerUnregisterSuspendResumeNotification(registrationhandle as _) }
}
