windows_link::link!("kernel32.dll" "system" fn ExpandEnvironmentStringsA(lpsrc : windows_sys::core::PCSTR, lpdst : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn ExpandEnvironmentStringsW(lpsrc : windows_sys::core::PCWSTR, lpdst : windows_sys::core::PWSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn FreeEnvironmentStringsA(penv : *const i8) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn FreeEnvironmentStringsW(penv : *const u16) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetCommandLineA() -> windows_sys::core::PSTR);
windows_link::link!("kernel32.dll" "system" fn GetCommandLineW() -> windows_sys::core::PWSTR);
windows_link::link!("kernel32.dll" "system" fn GetCurrentDirectoryA(nbufferlength : u32, lpbuffer : windows_sys::core::PSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetCurrentDirectoryW(nbufferlength : u32, lpbuffer : windows_sys::core::PWSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn GetEnvironmentStrings() -> super::winnt::LPCH);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn GetEnvironmentStringsW() -> super::winnt::LPWCH);
windows_link::link!("kernel32.dll" "system" fn GetEnvironmentVariableA(lpname : windows_sys::core::PCSTR, lpbuffer : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetEnvironmentVariableW(lpname : windows_sys::core::PCWSTR, lpbuffer : windows_sys::core::PWSTR, nsize : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn GetStdHandle(nstdhandle : u32) -> super::winnt::HANDLE);
windows_link::link!("kernel32.dll" "system" fn NeedCurrentDirectoryForExePathA(exename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn NeedCurrentDirectoryForExePathW(exename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SearchPathA(lppath : windows_sys::core::PCSTR, lpfilename : windows_sys::core::PCSTR, lpextension : windows_sys::core::PCSTR, nbufferlength : u32, lpbuffer : windows_sys::core::PSTR, lpfilepart : *mut windows_sys::core::PSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn SearchPathW(lppath : windows_sys::core::PCWSTR, lpfilename : windows_sys::core::PCWSTR, lpextension : windows_sys::core::PCWSTR, nbufferlength : u32, lpbuffer : windows_sys::core::PWSTR, lpfilepart : *mut windows_sys::core::PWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn SetCurrentDirectoryA(lppathname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetCurrentDirectoryW(lppathname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetEnvironmentStringsW(newenvironment : *const u16) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetEnvironmentVariableA(lpname : windows_sys::core::PCSTR, lpvalue : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetEnvironmentVariableW(lpname : windows_sys::core::PCWSTR, lpvalue : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn SetStdHandle(nstdhandle : u32, hhandle : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn SetStdHandleEx(nstdhandle : u32, hhandle : super::winnt::HANDLE, phprevvalue : *mut super::winnt::HANDLE) -> windows_sys::core::BOOL);
