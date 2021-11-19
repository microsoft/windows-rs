#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
pub type APPSTATE = i32;
pub const ABSENT: APPSTATE = 0i32;
pub const ASSIGNED: APPSTATE = 1i32;
pub const PUBLISHED: APPSTATE = 2i32;
pub const CLSID_GPESnapIn: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2411771700, data2: 41185, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
pub const CLSID_GroupPolicyObject: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3931121442, data2: 41533, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
pub const CLSID_RSOPSnapIn: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1841528907,
    data2: 29202,
    data3: 17805,
    data4: [173, 176, 154, 7, 226, 174, 31, 162],
};
pub type CriticalPolicySectionHandle = isize;
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
pub const GPM: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4117317384,
    data2: 35070,
    data3: 19253,
    data4: [186, 191, 229, 97, 98, 213, 251, 200],
};
pub const GPMAsyncCancel: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 925341353, data2: 30444, data3: 18333, data4: [173, 108, 85, 99, 24, 237, 95, 157] };
pub const GPMBackup: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3977925816,
    data2: 24314,
    data3: 18474,
    data4: [147, 192, 138, 216, 111, 13, 104, 195],
};
pub const GPMBackupCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3952018267,
    data2: 28891,
    data3: 19103,
    data4: [150, 118, 55, 194, 89, 148, 233, 220],
};
pub const GPMBackupDir: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4242843037, data2: 3873, data3: 19194, data4: [184, 89, 230, 208, 198, 44, 209, 12] };
pub const GPMBackupDirEx: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3904936074,
    data2: 52995,
    data3: 19547,
    data4: [139, 226, 42, 169, 173, 50, 170, 218],
};
pub type GPMBackupType = i32;
pub const typeGPO: GPMBackupType = 0i32;
pub const typeStarterGPO: GPMBackupType = 1i32;
pub const GPMCSECollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3482499112,
    data2: 11588,
    data3: 19297,
    data4: [177, 10, 179, 39, 175, 212, 45, 168],
};
pub const GPMClientSideExtension: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3248678670,
    data2: 26012,
    data3: 19226,
    data4: [148, 11, 248, 139, 10, 249, 200, 164],
};
pub const GPMConstants: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 945154176, data2: 52638, data3: 19724, data4: [158, 175, 21, 121, 40, 58, 24, 136] };
pub type GPMDestinationOption = i32;
pub const opDestinationSameAsSource: GPMDestinationOption = 0i32;
pub const opDestinationNone: GPMDestinationOption = 1i32;
pub const opDestinationByRelativeName: GPMDestinationOption = 2i32;
pub const opDestinationSet: GPMDestinationOption = 3i32;
pub const GPMDomain: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1896415678,
    data2: 4176,
    data3: 19633,
    data4: [131, 138, 197, 207, 242, 89, 225, 131],
};
pub type GPMEntryType = i32;
pub const typeUser: GPMEntryType = 0i32;
pub const typeComputer: GPMEntryType = 1i32;
pub const typeLocalGroup: GPMEntryType = 2i32;
pub const typeGlobalGroup: GPMEntryType = 3i32;
pub const typeUniversalGroup: GPMEntryType = 4i32;
pub const typeUNCPath: GPMEntryType = 5i32;
pub const typeUnknown: GPMEntryType = 6i32;
pub const GPMGPO: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3536726420,
    data2: 22965,
    data3: 16484,
    data4: [181, 129, 77, 104, 72, 106, 22, 196],
};
pub const GPMGPOCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2047177509, data2: 33581, data3: 19939, data4: [164, 31, 199, 128, 67, 106, 78, 9] };
pub const GPMGPOLink: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3252656256,
    data2: 21251,
    data3: 17094,
    data4: [138, 60, 4, 136, 225, 191, 115, 100],
};
pub const GPMGPOLinksCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4142749722,
    data2: 18853,
    data3: 18402,
    data4: [183, 113, 253, 141, 192, 43, 98, 89],
};
pub const GPMMapEntry: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2358727251, data2: 21553, data3: 17521, data4: [179, 93, 6, 38, 201, 40, 37, 138] };
pub const GPMMapEntryCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 217537883,
    data2: 41889,
    data3: 19541,
    data4: [180, 254, 158, 20, 156, 65, 246, 109],
};
pub const GPMMigrationTable: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1437548611, data2: 10758, data3: 20338, data4: [171, 239, 99, 27, 68, 7, 156, 118] };
pub const GPMPermission: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1483842570, data2: 59840, data3: 18156, data4: [145, 62, 148, 78, 249, 34, 90, 148] };
pub type GPMPermissionType = i32;
pub const permGPOApply: GPMPermissionType = 65536i32;
pub const permGPORead: GPMPermissionType = 65792i32;
pub const permGPOEdit: GPMPermissionType = 65793i32;
pub const permGPOEditSecurityAndDelete: GPMPermissionType = 65794i32;
pub const permGPOCustom: GPMPermissionType = 65795i32;
pub const permWMIFilterEdit: GPMPermissionType = 131072i32;
pub const permWMIFilterFullControl: GPMPermissionType = 131073i32;
pub const permWMIFilterCustom: GPMPermissionType = 131074i32;
pub const permSOMLink: GPMPermissionType = 1835008i32;
pub const permSOMLogging: GPMPermissionType = 1573120i32;
pub const permSOMPlanning: GPMPermissionType = 1573376i32;
pub const permSOMWMICreate: GPMPermissionType = 1049344i32;
pub const permSOMWMIFullControl: GPMPermissionType = 1049345i32;
pub const permSOMGPOCreate: GPMPermissionType = 1049600i32;
pub const permStarterGPORead: GPMPermissionType = 197888i32;
pub const permStarterGPOEdit: GPMPermissionType = 197889i32;
pub const permStarterGPOFullControl: GPMPermissionType = 197890i32;
pub const permStarterGPOCustom: GPMPermissionType = 197891i32;
pub const permSOMStarterGPOCreate: GPMPermissionType = 1049856i32;
pub const GPMRSOP: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1218120879,
    data2: 40642,
    data3: 20151,
    data4: [145, 245, 182, 247, 29, 67, 218, 140],
};
pub type GPMRSOPMode = i32;
pub const rsopUnknown: GPMRSOPMode = 0i32;
pub const rsopPlanning: GPMRSOPMode = 1i32;
pub const rsopLogging: GPMRSOPMode = 2i32;
pub type GPMReportType = i32;
pub const repXML: GPMReportType = 0i32;
pub const repHTML: GPMReportType = 1i32;
pub const repInfraXML: GPMReportType = 2i32;
pub const repInfraRefreshXML: GPMReportType = 3i32;
pub const repClientHealthXML: GPMReportType = 4i32;
pub const repClientHealthRefreshXML: GPMReportType = 5i32;
pub type GPMReportingOptions = i32;
pub const opReportLegacy: GPMReportingOptions = 0i32;
pub const opReportComments: GPMReportingOptions = 1i32;
pub const GPMResult: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2450528960,
    data2: 37511,
    data3: 16902,
    data4: [163, 178, 75, 219, 115, 210, 37, 246],
};
pub const GPMSOM: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 853098412,
    data2: 17678,
    data3: 17615,
    data4: [130, 156, 139, 34, 255, 107, 218, 225],
};
pub const GPMSOMCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 616689991, data2: 14112, data3: 20315, data4: [169, 195, 6, 180, 228, 249, 49, 210] };
pub type GPMSOMType = i32;
pub const somSite: GPMSOMType = 0i32;
pub const somDomain: GPMSOMType = 1i32;
pub const somOU: GPMSOMType = 2i32;
pub const GPMSearchCriteria: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 397068838, data2: 23776, data3: 17658, data4: [140, 192, 82, 89, 230, 72, 53, 102] };
pub type GPMSearchOperation = i32;
pub const opEquals: GPMSearchOperation = 0i32;
pub const opContains: GPMSearchOperation = 1i32;
pub const opNotContains: GPMSearchOperation = 2i32;
pub const opNotEquals: GPMSearchOperation = 3i32;
pub type GPMSearchProperty = i32;
pub const gpoPermissions: GPMSearchProperty = 0i32;
pub const gpoEffectivePermissions: GPMSearchProperty = 1i32;
pub const gpoDisplayName: GPMSearchProperty = 2i32;
pub const gpoWMIFilter: GPMSearchProperty = 3i32;
pub const gpoID: GPMSearchProperty = 4i32;
pub const gpoComputerExtensions: GPMSearchProperty = 5i32;
pub const gpoUserExtensions: GPMSearchProperty = 6i32;
pub const somLinks: GPMSearchProperty = 7i32;
pub const gpoDomain: GPMSearchProperty = 8i32;
pub const backupMostRecent: GPMSearchProperty = 9i32;
pub const starterGPOPermissions: GPMSearchProperty = 10i32;
pub const starterGPOEffectivePermissions: GPMSearchProperty = 11i32;
pub const starterGPODisplayName: GPMSearchProperty = 12i32;
pub const starterGPOID: GPMSearchProperty = 13i32;
pub const starterGPODomain: GPMSearchProperty = 14i32;
pub const GPMSecurityInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1417305743,
    data2: 37218,
    data3: 17686,
    data4: [164, 223, 157, 219, 150, 134, 216, 70],
};
pub const GPMSitesContainer: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 580869186,
    data2: 34092,
    data3: 19248,
    data4: [148, 95, 197, 34, 190, 155, 211, 134],
};
pub const GPMStarterGPOBackup: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 949895178, data2: 55535, data3: 17755, data4: [168, 97, 95, 156, 163, 74, 106, 2] };
pub const GPMStarterGPOBackupCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3881739677, data2: 6891, data3: 19637, data4: [167, 138, 40, 29, 170, 88, 36, 6] };
pub const GPMStarterGPOCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2197334667,
    data2: 18874,
    data3: 17330,
    data4: [149, 110, 51, 151, 249, 185, 76, 58],
};
pub type GPMStarterGPOType = i32;
pub const typeSystem: GPMStarterGPOType = 0i32;
pub const typeCustom: GPMStarterGPOType = 1i32;
pub const GPMStatusMessage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1266142356, data2: 53845, data3: 16539, data4: [188, 98, 55, 8, 129, 113, 90, 25] };
pub const GPMStatusMsgCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 673506494, data2: 19404, data3: 19628, data4: [158, 96, 14, 62, 215, 241, 36, 150] };
pub const GPMTemplate: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3975271508,
    data2: 29146,
    data3: 20015,
    data4: [168, 192, 129, 133, 70, 89, 17, 217],
};
pub const GPMTrustee: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3309989901, data2: 6582, data3: 16913, data4: [188, 176, 232, 226, 71, 94, 71, 30] };
pub const GPMWMIFilter: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1650935256,
    data2: 3562,
    data3: 16482,
    data4: [191, 96, 207, 197, 177, 202, 18, 134],
};
pub const GPMWMIFilterCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1960602920,
    data2: 59424,
    data3: 18390,
    data4: [160, 184, 240, 141, 147, 215, 250, 51],
};
pub const GPM_DONOTUSE_W2KDC: u32 = 2u32;
pub const GPM_DONOT_VALIDATEDC: u32 = 1u32;
pub const GPM_MIGRATIONTABLE_ONLY: u32 = 1u32;
pub const GPM_PROCESS_SECURITY: u32 = 2u32;
pub const GPM_USE_ANYDC: u32 = 1u32;
pub const GPM_USE_PDC: u32 = 0u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GPOBROWSEINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpTitle: super::super::Foundation::PWSTR,
    pub lpInitialOU: super::super::Foundation::PWSTR,
    pub lpDSPath: super::super::Foundation::PWSTR,
    pub dwDSPathSize: u32,
    pub lpName: super::super::Foundation::PWSTR,
    pub dwNameSize: u32,
    pub gpoType: GROUP_POLICY_OBJECT_TYPE,
    pub gpoHint: GROUP_POLICY_HINT_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GPOBROWSEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GPOBROWSEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
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
pub type GPO_LINK = i32;
pub const GPLinkUnknown: GPO_LINK = 0i32;
pub const GPLinkMachine: GPO_LINK = 1i32;
pub const GPLinkSite: GPO_LINK = 2i32;
pub const GPLinkDomain: GPO_LINK = 3i32;
pub const GPLinkOrganizationalUnit: GPO_LINK = 4i32;
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
pub type GROUP_POLICY_HINT_TYPE = i32;
pub const GPHintUnknown: GROUP_POLICY_HINT_TYPE = 0i32;
pub const GPHintMachine: GROUP_POLICY_HINT_TYPE = 1i32;
pub const GPHintSite: GROUP_POLICY_HINT_TYPE = 2i32;
pub const GPHintDomain: GROUP_POLICY_HINT_TYPE = 3i32;
pub const GPHintOrganizationalUnit: GROUP_POLICY_HINT_TYPE = 4i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUP_POLICY_OBJECTA {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: super::super::Foundation::PSTR,
    pub lpFileSysPath: super::super::Foundation::PSTR,
    pub lpDisplayName: super::super::Foundation::PSTR,
    pub szGPOName: [super::super::Foundation::CHAR; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::super::Foundation::LPARAM,
    pub pNext: *mut GROUP_POLICY_OBJECTA,
    pub pPrev: *mut GROUP_POLICY_OBJECTA,
    pub lpExtensions: super::super::Foundation::PSTR,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lpLink: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUP_POLICY_OBJECTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUP_POLICY_OBJECTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUP_POLICY_OBJECTW {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: super::super::Foundation::PWSTR,
    pub lpFileSysPath: super::super::Foundation::PWSTR,
    pub lpDisplayName: super::super::Foundation::PWSTR,
    pub szGPOName: [u16; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::super::Foundation::LPARAM,
    pub pNext: *mut GROUP_POLICY_OBJECTW,
    pub pPrev: *mut GROUP_POLICY_OBJECTW,
    pub lpExtensions: super::super::Foundation::PWSTR,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lpLink: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUP_POLICY_OBJECTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUP_POLICY_OBJECTW {
    fn clone(&self) -> Self {
        *self
    }
}
pub type GROUP_POLICY_OBJECT_TYPE = i32;
pub const GPOTypeLocal: GROUP_POLICY_OBJECT_TYPE = 0i32;
pub const GPOTypeRemote: GROUP_POLICY_OBJECT_TYPE = 1i32;
pub const GPOTypeDS: GROUP_POLICY_OBJECT_TYPE = 2i32;
pub const GPOTypeLocalUser: GROUP_POLICY_OBJECT_TYPE = 3i32;
pub const GPOTypeLocalGroup: GROUP_POLICY_OBJECT_TYPE = 4i32;
pub type IGPEInformation = *mut ::core::ffi::c_void;
pub type IGPM = *mut ::core::ffi::c_void;
pub type IGPM2 = *mut ::core::ffi::c_void;
pub type IGPMAsyncCancel = *mut ::core::ffi::c_void;
pub type IGPMAsyncProgress = *mut ::core::ffi::c_void;
pub type IGPMBackup = *mut ::core::ffi::c_void;
pub type IGPMBackupCollection = *mut ::core::ffi::c_void;
pub type IGPMBackupDir = *mut ::core::ffi::c_void;
pub type IGPMBackupDirEx = *mut ::core::ffi::c_void;
pub type IGPMCSECollection = *mut ::core::ffi::c_void;
pub type IGPMClientSideExtension = *mut ::core::ffi::c_void;
pub type IGPMConstants = *mut ::core::ffi::c_void;
pub type IGPMConstants2 = *mut ::core::ffi::c_void;
pub type IGPMDomain = *mut ::core::ffi::c_void;
pub type IGPMDomain2 = *mut ::core::ffi::c_void;
pub type IGPMDomain3 = *mut ::core::ffi::c_void;
pub type IGPMGPO = *mut ::core::ffi::c_void;
pub type IGPMGPO2 = *mut ::core::ffi::c_void;
pub type IGPMGPO3 = *mut ::core::ffi::c_void;
pub type IGPMGPOCollection = *mut ::core::ffi::c_void;
pub type IGPMGPOLink = *mut ::core::ffi::c_void;
pub type IGPMGPOLinksCollection = *mut ::core::ffi::c_void;
pub type IGPMMapEntry = *mut ::core::ffi::c_void;
pub type IGPMMapEntryCollection = *mut ::core::ffi::c_void;
pub type IGPMMigrationTable = *mut ::core::ffi::c_void;
pub type IGPMPermission = *mut ::core::ffi::c_void;
pub type IGPMRSOP = *mut ::core::ffi::c_void;
pub type IGPMResult = *mut ::core::ffi::c_void;
pub type IGPMSOM = *mut ::core::ffi::c_void;
pub type IGPMSOMCollection = *mut ::core::ffi::c_void;
pub type IGPMSearchCriteria = *mut ::core::ffi::c_void;
pub type IGPMSecurityInfo = *mut ::core::ffi::c_void;
pub type IGPMSitesContainer = *mut ::core::ffi::c_void;
pub type IGPMStarterGPO = *mut ::core::ffi::c_void;
pub type IGPMStarterGPOBackup = *mut ::core::ffi::c_void;
pub type IGPMStarterGPOBackupCollection = *mut ::core::ffi::c_void;
pub type IGPMStarterGPOCollection = *mut ::core::ffi::c_void;
pub type IGPMStatusMessage = *mut ::core::ffi::c_void;
pub type IGPMStatusMsgCollection = *mut ::core::ffi::c_void;
pub type IGPMTrustee = *mut ::core::ffi::c_void;
pub type IGPMWMIFilter = *mut ::core::ffi::c_void;
pub type IGPMWMIFilterCollection = *mut ::core::ffi::c_void;
pub type IGroupPolicyObject = *mut ::core::ffi::c_void;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INSTALLDATA {
    pub Type: INSTALLSPECTYPE,
    pub Spec: INSTALLSPEC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INSTALLDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INSTALLDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union INSTALLSPEC {
    pub AppName: INSTALLSPEC_0,
    pub FileExt: super::super::Foundation::PWSTR,
    pub ProgId: super::super::Foundation::PWSTR,
    pub COMClass: INSTALLSPEC_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INSTALLSPEC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INSTALLSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INSTALLSPEC_0 {
    pub Name: super::super::Foundation::PWSTR,
    pub GPOId: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INSTALLSPEC_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INSTALLSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INSTALLSPEC_1 {
    pub Clsid: ::windows_sys::core::GUID,
    pub ClsCtx: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INSTALLSPEC_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INSTALLSPEC_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type INSTALLSPECTYPE = i32;
pub const APPNAME: INSTALLSPECTYPE = 1i32;
pub const FILEEXT: INSTALLSPECTYPE = 2i32;
pub const PROGID: INSTALLSPECTYPE = 3i32;
pub const COMCLASS: INSTALLSPECTYPE = 4i32;
pub type IRSOPInformation = *mut ::core::ffi::c_void;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct LOCALMANAGEDAPPLICATION {
    pub pszDeploymentName: super::super::Foundation::PWSTR,
    pub pszPolicyName: super::super::Foundation::PWSTR,
    pub pszProductId: super::super::Foundation::PWSTR,
    pub dwState: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LOCALMANAGEDAPPLICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LOCALMANAGEDAPPLICATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const LOCALSTATE_ASSIGNED: u32 = 1u32;
pub const LOCALSTATE_ORPHANED: u32 = 32u32;
pub const LOCALSTATE_POLICYREMOVE_ORPHAN: u32 = 8u32;
pub const LOCALSTATE_POLICYREMOVE_UNINSTALL: u32 = 16u32;
pub const LOCALSTATE_PUBLISHED: u32 = 2u32;
pub const LOCALSTATE_UNINSTALLED: u32 = 64u32;
pub const LOCALSTATE_UNINSTALL_UNMANAGED: u32 = 4u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MANAGEDAPPLICATION {
    pub pszPackageName: super::super::Foundation::PWSTR,
    pub pszPublisher: super::super::Foundation::PWSTR,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwRevision: u32,
    pub GpoId: ::windows_sys::core::GUID,
    pub pszPolicyName: super::super::Foundation::PWSTR,
    pub ProductId: ::windows_sys::core::GUID,
    pub Language: u16,
    pub pszOwner: super::super::Foundation::PWSTR,
    pub pszCompany: super::super::Foundation::PWSTR,
    pub pszComments: super::super::Foundation::PWSTR,
    pub pszContact: super::super::Foundation::PWSTR,
    pub pszSupportUrl: super::super::Foundation::PWSTR,
    pub dwPathType: u32,
    pub bInstalled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MANAGEDAPPLICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MANAGEDAPPLICATION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MANAGED_APPS_FROMCATEGORY: u32 = 2u32;
pub const MANAGED_APPS_INFOLEVEL_DEFAULT: u32 = 65536u32;
pub const MANAGED_APPS_USERAPPLICATIONS: u32 = 1u32;
pub const MANAGED_APPTYPE_SETUPEXE: u32 = 2u32;
pub const MANAGED_APPTYPE_UNSUPPORTED: u32 = 3u32;
pub const MANAGED_APPTYPE_WINDOWSINSTALLER: u32 = 1u32;
pub const NODEID_Machine: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2411771703, data2: 41185, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
pub const NODEID_MachineSWSettings: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2411771706, data2: 41185, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
pub const NODEID_RSOPMachine: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3175881262,
    data2: 2938,
    data3: 19042,
    data4: [166, 176, 192, 87, 117, 57, 201, 126],
};
pub const NODEID_RSOPMachineSWSettings: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1786128190, data2: 60302, data3: 17883, data4: [148, 197, 37, 102, 58, 95, 44, 26] };
pub const NODEID_RSOPUser: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2877765199,
    data2: 3308,
    data3: 19672,
    data4: [155, 248, 137, 143, 52, 98, 143, 184],
};
pub const NODEID_RSOPUserSWSettings: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3844889827,
    data2: 64807,
    data3: 17410,
    data4: [132, 222, 217, 165, 242, 133, 137, 16],
};
pub const NODEID_User: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2411771704, data2: 41185, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
pub const NODEID_UserSWSettings: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2411771708, data2: 41185, data3: 4561, data4: [167, 211, 0, 0, 248, 117, 113, 227] };
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
pub type PFNGENERATEGROUPPOLICY = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, pbabort: *mut super::super::Foundation::BOOL, pwszsite: super::super::Foundation::PWSTR, pcomputertarget: *const RSOP_TARGET, pusertarget: *const RSOP_TARGET) -> u32>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PFNPROCESSGROUPPOLICY = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, htoken: super::super::Foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut super::super::Foundation::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK) -> u32>;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_System_Wmi"))]
pub type PFNPROCESSGROUPPOLICYEX = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, htoken: super::super::Foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut super::super::Foundation::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK, pwbemservices: super::Wmi::IWbemServices, prsopstatus: *mut ::windows_sys::core::HRESULT) -> u32>;
#[cfg(feature = "Win32_Foundation")]
pub type PFNSTATUSMESSAGECALLBACK = ::core::option::Option<unsafe extern "system" fn(bverbose: super::super::Foundation::BOOL, lpmessage: super::super::Foundation::PWSTR) -> u32>;
pub const PI_APPLYPOLICY: u32 = 2u32;
pub const PI_NOUI: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct POLICYSETTINGSTATUSINFO {
    pub szKey: super::super::Foundation::PWSTR,
    pub szEventSource: super::super::Foundation::PWSTR,
    pub szEventLogName: super::super::Foundation::PWSTR,
    pub dwEventID: u32,
    pub dwErrorCode: u32,
    pub status: SETTINGSTATUS,
    pub timeLogged: super::super::Foundation::SYSTEMTIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for POLICYSETTINGSTATUSINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for POLICYSETTINGSTATUSINFO {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
pub struct RSOP_TARGET {
    pub pwszAccountName: super::super::Foundation::PWSTR,
    pub pwszNewSOM: super::super::Foundation::PWSTR,
    pub psaSecurityGroups: *mut super::Com::SAFEARRAY,
    pub pRsopToken: *mut ::core::ffi::c_void,
    pub pGPOList: *mut GROUP_POLICY_OBJECTA,
    pub pWbemServices: super::Wmi::IWbemServices,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl ::core::marker::Copy for RSOP_TARGET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl ::core::clone::Clone for RSOP_TARGET {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RSOP_TEMPNAMESPACE_EXISTS: u32 = 4u32;
pub const RSOP_USER_ACCESS_DENIED: u32 = 1u32;
pub type SETTINGSTATUS = i32;
pub const RSOPUnspecified: SETTINGSTATUS = 0i32;
pub const RSOPApplied: SETTINGSTATUS = 1i32;
pub const RSOPIgnored: SETTINGSTATUS = 2i32;
pub const RSOPFailed: SETTINGSTATUS = 3i32;
pub const RSOPSubsettingFailed: SETTINGSTATUS = 4i32;
