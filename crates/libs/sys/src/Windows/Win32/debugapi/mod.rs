#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CheckRemoteDebuggerPresent(hprocess : super::HANDLE, pbdebuggerpresent : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn ContinueDebugEvent(dwprocessid : super::DWORD, dwthreadid : super::DWORD, dwcontinuestatus : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn DebugActiveProcess(dwprocessid : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn DebugActiveProcessStop(dwprocessid : super::DWORD) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DebugBreak());
windows_link::link!("kernel32.dll" "system" fn IsDebuggerPresent() -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn OutputDebugStringA(lpoutputstring : windows_sys::core::PCSTR));
windows_link::link!("kernel32.dll" "system" fn OutputDebugStringW(lpoutputstring : windows_sys::core::PCWSTR));
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WaitForDebugEvent(lpdebugevent : super::LPDEBUG_EVENT, dwmilliseconds : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WaitForDebugEventEx(lpdebugevent : super::LPDEBUG_EVENT, dwmilliseconds : super::DWORD) -> windows_sys::core::BOOL);
