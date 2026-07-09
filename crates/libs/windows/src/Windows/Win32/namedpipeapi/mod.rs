#[inline]
pub unsafe fn CallNamedPipeW<P0>(lpnamedpipename: P0, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CallNamedPipeW(lpnamedpipename : windows_core::PCWSTR, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, ntimeout : u32) -> windows_core::BOOL);
    unsafe { CallNamedPipeW(lpnamedpipename.param().abi(), lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesread as _, ntimeout) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn ConnectNamedPipe(hnamedpipe: super::winnt::HANDLE, lpoverlapped: Option<*mut super::minwinbase::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ConnectNamedPipe(hnamedpipe : super::winnt::HANDLE, lpoverlapped : *mut super::minwinbase::OVERLAPPED) -> windows_core::BOOL);
    unsafe { ConnectNamedPipe(hnamedpipe, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateNamedPipeW<P0>(lpname: P0, dwopenmode: u32, dwpipemode: u32, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateNamedPipeW(lpname : windows_core::PCWSTR, dwopenmode : u32, dwpipemode : u32, nmaxinstances : u32, noutbuffersize : u32, ninbuffersize : u32, ndefaulttimeout : u32, lpsecurityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES) -> super::winnt::HANDLE);
    unsafe { CreateNamedPipeW(lpname.param().abi(), dwopenmode, dwpipemode, nmaxinstances, noutbuffersize, ninbuffersize, ndefaulttimeout, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreatePipe(hreadpipe: *mut super::winnt::HANDLE, hwritepipe: *mut super::winnt::HANDLE, lppipeattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, nsize: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CreatePipe(hreadpipe : *mut super::winnt::HANDLE, hwritepipe : *mut super::winnt::HANDLE, lppipeattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, nsize : u32) -> windows_core::BOOL);
    unsafe { CreatePipe(hreadpipe as _, hwritepipe as _, lppipeattributes.unwrap_or(core::mem::zeroed()) as _, nsize) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DisconnectNamedPipe(hnamedpipe: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DisconnectNamedPipe(hnamedpipe : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { DisconnectNamedPipe(hnamedpipe) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetNamedPipeClientComputerNameW(pipe: super::winnt::HANDLE, clientcomputername: windows_core::PWSTR, clientcomputernamelength: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeClientComputerNameW(pipe : super::winnt::HANDLE, clientcomputername : windows_core::PWSTR, clientcomputernamelength : u32) -> windows_core::BOOL);
    unsafe { GetNamedPipeClientComputerNameW(pipe, core::mem::transmute(clientcomputername), clientcomputernamelength) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetNamedPipeHandleStateW(hnamedpipe: super::winnt::HANDLE, lpstate: Option<*mut u32>, lpcurinstances: Option<*mut u32>, lpmaxcollectioncount: Option<*mut u32>, lpcollectdatatimeout: Option<*mut u32>, lpusername: Option<&mut [u16]>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeHandleStateW(hnamedpipe : super::winnt::HANDLE, lpstate : *mut u32, lpcurinstances : *mut u32, lpmaxcollectioncount : *mut u32, lpcollectdatatimeout : *mut u32, lpusername : windows_core::PWSTR, nmaxusernamesize : u32) -> windows_core::BOOL);
    unsafe { GetNamedPipeHandleStateW(hnamedpipe, lpstate.unwrap_or(core::mem::zeroed()) as _, lpcurinstances.unwrap_or(core::mem::zeroed()) as _, lpmaxcollectioncount.unwrap_or(core::mem::zeroed()) as _, lpcollectdatatimeout.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpusername.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), lpusername.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetNamedPipeInfo(hnamedpipe: super::winnt::HANDLE, lpflags: Option<*mut u32>, lpoutbuffersize: Option<*mut u32>, lpinbuffersize: Option<*mut u32>, lpmaxinstances: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeInfo(hnamedpipe : super::winnt::HANDLE, lpflags : *mut u32, lpoutbuffersize : *mut u32, lpinbuffersize : *mut u32, lpmaxinstances : *mut u32) -> windows_core::BOOL);
    unsafe { GetNamedPipeInfo(hnamedpipe, lpflags.unwrap_or(core::mem::zeroed()) as _, lpoutbuffersize.unwrap_or(core::mem::zeroed()) as _, lpinbuffersize.unwrap_or(core::mem::zeroed()) as _, lpmaxinstances.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn ImpersonateNamedPipeClient(hnamedpipe: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ImpersonateNamedPipeClient(hnamedpipe : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { ImpersonateNamedPipeClient(hnamedpipe) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn PeekNamedPipe(hnamedpipe: super::winnt::HANDLE, lpbuffer: Option<*mut core::ffi::c_void>, nbuffersize: u32, lpbytesread: Option<*mut u32>, lptotalbytesavail: Option<*mut u32>, lpbytesleftthismessage: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn PeekNamedPipe(hnamedpipe : super::winnt::HANDLE, lpbuffer : *mut core::ffi::c_void, nbuffersize : u32, lpbytesread : *mut u32, lptotalbytesavail : *mut u32, lpbytesleftthismessage : *mut u32) -> windows_core::BOOL);
    unsafe { PeekNamedPipe(hnamedpipe, lpbuffer.unwrap_or(core::mem::zeroed()) as _, nbuffersize, lpbytesread.unwrap_or(core::mem::zeroed()) as _, lptotalbytesavail.unwrap_or(core::mem::zeroed()) as _, lpbytesleftthismessage.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetNamedPipeHandleState(hnamedpipe: super::winnt::HANDLE, lpmode: Option<*const u32>, lpmaxcollectioncount: Option<*const u32>, lpcollectdatatimeout: Option<*const u32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetNamedPipeHandleState(hnamedpipe : super::winnt::HANDLE, lpmode : *const u32, lpmaxcollectioncount : *const u32, lpcollectdatatimeout : *const u32) -> windows_core::BOOL);
    unsafe { SetNamedPipeHandleState(hnamedpipe, lpmode.unwrap_or(core::mem::zeroed()) as _, lpmaxcollectioncount.unwrap_or(core::mem::zeroed()) as _, lpcollectdatatimeout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn TransactNamedPipe(hnamedpipe: super::winnt::HANDLE, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesread: *mut u32, lpoverlapped: Option<*mut super::minwinbase::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TransactNamedPipe(hnamedpipe : super::winnt::HANDLE, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED) -> windows_core::BOOL);
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
