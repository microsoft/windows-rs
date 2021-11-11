#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsAcquireContextHandle_A(credentialflags: u32, credentials: *const ::core::ffi::c_void, pcontext: *mut DnsContextHandle) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsAcquireContextHandle_W(credentialflags: u32, credentials: *const ::core::ffi::c_void, pcontext: *mut DnsContextHandle) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsCancelQuery(pcancelhandle: *const DNS_QUERY_CANCEL) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsConnectionDeletePolicyEntries(policyentrytag: DNS_CONNECTION_POLICY_TAG) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionDeleteProxyInfo(pwszconnectionname: super::super::Foundation::PWSTR, r#type: DNS_CONNECTION_PROXY_TYPE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsConnectionFreeNameList(pnamelist: *mut DNS_CONNECTION_NAME_LIST);
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionFreeProxyInfo(pproxyinfo: *mut DNS_CONNECTION_PROXY_INFO);
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionFreeProxyInfoEx(pproxyinfoex: *mut DNS_CONNECTION_PROXY_INFO_EX);
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionFreeProxyList(pproxylist: *mut DNS_CONNECTION_PROXY_LIST);
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsConnectionGetNameList(pnamelist: *mut DNS_CONNECTION_NAME_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionGetProxyInfo(pwszconnectionname: super::super::Foundation::PWSTR, r#type: DNS_CONNECTION_PROXY_TYPE, pproxyinfo: *mut DNS_CONNECTION_PROXY_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionGetProxyInfoForHostUrl(pwszhosturl: super::super::Foundation::PWSTR, pselectioncontext: *const u8, dwselectioncontextlength: u32, dwexplicitinterfaceindex: u32, pproxyinfoex: *mut DNS_CONNECTION_PROXY_INFO_EX) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionGetProxyList(pwszconnectionname: super::super::Foundation::PWSTR, pproxylist: *mut DNS_CONNECTION_PROXY_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionSetPolicyEntries(policyentrytag: DNS_CONNECTION_POLICY_TAG, ppolicyentrylist: *const DNS_CONNECTION_POLICY_ENTRY_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionSetProxyInfo(pwszconnectionname: super::super::Foundation::PWSTR, r#type: DNS_CONNECTION_PROXY_TYPE, pproxyinfo: *const DNS_CONNECTION_PROXY_INFO) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionUpdateIfIndexTable(pconnectionifindexentries: *const DNS_CONNECTION_IFINDEX_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsExtractRecordsFromMessage_UTF8(pdnsbuffer: *const DNS_MESSAGE_BUFFER, wmessagelength: u16, pprecord: *mut *mut DNS_RECORDA) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsExtractRecordsFromMessage_W(pdnsbuffer: *const DNS_MESSAGE_BUFFER, wmessagelength: u16, pprecord: *mut *mut DNS_RECORDA) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsFree(pdata: *const ::core::ffi::c_void, freetype: DNS_FREE_TYPE);
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsFreeCustomServers(pcservers: *mut u32, ppservers: *mut *mut DNS_CUSTOM_SERVER);
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsFreeProxyName(proxyname: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsGetApplicationSettings(pcservers: *mut u32, ppdefaultservers: *mut *mut DNS_CUSTOM_SERVER, psettings: *mut DNS_APPLICATION_SETTINGS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsGetProxyInformation(hostname: super::super::Foundation::PWSTR, proxyinformation: *mut DNS_PROXY_INFORMATION, defaultproxyinformation: *mut DNS_PROXY_INFORMATION, completionroutine: ::windows::runtime::RawPtr, completioncontext: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsModifyRecordsInSet_A(paddrecords: *const DNS_RECORDA, pdeleterecords: *const DNS_RECORDA, options: u32, hcredentials: super::super::Foundation::HANDLE, pextralist: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsModifyRecordsInSet_UTF8(paddrecords: *const DNS_RECORDA, pdeleterecords: *const DNS_RECORDA, options: u32, hcredentials: super::super::Foundation::HANDLE, pextralist: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsModifyRecordsInSet_W(paddrecords: *const DNS_RECORDA, pdeleterecords: *const DNS_RECORDA, options: u32, hcredentials: super::super::Foundation::HANDLE, pextralist: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsNameCompare_A(pname1: super::super::Foundation::PSTR, pname2: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsNameCompare_W(pname1: super::super::Foundation::PWSTR, pname2: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQueryConfig(config: DNS_CONFIG_TYPE, flag: u32, pwsadaptername: super::super::Foundation::PWSTR, preserved: *const ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void, pbuflen: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQueryEx(pqueryrequest: *const ::core::mem::ManuallyDrop<DNS_QUERY_REQUEST>, pqueryresults: *mut DNS_QUERY_RESULT, pcancelhandle: *mut DNS_QUERY_CANCEL) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQuery_A(pszname: super::super::Foundation::PSTR, wtype: u16, options: u32, pextra: *mut ::core::ffi::c_void, ppqueryresults: *mut *mut DNS_RECORDA, preserved: *mut *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQuery_UTF8(pszname: super::super::Foundation::PSTR, wtype: u16, options: u32, pextra: *mut ::core::ffi::c_void, ppqueryresults: *mut *mut DNS_RECORDA, preserved: *mut *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQuery_W(pszname: super::super::Foundation::PWSTR, wtype: u16, options: u32, pextra: *mut ::core::ffi::c_void, ppqueryresults: *mut *mut DNS_RECORDA, preserved: *mut *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordCompare(precord1: *const DNS_RECORDA, precord2: *const DNS_RECORDA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordCopyEx(precord: *const DNS_RECORDA, charsetin: DNS_CHARSET, charsetout: DNS_CHARSET) -> *mut DNS_RECORDA;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordSetCompare(prr1: *mut DNS_RECORDA, prr2: *mut DNS_RECORDA, ppdiff1: *mut *mut DNS_RECORDA, ppdiff2: *mut *mut DNS_RECORDA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordSetCopyEx(precordset: *const DNS_RECORDA, charsetin: DNS_CHARSET, charsetout: DNS_CHARSET) -> *mut DNS_RECORDA;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordSetDetach(precordlist: *mut DNS_RECORDA) -> *mut DNS_RECORDA;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsReleaseContextHandle(hcontext: super::super::Foundation::HANDLE);
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsReplaceRecordSetA(preplaceset: *const DNS_RECORDA, options: u32, hcontext: super::super::Foundation::HANDLE, pextrainfo: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsReplaceRecordSetUTF8(preplaceset: *const DNS_RECORDA, options: u32, hcontext: super::super::Foundation::HANDLE, pextrainfo: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsReplaceRecordSetW(preplaceset: *const DNS_RECORDA, options: u32, hcontext: super::super::Foundation::HANDLE, pextrainfo: *mut ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceBrowse(prequest: *const ::core::mem::ManuallyDrop<DNS_SERVICE_BROWSE_REQUEST>, pcancel: *mut DNS_SERVICE_CANCEL) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsServiceBrowseCancel(pcancelhandle: *const DNS_SERVICE_CANCEL) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceConstructInstance(pservicename: super::super::Foundation::PWSTR, phostname: super::super::Foundation::PWSTR, pip4: *const u32, pip6: *const IP6_ADDRESS, wport: u16, wpriority: u16, wweight: u16, dwpropertiescount: u32, keys: *const super::super::Foundation::PWSTR, values: *const super::super::Foundation::PWSTR) -> *mut DNS_SERVICE_INSTANCE;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceCopyInstance(porig: *const DNS_SERVICE_INSTANCE) -> *mut DNS_SERVICE_INSTANCE;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceDeRegister(prequest: *const ::core::mem::ManuallyDrop<DNS_SERVICE_REGISTER_REQUEST>, pcancel: *mut DNS_SERVICE_CANCEL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceFreeInstance(pinstance: *const DNS_SERVICE_INSTANCE);
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceRegister(prequest: *const ::core::mem::ManuallyDrop<DNS_SERVICE_REGISTER_REQUEST>, pcancel: *mut DNS_SERVICE_CANCEL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsServiceRegisterCancel(pcancelhandle: *const DNS_SERVICE_CANCEL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceResolve(prequest: *const ::core::mem::ManuallyDrop<DNS_SERVICE_RESOLVE_REQUEST>, pcancel: *mut DNS_SERVICE_CANCEL) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsServiceResolveCancel(pcancelhandle: *const DNS_SERVICE_CANCEL) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsSetApplicationSettings(cservers: u32, pservers: *const DNS_CUSTOM_SERVER, psettings: *const DNS_APPLICATION_SETTINGS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsStartMulticastQuery(pqueryrequest: *const ::core::mem::ManuallyDrop<MDNS_QUERY_REQUEST>, phandle: *mut MDNS_QUERY_HANDLE) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsStopMulticastQuery(phandle: *mut MDNS_QUERY_HANDLE) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsValidateName_A(pszname: super::super::Foundation::PSTR, format: DNS_NAME_FORMAT) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsValidateName_UTF8(pszname: super::super::Foundation::PSTR, format: DNS_NAME_FORMAT) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsValidateName_W(pszname: super::super::Foundation::PWSTR, format: DNS_NAME_FORMAT) -> i32;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsWriteQuestionToBuffer_UTF8(pdnsbuffer: *mut DNS_MESSAGE_BUFFER, pdwbuffersize: *mut u32, pszname: super::super::Foundation::PSTR, wtype: u16, xid: u16, frecursiondesired: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsWriteQuestionToBuffer_W(pdnsbuffer: *mut DNS_MESSAGE_BUFFER, pdwbuffersize: *mut u32, pszname: super::super::Foundation::PWSTR, wtype: u16, xid: u16, frecursiondesired: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
}
