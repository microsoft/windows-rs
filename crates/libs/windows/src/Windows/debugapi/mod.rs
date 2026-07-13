#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CheckRemoteDebuggerPresent(hprocess: super::winnt::HANDLE, pbdebuggerpresent: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CheckRemoteDebuggerPresent(hprocess : super::winnt::HANDLE, pbdebuggerpresent : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CheckRemoteDebuggerPresent(hprocess, pbdebuggerpresent as _) }
}
#[inline]
pub unsafe fn ContinueDebugEvent(dwprocessid: u32, dwthreadid: u32, dwcontinuestatus: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ContinueDebugEvent(dwprocessid : u32, dwthreadid : u32, dwcontinuestatus : u32) -> windows_core::BOOL);
    unsafe { ContinueDebugEvent(dwprocessid, dwthreadid, dwcontinuestatus) }
}
#[inline]
pub unsafe fn DebugActiveProcess(dwprocessid: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DebugActiveProcess(dwprocessid : u32) -> windows_core::BOOL);
    unsafe { DebugActiveProcess(dwprocessid) }
}
#[inline]
pub unsafe fn DebugActiveProcessStop(dwprocessid: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DebugActiveProcessStop(dwprocessid : u32) -> windows_core::BOOL);
    unsafe { DebugActiveProcessStop(dwprocessid) }
}
#[inline]
pub unsafe fn DebugBreak() {
    windows_core::link!("kernel32.dll" "system" fn DebugBreak());
    unsafe { DebugBreak() }
}
#[inline]
pub unsafe fn IsDebuggerPresent() -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsDebuggerPresent() -> windows_core::BOOL);
    unsafe { IsDebuggerPresent() }
}
#[inline]
pub unsafe fn OutputDebugStringA<P0>(lpoutputstring: P0)
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OutputDebugStringA(lpoutputstring : windows_core::PCSTR));
    unsafe { OutputDebugStringA(lpoutputstring.param().abi()) }
}
#[inline]
pub unsafe fn OutputDebugStringW<P0>(lpoutputstring: P0)
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OutputDebugStringW(lpoutputstring : windows_core::PCWSTR));
    unsafe { OutputDebugStringW(lpoutputstring.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn WaitForDebugEvent(lpdebugevent: *mut super::minwinbase::DEBUG_EVENT, dwmilliseconds: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WaitForDebugEvent(lpdebugevent : *mut super::minwinbase::DEBUG_EVENT, dwmilliseconds : u32) -> windows_core::BOOL);
    unsafe { WaitForDebugEvent(lpdebugevent as _, dwmilliseconds) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn WaitForDebugEventEx(lpdebugevent: *mut super::minwinbase::DEBUG_EVENT, dwmilliseconds: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WaitForDebugEventEx(lpdebugevent : *mut super::minwinbase::DEBUG_EVENT, dwmilliseconds : u32) -> windows_core::BOOL);
    unsafe { WaitForDebugEventEx(lpdebugevent as _, dwmilliseconds) }
}
