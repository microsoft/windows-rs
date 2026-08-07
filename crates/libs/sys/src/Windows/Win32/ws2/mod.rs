#[cfg(feature = "guiddef")]
windows_link::link!("ws2_32.dll" "system" fn FreeAddrInfoEx(paddrinfoex : *const ADDRINFOEXA));
#[cfg(feature = "guiddef")]
windows_link::link!("ws2_32.dll" "system" fn FreeAddrInfoExW(paddrinfoex : *const ADDRINFOEXW));
windows_link::link!("ws2_32.dll" "system" fn FreeAddrInfoW(paddrinfo : *const ADDRINFOW));
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "winnt", feature = "winsock2"))]
windows_link::link!("ws2_32.dll" "system" fn GetAddrInfoExA(pname : windows_sys::core::PCSTR, pservicename : windows_sys::core::PCSTR, dwnamespace : u32, lpnspid : *const windows_sys::core::GUID, hints : *const ADDRINFOEXA, ppresult : *mut PADDRINFOEXA, timeout : *const super::timeval, lpoverlapped : *const super::OVERLAPPED, lpcompletionroutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle : *mut super::HANDLE) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("ws2_32.dll" "system" fn GetAddrInfoExCancel(lphandle : *const super::HANDLE) -> i32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("ws2_32.dll" "system" fn GetAddrInfoExOverlappedResult(lpoverlapped : *const super::OVERLAPPED) -> i32);
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "winnt", feature = "winsock2"))]
windows_link::link!("ws2_32.dll" "system" fn GetAddrInfoExW(pname : windows_sys::core::PCWSTR, pservicename : windows_sys::core::PCWSTR, dwnamespace : u32, lpnspid : *const windows_sys::core::GUID, hints : *const ADDRINFOEXW, ppresult : *mut PADDRINFOEXW, timeout : *const super::timeval, lpoverlapped : *const super::OVERLAPPED, lpcompletionroutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE, lphandle : *mut super::HANDLE) -> i32);
windows_link::link!("ws2_32.dll" "system" fn GetAddrInfoW(pnodename : windows_sys::core::PCWSTR, pservicename : windows_sys::core::PCWSTR, phints : *const ADDRINFOW, ppresult : *mut PADDRINFOW) -> i32);
windows_link::link!("ws2_32.dll" "system" fn GetNameInfoW(psockaddr : *const SOCKADDR, sockaddrlength : socklen_t, pnodebuffer : *mut u16, nodebuffersize : u32, pservicebuffer : *mut u16, servicebuffersize : u32, flags : i32) -> i32);
windows_link::link!("ws2_32.dll" "system" fn InetNtopW(family : i32, paddr : *const core::ffi::c_void, pstringbuf : windows_sys::core::PWSTR, stringbufsize : usize) -> windows_sys::core::PCWSTR);
windows_link::link!("ws2_32.dll" "system" fn InetPtonW(family : i32, pszaddrstring : windows_sys::core::PCWSTR, paddrbuf : *mut core::ffi::c_void) -> i32);
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "winsock2", feature = "wtypesbase"))]
windows_link::link!("ws2_32.dll" "system" fn SetAddrInfoExA(pname : windows_sys::core::PCSTR, pservicename : windows_sys::core::PCSTR, paddresses : *const SOCKET_ADDRESS, dwaddresscount : u32, lpblob : *const super::BLOB, dwflags : u32, dwnamespace : u32, lpnspid : *const windows_sys::core::GUID, timeout : *const super::timeval, lpoverlapped : *const super::OVERLAPPED, lpcompletionroutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle : *mut super::HANDLE) -> i32);
#[cfg(all(feature = "minwinbase", feature = "winnt", feature = "winsock2", feature = "wtypesbase"))]
windows_link::link!("ws2_32.dll" "system" fn SetAddrInfoExW(pname : windows_sys::core::PCWSTR, pservicename : windows_sys::core::PCWSTR, paddresses : *const SOCKET_ADDRESS, dwaddresscount : u32, lpblob : *const super::BLOB, dwflags : u32, dwnamespace : u32, lpnspid : *const windows_sys::core::GUID, timeout : *const super::timeval, lpoverlapped : *const super::OVERLAPPED, lpcompletionroutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE, lpnamehandle : *mut super::HANDLE) -> i32);
windows_link::link!("ws2_32.dll" "system" fn freeaddrinfo(paddrinfo : *const ADDRINFOA));
windows_link::link!("ws2_32.dll" "system" fn getaddrinfo(pnodename : windows_sys::core::PCSTR, pservicename : windows_sys::core::PCSTR, phints : *const ADDRINFOA, ppresult : *mut PADDRINFOA) -> i32);
windows_link::link!("ws2_32.dll" "system" fn getnameinfo(psockaddr : *const SOCKADDR, sockaddrlength : socklen_t, pnodebuffer : *mut i8, nodebuffersize : u32, pservicebuffer : *mut i8, servicebuffersize : u32, flags : i32) -> i32);
windows_link::link!("ws2_32.dll" "system" fn inet_ntop(family : i32, paddr : *const core::ffi::c_void, pstringbuf : windows_sys::core::PSTR, stringbufsize : usize) -> windows_sys::core::PCSTR);
windows_link::link!("ws2_32.dll" "system" fn inet_pton(family : i32, pszaddrstring : windows_sys::core::PCSTR, paddrbuf : *mut core::ffi::c_void) -> i32);
pub type ADDRESS_FAMILY = u16;
pub type ADDRINFO = ADDRINFOA;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[cfg(feature = "guiddef")]
pub type ADDRINFOEX = ADDRINFOEXA;
#[repr(C)]
#[cfg(feature = "guiddef")]
#[derive(Clone, Copy, Default)]
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
    pub ai_provider: super::LPGUID,
    pub ai_next: *mut Self,
    pub ai_version: i32,
    pub ai_fqdn: *mut i8,
}
#[repr(C)]
#[cfg(feature = "guiddef")]
#[derive(Clone, Copy, Default)]
pub struct ADDRINFOEX2W {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: windows_sys::core::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: super::LPGUID,
    pub ai_next: *mut Self,
    pub ai_version: i32,
    pub ai_fqdn: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "guiddef")]
#[derive(Clone, Copy, Default)]
pub struct ADDRINFOEX3 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: windows_sys::core::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: super::LPGUID,
    pub ai_next: *mut Self,
    pub ai_version: i32,
    pub ai_fqdn: windows_sys::core::PWSTR,
    pub ai_interfaceindex: i32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct ADDRINFOEX4 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: windows_sys::core::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut windows_sys::core::GUID,
    pub ai_next: *mut Self,
    pub ai_version: i32,
    pub ai_fqdn: windows_sys::core::PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: super::HANDLE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct ADDRINFOEX5 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: windows_sys::core::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut windows_sys::core::GUID,
    pub ai_next: *mut Self,
    pub ai_version: i32,
    pub ai_fqdn: windows_sys::core::PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: super::HANDLE,
    pub ai_ttl: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct ADDRINFOEX6 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: windows_sys::core::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut windows_sys::core::GUID,
    pub ai_next: *mut ADDRINFOEX5,
    pub ai_version: i32,
    pub ai_fqdn: windows_sys::core::PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: super::HANDLE,
    pub ai_ttl: u32,
    pub ai_numservers: u32,
    pub ai_servers: *mut ADDRINFO_DNS_SERVER,
    pub ai_responseflags: u64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct ADDRINFOEX7 {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: windows_sys::core::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: *mut windows_sys::core::GUID,
    pub ai_next: *mut Self,
    pub ai_version: i32,
    pub ai_fqdn: windows_sys::core::PWSTR,
    pub ai_interfaceindex: i32,
    pub ai_resolutionhandle: super::HANDLE,
    pub ai_ttl: u32,
    pub ai_numservers: u32,
    pub ai_servers: *mut ADDRINFO_DNS_SERVER,
    pub ai_responseflags: u64,
    pub ai_extraflags: u64,
}
#[repr(C)]
#[cfg(feature = "guiddef")]
#[derive(Clone, Copy, Default)]
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
    pub ai_provider: super::LPGUID,
    pub ai_next: *mut Self,
}
#[repr(C)]
#[cfg(feature = "guiddef")]
#[derive(Clone, Copy, Default)]
pub struct ADDRINFOEXW {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: windows_sys::core::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_blob: *mut core::ffi::c_void,
    pub ai_bloblen: usize,
    pub ai_provider: super::LPGUID,
    pub ai_next: *mut Self,
}
pub const ADDRINFOEX_VERSION_2: i32 = 2;
pub const ADDRINFOEX_VERSION_3: i32 = 3;
pub const ADDRINFOEX_VERSION_4: i32 = 4;
pub const ADDRINFOEX_VERSION_5: i32 = 5;
pub const ADDRINFOEX_VERSION_6: i32 = 6;
pub const ADDRINFOEX_VERSION_7: i32 = 7;
pub type ADDRINFOT = ADDRINFOA;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ADDRINFOW {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: windows_sys::core::PWSTR,
    pub ai_addr: *mut SOCKADDR,
    pub ai_next: *mut Self,
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
    pub ai_template: windows_sys::core::PWSTR,
    pub ai_hostname: windows_sys::core::PWSTR,
}
impl Default for ADDRINFO_DNS_SERVER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AF_12844: i32 = 25;
pub const AF_APPLETALK: i32 = 16;
pub const AF_ATM: i32 = 22;
pub const AF_BAN: i32 = 21;
pub const AF_BTH: i32 = 32;
pub const AF_CCITT: i32 = 10;
pub const AF_CHAOS: i32 = 5;
pub const AF_CLUSTER: i32 = 24;
pub const AF_DATAKIT: i32 = 9;
pub const AF_DECnet: i32 = 12;
pub const AF_DLI: i32 = 13;
pub const AF_ECMA: i32 = 8;
pub const AF_FIREFOX: i32 = 19;
pub const AF_HYLINK: i32 = 15;
pub const AF_HYPERV: i32 = 34;
pub const AF_ICLFXBM: i32 = 31;
pub const AF_IMPLINK: i32 = 3;
pub const AF_INET: i32 = 2;
pub const AF_INET6: i32 = 23;
pub const AF_IPX: i32 = 6;
pub const AF_IRDA: i32 = 26;
pub const AF_ISO: i32 = 7;
pub const AF_LAT: i32 = 14;
pub const AF_LINK: i32 = 33;
pub const AF_MAX: i32 = 35;
pub const AF_NETBIOS: i32 = 17;
pub const AF_NETDES: i32 = 28;
pub const AF_NS: i32 = 6;
pub const AF_OSI: i32 = 7;
pub const AF_PUP: i32 = 4;
pub const AF_SNA: i32 = 11;
pub const AF_TCNMESSAGE: i32 = 30;
pub const AF_TCNPROCESS: i32 = 29;
pub const AF_UNIX: i32 = 1;
pub const AF_UNKNOWN1: i32 = 20;
pub const AF_UNSPEC: i32 = 0;
pub const AF_VOICEVIEW: i32 = 18;
pub const AI_ADDRCONFIG: i32 = 1024;
pub const AI_ALL: i32 = 256;
pub const AI_BYPASS_DNS_CACHE: i32 = 64;
pub const AI_CANONNAME: i32 = 2;
pub const AI_DISABLE_IDN_ENCODING: i32 = 524288;
pub const AI_DNS_ONLY: i32 = 16;
pub const AI_DNS_RESPONSE_HOSTFILE: i32 = 2;
pub const AI_DNS_RESPONSE_SECURE: i32 = 1;
pub const AI_DNS_SERVER_TYPE_DOH: i32 = 2;
pub const AI_DNS_SERVER_TYPE_DOT: i32 = 3;
pub const AI_DNS_SERVER_TYPE_UDP: i32 = 1;
pub const AI_DNS_SERVER_UDP_FALLBACK: i32 = 1;
pub const AI_EXCLUSIVE_CUSTOM_SERVERS: i32 = 2097152;
pub const AI_EXTENDED: u32 = 2147483648;
pub const AI_EXTRA_DNSSEC_REQUIRED: i32 = 1;
pub const AI_FILESERVER: i32 = 262144;
pub const AI_FORCE_CLEAR_TEXT: i32 = 32;
pub const AI_FQDN: i32 = 131072;
pub const AI_NON_AUTHORITATIVE: i32 = 16384;
pub const AI_NUMERICHOST: i32 = 4;
pub const AI_NUMERICSERV: i32 = 8;
pub const AI_PASSIVE: i32 = 1;
pub const AI_REQUIRE_SECURE: i32 = 536870912;
pub const AI_RESOLUTION_HANDLE: i32 = 1073741824;
pub const AI_RETURN_PREFERRED_NAMES: i32 = 65536;
pub const AI_RETURN_RESPONSE_FLAGS: i32 = 268435456;
pub const AI_RETURN_TTL: i32 = 128;
pub const AI_SECURE: i32 = 32768;
pub const AI_SECURE_WITH_FALLBACK: i32 = 1048576;
pub const AI_V4MAPPED: i32 = 2048;
pub type BTHNS_INQUIRYBLOB = BTH_QUERY_DEVICE;
#[cfg(feature = "bthsdpdef")]
pub type BTHNS_RESTRICTIONBLOB = BTH_QUERY_SERVICE;
pub const BTHNS_RESULT_DEVICE_AUTHENTICATED: i32 = 262144;
pub const BTHNS_RESULT_DEVICE_CONNECTED: i32 = 65536;
pub const BTHNS_RESULT_DEVICE_REMEMBERED: i32 = 131072;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type BTHNS_SETBLOB = BTH_SET_SERVICE;
pub const BTHPROTO_L2CAP: i32 = 256;
pub const BTHPROTO_RFCOMM: i32 = 3;
pub const BTH_ADDR_STRING_SIZE: i32 = 12;
#[repr(C, packed(1))]
#[cfg(feature = "bthdef")]
#[derive(Clone, Copy, Default)]
pub struct BTH_INFO_REQ {
    pub btAddr: super::BTH_ADDR,
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
    pub btAddr: super::BTH_ADDR,
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
#[derive(Clone, Copy)]
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
    pub uuids: [super::SdpQueryUuid; 12],
    pub numRange: u32,
    pub pRange: [super::SdpAttributeRange; 1],
}
#[cfg(feature = "bthsdpdef")]
impl Default for BTH_QUERY_SERVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const BTH_SDP_VERSION: i32 = 1;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct BTH_SET_SERVICE {
    pub pSdpVersion: super::PULONG,
    pub pRecordHandle: *mut super::HANDLE,
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
pub const BT_PORT_DYN_FIRST: i32 = 4097;
pub const BT_PORT_MAX: i32 = 65535;
pub const BT_PORT_MIN: i32 = 1;
pub type CMSGHDR = WSACMSGHDR;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CSADDR_INFO {
    pub LocalAddr: SOCKET_ADDRESS,
    pub RemoteAddr: SOCKET_ADDRESS,
    pub iSocketType: i32,
    pub iProtocol: i32,
}
pub const EAI_AGAIN: i32 = 11002;
pub const EAI_BADFLAGS: i32 = 10022;
pub const EAI_FAIL: i32 = 11003;
pub const EAI_FAMILY: i32 = 10047;
pub const EAI_IPSECPOLICY: i32 = 11033;
pub const EAI_MEMORY: i32 = 8;
pub const EAI_NODATA: i32 = 11001;
pub const EAI_NONAME: i32 = 11001;
pub const EAI_NOSECURENAME: i32 = 11032;
pub const EAI_SERVICE: i32 = 10109;
pub const EAI_SOCKTYPE: i32 = 10044;
pub const GAI_STRERROR_BUFFER_SIZE: i32 = 1024;
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Default)]
pub struct GROUP_REQ {
    pub gr_interface: u32,
    pub gr_group: SOCKADDR_STORAGE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const IFF_BROADCAST: i32 = 2;
pub const IFF_LOOPBACK: i32 = 4;
pub const IFF_MULTICAST: i32 = 16;
pub const IFF_POINTTOPOINT: i32 = 8;
pub const IFF_UP: i32 = 1;
pub const IN6ADDR_6TO4PREFIX_LENGTH: i32 = 16;
pub const IN6ADDR_LINKLOCALPREFIX_LENGTH: i32 = 64;
pub const IN6ADDR_MULTICASTPREFIX_LENGTH: i32 = 8;
pub const IN6ADDR_SOLICITEDNODEMULTICASTPREFIX_LENGTH: i32 = 104;
pub const IN6ADDR_TEREDOPREFIX_LENGTH: i32 = 32;
pub const IN6ADDR_V4MAPPEDPREFIX_LENGTH: i32 = 96;
#[repr(C)]
#[cfg(feature = "in6addr")]
#[derive(Clone, Copy)]
pub struct IN6_PKTINFO {
    pub ipi6_addr: super::IN6_ADDR,
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
pub const INADDR_LOOPBACK: i32 = 2130706433;
pub const INADDR_NONE: u32 = 4294967295;
pub const INET6_ADDRSTRLEN: i32 = 65;
pub const INET_ADDRSTRLEN: i32 = 22;
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
#[derive(Clone, Copy, Default)]
pub struct INTERFACE_INFO_EX {
    pub iiFlags: u32,
    pub iiAddress: SOCKET_ADDRESS,
    pub iiBroadcastAddress: SOCKET_ADDRESS,
    pub iiNetmask: SOCKET_ADDRESS,
}
pub const IN_CLASSA_HOST: i32 = 16777215;
pub const IN_CLASSA_MAX: i32 = 128;
pub const IN_CLASSA_NET: u32 = 4278190080;
pub const IN_CLASSA_NSHIFT: i32 = 24;
pub const IN_CLASSB_HOST: i32 = 65535;
pub const IN_CLASSB_MAX: i32 = 65536;
pub const IN_CLASSB_NET: u32 = 4294901760;
pub const IN_CLASSB_NSHIFT: i32 = 16;
pub const IN_CLASSC_HOST: i32 = 255;
pub const IN_CLASSC_NET: u32 = 4294967040;
pub const IN_CLASSC_NSHIFT: i32 = 8;
pub const IN_CLASSD_HOST: i32 = 268435455;
pub const IN_CLASSD_NET: u32 = 4026531840;
pub const IN_CLASSD_NSHIFT: i32 = 28;
#[repr(C)]
#[cfg(feature = "inaddr")]
#[derive(Clone, Copy)]
pub struct IN_PKTINFO {
    pub ipi_addr: super::IN_ADDR,
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
#[derive(Clone, Copy, Default)]
pub struct IN_RECVERR {
    pub protocol: IPPROTO,
    pub info: u32,
    pub r#type: u8,
    pub code: u8,
}
pub const IOCPARM_MASK: i32 = 127;
pub const IOC_IN: u32 = 2147483648;
pub const IOC_INOUT: u32 = 3221225472;
pub const IOC_OUT: i32 = 1073741824;
pub const IOC_PROTOCOL: i32 = 268435456;
pub const IOC_UNIX: i32 = 0;
pub const IOC_VENDOR: i32 = 402653184;
pub const IOC_VOID: i32 = 536870912;
pub const IOC_WS2: i32 = 134217728;
pub const IOC_WSK: i32 = 251658240;
pub const IP6T_SO_ORIGINAL_DST: i32 = 12303;
pub const IPPORT_BIFFUDP: i32 = 512;
pub const IPPORT_CHARGEN: i32 = 19;
pub const IPPORT_CMDSERVER: i32 = 514;
pub const IPPORT_DAYTIME: i32 = 13;
pub const IPPORT_DISCARD: i32 = 9;
pub const IPPORT_DYNAMIC_MAX: i32 = 65535;
pub const IPPORT_DYNAMIC_MIN: i32 = 49152;
pub const IPPORT_ECHO: i32 = 7;
pub const IPPORT_EFSSERVER: i32 = 520;
pub const IPPORT_EPMAP: i32 = 135;
pub const IPPORT_EXECSERVER: i32 = 512;
pub const IPPORT_FINGER: i32 = 79;
pub const IPPORT_FTP: i32 = 21;
pub const IPPORT_FTP_DATA: i32 = 20;
pub const IPPORT_HTTPS: i32 = 443;
pub const IPPORT_IMAP: i32 = 143;
pub const IPPORT_IMAP3: i32 = 220;
pub const IPPORT_LDAP: i32 = 389;
pub const IPPORT_LOGINSERVER: i32 = 513;
pub const IPPORT_MICROSOFT_DS: i32 = 445;
pub const IPPORT_MSP: i32 = 18;
pub const IPPORT_MTP: i32 = 57;
pub const IPPORT_NAMESERVER: i32 = 42;
pub const IPPORT_NETBIOS_DGM: i32 = 138;
pub const IPPORT_NETBIOS_NS: i32 = 137;
pub const IPPORT_NETBIOS_SSN: i32 = 139;
pub const IPPORT_NETSTAT: i32 = 15;
pub const IPPORT_NTP: i32 = 123;
pub const IPPORT_POP3: i32 = 110;
pub const IPPORT_QOTD: i32 = 17;
pub const IPPORT_REGISTERED_MAX: i32 = 49151;
pub const IPPORT_REGISTERED_MIN: i32 = 1024;
pub const IPPORT_RESERVED: i32 = 1024;
pub const IPPORT_RJE: i32 = 77;
pub const IPPORT_ROUTESERVER: i32 = 520;
pub const IPPORT_SMTP: i32 = 25;
pub const IPPORT_SNMP: i32 = 161;
pub const IPPORT_SNMP_TRAP: i32 = 162;
pub const IPPORT_SUPDUP: i32 = 95;
pub const IPPORT_SYSTAT: i32 = 11;
pub const IPPORT_TCPMUX: i32 = 1;
pub const IPPORT_TELNET: i32 = 23;
pub const IPPORT_TFTP: i32 = 69;
pub const IPPORT_TIMESERVER: i32 = 37;
pub const IPPORT_TTYLINK: i32 = 87;
pub const IPPORT_WHOIS: i32 = 43;
pub const IPPORT_WHOSERVER: i32 = 513;
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
pub const IPPROTO_IP: i32 = 0;
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
#[cfg(target_arch = "x86")]
pub const IPV6_ADDRESS_BITS: u32 = 128;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const IPV6_ADDRESS_BITS: u64 = 128;
pub const IPV6_ADD_IFLIST: i32 = 29;
pub const IPV6_ADD_MEMBERSHIP: i32 = 12;
pub const IPV6_CHECKSUM: i32 = 26;
pub const IPV6_DEL_IFLIST: i32 = 30;
pub const IPV6_DONTFRAG: i32 = 14;
pub const IPV6_DROP_MEMBERSHIP: i32 = 13;
pub const IPV6_ECN: i32 = 50;
pub const IPV6_GET_IFLIST: i32 = 33;
pub const IPV6_HDRINCL: i32 = 2;
pub const IPV6_HOPLIMIT: i32 = 21;
pub const IPV6_HOPOPTS: i32 = 1;
pub const IPV6_IFLIST: i32 = 28;
pub const IPV6_JOIN_GROUP: i32 = 12;
pub const IPV6_LEAVE_GROUP: i32 = 13;
#[repr(C)]
#[cfg(feature = "in6addr")]
#[derive(Clone, Copy)]
pub struct IPV6_MREQ {
    pub ipv6mr_multiaddr: super::IN6_ADDR,
    pub ipv6mr_interface: u32,
}
#[cfg(feature = "in6addr")]
impl Default for IPV6_MREQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IPV6_MTU: i32 = 72;
pub const IPV6_MTU_DISCOVER: i32 = 71;
pub const IPV6_MULTICAST_HOPS: i32 = 10;
pub const IPV6_MULTICAST_IF: i32 = 9;
pub const IPV6_MULTICAST_LOOP: i32 = 11;
pub const IPV6_NRT_INTERFACE: i32 = 74;
pub const IPV6_PKTINFO: i32 = 19;
pub const IPV6_PKTINFO_EX: i32 = 51;
pub const IPV6_PROTECTION_LEVEL: i32 = 23;
pub const IPV6_RECVDSTADDR: i32 = 25;
pub const IPV6_RECVECN: i32 = 50;
pub const IPV6_RECVERR: i32 = 75;
pub const IPV6_RECVIF: i32 = 24;
pub const IPV6_RECVRTHDR: i32 = 38;
pub const IPV6_RECVTCLASS: i32 = 40;
pub const IPV6_RTHDR: i32 = 32;
pub const IPV6_TCLASS: i32 = 39;
pub const IPV6_UNICAST_HOPS: i32 = 4;
pub const IPV6_UNICAST_IF: i32 = 31;
pub const IPV6_USER_MTU: i32 = 76;
pub const IPV6_V6ONLY: i32 = 27;
pub const IPV6_WFP_REDIRECT_CONTEXT: i32 = 70;
pub const IPV6_WFP_REDIRECT_RECORDS: i32 = 60;
pub const IP_ADD_IFLIST: i32 = 29;
pub const IP_ADD_MEMBERSHIP: i32 = 12;
pub const IP_ADD_SOURCE_MEMBERSHIP: i32 = 15;
pub const IP_BLOCK_SOURCE: i32 = 17;
pub const IP_DEL_IFLIST: i32 = 30;
pub const IP_DONTFRAGMENT: i32 = 14;
pub const IP_DROP_MEMBERSHIP: i32 = 13;
pub const IP_DROP_SOURCE_MEMBERSHIP: i32 = 16;
pub const IP_ECN: i32 = 50;
pub const IP_GET_IFLIST: i32 = 33;
pub const IP_HDRINCL: i32 = 2;
pub const IP_HOPLIMIT: i32 = 21;
pub const IP_IFLIST: i32 = 28;
#[repr(C)]
#[cfg(feature = "inaddr")]
#[derive(Clone, Copy)]
pub struct IP_MREQ {
    pub imr_multiaddr: super::IN_ADDR,
    pub imr_interface: super::IN_ADDR,
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
    pub imr_multiaddr: super::IN_ADDR,
    pub imr_sourceaddr: super::IN_ADDR,
    pub imr_interface: super::IN_ADDR,
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
    pub imsf_multiaddr: super::IN_ADDR,
    pub imsf_interface: super::IN_ADDR,
    pub imsf_fmode: MULTICAST_MODE_TYPE,
    pub imsf_numsrc: u32,
    pub imsf_slist: [super::IN_ADDR; 1],
}
#[cfg(feature = "inaddr")]
impl Default for IP_MSFILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP_MTU: i32 = 73;
pub const IP_MTU_DISCOVER: i32 = 71;
pub const IP_MULTICAST_IF: i32 = 9;
pub const IP_MULTICAST_LOOP: i32 = 11;
pub const IP_MULTICAST_TTL: i32 = 10;
pub const IP_NRT_INTERFACE: i32 = 74;
pub const IP_OPTIONS: i32 = 1;
pub const IP_ORIGINAL_ARRIVAL_IF: i32 = 47;
pub const IP_PKTINFO: i32 = 19;
pub const IP_PKTINFO_EX: i32 = 51;
pub const IP_PMTUDISC_DO: PMTUD_STATE = 1;
pub const IP_PMTUDISC_DONT: PMTUD_STATE = 2;
pub const IP_PMTUDISC_MAX: PMTUD_STATE = 4;
pub const IP_PMTUDISC_NOT_SET: PMTUD_STATE = 0;
pub const IP_PMTUDISC_PROBE: PMTUD_STATE = 3;
pub const IP_PROTECTION_LEVEL: i32 = 23;
pub const IP_RECEIVE_BROADCAST: i32 = 22;
pub const IP_RECVDSTADDR: i32 = 25;
pub const IP_RECVECN: i32 = 50;
pub const IP_RECVERR: i32 = 75;
pub const IP_RECVIF: i32 = 24;
pub const IP_RECVRTHDR: i32 = 38;
pub const IP_RECVTCLASS: i32 = 40;
pub const IP_RECVTOS: i32 = 40;
pub const IP_RECVTTL: i32 = 21;
pub const IP_RTHDR: i32 = 32;
pub const IP_TCLASS: i32 = 39;
pub const IP_TOS: i32 = 3;
pub const IP_TTL: i32 = 4;
pub const IP_UNBLOCK_SOURCE: i32 = 18;
pub const IP_UNICAST_IF: i32 = 31;
pub const IP_UNSPECIFIED_HOP_LIMIT: i32 = -1;
pub const IP_UNSPECIFIED_TYPE_OF_SERVICE: i32 = -1;
pub const IP_USER_MTU: i32 = 76;
pub const IP_WFP_REDIRECT_CONTEXT: i32 = 70;
pub const IP_WFP_REDIRECT_RECORDS: i32 = 60;
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
pub type LPLOOKUPSERVICE_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(dwerror: u32, dwbytes: u32, lpoverlapped: *const super::OVERLAPPED)>;
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
pub const MCAST_BLOCK_SOURCE: i32 = 43;
pub const MCAST_EXCLUDE: MULTICAST_MODE_TYPE = 1;
pub const MCAST_INCLUDE: MULTICAST_MODE_TYPE = 0;
pub const MCAST_JOIN_GROUP: i32 = 41;
pub const MCAST_JOIN_SOURCE_GROUP: i32 = 45;
pub const MCAST_LEAVE_GROUP: i32 = 42;
pub const MCAST_LEAVE_SOURCE_GROUP: i32 = 46;
pub const MCAST_UNBLOCK_SOURCE: i32 = 44;
pub const MSC_BREAK_BIT: i32 = 2;
pub const MSC_DV_BIT: i32 = 128;
pub const MSC_FC_BIT: i32 = 2;
pub const MSC_IC_BIT: i32 = 64;
pub const MSC_RESERVED: i32 = 48;
pub const MSC_RTC_BIT: i32 = 4;
pub const MSC_RTR_BIT: i32 = 8;
pub const MSG_BCAST: i32 = 1024;
pub const MSG_CTRUNC: i32 = 512;
pub const MSG_ERRQUEUE: i32 = 4096;
pub const MSG_MCAST: i32 = 2048;
pub const MSG_TRUNC: i32 = 256;
pub type MULTICAST_MODE_TYPE = i32;
pub const NI_DGRAM: i32 = 16;
pub const NI_MAXHOST: i32 = 1025;
pub const NI_MAXSERV: i32 = 32;
pub const NI_NAMEREQD: i32 = 4;
pub const NI_NOFQDN: i32 = 1;
pub const NI_NUMERICHOST: i32 = 2;
pub const NI_NUMERICSERV: i32 = 8;
pub const NS_ALL: i32 = 0;
pub const NS_BTH: i32 = 16;
pub const NS_DHCP: i32 = 6;
pub const NS_DNS: i32 = 12;
pub const NS_EMAIL: i32 = 37;
pub const NS_MS: i32 = 30;
pub const NS_NBP: i32 = 20;
pub const NS_NDS: i32 = 2;
pub const NS_NETBT: i32 = 13;
pub const NS_NETDES: i32 = 60;
pub const NS_NIS: i32 = 41;
pub const NS_NISPLUS: i32 = 42;
pub const NS_NLA: i32 = 15;
pub const NS_NTDS: i32 = 32;
pub const NS_PEER_BROWSE: i32 = 3;
pub const NS_PNRPCLOUD: i32 = 39;
pub const NS_PNRPNAME: i32 = 38;
pub const NS_SAP: i32 = 1;
pub const NS_SLP: i32 = 5;
pub const NS_STDA: i32 = 31;
pub const NS_TCPIP_HOSTS: i32 = 11;
pub const NS_TCPIP_LOCAL: i32 = 10;
pub const NS_WINS: i32 = 14;
pub const NS_WRQ: i32 = 50;
pub const NS_X500: i32 = 40;
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
pub const PROTECTION_LEVEL_EDGERESTRICTED: i32 = 20;
pub const PROTECTION_LEVEL_RESTRICTED: i32 = 30;
pub const PROTECTION_LEVEL_UNRESTRICTED: i32 = 10;
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
pub const RFCOMM_CMD_MSC: i32 = 1;
pub const RFCOMM_CMD_NONE: i32 = 0;
pub const RFCOMM_CMD_RLS: i32 = 2;
pub const RFCOMM_CMD_RPN: i32 = 3;
pub const RFCOMM_CMD_RPN_REQUEST: i32 = 4;
pub const RFCOMM_CMD_RPN_RESPONSE: i32 = 5;
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
pub const RFCOMM_MAX_MTU: i32 = 1011;
pub const RFCOMM_MIN_MTU: i32 = 23;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RFCOMM_MSC_DATA {
    pub Signals: u8,
    pub Break: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RFCOMM_RLS_DATA {
    pub LineStatus: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RFCOMM_RPN_DATA {
    pub Baud: u8,
    pub Data: u8,
    pub FlowControl: u8,
    pub XonChar: u8,
    pub XoffChar: u8,
    pub ParameterMask1: u8,
    pub ParameterMask2: u8,
}
pub const RLS_ERROR: i32 = 1;
pub const RLS_FRAMING: i32 = 8;
pub const RLS_OVERRUN: i32 = 2;
pub const RLS_PARITY: i32 = 4;
pub const RPN_BAUD_115200: i32 = 7;
pub const RPN_BAUD_19200: i32 = 4;
pub const RPN_BAUD_230400: i32 = 8;
pub const RPN_BAUD_2400: i32 = 0;
pub const RPN_BAUD_38400: i32 = 5;
pub const RPN_BAUD_4800: i32 = 1;
pub const RPN_BAUD_57600: i32 = 6;
pub const RPN_BAUD_7200: i32 = 2;
pub const RPN_BAUD_9600: i32 = 3;
pub const RPN_DATA_5: i32 = 0;
pub const RPN_DATA_6: i32 = 1;
pub const RPN_DATA_7: i32 = 2;
pub const RPN_DATA_8: i32 = 3;
pub const RPN_FLOW_RTC_IN: i32 = 16;
pub const RPN_FLOW_RTC_OUT: i32 = 32;
pub const RPN_FLOW_RTR_IN: i32 = 4;
pub const RPN_FLOW_RTR_OUT: i32 = 8;
pub const RPN_FLOW_X_IN: i32 = 1;
pub const RPN_FLOW_X_OUT: i32 = 2;
pub const RPN_PARAM_BAUD: i32 = 1;
pub const RPN_PARAM_DATA: i32 = 2;
pub const RPN_PARAM_PARITY: i32 = 8;
pub const RPN_PARAM_P_TYPE: i32 = 16;
pub const RPN_PARAM_RTC_IN: i32 = 16;
pub const RPN_PARAM_RTC_OUT: i32 = 32;
pub const RPN_PARAM_RTR_IN: i32 = 4;
pub const RPN_PARAM_RTR_OUT: i32 = 8;
pub const RPN_PARAM_STOP: i32 = 4;
pub const RPN_PARAM_XOFF: i32 = 64;
pub const RPN_PARAM_XON: i32 = 32;
pub const RPN_PARAM_X_IN: i32 = 1;
pub const RPN_PARAM_X_OUT: i32 = 2;
pub const RPN_PARITY_EVEN: i32 = 24;
pub const RPN_PARITY_MARK: i32 = 40;
pub const RPN_PARITY_NONE: i32 = 0;
pub const RPN_PARITY_ODD: i32 = 8;
pub const RPN_PARITY_SPACE: i32 = 56;
pub const RPN_STOP_1: i32 = 0;
pub const RPN_STOP_1_5: i32 = 4;
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
#[derive(Clone, Copy, Default)]
pub struct SCOPE_ID_0_0 {
    pub _bitfield: u32,
}
pub type SCOPE_LEVEL = i32;
pub const SDP_DEFAULT_INQUIRY_MAX_RESPONSES: i32 = 255;
pub const SDP_DEFAULT_INQUIRY_SECONDS: i32 = 6;
pub const SDP_MAX_INQUIRY_SECONDS: i32 = 60;
pub const SDP_SERVICE_ATTRIBUTE_REQUEST: i32 = 2;
pub const SDP_SERVICE_SEARCH_ATTRIBUTE_REQUEST: i32 = 3;
pub const SDP_SERVICE_SEARCH_REQUEST: i32 = 1;
pub const SIOCGIPMSFILTER: u32 = 2147775612;
pub const SIOCGMSFILTER: u32 = 2147775615;
pub const SIOCSIPMSFILTER: u32 = 2147775613;
pub const SIOCSMSFILTER: u32 = 2147775614;
pub const SIO_ADDRESS_LIST_CHANGE: i32 = 671088663;
pub const SIO_ADDRESS_LIST_QUERY: i32 = 1207959574;
pub const SIO_ADDRESS_LIST_SORT: u32 = 3355443225;
pub const SIO_ASSOCIATE_HANDLE: u32 = 2281701377;
pub const SIO_BTH_INFO: u32 = 3623878665;
pub const SIO_BTH_PING: u32 = 3623878664;
pub const SIO_ENABLE_CIRCULAR_QUEUEING: i32 = 671088642;
pub const SIO_FIND_ROUTE: i32 = 1207959555;
pub const SIO_FLUSH: i32 = 671088644;
pub const SIO_GET_BROADCAST_ADDRESS: i32 = 1207959557;
pub const SIO_GET_EXTENSION_FUNCTION_POINTER: u32 = 3355443206;
pub const SIO_GET_GROUP_QOS: u32 = 3355443208;
pub const SIO_GET_INTERFACE_LIST: i32 = 1074033791;
pub const SIO_GET_INTERFACE_LIST_EX: i32 = 1074033790;
pub const SIO_GET_MULTICAST_FILTER: u32 = 2147775612;
pub const SIO_GET_MULTIPLE_EXTENSION_FUNCTION_POINTER: u32 = 3355443236;
pub const SIO_GET_QOS: u32 = 3355443207;
pub const SIO_IDEAL_SEND_BACKLOG_CHANGE: i32 = 536900730;
pub const SIO_IDEAL_SEND_BACKLOG_QUERY: i32 = 1074033787;
pub const SIO_MULTICAST_SCOPE: u32 = 2281701386;
pub const SIO_MULTIPOINT_LOOPBACK: u32 = 2281701385;
pub const SIO_QUERY_RSS_PROCESSOR_INFO: i32 = 1207959589;
pub const SIO_QUERY_TARGET_PNP_HANDLE: i32 = 1207959576;
pub const SIO_RESERVED_1: u32 = 2281701402;
pub const SIO_RESERVED_2: u32 = 2281701409;
pub const SIO_RFCOMM_SEND_COMMAND: u32 = 3623878757;
pub const SIO_RFCOMM_SESSION_FLOW_OFF: u32 = 3623878759;
pub const SIO_RFCOMM_TEST: u32 = 3623878760;
pub const SIO_RFCOMM_USECFC: u32 = 3623878761;
pub const SIO_RFCOMM_WAIT_COMMAND: u32 = 3623878758;
pub const SIO_ROUTING_INTERFACE_CHANGE: u32 = 2281701397;
pub const SIO_ROUTING_INTERFACE_QUERY: u32 = 3355443220;
pub const SIO_SET_GROUP_QOS: u32 = 2281701388;
pub const SIO_SET_MULTICAST_FILTER: u32 = 2147775613;
pub const SIO_SET_QOS: u32 = 2281701387;
pub const SIO_TRANSLATE_HANDLE: u32 = 3355443213;
#[repr(C)]
#[derive(Clone, Copy)]
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
    pub btAddr: super::BTH_ADDR,
    pub serviceClassId: windows_sys::core::GUID,
    pub port: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
    pub sin_addr: super::IN_ADDR,
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
    pub sin6_addr: super::IN6_ADDR,
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
#[derive(Clone, Copy, Default)]
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
    pub sin6_addr: super::IN6_ADDR,
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Default)]
pub struct SOCKET_ADDRESS {
    pub lpSockaddr: LPSOCKADDR,
    pub iSockaddrLength: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy, Default)]
pub struct SOCKET_PROCESSOR_AFFINITY {
    pub Processor: super::PROCESSOR_NUMBER,
    pub NumaNodeId: u16,
    pub Reserved: u16,
}
pub const SOCK_DGRAM: i32 = 2;
pub const SOCK_RAW: i32 = 3;
pub const SOCK_RDM: i32 = 4;
pub const SOCK_SEQPACKET: i32 = 5;
pub const SOCK_STREAM: i32 = 1;
pub const SOL_IP: i32 = 65531;
pub const SOL_IPV6: i32 = 65530;
pub const SOL_L2CAP: i32 = 256;
pub const SOL_RFCOMM: i32 = 3;
pub const SOL_SDP: i32 = 257;
pub const SOL_SOCKET: i32 = 65535;
pub const SO_ACCEPTCONN: i32 = 2;
pub const SO_BROADCAST: i32 = 32;
pub const SO_BSP_STATE: i32 = 4105;
pub const SO_BTH_AUTHENTICATE: u32 = 2147483649;
pub const SO_BTH_ENCRYPT: i32 = 2;
pub const SO_BTH_MTU: u32 = 2147483655;
pub const SO_BTH_MTU_MAX: u32 = 2147483656;
pub const SO_BTH_MTU_MIN: u32 = 2147483658;
pub const SO_COMPARTMENT_ID: i32 = 12292;
pub const SO_CONDITIONAL_ACCEPT: i32 = 12290;
pub const SO_DEBUG: i32 = 1;
pub const SO_DONTLINGER: i32 = -129;
pub const SO_DONTROUTE: i32 = 16;
pub const SO_ERROR: i32 = 4103;
pub const SO_EXCLUSIVEADDRUSE: i32 = -5;
pub const SO_GROUP_ID: i32 = 8193;
pub const SO_GROUP_PRIORITY: i32 = 8194;
pub const SO_KEEPALIVE: i32 = 8;
pub const SO_LINGER: i32 = 128;
pub const SO_MAX_MSG_SIZE: i32 = 8195;
pub const SO_OOBINLINE: i32 = 256;
pub const SO_ORIGINAL_DST: i32 = 12303;
pub const SO_PAUSE_ACCEPT: i32 = 12291;
pub const SO_PORT_SCALABILITY: i32 = 12294;
pub const SO_RANDOMIZE_PORT: i32 = 12293;
pub const SO_RCVBUF: i32 = 4098;
pub const SO_RCVLOWAT: i32 = 4100;
pub const SO_RCVTIMEO: i32 = 4102;
pub const SO_RECEIVED_HOPLIMIT: i32 = 12304;
pub const SO_RECEIVED_PROCESSOR: i32 = 12305;
pub const SO_REUSEADDR: i32 = 4;
pub const SO_REUSE_MULTICASTPORT: i32 = 12296;
pub const SO_REUSE_UNICASTPORT: i32 = 12295;
pub const SO_SNDBUF: i32 = 4097;
pub const SO_SNDLOWAT: i32 = 4099;
pub const SO_SNDTIMEO: i32 = 4101;
pub const SO_TYPE: i32 = 4104;
pub const SO_USELOOPBACK: i32 = 64;
pub const SVCID_BTH_PROVIDER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x06aa63e0_7d60_41ff_afb2_3ee6d2d9392d);
pub const ScopeLevelAdmin: SCOPE_LEVEL = 4;
pub const ScopeLevelCount: SCOPE_LEVEL = 16;
pub const ScopeLevelGlobal: SCOPE_LEVEL = 14;
pub const ScopeLevelInterface: SCOPE_LEVEL = 1;
pub const ScopeLevelLink: SCOPE_LEVEL = 2;
pub const ScopeLevelOrganization: SCOPE_LEVEL = 8;
pub const ScopeLevelSite: SCOPE_LEVEL = 5;
pub const ScopeLevelSubnet: SCOPE_LEVEL = 3;
pub const TCP_ATMARK: i32 = 8;
pub const TCP_CONGESTION_ALGORITHM: i32 = 12;
pub const TCP_DELAY_FIN_ACK: i32 = 13;
pub const TCP_EXPEDITED_1122: i32 = 2;
pub const TCP_FAIL_CONNECT_ON_ICMP_ERROR: i32 = 18;
pub const TCP_FASTOPEN: i32 = 15;
pub const TCP_ICMP_ERROR_INFO: i32 = 19;
pub const TCP_KEEPALIVE: i32 = 3;
pub const TCP_KEEPCNT: i32 = 16;
pub const TCP_KEEPIDLE: i32 = 3;
pub const TCP_KEEPINTVL: i32 = 17;
pub const TCP_MAXRT: i32 = 5;
pub const TCP_MAXRTMS: i32 = 14;
pub const TCP_MAXSEG: i32 = 4;
pub const TCP_NODELAY: i32 = 1;
pub const TCP_NOSYNRETRIES: i32 = 9;
pub const TCP_NOURG: i32 = 7;
pub const TCP_OFFLOAD_NOT_PREFERRED: i32 = 1;
pub const TCP_OFFLOAD_NO_PREFERENCE: i32 = 0;
pub const TCP_OFFLOAD_PREFERENCE: i32 = 11;
pub const TCP_OFFLOAD_PREFERRED: i32 = 2;
pub const TCP_STDURG: i32 = 6;
pub const TCP_TIMESTAMPS: i32 = 10;
pub const UDP_CHECKSUM_COVERAGE: i32 = 20;
pub const UDP_COALESCED_INFO: i32 = 3;
pub const UDP_NOCHECKSUM: i32 = 1;
pub const UDP_RECV_MAX_COALESCED_SIZE: i32 = 3;
pub const UDP_SEND_MSG_SIZE: i32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WSABUF {
    pub len: u32,
    pub buf: *mut i8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WSACMSGHDR {
    pub cmsg_len: usize,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WSAMSG {
    pub name: LPSOCKADDR,
    pub namelen: i32,
    pub lpBuffers: LPWSABUF,
    pub dwBufferCount: u32,
    pub Control: WSABUF,
    pub dwFlags: u32,
}
pub const WSK_SO_BASE: i32 = 16384;
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
    pub sin6_addr: super::IN6_ADDR,
}
#[cfg(feature = "in6addr")]
impl Default for sockaddr_in6_old {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type socklen_t = i32;
