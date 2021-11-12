#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct APPSTATE(pub i32);
pub const ABSENT: APPSTATE = APPSTATE(0i32);
pub const ASSIGNED: APPSTATE = APPSTATE(1i32);
pub const PUBLISHED: APPSTATE = APPSTATE(2i32);
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
pub const GPM: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4117317384,
    data2: 35070,
    data3: 19253,
    data4: [186, 191, 229, 97, 98, 213, 251, 200],
};
pub const GPMAsyncCancel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 925341353, data2: 30444, data3: 18333, data4: [173, 108, 85, 99, 24, 237, 95, 157] };
pub const GPMBackup: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3977925816,
    data2: 24314,
    data3: 18474,
    data4: [147, 192, 138, 216, 111, 13, 104, 195],
};
pub const GPMBackupCollection: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3952018267,
    data2: 28891,
    data3: 19103,
    data4: [150, 118, 55, 194, 89, 148, 233, 220],
};
pub const GPMBackupDir: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4242843037, data2: 3873, data3: 19194, data4: [184, 89, 230, 208, 198, 44, 209, 12] };
pub const GPMBackupDirEx: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3904936074,
    data2: 52995,
    data3: 19547,
    data4: [139, 226, 42, 169, 173, 50, 170, 218],
};
#[repr(transparent)]
pub struct GPMBackupType(pub i32);
pub const typeGPO: GPMBackupType = GPMBackupType(0i32);
pub const typeStarterGPO: GPMBackupType = GPMBackupType(1i32);
pub const GPMCSECollection: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3482499112,
    data2: 11588,
    data3: 19297,
    data4: [177, 10, 179, 39, 175, 212, 45, 168],
};
pub const GPMClientSideExtension: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3248678670,
    data2: 26012,
    data3: 19226,
    data4: [148, 11, 248, 139, 10, 249, 200, 164],
};
pub const GPMConstants: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 945154176, data2: 52638, data3: 19724, data4: [158, 175, 21, 121, 40, 58, 24, 136] };
#[repr(transparent)]
pub struct GPMDestinationOption(pub i32);
pub const opDestinationSameAsSource: GPMDestinationOption = GPMDestinationOption(0i32);
pub const opDestinationNone: GPMDestinationOption = GPMDestinationOption(1i32);
pub const opDestinationByRelativeName: GPMDestinationOption = GPMDestinationOption(2i32);
pub const opDestinationSet: GPMDestinationOption = GPMDestinationOption(3i32);
pub const GPMDomain: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1896415678,
    data2: 4176,
    data3: 19633,
    data4: [131, 138, 197, 207, 242, 89, 225, 131],
};
#[repr(transparent)]
pub struct GPMEntryType(pub i32);
pub const typeUser: GPMEntryType = GPMEntryType(0i32);
pub const typeComputer: GPMEntryType = GPMEntryType(1i32);
pub const typeLocalGroup: GPMEntryType = GPMEntryType(2i32);
pub const typeGlobalGroup: GPMEntryType = GPMEntryType(3i32);
pub const typeUniversalGroup: GPMEntryType = GPMEntryType(4i32);
pub const typeUNCPath: GPMEntryType = GPMEntryType(5i32);
pub const typeUnknown: GPMEntryType = GPMEntryType(6i32);
pub const GPMGPO: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3536726420,
    data2: 22965,
    data3: 16484,
    data4: [181, 129, 77, 104, 72, 106, 22, 196],
};
pub const GPMGPOCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2047177509, data2: 33581, data3: 19939, data4: [164, 31, 199, 128, 67, 106, 78, 9] };
pub const GPMGPOLink: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3252656256,
    data2: 21251,
    data3: 17094,
    data4: [138, 60, 4, 136, 225, 191, 115, 100],
};
pub const GPMGPOLinksCollection: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4142749722,
    data2: 18853,
    data3: 18402,
    data4: [183, 113, 253, 141, 192, 43, 98, 89],
};
pub const GPMMapEntry: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2358727251, data2: 21553, data3: 17521, data4: [179, 93, 6, 38, 201, 40, 37, 138] };
pub const GPMMapEntryCollection: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 217537883,
    data2: 41889,
    data3: 19541,
    data4: [180, 254, 158, 20, 156, 65, 246, 109],
};
pub const GPMMigrationTable: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1437548611, data2: 10758, data3: 20338, data4: [171, 239, 99, 27, 68, 7, 156, 118] };
pub const GPMPermission: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1483842570, data2: 59840, data3: 18156, data4: [145, 62, 148, 78, 249, 34, 90, 148] };
#[repr(transparent)]
pub struct GPMPermissionType(pub i32);
pub const permGPOApply: GPMPermissionType = GPMPermissionType(65536i32);
pub const permGPORead: GPMPermissionType = GPMPermissionType(65792i32);
pub const permGPOEdit: GPMPermissionType = GPMPermissionType(65793i32);
pub const permGPOEditSecurityAndDelete: GPMPermissionType = GPMPermissionType(65794i32);
pub const permGPOCustom: GPMPermissionType = GPMPermissionType(65795i32);
pub const permWMIFilterEdit: GPMPermissionType = GPMPermissionType(131072i32);
pub const permWMIFilterFullControl: GPMPermissionType = GPMPermissionType(131073i32);
pub const permWMIFilterCustom: GPMPermissionType = GPMPermissionType(131074i32);
pub const permSOMLink: GPMPermissionType = GPMPermissionType(1835008i32);
pub const permSOMLogging: GPMPermissionType = GPMPermissionType(1573120i32);
pub const permSOMPlanning: GPMPermissionType = GPMPermissionType(1573376i32);
pub const permSOMWMICreate: GPMPermissionType = GPMPermissionType(1049344i32);
pub const permSOMWMIFullControl: GPMPermissionType = GPMPermissionType(1049345i32);
pub const permSOMGPOCreate: GPMPermissionType = GPMPermissionType(1049600i32);
pub const permStarterGPORead: GPMPermissionType = GPMPermissionType(197888i32);
pub const permStarterGPOEdit: GPMPermissionType = GPMPermissionType(197889i32);
pub const permStarterGPOFullControl: GPMPermissionType = GPMPermissionType(197890i32);
pub const permStarterGPOCustom: GPMPermissionType = GPMPermissionType(197891i32);
pub const permSOMStarterGPOCreate: GPMPermissionType = GPMPermissionType(1049856i32);
pub const GPMRSOP: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1218120879,
    data2: 40642,
    data3: 20151,
    data4: [145, 245, 182, 247, 29, 67, 218, 140],
};
#[repr(transparent)]
pub struct GPMRSOPMode(pub i32);
pub const rsopUnknown: GPMRSOPMode = GPMRSOPMode(0i32);
pub const rsopPlanning: GPMRSOPMode = GPMRSOPMode(1i32);
pub const rsopLogging: GPMRSOPMode = GPMRSOPMode(2i32);
#[repr(transparent)]
pub struct GPMReportType(pub i32);
pub const repXML: GPMReportType = GPMReportType(0i32);
pub const repHTML: GPMReportType = GPMReportType(1i32);
pub const repInfraXML: GPMReportType = GPMReportType(2i32);
pub const repInfraRefreshXML: GPMReportType = GPMReportType(3i32);
pub const repClientHealthXML: GPMReportType = GPMReportType(4i32);
pub const repClientHealthRefreshXML: GPMReportType = GPMReportType(5i32);
#[repr(transparent)]
pub struct GPMReportingOptions(pub i32);
pub const opReportLegacy: GPMReportingOptions = GPMReportingOptions(0i32);
pub const opReportComments: GPMReportingOptions = GPMReportingOptions(1i32);
pub const GPMResult: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2450528960,
    data2: 37511,
    data3: 16902,
    data4: [163, 178, 75, 219, 115, 210, 37, 246],
};
pub const GPMSOM: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 853098412,
    data2: 17678,
    data3: 17615,
    data4: [130, 156, 139, 34, 255, 107, 218, 225],
};
pub const GPMSOMCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 616689991, data2: 14112, data3: 20315, data4: [169, 195, 6, 180, 228, 249, 49, 210] };
#[repr(transparent)]
pub struct GPMSOMType(pub i32);
pub const somSite: GPMSOMType = GPMSOMType(0i32);
pub const somDomain: GPMSOMType = GPMSOMType(1i32);
pub const somOU: GPMSOMType = GPMSOMType(2i32);
pub const GPMSearchCriteria: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 397068838, data2: 23776, data3: 17658, data4: [140, 192, 82, 89, 230, 72, 53, 102] };
#[repr(transparent)]
pub struct GPMSearchOperation(pub i32);
pub const opEquals: GPMSearchOperation = GPMSearchOperation(0i32);
pub const opContains: GPMSearchOperation = GPMSearchOperation(1i32);
pub const opNotContains: GPMSearchOperation = GPMSearchOperation(2i32);
pub const opNotEquals: GPMSearchOperation = GPMSearchOperation(3i32);
#[repr(transparent)]
pub struct GPMSearchProperty(pub i32);
pub const gpoPermissions: GPMSearchProperty = GPMSearchProperty(0i32);
pub const gpoEffectivePermissions: GPMSearchProperty = GPMSearchProperty(1i32);
pub const gpoDisplayName: GPMSearchProperty = GPMSearchProperty(2i32);
pub const gpoWMIFilter: GPMSearchProperty = GPMSearchProperty(3i32);
pub const gpoID: GPMSearchProperty = GPMSearchProperty(4i32);
pub const gpoComputerExtensions: GPMSearchProperty = GPMSearchProperty(5i32);
pub const gpoUserExtensions: GPMSearchProperty = GPMSearchProperty(6i32);
pub const somLinks: GPMSearchProperty = GPMSearchProperty(7i32);
pub const gpoDomain: GPMSearchProperty = GPMSearchProperty(8i32);
pub const backupMostRecent: GPMSearchProperty = GPMSearchProperty(9i32);
pub const starterGPOPermissions: GPMSearchProperty = GPMSearchProperty(10i32);
pub const starterGPOEffectivePermissions: GPMSearchProperty = GPMSearchProperty(11i32);
pub const starterGPODisplayName: GPMSearchProperty = GPMSearchProperty(12i32);
pub const starterGPOID: GPMSearchProperty = GPMSearchProperty(13i32);
pub const starterGPODomain: GPMSearchProperty = GPMSearchProperty(14i32);
pub const GPMSecurityInfo: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1417305743,
    data2: 37218,
    data3: 17686,
    data4: [164, 223, 157, 219, 150, 134, 216, 70],
};
pub const GPMSitesContainer: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 580869186,
    data2: 34092,
    data3: 19248,
    data4: [148, 95, 197, 34, 190, 155, 211, 134],
};
pub const GPMStarterGPOBackup: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 949895178, data2: 55535, data3: 17755, data4: [168, 97, 95, 156, 163, 74, 106, 2] };
pub const GPMStarterGPOBackupCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3881739677, data2: 6891, data3: 19637, data4: [167, 138, 40, 29, 170, 88, 36, 6] };
pub const GPMStarterGPOCollection: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2197334667,
    data2: 18874,
    data3: 17330,
    data4: [149, 110, 51, 151, 249, 185, 76, 58],
};
#[repr(transparent)]
pub struct GPMStarterGPOType(pub i32);
pub const typeSystem: GPMStarterGPOType = GPMStarterGPOType(0i32);
pub const typeCustom: GPMStarterGPOType = GPMStarterGPOType(1i32);
pub const GPMStatusMessage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1266142356, data2: 53845, data3: 16539, data4: [188, 98, 55, 8, 129, 113, 90, 25] };
pub const GPMStatusMsgCollection: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 673506494, data2: 19404, data3: 19628, data4: [158, 96, 14, 62, 215, 241, 36, 150] };
pub const GPMTemplate: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3975271508,
    data2: 29146,
    data3: 20015,
    data4: [168, 192, 129, 133, 70, 89, 17, 217],
};
pub const GPMTrustee: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3309989901, data2: 6582, data3: 16913, data4: [188, 176, 232, 226, 71, 94, 71, 30] };
pub const GPMWMIFilter: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1650935256,
    data2: 3562,
    data3: 16482,
    data4: [191, 96, 207, 197, 177, 202, 18, 134],
};
pub const GPMWMIFilterCollection: ::windows_sys::core::GUID = ::windows_sys::GUID {
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
#[repr(transparent)]
pub struct GPO_LINK(pub i32);
pub const GPLinkUnknown: GPO_LINK = GPO_LINK(0i32);
pub const GPLinkMachine: GPO_LINK = GPO_LINK(1i32);
pub const GPLinkSite: GPO_LINK = GPO_LINK(2i32);
pub const GPLinkDomain: GPO_LINK = GPO_LINK(3i32);
pub const GPLinkOrganizationalUnit: GPO_LINK = GPO_LINK(4i32);
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
#[repr(transparent)]
pub struct GROUP_POLICY_HINT_TYPE(pub i32);
pub const GPHintUnknown: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(0i32);
pub const GPHintMachine: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(1i32);
pub const GPHintSite: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(2i32);
pub const GPHintDomain: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(3i32);
pub const GPHintOrganizationalUnit: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(4i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GROUP_POLICY_OBJECTA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GROUP_POLICY_OBJECTW(i32);
#[repr(transparent)]
pub struct GROUP_POLICY_OBJECT_TYPE(pub i32);
pub const GPOTypeLocal: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(0i32);
pub const GPOTypeRemote: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(1i32);
pub const GPOTypeDS: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(2i32);
pub const GPOTypeLocalUser: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(3i32);
pub const GPOTypeLocalGroup: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(4i32);
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
#[repr(transparent)]
pub struct INSTALLSPECTYPE(pub i32);
pub const APPNAME: INSTALLSPECTYPE = INSTALLSPECTYPE(1i32);
pub const FILEEXT: INSTALLSPECTYPE = INSTALLSPECTYPE(2i32);
pub const PROGID: INSTALLSPECTYPE = INSTALLSPECTYPE(3i32);
pub const COMCLASS: INSTALLSPECTYPE = INSTALLSPECTYPE(4i32);
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
pub type PFNGENERATEGROUPPOLICY = unsafe extern "system" fn(dwflags: u32, pbabort: *mut super::super::Foundation::BOOL, pwszsite: super::super::Foundation::PWSTR, pcomputertarget: *const RSOP_TARGET, pusertarget: *const RSOP_TARGET) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PFNPROCESSGROUPPOLICY = unsafe extern "system" fn(dwflags: u32, htoken: super::super::Foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut super::super::Foundation::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_System_Wmi"))]
pub type PFNPROCESSGROUPPOLICYEX = unsafe extern "system" fn(dwflags: u32, htoken: super::super::Foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut super::super::Foundation::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK, pwbemservices: super::Wmi::IWbemServices, prsopstatus: *mut ::windows_sys::core::HRESULT) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNSTATUSMESSAGECALLBACK = unsafe extern "system" fn(bverbose: super::super::Foundation::BOOL, lpmessage: super::super::Foundation::PWSTR) -> u32;
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
#[repr(transparent)]
pub struct SETTINGSTATUS(pub i32);
pub const RSOPUnspecified: SETTINGSTATUS = SETTINGSTATUS(0i32);
pub const RSOPApplied: SETTINGSTATUS = SETTINGSTATUS(1i32);
pub const RSOPIgnored: SETTINGSTATUS = SETTINGSTATUS(2i32);
pub const RSOPFailed: SETTINGSTATUS = SETTINGSTATUS(3i32);
pub const RSOPSubsettingFailed: SETTINGSTATUS = SETTINGSTATUS(4i32);
