#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn CreateAppContainerProfile(pszappcontainername : windows_sys::core::PCWSTR, pszdisplayname : windows_sys::core::PCWSTR, pszdescription : windows_sys::core::PCWSTR, pcapabilities : *const super::winnt::SID_AND_ATTRIBUTES, dwcapabilitycount : u32, ppsidappcontainersid : *mut super::winnt::PSID) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn CreateEnvironmentBlock(lpenvironment : *mut *mut core::ffi::c_void, htoken : super::winnt::HANDLE, binherit : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("userenv.dll" "system" fn CreateProfile(pszusersid : windows_sys::core::PCWSTR, pszusername : windows_sys::core::PCWSTR, pszprofilepath : windows_sys::core::PWSTR, cchprofilepath : u32) -> windows_sys::core::HRESULT);
windows_link::link!("userenv.dll" "system" fn DeleteAppContainerProfile(pszappcontainername : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("userenv.dll" "system" fn DeleteProfileA(lpsidstring : windows_sys::core::PCSTR, lpprofilepath : windows_sys::core::PCSTR, lpcomputername : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("userenv.dll" "system" fn DeleteProfileW(lpsidstring : windows_sys::core::PCWSTR, lpprofilepath : windows_sys::core::PCWSTR, lpcomputername : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn DeriveAppContainerSidFromAppContainerName(pszappcontainername : windows_sys::core::PCWSTR, ppsidappcontainersid : *mut super::winnt::PSID) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid : super::winnt::PSID, pszrestrictedappcontainername : windows_sys::core::PCWSTR, ppsidrestrictedappcontainersid : *mut super::winnt::PSID) -> windows_sys::core::HRESULT);
windows_link::link!("userenv.dll" "system" fn DestroyEnvironmentBlock(lpenvironment : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn EnterCriticalPolicySection(bmachine : windows_sys::core::BOOL) -> super::winnt::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn ExpandEnvironmentStringsForUserA(htoken : super::winnt::HANDLE, lpsrc : windows_sys::core::PCSTR, lpdest : windows_sys::core::PSTR, dwsize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn ExpandEnvironmentStringsForUserW(htoken : super::winnt::HANDLE, lpsrc : windows_sys::core::PCWSTR, lpdest : windows_sys::core::PWSTR, dwsize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("userenv.dll" "system" fn FreeGPOListA(pgpolist : *const GROUP_POLICY_OBJECTA) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("userenv.dll" "system" fn FreeGPOListW(pgpolist : *const GROUP_POLICY_OBJECTW) -> windows_sys::core::BOOL);
windows_link::link!("userenv.dll" "system" fn GenerateGPNotification(bmachine : windows_sys::core::BOOL, lpwszmgmtproduct : windows_sys::core::PCWSTR, dwmgmtproductoptions : u32) -> u32);
windows_link::link!("userenv.dll" "system" fn GetAllUsersProfileDirectoryA(lpprofiledir : windows_sys::core::PSTR, lpcchsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("userenv.dll" "system" fn GetAllUsersProfileDirectoryW(lpprofiledir : windows_sys::core::PWSTR, lpcchsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("userenv.dll" "system" fn GetAppContainerFolderPath(pszappcontainersid : windows_sys::core::PCWSTR, ppszpath : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
windows_link::link!("userenv.dll" "system" fn GetAppContainerRegistryLocation(desiredaccess : super::winreg::REGSAM, phappcontainerkey : *mut super::minwindef::HKEY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn GetAppliedGPOListA(dwflags : u32, pmachinename : windows_sys::core::PCSTR, psiduser : super::winnt::PSID, pguidextension : *const windows_sys::core::GUID, ppgpolist : *mut PGROUP_POLICY_OBJECTA) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn GetAppliedGPOListW(dwflags : u32, pmachinename : windows_sys::core::PCWSTR, psiduser : super::winnt::PSID, pguidextension : *const windows_sys::core::GUID, ppgpolist : *mut PGROUP_POLICY_OBJECTW) -> u32);
windows_link::link!("userenv.dll" "system" fn GetDefaultUserProfileDirectoryA(lpprofiledir : windows_sys::core::PSTR, lpcchsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("userenv.dll" "system" fn GetDefaultUserProfileDirectoryW(lpprofiledir : windows_sys::core::PWSTR, lpcchsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn GetGPOListA(htoken : super::winnt::HANDLE, lpname : windows_sys::core::PCSTR, lphostname : windows_sys::core::PCSTR, lpcomputername : windows_sys::core::PCSTR, dwflags : u32, pgpolist : *mut PGROUP_POLICY_OBJECTA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn GetGPOListW(htoken : super::winnt::HANDLE, lpname : windows_sys::core::PCWSTR, lphostname : windows_sys::core::PCWSTR, lpcomputername : windows_sys::core::PCWSTR, dwflags : u32, pgpolist : *mut PGROUP_POLICY_OBJECTW) -> windows_sys::core::BOOL);
windows_link::link!("userenv.dll" "system" fn GetProfileType(dwflags : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("userenv.dll" "system" fn GetProfilesDirectoryA(lpprofiledir : windows_sys::core::PSTR, lpcchsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("userenv.dll" "system" fn GetProfilesDirectoryW(lpprofiledir : windows_sys::core::PWSTR, lpcchsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn GetUserProfileDirectoryA(htoken : super::winnt::HANDLE, lpprofiledir : windows_sys::core::PSTR, lpcchsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn GetUserProfileDirectoryW(htoken : super::winnt::HANDLE, lpprofiledir : windows_sys::core::PWSTR, lpcchsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn LeaveCriticalPolicySection(hsection : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "profinfo", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn LoadUserProfileA(htoken : super::winnt::HANDLE, lpprofileinfo : *mut super::profinfo::PROFILEINFOA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "profinfo", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn LoadUserProfileW(htoken : super::winnt::HANDLE, lpprofileinfo : *mut super::profinfo::PROFILEINFOW) -> windows_sys::core::BOOL);
windows_link::link!("userenv.dll" "system" fn ProcessGroupPolicyCompleted(extensionid : *const windows_sys::core::GUID, pasynchandle : ASYNCCOMPLETIONHANDLE, dwstatus : u32) -> u32);
windows_link::link!("userenv.dll" "system" fn ProcessGroupPolicyCompletedEx(extensionid : *const windows_sys::core::GUID, pasynchandle : ASYNCCOMPLETIONHANDLE, dwstatus : u32, rsopstatus : windows_sys::core::HRESULT) -> u32);
windows_link::link!("userenv.dll" "system" fn RefreshPolicy(bmachine : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("userenv.dll" "system" fn RefreshPolicyEx(bmachine : windows_sys::core::BOOL, dwoptions : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn RegisterGPNotification(hevent : super::winnt::HANDLE, bmachine : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn RsopAccessCheckByType(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, pprincipalselfsid : super::winnt::PSID, prsoptoken : PRSOPTOKEN, dwdesiredaccessmask : u32, pobjecttypelist : *const super::winnt::OBJECT_TYPE_LIST, objecttypelistlength : u32, pgenericmapping : *const super::winnt::GENERIC_MAPPING, pprivilegeset : *const super::winnt::PRIVILEGE_SET, pdwprivilegesetlength : *const u32, pdwgrantedaccessmask : *mut u32, pbaccessstatus : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("userenv.dll" "system" fn RsopFileAccessCheck(pszfilename : windows_sys::core::PCWSTR, prsoptoken : PRSOPTOKEN, dwdesiredaccessmask : u32, pdwgrantedaccessmask : *mut u32, pbaccessstatus : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "wbemcli")]
windows_link::link!("userenv.dll" "system" fn RsopResetPolicySettingStatus(dwflags : u32, pservices : *mut core::ffi::c_void, psettinginstance : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "wbemcli"))]
windows_link::link!("userenv.dll" "system" fn RsopSetPolicySettingStatus(dwflags : u32, pservices : *mut core::ffi::c_void, psettinginstance : *mut core::ffi::c_void, ninfo : u32, pstatus : *const POLICYSETTINGSTATUSINFO) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn UnloadUserProfile(htoken : super::winnt::HANDLE, hprofile : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn UnregisterGPNotification(hevent : super::winnt::HANDLE) -> windows_sys::core::BOOL);
pub type ASYNCCOMPLETIONHANDLE = usize;
pub const FLAG_ASSUME_COMP_WQLFILTER_TRUE: u32 = 33554432;
pub const FLAG_ASSUME_SLOW_LINK: u32 = 536870912;
pub const FLAG_ASSUME_USER_WQLFILTER_TRUE: u32 = 67108864;
pub const FLAG_FORCE_CREATENAMESPACE: u32 = 4;
pub const FLAG_LOOPBACK_MERGE: u32 = 268435456;
pub const FLAG_LOOPBACK_REPLACE: u32 = 134217728;
pub const FLAG_NO_COMPUTER: u32 = 2;
pub const FLAG_NO_CSE_INVOKE: u32 = 1073741824;
pub const FLAG_NO_GPO_FILTER: u32 = 2147483648;
pub const FLAG_NO_USER: u32 = 1;
pub const FLAG_PLANNING_MODE: u32 = 16777216;
pub const GPC_BLOCK_POLICY: u32 = 1;
pub const GPLinkDomain: GPO_LINK = 3;
pub const GPLinkMachine: GPO_LINK = 1;
pub const GPLinkOrganizationalUnit: GPO_LINK = 4;
pub const GPLinkSite: GPO_LINK = 2;
pub const GPLinkUnknown: GPO_LINK = 0;
pub const GPO_FLAG_DISABLE: u32 = 1;
pub const GPO_FLAG_FORCE: u32 = 2;
pub const GPO_INFO_FLAG_ASYNC_FOREGROUND: u32 = 4096;
pub const GPO_INFO_FLAG_BACKGROUND: u32 = 16;
pub const GPO_INFO_FLAG_FORCED_REFRESH: u32 = 1024;
pub const GPO_INFO_FLAG_LINKTRANSITION: u32 = 256;
pub const GPO_INFO_FLAG_LOGRSOP_TRANSITION: u32 = 512;
pub const GPO_INFO_FLAG_MACHINE: u32 = 1;
pub const GPO_INFO_FLAG_NOCHANGES: u32 = 128;
pub const GPO_INFO_FLAG_SAFEMODE_BOOT: u32 = 2048;
pub const GPO_INFO_FLAG_SLOWLINK: u32 = 32;
pub const GPO_INFO_FLAG_VERBOSE: u32 = 64;
pub type GPO_LINK = i32;
pub const GPO_LIST_FLAG_MACHINE: u32 = 1;
pub const GPO_LIST_FLAG_NO_SECURITYFILTERS: u32 = 8;
pub const GPO_LIST_FLAG_NO_WMIFILTERS: u32 = 4;
pub const GPO_LIST_FLAG_SITEONLY: u32 = 2;
#[cfg(feature = "minwindef")]
pub type GROUP_POLICY_OBJECT = GROUP_POLICY_OBJECTA;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct GROUP_POLICY_OBJECTA {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: windows_sys::core::PSTR,
    pub lpFileSysPath: windows_sys::core::PSTR,
    pub lpDisplayName: windows_sys::core::PSTR,
    pub szGPOName: [i8; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::minwindef::LPARAM,
    pub pNext: *mut Self,
    pub pPrev: *mut Self,
    pub lpExtensions: windows_sys::core::PSTR,
    pub lParam2: super::minwindef::LPARAM,
    pub lpLink: windows_sys::core::PSTR,
}
#[cfg(feature = "minwindef")]
impl Default for GROUP_POLICY_OBJECTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct GROUP_POLICY_OBJECTW {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: windows_sys::core::PWSTR,
    pub lpFileSysPath: windows_sys::core::PWSTR,
    pub lpDisplayName: windows_sys::core::PWSTR,
    pub szGPOName: [u16; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::minwindef::LPARAM,
    pub pNext: *mut Self,
    pub pPrev: *mut Self,
    pub lpExtensions: windows_sys::core::PWSTR,
    pub lParam2: super::minwindef::LPARAM,
    pub lpLink: windows_sys::core::PWSTR,
}
#[cfg(feature = "minwindef")]
impl Default for GROUP_POLICY_OBJECTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwinbase")]
pub type LPPOLICYSETTINGSTATUSINFO = *mut POLICYSETTINGSTATUSINFO;
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wbemcli"))]
pub type PFNGENERATEGROUPPOLICY = Option<unsafe extern "system" fn(dwflags: u32, pbabort: *mut windows_sys::core::BOOL, pwszsite: *const u16, pcomputertarget: *const RSOP_TARGET, pusertarget: *const RSOP_TARGET) -> u32>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PFNPROCESSGROUPPOLICY = Option<unsafe extern "system" fn(dwflags: u32, htoken: super::winnt::HANDLE, hkeyroot: super::minwindef::HKEY, pdeletedgpolist: PGROUP_POLICY_OBJECT, pchangedgpolist: PGROUP_POLICY_OBJECT, phandle: ASYNCCOMPLETIONHANDLE, pbabort: *mut windows_sys::core::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK) -> u32>;
#[cfg(all(feature = "minwindef", feature = "wbemcli", feature = "winnt"))]
pub type PFNPROCESSGROUPPOLICYEX = Option<unsafe extern "system" fn(dwflags: u32, htoken: super::winnt::HANDLE, hkeyroot: super::minwindef::HKEY, pdeletedgpolist: PGROUP_POLICY_OBJECT, pchangedgpolist: PGROUP_POLICY_OBJECT, phandle: ASYNCCOMPLETIONHANDLE, pbabort: *mut windows_sys::core::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK, pwbemservices: *mut core::ffi::c_void, prsopstatus: *mut windows_sys::core::HRESULT) -> u32>;
pub type PFNSTATUSMESSAGECALLBACK = Option<unsafe extern "system" fn(bverbose: windows_sys::core::BOOL, lpmessage: windows_sys::core::PCWSTR) -> u32>;
pub type PGPO_LINK = *mut GPO_LINK;
#[cfg(feature = "minwindef")]
pub type PGROUP_POLICY_OBJECT = PGROUP_POLICY_OBJECTA;
#[cfg(feature = "minwindef")]
pub type PGROUP_POLICY_OBJECTA = *mut GROUP_POLICY_OBJECTA;
#[cfg(feature = "minwindef")]
pub type PGROUP_POLICY_OBJECTW = *mut GROUP_POLICY_OBJECTW;
pub const PI_APPLYPOLICY: u32 = 2;
pub const PI_NOUI: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwinbase")]
#[derive(Clone, Copy)]
pub struct POLICYSETTINGSTATUSINFO {
    pub szKey: windows_sys::core::PWSTR,
    pub szEventSource: windows_sys::core::PWSTR,
    pub szEventLogName: windows_sys::core::PWSTR,
    pub dwEventID: u32,
    pub dwErrorCode: u32,
    pub status: SETTINGSTATUS,
    pub timeLogged: super::minwinbase::SYSTEMTIME,
}
#[cfg(feature = "minwinbase")]
impl Default for POLICYSETTINGSTATUSINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PRSOPTOKEN = *mut core::ffi::c_void;
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wbemcli"))]
pub type PRSOP_TARGET = *mut RSOP_TARGET;
pub const PT_MANDATORY: u32 = 4;
pub const PT_ROAMING: u32 = 2;
pub const PT_ROAMING_PREEXISTING: u32 = 8;
pub const PT_TEMPORARY: u32 = 1;
pub type REFGPEXTENSIONID = *mut windows_sys::core::GUID;
pub const RP_FORCE: u32 = 1;
pub const RP_SYNC: u32 = 2;
pub const RSOPApplied: SETTINGSTATUS = 1;
pub const RSOPFailed: SETTINGSTATUS = 3;
pub const RSOPIgnored: SETTINGSTATUS = 2;
pub const RSOPSubsettingFailed: SETTINGSTATUS = 4;
pub const RSOPUnspecified: SETTINGSTATUS = 0;
pub const RSOP_COMPUTER_ACCESS_DENIED: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wbemcli"))]
#[derive(Clone, Copy)]
pub struct RSOP_TARGET {
    pub pwszAccountName: *mut u16,
    pub pwszNewSOM: *mut u16,
    pub psaSecurityGroups: *mut super::oaidl::SAFEARRAY,
    pub pRsopToken: PRSOPTOKEN,
    pub pGPOList: PGROUP_POLICY_OBJECT,
    pub pWbemServices: *mut core::ffi::c_void,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wbemcli"))]
impl Default for RSOP_TARGET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RSOP_TEMPNAMESPACE_EXISTS: u32 = 4;
pub const RSOP_USER_ACCESS_DENIED: u32 = 1;
pub type SETTINGSTATUS = i32;
