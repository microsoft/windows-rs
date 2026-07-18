#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CheckRemoteDebuggerPresent(hprocess : super::HANDLE, pbdebuggerpresent : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ContinueDebugEvent(dwprocessid : u32, dwthreadid : u32, dwcontinuestatus : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DebugActiveProcess(dwprocessid : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DebugActiveProcessStop(dwprocessid : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DebugBreak());
windows_link::link!("kernel32.dll" "system" fn IsDebuggerPresent() -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn OutputDebugStringA(lpoutputstring : windows_sys::core::PCSTR));
windows_link::link!("kernel32.dll" "system" fn OutputDebugStringW(lpoutputstring : windows_sys::core::PCWSTR));
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WaitForDebugEvent(lpdebugevent : *mut super::DEBUG_EVENT, dwmilliseconds : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WaitForDebugEventEx(lpdebugevent : *mut super::DEBUG_EVENT, dwmilliseconds : u32) -> windows_sys::core::BOOL);
