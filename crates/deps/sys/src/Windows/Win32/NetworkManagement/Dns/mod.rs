#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    pub fn DnsAcquireContextHandle_A(credentialflags: u32, credentials: *const ::core::ffi::c_void, pcontext: *mut DnsContextHandle) -> i32;
    pub fn DnsAcquireContextHandle_W(credentialflags: u32, credentials: *const ::core::ffi::c_void, pcontext: *mut DnsContextHandle) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsCancelQuery(pcancelhandle: *const DNS_QUERY_CANCEL) -> i32;
    pub fn DnsConnectionDeletePolicyEntries(policyentrytag: DNS_CONNECTION_POLICY_TAG) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionDeleteProxyInfo(pwszconnectionname: super::super::Foundation::PWSTR, r#type: DNS_CONNECTION_PROXY_TYPE) -> u32;
    pub fn DnsConnectionFreeNameList(pnamelist: *mut DNS_CONNECTION_NAME_LIST);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionFreeProxyInfo(pproxyinfo: *mut DNS_CONNECTION_PROXY_INFO);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionFreeProxyInfoEx(pproxyinfoex: *mut DNS_CONNECTION_PROXY_INFO_EX);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionFreeProxyList(pproxylist: *mut DNS_CONNECTION_PROXY_LIST);
    pub fn DnsConnectionGetNameList(pnamelist: *mut DNS_CONNECTION_NAME_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionGetProxyInfo(pwszconnectionname: super::super::Foundation::PWSTR, r#type: DNS_CONNECTION_PROXY_TYPE, pproxyinfo: *mut DNS_CONNECTION_PROXY_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionGetProxyInfoForHostUrl(pwszhosturl: super::super::Foundation::PWSTR, pselectioncontext: *const u8, dwselectioncontextlength: u32, dwexplicitinterfaceindex: u32, pproxyinfoex: *mut DNS_CONNECTION_PROXY_INFO_EX) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionGetProxyList(pwszconnectionname: super::super::Foundation::PWSTR, pproxylist: *mut DNS_CONNECTION_PROXY_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionSetPolicyEntries(policyentrytag: DNS_CONNECTION_POLICY_TAG, ppolicyentrylist: *const DNS_CONNECTION_POLICY_ENTRY_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionSetProxyInfo(pwszconnectionname: super::super::Foundation::PWSTR, r#type: DNS_CONNECTION_PROXY_TYPE, pproxyinfo: *const DNS_CONNECTION_PROXY_INFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionUpdateIfIndexTable(pconnectionifindexentries: *const DNS_CONNECTION_IFINDEX_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsExtractRecordsFromMessage_UTF8(pdnsbuffer: *const DNS_MESSAGE_BUFFER, wmessagelength: u16, pprecord: *mut *mut DNS_RECORDA) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsExtractRecordsFromMessage_W(pdnsbuffer: *const DNS_MESSAGE_BUFFER, wmessagelength: u16, pprecord: *mut *mut DNS_RECORDA) -> i32;
    pub fn DnsFree(pdata: *const ::core::ffi::c_void, freetype: DNS_FREE_TYPE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsFreeCustomServers(pcservers: *mut u32, ppservers: *mut *mut DNS_CUSTOM_SERVER);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsFreeProxyName(proxyname: super::super::Foundation::PWSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsGetApplicationSettings(pcservers: *mut u32, ppdefaultservers: *mut *mut DNS_CUSTOM_SERVER, psettings: *mut DNS_APPLICATION_SETTINGS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsGetProxyInformation(hostname: super::super::Foundation::PWSTR, proxyinformation: *mut DNS_PROXY_INFORMATION, defaultproxyinformation: *mut DNS_PROXY_INFORMATION, completionroutine: DNS_PROXY_COMPLETION_ROUTINE, completioncontext: *const ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsModifyRecordsInSet_A(paddrecords: *const DNS_RECORDA, pdeleterecords: *const DNS_RECORDA, options: u32, hcredentials: super::super::Foundation::HANDLE, pextralist: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsModifyRecordsInSet_UTF8(paddrecords: *const DNS_RECORDA, pdeleterecords: *const DNS_RECORDA, options: u32, hcredentials: super::super::Foundation::HANDLE, pextralist: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsModifyRecordsInSet_W(paddrecords: *const DNS_RECORDA, pdeleterecords: *const DNS_RECORDA, options: u32, hcredentials: super::super::Foundation::HANDLE, pextralist: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsNameCompare_A(pname1: super::super::Foundation::PSTR, pname2: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsNameCompare_W(pname1: super::super::Foundation::PWSTR, pname2: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQueryConfig(config: DNS_CONFIG_TYPE, flag: u32, pwsadaptername: super::super::Foundation::PWSTR, preserved: *const ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void, pbuflen: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQueryEx(pqueryrequest: *const DNS_QUERY_REQUEST, pqueryresults: *mut DNS_QUERY_RESULT, pcancelhandle: *mut DNS_QUERY_CANCEL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQuery_A(pszname: super::super::Foundation::PSTR, wtype: u16, options: u32, pextra: *mut ::core::ffi::c_void, ppqueryresults: *mut *mut DNS_RECORDA, preserved: *mut *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQuery_UTF8(pszname: super::super::Foundation::PSTR, wtype: u16, options: u32, pextra: *mut ::core::ffi::c_void, ppqueryresults: *mut *mut DNS_RECORDA, preserved: *mut *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQuery_W(pszname: super::super::Foundation::PWSTR, wtype: u16, options: u32, pextra: *mut ::core::ffi::c_void, ppqueryresults: *mut *mut DNS_RECORDA, preserved: *mut *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordCompare(precord1: *const DNS_RECORDA, precord2: *const DNS_RECORDA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordCopyEx(precord: *const DNS_RECORDA, charsetin: DNS_CHARSET, charsetout: DNS_CHARSET) -> *mut DNS_RECORDA;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordSetCompare(prr1: *mut DNS_RECORDA, prr2: *mut DNS_RECORDA, ppdiff1: *mut *mut DNS_RECORDA, ppdiff2: *mut *mut DNS_RECORDA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordSetCopyEx(precordset: *const DNS_RECORDA, charsetin: DNS_CHARSET, charsetout: DNS_CHARSET) -> *mut DNS_RECORDA;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordSetDetach(precordlist: *mut DNS_RECORDA) -> *mut DNS_RECORDA;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsReleaseContextHandle(hcontext: super::super::Foundation::HANDLE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsReplaceRecordSetA(preplaceset: *const DNS_RECORDA, options: u32, hcontext: super::super::Foundation::HANDLE, pextrainfo: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsReplaceRecordSetUTF8(preplaceset: *const DNS_RECORDA, options: u32, hcontext: super::super::Foundation::HANDLE, pextrainfo: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsReplaceRecordSetW(preplaceset: *const DNS_RECORDA, options: u32, hcontext: super::super::Foundation::HANDLE, pextrainfo: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceBrowse(prequest: *const DNS_SERVICE_BROWSE_REQUEST, pcancel: *mut DNS_SERVICE_CANCEL) -> i32;
    pub fn DnsServiceBrowseCancel(pcancelhandle: *const DNS_SERVICE_CANCEL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceConstructInstance(pservicename: super::super::Foundation::PWSTR, phostname: super::super::Foundation::PWSTR, pip4: *const u32, pip6: *const IP6_ADDRESS, wport: u16, wpriority: u16, wweight: u16, dwpropertiescount: u32, keys: *const super::super::Foundation::PWSTR, values: *const super::super::Foundation::PWSTR) -> *mut DNS_SERVICE_INSTANCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceCopyInstance(porig: *const DNS_SERVICE_INSTANCE) -> *mut DNS_SERVICE_INSTANCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceDeRegister(prequest: *const DNS_SERVICE_REGISTER_REQUEST, pcancel: *mut DNS_SERVICE_CANCEL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceFreeInstance(pinstance: *const DNS_SERVICE_INSTANCE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceRegister(prequest: *const DNS_SERVICE_REGISTER_REQUEST, pcancel: *mut DNS_SERVICE_CANCEL) -> u32;
    pub fn DnsServiceRegisterCancel(pcancelhandle: *const DNS_SERVICE_CANCEL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceResolve(prequest: *const DNS_SERVICE_RESOLVE_REQUEST, pcancel: *mut DNS_SERVICE_CANCEL) -> i32;
    pub fn DnsServiceResolveCancel(pcancelhandle: *const DNS_SERVICE_CANCEL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsSetApplicationSettings(cservers: u32, pservers: *const DNS_CUSTOM_SERVER, psettings: *const DNS_APPLICATION_SETTINGS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsStartMulticastQuery(pqueryrequest: *const MDNS_QUERY_REQUEST, phandle: *mut MDNS_QUERY_HANDLE) -> i32;
    pub fn DnsStopMulticastQuery(phandle: *mut MDNS_QUERY_HANDLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsValidateName_A(pszname: super::super::Foundation::PSTR, format: DNS_NAME_FORMAT) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsValidateName_UTF8(pszname: super::super::Foundation::PSTR, format: DNS_NAME_FORMAT) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsValidateName_W(pszname: super::super::Foundation::PWSTR, format: DNS_NAME_FORMAT) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsWriteQuestionToBuffer_UTF8(pdnsbuffer: *mut DNS_MESSAGE_BUFFER, pdwbuffersize: *mut u32, pszname: super::super::Foundation::PSTR, wtype: u16, xid: u16, frecursiondesired: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsWriteQuestionToBuffer_W(pdnsbuffer: *mut DNS_MESSAGE_BUFFER, pdwbuffersize: *mut u32, pszname: super::super::Foundation::PWSTR, wtype: u16, xid: u16, frecursiondesired: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
}
pub const DNSREC_ADDITIONAL: u32 = 3u32;
pub const DNSREC_ANSWER: u32 = 1u32;
pub const DNSREC_AUTHORITY: u32 = 2u32;
pub const DNSREC_DELETE: u32 = 4u32;
pub const DNSREC_NOEXIST: u32 = 4u32;
pub const DNSREC_PREREQ: u32 = 1u32;
pub const DNSREC_QUESTION: u32 = 0u32;
pub const DNSREC_SECTION: u32 = 3u32;
pub const DNSREC_UPDATE: u32 = 2u32;
pub const DNSREC_ZONE: u32 = 0u32;
pub const DNSSEC_ALGORITHM_ECDSAP256_SHA256: u32 = 13u32;
pub const DNSSEC_ALGORITHM_ECDSAP384_SHA384: u32 = 14u32;
pub const DNSSEC_ALGORITHM_NULL: u32 = 253u32;
pub const DNSSEC_ALGORITHM_PRIVATE: u32 = 254u32;
pub const DNSSEC_ALGORITHM_RSAMD5: u32 = 1u32;
pub const DNSSEC_ALGORITHM_RSASHA1: u32 = 5u32;
pub const DNSSEC_ALGORITHM_RSASHA1_NSEC3: u32 = 7u32;
pub const DNSSEC_ALGORITHM_RSASHA256: u32 = 8u32;
pub const DNSSEC_ALGORITHM_RSASHA512: u32 = 10u32;
pub const DNSSEC_DIGEST_ALGORITHM_SHA1: u32 = 1u32;
pub const DNSSEC_DIGEST_ALGORITHM_SHA256: u32 = 2u32;
pub const DNSSEC_DIGEST_ALGORITHM_SHA384: u32 = 4u32;
pub const DNSSEC_KEY_FLAG_EXTEND: u32 = 8u32;
pub const DNSSEC_KEY_FLAG_FLAG10: u32 = 1024u32;
pub const DNSSEC_KEY_FLAG_FLAG11: u32 = 2048u32;
pub const DNSSEC_KEY_FLAG_FLAG2: u32 = 4u32;
pub const DNSSEC_KEY_FLAG_FLAG4: u32 = 16u32;
pub const DNSSEC_KEY_FLAG_FLAG5: u32 = 32u32;
pub const DNSSEC_KEY_FLAG_FLAG8: u32 = 256u32;
pub const DNSSEC_KEY_FLAG_FLAG9: u32 = 512u32;
pub const DNSSEC_KEY_FLAG_HOST: u32 = 128u32;
pub const DNSSEC_KEY_FLAG_NOAUTH: u32 = 1u32;
pub const DNSSEC_KEY_FLAG_NOCONF: u32 = 2u32;
pub const DNSSEC_KEY_FLAG_NTPE3: u32 = 192u32;
pub const DNSSEC_KEY_FLAG_SIG0: u32 = 0u32;
pub const DNSSEC_KEY_FLAG_SIG1: u32 = 4096u32;
pub const DNSSEC_KEY_FLAG_SIG10: u32 = 40960u32;
pub const DNSSEC_KEY_FLAG_SIG11: u32 = 45056u32;
pub const DNSSEC_KEY_FLAG_SIG12: u32 = 49152u32;
pub const DNSSEC_KEY_FLAG_SIG13: u32 = 53248u32;
pub const DNSSEC_KEY_FLAG_SIG14: u32 = 57344u32;
pub const DNSSEC_KEY_FLAG_SIG15: u32 = 61440u32;
pub const DNSSEC_KEY_FLAG_SIG2: u32 = 8192u32;
pub const DNSSEC_KEY_FLAG_SIG3: u32 = 12288u32;
pub const DNSSEC_KEY_FLAG_SIG4: u32 = 16384u32;
pub const DNSSEC_KEY_FLAG_SIG5: u32 = 20480u32;
pub const DNSSEC_KEY_FLAG_SIG6: u32 = 24576u32;
pub const DNSSEC_KEY_FLAG_SIG7: u32 = 28672u32;
pub const DNSSEC_KEY_FLAG_SIG8: u32 = 32768u32;
pub const DNSSEC_KEY_FLAG_SIG9: u32 = 36864u32;
pub const DNSSEC_KEY_FLAG_USER: u32 = 0u32;
pub const DNSSEC_KEY_FLAG_ZONE: u32 = 64u32;
pub const DNSSEC_PROTOCOL_DNSSEC: u32 = 3u32;
pub const DNSSEC_PROTOCOL_EMAIL: u32 = 2u32;
pub const DNSSEC_PROTOCOL_IPSEC: u32 = 4u32;
pub const DNSSEC_PROTOCOL_NONE: u32 = 0u32;
pub const DNSSEC_PROTOCOL_TLS: u32 = 1u32;
#[repr(C)]
pub struct DNS_AAAA_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_ADDR(i32);
pub const DNS_ADDRESS_STRING_LENGTH: u32 = 65u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_ADDR_ARRAY(i32);
pub const DNS_ADDR_MAX_SOCKADDR_LENGTH: u32 = 32u32;
#[repr(C)]
pub struct DNS_APPLICATION_SETTINGS(i32);
pub const DNS_APP_SETTINGS_EXCLUSIVE_SERVERS: u32 = 1u32;
pub const DNS_APP_SETTINGS_VERSION1: u32 = 1u32;
pub const DNS_ATMA_AESA_ADDR_LENGTH: u32 = 20u32;
#[repr(C)]
pub struct DNS_ATMA_DATA(i32);
pub const DNS_ATMA_FORMAT_AESA: u32 = 2u32;
pub const DNS_ATMA_FORMAT_E164: u32 = 1u32;
pub const DNS_ATMA_MAX_ADDR_LENGTH: u32 = 20u32;
pub const DNS_ATMA_MAX_RECORD_LENGTH: u32 = 21u32;
#[repr(C)]
pub struct DNS_A_DATA(i32);
#[repr(transparent)]
pub struct DNS_CHARSET(pub i32);
pub const DnsCharSetUnknown: DNS_CHARSET = DNS_CHARSET(0i32);
pub const DnsCharSetUnicode: DNS_CHARSET = DNS_CHARSET(1i32);
pub const DnsCharSetUtf8: DNS_CHARSET = DNS_CHARSET(2i32);
pub const DnsCharSetAnsi: DNS_CHARSET = DNS_CHARSET(3i32);
pub const DNS_CLASS_ALL: u32 = 255u32;
pub const DNS_CLASS_ANY: u32 = 255u32;
pub const DNS_CLASS_CHAOS: u32 = 3u32;
pub const DNS_CLASS_CSNET: u32 = 2u32;
pub const DNS_CLASS_HESIOD: u32 = 4u32;
pub const DNS_CLASS_INTERNET: u32 = 1u32;
pub const DNS_CLASS_NONE: u32 = 254u32;
pub const DNS_CLASS_UNICAST_RESPONSE: u32 = 32768u32;
pub const DNS_COMPRESSED_QUESTION_NAME: u32 = 49164u32;
pub const DNS_CONFIG_FLAG_ALLOC: u32 = 1u32;
#[repr(transparent)]
pub struct DNS_CONFIG_TYPE(pub i32);
pub const DnsConfigPrimaryDomainName_W: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(0i32);
pub const DnsConfigPrimaryDomainName_A: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(1i32);
pub const DnsConfigPrimaryDomainName_UTF8: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(2i32);
pub const DnsConfigAdapterDomainName_W: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(3i32);
pub const DnsConfigAdapterDomainName_A: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(4i32);
pub const DnsConfigAdapterDomainName_UTF8: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(5i32);
pub const DnsConfigDnsServerList: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(6i32);
pub const DnsConfigSearchList: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(7i32);
pub const DnsConfigAdapterInfo: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(8i32);
pub const DnsConfigPrimaryHostNameRegistrationEnabled: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(9i32);
pub const DnsConfigAdapterHostNameRegistrationEnabled: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(10i32);
pub const DnsConfigAddressRegistrationMaxCount: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(11i32);
pub const DnsConfigHostName_W: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(12i32);
pub const DnsConfigHostName_A: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(13i32);
pub const DnsConfigHostName_UTF8: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(14i32);
pub const DnsConfigFullHostName_W: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(15i32);
pub const DnsConfigFullHostName_A: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(16i32);
pub const DnsConfigFullHostName_UTF8: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(17i32);
pub const DnsConfigNameServer: DNS_CONFIG_TYPE = DNS_CONFIG_TYPE(18i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_CONNECTION_IFINDEX_ENTRY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_CONNECTION_IFINDEX_LIST(i32);
#[repr(C)]
pub struct DNS_CONNECTION_NAME(i32);
#[repr(C)]
pub struct DNS_CONNECTION_NAME_LIST(i32);
pub const DNS_CONNECTION_NAME_MAX_LENGTH: u32 = 64u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_CONNECTION_POLICY_ENTRY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_CONNECTION_POLICY_ENTRY_LIST(i32);
pub const DNS_CONNECTION_POLICY_ENTRY_ONDEMAND: u32 = 1u32;
#[repr(transparent)]
pub struct DNS_CONNECTION_POLICY_TAG(pub i32);
pub const TAG_DNS_CONNECTION_POLICY_TAG_DEFAULT: DNS_CONNECTION_POLICY_TAG = DNS_CONNECTION_POLICY_TAG(0i32);
pub const TAG_DNS_CONNECTION_POLICY_TAG_CONNECTION_MANAGER: DNS_CONNECTION_POLICY_TAG = DNS_CONNECTION_POLICY_TAG(1i32);
pub const TAG_DNS_CONNECTION_POLICY_TAG_WWWPT: DNS_CONNECTION_POLICY_TAG = DNS_CONNECTION_POLICY_TAG(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_CONNECTION_PROXY_ELEMENT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_CONNECTION_PROXY_INFO(i32);
pub const DNS_CONNECTION_PROXY_INFO_CURRENT_VERSION: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_CONNECTION_PROXY_INFO_EX(i32);
pub const DNS_CONNECTION_PROXY_INFO_EXCEPTION_MAX_LENGTH: u32 = 1024u32;
pub const DNS_CONNECTION_PROXY_INFO_EXTRA_INFO_MAX_LENGTH: u32 = 1024u32;
pub const DNS_CONNECTION_PROXY_INFO_FLAG_BYPASSLOCAL: u32 = 2u32;
pub const DNS_CONNECTION_PROXY_INFO_FLAG_DISABLED: u32 = 1u32;
pub const DNS_CONNECTION_PROXY_INFO_FRIENDLY_NAME_MAX_LENGTH: u32 = 64u32;
pub const DNS_CONNECTION_PROXY_INFO_PASSWORD_MAX_LENGTH: u32 = 128u32;
pub const DNS_CONNECTION_PROXY_INFO_SERVER_MAX_LENGTH: u32 = 256u32;
#[repr(transparent)]
pub struct DNS_CONNECTION_PROXY_INFO_SWITCH(pub i32);
pub const DNS_CONNECTION_PROXY_INFO_SWITCH_CONFIG: DNS_CONNECTION_PROXY_INFO_SWITCH = DNS_CONNECTION_PROXY_INFO_SWITCH(0i32);
pub const DNS_CONNECTION_PROXY_INFO_SWITCH_SCRIPT: DNS_CONNECTION_PROXY_INFO_SWITCH = DNS_CONNECTION_PROXY_INFO_SWITCH(1i32);
pub const DNS_CONNECTION_PROXY_INFO_SWITCH_WPAD: DNS_CONNECTION_PROXY_INFO_SWITCH = DNS_CONNECTION_PROXY_INFO_SWITCH(2i32);
pub const DNS_CONNECTION_PROXY_INFO_USERNAME_MAX_LENGTH: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_CONNECTION_PROXY_LIST(i32);
#[repr(transparent)]
pub struct DNS_CONNECTION_PROXY_TYPE(pub i32);
pub const DNS_CONNECTION_PROXY_TYPE_NULL: DNS_CONNECTION_PROXY_TYPE = DNS_CONNECTION_PROXY_TYPE(0i32);
pub const DNS_CONNECTION_PROXY_TYPE_HTTP: DNS_CONNECTION_PROXY_TYPE = DNS_CONNECTION_PROXY_TYPE(1i32);
pub const DNS_CONNECTION_PROXY_TYPE_WAP: DNS_CONNECTION_PROXY_TYPE = DNS_CONNECTION_PROXY_TYPE(2i32);
pub const DNS_CONNECTION_PROXY_TYPE_SOCKS4: DNS_CONNECTION_PROXY_TYPE = DNS_CONNECTION_PROXY_TYPE(4i32);
pub const DNS_CONNECTION_PROXY_TYPE_SOCKS5: DNS_CONNECTION_PROXY_TYPE = DNS_CONNECTION_PROXY_TYPE(5i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_CUSTOM_SERVER(i32);
pub const DNS_CUSTOM_SERVER_TYPE_DOH: u32 = 2u32;
pub const DNS_CUSTOM_SERVER_TYPE_UDP: u32 = 1u32;
pub const DNS_CUSTOM_SERVER_UDP_FALLBACK: u32 = 1u32;
#[repr(C)]
pub struct DNS_DHCID_DATA(i32);
#[repr(C)]
pub struct DNS_DS_DATA(i32);
#[repr(transparent)]
pub struct DNS_FREE_TYPE(pub i32);
pub const DnsFreeFlat: DNS_FREE_TYPE = DNS_FREE_TYPE(0i32);
pub const DnsFreeRecordList: DNS_FREE_TYPE = DNS_FREE_TYPE(1i32);
pub const DnsFreeParsedMessageFields: DNS_FREE_TYPE = DNS_FREE_TYPE(2i32);
#[repr(C)]
pub struct DNS_HEADER(i32);
#[repr(C)]
pub struct DNS_HEADER_EXT(i32);
#[repr(C)]
pub struct DNS_KEY_DATA(i32);
#[repr(C)]
pub struct DNS_LOC_DATA(i32);
pub const DNS_MAX_IP4_REVERSE_NAME_BUFFER_LENGTH: u32 = 31u32;
pub const DNS_MAX_IP4_REVERSE_NAME_LENGTH: u32 = 31u32;
pub const DNS_MAX_IP6_REVERSE_NAME_BUFFER_LENGTH: u32 = 75u32;
pub const DNS_MAX_IP6_REVERSE_NAME_LENGTH: u32 = 75u32;
pub const DNS_MAX_LABEL_BUFFER_LENGTH: u32 = 64u32;
pub const DNS_MAX_LABEL_LENGTH: u32 = 63u32;
pub const DNS_MAX_NAME_BUFFER_LENGTH: u32 = 256u32;
pub const DNS_MAX_NAME_LENGTH: u32 = 255u32;
pub const DNS_MAX_REVERSE_NAME_BUFFER_LENGTH: u32 = 75u32;
pub const DNS_MAX_REVERSE_NAME_LENGTH: u32 = 75u32;
pub const DNS_MAX_TEXT_STRING_LENGTH: u32 = 255u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_MESSAGE_BUFFER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_MINFO_DATAA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_MINFO_DATAW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_MX_DATAA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_MX_DATAW(i32);
#[repr(transparent)]
pub struct DNS_NAME_FORMAT(pub i32);
pub const DnsNameDomain: DNS_NAME_FORMAT = DNS_NAME_FORMAT(0i32);
pub const DnsNameDomainLabel: DNS_NAME_FORMAT = DNS_NAME_FORMAT(1i32);
pub const DnsNameHostnameFull: DNS_NAME_FORMAT = DNS_NAME_FORMAT(2i32);
pub const DnsNameHostnameLabel: DNS_NAME_FORMAT = DNS_NAME_FORMAT(3i32);
pub const DnsNameWildcard: DNS_NAME_FORMAT = DNS_NAME_FORMAT(4i32);
pub const DnsNameSrvRecord: DNS_NAME_FORMAT = DNS_NAME_FORMAT(5i32);
pub const DnsNameValidateTld: DNS_NAME_FORMAT = DNS_NAME_FORMAT(6i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_NAPTR_DATAA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_NAPTR_DATAW(i32);
#[repr(C)]
pub struct DNS_NSEC3PARAM_DATA(i32);
#[repr(C)]
pub struct DNS_NSEC3_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_NSEC_DATAA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_NSEC_DATAW(i32);
#[repr(C)]
pub struct DNS_NULL_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_NXT_DATAA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_NXT_DATAW(i32);
pub const DNS_OPCODE_IQUERY: u32 = 1u32;
pub const DNS_OPCODE_NOTIFY: u32 = 4u32;
pub const DNS_OPCODE_QUERY: u32 = 0u32;
pub const DNS_OPCODE_SERVER_STATUS: u32 = 2u32;
pub const DNS_OPCODE_UNKNOWN: u32 = 3u32;
pub const DNS_OPCODE_UPDATE: u32 = 5u32;
#[repr(C)]
pub struct DNS_OPT_DATA(i32);
pub const DNS_PORT_HOST_ORDER: u32 = 53u32;
pub const DNS_PORT_NET_ORDER: u32 = 13568u32;
pub type DNS_PROXY_COMPLETION_ROUTINE = unsafe extern "system" fn(completioncontext: *const ::core::ffi::c_void, status: i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_PROXY_INFORMATION(i32);
#[repr(transparent)]
pub struct DNS_PROXY_INFORMATION_TYPE(pub i32);
pub const DNS_PROXY_INFORMATION_DIRECT: DNS_PROXY_INFORMATION_TYPE = DNS_PROXY_INFORMATION_TYPE(0i32);
pub const DNS_PROXY_INFORMATION_DEFAULT_SETTINGS: DNS_PROXY_INFORMATION_TYPE = DNS_PROXY_INFORMATION_TYPE(1i32);
pub const DNS_PROXY_INFORMATION_PROXY_NAME: DNS_PROXY_INFORMATION_TYPE = DNS_PROXY_INFORMATION_TYPE(2i32);
pub const DNS_PROXY_INFORMATION_DOES_NOT_EXIST: DNS_PROXY_INFORMATION_TYPE = DNS_PROXY_INFORMATION_TYPE(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_PTR_DATAA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_PTR_DATAW(i32);
pub const DNS_QUERY_ACCEPT_TRUNCATED_RESPONSE: u32 = 1u32;
pub const DNS_QUERY_ADDRCONFIG: u32 = 8192u32;
pub const DNS_QUERY_APPEND_MULTILABEL: u32 = 8388608u32;
pub const DNS_QUERY_BYPASS_CACHE: u32 = 8u32;
pub const DNS_QUERY_CACHE_ONLY: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_QUERY_CANCEL(i32);
pub const DNS_QUERY_DISABLE_IDN_ENCODING: u32 = 2097152u32;
pub const DNS_QUERY_DNSSEC_CHECKING_DISABLED: u32 = 33554432u32;
pub const DNS_QUERY_DNSSEC_OK: u32 = 16777216u32;
pub const DNS_QUERY_DONT_RESET_TTL_VALUES: u32 = 1048576u32;
pub const DNS_QUERY_DUAL_ADDR: u32 = 16384u32;
pub const DNS_QUERY_MULTICAST_ONLY: u32 = 1024u32;
pub const DNS_QUERY_NO_HOSTS_FILE: u32 = 64u32;
pub const DNS_QUERY_NO_LOCAL_NAME: u32 = 32u32;
pub const DNS_QUERY_NO_MULTICAST: u32 = 2048u32;
pub const DNS_QUERY_NO_NETBT: u32 = 128u32;
pub const DNS_QUERY_NO_RECURSION: u32 = 4u32;
pub const DNS_QUERY_NO_WIRE_QUERY: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_QUERY_REQUEST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_QUERY_REQUEST3(i32);
pub const DNS_QUERY_REQUEST_VERSION1: u32 = 1u32;
pub const DNS_QUERY_REQUEST_VERSION2: u32 = 2u32;
pub const DNS_QUERY_REQUEST_VERSION3: u32 = 3u32;
pub const DNS_QUERY_RESERVED: u32 = 4026531840u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_QUERY_RESULT(i32);
pub const DNS_QUERY_RESULTS_VERSION1: u32 = 1u32;
pub const DNS_QUERY_RETURN_MESSAGE: u32 = 512u32;
pub const DNS_QUERY_STANDARD: u32 = 0u32;
pub const DNS_QUERY_TREAT_AS_FQDN: u32 = 4096u32;
pub const DNS_QUERY_USE_TCP_ONLY: u32 = 2u32;
pub const DNS_QUERY_WIRE_ONLY: u32 = 256u32;
pub const DNS_RCLASS_ALL: u32 = 65280u32;
pub const DNS_RCLASS_ANY: u32 = 65280u32;
pub const DNS_RCLASS_CHAOS: u32 = 768u32;
pub const DNS_RCLASS_CSNET: u32 = 512u32;
pub const DNS_RCLASS_HESIOD: u32 = 1024u32;
pub const DNS_RCLASS_INTERNET: u32 = 256u32;
pub const DNS_RCLASS_NONE: u32 = 65024u32;
pub const DNS_RCLASS_UNICAST_RESPONSE: u32 = 128u32;
pub const DNS_RCODE_BADKEY: u32 = 17u32;
pub const DNS_RCODE_BADSIG: u32 = 16u32;
pub const DNS_RCODE_BADTIME: u32 = 18u32;
pub const DNS_RCODE_BADVERS: u32 = 16u32;
pub const DNS_RCODE_FORMAT_ERROR: u32 = 1u32;
pub const DNS_RCODE_FORMERR: u32 = 1u32;
pub const DNS_RCODE_MAX: u32 = 15u32;
pub const DNS_RCODE_NAME_ERROR: u32 = 3u32;
pub const DNS_RCODE_NOERROR: u32 = 0u32;
pub const DNS_RCODE_NOTAUTH: u32 = 9u32;
pub const DNS_RCODE_NOTIMPL: u32 = 4u32;
pub const DNS_RCODE_NOTZONE: u32 = 10u32;
pub const DNS_RCODE_NOT_IMPLEMENTED: u32 = 4u32;
pub const DNS_RCODE_NO_ERROR: u32 = 0u32;
pub const DNS_RCODE_NXDOMAIN: u32 = 3u32;
pub const DNS_RCODE_NXRRSET: u32 = 8u32;
pub const DNS_RCODE_REFUSED: u32 = 5u32;
pub const DNS_RCODE_SERVER_FAILURE: u32 = 2u32;
pub const DNS_RCODE_SERVFAIL: u32 = 2u32;
pub const DNS_RCODE_YXDOMAIN: u32 = 6u32;
pub const DNS_RCODE_YXRRSET: u32 = 7u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_RECORDA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_RECORDW(i32);
#[repr(C)]
pub struct DNS_RECORD_FLAGS(i32);
pub const DNS_RFC_MAX_UDP_PACKET_LENGTH: u32 = 512u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_RRSET(i32);
pub const DNS_RTYPE_A: u32 = 256u32;
pub const DNS_RTYPE_A6: u32 = 9728u32;
pub const DNS_RTYPE_AAAA: u32 = 7168u32;
pub const DNS_RTYPE_AFSDB: u32 = 4608u32;
pub const DNS_RTYPE_ALL: u32 = 65280u32;
pub const DNS_RTYPE_ANY: u32 = 65280u32;
pub const DNS_RTYPE_ATMA: u32 = 8704u32;
pub const DNS_RTYPE_AXFR: u32 = 64512u32;
pub const DNS_RTYPE_CERT: u32 = 9472u32;
pub const DNS_RTYPE_CNAME: u32 = 1280u32;
pub const DNS_RTYPE_DHCID: u32 = 12544u32;
pub const DNS_RTYPE_DNAME: u32 = 9984u32;
pub const DNS_RTYPE_DNSKEY: u32 = 12288u32;
pub const DNS_RTYPE_DS: u32 = 11008u32;
pub const DNS_RTYPE_EID: u32 = 7936u32;
pub const DNS_RTYPE_GID: u32 = 26112u32;
pub const DNS_RTYPE_GPOS: u32 = 6912u32;
pub const DNS_RTYPE_HINFO: u32 = 3328u32;
pub const DNS_RTYPE_ISDN: u32 = 5120u32;
pub const DNS_RTYPE_IXFR: u32 = 64256u32;
pub const DNS_RTYPE_KEY: u32 = 6400u32;
pub const DNS_RTYPE_KX: u32 = 9216u32;
pub const DNS_RTYPE_LOC: u32 = 7424u32;
pub const DNS_RTYPE_MAILA: u32 = 65024u32;
pub const DNS_RTYPE_MAILB: u32 = 64768u32;
pub const DNS_RTYPE_MB: u32 = 1792u32;
pub const DNS_RTYPE_MD: u32 = 768u32;
pub const DNS_RTYPE_MF: u32 = 1024u32;
pub const DNS_RTYPE_MG: u32 = 2048u32;
pub const DNS_RTYPE_MINFO: u32 = 3584u32;
pub const DNS_RTYPE_MR: u32 = 2304u32;
pub const DNS_RTYPE_MX: u32 = 3840u32;
pub const DNS_RTYPE_NAPTR: u32 = 8960u32;
pub const DNS_RTYPE_NIMLOC: u32 = 8192u32;
pub const DNS_RTYPE_NS: u32 = 512u32;
pub const DNS_RTYPE_NSAP: u32 = 5632u32;
pub const DNS_RTYPE_NSAPPTR: u32 = 5888u32;
pub const DNS_RTYPE_NSEC: u32 = 12032u32;
pub const DNS_RTYPE_NSEC3: u32 = 12800u32;
pub const DNS_RTYPE_NSEC3PARAM: u32 = 13056u32;
pub const DNS_RTYPE_NULL: u32 = 2560u32;
pub const DNS_RTYPE_NXT: u32 = 7680u32;
pub const DNS_RTYPE_OPT: u32 = 10496u32;
pub const DNS_RTYPE_PTR: u32 = 3072u32;
pub const DNS_RTYPE_PX: u32 = 6656u32;
pub const DNS_RTYPE_RP: u32 = 4352u32;
pub const DNS_RTYPE_RRSIG: u32 = 11776u32;
pub const DNS_RTYPE_RT: u32 = 5376u32;
pub const DNS_RTYPE_SIG: u32 = 6144u32;
pub const DNS_RTYPE_SINK: u32 = 10240u32;
pub const DNS_RTYPE_SOA: u32 = 1536u32;
pub const DNS_RTYPE_SRV: u32 = 8448u32;
pub const DNS_RTYPE_TEXT: u32 = 4096u32;
pub const DNS_RTYPE_TKEY: u32 = 63744u32;
pub const DNS_RTYPE_TLSA: u32 = 13312u32;
pub const DNS_RTYPE_TSIG: u32 = 64000u32;
pub const DNS_RTYPE_UID: u32 = 25856u32;
pub const DNS_RTYPE_UINFO: u32 = 25600u32;
pub const DNS_RTYPE_UNSPEC: u32 = 26368u32;
pub const DNS_RTYPE_WINS: u32 = 511u32;
pub const DNS_RTYPE_WINSR: u32 = 767u32;
pub const DNS_RTYPE_WKS: u32 = 2816u32;
pub const DNS_RTYPE_X25: u32 = 4864u32;
#[repr(transparent)]
pub struct DNS_SECTION(pub i32);
pub const DnsSectionQuestion: DNS_SECTION = DNS_SECTION(0i32);
pub const DnsSectionAnswer: DNS_SECTION = DNS_SECTION(1i32);
pub const DnsSectionAuthority: DNS_SECTION = DNS_SECTION(2i32);
pub const DnsSectionAddtional: DNS_SECTION = DNS_SECTION(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_SERVICE_BROWSE_REQUEST(i32);
#[repr(C)]
pub struct DNS_SERVICE_CANCEL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_SERVICE_INSTANCE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_SERVICE_REGISTER_REQUEST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_SERVICE_RESOLVE_REQUEST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_SIG_DATAA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_SIG_DATAW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_SOA_DATAA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_SOA_DATAW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_SRV_DATAA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_SRV_DATAW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_TKEY_DATAA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_TKEY_DATAW(i32);
pub const DNS_TKEY_MODE_DIFFIE_HELLMAN: u32 = 2u32;
pub const DNS_TKEY_MODE_GSS: u32 = 3u32;
pub const DNS_TKEY_MODE_RESOLVER_ASSIGN: u32 = 4u32;
pub const DNS_TKEY_MODE_SERVER_ASSIGN: u32 = 1u32;
#[repr(C)]
pub struct DNS_TLSA_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_TSIG_DATAA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_TSIG_DATAW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_TXT_DATAA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_TXT_DATAW(i32);
pub const DNS_TYPE_A: u32 = 1u32;
pub const DNS_TYPE_A6: u32 = 38u32;
pub const DNS_TYPE_AAAA: u32 = 28u32;
pub const DNS_TYPE_ADDRS: u32 = 248u32;
pub const DNS_TYPE_AFSDB: u32 = 18u32;
pub const DNS_TYPE_ALL: u32 = 255u32;
pub const DNS_TYPE_ANY: u32 = 255u32;
pub const DNS_TYPE_ATMA: u32 = 34u32;
pub const DNS_TYPE_AXFR: u32 = 252u32;
pub const DNS_TYPE_CERT: u32 = 37u32;
pub const DNS_TYPE_CNAME: u32 = 5u32;
pub const DNS_TYPE_DHCID: u32 = 49u32;
pub const DNS_TYPE_DNAME: u32 = 39u32;
pub const DNS_TYPE_DNSKEY: u32 = 48u32;
pub const DNS_TYPE_DS: u32 = 43u32;
pub const DNS_TYPE_EID: u32 = 31u32;
pub const DNS_TYPE_GID: u32 = 102u32;
pub const DNS_TYPE_GPOS: u32 = 27u32;
pub const DNS_TYPE_HINFO: u32 = 13u32;
pub const DNS_TYPE_ISDN: u32 = 20u32;
pub const DNS_TYPE_IXFR: u32 = 251u32;
pub const DNS_TYPE_KEY: u32 = 25u32;
pub const DNS_TYPE_KX: u32 = 36u32;
pub const DNS_TYPE_LOC: u32 = 29u32;
pub const DNS_TYPE_MAILA: u32 = 254u32;
pub const DNS_TYPE_MAILB: u32 = 253u32;
pub const DNS_TYPE_MB: u32 = 7u32;
pub const DNS_TYPE_MD: u32 = 3u32;
pub const DNS_TYPE_MF: u32 = 4u32;
pub const DNS_TYPE_MG: u32 = 8u32;
pub const DNS_TYPE_MINFO: u32 = 14u32;
pub const DNS_TYPE_MR: u32 = 9u32;
pub const DNS_TYPE_MX: u32 = 15u32;
pub const DNS_TYPE_NAPTR: u32 = 35u32;
pub const DNS_TYPE_NBSTAT: u32 = 65282u32;
pub const DNS_TYPE_NIMLOC: u32 = 32u32;
pub const DNS_TYPE_NS: u32 = 2u32;
pub const DNS_TYPE_NSAP: u32 = 22u32;
pub const DNS_TYPE_NSAPPTR: u32 = 23u32;
pub const DNS_TYPE_NSEC: u32 = 47u32;
pub const DNS_TYPE_NSEC3: u32 = 50u32;
pub const DNS_TYPE_NSEC3PARAM: u32 = 51u32;
pub const DNS_TYPE_NULL: u32 = 10u32;
pub const DNS_TYPE_NXT: u32 = 30u32;
pub const DNS_TYPE_OPT: u32 = 41u32;
pub const DNS_TYPE_PTR: u32 = 12u32;
pub const DNS_TYPE_PX: u32 = 26u32;
pub const DNS_TYPE_RP: u32 = 17u32;
pub const DNS_TYPE_RRSIG: u32 = 46u32;
pub const DNS_TYPE_RT: u32 = 21u32;
pub const DNS_TYPE_SIG: u32 = 24u32;
pub const DNS_TYPE_SINK: u32 = 40u32;
pub const DNS_TYPE_SOA: u32 = 6u32;
pub const DNS_TYPE_SRV: u32 = 33u32;
pub const DNS_TYPE_TEXT: u32 = 16u32;
pub const DNS_TYPE_TKEY: u32 = 249u32;
pub const DNS_TYPE_TLSA: u32 = 52u32;
pub const DNS_TYPE_TSIG: u32 = 250u32;
pub const DNS_TYPE_UID: u32 = 101u32;
pub const DNS_TYPE_UINFO: u32 = 100u32;
pub const DNS_TYPE_UNSPEC: u32 = 103u32;
pub const DNS_TYPE_WINS: u32 = 65281u32;
pub const DNS_TYPE_WINSR: u32 = 65282u32;
pub const DNS_TYPE_WKS: u32 = 11u32;
pub const DNS_TYPE_X25: u32 = 19u32;
pub const DNS_TYPE_ZERO: u32 = 0u32;
#[repr(C)]
pub struct DNS_UNKNOWN_DATA(i32);
pub const DNS_UPDATE_CACHE_SECURITY_CONTEXT: u32 = 512u32;
pub const DNS_UPDATE_FORCE_SECURITY_NEGO: u32 = 2048u32;
pub const DNS_UPDATE_REMOTE_SERVER: u32 = 16384u32;
pub const DNS_UPDATE_RESERVED: u32 = 4294901760u32;
pub const DNS_UPDATE_SECURITY_OFF: u32 = 16u32;
pub const DNS_UPDATE_SECURITY_ON: u32 = 32u32;
pub const DNS_UPDATE_SECURITY_ONLY: u32 = 256u32;
pub const DNS_UPDATE_SECURITY_USE_DEFAULT: u32 = 0u32;
pub const DNS_UPDATE_SKIP_NO_UPDATE_ADAPTERS: u32 = 8192u32;
pub const DNS_UPDATE_TEST_USE_LOCAL_SYS_ACCT: u32 = 1024u32;
pub const DNS_UPDATE_TRY_ALL_MASTER_SERVERS: u32 = 4096u32;
pub const DNS_VALSVR_ERROR_INVALID_ADDR: u32 = 1u32;
pub const DNS_VALSVR_ERROR_INVALID_NAME: u32 = 2u32;
pub const DNS_VALSVR_ERROR_NO_AUTH: u32 = 5u32;
pub const DNS_VALSVR_ERROR_NO_RESPONSE: u32 = 4u32;
pub const DNS_VALSVR_ERROR_NO_TCP: u32 = 16u32;
pub const DNS_VALSVR_ERROR_REFUSED: u32 = 6u32;
pub const DNS_VALSVR_ERROR_UNKNOWN: u32 = 255u32;
pub const DNS_VALSVR_ERROR_UNREACHABLE: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_WINSR_DATAA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DNS_WINSR_DATAW(i32);
#[repr(C)]
pub struct DNS_WINS_DATA(i32);
pub const DNS_WINS_FLAG_LOCAL: u32 = 65536u32;
pub const DNS_WINS_FLAG_SCOPE: u32 = 2147483648u32;
#[repr(C)]
pub struct DNS_WIRE_QUESTION(i32);
#[repr(C)]
pub struct DNS_WIRE_RECORD(i32);
#[repr(C)]
pub struct DNS_WKS_DATA(i32);
#[repr(C)]
pub struct DnsContextHandle(i32);
pub const IP4_ADDRESS_STRING_BUFFER_LENGTH: u32 = 16u32;
pub const IP4_ADDRESS_STRING_LENGTH: u32 = 16u32;
#[repr(C)]
pub struct IP4_ARRAY(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct IP6_ADDRESS(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct IP6_ADDRESS(i32);
pub const IP6_ADDRESS_STRING_BUFFER_LENGTH: u32 = 65u32;
pub const IP6_ADDRESS_STRING_LENGTH: u32 = 65u32;
#[repr(C)]
pub struct MDNS_QUERY_HANDLE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MDNS_QUERY_REQUEST(i32);
#[cfg(feature = "Win32_Foundation")]
pub type PDNS_QUERY_COMPLETION_ROUTINE = unsafe extern "system" fn(pquerycontext: *const ::core::ffi::c_void, pqueryresults: *mut DNS_QUERY_RESULT);
#[cfg(feature = "Win32_Foundation")]
pub type PDNS_SERVICE_BROWSE_CALLBACK = unsafe extern "system" fn(status: u32, pquerycontext: *const ::core::ffi::c_void, pdnsrecord: *const DNS_RECORDA);
#[cfg(feature = "Win32_Foundation")]
pub type PDNS_SERVICE_REGISTER_COMPLETE = unsafe extern "system" fn(status: u32, pquerycontext: *const ::core::ffi::c_void, pinstance: *const DNS_SERVICE_INSTANCE);
#[cfg(feature = "Win32_Foundation")]
pub type PDNS_SERVICE_RESOLVE_COMPLETE = unsafe extern "system" fn(status: u32, pquerycontext: *const ::core::ffi::c_void, pinstance: *const DNS_SERVICE_INSTANCE);
#[cfg(feature = "Win32_Foundation")]
pub type PMDNS_QUERY_CALLBACK = unsafe extern "system" fn(pquerycontext: *const ::core::ffi::c_void, pqueryhandle: *mut MDNS_QUERY_HANDLE, pqueryresults: *mut DNS_QUERY_RESULT);
pub const SIZEOF_IP4_ADDRESS: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct _DnsRecordOptA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct _DnsRecordOptW(i32);
