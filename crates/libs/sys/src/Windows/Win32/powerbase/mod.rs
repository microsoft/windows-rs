#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn CallNtPowerInformation(informationlevel : super::POWER_INFORMATION_LEVEL, inputbuffer : super::PVOID, inputbufferlength : super::ULONG, outputbuffer : super::PVOID, outputbufferlength : super::ULONG) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn GetPwrCapabilities(lpspc : super::PSYSTEM_POWER_CAPABILITIES) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("powrprof.dll" "system" fn PowerDeterminePlatformRoleEx(version : super::ULONG) -> super::POWER_PLATFORM_ROLE);
#[cfg(all(feature = "shared", feature = "winnt", feature = "winuser"))]
windows_link::link!("powrprof.dll" "system" fn PowerRegisterSuspendResumeNotification(flags : super::DWORD, recipient : super::HANDLE, registrationhandle : super::PHPOWERNOTIFY) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winuser"))]
windows_link::link!("powrprof.dll" "system" fn PowerUnregisterSuspendResumeNotification(registrationhandle : super::HPOWERNOTIFY) -> super::DWORD);
