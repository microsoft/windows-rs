#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-wow64-l1-1-1.dll" "system" fn GetSystemWow64Directory2A(lpbuffer : windows_sys::core::PSTR, usize : super::UINT, imagefilemachinetype : super::WORD) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-wow64-l1-1-1.dll" "system" fn GetSystemWow64Directory2W(lpbuffer : windows_sys::core::PWSTR, usize : super::UINT, imagefilemachinetype : super::WORD) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetSystemWow64DirectoryA(lpbuffer : windows_sys::core::PSTR, usize : super::UINT) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetSystemWow64DirectoryW(lpbuffer : windows_sys::core::PWSTR, usize : super::UINT) -> super::UINT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn IsWow64GuestMachineSupported(wowguestmachine : super::USHORT, machineissupported : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn IsWow64Process(hprocess : super::HANDLE, wow64process : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn IsWow64Process2(hprocess : super::HANDLE, pprocessmachine : *mut super::USHORT, pnativemachine : *mut super::USHORT) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn Wow64DisableWow64FsRedirection(oldvalue : *mut super::PVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn Wow64EnableWow64FsRedirection(wow64fsenableredirection : super::BOOLEAN) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn Wow64GetThreadContext(hthread : super::HANDLE, lpcontext : super::PWOW64_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn Wow64RevertWow64FsRedirection(olvalue : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn Wow64SetThreadContext(hthread : super::HANDLE, lpcontext : *const super::WOW64_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-wow64-l1-1-1.dll" "system" fn Wow64SetThreadDefaultGuestMachine(machine : super::USHORT) -> super::USHORT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn Wow64SuspendThread(hthread : super::HANDLE) -> super::DWORD);
