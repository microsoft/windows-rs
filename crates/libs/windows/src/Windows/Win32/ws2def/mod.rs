#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ADDRESS_FAMILY(pub u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(C)]
#[cfg(feature = "Win32_guiddef")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[cfg(feature = "Win32_guiddef")]
impl Default for ADDRINFOEX2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_guiddef")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[cfg(feature = "Win32_guiddef")]
impl Default for ADDRINFOEX2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_guiddef")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[cfg(feature = "Win32_guiddef")]
impl Default for ADDRINFOEX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[cfg(feature = "Win32_winnt")]
impl Default for ADDRINFOEX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[cfg(feature = "Win32_winnt")]
impl Default for ADDRINFOEX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[cfg(feature = "Win32_winnt")]
impl Default for ADDRINFOEX6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[cfg(feature = "Win32_winnt")]
impl Default for ADDRINFOEX7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_guiddef")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[cfg(feature = "Win32_guiddef")]
impl Default for ADDRINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_guiddef")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[cfg(feature = "Win32_guiddef")]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub type CMSGHDR = WSACMSGHDR;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CSADDR_INFO {
    pub LocalAddr: SOCKET_ADDRESS,
    pub RemoteAddr: SOCKET_ADDRESS,
    pub iSocketType: i32,
    pub iProtocol: i32,
}
pub const INADDR_ANY: u32 = 0;
pub const INADDR_BROADCAST: u32 = 4294967295;
pub const INADDR_LOOPBACK: u32 = 2130706433;
pub const INADDR_NONE: u32 = 4294967295;
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
#[cfg(feature = "Win32_guiddef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPADDRINFOEX2A(pub *mut ADDRINFOEX2A);
#[cfg(feature = "Win32_guiddef")]
impl LPADDRINFOEX2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_guiddef")]
impl Default for LPADDRINFOEX2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_guiddef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPADDRINFOEX2W(pub *mut ADDRINFOEX2W);
#[cfg(feature = "Win32_guiddef")]
impl LPADDRINFOEX2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_guiddef")]
impl Default for LPADDRINFOEX2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_guiddef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPADDRINFOEX3(pub *mut ADDRINFOEX3);
#[cfg(feature = "Win32_guiddef")]
impl LPADDRINFOEX3 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_guiddef")]
impl Default for LPADDRINFOEX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPADDRINFOEX4(pub *mut ADDRINFOEX4);
#[cfg(feature = "Win32_winnt")]
impl LPADDRINFOEX4 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for LPADDRINFOEX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPADDRINFOEX5(pub *mut ADDRINFOEX5);
#[cfg(feature = "Win32_winnt")]
impl LPADDRINFOEX5 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for LPADDRINFOEX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_guiddef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPADDRINFOEXA(pub *mut ADDRINFOEXA);
#[cfg(feature = "Win32_guiddef")]
impl LPADDRINFOEXA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_guiddef")]
impl Default for LPADDRINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_guiddef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPADDRINFOEXW(pub *mut ADDRINFOEXW);
#[cfg(feature = "Win32_guiddef")]
impl LPADDRINFOEXW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_guiddef")]
impl Default for LPADDRINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCSADDR_INFO(pub *mut CSADDR_INFO);
impl LPCSADDR_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCSADDR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSOCKADDR(pub *mut SOCKADDR);
impl LPSOCKADDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSOCKADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSOCKADDR_STORAGE(pub *mut SOCKADDR_STORAGE);
impl LPSOCKADDR_STORAGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSOCKADDR_STORAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSOCKADDR_STORAGE_LH(pub *mut SOCKADDR_STORAGE_LH);
impl LPSOCKADDR_STORAGE_LH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSOCKADDR_STORAGE_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSOCKADDR_STORAGE_XP(pub *mut SOCKADDR_STORAGE_XP);
impl LPSOCKADDR_STORAGE_XP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSOCKADDR_STORAGE_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSOCKET_ADDRESS(pub *mut SOCKET_ADDRESS);
impl LPSOCKET_ADDRESS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSOCKET_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSOCKET_ADDRESS_LIST(pub *mut SOCKET_ADDRESS_LIST);
impl LPSOCKET_ADDRESS_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSOCKET_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWSABUF(pub *mut WSABUF);
impl LPWSABUF {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPWSABUF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWSACMSGHDR(pub *mut WSACMSGHDR);
impl LPWSACMSGHDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPWSACMSGHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWSAMSG(pub *mut WSAMSG);
impl LPWSAMSG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPWSAMSG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MSG_BCAST: u32 = 1024;
pub const MSG_CTRUNC: u32 = 512;
pub const MSG_ERRQUEUE: u32 = 4096;
pub const MSG_MCAST: u32 = 2048;
pub const MSG_TRUNC: u32 = 256;
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADDRINFOA(pub *mut ADDRINFOA);
impl PADDRINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADDRINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_guiddef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADDRINFOEX2A(pub *mut ADDRINFOEX2A);
#[cfg(feature = "Win32_guiddef")]
impl PADDRINFOEX2A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_guiddef")]
impl Default for PADDRINFOEX2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_guiddef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADDRINFOEX2W(pub *mut ADDRINFOEX2W);
#[cfg(feature = "Win32_guiddef")]
impl PADDRINFOEX2W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_guiddef")]
impl Default for PADDRINFOEX2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_guiddef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADDRINFOEX3(pub *mut ADDRINFOEX3);
#[cfg(feature = "Win32_guiddef")]
impl PADDRINFOEX3 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_guiddef")]
impl Default for PADDRINFOEX3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADDRINFOEX4(pub *mut ADDRINFOEX4);
#[cfg(feature = "Win32_winnt")]
impl PADDRINFOEX4 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PADDRINFOEX4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADDRINFOEX5(pub *mut ADDRINFOEX5);
#[cfg(feature = "Win32_winnt")]
impl PADDRINFOEX5 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PADDRINFOEX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADDRINFOEX6(pub *mut ADDRINFOEX6);
#[cfg(feature = "Win32_winnt")]
impl PADDRINFOEX6 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PADDRINFOEX6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADDRINFOEX7(pub *mut ADDRINFOEX7);
#[cfg(feature = "Win32_winnt")]
impl PADDRINFOEX7 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PADDRINFOEX7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_guiddef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADDRINFOEXA(pub *mut ADDRINFOEXA);
#[cfg(feature = "Win32_guiddef")]
impl PADDRINFOEXA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_guiddef")]
impl Default for PADDRINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_guiddef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADDRINFOEXW(pub *mut ADDRINFOEXW);
#[cfg(feature = "Win32_guiddef")]
impl PADDRINFOEXW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_guiddef")]
impl Default for PADDRINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADDRINFOW(pub *mut ADDRINFOW);
impl PADDRINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADDRINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCMSGHDR(pub *mut WSACMSGHDR);
impl PCMSGHDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCMSGHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCSADDR_INFO(pub *mut CSADDR_INFO);
impl PCSADDR_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCSADDR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIPROTO(pub *mut IPPROTO);
impl PIPROTO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIPROTO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSCOPE_ID(pub *mut SCOPE_ID);
impl PSCOPE_ID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSCOPE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSOCKADDR(pub *mut SOCKADDR);
impl PSOCKADDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSOCKADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSOCKADDR_DL(pub *mut SOCKADDR_DL);
impl PSOCKADDR_DL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSOCKADDR_DL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_inaddr")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSOCKADDR_IN(pub *mut SOCKADDR_IN);
#[cfg(feature = "Win32_inaddr")]
impl PSOCKADDR_IN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_inaddr")]
impl Default for PSOCKADDR_IN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSOCKADDR_STORAGE(pub *mut SOCKADDR_STORAGE);
impl PSOCKADDR_STORAGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSOCKADDR_STORAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSOCKADDR_STORAGE_LH(pub *mut SOCKADDR_STORAGE_LH);
impl PSOCKADDR_STORAGE_LH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSOCKADDR_STORAGE_LH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSOCKADDR_STORAGE_XP(pub *mut SOCKADDR_STORAGE_XP);
impl PSOCKADDR_STORAGE_XP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSOCKADDR_STORAGE_XP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSOCKET_ADDRESS(pub *mut SOCKET_ADDRESS);
impl PSOCKET_ADDRESS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSOCKET_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSOCKET_ADDRESS_LIST(pub *mut SOCKET_ADDRESS_LIST);
impl PSOCKET_ADDRESS_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSOCKET_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSOCKET_PROCESSOR_AFFINITY(pub *mut SOCKET_PROCESSOR_AFFINITY);
#[cfg(feature = "Win32_winnt")]
impl PSOCKET_PROCESSOR_AFFINITY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PSOCKET_PROCESSOR_AFFINITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWSACMSGHDR(pub *mut WSACMSGHDR);
impl PWSACMSGHDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWSACMSGHDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWSAMSG(pub *mut WSAMSG);
impl PWSAMSG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWSAMSG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCOPE_ID_0_0 {
    pub _bitfield: u32,
}
pub type SCOPE_LEVEL = i32;
pub const SIO_ADDRESS_LIST_CHANGE: u32 = 671088663;
pub const SIO_ADDRESS_LIST_QUERY: u32 = 1207959574;
pub const SIO_ADDRESS_LIST_SORT: i32 = -939524071;
pub const SIO_ASSOCIATE_HANDLE: i32 = -2013265919;
pub const SIO_ENABLE_CIRCULAR_QUEUEING: u32 = 671088642;
pub const SIO_FIND_ROUTE: u32 = 1207959555;
pub const SIO_FLUSH: u32 = 671088644;
pub const SIO_GET_BROADCAST_ADDRESS: u32 = 1207959557;
pub const SIO_GET_EXTENSION_FUNCTION_POINTER: i32 = -939524090;
pub const SIO_GET_GROUP_QOS: i32 = -939524088;
pub const SIO_GET_MULTIPLE_EXTENSION_FUNCTION_POINTER: i32 = -939524060;
pub const SIO_GET_QOS: i32 = -939524089;
pub const SIO_MULTICAST_SCOPE: i32 = -2013265910;
pub const SIO_MULTIPOINT_LOOPBACK: i32 = -2013265911;
pub const SIO_QUERY_RSS_PROCESSOR_INFO: u32 = 1207959589;
pub const SIO_QUERY_TARGET_PNP_HANDLE: u32 = 1207959576;
pub const SIO_RESERVED_1: i32 = -2013265894;
pub const SIO_RESERVED_2: i32 = -2013265887;
pub const SIO_ROUTING_INTERFACE_CHANGE: i32 = -2013265899;
pub const SIO_ROUTING_INTERFACE_QUERY: i32 = -939524076;
pub const SIO_SET_GROUP_QOS: i32 = -2013265908;
pub const SIO_SET_QOS: i32 = -2013265909;
pub const SIO_TRANSLATE_HANDLE: i32 = -939524083;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SOCKADDR {
    pub sa_family: ADDRESS_FAMILY,
    pub sa_data: [i8; 14],
}
impl Default for SOCKADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[cfg(feature = "Win32_inaddr")]
#[derive(Clone, Copy)]
pub struct SOCKADDR_IN {
    pub sin_family: ADDRESS_FAMILY,
    pub sin_port: u16,
    pub sin_addr: super::inaddr::IN_ADDR,
    pub sin_zero: [i8; 8],
}
#[cfg(feature = "Win32_inaddr")]
impl Default for SOCKADDR_IN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SOCKADDR_STORAGE = SOCKADDR_STORAGE_LH;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SOCKET_ADDRESS {
    pub lpSockaddr: LPSOCKADDR,
    pub iSockaddrLength: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const SOL_SOCKET: u32 = 65535;
pub const SO_ACCEPTCONN: u32 = 2;
pub const SO_BROADCAST: u32 = 32;
pub const SO_BSP_STATE: u32 = 4105;
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
pub const ScopeLevelAdmin: SCOPE_LEVEL = 4;
pub const ScopeLevelCount: SCOPE_LEVEL = 16;
pub const ScopeLevelGlobal: SCOPE_LEVEL = 14;
pub const ScopeLevelInterface: SCOPE_LEVEL = 1;
pub const ScopeLevelLink: SCOPE_LEVEL = 2;
pub const ScopeLevelOrganization: SCOPE_LEVEL = 8;
pub const ScopeLevelSite: SCOPE_LEVEL = 5;
pub const ScopeLevelSubnet: SCOPE_LEVEL = 3;
pub const TCP_NODELAY: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSACMSGHDR {
    pub cmsg_len: usize,
    pub cmsg_level: i32,
    pub cmsg_type: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSAMSG {
    pub name: LPSOCKADDR,
    pub namelen: i32,
    pub lpBuffers: LPWSABUF,
    pub dwBufferCount: u32,
    pub Control: WSABUF,
    pub dwFlags: u32,
}
pub const WSK_SO_BASE: u32 = 16384;
