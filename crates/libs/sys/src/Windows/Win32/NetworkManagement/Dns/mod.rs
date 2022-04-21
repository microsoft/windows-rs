#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsAcquireContextHandle_A(credentialflags: u32, credentials: *const ::core::ffi::c_void, pcontext: *mut DnsContextHandle) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsAcquireContextHandle_W(credentialflags: u32, credentials: *const ::core::ffi::c_void, pcontext: *mut DnsContextHandle) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsCancelQuery(pcancelhandle: *const DNS_QUERY_CANCEL) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsConnectionDeletePolicyEntries(policyentrytag: DNS_CONNECTION_POLICY_TAG) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsConnectionDeleteProxyInfo(pwszconnectionname: ::windows_sys::core::PCWSTR, r#type: DNS_CONNECTION_PROXY_TYPE) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsConnectionFreeNameList(pnamelist: *mut DNS_CONNECTION_NAME_LIST);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsConnectionFreeProxyInfo(pproxyinfo: *mut DNS_CONNECTION_PROXY_INFO);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionFreeProxyInfoEx(pproxyinfoex: *mut DNS_CONNECTION_PROXY_INFO_EX);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsConnectionFreeProxyList(pproxylist: *mut DNS_CONNECTION_PROXY_LIST);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsConnectionGetNameList(pnamelist: *mut DNS_CONNECTION_NAME_LIST) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsConnectionGetProxyInfo(pwszconnectionname: ::windows_sys::core::PCWSTR, r#type: DNS_CONNECTION_PROXY_TYPE, pproxyinfo: *mut DNS_CONNECTION_PROXY_INFO) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionGetProxyInfoForHostUrl(pwszhosturl: ::windows_sys::core::PCWSTR, pselectioncontext: *const u8, dwselectioncontextlength: u32, dwexplicitinterfaceindex: u32, pproxyinfoex: *mut DNS_CONNECTION_PROXY_INFO_EX) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsConnectionGetProxyList(pwszconnectionname: ::windows_sys::core::PCWSTR, pproxylist: *mut DNS_CONNECTION_PROXY_LIST) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsConnectionSetPolicyEntries(policyentrytag: DNS_CONNECTION_POLICY_TAG, ppolicyentrylist: *const DNS_CONNECTION_POLICY_ENTRY_LIST) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsConnectionSetProxyInfo(pwszconnectionname: ::windows_sys::core::PCWSTR, r#type: DNS_CONNECTION_PROXY_TYPE, pproxyinfo: *const DNS_CONNECTION_PROXY_INFO) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsConnectionUpdateIfIndexTable(pconnectionifindexentries: *const DNS_CONNECTION_IFINDEX_LIST) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsExtractRecordsFromMessage_UTF8(pdnsbuffer: *const DNS_MESSAGE_BUFFER, wmessagelength: u16, pprecord: *mut *mut DNS_RECORDA) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsExtractRecordsFromMessage_W(pdnsbuffer: *const DNS_MESSAGE_BUFFER, wmessagelength: u16, pprecord: *mut *mut DNS_RECORDA) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsFree(pdata: *const ::core::ffi::c_void, freetype: DNS_FREE_TYPE);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsFreeCustomServers(pcservers: *mut u32, ppservers: *mut *mut DNS_CUSTOM_SERVER);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsFreeProxyName(proxyname: ::windows_sys::core::PCWSTR);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsGetApplicationSettings(pcservers: *mut u32, ppdefaultservers: *mut *mut DNS_CUSTOM_SERVER, psettings: *mut DNS_APPLICATION_SETTINGS) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsGetProxyInformation(hostname: ::windows_sys::core::PCWSTR, proxyinformation: *mut DNS_PROXY_INFORMATION, defaultproxyinformation: *mut DNS_PROXY_INFORMATION, completionroutine: DNS_PROXY_COMPLETION_ROUTINE, completioncontext: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsModifyRecordsInSet_A(paddrecords: *const DNS_RECORDA, pdeleterecords: *const DNS_RECORDA, options: u32, hcredentials: super::super::Foundation::HANDLE, pextralist: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsModifyRecordsInSet_UTF8(paddrecords: *const DNS_RECORDA, pdeleterecords: *const DNS_RECORDA, options: u32, hcredentials: super::super::Foundation::HANDLE, pextralist: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsModifyRecordsInSet_W(paddrecords: *const DNS_RECORDA, pdeleterecords: *const DNS_RECORDA, options: u32, hcredentials: super::super::Foundation::HANDLE, pextralist: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsNameCompare_A(pname1: ::windows_sys::core::PCSTR, pname2: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsNameCompare_W(pname1: ::windows_sys::core::PCWSTR, pname2: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsQueryConfig(config: DNS_CONFIG_TYPE, flag: u32, pwsadaptername: ::windows_sys::core::PCWSTR, preserved: *const ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void, pbuflen: *mut u32) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQueryEx(pqueryrequest: *const DNS_QUERY_REQUEST, pqueryresults: *mut DNS_QUERY_RESULT, pcancelhandle: *mut DNS_QUERY_CANCEL) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQuery_A(pszname: ::windows_sys::core::PCSTR, wtype: u16, options: u32, pextra: *mut ::core::ffi::c_void, ppqueryresults: *mut *mut DNS_RECORDA, preserved: *mut *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQuery_UTF8(pszname: ::windows_sys::core::PCSTR, wtype: u16, options: u32, pextra: *mut ::core::ffi::c_void, ppqueryresults: *mut *mut DNS_RECORDA, preserved: *mut *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQuery_W(pszname: ::windows_sys::core::PCWSTR, wtype: u16, options: u32, pextra: *mut ::core::ffi::c_void, ppqueryresults: *mut *mut DNS_RECORDA, preserved: *mut *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordCompare(precord1: *const DNS_RECORDA, precord2: *const DNS_RECORDA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordCopyEx(precord: *const DNS_RECORDA, charsetin: DNS_CHARSET, charsetout: DNS_CHARSET) -> *mut DNS_RECORDA;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordSetCompare(prr1: *mut DNS_RECORDA, prr2: *mut DNS_RECORDA, ppdiff1: *mut *mut DNS_RECORDA, ppdiff2: *mut *mut DNS_RECORDA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordSetCopyEx(precordset: *const DNS_RECORDA, charsetin: DNS_CHARSET, charsetout: DNS_CHARSET) -> *mut DNS_RECORDA;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordSetDetach(precordlist: *mut DNS_RECORDA) -> *mut DNS_RECORDA;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsReleaseContextHandle(hcontext: super::super::Foundation::HANDLE);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsReplaceRecordSetA(preplaceset: *const DNS_RECORDA, options: u32, hcontext: super::super::Foundation::HANDLE, pextrainfo: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsReplaceRecordSetUTF8(preplaceset: *const DNS_RECORDA, options: u32, hcontext: super::super::Foundation::HANDLE, pextrainfo: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsReplaceRecordSetW(preplaceset: *const DNS_RECORDA, options: u32, hcontext: super::super::Foundation::HANDLE, pextrainfo: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceBrowse(prequest: *const DNS_SERVICE_BROWSE_REQUEST, pcancel: *mut DNS_SERVICE_CANCEL) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsServiceBrowseCancel(pcancelhandle: *const DNS_SERVICE_CANCEL) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsServiceConstructInstance(pservicename: ::windows_sys::core::PCWSTR, phostname: ::windows_sys::core::PCWSTR, pip4: *const u32, pip6: *const IP6_ADDRESS, wport: u16, wpriority: u16, wweight: u16, dwpropertiescount: u32, keys: *const ::windows_sys::core::PWSTR, values: *const ::windows_sys::core::PWSTR) -> *mut DNS_SERVICE_INSTANCE;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsServiceCopyInstance(porig: *const DNS_SERVICE_INSTANCE) -> *mut DNS_SERVICE_INSTANCE;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceDeRegister(prequest: *const DNS_SERVICE_REGISTER_REQUEST, pcancel: *mut DNS_SERVICE_CANCEL) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsServiceFreeInstance(pinstance: *const DNS_SERVICE_INSTANCE);
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceRegister(prequest: *const DNS_SERVICE_REGISTER_REQUEST, pcancel: *mut DNS_SERVICE_CANCEL) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsServiceRegisterCancel(pcancelhandle: *const DNS_SERVICE_CANCEL) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsServiceResolve(prequest: *const DNS_SERVICE_RESOLVE_REQUEST, pcancel: *mut DNS_SERVICE_CANCEL) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsServiceResolveCancel(pcancelhandle: *const DNS_SERVICE_CANCEL) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsSetApplicationSettings(cservers: u32, pservers: *const DNS_CUSTOM_SERVER, psettings: *const DNS_APPLICATION_SETTINGS) -> u32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsStartMulticastQuery(pqueryrequest: *const MDNS_QUERY_REQUEST, phandle: *mut MDNS_QUERY_HANDLE) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsStopMulticastQuery(phandle: *mut MDNS_QUERY_HANDLE) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsValidateName_A(pszname: ::windows_sys::core::PCSTR, format: DNS_NAME_FORMAT) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsValidateName_UTF8(pszname: ::windows_sys::core::PCSTR, format: DNS_NAME_FORMAT) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
    pub fn DnsValidateName_W(pszname: ::windows_sys::core::PCWSTR, format: DNS_NAME_FORMAT) -> i32;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsWriteQuestionToBuffer_UTF8(pdnsbuffer: *mut DNS_MESSAGE_BUFFER, pdwbuffersize: *mut u32, pszname: ::windows_sys::core::PCSTR, wtype: u16, xid: u16, frecursiondesired: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsWriteQuestionToBuffer_W(pdnsbuffer: *mut DNS_MESSAGE_BUFFER, pdwbuffersize: *mut u32, pszname: ::windows_sys::core::PCWSTR, wtype: u16, xid: u16, frecursiondesired: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSREC_ADDITIONAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSREC_ANSWER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSREC_AUTHORITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSREC_DELETE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSREC_NOEXIST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSREC_PREREQ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSREC_QUESTION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSREC_SECTION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSREC_UPDATE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSREC_ZONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_ALGORITHM_ECDSAP256_SHA256: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_ALGORITHM_ECDSAP384_SHA384: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_ALGORITHM_NULL: u32 = 253u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_ALGORITHM_PRIVATE: u32 = 254u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_ALGORITHM_RSAMD5: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_ALGORITHM_RSASHA1: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_ALGORITHM_RSASHA1_NSEC3: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_ALGORITHM_RSASHA256: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_ALGORITHM_RSASHA512: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_DIGEST_ALGORITHM_SHA1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_DIGEST_ALGORITHM_SHA256: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_DIGEST_ALGORITHM_SHA384: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_EXTEND: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_FLAG10: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_FLAG11: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_FLAG2: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_FLAG4: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_FLAG5: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_FLAG8: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_FLAG9: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_HOST: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_NOAUTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_NOCONF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_NTPE3: u32 = 192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_SIG0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_SIG1: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_SIG10: u32 = 40960u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_SIG11: u32 = 45056u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_SIG12: u32 = 49152u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_SIG13: u32 = 53248u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_SIG14: u32 = 57344u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_SIG15: u32 = 61440u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_SIG2: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_SIG3: u32 = 12288u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_SIG4: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_SIG5: u32 = 20480u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_SIG6: u32 = 24576u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_SIG7: u32 = 28672u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_SIG8: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_SIG9: u32 = 36864u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_USER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_KEY_FLAG_ZONE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_PROTOCOL_DNSSEC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_PROTOCOL_EMAIL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_PROTOCOL_IPSEC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_PROTOCOL_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNSSEC_PROTOCOL_TLS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_AAAA_DATA {
    pub Ip6Address: IP6_ADDRESS,
}
impl ::core::marker::Copy for DNS_AAAA_DATA {}
impl ::core::clone::Clone for DNS_AAAA_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_ADDR {
    pub MaxSa: [super::super::Foundation::CHAR; 32],
    pub Data: DNS_ADDR_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_ADDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_ADDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DNS_ADDR_0 {
    pub DnsAddrUserDword: [u32; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_ADDR_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_ADDR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_ADDRESS_STRING_LENGTH: u32 = 65u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_ADDR_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_ADDR_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_ADDR_MAX_SOCKADDR_LENGTH: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_APPLICATION_SETTINGS {
    pub Version: u32,
    pub Flags: u64,
}
impl ::core::marker::Copy for DNS_APPLICATION_SETTINGS {}
impl ::core::clone::Clone for DNS_APPLICATION_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_APP_SETTINGS_EXCLUSIVE_SERVERS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_APP_SETTINGS_VERSION1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_ATMA_AESA_ADDR_LENGTH: u32 = 20u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_ATMA_DATA {
    pub AddressType: u8,
    pub Address: [u8; 20],
}
impl ::core::marker::Copy for DNS_ATMA_DATA {}
impl ::core::clone::Clone for DNS_ATMA_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_ATMA_FORMAT_AESA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_ATMA_FORMAT_E164: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_ATMA_MAX_ADDR_LENGTH: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_ATMA_MAX_RECORD_LENGTH: u32 = 21u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_A_DATA {
    pub IpAddress: u32,
}
impl ::core::marker::Copy for DNS_A_DATA {}
impl ::core::clone::Clone for DNS_A_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub type DNS_CHARSET = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsCharSetUnknown: DNS_CHARSET = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsCharSetUnicode: DNS_CHARSET = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsCharSetUtf8: DNS_CHARSET = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsCharSetAnsi: DNS_CHARSET = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CLASS_ALL: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CLASS_ANY: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CLASS_CHAOS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CLASS_CSNET: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CLASS_HESIOD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CLASS_INTERNET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CLASS_NONE: u32 = 254u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CLASS_UNICAST_RESPONSE: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_COMPRESSED_QUESTION_NAME: u32 = 49164u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONFIG_FLAG_ALLOC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub type DNS_CONFIG_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigPrimaryDomainName_W: DNS_CONFIG_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigPrimaryDomainName_A: DNS_CONFIG_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigPrimaryDomainName_UTF8: DNS_CONFIG_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigAdapterDomainName_W: DNS_CONFIG_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigAdapterDomainName_A: DNS_CONFIG_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigAdapterDomainName_UTF8: DNS_CONFIG_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigDnsServerList: DNS_CONFIG_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigSearchList: DNS_CONFIG_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigAdapterInfo: DNS_CONFIG_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigPrimaryHostNameRegistrationEnabled: DNS_CONFIG_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigAdapterHostNameRegistrationEnabled: DNS_CONFIG_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigAddressRegistrationMaxCount: DNS_CONFIG_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigHostName_W: DNS_CONFIG_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigHostName_A: DNS_CONFIG_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigHostName_UTF8: DNS_CONFIG_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigFullHostName_W: DNS_CONFIG_TYPE = 15i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigFullHostName_A: DNS_CONFIG_TYPE = 16i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigFullHostName_UTF8: DNS_CONFIG_TYPE = 17i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsConfigNameServer: DNS_CONFIG_TYPE = 18i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_CONNECTION_IFINDEX_ENTRY {
    pub pwszConnectionName: ::windows_sys::core::PCWSTR,
    pub dwIfIndex: u32,
}
impl ::core::marker::Copy for DNS_CONNECTION_IFINDEX_ENTRY {}
impl ::core::clone::Clone for DNS_CONNECTION_IFINDEX_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_CONNECTION_IFINDEX_LIST {
    pub pConnectionIfIndexEntries: *mut DNS_CONNECTION_IFINDEX_ENTRY,
    pub nEntries: u32,
}
impl ::core::marker::Copy for DNS_CONNECTION_IFINDEX_LIST {}
impl ::core::clone::Clone for DNS_CONNECTION_IFINDEX_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_CONNECTION_NAME {
    pub wszName: [u16; 65],
}
impl ::core::marker::Copy for DNS_CONNECTION_NAME {}
impl ::core::clone::Clone for DNS_CONNECTION_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_CONNECTION_NAME_LIST {
    pub cNames: u32,
    pub pNames: *mut DNS_CONNECTION_NAME,
}
impl ::core::marker::Copy for DNS_CONNECTION_NAME_LIST {}
impl ::core::clone::Clone for DNS_CONNECTION_NAME_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_NAME_MAX_LENGTH: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_CONNECTION_POLICY_ENTRY {
    pub pwszHost: ::windows_sys::core::PCWSTR,
    pub pwszAppId: ::windows_sys::core::PCWSTR,
    pub cbAppSid: u32,
    pub pbAppSid: *mut u8,
    pub nConnections: u32,
    pub ppwszConnections: *mut ::windows_sys::core::PWSTR,
    pub dwPolicyEntryFlags: u32,
}
impl ::core::marker::Copy for DNS_CONNECTION_POLICY_ENTRY {}
impl ::core::clone::Clone for DNS_CONNECTION_POLICY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_CONNECTION_POLICY_ENTRY_LIST {
    pub pPolicyEntries: *mut DNS_CONNECTION_POLICY_ENTRY,
    pub nEntries: u32,
}
impl ::core::marker::Copy for DNS_CONNECTION_POLICY_ENTRY_LIST {}
impl ::core::clone::Clone for DNS_CONNECTION_POLICY_ENTRY_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_POLICY_ENTRY_ONDEMAND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub type DNS_CONNECTION_POLICY_TAG = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const TAG_DNS_CONNECTION_POLICY_TAG_DEFAULT: DNS_CONNECTION_POLICY_TAG = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const TAG_DNS_CONNECTION_POLICY_TAG_CONNECTION_MANAGER: DNS_CONNECTION_POLICY_TAG = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const TAG_DNS_CONNECTION_POLICY_TAG_WWWPT: DNS_CONNECTION_POLICY_TAG = 2i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_CONNECTION_PROXY_ELEMENT {
    pub Type: DNS_CONNECTION_PROXY_TYPE,
    pub Info: DNS_CONNECTION_PROXY_INFO,
}
impl ::core::marker::Copy for DNS_CONNECTION_PROXY_ELEMENT {}
impl ::core::clone::Clone for DNS_CONNECTION_PROXY_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_CONNECTION_PROXY_INFO {
    pub Version: u32,
    pub pwszFriendlyName: ::windows_sys::core::PWSTR,
    pub Flags: u32,
    pub Switch: DNS_CONNECTION_PROXY_INFO_SWITCH,
    pub Anonymous: DNS_CONNECTION_PROXY_INFO_0,
}
impl ::core::marker::Copy for DNS_CONNECTION_PROXY_INFO {}
impl ::core::clone::Clone for DNS_CONNECTION_PROXY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub union DNS_CONNECTION_PROXY_INFO_0 {
    pub Config: DNS_CONNECTION_PROXY_INFO_0_0,
    pub Script: DNS_CONNECTION_PROXY_INFO_0_1,
}
impl ::core::marker::Copy for DNS_CONNECTION_PROXY_INFO_0 {}
impl ::core::clone::Clone for DNS_CONNECTION_PROXY_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_CONNECTION_PROXY_INFO_0_0 {
    pub pwszServer: ::windows_sys::core::PWSTR,
    pub pwszUsername: ::windows_sys::core::PWSTR,
    pub pwszPassword: ::windows_sys::core::PWSTR,
    pub pwszException: ::windows_sys::core::PWSTR,
    pub pwszExtraInfo: ::windows_sys::core::PWSTR,
    pub Port: u16,
}
impl ::core::marker::Copy for DNS_CONNECTION_PROXY_INFO_0_0 {}
impl ::core::clone::Clone for DNS_CONNECTION_PROXY_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_CONNECTION_PROXY_INFO_0_1 {
    pub pwszScript: ::windows_sys::core::PWSTR,
    pub pwszUsername: ::windows_sys::core::PWSTR,
    pub pwszPassword: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for DNS_CONNECTION_PROXY_INFO_0_1 {}
impl ::core::clone::Clone for DNS_CONNECTION_PROXY_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_INFO_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_CONNECTION_PROXY_INFO_EX {
    pub ProxyInfo: DNS_CONNECTION_PROXY_INFO,
    pub dwInterfaceIndex: u32,
    pub pwszConnectionName: ::windows_sys::core::PWSTR,
    pub fDirectConfiguration: super::super::Foundation::BOOL,
    pub hConnection: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_CONNECTION_PROXY_INFO_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_CONNECTION_PROXY_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_INFO_EXCEPTION_MAX_LENGTH: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_INFO_EXTRA_INFO_MAX_LENGTH: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_INFO_FLAG_BYPASSLOCAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_INFO_FLAG_DISABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_INFO_FRIENDLY_NAME_MAX_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_INFO_PASSWORD_MAX_LENGTH: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_INFO_SERVER_MAX_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub type DNS_CONNECTION_PROXY_INFO_SWITCH = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_INFO_SWITCH_CONFIG: DNS_CONNECTION_PROXY_INFO_SWITCH = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_INFO_SWITCH_SCRIPT: DNS_CONNECTION_PROXY_INFO_SWITCH = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_INFO_SWITCH_WPAD: DNS_CONNECTION_PROXY_INFO_SWITCH = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_INFO_USERNAME_MAX_LENGTH: u32 = 128u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_CONNECTION_PROXY_LIST {
    pub cProxies: u32,
    pub pProxies: *mut DNS_CONNECTION_PROXY_ELEMENT,
}
impl ::core::marker::Copy for DNS_CONNECTION_PROXY_LIST {}
impl ::core::clone::Clone for DNS_CONNECTION_PROXY_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub type DNS_CONNECTION_PROXY_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_TYPE_NULL: DNS_CONNECTION_PROXY_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_TYPE_HTTP: DNS_CONNECTION_PROXY_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_TYPE_WAP: DNS_CONNECTION_PROXY_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_TYPE_SOCKS4: DNS_CONNECTION_PROXY_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CONNECTION_PROXY_TYPE_SOCKS5: DNS_CONNECTION_PROXY_TYPE = 5i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_CUSTOM_SERVER {
    pub dwServerType: u32,
    pub ullFlags: u64,
    pub Anonymous1: DNS_CUSTOM_SERVER_0,
    pub Anonymous2: DNS_CUSTOM_SERVER_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_CUSTOM_SERVER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_CUSTOM_SERVER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DNS_CUSTOM_SERVER_0 {
    pub pwszTemplate: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_CUSTOM_SERVER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_CUSTOM_SERVER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DNS_CUSTOM_SERVER_1 {
    pub MaxSa: [super::super::Foundation::CHAR; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_CUSTOM_SERVER_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_CUSTOM_SERVER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CUSTOM_SERVER_TYPE_DOH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CUSTOM_SERVER_TYPE_UDP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_CUSTOM_SERVER_UDP_FALLBACK: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_DHCID_DATA {
    pub dwByteCount: u32,
    pub DHCID: [u8; 1],
}
impl ::core::marker::Copy for DNS_DHCID_DATA {}
impl ::core::clone::Clone for DNS_DHCID_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_DS_DATA {
    pub wKeyTag: u16,
    pub chAlgorithm: u8,
    pub chDigestType: u8,
    pub wDigestLength: u16,
    pub wPad: u16,
    pub Digest: [u8; 1],
}
impl ::core::marker::Copy for DNS_DS_DATA {}
impl ::core::clone::Clone for DNS_DS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub type DNS_FREE_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsFreeFlat: DNS_FREE_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsFreeRecordList: DNS_FREE_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsFreeParsedMessageFields: DNS_FREE_TYPE = 2i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_HEADER {
    pub Xid: u16,
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub QuestionCount: u16,
    pub AnswerCount: u16,
    pub NameServerCount: u16,
    pub AdditionalCount: u16,
}
impl ::core::marker::Copy for DNS_HEADER {}
impl ::core::clone::Clone for DNS_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_HEADER_EXT {
    pub _bitfield: u16,
    pub chRcode: u8,
    pub chVersion: u8,
}
impl ::core::marker::Copy for DNS_HEADER_EXT {}
impl ::core::clone::Clone for DNS_HEADER_EXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_KEY_DATA {
    pub wFlags: u16,
    pub chProtocol: u8,
    pub chAlgorithm: u8,
    pub wKeyLength: u16,
    pub wPad: u16,
    pub Key: [u8; 1],
}
impl ::core::marker::Copy for DNS_KEY_DATA {}
impl ::core::clone::Clone for DNS_KEY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_LOC_DATA {
    pub wVersion: u16,
    pub wSize: u16,
    pub wHorPrec: u16,
    pub wVerPrec: u16,
    pub dwLatitude: u32,
    pub dwLongitude: u32,
    pub dwAltitude: u32,
}
impl ::core::marker::Copy for DNS_LOC_DATA {}
impl ::core::clone::Clone for DNS_LOC_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_MAX_IP4_REVERSE_NAME_BUFFER_LENGTH: u32 = 31u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_MAX_IP4_REVERSE_NAME_LENGTH: u32 = 31u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_MAX_IP6_REVERSE_NAME_BUFFER_LENGTH: u32 = 75u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_MAX_IP6_REVERSE_NAME_LENGTH: u32 = 75u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_MAX_LABEL_BUFFER_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_MAX_LABEL_LENGTH: u32 = 63u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_MAX_NAME_BUFFER_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_MAX_NAME_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_MAX_REVERSE_NAME_BUFFER_LENGTH: u32 = 75u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_MAX_REVERSE_NAME_LENGTH: u32 = 75u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_MAX_TEXT_STRING_LENGTH: u32 = 255u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_MESSAGE_BUFFER {
    pub MessageHead: DNS_HEADER,
    pub MessageBody: [super::super::Foundation::CHAR; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_MESSAGE_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_MESSAGE_BUFFER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_MINFO_DATAA {
    pub pNameMailbox: ::windows_sys::core::PSTR,
    pub pNameErrorsMailbox: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for DNS_MINFO_DATAA {}
impl ::core::clone::Clone for DNS_MINFO_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_MINFO_DATAW {
    pub pNameMailbox: ::windows_sys::core::PWSTR,
    pub pNameErrorsMailbox: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for DNS_MINFO_DATAW {}
impl ::core::clone::Clone for DNS_MINFO_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_MX_DATAA {
    pub pNameExchange: ::windows_sys::core::PSTR,
    pub wPreference: u16,
    pub Pad: u16,
}
impl ::core::marker::Copy for DNS_MX_DATAA {}
impl ::core::clone::Clone for DNS_MX_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_MX_DATAW {
    pub pNameExchange: ::windows_sys::core::PWSTR,
    pub wPreference: u16,
    pub Pad: u16,
}
impl ::core::marker::Copy for DNS_MX_DATAW {}
impl ::core::clone::Clone for DNS_MX_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub type DNS_NAME_FORMAT = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsNameDomain: DNS_NAME_FORMAT = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsNameDomainLabel: DNS_NAME_FORMAT = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsNameHostnameFull: DNS_NAME_FORMAT = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsNameHostnameLabel: DNS_NAME_FORMAT = 3i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsNameWildcard: DNS_NAME_FORMAT = 4i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsNameSrvRecord: DNS_NAME_FORMAT = 5i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsNameValidateTld: DNS_NAME_FORMAT = 6i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_NAPTR_DATAA {
    pub wOrder: u16,
    pub wPreference: u16,
    pub pFlags: ::windows_sys::core::PSTR,
    pub pService: ::windows_sys::core::PSTR,
    pub pRegularExpression: ::windows_sys::core::PSTR,
    pub pReplacement: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for DNS_NAPTR_DATAA {}
impl ::core::clone::Clone for DNS_NAPTR_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_NAPTR_DATAW {
    pub wOrder: u16,
    pub wPreference: u16,
    pub pFlags: ::windows_sys::core::PWSTR,
    pub pService: ::windows_sys::core::PWSTR,
    pub pRegularExpression: ::windows_sys::core::PWSTR,
    pub pReplacement: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for DNS_NAPTR_DATAW {}
impl ::core::clone::Clone for DNS_NAPTR_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_NSEC3PARAM_DATA {
    pub chAlgorithm: u8,
    pub bFlags: u8,
    pub wIterations: u16,
    pub bSaltLength: u8,
    pub bPad: [u8; 3],
    pub pbSalt: [u8; 1],
}
impl ::core::marker::Copy for DNS_NSEC3PARAM_DATA {}
impl ::core::clone::Clone for DNS_NSEC3PARAM_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_NSEC3_DATA {
    pub chAlgorithm: u8,
    pub bFlags: u8,
    pub wIterations: u16,
    pub bSaltLength: u8,
    pub bHashLength: u8,
    pub wTypeBitMapsLength: u16,
    pub chData: [u8; 1],
}
impl ::core::marker::Copy for DNS_NSEC3_DATA {}
impl ::core::clone::Clone for DNS_NSEC3_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_NSEC_DATAA {
    pub pNextDomainName: ::windows_sys::core::PSTR,
    pub wTypeBitMapsLength: u16,
    pub wPad: u16,
    pub TypeBitMaps: [u8; 1],
}
impl ::core::marker::Copy for DNS_NSEC_DATAA {}
impl ::core::clone::Clone for DNS_NSEC_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_NSEC_DATAW {
    pub pNextDomainName: ::windows_sys::core::PWSTR,
    pub wTypeBitMapsLength: u16,
    pub wPad: u16,
    pub TypeBitMaps: [u8; 1],
}
impl ::core::marker::Copy for DNS_NSEC_DATAW {}
impl ::core::clone::Clone for DNS_NSEC_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_NULL_DATA {
    pub dwByteCount: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for DNS_NULL_DATA {}
impl ::core::clone::Clone for DNS_NULL_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_NXT_DATAA {
    pub pNameNext: ::windows_sys::core::PSTR,
    pub wNumTypes: u16,
    pub wTypes: [u16; 1],
}
impl ::core::marker::Copy for DNS_NXT_DATAA {}
impl ::core::clone::Clone for DNS_NXT_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_NXT_DATAW {
    pub pNameNext: ::windows_sys::core::PWSTR,
    pub wNumTypes: u16,
    pub wTypes: [u16; 1],
}
impl ::core::marker::Copy for DNS_NXT_DATAW {}
impl ::core::clone::Clone for DNS_NXT_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_OPCODE_IQUERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_OPCODE_NOTIFY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_OPCODE_QUERY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_OPCODE_SERVER_STATUS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_OPCODE_UNKNOWN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_OPCODE_UPDATE: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_OPT_DATA {
    pub wDataLength: u16,
    pub wPad: u16,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for DNS_OPT_DATA {}
impl ::core::clone::Clone for DNS_OPT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_PORT_HOST_ORDER: u32 = 53u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_PORT_NET_ORDER: u32 = 13568u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub type DNS_PROXY_COMPLETION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(completioncontext: *const ::core::ffi::c_void, status: i32)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_PROXY_INFORMATION {
    pub version: u32,
    pub proxyInformationType: DNS_PROXY_INFORMATION_TYPE,
    pub proxyName: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for DNS_PROXY_INFORMATION {}
impl ::core::clone::Clone for DNS_PROXY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub type DNS_PROXY_INFORMATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_PROXY_INFORMATION_DIRECT: DNS_PROXY_INFORMATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_PROXY_INFORMATION_DEFAULT_SETTINGS: DNS_PROXY_INFORMATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_PROXY_INFORMATION_PROXY_NAME: DNS_PROXY_INFORMATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_PROXY_INFORMATION_DOES_NOT_EXIST: DNS_PROXY_INFORMATION_TYPE = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_PTR_DATAA {
    pub pNameHost: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for DNS_PTR_DATAA {}
impl ::core::clone::Clone for DNS_PTR_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_PTR_DATAW {
    pub pNameHost: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for DNS_PTR_DATAW {}
impl ::core::clone::Clone for DNS_PTR_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_ACCEPT_TRUNCATED_RESPONSE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_ADDRCONFIG: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_APPEND_MULTILABEL: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_BYPASS_CACHE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_CACHE_ONLY: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_QUERY_CANCEL {
    pub Reserved: [super::super::Foundation::CHAR; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_QUERY_CANCEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_QUERY_CANCEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_DISABLE_IDN_ENCODING: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_DNSSEC_CHECKING_DISABLED: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_DNSSEC_OK: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_DONT_RESET_TTL_VALUES: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_DUAL_ADDR: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_MULTICAST_ONLY: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_NO_HOSTS_FILE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_NO_LOCAL_NAME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_NO_MULTICAST: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_NO_NETBT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_NO_RECURSION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_NO_WIRE_QUERY: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_QUERY_REQUEST {
    pub Version: u32,
    pub QueryName: ::windows_sys::core::PCWSTR,
    pub QueryType: u16,
    pub QueryOptions: u64,
    pub pDnsServerList: *mut DNS_ADDR_ARRAY,
    pub InterfaceIndex: u32,
    pub pQueryCompletionCallback: PDNS_QUERY_COMPLETION_ROUTINE,
    pub pQueryContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_QUERY_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_QUERY_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_QUERY_REQUEST3 {
    pub Version: u32,
    pub QueryName: ::windows_sys::core::PCWSTR,
    pub QueryType: u16,
    pub QueryOptions: u64,
    pub pDnsServerList: *mut DNS_ADDR_ARRAY,
    pub InterfaceIndex: u32,
    pub pQueryCompletionCallback: PDNS_QUERY_COMPLETION_ROUTINE,
    pub pQueryContext: *mut ::core::ffi::c_void,
    pub IsNetworkQueryRequired: super::super::Foundation::BOOL,
    pub RequiredNetworkIndex: u32,
    pub cCustomServers: u32,
    pub pCustomServers: *mut DNS_CUSTOM_SERVER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_QUERY_REQUEST3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_QUERY_REQUEST3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_REQUEST_VERSION1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_REQUEST_VERSION2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_REQUEST_VERSION3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_RESERVED: u32 = 4026531840u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_QUERY_RESULT {
    pub Version: u32,
    pub QueryStatus: i32,
    pub QueryOptions: u64,
    pub pQueryRecords: *mut DNS_RECORDA,
    pub Reserved: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_QUERY_RESULT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_QUERY_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_RESULTS_VERSION1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_RETURN_MESSAGE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_STANDARD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_TREAT_AS_FQDN: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_USE_TCP_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_QUERY_WIRE_ONLY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCLASS_ALL: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCLASS_ANY: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCLASS_CHAOS: u32 = 768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCLASS_CSNET: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCLASS_HESIOD: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCLASS_INTERNET: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCLASS_NONE: u32 = 65024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCLASS_UNICAST_RESPONSE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_BADKEY: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_BADSIG: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_BADTIME: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_BADVERS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_FORMAT_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_FORMERR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_MAX: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_NAME_ERROR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_NOERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_NOTAUTH: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_NOTIMPL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_NOTZONE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_NOT_IMPLEMENTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_NO_ERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_NXDOMAIN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_NXRRSET: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_REFUSED: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_SERVER_FAILURE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_SERVFAIL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_YXDOMAIN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RCODE_YXRRSET: u32 = 7u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_RECORDA {
    pub pNext: *mut DNS_RECORDA,
    pub pName: ::windows_sys::core::PSTR,
    pub wType: u16,
    pub wDataLength: u16,
    pub Flags: DNS_RECORDA_1,
    pub dwTtl: u32,
    pub dwReserved: u32,
    pub Data: DNS_RECORDA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_RECORDA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_RECORDA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DNS_RECORDA_0 {
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
    pub RRSIG: DNS_SIG_DATAA,
    pub Rrsig: DNS_SIG_DATAA,
    pub NSEC: DNS_NSEC_DATAA,
    pub Nsec: DNS_NSEC_DATAA,
    pub DNSKEY: DNS_KEY_DATA,
    pub Dnskey: DNS_KEY_DATA,
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
    pub UNKNOWN: DNS_UNKNOWN_DATA,
    pub Unknown: DNS_UNKNOWN_DATA,
    pub pDataPtr: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_RECORDA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_RECORDA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DNS_RECORDA_1 {
    pub DW: u32,
    pub S: DNS_RECORD_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_RECORDA_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_RECORDA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_RECORDW {
    pub pNext: *mut DNS_RECORDW,
    pub pName: ::windows_sys::core::PWSTR,
    pub wType: u16,
    pub wDataLength: u16,
    pub Flags: DNS_RECORDW_1,
    pub dwTtl: u32,
    pub dwReserved: u32,
    pub Data: DNS_RECORDW_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_RECORDW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_RECORDW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DNS_RECORDW_0 {
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
    pub RRSIG: DNS_SIG_DATAW,
    pub Rrsig: DNS_SIG_DATAW,
    pub NSEC: DNS_NSEC_DATAW,
    pub Nsec: DNS_NSEC_DATAW,
    pub DNSKEY: DNS_KEY_DATA,
    pub Dnskey: DNS_KEY_DATA,
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
    pub UNKNOWN: DNS_UNKNOWN_DATA,
    pub Unknown: DNS_UNKNOWN_DATA,
    pub pDataPtr: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_RECORDW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_RECORDW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DNS_RECORDW_1 {
    pub DW: u32,
    pub S: DNS_RECORD_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_RECORDW_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_RECORDW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_RECORD_FLAGS {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for DNS_RECORD_FLAGS {}
impl ::core::clone::Clone for DNS_RECORD_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RFC_MAX_UDP_PACKET_LENGTH: u32 = 512u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_RRSET {
    pub pFirstRR: *mut DNS_RECORDA,
    pub pLastRR: *mut DNS_RECORDA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_RRSET {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_RRSET {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_A: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_A6: u32 = 9728u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_AAAA: u32 = 7168u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_AFSDB: u32 = 4608u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_ALL: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_ANY: u32 = 65280u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_ATMA: u32 = 8704u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_AXFR: u32 = 64512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_CERT: u32 = 9472u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_CNAME: u32 = 1280u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_DHCID: u32 = 12544u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_DNAME: u32 = 9984u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_DNSKEY: u32 = 12288u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_DS: u32 = 11008u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_EID: u32 = 7936u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_GID: u32 = 26112u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_GPOS: u32 = 6912u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_HINFO: u32 = 3328u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_ISDN: u32 = 5120u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_IXFR: u32 = 64256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_KEY: u32 = 6400u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_KX: u32 = 9216u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_LOC: u32 = 7424u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_MAILA: u32 = 65024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_MAILB: u32 = 64768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_MB: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_MD: u32 = 768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_MF: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_MG: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_MINFO: u32 = 3584u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_MR: u32 = 2304u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_MX: u32 = 3840u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_NAPTR: u32 = 8960u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_NIMLOC: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_NS: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_NSAP: u32 = 5632u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_NSAPPTR: u32 = 5888u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_NSEC: u32 = 12032u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_NSEC3: u32 = 12800u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_NSEC3PARAM: u32 = 13056u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_NULL: u32 = 2560u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_NXT: u32 = 7680u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_OPT: u32 = 10496u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_PTR: u32 = 3072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_PX: u32 = 6656u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_RP: u32 = 4352u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_RRSIG: u32 = 11776u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_RT: u32 = 5376u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_SIG: u32 = 6144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_SINK: u32 = 10240u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_SOA: u32 = 1536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_SRV: u32 = 8448u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_TEXT: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_TKEY: u32 = 63744u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_TLSA: u32 = 13312u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_TSIG: u32 = 64000u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_UID: u32 = 25856u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_UINFO: u32 = 25600u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_UNSPEC: u32 = 26368u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_WINS: u32 = 511u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_WINSR: u32 = 767u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_WKS: u32 = 2816u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_RTYPE_X25: u32 = 4864u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub type DNS_SECTION = i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsSectionQuestion: DNS_SECTION = 0i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsSectionAnswer: DNS_SECTION = 1i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsSectionAuthority: DNS_SECTION = 2i32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DnsSectionAddtional: DNS_SECTION = 3i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_SERVICE_BROWSE_REQUEST {
    pub Version: u32,
    pub InterfaceIndex: u32,
    pub QueryName: ::windows_sys::core::PCWSTR,
    pub Anonymous: DNS_SERVICE_BROWSE_REQUEST_0,
    pub pQueryContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_SERVICE_BROWSE_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_SERVICE_BROWSE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DNS_SERVICE_BROWSE_REQUEST_0 {
    pub pBrowseCallback: PDNS_SERVICE_BROWSE_CALLBACK,
    pub pBrowseCallbackV2: PDNS_QUERY_COMPLETION_ROUTINE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_SERVICE_BROWSE_REQUEST_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_SERVICE_BROWSE_REQUEST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_SERVICE_CANCEL {
    pub reserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DNS_SERVICE_CANCEL {}
impl ::core::clone::Clone for DNS_SERVICE_CANCEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_SERVICE_INSTANCE {
    pub pszInstanceName: ::windows_sys::core::PWSTR,
    pub pszHostName: ::windows_sys::core::PWSTR,
    pub ip4Address: *mut u32,
    pub ip6Address: *mut IP6_ADDRESS,
    pub wPort: u16,
    pub wPriority: u16,
    pub wWeight: u16,
    pub dwPropertyCount: u32,
    pub keys: *mut ::windows_sys::core::PWSTR,
    pub values: *mut ::windows_sys::core::PWSTR,
    pub dwInterfaceIndex: u32,
}
impl ::core::marker::Copy for DNS_SERVICE_INSTANCE {}
impl ::core::clone::Clone for DNS_SERVICE_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_SERVICE_REGISTER_REQUEST {
    pub Version: u32,
    pub InterfaceIndex: u32,
    pub pServiceInstance: *mut DNS_SERVICE_INSTANCE,
    pub pRegisterCompletionCallback: PDNS_SERVICE_REGISTER_COMPLETE,
    pub pQueryContext: *mut ::core::ffi::c_void,
    pub hCredentials: super::super::Foundation::HANDLE,
    pub unicastEnabled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_SERVICE_REGISTER_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_SERVICE_REGISTER_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_SERVICE_RESOLVE_REQUEST {
    pub Version: u32,
    pub InterfaceIndex: u32,
    pub QueryName: ::windows_sys::core::PWSTR,
    pub pResolveCompletionCallback: PDNS_SERVICE_RESOLVE_COMPLETE,
    pub pQueryContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DNS_SERVICE_RESOLVE_REQUEST {}
impl ::core::clone::Clone for DNS_SERVICE_RESOLVE_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_SIG_DATAA {
    pub wTypeCovered: u16,
    pub chAlgorithm: u8,
    pub chLabelCount: u8,
    pub dwOriginalTtl: u32,
    pub dwExpiration: u32,
    pub dwTimeSigned: u32,
    pub wKeyTag: u16,
    pub wSignatureLength: u16,
    pub pNameSigner: ::windows_sys::core::PSTR,
    pub Signature: [u8; 1],
}
impl ::core::marker::Copy for DNS_SIG_DATAA {}
impl ::core::clone::Clone for DNS_SIG_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_SIG_DATAW {
    pub wTypeCovered: u16,
    pub chAlgorithm: u8,
    pub chLabelCount: u8,
    pub dwOriginalTtl: u32,
    pub dwExpiration: u32,
    pub dwTimeSigned: u32,
    pub wKeyTag: u16,
    pub wSignatureLength: u16,
    pub pNameSigner: ::windows_sys::core::PWSTR,
    pub Signature: [u8; 1],
}
impl ::core::marker::Copy for DNS_SIG_DATAW {}
impl ::core::clone::Clone for DNS_SIG_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_SOA_DATAA {
    pub pNamePrimaryServer: ::windows_sys::core::PSTR,
    pub pNameAdministrator: ::windows_sys::core::PSTR,
    pub dwSerialNo: u32,
    pub dwRefresh: u32,
    pub dwRetry: u32,
    pub dwExpire: u32,
    pub dwDefaultTtl: u32,
}
impl ::core::marker::Copy for DNS_SOA_DATAA {}
impl ::core::clone::Clone for DNS_SOA_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_SOA_DATAW {
    pub pNamePrimaryServer: ::windows_sys::core::PWSTR,
    pub pNameAdministrator: ::windows_sys::core::PWSTR,
    pub dwSerialNo: u32,
    pub dwRefresh: u32,
    pub dwRetry: u32,
    pub dwExpire: u32,
    pub dwDefaultTtl: u32,
}
impl ::core::marker::Copy for DNS_SOA_DATAW {}
impl ::core::clone::Clone for DNS_SOA_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_SRV_DATAA {
    pub pNameTarget: ::windows_sys::core::PSTR,
    pub wPriority: u16,
    pub wWeight: u16,
    pub wPort: u16,
    pub Pad: u16,
}
impl ::core::marker::Copy for DNS_SRV_DATAA {}
impl ::core::clone::Clone for DNS_SRV_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_SRV_DATAW {
    pub pNameTarget: ::windows_sys::core::PWSTR,
    pub wPriority: u16,
    pub wWeight: u16,
    pub wPort: u16,
    pub Pad: u16,
}
impl ::core::marker::Copy for DNS_SRV_DATAW {}
impl ::core::clone::Clone for DNS_SRV_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_TKEY_DATAA {
    pub pNameAlgorithm: ::windows_sys::core::PSTR,
    pub pAlgorithmPacket: *mut u8,
    pub pKey: *mut u8,
    pub pOtherData: *mut u8,
    pub dwCreateTime: u32,
    pub dwExpireTime: u32,
    pub wMode: u16,
    pub wError: u16,
    pub wKeyLength: u16,
    pub wOtherLength: u16,
    pub cAlgNameLength: u8,
    pub bPacketPointers: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_TKEY_DATAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_TKEY_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_TKEY_DATAW {
    pub pNameAlgorithm: ::windows_sys::core::PWSTR,
    pub pAlgorithmPacket: *mut u8,
    pub pKey: *mut u8,
    pub pOtherData: *mut u8,
    pub dwCreateTime: u32,
    pub dwExpireTime: u32,
    pub wMode: u16,
    pub wError: u16,
    pub wKeyLength: u16,
    pub wOtherLength: u16,
    pub cAlgNameLength: u8,
    pub bPacketPointers: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_TKEY_DATAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_TKEY_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TKEY_MODE_DIFFIE_HELLMAN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TKEY_MODE_GSS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TKEY_MODE_RESOLVER_ASSIGN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TKEY_MODE_SERVER_ASSIGN: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_TLSA_DATA {
    pub bCertUsage: u8,
    pub bSelector: u8,
    pub bMatchingType: u8,
    pub bCertificateAssociationDataLength: u16,
    pub bPad: [u8; 3],
    pub bCertificateAssociationData: [u8; 1],
}
impl ::core::marker::Copy for DNS_TLSA_DATA {}
impl ::core::clone::Clone for DNS_TLSA_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_TSIG_DATAA {
    pub pNameAlgorithm: ::windows_sys::core::PSTR,
    pub pAlgorithmPacket: *mut u8,
    pub pSignature: *mut u8,
    pub pOtherData: *mut u8,
    pub i64CreateTime: i64,
    pub wFudgeTime: u16,
    pub wOriginalXid: u16,
    pub wError: u16,
    pub wSigLength: u16,
    pub wOtherLength: u16,
    pub cAlgNameLength: u8,
    pub bPacketPointers: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_TSIG_DATAA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_TSIG_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DNS_TSIG_DATAW {
    pub pNameAlgorithm: ::windows_sys::core::PWSTR,
    pub pAlgorithmPacket: *mut u8,
    pub pSignature: *mut u8,
    pub pOtherData: *mut u8,
    pub i64CreateTime: i64,
    pub wFudgeTime: u16,
    pub wOriginalXid: u16,
    pub wError: u16,
    pub wSigLength: u16,
    pub wOtherLength: u16,
    pub cAlgNameLength: u8,
    pub bPacketPointers: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DNS_TSIG_DATAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DNS_TSIG_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_TXT_DATAA {
    pub dwStringCount: u32,
    pub pStringArray: [::windows_sys::core::PSTR; 1],
}
impl ::core::marker::Copy for DNS_TXT_DATAA {}
impl ::core::clone::Clone for DNS_TXT_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_TXT_DATAW {
    pub dwStringCount: u32,
    pub pStringArray: [::windows_sys::core::PWSTR; 1],
}
impl ::core::marker::Copy for DNS_TXT_DATAW {}
impl ::core::clone::Clone for DNS_TXT_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_A: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_A6: u32 = 38u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_AAAA: u32 = 28u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_ADDRS: u32 = 248u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_AFSDB: u32 = 18u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_ALL: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_ANY: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_ATMA: u32 = 34u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_AXFR: u32 = 252u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_CERT: u32 = 37u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_CNAME: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_DHCID: u32 = 49u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_DNAME: u32 = 39u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_DNSKEY: u32 = 48u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_DS: u32 = 43u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_EID: u32 = 31u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_GID: u32 = 102u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_GPOS: u32 = 27u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_HINFO: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_ISDN: u32 = 20u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_IXFR: u32 = 251u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_KEY: u32 = 25u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_KX: u32 = 36u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_LOC: u32 = 29u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_MAILA: u32 = 254u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_MAILB: u32 = 253u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_MB: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_MD: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_MF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_MG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_MINFO: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_MR: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_MX: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_NAPTR: u32 = 35u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_NBSTAT: u32 = 65282u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_NIMLOC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_NS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_NSAP: u32 = 22u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_NSAPPTR: u32 = 23u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_NSEC: u32 = 47u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_NSEC3: u32 = 50u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_NSEC3PARAM: u32 = 51u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_NULL: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_NXT: u32 = 30u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_OPT: u32 = 41u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_PTR: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_PX: u32 = 26u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_RP: u32 = 17u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_RRSIG: u32 = 46u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_RT: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_SIG: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_SINK: u32 = 40u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_SOA: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_SRV: u32 = 33u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_TEXT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_TKEY: u32 = 249u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_TLSA: u32 = 52u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_TSIG: u32 = 250u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_UID: u32 = 101u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_UINFO: u32 = 100u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_UNSPEC: u32 = 103u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_WINS: u32 = 65281u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_WINSR: u32 = 65282u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_WKS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_X25: u32 = 19u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_TYPE_ZERO: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_UNKNOWN_DATA {
    pub dwByteCount: u32,
    pub bData: [u8; 1],
}
impl ::core::marker::Copy for DNS_UNKNOWN_DATA {}
impl ::core::clone::Clone for DNS_UNKNOWN_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_UPDATE_CACHE_SECURITY_CONTEXT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_UPDATE_FORCE_SECURITY_NEGO: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_UPDATE_REMOTE_SERVER: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_UPDATE_RESERVED: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_UPDATE_SECURITY_OFF: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_UPDATE_SECURITY_ON: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_UPDATE_SECURITY_ONLY: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_UPDATE_SECURITY_USE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_UPDATE_SKIP_NO_UPDATE_ADAPTERS: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_UPDATE_TEST_USE_LOCAL_SYS_ACCT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_UPDATE_TRY_ALL_MASTER_SERVERS: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_VALSVR_ERROR_INVALID_ADDR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_VALSVR_ERROR_INVALID_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_VALSVR_ERROR_NO_AUTH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_VALSVR_ERROR_NO_RESPONSE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_VALSVR_ERROR_NO_TCP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_VALSVR_ERROR_REFUSED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_VALSVR_ERROR_UNKNOWN: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_VALSVR_ERROR_UNREACHABLE: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_WINSR_DATAA {
    pub dwMappingFlag: u32,
    pub dwLookupTimeout: u32,
    pub dwCacheTimeout: u32,
    pub pNameResultDomain: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for DNS_WINSR_DATAA {}
impl ::core::clone::Clone for DNS_WINSR_DATAA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_WINSR_DATAW {
    pub dwMappingFlag: u32,
    pub dwLookupTimeout: u32,
    pub dwCacheTimeout: u32,
    pub pNameResultDomain: ::windows_sys::core::PWSTR,
}
impl ::core::marker::Copy for DNS_WINSR_DATAW {}
impl ::core::clone::Clone for DNS_WINSR_DATAW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_WINS_DATA {
    pub dwMappingFlag: u32,
    pub dwLookupTimeout: u32,
    pub dwCacheTimeout: u32,
    pub cWinsServerCount: u32,
    pub WinsServers: [u32; 1],
}
impl ::core::marker::Copy for DNS_WINS_DATA {}
impl ::core::clone::Clone for DNS_WINS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_WINS_FLAG_LOCAL: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const DNS_WINS_FLAG_SCOPE: u32 = 2147483648u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_WIRE_QUESTION {
    pub QuestionType: u16,
    pub QuestionClass: u16,
}
impl ::core::marker::Copy for DNS_WIRE_QUESTION {}
impl ::core::clone::Clone for DNS_WIRE_QUESTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_WIRE_RECORD {
    pub RecordType: u16,
    pub RecordClass: u16,
    pub TimeToLive: u32,
    pub DataLength: u16,
}
impl ::core::marker::Copy for DNS_WIRE_RECORD {}
impl ::core::clone::Clone for DNS_WIRE_RECORD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct DNS_WKS_DATA {
    pub IpAddress: u32,
    pub chProtocol: u8,
    pub BitMask: [u8; 1],
}
impl ::core::marker::Copy for DNS_WKS_DATA {}
impl ::core::clone::Clone for DNS_WKS_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub type DnsContextHandle = isize;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const IP4_ADDRESS_STRING_BUFFER_LENGTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const IP4_ADDRESS_STRING_LENGTH: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct IP4_ARRAY {
    pub AddrCount: u32,
    pub AddrArray: [u32; 1],
}
impl ::core::marker::Copy for IP4_ARRAY {}
impl ::core::clone::Clone for IP4_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub union IP6_ADDRESS {
    pub IP6Qword: [u64; 2],
    pub IP6Dword: [u32; 4],
    pub IP6Word: [u16; 8],
    pub IP6Byte: [u8; 16],
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for IP6_ADDRESS {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for IP6_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
#[cfg(target_arch = "x86")]
pub union IP6_ADDRESS {
    pub IP6Dword: [u32; 4],
    pub IP6Word: [u16; 8],
    pub IP6Byte: [u8; 16],
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for IP6_ADDRESS {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for IP6_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const IP6_ADDRESS_STRING_BUFFER_LENGTH: u32 = 65u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const IP6_ADDRESS_STRING_LENGTH: u32 = 65u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub struct MDNS_QUERY_HANDLE {
    pub nameBuf: [u16; 256],
    pub wType: u16,
    pub pSubscription: *mut ::core::ffi::c_void,
    pub pWnfCallbackParams: *mut ::core::ffi::c_void,
    pub stateNameData: [u32; 2],
}
impl ::core::marker::Copy for MDNS_QUERY_HANDLE {}
impl ::core::clone::Clone for MDNS_QUERY_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MDNS_QUERY_REQUEST {
    pub Version: u32,
    pub ulRefCount: u32,
    pub Query: ::windows_sys::core::PCWSTR,
    pub QueryType: u16,
    pub QueryOptions: u64,
    pub InterfaceIndex: u32,
    pub pQueryCallback: PMDNS_QUERY_CALLBACK,
    pub pQueryContext: *mut ::core::ffi::c_void,
    pub fAnswerReceived: super::super::Foundation::BOOL,
    pub ulResendCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MDNS_QUERY_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MDNS_QUERY_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDNS_QUERY_COMPLETION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(pquerycontext: *const ::core::ffi::c_void, pqueryresults: *mut DNS_QUERY_RESULT)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDNS_SERVICE_BROWSE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(status: u32, pquerycontext: *const ::core::ffi::c_void, pdnsrecord: *const DNS_RECORDW)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub type PDNS_SERVICE_REGISTER_COMPLETE = ::core::option::Option<unsafe extern "system" fn(status: u32, pquerycontext: *const ::core::ffi::c_void, pinstance: *const DNS_SERVICE_INSTANCE)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub type PDNS_SERVICE_RESOLVE_COMPLETE = ::core::option::Option<unsafe extern "system" fn(status: u32, pquerycontext: *const ::core::ffi::c_void, pinstance: *const DNS_SERVICE_INSTANCE)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMDNS_QUERY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pquerycontext: *const ::core::ffi::c_void, pqueryhandle: *mut MDNS_QUERY_HANDLE, pqueryresults: *mut DNS_QUERY_RESULT)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`*"]
pub const SIZEOF_IP4_ADDRESS: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct _DnsRecordOptA {
    pub pNext: *mut DNS_RECORDA,
    pub pName: ::windows_sys::core::PSTR,
    pub wType: u16,
    pub wDataLength: u16,
    pub Flags: _DnsRecordOptA_1,
    pub ExtHeader: DNS_HEADER_EXT,
    pub wPayloadSize: u16,
    pub wReserved: u16,
    pub Data: _DnsRecordOptA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _DnsRecordOptA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _DnsRecordOptA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union _DnsRecordOptA_0 {
    pub OPT: DNS_OPT_DATA,
    pub Opt: DNS_OPT_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _DnsRecordOptA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _DnsRecordOptA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union _DnsRecordOptA_1 {
    pub DW: u32,
    pub S: DNS_RECORD_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _DnsRecordOptA_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _DnsRecordOptA_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct _DnsRecordOptW {
    pub pNext: *mut DNS_RECORDW,
    pub pName: ::windows_sys::core::PWSTR,
    pub wType: u16,
    pub wDataLength: u16,
    pub Flags: _DnsRecordOptW_1,
    pub ExtHeader: DNS_HEADER_EXT,
    pub wPayloadSize: u16,
    pub wReserved: u16,
    pub Data: _DnsRecordOptW_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _DnsRecordOptW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _DnsRecordOptW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union _DnsRecordOptW_0 {
    pub OPT: DNS_OPT_DATA,
    pub Opt: DNS_OPT_DATA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _DnsRecordOptW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _DnsRecordOptW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Dns\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union _DnsRecordOptW_1 {
    pub DW: u32,
    pub S: DNS_RECORD_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _DnsRecordOptW_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _DnsRecordOptW_1 {
    fn clone(&self) -> Self {
        *self
    }
}
