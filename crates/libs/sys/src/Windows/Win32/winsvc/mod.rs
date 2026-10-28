#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn ChangeServiceConfig2A(hservice : SC_HANDLE, dwinfolevel : super::DWORD, lpinfo : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn ChangeServiceConfig2W(hservice : SC_HANDLE, dwinfolevel : super::DWORD, lpinfo : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn ChangeServiceConfigA(hservice : SC_HANDLE, dwservicetype : super::DWORD, dwstarttype : super::DWORD, dwerrorcontrol : super::DWORD, lpbinarypathname : windows_sys::core::PCSTR, lploadordergroup : windows_sys::core::PCSTR, lpdwtagid : super::LPDWORD, lpdependencies : windows_sys::core::PCSTR, lpservicestartname : windows_sys::core::PCSTR, lppassword : windows_sys::core::PCSTR, lpdisplayname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn ChangeServiceConfigW(hservice : SC_HANDLE, dwservicetype : super::DWORD, dwstarttype : super::DWORD, dwerrorcontrol : super::DWORD, lpbinarypathname : windows_sys::core::PCWSTR, lploadordergroup : windows_sys::core::PCWSTR, lpdwtagid : super::LPDWORD, lpdependencies : windows_sys::core::PCWSTR, lpservicestartname : windows_sys::core::PCWSTR, lppassword : windows_sys::core::PCWSTR, lpdisplayname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CloseServiceHandle(hscobject : SC_HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn ControlService(hservice : SC_HANDLE, dwcontrol : super::DWORD, lpservicestatus : LPSERVICE_STATUS) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn ControlServiceExA(hservice : SC_HANDLE, dwcontrol : super::DWORD, dwinfolevel : super::DWORD, pcontrolparams : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn ControlServiceExW(hservice : SC_HANDLE, dwcontrol : super::DWORD, dwinfolevel : super::DWORD, pcontrolparams : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn CreateServiceA(hscmanager : SC_HANDLE, lpservicename : windows_sys::core::PCSTR, lpdisplayname : windows_sys::core::PCSTR, dwdesiredaccess : super::DWORD, dwservicetype : super::DWORD, dwstarttype : super::DWORD, dwerrorcontrol : super::DWORD, lpbinarypathname : windows_sys::core::PCSTR, lploadordergroup : windows_sys::core::PCSTR, lpdwtagid : super::LPDWORD, lpdependencies : windows_sys::core::PCSTR, lpservicestartname : windows_sys::core::PCSTR, lppassword : windows_sys::core::PCSTR) -> SC_HANDLE);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn CreateServiceW(hscmanager : SC_HANDLE, lpservicename : windows_sys::core::PCWSTR, lpdisplayname : windows_sys::core::PCWSTR, dwdesiredaccess : super::DWORD, dwservicetype : super::DWORD, dwstarttype : super::DWORD, dwerrorcontrol : super::DWORD, lpbinarypathname : windows_sys::core::PCWSTR, lploadordergroup : windows_sys::core::PCWSTR, lpdwtagid : super::LPDWORD, lpdependencies : windows_sys::core::PCWSTR, lpservicestartname : windows_sys::core::PCWSTR, lppassword : windows_sys::core::PCWSTR) -> SC_HANDLE);
windows_link::link!("advapi32.dll" "system" fn DeleteService(hservice : SC_HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn EnumDependentServicesA(hservice : SC_HANDLE, dwservicestate : super::DWORD, lpservices : LPENUM_SERVICE_STATUSA, cbbufsize : super::DWORD, pcbbytesneeded : super::LPDWORD, lpservicesreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn EnumDependentServicesW(hservice : SC_HANDLE, dwservicestate : super::DWORD, lpservices : LPENUM_SERVICE_STATUSW, cbbufsize : super::DWORD, pcbbytesneeded : super::LPDWORD, lpservicesreturned : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn EnumServicesStatusA(hscmanager : SC_HANDLE, dwservicetype : super::DWORD, dwservicestate : super::DWORD, lpservices : LPENUM_SERVICE_STATUSA, cbbufsize : super::DWORD, pcbbytesneeded : super::LPDWORD, lpservicesreturned : super::LPDWORD, lpresumehandle : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn EnumServicesStatusExA(hscmanager : SC_HANDLE, infolevel : SC_ENUM_TYPE, dwservicetype : super::DWORD, dwservicestate : super::DWORD, lpservices : super::LPBYTE, cbbufsize : super::DWORD, pcbbytesneeded : super::LPDWORD, lpservicesreturned : super::LPDWORD, lpresumehandle : super::LPDWORD, pszgroupname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn EnumServicesStatusExW(hscmanager : SC_HANDLE, infolevel : SC_ENUM_TYPE, dwservicetype : super::DWORD, dwservicestate : super::DWORD, lpservices : super::LPBYTE, cbbufsize : super::DWORD, pcbbytesneeded : super::LPDWORD, lpservicesreturned : super::LPDWORD, lpresumehandle : super::LPDWORD, pszgroupname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn EnumServicesStatusW(hscmanager : SC_HANDLE, dwservicetype : super::DWORD, dwservicestate : super::DWORD, lpservices : LPENUM_SERVICE_STATUSW, cbbufsize : super::DWORD, pcbbytesneeded : super::LPDWORD, lpservicesreturned : super::LPDWORD, lpresumehandle : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-service-core-l1-1-4.dll" "system" fn GetServiceDirectory(hservicestatus : SERVICE_STATUS_HANDLE, edirectorytype : SERVICE_DIRECTORY_TYPE, lppathbuffer : super::PWCHAR, cchpathbufferlength : super::DWORD, lpcchrequiredbufferlength : *mut super::DWORD) -> super::DWORD);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn GetServiceDisplayNameA(hscmanager : SC_HANDLE, lpservicename : windows_sys::core::PCSTR, lpdisplayname : windows_sys::core::PSTR, lpcchbuffer : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn GetServiceDisplayNameW(hscmanager : SC_HANDLE, lpservicename : windows_sys::core::PCWSTR, lpdisplayname : windows_sys::core::PWSTR, lpcchbuffer : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn GetServiceKeyNameA(hscmanager : SC_HANDLE, lpdisplayname : windows_sys::core::PCSTR, lpservicename : windows_sys::core::PSTR, lpcchbuffer : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("advapi32.dll" "system" fn GetServiceKeyNameW(hscmanager : SC_HANDLE, lpdisplayname : windows_sys::core::PCWSTR, lpservicename : windows_sys::core::PWSTR, lpcchbuffer : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("api-ms-win-service-core-l1-1-3.dll" "system" fn GetServiceRegistryStateKey(servicestatushandle : SERVICE_STATUS_HANDLE, statetype : SERVICE_REGISTRY_STATE_TYPE, accessmask : super::DWORD, servicestatekey : *mut super::HKEY) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-service-core-l1-1-5.dll" "system" fn GetSharedServiceDirectory(servicehandle : SC_HANDLE, directorytype : SERVICE_SHARED_DIRECTORY_TYPE, pathbuffer : super::PWCHAR, pathbufferlength : super::DWORD, requiredbufferlength : *mut super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("api-ms-win-service-core-l1-1-5.dll" "system" fn GetSharedServiceRegistryStateKey(servicehandle : SC_HANDLE, statetype : SERVICE_SHARED_REGISTRY_STATE_TYPE, accessmask : super::DWORD, servicestatekey : *mut super::HKEY) -> super::DWORD);
windows_link::link!("advapi32.dll" "system" fn LockServiceDatabase(hscmanager : SC_HANDLE) -> SC_LOCK);
windows_link::link!("advapi32.dll" "system" fn NotifyBootConfigStatus(bootacceptable : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn NotifyServiceStatusChangeA(hservice : SC_HANDLE, dwnotifymask : super::DWORD, pnotifybuffer : PSERVICE_NOTIFYA) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn NotifyServiceStatusChangeW(hservice : SC_HANDLE, dwnotifymask : super::DWORD, pnotifybuffer : PSERVICE_NOTIFYW) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn OpenSCManagerA(lpmachinename : windows_sys::core::PCSTR, lpdatabasename : windows_sys::core::PCSTR, dwdesiredaccess : super::DWORD) -> SC_HANDLE);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn OpenSCManagerW(lpmachinename : windows_sys::core::PCWSTR, lpdatabasename : windows_sys::core::PCWSTR, dwdesiredaccess : super::DWORD) -> SC_HANDLE);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn OpenServiceA(hscmanager : SC_HANDLE, lpservicename : windows_sys::core::PCSTR, dwdesiredaccess : super::DWORD) -> SC_HANDLE);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn OpenServiceW(hscmanager : SC_HANDLE, lpservicename : windows_sys::core::PCWSTR, dwdesiredaccess : super::DWORD) -> SC_HANDLE);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn QueryServiceConfig2A(hservice : SC_HANDLE, dwinfolevel : super::DWORD, lpbuffer : super::LPBYTE, cbbufsize : super::DWORD, pcbbytesneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn QueryServiceConfig2W(hservice : SC_HANDLE, dwinfolevel : super::DWORD, lpbuffer : super::LPBYTE, cbbufsize : super::DWORD, pcbbytesneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn QueryServiceConfigA(hservice : SC_HANDLE, lpserviceconfig : LPQUERY_SERVICE_CONFIGA, cbbufsize : super::DWORD, pcbbytesneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn QueryServiceConfigW(hservice : SC_HANDLE, lpserviceconfig : LPQUERY_SERVICE_CONFIGW, cbbufsize : super::DWORD, pcbbytesneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn QueryServiceDynamicInformation(hservicestatus : SERVICE_STATUS_HANDLE, dwinfolevel : super::DWORD, ppdynamicinfo : *mut super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn QueryServiceLockStatusA(hscmanager : SC_HANDLE, lplockstatus : LPQUERY_SERVICE_LOCK_STATUSA, cbbufsize : super::DWORD, pcbbytesneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn QueryServiceLockStatusW(hscmanager : SC_HANDLE, lplockstatus : LPQUERY_SERVICE_LOCK_STATUSW, cbbufsize : super::DWORD, pcbbytesneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn QueryServiceObjectSecurity(hservice : SC_HANDLE, dwsecurityinformation : super::SECURITY_INFORMATION, lpsecuritydescriptor : super::PSECURITY_DESCRIPTOR, cbbufsize : super::DWORD, pcbbytesneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn QueryServiceStatus(hservice : SC_HANDLE, lpservicestatus : LPSERVICE_STATUS) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn QueryServiceStatusEx(hservice : SC_HANDLE, infolevel : SC_STATUS_TYPE, lpbuffer : super::LPBYTE, cbbufsize : super::DWORD, pcbbytesneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerA(lpservicename : windows_sys::core::PCSTR, lphandlerproc : LPHANDLER_FUNCTION) -> SERVICE_STATUS_HANDLE);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerExA(lpservicename : windows_sys::core::PCSTR, lphandlerproc : LPHANDLER_FUNCTION_EX, lpcontext : super::LPVOID) -> SERVICE_STATUS_HANDLE);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerExW(lpservicename : windows_sys::core::PCWSTR, lphandlerproc : LPHANDLER_FUNCTION_EX, lpcontext : super::LPVOID) -> SERVICE_STATUS_HANDLE);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerW(lpservicename : windows_sys::core::PCWSTR, lphandlerproc : LPHANDLER_FUNCTION) -> SERVICE_STATUS_HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetServiceObjectSecurity(hservice : SC_HANDLE, dwsecurityinformation : super::SECURITY_INFORMATION, lpsecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn SetServiceStatus(hservicestatus : SERVICE_STATUS_HANDLE, lpservicestatus : LPSERVICE_STATUS) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn StartServiceA(hservice : SC_HANDLE, dwnumserviceargs : super::DWORD, lpserviceargvectors : *const windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn StartServiceCtrlDispatcherA(lpservicestarttable : *const SERVICE_TABLE_ENTRYA) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn StartServiceCtrlDispatcherW(lpservicestarttable : *const SERVICE_TABLE_ENTRYW) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("advapi32.dll" "system" fn StartServiceW(hservice : SC_HANDLE, dwnumserviceargs : super::DWORD, lpserviceargvectors : *const windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn UnlockServiceDatabase(sclock : SC_LOCK) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn WaitServiceState(hservice : SC_HANDLE, dwnotify : super::DWORD, dwtimeout : super::DWORD, hcancelevent : super::HANDLE) -> super::DWORD);
pub const CUSTOM_SYSTEM_STATE_CHANGE_EVENT_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2d7a2816_0c5e_45fc_9ce7_570e5ecde9c9);
pub const DOMAIN_JOIN_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1ce20aba_9851_4421_9430_1ddeb766e809);
pub const DOMAIN_LEAVE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xddaf516e_58c2_4866_9574_c3b615d42ea1);
#[cfg(feature = "shared")]
pub type ENUM_SERVICE_STATUS = ENUM_SERVICE_STATUSA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ENUM_SERVICE_STATUSA {
    pub lpServiceName: windows_sys::core::PSTR,
    pub lpDisplayName: windows_sys::core::PSTR,
    pub ServiceStatus: SERVICE_STATUS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ENUM_SERVICE_STATUSW {
    pub lpServiceName: windows_sys::core::PWSTR,
    pub lpDisplayName: windows_sys::core::PWSTR,
    pub ServiceStatus: SERVICE_STATUS,
}
#[cfg(feature = "shared")]
pub type ENUM_SERVICE_STATUS_PROCESS = ENUM_SERVICE_STATUS_PROCESSA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ENUM_SERVICE_STATUS_PROCESSA {
    pub lpServiceName: windows_sys::core::PSTR,
    pub lpDisplayName: windows_sys::core::PSTR,
    pub ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ENUM_SERVICE_STATUS_PROCESSW {
    pub lpServiceName: windows_sys::core::PWSTR,
    pub lpDisplayName: windows_sys::core::PWSTR,
    pub ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}
pub const FIREWALL_PORT_CLOSE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa144ed38_8e12_4de4_9d96_e64740b1a524);
pub const FIREWALL_PORT_OPEN_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb7569e07_8421_4ee0_ad10_86915afdad09);
#[cfg(feature = "shared")]
pub type HANDLER_FUNCTION = Option<unsafe extern "system" fn(dwcontrol: super::DWORD)>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type HANDLER_FUNCTION_EX = Option<unsafe extern "system" fn(dwcontrol: super::DWORD, dweventtype: super::DWORD, lpeventdata: super::LPVOID, lpcontext: super::LPVOID) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type LPENUM_SERVICE_STATUS = *mut ENUM_SERVICE_STATUSA;
#[cfg(feature = "shared")]
pub type LPENUM_SERVICE_STATUSA = *mut ENUM_SERVICE_STATUSA;
#[cfg(feature = "shared")]
pub type LPENUM_SERVICE_STATUSW = *mut ENUM_SERVICE_STATUSW;
#[cfg(feature = "shared")]
pub type LPENUM_SERVICE_STATUS_PROCESS = *mut ENUM_SERVICE_STATUS_PROCESSA;
#[cfg(feature = "shared")]
pub type LPENUM_SERVICE_STATUS_PROCESSA = *mut ENUM_SERVICE_STATUS_PROCESSA;
#[cfg(feature = "shared")]
pub type LPENUM_SERVICE_STATUS_PROCESSW = *mut ENUM_SERVICE_STATUS_PROCESSW;
#[cfg(feature = "shared")]
pub type LPHANDLER_FUNCTION = Option<unsafe extern "system" fn(dwcontrol: super::DWORD)>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPHANDLER_FUNCTION_EX = Option<unsafe extern "system" fn(dwcontrol: super::DWORD, dweventtype: super::DWORD, lpeventdata: super::LPVOID, lpcontext: super::LPVOID) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type LPQUERY_SERVICE_CONFIG = *mut QUERY_SERVICE_CONFIGA;
#[cfg(feature = "shared")]
pub type LPQUERY_SERVICE_CONFIGA = *mut QUERY_SERVICE_CONFIGA;
#[cfg(feature = "shared")]
pub type LPQUERY_SERVICE_CONFIGW = *mut QUERY_SERVICE_CONFIGW;
#[cfg(feature = "shared")]
pub type LPQUERY_SERVICE_LOCK_STATUS = *mut QUERY_SERVICE_LOCK_STATUSA;
#[cfg(feature = "shared")]
pub type LPQUERY_SERVICE_LOCK_STATUSA = *mut QUERY_SERVICE_LOCK_STATUSA;
#[cfg(feature = "shared")]
pub type LPQUERY_SERVICE_LOCK_STATUSW = *mut QUERY_SERVICE_LOCK_STATUSW;
#[cfg(feature = "shared")]
pub type LPSC_ACTION = *mut SC_ACTION;
pub type LPSC_HANDLE = *mut *mut SC_HANDLE__;
#[cfg(feature = "shared")]
pub type LPSERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM = *mut SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM;
pub type LPSERVICE_DELAYED_AUTO_START_INFO = *mut SERVICE_DELAYED_AUTO_START_INFO;
pub type LPSERVICE_DESCRIPTION = *mut SERVICE_DESCRIPTIONA;
pub type LPSERVICE_DESCRIPTIONA = *mut SERVICE_DESCRIPTIONA;
pub type LPSERVICE_DESCRIPTIONW = *mut SERVICE_DESCRIPTIONW;
#[cfg(feature = "shared")]
pub type LPSERVICE_FAILURE_ACTIONS = *mut SERVICE_FAILURE_ACTIONSA;
#[cfg(feature = "shared")]
pub type LPSERVICE_FAILURE_ACTIONSA = *mut SERVICE_FAILURE_ACTIONSA;
#[cfg(feature = "shared")]
pub type LPSERVICE_FAILURE_ACTIONSW = *mut SERVICE_FAILURE_ACTIONSW;
pub type LPSERVICE_FAILURE_ACTIONS_FLAG = *mut SERVICE_FAILURE_ACTIONS_FLAG;
#[cfg(feature = "shared")]
pub type LPSERVICE_MAIN_FUNCTIONA = Option<unsafe extern "system" fn(dwnumservicesargs: super::DWORD, lpserviceargvectors: *mut windows_sys::core::PSTR)>;
#[cfg(feature = "shared")]
pub type LPSERVICE_MAIN_FUNCTIONW = Option<unsafe extern "system" fn(dwnumservicesargs: super::DWORD, lpserviceargvectors: *mut windows_sys::core::PWSTR)>;
#[cfg(feature = "shared")]
pub type LPSERVICE_PREFERRED_NODE_INFO = *mut SERVICE_PREFERRED_NODE_INFO;
#[cfg(feature = "shared")]
pub type LPSERVICE_PRESHUTDOWN_INFO = *mut SERVICE_PRESHUTDOWN_INFO;
pub type LPSERVICE_REQUIRED_PRIVILEGES_INFO = *mut SERVICE_REQUIRED_PRIVILEGES_INFOA;
pub type LPSERVICE_REQUIRED_PRIVILEGES_INFOA = *mut SERVICE_REQUIRED_PRIVILEGES_INFOA;
pub type LPSERVICE_REQUIRED_PRIVILEGES_INFOW = *mut SERVICE_REQUIRED_PRIVILEGES_INFOW;
#[cfg(feature = "shared")]
pub type LPSERVICE_SID_INFO = *mut SERVICE_SID_INFO;
#[cfg(feature = "shared")]
pub type LPSERVICE_STATUS = *mut SERVICE_STATUS;
#[cfg(feature = "shared")]
pub type LPSERVICE_STATUS_PROCESS = *mut SERVICE_STATUS_PROCESS;
#[cfg(feature = "shared")]
pub type LPSERVICE_TABLE_ENTRY = *mut SERVICE_TABLE_ENTRYA;
#[cfg(feature = "shared")]
pub type LPSERVICE_TABLE_ENTRYA = *mut SERVICE_TABLE_ENTRYA;
#[cfg(feature = "shared")]
pub type LPSERVICE_TABLE_ENTRYW = *mut SERVICE_TABLE_ENTRYW;
pub const MACHINE_POLICY_PRESENT_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x659fcae6_5bdb_4da9_b1ff_ca2a178d46e0);
pub const MaxServiceRegistryStateType: SERVICE_REGISTRY_STATE_TYPE = 2;
pub const NAMED_PIPE_EVENT_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1f81d131_3fac_4537_9e0c_7e7b0c2f4b55);
pub const NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4f27f2de_14e2_430b_a549_7cd48cbc8245);
pub const NETWORK_MANAGER_LAST_IP_ADDRESS_REMOVAL_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcc4ba62a_162e_4648_847a_b6bdf993e335);
#[cfg(feature = "winnt")]
pub type PFN_SC_NOTIFY_CALLBACK = Option<unsafe extern "system" fn(pparameter: super::PVOID)>;
pub type PSC_EVENT_TYPE = *mut SC_EVENT_TYPE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSC_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(dwnotify: super::DWORD, pcallbackcontext: super::PVOID)>;
pub type PSC_NOTIFICATION_REGISTRATION = *mut _SC_NOTIFICATION_REGISTRATION;
#[cfg(feature = "shared")]
pub type PSERVICE_CONTROL_STATUS_REASON_PARAMS = *mut SERVICE_CONTROL_STATUS_REASON_PARAMSA;
#[cfg(feature = "shared")]
pub type PSERVICE_CONTROL_STATUS_REASON_PARAMSA = *mut SERVICE_CONTROL_STATUS_REASON_PARAMSA;
#[cfg(feature = "shared")]
pub type PSERVICE_CONTROL_STATUS_REASON_PARAMSW = *mut SERVICE_CONTROL_STATUS_REASON_PARAMSW;
#[cfg(feature = "shared")]
pub type PSERVICE_LAUNCH_PROTECTED_INFO = *mut SERVICE_LAUNCH_PROTECTED_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSERVICE_NOTIFY = *mut SERVICE_NOTIFY_2A;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSERVICE_NOTIFYA = *mut SERVICE_NOTIFY_2A;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSERVICE_NOTIFYW = *mut SERVICE_NOTIFY_2W;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSERVICE_NOTIFY_1 = *mut SERVICE_NOTIFY_1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSERVICE_NOTIFY_2 = *mut SERVICE_NOTIFY_2A;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSERVICE_NOTIFY_2A = *mut SERVICE_NOTIFY_2A;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSERVICE_NOTIFY_2W = *mut SERVICE_NOTIFY_2W;
#[cfg(feature = "shared")]
pub type PSERVICE_START_REASON = *mut SERVICE_START_REASON;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSERVICE_TIMECHANGE_INFO = *mut SERVICE_TIMECHANGE_INFO;
#[cfg(feature = "shared")]
pub type PSERVICE_TRIGGER = *mut SERVICE_TRIGGER;
#[cfg(feature = "shared")]
pub type PSERVICE_TRIGGER_INFO = *mut SERVICE_TRIGGER_INFO;
#[cfg(feature = "shared")]
pub type PSERVICE_TRIGGER_SPECIFIC_DATA_ITEM = *mut SERVICE_TRIGGER_SPECIFIC_DATA_ITEM;
#[cfg(feature = "shared")]
pub type QUERY_SERVICE_CONFIG = QUERY_SERVICE_CONFIGA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct QUERY_SERVICE_CONFIGA {
    pub dwServiceType: super::DWORD,
    pub dwStartType: super::DWORD,
    pub dwErrorControl: super::DWORD,
    pub lpBinaryPathName: windows_sys::core::PSTR,
    pub lpLoadOrderGroup: windows_sys::core::PSTR,
    pub dwTagId: super::DWORD,
    pub lpDependencies: windows_sys::core::PSTR,
    pub lpServiceStartName: windows_sys::core::PSTR,
    pub lpDisplayName: windows_sys::core::PSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct QUERY_SERVICE_CONFIGW {
    pub dwServiceType: super::DWORD,
    pub dwStartType: super::DWORD,
    pub dwErrorControl: super::DWORD,
    pub lpBinaryPathName: windows_sys::core::PWSTR,
    pub lpLoadOrderGroup: windows_sys::core::PWSTR,
    pub dwTagId: super::DWORD,
    pub lpDependencies: windows_sys::core::PWSTR,
    pub lpServiceStartName: windows_sys::core::PWSTR,
    pub lpDisplayName: windows_sys::core::PWSTR,
}
#[cfg(feature = "shared")]
pub type QUERY_SERVICE_LOCK_STATUS = QUERY_SERVICE_LOCK_STATUSA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct QUERY_SERVICE_LOCK_STATUSA {
    pub fIsLocked: super::DWORD,
    pub lpLockOwner: windows_sys::core::PSTR,
    pub dwLockDuration: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct QUERY_SERVICE_LOCK_STATUSW {
    pub fIsLocked: super::DWORD,
    pub lpLockOwner: windows_sys::core::PWSTR,
    pub dwLockDuration: super::DWORD,
}
pub const RPC_INTERFACE_EVENT_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbc90d167_9470_4139_a9ba_be0bbbf5b74d);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SC_ACTION {
    pub Type: SC_ACTION_TYPE,
    pub Delay: super::DWORD,
}
pub const SC_ACTION_NONE: SC_ACTION_TYPE = 0;
pub const SC_ACTION_OWN_RESTART: SC_ACTION_TYPE = 4;
pub const SC_ACTION_REBOOT: SC_ACTION_TYPE = 2;
pub const SC_ACTION_RESTART: SC_ACTION_TYPE = 1;
pub const SC_ACTION_RUN_COMMAND: SC_ACTION_TYPE = 3;
pub type SC_ACTION_TYPE = i32;
pub const SC_AGGREGATE_STORAGE_KEY: windows_sys::core::PCWSTR = windows_sys::core::w!("System\\CurrentControlSet\\Control\\ServiceAggregatedEvents");
pub const SC_ENUM_PROCESS_INFO: SC_ENUM_TYPE = 0;
pub type SC_ENUM_TYPE = i32;
pub const SC_EVENT_DATABASE_CHANGE: SC_EVENT_TYPE = 0;
pub const SC_EVENT_PROPERTY_CHANGE: SC_EVENT_TYPE = 1;
pub const SC_EVENT_STATUS_CHANGE: SC_EVENT_TYPE = 2;
pub type SC_EVENT_TYPE = i32;
pub const SC_GROUP_IDENTIFIER: i8 = 43;
pub const SC_GROUP_IDENTIFIERA: i8 = 43;
pub const SC_GROUP_IDENTIFIERW: u16 = 43;
pub type SC_HANDLE = *mut SC_HANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SC_HANDLE__ {
    pub unused: i32,
}
pub type SC_LOCK = *mut core::ffi::c_void;
pub const SC_MANAGER_ALL_ACCESS: i32 = 983103;
pub const SC_MANAGER_CONNECT: i32 = 1;
pub const SC_MANAGER_CREATE_SERVICE: i32 = 2;
pub const SC_MANAGER_ENUMERATE_SERVICE: i32 = 4;
pub const SC_MANAGER_LOCK: i32 = 8;
pub const SC_MANAGER_MODIFY_BOOT_CONFIG: i32 = 32;
pub const SC_MANAGER_QUERY_LOCK_STATUS: i32 = 16;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type SC_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(dwnotify: super::DWORD, pcallbackcontext: super::PVOID)>;
pub const SC_STATUS_PROCESS_INFO: SC_STATUS_TYPE = 0;
pub type SC_STATUS_TYPE = i32;
pub const SERVICES_ACTIVE_DATABASE: windows_sys::core::PCSTR = windows_sys::core::s!("ServicesActive");
pub const SERVICES_ACTIVE_DATABASEA: windows_sys::core::PCSTR = windows_sys::core::s!("ServicesActive");
pub const SERVICES_ACTIVE_DATABASEW: windows_sys::core::PCWSTR = windows_sys::core::w!("ServicesActive");
pub const SERVICES_FAILED_DATABASE: windows_sys::core::PCSTR = windows_sys::core::s!("ServicesFailed");
pub const SERVICES_FAILED_DATABASEA: windows_sys::core::PCSTR = windows_sys::core::s!("ServicesFailed");
pub const SERVICES_FAILED_DATABASEW: windows_sys::core::PCWSTR = windows_sys::core::w!("ServicesFailed");
pub const SERVICE_ACCEPT_HARDWAREPROFILECHANGE: i32 = 32;
pub const SERVICE_ACCEPT_LOWRESOURCES: i32 = 8192;
pub const SERVICE_ACCEPT_NETBINDCHANGE: i32 = 16;
pub const SERVICE_ACCEPT_PARAMCHANGE: i32 = 8;
pub const SERVICE_ACCEPT_PAUSE_CONTINUE: i32 = 2;
pub const SERVICE_ACCEPT_POWEREVENT: i32 = 64;
pub const SERVICE_ACCEPT_PRESHUTDOWN: i32 = 256;
pub const SERVICE_ACCEPT_SESSIONCHANGE: i32 = 128;
pub const SERVICE_ACCEPT_SHUTDOWN: i32 = 4;
pub const SERVICE_ACCEPT_STOP: i32 = 1;
pub const SERVICE_ACCEPT_SYSTEMLOWRESOURCES: i32 = 16384;
pub const SERVICE_ACCEPT_TIMECHANGE: i32 = 512;
pub const SERVICE_ACCEPT_TRIGGEREVENT: i32 = 1024;
pub const SERVICE_ACCEPT_USER_LOGOFF: i32 = 2048;
pub const SERVICE_ACTIVE: i32 = 1;
pub const SERVICE_ALL_ACCESS: i32 = 983551;
pub const SERVICE_CHANGE_CONFIG: i32 = 2;
pub const SERVICE_CONFIG_DELAYED_AUTO_START_INFO: i32 = 3;
pub const SERVICE_CONFIG_DESCRIPTION: i32 = 1;
pub const SERVICE_CONFIG_FAILURE_ACTIONS: i32 = 2;
pub const SERVICE_CONFIG_FAILURE_ACTIONS_FLAG: i32 = 4;
pub const SERVICE_CONFIG_LAUNCH_PROTECTED: i32 = 12;
pub const SERVICE_CONFIG_PREFERRED_NODE: i32 = 9;
pub const SERVICE_CONFIG_PRESHUTDOWN_INFO: i32 = 7;
pub const SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO: i32 = 6;
pub const SERVICE_CONFIG_SERVICE_SID_INFO: i32 = 5;
pub const SERVICE_CONFIG_TRIGGER_INFO: i32 = 8;
pub const SERVICE_CONTINUE_PENDING: i32 = 5;
pub const SERVICE_CONTROL_CONTINUE: i32 = 3;
pub const SERVICE_CONTROL_DEVICEEVENT: i32 = 11;
pub const SERVICE_CONTROL_HARDWAREPROFILECHANGE: i32 = 12;
pub const SERVICE_CONTROL_INTERROGATE: i32 = 4;
pub const SERVICE_CONTROL_LOWRESOURCES: i32 = 96;
pub const SERVICE_CONTROL_NETBINDADD: i32 = 7;
pub const SERVICE_CONTROL_NETBINDDISABLE: i32 = 10;
pub const SERVICE_CONTROL_NETBINDENABLE: i32 = 9;
pub const SERVICE_CONTROL_NETBINDREMOVE: i32 = 8;
pub const SERVICE_CONTROL_PARAMCHANGE: i32 = 6;
pub const SERVICE_CONTROL_PAUSE: i32 = 2;
pub const SERVICE_CONTROL_POWEREVENT: i32 = 13;
pub const SERVICE_CONTROL_PRESHUTDOWN: i32 = 15;
pub const SERVICE_CONTROL_SESSIONCHANGE: i32 = 14;
pub const SERVICE_CONTROL_SHUTDOWN: i32 = 5;
pub const SERVICE_CONTROL_STATUS_REASON_INFO: i32 = 1;
#[cfg(feature = "shared")]
pub type SERVICE_CONTROL_STATUS_REASON_PARAMS = SERVICE_CONTROL_STATUS_REASON_PARAMSA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    pub dwReason: super::DWORD,
    pub pszComment: windows_sys::core::PSTR,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    pub dwReason: super::DWORD,
    pub pszComment: windows_sys::core::PWSTR,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
pub const SERVICE_CONTROL_STOP: i32 = 1;
pub const SERVICE_CONTROL_SYSTEMLOWRESOURCES: i32 = 97;
pub const SERVICE_CONTROL_TIMECHANGE: i32 = 16;
pub const SERVICE_CONTROL_TRIGGEREVENT: i32 = 32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    pub u: SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0,
}
#[cfg(feature = "shared")]
impl Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    pub CustomStateId: SERVICE_TRIGGER_CUSTOM_STATE_ID,
    pub s: SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0,
}
#[cfg(feature = "shared")]
impl Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    pub DataOffset: super::DWORD,
    pub Data: [super::BYTE; 1],
}
#[cfg(feature = "shared")]
impl Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_DELAYED_AUTO_START_INFO {
    pub fDelayedAutostart: windows_sys::core::BOOL,
}
pub type SERVICE_DESCRIPTION = SERVICE_DESCRIPTIONA;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_DESCRIPTIONA {
    pub lpDescription: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_DESCRIPTIONW {
    pub lpDescription: windows_sys::core::PWSTR,
}
pub type SERVICE_DIRECTORY_TYPE = i32;
pub const SERVICE_DYNAMIC_INFORMATION_LEVEL_START_REASON: i32 = 1;
pub const SERVICE_ENUMERATE_DEPENDENTS: i32 = 8;
#[cfg(feature = "shared")]
pub type SERVICE_FAILURE_ACTIONS = SERVICE_FAILURE_ACTIONSA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_FAILURE_ACTIONSA {
    pub dwResetPeriod: super::DWORD,
    pub lpRebootMsg: windows_sys::core::PSTR,
    pub lpCommand: windows_sys::core::PSTR,
    pub cActions: super::DWORD,
    pub lpsaActions: *mut SC_ACTION,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_FAILURE_ACTIONSW {
    pub dwResetPeriod: super::DWORD,
    pub lpRebootMsg: windows_sys::core::PWSTR,
    pub lpCommand: windows_sys::core::PWSTR,
    pub cActions: super::DWORD,
    pub lpsaActions: *mut SC_ACTION,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_FAILURE_ACTIONS_FLAG {
    pub fFailureActionsOnNonCrashFailures: windows_sys::core::BOOL,
}
pub const SERVICE_INACTIVE: i32 = 2;
pub const SERVICE_INTERROGATE: i32 = 128;
pub const SERVICE_LAUNCH_PROTECTED_ANTIMALWARE_LIGHT: i32 = 3;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_LAUNCH_PROTECTED_INFO {
    pub dwLaunchProtected: super::DWORD,
}
pub const SERVICE_LAUNCH_PROTECTED_NONE: i32 = 0;
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS: i32 = 1;
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS_LIGHT: i32 = 2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type SERVICE_MAIN_FUNCTIONA = Option<unsafe extern "system" fn(dwnumservicesargs: super::DWORD, lpserviceargvectors: *mut super::LPTSTR)>;
#[cfg(feature = "shared")]
pub type SERVICE_MAIN_FUNCTIONW = Option<unsafe extern "system" fn(dwnumservicesargs: super::DWORD, lpserviceargvectors: *mut windows_sys::core::PWSTR)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type SERVICE_NOTIFY = SERVICE_NOTIFY_2A;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type SERVICE_NOTIFYA = SERVICE_NOTIFY_2A;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type SERVICE_NOTIFYW = SERVICE_NOTIFY_2W;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_NOTIFY_1 {
    pub dwVersion: super::DWORD,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: super::PVOID,
    pub dwNotificationStatus: super::DWORD,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type SERVICE_NOTIFY_2 = SERVICE_NOTIFY_2A;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_NOTIFY_2A {
    pub dwVersion: super::DWORD,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: super::PVOID,
    pub dwNotificationStatus: super::DWORD,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: super::DWORD,
    pub pszServiceNames: windows_sys::core::PSTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_NOTIFY_2W {
    pub dwVersion: super::DWORD,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: super::PVOID,
    pub dwNotificationStatus: super::DWORD,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: super::DWORD,
    pub pszServiceNames: windows_sys::core::PWSTR,
}
pub const SERVICE_NOTIFY_CONTINUE_PENDING: i32 = 16;
pub const SERVICE_NOTIFY_CREATED: i32 = 128;
pub const SERVICE_NOTIFY_DELETED: i32 = 256;
pub const SERVICE_NOTIFY_DELETE_PENDING: i32 = 512;
pub const SERVICE_NOTIFY_PAUSED: i32 = 64;
pub const SERVICE_NOTIFY_PAUSE_PENDING: i32 = 32;
pub const SERVICE_NOTIFY_RUNNING: i32 = 8;
pub const SERVICE_NOTIFY_START_PENDING: i32 = 2;
pub const SERVICE_NOTIFY_STATUS_CHANGE: i32 = 2;
pub const SERVICE_NOTIFY_STATUS_CHANGE_1: i32 = 1;
pub const SERVICE_NOTIFY_STATUS_CHANGE_2: i32 = 2;
pub const SERVICE_NOTIFY_STOPPED: i32 = 1;
pub const SERVICE_NOTIFY_STOP_PENDING: i32 = 4;
pub const SERVICE_NO_CHANGE: u32 = 4294967295;
pub const SERVICE_PAUSED: i32 = 7;
pub const SERVICE_PAUSE_CONTINUE: i32 = 64;
pub const SERVICE_PAUSE_PENDING: i32 = 6;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_PREFERRED_NODE_INFO {
    pub usPreferredNode: super::USHORT,
    pub fDelete: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_PRESHUTDOWN_INFO {
    pub dwPreshutdownTimeout: super::DWORD,
}
pub const SERVICE_QUERY_CONFIG: i32 = 1;
pub const SERVICE_QUERY_STATUS: i32 = 4;
pub type SERVICE_REGISTRY_STATE_TYPE = i32;
pub type SERVICE_REQUIRED_PRIVILEGES_INFO = SERVICE_REQUIRED_PRIVILEGES_INFOA;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOA {
    pub pmszRequiredPrivileges: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOW {
    pub pmszRequiredPrivileges: windows_sys::core::PWSTR,
}
pub const SERVICE_RUNNING: i32 = 4;
pub const SERVICE_RUNS_IN_SYSTEM_PROCESS: i32 = 1;
pub type SERVICE_SHARED_DIRECTORY_TYPE = i32;
pub type SERVICE_SHARED_REGISTRY_STATE_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_SID_INFO {
    pub dwServiceSidType: super::DWORD,
}
pub const SERVICE_SID_TYPE_NONE: i32 = 0;
pub const SERVICE_SID_TYPE_RESTRICTED: i32 = 3;
pub const SERVICE_SID_TYPE_UNRESTRICTED: i32 = 1;
pub const SERVICE_START: i32 = 16;
pub const SERVICE_START_PENDING: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_START_REASON {
    pub dwReason: super::DWORD,
}
pub const SERVICE_START_REASON_AUTO: i32 = 2;
pub const SERVICE_START_REASON_DELAYEDAUTO: i32 = 16;
pub const SERVICE_START_REASON_DEMAND: i32 = 1;
pub const SERVICE_START_REASON_RESTART_ON_FAILURE: i32 = 8;
pub const SERVICE_START_REASON_TRIGGER: i32 = 4;
pub const SERVICE_STATE_ALL: i32 = 3;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_STATUS {
    pub dwServiceType: super::DWORD,
    pub dwCurrentState: super::DWORD,
    pub dwControlsAccepted: super::DWORD,
    pub dwWin32ExitCode: super::DWORD,
    pub dwServiceSpecificExitCode: super::DWORD,
    pub dwCheckPoint: super::DWORD,
    pub dwWaitHint: super::DWORD,
}
pub type SERVICE_STATUS_HANDLE = *mut SERVICE_STATUS_HANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_STATUS_HANDLE__ {
    pub unused: i32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_STATUS_PROCESS {
    pub dwServiceType: super::DWORD,
    pub dwCurrentState: super::DWORD,
    pub dwControlsAccepted: super::DWORD,
    pub dwWin32ExitCode: super::DWORD,
    pub dwServiceSpecificExitCode: super::DWORD,
    pub dwCheckPoint: super::DWORD,
    pub dwWaitHint: super::DWORD,
    pub dwProcessId: super::DWORD,
    pub dwServiceFlags: super::DWORD,
}
pub const SERVICE_STOP: i32 = 32;
pub const SERVICE_STOPPED: i32 = 1;
pub const SERVICE_STOP_PENDING: i32 = 3;
pub const SERVICE_STOP_REASON_FLAG_CUSTOM: i32 = 536870912;
pub const SERVICE_STOP_REASON_FLAG_MAX: u32 = 2147483648;
pub const SERVICE_STOP_REASON_FLAG_MIN: i32 = 0;
pub const SERVICE_STOP_REASON_FLAG_PLANNED: i32 = 1073741824;
pub const SERVICE_STOP_REASON_FLAG_UNPLANNED: i32 = 268435456;
pub const SERVICE_STOP_REASON_MAJOR_APPLICATION: i32 = 327680;
pub const SERVICE_STOP_REASON_MAJOR_HARDWARE: i32 = 131072;
pub const SERVICE_STOP_REASON_MAJOR_MAX: i32 = 458752;
pub const SERVICE_STOP_REASON_MAJOR_MAX_CUSTOM: i32 = 16711680;
pub const SERVICE_STOP_REASON_MAJOR_MIN: i32 = 0;
pub const SERVICE_STOP_REASON_MAJOR_MIN_CUSTOM: i32 = 4194304;
pub const SERVICE_STOP_REASON_MAJOR_NONE: i32 = 393216;
pub const SERVICE_STOP_REASON_MAJOR_OPERATINGSYSTEM: i32 = 196608;
pub const SERVICE_STOP_REASON_MAJOR_OTHER: i32 = 65536;
pub const SERVICE_STOP_REASON_MAJOR_SOFTWARE: i32 = 262144;
pub const SERVICE_STOP_REASON_MINOR_DISK: i32 = 8;
pub const SERVICE_STOP_REASON_MINOR_ENVIRONMENT: i32 = 10;
pub const SERVICE_STOP_REASON_MINOR_HARDWARE_DRIVER: i32 = 11;
pub const SERVICE_STOP_REASON_MINOR_HUNG: i32 = 6;
pub const SERVICE_STOP_REASON_MINOR_INSTALLATION: i32 = 3;
pub const SERVICE_STOP_REASON_MINOR_MAINTENANCE: i32 = 2;
pub const SERVICE_STOP_REASON_MINOR_MAX: i32 = 25;
pub const SERVICE_STOP_REASON_MINOR_MAX_CUSTOM: i32 = 65535;
pub const SERVICE_STOP_REASON_MINOR_MEMOTYLIMIT: i32 = 24;
pub const SERVICE_STOP_REASON_MINOR_MIN: i32 = 0;
pub const SERVICE_STOP_REASON_MINOR_MIN_CUSTOM: i32 = 256;
pub const SERVICE_STOP_REASON_MINOR_MMC: i32 = 22;
pub const SERVICE_STOP_REASON_MINOR_NETWORKCARD: i32 = 9;
pub const SERVICE_STOP_REASON_MINOR_NETWORK_CONNECTIVITY: i32 = 17;
pub const SERVICE_STOP_REASON_MINOR_NONE: i32 = 23;
pub const SERVICE_STOP_REASON_MINOR_OTHER: i32 = 1;
pub const SERVICE_STOP_REASON_MINOR_OTHERDRIVER: i32 = 12;
pub const SERVICE_STOP_REASON_MINOR_RECONFIG: i32 = 5;
pub const SERVICE_STOP_REASON_MINOR_SECURITY: i32 = 16;
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX: i32 = 15;
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX_UNINSTALL: i32 = 21;
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK: i32 = 13;
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK_UNINSTALL: i32 = 19;
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE: i32 = 14;
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE_UNINSTALL: i32 = 20;
pub const SERVICE_STOP_REASON_MINOR_UNSTABLE: i32 = 7;
pub const SERVICE_STOP_REASON_MINOR_UPGRADE: i32 = 4;
pub const SERVICE_STOP_REASON_MINOR_WMI: i32 = 18;
#[cfg(feature = "shared")]
pub type SERVICE_TABLE_ENTRY = SERVICE_TABLE_ENTRYA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_TABLE_ENTRYA {
    pub lpServiceName: windows_sys::core::PSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONA,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_TABLE_ENTRYW {
    pub lpServiceName: windows_sys::core::PWSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONW,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SERVICE_TIMECHANGE_INFO {
    pub liNewTime: super::LARGE_INTEGER,
    pub liOldTime: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SERVICE_TIMECHANGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_TRIGGER {
    pub dwTriggerType: super::DWORD,
    pub dwAction: super::DWORD,
    pub pTriggerSubtype: *mut windows_sys::core::GUID,
    pub cDataItems: super::DWORD,
    pub pDataItems: PSERVICE_TRIGGER_SPECIFIC_DATA_ITEM,
}
pub const SERVICE_TRIGGER_ACTION_SERVICE_START: i32 = 1;
pub const SERVICE_TRIGGER_ACTION_SERVICE_STOP: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SERVICE_TRIGGER_CUSTOM_STATE_ID {
    pub Data: [super::DWORD; 2],
}
#[cfg(feature = "shared")]
impl Default for SERVICE_TRIGGER_CUSTOM_STATE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERVICE_TRIGGER_DATA_TYPE_BINARY: i32 = 1;
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ALL: i32 = 5;
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ANY: i32 = 4;
pub const SERVICE_TRIGGER_DATA_TYPE_LEVEL: i32 = 3;
pub const SERVICE_TRIGGER_DATA_TYPE_STRING: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_TRIGGER_INFO {
    pub cTriggers: super::DWORD,
    pub pTriggers: PSERVICE_TRIGGER,
    pub pReserved: super::PBYTE,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    pub dwDataType: super::DWORD,
    pub cbData: super::DWORD,
    pub pData: super::PBYTE,
}
pub const SERVICE_TRIGGER_STARTED_ARGUMENT: windows_sys::core::PCWSTR = windows_sys::core::w!("TriggerStarted");
pub const SERVICE_TRIGGER_TYPE_AGGREGATE: i32 = 30;
pub const SERVICE_TRIGGER_TYPE_CUSTOM: i32 = 20;
pub const SERVICE_TRIGGER_TYPE_CUSTOM_SYSTEM_STATE_CHANGE: i32 = 7;
pub const SERVICE_TRIGGER_TYPE_DEVICE_INTERFACE_ARRIVAL: i32 = 1;
pub const SERVICE_TRIGGER_TYPE_DOMAIN_JOIN: i32 = 3;
pub const SERVICE_TRIGGER_TYPE_FIREWALL_PORT_EVENT: i32 = 4;
pub const SERVICE_TRIGGER_TYPE_GROUP_POLICY: i32 = 5;
pub const SERVICE_TRIGGER_TYPE_IP_ADDRESS_AVAILABILITY: i32 = 2;
pub const SERVICE_TRIGGER_TYPE_NETWORK_ENDPOINT: i32 = 6;
pub const SERVICE_USER_DEFINED_CONTROL: i32 = 256;
pub const ServiceDirectoryPersistentState: SERVICE_DIRECTORY_TYPE = 0;
pub const ServiceDirectoryTypeMax: SERVICE_DIRECTORY_TYPE = 1;
pub const ServiceRegistryStateParameters: SERVICE_REGISTRY_STATE_TYPE = 0;
pub const ServiceRegistryStatePersistent: SERVICE_REGISTRY_STATE_TYPE = 1;
pub const ServiceSharedDirectoryPersistentState: SERVICE_SHARED_DIRECTORY_TYPE = 0;
pub const ServiceSharedRegistryPersistentState: SERVICE_SHARED_REGISTRY_STATE_TYPE = 0;
pub const USER_POLICY_PRESENT_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x54fb46c8_f089_464c_b1fd_59d1b62c3b50);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _SC_NOTIFICATION_REGISTRATION(pub u8);
