#[cfg(feature = "ws2")]
windows_link::link!("netapi32.dll" "system" fn DsAddressToSiteNamesA(computername : windows_sys::core::PCSTR, entrycount : u32, socketaddresses : *const super::SOCKET_ADDRESS, sitenames : *mut *mut windows_sys::core::PSTR) -> u32);
#[cfg(feature = "ws2")]
windows_link::link!("netapi32.dll" "system" fn DsAddressToSiteNamesExA(computername : windows_sys::core::PCSTR, entrycount : u32, socketaddresses : *const super::SOCKET_ADDRESS, sitenames : *mut *mut windows_sys::core::PSTR, subnetnames : *mut *mut windows_sys::core::PSTR) -> u32);
#[cfg(feature = "ws2")]
windows_link::link!("netapi32.dll" "system" fn DsAddressToSiteNamesExW(computername : windows_sys::core::PCWSTR, entrycount : u32, socketaddresses : *const super::SOCKET_ADDRESS, sitenames : *mut *mut windows_sys::core::PWSTR, subnetnames : *mut *mut windows_sys::core::PWSTR) -> u32);
#[cfg(feature = "ws2")]
windows_link::link!("netapi32.dll" "system" fn DsAddressToSiteNamesW(computername : windows_sys::core::PCWSTR, entrycount : u32, socketaddresses : *const super::SOCKET_ADDRESS, sitenames : *mut *mut windows_sys::core::PWSTR) -> u32);
windows_link::link!("netapi32.dll" "system" fn DsDeregisterDnsHostRecordsA(servername : windows_sys::core::PCSTR, dnsdomainname : windows_sys::core::PCSTR, domainguid : *const windows_sys::core::GUID, dsaguid : *const windows_sys::core::GUID, dnshostname : windows_sys::core::PCSTR) -> u32);
windows_link::link!("netapi32.dll" "system" fn DsDeregisterDnsHostRecordsW(servername : windows_sys::core::PCWSTR, dnsdomainname : windows_sys::core::PCWSTR, domainguid : *const windows_sys::core::GUID, dsaguid : *const windows_sys::core::GUID, dnshostname : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("netapi32.dll" "system" fn DsEnumerateDomainTrustsA(servername : windows_sys::core::PCSTR, flags : u32, domains : *mut PDS_DOMAIN_TRUSTSA, domaincount : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("netapi32.dll" "system" fn DsEnumerateDomainTrustsW(servername : windows_sys::core::PCWSTR, flags : u32, domains : *mut PDS_DOMAIN_TRUSTSW, domaincount : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("netapi32.dll" "system" fn DsGetDcCloseW(getdccontexthandle : super::HANDLE));
windows_link::link!("netapi32.dll" "system" fn DsGetDcNameA(computername : windows_sys::core::PCSTR, domainname : windows_sys::core::PCSTR, domainguid : *const windows_sys::core::GUID, sitename : windows_sys::core::PCSTR, flags : u32, domaincontrollerinfo : *mut PDOMAIN_CONTROLLER_INFOA) -> u32);
windows_link::link!("netapi32.dll" "system" fn DsGetDcNameW(computername : windows_sys::core::PCWSTR, domainname : windows_sys::core::PCWSTR, domainguid : *const windows_sys::core::GUID, sitename : windows_sys::core::PCWSTR, flags : u32, domaincontrollerinfo : *mut PDOMAIN_CONTROLLER_INFOW) -> u32);
#[cfg(all(feature = "winnt", feature = "ws2"))]
windows_link::link!("netapi32.dll" "system" fn DsGetDcNextA(getdccontexthandle : super::HANDLE, sockaddresscount : *mut u32, sockaddresses : *mut super::LPSOCKET_ADDRESS, dnshostname : *mut windows_sys::core::PSTR) -> u32);
#[cfg(all(feature = "winnt", feature = "ws2"))]
windows_link::link!("netapi32.dll" "system" fn DsGetDcNextW(getdccontexthandle : super::HANDLE, sockaddresscount : *mut u32, sockaddresses : *mut super::LPSOCKET_ADDRESS, dnshostname : *mut windows_sys::core::PWSTR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("netapi32.dll" "system" fn DsGetDcOpenA(dnsname : windows_sys::core::PCSTR, optionflags : u32, sitename : windows_sys::core::PCSTR, domainguid : *const windows_sys::core::GUID, dnsforestname : windows_sys::core::PCSTR, dcflags : u32, retgetdccontext : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("netapi32.dll" "system" fn DsGetDcOpenW(dnsname : windows_sys::core::PCWSTR, optionflags : u32, sitename : windows_sys::core::PCWSTR, domainguid : *const windows_sys::core::GUID, dnsforestname : windows_sys::core::PCWSTR, dcflags : u32, retgetdccontext : *mut super::HANDLE) -> u32);
windows_link::link!("netapi32.dll" "system" fn DsGetDcSiteCoverageA(servername : windows_sys::core::PCSTR, entrycount : *mut u32, sitenames : *mut *mut windows_sys::core::PSTR) -> u32);
windows_link::link!("netapi32.dll" "system" fn DsGetDcSiteCoverageW(servername : windows_sys::core::PCWSTR, entrycount : *mut u32, sitenames : *mut *mut windows_sys::core::PWSTR) -> u32);
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("netapi32.dll" "system" fn DsGetForestTrustInformationW(servername : windows_sys::core::PCWSTR, trusteddomainname : windows_sys::core::PCWSTR, flags : u32, foresttrustinfo : *mut super::PLSA_FOREST_TRUST_INFORMATION) -> u32);
windows_link::link!("netapi32.dll" "system" fn DsGetSiteNameA(computername : windows_sys::core::PCSTR, sitename : *mut windows_sys::core::PSTR) -> u32);
windows_link::link!("netapi32.dll" "system" fn DsGetSiteNameW(computername : windows_sys::core::PCWSTR, sitename : *mut windows_sys::core::PWSTR) -> u32);
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("netapi32.dll" "system" fn DsMergeForestTrustInformationW(domainname : windows_sys::core::PCWSTR, newforesttrustinfo : *const super::LSA_FOREST_TRUST_INFORMATION, oldforesttrustinfo : *const super::LSA_FOREST_TRUST_INFORMATION, mergedforesttrustinfo : *mut super::PLSA_FOREST_TRUST_INFORMATION) -> u32);
windows_link::link!("netapi32.dll" "system" fn DsValidateSubnetNameA(subnetname : windows_sys::core::PCSTR) -> u32);
windows_link::link!("netapi32.dll" "system" fn DsValidateSubnetNameW(subnetname : windows_sys::core::PCWSTR) -> u32);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DOMAIN_CONTROLLER_INFOA {
    pub DomainControllerName: windows_sys::core::PSTR,
    pub DomainControllerAddress: windows_sys::core::PSTR,
    pub DomainControllerAddressType: u32,
    pub DomainGuid: windows_sys::core::GUID,
    pub DomainName: windows_sys::core::PSTR,
    pub DnsForestName: windows_sys::core::PSTR,
    pub Flags: u32,
    pub DcSiteName: windows_sys::core::PSTR,
    pub ClientSiteName: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DOMAIN_CONTROLLER_INFOW {
    pub DomainControllerName: windows_sys::core::PWSTR,
    pub DomainControllerAddress: windows_sys::core::PWSTR,
    pub DomainControllerAddressType: u32,
    pub DomainGuid: windows_sys::core::GUID,
    pub DomainName: windows_sys::core::PWSTR,
    pub DnsForestName: windows_sys::core::PWSTR,
    pub Flags: u32,
    pub DcSiteName: windows_sys::core::PWSTR,
    pub ClientSiteName: windows_sys::core::PWSTR,
}
pub const DSGETDC_VALID_FLAGS: u32 = 3355443185;
pub const DS_AVOID_SELF: i32 = 16384;
pub const DS_BACKGROUND_ONLY: i32 = 256;
pub const DS_CLOSEST_FLAG: i32 = 128;
pub const DS_DIRECTORY_SERVICE_10_REQUIRED: i32 = 8388608;
pub const DS_DIRECTORY_SERVICE_12_REQUIRED: i32 = 67108864;
pub const DS_DIRECTORY_SERVICE_13_REQUIRED: i32 = 33554432;
pub const DS_DIRECTORY_SERVICE_6_REQUIRED: i32 = 524288;
pub const DS_DIRECTORY_SERVICE_8_REQUIRED: i32 = 2097152;
pub const DS_DIRECTORY_SERVICE_9_REQUIRED: i32 = 4194304;
pub const DS_DIRECTORY_SERVICE_ALL_VERSIONS: i32 = 115867664;
pub const DS_DIRECTORY_SERVICE_PREFERRED: i32 = 32;
pub const DS_DIRECTORY_SERVICE_REQUIRED: i32 = 16;
pub const DS_DNS_CONTROLLER_FLAG: i32 = 536870912;
pub const DS_DNS_DOMAIN_FLAG: i32 = 1073741824;
pub const DS_DNS_FOREST_FLAG: u32 = 2147483648;
pub const DS_DOMAIN_DIRECT_INBOUND: i32 = 32;
pub const DS_DOMAIN_DIRECT_OUTBOUND: i32 = 2;
pub const DS_DOMAIN_IN_FOREST: i32 = 1;
pub const DS_DOMAIN_NATIVE_MODE: i32 = 16;
pub const DS_DOMAIN_PRIMARY: i32 = 8;
pub const DS_DOMAIN_TREE_ROOT: i32 = 4;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DS_DOMAIN_TRUSTSA {
    pub NetbiosDomainName: windows_sys::core::PSTR,
    pub DnsDomainName: windows_sys::core::PSTR,
    pub Flags: u32,
    pub ParentIndex: u32,
    pub TrustType: u32,
    pub TrustAttributes: u32,
    pub DomainSid: super::PSID,
    pub DomainGuid: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DS_DOMAIN_TRUSTSW {
    pub NetbiosDomainName: windows_sys::core::PWSTR,
    pub DnsDomainName: windows_sys::core::PWSTR,
    pub Flags: u32,
    pub ParentIndex: u32,
    pub TrustType: u32,
    pub TrustAttributes: u32,
    pub DomainSid: super::PSID,
    pub DomainGuid: windows_sys::core::GUID,
}
pub const DS_DOMAIN_VALID_FLAGS: i32 = 63;
pub const DS_DS_10_FLAG: i32 = 65536;
pub const DS_DS_12_FLAG: i32 = 524288;
pub const DS_DS_13_FLAG: i32 = 262144;
pub const DS_DS_8_FLAG: i32 = 16384;
pub const DS_DS_9_FLAG: i32 = 32768;
pub const DS_DS_FLAG: i32 = 16;
pub const DS_FORCE_REDISCOVERY: i32 = 1;
pub const DS_FULL_SECRET_DOMAIN_6_FLAG: i32 = 4096;
pub const DS_GC_FLAG: i32 = 4;
pub const DS_GC_SERVER_REQUIRED: i32 = 64;
pub const DS_GFTI_UPDATE_TDO: i32 = 1;
pub const DS_GFTI_VALID_FLAGS: i32 = 1;
pub const DS_GOOD_TIMESERV_FLAG: i32 = 512;
pub const DS_GOOD_TIMESERV_PREFERRED: i32 = 8192;
pub const DS_INET_ADDRESS: i32 = 1;
pub const DS_IP_REQUIRED: i32 = 512;
pub const DS_IS_DNS_NAME: i32 = 131072;
pub const DS_IS_FLAT_NAME: i32 = 65536;
pub const DS_KDC_FLAG: i32 = 32;
pub const DS_KDC_REQUIRED: i32 = 1024;
pub const DS_KEY_LIST_FLAG: i32 = 131072;
pub const DS_KEY_LIST_SUPPORT_REQUIRED: i32 = 16777216;
pub const DS_LDAP_FLAG: i32 = 8;
pub const DS_NDNC_FLAG: i32 = 1024;
pub const DS_NETBIOS_ADDRESS: i32 = 2;
pub const DS_NOTIFY_AFTER_SITE_RECORDS: i32 = 2;
pub const DS_ONLY_DO_SITE_NAME: i32 = 1;
pub const DS_ONLY_LDAP_NEEDED: i32 = 32768;
pub const DS_OPEN_VALID_FLAGS: i32 = 38081;
pub const DS_OPEN_VALID_OPTION_FLAGS: i32 = 3;
pub const DS_PDC_FLAG: i32 = 1;
pub const DS_PDC_REQUIRED: i32 = 128;
pub const DS_PING_FLAGS: i32 = 1048575;
pub const DS_RETURN_DNS_NAME: i32 = 1073741824;
pub const DS_RETURN_FLAT_NAME: u32 = 2147483648;
pub const DS_SELECT_SECRET_DOMAIN_6_FLAG: i32 = 2048;
pub const DS_TIMESERV_FLAG: i32 = 64;
pub const DS_TIMESERV_REQUIRED: i32 = 2048;
pub const DS_TRY_NEXTCLOSEST_SITE: i32 = 262144;
pub const DS_WEB_SERVICE_REQUIRED: i32 = 1048576;
pub const DS_WRITABLE_FLAG: i32 = 256;
pub const DS_WRITABLE_REQUIRED: i32 = 4096;
pub const DS_WS_FLAG: i32 = 8192;
pub type PDOMAIN_CONTROLLER_INFOA = *mut DOMAIN_CONTROLLER_INFOA;
pub type PDOMAIN_CONTROLLER_INFOW = *mut DOMAIN_CONTROLLER_INFOW;
#[cfg(feature = "winnt")]
pub type PDS_DOMAIN_TRUSTSA = *mut DS_DOMAIN_TRUSTSA;
#[cfg(feature = "winnt")]
pub type PDS_DOMAIN_TRUSTSW = *mut DS_DOMAIN_TRUSTSW;
