#[cfg(all(feature = "Win32_guiddef", feature = "Win32_ws2def"))]
#[inline]
pub unsafe fn FreeAddrInfoEx(paddrinfoex: Option<*const super::ws2def::ADDRINFOEXA>) {
    windows_core::link!("ws2_32.dll" "system" fn FreeAddrInfoEx(paddrinfoex : *const super::ws2def::ADDRINFOEXA));
    unsafe { FreeAddrInfoEx(paddrinfoex.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_guiddef", feature = "Win32_ws2def"))]
#[inline]
pub unsafe fn FreeAddrInfoExW(paddrinfoex: Option<*const super::ws2def::ADDRINFOEXW>) {
    windows_core::link!("ws2_32.dll" "system" fn FreeAddrInfoExW(paddrinfoex : *const super::ws2def::ADDRINFOEXW));
    unsafe { FreeAddrInfoExW(paddrinfoex.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_ws2def")]
#[inline]
pub unsafe fn FreeAddrInfoW(paddrinfo: Option<*const super::ws2def::ADDRINFOW>) {
    windows_core::link!("ws2_32.dll" "system" fn FreeAddrInfoW(paddrinfo : *const super::ws2def::ADDRINFOW));
    unsafe { FreeAddrInfoW(paddrinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_guiddef", feature = "Win32_minwinbase", feature = "Win32_winnt", feature = "Win32_winsock2", feature = "Win32_ws2def"))]
#[inline]
pub unsafe fn GetAddrInfoExA<P0, P1>(pname: P0, pservicename: P1, dwnamespace: u32, lpnspid: Option<*const windows_core::GUID>, hints: Option<*const super::ws2def::ADDRINFOEXA>, ppresult: *mut super::ws2def::PADDRINFOEXA, timeout: Option<*const super::winsock2::timeval>, lpoverlapped: Option<*const super::minwinbase::OVERLAPPED>, lpcompletionroutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle: Option<*mut super::winnt::HANDLE>) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn GetAddrInfoExA(pname : windows_core::PCSTR, pservicename : windows_core::PCSTR, dwnamespace : u32, lpnspid : *const windows_core::GUID, hints : *const super::ws2def::ADDRINFOEXA, ppresult : *mut super::ws2def::PADDRINFOEXA, timeout : *const super::winsock2::timeval, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpcompletionroutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle : *mut super::winnt::HANDLE) -> i32);
    unsafe { GetAddrInfoExA(pname.param().abi(), pservicename.param().abi(), dwnamespace, lpnspid.unwrap_or(core::mem::zeroed()) as _, hints.unwrap_or(core::mem::zeroed()) as _, ppresult as _, timeout.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine, lpnamehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetAddrInfoExCancel(lphandle: *const super::winnt::HANDLE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn GetAddrInfoExCancel(lphandle : *const super::winnt::HANDLE) -> i32);
    unsafe { GetAddrInfoExCancel(lphandle) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn GetAddrInfoExOverlappedResult(lpoverlapped: *const super::minwinbase::OVERLAPPED) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn GetAddrInfoExOverlappedResult(lpoverlapped : *const super::minwinbase::OVERLAPPED) -> i32);
    unsafe { GetAddrInfoExOverlappedResult(lpoverlapped) }
}
#[cfg(all(feature = "Win32_guiddef", feature = "Win32_minwinbase", feature = "Win32_winnt", feature = "Win32_winsock2", feature = "Win32_ws2def"))]
#[inline]
pub unsafe fn GetAddrInfoExW<P0, P1>(pname: P0, pservicename: P1, dwnamespace: u32, lpnspid: Option<*const windows_core::GUID>, hints: Option<*const super::ws2def::ADDRINFOEXW>, ppresult: *mut super::ws2def::PADDRINFOEXW, timeout: Option<*const super::winsock2::timeval>, lpoverlapped: Option<*const super::minwinbase::OVERLAPPED>, lpcompletionroutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE, lphandle: Option<*mut super::winnt::HANDLE>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn GetAddrInfoExW(pname : windows_core::PCWSTR, pservicename : windows_core::PCWSTR, dwnamespace : u32, lpnspid : *const windows_core::GUID, hints : *const super::ws2def::ADDRINFOEXW, ppresult : *mut super::ws2def::PADDRINFOEXW, timeout : *const super::winsock2::timeval, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpcompletionroutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE, lphandle : *mut super::winnt::HANDLE) -> i32);
    unsafe { GetAddrInfoExW(pname.param().abi(), pservicename.param().abi(), dwnamespace, lpnspid.unwrap_or(core::mem::zeroed()) as _, hints.unwrap_or(core::mem::zeroed()) as _, ppresult as _, timeout.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine, lphandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_ws2def")]
#[inline]
pub unsafe fn GetAddrInfoW<P0, P1>(pnodename: P0, pservicename: P1, phints: Option<*const super::ws2def::ADDRINFOW>, ppresult: *mut super::ws2def::PADDRINFOW) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn GetAddrInfoW(pnodename : windows_core::PCWSTR, pservicename : windows_core::PCWSTR, phints : *const super::ws2def::ADDRINFOW, ppresult : *mut super::ws2def::PADDRINFOW) -> i32);
    unsafe { GetAddrInfoW(pnodename.param().abi(), pservicename.param().abi(), phints.unwrap_or(core::mem::zeroed()) as _, ppresult as _) }
}
#[cfg(feature = "Win32_ws2def")]
#[inline]
pub unsafe fn GetNameInfoW(psockaddr: *const super::ws2def::SOCKADDR, sockaddrlength: socklen_t, pnodebuffer: Option<&mut [u16]>, pservicebuffer: Option<&mut [u16]>, flags: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn GetNameInfoW(psockaddr : *const super::ws2def::SOCKADDR, sockaddrlength : socklen_t, pnodebuffer : *mut u16, nodebuffersize : u32, pservicebuffer : *mut u16, servicebuffersize : u32, flags : i32) -> i32);
    unsafe { GetNameInfoW(psockaddr, sockaddrlength, core::mem::transmute(pnodebuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pnodebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pservicebuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pservicebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), flags) }
}
#[inline]
pub unsafe fn InetNtopW(family: i32, paddr: *const core::ffi::c_void, pstringbuf: &mut [u16]) -> windows_core::PCWSTR {
    windows_core::link!("ws2_32.dll" "system" fn InetNtopW(family : i32, paddr : *const core::ffi::c_void, pstringbuf : windows_core::PWSTR, stringbufsize : usize) -> windows_core::PCWSTR);
    unsafe { InetNtopW(family, paddr, core::mem::transmute(pstringbuf.as_ptr()), pstringbuf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn InetPtonW<P1>(family: i32, pszaddrstring: P1, paddrbuf: *mut core::ffi::c_void) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn InetPtonW(family : i32, pszaddrstring : windows_core::PCWSTR, paddrbuf : *mut core::ffi::c_void) -> i32);
    unsafe { InetPtonW(family, pszaddrstring.param().abi(), paddrbuf as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt", feature = "Win32_winsock2", feature = "Win32_ws2def", feature = "Win32_wtypesbase"))]
#[inline]
pub unsafe fn SetAddrInfoExA<P0, P1>(pname: P0, pservicename: P1, paddresses: Option<*const super::ws2def::SOCKET_ADDRESS>, dwaddresscount: u32, lpblob: Option<*const super::wtypesbase::BLOB>, dwflags: u32, dwnamespace: u32, lpnspid: Option<*const windows_core::GUID>, timeout: Option<*const super::winsock2::timeval>, lpoverlapped: Option<*const super::minwinbase::OVERLAPPED>, lpcompletionroutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle: Option<*mut super::winnt::HANDLE>) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn SetAddrInfoExA(pname : windows_core::PCSTR, pservicename : windows_core::PCSTR, paddresses : *const super::ws2def::SOCKET_ADDRESS, dwaddresscount : u32, lpblob : *const super::wtypesbase::BLOB, dwflags : u32, dwnamespace : u32, lpnspid : *const windows_core::GUID, timeout : *const super::winsock2::timeval, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpcompletionroutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle : *mut super::winnt::HANDLE) -> i32);
    unsafe { SetAddrInfoExA(pname.param().abi(), pservicename.param().abi(), paddresses.unwrap_or(core::mem::zeroed()) as _, dwaddresscount, lpblob.unwrap_or(core::mem::zeroed()) as _, dwflags, dwnamespace, lpnspid.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine, lpnamehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt", feature = "Win32_winsock2", feature = "Win32_ws2def", feature = "Win32_wtypesbase"))]
#[inline]
pub unsafe fn SetAddrInfoExW<P0, P1>(pname: P0, pservicename: P1, paddresses: Option<*const super::ws2def::SOCKET_ADDRESS>, dwaddresscount: u32, lpblob: Option<*const super::wtypesbase::BLOB>, dwflags: u32, dwnamespace: u32, lpnspid: Option<*const windows_core::GUID>, timeout: Option<*const super::winsock2::timeval>, lpoverlapped: Option<*const super::minwinbase::OVERLAPPED>, lpcompletionroutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle: Option<*mut super::winnt::HANDLE>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn SetAddrInfoExW(pname : windows_core::PCWSTR, pservicename : windows_core::PCWSTR, paddresses : *const super::ws2def::SOCKET_ADDRESS, dwaddresscount : u32, lpblob : *const super::wtypesbase::BLOB, dwflags : u32, dwnamespace : u32, lpnspid : *const windows_core::GUID, timeout : *const super::winsock2::timeval, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpcompletionroutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle : *mut super::winnt::HANDLE) -> i32);
    unsafe { SetAddrInfoExW(pname.param().abi(), pservicename.param().abi(), paddresses.unwrap_or(core::mem::zeroed()) as _, dwaddresscount, lpblob.unwrap_or(core::mem::zeroed()) as _, dwflags, dwnamespace, lpnspid.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine, lpnamehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_ws2def")]
#[inline]
pub unsafe fn freeaddrinfo(paddrinfo: Option<*const super::ws2def::ADDRINFOA>) {
    windows_core::link!("ws2_32.dll" "system" fn freeaddrinfo(paddrinfo : *const super::ws2def::ADDRINFOA));
    unsafe { freeaddrinfo(paddrinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_ws2def")]
#[inline]
pub unsafe fn getaddrinfo<P0, P1>(pnodename: P0, pservicename: P1, phints: Option<*const super::ws2def::ADDRINFOA>, ppresult: *mut super::ws2def::PADDRINFOA) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn getaddrinfo(pnodename : windows_core::PCSTR, pservicename : windows_core::PCSTR, phints : *const super::ws2def::ADDRINFOA, ppresult : *mut super::ws2def::PADDRINFOA) -> i32);
    unsafe { getaddrinfo(pnodename.param().abi(), pservicename.param().abi(), phints.unwrap_or(core::mem::zeroed()) as _, ppresult as _) }
}
#[cfg(feature = "Win32_ws2def")]
#[inline]
pub unsafe fn getnameinfo(psockaddr: *const super::ws2def::SOCKADDR, sockaddrlength: socklen_t, pnodebuffer: Option<&mut [i8]>, pservicebuffer: Option<&mut [i8]>, flags: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn getnameinfo(psockaddr : *const super::ws2def::SOCKADDR, sockaddrlength : socklen_t, pnodebuffer : *mut i8, nodebuffersize : u32, pservicebuffer : *mut i8, servicebuffersize : u32, flags : i32) -> i32);
    unsafe { getnameinfo(psockaddr, sockaddrlength, core::mem::transmute(pnodebuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pnodebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pservicebuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pservicebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), flags) }
}
#[inline]
pub unsafe fn inet_ntop(family: i32, paddr: *const core::ffi::c_void, pstringbuf: &mut [u8]) -> windows_core::PCSTR {
    windows_core::link!("ws2_32.dll" "system" fn inet_ntop(family : i32, paddr : *const core::ffi::c_void, pstringbuf : windows_core::PSTR, stringbufsize : usize) -> windows_core::PCSTR);
    unsafe { inet_ntop(family, paddr, core::mem::transmute(pstringbuf.as_ptr()), pstringbuf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn inet_pton<P1>(family: i32, pszaddrstring: P1, paddrbuf: *mut core::ffi::c_void) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn inet_pton(family : i32, pszaddrstring : windows_core::PCSTR, paddrbuf : *mut core::ffi::c_void) -> i32);
    unsafe { inet_pton(family, pszaddrstring.param().abi(), paddrbuf as _) }
}
#[cfg(feature = "Win32_ws2def")]
pub type ADDRINFO = super::ws2def::ADDRINFOA;
#[cfg(all(feature = "Win32_guiddef", feature = "Win32_ws2def"))]
pub type ADDRINFOEX = super::ws2def::ADDRINFOEXA;
#[cfg(feature = "Win32_ws2def")]
pub type ADDRINFOT = super::ws2def::ADDRINFOA;
pub const EAI_AGAIN: u32 = 11002;
pub const EAI_BADFLAGS: u32 = 10022;
pub const EAI_FAIL: u32 = 11003;
pub const EAI_FAMILY: u32 = 10047;
pub const EAI_IPSECPOLICY: u32 = 11033;
pub const EAI_MEMORY: u32 = 8;
pub const EAI_NODATA: u32 = 11001;
pub const EAI_NONAME: u32 = 11001;
pub const EAI_NOSECURENAME: u32 = 11032;
pub const EAI_SERVICE: u32 = 10109;
pub const EAI_SOCKTYPE: u32 = 10044;
pub const GAI_STRERROR_BUFFER_SIZE: u32 = 1024;
#[cfg(feature = "Win32_ws2def")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPADDRINFO(pub *mut super::ws2def::ADDRINFOA);
#[cfg(feature = "Win32_ws2def")]
impl LPADDRINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_ws2def")]
impl Default for LPADDRINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
pub type LPLOOKUPSERVICE_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(dwerror: u32, dwbytes: u32, lpoverlapped: *const super::minwinbase::OVERLAPPED)>;
#[cfg(all(feature = "Win32_guiddef", feature = "Win32_ws2def"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADDRINFOEX(pub *mut super::ws2def::ADDRINFOEXA);
#[cfg(all(feature = "Win32_guiddef", feature = "Win32_ws2def"))]
impl PADDRINFOEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_guiddef", feature = "Win32_ws2def"))]
impl Default for PADDRINFOEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_ws2def")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADDRINFOT(pub *mut super::ws2def::ADDRINFOA);
#[cfg(feature = "Win32_ws2def")]
impl PADDRINFOT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_ws2def")]
impl Default for PADDRINFOT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UDP_CHECKSUM_COVERAGE: u32 = 20;
pub const UDP_NOCHECKSUM: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct socklen_t(pub i32);
