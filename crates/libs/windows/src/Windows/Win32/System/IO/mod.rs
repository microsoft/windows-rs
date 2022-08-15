#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BindIoCompletionCallback<'a, P0>(filehandle: P0, function: LPOVERLAPPED_COMPLETION_ROUTINE, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn BindIoCompletionCallback(filehandle: super::super::Foundation::HANDLE, function: *mut ::core::ffi::c_void, flags: u32) -> super::super::Foundation::BOOL;
    }
    BindIoCompletionCallback(filehandle.into(), ::core::mem::transmute(function), flags)
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelIo<'a, P0>(hfile: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CancelIo(hfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    }
    CancelIo(hfile.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelIoEx<'a, P0>(hfile: P0, lpoverlapped: ::core::option::Option<&OVERLAPPED>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CancelIoEx(hfile: super::super::Foundation::HANDLE, lpoverlapped: *const OVERLAPPED) -> super::super::Foundation::BOOL;
    }
    CancelIoEx(hfile.into(), ::core::mem::transmute(lpoverlapped))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelSynchronousIo<'a, P0>(hthread: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CancelSynchronousIo(hthread: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    }
    CancelSynchronousIo(hthread.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateIoCompletionPort<'a, P0, P1>(filehandle: P0, existingcompletionport: P1, completionkey: usize, numberofconcurrentthreads: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateIoCompletionPort(filehandle: super::super::Foundation::HANDLE, existingcompletionport: super::super::Foundation::HANDLE, completionkey: usize, numberofconcurrentthreads: u32) -> super::super::Foundation::HANDLE;
    }
    let result__ = CreateIoCompletionPort(filehandle.into(), existingcompletionport.into(), completionkey, numberofconcurrentthreads);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeviceIoControl<'a, P0>(hdevice: P0, dwiocontrolcode: u32, lpinbuffer: ::core::option::Option<&[u8]>, lpoutbuffer: ::core::option::Option<&mut [u8]>, lpbytesreturned: ::core::option::Option<&mut u32>, lpoverlapped: ::core::option::Option<&mut OVERLAPPED>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeviceIoControl(hdevice: super::super::Foundation::HANDLE, dwiocontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut OVERLAPPED) -> super::super::Foundation::BOOL;
    }
    DeviceIoControl(hdevice.into(), dwiocontrolcode, ::core::mem::transmute(lpinbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpinbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpoutbuffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpoutbuffer.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpbytesreturned), ::core::mem::transmute(lpoverlapped))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOverlappedResult<'a, P0, P1>(hfile: P0, lpoverlapped: &OVERLAPPED, lpnumberofbytestransferred: &mut u32, bwait: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetOverlappedResult(hfile: super::super::Foundation::HANDLE, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    GetOverlappedResult(hfile.into(), ::core::mem::transmute(lpoverlapped), ::core::mem::transmute(lpnumberofbytestransferred), bwait.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOverlappedResultEx<'a, P0, P1>(hfile: P0, lpoverlapped: &OVERLAPPED, lpnumberofbytestransferred: &mut u32, dwmilliseconds: u32, balertable: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetOverlappedResultEx(hfile: super::super::Foundation::HANDLE, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    GetOverlappedResultEx(hfile.into(), ::core::mem::transmute(lpoverlapped), ::core::mem::transmute(lpnumberofbytestransferred), dwmilliseconds, balertable.into())
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetQueuedCompletionStatus<'a, P0>(completionport: P0, lpnumberofbytestransferred: &mut u32, lpcompletionkey: &mut usize, lpoverlapped: &mut *mut OVERLAPPED, dwmilliseconds: u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetQueuedCompletionStatus(completionport: super::super::Foundation::HANDLE, lpnumberofbytestransferred: *mut u32, lpcompletionkey: *mut usize, lpoverlapped: *mut *mut OVERLAPPED, dwmilliseconds: u32) -> super::super::Foundation::BOOL;
    }
    GetQueuedCompletionStatus(completionport.into(), ::core::mem::transmute(lpnumberofbytestransferred), ::core::mem::transmute(lpcompletionkey), ::core::mem::transmute(lpoverlapped), dwmilliseconds)
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetQueuedCompletionStatusEx<'a, P0, P1>(completionport: P0, lpcompletionportentries: &mut [OVERLAPPED_ENTRY], ulnumentriesremoved: &mut u32, dwmilliseconds: u32, falertable: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetQueuedCompletionStatusEx(completionport: super::super::Foundation::HANDLE, lpcompletionportentries: *mut OVERLAPPED_ENTRY, ulcount: u32, ulnumentriesremoved: *mut u32, dwmilliseconds: u32, falertable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    GetQueuedCompletionStatusEx(completionport.into(), ::core::mem::transmute(lpcompletionportentries.as_ptr()), lpcompletionportentries.len() as _, ::core::mem::transmute(ulnumentriesremoved), dwmilliseconds, falertable.into())
}
#[cfg(feature = "Win32_Foundation")]
pub type LPOVERLAPPED_COMPLETION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(dwerrorcode: u32, dwnumberofbytestransfered: u32, lpoverlapped: *mut OVERLAPPED)>;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OVERLAPPED {
    pub Internal: usize,
    pub InternalHigh: usize,
    pub Anonymous: OVERLAPPED_0,
    pub hEvent: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OVERLAPPED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OVERLAPPED {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OVERLAPPED {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OVERLAPPED {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OVERLAPPED>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OVERLAPPED {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OVERLAPPED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union OVERLAPPED_0 {
    pub Anonymous: OVERLAPPED_0_0,
    pub Pointer: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OVERLAPPED_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OVERLAPPED_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OVERLAPPED_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OVERLAPPED_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OVERLAPPED_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OVERLAPPED_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OVERLAPPED_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OVERLAPPED_0_0 {
    pub Offset: u32,
    pub OffsetHigh: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OVERLAPPED_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OVERLAPPED_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OVERLAPPED_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OVERLAPPED_0_0").field("Offset", &self.Offset).field("OffsetHigh", &self.OffsetHigh).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OVERLAPPED_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OVERLAPPED_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OVERLAPPED_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OVERLAPPED_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OVERLAPPED_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OVERLAPPED_ENTRY {
    pub lpCompletionKey: usize,
    pub lpOverlapped: *mut OVERLAPPED,
    pub Internal: usize,
    pub dwNumberOfBytesTransferred: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OVERLAPPED_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OVERLAPPED_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OVERLAPPED_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OVERLAPPED_ENTRY").field("lpCompletionKey", &self.lpCompletionKey).field("lpOverlapped", &self.lpOverlapped).field("Internal", &self.Internal).field("dwNumberOfBytesTransferred", &self.dwNumberOfBytesTransferred).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for OVERLAPPED_ENTRY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OVERLAPPED_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<OVERLAPPED_ENTRY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OVERLAPPED_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OVERLAPPED_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PostQueuedCompletionStatus<'a, P0>(completionport: P0, dwnumberofbytestransferred: u32, dwcompletionkey: usize, lpoverlapped: ::core::option::Option<&OVERLAPPED>) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PostQueuedCompletionStatus(completionport: super::super::Foundation::HANDLE, dwnumberofbytestransferred: u32, dwcompletionkey: usize, lpoverlapped: *const OVERLAPPED) -> super::super::Foundation::BOOL;
    }
    PostQueuedCompletionStatus(completionport.into(), dwnumberofbytestransferred, dwcompletionkey, ::core::mem::transmute(lpoverlapped))
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
