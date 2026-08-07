windows_link::link!("netapi32.dll" "system" fn NetAddAlternateComputerName(server : windows_sys::core::PCWSTR, alternatename : windows_sys::core::PCWSTR, domainaccount : windows_sys::core::PCWSTR, domainaccountpassword : windows_sys::core::PCWSTR, reserved : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetCreateProvisioningPackage(pprovisioningparams : *const NETSETUP_PROVISIONING_PARAMS, pppackagebindata : *mut super::PBYTE, pdwpackagebindatasize : *mut u32, pppackagetextdata : *mut windows_sys::core::PWSTR) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetEnumerateComputerNames(server : windows_sys::core::PCWSTR, nametype : NET_COMPUTER_NAME_TYPE, reserved : u32, entrycount : *mut u32, computernames : *mut *mut windows_sys::core::PWSTR) -> u32);
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
windows_link::link!("netapi32.dll" "system" fn NetFreeAadJoinInformation(pjoininfo : *const DSREG_JOIN_INFO));
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
windows_link::link!("netapi32.dll" "system" fn NetGetAadJoinInformation(pcsztenantid : windows_sys::core::PCWSTR, ppjoininfo : *mut PDSREG_JOIN_INFO) -> windows_sys::core::HRESULT);
windows_link::link!("netapi32.dll" "system" fn NetGetJoinInformation(lpserver : windows_sys::core::PCWSTR, lpnamebuffer : *mut windows_sys::core::PWSTR, buffertype : *mut NETSETUP_JOIN_STATUS) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetGetJoinableOUs(lpserver : windows_sys::core::PCWSTR, lpdomain : windows_sys::core::PCWSTR, lpaccount : windows_sys::core::PCWSTR, lppassword : windows_sys::core::PCWSTR, oucount : *mut u32, ous : *mut *mut windows_sys::core::PWSTR) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetJoinDomain(lpserver : windows_sys::core::PCWSTR, lpdomain : windows_sys::core::PCWSTR, lpmachineaccountou : windows_sys::core::PCWSTR, lpaccount : windows_sys::core::PCWSTR, lppassword : windows_sys::core::PCWSTR, fjoinoptions : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetProvisionComputerAccount(lpdomain : windows_sys::core::PCWSTR, lpmachinename : windows_sys::core::PCWSTR, lpmachineaccountou : windows_sys::core::PCWSTR, lpdcname : windows_sys::core::PCWSTR, dwoptions : u32, pprovisionbindata : *mut super::PBYTE, pdwprovisionbindatasize : *mut u32, pprovisiontextdata : *mut windows_sys::core::PWSTR) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetRemoveAlternateComputerName(server : windows_sys::core::PCWSTR, alternatename : windows_sys::core::PCWSTR, domainaccount : windows_sys::core::PCWSTR, domainaccountpassword : windows_sys::core::PCWSTR, reserved : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetRenameMachineInDomain(lpserver : windows_sys::core::PCWSTR, lpnewmachinename : windows_sys::core::PCWSTR, lpaccount : windows_sys::core::PCWSTR, lppassword : windows_sys::core::PCWSTR, frenameoptions : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetRequestOfflineDomainJoin(pprovisionbindata : *const u8, cbprovisionbindatasize : u32, dwoptions : u32, lpwindowspath : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetRequestProvisioningPackageInstall(ppackagebindata : *const u8, dwpackagebindatasize : u32, dwprovisionoptions : u32, lpwindowspath : windows_sys::core::PCWSTR, pvreserved : *const core::ffi::c_void) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetSetPrimaryComputerName(server : windows_sys::core::PCWSTR, primaryname : windows_sys::core::PCWSTR, domainaccount : windows_sys::core::PCWSTR, domainaccountpassword : windows_sys::core::PCWSTR, reserved : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetUnjoinDomain(lpserver : windows_sys::core::PCWSTR, lpaccount : windows_sys::core::PCWSTR, lppassword : windows_sys::core::PCWSTR, funjoinoptions : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetValidateName(lpserver : windows_sys::core::PCWSTR, lpname : windows_sys::core::PCWSTR, lpaccount : windows_sys::core::PCWSTR, lppassword : windows_sys::core::PCWSTR, nametype : NETSETUP_NAME_TYPE) -> u32);
pub const DSREG_DEVICE_JOIN: DSREG_JOIN_TYPE = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy, Default)]
pub struct DSREG_JOIN_INFO {
    pub joinType: DSREG_JOIN_TYPE,
    pub pJoinCertificate: super::PCCERT_CONTEXT,
    pub pszDeviceId: windows_sys::core::PWSTR,
    pub pszIdpDomain: windows_sys::core::PWSTR,
    pub pszTenantId: windows_sys::core::PWSTR,
    pub pszJoinUserEmail: windows_sys::core::PWSTR,
    pub pszTenantDisplayName: windows_sys::core::PWSTR,
    pub pszMdmEnrollmentUrl: windows_sys::core::PWSTR,
    pub pszMdmTermsOfUseUrl: windows_sys::core::PWSTR,
    pub pszMdmComplianceUrl: windows_sys::core::PWSTR,
    pub pszUserSettingSyncUrl: windows_sys::core::PWSTR,
    pub pUserInfo: *mut DSREG_USER_INFO,
}
pub type DSREG_JOIN_TYPE = i32;
pub const DSREG_UNKNOWN_JOIN: DSREG_JOIN_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DSREG_USER_INFO {
    pub pszUserEmail: windows_sys::core::PWSTR,
    pub pszUserKeyId: windows_sys::core::PWSTR,
    pub pszUserKeyName: windows_sys::core::PWSTR,
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
#[derive(Clone, Copy, Default)]
pub struct NETSETUP_PROVISIONING_PARAMS {
    pub dwVersion: u32,
    pub lpDomain: windows_sys::core::PCWSTR,
    pub lpHostName: windows_sys::core::PCWSTR,
    pub lpMachineAccountOU: windows_sys::core::PCWSTR,
    pub lpDcName: windows_sys::core::PCWSTR,
    pub dwProvisionOptions: u32,
    pub aCertTemplateNames: *mut windows_sys::core::PCWSTR,
    pub cCertTemplateNames: u32,
    pub aMachinePolicyNames: *mut windows_sys::core::PCWSTR,
    pub cMachinePolicyNames: u32,
    pub aMachinePolicyPaths: *mut windows_sys::core::PCWSTR,
    pub cMachinePolicyPaths: u32,
    pub lpNetbiosName: windows_sys::core::PWSTR,
    pub lpSiteName: windows_sys::core::PWSTR,
    pub lpPrimaryDNSDomain: windows_sys::core::PWSTR,
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
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PDSREG_JOIN_INFO = *mut DSREG_JOIN_INFO;
pub type PDSREG_JOIN_TYPE = *mut DSREG_JOIN_TYPE;
pub type PDSREG_USER_INFO = *mut DSREG_USER_INFO;
pub type PNETSETUP_JOIN_STATUS = *mut NETSETUP_JOIN_STATUS;
pub type PNETSETUP_NAME_TYPE = *mut NETSETUP_NAME_TYPE;
pub type PNETSETUP_PROVISIONING_PARAMS = *mut NETSETUP_PROVISIONING_PARAMS;
pub type PNET_COMPUTER_NAME_TYPE = *mut NET_COMPUTER_NAME_TYPE;
