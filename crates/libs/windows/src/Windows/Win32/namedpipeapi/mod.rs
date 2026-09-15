#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn CallNamedPipeW<P0>(lpnamedpipename: P0, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesread: super::LPDWORD, ntimeout: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CallNamedPipeW(lpnamedpipename : windows_core::PCWSTR, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : super::LPDWORD, ntimeout : u32) -> windows_core::BOOL);
    unsafe { CallNamedPipeW(lpnamedpipename.param().abi(), lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesread as _, ntimeout) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn ConnectNamedPipe(hnamedpipe: super::HANDLE, lpoverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ConnectNamedPipe(hnamedpipe : super::HANDLE, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { ConnectNamedPipe(hnamedpipe, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateNamedPipeW<P0>(lpname: P0, dwopenmode: u32, dwpipemode: u32, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateNamedPipeW(lpname : windows_core::PCWSTR, dwopenmode : u32, dwpipemode : u32, nmaxinstances : u32, noutbuffersize : u32, ninbuffersize : u32, ndefaulttimeout : u32, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> super::HANDLE);
    unsafe { CreateNamedPipeW(lpname.param().abi(), dwopenmode, dwpipemode, nmaxinstances, noutbuffersize, ninbuffersize, ndefaulttimeout, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreatePipe(hreadpipe: super::PHANDLE, hwritepipe: super::PHANDLE, lppipeattributes: Option<super::LPSECURITY_ATTRIBUTES>, nsize: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CreatePipe(hreadpipe : super::PHANDLE, hwritepipe : super::PHANDLE, lppipeattributes : super::LPSECURITY_ATTRIBUTES, nsize : u32) -> windows_core::BOOL);
    unsafe { CreatePipe(hreadpipe as _, hwritepipe as _, lppipeattributes.unwrap_or(core::mem::zeroed()) as _, nsize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DisconnectNamedPipe(hnamedpipe: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DisconnectNamedPipe(hnamedpipe : super::HANDLE) -> windows_core::BOOL);
    unsafe { DisconnectNamedPipe(hnamedpipe) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetNamedPipeClientComputerNameW(pipe: super::HANDLE, clientcomputername: windows_core::PWSTR, clientcomputernamelength: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeClientComputerNameW(pipe : super::HANDLE, clientcomputername : windows_core::PWSTR, clientcomputernamelength : u32) -> windows_core::BOOL);
    unsafe { GetNamedPipeClientComputerNameW(pipe, clientcomputername, clientcomputernamelength) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetNamedPipeHandleStateW(hnamedpipe: super::HANDLE, lpstate: Option<super::LPDWORD>, lpcurinstances: Option<super::LPDWORD>, lpmaxcollectioncount: Option<super::LPDWORD>, lpcollectdatatimeout: Option<super::LPDWORD>, lpusername: Option<windows_core::PWSTR>, nmaxusernamesize: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeHandleStateW(hnamedpipe : super::HANDLE, lpstate : super::LPDWORD, lpcurinstances : super::LPDWORD, lpmaxcollectioncount : super::LPDWORD, lpcollectdatatimeout : super::LPDWORD, lpusername : windows_core::PWSTR, nmaxusernamesize : u32) -> windows_core::BOOL);
    unsafe { GetNamedPipeHandleStateW(hnamedpipe, lpstate.unwrap_or(core::mem::zeroed()) as _, lpcurinstances.unwrap_or(core::mem::zeroed()) as _, lpmaxcollectioncount.unwrap_or(core::mem::zeroed()) as _, lpcollectdatatimeout.unwrap_or(core::mem::zeroed()) as _, lpusername.unwrap_or(core::mem::zeroed()) as _, nmaxusernamesize) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetNamedPipeInfo(hnamedpipe: super::HANDLE, lpflags: Option<super::LPDWORD>, lpoutbuffersize: Option<super::LPDWORD>, lpinbuffersize: Option<super::LPDWORD>, lpmaxinstances: Option<super::LPDWORD>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeInfo(hnamedpipe : super::HANDLE, lpflags : super::LPDWORD, lpoutbuffersize : super::LPDWORD, lpinbuffersize : super::LPDWORD, lpmaxinstances : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetNamedPipeInfo(hnamedpipe, lpflags.unwrap_or(core::mem::zeroed()) as _, lpoutbuffersize.unwrap_or(core::mem::zeroed()) as _, lpinbuffersize.unwrap_or(core::mem::zeroed()) as _, lpmaxinstances.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ImpersonateNamedPipeClient(hnamedpipe: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ImpersonateNamedPipeClient(hnamedpipe : super::HANDLE) -> windows_core::BOOL);
    unsafe { ImpersonateNamedPipeClient(hnamedpipe) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn PeekNamedPipe(hnamedpipe: super::HANDLE, lpbuffer: Option<*mut core::ffi::c_void>, nbuffersize: u32, lpbytesread: Option<super::LPDWORD>, lptotalbytesavail: Option<super::LPDWORD>, lpbytesleftthismessage: Option<super::LPDWORD>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn PeekNamedPipe(hnamedpipe : super::HANDLE, lpbuffer : *mut core::ffi::c_void, nbuffersize : u32, lpbytesread : super::LPDWORD, lptotalbytesavail : super::LPDWORD, lpbytesleftthismessage : super::LPDWORD) -> windows_core::BOOL);
    unsafe { PeekNamedPipe(hnamedpipe, lpbuffer.unwrap_or(core::mem::zeroed()) as _, nbuffersize, lpbytesread.unwrap_or(core::mem::zeroed()) as _, lptotalbytesavail.unwrap_or(core::mem::zeroed()) as _, lpbytesleftthismessage.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn SetNamedPipeHandleState(hnamedpipe: super::HANDLE, lpmode: Option<super::LPDWORD>, lpmaxcollectioncount: Option<super::LPDWORD>, lpcollectdatatimeout: Option<super::LPDWORD>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetNamedPipeHandleState(hnamedpipe : super::HANDLE, lpmode : super::LPDWORD, lpmaxcollectioncount : super::LPDWORD, lpcollectdatatimeout : super::LPDWORD) -> windows_core::BOOL);
    unsafe { SetNamedPipeHandleState(hnamedpipe, lpmode.unwrap_or(core::mem::zeroed()) as _, lpmaxcollectioncount.unwrap_or(core::mem::zeroed()) as _, lpcollectdatatimeout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn TransactNamedPipe(hnamedpipe: super::HANDLE, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesread: super::LPDWORD, lpoverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TransactNamedPipe(hnamedpipe : super::HANDLE, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { TransactNamedPipe(hnamedpipe, lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesread as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WaitNamedPipeW<P0>(lpnamedpipename: P0, ntimeout: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WaitNamedPipeW(lpnamedpipename : windows_core::PCWSTR, ntimeout : u32) -> windows_core::BOOL);
    unsafe { WaitNamedPipeW(lpnamedpipename.param().abi(), ntimeout) }
}
