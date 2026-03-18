#[inline]
pub unsafe fn CallNamedPipeA<P0>(lpnamedpipename: P0, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CallNamedPipeA(lpnamedpipename : windows_core::PCSTR, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, ntimeout : u32) -> windows_core::BOOL);
    unsafe { CallNamedPipeA(lpnamedpipename.param().abi(), lpinbuffer, ninbuffersize, lpoutbuffer as _, noutbuffersize, lpbytesread as _, ntimeout) }
}
#[inline]
pub unsafe fn CallNamedPipeW<P0>(lpnamedpipename: P0, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, ntimeout: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CallNamedPipeW(lpnamedpipename : windows_core::PCWSTR, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, ntimeout : u32) -> windows_core::BOOL);
    unsafe { CallNamedPipeW(lpnamedpipename.param().abi(), lpinbuffer, ninbuffersize, lpoutbuffer as _, noutbuffersize, lpbytesread as _, ntimeout) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn ConnectNamedPipe(hnamedpipe: super::super::Foundation::HANDLE, lpoverlapped: *mut super::IO::OVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ConnectNamedPipe(hnamedpipe : super::super::Foundation:: HANDLE, lpoverlapped : *mut super::IO:: OVERLAPPED) -> windows_core::BOOL);
    unsafe { ConnectNamedPipe(hnamedpipe, lpoverlapped as _) }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn CreateNamedPipeA<P0>(lpname: P0, dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, dwpipemode: NAMED_PIPE_MODE, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateNamedPipeA(lpname : windows_core::PCSTR, dwopenmode : super::super::Storage::FileSystem:: FILE_FLAGS_AND_ATTRIBUTES, dwpipemode : NAMED_PIPE_MODE, nmaxinstances : u32, noutbuffersize : u32, ninbuffersize : u32, ndefaulttimeout : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
    unsafe { CreateNamedPipeA(lpname.param().abi(), dwopenmode, dwpipemode, nmaxinstances, noutbuffersize, ninbuffersize, ndefaulttimeout, lpsecurityattributes) }
}
#[cfg(all(feature = "Win32_Security", feature = "Win32_Storage_FileSystem"))]
#[inline]
pub unsafe fn CreateNamedPipeW<P0>(lpname: P0, dwopenmode: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, dwpipemode: NAMED_PIPE_MODE, nmaxinstances: u32, noutbuffersize: u32, ninbuffersize: u32, ndefaulttimeout: u32, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> super::super::Foundation::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateNamedPipeW(lpname : windows_core::PCWSTR, dwopenmode : super::super::Storage::FileSystem:: FILE_FLAGS_AND_ATTRIBUTES, dwpipemode : NAMED_PIPE_MODE, nmaxinstances : u32, noutbuffersize : u32, ninbuffersize : u32, ndefaulttimeout : u32, lpsecurityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES) -> super::super::Foundation:: HANDLE);
    unsafe { CreateNamedPipeW(lpname.param().abi(), dwopenmode, dwpipemode, nmaxinstances, noutbuffersize, ninbuffersize, ndefaulttimeout, lpsecurityattributes) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreatePipe(hreadpipe: *mut super::super::Foundation::HANDLE, hwritepipe: *mut super::super::Foundation::HANDLE, lppipeattributes: *const super::super::Security::SECURITY_ATTRIBUTES, nsize: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CreatePipe(hreadpipe : *mut super::super::Foundation:: HANDLE, hwritepipe : *mut super::super::Foundation:: HANDLE, lppipeattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, nsize : u32) -> windows_core::BOOL);
    unsafe { CreatePipe(hreadpipe as _, hwritepipe as _, lppipeattributes, nsize) }
}
#[inline]
pub unsafe fn DisconnectNamedPipe(hnamedpipe: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DisconnectNamedPipe(hnamedpipe : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { DisconnectNamedPipe(hnamedpipe) }
}
#[inline]
pub unsafe fn GetNamedPipeClientComputerNameA(pipe: super::super::Foundation::HANDLE, clientcomputername: windows_core::PSTR, clientcomputernamelength: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeClientComputerNameA(pipe : super::super::Foundation:: HANDLE, clientcomputername : windows_core::PSTR, clientcomputernamelength : u32) -> windows_core::BOOL);
    unsafe { GetNamedPipeClientComputerNameA(pipe, core::mem::transmute(clientcomputername), clientcomputernamelength) }
}
#[inline]
pub unsafe fn GetNamedPipeClientComputerNameW(pipe: super::super::Foundation::HANDLE, clientcomputername: windows_core::PWSTR, clientcomputernamelength: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeClientComputerNameW(pipe : super::super::Foundation:: HANDLE, clientcomputername : windows_core::PWSTR, clientcomputernamelength : u32) -> windows_core::BOOL);
    unsafe { GetNamedPipeClientComputerNameW(pipe, core::mem::transmute(clientcomputername), clientcomputernamelength) }
}
#[inline]
pub unsafe fn GetNamedPipeClientProcessId(pipe: super::super::Foundation::HANDLE, clientprocessid: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeClientProcessId(pipe : super::super::Foundation:: HANDLE, clientprocessid : *mut u32) -> windows_core::BOOL);
    unsafe { GetNamedPipeClientProcessId(pipe, clientprocessid as _) }
}
#[inline]
pub unsafe fn GetNamedPipeClientSessionId(pipe: super::super::Foundation::HANDLE, clientsessionid: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeClientSessionId(pipe : super::super::Foundation:: HANDLE, clientsessionid : *mut u32) -> windows_core::BOOL);
    unsafe { GetNamedPipeClientSessionId(pipe, clientsessionid as _) }
}
#[inline]
pub unsafe fn GetNamedPipeHandleStateA(hnamedpipe: super::super::Foundation::HANDLE, lpstate: *mut NAMED_PIPE_MODE, lpcurinstances: *mut u32, lpmaxcollectioncount: *mut u32, lpcollectdatatimeout: *mut u32, lpusername: windows_core::PSTR, nmaxusernamesize: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeHandleStateA(hnamedpipe : super::super::Foundation:: HANDLE, lpstate : *mut NAMED_PIPE_MODE, lpcurinstances : *mut u32, lpmaxcollectioncount : *mut u32, lpcollectdatatimeout : *mut u32, lpusername : windows_core::PSTR, nmaxusernamesize : u32) -> windows_core::BOOL);
    unsafe { GetNamedPipeHandleStateA(hnamedpipe, lpstate as _, lpcurinstances as _, lpmaxcollectioncount as _, lpcollectdatatimeout as _, core::mem::transmute(lpusername), nmaxusernamesize) }
}
#[inline]
pub unsafe fn GetNamedPipeHandleStateW(hnamedpipe: super::super::Foundation::HANDLE, lpstate: *mut NAMED_PIPE_MODE, lpcurinstances: *mut u32, lpmaxcollectioncount: *mut u32, lpcollectdatatimeout: *mut u32, lpusername: windows_core::PWSTR, nmaxusernamesize: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeHandleStateW(hnamedpipe : super::super::Foundation:: HANDLE, lpstate : *mut NAMED_PIPE_MODE, lpcurinstances : *mut u32, lpmaxcollectioncount : *mut u32, lpcollectdatatimeout : *mut u32, lpusername : windows_core::PWSTR, nmaxusernamesize : u32) -> windows_core::BOOL);
    unsafe { GetNamedPipeHandleStateW(hnamedpipe, lpstate as _, lpcurinstances as _, lpmaxcollectioncount as _, lpcollectdatatimeout as _, core::mem::transmute(lpusername), nmaxusernamesize) }
}
#[inline]
pub unsafe fn GetNamedPipeInfo(hnamedpipe: super::super::Foundation::HANDLE, lpflags: *mut NAMED_PIPE_MODE, lpoutbuffersize: *mut u32, lpinbuffersize: *mut u32, lpmaxinstances: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeInfo(hnamedpipe : super::super::Foundation:: HANDLE, lpflags : *mut NAMED_PIPE_MODE, lpoutbuffersize : *mut u32, lpinbuffersize : *mut u32, lpmaxinstances : *mut u32) -> windows_core::BOOL);
    unsafe { GetNamedPipeInfo(hnamedpipe, lpflags as _, lpoutbuffersize as _, lpinbuffersize as _, lpmaxinstances as _) }
}
#[inline]
pub unsafe fn GetNamedPipeServerProcessId(pipe: super::super::Foundation::HANDLE, serverprocessid: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeServerProcessId(pipe : super::super::Foundation:: HANDLE, serverprocessid : *mut u32) -> windows_core::BOOL);
    unsafe { GetNamedPipeServerProcessId(pipe, serverprocessid as _) }
}
#[inline]
pub unsafe fn GetNamedPipeServerSessionId(pipe: super::super::Foundation::HANDLE, serversessionid: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeServerSessionId(pipe : super::super::Foundation:: HANDLE, serversessionid : *mut u32) -> windows_core::BOOL);
    unsafe { GetNamedPipeServerSessionId(pipe, serversessionid as _) }
}
#[inline]
pub unsafe fn ImpersonateNamedPipeClient(hnamedpipe: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ImpersonateNamedPipeClient(hnamedpipe : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { ImpersonateNamedPipeClient(hnamedpipe) }
}
#[inline]
pub unsafe fn PeekNamedPipe(hnamedpipe: super::super::Foundation::HANDLE, lpbuffer: *mut core::ffi::c_void, nbuffersize: u32, lpbytesread: *mut u32, lptotalbytesavail: *mut u32, lpbytesleftthismessage: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn PeekNamedPipe(hnamedpipe : super::super::Foundation:: HANDLE, lpbuffer : *mut core::ffi::c_void, nbuffersize : u32, lpbytesread : *mut u32, lptotalbytesavail : *mut u32, lpbytesleftthismessage : *mut u32) -> windows_core::BOOL);
    unsafe { PeekNamedPipe(hnamedpipe, lpbuffer as _, nbuffersize, lpbytesread as _, lptotalbytesavail as _, lpbytesleftthismessage as _) }
}
#[inline]
pub unsafe fn SetNamedPipeHandleState(hnamedpipe: super::super::Foundation::HANDLE, lpmode: *const NAMED_PIPE_MODE, lpmaxcollectioncount: *const u32, lpcollectdatatimeout: *const u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetNamedPipeHandleState(hnamedpipe : super::super::Foundation:: HANDLE, lpmode : *const NAMED_PIPE_MODE, lpmaxcollectioncount : *const u32, lpcollectdatatimeout : *const u32) -> windows_core::BOOL);
    unsafe { SetNamedPipeHandleState(hnamedpipe, lpmode, lpmaxcollectioncount, lpcollectdatatimeout) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn TransactNamedPipe(hnamedpipe: super::super::Foundation::HANDLE, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesread: *mut u32, lpoverlapped: *mut super::IO::OVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TransactNamedPipe(hnamedpipe : super::super::Foundation:: HANDLE, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, lpoverlapped : *mut super::IO:: OVERLAPPED) -> windows_core::BOOL);
    unsafe { TransactNamedPipe(hnamedpipe, lpinbuffer, ninbuffersize, lpoutbuffer as _, noutbuffersize, lpbytesread as _, lpoverlapped as _) }
}
#[inline]
pub unsafe fn WaitNamedPipeA<P0>(lpnamedpipename: P0, ntimeout: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WaitNamedPipeA(lpnamedpipename : windows_core::PCSTR, ntimeout : u32) -> windows_core::BOOL);
    unsafe { WaitNamedPipeA(lpnamedpipename.param().abi(), ntimeout) }
}
#[inline]
pub unsafe fn WaitNamedPipeW<P0>(lpnamedpipename: P0, ntimeout: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WaitNamedPipeW(lpnamedpipename : windows_core::PCWSTR, ntimeout : u32) -> windows_core::BOOL);
    unsafe { WaitNamedPipeW(lpnamedpipename.param().abi(), ntimeout) }
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
