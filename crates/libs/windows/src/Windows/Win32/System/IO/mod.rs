#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BindIoCompletionCallback<P0>(filehandle: P0, function: LPOVERLAPPED_COMPLETION_ROUTINE, flags: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn BindIoCompletionCallback ( filehandle : super::super::Foundation:: HANDLE , function : LPOVERLAPPED_COMPLETION_ROUTINE , flags : u32 ) -> super::super::Foundation:: BOOL );
    BindIoCompletionCallback(filehandle.into_param().abi(), function, flags)
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelIo<P0>(hfile: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CancelIo ( hfile : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    CancelIo(hfile.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelIoEx<P0>(hfile: P0, lpoverlapped: ::core::option::Option<*const OVERLAPPED>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CancelIoEx ( hfile : super::super::Foundation:: HANDLE , lpoverlapped : *const OVERLAPPED ) -> super::super::Foundation:: BOOL );
    CancelIoEx(hfile.into_param().abi(), ::core::mem::transmute(lpoverlapped.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelSynchronousIo<P0>(hthread: P0) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CancelSynchronousIo ( hthread : super::super::Foundation:: HANDLE ) -> super::super::Foundation:: BOOL );
    CancelSynchronousIo(hthread.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateIoCompletionPort<P0, P1>(filehandle: P0, existingcompletionport: P1, completionkey: usize, numberofconcurrentthreads: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn CreateIoCompletionPort ( filehandle : super::super::Foundation:: HANDLE , existingcompletionport : super::super::Foundation:: HANDLE , completionkey : usize , numberofconcurrentthreads : u32 ) -> super::super::Foundation:: HANDLE );
    let result__ = CreateIoCompletionPort(filehandle.into_param().abi(), existingcompletionport.into_param().abi(), completionkey, numberofconcurrentthreads);
    ::windows::imp::then(!result__.is_invalid(), || result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeviceIoControl<P0>(hdevice: P0, dwiocontrolcode: u32, lpinbuffer: ::core::option::Option<*const ::core::ffi::c_void>, ninbuffersize: u32, lpoutbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, noutbuffersize: u32, lpbytesreturned: ::core::option::Option<*mut u32>, lpoverlapped: ::core::option::Option<*mut OVERLAPPED>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn DeviceIoControl ( hdevice : super::super::Foundation:: HANDLE , dwiocontrolcode : u32 , lpinbuffer : *const ::core::ffi::c_void , ninbuffersize : u32 , lpoutbuffer : *mut ::core::ffi::c_void , noutbuffersize : u32 , lpbytesreturned : *mut u32 , lpoverlapped : *mut OVERLAPPED ) -> super::super::Foundation:: BOOL );
    DeviceIoControl(hdevice.into_param().abi(), dwiocontrolcode, ::core::mem::transmute(lpinbuffer.unwrap_or(::std::ptr::null())), ninbuffersize, ::core::mem::transmute(lpoutbuffer.unwrap_or(::std::ptr::null_mut())), noutbuffersize, ::core::mem::transmute(lpbytesreturned.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpoverlapped.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOverlappedResult<P0, P1>(hfile: P0, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetOverlappedResult ( hfile : super::super::Foundation:: HANDLE , lpoverlapped : *const OVERLAPPED , lpnumberofbytestransferred : *mut u32 , bwait : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    GetOverlappedResult(hfile.into_param().abi(), lpoverlapped, lpnumberofbytestransferred, bwait.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOverlappedResultEx<P0, P1>(hfile: P0, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, dwmilliseconds: u32, balertable: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetOverlappedResultEx ( hfile : super::super::Foundation:: HANDLE , lpoverlapped : *const OVERLAPPED , lpnumberofbytestransferred : *mut u32 , dwmilliseconds : u32 , balertable : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    GetOverlappedResultEx(hfile.into_param().abi(), lpoverlapped, lpnumberofbytestransferred, dwmilliseconds, balertable.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetQueuedCompletionStatus<P0>(completionport: P0, lpnumberofbytestransferred: *mut u32, lpcompletionkey: *mut usize, lpoverlapped: *mut *mut OVERLAPPED, dwmilliseconds: u32) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetQueuedCompletionStatus ( completionport : super::super::Foundation:: HANDLE , lpnumberofbytestransferred : *mut u32 , lpcompletionkey : *mut usize , lpoverlapped : *mut *mut OVERLAPPED , dwmilliseconds : u32 ) -> super::super::Foundation:: BOOL );
    GetQueuedCompletionStatus(completionport.into_param().abi(), lpnumberofbytestransferred, lpcompletionkey, lpoverlapped, dwmilliseconds)
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetQueuedCompletionStatusEx<P0, P1>(completionport: P0, lpcompletionportentries: &mut [OVERLAPPED_ENTRY], ulnumentriesremoved: *mut u32, dwmilliseconds: u32, falertable: P1) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn GetQueuedCompletionStatusEx ( completionport : super::super::Foundation:: HANDLE , lpcompletionportentries : *mut OVERLAPPED_ENTRY , ulcount : u32 , ulnumentriesremoved : *mut u32 , dwmilliseconds : u32 , falertable : super::super::Foundation:: BOOL ) -> super::super::Foundation:: BOOL );
    GetQueuedCompletionStatusEx(completionport.into_param().abi(), ::core::mem::transmute(lpcompletionportentries.as_ptr()), lpcompletionportentries.len() as _, ulnumentriesremoved, dwmilliseconds, falertable.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PostQueuedCompletionStatus<P0>(completionport: P0, dwnumberofbytestransferred: u32, dwcompletionkey: usize, lpoverlapped: ::core::option::Option<*const OVERLAPPED>) -> super::super::Foundation::BOOL
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "kernel32.dll""system" fn PostQueuedCompletionStatus ( completionport : super::super::Foundation:: HANDLE , dwnumberofbytestransferred : u32 , dwcompletionkey : usize , lpoverlapped : *const OVERLAPPED ) -> super::super::Foundation:: BOOL );
    PostQueuedCompletionStatus(completionport.into_param().abi(), dwnumberofbytestransferred, dwcompletionkey, ::core::mem::transmute(lpoverlapped.unwrap_or(::std::ptr::null())))
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
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
impl ::windows::core::TypeKind for OVERLAPPED {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OVERLAPPED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
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
impl ::windows::core::TypeKind for OVERLAPPED_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OVERLAPPED_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
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
impl ::windows::core::TypeKind for OVERLAPPED_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OVERLAPPED_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.OffsetHigh == other.OffsetHigh
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
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
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
impl ::windows::core::TypeKind for OVERLAPPED_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OVERLAPPED_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.lpCompletionKey == other.lpCompletionKey && self.lpOverlapped == other.lpOverlapped && self.Internal == other.Internal && self.dwNumberOfBytesTransferred == other.dwNumberOfBytesTransferred
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
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPOVERLAPPED_COMPLETION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(dwerrorcode: u32, dwnumberofbytestransfered: u32, lpoverlapped: *mut OVERLAPPED) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
