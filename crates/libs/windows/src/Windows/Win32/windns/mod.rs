#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DnsAcquireContextHandle_A(credentialflags: u32, credentials: Option<*const core::ffi::c_void>, pcontext: *mut super::HANDLE) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsAcquireContextHandle_A(credentialflags : u32, credentials : *const core::ffi::c_void, pcontext : *mut super::HANDLE) -> DNS_STATUS);
    unsafe { DnsAcquireContextHandle_A(credentialflags, credentials.unwrap_or(core::mem::zeroed()) as _, pcontext as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DnsAcquireContextHandle_W(credentialflags: u32, credentials: Option<*const core::ffi::c_void>, pcontext: *mut super::HANDLE) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsAcquireContextHandle_W(credentialflags : u32, credentials : *const core::ffi::c_void, pcontext : *mut super::HANDLE) -> DNS_STATUS);
    unsafe { DnsAcquireContextHandle_W(credentialflags, credentials.unwrap_or(core::mem::zeroed()) as _, pcontext as _) }
}
#[inline]
pub unsafe fn DnsCancelQuery(pcancelhandle: *const DNS_QUERY_CANCEL) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsCancelQuery(pcancelhandle : *const DNS_QUERY_CANCEL) -> DNS_STATUS);
    unsafe { DnsCancelQuery(pcancelhandle) }
}
#[inline]
pub unsafe fn DnsCancelQueryRaw(cancelhandle: *const DNS_QUERY_RAW_CANCEL) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsCancelQueryRaw(cancelhandle : *const DNS_QUERY_RAW_CANCEL) -> DNS_STATUS);
    unsafe { DnsCancelQueryRaw(cancelhandle) }
}
#[inline]
pub unsafe fn DnsConnectionDeletePolicyEntries(policyentrytag: DNS_CONNECTION_POLICY_TAG) -> u32 {
    windows_core::link!("dnsapi.dll" "C" fn DnsConnectionDeletePolicyEntries(policyentrytag : DNS_CONNECTION_POLICY_TAG) -> u32);
    unsafe { DnsConnectionDeletePolicyEntries(policyentrytag) }
}
#[inline]
pub unsafe fn DnsConnectionDeleteProxyInfo<P0>(pwszconnectionname: P0, r#type: DNS_CONNECTION_PROXY_TYPE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dnsapi.dll" "C" fn DnsConnectionDeleteProxyInfo(pwszconnectionname : windows_core::PCWSTR, r#type : DNS_CONNECTION_PROXY_TYPE) -> u32);
    unsafe { DnsConnectionDeleteProxyInfo(pwszconnectionname.param().abi(), r#type) }
}
#[inline]
pub unsafe fn DnsConnectionFreeNameList(pnamelist: *mut DNS_CONNECTION_NAME_LIST) {
    windows_core::link!("dnsapi.dll" "C" fn DnsConnectionFreeNameList(pnamelist : *mut DNS_CONNECTION_NAME_LIST));
    unsafe { DnsConnectionFreeNameList(pnamelist as _) }
}
#[inline]
pub unsafe fn DnsConnectionFreeProxyInfo(pproxyinfo: *mut DNS_CONNECTION_PROXY_INFO) {
    windows_core::link!("dnsapi.dll" "C" fn DnsConnectionFreeProxyInfo(pproxyinfo : *mut DNS_CONNECTION_PROXY_INFO));
    unsafe { DnsConnectionFreeProxyInfo(pproxyinfo as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DnsConnectionFreeProxyInfoEx(pproxyinfoex: *mut DNS_CONNECTION_PROXY_INFO_EX) {
    windows_core::link!("dnsapi.dll" "C" fn DnsConnectionFreeProxyInfoEx(pproxyinfoex : *mut DNS_CONNECTION_PROXY_INFO_EX));
    unsafe { DnsConnectionFreeProxyInfoEx(pproxyinfoex as _) }
}
#[inline]
pub unsafe fn DnsConnectionFreeProxyList(pproxylist: *mut DNS_CONNECTION_PROXY_LIST) {
    windows_core::link!("dnsapi.dll" "C" fn DnsConnectionFreeProxyList(pproxylist : *mut DNS_CONNECTION_PROXY_LIST));
    unsafe { DnsConnectionFreeProxyList(pproxylist as _) }
}
#[inline]
pub unsafe fn DnsConnectionGetNameList(pnamelist: *mut DNS_CONNECTION_NAME_LIST) -> u32 {
    windows_core::link!("dnsapi.dll" "C" fn DnsConnectionGetNameList(pnamelist : *mut DNS_CONNECTION_NAME_LIST) -> u32);
    unsafe { DnsConnectionGetNameList(pnamelist as _) }
}
#[inline]
pub unsafe fn DnsConnectionGetProxyInfo<P0>(pwszconnectionname: P0, r#type: DNS_CONNECTION_PROXY_TYPE, pproxyinfo: *mut DNS_CONNECTION_PROXY_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dnsapi.dll" "C" fn DnsConnectionGetProxyInfo(pwszconnectionname : windows_core::PCWSTR, r#type : DNS_CONNECTION_PROXY_TYPE, pproxyinfo : *mut DNS_CONNECTION_PROXY_INFO) -> u32);
    unsafe { DnsConnectionGetProxyInfo(pwszconnectionname.param().abi(), r#type, pproxyinfo as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DnsConnectionGetProxyInfoForHostUrl<P0>(pwszhosturl: P0, pselectioncontext: Option<&[u8]>, dwexplicitinterfaceindex: u32, pproxyinfoex: *mut DNS_CONNECTION_PROXY_INFO_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dnsapi.dll" "C" fn DnsConnectionGetProxyInfoForHostUrl(pwszhosturl : windows_core::PCWSTR, pselectioncontext : *const u8, dwselectioncontextlength : u32, dwexplicitinterfaceindex : u32, pproxyinfoex : *mut DNS_CONNECTION_PROXY_INFO_EX) -> u32);
    unsafe { DnsConnectionGetProxyInfoForHostUrl(pwszhosturl.param().abi(), pselectioncontext.map_or(core::ptr::null(), |slice| slice.as_ptr()), pselectioncontext.map_or(0, |slice| slice.len().try_into().unwrap()), dwexplicitinterfaceindex, pproxyinfoex as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DnsConnectionGetProxyInfoForHostUrlEx<P0, P4>(pwszhosturl: P0, pselectioncontext: Option<&[u8]>, dwexplicitinterfaceindex: u32, pwszconnectionname: P4, pproxyinfoex: *mut DNS_CONNECTION_PROXY_INFO_EX) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dnsapi.dll" "C" fn DnsConnectionGetProxyInfoForHostUrlEx(pwszhosturl : windows_core::PCWSTR, pselectioncontext : *const u8, dwselectioncontextlength : u32, dwexplicitinterfaceindex : u32, pwszconnectionname : windows_core::PCWSTR, pproxyinfoex : *mut DNS_CONNECTION_PROXY_INFO_EX) -> u32);
    unsafe { DnsConnectionGetProxyInfoForHostUrlEx(pwszhosturl.param().abi(), pselectioncontext.map_or(core::ptr::null(), |slice| slice.as_ptr()), pselectioncontext.map_or(0, |slice| slice.len().try_into().unwrap()), dwexplicitinterfaceindex, pwszconnectionname.param().abi(), pproxyinfoex as _) }
}
#[inline]
pub unsafe fn DnsConnectionGetProxyList<P0>(pwszconnectionname: P0, pproxylist: *mut DNS_CONNECTION_PROXY_LIST) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dnsapi.dll" "C" fn DnsConnectionGetProxyList(pwszconnectionname : windows_core::PCWSTR, pproxylist : *mut DNS_CONNECTION_PROXY_LIST) -> u32);
    unsafe { DnsConnectionGetProxyList(pwszconnectionname.param().abi(), pproxylist as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsConnectionSetPolicyEntries(policyentrytag: DNS_CONNECTION_POLICY_TAG, ppolicyentrylist: *const DNS_CONNECTION_POLICY_ENTRY_LIST) -> u32 {
    windows_core::link!("dnsapi.dll" "C" fn DnsConnectionSetPolicyEntries(policyentrytag : DNS_CONNECTION_POLICY_TAG, ppolicyentrylist : *const DNS_CONNECTION_POLICY_ENTRY_LIST) -> u32);
    unsafe { DnsConnectionSetPolicyEntries(policyentrytag, ppolicyentrylist) }
}
#[inline]
pub unsafe fn DnsConnectionSetProxyInfo<P0>(pwszconnectionname: P0, r#type: DNS_CONNECTION_PROXY_TYPE, pproxyinfo: *const DNS_CONNECTION_PROXY_INFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dnsapi.dll" "C" fn DnsConnectionSetProxyInfo(pwszconnectionname : windows_core::PCWSTR, r#type : DNS_CONNECTION_PROXY_TYPE, pproxyinfo : *const DNS_CONNECTION_PROXY_INFO) -> u32);
    unsafe { DnsConnectionSetProxyInfo(pwszconnectionname.param().abi(), r#type, pproxyinfo) }
}
#[inline]
pub unsafe fn DnsConnectionUpdateIfIndexTable(pconnectionifindexentries: *const DNS_CONNECTION_IFINDEX_LIST) -> u32 {
    windows_core::link!("dnsapi.dll" "C" fn DnsConnectionUpdateIfIndexTable(pconnectionifindexentries : *const DNS_CONNECTION_IFINDEX_LIST) -> u32);
    unsafe { DnsConnectionUpdateIfIndexTable(pconnectionifindexentries) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsExtractRecordsFromMessage_UTF8(pdnsbuffer: *const DNS_MESSAGE_BUFFER, wmessagelength: u16, pprecord: *mut PDNS_RECORD) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsExtractRecordsFromMessage_UTF8(pdnsbuffer : *const DNS_MESSAGE_BUFFER, wmessagelength : u16, pprecord : *mut PDNS_RECORD) -> DNS_STATUS);
    unsafe { DnsExtractRecordsFromMessage_UTF8(pdnsbuffer, wmessagelength, pprecord as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsExtractRecordsFromMessage_W(pdnsbuffer: *const DNS_MESSAGE_BUFFER, wmessagelength: u16, pprecord: *mut PDNS_RECORD) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsExtractRecordsFromMessage_W(pdnsbuffer : *const DNS_MESSAGE_BUFFER, wmessagelength : u16, pprecord : *mut PDNS_RECORD) -> DNS_STATUS);
    unsafe { DnsExtractRecordsFromMessage_W(pdnsbuffer, wmessagelength, pprecord as _) }
}
#[inline]
pub unsafe fn DnsFree(pdata: *mut core::ffi::c_void, freetype: DNS_FREE_TYPE) {
    windows_core::link!("dnsapi.dll" "system" fn DnsFree(pdata : *mut core::ffi::c_void, freetype : DNS_FREE_TYPE));
    unsafe { DnsFree(pdata as _, freetype) }
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[inline]
pub unsafe fn DnsFreeCustomServers(pcservers: *mut u32, ppservers: *mut *mut DNS_CUSTOM_SERVER) {
    windows_core::link!("dnsapi.dll" "C" fn DnsFreeCustomServers(pcservers : *mut u32, ppservers : *mut *mut DNS_CUSTOM_SERVER));
    unsafe { DnsFreeCustomServers(pcservers as _, ppservers as _) }
}
#[inline]
pub unsafe fn DnsFreeProxyName<P0>(proxyname: P0)
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dnsapi.dll" "system" fn DnsFreeProxyName(proxyname : windows_core::PCWSTR));
    unsafe { DnsFreeProxyName(proxyname.param().abi()) }
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[inline]
pub unsafe fn DnsGetApplicationSettings(pcservers: *mut u32, ppdefaultservers: *mut *mut DNS_CUSTOM_SERVER, psettings: Option<*mut DNS_APPLICATION_SETTINGS>) -> u32 {
    windows_core::link!("dnsapi.dll" "C" fn DnsGetApplicationSettings(pcservers : *mut u32, ppdefaultservers : *mut *mut DNS_CUSTOM_SERVER, psettings : *mut DNS_APPLICATION_SETTINGS) -> u32);
    unsafe { DnsGetApplicationSettings(pcservers as _, ppdefaultservers as _, psettings.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DnsGetProxyInformation<P0>(hostname: P0, proxyinformation: *mut DNS_PROXY_INFORMATION, defaultproxyinformation: Option<*mut DNS_PROXY_INFORMATION>, completionroutine: DNS_PROXY_COMPLETION_ROUTINE, completioncontext: Option<*const core::ffi::c_void>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dnsapi.dll" "system" fn DnsGetProxyInformation(hostname : windows_core::PCWSTR, proxyinformation : *mut DNS_PROXY_INFORMATION, defaultproxyinformation : *mut DNS_PROXY_INFORMATION, completionroutine : DNS_PROXY_COMPLETION_ROUTINE, completioncontext : *const core::ffi::c_void) -> u32);
    unsafe { DnsGetProxyInformation(hostname.param().abi(), proxyinformation as _, defaultproxyinformation.unwrap_or(core::mem::zeroed()) as _, completionroutine, completioncontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsIsFlatRecord(precord: *const DNS_RECORDA, ullflags: u64, pfflat: *mut windows_core::BOOL) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsIsFlatRecord(precord : *const DNS_RECORDA, ullflags : u64, pfflat : *mut windows_core::BOOL) -> DNS_STATUS);
    unsafe { DnsIsFlatRecord(precord, ullflags, pfflat as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn DnsModifyRecordsInSet_A(paddrecords: Option<*const DNS_RECORDA>, pdeleterecords: Option<*const DNS_RECORDA>, options: u32, hcredentials: Option<super::HANDLE>, pextralist: Option<*mut core::ffi::c_void>, preserved: Option<*mut core::ffi::c_void>) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsModifyRecordsInSet_A(paddrecords : *const DNS_RECORDA, pdeleterecords : *const DNS_RECORDA, options : u32, hcredentials : super::HANDLE, pextralist : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> DNS_STATUS);
    unsafe { DnsModifyRecordsInSet_A(paddrecords.unwrap_or(core::mem::zeroed()) as _, pdeleterecords.unwrap_or(core::mem::zeroed()) as _, options, hcredentials.unwrap_or(core::mem::zeroed()) as _, pextralist.unwrap_or(core::mem::zeroed()) as _, preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn DnsModifyRecordsInSet_UTF8(paddrecords: Option<*const DNS_RECORDA>, pdeleterecords: Option<*const DNS_RECORDA>, options: u32, hcredentials: Option<super::HANDLE>, pextralist: Option<*mut core::ffi::c_void>, preserved: Option<*mut core::ffi::c_void>) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsModifyRecordsInSet_UTF8(paddrecords : *const DNS_RECORDA, pdeleterecords : *const DNS_RECORDA, options : u32, hcredentials : super::HANDLE, pextralist : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> DNS_STATUS);
    unsafe { DnsModifyRecordsInSet_UTF8(paddrecords.unwrap_or(core::mem::zeroed()) as _, pdeleterecords.unwrap_or(core::mem::zeroed()) as _, options, hcredentials.unwrap_or(core::mem::zeroed()) as _, pextralist.unwrap_or(core::mem::zeroed()) as _, preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn DnsModifyRecordsInSet_W(paddrecords: Option<*const DNS_RECORDA>, pdeleterecords: Option<*const DNS_RECORDA>, options: u32, hcredentials: Option<super::HANDLE>, pextralist: Option<*mut core::ffi::c_void>, preserved: Option<*mut core::ffi::c_void>) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsModifyRecordsInSet_W(paddrecords : *const DNS_RECORDA, pdeleterecords : *const DNS_RECORDA, options : u32, hcredentials : super::HANDLE, pextralist : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> DNS_STATUS);
    unsafe { DnsModifyRecordsInSet_W(paddrecords.unwrap_or(core::mem::zeroed()) as _, pdeleterecords.unwrap_or(core::mem::zeroed()) as _, options, hcredentials.unwrap_or(core::mem::zeroed()) as _, pextralist.unwrap_or(core::mem::zeroed()) as _, preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DnsNameCompare_A<P0, P1>(pname1: P0, pname2: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dnsapi.dll" "system" fn DnsNameCompare_A(pname1 : windows_core::PCSTR, pname2 : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DnsNameCompare_A(pname1.param().abi(), pname2.param().abi()) }
}
#[inline]
pub unsafe fn DnsNameCompare_W<P0, P1>(pname1: P0, pname2: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dnsapi.dll" "system" fn DnsNameCompare_W(pname1 : windows_core::PCWSTR, pname2 : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DnsNameCompare_W(pname1.param().abi(), pname2.param().abi()) }
}
#[inline]
pub unsafe fn DnsQueryConfig<P2>(config: DNS_CONFIG_TYPE, flag: u32, pwsadaptername: P2, preserved: Option<*const core::ffi::c_void>, pbuffer: Option<*mut core::ffi::c_void>, pbuflen: *mut u32) -> DNS_STATUS
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dnsapi.dll" "system" fn DnsQueryConfig(config : DNS_CONFIG_TYPE, flag : u32, pwsadaptername : windows_core::PCWSTR, preserved : *const core::ffi::c_void, pbuffer : *mut core::ffi::c_void, pbuflen : *mut u32) -> DNS_STATUS);
    unsafe { DnsQueryConfig(config, flag, pwsadaptername.param().abi(), preserved.unwrap_or(core::mem::zeroed()) as _, pbuffer.unwrap_or(core::mem::zeroed()) as _, pbuflen as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsQueryEx(pqueryrequest: *const DNS_QUERY_REQUEST, pqueryresults: *mut DNS_QUERY_RESULT, pcancelhandle: Option<*mut DNS_QUERY_CANCEL>) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsQueryEx(pqueryrequest : *const DNS_QUERY_REQUEST, pqueryresults : *mut DNS_QUERY_RESULT, pcancelhandle : *mut DNS_QUERY_CANCEL) -> DNS_STATUS);
    unsafe { DnsQueryEx(pqueryrequest, pqueryresults as _, pcancelhandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "minwindef", feature = "ws2"))]
#[inline]
pub unsafe fn DnsQueryRaw(queryrequest: *const DNS_QUERY_RAW_REQUEST, cancelhandle: *mut DNS_QUERY_RAW_CANCEL) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsQueryRaw(queryrequest : *const DNS_QUERY_RAW_REQUEST, cancelhandle : *mut DNS_QUERY_RAW_CANCEL) -> DNS_STATUS);
    unsafe { DnsQueryRaw(queryrequest, cancelhandle as _) }
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "minwindef", feature = "ws2"))]
#[inline]
pub unsafe fn DnsQueryRawResultFree(queryresults: *mut DNS_QUERY_RAW_RESULT) {
    windows_core::link!("dnsapi.dll" "system" fn DnsQueryRawResultFree(queryresults : *mut DNS_QUERY_RAW_RESULT));
    unsafe { DnsQueryRawResultFree(queryresults as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsQuery_A<P0>(pszname: P0, wtype: u16, options: u32, pextra: Option<*mut core::ffi::c_void>, ppqueryresults: *mut PDNS_RECORD, preserved: *mut *mut core::ffi::c_void) -> DNS_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dnsapi.dll" "system" fn DnsQuery_A(pszname : windows_core::PCSTR, wtype : u16, options : u32, pextra : *mut core::ffi::c_void, ppqueryresults : *mut PDNS_RECORD, preserved : *mut *mut core::ffi::c_void) -> DNS_STATUS);
    unsafe { DnsQuery_A(pszname.param().abi(), wtype, options, pextra.unwrap_or(core::mem::zeroed()) as _, ppqueryresults as _, preserved as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsQuery_UTF8<P0>(pszname: P0, wtype: u16, options: u32, pextra: Option<*mut core::ffi::c_void>, ppqueryresults: *mut PDNS_RECORD, preserved: *mut *mut core::ffi::c_void) -> DNS_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dnsapi.dll" "system" fn DnsQuery_UTF8(pszname : windows_core::PCSTR, wtype : u16, options : u32, pextra : *mut core::ffi::c_void, ppqueryresults : *mut PDNS_RECORD, preserved : *mut *mut core::ffi::c_void) -> DNS_STATUS);
    unsafe { DnsQuery_UTF8(pszname.param().abi(), wtype, options, pextra.unwrap_or(core::mem::zeroed()) as _, ppqueryresults as _, preserved as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsQuery_W<P0>(pszname: P0, wtype: u16, options: u32, pextra: Option<*mut core::ffi::c_void>, ppqueryresults: *mut PDNS_RECORD, preserved: *mut *mut core::ffi::c_void) -> DNS_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dnsapi.dll" "system" fn DnsQuery_W(pszname : windows_core::PCWSTR, wtype : u16, options : u32, pextra : *mut core::ffi::c_void, ppqueryresults : *mut PDNS_RECORD, preserved : *mut *mut core::ffi::c_void) -> DNS_STATUS);
    unsafe { DnsQuery_W(pszname.param().abi(), wtype, options, pextra.unwrap_or(core::mem::zeroed()) as _, ppqueryresults as _, preserved as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsRecordCompare(precord1: *const DNS_RECORDA, precord2: *const DNS_RECORDA) -> windows_core::BOOL {
    windows_core::link!("dnsapi.dll" "system" fn DnsRecordCompare(precord1 : *const DNS_RECORDA, precord2 : *const DNS_RECORDA) -> windows_core::BOOL);
    unsafe { DnsRecordCompare(precord1, precord2) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsRecordCopyEx(precord: *const DNS_RECORDA, charsetin: DNS_CHARSET, charsetout: DNS_CHARSET) -> PDNS_RECORD {
    windows_core::link!("dnsapi.dll" "system" fn DnsRecordCopyEx(precord : *const DNS_RECORDA, charsetin : DNS_CHARSET, charsetout : DNS_CHARSET) -> PDNS_RECORD);
    unsafe { DnsRecordCopyEx(precord, charsetin, charsetout) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsRecordSetCompare(prr1: *mut DNS_RECORDA, prr2: *mut DNS_RECORDA, ppdiff1: *mut PDNS_RECORD, ppdiff2: *mut PDNS_RECORD) -> windows_core::BOOL {
    windows_core::link!("dnsapi.dll" "system" fn DnsRecordSetCompare(prr1 : *mut DNS_RECORDA, prr2 : *mut DNS_RECORDA, ppdiff1 : *mut PDNS_RECORD, ppdiff2 : *mut PDNS_RECORD) -> windows_core::BOOL);
    unsafe { DnsRecordSetCompare(prr1 as _, prr2 as _, ppdiff1 as _, ppdiff2 as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsRecordSetCopyEx(precordset: *const DNS_RECORDA, charsetin: DNS_CHARSET, charsetout: DNS_CHARSET) -> PDNS_RECORD {
    windows_core::link!("dnsapi.dll" "system" fn DnsRecordSetCopyEx(precordset : *const DNS_RECORDA, charsetin : DNS_CHARSET, charsetout : DNS_CHARSET) -> PDNS_RECORD);
    unsafe { DnsRecordSetCopyEx(precordset, charsetin, charsetout) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsRecordSetDetach(precordlist: *mut DNS_RECORDA) -> PDNS_RECORD {
    windows_core::link!("dnsapi.dll" "C" fn DnsRecordSetDetach(precordlist : *mut DNS_RECORDA) -> PDNS_RECORD);
    unsafe { DnsRecordSetDetach(precordlist as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DnsReleaseContextHandle(hcontext: super::HANDLE) {
    windows_core::link!("dnsapi.dll" "system" fn DnsReleaseContextHandle(hcontext : super::HANDLE));
    unsafe { DnsReleaseContextHandle(hcontext) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn DnsReplaceRecordSetA(preplaceset: *const DNS_RECORDA, options: u32, hcontext: Option<super::HANDLE>, pextrainfo: Option<*mut core::ffi::c_void>, preserved: Option<*mut core::ffi::c_void>) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsReplaceRecordSetA(preplaceset : *const DNS_RECORDA, options : u32, hcontext : super::HANDLE, pextrainfo : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> DNS_STATUS);
    unsafe { DnsReplaceRecordSetA(preplaceset, options, hcontext.unwrap_or(core::mem::zeroed()) as _, pextrainfo.unwrap_or(core::mem::zeroed()) as _, preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn DnsReplaceRecordSetUTF8(preplaceset: *const DNS_RECORDA, options: u32, hcontext: Option<super::HANDLE>, pextrainfo: Option<*mut core::ffi::c_void>, preserved: Option<*mut core::ffi::c_void>) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsReplaceRecordSetUTF8(preplaceset : *const DNS_RECORDA, options : u32, hcontext : super::HANDLE, pextrainfo : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> DNS_STATUS);
    unsafe { DnsReplaceRecordSetUTF8(preplaceset, options, hcontext.unwrap_or(core::mem::zeroed()) as _, pextrainfo.unwrap_or(core::mem::zeroed()) as _, preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn DnsReplaceRecordSetW(preplaceset: *const DNS_RECORDA, options: u32, hcontext: Option<super::HANDLE>, pextrainfo: Option<*mut core::ffi::c_void>, preserved: Option<*mut core::ffi::c_void>) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsReplaceRecordSetW(preplaceset : *const DNS_RECORDA, options : u32, hcontext : super::HANDLE, pextrainfo : *mut core::ffi::c_void, preserved : *mut core::ffi::c_void) -> DNS_STATUS);
    unsafe { DnsReplaceRecordSetW(preplaceset, options, hcontext.unwrap_or(core::mem::zeroed()) as _, pextrainfo.unwrap_or(core::mem::zeroed()) as _, preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsServiceBrowse(prequest: *const DNS_SERVICE_BROWSE_REQUEST, pcancel: *mut DNS_SERVICE_CANCEL) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsServiceBrowse(prequest : *const DNS_SERVICE_BROWSE_REQUEST, pcancel : *mut DNS_SERVICE_CANCEL) -> DNS_STATUS);
    unsafe { DnsServiceBrowse(prequest, pcancel as _) }
}
#[inline]
pub unsafe fn DnsServiceBrowseCancel(pcancelhandle: *const DNS_SERVICE_CANCEL) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsServiceBrowseCancel(pcancelhandle : *const DNS_SERVICE_CANCEL) -> DNS_STATUS);
    unsafe { DnsServiceBrowseCancel(pcancelhandle) }
}
#[inline]
pub unsafe fn DnsServiceConstructInstance<P0, P1>(pservicename: P0, phostname: P1, pip4: Option<*const u32>, pip6: Option<*const IP6_ADDRESS>, wport: u16, wpriority: u16, wweight: u16, dwpropertiescount: u32, keys: *const windows_core::PCWSTR, values: *const windows_core::PCWSTR) -> PDNS_SERVICE_INSTANCE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dnsapi.dll" "system" fn DnsServiceConstructInstance(pservicename : windows_core::PCWSTR, phostname : windows_core::PCWSTR, pip4 : *const u32, pip6 : *const IP6_ADDRESS, wport : u16, wpriority : u16, wweight : u16, dwpropertiescount : u32, keys : *const windows_core::PCWSTR, values : *const windows_core::PCWSTR) -> PDNS_SERVICE_INSTANCE);
    unsafe { DnsServiceConstructInstance(pservicename.param().abi(), phostname.param().abi(), pip4.unwrap_or(core::mem::zeroed()) as _, pip6.unwrap_or(core::mem::zeroed()) as _, wport, wpriority, wweight, dwpropertiescount, keys, values) }
}
#[inline]
pub unsafe fn DnsServiceCopyInstance(porig: *const DNS_SERVICE_INSTANCE) -> PDNS_SERVICE_INSTANCE {
    windows_core::link!("dnsapi.dll" "system" fn DnsServiceCopyInstance(porig : *const DNS_SERVICE_INSTANCE) -> PDNS_SERVICE_INSTANCE);
    unsafe { DnsServiceCopyInstance(porig) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DnsServiceDeRegister(prequest: *const DNS_SERVICE_REGISTER_REQUEST, pcancel: Option<*mut DNS_SERVICE_CANCEL>) -> u32 {
    windows_core::link!("dnsapi.dll" "system" fn DnsServiceDeRegister(prequest : *const DNS_SERVICE_REGISTER_REQUEST, pcancel : *mut DNS_SERVICE_CANCEL) -> u32);
    unsafe { DnsServiceDeRegister(prequest, pcancel.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DnsServiceFreeInstance(pinstance: *const DNS_SERVICE_INSTANCE) {
    windows_core::link!("dnsapi.dll" "system" fn DnsServiceFreeInstance(pinstance : *const DNS_SERVICE_INSTANCE));
    unsafe { DnsServiceFreeInstance(pinstance) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DnsServiceRegister(prequest: *const DNS_SERVICE_REGISTER_REQUEST, pcancel: Option<*mut DNS_SERVICE_CANCEL>) -> u32 {
    windows_core::link!("dnsapi.dll" "system" fn DnsServiceRegister(prequest : *const DNS_SERVICE_REGISTER_REQUEST, pcancel : *mut DNS_SERVICE_CANCEL) -> u32);
    unsafe { DnsServiceRegister(prequest, pcancel.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DnsServiceRegisterCancel(pcancelhandle: *const DNS_SERVICE_CANCEL) -> u32 {
    windows_core::link!("dnsapi.dll" "system" fn DnsServiceRegisterCancel(pcancelhandle : *const DNS_SERVICE_CANCEL) -> u32);
    unsafe { DnsServiceRegisterCancel(pcancelhandle) }
}
#[inline]
pub unsafe fn DnsServiceResolve(prequest: *const DNS_SERVICE_RESOLVE_REQUEST, pcancel: *mut DNS_SERVICE_CANCEL) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsServiceResolve(prequest : *const DNS_SERVICE_RESOLVE_REQUEST, pcancel : *mut DNS_SERVICE_CANCEL) -> DNS_STATUS);
    unsafe { DnsServiceResolve(prequest, pcancel as _) }
}
#[inline]
pub unsafe fn DnsServiceResolveCancel(pcancelhandle: *const DNS_SERVICE_CANCEL) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsServiceResolveCancel(pcancelhandle : *const DNS_SERVICE_CANCEL) -> DNS_STATUS);
    unsafe { DnsServiceResolveCancel(pcancelhandle) }
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[inline]
pub unsafe fn DnsSetApplicationSettings(pservers: &[DNS_CUSTOM_SERVER], psettings: Option<*const DNS_APPLICATION_SETTINGS>) -> u32 {
    windows_core::link!("dnsapi.dll" "C" fn DnsSetApplicationSettings(cservers : u32, pservers : *const DNS_CUSTOM_SERVER, psettings : *const DNS_APPLICATION_SETTINGS) -> u32);
    unsafe { DnsSetApplicationSettings(pservers.len().try_into().unwrap(), pservers.as_ptr(), psettings.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsStartMulticastQuery(pqueryrequest: *const MDNS_QUERY_REQUEST, phandle: *mut MDNS_QUERY_HANDLE) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsStartMulticastQuery(pqueryrequest : *const MDNS_QUERY_REQUEST, phandle : *mut MDNS_QUERY_HANDLE) -> DNS_STATUS);
    unsafe { DnsStartMulticastQuery(pqueryrequest, phandle as _) }
}
#[inline]
pub unsafe fn DnsStopMulticastQuery(phandle: *mut MDNS_QUERY_HANDLE) -> DNS_STATUS {
    windows_core::link!("dnsapi.dll" "system" fn DnsStopMulticastQuery(phandle : *mut MDNS_QUERY_HANDLE) -> DNS_STATUS);
    unsafe { DnsStopMulticastQuery(phandle as _) }
}
#[inline]
pub unsafe fn DnsValidateName_A<P0>(pszname: P0, format: DNS_NAME_FORMAT) -> DNS_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dnsapi.dll" "C" fn DnsValidateName_A(pszname : windows_core::PCSTR, format : DNS_NAME_FORMAT) -> DNS_STATUS);
    unsafe { DnsValidateName_A(pszname.param().abi(), format) }
}
#[inline]
pub unsafe fn DnsValidateName_UTF8<P0>(pszname: P0, format: DNS_NAME_FORMAT) -> DNS_STATUS
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dnsapi.dll" "C" fn DnsValidateName_UTF8(pszname : windows_core::PCSTR, format : DNS_NAME_FORMAT) -> DNS_STATUS);
    unsafe { DnsValidateName_UTF8(pszname.param().abi(), format) }
}
#[inline]
pub unsafe fn DnsValidateName_W<P0>(pszname: P0, format: DNS_NAME_FORMAT) -> DNS_STATUS
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dnsapi.dll" "C" fn DnsValidateName_W(pszname : windows_core::PCWSTR, format : DNS_NAME_FORMAT) -> DNS_STATUS);
    unsafe { DnsValidateName_W(pszname.param().abi(), format) }
}
#[cfg(feature = "ws2")]
#[inline]
pub unsafe fn DnsValidateServerStatus<P1>(server: *const super::SOCKADDR, queryname: P1, serverstatus: *mut u32) -> DNS_STATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dnsapi.dll" "C" fn DnsValidateServerStatus(server : *const super::SOCKADDR, queryname : windows_core::PCWSTR, serverstatus : *mut u32) -> DNS_STATUS);
    unsafe { DnsValidateServerStatus(server, queryname.param().abi(), serverstatus as _) }
}
#[inline]
pub unsafe fn DnsWriteQuestionToBuffer_UTF8<P2>(pdnsbuffer: *mut DNS_MESSAGE_BUFFER, pdwbuffersize: *mut u32, pszname: P2, wtype: u16, xid: u16, frecursiondesired: bool) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("dnsapi.dll" "system" fn DnsWriteQuestionToBuffer_UTF8(pdnsbuffer : *mut DNS_MESSAGE_BUFFER, pdwbuffersize : *mut u32, pszname : windows_core::PCSTR, wtype : u16, xid : u16, frecursiondesired : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DnsWriteQuestionToBuffer_UTF8(pdnsbuffer as _, pdwbuffersize as _, pszname.param().abi(), wtype, xid, frecursiondesired.into()) }
}
#[inline]
pub unsafe fn DnsWriteQuestionToBuffer_W<P2>(pdnsbuffer: *mut DNS_MESSAGE_BUFFER, pdwbuffersize: *mut u32, pszname: P2, wtype: u16, xid: u16, frecursiondesired: bool) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("dnsapi.dll" "system" fn DnsWriteQuestionToBuffer_W(pdnsbuffer : *mut DNS_MESSAGE_BUFFER, pdwbuffersize : *mut u32, pszname : windows_core::PCWSTR, wtype : u16, xid : u16, frecursiondesired : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DnsWriteQuestionToBuffer_W(pdnsbuffer as _, pdwbuffersize as _, pszname.param().abi(), wtype, xid, frecursiondesired.into()) }
}
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
pub const DNS_ADDRESS_STRING_LENGTH: u32 = 65;
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_APPLICATION_SETTINGS {
    pub Version: u32,
    pub Flags: u64,
}
pub const DNS_APP_SETTINGS_EXCLUSIVE_SERVERS: u32 = 1;
pub const DNS_APP_SETTINGS_VERSION1: u32 = 1;
pub const DNS_ATMA_AESA_ADDR_LENGTH: u32 = 20;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_A_DATA {
    pub IpAddress: IP4_ADDRESS,
}
pub type DNS_CHARSET = i32;
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_CONNECTION_IFINDEX_ENTRY {
    pub pwszConnectionName: windows_core::PCWSTR,
    pub dwIfIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DNS_CONNECTION_NAME {
    pub wszName: [u16; 65],
}
impl Default for DNS_CONNECTION_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DNS_CONNECTION_POLICY_ENTRY {
    pub pwszHost: windows_core::PCWSTR,
    pub pwszAppId: windows_core::PCWSTR,
    pub cbAppSid: u32,
    pub pbAppSid: super::PBYTE,
    pub nConnections: u32,
    pub ppwszConnections: *mut windows_core::PCWSTR,
    pub dwPolicyEntryFlags: u32,
}
#[cfg(feature = "minwindef")]
impl Default for DNS_CONNECTION_POLICY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DNS_CONNECTION_POLICY_ENTRY_LIST {
    pub pPolicyEntries: *mut DNS_CONNECTION_POLICY_ENTRY,
    pub nEntries: u32,
}
#[cfg(feature = "minwindef")]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DNS_CONNECTION_PROXY_INFO_EX {
    pub ProxyInfo: DNS_CONNECTION_PROXY_INFO,
    pub dwInterfaceIndex: u32,
    pub pwszConnectionName: *mut u16,
    pub fDirectConfiguration: windows_core::BOOL,
    pub hConnection: super::HANDLE,
}
#[cfg(feature = "winnt")]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct DNS_CUSTOM_SERVER {
    pub dwServerType: u32,
    pub ullFlags: u64,
    pub Anonymous: DNS_CUSTOM_SERVER_0,
    pub Anonymous2: DNS_CUSTOM_SERVER_1,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "ws2"))]
impl Default for DNS_CUSTOM_SERVER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union DNS_CUSTOM_SERVER_0 {
    pub pwszTemplate: windows_core::PWSTR,
    pub pwszHostname: windows_core::PWSTR,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "ws2"))]
impl Default for DNS_CUSTOM_SERVER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union DNS_CUSTOM_SERVER_1 {
    pub ServerAddr: super::SOCKADDR_INET,
    pub MaxSa: [i8; 32],
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "ws2"))]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub type DNS_FREE_TYPE = i32;
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
impl DNS_HEADER {
    pub fn RecursionDesired(&self) -> bool {
        self._bitfield1 & 1 != 0
    }
    pub fn set_RecursionDesired(&mut self, value: bool) {
        self._bitfield1 = (self._bitfield1 & !1) | (value as u8);
    }
    pub fn Truncation(&self) -> bool {
        (self._bitfield1 >> 1) & 1 != 0
    }
    pub fn set_Truncation(&mut self, value: bool) {
        self._bitfield1 = (self._bitfield1 & !(1 << 1)) | ((value as u8) << 1);
    }
    pub fn Authoritative(&self) -> bool {
        (self._bitfield1 >> 2) & 1 != 0
    }
    pub fn set_Authoritative(&mut self, value: bool) {
        self._bitfield1 = (self._bitfield1 & !(1 << 2)) | ((value as u8) << 2);
    }
    pub fn Opcode(&self) -> u8 {
        (self._bitfield1 << 1) >> 4
    }
    pub fn set_Opcode(&mut self, value: u8) {
        self._bitfield1 = (self._bitfield1 & !(15 << 3)) | ((value & 15) << 3);
    }
    pub fn IsResponse(&self) -> bool {
        (self._bitfield1 >> 7) & 1 != 0
    }
    pub fn set_IsResponse(&mut self, value: bool) {
        self._bitfield1 = (self._bitfield1 & !(1 << 7)) | ((value as u8) << 7);
    }
    pub fn ResponseCode(&self) -> u8 {
        (self._bitfield2 << 4) >> 4
    }
    pub fn set_ResponseCode(&mut self, value: u8) {
        self._bitfield2 = (self._bitfield2 & !15) | (value & 15);
    }
    pub fn CheckingDisabled(&self) -> bool {
        (self._bitfield2 >> 4) & 1 != 0
    }
    pub fn set_CheckingDisabled(&mut self, value: bool) {
        self._bitfield2 = (self._bitfield2 & !(1 << 4)) | ((value as u8) << 4);
    }
    pub fn AuthenticatedData(&self) -> bool {
        (self._bitfield2 >> 5) & 1 != 0
    }
    pub fn set_AuthenticatedData(&mut self, value: bool) {
        self._bitfield2 = (self._bitfield2 & !(1 << 5)) | ((value as u8) << 5);
    }
    pub fn Reserved(&self) -> bool {
        (self._bitfield2 >> 6) & 1 != 0
    }
    pub fn set_Reserved(&mut self, value: bool) {
        self._bitfield2 = (self._bitfield2 & !(1 << 6)) | ((value as u8) << 6);
    }
    pub fn RecursionAvailable(&self) -> bool {
        (self._bitfield2 >> 7) & 1 != 0
    }
    pub fn set_RecursionAvailable(&mut self, value: bool) {
        self._bitfield2 = (self._bitfield2 & !(1 << 7)) | ((value as u8) << 7);
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_HEADER_EXT {
    pub _bitfield: u16,
    pub chRcode: u8,
    pub chVersion: u8,
}
impl DNS_HEADER_EXT {
    pub fn Reserved(&self) -> u16 {
        (self._bitfield << 1) >> 1
    }
    pub fn set_Reserved(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !32767) | (value & 32767);
    }
    pub fn DnssecOk(&self) -> bool {
        (self._bitfield >> 15) & 1 != 0
    }
    pub fn set_DnssecOk(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 15)) | ((value as u16) << 15);
    }
}
pub const DNS_IP4_REVERSE_DOMAIN_STRING_A: windows_core::PCSTR = windows_core::s!("in-addr.arpa.");
pub const DNS_IP4_REVERSE_DOMAIN_STRING_W: windows_core::PCWSTR = windows_core::w!("in-addr.arpa.");
pub const DNS_IP6_REVERSE_DOMAIN_STRING_A: windows_core::PCSTR = windows_core::s!("ip6.arpa.");
pub const DNS_IP6_REVERSE_DOMAIN_STRING_W: windows_core::PCWSTR = windows_core::w!("ip6.arpa.");
pub type DNS_KEY_DATA = DNS_DNSKEY_DATA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_LOC_DATA {
    pub wVersion: u16,
    pub wSize: u16,
    pub wHorPrec: u16,
    pub wVerPrec: u16,
    pub dwLatitude: u32,
    pub dwLongitude: u32,
    pub dwAltitude: u32,
}
pub const DNS_MAX_IP4_REVERSE_NAME_BUFFER_LENGTH: u32 = 31;
pub const DNS_MAX_IP4_REVERSE_NAME_LENGTH: u32 = 31;
pub const DNS_MAX_IP6_REVERSE_NAME_BUFFER_LENGTH: u32 = 75;
pub const DNS_MAX_IP6_REVERSE_NAME_LENGTH: u32 = 75;
pub const DNS_MAX_LABEL_BUFFER_LENGTH: u32 = 64;
pub const DNS_MAX_LABEL_LENGTH: u32 = 63;
pub const DNS_MAX_NAME_BUFFER_LENGTH: u32 = 256;
pub const DNS_MAX_NAME_LENGTH: u32 = 255;
pub const DNS_MAX_REVERSE_NAME_BUFFER_LENGTH: u32 = 75;
pub const DNS_MAX_REVERSE_NAME_LENGTH: u32 = 75;
pub const DNS_MAX_TEXT_STRING_LENGTH: u32 = 255;
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_MINFO_DATAA {
    pub pNameMailbox: windows_core::PSTR,
    pub pNameErrorsMailbox: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_MINFO_DATAW {
    pub pNameMailbox: windows_core::PWSTR,
    pub pNameErrorsMailbox: windows_core::PWSTR,
}
pub type DNS_MX_DATA = DNS_MX_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_MX_DATAA {
    pub pNameExchange: windows_core::PSTR,
    pub wPreference: u16,
    pub Pad: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_MX_DATAW {
    pub pNameExchange: windows_core::PWSTR,
    pub wPreference: u16,
    pub Pad: u16,
}
pub type DNS_NAME_FORMAT = i32;
pub type DNS_NAPTR_DATA = DNS_NAPTR_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_NAPTR_DATAA {
    pub wOrder: u16,
    pub wPreference: u16,
    pub pFlags: windows_core::PSTR,
    pub pService: windows_core::PSTR,
    pub pRegularExpression: windows_core::PSTR,
    pub pReplacement: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_NAPTR_DATAW {
    pub wOrder: u16,
    pub wPreference: u16,
    pub pFlags: windows_core::PWSTR,
    pub pService: windows_core::PWSTR,
    pub pRegularExpression: windows_core::PWSTR,
    pub pReplacement: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const DNS_OPCODE_IQUERY: u32 = 1;
pub const DNS_OPCODE_NOTIFY: u32 = 4;
pub const DNS_OPCODE_QUERY: u32 = 0;
pub const DNS_OPCODE_SERVER_STATUS: u32 = 2;
pub const DNS_OPCODE_UNKNOWN: u32 = 3;
pub const DNS_OPCODE_UPDATE: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const DNS_PORT_HOST_ORDER: u32 = 53;
pub const DNS_PORT_NET_ORDER: u32 = 13568;
pub const DNS_PROTOCOL_DOH: u32 = 3;
pub const DNS_PROTOCOL_DOT: u32 = 4;
pub const DNS_PROTOCOL_NO_WIRE: u32 = 5;
pub const DNS_PROTOCOL_TCP: u32 = 2;
pub const DNS_PROTOCOL_UDP: u32 = 1;
pub const DNS_PROTOCOL_UNSPECIFIED: u32 = 0;
pub type DNS_PROXY_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(completioncontext: *const core::ffi::c_void, status: DNS_STATUS)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_PROXY_INFORMATION {
    pub version: u32,
    pub proxyInformationType: DNS_PROXY_INFORMATION_TYPE,
    pub proxyName: windows_core::PWSTR,
}
pub const DNS_PROXY_INFORMATION_DEFAULT_SETTINGS: DNS_PROXY_INFORMATION_TYPE = 1;
pub const DNS_PROXY_INFORMATION_DIRECT: DNS_PROXY_INFORMATION_TYPE = 0;
pub const DNS_PROXY_INFORMATION_DOES_NOT_EXIST: DNS_PROXY_INFORMATION_TYPE = 3;
pub const DNS_PROXY_INFORMATION_PROXY_NAME: DNS_PROXY_INFORMATION_TYPE = 2;
pub type DNS_PROXY_INFORMATION_TYPE = i32;
pub type DNS_PTR_DATA = DNS_PTR_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_PTR_DATAA {
    pub pNameHost: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_PTR_DATAW {
    pub pNameHost: windows_core::PWSTR,
}
pub const DNS_QUERY_ACCEPT_TRUNCATED_RESPONSE: u32 = 1;
pub const DNS_QUERY_ADDRCONFIG: u32 = 8192;
pub const DNS_QUERY_APPEND_MULTILABEL: u32 = 8388608;
pub const DNS_QUERY_BYPASS_CACHE: u32 = 8;
pub const DNS_QUERY_CACHE_ONLY: u32 = 16;
#[repr(C, align(8))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DNS_QUERY_CANCEL {
    pub Reserved: [i8; 32],
}
impl Default for DNS_QUERY_CANCEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DNS_QUERY_RAW_CANCEL {
    pub reserved: [i8; 32],
}
impl Default for DNS_QUERY_RAW_CANCEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "minwindef", feature = "ws2"))]
pub type DNS_QUERY_RAW_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(querycontext: *const core::ffi::c_void, queryresults: *const DNS_QUERY_RAW_RESULT)>;
pub const DNS_QUERY_RAW_OPTION_BEST_EFFORT_PARSE: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "minwindef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct DNS_QUERY_RAW_REQUEST {
    pub version: u32,
    pub resultsVersion: u32,
    pub dnsQueryRawSize: u32,
    pub dnsQueryRaw: *mut u8,
    pub dnsQueryName: windows_core::PWSTR,
    pub dnsQueryType: u16,
    pub queryOptions: u64,
    pub interfaceIndex: u32,
    pub queryCompletionCallback: DNS_QUERY_RAW_COMPLETION_ROUTINE,
    pub queryContext: *mut core::ffi::c_void,
    pub queryRawOptions: u64,
    pub customServersSize: u32,
    pub customServers: *mut DNS_CUSTOM_SERVER,
    pub protocol: u32,
    pub Anonymous: DNS_QUERY_RAW_REQUEST_0,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "minwindef", feature = "ws2"))]
impl Default for DNS_QUERY_RAW_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "minwindef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union DNS_QUERY_RAW_REQUEST_0 {
    pub sourceAddr: super::SOCKADDR_INET,
    pub maxSa: [i8; 32],
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "minwindef", feature = "ws2"))]
impl Default for DNS_QUERY_RAW_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_QUERY_RAW_REQUEST_VERSION1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "minwindef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub struct DNS_QUERY_RAW_RESULT {
    pub version: u32,
    pub queryStatus: DNS_STATUS,
    pub queryOptions: u64,
    pub queryRawOptions: u64,
    pub responseFlags: u64,
    pub queryRawResponseSize: u32,
    pub queryRawResponse: *mut u8,
    pub queryRecords: PDNS_RECORD,
    pub protocol: u32,
    pub Anonymous: DNS_QUERY_RAW_RESULT_0,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "minwindef", feature = "ws2"))]
impl Default for DNS_QUERY_RAW_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "minwindef", feature = "ws2"))]
#[derive(Clone, Copy)]
pub union DNS_QUERY_RAW_RESULT_0 {
    pub sourceAddr: super::SOCKADDR_INET,
    pub maxSa: [i8; 32],
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "minwindef", feature = "ws2"))]
impl Default for DNS_QUERY_RAW_RESULT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_QUERY_RAW_RESULTS_VERSION1: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug)]
pub struct DNS_QUERY_REQUEST {
    pub Version: u32,
    pub QueryName: windows_core::PCWSTR,
    pub QueryType: u16,
    pub QueryOptions: u64,
    pub pDnsServerList: PDNS_ADDR_ARRAY,
    pub InterfaceIndex: u32,
    pub pQueryCompletionCallback: PDNS_QUERY_COMPLETION_ROUTINE,
    pub pQueryContext: *mut core::ffi::c_void,
}
#[cfg(feature = "minwindef")]
impl Default for DNS_QUERY_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "minwindef", feature = "ws2"))]
#[derive(Clone, Copy, Debug)]
pub struct DNS_QUERY_REQUEST3 {
    pub Version: u32,
    pub QueryName: windows_core::PCWSTR,
    pub QueryType: u16,
    pub QueryOptions: u64,
    pub pDnsServerList: PDNS_ADDR_ARRAY,
    pub InterfaceIndex: u32,
    pub pQueryCompletionCallback: PDNS_QUERY_COMPLETION_ROUTINE,
    pub pQueryContext: *mut core::ffi::c_void,
    pub IsNetworkQueryRequired: windows_core::BOOL,
    pub RequiredNetworkIndex: u32,
    pub cCustomServers: u32,
    pub pCustomServers: *mut DNS_CUSTOM_SERVER,
}
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "minwindef", feature = "ws2"))]
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DNS_QUERY_RESULT {
    pub Version: u32,
    pub QueryStatus: DNS_STATUS,
    pub QueryOptions: u64,
    pub pQueryRecords: PDNS_RECORD,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(feature = "minwindef")]
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
#[cfg(feature = "minwindef")]
pub type DNS_RECORD = DNS_RECORDA;
#[repr(C)]
#[cfg(feature = "minwindef")]
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
#[cfg(feature = "minwindef")]
impl Default for DNS_RECORDA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union DNS_RECORDA_0 {
    pub DW: u32,
    pub S: DNS_RECORD_FLAGS,
}
#[cfg(feature = "minwindef")]
impl Default for DNS_RECORDA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
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
    pub pDataPtr: super::PBYTE,
}
#[cfg(feature = "minwindef")]
impl Default for DNS_RECORDA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
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
#[cfg(feature = "minwindef")]
impl Default for DNS_RECORDW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union DNS_RECORDW_0 {
    pub DW: u32,
    pub S: DNS_RECORD_FLAGS,
}
#[cfg(feature = "minwindef")]
impl Default for DNS_RECORDW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
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
    pub pDataPtr: super::PBYTE,
}
#[cfg(feature = "minwindef")]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_RECORD_FLAGS {
    pub _bitfield: u32,
}
impl DNS_RECORD_FLAGS {
    pub fn Section(&self) -> u32 {
        (self._bitfield << 30) >> 30
    }
    pub fn set_Section(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !3) | (value & 3);
    }
    pub fn Delete(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_Delete(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u32) << 2);
    }
    pub fn CharSet(&self) -> u32 {
        (self._bitfield << 27) >> 30
    }
    pub fn set_CharSet(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(3 << 3)) | ((value & 3) << 3);
    }
    pub fn Unused(&self) -> u32 {
        (self._bitfield << 24) >> 29
    }
    pub fn set_Unused(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(7 << 5)) | ((value & 7) << 5);
    }
    pub fn Reserved(&self) -> u32 {
        self._bitfield >> 8
    }
    pub fn set_Reserved(&mut self, value: u32) {
        self._bitfield = (self._bitfield & !(16777215 << 8)) | ((value & 16777215) << 8);
    }
}
#[cfg(feature = "minwindef")]
pub type DNS_RECORD_OPT = DNS_RECORD_OPTA;
#[repr(C)]
#[cfg(feature = "minwindef")]
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
#[cfg(feature = "minwindef")]
impl Default for DNS_RECORD_OPTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union DNS_RECORD_OPTA_0 {
    pub DW: u32,
    pub S: DNS_RECORD_FLAGS,
}
#[cfg(feature = "minwindef")]
impl Default for DNS_RECORD_OPTA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union DNS_RECORD_OPTA_1 {
    pub OPT: DNS_OPT_DATA,
    pub Opt: DNS_OPT_DATA,
}
#[cfg(feature = "minwindef")]
impl Default for DNS_RECORD_OPTA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
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
#[cfg(feature = "minwindef")]
impl Default for DNS_RECORD_OPTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union DNS_RECORD_OPTW_0 {
    pub DW: u32,
    pub S: DNS_RECORD_FLAGS,
}
#[cfg(feature = "minwindef")]
impl Default for DNS_RECORD_OPTW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union DNS_RECORD_OPTW_1 {
    pub OPT: DNS_OPT_DATA,
    pub Opt: DNS_OPT_DATA,
}
#[cfg(feature = "minwindef")]
impl Default for DNS_RECORD_OPTW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DNS_RFC_MAX_UDP_PACKET_LENGTH: u32 = 512;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_RRSET {
    pub pFirstRR: PDNS_RECORD,
    pub pLastRR: PDNS_RECORD,
}
pub type DNS_RRSIG_DATA = DNS_RRSIG_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "minwindef")]
pub type DNS_SERVICE_BROWSE_CALLBACK = Option<unsafe extern "system" fn(status: u32, pquerycontext: *const core::ffi::c_void, pdnsrecord: *const DNS_RECORDA)>;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct DNS_SERVICE_BROWSE_REQUEST {
    pub Version: u32,
    pub InterfaceIndex: u32,
    pub QueryName: windows_core::PCWSTR,
    pub Anonymous: DNS_SERVICE_BROWSE_REQUEST_0,
    pub pQueryContext: *mut core::ffi::c_void,
}
#[cfg(feature = "minwindef")]
impl Default for DNS_SERVICE_BROWSE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union DNS_SERVICE_BROWSE_REQUEST_0 {
    pub pBrowseCallback: PDNS_SERVICE_BROWSE_CALLBACK,
    pub pBrowseCallbackV2: DNS_QUERY_COMPLETION_ROUTINE,
}
#[cfg(feature = "minwindef")]
impl Default for DNS_SERVICE_BROWSE_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DNS_SERVICE_CANCEL {
    pub reserved: *mut core::ffi::c_void,
}
impl Default for DNS_SERVICE_CANCEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DNS_SERVICE_INSTANCE {
    pub pszInstanceName: windows_core::PWSTR,
    pub pszHostName: windows_core::PWSTR,
    pub ip4Address: *mut IP4_ADDRESS,
    pub ip6Address: *mut IP6_ADDRESS,
    pub wPort: u16,
    pub wPriority: u16,
    pub wWeight: u16,
    pub dwPropertyCount: u32,
    pub keys: *mut windows_core::PWSTR,
    pub values: *mut windows_core::PWSTR,
    pub dwInterfaceIndex: u32,
}
impl Default for DNS_SERVICE_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DNS_SERVICE_REGISTER_COMPLETE = Option<unsafe extern "system" fn(status: u32, pquerycontext: *const core::ffi::c_void, pinstance: *const DNS_SERVICE_INSTANCE)>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug)]
pub struct DNS_SERVICE_REGISTER_REQUEST {
    pub Version: u32,
    pub InterfaceIndex: u32,
    pub pServiceInstance: PDNS_SERVICE_INSTANCE,
    pub pRegisterCompletionCallback: PDNS_SERVICE_REGISTER_COMPLETE,
    pub pQueryContext: *mut core::ffi::c_void,
    pub hCredentials: super::HANDLE,
    pub unicastEnabled: windows_core::BOOL,
}
#[cfg(feature = "winnt")]
impl Default for DNS_SERVICE_REGISTER_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DNS_SERVICE_RESOLVE_COMPLETE = Option<unsafe extern "system" fn(status: u32, pquerycontext: *const core::ffi::c_void, pinstance: *const DNS_SERVICE_INSTANCE)>;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DNS_SERVICE_RESOLVE_REQUEST {
    pub Version: u32,
    pub InterfaceIndex: u32,
    pub QueryName: windows_core::PWSTR,
    pub pResolveCompletionCallback: PDNS_SERVICE_RESOLVE_COMPLETE,
    pub pQueryContext: *mut core::ffi::c_void,
}
impl Default for DNS_SERVICE_RESOLVE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DNS_SIG_DATA = DNS_SIG_DATAA;
pub type DNS_SIG_DATAA = DNS_RRSIG_DATAA;
pub type DNS_SIG_DATAW = DNS_RRSIG_DATAW;
pub type DNS_SOA_DATA = DNS_SOA_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_SRV_DATAA {
    pub pNameTarget: windows_core::PSTR,
    pub wPriority: u16,
    pub wWeight: u16,
    pub wPort: u16,
    pub Pad: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DNS_SVCB_PARAM_UNKNOWN {
    pub cBytes: u16,
    pub pbSvcParamValue: [u8; 1],
}
impl Default for DNS_SVCB_PARAM_UNKNOWN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
pub type DNS_TKEY_DATA = DNS_TKEY_DATAA;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_TKEY_DATAA {
    pub pNameAlgorithm: windows_core::PSTR,
    pub pAlgorithmPacket: super::PBYTE,
    pub pKey: super::PBYTE,
    pub pOtherData: super::PBYTE,
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_TKEY_DATAW {
    pub pNameAlgorithm: windows_core::PWSTR,
    pub pAlgorithmPacket: super::PBYTE,
    pub pKey: super::PBYTE,
    pub pOtherData: super::PBYTE,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "minwindef")]
pub type DNS_TSIG_DATA = DNS_TSIG_DATAA;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_TSIG_DATAA {
    pub pNameAlgorithm: windows_core::PSTR,
    pub pAlgorithmPacket: super::PBYTE,
    pub pSignature: super::PBYTE,
    pub pOtherData: super::PBYTE,
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_TSIG_DATAW {
    pub pNameAlgorithm: windows_core::PWSTR,
    pub pAlgorithmPacket: super::PBYTE,
    pub pSignature: super::PBYTE,
    pub pOtherData: super::PBYTE,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DNS_UNKNOWN_DATA {
    pub dwByteCount: u32,
    pub bData: [u8; 1],
}
impl Default for DNS_UNKNOWN_DATA {
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
pub type DNS_WINSR_DATA = DNS_WINSR_DATAA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_WINSR_DATAA {
    pub dwMappingFlag: u32,
    pub dwLookupTimeout: u32,
    pub dwCacheTimeout: u32,
    pub pNameResultDomain: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DNS_WINSR_DATAW {
    pub dwMappingFlag: u32,
    pub dwLookupTimeout: u32,
    pub dwCacheTimeout: u32,
    pub pNameResultDomain: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const INTERNET_DEFAULT_DNS_PORT: u32 = 53;
pub const INTERNET_DEFAULT_DOT_PORT: u32 = 853;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct IP4_ADDRESS(pub u32);
pub const IP4_ADDRESS_STRING_BUFFER_LENGTH: u32 = 16;
pub const IP4_ADDRESS_STRING_LENGTH: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IP4_ARRAY {
    pub AddrCount: u32,
    pub AddrArray: [IP4_ADDRESS; 1],
}
impl Default for IP4_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
pub const IP6_ADDRESS_STRING_BUFFER_LENGTH: u32 = 65;
pub const IP6_ADDRESS_STRING_LENGTH: u32 = 65;
#[cfg(feature = "minwindef")]
pub type MDNS_QUERY_CALLBACK = Option<unsafe extern "system" fn(pquerycontext: *const core::ffi::c_void, pqueryhandle: *mut MDNS_QUERY_HANDLE, pqueryresults: *mut DNS_QUERY_RESULT)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug)]
pub struct MDNS_QUERY_REQUEST {
    pub Version: u32,
    pub ulRefCount: u32,
    pub Query: windows_core::PCWSTR,
    pub QueryType: u16,
    pub QueryOptions: u64,
    pub InterfaceIndex: u32,
    pub pQueryCallback: PMDNS_QUERY_CALLBACK,
    pub pQueryContext: *mut core::ffi::c_void,
    pub fAnswerReceived: windows_core::BOOL,
    pub ulResendCount: u32,
}
#[cfg(feature = "minwindef")]
impl Default for MDNS_QUERY_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PDNS_AAAA_DATA = *mut DNS_AAAA_DATA;
pub type PDNS_ADDR = *mut DNS_ADDR;
pub type PDNS_ADDR_ARRAY = *mut DNS_ADDR_ARRAY;
pub type PDNS_ATMA_DATA = *mut DNS_ATMA_DATA;
pub type PDNS_A_DATA = *mut DNS_A_DATA;
#[cfg(feature = "minwindef")]
pub type PDNS_CONNECTION_POLICY_ENTRY = *mut DNS_CONNECTION_POLICY_ENTRY;
pub type PDNS_CONNECTION_PROXY_INFO = *mut DNS_CONNECTION_PROXY_INFO;
#[cfg(feature = "winnt")]
pub type PDNS_CONNECTION_PROXY_INFO_EX = *mut DNS_CONNECTION_PROXY_INFO_EX;
pub type PDNS_DHCID_DATA = *mut DNS_DHCID_DATA;
pub type PDNS_DNSKEY_DATA = *mut DNS_DNSKEY_DATA;
pub type PDNS_DS_DATA = *mut DNS_DS_DATA;
pub type PDNS_HEADER = *mut DNS_HEADER;
pub type PDNS_HEADER_EXT = *mut DNS_HEADER_EXT;
pub type PDNS_KEY_DATA = *mut DNS_DNSKEY_DATA;
pub type PDNS_LOC_DATA = *mut DNS_LOC_DATA;
pub type PDNS_MESSAGE_BUFFER = *mut DNS_MESSAGE_BUFFER;
pub type PDNS_MINFO_DATA = *mut DNS_MINFO_DATAA;
pub type PDNS_MINFO_DATAA = *mut DNS_MINFO_DATAA;
pub type PDNS_MINFO_DATAW = *mut DNS_MINFO_DATAW;
pub type PDNS_MX_DATA = *mut DNS_MX_DATAA;
pub type PDNS_MX_DATAA = *mut DNS_MX_DATAA;
pub type PDNS_MX_DATAW = *mut DNS_MX_DATAW;
pub type PDNS_NAPTR_DATA = *mut DNS_NAPTR_DATAA;
pub type PDNS_NAPTR_DATAA = *mut DNS_NAPTR_DATAA;
pub type PDNS_NAPTR_DATAW = *mut DNS_NAPTR_DATAW;
pub type PDNS_NSEC3PARAM_DATA = *mut DNS_NSEC3PARAM_DATA;
pub type PDNS_NSEC3_DATA = *mut DNS_NSEC3_DATA;
pub type PDNS_NSEC_DATA = *mut DNS_NSEC_DATAA;
pub type PDNS_NSEC_DATAA = *mut DNS_NSEC_DATAA;
pub type PDNS_NSEC_DATAW = *mut DNS_NSEC_DATAW;
pub type PDNS_NULL_DATA = *mut DNS_NULL_DATA;
pub type PDNS_NXT_DATA = *mut DNS_NXT_DATAA;
pub type PDNS_NXT_DATAA = *mut DNS_NXT_DATAA;
pub type PDNS_NXT_DATAW = *mut DNS_NXT_DATAW;
pub type PDNS_OPT_DATA = *mut DNS_OPT_DATA;
pub type PDNS_PTR_DATA = *mut DNS_PTR_DATAA;
pub type PDNS_PTR_DATAA = *mut DNS_PTR_DATAA;
pub type PDNS_PTR_DATAW = *mut DNS_PTR_DATAW;
pub type PDNS_QUERY_CANCEL = *mut DNS_QUERY_CANCEL;
#[cfg(feature = "minwindef")]
pub type PDNS_QUERY_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(pquerycontext: *const core::ffi::c_void, pqueryresults: *mut DNS_QUERY_RESULT)>;
#[cfg(feature = "minwindef")]
pub type PDNS_QUERY_REQUEST = *mut DNS_QUERY_REQUEST;
#[cfg(all(feature = "in6addr", feature = "inaddr", feature = "minwindef", feature = "ws2"))]
pub type PDNS_QUERY_REQUEST3 = *mut DNS_QUERY_REQUEST3;
#[cfg(feature = "minwindef")]
pub type PDNS_QUERY_RESULT = *mut DNS_QUERY_RESULT;
#[cfg(feature = "minwindef")]
pub type PDNS_RECORD = *mut DNS_RECORDA;
#[cfg(feature = "minwindef")]
pub type PDNS_RECORDA = *mut DNS_RECORDA;
#[cfg(feature = "minwindef")]
pub type PDNS_RECORDW = *mut DNS_RECORDW;
#[cfg(feature = "minwindef")]
pub type PDNS_RECORD_OPT = *mut DNS_RECORD_OPTA;
#[cfg(feature = "minwindef")]
pub type PDNS_RECORD_OPTA = *mut DNS_RECORD_OPTA;
#[cfg(feature = "minwindef")]
pub type PDNS_RECORD_OPTW = *mut DNS_RECORD_OPTW;
#[cfg(feature = "minwindef")]
pub type PDNS_RRSET = *mut DNS_RRSET;
pub type PDNS_RRSIG_DATA = *mut DNS_RRSIG_DATAA;
pub type PDNS_RRSIG_DATAA = *mut DNS_RRSIG_DATAA;
pub type PDNS_RRSIG_DATAW = *mut DNS_RRSIG_DATAW;
#[cfg(feature = "minwindef")]
pub type PDNS_SERVICE_BROWSE_CALLBACK = Option<unsafe extern "system" fn(status: u32, pquerycontext: *const core::ffi::c_void, pdnsrecord: *const DNS_RECORDA)>;
#[cfg(feature = "minwindef")]
pub type PDNS_SERVICE_BROWSE_REQUEST = *mut DNS_SERVICE_BROWSE_REQUEST;
pub type PDNS_SERVICE_CANCEL = *mut DNS_SERVICE_CANCEL;
pub type PDNS_SERVICE_INSTANCE = *mut DNS_SERVICE_INSTANCE;
pub type PDNS_SERVICE_REGISTER_COMPLETE = Option<unsafe extern "system" fn(status: u32, pquerycontext: *const core::ffi::c_void, pinstance: *const DNS_SERVICE_INSTANCE)>;
#[cfg(feature = "winnt")]
pub type PDNS_SERVICE_REGISTER_REQUEST = *mut DNS_SERVICE_REGISTER_REQUEST;
pub type PDNS_SERVICE_RESOLVE_COMPLETE = Option<unsafe extern "system" fn(status: u32, pquerycontext: *const core::ffi::c_void, pinstance: *const DNS_SERVICE_INSTANCE)>;
pub type PDNS_SERVICE_RESOLVE_REQUEST = *mut DNS_SERVICE_RESOLVE_REQUEST;
pub type PDNS_SIG_DATA = *mut DNS_SIG_DATAA;
pub type PDNS_SIG_DATAA = *mut DNS_RRSIG_DATAA;
pub type PDNS_SIG_DATAW = *mut DNS_RRSIG_DATAW;
pub type PDNS_SOA_DATA = *mut DNS_SOA_DATAA;
pub type PDNS_SOA_DATAA = *mut DNS_SOA_DATAA;
pub type PDNS_SOA_DATAW = *mut DNS_SOA_DATAW;
pub type PDNS_SRV_DATA = *mut DNS_SRV_DATAA;
pub type PDNS_SRV_DATAA = *mut DNS_SRV_DATAA;
pub type PDNS_SRV_DATAW = *mut DNS_SRV_DATAW;
pub type PDNS_STATUS = *mut DNS_STATUS;
#[cfg(feature = "minwindef")]
pub type PDNS_TKEY_DATA = *mut DNS_TKEY_DATAA;
#[cfg(feature = "minwindef")]
pub type PDNS_TKEY_DATAA = *mut DNS_TKEY_DATAA;
#[cfg(feature = "minwindef")]
pub type PDNS_TKEY_DATAW = *mut DNS_TKEY_DATAW;
pub type PDNS_TLSA_DATA = *mut DNS_TLSA_DATA;
#[cfg(feature = "minwindef")]
pub type PDNS_TSIG_DATA = *mut DNS_TSIG_DATAA;
#[cfg(feature = "minwindef")]
pub type PDNS_TSIG_DATAA = *mut DNS_TSIG_DATAA;
#[cfg(feature = "minwindef")]
pub type PDNS_TSIG_DATAW = *mut DNS_TSIG_DATAW;
pub type PDNS_TXT_DATA = *mut DNS_TXT_DATAA;
pub type PDNS_TXT_DATAA = *mut DNS_TXT_DATAA;
pub type PDNS_TXT_DATAW = *mut DNS_TXT_DATAW;
pub type PDNS_UNKNOWN_DATA = *mut DNS_UNKNOWN_DATA;
pub type PDNS_WINSR_DATA = *mut DNS_WINSR_DATAA;
pub type PDNS_WINSR_DATAA = *mut DNS_WINSR_DATAA;
pub type PDNS_WINSR_DATAW = *mut DNS_WINSR_DATAW;
pub type PDNS_WINS_DATA = *mut DNS_WINS_DATA;
pub type PDNS_WIRE_QUESTION = *mut DNS_WIRE_QUESTION;
pub type PDNS_WIRE_RECORD = *mut DNS_WIRE_RECORD;
pub type PDNS_WKS_DATA = *mut DNS_WKS_DATA;
pub type PIP4_ADDRESS = *mut u32;
pub type PIP4_ARRAY = *mut IP4_ARRAY;
pub type PIP6_ADDRESS = *mut IP6_ADDRESS;
#[cfg(feature = "minwindef")]
pub type PMDNS_QUERY_CALLBACK = Option<unsafe extern "system" fn(pquerycontext: *const core::ffi::c_void, pqueryhandle: *mut MDNS_QUERY_HANDLE, pqueryresults: *mut DNS_QUERY_RESULT)>;
pub type PMDNS_QUERY_HANDLE = *mut MDNS_QUERY_HANDLE;
#[cfg(feature = "minwindef")]
pub type PMDNS_QUERY_REQUEST = *mut MDNS_QUERY_REQUEST;
pub type PQWORD = *mut u64;
#[cfg(target_arch = "x86")]
pub const SIZEOF_DNS_RECORD_HEADER: u32 = 24;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SIZEOF_DNS_RECORD_HEADER: u32 = 32;
pub const SIZEOF_IP4_ADDRESS: u32 = 4;
pub const TAG_DNS_CONNECTION_POLICY_TAG_CONNECTION_MANAGER: DNS_CONNECTION_POLICY_TAG = 1;
pub const TAG_DNS_CONNECTION_POLICY_TAG_DEFAULT: DNS_CONNECTION_POLICY_TAG = 0;
pub const TAG_DNS_CONNECTION_POLICY_TAG_WWWPT: DNS_CONNECTION_POLICY_TAG = 2;
