#[cfg(feature = "Win32_winnt")]
windows_link::link!("powrprof.dll" "system" fn CallNtPowerInformation(informationlevel : super::winnt::POWER_INFORMATION_LEVEL, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("powrprof.dll" "system" fn GetPwrCapabilities(lpspc : *mut super::winnt::SYSTEM_POWER_CAPABILITIES) -> bool);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("powrprof.dll" "system" fn PowerDeterminePlatformRoleEx(version : u32) -> super::winnt::POWER_PLATFORM_ROLE);
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winuser"))]
windows_link::link!("powrprof.dll" "system" fn PowerRegisterSuspendResumeNotification(flags : u32, recipient : super::winnt::HANDLE, registrationhandle : *mut super::winuser::HPOWERNOTIFY) -> u32);
#[cfg(feature = "Win32_winuser")]
windows_link::link!("powrprof.dll" "system" fn PowerUnregisterSuspendResumeNotification(registrationhandle : super::winuser::HPOWERNOTIFY) -> u32);
