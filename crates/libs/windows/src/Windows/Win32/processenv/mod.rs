#[inline]
pub unsafe fn ExpandEnvironmentStringsA<P0>(lpsrc: P0, lpdst: Option<&mut [u8]>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn ExpandEnvironmentStringsA(lpsrc : windows_core::PCSTR, lpdst : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { ExpandEnvironmentStringsA(lpsrc.param().abi(), core::mem::transmute(lpdst.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpdst.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn ExpandEnvironmentStringsW<P0>(lpsrc: P0, lpdst: Option<&mut [u16]>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn ExpandEnvironmentStringsW(lpsrc : windows_core::PCWSTR, lpdst : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { ExpandEnvironmentStringsW(lpsrc.param().abi(), core::mem::transmute(lpdst.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpdst.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn FreeEnvironmentStringsA(penv: *const i8) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FreeEnvironmentStringsA(penv : *const i8) -> windows_core::BOOL);
    unsafe { FreeEnvironmentStringsA(penv) }
}
#[inline]
pub unsafe fn FreeEnvironmentStringsW(penv: *const u16) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FreeEnvironmentStringsW(penv : *const u16) -> windows_core::BOOL);
    unsafe { FreeEnvironmentStringsW(penv) }
}
#[inline]
pub unsafe fn GetCommandLineA() -> windows_core::PSTR {
    windows_core::link!("kernel32.dll" "system" fn GetCommandLineA() -> windows_core::PSTR);
    unsafe { GetCommandLineA() }
}
#[inline]
pub unsafe fn GetCommandLineW() -> windows_core::PWSTR {
    windows_core::link!("kernel32.dll" "system" fn GetCommandLineW() -> windows_core::PWSTR);
    unsafe { GetCommandLineW() }
}
#[inline]
pub unsafe fn GetCurrentDirectoryA(lpbuffer: Option<&mut [u8]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentDirectoryA(nbufferlength : u32, lpbuffer : windows_core::PSTR) -> u32);
    unsafe { GetCurrentDirectoryA(lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()))) }
}
#[inline]
pub unsafe fn GetCurrentDirectoryW(lpbuffer: Option<&mut [u16]>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentDirectoryW(nbufferlength : u32, lpbuffer : windows_core::PWSTR) -> u32);
    unsafe { GetCurrentDirectoryW(lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()))) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetEnvironmentStrings() -> super::LPCH {
    windows_core::link!("kernel32.dll" "system" fn GetEnvironmentStrings() -> super::LPCH);
    unsafe { GetEnvironmentStrings() }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetEnvironmentStringsW() -> super::LPWCH {
    windows_core::link!("kernel32.dll" "system" fn GetEnvironmentStringsW() -> super::LPWCH);
    unsafe { GetEnvironmentStringsW() }
}
#[inline]
pub unsafe fn GetEnvironmentVariableA<P0>(lpname: P0, lpbuffer: Option<&mut [u8]>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetEnvironmentVariableA(lpname : windows_core::PCSTR, lpbuffer : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetEnvironmentVariableA(lpname.param().abi(), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[inline]
pub unsafe fn GetEnvironmentVariableW<P0>(lpname: P0, lpbuffer: Option<&mut [u16]>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetEnvironmentVariableW(lpname : windows_core::PCWSTR, lpbuffer : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetEnvironmentVariableW(lpname.param().abi(), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetStdHandle(nstdhandle: u32) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn GetStdHandle(nstdhandle : u32) -> super::HANDLE);
    unsafe { GetStdHandle(nstdhandle) }
}
#[inline]
pub unsafe fn NeedCurrentDirectoryForExePathA<P0>(exename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn NeedCurrentDirectoryForExePathA(exename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { NeedCurrentDirectoryForExePathA(exename.param().abi()) }
}
#[inline]
pub unsafe fn NeedCurrentDirectoryForExePathW<P0>(exename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn NeedCurrentDirectoryForExePathW(exename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { NeedCurrentDirectoryForExePathW(exename.param().abi()) }
}
#[inline]
pub unsafe fn SearchPathA<P0, P1, P2>(lppath: P0, lpfilename: P1, lpextension: P2, lpbuffer: Option<&mut [u8]>, lpfilepart: Option<*mut windows_core::PSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SearchPathA(lppath : windows_core::PCSTR, lpfilename : windows_core::PCSTR, lpextension : windows_core::PCSTR, nbufferlength : u32, lpbuffer : windows_core::PSTR, lpfilepart : *mut windows_core::PSTR) -> u32);
    unsafe { SearchPathA(lppath.param().abi(), lpfilename.param().abi(), lpextension.param().abi(), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpfilepart.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SearchPathW<P0, P1, P2>(lppath: P0, lpfilename: P1, lpextension: P2, lpbuffer: Option<&mut [u16]>, lpfilepart: Option<*mut windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SearchPathW(lppath : windows_core::PCWSTR, lpfilename : windows_core::PCWSTR, lpextension : windows_core::PCWSTR, nbufferlength : u32, lpbuffer : windows_core::PWSTR, lpfilepart : *mut windows_core::PWSTR) -> u32);
    unsafe { SearchPathW(lppath.param().abi(), lpfilename.param().abi(), lpextension.param().abi(), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut())), lpfilepart.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SetCurrentDirectoryA<P0>(lppathname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetCurrentDirectoryA(lppathname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetCurrentDirectoryA(lppathname.param().abi()) }
}
#[inline]
pub unsafe fn SetCurrentDirectoryW<P0>(lppathname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetCurrentDirectoryW(lppathname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetCurrentDirectoryW(lppathname.param().abi()) }
}
#[inline]
pub unsafe fn SetEnvironmentStringsW(newenvironment: *const u16) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetEnvironmentStringsW(newenvironment : *const u16) -> windows_core::BOOL);
    unsafe { SetEnvironmentStringsW(newenvironment) }
}
#[inline]
pub unsafe fn SetEnvironmentVariableA<P0, P1>(lpname: P0, lpvalue: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetEnvironmentVariableA(lpname : windows_core::PCSTR, lpvalue : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetEnvironmentVariableA(lpname.param().abi(), lpvalue.param().abi()) }
}
#[inline]
pub unsafe fn SetEnvironmentVariableW<P0, P1>(lpname: P0, lpvalue: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetEnvironmentVariableW(lpname : windows_core::PCWSTR, lpvalue : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetEnvironmentVariableW(lpname.param().abi(), lpvalue.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetStdHandle(nstdhandle: u32, hhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetStdHandle(nstdhandle : u32, hhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { SetStdHandle(nstdhandle, hhandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetStdHandleEx(nstdhandle: u32, hhandle: super::HANDLE, phprevvalue: Option<*mut super::HANDLE>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetStdHandleEx(nstdhandle : u32, hhandle : super::HANDLE, phprevvalue : *mut super::HANDLE) -> windows_core::BOOL);
    unsafe { SetStdHandleEx(nstdhandle, hhandle, phprevvalue.unwrap_or(core::mem::zeroed()) as _) }
}
