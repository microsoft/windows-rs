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
    pub fn NotifyServiceStatusChangeA(hservice: super::super::Security::SC_HANDLE, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const ::core::mem::ManuallyDrop<SERVICE_NOTIFY_2A>) -> u32;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NotifyServiceStatusChangeW(hservice: super::super::Security::SC_HANDLE, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const ::core::mem::ManuallyDrop<SERVICE_NOTIFY_2W>) -> u32;
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
    pub fn RegisterServiceCtrlHandlerA(lpservicename: super::super::Foundation::PSTR, lphandlerproc: ::windows::runtime::RawPtr) -> SERVICE_STATUS_HANDLE;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterServiceCtrlHandlerExA(lpservicename: super::super::Foundation::PSTR, lphandlerproc: ::windows::runtime::RawPtr, lpcontext: *const ::core::ffi::c_void) -> SERVICE_STATUS_HANDLE;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterServiceCtrlHandlerExW(lpservicename: super::super::Foundation::PWSTR, lphandlerproc: ::windows::runtime::RawPtr, lpcontext: *const ::core::ffi::c_void) -> SERVICE_STATUS_HANDLE;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterServiceCtrlHandlerW(lpservicename: super::super::Foundation::PWSTR, lphandlerproc: ::windows::runtime::RawPtr) -> SERVICE_STATUS_HANDLE;
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
    pub fn StartServiceCtrlDispatcherA(lpservicestarttable: *const ::core::mem::ManuallyDrop<SERVICE_TABLE_ENTRYA>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartServiceCtrlDispatcherW(lpservicestarttable: *const ::core::mem::ManuallyDrop<SERVICE_TABLE_ENTRYW>) -> super::super::Foundation::BOOL;
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
