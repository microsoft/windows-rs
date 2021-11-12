#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ChangeServiceConfig2A(hservice: super::super::Security::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ChangeServiceConfig2W(hservice: super::super::Security::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ChangeServiceConfigA(
        hservice: super::super::Security::SC_HANDLE,
        dwservicetype: u32,
        dwstarttype: SERVICE_START_TYPE,
        dwerrorcontrol: SERVICE_ERROR,
        lpbinarypathname: super::super::Foundation::PSTR,
        lploadordergroup: super::super::Foundation::PSTR,
        lpdwtagid: *mut u32,
        lpdependencies: super::super::Foundation::PSTR,
        lpservicestartname: super::super::Foundation::PSTR,
        lppassword: super::super::Foundation::PSTR,
        lpdisplayname: super::super::Foundation::PSTR,
    ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ChangeServiceConfigW(
        hservice: super::super::Security::SC_HANDLE,
        dwservicetype: u32,
        dwstarttype: SERVICE_START_TYPE,
        dwerrorcontrol: SERVICE_ERROR,
        lpbinarypathname: super::super::Foundation::PWSTR,
        lploadordergroup: super::super::Foundation::PWSTR,
        lpdwtagid: *mut u32,
        lpdependencies: super::super::Foundation::PWSTR,
        lpservicestartname: super::super::Foundation::PWSTR,
        lppassword: super::super::Foundation::PWSTR,
        lpdisplayname: super::super::Foundation::PWSTR,
    ) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CloseServiceHandle(hscobject: super::super::Security::SC_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ControlService(hservice: super::super::Security::SC_HANDLE, dwcontrol: u32, lpservicestatus: *mut SERVICE_STATUS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ControlServiceExA(hservice: super::super::Security::SC_HANDLE, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn ControlServiceExW(hservice: super::super::Security::SC_HANDLE, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateServiceA(
        hscmanager: super::super::Security::SC_HANDLE,
        lpservicename: super::super::Foundation::PSTR,
        lpdisplayname: super::super::Foundation::PSTR,
        dwdesiredaccess: u32,
        dwservicetype: ENUM_SERVICE_TYPE,
        dwstarttype: SERVICE_START_TYPE,
        dwerrorcontrol: SERVICE_ERROR,
        lpbinarypathname: super::super::Foundation::PSTR,
        lploadordergroup: super::super::Foundation::PSTR,
        lpdwtagid: *mut u32,
        lpdependencies: super::super::Foundation::PSTR,
        lpservicestartname: super::super::Foundation::PSTR,
        lppassword: super::super::Foundation::PSTR,
    ) -> super::super::Security::SC_HANDLE;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateServiceW(
        hscmanager: super::super::Security::SC_HANDLE,
        lpservicename: super::super::Foundation::PWSTR,
        lpdisplayname: super::super::Foundation::PWSTR,
        dwdesiredaccess: u32,
        dwservicetype: ENUM_SERVICE_TYPE,
        dwstarttype: SERVICE_START_TYPE,
        dwerrorcontrol: SERVICE_ERROR,
        lpbinarypathname: super::super::Foundation::PWSTR,
        lploadordergroup: super::super::Foundation::PWSTR,
        lpdwtagid: *mut u32,
        lpdependencies: super::super::Foundation::PWSTR,
        lpservicestartname: super::super::Foundation::PWSTR,
        lppassword: super::super::Foundation::PWSTR,
    ) -> super::super::Security::SC_HANDLE;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DeleteService(hservice: super::super::Security::SC_HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn EnumDependentServicesA(hservice: super::super::Security::SC_HANDLE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn EnumDependentServicesW(hservice: super::super::Security::SC_HANDLE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn EnumServicesStatusA(hscmanager: super::super::Security::SC_HANDLE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn EnumServicesStatusExA(hscmanager: super::super::Security::SC_HANDLE, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32, pszgroupname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn EnumServicesStatusExW(hscmanager: super::super::Security::SC_HANDLE, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32, pszgroupname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn EnumServicesStatusW(hscmanager: super::super::Security::SC_HANDLE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetServiceDirectory(hservicestatus: SERVICE_STATUS_HANDLE, edirectorytype: SERVICE_DIRECTORY_TYPE, lppathbuffer: super::super::Foundation::PWSTR, cchpathbufferlength: u32, lpcchrequiredbufferlength: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn GetServiceDisplayNameA(hscmanager: super::super::Security::SC_HANDLE, lpservicename: super::super::Foundation::PSTR, lpdisplayname: super::super::Foundation::PSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn GetServiceDisplayNameW(hscmanager: super::super::Security::SC_HANDLE, lpservicename: super::super::Foundation::PWSTR, lpdisplayname: super::super::Foundation::PWSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn GetServiceKeyNameA(hscmanager: super::super::Security::SC_HANDLE, lpdisplayname: super::super::Foundation::PSTR, lpservicename: super::super::Foundation::PSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn GetServiceKeyNameW(hscmanager: super::super::Security::SC_HANDLE, lpdisplayname: super::super::Foundation::PWSTR, lpservicename: super::super::Foundation::PWSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetServiceRegistryStateKey(servicestatushandle: SERVICE_STATUS_HANDLE, statetype: SERVICE_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn GetSharedServiceDirectory(servicehandle: super::super::Security::SC_HANDLE, directorytype: SERVICE_SHARED_DIRECTORY_TYPE, pathbuffer: super::super::Foundation::PWSTR, pathbufferlength: u32, requiredbufferlength: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Security`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
    pub fn GetSharedServiceRegistryStateKey(servicehandle: super::super::Security::SC_HANDLE, statetype: SERVICE_SHARED_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Security`*"]
    #[cfg(feature = "Win32_Security")]
    pub fn LockServiceDatabase(hscmanager: super::super::Security::SC_HANDLE) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NotifyBootConfigStatus(bootacceptable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NotifyServiceStatusChangeA(hservice: super::super::Security::SC_HANDLE, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const SERVICE_NOTIFY_2A) -> u32;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NotifyServiceStatusChangeW(hservice: super::super::Security::SC_HANDLE, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const SERVICE_NOTIFY_2W) -> u32;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenSCManagerA(lpmachinename: super::super::Foundation::PSTR, lpdatabasename: super::super::Foundation::PSTR, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenSCManagerW(lpmachinename: super::super::Foundation::PWSTR, lpdatabasename: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenServiceA(hscmanager: super::super::Security::SC_HANDLE, lpservicename: super::super::Foundation::PSTR, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn OpenServiceW(hscmanager: super::super::Security::SC_HANDLE, lpservicename: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceConfig2A(hservice: super::super::Security::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceConfig2W(hservice: super::super::Security::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceConfigA(hservice: super::super::Security::SC_HANDLE, lpserviceconfig: *mut QUERY_SERVICE_CONFIGA, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceConfigW(hservice: super::super::Security::SC_HANDLE, lpserviceconfig: *mut QUERY_SERVICE_CONFIGW, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryServiceDynamicInformation(hservicestatus: SERVICE_STATUS_HANDLE, dwinfolevel: u32, ppdynamicinfo: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceLockStatusA(hscmanager: super::super::Security::SC_HANDLE, lplockstatus: *mut QUERY_SERVICE_LOCK_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceLockStatusW(hscmanager: super::super::Security::SC_HANDLE, lplockstatus: *mut QUERY_SERVICE_LOCK_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceObjectSecurity(hservice: super::super::Security::SC_HANDLE, dwsecurityinformation: u32, lpsecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceStatus(hservice: super::super::Security::SC_HANDLE, lpservicestatus: *mut SERVICE_STATUS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn QueryServiceStatusEx(hservice: super::super::Security::SC_HANDLE, infolevel: SC_STATUS_TYPE, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterServiceCtrlHandlerA(lpservicename: super::super::Foundation::PSTR, lphandlerproc: LPHANDLER_FUNCTION) -> SERVICE_STATUS_HANDLE;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterServiceCtrlHandlerExA(lpservicename: super::super::Foundation::PSTR, lphandlerproc: LPHANDLER_FUNCTION_EX, lpcontext: *const ::core::ffi::c_void) -> SERVICE_STATUS_HANDLE;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterServiceCtrlHandlerExW(lpservicename: super::super::Foundation::PWSTR, lphandlerproc: LPHANDLER_FUNCTION_EX, lpcontext: *const ::core::ffi::c_void) -> SERVICE_STATUS_HANDLE;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterServiceCtrlHandlerW(lpservicename: super::super::Foundation::PWSTR, lphandlerproc: LPHANDLER_FUNCTION) -> SERVICE_STATUS_HANDLE;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetServiceBits(hservicestatus: SERVICE_STATUS_HANDLE, dwservicebits: u32, bsetbitson: super::super::Foundation::BOOL, bupdateimmediately: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn SetServiceObjectSecurity(hservice: super::super::Security::SC_HANDLE, dwsecurityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, lpsecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetServiceStatus(hservicestatus: SERVICE_STATUS_HANDLE, lpservicestatus: *const SERVICE_STATUS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn StartServiceA(hservice: super::super::Security::SC_HANDLE, dwnumserviceargs: u32, lpserviceargvectors: *const super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartServiceCtrlDispatcherA(lpservicestarttable: *const SERVICE_TABLE_ENTRYA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartServiceCtrlDispatcherW(lpservicestarttable: *const SERVICE_TABLE_ENTRYW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn StartServiceW(hservice: super::super::Security::SC_HANDLE, dwnumserviceargs: u32, lpserviceargvectors: *const super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnlockServiceDatabase(sclock: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn WaitServiceState(hservice: super::super::Security::SC_HANDLE, dwnotify: u32, dwtimeout: u32, hcancelevent: super::super::Foundation::HANDLE) -> u32;
}
pub const CUSTOM_SYSTEM_STATE_CHANGE_EVENT_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 762980374, data2: 3166, data3: 17916, data4: [156, 231, 87, 14, 94, 205, 233, 201] };
pub const DOMAIN_JOIN_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 484575930, data2: 38993, data3: 17441, data4: [148, 48, 29, 222, 183, 102, 232, 9] };
pub const DOMAIN_LEAVE_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3719254382,
    data2: 22722,
    data3: 18534,
    data4: [149, 116, 195, 182, 21, 212, 46, 161],
};
pub struct ENUM_SERVICE_STATE(i32);
pub struct ENUM_SERVICE_STATUSA(i32);
pub struct ENUM_SERVICE_STATUSW(i32);
pub struct ENUM_SERVICE_STATUS_PROCESSA(i32);
pub struct ENUM_SERVICE_STATUS_PROCESSW(i32);
pub struct ENUM_SERVICE_TYPE(i32);
pub const FIREWALL_PORT_CLOSE_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2705648952,
    data2: 36370,
    data3: 19940,
    data4: [157, 150, 230, 71, 64, 177, 165, 36],
};
pub const FIREWALL_PORT_OPEN_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3075907079, data2: 33825, data3: 20192, data4: [173, 16, 134, 145, 90, 253, 173, 9] };
pub struct HANDLER_FUNCTION(i32);
pub struct HANDLER_FUNCTION_EX(i32);
pub struct LPHANDLER_FUNCTION(i32);
pub struct LPHANDLER_FUNCTION_EX(i32);
pub struct LPSERVICE_MAIN_FUNCTIONA(i32);
pub struct LPSERVICE_MAIN_FUNCTIONW(i32);
pub const MACHINE_POLICY_PRESENT_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1704970982,
    data2: 23515,
    data3: 19881,
    data4: [177, 255, 202, 42, 23, 141, 70, 224],
};
pub const NAMED_PIPE_EVENT_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 528601393, data2: 16300, data3: 17719, data4: [158, 12, 126, 123, 12, 47, 75, 85] };
pub const NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1328018142,
    data2: 5346,
    data3: 17163,
    data4: [165, 73, 124, 212, 140, 188, 130, 69],
};
pub const NETWORK_MANAGER_LAST_IP_ADDRESS_REMOVAL_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3427509802,
    data2: 5678,
    data3: 17992,
    data4: [132, 122, 182, 189, 249, 147, 227, 53],
};
pub struct PFN_SC_NOTIFY_CALLBACK(i32);
pub struct PSC_NOTIFICATION_CALLBACK(i32);
pub struct QUERY_SERVICE_CONFIGA(i32);
pub struct QUERY_SERVICE_CONFIGW(i32);
pub struct QUERY_SERVICE_LOCK_STATUSA(i32);
pub struct QUERY_SERVICE_LOCK_STATUSW(i32);
pub const RPC_INTERFACE_EVENT_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3163607399,
    data2: 38000,
    data3: 16697,
    data4: [169, 186, 190, 11, 187, 245, 183, 77],
};
pub struct SC_ACTION(i32);
pub struct SC_ACTION_TYPE(i32);
pub struct SC_ENUM_TYPE(i32);
pub struct SC_EVENT_TYPE(i32);
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SC_MANAGER_ALL_ACCESS: u32 = 983103u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SC_MANAGER_CONNECT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SC_MANAGER_CREATE_SERVICE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SC_MANAGER_ENUMERATE_SERVICE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SC_MANAGER_LOCK: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SC_MANAGER_MODIFY_BOOT_CONFIG: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SC_MANAGER_QUERY_LOCK_STATUS: u32 = 16u32;
pub struct SC_STATUS_TYPE(i32);
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_HARDWAREPROFILECHANGE: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_LOWRESOURCES: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_NETBINDCHANGE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_PARAMCHANGE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_PAUSE_CONTINUE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_POWEREVENT: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_PRESHUTDOWN: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_SESSIONCHANGE: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_SHUTDOWN: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_STOP: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_SYSTEMLOWRESOURCES: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_TIMECHANGE: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_TRIGGEREVENT: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_USER_LOGOFF: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ALL_ACCESS: u32 = 983551u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CHANGE_CONFIG: u32 = 2u32;
pub struct SERVICE_CONFIG(i32);
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_CONTINUE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_DEVICEEVENT: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_HARDWAREPROFILECHANGE: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_INTERROGATE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_LOWRESOURCES: u32 = 96u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_NETBINDADD: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_NETBINDDISABLE: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_NETBINDENABLE: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_NETBINDREMOVE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_PARAMCHANGE: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_PAUSE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_POWEREVENT: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_PRESHUTDOWN: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_SESSIONCHANGE: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_SHUTDOWN: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_STATUS_REASON_INFO: u32 = 1u32;
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSA(i32);
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSW(i32);
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_STOP: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_SYSTEMLOWRESOURCES: u32 = 97u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_TIMECHANGE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_TRIGGEREVENT: u32 = 32u32;
pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM(i32);
pub struct SERVICE_DELAYED_AUTO_START_INFO(i32);
pub struct SERVICE_DESCRIPTIONA(i32);
pub struct SERVICE_DESCRIPTIONW(i32);
pub struct SERVICE_DIRECTORY_TYPE(i32);
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_DYNAMIC_INFORMATION_LEVEL_START_REASON: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ENUMERATE_DEPENDENTS: u32 = 8u32;
pub struct SERVICE_ERROR(i32);
pub struct SERVICE_FAILURE_ACTIONSA(i32);
pub struct SERVICE_FAILURE_ACTIONSW(i32);
pub struct SERVICE_FAILURE_ACTIONS_FLAG(i32);
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_INTERROGATE: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_LAUNCH_PROTECTED_ANTIMALWARE_LIGHT: u32 = 3u32;
pub struct SERVICE_LAUNCH_PROTECTED_INFO(i32);
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_LAUNCH_PROTECTED_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS_LIGHT: u32 = 2u32;
pub struct SERVICE_MAIN_FUNCTIONA(i32);
pub struct SERVICE_MAIN_FUNCTIONW(i32);
pub struct SERVICE_NOTIFY(i32);
pub struct SERVICE_NOTIFY_1(i32);
pub struct SERVICE_NOTIFY_2A(i32);
pub struct SERVICE_NOTIFY_2W(i32);
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_NOTIFY_STATUS_CHANGE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_NOTIFY_STATUS_CHANGE_1: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_NOTIFY_STATUS_CHANGE_2: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_NO_CHANGE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_PAUSE_CONTINUE: u32 = 64u32;
pub struct SERVICE_PREFERRED_NODE_INFO(i32);
pub struct SERVICE_PRESHUTDOWN_INFO(i32);
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_QUERY_CONFIG: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_QUERY_STATUS: u32 = 4u32;
pub struct SERVICE_REGISTRY_STATE_TYPE(i32);
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOA(i32);
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOW(i32);
pub struct SERVICE_RUNS_IN_PROCESS(i32);
pub struct SERVICE_SHARED_DIRECTORY_TYPE(i32);
pub struct SERVICE_SHARED_REGISTRY_STATE_TYPE(i32);
pub struct SERVICE_SID_INFO(i32);
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_SID_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_SID_TYPE_UNRESTRICTED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_START: u32 = 16u32;
pub struct SERVICE_START_REASON(i32);
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_START_REASON_AUTO: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_START_REASON_DELAYEDAUTO: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_START_REASON_DEMAND: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_START_REASON_RESTART_ON_FAILURE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_START_REASON_TRIGGER: u32 = 4u32;
pub struct SERVICE_START_TYPE(i32);
pub struct SERVICE_STATUS(i32);
pub struct SERVICE_STATUS_CURRENT_STATE(i32);
pub struct SERVICE_STATUS_HANDLE(i32);
pub struct SERVICE_STATUS_PROCESS(i32);
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_FLAG_CUSTOM: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_FLAG_MAX: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_FLAG_MIN: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_FLAG_PLANNED: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_FLAG_UNPLANNED: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_APPLICATION: u32 = 327680u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_HARDWARE: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_MAX: u32 = 458752u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_MAX_CUSTOM: u32 = 16711680u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_MIN: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_MIN_CUSTOM: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_NONE: u32 = 393216u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_OPERATINGSYSTEM: u32 = 196608u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_OTHER: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_SOFTWARE: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_DISK: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_ENVIRONMENT: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_HARDWARE_DRIVER: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_HUNG: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_INSTALLATION: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_MAINTENANCE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_MAX: u32 = 25u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_MAX_CUSTOM: u32 = 65535u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_MEMOTYLIMIT: u32 = 24u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_MIN: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_MIN_CUSTOM: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_MMC: u32 = 22u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_NETWORKCARD: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_NETWORK_CONNECTIVITY: u32 = 17u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_NONE: u32 = 23u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_OTHER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_OTHERDRIVER: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_RECONFIG: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_SECURITY: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX_UNINSTALL: u32 = 21u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK_UNINSTALL: u32 = 19u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE_UNINSTALL: u32 = 20u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_UNSTABLE: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_UPGRADE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_WMI: u32 = 18u32;
pub struct SERVICE_TABLE_ENTRYA(i32);
pub struct SERVICE_TABLE_ENTRYW(i32);
pub struct SERVICE_TIMECHANGE_INFO(i32);
pub struct SERVICE_TRIGGER(i32);
pub struct SERVICE_TRIGGER_ACTION(i32);
pub struct SERVICE_TRIGGER_CUSTOM_STATE_ID(i32);
pub struct SERVICE_TRIGGER_INFO(i32);
pub struct SERVICE_TRIGGER_SPECIFIC_DATA_ITEM(i32);
pub struct SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(i32);
pub struct SERVICE_TRIGGER_TYPE(i32);
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_TRIGGER_TYPE_AGGREGATE: u32 = 30u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_TRIGGER_TYPE_CUSTOM_SYSTEM_STATE_CHANGE: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_USER_DEFINED_CONTROL: u32 = 256u32;
pub const USER_POLICY_PRESENT_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1425753800, data2: 61577, data3: 17996, data4: [177, 253, 89, 209, 182, 44, 59, 80] };
pub struct _SC_NOTIFICATION_REGISTRATION(i32);
