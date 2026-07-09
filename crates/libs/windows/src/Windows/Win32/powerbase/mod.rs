#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CallNtPowerInformation(informationlevel: super::winnt::POWER_INFORMATION_LEVEL, inputbuffer: Option<*const core::ffi::c_void>, inputbufferlength: u32, outputbuffer: Option<*mut core::ffi::c_void>, outputbufferlength: u32) -> i32 {
    windows_core::link!("powrprof.dll" "system" fn CallNtPowerInformation(informationlevel : super::winnt::POWER_INFORMATION_LEVEL, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> i32);
    unsafe { CallNtPowerInformation(informationlevel, inputbuffer.unwrap_or(core::mem::zeroed()) as _, inputbufferlength, outputbuffer.unwrap_or(core::mem::zeroed()) as _, outputbufferlength) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetPwrCapabilities(lpspc: *mut super::winnt::SYSTEM_POWER_CAPABILITIES) -> bool {
    windows_core::link!("powrprof.dll" "system" fn GetPwrCapabilities(lpspc : *mut super::winnt::SYSTEM_POWER_CAPABILITIES) -> bool);
    unsafe { GetPwrCapabilities(lpspc as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn PowerDeterminePlatformRoleEx(version: u32) -> super::winnt::POWER_PLATFORM_ROLE {
    windows_core::link!("powrprof.dll" "system" fn PowerDeterminePlatformRoleEx(version : u32) -> super::winnt::POWER_PLATFORM_ROLE);
    unsafe { PowerDeterminePlatformRoleEx(version) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winuser"))]
#[inline]
pub unsafe fn PowerRegisterSuspendResumeNotification(flags: u32, recipient: super::winnt::HANDLE, registrationhandle: *mut super::winuser::HPOWERNOTIFY) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerRegisterSuspendResumeNotification(flags : u32, recipient : super::winnt::HANDLE, registrationhandle : *mut super::winuser::HPOWERNOTIFY) -> u32);
    unsafe { PowerRegisterSuspendResumeNotification(flags, recipient, registrationhandle as _) }
}
#[cfg(feature = "Win32_winuser")]
#[inline]
pub unsafe fn PowerUnregisterSuspendResumeNotification(registrationhandle: super::winuser::HPOWERNOTIFY) -> u32 {
    windows_core::link!("powrprof.dll" "system" fn PowerUnregisterSuspendResumeNotification(registrationhandle : super::winuser::HPOWERNOTIFY) -> u32);
    unsafe { PowerUnregisterSuspendResumeNotification(registrationhandle as _) }
}
