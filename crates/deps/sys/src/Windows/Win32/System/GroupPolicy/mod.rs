#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn BrowseForGPO(lpbrowseinfo: *mut GPOBROWSEINFO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommandLineFromMsiDescriptor(descriptor: super::super::Foundation::PWSTR, commandline: super::super::Foundation::PWSTR, commandlinelength: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateGPOLink(lpgpo: super::super::Foundation::PWSTR, lpcontainer: super::super::Foundation::PWSTR, fhighpriority: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteAllGPOLinks(lpcontainer: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteGPOLink(lpgpo: super::super::Foundation::PWSTR, lpcontainer: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnterCriticalPolicySection(bmachine: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExportRSoPData(lpnamespace: super::super::Foundation::PWSTR, lpfilename: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeGPOListA(pgpolist: *const GROUP_POLICY_OBJECTA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeGPOListW(pgpolist: *const GROUP_POLICY_OBJECTW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GenerateGPNotification(bmachine: super::super::Foundation::BOOL, lpwszmgmtproduct: super::super::Foundation::PWSTR, dwmgmtproductoptions: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAppliedGPOListA(dwflags: u32, pmachinename: super::super::Foundation::PSTR, psiduser: super::super::Foundation::PSID, pguidextension: *const ::windows_sys::core::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAppliedGPOListW(dwflags: u32, pmachinename: super::super::Foundation::PWSTR, psiduser: super::super::Foundation::PSID, pguidextension: *const ::windows_sys::core::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGPOListA(htoken: super::super::Foundation::HANDLE, lpname: super::super::Foundation::PSTR, lphostname: super::super::Foundation::PSTR, lpcomputername: super::super::Foundation::PSTR, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGPOListW(htoken: super::super::Foundation::HANDLE, lpname: super::super::Foundation::PWSTR, lphostname: super::super::Foundation::PWSTR, lpcomputername: super::super::Foundation::PWSTR, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocalManagedApplicationData(productcode: super::super::Foundation::PWSTR, displayname: *mut super::super::Foundation::PWSTR, supporturl: *mut super::super::Foundation::PWSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocalManagedApplications(buserapps: super::super::Foundation::BOOL, pdwapps: *mut u32, prglocalapps: *mut *mut LOCALMANAGEDAPPLICATION) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    pub fn GetManagedApplicationCategories(dwreserved: u32, pappcategory: *mut super::super::UI::Shell::APPCATEGORYINFOLIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetManagedApplications(pcategory: *const ::windows_sys::core::GUID, dwqueryflags: u32, dwinfolevel: u32, pdwapps: *mut u32, prgmanagedapps: *mut *mut MANAGEDAPPLICATION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImportRSoPData(lpnamespace: super::super::Foundation::PWSTR, lpfilename: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InstallApplication(pinstallinfo: *const INSTALLDATA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LeaveCriticalPolicySection(hsection: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    pub fn ProcessGroupPolicyCompleted(extensionid: *const ::windows_sys::core::GUID, pasynchandle: usize, dwstatus: u32) -> u32;
    pub fn ProcessGroupPolicyCompletedEx(extensionid: *const ::windows_sys::core::GUID, pasynchandle: usize, dwstatus: u32, rsopstatus: ::windows_sys::core::HRESULT) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshPolicy(bmachine: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RefreshPolicyEx(bmachine: super::super::Foundation::BOOL, dwoptions: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterGPNotification(hevent: super::super::Foundation::HANDLE, bmachine: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn RsopAccessCheckByType(
        psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR,
        pprincipalselfsid: super::super::Foundation::PSID,
        prsoptoken: *const ::core::ffi::c_void,
        dwdesiredaccessmask: u32,
        pobjecttypelist: *const super::super::Security::OBJECT_TYPE_LIST,
        objecttypelistlength: u32,
        pgenericmapping: *const super::super::Security::GENERIC_MAPPING,
        pprivilegeset: *const super::super::Security::PRIVILEGE_SET,
        pdwprivilegesetlength: *const u32,
        pdwgrantedaccessmask: *mut u32,
        pbaccessstatus: *mut i32,
    ) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RsopFileAccessCheck(pszfilename: super::super::Foundation::PWSTR, prsoptoken: *const ::core::ffi::c_void, dwdesiredaccessmask: u32, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Wmi")]
    pub fn RsopResetPolicySettingStatus(dwflags: u32, pservices: super::Wmi::IWbemServices, psettinginstance: super::Wmi::IWbemClassObject) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Wmi"))]
    pub fn RsopSetPolicySettingStatus(dwflags: u32, pservices: super::Wmi::IWbemServices, psettinginstance: super::Wmi::IWbemClassObject, ninfo: u32, pstatus: *const POLICYSETTINGSTATUSINFO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UninstallApplication(productcode: super::super::Foundation::PWSTR, dwstatus: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterGPNotification(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
}
#[repr(C)]
pub struct APPSTATE(i32);
pub const CLSID_GPESnapIn: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2411771700, data2: 41185, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
pub const CLSID_GroupPolicyObject: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3931121442, data2: 41533, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
pub const CLSID_RSOPSnapIn: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1841528907,
    data2: 29202,
    data3: 17805,
    data4: [173, 176, 154, 7, 226, 174, 31, 162],
};
#[repr(C)]
pub struct CriticalPolicySectionHandle(i32);
pub const FLAG_ASSUME_COMP_WQLFILTER_TRUE: u32 = 33554432u32;
pub const FLAG_ASSUME_SLOW_LINK: u32 = 536870912u32;
pub const FLAG_ASSUME_USER_WQLFILTER_TRUE: u32 = 67108864u32;
pub const FLAG_FORCE_CREATENAMESPACE: u32 = 4u32;
pub const FLAG_LOOPBACK_MERGE: u32 = 268435456u32;
pub const FLAG_LOOPBACK_REPLACE: u32 = 134217728u32;
pub const FLAG_NO_COMPUTER: u32 = 2u32;
pub const FLAG_NO_CSE_INVOKE: u32 = 1073741824u32;
pub const FLAG_NO_GPO_FILTER: u32 = 2147483648u32;
pub const FLAG_NO_USER: u32 = 1u32;
pub const FLAG_PLANNING_MODE: u32 = 16777216u32;
pub const GPC_BLOCK_POLICY: u32 = 1u32;
#[repr(C)]
pub struct GPM(i32);
#[repr(C)]
pub struct GPMAsyncCancel(i32);
#[repr(C)]
pub struct GPMBackup(i32);
#[repr(C)]
pub struct GPMBackupCollection(i32);
#[repr(C)]
pub struct GPMBackupDir(i32);
#[repr(C)]
pub struct GPMBackupDirEx(i32);
#[repr(C)]
pub struct GPMBackupType(i32);
#[repr(C)]
pub struct GPMCSECollection(i32);
#[repr(C)]
pub struct GPMClientSideExtension(i32);
#[repr(C)]
pub struct GPMConstants(i32);
#[repr(C)]
pub struct GPMDestinationOption(i32);
#[repr(C)]
pub struct GPMDomain(i32);
#[repr(C)]
pub struct GPMEntryType(i32);
#[repr(C)]
pub struct GPMGPO(i32);
#[repr(C)]
pub struct GPMGPOCollection(i32);
#[repr(C)]
pub struct GPMGPOLink(i32);
#[repr(C)]
pub struct GPMGPOLinksCollection(i32);
#[repr(C)]
pub struct GPMMapEntry(i32);
#[repr(C)]
pub struct GPMMapEntryCollection(i32);
#[repr(C)]
pub struct GPMMigrationTable(i32);
#[repr(C)]
pub struct GPMPermission(i32);
#[repr(C)]
pub struct GPMPermissionType(i32);
#[repr(C)]
pub struct GPMRSOP(i32);
#[repr(C)]
pub struct GPMRSOPMode(i32);
#[repr(C)]
pub struct GPMReportType(i32);
#[repr(C)]
pub struct GPMReportingOptions(i32);
#[repr(C)]
pub struct GPMResult(i32);
#[repr(C)]
pub struct GPMSOM(i32);
#[repr(C)]
pub struct GPMSOMCollection(i32);
#[repr(C)]
pub struct GPMSOMType(i32);
#[repr(C)]
pub struct GPMSearchCriteria(i32);
#[repr(C)]
pub struct GPMSearchOperation(i32);
#[repr(C)]
pub struct GPMSearchProperty(i32);
#[repr(C)]
pub struct GPMSecurityInfo(i32);
#[repr(C)]
pub struct GPMSitesContainer(i32);
#[repr(C)]
pub struct GPMStarterGPOBackup(i32);
#[repr(C)]
pub struct GPMStarterGPOBackupCollection(i32);
#[repr(C)]
pub struct GPMStarterGPOCollection(i32);
#[repr(C)]
pub struct GPMStarterGPOType(i32);
#[repr(C)]
pub struct GPMStatusMessage(i32);
#[repr(C)]
pub struct GPMStatusMsgCollection(i32);
#[repr(C)]
pub struct GPMTemplate(i32);
#[repr(C)]
pub struct GPMTrustee(i32);
#[repr(C)]
pub struct GPMWMIFilter(i32);
#[repr(C)]
pub struct GPMWMIFilterCollection(i32);
pub const GPM_DONOTUSE_W2KDC: u32 = 2u32;
pub const GPM_DONOT_VALIDATEDC: u32 = 1u32;
pub const GPM_MIGRATIONTABLE_ONLY: u32 = 1u32;
pub const GPM_PROCESS_SECURITY: u32 = 2u32;
pub const GPM_USE_ANYDC: u32 = 1u32;
pub const GPM_USE_PDC: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GPOBROWSEINFO(i32);
pub const GPO_BROWSE_DISABLENEW: u32 = 1u32;
pub const GPO_BROWSE_INITTOALL: u32 = 16u32;
pub const GPO_BROWSE_NOCOMPUTERS: u32 = 2u32;
pub const GPO_BROWSE_NODSGPOS: u32 = 4u32;
pub const GPO_BROWSE_NOUSERGPOS: u32 = 32u32;
pub const GPO_BROWSE_OPENBUTTON: u32 = 8u32;
pub const GPO_BROWSE_SENDAPPLYONEDIT: u32 = 64u32;
pub const GPO_FLAG_DISABLE: u32 = 1u32;
pub const GPO_FLAG_FORCE: u32 = 2u32;
pub const GPO_INFO_FLAG_ASYNC_FOREGROUND: u32 = 4096u32;
pub const GPO_INFO_FLAG_BACKGROUND: u32 = 16u32;
pub const GPO_INFO_FLAG_FORCED_REFRESH: u32 = 1024u32;
pub const GPO_INFO_FLAG_LINKTRANSITION: u32 = 256u32;
pub const GPO_INFO_FLAG_LOGRSOP_TRANSITION: u32 = 512u32;
pub const GPO_INFO_FLAG_MACHINE: u32 = 1u32;
pub const GPO_INFO_FLAG_NOCHANGES: u32 = 128u32;
pub const GPO_INFO_FLAG_SAFEMODE_BOOT: u32 = 2048u32;
pub const GPO_INFO_FLAG_SLOWLINK: u32 = 32u32;
pub const GPO_INFO_FLAG_VERBOSE: u32 = 64u32;
#[repr(C)]
pub struct GPO_LINK(i32);
pub const GPO_LIST_FLAG_MACHINE: u32 = 1u32;
pub const GPO_LIST_FLAG_NO_SECURITYFILTERS: u32 = 8u32;
pub const GPO_LIST_FLAG_NO_WMIFILTERS: u32 = 4u32;
pub const GPO_LIST_FLAG_SITEONLY: u32 = 2u32;
pub const GPO_OPEN_LOAD_REGISTRY: u32 = 1u32;
pub const GPO_OPEN_READ_ONLY: u32 = 2u32;
pub const GPO_OPTION_DISABLE_MACHINE: u32 = 2u32;
pub const GPO_OPTION_DISABLE_USER: u32 = 1u32;
pub const GPO_SECTION_MACHINE: u32 = 2u32;
pub const GPO_SECTION_ROOT: u32 = 0u32;
pub const GPO_SECTION_USER: u32 = 1u32;
#[repr(C)]
pub struct GROUP_POLICY_HINT_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GROUP_POLICY_OBJECTA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GROUP_POLICY_OBJECTW(i32);
#[repr(C)]
pub struct GROUP_POLICY_OBJECT_TYPE(i32);
#[repr(transparent)]
pub struct IGPEInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPM(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPM2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMAsyncCancel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMAsyncProgress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMBackup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMBackupCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMBackupDir(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMBackupDirEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMCSECollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMClientSideExtension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMConstants(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMConstants2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMDomain(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMDomain2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMDomain3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMGPO(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMGPO2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMGPO3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMGPOCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMGPOLink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMGPOLinksCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMMapEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMMapEntryCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMMigrationTable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMPermission(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMRSOP(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMSOM(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMSOMCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMSearchCriteria(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMSecurityInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMSitesContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMStarterGPO(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMStarterGPOBackup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMStarterGPOBackupCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMStarterGPOCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMStatusMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMStatusMsgCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMTrustee(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMWMIFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGPMWMIFilterCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGroupPolicyObject(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct INSTALLDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct INSTALLSPEC(i32);
#[repr(C)]
pub struct INSTALLSPECTYPE(i32);
#[repr(transparent)]
pub struct IRSOPInformation(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LOCALMANAGEDAPPLICATION(i32);
pub const LOCALSTATE_ASSIGNED: u32 = 1u32;
pub const LOCALSTATE_ORPHANED: u32 = 32u32;
pub const LOCALSTATE_POLICYREMOVE_ORPHAN: u32 = 8u32;
pub const LOCALSTATE_POLICYREMOVE_UNINSTALL: u32 = 16u32;
pub const LOCALSTATE_PUBLISHED: u32 = 2u32;
pub const LOCALSTATE_UNINSTALLED: u32 = 64u32;
pub const LOCALSTATE_UNINSTALL_UNMANAGED: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MANAGEDAPPLICATION(i32);
pub const MANAGED_APPS_FROMCATEGORY: u32 = 2u32;
pub const MANAGED_APPS_INFOLEVEL_DEFAULT: u32 = 65536u32;
pub const MANAGED_APPS_USERAPPLICATIONS: u32 = 1u32;
pub const MANAGED_APPTYPE_SETUPEXE: u32 = 2u32;
pub const MANAGED_APPTYPE_UNSUPPORTED: u32 = 3u32;
pub const MANAGED_APPTYPE_WINDOWSINSTALLER: u32 = 1u32;
pub const NODEID_Machine: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2411771703, data2: 41185, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
pub const NODEID_MachineSWSettings: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2411771706, data2: 41185, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
pub const NODEID_RSOPMachine: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3175881262,
    data2: 2938,
    data3: 19042,
    data4: [166, 176, 192, 87, 117, 57, 201, 126],
};
pub const NODEID_RSOPMachineSWSettings: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1786128190, data2: 60302, data3: 17883, data4: [148, 197, 37, 102, 58, 95, 44, 26] };
pub const NODEID_RSOPUser: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2877765199,
    data2: 3308,
    data3: 19672,
    data4: [155, 248, 137, 143, 52, 98, 143, 184],
};
pub const NODEID_RSOPUserSWSettings: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3844889827,
    data2: 64807,
    data3: 17410,
    data4: [132, 222, 217, 165, 242, 133, 137, 16],
};
pub const NODEID_User: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2411771704, data2: 41185, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
pub const NODEID_UserSWSettings: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2411771708, data2: 41185, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
#[repr(C)]
pub struct PFNGENERATEGROUPPOLICY(i32);
#[repr(C)]
pub struct PFNPROCESSGROUPPOLICY(i32);
#[repr(C)]
pub struct PFNPROCESSGROUPPOLICYEX(i32);
#[repr(C)]
pub struct PFNSTATUSMESSAGECALLBACK(i32);
pub const PI_APPLYPOLICY: u32 = 2u32;
pub const PI_NOUI: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct POLICYSETTINGSTATUSINFO(i32);
pub const PT_MANDATORY: u32 = 4u32;
pub const PT_ROAMING: u32 = 2u32;
pub const PT_ROAMING_PREEXISTING: u32 = 8u32;
pub const PT_TEMPORARY: u32 = 1u32;
pub const RP_FORCE: u32 = 1u32;
pub const RP_SYNC: u32 = 2u32;
pub const RSOP_COMPUTER_ACCESS_DENIED: u32 = 2u32;
pub const RSOP_INFO_FLAG_DIAGNOSTIC_MODE: u32 = 1u32;
pub const RSOP_NO_COMPUTER: u32 = 65536u32;
pub const RSOP_NO_USER: u32 = 131072u32;
pub const RSOP_PLANNING_ASSUME_COMP_WQLFILTER_TRUE: u32 = 16u32;
pub const RSOP_PLANNING_ASSUME_LOOPBACK_MERGE: u32 = 2u32;
pub const RSOP_PLANNING_ASSUME_LOOPBACK_REPLACE: u32 = 4u32;
pub const RSOP_PLANNING_ASSUME_SLOW_LINK: u32 = 1u32;
pub const RSOP_PLANNING_ASSUME_USER_WQLFILTER_TRUE: u32 = 8u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
#[repr(C)]
pub struct RSOP_TARGET(i32);
pub const RSOP_TEMPNAMESPACE_EXISTS: u32 = 4u32;
pub const RSOP_USER_ACCESS_DENIED: u32 = 1u32;
#[repr(C)]
pub struct SETTINGSTATUS(i32);
