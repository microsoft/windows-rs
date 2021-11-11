#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsAcquireContextHandle_A();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsAcquireContextHandle_W();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsCancelQuery();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsConnectionDeletePolicyEntries();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionDeleteProxyInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsConnectionFreeNameList();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionFreeProxyInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionFreeProxyInfoEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionFreeProxyList();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsConnectionGetNameList();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionGetProxyInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionGetProxyInfoForHostUrl();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionGetProxyList();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionSetPolicyEntries();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionSetProxyInfo();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsConnectionUpdateIfIndexTable();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsExtractRecordsFromMessage_UTF8();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsExtractRecordsFromMessage_W();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsFree();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsFreeCustomServers();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsFreeProxyName();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsGetApplicationSettings();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsGetProxyInformation();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsModifyRecordsInSet_A();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsModifyRecordsInSet_UTF8();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsModifyRecordsInSet_W();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsNameCompare_A();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsNameCompare_W();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQueryConfig();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQueryEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQuery_A();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQuery_UTF8();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsQuery_W();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordCompare();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordCopyEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordSetCompare();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordSetCopyEx();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsRecordSetDetach();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsReleaseContextHandle();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsReplaceRecordSetA();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsReplaceRecordSetUTF8();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsReplaceRecordSetW();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceBrowse();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsServiceBrowseCancel();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceConstructInstance();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceCopyInstance();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceDeRegister();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceFreeInstance();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceRegister();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsServiceRegisterCancel();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsServiceResolve();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsServiceResolveCancel();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsSetApplicationSettings();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsStartMulticastQuery();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`*"]
    pub fn DnsStopMulticastQuery();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsValidateName_A();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsValidateName_UTF8();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsValidateName_W();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsWriteQuestionToBuffer_UTF8();
    #[doc = "*Required features: `Win32_NetworkManagement_Dns`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsWriteQuestionToBuffer_W();
}
