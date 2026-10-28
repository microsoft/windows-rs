#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetAddAlternateComputerName<P0, P1, P2, P3>(server: P0, alternatename: P1, domainaccount: P2, domainaccountpassword: P3, reserved: super::ULONG) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetAddAlternateComputerName(server : windows_core::PCWSTR, alternatename : windows_core::PCWSTR, domainaccount : windows_core::PCWSTR, domainaccountpassword : windows_core::PCWSTR, reserved : super::ULONG) -> super::DWORD);
    unsafe { NetAddAlternateComputerName(server.param().abi(), alternatename.param().abi(), domainaccount.param().abi(), domainaccountpassword.param().abi(), reserved) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetCreateProvisioningPackage(pprovisioningparams: PNETSETUP_PROVISIONING_PARAMS, pppackagebindata: Option<*mut super::PBYTE>, pdwpackagebindatasize: Option<*mut super::DWORD>, pppackagetextdata: Option<*mut windows_core::PWSTR>) -> super::DWORD {
    windows_core::link!("netapi32.dll" "system" fn NetCreateProvisioningPackage(pprovisioningparams : PNETSETUP_PROVISIONING_PARAMS, pppackagebindata : *mut super::PBYTE, pdwpackagebindatasize : *mut super::DWORD, pppackagetextdata : *mut windows_core::PWSTR) -> super::DWORD);
    unsafe { NetCreateProvisioningPackage(pprovisioningparams, pppackagebindata.unwrap_or(core::mem::zeroed()) as _, pdwpackagebindatasize.unwrap_or(core::mem::zeroed()) as _, pppackagetextdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetEnumerateComputerNames<P0>(server: P0, nametype: NET_COMPUTER_NAME_TYPE, reserved: super::ULONG, entrycount: super::PDWORD, computernames: *mut *mut windows_core::PWSTR) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetEnumerateComputerNames(server : windows_core::PCWSTR, nametype : NET_COMPUTER_NAME_TYPE, reserved : super::ULONG, entrycount : super::PDWORD, computernames : *mut *mut windows_core::PWSTR) -> super::DWORD);
    unsafe { NetEnumerateComputerNames(server.param().abi(), nametype, reserved, entrycount as _, computernames as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
#[inline]
pub unsafe fn NetFreeAadJoinInformation(pjoininfo: Option<PDSREG_JOIN_INFO>) {
    windows_core::link!("netapi32.dll" "system" fn NetFreeAadJoinInformation(pjoininfo : PDSREG_JOIN_INFO));
    unsafe { NetFreeAadJoinInformation(pjoininfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
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
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetGetJoinInformation<P0>(lpserver: P0, lpnamebuffer: *mut windows_core::PWSTR, buffertype: PNETSETUP_JOIN_STATUS) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGetJoinInformation(lpserver : windows_core::PCWSTR, lpnamebuffer : *mut windows_core::PWSTR, buffertype : PNETSETUP_JOIN_STATUS) -> super::DWORD);
    unsafe { NetGetJoinInformation(lpserver.param().abi(), lpnamebuffer as _, buffertype as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetGetJoinableOUs<P0, P1, P2, P3>(lpserver: P0, lpdomain: P1, lpaccount: P2, lppassword: P3, oucount: *mut super::DWORD, ous: *mut *mut windows_core::PWSTR) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetGetJoinableOUs(lpserver : windows_core::PCWSTR, lpdomain : windows_core::PCWSTR, lpaccount : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, oucount : *mut super::DWORD, ous : *mut *mut windows_core::PWSTR) -> super::DWORD);
    unsafe { NetGetJoinableOUs(lpserver.param().abi(), lpdomain.param().abi(), lpaccount.param().abi(), lppassword.param().abi(), oucount as _, ous as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetJoinDomain<P0, P1, P2, P3, P4>(lpserver: P0, lpdomain: P1, lpmachineaccountou: P2, lpaccount: P3, lppassword: P4, fjoinoptions: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetJoinDomain(lpserver : windows_core::PCWSTR, lpdomain : windows_core::PCWSTR, lpmachineaccountou : windows_core::PCWSTR, lpaccount : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, fjoinoptions : super::DWORD) -> super::DWORD);
    unsafe { NetJoinDomain(lpserver.param().abi(), lpdomain.param().abi(), lpmachineaccountou.param().abi(), lpaccount.param().abi(), lppassword.param().abi(), fjoinoptions) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetProvisionComputerAccount<P0, P1, P2, P3>(lpdomain: P0, lpmachinename: P1, lpmachineaccountou: P2, lpdcname: P3, dwoptions: super::DWORD, pprovisionbindata: Option<*mut super::PBYTE>, pdwprovisionbindatasize: Option<*mut super::DWORD>, pprovisiontextdata: Option<*mut windows_core::PWSTR>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetProvisionComputerAccount(lpdomain : windows_core::PCWSTR, lpmachinename : windows_core::PCWSTR, lpmachineaccountou : windows_core::PCWSTR, lpdcname : windows_core::PCWSTR, dwoptions : super::DWORD, pprovisionbindata : *mut super::PBYTE, pdwprovisionbindatasize : *mut super::DWORD, pprovisiontextdata : *mut windows_core::PWSTR) -> super::DWORD);
    unsafe { NetProvisionComputerAccount(lpdomain.param().abi(), lpmachinename.param().abi(), lpmachineaccountou.param().abi(), lpdcname.param().abi(), dwoptions, pprovisionbindata.unwrap_or(core::mem::zeroed()) as _, pdwprovisionbindatasize.unwrap_or(core::mem::zeroed()) as _, pprovisiontextdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetRemoveAlternateComputerName<P0, P1, P2, P3>(server: P0, alternatename: P1, domainaccount: P2, domainaccountpassword: P3, reserved: super::ULONG) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetRemoveAlternateComputerName(server : windows_core::PCWSTR, alternatename : windows_core::PCWSTR, domainaccount : windows_core::PCWSTR, domainaccountpassword : windows_core::PCWSTR, reserved : super::ULONG) -> super::DWORD);
    unsafe { NetRemoveAlternateComputerName(server.param().abi(), alternatename.param().abi(), domainaccount.param().abi(), domainaccountpassword.param().abi(), reserved) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetRenameMachineInDomain<P0, P1, P2, P3>(lpserver: P0, lpnewmachinename: P1, lpaccount: P2, lppassword: P3, frenameoptions: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetRenameMachineInDomain(lpserver : windows_core::PCWSTR, lpnewmachinename : windows_core::PCWSTR, lpaccount : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, frenameoptions : super::DWORD) -> super::DWORD);
    unsafe { NetRenameMachineInDomain(lpserver.param().abi(), lpnewmachinename.param().abi(), lpaccount.param().abi(), lppassword.param().abi(), frenameoptions) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetRequestOfflineDomainJoin<P3>(pprovisionbindata: &[u8], dwoptions: super::DWORD, lpwindowspath: P3) -> super::DWORD
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetRequestOfflineDomainJoin(pprovisionbindata : *const super::BYTE, cbprovisionbindatasize : super::DWORD, dwoptions : super::DWORD, lpwindowspath : windows_core::PCWSTR) -> super::DWORD);
    unsafe { NetRequestOfflineDomainJoin(core::mem::transmute(pprovisionbindata.as_ptr()), pprovisionbindata.len().try_into().unwrap(), dwoptions, lpwindowspath.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NetRequestProvisioningPackageInstall<P3>(ppackagebindata: &[u8], dwprovisionoptions: super::DWORD, lpwindowspath: P3, pvreserved: Option<super::PVOID>) -> super::DWORD
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetRequestProvisioningPackageInstall(ppackagebindata : *const super::BYTE, dwpackagebindatasize : super::DWORD, dwprovisionoptions : super::DWORD, lpwindowspath : windows_core::PCWSTR, pvreserved : super::PVOID) -> super::DWORD);
    unsafe { NetRequestProvisioningPackageInstall(core::mem::transmute(ppackagebindata.as_ptr()), ppackagebindata.len().try_into().unwrap(), dwprovisionoptions, lpwindowspath.param().abi(), pvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetSetPrimaryComputerName<P0, P1, P2, P3>(server: P0, primaryname: P1, domainaccount: P2, domainaccountpassword: P3, reserved: super::ULONG) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetSetPrimaryComputerName(server : windows_core::PCWSTR, primaryname : windows_core::PCWSTR, domainaccount : windows_core::PCWSTR, domainaccountpassword : windows_core::PCWSTR, reserved : super::ULONG) -> super::DWORD);
    unsafe { NetSetPrimaryComputerName(server.param().abi(), primaryname.param().abi(), domainaccount.param().abi(), domainaccountpassword.param().abi(), reserved) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetUnjoinDomain<P0, P1, P2>(lpserver: P0, lpaccount: P1, lppassword: P2, funjoinoptions: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetUnjoinDomain(lpserver : windows_core::PCWSTR, lpaccount : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, funjoinoptions : super::DWORD) -> super::DWORD);
    unsafe { NetUnjoinDomain(lpserver.param().abi(), lpaccount.param().abi(), lppassword.param().abi(), funjoinoptions) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetValidateName<P0, P1, P2, P3>(lpserver: P0, lpname: P1, lpaccount: P2, lppassword: P3, nametype: NETSETUP_NAME_TYPE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetValidateName(lpserver : windows_core::PCWSTR, lpname : windows_core::PCWSTR, lpaccount : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, nametype : NETSETUP_NAME_TYPE) -> super::DWORD);
    unsafe { NetValidateName(lpserver.param().abi(), lpname.param().abi(), lpaccount.param().abi(), lppassword.param().abi(), nametype) }
}
pub const DSREG_DEVICE_JOIN: DSREG_JOIN_TYPE = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSREG_JOIN_INFO {
    pub joinType: DSREG_JOIN_TYPE,
    pub pJoinCertificate: super::PCCERT_CONTEXT,
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
pub type DSREG_JOIN_TYPE = i32;
pub const DSREG_UNKNOWN_JOIN: DSREG_JOIN_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DSREG_USER_INFO {
    pub pszUserEmail: windows_core::PWSTR,
    pub pszUserKeyId: windows_core::PWSTR,
    pub pszUserKeyName: windows_core::PWSTR,
}
pub const DSREG_WORKPLACE_JOIN: DSREG_JOIN_TYPE = 2;
pub const NETSETUP_ACCT_CREATE: i32 = 2;
pub const NETSETUP_ACCT_DELETE: i32 = 4;
pub const NETSETUP_ALT_SAMACCOUNTNAME: i32 = 131072;
pub const NETSETUP_AMBIGUOUS_DC: i32 = 4096;
pub const NETSETUP_DEFER_SPN_SET: i32 = 256;
pub const NETSETUP_DNS_NAME_CHANGES_ONLY: i32 = 4096;
pub const NETSETUP_DOMAIN_JOIN_IF_JOINED: i32 = 32;
pub const NETSETUP_DONT_CONTROL_SERVICES: i32 = 16384;
pub const NETSETUP_FORCE_SPN_SET: i32 = 65536;
pub const NETSETUP_IGNORE_UNSUPPORTED_FLAGS: i32 = 268435456;
pub const NETSETUP_INSTALL_INVOCATION: i32 = 262144;
pub const NETSETUP_JOIN_DC_ACCOUNT: i32 = 512;
pub const NETSETUP_JOIN_DOMAIN: i32 = 1;
pub const NETSETUP_JOIN_READONLY: i32 = 2048;
pub type NETSETUP_JOIN_STATUS = i32;
pub const NETSETUP_JOIN_UNSECURE: i32 = 64;
pub const NETSETUP_JOIN_WITH_NEW_NAME: i32 = 1024;
pub const NETSETUP_MACHINE_PWD_PASSED: i32 = 128;
pub type NETSETUP_NAME_TYPE = i32;
pub const NETSETUP_NO_ACCT_REUSE: i32 = 131072;
pub const NETSETUP_NO_NETLOGON_CACHE: i32 = 8192;
pub const NETSETUP_PROCESS_OFFLINE_FLAGS: i32 = 17569;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NETSETUP_PROVISIONING_PARAMS {
    pub dwVersion: super::DWORD,
    pub lpDomain: windows_core::PCWSTR,
    pub lpHostName: windows_core::PCWSTR,
    pub lpMachineAccountOU: windows_core::PCWSTR,
    pub lpDcName: windows_core::PCWSTR,
    pub dwProvisionOptions: super::DWORD,
    pub aCertTemplateNames: *mut windows_core::PCWSTR,
    pub cCertTemplateNames: super::DWORD,
    pub aMachinePolicyNames: *mut windows_core::PCWSTR,
    pub cMachinePolicyNames: super::DWORD,
    pub aMachinePolicyPaths: *mut windows_core::PCWSTR,
    pub cMachinePolicyPaths: super::DWORD,
    pub lpNetbiosName: windows_core::PWSTR,
    pub lpSiteName: windows_core::PWSTR,
    pub lpPrimaryDNSDomain: windows_core::PWSTR,
}
pub const NETSETUP_PROVISIONING_PARAMS_CURRENT_VERSION: i32 = 2;
pub const NETSETUP_PROVISIONING_PARAMS_WIN8_VERSION: i32 = 1;
pub const NETSETUP_PROVISION_CHECK_PWD_ONLY: u32 = 2147483648;
pub const NETSETUP_PROVISION_DOWNLEVEL_PRIV_SUPPORT: i32 = 1;
pub const NETSETUP_PROVISION_ONLINE_CALLER: i32 = 1073741824;
pub const NETSETUP_PROVISION_PERSISTENTSITE: i32 = 32;
pub const NETSETUP_PROVISION_REUSE_ACCOUNT: i32 = 2;
pub const NETSETUP_PROVISION_ROOT_CA_CERTS: i32 = 16;
pub const NETSETUP_PROVISION_SKIP_ACCOUNT_SEARCH: i32 = 8;
pub const NETSETUP_PROVISION_USE_DEFAULT_PASSWORD: i32 = 4;
pub const NETSETUP_SET_MACHINE_NAME: i32 = 32768;
pub const NETSETUP_VALID_UNJOIN_FLAGS: i32 = 268435972;
pub const NETSETUP_WIN9X_UPGRADE: i32 = 16;
pub type NET_COMPUTER_NAME_TYPE = i32;
pub const NET_IGNORE_UNSUPPORTED_FLAGS: i32 = 1;
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
pub type PDSREG_JOIN_INFO = *mut DSREG_JOIN_INFO;
pub type PDSREG_JOIN_TYPE = *mut DSREG_JOIN_TYPE;
pub type PDSREG_USER_INFO = *mut DSREG_USER_INFO;
pub type PNETSETUP_JOIN_STATUS = *mut NETSETUP_JOIN_STATUS;
pub type PNETSETUP_NAME_TYPE = *mut NETSETUP_NAME_TYPE;
#[cfg(feature = "shared")]
pub type PNETSETUP_PROVISIONING_PARAMS = *mut NETSETUP_PROVISIONING_PARAMS;
pub type PNET_COMPUTER_NAME_TYPE = *mut NET_COMPUTER_NAME_TYPE;
