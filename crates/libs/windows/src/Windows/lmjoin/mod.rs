#[inline]
pub unsafe fn NetAddAlternateComputerName<P0, P1, P2, P3>(server: P0, alternatename: P1, domainaccount: P2, domainaccountpassword: P3, reserved: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetAddAlternateComputerName(server : windows_core::PCWSTR, alternatename : windows_core::PCWSTR, domainaccount : windows_core::PCWSTR, domainaccountpassword : windows_core::PCWSTR, reserved : u32) -> u32);
    unsafe { NetAddAlternateComputerName(server.param().abi(), alternatename.param().abi(), domainaccount.param().abi(), domainaccountpassword.param().abi(), reserved) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetCreateProvisioningPackage(pprovisioningparams: *const NETSETUP_PROVISIONING_PARAMS, pppackagebindata: *mut super::minwindef::PBYTE, pdwpackagebindatasize: Option<*mut u32>, pppackagetextdata: *mut windows_core::PWSTR) -> u32 {
    windows_core::link!("netapi32.dll" "system" fn NetCreateProvisioningPackage(pprovisioningparams : *const NETSETUP_PROVISIONING_PARAMS, pppackagebindata : *mut super::minwindef::PBYTE, pdwpackagebindatasize : *mut u32, pppackagetextdata : *mut windows_core::PWSTR) -> u32);
    unsafe { NetCreateProvisioningPackage(pprovisioningparams, pppackagebindata as _, pdwpackagebindatasize.unwrap_or(core::mem::zeroed()) as _, pppackagetextdata as _) }
}
#[inline]
pub unsafe fn NetEnumerateComputerNames<P0>(server: P0, nametype: NET_COMPUTER_NAME_TYPE, reserved: u32, entrycount: *mut u32, computernames: *mut *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetEnumerateComputerNames(server : windows_core::PCWSTR, nametype : NET_COMPUTER_NAME_TYPE, reserved : u32, entrycount : *mut u32, computernames : *mut *mut windows_core::PWSTR) -> u32);
    unsafe { NetEnumerateComputerNames(server.param().abi(), nametype, reserved, entrycount as _, computernames as _) }
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[inline]
pub unsafe fn NetFreeAadJoinInformation(pjoininfo: Option<*const DSREG_JOIN_INFO>) {
    windows_core::link!("netapi32.dll" "system" fn NetFreeAadJoinInformation(pjoininfo : *const DSREG_JOIN_INFO));
    unsafe { NetFreeAadJoinInformation(pjoininfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[inline]
pub unsafe fn NetGetAadJoinInformation<P0>(pcsztenantid: P0) -> windows_core::Result<PDSREG_JOIN_INFO>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGetAadJoinInformation(pcsztenantid : windows_core::PCWSTR, ppjoininfo : *mut PDSREG_JOIN_INFO) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        NetGetAadJoinInformation(pcsztenantid.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn NetGetJoinInformation<P0>(lpserver: P0, lpnamebuffer: *mut windows_core::PWSTR, buffertype: *mut NETSETUP_JOIN_STATUS) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGetJoinInformation(lpserver : windows_core::PCWSTR, lpnamebuffer : *mut windows_core::PWSTR, buffertype : *mut NETSETUP_JOIN_STATUS) -> u32);
    unsafe { NetGetJoinInformation(lpserver.param().abi(), lpnamebuffer as _, buffertype as _) }
}
#[inline]
pub unsafe fn NetGetJoinableOUs<P0, P1, P2, P3>(lpserver: P0, lpdomain: P1, lpaccount: P2, lppassword: P3, oucount: *mut u32, ous: *mut *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGetJoinableOUs(lpserver : windows_core::PCWSTR, lpdomain : windows_core::PCWSTR, lpaccount : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, oucount : *mut u32, ous : *mut *mut windows_core::PWSTR) -> u32);
    unsafe { NetGetJoinableOUs(lpserver.param().abi(), lpdomain.param().abi(), lpaccount.param().abi(), lppassword.param().abi(), oucount as _, ous as _) }
}
#[inline]
pub unsafe fn NetJoinDomain<P0, P1, P2, P3, P4>(lpserver: P0, lpdomain: P1, lpmachineaccountou: P2, lpaccount: P3, lppassword: P4, fjoinoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetJoinDomain(lpserver : windows_core::PCWSTR, lpdomain : windows_core::PCWSTR, lpmachineaccountou : windows_core::PCWSTR, lpaccount : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, fjoinoptions : u32) -> u32);
    unsafe { NetJoinDomain(lpserver.param().abi(), lpdomain.param().abi(), lpmachineaccountou.param().abi(), lpaccount.param().abi(), lppassword.param().abi(), fjoinoptions) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NetProvisionComputerAccount<P0, P1, P2, P3>(lpdomain: P0, lpmachinename: P1, lpmachineaccountou: P2, lpdcname: P3, dwoptions: u32, pprovisionbindata: *mut super::minwindef::PBYTE, pdwprovisionbindatasize: Option<*mut u32>, pprovisiontextdata: *mut windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetProvisionComputerAccount(lpdomain : windows_core::PCWSTR, lpmachinename : windows_core::PCWSTR, lpmachineaccountou : windows_core::PCWSTR, lpdcname : windows_core::PCWSTR, dwoptions : u32, pprovisionbindata : *mut super::minwindef::PBYTE, pdwprovisionbindatasize : *mut u32, pprovisiontextdata : *mut windows_core::PWSTR) -> u32);
    unsafe { NetProvisionComputerAccount(lpdomain.param().abi(), lpmachinename.param().abi(), lpmachineaccountou.param().abi(), lpdcname.param().abi(), dwoptions, pprovisionbindata as _, pdwprovisionbindatasize.unwrap_or(core::mem::zeroed()) as _, pprovisiontextdata as _) }
}
#[inline]
pub unsafe fn NetRemoveAlternateComputerName<P0, P1, P2, P3>(server: P0, alternatename: P1, domainaccount: P2, domainaccountpassword: P3, reserved: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetRemoveAlternateComputerName(server : windows_core::PCWSTR, alternatename : windows_core::PCWSTR, domainaccount : windows_core::PCWSTR, domainaccountpassword : windows_core::PCWSTR, reserved : u32) -> u32);
    unsafe { NetRemoveAlternateComputerName(server.param().abi(), alternatename.param().abi(), domainaccount.param().abi(), domainaccountpassword.param().abi(), reserved) }
}
#[inline]
pub unsafe fn NetRenameMachineInDomain<P0, P1, P2, P3>(lpserver: P0, lpnewmachinename: P1, lpaccount: P2, lppassword: P3, frenameoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetRenameMachineInDomain(lpserver : windows_core::PCWSTR, lpnewmachinename : windows_core::PCWSTR, lpaccount : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, frenameoptions : u32) -> u32);
    unsafe { NetRenameMachineInDomain(lpserver.param().abi(), lpnewmachinename.param().abi(), lpaccount.param().abi(), lppassword.param().abi(), frenameoptions) }
}
#[inline]
pub unsafe fn NetRequestOfflineDomainJoin<P3>(pprovisionbindata: &[u8], dwoptions: u32, lpwindowspath: P3) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetRequestOfflineDomainJoin(pprovisionbindata : *const u8, cbprovisionbindatasize : u32, dwoptions : u32, lpwindowspath : windows_core::PCWSTR) -> u32);
    unsafe { NetRequestOfflineDomainJoin(core::mem::transmute(pprovisionbindata.as_ptr()), pprovisionbindata.len().try_into().unwrap(), dwoptions, lpwindowspath.param().abi()) }
}
#[inline]
pub unsafe fn NetRequestProvisioningPackageInstall<P3>(ppackagebindata: &[u8], dwprovisionoptions: u32, lpwindowspath: P3, pvreserved: Option<*const core::ffi::c_void>) -> u32
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetRequestProvisioningPackageInstall(ppackagebindata : *const u8, dwpackagebindatasize : u32, dwprovisionoptions : u32, lpwindowspath : windows_core::PCWSTR, pvreserved : *const core::ffi::c_void) -> u32);
    unsafe { NetRequestProvisioningPackageInstall(core::mem::transmute(ppackagebindata.as_ptr()), ppackagebindata.len().try_into().unwrap(), dwprovisionoptions, lpwindowspath.param().abi(), pvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn NetSetPrimaryComputerName<P0, P1, P2, P3>(server: P0, primaryname: P1, domainaccount: P2, domainaccountpassword: P3, reserved: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetSetPrimaryComputerName(server : windows_core::PCWSTR, primaryname : windows_core::PCWSTR, domainaccount : windows_core::PCWSTR, domainaccountpassword : windows_core::PCWSTR, reserved : u32) -> u32);
    unsafe { NetSetPrimaryComputerName(server.param().abi(), primaryname.param().abi(), domainaccount.param().abi(), domainaccountpassword.param().abi(), reserved) }
}
#[inline]
pub unsafe fn NetUnjoinDomain<P0, P1, P2>(lpserver: P0, lpaccount: P1, lppassword: P2, funjoinoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUnjoinDomain(lpserver : windows_core::PCWSTR, lpaccount : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, funjoinoptions : u32) -> u32);
    unsafe { NetUnjoinDomain(lpserver.param().abi(), lpaccount.param().abi(), lppassword.param().abi(), funjoinoptions) }
}
#[inline]
pub unsafe fn NetValidateName<P0, P1, P2, P3>(lpserver: P0, lpname: P1, lpaccount: P2, lppassword: P3, nametype: NETSETUP_NAME_TYPE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetValidateName(lpserver : windows_core::PCWSTR, lpname : windows_core::PCWSTR, lpaccount : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, nametype : NETSETUP_NAME_TYPE) -> u32);
    unsafe { NetValidateName(lpserver.param().abi(), lpname.param().abi(), lpaccount.param().abi(), lppassword.param().abi(), nametype) }
}
pub const DSREG_DEVICE_JOIN: DSREG_JOIN_TYPE = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DSREG_JOIN_INFO {
    pub joinType: DSREG_JOIN_TYPE,
    pub pJoinCertificate: super::wincrypt::PCCERT_CONTEXT,
    pub pszDeviceId: windows_core::PWSTR,
    pub pszIdpDomain: windows_core::PWSTR,
    pub pszTenantId: windows_core::PWSTR,
    pub pszJoinUserEmail: windows_core::PWSTR,
    pub pszTenantDisplayName: windows_core::PWSTR,
    pub pszMdmEnrollmentUrl: windows_core::PWSTR,
    pub pszMdmTermsOfUseUrl: windows_core::PWSTR,
    pub pszMdmComplianceUrl: windows_core::PWSTR,
    pub pszUserSettingSyncUrl: windows_core::PWSTR,
    pub pUserInfo: *mut DSREG_USER_INFO,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for DSREG_JOIN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DSREG_JOIN_TYPE = i32;
pub const DSREG_UNKNOWN_JOIN: DSREG_JOIN_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DSREG_USER_INFO {
    pub pszUserEmail: windows_core::PWSTR,
    pub pszUserKeyId: windows_core::PWSTR,
    pub pszUserKeyName: windows_core::PWSTR,
}
pub const DSREG_WORKPLACE_JOIN: DSREG_JOIN_TYPE = 2;
pub const NETSETUP_ACCT_CREATE: u32 = 2;
pub const NETSETUP_ACCT_DELETE: u32 = 4;
pub const NETSETUP_ALT_SAMACCOUNTNAME: u32 = 131072;
pub const NETSETUP_AMBIGUOUS_DC: u32 = 4096;
pub const NETSETUP_DEFER_SPN_SET: u32 = 256;
pub const NETSETUP_DNS_NAME_CHANGES_ONLY: u32 = 4096;
pub const NETSETUP_DOMAIN_JOIN_IF_JOINED: u32 = 32;
pub const NETSETUP_DONT_CONTROL_SERVICES: u32 = 16384;
pub const NETSETUP_FORCE_SPN_SET: u32 = 65536;
pub const NETSETUP_IGNORE_UNSUPPORTED_FLAGS: u32 = 268435456;
pub const NETSETUP_INSTALL_INVOCATION: u32 = 262144;
pub const NETSETUP_JOIN_DC_ACCOUNT: u32 = 512;
pub const NETSETUP_JOIN_DOMAIN: u32 = 1;
pub const NETSETUP_JOIN_READONLY: u32 = 2048;
pub type NETSETUP_JOIN_STATUS = i32;
pub const NETSETUP_JOIN_UNSECURE: u32 = 64;
pub const NETSETUP_JOIN_WITH_NEW_NAME: u32 = 1024;
pub const NETSETUP_MACHINE_PWD_PASSED: u32 = 128;
pub type NETSETUP_NAME_TYPE = i32;
pub const NETSETUP_NO_ACCT_REUSE: u32 = 131072;
pub const NETSETUP_NO_NETLOGON_CACHE: u32 = 8192;
pub const NETSETUP_PROCESS_OFFLINE_FLAGS: u32 = 17569;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NETSETUP_PROVISIONING_PARAMS {
    pub dwVersion: u32,
    pub lpDomain: windows_core::PCWSTR,
    pub lpHostName: windows_core::PCWSTR,
    pub lpMachineAccountOU: windows_core::PCWSTR,
    pub lpDcName: windows_core::PCWSTR,
    pub dwProvisionOptions: u32,
    pub aCertTemplateNames: *mut windows_core::PCWSTR,
    pub cCertTemplateNames: u32,
    pub aMachinePolicyNames: *mut windows_core::PCWSTR,
    pub cMachinePolicyNames: u32,
    pub aMachinePolicyPaths: *mut windows_core::PCWSTR,
    pub cMachinePolicyPaths: u32,
    pub lpNetbiosName: windows_core::PWSTR,
    pub lpSiteName: windows_core::PWSTR,
    pub lpPrimaryDNSDomain: windows_core::PWSTR,
}
impl Default for NETSETUP_PROVISIONING_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NETSETUP_PROVISIONING_PARAMS_CURRENT_VERSION: u32 = 2;
pub const NETSETUP_PROVISIONING_PARAMS_WIN8_VERSION: u32 = 1;
pub const NETSETUP_PROVISION_CHECK_PWD_ONLY: u32 = 2147483648;
pub const NETSETUP_PROVISION_DOWNLEVEL_PRIV_SUPPORT: u32 = 1;
pub const NETSETUP_PROVISION_ONLINE_CALLER: u32 = 1073741824;
pub const NETSETUP_PROVISION_PERSISTENTSITE: u32 = 32;
pub const NETSETUP_PROVISION_REUSE_ACCOUNT: u32 = 2;
pub const NETSETUP_PROVISION_ROOT_CA_CERTS: u32 = 16;
pub const NETSETUP_PROVISION_SKIP_ACCOUNT_SEARCH: u32 = 8;
pub const NETSETUP_PROVISION_USE_DEFAULT_PASSWORD: u32 = 4;
pub const NETSETUP_SET_MACHINE_NAME: u32 = 32768;
pub const NETSETUP_VALID_UNJOIN_FLAGS: u32 = 268435972;
pub const NETSETUP_WIN9X_UPGRADE: u32 = 16;
pub type NET_COMPUTER_NAME_TYPE = i32;
pub const NET_IGNORE_UNSUPPORTED_FLAGS: u32 = 1;
pub const NetAllComputerNames: NET_COMPUTER_NAME_TYPE = 2;
pub const NetAlternateComputerNames: NET_COMPUTER_NAME_TYPE = 1;
pub const NetComputerNameTypeMax: NET_COMPUTER_NAME_TYPE = 3;
pub const NetPrimaryComputerName: NET_COMPUTER_NAME_TYPE = 0;
pub const NetSetupDnsMachine: NETSETUP_NAME_TYPE = 5;
pub const NetSetupDomain: NETSETUP_NAME_TYPE = 3;
pub const NetSetupDomainName: NETSETUP_JOIN_STATUS = 3;
pub const NetSetupMachine: NETSETUP_NAME_TYPE = 1;
pub const NetSetupNonExistentDomain: NETSETUP_NAME_TYPE = 4;
pub const NetSetupUnjoined: NETSETUP_JOIN_STATUS = 1;
pub const NetSetupUnknown: NETSETUP_NAME_TYPE = 0;
pub const NetSetupUnknownStatus: NETSETUP_JOIN_STATUS = 0;
pub const NetSetupWorkgroup: NETSETUP_NAME_TYPE = 2;
pub const NetSetupWorkgroupName: NETSETUP_JOIN_STATUS = 2;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PDSREG_JOIN_INFO = *mut DSREG_JOIN_INFO;
pub type PDSREG_JOIN_TYPE = *mut DSREG_JOIN_TYPE;
pub type PDSREG_USER_INFO = *mut DSREG_USER_INFO;
pub type PNETSETUP_JOIN_STATUS = *mut NETSETUP_JOIN_STATUS;
pub type PNETSETUP_NAME_TYPE = *mut NETSETUP_NAME_TYPE;
pub type PNETSETUP_PROVISIONING_PARAMS = *mut NETSETUP_PROVISIONING_PARAMS;
pub type PNET_COMPUTER_NAME_TYPE = *mut NET_COMPUTER_NAME_TYPE;
