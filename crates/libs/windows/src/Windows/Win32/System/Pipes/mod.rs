#[inline]
pub unsafe fn CallNamedPipeA<P0>(lpnamedpipename: P0, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CallNamedPipeA(lpnamedpipename : windows_core::PCSTR, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, ntimeout : u32) -> super::super::Foundation:: BOOL);
    CallNamedPipeA(lpnamedpipename.param().abi(), core::mem::transmute(lpinbuffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(ninbuffersize), core::mem::transmute(lpoutbuffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(noutbuffersize), core::mem::transmute(lpbytesread), core::mem::transmute(ntimeout)).ok()
}
#[inline]
pub unsafe fn CallNamedPipeW<P0>(lpnamedpipename: P0, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CallNamedPipeW(lpnamedpipename : windows_core::PCWSTR, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, ntimeout : u32) -> super::super::Foundation:: BOOL);
    CallNamedPipeW(lpnamedpipename.param().abi(), core::mem::transmute(lpinbuffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(ninbuffersize), core::mem::transmute(lpoutbuffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(noutbuffersize), core::mem::transmute(lpbytesread), core::mem::transmute(ntimeout))
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ConnectNamedPipe(hnamedpipe: super::super::Foundation::HANDLE, lpoverlapped: Option<*mut super::IO::OVERLAPPED>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn ConnectNamedPipe(hnamedpipe : super::super::Foundation:: HANDLE, lpoverlapped : *mut super::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    ConnectNamedPipe(core::mem::transmute(hnamedpipe), core::mem::transmute(lpoverlapped.unwrap_or(core::mem::zeroed()))).ok()
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn CreateNamedPipeA<P0>(lpname: P0, dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, dwpipemode: NAMED_PIPE_MODE, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateNamedPipeA(lpname : windows_core::PCSTR, dwopenmode : super::super::Storage::FileSystem:: FILE_FLAGS_AND_ATTRIBUTES, dwpipemode : NAMED_PIPE_MODE, nmaxinstances : u32, noutbuffersize : u32, ninbuffersize : u32, ndefaulttimeout : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
    let result__ = CreateNamedPipeA(lpname.param().abi(), core::mem::transmute(dwopenmode), core::mem::transmute(dwpipemode), core::mem::transmute(nmaxinstances), core::mem::transmute(noutbuffersize), core::mem::transmute(ninbuffersize), core::mem::transmute(ndefaulttimeout), core::mem::transmute(lpsecurityattributes.unwrap_or(core::mem::zeroed())));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn CreateNamedPipeW<P0>(lpname: P0, dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, dwpipemode: NAMED_PIPE_MODE, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateNamedPipeW(lpname : windows_core::PCWSTR, dwopenmode : super::super::Storage::FileSystem:: FILE_FLAGS_AND_ATTRIBUTES, dwpipemode : NAMED_PIPE_MODE, nmaxinstances : u32, noutbuffersize : u32, ninbuffersize : u32, ndefaulttimeout : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
    CreateNamedPipeW(lpname.param().abi(), core::mem::transmute(dwopenmode), core::mem::transmute(dwpipemode), core::mem::transmute(nmaxinstances), core::mem::transmute(noutbuffersize), core::mem::transmute(ninbuffersize), core::mem::transmute(ndefaulttimeout), core::mem::transmute(lpsecurityattributes.unwrap_or(core::mem::zeroed())))
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreatePipe(hreadpipe: *mut super::super::Foundation::HANDLE, hwritepipe: *mut super::super::Foundation::HANDLE, lppipeattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, nsize: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn CreatePipe(hreadpipe : *mut super::super::Foundation:: HANDLE, hwritepipe : *mut super::super::Foundation:: HANDLE, lppipeattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, nsize : u32) -> super::super::Foundation:: BOOL);
    CreatePipe(core::mem::transmute(hreadpipe), core::mem::transmute(hwritepipe), core::mem::transmute(lppipeattributes.unwrap_or(core::mem::zeroed())), core::mem::transmute(nsize)).ok()
}
#[inline]
pub unsafe fn DisconnectNamedPipe(hnamedpipe: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn DisconnectNamedPipe(hnamedpipe : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    DisconnectNamedPipe(core::mem::transmute(hnamedpipe)).ok()
}
#[inline]
pub unsafe fn GetNamedPipeClientComputerNameA(pipe: super::super::Foundation::HANDLE, clientcomputername: &mut [u8]) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeClientComputerNameA(pipe : super::super::Foundation:: HANDLE, clientcomputername : windows_core::PSTR, clientcomputernamelength : u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeClientComputerNameA(core::mem::transmute(pipe), core::mem::transmute(clientcomputername.as_ptr()), clientcomputername.len().try_into().unwrap()).ok()
}
#[inline]
pub unsafe fn GetNamedPipeClientComputerNameW(pipe: super::super::Foundation::HANDLE, clientcomputername: windows_core::PWSTR, clientcomputernamelength: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeClientComputerNameW(pipe : super::super::Foundation:: HANDLE, clientcomputername : windows_core::PWSTR, clientcomputernamelength : u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeClientComputerNameW(core::mem::transmute(pipe), core::mem::transmute(clientcomputername), core::mem::transmute(clientcomputernamelength))
}
#[inline]
pub unsafe fn GetNamedPipeClientProcessId(pipe: super::super::Foundation::HANDLE, clientprocessid: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeClientProcessId(pipe : super::super::Foundation:: HANDLE, clientprocessid : *mut u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeClientProcessId(core::mem::transmute(pipe), core::mem::transmute(clientprocessid)).ok()
}
#[inline]
pub unsafe fn GetNamedPipeClientSessionId(pipe: super::super::Foundation::HANDLE, clientsessionid: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeClientSessionId(pipe : super::super::Foundation:: HANDLE, clientsessionid : *mut u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeClientSessionId(core::mem::transmute(pipe), core::mem::transmute(clientsessionid)).ok()
}
#[inline]
pub unsafe fn GetNamedPipeHandleStateA(hnamedpipe: super::super::Foundation::HANDLE, lpstate: Option<*mut NAMED_PIPE_MODE>, lpcurinstances: Option<*mut u32>, lpmaxcollectioncount: Option<*mut u32>, lpcollectdatatimeout: Option<*mut u32>, lpusername: Option<&mut [u8]>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeHandleStateA(hnamedpipe : super::super::Foundation:: HANDLE, lpstate : *mut NAMED_PIPE_MODE, lpcurinstances : *mut u32, lpmaxcollectioncount : *mut u32, lpcollectdatatimeout : *mut u32, lpusername : windows_core::PSTR, nmaxusernamesize : u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeHandleStateA(
        core::mem::transmute(hnamedpipe),
        core::mem::transmute(lpstate.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpcurinstances.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpmaxcollectioncount.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpcollectdatatimeout.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpusername.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        lpusername.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
    .ok()
}
#[inline]
pub unsafe fn GetNamedPipeHandleStateW(hnamedpipe: super::super::Foundation::HANDLE, lpstate: Option<*mut NAMED_PIPE_MODE>, lpcurinstances: Option<*mut u32>, lpmaxcollectioncount: Option<*mut u32>, lpcollectdatatimeout: Option<*mut u32>, lpusername: Option<&mut [u16]>) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeHandleStateW(hnamedpipe : super::super::Foundation:: HANDLE, lpstate : *mut NAMED_PIPE_MODE, lpcurinstances : *mut u32, lpmaxcollectioncount : *mut u32, lpcollectdatatimeout : *mut u32, lpusername : windows_core::PWSTR, nmaxusernamesize : u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeHandleStateW(
        core::mem::transmute(hnamedpipe),
        core::mem::transmute(lpstate.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpcurinstances.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpmaxcollectioncount.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpcollectdatatimeout.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(lpusername.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        lpusername.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn GetNamedPipeInfo(hnamedpipe: super::super::Foundation::HANDLE, lpflags: Option<*mut NAMED_PIPE_MODE>, lpoutbuffersize: Option<*mut u32>, lpinbuffersize: Option<*mut u32>, lpmaxinstances: Option<*mut u32>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeInfo(hnamedpipe : super::super::Foundation:: HANDLE, lpflags : *mut NAMED_PIPE_MODE, lpoutbuffersize : *mut u32, lpinbuffersize : *mut u32, lpmaxinstances : *mut u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeInfo(core::mem::transmute(hnamedpipe), core::mem::transmute(lpflags.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpoutbuffersize.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpinbuffersize.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpmaxinstances.unwrap_or(core::mem::zeroed()))).ok()
}
#[inline]
pub unsafe fn GetNamedPipeServerProcessId(pipe: super::super::Foundation::HANDLE, serverprocessid: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeServerProcessId(pipe : super::super::Foundation:: HANDLE, serverprocessid : *mut u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeServerProcessId(core::mem::transmute(pipe), core::mem::transmute(serverprocessid)).ok()
}
#[inline]
pub unsafe fn GetNamedPipeServerSessionId(pipe: super::super::Foundation::HANDLE, serversessionid: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetNamedPipeServerSessionId(pipe : super::super::Foundation:: HANDLE, serversessionid : *mut u32) -> super::super::Foundation:: BOOL);
    GetNamedPipeServerSessionId(core::mem::transmute(pipe), core::mem::transmute(serversessionid)).ok()
}
#[inline]
pub unsafe fn ImpersonateNamedPipeClient(hnamedpipe: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("advapi32.dll" "system" fn ImpersonateNamedPipeClient(hnamedpipe : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    ImpersonateNamedPipeClient(core::mem::transmute(hnamedpipe)).ok()
}
#[inline]
pub unsafe fn PeekNamedPipe(hnamedpipe: super::super::Foundation::HANDLE, lpbuffer: Option<*mut core::ffi::c_void>, nbuffersize: u32, lpbytesread: Option<*mut u32>, lptotalbytesavail: Option<*mut u32>, lpbytesleftthismessage: Option<*mut u32>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn PeekNamedPipe(hnamedpipe : super::super::Foundation:: HANDLE, lpbuffer : *mut core::ffi::c_void, nbuffersize : u32, lpbytesread : *mut u32, lptotalbytesavail : *mut u32, lpbytesleftthismessage : *mut u32) -> super::super::Foundation:: BOOL);
    PeekNamedPipe(core::mem::transmute(hnamedpipe), core::mem::transmute(lpbuffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(nbuffersize), core::mem::transmute(lpbytesread.unwrap_or(core::mem::zeroed())), core::mem::transmute(lptotalbytesavail.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpbytesleftthismessage.unwrap_or(core::mem::zeroed()))).ok()
}
#[inline]
pub unsafe fn SetNamedPipeHandleState(hnamedpipe: super::super::Foundation::HANDLE, lpmode: Option<*const NAMED_PIPE_MODE>, lpmaxcollectioncount: Option<*const u32>, lpcollectdatatimeout: Option<*const u32>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetNamedPipeHandleState(hnamedpipe : super::super::Foundation:: HANDLE, lpmode : *const NAMED_PIPE_MODE, lpmaxcollectioncount : *const u32, lpcollectdatatimeout : *const u32) -> super::super::Foundation:: BOOL);
    SetNamedPipeHandleState(core::mem::transmute(hnamedpipe), core::mem::transmute(lpmode.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpmaxcollectioncount.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpcollectdatatimeout.unwrap_or(core::mem::zeroed()))).ok()
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn TransactNamedPipe(hnamedpipe: super::super::Foundation::HANDLE, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesread: *mut u32, lpoverlapped: Option<*mut super::IO::OVERLAPPED>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn TransactNamedPipe(hnamedpipe : super::super::Foundation:: HANDLE, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, lpoverlapped : *mut super::IO:: OVERLAPPED) -> super::super::Foundation:: BOOL);
    TransactNamedPipe(core::mem::transmute(hnamedpipe), core::mem::transmute(lpinbuffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(ninbuffersize), core::mem::transmute(lpoutbuffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(noutbuffersize), core::mem::transmute(lpbytesread), core::mem::transmute(lpoverlapped.unwrap_or(core::mem::zeroed()))).ok()
}
#[inline]
pub unsafe fn WaitNamedPipeA<P0>(lpnamedpipename: P0, ntimeout: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn WaitNamedPipeA(lpnamedpipename : windows_core::PCSTR, ntimeout : u32) -> super::super::Foundation:: BOOL);
    WaitNamedPipeA(lpnamedpipename.param().abi(), core::mem::transmute(ntimeout)).ok()
}
#[inline]
pub unsafe fn WaitNamedPipeW<P0>(lpnamedpipename: P0, ntimeout: u32) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn WaitNamedPipeW(lpnamedpipename : windows_core::PCWSTR, ntimeout : u32) -> super::super::Foundation:: BOOL);
    WaitNamedPipeW(lpnamedpipename.param().abi(), core::mem::transmute(ntimeout))
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NAMED_PIPE_MODE(pub u32);
impl NAMED_PIPE_MODE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for NAMED_PIPE_MODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for NAMED_PIPE_MODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for NAMED_PIPE_MODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for NAMED_PIPE_MODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for NAMED_PIPE_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const NMPWAIT_NOWAIT: u32 = 1u32;
pub const NMPWAIT_USE_DEFAULT_WAIT: u32 = 0u32;
pub const NMPWAIT_WAIT_FOREVER: u32 = 4294967295u32;
pub const PIPE_ACCEPT_REMOTE_CLIENTS: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
pub const PIPE_CLIENT_END: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
pub const PIPE_NOWAIT: NAMED_PIPE_MODE = NAMED_PIPE_MODE(1u32);
pub const PIPE_READMODE_BYTE: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
pub const PIPE_READMODE_MESSAGE: NAMED_PIPE_MODE = NAMED_PIPE_MODE(2u32);
pub const PIPE_REJECT_REMOTE_CLIENTS: NAMED_PIPE_MODE = NAMED_PIPE_MODE(8u32);
pub const PIPE_SERVER_END: NAMED_PIPE_MODE = NAMED_PIPE_MODE(1u32);
pub const PIPE_TYPE_BYTE: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
pub const PIPE_TYPE_MESSAGE: NAMED_PIPE_MODE = NAMED_PIPE_MODE(4u32);
pub const PIPE_UNLIMITED_INSTANCES: u32 = 255u32;
pub const PIPE_WAIT: NAMED_PIPE_MODE = NAMED_PIPE_MODE(0u32);
