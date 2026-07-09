windows_link::link!("api-ms-win-core-wow64-l1-1-1.dll" "system" fn GetSystemWow64Directory2A(lpbuffer : windows_sys::core::PSTR, usize : u32, imagefilemachinetype : u16) -> u32);
windows_link::link!("api-ms-win-core-wow64-l1-1-1.dll" "system" fn GetSystemWow64Directory2W(lpbuffer : windows_sys::core::PWSTR, usize : u32, imagefilemachinetype : u16) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetSystemWow64DirectoryA(lpbuffer : windows_sys::core::PSTR, usize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetSystemWow64DirectoryW(lpbuffer : windows_sys::core::PWSTR, usize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn IsWow64GuestMachineSupported(wowguestmachine : u16, machineissupported : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn IsWow64Process(hprocess : super::winnt::HANDLE, wow64process : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn IsWow64Process2(hprocess : super::winnt::HANDLE, pprocessmachine : *mut u16, pnativemachine : *mut u16) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn Wow64DisableWow64FsRedirection(oldvalue : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn Wow64EnableWow64FsRedirection(wow64fsenableredirection : bool) -> bool);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn Wow64GetThreadContext(hthread : super::winnt::HANDLE, lpcontext : *mut super::winnt::WOW64_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn Wow64RevertWow64FsRedirection(olvalue : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn Wow64SetThreadContext(hthread : super::winnt::HANDLE, lpcontext : *const super::winnt::WOW64_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-wow64-l1-1-1.dll" "system" fn Wow64SetThreadDefaultGuestMachine(machine : u16) -> u16);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn Wow64SuspendThread(hthread : super::winnt::HANDLE) -> u32);
