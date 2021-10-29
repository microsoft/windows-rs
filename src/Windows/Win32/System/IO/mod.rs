#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelIo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelIo(hfile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CancelIo(hfile.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelIoEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hfile: Param0,
    lpoverlapped: *const OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelIoEx(
                hfile: super::super::Foundation::HANDLE,
                lpoverlapped: *const OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CancelIoEx(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpoverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelSynchronousIo<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hthread: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelSynchronousIo(
                hthread: super::super::Foundation::HANDLE,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CancelSynchronousIo(hthread.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateIoCompletionPort<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    filehandle: Param0,
    existingcompletionport: Param1,
    completionkey: usize,
    numberofconcurrentthreads: u32,
) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateIoCompletionPort(
                filehandle: super::super::Foundation::HANDLE,
                existingcompletionport: super::super::Foundation::HANDLE,
                completionkey: usize,
                numberofconcurrentthreads: u32,
            ) -> super::super::Foundation::HANDLE;
        }
        ::std::mem::transmute(CreateIoCompletionPort(
            filehandle.into_param().abi(),
            existingcompletionport.into_param().abi(),
            ::std::mem::transmute(completionkey),
            ::std::mem::transmute(numberofconcurrentthreads),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeviceIoControl<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hdevice: Param0,
    dwiocontrolcode: u32,
    lpinbuffer: *const ::std::ffi::c_void,
    ninbuffersize: u32,
    lpoutbuffer: *mut ::std::ffi::c_void,
    noutbuffersize: u32,
    lpbytesreturned: *mut u32,
    lpoverlapped: *mut OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeviceIoControl(
                hdevice: super::super::Foundation::HANDLE,
                dwiocontrolcode: u32,
                lpinbuffer: *const ::std::ffi::c_void,
                ninbuffersize: u32,
                lpoutbuffer: *mut ::std::ffi::c_void,
                noutbuffersize: u32,
                lpbytesreturned: *mut u32,
                lpoverlapped: *mut OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeviceIoControl(
            hdevice.into_param().abi(),
            ::std::mem::transmute(dwiocontrolcode),
            ::std::mem::transmute(lpinbuffer),
            ::std::mem::transmute(ninbuffersize),
            ::std::mem::transmute(lpoutbuffer),
            ::std::mem::transmute(noutbuffersize),
            ::std::mem::transmute(lpbytesreturned),
            ::std::mem::transmute(lpoverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOverlappedResult<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hfile: Param0,
    lpoverlapped: *const OVERLAPPED,
    lpnumberofbytestransferred: *mut u32,
    bwait: Param3,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOverlappedResult(
                hfile: super::super::Foundation::HANDLE,
                lpoverlapped: *const OVERLAPPED,
                lpnumberofbytestransferred: *mut u32,
                bwait: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetOverlappedResult(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpoverlapped),
            ::std::mem::transmute(lpnumberofbytestransferred),
            bwait.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOverlappedResultEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hfile: Param0,
    lpoverlapped: *const OVERLAPPED,
    lpnumberofbytestransferred: *mut u32,
    dwmilliseconds: u32,
    balertable: Param4,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetOverlappedResultEx(
                hfile: super::super::Foundation::HANDLE,
                lpoverlapped: *const OVERLAPPED,
                lpnumberofbytestransferred: *mut u32,
                dwmilliseconds: u32,
                balertable: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetOverlappedResultEx(
            hfile.into_param().abi(),
            ::std::mem::transmute(lpoverlapped),
            ::std::mem::transmute(lpnumberofbytestransferred),
            ::std::mem::transmute(dwmilliseconds),
            balertable.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetQueuedCompletionStatus<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    completionport: Param0,
    lpnumberofbytestransferred: *mut u32,
    lpcompletionkey: *mut usize,
    lpoverlapped: *mut *mut OVERLAPPED,
    dwmilliseconds: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetQueuedCompletionStatus(
                completionport: super::super::Foundation::HANDLE,
                lpnumberofbytestransferred: *mut u32,
                lpcompletionkey: *mut usize,
                lpoverlapped: *mut *mut OVERLAPPED,
                dwmilliseconds: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetQueuedCompletionStatus(
            completionport.into_param().abi(),
            ::std::mem::transmute(lpnumberofbytestransferred),
            ::std::mem::transmute(lpcompletionkey),
            ::std::mem::transmute(lpoverlapped),
            ::std::mem::transmute(dwmilliseconds),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetQueuedCompletionStatusEx<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    completionport: Param0,
    lpcompletionportentries: *mut OVERLAPPED_ENTRY,
    ulcount: u32,
    ulnumentriesremoved: *mut u32,
    dwmilliseconds: u32,
    falertable: Param5,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetQueuedCompletionStatusEx(
                completionport: super::super::Foundation::HANDLE,
                lpcompletionportentries: *mut OVERLAPPED_ENTRY,
                ulcount: u32,
                ulnumentriesremoved: *mut u32,
                dwmilliseconds: u32,
                falertable: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetQueuedCompletionStatusEx(
            completionport.into_param().abi(),
            ::std::mem::transmute(lpcompletionportentries),
            ::std::mem::transmute(ulcount),
            ::std::mem::transmute(ulnumentriesremoved),
            ::std::mem::transmute(dwmilliseconds),
            falertable.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub type LPOVERLAPPED_COMPLETION_ROUTINE = unsafe extern "system" fn(
    dwerrorcode: u32,
    dwnumberofbytestransfered: u32,
    lpoverlapped: *mut OVERLAPPED,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OVERLAPPED {
    pub Internal: usize,
    pub InternalHigh: usize,
    pub Anonymous: OVERLAPPED_0,
    pub hEvent: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl OVERLAPPED {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OVERLAPPED {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OVERLAPPED {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OVERLAPPED {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OVERLAPPED {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union OVERLAPPED_0 {
    pub Anonymous: OVERLAPPED_0_0,
    pub Pointer: *mut ::std::ffi::c_void,
}
impl OVERLAPPED_0 {}
impl ::std::default::Default for OVERLAPPED_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for OVERLAPPED_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for OVERLAPPED_0 {}
unsafe impl ::windows::runtime::Abi for OVERLAPPED_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct OVERLAPPED_0_0 {
    pub Offset: u32,
    pub OffsetHigh: u32,
}
impl OVERLAPPED_0_0 {}
impl ::std::default::Default for OVERLAPPED_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OVERLAPPED_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("Offset", &self.Offset)
            .field("OffsetHigh", &self.OffsetHigh)
            .finish()
    }
}
impl ::std::cmp::PartialEq for OVERLAPPED_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.OffsetHigh == other.OffsetHigh
    }
}
impl ::std::cmp::Eq for OVERLAPPED_0_0 {}
unsafe impl ::windows::runtime::Abi for OVERLAPPED_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OVERLAPPED_ENTRY {
    pub lpCompletionKey: usize,
    pub lpOverlapped: *mut OVERLAPPED,
    pub Internal: usize,
    pub dwNumberOfBytesTransferred: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl OVERLAPPED_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OVERLAPPED_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OVERLAPPED_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("OVERLAPPED_ENTRY")
            .field("lpCompletionKey", &self.lpCompletionKey)
            .field("lpOverlapped", &self.lpOverlapped)
            .field("Internal", &self.Internal)
            .field(
                "dwNumberOfBytesTransferred",
                &self.dwNumberOfBytesTransferred,
            )
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OVERLAPPED_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.lpCompletionKey == other.lpCompletionKey
            && self.lpOverlapped == other.lpOverlapped
            && self.Internal == other.Internal
            && self.dwNumberOfBytesTransferred == other.dwNumberOfBytesTransferred
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OVERLAPPED_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OVERLAPPED_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PostQueuedCompletionStatus<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    completionport: Param0,
    dwnumberofbytestransferred: u32,
    dwcompletionkey: usize,
    lpoverlapped: *const OVERLAPPED,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PostQueuedCompletionStatus(
                completionport: super::super::Foundation::HANDLE,
                dwnumberofbytestransferred: u32,
                dwcompletionkey: usize,
                lpoverlapped: *const OVERLAPPED,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(PostQueuedCompletionStatus(
            completionport.into_param().abi(),
            ::std::mem::transmute(dwnumberofbytestransferred),
            ::std::mem::transmute(dwcompletionkey),
            ::std::mem::transmute(lpoverlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
