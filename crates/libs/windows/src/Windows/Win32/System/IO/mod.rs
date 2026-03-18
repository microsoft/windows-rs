#[inline]
pub unsafe fn BindIoCompletionCallback(filehandle: super::super::Foundation::HANDLE, function: LPOVERLAPPED_COMPLETION_ROUTINE, flags: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn BindIoCompletionCallback(filehandle : super::super::Foundation:: HANDLE, function : LPOVERLAPPED_COMPLETION_ROUTINE, flags : u32) -> windows_core::BOOL);
    unsafe { BindIoCompletionCallback(filehandle, function, flags) }
}
#[inline]
pub unsafe fn CancelIo(hfile: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CancelIo(hfile : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { CancelIo(hfile) }
}
#[inline]
pub unsafe fn CancelIoEx(hfile: super::super::Foundation::HANDLE, lpoverlapped: *mut OVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CancelIoEx(hfile : super::super::Foundation:: HANDLE, lpoverlapped : *mut OVERLAPPED) -> windows_core::BOOL);
    unsafe { CancelIoEx(hfile, lpoverlapped as _) }
}
#[inline]
pub unsafe fn CancelSynchronousIo(hthread: super::super::Foundation::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CancelSynchronousIo(hthread : super::super::Foundation:: HANDLE) -> windows_core::BOOL);
    unsafe { CancelSynchronousIo(hthread) }
}
#[inline]
pub unsafe fn CreateIoCompletionPort(filehandle: super::super::Foundation::HANDLE, existingcompletionport: super::super::Foundation::HANDLE, completionkey: usize, numberofconcurrentthreads: u32) -> super::super::Foundation::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateIoCompletionPort(filehandle : super::super::Foundation:: HANDLE, existingcompletionport : super::super::Foundation:: HANDLE, completionkey : usize, numberofconcurrentthreads : u32) -> super::super::Foundation:: HANDLE);
    unsafe { CreateIoCompletionPort(filehandle, existingcompletionport, completionkey, numberofconcurrentthreads) }
}
#[inline]
pub unsafe fn DeviceIoControl(hdevice: super::super::Foundation::HANDLE, dwiocontrolcode: u32, lpinbuffer: *const core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut OVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DeviceIoControl(hdevice : super::super::Foundation:: HANDLE, dwiocontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpoverlapped : *mut OVERLAPPED) -> windows_core::BOOL);
    unsafe { DeviceIoControl(hdevice, dwiocontrolcode, lpinbuffer, ninbuffersize, lpoutbuffer as _, noutbuffersize, lpbytesreturned as _, lpoverlapped as _) }
}
#[inline]
pub unsafe fn GetOverlappedResult(hfile: super::super::Foundation::HANDLE, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetOverlappedResult(hfile : super::super::Foundation:: HANDLE, lpoverlapped : *const OVERLAPPED, lpnumberofbytestransferred : *mut u32, bwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetOverlappedResult(hfile, lpoverlapped, lpnumberofbytestransferred as _, bwait.into()) }
}
#[inline]
pub unsafe fn GetOverlappedResultEx(hfile: super::super::Foundation::HANDLE, lpoverlapped: *mut OVERLAPPED, lpnumberofbytestransferred: *mut u32, dwmilliseconds: u32, balertable: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetOverlappedResultEx(hfile : super::super::Foundation:: HANDLE, lpoverlapped : *mut OVERLAPPED, lpnumberofbytestransferred : *mut u32, dwmilliseconds : u32, balertable : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetOverlappedResultEx(hfile, lpoverlapped as _, lpnumberofbytestransferred as _, dwmilliseconds, balertable.into()) }
}
#[inline]
pub unsafe fn GetQueuedCompletionStatus(completionport: super::super::Foundation::HANDLE, lpnumberofbytestransferred: *mut u32, lpcompletionkey: *mut usize, lpoverlapped: *mut *mut OVERLAPPED, dwmilliseconds: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetQueuedCompletionStatus(completionport : super::super::Foundation:: HANDLE, lpnumberofbytestransferred : *mut u32, lpcompletionkey : *mut usize, lpoverlapped : *mut *mut OVERLAPPED, dwmilliseconds : u32) -> windows_core::BOOL);
    unsafe { GetQueuedCompletionStatus(completionport, lpnumberofbytestransferred as _, lpcompletionkey as _, lpoverlapped as _, dwmilliseconds) }
}
#[inline]
pub unsafe fn GetQueuedCompletionStatusEx(completionport: super::super::Foundation::HANDLE, lpcompletionportentries: *mut OVERLAPPED_ENTRY, ulcount: u32, ulnumentriesremoved: *mut u32, dwmilliseconds: u32, falertable: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetQueuedCompletionStatusEx(completionport : super::super::Foundation:: HANDLE, lpcompletionportentries : *mut OVERLAPPED_ENTRY, ulcount : u32, ulnumentriesremoved : *mut u32, dwmilliseconds : u32, falertable : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetQueuedCompletionStatusEx(completionport, lpcompletionportentries as _, ulcount, ulnumentriesremoved as _, dwmilliseconds, falertable.into()) }
}
#[inline]
pub unsafe fn PostQueuedCompletionStatus(completionport: super::super::Foundation::HANDLE, dwnumberofbytestransferred: u32, dwcompletionkey: usize, lpoverlapped: *mut OVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn PostQueuedCompletionStatus(completionport : super::super::Foundation:: HANDLE, dwnumberofbytestransferred : u32, dwcompletionkey : usize, lpoverlapped : *mut OVERLAPPED) -> windows_core::BOOL);
    unsafe { PostQueuedCompletionStatus(completionport, dwnumberofbytestransferred, dwcompletionkey, lpoverlapped as _) }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_STATUS_BLOCK {
    pub Anonymous: IO_STATUS_BLOCK_0,
    pub Information: usize,
}
impl Default for IO_STATUS_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IO_STATUS_BLOCK_0 {
    pub Status: windows_core::NTSTATUS,
    pub Pointer: *mut core::ffi::c_void,
}
impl Default for IO_STATUS_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LPOVERLAPPED_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(dwerrorcode: u32, dwnumberofbytestransfered: u32, lpoverlapped: *mut OVERLAPPED)>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OVERLAPPED {
    pub Internal: usize,
    pub InternalHigh: usize,
    pub Anonymous: OVERLAPPED_0,
    pub hEvent: super::super::Foundation::HANDLE,
}
impl Default for OVERLAPPED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union OVERLAPPED_0 {
    pub Anonymous: OVERLAPPED_0_0,
    pub Pointer: *mut core::ffi::c_void,
}
impl Default for OVERLAPPED_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OVERLAPPED_0_0 {
    pub Offset: u32,
    pub OffsetHigh: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OVERLAPPED_ENTRY {
    pub lpCompletionKey: usize,
    pub lpOverlapped: *mut OVERLAPPED,
    pub Internal: usize,
    pub dwNumberOfBytesTransferred: u32,
}
impl Default for OVERLAPPED_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PIO_APC_ROUTINE = Option<unsafe extern "system" fn(apccontext: *mut core::ffi::c_void, iostatusblock: *mut IO_STATUS_BLOCK, reserved: u32)>;
