#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BindIoCompletionCallback<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, function: LPOVERLAPPED_COMPLETION_ROUTINE, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BindIoCompletionCallback(filehandle: super::super::Foundation::HANDLE, function: ::windows::core::RawPtr, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(BindIoCompletionCallback(filehandle.into_param().abi(), ::core::mem::transmute(function), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelIo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelIo(hfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CancelIo(hfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelIoEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, lpoverlapped: *const OVERLAPPED) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelIoEx(hfile: super::super::Foundation::HANDLE, lpoverlapped: *const OVERLAPPED) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CancelIoEx(hfile.into_param().abi(), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelSynchronousIo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelSynchronousIo(hthread: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CancelSynchronousIo(hthread.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateIoCompletionPort<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(filehandle: Param0, existingcompletionport: Param1, completionkey: usize, numberofconcurrentthreads: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateIoCompletionPort(filehandle: super::super::Foundation::HANDLE, existingcompletionport: super::super::Foundation::HANDLE, completionkey: usize, numberofconcurrentthreads: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateIoCompletionPort(filehandle.into_param().abi(), existingcompletionport.into_param().abi(), ::core::mem::transmute(completionkey), ::core::mem::transmute(numberofconcurrentthreads)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeviceIoControl<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hdevice: Param0, dwiocontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut OVERLAPPED) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeviceIoControl(hdevice: super::super::Foundation::HANDLE, dwiocontrolcode: u32, lpinbuffer: *const ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut OVERLAPPED) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeviceIoControl(hdevice.into_param().abi(), ::core::mem::transmute(dwiocontrolcode), ::core::mem::transmute(lpinbuffer), ::core::mem::transmute(ninbuffersize), ::core::mem::transmute(lpoutbuffer), ::core::mem::transmute(noutbuffersize), ::core::mem::transmute(lpbytesreturned), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOverlappedResult<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hfile: Param0, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOverlappedResult(hfile: super::super::Foundation::HANDLE, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, bwait: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetOverlappedResult(hfile.into_param().abi(), ::core::mem::transmute(lpoverlapped), ::core::mem::transmute(lpnumberofbytestransferred), bwait.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOverlappedResultEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hfile: Param0, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, dwmilliseconds: u32, balertable: Param4) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOverlappedResultEx(hfile: super::super::Foundation::HANDLE, lpoverlapped: *const OVERLAPPED, lpnumberofbytestransferred: *mut u32, dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetOverlappedResultEx(hfile.into_param().abi(), ::core::mem::transmute(lpoverlapped), ::core::mem::transmute(lpnumberofbytestransferred), ::core::mem::transmute(dwmilliseconds), balertable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetQueuedCompletionStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(completionport: Param0, lpnumberofbytestransferred: *mut u32, lpcompletionkey: *mut usize, lpoverlapped: *mut *mut OVERLAPPED, dwmilliseconds: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetQueuedCompletionStatus(completionport: super::super::Foundation::HANDLE, lpnumberofbytestransferred: *mut u32, lpcompletionkey: *mut usize, lpoverlapped: *mut *mut OVERLAPPED, dwmilliseconds: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetQueuedCompletionStatus(completionport.into_param().abi(), ::core::mem::transmute(lpnumberofbytestransferred), ::core::mem::transmute(lpcompletionkey), ::core::mem::transmute(lpoverlapped), ::core::mem::transmute(dwmilliseconds)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetQueuedCompletionStatusEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(completionport: Param0, lpcompletionportentries: &mut [OVERLAPPED_ENTRY], ulnumentriesremoved: *mut u32, dwmilliseconds: u32, falertable: Param5) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetQueuedCompletionStatusEx(completionport: super::super::Foundation::HANDLE, lpcompletionportentries: *mut OVERLAPPED_ENTRY, ulcount: u32, ulnumentriesremoved: *mut u32, dwmilliseconds: u32, falertable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetQueuedCompletionStatusEx(completionport.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpcompletionportentries)), lpcompletionportentries.len() as _, ::core::mem::transmute(ulnumentriesremoved), ::core::mem::transmute(dwmilliseconds), falertable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPOVERLAPPED_COMPLETION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(dwerrorcode: u32, dwnumberofbytestransfered: u32, lpoverlapped: *mut OVERLAPPED)>;
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
#[doc = "*Required features: `\"Win32_System_IO\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PostQueuedCompletionStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(completionport: Param0, dwnumberofbytestransferred: u32, dwcompletionkey: usize, lpoverlapped: *const OVERLAPPED) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PostQueuedCompletionStatus(completionport: super::super::Foundation::HANDLE, dwnumberofbytestransferred: u32, dwcompletionkey: usize, lpoverlapped: *const OVERLAPPED) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PostQueuedCompletionStatus(completionport.into_param().abi(), ::core::mem::transmute(dwnumberofbytestransferred), ::core::mem::transmute(dwcompletionkey), ::core::mem::transmute(lpoverlapped)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
