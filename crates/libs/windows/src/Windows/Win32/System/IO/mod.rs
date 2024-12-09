#[inline]
pub unsafe fn BindIoCompletionCallback(filehandle: super::super::Foundation::HANDLE, function: LPOVERLAPPED_COMPLETION_ROUTINE, flags: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn BindIoCompletionCallback(filehandle : super::super::Foundation:: HANDLE, function : LPOVERLAPPED_COMPLETION_ROUTINE, flags : u32) -> super::super::Foundation:: BOOL);
    BindIoCompletionCallback(core::mem::transmute(filehandle), core::mem::transmute(function), core::mem::transmute(flags)).ok()
}
#[inline]
pub unsafe fn CancelIo(hfile: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn CancelIo(hfile : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    CancelIo(core::mem::transmute(hfile)).ok()
}
#[inline]
pub unsafe fn CancelIoEx(hfile: super::super::Foundation::HANDLE, lpoverlapped: Option<*const OVERLAPPED>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn CancelIoEx(hfile : super::super::Foundation:: HANDLE, lpoverlapped : *const OVERLAPPED) -> super::super::Foundation:: BOOL);
    CancelIoEx(core::mem::transmute(hfile), core::mem::transmute(lpoverlapped.unwrap_or(core::mem::zeroed()))).ok()
}
#[inline]
pub unsafe fn CancelSynchronousIo(hthread: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn CancelSynchronousIo(hthread : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    CancelSynchronousIo(core::mem::transmute(hthread)).ok()
}
#[inline]
pub unsafe fn CreateIoCompletionPort(filehandle: super::super::Foundation::HANDLE, existingcompletionport: Option<super::super::Foundation::HANDLE>, completionkey: usize, numberofconcurrentthreads: u32) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("kernel32.dll" "system" fn CreateIoCompletionPort(filehandle : super::super::Foundation:: HANDLE, existingcompletionport : super::super::Foundation:: HANDLE, completionkey : usize, numberofconcurrentthreads : u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateIoCompletionPort(core::mem::transmute(filehandle), core::mem::transmute(existingcompletionport.unwrap_or(core::mem::zeroed())), core::mem::transmute(completionkey), core::mem::transmute(numberofconcurrentthreads));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn DeviceIoControl(hdevice: super::super::Foundation::HANDLE, dwiocontrolcode: u32, lpinbuffer: Option<*const core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: Option<*mut core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: Option<*mut u32>, lpoverlapped: Option<*mut OVERLAPPED>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn DeviceIoControl(hdevice : super::super::Foundation:: HANDLE, dwiocontrolcode : u32, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesreturned : *mut u32, lpoverlapped : *mut OVERLAPPED) -> super::super::Foundation:: BOOL);
    DeviceIoControl(core::mem::transmute(hdevice), core::mem::transmute(dwiocontrolcode), core::mem::transmute(lpinbuffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(ninbuffersize), core::mem::transmute(lpoutbuffer.unwrap_or(core::mem::zeroed())), core::mem::transmute(noutbuffersize), core::mem::transmute(lpbytesreturned.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpoverlapped.unwrap_or(core::mem::zeroed()))).ok()
}
#[inline]
pub unsafe fn GetOverlappedResult<P3>(hfile: super::super::Foundation::HANDLE, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: P3) -> windows_core::Result<()>
where
    P3: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetOverlappedResult(hfile : super::super::Foundation:: HANDLE, lpoverlapped : *const OVERLAPPED, lpnumberofbytestransferred : *mut u32, bwait : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    GetOverlappedResult(core::mem::transmute(hfile), core::mem::transmute(lpoverlapped), core::mem::transmute(lpnumberofbytestransferred), bwait.param().abi()).ok()
}
#[inline]
pub unsafe fn GetOverlappedResultEx<P4>(hfile: super::super::Foundation::HANDLE, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, dwmilliseconds: u32, balertable: P4) -> windows_core::Result<()>
where
    P4: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetOverlappedResultEx(hfile : super::super::Foundation:: HANDLE, lpoverlapped : *const OVERLAPPED, lpnumberofbytestransferred : *mut u32, dwmilliseconds : u32, balertable : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    GetOverlappedResultEx(core::mem::transmute(hfile), core::mem::transmute(lpoverlapped), core::mem::transmute(lpnumberofbytestransferred), core::mem::transmute(dwmilliseconds), balertable.param().abi()).ok()
}
#[inline]
pub unsafe fn GetQueuedCompletionStatus(completionport: super::super::Foundation::HANDLE, lpnumberofbytestransferred: *mut u32, lpcompletionkey: *mut usize, lpoverlapped: *mut *mut OVERLAPPED, dwmilliseconds: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetQueuedCompletionStatus(completionport : super::super::Foundation:: HANDLE, lpnumberofbytestransferred : *mut u32, lpcompletionkey : *mut usize, lpoverlapped : *mut *mut OVERLAPPED, dwmilliseconds : u32) -> super::super::Foundation:: BOOL);
    GetQueuedCompletionStatus(core::mem::transmute(completionport), core::mem::transmute(lpnumberofbytestransferred), core::mem::transmute(lpcompletionkey), core::mem::transmute(lpoverlapped), core::mem::transmute(dwmilliseconds)).ok()
}
#[inline]
pub unsafe fn GetQueuedCompletionStatusEx<P5>(completionport: super::super::Foundation::HANDLE, lpcompletionportentries: &mut [OVERLAPPED_ENTRY], ulnumentriesremoved: *mut u32, dwmilliseconds: u32, falertable: P5) -> windows_core::Result<()>
where
    P5: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("kernel32.dll" "system" fn GetQueuedCompletionStatusEx(completionport : super::super::Foundation:: HANDLE, lpcompletionportentries : *mut OVERLAPPED_ENTRY, ulcount : u32, ulnumentriesremoved : *mut u32, dwmilliseconds : u32, falertable : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    GetQueuedCompletionStatusEx(core::mem::transmute(completionport), core::mem::transmute(lpcompletionportentries.as_ptr()), lpcompletionportentries.len().try_into().unwrap(), core::mem::transmute(ulnumentriesremoved), core::mem::transmute(dwmilliseconds), falertable.param().abi()).ok()
}
#[inline]
pub unsafe fn PostQueuedCompletionStatus(completionport: super::super::Foundation::HANDLE, dwnumberofbytestransferred: u32, dwcompletionkey: usize, lpoverlapped: Option<*const OVERLAPPED>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn PostQueuedCompletionStatus(completionport : super::super::Foundation:: HANDLE, dwnumberofbytestransferred : u32, dwcompletionkey : usize, lpoverlapped : *const OVERLAPPED) -> super::super::Foundation:: BOOL);
    PostQueuedCompletionStatus(core::mem::transmute(completionport), core::mem::transmute(dwnumberofbytestransferred), core::mem::transmute(dwcompletionkey), core::mem::transmute(lpoverlapped.unwrap_or(core::mem::zeroed()))).ok()
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
impl windows_core::TypeKind for IO_STATUS_BLOCK {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IO_STATUS_BLOCK_0 {
    pub Status: super::super::Foundation::NTSTATUS,
    pub Pointer: *mut core::ffi::c_void,
}
impl Default for IO_STATUS_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IO_STATUS_BLOCK_0 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for OVERLAPPED {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for OVERLAPPED_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OVERLAPPED_0_0 {
    pub Offset: u32,
    pub OffsetHigh: u32,
}
impl Default for OVERLAPPED_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for OVERLAPPED_0_0 {
    type TypeKind = windows_core::CopyType;
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
impl windows_core::TypeKind for OVERLAPPED_ENTRY {
    type TypeKind = windows_core::CopyType;
}
pub type PIO_APC_ROUTINE = Option<unsafe extern "system" fn(apccontext: *mut core::ffi::c_void, iostatusblock: *mut IO_STATUS_BLOCK, reserved: u32)>;
