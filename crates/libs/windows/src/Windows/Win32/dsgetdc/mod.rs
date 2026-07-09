#[cfg(feature = "Win32_ws2def")]
#[inline]
pub unsafe fn DsAddressToSiteNamesA<P0>(computername: P0, socketaddresses: &[super::ws2def::SOCKET_ADDRESS], sitenames: *mut *mut windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsAddressToSiteNamesA(computername : windows_core::PCSTR, entrycount : u32, socketaddresses : *const super::ws2def::SOCKET_ADDRESS, sitenames : *mut *mut windows_core::PSTR) -> u32);
    unsafe { DsAddressToSiteNamesA(computername.param().abi(), socketaddresses.len().try_into().unwrap(), core::mem::transmute(socketaddresses.as_ptr()), sitenames as _) }
}
#[cfg(feature = "Win32_ws2def")]
#[inline]
pub unsafe fn DsAddressToSiteNamesExA<P0>(computername: P0, socketaddresses: &[super::ws2def::SOCKET_ADDRESS], sitenames: *mut *mut windows_core::PSTR, subnetnames: *mut *mut windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsAddressToSiteNamesExA(computername : windows_core::PCSTR, entrycount : u32, socketaddresses : *const super::ws2def::SOCKET_ADDRESS, sitenames : *mut *mut windows_core::PSTR, subnetnames : *mut *mut windows_core::PSTR) -> u32);
    unsafe { DsAddressToSiteNamesExA(computername.param().abi(), socketaddresses.len().try_into().unwrap(), core::mem::transmute(socketaddresses.as_ptr()), sitenames as _, subnetnames as _) }
}
#[cfg(feature = "Win32_ws2def")]
#[inline]
pub unsafe fn DsAddressToSiteNamesExW<P0>(computername: P0, socketaddresses: &[super::ws2def::SOCKET_ADDRESS], sitenames: *mut *mut windows_core::PWSTR, subnetnames: *mut *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsAddressToSiteNamesExW(computername : windows_core::PCWSTR, entrycount : u32, socketaddresses : *const super::ws2def::SOCKET_ADDRESS, sitenames : *mut *mut windows_core::PWSTR, subnetnames : *mut *mut windows_core::PWSTR) -> u32);
    unsafe { DsAddressToSiteNamesExW(computername.param().abi(), socketaddresses.len().try_into().unwrap(), core::mem::transmute(socketaddresses.as_ptr()), sitenames as _, subnetnames as _) }
}
#[cfg(feature = "Win32_ws2def")]
#[inline]
pub unsafe fn DsAddressToSiteNamesW<P0>(computername: P0, socketaddresses: &[super::ws2def::SOCKET_ADDRESS], sitenames: *mut *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsAddressToSiteNamesW(computername : windows_core::PCWSTR, entrycount : u32, socketaddresses : *const super::ws2def::SOCKET_ADDRESS, sitenames : *mut *mut windows_core::PWSTR) -> u32);
    unsafe { DsAddressToSiteNamesW(computername.param().abi(), socketaddresses.len().try_into().unwrap(), core::mem::transmute(socketaddresses.as_ptr()), sitenames as _) }
}
#[inline]
pub unsafe fn DsDeregisterDnsHostRecordsA<P0, P1, P4>(servername: P0, dnsdomainname: P1, domainguid: Option<*const windows_core::GUID>, dsaguid: Option<*const windows_core::GUID>, dnshostname: P4) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsDeregisterDnsHostRecordsA(servername : windows_core::PCSTR, dnsdomainname : windows_core::PCSTR, domainguid : *const windows_core::GUID, dsaguid : *const windows_core::GUID, dnshostname : windows_core::PCSTR) -> u32);
    unsafe { DsDeregisterDnsHostRecordsA(servername.param().abi(), dnsdomainname.param().abi(), domainguid.unwrap_or(core::mem::zeroed()) as _, dsaguid.unwrap_or(core::mem::zeroed()) as _, dnshostname.param().abi()) }
}
#[inline]
pub unsafe fn DsDeregisterDnsHostRecordsW<P0, P1, P4>(servername: P0, dnsdomainname: P1, domainguid: Option<*const windows_core::GUID>, dsaguid: Option<*const windows_core::GUID>, dnshostname: P4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsDeregisterDnsHostRecordsW(servername : windows_core::PCWSTR, dnsdomainname : windows_core::PCWSTR, domainguid : *const windows_core::GUID, dsaguid : *const windows_core::GUID, dnshostname : windows_core::PCWSTR) -> u32);
    unsafe { DsDeregisterDnsHostRecordsW(servername.param().abi(), dnsdomainname.param().abi(), domainguid.unwrap_or(core::mem::zeroed()) as _, dsaguid.unwrap_or(core::mem::zeroed()) as _, dnshostname.param().abi()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsEnumerateDomainTrustsA<P0>(servername: P0, flags: u32, domains: *mut PDS_DOMAIN_TRUSTSA, domaincount: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsEnumerateDomainTrustsA(servername : windows_core::PCSTR, flags : u32, domains : *mut PDS_DOMAIN_TRUSTSA, domaincount : *mut u32) -> u32);
    unsafe { DsEnumerateDomainTrustsA(servername.param().abi(), flags, domains as _, domaincount as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsEnumerateDomainTrustsW<P0>(servername: P0, flags: u32, domains: *mut PDS_DOMAIN_TRUSTSW, domaincount: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsEnumerateDomainTrustsW(servername : windows_core::PCWSTR, flags : u32, domains : *mut PDS_DOMAIN_TRUSTSW, domaincount : *mut u32) -> u32);
    unsafe { DsEnumerateDomainTrustsW(servername.param().abi(), flags, domains as _, domaincount as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsGetDcCloseW(getdccontexthandle: super::winnt::HANDLE) {
    windows_core::link!("netapi32.dll" "system" fn DsGetDcCloseW(getdccontexthandle : super::winnt::HANDLE));
    unsafe { DsGetDcCloseW(getdccontexthandle) }
}
#[inline]
pub unsafe fn DsGetDcNameA<P0, P1, P3>(computername: P0, domainname: P1, domainguid: Option<*const windows_core::GUID>, sitename: P3, flags: u32, domaincontrollerinfo: *mut PDOMAIN_CONTROLLER_INFOA) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsGetDcNameA(computername : windows_core::PCSTR, domainname : windows_core::PCSTR, domainguid : *const windows_core::GUID, sitename : windows_core::PCSTR, flags : u32, domaincontrollerinfo : *mut PDOMAIN_CONTROLLER_INFOA) -> u32);
    unsafe { DsGetDcNameA(computername.param().abi(), domainname.param().abi(), domainguid.unwrap_or(core::mem::zeroed()) as _, sitename.param().abi(), flags, domaincontrollerinfo as _) }
}
#[inline]
pub unsafe fn DsGetDcNameW<P0, P1, P3>(computername: P0, domainname: P1, domainguid: Option<*const windows_core::GUID>, sitename: P3, flags: u32, domaincontrollerinfo: *mut PDOMAIN_CONTROLLER_INFOW) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsGetDcNameW(computername : windows_core::PCWSTR, domainname : windows_core::PCWSTR, domainguid : *const windows_core::GUID, sitename : windows_core::PCWSTR, flags : u32, domaincontrollerinfo : *mut PDOMAIN_CONTROLLER_INFOW) -> u32);
    unsafe { DsGetDcNameW(computername.param().abi(), domainname.param().abi(), domainguid.unwrap_or(core::mem::zeroed()) as _, sitename.param().abi(), flags, domaincontrollerinfo as _) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_ws2def"))]
#[inline]
pub unsafe fn DsGetDcNextA(getdccontexthandle: super::winnt::HANDLE, sockaddresscount: Option<*mut u32>, sockaddresses: *mut super::ws2def::LPSOCKET_ADDRESS, dnshostname: *mut windows_core::PSTR) -> u32 {
    windows_core::link!("netapi32.dll" "system" fn DsGetDcNextA(getdccontexthandle : super::winnt::HANDLE, sockaddresscount : *mut u32, sockaddresses : *mut super::ws2def::LPSOCKET_ADDRESS, dnshostname : *mut windows_core::PSTR) -> u32);
    unsafe { DsGetDcNextA(getdccontexthandle, sockaddresscount.unwrap_or(core::mem::zeroed()) as _, sockaddresses as _, dnshostname as _) }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_ws2def"))]
#[inline]
pub unsafe fn DsGetDcNextW(getdccontexthandle: super::winnt::HANDLE, sockaddresscount: Option<*mut u32>, sockaddresses: *mut super::ws2def::LPSOCKET_ADDRESS, dnshostname: *mut windows_core::PWSTR) -> u32 {
    windows_core::link!("netapi32.dll" "system" fn DsGetDcNextW(getdccontexthandle : super::winnt::HANDLE, sockaddresscount : *mut u32, sockaddresses : *mut super::ws2def::LPSOCKET_ADDRESS, dnshostname : *mut windows_core::PWSTR) -> u32);
    unsafe { DsGetDcNextW(getdccontexthandle, sockaddresscount.unwrap_or(core::mem::zeroed()) as _, sockaddresses as _, dnshostname as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsGetDcOpenA<P0, P2, P4>(dnsname: P0, optionflags: u32, sitename: P2, domainguid: Option<*const windows_core::GUID>, dnsforestname: P4, dcflags: u32, retgetdccontext: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsGetDcOpenA(dnsname : windows_core::PCSTR, optionflags : u32, sitename : windows_core::PCSTR, domainguid : *const windows_core::GUID, dnsforestname : windows_core::PCSTR, dcflags : u32, retgetdccontext : *mut super::winnt::HANDLE) -> u32);
    unsafe { DsGetDcOpenA(dnsname.param().abi(), optionflags, sitename.param().abi(), domainguid.unwrap_or(core::mem::zeroed()) as _, dnsforestname.param().abi(), dcflags, retgetdccontext as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DsGetDcOpenW<P0, P2, P4>(dnsname: P0, optionflags: u32, sitename: P2, domainguid: Option<*const windows_core::GUID>, dnsforestname: P4, dcflags: u32, retgetdccontext: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsGetDcOpenW(dnsname : windows_core::PCWSTR, optionflags : u32, sitename : windows_core::PCWSTR, domainguid : *const windows_core::GUID, dnsforestname : windows_core::PCWSTR, dcflags : u32, retgetdccontext : *mut super::winnt::HANDLE) -> u32);
    unsafe { DsGetDcOpenW(dnsname.param().abi(), optionflags, sitename.param().abi(), domainguid.unwrap_or(core::mem::zeroed()) as _, dnsforestname.param().abi(), dcflags, retgetdccontext as _) }
}
#[inline]
pub unsafe fn DsGetDcSiteCoverageA<P0>(servername: P0, entrycount: *mut u32, sitenames: *mut *mut windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsGetDcSiteCoverageA(servername : windows_core::PCSTR, entrycount : *mut u32, sitenames : *mut *mut windows_core::PSTR) -> u32);
    unsafe { DsGetDcSiteCoverageA(servername.param().abi(), entrycount as _, sitenames as _) }
}
#[inline]
pub unsafe fn DsGetDcSiteCoverageW<P0>(servername: P0, entrycount: *mut u32, sitenames: *mut *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsGetDcSiteCoverageW(servername : windows_core::PCWSTR, entrycount : *mut u32, sitenames : *mut *mut windows_core::PWSTR) -> u32);
    unsafe { DsGetDcSiteCoverageW(servername.param().abi(), entrycount as _, sitenames as _) }
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_ntsecapi", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn DsGetForestTrustInformationW<P0, P1>(servername: P0, trusteddomainname: P1, flags: u32, foresttrustinfo: *mut super::ntsecapi::PLSA_FOREST_TRUST_INFORMATION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsGetForestTrustInformationW(servername : windows_core::PCWSTR, trusteddomainname : windows_core::PCWSTR, flags : u32, foresttrustinfo : *mut super::ntsecapi::PLSA_FOREST_TRUST_INFORMATION) -> u32);
    unsafe { DsGetForestTrustInformationW(servername.param().abi(), trusteddomainname.param().abi(), flags, foresttrustinfo as _) }
}
#[inline]
pub unsafe fn DsGetSiteNameA<P0>(computername: P0, sitename: *mut windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsGetSiteNameA(computername : windows_core::PCSTR, sitename : *mut windows_core::PSTR) -> u32);
    unsafe { DsGetSiteNameA(computername.param().abi(), sitename as _) }
}
#[inline]
pub unsafe fn DsGetSiteNameW<P0>(computername: P0, sitename: *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsGetSiteNameW(computername : windows_core::PCWSTR, sitename : *mut windows_core::PWSTR) -> u32);
    unsafe { DsGetSiteNameW(computername.param().abi(), sitename as _) }
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_ntsecapi", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn DsMergeForestTrustInformationW<P0>(domainname: P0, newforesttrustinfo: *const super::ntsecapi::LSA_FOREST_TRUST_INFORMATION, oldforesttrustinfo: Option<*const super::ntsecapi::LSA_FOREST_TRUST_INFORMATION>, mergedforesttrustinfo: *mut super::ntsecapi::PLSA_FOREST_TRUST_INFORMATION) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsMergeForestTrustInformationW(domainname : windows_core::PCWSTR, newforesttrustinfo : *const super::ntsecapi::LSA_FOREST_TRUST_INFORMATION, oldforesttrustinfo : *const super::ntsecapi::LSA_FOREST_TRUST_INFORMATION, mergedforesttrustinfo : *mut super::ntsecapi::PLSA_FOREST_TRUST_INFORMATION) -> u32);
    unsafe { DsMergeForestTrustInformationW(domainname.param().abi(), newforesttrustinfo, oldforesttrustinfo.unwrap_or(core::mem::zeroed()) as _, mergedforesttrustinfo as _) }
}
#[inline]
pub unsafe fn DsValidateSubnetNameA<P0>(subnetname: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsValidateSubnetNameA(subnetname : windows_core::PCSTR) -> u32);
    unsafe { DsValidateSubnetNameA(subnetname.param().abi()) }
}
#[inline]
pub unsafe fn DsValidateSubnetNameW<P0>(subnetname: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn DsValidateSubnetNameW(subnetname : windows_core::PCWSTR) -> u32);
    unsafe { DsValidateSubnetNameW(subnetname.param().abi()) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOMAIN_CONTROLLER_INFOA {
    pub DomainControllerName: windows_core::PSTR,
    pub DomainControllerAddress: windows_core::PSTR,
    pub DomainControllerAddressType: u32,
    pub DomainGuid: windows_core::GUID,
    pub DomainName: windows_core::PSTR,
    pub DnsForestName: windows_core::PSTR,
    pub Flags: u32,
    pub DcSiteName: windows_core::PSTR,
    pub ClientSiteName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOMAIN_CONTROLLER_INFOW {
    pub DomainControllerName: windows_core::PWSTR,
    pub DomainControllerAddress: windows_core::PWSTR,
    pub DomainControllerAddressType: u32,
    pub DomainGuid: windows_core::GUID,
    pub DomainName: windows_core::PWSTR,
    pub DnsForestName: windows_core::PWSTR,
    pub Flags: u32,
    pub DcSiteName: windows_core::PWSTR,
    pub ClientSiteName: windows_core::PWSTR,
}
pub const DSGETDC_VALID_FLAGS: i32 = -939524111;
pub const DS_AVOID_SELF: u32 = 16384;
pub const DS_BACKGROUND_ONLY: u32 = 256;
pub const DS_CLOSEST_FLAG: u32 = 128;
pub const DS_DIRECTORY_SERVICE_10_REQUIRED: u32 = 8388608;
pub const DS_DIRECTORY_SERVICE_12_REQUIRED: u32 = 67108864;
pub const DS_DIRECTORY_SERVICE_13_REQUIRED: u32 = 33554432;
pub const DS_DIRECTORY_SERVICE_6_REQUIRED: u32 = 524288;
pub const DS_DIRECTORY_SERVICE_8_REQUIRED: u32 = 2097152;
pub const DS_DIRECTORY_SERVICE_9_REQUIRED: u32 = 4194304;
pub const DS_DIRECTORY_SERVICE_ALL_VERSIONS: u32 = 115867664;
pub const DS_DIRECTORY_SERVICE_PREFERRED: u32 = 32;
pub const DS_DIRECTORY_SERVICE_REQUIRED: u32 = 16;
pub const DS_DNS_CONTROLLER_FLAG: u32 = 536870912;
pub const DS_DNS_DOMAIN_FLAG: u32 = 1073741824;
pub const DS_DNS_FOREST_FLAG: u32 = 2147483648;
pub const DS_DOMAIN_DIRECT_INBOUND: u32 = 32;
pub const DS_DOMAIN_DIRECT_OUTBOUND: u32 = 2;
pub const DS_DOMAIN_IN_FOREST: u32 = 1;
pub const DS_DOMAIN_NATIVE_MODE: u32 = 16;
pub const DS_DOMAIN_PRIMARY: u32 = 8;
pub const DS_DOMAIN_TREE_ROOT: u32 = 4;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_DOMAIN_TRUSTSA {
    pub NetbiosDomainName: windows_core::PSTR,
    pub DnsDomainName: windows_core::PSTR,
    pub Flags: u32,
    pub ParentIndex: u32,
    pub TrustType: u32,
    pub TrustAttributes: u32,
    pub DomainSid: super::winnt::PSID,
    pub DomainGuid: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DS_DOMAIN_TRUSTSW {
    pub NetbiosDomainName: windows_core::PWSTR,
    pub DnsDomainName: windows_core::PWSTR,
    pub Flags: u32,
    pub ParentIndex: u32,
    pub TrustType: u32,
    pub TrustAttributes: u32,
    pub DomainSid: super::winnt::PSID,
    pub DomainGuid: windows_core::GUID,
}
pub const DS_DOMAIN_VALID_FLAGS: u32 = 63;
pub const DS_DS_10_FLAG: u32 = 65536;
pub const DS_DS_12_FLAG: u32 = 524288;
pub const DS_DS_13_FLAG: u32 = 262144;
pub const DS_DS_8_FLAG: u32 = 16384;
pub const DS_DS_9_FLAG: u32 = 32768;
pub const DS_DS_FLAG: u32 = 16;
pub const DS_FORCE_REDISCOVERY: u32 = 1;
pub const DS_FULL_SECRET_DOMAIN_6_FLAG: u32 = 4096;
pub const DS_GC_FLAG: u32 = 4;
pub const DS_GC_SERVER_REQUIRED: u32 = 64;
pub const DS_GFTI_UPDATE_TDO: u32 = 1;
pub const DS_GFTI_VALID_FLAGS: u32 = 1;
pub const DS_GOOD_TIMESERV_FLAG: u32 = 512;
pub const DS_GOOD_TIMESERV_PREFERRED: u32 = 8192;
pub const DS_INET_ADDRESS: u32 = 1;
pub const DS_IP_REQUIRED: u32 = 512;
pub const DS_IS_DNS_NAME: u32 = 131072;
pub const DS_IS_FLAT_NAME: u32 = 65536;
pub const DS_KDC_FLAG: u32 = 32;
pub const DS_KDC_REQUIRED: u32 = 1024;
pub const DS_KEY_LIST_FLAG: u32 = 131072;
pub const DS_KEY_LIST_SUPPORT_REQUIRED: u32 = 16777216;
pub const DS_LDAP_FLAG: u32 = 8;
pub const DS_NDNC_FLAG: u32 = 1024;
pub const DS_NETBIOS_ADDRESS: u32 = 2;
pub const DS_NOTIFY_AFTER_SITE_RECORDS: u32 = 2;
pub const DS_ONLY_DO_SITE_NAME: u32 = 1;
pub const DS_ONLY_LDAP_NEEDED: u32 = 32768;
pub const DS_OPEN_VALID_FLAGS: u32 = 38081;
pub const DS_OPEN_VALID_OPTION_FLAGS: u32 = 3;
pub const DS_PDC_FLAG: u32 = 1;
pub const DS_PDC_REQUIRED: u32 = 128;
pub const DS_PING_FLAGS: u32 = 1048575;
pub const DS_RETURN_DNS_NAME: u32 = 1073741824;
pub const DS_RETURN_FLAT_NAME: u32 = 2147483648;
pub const DS_SELECT_SECRET_DOMAIN_6_FLAG: u32 = 2048;
pub const DS_TIMESERV_FLAG: u32 = 64;
pub const DS_TIMESERV_REQUIRED: u32 = 2048;
pub const DS_TRY_NEXTCLOSEST_SITE: u32 = 262144;
pub const DS_WEB_SERVICE_REQUIRED: u32 = 1048576;
pub const DS_WRITABLE_FLAG: u32 = 256;
pub const DS_WRITABLE_REQUIRED: u32 = 4096;
pub const DS_WS_FLAG: u32 = 8192;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOMAIN_CONTROLLER_INFOA(pub *mut DOMAIN_CONTROLLER_INFOA);
impl PDOMAIN_CONTROLLER_INFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOMAIN_CONTROLLER_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDOMAIN_CONTROLLER_INFOW(pub *mut DOMAIN_CONTROLLER_INFOW);
impl PDOMAIN_CONTROLLER_INFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDOMAIN_CONTROLLER_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_DOMAIN_TRUSTSA(pub *mut DS_DOMAIN_TRUSTSA);
#[cfg(feature = "Win32_winnt")]
impl PDS_DOMAIN_TRUSTSA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PDS_DOMAIN_TRUSTSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDS_DOMAIN_TRUSTSW(pub *mut DS_DOMAIN_TRUSTSW);
#[cfg(feature = "Win32_winnt")]
impl PDS_DOMAIN_TRUSTSW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PDS_DOMAIN_TRUSTSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
