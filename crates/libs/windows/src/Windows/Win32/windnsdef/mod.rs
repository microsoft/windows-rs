pub const DDR_MAX_IP_HINTS: u32 = 4;
pub const DNSREC_ADDITIONAL: u32 = 3;
pub const DNSREC_ANSWER: u32 = 1;
pub const DNSREC_AUTHORITY: u32 = 2;
pub const DNSREC_DELETE: u32 = 4;
pub const DNSREC_NOEXIST: u32 = 4;
pub const DNSREC_PREREQ: u32 = 1;
pub const DNSREC_QUESTION: u32 = 0;
pub const DNSREC_SECTION: u32 = 3;
pub const DNSREC_UPDATE: u32 = 2;
pub const DNSREC_ZONE: u32 = 0;
pub const DNSSEC_ALGORITHM_ECDSAP256_SHA256: u32 = 13;
pub const DNSSEC_ALGORITHM_ECDSAP384_SHA384: u32 = 14;
pub const DNSSEC_ALGORITHM_NULL: u32 = 253;
pub const DNSSEC_ALGORITHM_PRIVATE: u32 = 254;
pub const DNSSEC_ALGORITHM_RSAMD5: u32 = 1;
pub const DNSSEC_ALGORITHM_RSASHA1: u32 = 5;
pub const DNSSEC_ALGORITHM_RSASHA1_NSEC3: u32 = 7;
pub const DNSSEC_ALGORITHM_RSASHA256: u32 = 8;
pub const DNSSEC_ALGORITHM_RSASHA512: u32 = 10;
pub const DNSSEC_DIGEST_ALGORITHM_SHA1: u32 = 1;
pub const DNSSEC_DIGEST_ALGORITHM_SHA256: u32 = 2;
pub const DNSSEC_DIGEST_ALGORITHM_SHA384: u32 = 4;
pub const DNSSEC_KEY_FLAG_EXTEND: u32 = 8;
pub const DNSSEC_KEY_FLAG_FLAG10: u32 = 1024;
pub const DNSSEC_KEY_FLAG_FLAG11: u32 = 2048;
pub const DNSSEC_KEY_FLAG_FLAG2: u32 = 4;
pub const DNSSEC_KEY_FLAG_FLAG4: u32 = 16;
pub const DNSSEC_KEY_FLAG_FLAG5: u32 = 32;
pub const DNSSEC_KEY_FLAG_FLAG8: u32 = 256;
pub const DNSSEC_KEY_FLAG_FLAG9: u32 = 512;
pub const DNSSEC_KEY_FLAG_HOST: u32 = 128;
pub const DNSSEC_KEY_FLAG_NOAUTH: u32 = 1;
pub const DNSSEC_KEY_FLAG_NOCONF: u32 = 2;
pub const DNSSEC_KEY_FLAG_NTPE3: u32 = 192;
pub const DNSSEC_KEY_FLAG_SIG0: u32 = 0;
pub const DNSSEC_KEY_FLAG_SIG1: u32 = 4096;
pub const DNSSEC_KEY_FLAG_SIG10: u32 = 40960;
pub const DNSSEC_KEY_FLAG_SIG11: u32 = 45056;
pub const DNSSEC_KEY_FLAG_SIG12: u32 = 49152;
pub const DNSSEC_KEY_FLAG_SIG13: u32 = 53248;
pub const DNSSEC_KEY_FLAG_SIG14: u32 = 57344;
pub const DNSSEC_KEY_FLAG_SIG15: u32 = 61440;
pub const DNSSEC_KEY_FLAG_SIG2: u32 = 8192;
pub const DNSSEC_KEY_FLAG_SIG3: u32 = 12288;
pub const DNSSEC_KEY_FLAG_SIG4: u32 = 16384;
pub const DNSSEC_KEY_FLAG_SIG5: u32 = 20480;
pub const DNSSEC_KEY_FLAG_SIG6: u32 = 24576;
pub const DNSSEC_KEY_FLAG_SIG7: u32 = 28672;
pub const DNSSEC_KEY_FLAG_SIG8: u32 = 32768;
pub const DNSSEC_KEY_FLAG_SIG9: u32 = 36864;
pub const DNSSEC_KEY_FLAG_USER: u32 = 0;
pub const DNSSEC_KEY_FLAG_ZONE: u32 = 64;
pub const DNSSEC_PROTOCOL_DNSSEC: u32 = 3;
pub const DNSSEC_PROTOCOL_EMAIL: u32 = 2;
pub const DNSSEC_PROTOCOL_IPSEC: u32 = 4;
pub const DNSSEC_PROTOCOL_NONE: u32 = 0;
pub const DNSSEC_PROTOCOL_TLS: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_AAAA_DATA {
    pub Ip6Address: IP6_ADDRESS,
}
impl Default for DNS_AAAA_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_ADDR {
    pub MaxSa: [i8; 32],
    pub Data: DNS_ADDR_0,
}
impl Default for DNS_ADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DNS_ADDR_0 {
    pub DnsAddrUserDword: [u32; 8],
}
impl Default for DNS_ADDR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct DNS_ADDR_ARRAY {
    pub MaxCount: u32,
    pub AddrCount: u32,
    pub Tag: u32,
    pub Family: u16,
    pub WordReserved: u16,
    pub Flags: u32,
    pub MatchFlag: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub AddrArray: [DNS_ADDR; 1],
}
impl Default for DNS_ADDR_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_ADDR_MAX_SOCKADDR_LENGTH: u32 = 32;
pub const DNS_ATMA_AESA_ADDR_LENGTH: u32 = 20;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_ATMA_DATA {
    pub AddressType: u8,
    pub Address: [u8; 20],
}
impl Default for DNS_ATMA_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_ATMA_FORMAT_AESA: u32 = 2;
pub const DNS_ATMA_FORMAT_E164: u32 = 1;
pub const DNS_ATMA_MAX_ADDR_LENGTH: u32 = 20;
pub const DNS_ATMA_MAX_RECORD_LENGTH: u32 = 21;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_A_DATA {
    pub IpAddress: IP4_ADDRESS,
}
pub type DNS_CHARSET = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct DNS_CUSTOM_SERVER {
    pub dwServerType: u32,
    pub ullFlags: u64,
    pub Anonymous: DNS_CUSTOM_SERVER_0,
    pub Anonymous2: DNS_CUSTOM_SERVER_1,
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for DNS_CUSTOM_SERVER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub union DNS_CUSTOM_SERVER_0 {
    pub pwszTemplate: windows_core::PWSTR,
    pub pwszHostname: windows_core::PWSTR,
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for DNS_CUSTOM_SERVER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub union DNS_CUSTOM_SERVER_1 {
    pub ServerAddr: super::ws2ipdef::SOCKADDR_INET,
    pub MaxSa: [i8; 32],
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for DNS_CUSTOM_SERVER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_CUSTOM_SERVER_TYPE_DOH: u32 = 2;
pub const DNS_CUSTOM_SERVER_TYPE_DOT: u32 = 3;
pub const DNS_CUSTOM_SERVER_TYPE_UDP: u32 = 1;
pub const DNS_CUSTOM_SERVER_UDP_FALLBACK: u32 = 1;
pub const DNS_CUSTOM_SERVER_UPGRADE_FROM_WELL_KNOWN_SERVERS: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_DHCID_DATA {
    pub dwByteCount: u32,
    pub DHCID: [u8; 1],
}
impl Default for DNS_DHCID_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_DNSKEY_DATA {
    pub wFlags: u16,
    pub chProtocol: u8,
    pub chAlgorithm: u8,
    pub wKeyLength: u16,
    pub wPad: u16,
    pub Key: [u8; 1],
}
impl Default for DNS_DNSKEY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_DS_DATA {
    pub wKeyTag: u16,
    pub chAlgorithm: u8,
    pub chDigestType: u8,
    pub wDigestLength: u16,
    pub wPad: u16,
    pub Digest: [u8; 1],
}
impl Default for DNS_DS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DNS_HEADER {
    pub Xid: u16,
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub QuestionCount: u16,
    pub AnswerCount: u16,
    pub NameServerCount: u16,
    pub AdditionalCount: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_HEADER_EXT {
    pub _bitfield: u16,
    pub chRcode: u8,
    pub chVersion: u8,
}
pub type DNS_KEY_DATA = DNS_DNSKEY_DATA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_LOC_DATA {
    pub wVersion: u16,
    pub wSize: u16,
    pub wHorPrec: u16,
    pub wVerPrec: u16,
    pub dwLatitude: u32,
    pub dwLongitude: u32,
    pub dwAltitude: u32,
}
pub const DNS_MAX_LABEL_BUFFER_LENGTH: u32 = 64;
pub const DNS_MAX_LABEL_LENGTH: u32 = 63;
pub const DNS_MAX_NAME_BUFFER_LENGTH: u32 = 256;
pub const DNS_MAX_NAME_LENGTH: u32 = 255;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_MESSAGE_BUFFER {
    pub MessageHead: DNS_HEADER,
    pub MessageBody: [i8; 1],
}
impl Default for DNS_MESSAGE_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DNS_MINFO_DATA = DNS_MINFO_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_MINFO_DATAA {
    pub pNameMailbox: windows_core::PSTR,
    pub pNameErrorsMailbox: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_MINFO_DATAW {
    pub pNameMailbox: windows_core::PWSTR,
    pub pNameErrorsMailbox: windows_core::PWSTR,
}
pub type DNS_MX_DATA = DNS_MX_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_MX_DATAA {
    pub pNameExchange: windows_core::PSTR,
    pub wPreference: u16,
    pub Pad: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_MX_DATAW {
    pub pNameExchange: windows_core::PWSTR,
    pub wPreference: u16,
    pub Pad: u16,
}
pub type DNS_NAPTR_DATA = DNS_NAPTR_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_NAPTR_DATAA {
    pub wOrder: u16,
    pub wPreference: u16,
    pub pFlags: windows_core::PSTR,
    pub pService: windows_core::PSTR,
    pub pRegularExpression: windows_core::PSTR,
    pub pReplacement: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_NAPTR_DATAW {
    pub wOrder: u16,
    pub wPreference: u16,
    pub pFlags: windows_core::PWSTR,
    pub pService: windows_core::PWSTR,
    pub pRegularExpression: windows_core::PWSTR,
    pub pReplacement: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_NSEC3PARAM_DATA {
    pub chAlgorithm: u8,
    pub bFlags: u8,
    pub wIterations: u16,
    pub bSaltLength: u8,
    pub bPad: [u8; 3],
    pub pbSalt: [u8; 1],
}
impl Default for DNS_NSEC3PARAM_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_NSEC3_DATA {
    pub chAlgorithm: u8,
    pub bFlags: u8,
    pub wIterations: u16,
    pub bSaltLength: u8,
    pub bHashLength: u8,
    pub wTypeBitMapsLength: u16,
    pub chData: [u8; 1],
}
impl Default for DNS_NSEC3_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DNS_NSEC_DATA = DNS_NSEC_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_NSEC_DATAA {
    pub pNextDomainName: windows_core::PSTR,
    pub wTypeBitMapsLength: u16,
    pub wPad: u16,
    pub TypeBitMaps: [u8; 1],
}
impl Default for DNS_NSEC_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_NSEC_DATAW {
    pub pNextDomainName: windows_core::PWSTR,
    pub wTypeBitMapsLength: u16,
    pub wPad: u16,
    pub TypeBitMaps: [u8; 1],
}
impl Default for DNS_NSEC_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_NULL_DATA {
    pub dwByteCount: u32,
    pub Data: [u8; 1],
}
impl Default for DNS_NULL_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DNS_NXT_DATA = DNS_NXT_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_NXT_DATAA {
    pub pNameNext: windows_core::PSTR,
    pub wNumTypes: u16,
    pub wTypes: [u16; 1],
}
impl Default for DNS_NXT_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_NXT_DATAW {
    pub pNameNext: windows_core::PWSTR,
    pub wNumTypes: u16,
    pub wTypes: [u16; 1],
}
impl Default for DNS_NXT_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_OPT_DATA {
    pub wDataLength: u16,
    pub wPad: u16,
    pub Data: [u8; 1],
}
impl Default for DNS_OPT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DNS_PTR_DATA = DNS_PTR_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_PTR_DATAA {
    pub pNameHost: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_PTR_DATAW {
    pub pNameHost: windows_core::PWSTR,
}
#[cfg(feature = "Win32_minwindef")]
pub type DNS_RECORD = DNS_RECORDA;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct DNS_RECORDA {
    pub pNext: *mut Self,
    pub pName: windows_core::PSTR,
    pub wType: u16,
    pub wDataLength: u16,
    pub Flags: DNS_RECORDA_0,
    pub dwTtl: u32,
    pub dwReserved: u32,
    pub Data: DNS_RECORDA_1,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DNS_RECORDA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub union DNS_RECORDA_0 {
    pub DW: u32,
    pub S: DNS_RECORD_FLAGS,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DNS_RECORDA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub union DNS_RECORDA_1 {
    pub A: DNS_A_DATA,
    pub SOA: DNS_SOA_DATAA,
    pub Soa: DNS_SOA_DATAA,
    pub PTR: DNS_PTR_DATAA,
    pub Ptr: DNS_PTR_DATAA,
    pub NS: DNS_PTR_DATAA,
    pub Ns: DNS_PTR_DATAA,
    pub CNAME: DNS_PTR_DATAA,
    pub Cname: DNS_PTR_DATAA,
    pub DNAME: DNS_PTR_DATAA,
    pub Dname: DNS_PTR_DATAA,
    pub MB: DNS_PTR_DATAA,
    pub Mb: DNS_PTR_DATAA,
    pub MD: DNS_PTR_DATAA,
    pub Md: DNS_PTR_DATAA,
    pub MF: DNS_PTR_DATAA,
    pub Mf: DNS_PTR_DATAA,
    pub MG: DNS_PTR_DATAA,
    pub Mg: DNS_PTR_DATAA,
    pub MR: DNS_PTR_DATAA,
    pub Mr: DNS_PTR_DATAA,
    pub MINFO: DNS_MINFO_DATAA,
    pub Minfo: DNS_MINFO_DATAA,
    pub RP: DNS_MINFO_DATAA,
    pub Rp: DNS_MINFO_DATAA,
    pub MX: DNS_MX_DATAA,
    pub Mx: DNS_MX_DATAA,
    pub AFSDB: DNS_MX_DATAA,
    pub Afsdb: DNS_MX_DATAA,
    pub RT: DNS_MX_DATAA,
    pub Rt: DNS_MX_DATAA,
    pub HINFO: DNS_TXT_DATAA,
    pub Hinfo: DNS_TXT_DATAA,
    pub ISDN: DNS_TXT_DATAA,
    pub Isdn: DNS_TXT_DATAA,
    pub TXT: DNS_TXT_DATAA,
    pub Txt: DNS_TXT_DATAA,
    pub X25: DNS_TXT_DATAA,
    pub Null: DNS_NULL_DATA,
    pub WKS: DNS_WKS_DATA,
    pub Wks: DNS_WKS_DATA,
    pub AAAA: DNS_AAAA_DATA,
    pub KEY: DNS_KEY_DATA,
    pub Key: DNS_KEY_DATA,
    pub SIG: DNS_SIG_DATAA,
    pub Sig: DNS_SIG_DATAA,
    pub ATMA: DNS_ATMA_DATA,
    pub Atma: DNS_ATMA_DATA,
    pub NXT: DNS_NXT_DATAA,
    pub Nxt: DNS_NXT_DATAA,
    pub SRV: DNS_SRV_DATAA,
    pub Srv: DNS_SRV_DATAA,
    pub NAPTR: DNS_NAPTR_DATAA,
    pub Naptr: DNS_NAPTR_DATAA,
    pub OPT: DNS_OPT_DATA,
    pub Opt: DNS_OPT_DATA,
    pub DS: DNS_DS_DATA,
    pub Ds: DNS_DS_DATA,
    pub RRSIG: DNS_RRSIG_DATAA,
    pub Rrsig: DNS_RRSIG_DATAA,
    pub NSEC: DNS_NSEC_DATAA,
    pub Nsec: DNS_NSEC_DATAA,
    pub DNSKEY: DNS_DNSKEY_DATA,
    pub Dnskey: DNS_DNSKEY_DATA,
    pub TKEY: DNS_TKEY_DATAA,
    pub Tkey: DNS_TKEY_DATAA,
    pub TSIG: DNS_TSIG_DATAA,
    pub Tsig: DNS_TSIG_DATAA,
    pub WINS: DNS_WINS_DATA,
    pub Wins: DNS_WINS_DATA,
    pub WINSR: DNS_WINSR_DATAA,
    pub WinsR: DNS_WINSR_DATAA,
    pub NBSTAT: DNS_WINSR_DATAA,
    pub Nbstat: DNS_WINSR_DATAA,
    pub DHCID: DNS_DHCID_DATA,
    pub NSEC3: DNS_NSEC3_DATA,
    pub Nsec3: DNS_NSEC3_DATA,
    pub NSEC3PARAM: DNS_NSEC3PARAM_DATA,
    pub Nsec3Param: DNS_NSEC3PARAM_DATA,
    pub TLSA: DNS_TLSA_DATA,
    pub Tlsa: DNS_TLSA_DATA,
    pub SVCB: DNS_SVCB_DATA,
    pub Svcb: DNS_SVCB_DATA,
    pub UNKNOWN: DNS_UNKNOWN_DATA,
    pub Unknown: DNS_UNKNOWN_DATA,
    pub pDataPtr: super::minwindef::PBYTE,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DNS_RECORDA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct DNS_RECORDW {
    pub pNext: *mut Self,
    pub pName: windows_core::PWSTR,
    pub wType: u16,
    pub wDataLength: u16,
    pub Flags: DNS_RECORDW_0,
    pub dwTtl: u32,
    pub dwReserved: u32,
    pub Data: DNS_RECORDW_1,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DNS_RECORDW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub union DNS_RECORDW_0 {
    pub DW: u32,
    pub S: DNS_RECORD_FLAGS,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DNS_RECORDW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub union DNS_RECORDW_1 {
    pub A: DNS_A_DATA,
    pub SOA: DNS_SOA_DATAW,
    pub Soa: DNS_SOA_DATAW,
    pub PTR: DNS_PTR_DATAW,
    pub Ptr: DNS_PTR_DATAW,
    pub NS: DNS_PTR_DATAW,
    pub Ns: DNS_PTR_DATAW,
    pub CNAME: DNS_PTR_DATAW,
    pub Cname: DNS_PTR_DATAW,
    pub DNAME: DNS_PTR_DATAW,
    pub Dname: DNS_PTR_DATAW,
    pub MB: DNS_PTR_DATAW,
    pub Mb: DNS_PTR_DATAW,
    pub MD: DNS_PTR_DATAW,
    pub Md: DNS_PTR_DATAW,
    pub MF: DNS_PTR_DATAW,
    pub Mf: DNS_PTR_DATAW,
    pub MG: DNS_PTR_DATAW,
    pub Mg: DNS_PTR_DATAW,
    pub MR: DNS_PTR_DATAW,
    pub Mr: DNS_PTR_DATAW,
    pub MINFO: DNS_MINFO_DATAW,
    pub Minfo: DNS_MINFO_DATAW,
    pub RP: DNS_MINFO_DATAW,
    pub Rp: DNS_MINFO_DATAW,
    pub MX: DNS_MX_DATAW,
    pub Mx: DNS_MX_DATAW,
    pub AFSDB: DNS_MX_DATAW,
    pub Afsdb: DNS_MX_DATAW,
    pub RT: DNS_MX_DATAW,
    pub Rt: DNS_MX_DATAW,
    pub HINFO: DNS_TXT_DATAW,
    pub Hinfo: DNS_TXT_DATAW,
    pub ISDN: DNS_TXT_DATAW,
    pub Isdn: DNS_TXT_DATAW,
    pub TXT: DNS_TXT_DATAW,
    pub Txt: DNS_TXT_DATAW,
    pub X25: DNS_TXT_DATAW,
    pub Null: DNS_NULL_DATA,
    pub WKS: DNS_WKS_DATA,
    pub Wks: DNS_WKS_DATA,
    pub AAAA: DNS_AAAA_DATA,
    pub KEY: DNS_KEY_DATA,
    pub Key: DNS_KEY_DATA,
    pub SIG: DNS_SIG_DATAW,
    pub Sig: DNS_SIG_DATAW,
    pub ATMA: DNS_ATMA_DATA,
    pub Atma: DNS_ATMA_DATA,
    pub NXT: DNS_NXT_DATAW,
    pub Nxt: DNS_NXT_DATAW,
    pub SRV: DNS_SRV_DATAW,
    pub Srv: DNS_SRV_DATAW,
    pub NAPTR: DNS_NAPTR_DATAW,
    pub Naptr: DNS_NAPTR_DATAW,
    pub OPT: DNS_OPT_DATA,
    pub Opt: DNS_OPT_DATA,
    pub DS: DNS_DS_DATA,
    pub Ds: DNS_DS_DATA,
    pub RRSIG: DNS_RRSIG_DATAW,
    pub Rrsig: DNS_RRSIG_DATAW,
    pub NSEC: DNS_NSEC_DATAW,
    pub Nsec: DNS_NSEC_DATAW,
    pub DNSKEY: DNS_DNSKEY_DATA,
    pub Dnskey: DNS_DNSKEY_DATA,
    pub TKEY: DNS_TKEY_DATAW,
    pub Tkey: DNS_TKEY_DATAW,
    pub TSIG: DNS_TSIG_DATAW,
    pub Tsig: DNS_TSIG_DATAW,
    pub WINS: DNS_WINS_DATA,
    pub Wins: DNS_WINS_DATA,
    pub WINSR: DNS_WINSR_DATAW,
    pub WinsR: DNS_WINSR_DATAW,
    pub NBSTAT: DNS_WINSR_DATAW,
    pub Nbstat: DNS_WINSR_DATAW,
    pub DHCID: DNS_DHCID_DATA,
    pub NSEC3: DNS_NSEC3_DATA,
    pub Nsec3: DNS_NSEC3_DATA,
    pub NSEC3PARAM: DNS_NSEC3PARAM_DATA,
    pub Nsec3Param: DNS_NSEC3PARAM_DATA,
    pub TLSA: DNS_TLSA_DATA,
    pub Tlsa: DNS_TLSA_DATA,
    pub SVCB: DNS_SVCB_DATA,
    pub Svcb: DNS_SVCB_DATA,
    pub UNKNOWN: DNS_UNKNOWN_DATA,
    pub Unknown: DNS_UNKNOWN_DATA,
    pub pDataPtr: super::minwindef::PBYTE,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DNS_RECORDW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const DNS_RECORD_FIXED_SIZE: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DNS_RECORD_FIXED_SIZE: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_RECORD_FLAGS {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_minwindef")]
pub type DNS_RECORD_OPT = DNS_RECORD_OPTA;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct DNS_RECORD_OPTA {
    pub pNext: *mut DNS_RECORDA,
    pub pName: windows_core::PSTR,
    pub wType: u16,
    pub wDataLength: u16,
    pub Flags: DNS_RECORD_OPTA_0,
    pub ExtHeader: DNS_HEADER_EXT,
    pub wPayloadSize: u16,
    pub wReserved: u16,
    pub Data: DNS_RECORD_OPTA_1,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DNS_RECORD_OPTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub union DNS_RECORD_OPTA_0 {
    pub DW: u32,
    pub S: DNS_RECORD_FLAGS,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DNS_RECORD_OPTA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub union DNS_RECORD_OPTA_1 {
    pub OPT: DNS_OPT_DATA,
    pub Opt: DNS_OPT_DATA,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DNS_RECORD_OPTA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct DNS_RECORD_OPTW {
    pub pNext: *mut DNS_RECORDW,
    pub pName: windows_core::PWSTR,
    pub wType: u16,
    pub wDataLength: u16,
    pub Flags: DNS_RECORD_OPTW_0,
    pub ExtHeader: DNS_HEADER_EXT,
    pub wPayloadSize: u16,
    pub wReserved: u16,
    pub Data: DNS_RECORD_OPTW_1,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DNS_RECORD_OPTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub union DNS_RECORD_OPTW_0 {
    pub DW: u32,
    pub S: DNS_RECORD_FLAGS,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DNS_RECORD_OPTW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub union DNS_RECORD_OPTW_1 {
    pub OPT: DNS_OPT_DATA,
    pub Opt: DNS_OPT_DATA,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DNS_RECORD_OPTW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_RFC_MAX_UDP_PACKET_LENGTH: u32 = 512;
pub type DNS_RRSIG_DATA = DNS_RRSIG_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_RRSIG_DATAA {
    pub wTypeCovered: u16,
    pub chAlgorithm: u8,
    pub chLabelCount: u8,
    pub dwOriginalTtl: u32,
    pub dwExpiration: u32,
    pub dwTimeSigned: u32,
    pub wKeyTag: u16,
    pub wSignatureLength: u16,
    pub pNameSigner: windows_core::PSTR,
    pub Signature: [u8; 1],
}
impl Default for DNS_RRSIG_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_RRSIG_DATAW {
    pub wTypeCovered: u16,
    pub chAlgorithm: u8,
    pub chLabelCount: u8,
    pub dwOriginalTtl: u32,
    pub dwExpiration: u32,
    pub dwTimeSigned: u32,
    pub wKeyTag: u16,
    pub wSignatureLength: u16,
    pub pNameSigner: windows_core::PWSTR,
    pub Signature: [u8; 1],
}
impl Default for DNS_RRSIG_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_RTYPE_A: u32 = 256;
pub const DNS_RTYPE_A6: u32 = 9728;
pub const DNS_RTYPE_AAAA: u32 = 7168;
pub const DNS_RTYPE_AFSDB: u32 = 4608;
pub const DNS_RTYPE_ALL: u32 = 65280;
pub const DNS_RTYPE_ANY: u32 = 65280;
pub const DNS_RTYPE_ATMA: u32 = 8704;
pub const DNS_RTYPE_AXFR: u32 = 64512;
pub const DNS_RTYPE_CERT: u32 = 9472;
pub const DNS_RTYPE_CNAME: u32 = 1280;
pub const DNS_RTYPE_DHCID: u32 = 12544;
pub const DNS_RTYPE_DNAME: u32 = 9984;
pub const DNS_RTYPE_DNSKEY: u32 = 12288;
pub const DNS_RTYPE_DS: u32 = 11008;
pub const DNS_RTYPE_EID: u32 = 7936;
pub const DNS_RTYPE_GID: u32 = 26112;
pub const DNS_RTYPE_GPOS: u32 = 6912;
pub const DNS_RTYPE_HINFO: u32 = 3328;
pub const DNS_RTYPE_ISDN: u32 = 5120;
pub const DNS_RTYPE_IXFR: u32 = 64256;
pub const DNS_RTYPE_KEY: u32 = 6400;
pub const DNS_RTYPE_KX: u32 = 9216;
pub const DNS_RTYPE_LOC: u32 = 7424;
pub const DNS_RTYPE_MAILA: u32 = 65024;
pub const DNS_RTYPE_MAILB: u32 = 64768;
pub const DNS_RTYPE_MB: u32 = 1792;
pub const DNS_RTYPE_MD: u32 = 768;
pub const DNS_RTYPE_MF: u32 = 1024;
pub const DNS_RTYPE_MG: u32 = 2048;
pub const DNS_RTYPE_MINFO: u32 = 3584;
pub const DNS_RTYPE_MR: u32 = 2304;
pub const DNS_RTYPE_MX: u32 = 3840;
pub const DNS_RTYPE_NAPTR: u32 = 8960;
pub const DNS_RTYPE_NIMLOC: u32 = 8192;
pub const DNS_RTYPE_NS: u32 = 512;
pub const DNS_RTYPE_NSAP: u32 = 5632;
pub const DNS_RTYPE_NSAPPTR: u32 = 5888;
pub const DNS_RTYPE_NSEC: u32 = 12032;
pub const DNS_RTYPE_NSEC3: u32 = 12800;
pub const DNS_RTYPE_NSEC3PARAM: u32 = 13056;
pub const DNS_RTYPE_NULL: u32 = 2560;
pub const DNS_RTYPE_NXT: u32 = 7680;
pub const DNS_RTYPE_OPT: u32 = 10496;
pub const DNS_RTYPE_PTR: u32 = 3072;
pub const DNS_RTYPE_PX: u32 = 6656;
pub const DNS_RTYPE_RP: u32 = 4352;
pub const DNS_RTYPE_RRSIG: u32 = 11776;
pub const DNS_RTYPE_RT: u32 = 5376;
pub const DNS_RTYPE_SIG: u32 = 6144;
pub const DNS_RTYPE_SINK: u32 = 10240;
pub const DNS_RTYPE_SOA: u32 = 1536;
pub const DNS_RTYPE_SRV: u32 = 8448;
pub const DNS_RTYPE_TEXT: u32 = 4096;
pub const DNS_RTYPE_TKEY: u32 = 63744;
pub const DNS_RTYPE_TLSA: u32 = 13312;
pub const DNS_RTYPE_TSIG: u32 = 64000;
pub const DNS_RTYPE_UID: u32 = 25856;
pub const DNS_RTYPE_UINFO: u32 = 25600;
pub const DNS_RTYPE_UNSPEC: u32 = 26368;
pub const DNS_RTYPE_WINS: u32 = 511;
pub const DNS_RTYPE_WINSR: u32 = 767;
pub const DNS_RTYPE_WKS: u32 = 2816;
pub const DNS_RTYPE_X25: u32 = 4864;
pub type DNS_SECTION = i32;
pub type DNS_SIG_DATA = DNS_SIG_DATAA;
pub type DNS_SIG_DATAA = DNS_RRSIG_DATAA;
pub type DNS_SIG_DATAW = DNS_RRSIG_DATAW;
pub type DNS_SOA_DATA = DNS_SOA_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_SOA_DATAA {
    pub pNamePrimaryServer: windows_core::PSTR,
    pub pNameAdministrator: windows_core::PSTR,
    pub dwSerialNo: u32,
    pub dwRefresh: u32,
    pub dwRetry: u32,
    pub dwExpire: u32,
    pub dwDefaultTtl: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_SOA_DATAW {
    pub pNamePrimaryServer: windows_core::PWSTR,
    pub pNameAdministrator: windows_core::PWSTR,
    pub dwSerialNo: u32,
    pub dwRefresh: u32,
    pub dwRetry: u32,
    pub dwExpire: u32,
    pub dwDefaultTtl: u32,
}
pub type DNS_SRV_DATA = DNS_SRV_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_SRV_DATAA {
    pub pNameTarget: windows_core::PSTR,
    pub wPriority: u16,
    pub wWeight: u16,
    pub wPort: u16,
    pub Pad: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_SRV_DATAW {
    pub pNameTarget: windows_core::PWSTR,
    pub wPriority: u16,
    pub wWeight: u16,
    pub wPort: u16,
    pub Pad: u16,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DNS_STATUS(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_SVCB_DATA {
    pub wSvcPriority: u16,
    pub pszTargetName: windows_core::PSTR,
    pub cSvcParams: u16,
    pub pSvcParams: *mut DNS_SVCB_PARAM,
}
impl Default for DNS_SVCB_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_SVCB_PARAM {
    pub wSvcParamKey: u16,
    pub Anonymous: DNS_SVCB_PARAM_0,
}
impl Default for DNS_SVCB_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DNS_SVCB_PARAM_0 {
    pub pIpv4Hints: *mut DNS_SVCB_PARAM_IPV4,
    pub pIpv6Hints: *mut DNS_SVCB_PARAM_IPV6,
    pub pMandatory: *mut DNS_SVCB_PARAM_MANDATORY,
    pub pAlpn: *mut DNS_SVCB_PARAM_ALPN,
    pub wPort: u16,
    pub pUnknown: *mut DNS_SVCB_PARAM_UNKNOWN,
    pub pszDohPath: windows_core::PSTR,
    pub pReserved: *mut core::ffi::c_void,
}
impl Default for DNS_SVCB_PARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_SVCB_PARAM_ALPN {
    pub cIds: u16,
    pub rgIds: [DNS_SVCB_PARAM_ALPN_ID; 1],
}
impl Default for DNS_SVCB_PARAM_ALPN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_SVCB_PARAM_ALPN_ID {
    pub cBytes: u8,
    pub pbId: *mut u8,
}
impl Default for DNS_SVCB_PARAM_ALPN_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_SVCB_PARAM_IPV4 {
    pub cIps: u16,
    pub rgIps: [IP4_ADDRESS; 1],
}
impl Default for DNS_SVCB_PARAM_IPV4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_SVCB_PARAM_IPV6 {
    pub cIps: u16,
    pub rgIps: [IP6_ADDRESS; 1],
}
impl Default for DNS_SVCB_PARAM_IPV6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_SVCB_PARAM_MANDATORY {
    pub cMandatoryKeys: u16,
    pub rgwMandatoryKeys: [u16; 1],
}
impl Default for DNS_SVCB_PARAM_MANDATORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DNS_SVCB_PARAM_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_SVCB_PARAM_UNKNOWN {
    pub cBytes: u16,
    pub pbSvcParamValue: [u8; 1],
}
impl Default for DNS_SVCB_PARAM_UNKNOWN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
pub type DNS_TKEY_DATA = DNS_TKEY_DATAA;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_TKEY_DATAA {
    pub pNameAlgorithm: windows_core::PSTR,
    pub pAlgorithmPacket: super::minwindef::PBYTE,
    pub pKey: super::minwindef::PBYTE,
    pub pOtherData: super::minwindef::PBYTE,
    pub dwCreateTime: u32,
    pub dwExpireTime: u32,
    pub wMode: u16,
    pub wError: u16,
    pub wKeyLength: u16,
    pub wOtherLength: u16,
    pub cAlgNameLength: u8,
    pub bPacketPointers: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_TKEY_DATAW {
    pub pNameAlgorithm: windows_core::PWSTR,
    pub pAlgorithmPacket: super::minwindef::PBYTE,
    pub pKey: super::minwindef::PBYTE,
    pub pOtherData: super::minwindef::PBYTE,
    pub dwCreateTime: u32,
    pub dwExpireTime: u32,
    pub wMode: u16,
    pub wError: u16,
    pub wKeyLength: u16,
    pub wOtherLength: u16,
    pub cAlgNameLength: u8,
    pub bPacketPointers: windows_core::BOOL,
}
pub const DNS_TKEY_MODE_DIFFIE_HELLMAN: u32 = 2;
pub const DNS_TKEY_MODE_GSS: u32 = 3;
pub const DNS_TKEY_MODE_RESOLVER_ASSIGN: u32 = 4;
pub const DNS_TKEY_MODE_SERVER_ASSIGN: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_TLSA_DATA {
    pub bCertUsage: u8,
    pub bSelector: u8,
    pub bMatchingType: u8,
    pub bCertificateAssociationDataLength: u16,
    pub bPad: [u8; 3],
    pub bCertificateAssociationData: [u8; 1],
}
impl Default for DNS_TLSA_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
pub type DNS_TSIG_DATA = DNS_TSIG_DATAA;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_TSIG_DATAA {
    pub pNameAlgorithm: windows_core::PSTR,
    pub pAlgorithmPacket: super::minwindef::PBYTE,
    pub pSignature: super::minwindef::PBYTE,
    pub pOtherData: super::minwindef::PBYTE,
    pub i64CreateTime: i64,
    pub wFudgeTime: u16,
    pub wOriginalXid: u16,
    pub wError: u16,
    pub wSigLength: u16,
    pub wOtherLength: u16,
    pub cAlgNameLength: u8,
    pub bPacketPointers: windows_core::BOOL,
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_TSIG_DATAW {
    pub pNameAlgorithm: windows_core::PWSTR,
    pub pAlgorithmPacket: super::minwindef::PBYTE,
    pub pSignature: super::minwindef::PBYTE,
    pub pOtherData: super::minwindef::PBYTE,
    pub i64CreateTime: i64,
    pub wFudgeTime: u16,
    pub wOriginalXid: u16,
    pub wError: u16,
    pub wSigLength: u16,
    pub wOtherLength: u16,
    pub cAlgNameLength: u8,
    pub bPacketPointers: windows_core::BOOL,
}
pub type DNS_TXT_DATA = DNS_TXT_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_TXT_DATAA {
    pub dwStringCount: u32,
    pub pStringArray: [windows_core::PSTR; 1],
}
impl Default for DNS_TXT_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_TXT_DATAW {
    pub dwStringCount: u32,
    pub pStringArray: [windows_core::PWSTR; 1],
}
impl Default for DNS_TXT_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_TYPE_A: u32 = 1;
pub const DNS_TYPE_A6: u32 = 38;
pub const DNS_TYPE_AAAA: u32 = 28;
pub const DNS_TYPE_ADDRS: u32 = 248;
pub const DNS_TYPE_AFSDB: u32 = 18;
pub const DNS_TYPE_ALL: u32 = 255;
pub const DNS_TYPE_ANY: u32 = 255;
pub const DNS_TYPE_ATMA: u32 = 34;
pub const DNS_TYPE_AXFR: u32 = 252;
pub const DNS_TYPE_CERT: u32 = 37;
pub const DNS_TYPE_CNAME: u32 = 5;
pub const DNS_TYPE_DHCID: u32 = 49;
pub const DNS_TYPE_DNAME: u32 = 39;
pub const DNS_TYPE_DNSKEY: u32 = 48;
pub const DNS_TYPE_DS: u32 = 43;
pub const DNS_TYPE_EID: u32 = 31;
pub const DNS_TYPE_GID: u32 = 102;
pub const DNS_TYPE_GPOS: u32 = 27;
pub const DNS_TYPE_HINFO: u32 = 13;
pub const DNS_TYPE_HTTPS: u32 = 65;
pub const DNS_TYPE_ISDN: u32 = 20;
pub const DNS_TYPE_IXFR: u32 = 251;
pub const DNS_TYPE_KEY: u32 = 25;
pub const DNS_TYPE_KX: u32 = 36;
pub const DNS_TYPE_LOC: u32 = 29;
pub const DNS_TYPE_MAILA: u32 = 254;
pub const DNS_TYPE_MAILB: u32 = 253;
pub const DNS_TYPE_MB: u32 = 7;
pub const DNS_TYPE_MD: u32 = 3;
pub const DNS_TYPE_MF: u32 = 4;
pub const DNS_TYPE_MG: u32 = 8;
pub const DNS_TYPE_MINFO: u32 = 14;
pub const DNS_TYPE_MR: u32 = 9;
pub const DNS_TYPE_MX: u32 = 15;
pub const DNS_TYPE_NAPTR: u32 = 35;
pub const DNS_TYPE_NBSTAT: u32 = 65282;
pub const DNS_TYPE_NIMLOC: u32 = 32;
pub const DNS_TYPE_NS: u32 = 2;
pub const DNS_TYPE_NSAP: u32 = 22;
pub const DNS_TYPE_NSAPPTR: u32 = 23;
pub const DNS_TYPE_NSEC: u32 = 47;
pub const DNS_TYPE_NSEC3: u32 = 50;
pub const DNS_TYPE_NSEC3PARAM: u32 = 51;
pub const DNS_TYPE_NULL: u32 = 10;
pub const DNS_TYPE_NXT: u32 = 30;
pub const DNS_TYPE_OPT: u32 = 41;
pub const DNS_TYPE_PTR: u32 = 12;
pub const DNS_TYPE_PX: u32 = 26;
pub const DNS_TYPE_RP: u32 = 17;
pub const DNS_TYPE_RRSIG: u32 = 46;
pub const DNS_TYPE_RT: u32 = 21;
pub const DNS_TYPE_SIG: u32 = 24;
pub const DNS_TYPE_SINK: u32 = 40;
pub const DNS_TYPE_SOA: u32 = 6;
pub const DNS_TYPE_SRV: u32 = 33;
pub const DNS_TYPE_SVCB: u32 = 64;
pub const DNS_TYPE_TEXT: u32 = 16;
pub const DNS_TYPE_TKEY: u32 = 249;
pub const DNS_TYPE_TLSA: u32 = 52;
pub const DNS_TYPE_TSIG: u32 = 250;
pub const DNS_TYPE_UID: u32 = 101;
pub const DNS_TYPE_UINFO: u32 = 100;
pub const DNS_TYPE_UNSPEC: u32 = 103;
pub const DNS_TYPE_WINS: u32 = 65281;
pub const DNS_TYPE_WINSR: u32 = 65282;
pub const DNS_TYPE_WKS: u32 = 11;
pub const DNS_TYPE_X25: u32 = 19;
pub const DNS_TYPE_ZERO: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_UNKNOWN_DATA {
    pub dwByteCount: u32,
    pub bData: [u8; 1],
}
impl Default for DNS_UNKNOWN_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DNS_WINSR_DATA = DNS_WINSR_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_WINSR_DATAA {
    pub dwMappingFlag: u32,
    pub dwLookupTimeout: u32,
    pub dwCacheTimeout: u32,
    pub pNameResultDomain: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DNS_WINSR_DATAW {
    pub dwMappingFlag: u32,
    pub dwLookupTimeout: u32,
    pub dwCacheTimeout: u32,
    pub pNameResultDomain: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_WINS_DATA {
    pub dwMappingFlag: u32,
    pub dwLookupTimeout: u32,
    pub dwCacheTimeout: u32,
    pub cWinsServerCount: u32,
    pub WinsServers: [IP4_ADDRESS; 1],
}
impl Default for DNS_WINS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DNS_WKS_DATA {
    pub IpAddress: IP4_ADDRESS,
    pub chProtocol: u8,
    pub BitMask: [u8; 1],
}
impl Default for DNS_WKS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DnsCharSetAnsi: DNS_CHARSET = 3;
pub const DnsCharSetUnicode: DNS_CHARSET = 1;
pub const DnsCharSetUnknown: DNS_CHARSET = 0;
pub const DnsCharSetUtf8: DNS_CHARSET = 2;
pub const DnsSectionAddtional: DNS_SECTION = 3;
pub const DnsSectionAnswer: DNS_SECTION = 1;
pub const DnsSectionAuthority: DNS_SECTION = 2;
pub const DnsSectionPrereq: u32 = 1;
pub const DnsSectionQuestion: DNS_SECTION = 0;
pub const DnsSectionUpdate: u32 = 2;
pub const DnsSectionZone: u32 = 0;
pub const DnsSvcbParamAlpn: DNS_SVCB_PARAM_TYPE = 1;
pub const DnsSvcbParamDohPath: DNS_SVCB_PARAM_TYPE = 7;
pub const DnsSvcbParamDohPathOpenDns: DNS_SVCB_PARAM_TYPE = 65432;
pub const DnsSvcbParamEch: DNS_SVCB_PARAM_TYPE = 5;
pub const DnsSvcbParamIpv4Hint: DNS_SVCB_PARAM_TYPE = 4;
pub const DnsSvcbParamIpv6Hint: DNS_SVCB_PARAM_TYPE = 6;
pub const DnsSvcbParamMandatory: DNS_SVCB_PARAM_TYPE = 0;
pub const DnsSvcbParamNoDefaultAlpn: DNS_SVCB_PARAM_TYPE = 2;
pub const DnsSvcbParamPort: DNS_SVCB_PARAM_TYPE = 3;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct IP4_ADDRESS(pub u32);
pub const IP4_ADDRESS_STRING_BUFFER_LENGTH: u32 = 16;
pub const IP4_ADDRESS_STRING_LENGTH: u32 = 16;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union IP6_ADDRESS {
    pub IP6Dword: [u32; 4],
    pub IP6Word: [u16; 8],
    pub IP6Byte: [u8; 16],
}
#[cfg(target_arch = "x86")]
impl Default for IP6_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union IP6_ADDRESS {
    pub IP6Qword: [u64; 2],
    pub IP6Dword: [u32; 4],
    pub IP6Word: [u16; 8],
    pub IP6Byte: [u8; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for IP6_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_AAAA_DATA(pub *mut DNS_AAAA_DATA);
impl PDNS_AAAA_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_AAAA_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_ADDR(pub *mut DNS_ADDR);
impl PDNS_ADDR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_ADDR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_ADDR_ARRAY(pub *mut DNS_ADDR_ARRAY);
impl PDNS_ADDR_ARRAY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_ADDR_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_ATMA_DATA(pub *mut DNS_ATMA_DATA);
impl PDNS_ATMA_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_ATMA_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_A_DATA(pub *mut DNS_A_DATA);
impl PDNS_A_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_A_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_DHCID_DATA(pub *mut DNS_DHCID_DATA);
impl PDNS_DHCID_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_DHCID_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_DNSKEY_DATA(pub *mut DNS_DNSKEY_DATA);
impl PDNS_DNSKEY_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_DNSKEY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_DS_DATA(pub *mut DNS_DS_DATA);
impl PDNS_DS_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_DS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_HEADER(pub *mut DNS_HEADER);
impl PDNS_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_HEADER_EXT(pub *mut DNS_HEADER_EXT);
impl PDNS_HEADER_EXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_HEADER_EXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_KEY_DATA(pub *mut DNS_DNSKEY_DATA);
impl PDNS_KEY_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_KEY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_LOC_DATA(pub *mut DNS_LOC_DATA);
impl PDNS_LOC_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_LOC_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_MESSAGE_BUFFER(pub *mut DNS_MESSAGE_BUFFER);
impl PDNS_MESSAGE_BUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_MESSAGE_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_MINFO_DATA(pub *mut DNS_MINFO_DATAA);
impl PDNS_MINFO_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_MINFO_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_MINFO_DATAA(pub *mut DNS_MINFO_DATAA);
impl PDNS_MINFO_DATAA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_MINFO_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_MINFO_DATAW(pub *mut DNS_MINFO_DATAW);
impl PDNS_MINFO_DATAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_MINFO_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_MX_DATA(pub *mut DNS_MX_DATAA);
impl PDNS_MX_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_MX_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_MX_DATAA(pub *mut DNS_MX_DATAA);
impl PDNS_MX_DATAA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_MX_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_MX_DATAW(pub *mut DNS_MX_DATAW);
impl PDNS_MX_DATAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_MX_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_NAPTR_DATA(pub *mut DNS_NAPTR_DATAA);
impl PDNS_NAPTR_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_NAPTR_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_NAPTR_DATAA(pub *mut DNS_NAPTR_DATAA);
impl PDNS_NAPTR_DATAA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_NAPTR_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_NAPTR_DATAW(pub *mut DNS_NAPTR_DATAW);
impl PDNS_NAPTR_DATAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_NAPTR_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_NSEC3PARAM_DATA(pub *mut DNS_NSEC3PARAM_DATA);
impl PDNS_NSEC3PARAM_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_NSEC3PARAM_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_NSEC3_DATA(pub *mut DNS_NSEC3_DATA);
impl PDNS_NSEC3_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_NSEC3_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_NSEC_DATA(pub *mut DNS_NSEC_DATAA);
impl PDNS_NSEC_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_NSEC_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_NSEC_DATAA(pub *mut DNS_NSEC_DATAA);
impl PDNS_NSEC_DATAA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_NSEC_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_NSEC_DATAW(pub *mut DNS_NSEC_DATAW);
impl PDNS_NSEC_DATAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_NSEC_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_NULL_DATA(pub *mut DNS_NULL_DATA);
impl PDNS_NULL_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_NULL_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_NXT_DATA(pub *mut DNS_NXT_DATAA);
impl PDNS_NXT_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_NXT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_NXT_DATAA(pub *mut DNS_NXT_DATAA);
impl PDNS_NXT_DATAA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_NXT_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_NXT_DATAW(pub *mut DNS_NXT_DATAW);
impl PDNS_NXT_DATAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_NXT_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_OPT_DATA(pub *mut DNS_OPT_DATA);
impl PDNS_OPT_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_OPT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_PTR_DATA(pub *mut DNS_PTR_DATAA);
impl PDNS_PTR_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_PTR_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_PTR_DATAA(pub *mut DNS_PTR_DATAA);
impl PDNS_PTR_DATAA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_PTR_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_PTR_DATAW(pub *mut DNS_PTR_DATAW);
impl PDNS_PTR_DATAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_PTR_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_RECORD(pub *mut DNS_RECORDA);
#[cfg(feature = "Win32_minwindef")]
impl PDNS_RECORD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PDNS_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_RECORDA(pub *mut DNS_RECORDA);
#[cfg(feature = "Win32_minwindef")]
impl PDNS_RECORDA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PDNS_RECORDA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_RECORDW(pub *mut DNS_RECORDW);
#[cfg(feature = "Win32_minwindef")]
impl PDNS_RECORDW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PDNS_RECORDW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_RECORD_OPT(pub *mut DNS_RECORD_OPTA);
#[cfg(feature = "Win32_minwindef")]
impl PDNS_RECORD_OPT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PDNS_RECORD_OPT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_RECORD_OPTA(pub *mut DNS_RECORD_OPTA);
#[cfg(feature = "Win32_minwindef")]
impl PDNS_RECORD_OPTA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PDNS_RECORD_OPTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_RECORD_OPTW(pub *mut DNS_RECORD_OPTW);
#[cfg(feature = "Win32_minwindef")]
impl PDNS_RECORD_OPTW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PDNS_RECORD_OPTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_RRSIG_DATA(pub *mut DNS_RRSIG_DATAA);
impl PDNS_RRSIG_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_RRSIG_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_RRSIG_DATAA(pub *mut DNS_RRSIG_DATAA);
impl PDNS_RRSIG_DATAA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_RRSIG_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_RRSIG_DATAW(pub *mut DNS_RRSIG_DATAW);
impl PDNS_RRSIG_DATAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_RRSIG_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_SIG_DATA(pub *mut DNS_SIG_DATAA);
impl PDNS_SIG_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_SIG_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_SIG_DATAA(pub *mut DNS_RRSIG_DATAA);
impl PDNS_SIG_DATAA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_SIG_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_SIG_DATAW(pub *mut DNS_RRSIG_DATAW);
impl PDNS_SIG_DATAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_SIG_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_SOA_DATA(pub *mut DNS_SOA_DATAA);
impl PDNS_SOA_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_SOA_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_SOA_DATAA(pub *mut DNS_SOA_DATAA);
impl PDNS_SOA_DATAA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_SOA_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_SOA_DATAW(pub *mut DNS_SOA_DATAW);
impl PDNS_SOA_DATAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_SOA_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_SRV_DATA(pub *mut DNS_SRV_DATAA);
impl PDNS_SRV_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_SRV_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_SRV_DATAA(pub *mut DNS_SRV_DATAA);
impl PDNS_SRV_DATAA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_SRV_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_SRV_DATAW(pub *mut DNS_SRV_DATAW);
impl PDNS_SRV_DATAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_SRV_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_STATUS(pub *mut DNS_STATUS);
impl PDNS_STATUS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_TKEY_DATA(pub *mut DNS_TKEY_DATAA);
#[cfg(feature = "Win32_minwindef")]
impl PDNS_TKEY_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PDNS_TKEY_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_TKEY_DATAA(pub *mut DNS_TKEY_DATAA);
#[cfg(feature = "Win32_minwindef")]
impl PDNS_TKEY_DATAA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PDNS_TKEY_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_TKEY_DATAW(pub *mut DNS_TKEY_DATAW);
#[cfg(feature = "Win32_minwindef")]
impl PDNS_TKEY_DATAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PDNS_TKEY_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_TLSA_DATA(pub *mut DNS_TLSA_DATA);
impl PDNS_TLSA_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_TLSA_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_TSIG_DATA(pub *mut DNS_TSIG_DATAA);
#[cfg(feature = "Win32_minwindef")]
impl PDNS_TSIG_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PDNS_TSIG_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_TSIG_DATAA(pub *mut DNS_TSIG_DATAA);
#[cfg(feature = "Win32_minwindef")]
impl PDNS_TSIG_DATAA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PDNS_TSIG_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_TSIG_DATAW(pub *mut DNS_TSIG_DATAW);
#[cfg(feature = "Win32_minwindef")]
impl PDNS_TSIG_DATAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PDNS_TSIG_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_TXT_DATA(pub *mut DNS_TXT_DATAA);
impl PDNS_TXT_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_TXT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_TXT_DATAA(pub *mut DNS_TXT_DATAA);
impl PDNS_TXT_DATAA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_TXT_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_TXT_DATAW(pub *mut DNS_TXT_DATAW);
impl PDNS_TXT_DATAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_TXT_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_UNKNOWN_DATA(pub *mut DNS_UNKNOWN_DATA);
impl PDNS_UNKNOWN_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_UNKNOWN_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_WINSR_DATA(pub *mut DNS_WINSR_DATAA);
impl PDNS_WINSR_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_WINSR_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_WINSR_DATAA(pub *mut DNS_WINSR_DATAA);
impl PDNS_WINSR_DATAA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_WINSR_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_WINSR_DATAW(pub *mut DNS_WINSR_DATAW);
impl PDNS_WINSR_DATAW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_WINSR_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_WINS_DATA(pub *mut DNS_WINS_DATA);
impl PDNS_WINS_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_WINS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDNS_WKS_DATA(pub *mut DNS_WKS_DATA);
impl PDNS_WKS_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDNS_WKS_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIP4_ADDRESS(pub *mut u32);
impl PIP4_ADDRESS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIP4_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIP6_ADDRESS(pub *mut IP6_ADDRESS);
impl PIP6_ADDRESS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIP6_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PQWORD(pub *mut u64);
impl PQWORD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PQWORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const SIZEOF_DNS_RECORD_HEADER: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SIZEOF_DNS_RECORD_HEADER: u32 = 32;
pub const SIZEOF_IP4_ADDRESS: u32 = 4;
