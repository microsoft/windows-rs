#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn FreeAddrInfoEx(paddrinfoex: Option<*const ADDRINFOEXA>) {
    windows_core::link!("ws2_32.dll" "system" fn FreeAddrInfoEx(paddrinfoex : *const ADDRINFOEXA));
    unsafe { FreeAddrInfoEx(paddrinfoex.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn FreeAddrInfoExW(paddrinfoex: Option<*const ADDRINFOEXW>) {
    windows_core::link!("ws2_32.dll" "system" fn FreeAddrInfoExW(paddrinfoex : *const ADDRINFOEXW));
    unsafe { FreeAddrInfoExW(paddrinfoex.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FreeAddrInfoW(paddrinfo: Option<*const ADDRINFOW>) {
    windows_core::link!("ws2_32.dll" "system" fn FreeAddrInfoW(paddrinfo : *const ADDRINFOW));
    unsafe { FreeAddrInfoW(paddrinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "winnt", feature = "winsock2"))]
#[inline]
pub unsafe fn GetAddrInfoExA<P0, P1>(pname: P0, pservicename: P1, dwnamespace: u32, lpnspid: Option<*const windows_core::GUID>, hints: Option<*const ADDRINFOEXA>, ppresult: *mut PADDRINFOEXA, timeout: Option<*const super::winsock2::timeval>, lpoverlapped: Option<*const super::minwinbase::OVERLAPPED>, lpcompletionroutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle: Option<*mut super::winnt::HANDLE>) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn GetAddrInfoExA(pname : windows_core::PCSTR, pservicename : windows_core::PCSTR, dwnamespace : u32, lpnspid : *const windows_core::GUID, hints : *const ADDRINFOEXA, ppresult : *mut PADDRINFOEXA, timeout : *const super::winsock2::timeval, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpcompletionroutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle : *mut super::winnt::HANDLE) -> i32);
    unsafe { GetAddrInfoExA(pname.param().abi(), pservicename.param().abi(), dwnamespace, lpnspid.unwrap_or(core::mem::zeroed()) as _, hints.unwrap_or(core::mem::zeroed()) as _, ppresult as _, timeout.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine, lpnamehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetAddrInfoExCancel(lphandle: *const super::winnt::HANDLE) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn GetAddrInfoExCancel(lphandle : *const super::winnt::HANDLE) -> i32);
    unsafe { GetAddrInfoExCancel(lphandle) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn GetAddrInfoExOverlappedResult(lpoverlapped: *const super::minwinbase::OVERLAPPED) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn GetAddrInfoExOverlappedResult(lpoverlapped : *const super::minwinbase::OVERLAPPED) -> i32);
    unsafe { GetAddrInfoExOverlappedResult(lpoverlapped) }
}
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "winnt", feature = "winsock2"))]
#[inline]
pub unsafe fn GetAddrInfoExW<P0, P1>(pname: P0, pservicename: P1, dwnamespace: u32, lpnspid: Option<*const windows_core::GUID>, hints: Option<*const ADDRINFOEXW>, ppresult: *mut PADDRINFOEXW, timeout: Option<*const super::winsock2::timeval>, lpoverlapped: Option<*const super::minwinbase::OVERLAPPED>, lpcompletionroutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE, lphandle: Option<*mut super::winnt::HANDLE>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn GetAddrInfoExW(pname : windows_core::PCWSTR, pservicename : windows_core::PCWSTR, dwnamespace : u32, lpnspid : *const windows_core::GUID, hints : *const ADDRINFOEXW, ppresult : *mut PADDRINFOEXW, timeout : *const super::winsock2::timeval, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpcompletionroutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE, lphandle : *mut super::winnt::HANDLE) -> i32);
    unsafe { GetAddrInfoExW(pname.param().abi(), pservicename.param().abi(), dwnamespace, lpnspid.unwrap_or(core::mem::zeroed()) as _, hints.unwrap_or(core::mem::zeroed()) as _, ppresult as _, timeout.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine, lphandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetAddrInfoW<P0, P1>(pnodename: P0, pservicename: P1, phints: Option<*const ADDRINFOW>, ppresult: *mut PADDRINFOW) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn GetAddrInfoW(pnodename : windows_core::PCWSTR, pservicename : windows_core::PCWSTR, phints : *const ADDRINFOW, ppresult : *mut PADDRINFOW) -> i32);
    unsafe { GetAddrInfoW(pnodename.param().abi(), pservicename.param().abi(), phints.unwrap_or(core::mem::zeroed()) as _, ppresult as _) }
}
#[inline]
pub unsafe fn GetNameInfoW(psockaddr: *const SOCKADDR, sockaddrlength: socklen_t, pnodebuffer: Option<&mut [u16]>, pservicebuffer: Option<&mut [u16]>, flags: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn GetNameInfoW(psockaddr : *const SOCKADDR, sockaddrlength : socklen_t, pnodebuffer : *mut u16, nodebuffersize : u32, pservicebuffer : *mut u16, servicebuffersize : u32, flags : i32) -> i32);
    unsafe { GetNameInfoW(psockaddr, sockaddrlength, pnodebuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), pnodebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pservicebuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), pservicebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), flags) }
}
#[inline]
pub unsafe fn InetNtopW(family: i32, paddr: *const core::ffi::c_void, pstringbuf: &mut [u16]) -> windows_core::PCWSTR {
    windows_core::link!("ws2_32.dll" "system" fn InetNtopW(family : i32, paddr : *const core::ffi::c_void, pstringbuf : windows_core::PWSTR, stringbufsize : usize) -> windows_core::PCWSTR);
    unsafe { InetNtopW(family, paddr, core::mem::transmute(pstringbuf.as_mut_ptr()), pstringbuf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn InetPtonW<P1>(family: i32, pszaddrstring: P1, paddrbuf: *mut core::ffi::c_void) -> i32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn InetPtonW(family : i32, pszaddrstring : windows_core::PCWSTR, paddrbuf : *mut core::ffi::c_void) -> i32);
    unsafe { InetPtonW(family, pszaddrstring.param().abi(), paddrbuf as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "winsock2", feature = "wtypesbase"))]
#[inline]
pub unsafe fn SetAddrInfoExA<P0, P1>(pname: P0, pservicename: P1, paddresses: Option<*const SOCKET_ADDRESS>, dwaddresscount: u32, lpblob: Option<*const super::wtypesbase::BLOB>, dwflags: u32, dwnamespace: u32, lpnspid: Option<*const windows_core::GUID>, timeout: Option<*const super::winsock2::timeval>, lpoverlapped: Option<*const super::minwinbase::OVERLAPPED>, lpcompletionroutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle: Option<*mut super::winnt::HANDLE>) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn SetAddrInfoExA(pname : windows_core::PCSTR, pservicename : windows_core::PCSTR, paddresses : *const SOCKET_ADDRESS, dwaddresscount : u32, lpblob : *const super::wtypesbase::BLOB, dwflags : u32, dwnamespace : u32, lpnspid : *const windows_core::GUID, timeout : *const super::winsock2::timeval, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpcompletionroutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle : *mut super::winnt::HANDLE) -> i32);
    unsafe { SetAddrInfoExA(pname.param().abi(), pservicename.param().abi(), paddresses.unwrap_or(core::mem::zeroed()) as _, dwaddresscount, lpblob.unwrap_or(core::mem::zeroed()) as _, dwflags, dwnamespace, lpnspid.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine, lpnamehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "winsock2", feature = "wtypesbase"))]
#[inline]
pub unsafe fn SetAddrInfoExW<P0, P1>(pname: P0, pservicename: P1, paddresses: Option<*const SOCKET_ADDRESS>, dwaddresscount: u32, lpblob: Option<*const super::wtypesbase::BLOB>, dwflags: u32, dwnamespace: u32, lpnspid: Option<*const windows_core::GUID>, timeout: Option<*const super::winsock2::timeval>, lpoverlapped: Option<*const super::minwinbase::OVERLAPPED>, lpcompletionroutine: LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle: Option<*mut super::winnt::HANDLE>) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn SetAddrInfoExW(pname : windows_core::PCWSTR, pservicename : windows_core::PCWSTR, paddresses : *const SOCKET_ADDRESS, dwaddresscount : u32, lpblob : *const super::wtypesbase::BLOB, dwflags : u32, dwnamespace : u32, lpnspid : *const windows_core::GUID, timeout : *const super::winsock2::timeval, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpcompletionroutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle : *mut super::winnt::HANDLE) -> i32);
    unsafe { SetAddrInfoExW(pname.param().abi(), pservicename.param().abi(), paddresses.unwrap_or(core::mem::zeroed()) as _, dwaddresscount, lpblob.unwrap_or(core::mem::zeroed()) as _, dwflags, dwnamespace, lpnspid.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine, lpnamehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn freeaddrinfo(paddrinfo: Option<*const ADDRINFOA>) {
    windows_core::link!("ws2_32.dll" "system" fn freeaddrinfo(paddrinfo : *const ADDRINFOA));
    unsafe { freeaddrinfo(paddrinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn getaddrinfo<P0, P1>(pnodename: P0, pservicename: P1, phints: Option<*const ADDRINFOA>, ppresult: *mut PADDRINFOA) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn getaddrinfo(pnodename : windows_core::PCSTR, pservicename : windows_core::PCSTR, phints : *const ADDRINFOA, ppresult : *mut PADDRINFOA) -> i32);
    unsafe { getaddrinfo(pnodename.param().abi(), pservicename.param().abi(), phints.unwrap_or(core::mem::zeroed()) as _, ppresult as _) }
}
#[inline]
pub unsafe fn getnameinfo(psockaddr: *const SOCKADDR, sockaddrlength: socklen_t, pnodebuffer: Option<&mut [i8]>, pservicebuffer: Option<&mut [i8]>, flags: i32) -> i32 {
    windows_core::link!("ws2_32.dll" "system" fn getnameinfo(psockaddr : *const SOCKADDR, sockaddrlength : socklen_t, pnodebuffer : *mut i8, nodebuffersize : u32, pservicebuffer : *mut i8, servicebuffersize : u32, flags : i32) -> i32);
    unsafe { getnameinfo(psockaddr, sockaddrlength, pnodebuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), pnodebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pservicebuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), pservicebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), flags) }
}
#[inline]
pub unsafe fn inet_ntop(family: i32, paddr: *const core::ffi::c_void, pstringbuf: &mut [u8]) -> windows_core::PCSTR {
    windows_core::link!("ws2_32.dll" "system" fn inet_ntop(family : i32, paddr : *const core::ffi::c_void, pstringbuf : windows_core::PSTR, stringbufsize : usize) -> windows_core::PCSTR);
    unsafe { inet_ntop(family, paddr, core::mem::transmute(pstringbuf.as_mut_ptr()), pstringbuf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn inet_pton<P1>(family: i32, pszaddrstring: P1, paddrbuf: *mut core::ffi::c_void) -> i32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ws2_32.dll" "system" fn inet_pton(family : i32, pszaddrstring : windows_core::PCSTR, paddrbuf : *mut core::ffi::c_void) -> i32);
    unsafe { inet_pton(family, pszaddrstring.param().abi(), paddrbuf as _) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ADDRESS_FAMILY(pub u16);
pub type ADDRINFO = ADDRINFOA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADDRINFOA {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: *mut i8,
    pub ai_addr: *mut SOCKADDR,
    pub ai_next: *mut Self,
}
impl Default for ADDRINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "guiddef")]
pub type ADDRINFOEX = ADDRINFOEXA;
#[repr(C)]
#[cfg(feature = "guiddef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADDRINFOEX2A {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: *mut i8,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: super::guiddef::LPGUID,
    pub ai_next: *mut Self,
    pub ai_version: i32,
    pub ai_fqdn: *mut i8,
}
#[cfg(feature = "guiddef")]
impl Default for ADDRINFOEX2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "guiddef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADDRINFOEX2W {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: windows_core::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: super::guiddef::LPGUID,
    pub ai_next: *mut Self,
    pub ai_version: i32,
    pub ai_fqdn: windows_core::PWSTR,
}
#[cfg(feature = "guiddef")]
impl Default for ADDRINFOEX2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "guiddef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADDRINFOEX3 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: windows_core::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: super::guiddef::LPGUID,
    pub ai_next: *mut Self,
    pub ai_version: i32,
    pub ai_fqdn: windows_core::PWSTR,
    pub ai_interfaceindex: i32,
}
#[cfg(feature = "guiddef")]
impl Default for ADDRINFOEX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADDRINFOEX4 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: windows_core::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut windows_core::GUID,
    pub ai_next: *mut Self,
    pub ai_version: i32,
    pub ai_fqdn: windows_core::PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for ADDRINFOEX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADDRINFOEX5 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: windows_core::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut windows_core::GUID,
    pub ai_next: *mut Self,
    pub ai_version: i32,
    pub ai_fqdn: windows_core::PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: super::winnt::HANDLE,
    pub ai_ttl: u32,
}
#[cfg(feature = "winnt")]
impl Default for ADDRINFOEX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADDRINFOEX6 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: windows_core::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut windows_core::GUID,
    pub ai_next: *mut ADDRINFOEX5,
    pub ai_version: i32,
    pub ai_fqdn: windows_core::PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: super::winnt::HANDLE,
    pub ai_ttl: u32,
    pub ai_numservers: u32,
    pub ai_servers: *mut ADDRINFO_DNS_SERVER,
    pub ai_responseflags: u64,
}
#[cfg(feature = "winnt")]
impl Default for ADDRINFOEX6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADDRINFOEX7 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: windows_core::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut windows_core::GUID,
    pub ai_next: *mut Self,
    pub ai_version: i32,
    pub ai_fqdn: windows_core::PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: super::winnt::HANDLE,
    pub ai_ttl: u32,
    pub ai_numservers: u32,
    pub ai_servers: *mut ADDRINFO_DNS_SERVER,
    pub ai_responseflags: u64,
    pub ai_extraflags: u64,
}
#[cfg(feature = "winnt")]
impl Default for ADDRINFOEX7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "guiddef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADDRINFOEXA {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: *mut i8,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: super::guiddef::LPGUID,
    pub ai_next: *mut Self,
}
#[cfg(feature = "guiddef")]
impl Default for ADDRINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "guiddef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADDRINFOEXW {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: windows_core::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: super::guiddef::LPGUID,
    pub ai_next: *mut Self,
}
#[cfg(feature = "guiddef")]
impl Default for ADDRINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ADDRINFOEX_VERSION_2: u32 = 2;
pub const ADDRINFOEX_VERSION_3: u32 = 3;
pub const ADDRINFOEX_VERSION_4: u32 = 4;
pub const ADDRINFOEX_VERSION_5: u32 = 5;
pub const ADDRINFOEX_VERSION_6: u32 = 6;
pub const ADDRINFOEX_VERSION_7: u32 = 7;
pub type ADDRINFOT = ADDRINFOA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ADDRINFOW {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: windows_core::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_next: *mut Self,
}
impl Default for ADDRINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ADDRINFO_DNS_SERVER {
    pub ai_servertype: u32,
    pub ai_flags: u64,
    pub ai_addrlen: u32,
    pub ai_addr: *mut SOCKADDR,
    pub Anonymous: ADDRINFO_DNS_SERVER_0,
}
impl Default for ADDRINFO_DNS_SERVER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ADDRINFO_DNS_SERVER_0 {
    pub ai_template: windows_core::PWSTR,
    pub ai_hostname: windows_core::PWSTR,
}
impl Default for ADDRINFO_DNS_SERVER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AF_12844: u32 = 25;
pub const AF_APPLETALK: u32 = 16;
pub const AF_ATM: u32 = 22;
pub const AF_BAN: u32 = 21;
pub const AF_BTH: u32 = 32;
pub const AF_CCITT: u32 = 10;
pub const AF_CHAOS: u32 = 5;
pub const AF_CLUSTER: u32 = 24;
pub const AF_DATAKIT: u32 = 9;
pub const AF_DECnet: u32 = 12;
pub const AF_DLI: u32 = 13;
pub const AF_ECMA: u32 = 8;
pub const AF_FIREFOX: u32 = 19;
pub const AF_HYLINK: u32 = 15;
pub const AF_HYPERV: u32 = 34;
pub const AF_ICLFXBM: u32 = 31;
pub const AF_IMPLINK: u32 = 3;
pub const AF_INET: u32 = 2;
pub const AF_INET6: u32 = 23;
pub const AF_IPX: u32 = 6;
pub const AF_IRDA: u32 = 26;
pub const AF_ISO: u32 = 7;
pub const AF_LAT: u32 = 14;
pub const AF_LINK: u32 = 33;
pub const AF_MAX: u32 = 35;
pub const AF_NETBIOS: u32 = 17;
pub const AF_NETDES: u32 = 28;
pub const AF_NS: u32 = 6;
pub const AF_OSI: u32 = 7;
pub const AF_PUP: u32 = 4;
pub const AF_SNA: u32 = 11;
pub const AF_TCNMESSAGE: u32 = 30;
pub const AF_TCNPROCESS: u32 = 29;
pub const AF_UNIX: u32 = 1;
pub const AF_UNKNOWN1: u32 = 20;
pub const AF_UNSPEC: u32 = 0;
pub const AF_VOICEVIEW: u32 = 18;
pub const AI_ADDRCONFIG: u32 = 1024;
pub const AI_ALL: u32 = 256;
pub const AI_BYPASS_DNS_CACHE: u32 = 64;
pub const AI_CANONNAME: u32 = 2;
pub const AI_DISABLE_IDN_ENCODING: u32 = 524288;
pub const AI_DNS_ONLY: u32 = 16;
pub const AI_DNS_RESPONSE_HOSTFILE: u32 = 2;
pub const AI_DNS_RESPONSE_SECURE: u32 = 1;
pub const AI_DNS_SERVER_TYPE_DOH: u32 = 2;
pub const AI_DNS_SERVER_TYPE_DOT: u32 = 3;
pub const AI_DNS_SERVER_TYPE_UDP: u32 = 1;
pub const AI_DNS_SERVER_UDP_FALLBACK: u32 = 1;
pub const AI_EXCLUSIVE_CUSTOM_SERVERS: u32 = 2097152;
pub const AI_EXTENDED: u32 = 2147483648;
pub const AI_EXTRA_DNSSEC_REQUIRED: u32 = 1;
pub const AI_FILESERVER: u32 = 262144;
pub const AI_FORCE_CLEAR_TEXT: u32 = 32;
pub const AI_FQDN: u32 = 131072;
pub const AI_NON_AUTHORITATIVE: u32 = 16384;
pub const AI_NUMERICHOST: u32 = 4;
pub const AI_NUMERICSERV: u32 = 8;
pub const AI_PASSIVE: u32 = 1;
pub const AI_REQUIRE_SECURE: u32 = 536870912;
pub const AI_RESOLUTION_HANDLE: u32 = 1073741824;
pub const AI_RETURN_PREFERRED_NAMES: u32 = 65536;
pub const AI_RETURN_RESPONSE_FLAGS: u32 = 268435456;
pub const AI_RETURN_TTL: u32 = 128;
pub const AI_SECURE: u32 = 32768;
pub const AI_SECURE_WITH_FALLBACK: u32 = 1048576;
pub const AI_V4MAPPED: u32 = 2048;
pub type BTHNS_INQUIRYBLOB = BTH_QUERY_DEVICE;
#[cfg(feature = "bthsdpdef")]
pub type BTHNS_RESTRICTIONBLOB = BTH_QUERY_SERVICE;
pub const BTHNS_RESULT_DEVICE_AUTHENTICATED: u32 = 262144;
pub const BTHNS_RESULT_DEVICE_CONNECTED: u32 = 65536;
pub const BTHNS_RESULT_DEVICE_REMEMBERED: u32 = 131072;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type BTHNS_SETBLOB = BTH_SET_SERVICE;
pub const BTHPROTO_L2CAP: u32 = 256;
pub const BTHPROTO_RFCOMM: u32 = 3;
pub const BTH_ADDR_STRING_SIZE: u32 = 12;
#[repr(C, packed(1))]
#[cfg(feature = "bthdef")]
#[derive(Clone, Copy, Default)]
pub struct BTH_INFO_REQ {
    pub btAddr: super::bthdef::BTH_ADDR,
    pub infoType: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct BTH_INFO_RSP {
    pub result: u16,
    pub dataLen: u8,
    pub Anonymous: BTH_INFO_RSP_0,
}
impl Default for BTH_INFO_RSP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union BTH_INFO_RSP_0 {
    pub connectionlessMTU: u16,
    pub data: [u8; 44],
}
impl Default for BTH_INFO_RSP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "bthdef")]
#[derive(Clone, Copy)]
pub struct BTH_PING_REQ {
    pub btAddr: super::bthdef::BTH_ADDR,
    pub dataLen: u8,
    pub data: [u8; 44],
}
#[cfg(feature = "bthdef")]
impl Default for BTH_PING_REQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BTH_PING_RSP {
    pub dataLen: u8,
    pub data: [u8; 44],
}
impl Default for BTH_PING_RSP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct BTH_QUERY_DEVICE {
    pub LAP: u32,
    pub length: u8,
}
#[repr(C, packed(1))]
#[cfg(feature = "bthsdpdef")]
#[derive(Clone, Copy)]
pub struct BTH_QUERY_SERVICE {
    pub r#type: u32,
    pub serviceHandle: u32,
    pub uuids: [super::bthsdpdef::SdpQueryUuid; 12],
    pub numRange: u32,
    pub pRange: [super::bthsdpdef::SdpAttributeRange; 1],
}
#[cfg(feature = "bthsdpdef")]
impl Default for BTH_QUERY_SERVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BTH_SDP_VERSION: u32 = 1;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct BTH_SET_SERVICE {
    pub pSdpVersion: super::minwindef::PULONG,
    pub pRecordHandle: *mut super::winnt::HANDLE,
    pub fCodService: u32,
    pub Reserved: [u32; 5],
    pub ulRecordLength: u32,
    pub pRecord: [u8; 1],
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for BTH_SET_SERVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BT_PORT_ANY: u32 = 4294967295;
pub const BT_PORT_DYN_FIRST: u32 = 4097;
pub const BT_PORT_MAX: u32 = 65535;
pub const BT_PORT_MIN: u32 = 1;
pub type CMSGHDR = WSACMSGHDR;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CSADDR_INFO {
    pub LocalAddr: SOCKET_ADDRESS,
    pub RemoteAddr: SOCKET_ADDRESS,
    pub iSocketType: i32,
    pub iProtocol: i32,
}
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GROUP_FILTER {
    pub gf_interface: u32,
    pub gf_group: SOCKADDR_STORAGE,
    pub gf_fmode: MULTICAST_MODE_TYPE,
    pub gf_numsrc: u32,
    pub gf_slist: [SOCKADDR_STORAGE; 1],
}
impl Default for GROUP_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GROUP_REQ {
    pub gr_interface: u32,
    pub gr_group: SOCKADDR_STORAGE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GROUP_SOURCE_REQ {
    pub gsr_interface: u32,
    pub gsr_group: SOCKADDR_STORAGE,
    pub gsr_source: SOCKADDR_STORAGE,
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy)]
pub struct ICMP_ERROR_INFO {
    pub srcaddress: SOCKADDR_INET,
    pub protocol: IPPROTO,
    pub r#type: u8,
    pub code: u8,
}
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
impl Default for ICMP_ERROR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IFF_BROADCAST: u32 = 2;
pub const IFF_LOOPBACK: u32 = 4;
pub const IFF_MULTICAST: u32 = 16;
pub const IFF_POINTTOPOINT: u32 = 8;
pub const IFF_UP: u32 = 1;
pub const IN6ADDR_6TO4PREFIX_LENGTH: u32 = 16;
pub const IN6ADDR_LINKLOCALPREFIX_LENGTH: u32 = 64;
pub const IN6ADDR_MULTICASTPREFIX_LENGTH: u32 = 8;
pub const IN6ADDR_SOLICITEDNODEMULTICASTPREFIX_LENGTH: u32 = 104;
pub const IN6ADDR_TEREDOPREFIX_LENGTH: u32 = 32;
pub const IN6ADDR_V4MAPPEDPREFIX_LENGTH: u32 = 96;
#[repr(C)]
#[cfg(feature = "in6addr")]
#[derive(Clone, Copy)]
pub struct IN6_PKTINFO {
    pub ipi6_addr: super::in6addr::IN6_ADDR,
    pub ipi6_ifindex: u32,
}
#[cfg(feature = "in6addr")]
impl Default for IN6_PKTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "in6addr")]
#[derive(Clone, Copy)]
pub struct IN6_PKTINFO_EX {
    pub pkt_info: IN6_PKTINFO,
    pub scope_id: SCOPE_ID,
}
#[cfg(feature = "in6addr")]
impl Default for IN6_PKTINFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INADDR_ANY: u32 = 0;
pub const INADDR_BROADCAST: u32 = 4294967295;
pub const INADDR_LOOPBACK: u32 = 2130706433;
pub const INADDR_NONE: u32 = 4294967295;
pub const INET6_ADDRSTRLEN: u32 = 65;
pub const INET_ADDRSTRLEN: u32 = 22;
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy)]
pub struct INTERFACE_INFO {
    pub iiFlags: u32,
    pub iiAddress: sockaddr_gen,
    pub iiBroadcastAddress: sockaddr_gen,
    pub iiNetmask: sockaddr_gen,
}
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
impl Default for INTERFACE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INTERFACE_INFO_EX {
    pub iiFlags: u32,
    pub iiAddress: SOCKET_ADDRESS,
    pub iiBroadcastAddress: SOCKET_ADDRESS,
    pub iiNetmask: SOCKET_ADDRESS,
}
pub const IN_CLASSA_HOST: u32 = 16777215;
pub const IN_CLASSA_MAX: u32 = 128;
pub const IN_CLASSA_NET: u32 = 4278190080;
pub const IN_CLASSA_NSHIFT: u32 = 24;
pub const IN_CLASSB_HOST: u32 = 65535;
pub const IN_CLASSB_MAX: u32 = 65536;
pub const IN_CLASSB_NET: u32 = 4294901760;
pub const IN_CLASSB_NSHIFT: u32 = 16;
pub const IN_CLASSC_HOST: u32 = 255;
pub const IN_CLASSC_NET: u32 = 4294967040;
pub const IN_CLASSC_NSHIFT: u32 = 8;
pub const IN_CLASSD_HOST: u32 = 268435455;
pub const IN_CLASSD_NET: u32 = 4026531840;
pub const IN_CLASSD_NSHIFT: u32 = 28;
#[repr(C)]
#[cfg(feature = "inaddr")]
#[derive(Clone, Copy)]
pub struct IN_PKTINFO {
    pub ipi_addr: super::inaddr::IN_ADDR,
    pub ipi_ifindex: u32,
}
#[cfg(feature = "inaddr")]
impl Default for IN_PKTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "inaddr")]
#[derive(Clone, Copy)]
pub struct IN_PKTINFO_EX {
    pub pkt_info: IN_PKTINFO,
    pub scope_id: SCOPE_ID,
}
#[cfg(feature = "inaddr")]
impl Default for IN_PKTINFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IN_RECVERR {
    pub protocol: IPPROTO,
    pub info: u32,
    pub r#type: u8,
    pub code: u8,
}
pub const IOCPARM_MASK: u32 = 127;
pub const IOC_IN: u32 = 2147483648;
pub const IOC_INOUT: i32 = -1073741824;
pub const IOC_OUT: u32 = 1073741824;
pub const IOC_PROTOCOL: u32 = 268435456;
pub const IOC_UNIX: u32 = 0;
pub const IOC_VENDOR: u32 = 402653184;
pub const IOC_VOID: u32 = 536870912;
pub const IOC_WS2: u32 = 134217728;
pub const IOC_WSK: u32 = 251658240;
pub const IP6T_SO_ORIGINAL_DST: u32 = 12303;
pub const IPPORT_BIFFUDP: u32 = 512;
pub const IPPORT_CHARGEN: u32 = 19;
pub const IPPORT_CMDSERVER: u32 = 514;
pub const IPPORT_DAYTIME: u32 = 13;
pub const IPPORT_DISCARD: u32 = 9;
pub const IPPORT_DYNAMIC_MAX: u32 = 65535;
pub const IPPORT_DYNAMIC_MIN: u32 = 49152;
pub const IPPORT_ECHO: u32 = 7;
pub const IPPORT_EFSSERVER: u32 = 520;
pub const IPPORT_EPMAP: u32 = 135;
pub const IPPORT_EXECSERVER: u32 = 512;
pub const IPPORT_FINGER: u32 = 79;
pub const IPPORT_FTP: u32 = 21;
pub const IPPORT_FTP_DATA: u32 = 20;
pub const IPPORT_HTTPS: u32 = 443;
pub const IPPORT_IMAP: u32 = 143;
pub const IPPORT_IMAP3: u32 = 220;
pub const IPPORT_LDAP: u32 = 389;
pub const IPPORT_LOGINSERVER: u32 = 513;
pub const IPPORT_MICROSOFT_DS: u32 = 445;
pub const IPPORT_MSP: u32 = 18;
pub const IPPORT_MTP: u32 = 57;
pub const IPPORT_NAMESERVER: u32 = 42;
pub const IPPORT_NETBIOS_DGM: u32 = 138;
pub const IPPORT_NETBIOS_NS: u32 = 137;
pub const IPPORT_NETBIOS_SSN: u32 = 139;
pub const IPPORT_NETSTAT: u32 = 15;
pub const IPPORT_NTP: u32 = 123;
pub const IPPORT_POP3: u32 = 110;
pub const IPPORT_QOTD: u32 = 17;
pub const IPPORT_REGISTERED_MAX: u32 = 49151;
pub const IPPORT_REGISTERED_MIN: u32 = 1024;
pub const IPPORT_RESERVED: u32 = 1024;
pub const IPPORT_RJE: u32 = 77;
pub const IPPORT_ROUTESERVER: u32 = 520;
pub const IPPORT_SMTP: u32 = 25;
pub const IPPORT_SNMP: u32 = 161;
pub const IPPORT_SNMP_TRAP: u32 = 162;
pub const IPPORT_SUPDUP: u32 = 95;
pub const IPPORT_SYSTAT: u32 = 11;
pub const IPPORT_TCPMUX: u32 = 1;
pub const IPPORT_TELNET: u32 = 23;
pub const IPPORT_TFTP: u32 = 69;
pub const IPPORT_TIMESERVER: u32 = 37;
pub const IPPORT_TTYLINK: u32 = 87;
pub const IPPORT_WHOIS: u32 = 43;
pub const IPPORT_WHOSERVER: u32 = 513;
pub type IPPROTO = i32;
pub const IPPROTO_AH: IPPROTO = 51;
pub const IPPROTO_CBT: IPPROTO = 7;
pub const IPPROTO_DSTOPTS: IPPROTO = 60;
pub const IPPROTO_EGP: IPPROTO = 8;
pub const IPPROTO_ESP: IPPROTO = 50;
pub const IPPROTO_FRAGMENT: IPPROTO = 44;
pub const IPPROTO_GGP: IPPROTO = 3;
pub const IPPROTO_HOPOPTS: IPPROTO = 0;
pub const IPPROTO_ICLFXBM: IPPROTO = 78;
pub const IPPROTO_ICMP: IPPROTO = 1;
pub const IPPROTO_ICMPV6: IPPROTO = 58;
pub const IPPROTO_IDP: IPPROTO = 22;
pub const IPPROTO_IGMP: IPPROTO = 2;
pub const IPPROTO_IGP: IPPROTO = 9;
pub const IPPROTO_IP: u32 = 0;
pub const IPPROTO_IPV4: IPPROTO = 4;
pub const IPPROTO_IPV6: IPPROTO = 41;
pub const IPPROTO_L2TP: IPPROTO = 115;
pub const IPPROTO_MAX: IPPROTO = 256;
pub const IPPROTO_ND: IPPROTO = 77;
pub const IPPROTO_NONE: IPPROTO = 59;
pub const IPPROTO_PGM: IPPROTO = 113;
pub const IPPROTO_PIM: IPPROTO = 103;
pub const IPPROTO_PUP: IPPROTO = 12;
pub const IPPROTO_RAW: IPPROTO = 255;
pub const IPPROTO_RDP: IPPROTO = 27;
pub const IPPROTO_RESERVED_IPSEC: IPPROTO = 258;
pub const IPPROTO_RESERVED_IPSECOFFLOAD: IPPROTO = 259;
pub const IPPROTO_RESERVED_MAX: IPPROTO = 261;
pub const IPPROTO_RESERVED_RAW: IPPROTO = 257;
pub const IPPROTO_RESERVED_WNV: IPPROTO = 260;
pub const IPPROTO_ROUTING: IPPROTO = 43;
pub const IPPROTO_SCTP: IPPROTO = 132;
pub const IPPROTO_ST: IPPROTO = 5;
pub const IPPROTO_TCP: IPPROTO = 6;
pub const IPPROTO_UDP: IPPROTO = 17;
pub const IPV6_ADDRESS_BITS: u32 = 128;
pub const IPV6_ADD_IFLIST: u32 = 29;
pub const IPV6_ADD_MEMBERSHIP: u32 = 12;
pub const IPV6_CHECKSUM: u32 = 26;
pub const IPV6_DEL_IFLIST: u32 = 30;
pub const IPV6_DONTFRAG: u32 = 14;
pub const IPV6_DROP_MEMBERSHIP: u32 = 13;
pub const IPV6_ECN: u32 = 50;
pub const IPV6_GET_IFLIST: u32 = 33;
pub const IPV6_HDRINCL: u32 = 2;
pub const IPV6_HOPLIMIT: u32 = 21;
pub const IPV6_HOPOPTS: u32 = 1;
pub const IPV6_IFLIST: u32 = 28;
pub const IPV6_JOIN_GROUP: u32 = 12;
pub const IPV6_LEAVE_GROUP: u32 = 13;
#[repr(C)]
#[cfg(feature = "in6addr")]
#[derive(Clone, Copy)]
pub struct IPV6_MREQ {
    pub ipv6mr_multiaddr: super::in6addr::IN6_ADDR,
    pub ipv6mr_interface: u32,
}
#[cfg(feature = "in6addr")]
impl Default for IPV6_MREQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IPV6_MTU: u32 = 72;
pub const IPV6_MTU_DISCOVER: u32 = 71;
pub const IPV6_MULTICAST_HOPS: u32 = 10;
pub const IPV6_MULTICAST_IF: u32 = 9;
pub const IPV6_MULTICAST_LOOP: u32 = 11;
pub const IPV6_NRT_INTERFACE: u32 = 74;
pub const IPV6_PKTINFO: u32 = 19;
pub const IPV6_PKTINFO_EX: u32 = 51;
pub const IPV6_PROTECTION_LEVEL: u32 = 23;
pub const IPV6_RECVDSTADDR: u32 = 25;
pub const IPV6_RECVECN: u32 = 50;
pub const IPV6_RECVERR: u32 = 75;
pub const IPV6_RECVIF: u32 = 24;
pub const IPV6_RECVRTHDR: u32 = 38;
pub const IPV6_RECVTCLASS: u32 = 40;
pub const IPV6_RTHDR: u32 = 32;
pub const IPV6_TCLASS: u32 = 39;
pub const IPV6_UNICAST_HOPS: u32 = 4;
pub const IPV6_UNICAST_IF: u32 = 31;
pub const IPV6_USER_MTU: u32 = 76;
pub const IPV6_V6ONLY: u32 = 27;
pub const IPV6_WFP_REDIRECT_CONTEXT: u32 = 70;
pub const IPV6_WFP_REDIRECT_RECORDS: u32 = 60;
pub const IP_ADD_IFLIST: u32 = 29;
pub const IP_ADD_MEMBERSHIP: u32 = 12;
pub const IP_ADD_SOURCE_MEMBERSHIP: u32 = 15;
pub const IP_BLOCK_SOURCE: u32 = 17;
pub const IP_DEL_IFLIST: u32 = 30;
pub const IP_DONTFRAGMENT: u32 = 14;
pub const IP_DROP_MEMBERSHIP: u32 = 13;
pub const IP_DROP_SOURCE_MEMBERSHIP: u32 = 16;
pub const IP_ECN: u32 = 50;
pub const IP_GET_IFLIST: u32 = 33;
pub const IP_HDRINCL: u32 = 2;
pub const IP_HOPLIMIT: u32 = 21;
pub const IP_IFLIST: u32 = 28;
#[repr(C)]
#[cfg(feature = "inaddr")]
#[derive(Clone, Copy)]
pub struct IP_MREQ {
    pub imr_multiaddr: super::inaddr::IN_ADDR,
    pub imr_interface: super::inaddr::IN_ADDR,
}
#[cfg(feature = "inaddr")]
impl Default for IP_MREQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "inaddr")]
#[derive(Clone, Copy)]
pub struct IP_MREQ_SOURCE {
    pub imr_multiaddr: super::inaddr::IN_ADDR,
    pub imr_sourceaddr: super::inaddr::IN_ADDR,
    pub imr_interface: super::inaddr::IN_ADDR,
}
#[cfg(feature = "inaddr")]
impl Default for IP_MREQ_SOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "inaddr")]
#[derive(Clone, Copy)]
pub struct IP_MSFILTER {
    pub imsf_multiaddr: super::inaddr::IN_ADDR,
    pub imsf_interface: super::inaddr::IN_ADDR,
    pub imsf_fmode: MULTICAST_MODE_TYPE,
    pub imsf_numsrc: u32,
    pub imsf_slist: [super::inaddr::IN_ADDR; 1],
}
#[cfg(feature = "inaddr")]
impl Default for IP_MSFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_MTU: u32 = 73;
pub const IP_MTU_DISCOVER: u32 = 71;
pub const IP_MULTICAST_IF: u32 = 9;
pub const IP_MULTICAST_LOOP: u32 = 11;
pub const IP_MULTICAST_TTL: u32 = 10;
pub const IP_NRT_INTERFACE: u32 = 74;
pub const IP_OPTIONS: u32 = 1;
pub const IP_ORIGINAL_ARRIVAL_IF: u32 = 47;
pub const IP_PKTINFO: u32 = 19;
pub const IP_PKTINFO_EX: u32 = 51;
pub const IP_PMTUDISC_DO: PMTUD_STATE = 1;
pub const IP_PMTUDISC_DONT: PMTUD_STATE = 2;
pub const IP_PMTUDISC_MAX: PMTUD_STATE = 4;
pub const IP_PMTUDISC_NOT_SET: PMTUD_STATE = 0;
pub const IP_PMTUDISC_PROBE: PMTUD_STATE = 3;
pub const IP_PROTECTION_LEVEL: u32 = 23;
pub const IP_RECEIVE_BROADCAST: u32 = 22;
pub const IP_RECVDSTADDR: u32 = 25;
pub const IP_RECVECN: u32 = 50;
pub const IP_RECVERR: u32 = 75;
pub const IP_RECVIF: u32 = 24;
pub const IP_RECVRTHDR: u32 = 38;
pub const IP_RECVTCLASS: u32 = 40;
pub const IP_RECVTOS: u32 = 40;
pub const IP_RECVTTL: u32 = 21;
pub const IP_RTHDR: u32 = 32;
pub const IP_TCLASS: u32 = 39;
pub const IP_TOS: u32 = 3;
pub const IP_TTL: u32 = 4;
pub const IP_UNBLOCK_SOURCE: u32 = 18;
pub const IP_UNICAST_IF: u32 = 31;
pub const IP_UNSPECIFIED_HOP_LIMIT: i32 = -1;
pub const IP_UNSPECIFIED_TYPE_OF_SERVICE: i32 = -1;
pub const IP_USER_MTU: u32 = 76;
pub const IP_WFP_REDIRECT_CONTEXT: u32 = 70;
pub const IP_WFP_REDIRECT_RECORDS: u32 = 60;
pub type LPADDRINFO = *mut ADDRINFOA;
#[cfg(feature = "guiddef")]
pub type LPADDRINFOEX2A = *mut ADDRINFOEX2A;
#[cfg(feature = "guiddef")]
pub type LPADDRINFOEX2W = *mut ADDRINFOEX2W;
#[cfg(feature = "guiddef")]
pub type LPADDRINFOEX3 = *mut ADDRINFOEX3;
#[cfg(feature = "winnt")]
pub type LPADDRINFOEX4 = *mut ADDRINFOEX4;
#[cfg(feature = "winnt")]
pub type LPADDRINFOEX5 = *mut ADDRINFOEX5;
#[cfg(feature = "guiddef")]
pub type LPADDRINFOEXA = *mut ADDRINFOEXA;
#[cfg(feature = "guiddef")]
pub type LPADDRINFOEXW = *mut ADDRINFOEXW;
pub type LPCSADDR_INFO = *mut CSADDR_INFO;
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
pub type LPINTERFACE_INFO = *mut INTERFACE_INFO;
pub type LPINTERFACE_INFO_EX = *mut INTERFACE_INFO_EX;
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
pub type LPLOOKUPSERVICE_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(dwerror: u32, dwbytes: u32, lpoverlapped: *const super::minwinbase::OVERLAPPED)>;
pub type LPSOCKADDR = *mut SOCKADDR;
#[cfg(feature = "in6addr")]
pub type LPSOCKADDR_IN6 = *mut SOCKADDR_IN6_LH;
#[cfg(feature = "in6addr")]
pub type LPSOCKADDR_IN6_LH = *mut SOCKADDR_IN6_LH;
#[cfg(feature = "in6addr")]
pub type LPSOCKADDR_IN6_W2KSP1 = *mut SOCKADDR_IN6_W2KSP1;
pub type LPSOCKADDR_STORAGE = *mut SOCKADDR_STORAGE;
pub type LPSOCKADDR_STORAGE_LH = *mut SOCKADDR_STORAGE_LH;
pub type LPSOCKADDR_STORAGE_XP = *mut SOCKADDR_STORAGE_XP;
pub type LPSOCKET_ADDRESS = *mut SOCKET_ADDRESS;
pub type LPSOCKET_ADDRESS_LIST = *mut SOCKET_ADDRESS_LIST;
pub type LPWSABUF = *mut WSABUF;
pub type LPWSACMSGHDR = *mut WSACMSGHDR;
pub type LPWSAMSG = *mut WSAMSG;
pub const MCAST_BLOCK_SOURCE: u32 = 43;
pub const MCAST_EXCLUDE: MULTICAST_MODE_TYPE = 1;
pub const MCAST_INCLUDE: MULTICAST_MODE_TYPE = 0;
pub const MCAST_JOIN_GROUP: u32 = 41;
pub const MCAST_JOIN_SOURCE_GROUP: u32 = 45;
pub const MCAST_LEAVE_GROUP: u32 = 42;
pub const MCAST_LEAVE_SOURCE_GROUP: u32 = 46;
pub const MCAST_UNBLOCK_SOURCE: u32 = 44;
pub const MSC_BREAK_BIT: u32 = 2;
pub const MSC_DV_BIT: u32 = 128;
pub const MSC_FC_BIT: u32 = 2;
pub const MSC_IC_BIT: u32 = 64;
pub const MSC_RESERVED: u32 = 48;
pub const MSC_RTC_BIT: u32 = 4;
pub const MSC_RTR_BIT: u32 = 8;
pub const MSG_BCAST: u32 = 1024;
pub const MSG_CTRUNC: u32 = 512;
pub const MSG_ERRQUEUE: u32 = 4096;
pub const MSG_MCAST: u32 = 2048;
pub const MSG_TRUNC: u32 = 256;
pub type MULTICAST_MODE_TYPE = i32;
pub const NI_DGRAM: u32 = 16;
pub const NI_MAXHOST: u32 = 1025;
pub const NI_MAXSERV: u32 = 32;
pub const NI_NAMEREQD: u32 = 4;
pub const NI_NOFQDN: u32 = 1;
pub const NI_NUMERICHOST: u32 = 2;
pub const NI_NUMERICSERV: u32 = 8;
pub const NS_ALL: u32 = 0;
pub const NS_BTH: u32 = 16;
pub const NS_DHCP: u32 = 6;
pub const NS_DNS: u32 = 12;
pub const NS_EMAIL: u32 = 37;
pub const NS_MS: u32 = 30;
pub const NS_NBP: u32 = 20;
pub const NS_NDS: u32 = 2;
pub const NS_NETBT: u32 = 13;
pub const NS_NETDES: u32 = 60;
pub const NS_NIS: u32 = 41;
pub const NS_NISPLUS: u32 = 42;
pub const NS_NLA: u32 = 15;
pub const NS_NTDS: u32 = 32;
pub const NS_PEER_BROWSE: u32 = 3;
pub const NS_PNRPCLOUD: u32 = 39;
pub const NS_PNRPNAME: u32 = 38;
pub const NS_SAP: u32 = 1;
pub const NS_SLP: u32 = 5;
pub const NS_STDA: u32 = 31;
pub const NS_TCPIP_HOSTS: u32 = 11;
pub const NS_TCPIP_LOCAL: u32 = 10;
pub const NS_WINS: u32 = 14;
pub const NS_WRQ: u32 = 50;
pub const NS_X500: u32 = 40;
pub type PADDRINFOA = *mut ADDRINFOA;
#[cfg(feature = "guiddef")]
pub type PADDRINFOEX = *mut ADDRINFOEXA;
#[cfg(feature = "guiddef")]
pub type PADDRINFOEX2A = *mut ADDRINFOEX2A;
#[cfg(feature = "guiddef")]
pub type PADDRINFOEX2W = *mut ADDRINFOEX2W;
#[cfg(feature = "guiddef")]
pub type PADDRINFOEX3 = *mut ADDRINFOEX3;
#[cfg(feature = "winnt")]
pub type PADDRINFOEX4 = *mut ADDRINFOEX4;
#[cfg(feature = "winnt")]
pub type PADDRINFOEX5 = *mut ADDRINFOEX5;
#[cfg(feature = "winnt")]
pub type PADDRINFOEX6 = *mut ADDRINFOEX6;
#[cfg(feature = "winnt")]
pub type PADDRINFOEX7 = *mut ADDRINFOEX7;
#[cfg(feature = "guiddef")]
pub type PADDRINFOEXA = *mut ADDRINFOEXA;
#[cfg(feature = "guiddef")]
pub type PADDRINFOEXW = *mut ADDRINFOEXW;
pub type PADDRINFOT = *mut ADDRINFOA;
pub type PADDRINFOW = *mut ADDRINFOW;
pub type PBTHNS_INQUIRYBLOB = *mut BTH_QUERY_DEVICE;
#[cfg(feature = "bthsdpdef")]
pub type PBTHNS_RESTRICTIONBLOB = *mut BTH_QUERY_SERVICE;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PBTHNS_SETBLOB = *mut BTH_SET_SERVICE;
#[cfg(feature = "bthdef")]
pub type PBTH_INFO_REQ = *mut BTH_INFO_REQ;
pub type PBTH_INFO_RSP = *mut BTH_INFO_RSP;
#[cfg(feature = "bthdef")]
pub type PBTH_PING_REQ = *mut BTH_PING_REQ;
pub type PBTH_PING_RSP = *mut BTH_PING_RSP;
pub type PBTH_QUERY_DEVICE = *mut BTH_QUERY_DEVICE;
#[cfg(feature = "bthsdpdef")]
pub type PBTH_QUERY_SERVICE = *mut BTH_QUERY_SERVICE;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PBTH_SET_SERVICE = *mut BTH_SET_SERVICE;
pub type PCMSGHDR = *mut WSACMSGHDR;
pub type PCSADDR_INFO = *mut CSADDR_INFO;
pub type PGROUP_FILTER = *mut GROUP_FILTER;
pub type PGROUP_REQ = *mut GROUP_REQ;
pub type PGROUP_SOURCE_REQ = *mut GROUP_SOURCE_REQ;
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
pub type PICMP_ERROR_INFO = *mut ICMP_ERROR_INFO;
#[cfg(feature = "in6addr")]
pub type PIN6_PKTINFO = *mut IN6_PKTINFO;
#[cfg(feature = "in6addr")]
pub type PIN6_PKTINFO_EX = *mut IN6_PKTINFO_EX;
#[cfg(feature = "inaddr")]
pub type PIN_PKTINFO = *mut IN_PKTINFO;
#[cfg(feature = "inaddr")]
pub type PIN_PKTINFO_EX = *mut IN_PKTINFO_EX;
pub type PIN_RECVERR = *mut IN_RECVERR;
pub type PIPROTO = *mut IPPROTO;
#[cfg(feature = "in6addr")]
pub type PIPV6_MREQ = *mut IPV6_MREQ;
#[cfg(feature = "inaddr")]
pub type PIP_MREQ = *mut IP_MREQ;
#[cfg(feature = "inaddr")]
pub type PIP_MREQ_SOURCE = *mut IP_MREQ_SOURCE;
#[cfg(feature = "inaddr")]
pub type PIP_MSFILTER = *mut IP_MSFILTER;
pub type PMTUD_STATE = i32;
pub type PPMTUD_STATE = *mut PMTUD_STATE;
pub type PRFCOMM_COMMAND = *mut RFCOMM_COMMAND;
pub type PRFCOMM_MSC_DATA = *mut RFCOMM_MSC_DATA;
pub type PRFCOMM_RLS_DATA = *mut RFCOMM_RLS_DATA;
pub type PRFCOMM_RPN_DATA = *mut RFCOMM_RPN_DATA;
pub const PROTECTION_LEVEL_DEFAULT: u32 = 4294967295;
pub const PROTECTION_LEVEL_EDGERESTRICTED: u32 = 20;
pub const PROTECTION_LEVEL_RESTRICTED: u32 = 30;
pub const PROTECTION_LEVEL_UNRESTRICTED: u32 = 10;
pub type PSCOPE_ID = *mut SCOPE_ID;
pub type PSOCKADDR = *mut SOCKADDR;
#[cfg(feature = "bthdef")]
pub type PSOCKADDR_BTH = *mut SOCKADDR_BTH;
pub type PSOCKADDR_DL = *mut SOCKADDR_DL;
#[cfg(feature = "inaddr")]
pub type PSOCKADDR_IN = *mut SOCKADDR_IN;
#[cfg(feature = "in6addr")]
pub type PSOCKADDR_IN6 = *mut SOCKADDR_IN6_LH;
#[cfg(feature = "in6addr")]
pub type PSOCKADDR_IN6_LH = *mut SOCKADDR_IN6_LH;
#[cfg(feature = "in6addr")]
pub type PSOCKADDR_IN6_PAIR = *mut SOCKADDR_IN6_PAIR;
#[cfg(feature = "in6addr")]
pub type PSOCKADDR_IN6_W2KSP1 = *mut SOCKADDR_IN6_W2KSP1;
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
pub type PSOCKADDR_INET = *mut SOCKADDR_INET;
pub type PSOCKADDR_STORAGE = *mut SOCKADDR_STORAGE;
pub type PSOCKADDR_STORAGE_LH = *mut SOCKADDR_STORAGE_LH;
pub type PSOCKADDR_STORAGE_XP = *mut SOCKADDR_STORAGE_XP;
pub type PSOCKET_ADDRESS = *mut SOCKET_ADDRESS;
pub type PSOCKET_ADDRESS_LIST = *mut SOCKET_ADDRESS_LIST;
#[cfg(feature = "winnt")]
pub type PSOCKET_PROCESSOR_AFFINITY = *mut SOCKET_PROCESSOR_AFFINITY;
pub type PWSACMSGHDR = *mut WSACMSGHDR;
pub type PWSAMSG = *mut WSAMSG;
pub const RFCOMM_CMD_MSC: u32 = 1;
pub const RFCOMM_CMD_NONE: u32 = 0;
pub const RFCOMM_CMD_RLS: u32 = 2;
pub const RFCOMM_CMD_RPN: u32 = 3;
pub const RFCOMM_CMD_RPN_REQUEST: u32 = 4;
pub const RFCOMM_CMD_RPN_RESPONSE: u32 = 5;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct RFCOMM_COMMAND {
    pub CmdType: u32,
    pub Data: RFCOMM_COMMAND_0,
}
impl Default for RFCOMM_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RFCOMM_COMMAND_0 {
    pub MSC: RFCOMM_MSC_DATA,
    pub RLS: RFCOMM_RLS_DATA,
    pub RPN: RFCOMM_RPN_DATA,
}
impl Default for RFCOMM_COMMAND_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RFCOMM_MAX_MTU: u32 = 1011;
pub const RFCOMM_MIN_MTU: u32 = 23;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RFCOMM_MSC_DATA {
    pub Signals: u8,
    pub Break: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RFCOMM_RLS_DATA {
    pub LineStatus: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RFCOMM_RPN_DATA {
    pub Baud: u8,
    pub Data: u8,
    pub FlowControl: u8,
    pub XonChar: u8,
    pub XoffChar: u8,
    pub ParameterMask1: u8,
    pub ParameterMask2: u8,
}
pub const RLS_ERROR: u32 = 1;
pub const RLS_FRAMING: u32 = 8;
pub const RLS_OVERRUN: u32 = 2;
pub const RLS_PARITY: u32 = 4;
pub const RPN_BAUD_115200: u32 = 7;
pub const RPN_BAUD_19200: u32 = 4;
pub const RPN_BAUD_230400: u32 = 8;
pub const RPN_BAUD_2400: u32 = 0;
pub const RPN_BAUD_38400: u32 = 5;
pub const RPN_BAUD_4800: u32 = 1;
pub const RPN_BAUD_57600: u32 = 6;
pub const RPN_BAUD_7200: u32 = 2;
pub const RPN_BAUD_9600: u32 = 3;
pub const RPN_DATA_5: u32 = 0;
pub const RPN_DATA_6: u32 = 1;
pub const RPN_DATA_7: u32 = 2;
pub const RPN_DATA_8: u32 = 3;
pub const RPN_FLOW_RTC_IN: u32 = 16;
pub const RPN_FLOW_RTC_OUT: u32 = 32;
pub const RPN_FLOW_RTR_IN: u32 = 4;
pub const RPN_FLOW_RTR_OUT: u32 = 8;
pub const RPN_FLOW_X_IN: u32 = 1;
pub const RPN_FLOW_X_OUT: u32 = 2;
pub const RPN_PARAM_BAUD: u32 = 1;
pub const RPN_PARAM_DATA: u32 = 2;
pub const RPN_PARAM_PARITY: u32 = 8;
pub const RPN_PARAM_P_TYPE: u32 = 16;
pub const RPN_PARAM_RTC_IN: u32 = 16;
pub const RPN_PARAM_RTC_OUT: u32 = 32;
pub const RPN_PARAM_RTR_IN: u32 = 4;
pub const RPN_PARAM_RTR_OUT: u32 = 8;
pub const RPN_PARAM_STOP: u32 = 4;
pub const RPN_PARAM_XOFF: u32 = 64;
pub const RPN_PARAM_XON: u32 = 32;
pub const RPN_PARAM_X_IN: u32 = 1;
pub const RPN_PARAM_X_OUT: u32 = 2;
pub const RPN_PARITY_EVEN: u32 = 24;
pub const RPN_PARITY_MARK: u32 = 40;
pub const RPN_PARITY_NONE: u32 = 0;
pub const RPN_PARITY_ODD: u32 = 8;
pub const RPN_PARITY_SPACE: u32 = 56;
pub const RPN_STOP_1: u32 = 0;
pub const RPN_STOP_1_5: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCOPE_ID {
    pub Anonymous: SCOPE_ID_0,
}
impl Default for SCOPE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SCOPE_ID_0 {
    pub Anonymous: SCOPE_ID_0_0,
    pub Value: u32,
}
impl Default for SCOPE_ID_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SCOPE_ID_0_0 {
    pub _bitfield: u32,
}
pub type SCOPE_LEVEL = i32;
pub const SDP_DEFAULT_INQUIRY_MAX_RESPONSES: u32 = 255;
pub const SDP_DEFAULT_INQUIRY_SECONDS: u32 = 6;
pub const SDP_MAX_INQUIRY_SECONDS: u32 = 60;
pub const SDP_SERVICE_ATTRIBUTE_REQUEST: u32 = 2;
pub const SDP_SERVICE_SEARCH_ATTRIBUTE_REQUEST: u32 = 3;
pub const SDP_SERVICE_SEARCH_REQUEST: u32 = 1;
pub const SIOCGIPMSFILTER: i32 = -2147191684;
pub const SIOCGMSFILTER: i32 = -2147191681;
pub const SIOCSIPMSFILTER: i32 = -2147191683;
pub const SIOCSMSFILTER: i32 = -2147191682;
pub const SIO_ADDRESS_LIST_CHANGE: u32 = 671088663;
pub const SIO_ADDRESS_LIST_QUERY: u32 = 1207959574;
pub const SIO_ADDRESS_LIST_SORT: i32 = -939524071;
pub const SIO_ASSOCIATE_HANDLE: i32 = -2013265919;
pub const SIO_BTH_INFO: i32 = -671088631;
pub const SIO_BTH_PING: i32 = -671088632;
pub const SIO_ENABLE_CIRCULAR_QUEUEING: u32 = 671088642;
pub const SIO_FIND_ROUTE: u32 = 1207959555;
pub const SIO_FLUSH: u32 = 671088644;
pub const SIO_GET_BROADCAST_ADDRESS: u32 = 1207959557;
pub const SIO_GET_EXTENSION_FUNCTION_POINTER: i32 = -939524090;
pub const SIO_GET_GROUP_QOS: i32 = -939524088;
pub const SIO_GET_INTERFACE_LIST: u32 = 1074033791;
pub const SIO_GET_INTERFACE_LIST_EX: u32 = 1074033790;
pub const SIO_GET_MULTICAST_FILTER: i32 = -2147191684;
pub const SIO_GET_MULTIPLE_EXTENSION_FUNCTION_POINTER: i32 = -939524060;
pub const SIO_GET_QOS: i32 = -939524089;
pub const SIO_IDEAL_SEND_BACKLOG_CHANGE: u32 = 536900730;
pub const SIO_IDEAL_SEND_BACKLOG_QUERY: u32 = 1074033787;
pub const SIO_MULTICAST_SCOPE: i32 = -2013265910;
pub const SIO_MULTIPOINT_LOOPBACK: i32 = -2013265911;
pub const SIO_QUERY_RSS_PROCESSOR_INFO: u32 = 1207959589;
pub const SIO_QUERY_TARGET_PNP_HANDLE: u32 = 1207959576;
pub const SIO_RESERVED_1: i32 = -2013265894;
pub const SIO_RESERVED_2: i32 = -2013265887;
pub const SIO_RFCOMM_SEND_COMMAND: i32 = -671088539;
pub const SIO_RFCOMM_SESSION_FLOW_OFF: i32 = -671088537;
pub const SIO_RFCOMM_TEST: i32 = -671088536;
pub const SIO_RFCOMM_USECFC: i32 = -671088535;
pub const SIO_RFCOMM_WAIT_COMMAND: i32 = -671088538;
pub const SIO_ROUTING_INTERFACE_CHANGE: i32 = -2013265899;
pub const SIO_ROUTING_INTERFACE_QUERY: i32 = -939524076;
pub const SIO_SET_GROUP_QOS: i32 = -2013265908;
pub const SIO_SET_MULTICAST_FILTER: i32 = -2147191683;
pub const SIO_SET_QOS: i32 = -2013265909;
pub const SIO_TRANSLATE_HANDLE: i32 = -939524083;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOCKADDR {
    pub sa_family: ADDRESS_FAMILY,
    pub sa_data: [i8; 14],
}
impl Default for SOCKADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "bthdef")]
#[derive(Clone, Copy, Default)]
pub struct SOCKADDR_BTH {
    pub addressFamily: u16,
    pub btAddr: super::bthdef::BTH_ADDR,
    pub serviceClassId: windows_core::GUID,
    pub port: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOCKADDR_DL {
    pub sdl_family: ADDRESS_FAMILY,
    pub sdl_data: [u8; 8],
    pub sdl_zero: [u8; 4],
}
impl Default for SOCKADDR_DL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "inaddr")]
#[derive(Clone, Copy)]
pub struct SOCKADDR_IN {
    pub sin_family: ADDRESS_FAMILY,
    pub sin_port: u16,
    pub sin_addr: super::inaddr::IN_ADDR,
    pub sin_zero: [i8; 8],
}
#[cfg(feature = "inaddr")]
impl Default for SOCKADDR_IN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "in6addr")]
pub type SOCKADDR_IN6 = SOCKADDR_IN6_LH;
#[repr(C)]
#[cfg(feature = "in6addr")]
#[derive(Clone, Copy)]
pub struct SOCKADDR_IN6_LH {
    pub sin6_family: ADDRESS_FAMILY,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: super::in6addr::IN6_ADDR,
    pub Anonymous: SOCKADDR_IN6_LH_0,
}
#[cfg(feature = "in6addr")]
impl Default for SOCKADDR_IN6_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "in6addr")]
#[derive(Clone, Copy)]
pub union SOCKADDR_IN6_LH_0 {
    pub sin6_scope_id: u32,
    pub sin6_scope_struct: SCOPE_ID,
}
#[cfg(feature = "in6addr")]
impl Default for SOCKADDR_IN6_LH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "in6addr")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SOCKADDR_IN6_PAIR {
    pub SourceAddress: PSOCKADDR_IN6,
    pub DestinationAddress: PSOCKADDR_IN6,
}
#[repr(C)]
#[cfg(feature = "in6addr")]
#[derive(Clone, Copy)]
pub struct SOCKADDR_IN6_W2KSP1 {
    pub sin6_family: i16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: super::in6addr::IN6_ADDR,
    pub sin6_scope_id: u32,
}
#[cfg(feature = "in6addr")]
impl Default for SOCKADDR_IN6_W2KSP1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy)]
pub union SOCKADDR_INET {
    pub Ipv4: SOCKADDR_IN,
    pub Ipv6: SOCKADDR_IN6,
    pub si_family: ADDRESS_FAMILY,
}
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
impl Default for SOCKADDR_INET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SOCKADDR_STORAGE = SOCKADDR_STORAGE_LH;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOCKADDR_STORAGE_LH {
    pub ss_family: ADDRESS_FAMILY,
    pub __ss_pad1: [i8; 6],
    pub __ss_align: i64,
    pub __ss_pad2: [i8; 112],
}
impl Default for SOCKADDR_STORAGE_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOCKADDR_STORAGE_XP {
    pub ss_family: i16,
    pub __ss_pad1: [i8; 6],
    pub __ss_align: i64,
    pub __ss_pad2: [i8; 112],
}
impl Default for SOCKADDR_STORAGE_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SOCKET_ADDRESS {
    pub lpSockaddr: LPSOCKADDR,
    pub iSockaddrLength: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SOCKET_ADDRESS_LIST {
    pub iAddressCount: i32,
    pub Address: [SOCKET_ADDRESS; 1],
}
impl Default for SOCKET_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SOCKET_PROCESSOR_AFFINITY {
    pub Processor: super::winnt::PROCESSOR_NUMBER,
    pub NumaNodeId: u16,
    pub Reserved: u16,
}
pub const SOCK_DGRAM: u32 = 2;
pub const SOCK_RAW: u32 = 3;
pub const SOCK_RDM: u32 = 4;
pub const SOCK_SEQPACKET: u32 = 5;
pub const SOCK_STREAM: u32 = 1;
pub const SOL_IP: u32 = 65531;
pub const SOL_IPV6: u32 = 65530;
pub const SOL_L2CAP: u32 = 256;
pub const SOL_RFCOMM: u32 = 3;
pub const SOL_SDP: u32 = 257;
pub const SOL_SOCKET: u32 = 65535;
pub const SO_ACCEPTCONN: u32 = 2;
pub const SO_BROADCAST: u32 = 32;
pub const SO_BSP_STATE: u32 = 4105;
pub const SO_BTH_AUTHENTICATE: u32 = 2147483649;
pub const SO_BTH_ENCRYPT: u32 = 2;
pub const SO_BTH_MTU: u32 = 2147483655;
pub const SO_BTH_MTU_MAX: u32 = 2147483656;
pub const SO_BTH_MTU_MIN: u32 = 2147483658;
pub const SO_COMPARTMENT_ID: u32 = 12292;
pub const SO_CONDITIONAL_ACCEPT: u32 = 12290;
pub const SO_DEBUG: u32 = 1;
pub const SO_DONTLINGER: i32 = -129;
pub const SO_DONTROUTE: u32 = 16;
pub const SO_ERROR: u32 = 4103;
pub const SO_EXCLUSIVEADDRUSE: i32 = -5;
pub const SO_GROUP_ID: u32 = 8193;
pub const SO_GROUP_PRIORITY: u32 = 8194;
pub const SO_KEEPALIVE: u32 = 8;
pub const SO_LINGER: u32 = 128;
pub const SO_MAX_MSG_SIZE: u32 = 8195;
pub const SO_OOBINLINE: u32 = 256;
pub const SO_ORIGINAL_DST: u32 = 12303;
pub const SO_PAUSE_ACCEPT: u32 = 12291;
pub const SO_PORT_SCALABILITY: u32 = 12294;
pub const SO_RANDOMIZE_PORT: u32 = 12293;
pub const SO_RCVBUF: u32 = 4098;
pub const SO_RCVLOWAT: u32 = 4100;
pub const SO_RCVTIMEO: u32 = 4102;
pub const SO_RECEIVED_HOPLIMIT: u32 = 12304;
pub const SO_RECEIVED_PROCESSOR: u32 = 12305;
pub const SO_REUSEADDR: u32 = 4;
pub const SO_REUSE_MULTICASTPORT: u32 = 12296;
pub const SO_REUSE_UNICASTPORT: u32 = 12295;
pub const SO_SNDBUF: u32 = 4097;
pub const SO_SNDLOWAT: u32 = 4099;
pub const SO_SNDTIMEO: u32 = 4101;
pub const SO_TYPE: u32 = 4104;
pub const SO_USELOOPBACK: u32 = 64;
pub const SVCID_BTH_PROVIDER: windows_core::GUID = windows_core::GUID::from_u128(0x06aa63e0_7d60_41ff_afb2_3ee6d2d9392d);
pub const ScopeLevelAdmin: SCOPE_LEVEL = 4;
pub const ScopeLevelCount: SCOPE_LEVEL = 16;
pub const ScopeLevelGlobal: SCOPE_LEVEL = 14;
pub const ScopeLevelInterface: SCOPE_LEVEL = 1;
pub const ScopeLevelLink: SCOPE_LEVEL = 2;
pub const ScopeLevelOrganization: SCOPE_LEVEL = 8;
pub const ScopeLevelSite: SCOPE_LEVEL = 5;
pub const ScopeLevelSubnet: SCOPE_LEVEL = 3;
pub const TCP_ATMARK: u32 = 8;
pub const TCP_CONGESTION_ALGORITHM: u32 = 12;
pub const TCP_DELAY_FIN_ACK: u32 = 13;
pub const TCP_EXPEDITED_1122: u32 = 2;
pub const TCP_FAIL_CONNECT_ON_ICMP_ERROR: u32 = 18;
pub const TCP_FASTOPEN: u32 = 15;
pub const TCP_ICMP_ERROR_INFO: u32 = 19;
pub const TCP_KEEPALIVE: u32 = 3;
pub const TCP_KEEPCNT: u32 = 16;
pub const TCP_KEEPIDLE: u32 = 3;
pub const TCP_KEEPINTVL: u32 = 17;
pub const TCP_MAXRT: u32 = 5;
pub const TCP_MAXRTMS: u32 = 14;
pub const TCP_MAXSEG: u32 = 4;
pub const TCP_NODELAY: u32 = 1;
pub const TCP_NOSYNRETRIES: u32 = 9;
pub const TCP_NOURG: u32 = 7;
pub const TCP_OFFLOAD_NOT_PREFERRED: u32 = 1;
pub const TCP_OFFLOAD_NO_PREFERENCE: u32 = 0;
pub const TCP_OFFLOAD_PREFERENCE: u32 = 11;
pub const TCP_OFFLOAD_PREFERRED: u32 = 2;
pub const TCP_STDURG: u32 = 6;
pub const TCP_TIMESTAMPS: u32 = 10;
pub const UDP_CHECKSUM_COVERAGE: u32 = 20;
pub const UDP_COALESCED_INFO: u32 = 3;
pub const UDP_NOCHECKSUM: u32 = 1;
pub const UDP_RECV_MAX_COALESCED_SIZE: u32 = 3;
pub const UDP_SEND_MSG_SIZE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WSABUF {
    pub len: u32,
    pub buf: *mut i8,
}
impl Default for WSABUF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSACMSGHDR {
    pub cmsg_len: usize,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSAMSG {
    pub name: LPSOCKADDR,
    pub namelen: i32,
    pub lpBuffers: LPWSABUF,
    pub dwBufferCount: u32,
    pub Control: WSABUF,
    pub dwFlags: u32,
}
pub const WSK_SO_BASE: u32 = 16384;
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
#[derive(Clone, Copy)]
pub union sockaddr_gen {
    pub Address: SOCKADDR,
    pub AddressIn: SOCKADDR_IN,
    pub AddressIn6: sockaddr_in6_old,
}
#[cfg(all(feature = "in6addr", feature = "inaddr"))]
impl Default for sockaddr_gen {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "in6addr")]
#[derive(Clone, Copy)]
pub struct sockaddr_in6_old {
    pub sin6_family: i16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: super::in6addr::IN6_ADDR,
}
#[cfg(feature = "in6addr")]
impl Default for sockaddr_in6_old {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct socklen_t(pub i32);
