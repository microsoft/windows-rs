#[cfg(all(feature = "Win32_guiddef", feature = "Win32_ws2def"))]
windows_link::link!("ws2_32.dll" "system" fn FreeAddrInfoEx(paddrinfoex : *const super::ws2def::ADDRINFOEXA));
#[cfg(all(feature = "Win32_guiddef", feature = "Win32_ws2def"))]
windows_link::link!("ws2_32.dll" "system" fn FreeAddrInfoExW(paddrinfoex : *const super::ws2def::ADDRINFOEXW));
#[cfg(feature = "Win32_ws2def")]
windows_link::link!("ws2_32.dll" "system" fn FreeAddrInfoW(paddrinfo : *const super::ws2def::ADDRINFOW));
#[cfg(all(feature = "Win32_guiddef", feature = "Win32_minwinbase", feature = "Win32_winnt", feature = "Win32_winsock2", feature = "Win32_ws2def"))]
windows_link::link!("ws2_32.dll" "system" fn GetAddrInfoExA(pname : windows_sys::core::PCSTR, pservicename : windows_sys::core::PCSTR, dwnamespace : u32, lpnspid : *const windows_sys::core::GUID, hints : *const super::ws2def::ADDRINFOEXA, ppresult : *mut super::ws2def::PADDRINFOEXA, timeout : *const super::winsock2::timeval, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpcompletionroutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle : *mut super::winnt::HANDLE) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ws2_32.dll" "system" fn GetAddrInfoExCancel(lphandle : *const super::winnt::HANDLE) -> i32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("ws2_32.dll" "system" fn GetAddrInfoExOverlappedResult(lpoverlapped : *const super::minwinbase::OVERLAPPED) -> i32);
#[cfg(all(feature = "Win32_guiddef", feature = "Win32_minwinbase", feature = "Win32_winnt", feature = "Win32_winsock2", feature = "Win32_ws2def"))]
windows_link::link!("ws2_32.dll" "system" fn GetAddrInfoExW(pname : windows_sys::core::PCWSTR, pservicename : windows_sys::core::PCWSTR, dwnamespace : u32, lpnspid : *const windows_sys::core::GUID, hints : *const super::ws2def::ADDRINFOEXW, ppresult : *mut super::ws2def::PADDRINFOEXW, timeout : *const super::winsock2::timeval, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpcompletionroutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE, lphandle : *mut super::winnt::HANDLE) -> i32);
#[cfg(feature = "Win32_ws2def")]
windows_link::link!("ws2_32.dll" "system" fn GetAddrInfoW(pnodename : windows_sys::core::PCWSTR, pservicename : windows_sys::core::PCWSTR, phints : *const super::ws2def::ADDRINFOW, ppresult : *mut super::ws2def::PADDRINFOW) -> i32);
#[cfg(feature = "Win32_ws2def")]
windows_link::link!("ws2_32.dll" "system" fn GetNameInfoW(psockaddr : *const super::ws2def::SOCKADDR, sockaddrlength : socklen_t, pnodebuffer : *mut u16, nodebuffersize : u32, pservicebuffer : *mut u16, servicebuffersize : u32, flags : i32) -> i32);
windows_link::link!("ws2_32.dll" "system" fn InetNtopW(family : i32, paddr : *const core::ffi::c_void, pstringbuf : windows_sys::core::PWSTR, stringbufsize : usize) -> windows_sys::core::PCWSTR);
windows_link::link!("ws2_32.dll" "system" fn InetPtonW(family : i32, pszaddrstring : windows_sys::core::PCWSTR, paddrbuf : *mut core::ffi::c_void) -> i32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt", feature = "Win32_winsock2", feature = "Win32_ws2def", feature = "Win32_wtypesbase"))]
windows_link::link!("ws2_32.dll" "system" fn SetAddrInfoExA(pname : windows_sys::core::PCSTR, pservicename : windows_sys::core::PCSTR, paddresses : *const super::ws2def::SOCKET_ADDRESS, dwaddresscount : u32, lpblob : *const super::wtypesbase::BLOB, dwflags : u32, dwnamespace : u32, lpnspid : *const windows_sys::core::GUID, timeout : *const super::winsock2::timeval, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpcompletionroutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle : *mut super::winnt::HANDLE) -> i32);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt", feature = "Win32_winsock2", feature = "Win32_ws2def", feature = "Win32_wtypesbase"))]
windows_link::link!("ws2_32.dll" "system" fn SetAddrInfoExW(pname : windows_sys::core::PCWSTR, pservicename : windows_sys::core::PCWSTR, paddresses : *const super::ws2def::SOCKET_ADDRESS, dwaddresscount : u32, lpblob : *const super::wtypesbase::BLOB, dwflags : u32, dwnamespace : u32, lpnspid : *const windows_sys::core::GUID, timeout : *const super::winsock2::timeval, lpoverlapped : *const super::minwinbase::OVERLAPPED, lpcompletionroutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle : *mut super::winnt::HANDLE) -> i32);
#[cfg(feature = "Win32_ws2def")]
windows_link::link!("ws2_32.dll" "system" fn freeaddrinfo(paddrinfo : *const super::ws2def::ADDRINFOA));
#[cfg(feature = "Win32_ws2def")]
windows_link::link!("ws2_32.dll" "system" fn getaddrinfo(pnodename : windows_sys::core::PCSTR, pservicename : windows_sys::core::PCSTR, phints : *const super::ws2def::ADDRINFOA, ppresult : *mut super::ws2def::PADDRINFOA) -> i32);
#[cfg(feature = "Win32_ws2def")]
windows_link::link!("ws2_32.dll" "system" fn getnameinfo(psockaddr : *const super::ws2def::SOCKADDR, sockaddrlength : socklen_t, pnodebuffer : *mut i8, nodebuffersize : u32, pservicebuffer : *mut i8, servicebuffersize : u32, flags : i32) -> i32);
windows_link::link!("ws2_32.dll" "system" fn inet_ntop(family : i32, paddr : *const core::ffi::c_void, pstringbuf : windows_sys::core::PSTR, stringbufsize : usize) -> windows_sys::core::PCSTR);
windows_link::link!("ws2_32.dll" "system" fn inet_pton(family : i32, pszaddrstring : windows_sys::core::PCSTR, paddrbuf : *mut core::ffi::c_void) -> i32);
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
pub type LPADDRINFO = *mut super::ws2def::ADDRINFOA;
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
pub type LPLOOKUPSERVICE_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(dwerror: u32, dwbytes: u32, lpoverlapped: *const super::minwinbase::OVERLAPPED)>;
#[cfg(all(feature = "Win32_guiddef", feature = "Win32_ws2def"))]
pub type PADDRINFOEX = *mut super::ws2def::ADDRINFOEXA;
#[cfg(feature = "Win32_ws2def")]
pub type PADDRINFOT = *mut super::ws2def::ADDRINFOA;
pub const UDP_CHECKSUM_COVERAGE: u32 = 20;
pub const UDP_NOCHECKSUM: u32 = 1;
pub type socklen_t = i32;
