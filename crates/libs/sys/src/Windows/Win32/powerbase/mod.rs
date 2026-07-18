#[cfg(feature = "winnt")]
windows_link::link!("powrprof.dll" "system" fn CallNtPowerInformation(informationlevel : super::POWER_INFORMATION_LEVEL, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("powrprof.dll" "system" fn GetPwrCapabilities(lpspc : *mut super::SYSTEM_POWER_CAPABILITIES) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("powrprof.dll" "system" fn PowerDeterminePlatformRoleEx(version : u32) -> super::POWER_PLATFORM_ROLE);
#[cfg(all(feature = "winnt", feature = "winuser"))]
windows_link::link!("powrprof.dll" "system" fn PowerRegisterSuspendResumeNotification(flags : u32, recipient : super::HANDLE, registrationhandle : *mut super::HPOWERNOTIFY) -> u32);
#[cfg(feature = "winuser")]
windows_link::link!("powrprof.dll" "system" fn PowerUnregisterSuspendResumeNotification(registrationhandle : super::HPOWERNOTIFY) -> u32);
