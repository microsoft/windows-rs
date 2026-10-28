#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn CreateAppContainerProfile(pszappcontainername : windows_sys::core::PCWSTR, pszdisplayname : windows_sys::core::PCWSTR, pszdescription : windows_sys::core::PCWSTR, pcapabilities : super::PSID_AND_ATTRIBUTES, dwcapabilitycount : super::DWORD, ppsidappcontainersid : *mut super::PSID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn CreateEnvironmentBlock(lpenvironment : *mut super::LPVOID, htoken : super::HANDLE, binherit : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("userenv.dll" "system" fn CreateProfile(pszusersid : windows_sys::core::PCWSTR, pszusername : windows_sys::core::PCWSTR, pszprofilepath : windows_sys::core::PWSTR, cchprofilepath : super::DWORD) -> windows_sys::core::HRESULT);
windows_link::link!("userenv.dll" "system" fn DeleteAppContainerProfile(pszappcontainername : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("userenv.dll" "system" fn DeleteProfileA(lpsidstring : windows_sys::core::PCSTR, lpprofilepath : windows_sys::core::PCSTR, lpcomputername : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("userenv.dll" "system" fn DeleteProfileW(lpsidstring : windows_sys::core::PCWSTR, lpprofilepath : windows_sys::core::PCWSTR, lpcomputername : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn DeriveAppContainerSidFromAppContainerName(pszappcontainername : windows_sys::core::PCWSTR, ppsidappcontainersid : *mut super::PSID) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid : super::PSID, pszrestrictedappcontainername : windows_sys::core::PCWSTR, ppsidrestrictedappcontainersid : *mut super::PSID) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("userenv.dll" "system" fn DestroyEnvironmentBlock(lpenvironment : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn EnterCriticalPolicySection(bmachine : windows_sys::core::BOOL) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn ExpandEnvironmentStringsForUserA(htoken : super::HANDLE, lpsrc : windows_sys::core::PCSTR, lpdest : windows_sys::core::PSTR, dwsize : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn ExpandEnvironmentStringsForUserW(htoken : super::HANDLE, lpsrc : windows_sys::core::PCWSTR, lpdest : windows_sys::core::PWSTR, dwsize : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("userenv.dll" "system" fn FreeGPOListA(pgpolist : PGROUP_POLICY_OBJECTA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn FreeGPOListW(pgpolist : PGROUP_POLICY_OBJECTW) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("userenv.dll" "system" fn GenerateGPNotification(bmachine : windows_sys::core::BOOL, lpwszmgmtproduct : windows_sys::core::PCWSTR, dwmgmtproductoptions : super::DWORD) -> super::DWORD);
#[cfg(feature = "minwindef")]
windows_link::link!("userenv.dll" "system" fn GetAllUsersProfileDirectoryA(lpprofiledir : windows_sys::core::PSTR, lpcchsize : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("userenv.dll" "system" fn GetAllUsersProfileDirectoryW(lpprofiledir : windows_sys::core::PWSTR, lpcchsize : super::LPDWORD) -> windows_sys::core::BOOL);
windows_link::link!("userenv.dll" "system" fn GetAppContainerFolderPath(pszappcontainersid : windows_sys::core::PCWSTR, ppszpath : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "winreg"))]
windows_link::link!("userenv.dll" "system" fn GetAppContainerRegistryLocation(desiredaccess : super::REGSAM, phappcontainerkey : super::PHKEY) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn GetAppliedGPOListA(dwflags : super::DWORD, pmachinename : windows_sys::core::PCSTR, psiduser : super::PSID, pguidextension : *const windows_sys::core::GUID, ppgpolist : *mut PGROUP_POLICY_OBJECTA) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn GetAppliedGPOListW(dwflags : super::DWORD, pmachinename : windows_sys::core::PCWSTR, psiduser : super::PSID, pguidextension : *const windows_sys::core::GUID, ppgpolist : *mut PGROUP_POLICY_OBJECTW) -> super::DWORD);
#[cfg(feature = "minwindef")]
windows_link::link!("userenv.dll" "system" fn GetDefaultUserProfileDirectoryA(lpprofiledir : windows_sys::core::PSTR, lpcchsize : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("userenv.dll" "system" fn GetDefaultUserProfileDirectoryW(lpprofiledir : windows_sys::core::PWSTR, lpcchsize : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn GetGPOListA(htoken : super::HANDLE, lpname : windows_sys::core::PCSTR, lphostname : windows_sys::core::PCSTR, lpcomputername : windows_sys::core::PCSTR, dwflags : super::DWORD, pgpolist : *mut PGROUP_POLICY_OBJECTA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn GetGPOListW(htoken : super::HANDLE, lpname : windows_sys::core::PCWSTR, lphostname : windows_sys::core::PCWSTR, lpcomputername : windows_sys::core::PCWSTR, dwflags : super::DWORD, pgpolist : *mut PGROUP_POLICY_OBJECTW) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("userenv.dll" "system" fn GetProfileType(dwflags : *mut super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("userenv.dll" "system" fn GetProfilesDirectoryA(lpprofiledir : windows_sys::core::PSTR, lpcchsize : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("userenv.dll" "system" fn GetProfilesDirectoryW(lpprofiledir : windows_sys::core::PWSTR, lpcchsize : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn GetUserProfileDirectoryA(htoken : super::HANDLE, lpprofiledir : windows_sys::core::PSTR, lpcchsize : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn GetUserProfileDirectoryW(htoken : super::HANDLE, lpprofiledir : windows_sys::core::PWSTR, lpcchsize : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn LeaveCriticalPolicySection(hsection : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "profinfo", feature = "shared", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn LoadUserProfileA(htoken : super::HANDLE, lpprofileinfo : super::LPPROFILEINFOA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "profinfo", feature = "shared", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn LoadUserProfileW(htoken : super::HANDLE, lpprofileinfo : super::LPPROFILEINFOW) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("userenv.dll" "system" fn ProcessGroupPolicyCompleted(extensionid : REFGPEXTENSIONID, pasynchandle : ASYNCCOMPLETIONHANDLE, dwstatus : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("userenv.dll" "system" fn ProcessGroupPolicyCompletedEx(extensionid : REFGPEXTENSIONID, pasynchandle : ASYNCCOMPLETIONHANDLE, dwstatus : super::DWORD, rsopstatus : windows_sys::core::HRESULT) -> super::DWORD);
windows_link::link!("userenv.dll" "system" fn RefreshPolicy(bmachine : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("userenv.dll" "system" fn RefreshPolicyEx(bmachine : windows_sys::core::BOOL, dwoptions : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn RegisterGPNotification(hevent : super::HANDLE, bmachine : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("userenv.dll" "system" fn RsopAccessCheckByType(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, pprincipalselfsid : super::PSID, prsoptoken : PRSOPTOKEN, dwdesiredaccessmask : super::DWORD, pobjecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : super::DWORD, pgenericmapping : super::PGENERIC_MAPPING, pprivilegeset : super::PPRIVILEGE_SET, pdwprivilegesetlength : super::LPDWORD, pdwgrantedaccessmask : super::LPDWORD, pbaccessstatus : super::LPBOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("userenv.dll" "system" fn RsopFileAccessCheck(pszfilename : windows_sys::core::PCWSTR, prsoptoken : PRSOPTOKEN, dwdesiredaccessmask : super::DWORD, pdwgrantedaccessmask : super::LPDWORD, pbaccessstatus : super::LPBOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "wbemcli"))]
windows_link::link!("userenv.dll" "system" fn RsopResetPolicySettingStatus(dwflags : super::DWORD, pservices : *mut core::ffi::c_void, psettinginstance : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "wbemcli"))]
windows_link::link!("userenv.dll" "system" fn RsopSetPolicySettingStatus(dwflags : super::DWORD, pservices : *mut core::ffi::c_void, psettinginstance : *mut core::ffi::c_void, ninfo : super::DWORD, pstatus : *const POLICYSETTINGSTATUSINFO) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn UnloadUserProfile(htoken : super::HANDLE, hprofile : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("userenv.dll" "system" fn UnregisterGPNotification(hevent : super::HANDLE) -> windows_sys::core::BOOL);
pub type ASYNCCOMPLETIONHANDLE = usize;
pub const FLAG_ASSUME_COMP_WQLFILTER_TRUE: i32 = 33554432;
pub const FLAG_ASSUME_SLOW_LINK: i32 = 536870912;
pub const FLAG_ASSUME_USER_WQLFILTER_TRUE: i32 = 67108864;
pub const FLAG_FORCE_CREATENAMESPACE: i32 = 4;
pub const FLAG_LOOPBACK_MERGE: i32 = 268435456;
pub const FLAG_LOOPBACK_REPLACE: i32 = 134217728;
pub const FLAG_NO_COMPUTER: i32 = 2;
pub const FLAG_NO_CSE_INVOKE: i32 = 1073741824;
pub const FLAG_NO_GPO_FILTER: u32 = 2147483648;
pub const FLAG_NO_USER: i32 = 1;
pub const FLAG_PLANNING_MODE: i32 = 16777216;
pub const GPC_BLOCK_POLICY: i32 = 1;
pub const GPLinkDomain: GPO_LINK = 3;
pub const GPLinkMachine: GPO_LINK = 1;
pub const GPLinkOrganizationalUnit: GPO_LINK = 4;
pub const GPLinkSite: GPO_LINK = 2;
pub const GPLinkUnknown: GPO_LINK = 0;
pub const GPO_FLAG_DISABLE: i32 = 1;
pub const GPO_FLAG_FORCE: i32 = 2;
pub const GPO_INFO_FLAG_ASYNC_FOREGROUND: i32 = 4096;
pub const GPO_INFO_FLAG_BACKGROUND: i32 = 16;
pub const GPO_INFO_FLAG_FORCED_REFRESH: i32 = 1024;
pub const GPO_INFO_FLAG_LINKTRANSITION: i32 = 256;
pub const GPO_INFO_FLAG_LOGRSOP_TRANSITION: i32 = 512;
pub const GPO_INFO_FLAG_MACHINE: i32 = 1;
pub const GPO_INFO_FLAG_NOCHANGES: i32 = 128;
pub const GPO_INFO_FLAG_SAFEMODE_BOOT: i32 = 2048;
pub const GPO_INFO_FLAG_SLOWLINK: i32 = 32;
pub const GPO_INFO_FLAG_VERBOSE: i32 = 64;
pub type GPO_LINK = i32;
pub const GPO_LIST_FLAG_MACHINE: i32 = 1;
pub const GPO_LIST_FLAG_NO_SECURITYFILTERS: i32 = 8;
pub const GPO_LIST_FLAG_NO_WMIFILTERS: i32 = 4;
pub const GPO_LIST_FLAG_SITEONLY: i32 = 2;
pub const GP_DLLNAME: windows_sys::core::PCSTR = windows_sys::core::s!("DllName");
pub const GP_ENABLEASYNCHRONOUSPROCESSING: windows_sys::core::PCSTR = windows_sys::core::s!("EnableAsynchronousProcessing");
pub const GP_MAXNOGPOLISTCHANGESINTERVAL: windows_sys::core::PCSTR = windows_sys::core::s!("MaxNoGPOListChangesInterval");
pub const GP_NOBACKGROUNDPOLICY: windows_sys::core::PCSTR = windows_sys::core::s!("NoBackgroundPolicy");
pub const GP_NOGPOLISTCHANGES: windows_sys::core::PCSTR = windows_sys::core::s!("NoGPOListChanges");
pub const GP_NOMACHINEPOLICY: windows_sys::core::PCSTR = windows_sys::core::s!("NoMachinePolicy");
pub const GP_NOSLOWLINK: windows_sys::core::PCSTR = windows_sys::core::s!("NoSlowLink");
pub const GP_NOTIFYLINKTRANSITION: windows_sys::core::PCSTR = windows_sys::core::s!("NotifyLinkTransition");
pub const GP_NOUSERPOLICY: windows_sys::core::PCSTR = windows_sys::core::s!("NoUserPolicy");
pub const GP_PERUSERLOCALSETTINGS: windows_sys::core::PCSTR = windows_sys::core::s!("PerUserLocalSettings");
pub const GP_PROCESSGROUPPOLICY: windows_sys::core::PCSTR = windows_sys::core::s!("ProcessGroupPolicy");
pub const GP_REQUIRESSUCCESSFULREGISTRY: windows_sys::core::PCSTR = windows_sys::core::s!("RequiresSuccessfulRegistry");
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type GROUP_POLICY_OBJECT = GROUP_POLICY_OBJECTA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct GROUP_POLICY_OBJECTA {
    pub dwOptions: super::DWORD,
    pub dwVersion: super::DWORD,
    pub lpDSPath: windows_sys::core::PSTR,
    pub lpFileSysPath: windows_sys::core::PSTR,
    pub lpDisplayName: windows_sys::core::PSTR,
    pub szGPOName: [super::CHAR; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::LPARAM,
    pub pNext: *mut Self,
    pub pPrev: *mut Self,
    pub lpExtensions: windows_sys::core::PSTR,
    pub lParam2: super::LPARAM,
    pub lpLink: windows_sys::core::PSTR,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for GROUP_POLICY_OBJECTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct GROUP_POLICY_OBJECTW {
    pub dwOptions: super::DWORD,
    pub dwVersion: super::DWORD,
    pub lpDSPath: windows_sys::core::PWSTR,
    pub lpFileSysPath: windows_sys::core::PWSTR,
    pub lpDisplayName: windows_sys::core::PWSTR,
    pub szGPOName: [super::WCHAR; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::LPARAM,
    pub pNext: *mut Self,
    pub pPrev: *mut Self,
    pub lpExtensions: windows_sys::core::PWSTR,
    pub lParam2: super::LPARAM,
    pub lpLink: windows_sys::core::PWSTR,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for GROUP_POLICY_OBJECTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwinbase", feature = "shared"))]
pub type LPPOLICYSETTINGSTATUSINFO = *mut POLICYSETTINGSTATUSINFO;
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "wbemcli", feature = "winnt"))]
pub type PFNGENERATEGROUPPOLICY = Option<unsafe extern "C" fn(dwflags: super::DWORD, pbabort: *mut windows_sys::core::BOOL, pwszsite: *const super::WCHAR, pcomputertarget: PRSOP_TARGET, pusertarget: PRSOP_TARGET) -> super::DWORD>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PFNPROCESSGROUPPOLICY = Option<unsafe extern "C" fn(dwflags: super::DWORD, htoken: super::HANDLE, hkeyroot: super::HKEY, pdeletedgpolist: PGROUP_POLICY_OBJECT, pchangedgpolist: PGROUP_POLICY_OBJECT, phandle: ASYNCCOMPLETIONHANDLE, pbabort: *mut windows_sys::core::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK) -> super::DWORD>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wbemcli", feature = "winnt"))]
pub type PFNPROCESSGROUPPOLICYEX = Option<unsafe extern "C" fn(dwflags: super::DWORD, htoken: super::HANDLE, hkeyroot: super::HKEY, pdeletedgpolist: PGROUP_POLICY_OBJECT, pchangedgpolist: PGROUP_POLICY_OBJECT, phandle: ASYNCCOMPLETIONHANDLE, pbabort: *mut windows_sys::core::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK, pwbemservices: *mut core::ffi::c_void, prsopstatus: *mut windows_sys::core::HRESULT) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type PFNSTATUSMESSAGECALLBACK = Option<unsafe extern "C" fn(bverbose: windows_sys::core::BOOL, lpmessage: windows_sys::core::PCWSTR) -> super::DWORD>;
pub type PGPO_LINK = *mut GPO_LINK;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PGROUP_POLICY_OBJECT = *mut GROUP_POLICY_OBJECTA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PGROUP_POLICY_OBJECTA = *mut GROUP_POLICY_OBJECTA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PGROUP_POLICY_OBJECTW = *mut GROUP_POLICY_OBJECTW;
pub const PI_APPLYPOLICY: i32 = 2;
pub const PI_NOUI: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwinbase", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct POLICYSETTINGSTATUSINFO {
    pub szKey: windows_sys::core::PWSTR,
    pub szEventSource: windows_sys::core::PWSTR,
    pub szEventLogName: windows_sys::core::PWSTR,
    pub dwEventID: super::DWORD,
    pub dwErrorCode: super::DWORD,
    pub status: SETTINGSTATUS,
    pub timeLogged: super::SYSTEMTIME,
}
pub type PRSOPTOKEN = *mut core::ffi::c_void;
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "wbemcli", feature = "winnt"))]
pub type PRSOP_TARGET = *mut RSOP_TARGET;
pub const PT_MANDATORY: i32 = 4;
pub const PT_ROAMING: i32 = 2;
pub const PT_ROAMING_PREEXISTING: i32 = 8;
pub const PT_TEMPORARY: i32 = 1;
pub type REFGPEXTENSIONID = *mut windows_sys::core::GUID;
pub const RP_FORCE: i32 = 1;
pub const RP_SYNC: i32 = 2;
pub const RSOPApplied: SETTINGSTATUS = 1;
pub const RSOPFailed: SETTINGSTATUS = 3;
pub const RSOPIgnored: SETTINGSTATUS = 2;
pub const RSOPSubsettingFailed: SETTINGSTATUS = 4;
pub const RSOPUnspecified: SETTINGSTATUS = 0;
pub const RSOP_COMPUTER_ACCESS_DENIED: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "wbemcli", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct RSOP_TARGET {
    pub pwszAccountName: *mut super::WCHAR,
    pub pwszNewSOM: *mut super::WCHAR,
    pub psaSecurityGroups: *mut super::SAFEARRAY,
    pub pRsopToken: PRSOPTOKEN,
    pub pGPOList: PGROUP_POLICY_OBJECT,
    pub pWbemServices: *mut core::ffi::c_void,
}
pub const RSOP_TEMPNAMESPACE_EXISTS: i32 = 4;
pub const RSOP_USER_ACCESS_DENIED: i32 = 1;
pub type SETTINGSTATUS = i32;
