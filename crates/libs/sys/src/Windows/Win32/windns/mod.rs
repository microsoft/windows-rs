#[cfg(all(feature = "Win32_windnsdef", feature = "Win32_winnt"))]
windows_link::link!("dnsapi.dll" "system" fn DnsAcquireContextHandle_A(credentialflags : u32, credentials : *const core::ffi::c_void, pcontext : *mut super::winnt::HANDLE) -> super::windnsdef::DNS_STATUS);
#[cfg(all(feature = "Win32_windnsdef", feature = "Win32_winnt"))]
windows_link::link!("dnsapi.dll" "system" fn DnsAcquireContextHandle_W(credentialflags : u32, credentials : *const core::ffi::c_void, pcontext : *mut super::winnt::HANDLE) -> super::windnsdef::DNS_STATUS);
#[cfg(feature = "Win32_windnsdef")]
windows_link::link!("dnsapi.dll" "system" fn DnsCancelQuery(pcancelhandle : *const DNS_QUERY_CANCEL) -> super::windnsdef::DNS_STATUS);
#[cfg(feature = "Win32_windnsdef")]
windows_link::link!("dnsapi.dll" "system" fn DnsCancelQueryRaw(cancelhandle : *const DNS_QUERY_RAW_CANCEL) -> super::windnsdef::DNS_STATUS);
windows_link::link!("dnsapi.dll" "C" fn DnsConnectionDeletePolicyEntries(policyentrytag : DNS_CONNECTION_POLICY_TAG) -> u32);
windows_link::link!("dnsapi.dll" "C" fn DnsConnectionDeleteProxyInfo(pwszconnectionname : windows_sys::core::PCWSTR, r#type : DNS_CONNECTION_PROXY_TYPE) -> u32);
windows_link::link!("dnsapi.dll" "C" fn DnsConnectionFreeNameList(pnamelist : *mut DNS_CONNECTION_NAME_LIST));
windows_link::link!("dnsapi.dll" "C" fn DnsConnectionFreeProxyInfo(pproxyinfo : *mut DNS_CONNECTION_PROXY_INFO));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("dnsapi.dll" "C" fn DnsConnectionFreeProxyInfoEx(pproxyinfoex : *mut DNS_CONNECTION_PROXY_INFO_EX));
windows_link::link!("dnsapi.dll" "C" fn DnsConnectionFreeProxyList(pproxylist : *mut DNS_CONNECTION_PROXY_LIST));
windows_link::link!("dnsapi.dll" "C" fn DnsConnectionGetNameList(pnamelist : *mut DNS_CONNECTION_NAME_LIST) -> u32);
windows_link::link!("dnsapi.dll" "C" fn DnsConnectionGetProxyInfo(pwszconnectionname : windows_sys::core::PCWSTR, r#type : DNS_CONNECTION_PROXY_TYPE, pproxyinfo : *mut DNS_CONNECTION_PROXY_INFO) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("dnsapi.dll" "C" fn DnsConnectionGetProxyInfoForHostUrl(pwszhosturl : windows_sys::core::PCWSTR, pselectioncontext : *const u8, dwselectioncontextlength : u32, dwexplicitinterfaceindex : u32, pproxyinfoex : *mut DNS_CONNECTION_PROXY_INFO_EX) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("dnsapi.dll" "C" fn DnsConnectionGetProxyInfoForHostUrlEx(pwszhosturl : windows_sys::core::PCWSTR, pselectioncontext : *const u8, dwselectioncontextlength : u32, dwexplicitinterfaceindex : u32, pwszconnectionname : windows_sys::core::PCWSTR, pproxyinfoex : *mut DNS_CONNECTION_PROXY_INFO_EX) -> u32);
windows_link::link!("dnsapi.dll" "C" fn DnsConnectionGetProxyList(pwszconnectionname : windows_sys::core::PCWSTR, pproxylist : *mut DNS_CONNECTION_PROXY_LIST) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("dnsapi.dll" "C" fn DnsConnectionSetPolicyEntries(policyentrytag : DNS_CONNECTION_POLICY_TAG, ppolicyentrylist : *const DNS_CONNECTION_POLICY_ENTRY_LIST) -> u32);
windows_link::link!("dnsapi.dll" "C" fn DnsConnectionSetProxyInfo(pwszconnectionname : windows_sys::core::PCWSTR, r#type : DNS_CONNECTION_PROXY_TYPE, pproxyinfo : *const DNS_CONNECTION_PROXY_INFO) -> u32);
windows_link::link!("dnsapi.dll" "C" fn DnsConnectionUpdateIfIndexTable(pconnectionifindexentries : *const DNS_CONNECTION_IFINDEX_LIST) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
windows_link::link!("dnsapi.dll" "system" fn DnsExtractRecordsFromMessage_UTF8(pdnsbuffer : *const super::windnsdef::DNS_MESSAGE_BUFFER, wmessagelength : u16, pprecord : *mut super::windnsdef::PDNS_RECORD) -> super::windnsdef::DNS_STATUS);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
windows_link::link!("dnsapi.dll" "system" fn DnsExtractRecordsFromMessage_W(pdnsbuffer : *const super::windnsdef::DNS_MESSAGE_BUFFER, wmessagelength : u16, pprecord : *mut super::windnsdef::PDNS_RECORD) -> super::windnsdef::DNS_STATUS);
windows_link::link!("dnsapi.dll" "system" fn DnsFree(pdata : *mut core::ffi::c_void, freetype : DNS_FREE_TYPE));
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("dnsapi.dll" "C" fn DnsFreeCustomServers(pcservers : *mut u32, ppservers : *mut *mut super::windnsdef::DNS_CUSTOM_SERVER));
windows_link::link!("dnsapi.dll" "system" fn DnsFreeProxyName(proxyname : windows_sys::core::PCWSTR));
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("dnsapi.dll" "C" fn DnsGetApplicationSettings(pcservers : *mut u32, ppdefaultservers : *mut *mut super::windnsdef::DNS_CUSTOM_SERVER, psettings : *mut DNS_APPLICATION_SETTINGS) -> u32);
#[cfg(feature = "Win32_windnsdef")]
windows_link::link!("dnsapi.dll" "system" fn DnsGetProxyInformation(hostname : windows_sys::core::PCWSTR, proxyinformation : *mut DNS_PROXY_INFORMATION, defaultproxyinformation : *mut DNS_PROXY_INFORMATION, completionroutine : DNS_PROXY_COMPLETION_ROUTINE, completioncontext : *const core::ffi::c_void) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
windows_link::link!("dnsapi.dll" "system" fn DnsIsFlatRecord(precord : *const super::windnsdef::DNS_RECORDA, ullflags : u64, pfflat : *mut windows_sys::core::BOOL) -> super::windnsdef::DNS_STATUS);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_winnt"))]
windows_link::link!("dnsapi.dll" "system" fn DnsModifyRecordsInSet_A(paddrecords : *const super::windnsdef::DNS_RECORDA, pdeleterecords : *const super::windnsdef::DNS_RECORDA, options : u32, hcredentials : super::winnt::HANDLE, pextralist : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> super::windnsdef::DNS_STATUS);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_winnt"))]
windows_link::link!("dnsapi.dll" "system" fn DnsModifyRecordsInSet_UTF8(paddrecords : *const super::windnsdef::DNS_RECORDA, pdeleterecords : *const super::windnsdef::DNS_RECORDA, options : u32, hcredentials : super::winnt::HANDLE, pextralist : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> super::windnsdef::DNS_STATUS);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_winnt"))]
windows_link::link!("dnsapi.dll" "system" fn DnsModifyRecordsInSet_W(paddrecords : *const super::windnsdef::DNS_RECORDA, pdeleterecords : *const super::windnsdef::DNS_RECORDA, options : u32, hcredentials : super::winnt::HANDLE, pextralist : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> super::windnsdef::DNS_STATUS);
windows_link::link!("dnsapi.dll" "system" fn DnsNameCompare_A(pname1 : windows_sys::core::PCSTR, pname2 : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("dnsapi.dll" "system" fn DnsNameCompare_W(pname1 : windows_sys::core::PCWSTR, pname2 : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windnsdef")]
windows_link::link!("dnsapi.dll" "system" fn DnsQueryConfig(config : DNS_CONFIG_TYPE, flag : u32, pwsadaptername : windows_sys::core::PCWSTR, preserved : *const core::ffi::c_void, pbuffer : *mut core::ffi::c_void, pbuflen : *mut u32) -> super::windnsdef::DNS_STATUS);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
windows_link::link!("dnsapi.dll" "system" fn DnsQueryEx(pqueryrequest : *const DNS_QUERY_REQUEST, pqueryresults : *mut DNS_QUERY_RESULT, pcancelhandle : *mut DNS_QUERY_CANCEL) -> super::windnsdef::DNS_STATUS);
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("dnsapi.dll" "system" fn DnsQueryRaw(queryrequest : *const DNS_QUERY_RAW_REQUEST, cancelhandle : *mut DNS_QUERY_RAW_CANCEL) -> super::windnsdef::DNS_STATUS);
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("dnsapi.dll" "system" fn DnsQueryRawResultFree(queryresults : *mut DNS_QUERY_RAW_RESULT));
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
windows_link::link!("dnsapi.dll" "system" fn DnsQuery_A(pszname : windows_sys::core::PCSTR, wtype : u16, options : u32, pextra : *mut core::ffi::c_void, ppqueryresults : *mut super::windnsdef::PDNS_RECORD, preserved : *mut *mut core::ffi::c_void) -> super::windnsdef::DNS_STATUS);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
windows_link::link!("dnsapi.dll" "system" fn DnsQuery_UTF8(pszname : windows_sys::core::PCSTR, wtype : u16, options : u32, pextra : *mut core::ffi::c_void, ppqueryresults : *mut super::windnsdef::PDNS_RECORD, preserved : *mut *mut core::ffi::c_void) -> super::windnsdef::DNS_STATUS);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
windows_link::link!("dnsapi.dll" "system" fn DnsQuery_W(pszname : windows_sys::core::PCWSTR, wtype : u16, options : u32, pextra : *mut core::ffi::c_void, ppqueryresults : *mut super::windnsdef::PDNS_RECORD, preserved : *mut *mut core::ffi::c_void) -> super::windnsdef::DNS_STATUS);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
windows_link::link!("dnsapi.dll" "system" fn DnsRecordCompare(precord1 : *const super::windnsdef::DNS_RECORDA, precord2 : *const super::windnsdef::DNS_RECORDA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
windows_link::link!("dnsapi.dll" "system" fn DnsRecordCopyEx(precord : *const super::windnsdef::DNS_RECORDA, charsetin : super::windnsdef::DNS_CHARSET, charsetout : super::windnsdef::DNS_CHARSET) -> super::windnsdef::PDNS_RECORD);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
windows_link::link!("dnsapi.dll" "system" fn DnsRecordSetCompare(prr1 : *mut super::windnsdef::DNS_RECORDA, prr2 : *mut super::windnsdef::DNS_RECORDA, ppdiff1 : *mut super::windnsdef::PDNS_RECORD, ppdiff2 : *mut super::windnsdef::PDNS_RECORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
windows_link::link!("dnsapi.dll" "system" fn DnsRecordSetCopyEx(precordset : *const super::windnsdef::DNS_RECORDA, charsetin : super::windnsdef::DNS_CHARSET, charsetout : super::windnsdef::DNS_CHARSET) -> super::windnsdef::PDNS_RECORD);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
windows_link::link!("dnsapi.dll" "C" fn DnsRecordSetDetach(precordlist : *mut super::windnsdef::DNS_RECORDA) -> super::windnsdef::PDNS_RECORD);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("dnsapi.dll" "system" fn DnsReleaseContextHandle(hcontext : super::winnt::HANDLE));
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_winnt"))]
windows_link::link!("dnsapi.dll" "system" fn DnsReplaceRecordSetA(preplaceset : *const super::windnsdef::DNS_RECORDA, options : u32, hcontext : super::winnt::HANDLE, pextrainfo : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> super::windnsdef::DNS_STATUS);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_winnt"))]
windows_link::link!("dnsapi.dll" "system" fn DnsReplaceRecordSetUTF8(preplaceset : *const super::windnsdef::DNS_RECORDA, options : u32, hcontext : super::winnt::HANDLE, pextrainfo : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> super::windnsdef::DNS_STATUS);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_winnt"))]
windows_link::link!("dnsapi.dll" "system" fn DnsReplaceRecordSetW(preplaceset : *const super::windnsdef::DNS_RECORDA, options : u32, hcontext : super::winnt::HANDLE, pextrainfo : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> super::windnsdef::DNS_STATUS);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
windows_link::link!("dnsapi.dll" "system" fn DnsServiceBrowse(prequest : *const DNS_SERVICE_BROWSE_REQUEST, pcancel : *mut DNS_SERVICE_CANCEL) -> super::windnsdef::DNS_STATUS);
#[cfg(feature = "Win32_windnsdef")]
windows_link::link!("dnsapi.dll" "system" fn DnsServiceBrowseCancel(pcancelhandle : *const DNS_SERVICE_CANCEL) -> super::windnsdef::DNS_STATUS);
#[cfg(feature = "Win32_windnsdef")]
windows_link::link!("dnsapi.dll" "system" fn DnsServiceConstructInstance(pservicename : windows_sys::core::PCWSTR, phostname : windows_sys::core::PCWSTR, pip4 : *const u32, pip6 : *const super::windnsdef::IP6_ADDRESS, wport : u16, wpriority : u16, wweight : u16, dwpropertiescount : u32, keys : *const windows_sys::core::PCWSTR, values : *const windows_sys::core::PCWSTR) -> PDNS_SERVICE_INSTANCE);
#[cfg(feature = "Win32_windnsdef")]
windows_link::link!("dnsapi.dll" "system" fn DnsServiceCopyInstance(porig : *const DNS_SERVICE_INSTANCE) -> PDNS_SERVICE_INSTANCE);
#[cfg(all(feature = "Win32_windnsdef", feature = "Win32_winnt"))]
windows_link::link!("dnsapi.dll" "system" fn DnsServiceDeRegister(prequest : *const DNS_SERVICE_REGISTER_REQUEST, pcancel : *mut DNS_SERVICE_CANCEL) -> u32);
#[cfg(feature = "Win32_windnsdef")]
windows_link::link!("dnsapi.dll" "system" fn DnsServiceFreeInstance(pinstance : *const DNS_SERVICE_INSTANCE));
#[cfg(all(feature = "Win32_windnsdef", feature = "Win32_winnt"))]
windows_link::link!("dnsapi.dll" "system" fn DnsServiceRegister(prequest : *const DNS_SERVICE_REGISTER_REQUEST, pcancel : *mut DNS_SERVICE_CANCEL) -> u32);
windows_link::link!("dnsapi.dll" "system" fn DnsServiceRegisterCancel(pcancelhandle : *const DNS_SERVICE_CANCEL) -> u32);
#[cfg(feature = "Win32_windnsdef")]
windows_link::link!("dnsapi.dll" "system" fn DnsServiceResolve(prequest : *const DNS_SERVICE_RESOLVE_REQUEST, pcancel : *mut DNS_SERVICE_CANCEL) -> super::windnsdef::DNS_STATUS);
#[cfg(feature = "Win32_windnsdef")]
windows_link::link!("dnsapi.dll" "system" fn DnsServiceResolveCancel(pcancelhandle : *const DNS_SERVICE_CANCEL) -> super::windnsdef::DNS_STATUS);
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
windows_link::link!("dnsapi.dll" "C" fn DnsSetApplicationSettings(cservers : u32, pservers : *const super::windnsdef::DNS_CUSTOM_SERVER, psettings : *const DNS_APPLICATION_SETTINGS) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
windows_link::link!("dnsapi.dll" "system" fn DnsStartMulticastQuery(pqueryrequest : *const MDNS_QUERY_REQUEST, phandle : *mut MDNS_QUERY_HANDLE) -> super::windnsdef::DNS_STATUS);
#[cfg(feature = "Win32_windnsdef")]
windows_link::link!("dnsapi.dll" "system" fn DnsStopMulticastQuery(phandle : *mut MDNS_QUERY_HANDLE) -> super::windnsdef::DNS_STATUS);
#[cfg(feature = "Win32_windnsdef")]
windows_link::link!("dnsapi.dll" "C" fn DnsValidateName_A(pszname : windows_sys::core::PCSTR, format : DNS_NAME_FORMAT) -> super::windnsdef::DNS_STATUS);
#[cfg(feature = "Win32_windnsdef")]
windows_link::link!("dnsapi.dll" "C" fn DnsValidateName_UTF8(pszname : windows_sys::core::PCSTR, format : DNS_NAME_FORMAT) -> super::windnsdef::DNS_STATUS);
#[cfg(feature = "Win32_windnsdef")]
windows_link::link!("dnsapi.dll" "C" fn DnsValidateName_W(pszname : windows_sys::core::PCWSTR, format : DNS_NAME_FORMAT) -> super::windnsdef::DNS_STATUS);
#[cfg(all(feature = "Win32_windnsdef", feature = "Win32_ws2def"))]
windows_link::link!("dnsapi.dll" "C" fn DnsValidateServerStatus(server : *const super::ws2def::SOCKADDR, queryname : windows_sys::core::PCWSTR, serverstatus : *mut u32) -> super::windnsdef::DNS_STATUS);
#[cfg(feature = "Win32_windnsdef")]
windows_link::link!("dnsapi.dll" "system" fn DnsWriteQuestionToBuffer_UTF8(pdnsbuffer : *mut super::windnsdef::DNS_MESSAGE_BUFFER, pdwbuffersize : *mut u32, pszname : windows_sys::core::PCSTR, wtype : u16, xid : u16, frecursiondesired : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_windnsdef")]
windows_link::link!("dnsapi.dll" "system" fn DnsWriteQuestionToBuffer_W(pdnsbuffer : *mut super::windnsdef::DNS_MESSAGE_BUFFER, pdwbuffersize : *mut u32, pszname : windows_sys::core::PCWSTR, wtype : u16, xid : u16, frecursiondesired : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
pub const DNS_ADDRESS_STRING_LENGTH: u32 = 65;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DNS_APPLICATION_SETTINGS {
    pub Version: u32,
    pub Flags: u64,
}
pub const DNS_APP_SETTINGS_EXCLUSIVE_SERVERS: u32 = 1;
pub const DNS_APP_SETTINGS_VERSION1: u32 = 1;
pub const DNS_CLASS_ALL: u32 = 255;
pub const DNS_CLASS_ANY: u32 = 255;
pub const DNS_CLASS_CHAOS: u32 = 3;
pub const DNS_CLASS_CSNET: u32 = 2;
pub const DNS_CLASS_HESIOD: u32 = 4;
pub const DNS_CLASS_INTERNET: u32 = 1;
pub const DNS_CLASS_NONE: u32 = 254;
pub const DNS_CLASS_UNICAST_RESPONSE: u32 = 32768;
pub const DNS_COMPRESSED_QUESTION_NAME: u32 = 49164;
pub const DNS_CONFIG_FLAG_ALLOC: u32 = 1;
pub type DNS_CONFIG_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_CONNECTION_IFINDEX_ENTRY {
    pub pwszConnectionName: windows_sys::core::PCWSTR,
    pub dwIfIndex: u32,
}
impl Default for DNS_CONNECTION_IFINDEX_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_CONNECTION_IFINDEX_LIST {
    pub pConnectionIfIndexEntries: *mut DNS_CONNECTION_IFINDEX_ENTRY,
    pub nEntries: u32,
}
impl Default for DNS_CONNECTION_IFINDEX_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_CONNECTION_NAME {
    pub wszName: [u16; 65],
}
impl Default for DNS_CONNECTION_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_CONNECTION_NAME_LIST {
    pub cNames: u32,
    pub pNames: *mut DNS_CONNECTION_NAME,
}
impl Default for DNS_CONNECTION_NAME_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_CONNECTION_NAME_MAX_LENGTH: u32 = 64;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct DNS_CONNECTION_POLICY_ENTRY {
    pub pwszHost: windows_sys::core::PCWSTR,
    pub pwszAppId: windows_sys::core::PCWSTR,
    pub cbAppSid: u32,
    pub pbAppSid: super::minwindef::PBYTE,
    pub nConnections: u32,
    pub ppwszConnections: *mut windows_sys::core::PCWSTR,
    pub dwPolicyEntryFlags: u32,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DNS_CONNECTION_POLICY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct DNS_CONNECTION_POLICY_ENTRY_LIST {
    pub pPolicyEntries: *mut DNS_CONNECTION_POLICY_ENTRY,
    pub nEntries: u32,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DNS_CONNECTION_POLICY_ENTRY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_CONNECTION_POLICY_ENTRY_ONDEMAND: u32 = 1;
pub type DNS_CONNECTION_POLICY_TAG = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_CONNECTION_PROXY_ELEMENT {
    pub Type: DNS_CONNECTION_PROXY_TYPE,
    pub Info: DNS_CONNECTION_PROXY_INFO,
}
impl Default for DNS_CONNECTION_PROXY_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_CONNECTION_PROXY_INFO {
    pub Version: u32,
    pub pwszFriendlyName: *mut u16,
    pub Flags: u32,
    pub Switch: DNS_CONNECTION_PROXY_INFO_SWITCH,
    pub Anonymous: DNS_CONNECTION_PROXY_INFO_0,
}
impl Default for DNS_CONNECTION_PROXY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DNS_CONNECTION_PROXY_INFO_0 {
    pub Config: DNS_CONNECTION_PROXY_INFO_0_0,
    pub Script: DNS_CONNECTION_PROXY_INFO_0_1,
}
impl Default for DNS_CONNECTION_PROXY_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_CONNECTION_PROXY_INFO_0_0 {
    pub pwszServer: *mut u16,
    pub pwszUsername: *mut u16,
    pub pwszPassword: *mut u16,
    pub pwszException: *mut u16,
    pub pwszExtraInfo: *mut u16,
    pub Port: u16,
}
impl Default for DNS_CONNECTION_PROXY_INFO_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_CONNECTION_PROXY_INFO_0_1 {
    pub pwszScript: *mut u16,
    pub pwszUsername: *mut u16,
    pub pwszPassword: *mut u16,
}
impl Default for DNS_CONNECTION_PROXY_INFO_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_CONNECTION_PROXY_INFO_CURRENT_VERSION: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct DNS_CONNECTION_PROXY_INFO_EX {
    pub ProxyInfo: DNS_CONNECTION_PROXY_INFO,
    pub dwInterfaceIndex: u32,
    pub pwszConnectionName: *mut u16,
    pub fDirectConfiguration: windows_sys::core::BOOL,
    pub hConnection: super::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for DNS_CONNECTION_PROXY_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_CONNECTION_PROXY_INFO_EXCEPTION_MAX_LENGTH: u32 = 1024;
pub const DNS_CONNECTION_PROXY_INFO_EXTRA_INFO_MAX_LENGTH: u32 = 1024;
pub const DNS_CONNECTION_PROXY_INFO_FLAG_BYPASSLOCAL: u32 = 2;
pub const DNS_CONNECTION_PROXY_INFO_FLAG_DISABLED: u32 = 1;
pub const DNS_CONNECTION_PROXY_INFO_FRIENDLY_NAME_MAX_LENGTH: u32 = 64;
pub const DNS_CONNECTION_PROXY_INFO_PASSWORD_MAX_LENGTH: u32 = 128;
pub const DNS_CONNECTION_PROXY_INFO_SERVER_MAX_LENGTH: u32 = 256;
pub type DNS_CONNECTION_PROXY_INFO_SWITCH = i32;
pub const DNS_CONNECTION_PROXY_INFO_SWITCH_CONFIG: DNS_CONNECTION_PROXY_INFO_SWITCH = 0;
pub const DNS_CONNECTION_PROXY_INFO_SWITCH_SCRIPT: DNS_CONNECTION_PROXY_INFO_SWITCH = 1;
pub const DNS_CONNECTION_PROXY_INFO_SWITCH_WPAD: DNS_CONNECTION_PROXY_INFO_SWITCH = 2;
pub const DNS_CONNECTION_PROXY_INFO_USERNAME_MAX_LENGTH: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_CONNECTION_PROXY_LIST {
    pub cProxies: u32,
    pub pProxies: *mut DNS_CONNECTION_PROXY_ELEMENT,
}
impl Default for DNS_CONNECTION_PROXY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DNS_CONNECTION_PROXY_TYPE = i32;
pub const DNS_CONNECTION_PROXY_TYPE_HTTP: DNS_CONNECTION_PROXY_TYPE = 1;
pub const DNS_CONNECTION_PROXY_TYPE_NULL: DNS_CONNECTION_PROXY_TYPE = 0;
pub const DNS_CONNECTION_PROXY_TYPE_SOCKS4: DNS_CONNECTION_PROXY_TYPE = 4;
pub const DNS_CONNECTION_PROXY_TYPE_SOCKS5: DNS_CONNECTION_PROXY_TYPE = 5;
pub const DNS_CONNECTION_PROXY_TYPE_WAP: DNS_CONNECTION_PROXY_TYPE = 2;
pub type DNS_FREE_TYPE = i32;
pub const DNS_IP4_REVERSE_DOMAIN_STRING_A: windows_sys::core::PCSTR = windows_sys::core::s!("in-addr.arpa.");
pub const DNS_IP4_REVERSE_DOMAIN_STRING_W: windows_sys::core::PCWSTR = windows_sys::core::w!("in-addr.arpa.");
pub const DNS_IP6_REVERSE_DOMAIN_STRING_A: windows_sys::core::PCSTR = windows_sys::core::s!("ip6.arpa.");
pub const DNS_IP6_REVERSE_DOMAIN_STRING_W: windows_sys::core::PCWSTR = windows_sys::core::w!("ip6.arpa.");
pub const DNS_MAX_IP4_REVERSE_NAME_BUFFER_LENGTH: u32 = 31;
pub const DNS_MAX_IP4_REVERSE_NAME_LENGTH: u32 = 31;
pub const DNS_MAX_IP6_REVERSE_NAME_BUFFER_LENGTH: u32 = 75;
pub const DNS_MAX_IP6_REVERSE_NAME_LENGTH: u32 = 75;
pub const DNS_MAX_REVERSE_NAME_BUFFER_LENGTH: u32 = 75;
pub const DNS_MAX_REVERSE_NAME_LENGTH: u32 = 75;
pub const DNS_MAX_TEXT_STRING_LENGTH: u32 = 255;
pub type DNS_NAME_FORMAT = i32;
pub const DNS_OPCODE_IQUERY: u32 = 1;
pub const DNS_OPCODE_NOTIFY: u32 = 4;
pub const DNS_OPCODE_QUERY: u32 = 0;
pub const DNS_OPCODE_SERVER_STATUS: u32 = 2;
pub const DNS_OPCODE_UNKNOWN: u32 = 3;
pub const DNS_OPCODE_UPDATE: u32 = 5;
pub const DNS_PORT_HOST_ORDER: u32 = 53;
pub const DNS_PORT_NET_ORDER: u32 = 13568;
pub const DNS_PROTOCOL_DOH: u32 = 3;
pub const DNS_PROTOCOL_DOT: u32 = 4;
pub const DNS_PROTOCOL_NO_WIRE: u32 = 5;
pub const DNS_PROTOCOL_TCP: u32 = 2;
pub const DNS_PROTOCOL_UDP: u32 = 1;
pub const DNS_PROTOCOL_UNSPECIFIED: u32 = 0;
#[cfg(feature = "Win32_windnsdef")]
pub type DNS_PROXY_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(completioncontext: *const core::ffi::c_void, status: super::windnsdef::DNS_STATUS)>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_PROXY_INFORMATION {
    pub version: u32,
    pub proxyInformationType: DNS_PROXY_INFORMATION_TYPE,
    pub proxyName: windows_sys::core::PWSTR,
}
impl Default for DNS_PROXY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_PROXY_INFORMATION_DEFAULT_SETTINGS: DNS_PROXY_INFORMATION_TYPE = 1;
pub const DNS_PROXY_INFORMATION_DIRECT: DNS_PROXY_INFORMATION_TYPE = 0;
pub const DNS_PROXY_INFORMATION_DOES_NOT_EXIST: DNS_PROXY_INFORMATION_TYPE = 3;
pub const DNS_PROXY_INFORMATION_PROXY_NAME: DNS_PROXY_INFORMATION_TYPE = 2;
pub type DNS_PROXY_INFORMATION_TYPE = i32;
pub const DNS_QUERY_ACCEPT_TRUNCATED_RESPONSE: u32 = 1;
pub const DNS_QUERY_ADDRCONFIG: u32 = 8192;
pub const DNS_QUERY_APPEND_MULTILABEL: u32 = 8388608;
pub const DNS_QUERY_BYPASS_CACHE: u32 = 8;
pub const DNS_QUERY_CACHE_ONLY: u32 = 16;
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct DNS_QUERY_CANCEL {
    pub Reserved: [i8; 32],
}
impl Default for DNS_QUERY_CANCEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
pub type DNS_QUERY_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(pquerycontext: *const core::ffi::c_void, pqueryresults: *mut DNS_QUERY_RESULT)>;
pub const DNS_QUERY_DISABLE_IDN_ENCODING: u32 = 2097152;
pub const DNS_QUERY_DNSSEC_CHECKING_DISABLED: u32 = 33554432;
pub const DNS_QUERY_DNSSEC_OK: u32 = 16777216;
pub const DNS_QUERY_DNSSEC_REQUIRED: u32 = 67108864;
pub const DNS_QUERY_DONT_RESET_TTL_VALUES: u32 = 1048576;
pub const DNS_QUERY_DUAL_ADDR: u32 = 16384;
pub const DNS_QUERY_MULTICAST_ONLY: u32 = 1024;
pub const DNS_QUERY_NO_HOSTS_FILE: u32 = 64;
pub const DNS_QUERY_NO_LOCAL_NAME: u32 = 32;
pub const DNS_QUERY_NO_MULTICAST: u32 = 2048;
pub const DNS_QUERY_NO_NETBT: u32 = 128;
pub const DNS_QUERY_NO_RECURSION: u32 = 4;
pub const DNS_QUERY_NO_WIRE_QUERY: u32 = 16;
pub const DNS_QUERY_PARSE_ALL_RECORDS: u64 = 288230376151711744;
#[repr(C, align(8))]
#[derive(Clone, Copy)]
pub struct DNS_QUERY_RAW_CANCEL {
    pub reserved: [i8; 32],
}
impl Default for DNS_QUERY_RAW_CANCEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type DNS_QUERY_RAW_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(querycontext: *const core::ffi::c_void, queryresults: *const DNS_QUERY_RAW_RESULT)>;
pub const DNS_QUERY_RAW_OPTION_BEST_EFFORT_PARSE: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct DNS_QUERY_RAW_REQUEST {
    pub version: u32,
    pub resultsVersion: u32,
    pub dnsQueryRawSize: u32,
    pub dnsQueryRaw: *mut u8,
    pub dnsQueryName: windows_sys::core::PWSTR,
    pub dnsQueryType: u16,
    pub queryOptions: u64,
    pub interfaceIndex: u32,
    pub queryCompletionCallback: DNS_QUERY_RAW_COMPLETION_ROUTINE,
    pub queryContext: *mut core::ffi::c_void,
    pub queryRawOptions: u64,
    pub customServersSize: u32,
    pub customServers: *mut super::windnsdef::DNS_CUSTOM_SERVER,
    pub protocol: u32,
    pub Anonymous: DNS_QUERY_RAW_REQUEST_0,
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for DNS_QUERY_RAW_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub union DNS_QUERY_RAW_REQUEST_0 {
    pub sourceAddr: super::ws2ipdef::SOCKADDR_INET,
    pub maxSa: [i8; 32],
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for DNS_QUERY_RAW_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_QUERY_RAW_REQUEST_VERSION1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct DNS_QUERY_RAW_RESULT {
    pub version: u32,
    pub queryStatus: super::windnsdef::DNS_STATUS,
    pub queryOptions: u64,
    pub queryRawOptions: u64,
    pub responseFlags: u64,
    pub queryRawResponseSize: u32,
    pub queryRawResponse: *mut u8,
    pub queryRecords: super::windnsdef::PDNS_RECORD,
    pub protocol: u32,
    pub Anonymous: DNS_QUERY_RAW_RESULT_0,
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for DNS_QUERY_RAW_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub union DNS_QUERY_RAW_RESULT_0 {
    pub sourceAddr: super::ws2ipdef::SOCKADDR_INET,
    pub maxSa: [i8; 32],
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for DNS_QUERY_RAW_RESULT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_QUERY_RAW_RESULTS_VERSION1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
#[derive(Clone, Copy)]
pub struct DNS_QUERY_REQUEST {
    pub Version: u32,
    pub QueryName: windows_sys::core::PCWSTR,
    pub QueryType: u16,
    pub QueryOptions: u64,
    pub pDnsServerList: super::windnsdef::PDNS_ADDR_ARRAY,
    pub InterfaceIndex: u32,
    pub pQueryCompletionCallback: PDNS_QUERY_COMPLETION_ROUTINE,
    pub pQueryContext: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
impl Default for DNS_QUERY_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
#[derive(Clone, Copy)]
pub struct DNS_QUERY_REQUEST3 {
    pub Version: u32,
    pub QueryName: windows_sys::core::PCWSTR,
    pub QueryType: u16,
    pub QueryOptions: u64,
    pub pDnsServerList: super::windnsdef::PDNS_ADDR_ARRAY,
    pub InterfaceIndex: u32,
    pub pQueryCompletionCallback: PDNS_QUERY_COMPLETION_ROUTINE,
    pub pQueryContext: *mut core::ffi::c_void,
    pub IsNetworkQueryRequired: windows_sys::core::BOOL,
    pub RequiredNetworkIndex: u32,
    pub cCustomServers: u32,
    pub pCustomServers: *mut super::windnsdef::DNS_CUSTOM_SERVER,
}
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
impl Default for DNS_QUERY_REQUEST3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_QUERY_REQUEST_VERSION1: u32 = 1;
pub const DNS_QUERY_REQUEST_VERSION2: u32 = 2;
pub const DNS_QUERY_REQUEST_VERSION3: u32 = 3;
pub const DNS_QUERY_RESERVED: u32 = 4026531840;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
#[derive(Clone, Copy)]
pub struct DNS_QUERY_RESULT {
    pub Version: u32,
    pub QueryStatus: super::windnsdef::DNS_STATUS,
    pub QueryOptions: u64,
    pub pQueryRecords: super::windnsdef::PDNS_RECORD,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
impl Default for DNS_QUERY_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_QUERY_RESULTS_VERSION1: u32 = 1;
pub const DNS_QUERY_RETURN_MESSAGE: u32 = 512;
pub const DNS_QUERY_STANDARD: u32 = 0;
pub const DNS_QUERY_TREAT_AS_FQDN: u32 = 4096;
pub const DNS_QUERY_USE_TCP_ONLY: u32 = 2;
pub const DNS_QUERY_WIRE_ONLY: u32 = 256;
pub const DNS_RCLASS_ALL: u32 = 65280;
pub const DNS_RCLASS_ANY: u32 = 65280;
pub const DNS_RCLASS_CHAOS: u32 = 768;
pub const DNS_RCLASS_CSNET: u32 = 512;
pub const DNS_RCLASS_HESIOD: u32 = 1024;
pub const DNS_RCLASS_INTERNET: u32 = 256;
pub const DNS_RCLASS_MDNS_CACHE_FLUSH: u32 = 128;
pub const DNS_RCLASS_NONE: u32 = 65024;
pub const DNS_RCLASS_UNICAST_RESPONSE: u32 = 128;
pub const DNS_RCODE_BADKEY: u32 = 17;
pub const DNS_RCODE_BADSIG: u32 = 16;
pub const DNS_RCODE_BADTIME: u32 = 18;
pub const DNS_RCODE_BADVERS: u32 = 16;
pub const DNS_RCODE_FORMAT_ERROR: u32 = 1;
pub const DNS_RCODE_FORMERR: u32 = 1;
pub const DNS_RCODE_MAX: u32 = 15;
pub const DNS_RCODE_NAME_ERROR: u32 = 3;
pub const DNS_RCODE_NOERROR: u32 = 0;
pub const DNS_RCODE_NOTAUTH: u32 = 9;
pub const DNS_RCODE_NOTIMPL: u32 = 4;
pub const DNS_RCODE_NOTZONE: u32 = 10;
pub const DNS_RCODE_NOT_IMPLEMENTED: u32 = 4;
pub const DNS_RCODE_NO_ERROR: u32 = 0;
pub const DNS_RCODE_NXDOMAIN: u32 = 3;
pub const DNS_RCODE_NXRRSET: u32 = 8;
pub const DNS_RCODE_REFUSED: u32 = 5;
pub const DNS_RCODE_SERVER_FAILURE: u32 = 2;
pub const DNS_RCODE_SERVFAIL: u32 = 2;
pub const DNS_RCODE_YXDOMAIN: u32 = 6;
pub const DNS_RCODE_YXRRSET: u32 = 7;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
#[derive(Clone, Copy)]
pub struct DNS_RRSET {
    pub pFirstRR: super::windnsdef::PDNS_RECORD,
    pub pLastRR: super::windnsdef::PDNS_RECORD,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
impl Default for DNS_RRSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
pub type DNS_SERVICE_BROWSE_CALLBACK = Option<unsafe extern "system" fn(status: u32, pquerycontext: *const core::ffi::c_void, pdnsrecord: *const super::windnsdef::DNS_RECORDA)>;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
#[derive(Clone, Copy)]
pub struct DNS_SERVICE_BROWSE_REQUEST {
    pub Version: u32,
    pub InterfaceIndex: u32,
    pub QueryName: windows_sys::core::PCWSTR,
    pub Anonymous: DNS_SERVICE_BROWSE_REQUEST_0,
    pub pQueryContext: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
impl Default for DNS_SERVICE_BROWSE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
#[derive(Clone, Copy)]
pub union DNS_SERVICE_BROWSE_REQUEST_0 {
    pub pBrowseCallback: PDNS_SERVICE_BROWSE_CALLBACK,
    pub pBrowseCallbackV2: DNS_QUERY_COMPLETION_ROUTINE,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
impl Default for DNS_SERVICE_BROWSE_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DNS_SERVICE_CANCEL {
    pub reserved: *mut core::ffi::c_void,
}
impl Default for DNS_SERVICE_CANCEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windnsdef")]
#[derive(Clone, Copy)]
pub struct DNS_SERVICE_INSTANCE {
    pub pszInstanceName: windows_sys::core::PWSTR,
    pub pszHostName: windows_sys::core::PWSTR,
    pub ip4Address: *mut super::windnsdef::IP4_ADDRESS,
    pub ip6Address: *mut super::windnsdef::IP6_ADDRESS,
    pub wPort: u16,
    pub wPriority: u16,
    pub wWeight: u16,
    pub dwPropertyCount: u32,
    pub keys: *mut windows_sys::core::PWSTR,
    pub values: *mut windows_sys::core::PWSTR,
    pub dwInterfaceIndex: u32,
}
#[cfg(feature = "Win32_windnsdef")]
impl Default for DNS_SERVICE_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_windnsdef")]
pub type DNS_SERVICE_REGISTER_COMPLETE = Option<unsafe extern "system" fn(status: u32, pquerycontext: *const core::ffi::c_void, pinstance: *const DNS_SERVICE_INSTANCE)>;
#[repr(C)]
#[cfg(all(feature = "Win32_windnsdef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct DNS_SERVICE_REGISTER_REQUEST {
    pub Version: u32,
    pub InterfaceIndex: u32,
    pub pServiceInstance: PDNS_SERVICE_INSTANCE,
    pub pRegisterCompletionCallback: PDNS_SERVICE_REGISTER_COMPLETE,
    pub pQueryContext: *mut core::ffi::c_void,
    pub hCredentials: super::winnt::HANDLE,
    pub unicastEnabled: windows_sys::core::BOOL,
}
#[cfg(all(feature = "Win32_windnsdef", feature = "Win32_winnt"))]
impl Default for DNS_SERVICE_REGISTER_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_windnsdef")]
pub type DNS_SERVICE_RESOLVE_COMPLETE = Option<unsafe extern "system" fn(status: u32, pquerycontext: *const core::ffi::c_void, pinstance: *const DNS_SERVICE_INSTANCE)>;
#[repr(C)]
#[cfg(feature = "Win32_windnsdef")]
#[derive(Clone, Copy)]
pub struct DNS_SERVICE_RESOLVE_REQUEST {
    pub Version: u32,
    pub InterfaceIndex: u32,
    pub QueryName: windows_sys::core::PWSTR,
    pub pResolveCompletionCallback: PDNS_SERVICE_RESOLVE_COMPLETE,
    pub pQueryContext: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_windnsdef")]
impl Default for DNS_SERVICE_RESOLVE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_UPDATE_CACHE_SECURITY_CONTEXT: u32 = 512;
pub const DNS_UPDATE_FORCE_SECURITY_NEGO: u32 = 2048;
pub const DNS_UPDATE_REMOTE_SERVER: u32 = 16384;
pub const DNS_UPDATE_RESERVED: u32 = 4294901760;
pub const DNS_UPDATE_SECURITY_OFF: u32 = 16;
pub const DNS_UPDATE_SECURITY_ON: u32 = 32;
pub const DNS_UPDATE_SECURITY_ONLY: u32 = 256;
pub const DNS_UPDATE_SECURITY_USE_DEFAULT: u32 = 0;
pub const DNS_UPDATE_SKIP_NO_UPDATE_ADAPTERS: u32 = 8192;
pub const DNS_UPDATE_TEST_USE_LOCAL_SYS_ACCT: u32 = 1024;
pub const DNS_UPDATE_TRY_ALL_MASTER_SERVERS: u32 = 4096;
pub const DNS_VALSVR_ERROR_INVALID_ADDR: u32 = 1;
pub const DNS_VALSVR_ERROR_INVALID_NAME: u32 = 2;
pub const DNS_VALSVR_ERROR_NO_AUTH: u32 = 5;
pub const DNS_VALSVR_ERROR_NO_RESPONSE: u32 = 4;
pub const DNS_VALSVR_ERROR_NO_TCP: u32 = 16;
pub const DNS_VALSVR_ERROR_REFUSED: u32 = 6;
pub const DNS_VALSVR_ERROR_UNKNOWN: u32 = 255;
pub const DNS_VALSVR_ERROR_UNREACHABLE: u32 = 3;
pub const DNS_WINS_FLAG_LOCAL: u32 = 65536;
pub const DNS_WINS_FLAG_SCOPE: u32 = 2147483648;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DNS_WIRE_QUESTION {
    pub QuestionType: u16,
    pub QuestionClass: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DNS_WIRE_RECORD {
    pub RecordType: u16,
    pub RecordClass: u16,
    pub TimeToLive: u32,
    pub DataLength: u16,
}
pub const DnsConfigAdapterDomainName_A: DNS_CONFIG_TYPE = 4;
pub const DnsConfigAdapterDomainName_UTF8: DNS_CONFIG_TYPE = 5;
pub const DnsConfigAdapterDomainName_W: DNS_CONFIG_TYPE = 3;
pub const DnsConfigAdapterHostNameRegistrationEnabled: DNS_CONFIG_TYPE = 10;
pub const DnsConfigAdapterInfo: DNS_CONFIG_TYPE = 8;
pub const DnsConfigAddressRegistrationMaxCount: DNS_CONFIG_TYPE = 11;
pub const DnsConfigDnsServerList: DNS_CONFIG_TYPE = 6;
pub const DnsConfigFullHostName_A: DNS_CONFIG_TYPE = 16;
pub const DnsConfigFullHostName_UTF8: DNS_CONFIG_TYPE = 17;
pub const DnsConfigFullHostName_W: DNS_CONFIG_TYPE = 15;
pub const DnsConfigHostName_A: DNS_CONFIG_TYPE = 13;
pub const DnsConfigHostName_UTF8: DNS_CONFIG_TYPE = 14;
pub const DnsConfigHostName_W: DNS_CONFIG_TYPE = 12;
pub const DnsConfigNameServer: DNS_CONFIG_TYPE = 18;
pub const DnsConfigPrimaryDomainName_A: DNS_CONFIG_TYPE = 1;
pub const DnsConfigPrimaryDomainName_UTF8: DNS_CONFIG_TYPE = 2;
pub const DnsConfigPrimaryDomainName_W: DNS_CONFIG_TYPE = 0;
pub const DnsConfigPrimaryHostNameRegistrationEnabled: DNS_CONFIG_TYPE = 9;
pub const DnsConfigSearchList: DNS_CONFIG_TYPE = 7;
pub const DnsFreeFlat: DNS_FREE_TYPE = 0;
pub const DnsFreeParsedMessageFields: DNS_FREE_TYPE = 2;
pub const DnsFreeRecordList: DNS_FREE_TYPE = 1;
pub const DnsFreeRecordListDeep: u32 = 1;
pub const DnsNameDomain: DNS_NAME_FORMAT = 0;
pub const DnsNameDomainLabel: DNS_NAME_FORMAT = 1;
pub const DnsNameHostnameFull: DNS_NAME_FORMAT = 2;
pub const DnsNameHostnameLabel: DNS_NAME_FORMAT = 3;
pub const DnsNameSrvRecord: DNS_NAME_FORMAT = 5;
pub const DnsNameValidateTld: DNS_NAME_FORMAT = 6;
pub const DnsNameWildcard: DNS_NAME_FORMAT = 4;
pub const INTERNET_DEFAULT_DNS_PORT: u32 = 53;
pub const INTERNET_DEFAULT_DOT_PORT: u32 = 853;
#[repr(C)]
#[cfg(feature = "Win32_windnsdef")]
#[derive(Clone, Copy)]
pub struct IP4_ARRAY {
    pub AddrCount: u32,
    pub AddrArray: [super::windnsdef::IP4_ADDRESS; 1],
}
#[cfg(feature = "Win32_windnsdef")]
impl Default for IP4_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IP6_ADDRESS_STRING_BUFFER_LENGTH: u32 = 65;
pub const IP6_ADDRESS_STRING_LENGTH: u32 = 65;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
pub type MDNS_QUERY_CALLBACK = Option<unsafe extern "system" fn(pquerycontext: *const core::ffi::c_void, pqueryhandle: *mut MDNS_QUERY_HANDLE, pqueryresults: *mut DNS_QUERY_RESULT)>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MDNS_QUERY_HANDLE {
    pub nameBuf: [u16; 256],
    pub wType: u16,
    pub pSubscription: *mut core::ffi::c_void,
    pub pWnfCallbackParams: *mut core::ffi::c_void,
    pub stateNameData: [u32; 2],
}
impl Default for MDNS_QUERY_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
#[derive(Clone, Copy)]
pub struct MDNS_QUERY_REQUEST {
    pub Version: u32,
    pub ulRefCount: u32,
    pub Query: windows_sys::core::PCWSTR,
    pub QueryType: u16,
    pub QueryOptions: u64,
    pub InterfaceIndex: u32,
    pub pQueryCallback: PMDNS_QUERY_CALLBACK,
    pub pQueryContext: *mut core::ffi::c_void,
    pub fAnswerReceived: windows_sys::core::BOOL,
    pub ulResendCount: u32,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
impl Default for MDNS_QUERY_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
pub type PDNS_CONNECTION_POLICY_ENTRY = *mut DNS_CONNECTION_POLICY_ENTRY;
pub type PDNS_CONNECTION_PROXY_INFO = *mut DNS_CONNECTION_PROXY_INFO;
#[cfg(feature = "Win32_winnt")]
pub type PDNS_CONNECTION_PROXY_INFO_EX = *mut DNS_CONNECTION_PROXY_INFO_EX;
pub type PDNS_QUERY_CANCEL = *mut DNS_QUERY_CANCEL;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
pub type PDNS_QUERY_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(pquerycontext: *const core::ffi::c_void, pqueryresults: *mut DNS_QUERY_RESULT)>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
pub type PDNS_QUERY_REQUEST = *mut DNS_QUERY_REQUEST;
#[cfg(all(feature = "Win32_in6addr", feature = "Win32_inaddr", feature = "Win32_minwindef", feature = "Win32_windnsdef", feature = "Win32_ws2def", feature = "Win32_ws2ipdef"))]
pub type PDNS_QUERY_REQUEST3 = *mut DNS_QUERY_REQUEST3;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
pub type PDNS_QUERY_RESULT = *mut DNS_QUERY_RESULT;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
pub type PDNS_RRSET = *mut DNS_RRSET;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
pub type PDNS_SERVICE_BROWSE_CALLBACK = Option<unsafe extern "system" fn(status: u32, pquerycontext: *const core::ffi::c_void, pdnsrecord: *const super::windnsdef::DNS_RECORDA)>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
pub type PDNS_SERVICE_BROWSE_REQUEST = *mut DNS_SERVICE_BROWSE_REQUEST;
pub type PDNS_SERVICE_CANCEL = *mut DNS_SERVICE_CANCEL;
#[cfg(feature = "Win32_windnsdef")]
pub type PDNS_SERVICE_INSTANCE = *mut DNS_SERVICE_INSTANCE;
#[cfg(feature = "Win32_windnsdef")]
pub type PDNS_SERVICE_REGISTER_COMPLETE = Option<unsafe extern "system" fn(status: u32, pquerycontext: *const core::ffi::c_void, pinstance: *const DNS_SERVICE_INSTANCE)>;
#[cfg(all(feature = "Win32_windnsdef", feature = "Win32_winnt"))]
pub type PDNS_SERVICE_REGISTER_REQUEST = *mut DNS_SERVICE_REGISTER_REQUEST;
#[cfg(feature = "Win32_windnsdef")]
pub type PDNS_SERVICE_RESOLVE_COMPLETE = Option<unsafe extern "system" fn(status: u32, pquerycontext: *const core::ffi::c_void, pinstance: *const DNS_SERVICE_INSTANCE)>;
#[cfg(feature = "Win32_windnsdef")]
pub type PDNS_SERVICE_RESOLVE_REQUEST = *mut DNS_SERVICE_RESOLVE_REQUEST;
pub type PDNS_WIRE_QUESTION = *mut DNS_WIRE_QUESTION;
pub type PDNS_WIRE_RECORD = *mut DNS_WIRE_RECORD;
#[cfg(feature = "Win32_windnsdef")]
pub type PIP4_ARRAY = *mut IP4_ARRAY;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
pub type PMDNS_QUERY_CALLBACK = Option<unsafe extern "system" fn(pquerycontext: *const core::ffi::c_void, pqueryhandle: *mut MDNS_QUERY_HANDLE, pqueryresults: *mut DNS_QUERY_RESULT)>;
pub type PMDNS_QUERY_HANDLE = *mut MDNS_QUERY_HANDLE;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windnsdef"))]
pub type PMDNS_QUERY_REQUEST = *mut MDNS_QUERY_REQUEST;
pub const TAG_DNS_CONNECTION_POLICY_TAG_CONNECTION_MANAGER: DNS_CONNECTION_POLICY_TAG = 1;
pub const TAG_DNS_CONNECTION_POLICY_TAG_DEFAULT: DNS_CONNECTION_POLICY_TAG = 0;
pub const TAG_DNS_CONNECTION_POLICY_TAG_WWWPT: DNS_CONNECTION_POLICY_TAG = 2;
