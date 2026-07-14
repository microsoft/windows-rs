#[inline]
pub unsafe fn ChangeServiceConfig2A(hservice: SC_HANDLE, dwinfolevel: u32, lpinfo: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ChangeServiceConfig2A(hservice : SC_HANDLE, dwinfolevel : u32, lpinfo : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ChangeServiceConfig2A(hservice, dwinfolevel, lpinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ChangeServiceConfig2W(hservice: SC_HANDLE, dwinfolevel: u32, lpinfo: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ChangeServiceConfig2W(hservice : SC_HANDLE, dwinfolevel : u32, lpinfo : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ChangeServiceConfig2W(hservice, dwinfolevel, lpinfo.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ChangeServiceConfigA<P4, P5, P7, P8, P9, P10>(hservice: SC_HANDLE, dwservicetype: u32, dwstarttype: u32, dwerrorcontrol: u32, lpbinarypathname: P4, lploadordergroup: P5, lpdwtagid: Option<*mut u32>, lpdependencies: P7, lpservicestartname: P8, lppassword: P9, lpdisplayname: P10) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P7: windows_core::Param<windows_core::PCSTR>,
    P8: windows_core::Param<windows_core::PCSTR>,
    P9: windows_core::Param<windows_core::PCSTR>,
    P10: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ChangeServiceConfigA(hservice : SC_HANDLE, dwservicetype : u32, dwstarttype : u32, dwerrorcontrol : u32, lpbinarypathname : windows_core::PCSTR, lploadordergroup : windows_core::PCSTR, lpdwtagid : *mut u32, lpdependencies : windows_core::PCSTR, lpservicestartname : windows_core::PCSTR, lppassword : windows_core::PCSTR, lpdisplayname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { ChangeServiceConfigA(hservice, dwservicetype, dwstarttype, dwerrorcontrol, lpbinarypathname.param().abi(), lploadordergroup.param().abi(), lpdwtagid.unwrap_or(core::mem::zeroed()) as _, lpdependencies.param().abi(), lpservicestartname.param().abi(), lppassword.param().abi(), lpdisplayname.param().abi()) }
}
#[inline]
pub unsafe fn ChangeServiceConfigW<P4, P5, P7, P8, P9, P10>(hservice: SC_HANDLE, dwservicetype: u32, dwstarttype: u32, dwerrorcontrol: u32, lpbinarypathname: P4, lploadordergroup: P5, lpdwtagid: Option<*mut u32>, lpdependencies: P7, lpservicestartname: P8, lppassword: P9, lpdisplayname: P10) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<windows_core::PCWSTR>,
    P9: windows_core::Param<windows_core::PCWSTR>,
    P10: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ChangeServiceConfigW(hservice : SC_HANDLE, dwservicetype : u32, dwstarttype : u32, dwerrorcontrol : u32, lpbinarypathname : windows_core::PCWSTR, lploadordergroup : windows_core::PCWSTR, lpdwtagid : *mut u32, lpdependencies : windows_core::PCWSTR, lpservicestartname : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, lpdisplayname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ChangeServiceConfigW(hservice, dwservicetype, dwstarttype, dwerrorcontrol, lpbinarypathname.param().abi(), lploadordergroup.param().abi(), lpdwtagid.unwrap_or(core::mem::zeroed()) as _, lpdependencies.param().abi(), lpservicestartname.param().abi(), lppassword.param().abi(), lpdisplayname.param().abi()) }
}
#[inline]
pub unsafe fn CloseServiceHandle(hscobject: SC_HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CloseServiceHandle(hscobject : SC_HANDLE) -> windows_core::BOOL);
    unsafe { CloseServiceHandle(hscobject) }
}
#[inline]
pub unsafe fn ControlService(hservice: SC_HANDLE, dwcontrol: u32, lpservicestatus: *mut SERVICE_STATUS) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ControlService(hservice : SC_HANDLE, dwcontrol : u32, lpservicestatus : *mut SERVICE_STATUS) -> windows_core::BOOL);
    unsafe { ControlService(hservice, dwcontrol, lpservicestatus as _) }
}
#[inline]
pub unsafe fn ControlServiceExA(hservice: SC_HANDLE, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ControlServiceExA(hservice : SC_HANDLE, dwcontrol : u32, dwinfolevel : u32, pcontrolparams : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ControlServiceExA(hservice, dwcontrol, dwinfolevel, pcontrolparams as _) }
}
#[inline]
pub unsafe fn ControlServiceExW(hservice: SC_HANDLE, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ControlServiceExW(hservice : SC_HANDLE, dwcontrol : u32, dwinfolevel : u32, pcontrolparams : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ControlServiceExW(hservice, dwcontrol, dwinfolevel, pcontrolparams as _) }
}
#[inline]
pub unsafe fn CreateServiceA<P1, P2, P7, P8, P10, P11, P12>(hscmanager: SC_HANDLE, lpservicename: P1, lpdisplayname: P2, dwdesiredaccess: u32, dwservicetype: u32, dwstarttype: u32, dwerrorcontrol: u32, lpbinarypathname: P7, lploadordergroup: P8, lpdwtagid: Option<*mut u32>, lpdependencies: P10, lpservicestartname: P11, lppassword: P12) -> SC_HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P7: windows_core::Param<windows_core::PCSTR>,
    P8: windows_core::Param<windows_core::PCSTR>,
    P10: windows_core::Param<windows_core::PCSTR>,
    P11: windows_core::Param<windows_core::PCSTR>,
    P12: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CreateServiceA(hscmanager : SC_HANDLE, lpservicename : windows_core::PCSTR, lpdisplayname : windows_core::PCSTR, dwdesiredaccess : u32, dwservicetype : u32, dwstarttype : u32, dwerrorcontrol : u32, lpbinarypathname : windows_core::PCSTR, lploadordergroup : windows_core::PCSTR, lpdwtagid : *mut u32, lpdependencies : windows_core::PCSTR, lpservicestartname : windows_core::PCSTR, lppassword : windows_core::PCSTR) -> SC_HANDLE);
    unsafe { CreateServiceA(hscmanager, lpservicename.param().abi(), lpdisplayname.param().abi(), dwdesiredaccess, dwservicetype, dwstarttype, dwerrorcontrol, lpbinarypathname.param().abi(), lploadordergroup.param().abi(), lpdwtagid.unwrap_or(core::mem::zeroed()) as _, lpdependencies.param().abi(), lpservicestartname.param().abi(), lppassword.param().abi()) }
}
#[inline]
pub unsafe fn CreateServiceW<P1, P2, P7, P8, P10, P11, P12>(hscmanager: SC_HANDLE, lpservicename: P1, lpdisplayname: P2, dwdesiredaccess: u32, dwservicetype: u32, dwstarttype: u32, dwerrorcontrol: u32, lpbinarypathname: P7, lploadordergroup: P8, lpdwtagid: Option<*mut u32>, lpdependencies: P10, lpservicestartname: P11, lppassword: P12) -> SC_HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<windows_core::PCWSTR>,
    P10: windows_core::Param<windows_core::PCWSTR>,
    P11: windows_core::Param<windows_core::PCWSTR>,
    P12: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CreateServiceW(hscmanager : SC_HANDLE, lpservicename : windows_core::PCWSTR, lpdisplayname : windows_core::PCWSTR, dwdesiredaccess : u32, dwservicetype : u32, dwstarttype : u32, dwerrorcontrol : u32, lpbinarypathname : windows_core::PCWSTR, lploadordergroup : windows_core::PCWSTR, lpdwtagid : *mut u32, lpdependencies : windows_core::PCWSTR, lpservicestartname : windows_core::PCWSTR, lppassword : windows_core::PCWSTR) -> SC_HANDLE);
    unsafe { CreateServiceW(hscmanager, lpservicename.param().abi(), lpdisplayname.param().abi(), dwdesiredaccess, dwservicetype, dwstarttype, dwerrorcontrol, lpbinarypathname.param().abi(), lploadordergroup.param().abi(), lpdwtagid.unwrap_or(core::mem::zeroed()) as _, lpdependencies.param().abi(), lpservicestartname.param().abi(), lppassword.param().abi()) }
}
#[inline]
pub unsafe fn DeleteService(hservice: SC_HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn DeleteService(hservice : SC_HANDLE) -> windows_core::BOOL);
    unsafe { DeleteService(hservice) }
}
#[inline]
pub unsafe fn EnumDependentServicesA(hservice: SC_HANDLE, dwservicestate: u32, lpservices: Option<*mut ENUM_SERVICE_STATUSA>, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn EnumDependentServicesA(hservice : SC_HANDLE, dwservicestate : u32, lpservices : *mut ENUM_SERVICE_STATUSA, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumDependentServicesA(hservice, dwservicestate, lpservices.unwrap_or(core::mem::zeroed()) as _, cbbufsize, pcbbytesneeded as _, lpservicesreturned as _) }
}
#[inline]
pub unsafe fn EnumDependentServicesW(hservice: SC_HANDLE, dwservicestate: u32, lpservices: Option<*mut ENUM_SERVICE_STATUSW>, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn EnumDependentServicesW(hservice : SC_HANDLE, dwservicestate : u32, lpservices : *mut ENUM_SERVICE_STATUSW, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32) -> windows_core::BOOL);
    unsafe { EnumDependentServicesW(hservice, dwservicestate, lpservices.unwrap_or(core::mem::zeroed()) as _, cbbufsize, pcbbytesneeded as _, lpservicesreturned as _) }
}
#[inline]
pub unsafe fn EnumServicesStatusA(hscmanager: SC_HANDLE, dwservicetype: u32, dwservicestate: u32, lpservices: Option<*mut ENUM_SERVICE_STATUSA>, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn EnumServicesStatusA(hscmanager : SC_HANDLE, dwservicetype : u32, dwservicestate : u32, lpservices : *mut ENUM_SERVICE_STATUSA, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32, lpresumehandle : *mut u32) -> windows_core::BOOL);
    unsafe { EnumServicesStatusA(hscmanager, dwservicetype, dwservicestate, lpservices.unwrap_or(core::mem::zeroed()) as _, cbbufsize, pcbbytesneeded as _, lpservicesreturned as _, lpresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn EnumServicesStatusExA<P9>(hscmanager: SC_HANDLE, infolevel: SC_ENUM_TYPE, dwservicetype: u32, dwservicestate: u32, lpservices: Option<&mut [u8]>, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: Option<*mut u32>, pszgroupname: P9) -> windows_core::BOOL
where
    P9: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn EnumServicesStatusExA(hscmanager : SC_HANDLE, infolevel : SC_ENUM_TYPE, dwservicetype : u32, dwservicestate : u32, lpservices : *mut u8, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32, lpresumehandle : *mut u32, pszgroupname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { EnumServicesStatusExA(hscmanager, infolevel, dwservicetype, dwservicestate, lpservices.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), lpservices.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbbytesneeded as _, lpservicesreturned as _, lpresumehandle.unwrap_or(core::mem::zeroed()) as _, pszgroupname.param().abi()) }
}
#[inline]
pub unsafe fn EnumServicesStatusExW<P9>(hscmanager: SC_HANDLE, infolevel: SC_ENUM_TYPE, dwservicetype: u32, dwservicestate: u32, lpservices: Option<&mut [u8]>, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: Option<*mut u32>, pszgroupname: P9) -> windows_core::BOOL
where
    P9: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn EnumServicesStatusExW(hscmanager : SC_HANDLE, infolevel : SC_ENUM_TYPE, dwservicetype : u32, dwservicestate : u32, lpservices : *mut u8, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32, lpresumehandle : *mut u32, pszgroupname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { EnumServicesStatusExW(hscmanager, infolevel, dwservicetype, dwservicestate, lpservices.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), lpservices.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbbytesneeded as _, lpservicesreturned as _, lpresumehandle.unwrap_or(core::mem::zeroed()) as _, pszgroupname.param().abi()) }
}
#[inline]
pub unsafe fn EnumServicesStatusW(hscmanager: SC_HANDLE, dwservicetype: u32, dwservicestate: u32, lpservices: Option<*mut ENUM_SERVICE_STATUSW>, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn EnumServicesStatusW(hscmanager : SC_HANDLE, dwservicetype : u32, dwservicestate : u32, lpservices : *mut ENUM_SERVICE_STATUSW, cbbufsize : u32, pcbbytesneeded : *mut u32, lpservicesreturned : *mut u32, lpresumehandle : *mut u32) -> windows_core::BOOL);
    unsafe { EnumServicesStatusW(hscmanager, dwservicetype, dwservicestate, lpservices.unwrap_or(core::mem::zeroed()) as _, cbbufsize, pcbbytesneeded as _, lpservicesreturned as _, lpresumehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetServiceDirectory(hservicestatus: SERVICE_STATUS_HANDLE, edirectorytype: SERVICE_DIRECTORY_TYPE, lppathbuffer: Option<&mut [u16]>, lpcchrequiredbufferlength: *mut u32) -> u32 {
    windows_core::link!("api-ms-win-service-core-l1-1-4.dll" "system" fn GetServiceDirectory(hservicestatus : SERVICE_STATUS_HANDLE, edirectorytype : SERVICE_DIRECTORY_TYPE, lppathbuffer : *mut u16, cchpathbufferlength : u32, lpcchrequiredbufferlength : *mut u32) -> u32);
    unsafe { GetServiceDirectory(hservicestatus, edirectorytype, lppathbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), lppathbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), lpcchrequiredbufferlength as _) }
}
#[inline]
pub unsafe fn GetServiceDisplayNameA<P1>(hscmanager: SC_HANDLE, lpservicename: P1, lpdisplayname: Option<windows_core::PSTR>, lpcchbuffer: *mut u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetServiceDisplayNameA(hscmanager : SC_HANDLE, lpservicename : windows_core::PCSTR, lpdisplayname : windows_core::PSTR, lpcchbuffer : *mut u32) -> windows_core::BOOL);
    unsafe { GetServiceDisplayNameA(hscmanager, lpservicename.param().abi(), lpdisplayname.unwrap_or(core::mem::zeroed()) as _, lpcchbuffer as _) }
}
#[inline]
pub unsafe fn GetServiceDisplayNameW<P1>(hscmanager: SC_HANDLE, lpservicename: P1, lpdisplayname: Option<windows_core::PWSTR>, lpcchbuffer: *mut u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetServiceDisplayNameW(hscmanager : SC_HANDLE, lpservicename : windows_core::PCWSTR, lpdisplayname : windows_core::PWSTR, lpcchbuffer : *mut u32) -> windows_core::BOOL);
    unsafe { GetServiceDisplayNameW(hscmanager, lpservicename.param().abi(), lpdisplayname.unwrap_or(core::mem::zeroed()) as _, lpcchbuffer as _) }
}
#[inline]
pub unsafe fn GetServiceKeyNameA<P1>(hscmanager: SC_HANDLE, lpdisplayname: P1, lpservicename: Option<windows_core::PSTR>, lpcchbuffer: *mut u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetServiceKeyNameA(hscmanager : SC_HANDLE, lpdisplayname : windows_core::PCSTR, lpservicename : windows_core::PSTR, lpcchbuffer : *mut u32) -> windows_core::BOOL);
    unsafe { GetServiceKeyNameA(hscmanager, lpdisplayname.param().abi(), lpservicename.unwrap_or(core::mem::zeroed()) as _, lpcchbuffer as _) }
}
#[inline]
pub unsafe fn GetServiceKeyNameW<P1>(hscmanager: SC_HANDLE, lpdisplayname: P1, lpservicename: Option<windows_core::PWSTR>, lpcchbuffer: *mut u32) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetServiceKeyNameW(hscmanager : SC_HANDLE, lpdisplayname : windows_core::PCWSTR, lpservicename : windows_core::PWSTR, lpcchbuffer : *mut u32) -> windows_core::BOOL);
    unsafe { GetServiceKeyNameW(hscmanager, lpdisplayname.param().abi(), lpservicename.unwrap_or(core::mem::zeroed()) as _, lpcchbuffer as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetServiceRegistryStateKey(servicestatushandle: SERVICE_STATUS_HANDLE, statetype: SERVICE_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::minwindef::HKEY) -> u32 {
    windows_core::link!("api-ms-win-service-core-l1-1-3.dll" "system" fn GetServiceRegistryStateKey(servicestatushandle : SERVICE_STATUS_HANDLE, statetype : SERVICE_REGISTRY_STATE_TYPE, accessmask : u32, servicestatekey : *mut super::minwindef::HKEY) -> u32);
    unsafe { GetServiceRegistryStateKey(servicestatushandle, statetype, accessmask, servicestatekey as _) }
}
#[inline]
pub unsafe fn GetSharedServiceDirectory(servicehandle: SC_HANDLE, directorytype: SERVICE_SHARED_DIRECTORY_TYPE, pathbuffer: Option<&mut [u16]>, requiredbufferlength: *mut u32) -> u32 {
    windows_core::link!("api-ms-win-service-core-l1-1-5.dll" "system" fn GetSharedServiceDirectory(servicehandle : SC_HANDLE, directorytype : SERVICE_SHARED_DIRECTORY_TYPE, pathbuffer : *mut u16, pathbufferlength : u32, requiredbufferlength : *mut u32) -> u32);
    unsafe { GetSharedServiceDirectory(servicehandle, directorytype, pathbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), pathbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredbufferlength as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetSharedServiceRegistryStateKey(servicehandle: SC_HANDLE, statetype: SERVICE_SHARED_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::minwindef::HKEY) -> u32 {
    windows_core::link!("api-ms-win-service-core-l1-1-5.dll" "system" fn GetSharedServiceRegistryStateKey(servicehandle : SC_HANDLE, statetype : SERVICE_SHARED_REGISTRY_STATE_TYPE, accessmask : u32, servicestatekey : *mut super::minwindef::HKEY) -> u32);
    unsafe { GetSharedServiceRegistryStateKey(servicehandle, statetype, accessmask, servicestatekey as _) }
}
#[inline]
pub unsafe fn LockServiceDatabase(hscmanager: SC_HANDLE) -> SC_LOCK {
    windows_core::link!("advapi32.dll" "system" fn LockServiceDatabase(hscmanager : SC_HANDLE) -> SC_LOCK);
    unsafe { LockServiceDatabase(hscmanager) }
}
#[inline]
pub unsafe fn NotifyBootConfigStatus(bootacceptable: bool) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn NotifyBootConfigStatus(bootacceptable : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { NotifyBootConfigStatus(bootacceptable.into()) }
}
#[inline]
pub unsafe fn NotifyServiceStatusChangeA(hservice: SC_HANDLE, dwnotifymask: u32, pnotifybuffer: *const SERVICE_NOTIFY_2A) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn NotifyServiceStatusChangeA(hservice : SC_HANDLE, dwnotifymask : u32, pnotifybuffer : *const SERVICE_NOTIFY_2A) -> u32);
    unsafe { NotifyServiceStatusChangeA(hservice, dwnotifymask, pnotifybuffer) }
}
#[inline]
pub unsafe fn NotifyServiceStatusChangeW(hservice: SC_HANDLE, dwnotifymask: u32, pnotifybuffer: *const SERVICE_NOTIFY_2W) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn NotifyServiceStatusChangeW(hservice : SC_HANDLE, dwnotifymask : u32, pnotifybuffer : *const SERVICE_NOTIFY_2W) -> u32);
    unsafe { NotifyServiceStatusChangeW(hservice, dwnotifymask, pnotifybuffer) }
}
#[inline]
pub unsafe fn OpenSCManagerA<P0, P1>(lpmachinename: P0, lpdatabasename: P1, dwdesiredaccess: u32) -> SC_HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn OpenSCManagerA(lpmachinename : windows_core::PCSTR, lpdatabasename : windows_core::PCSTR, dwdesiredaccess : u32) -> SC_HANDLE);
    unsafe { OpenSCManagerA(lpmachinename.param().abi(), lpdatabasename.param().abi(), dwdesiredaccess) }
}
#[inline]
pub unsafe fn OpenSCManagerW<P0, P1>(lpmachinename: P0, lpdatabasename: P1, dwdesiredaccess: u32) -> SC_HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn OpenSCManagerW(lpmachinename : windows_core::PCWSTR, lpdatabasename : windows_core::PCWSTR, dwdesiredaccess : u32) -> SC_HANDLE);
    unsafe { OpenSCManagerW(lpmachinename.param().abi(), lpdatabasename.param().abi(), dwdesiredaccess) }
}
#[inline]
pub unsafe fn OpenServiceA<P1>(hscmanager: SC_HANDLE, lpservicename: P1, dwdesiredaccess: u32) -> SC_HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn OpenServiceA(hscmanager : SC_HANDLE, lpservicename : windows_core::PCSTR, dwdesiredaccess : u32) -> SC_HANDLE);
    unsafe { OpenServiceA(hscmanager, lpservicename.param().abi(), dwdesiredaccess) }
}
#[inline]
pub unsafe fn OpenServiceW<P1>(hscmanager: SC_HANDLE, lpservicename: P1, dwdesiredaccess: u32) -> SC_HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn OpenServiceW(hscmanager : SC_HANDLE, lpservicename : windows_core::PCWSTR, dwdesiredaccess : u32) -> SC_HANDLE);
    unsafe { OpenServiceW(hscmanager, lpservicename.param().abi(), dwdesiredaccess) }
}
#[inline]
pub unsafe fn QueryServiceConfig2A(hservice: SC_HANDLE, dwinfolevel: u32, lpbuffer: Option<&mut [u8]>, pcbbytesneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn QueryServiceConfig2A(hservice : SC_HANDLE, dwinfolevel : u32, lpbuffer : *mut u8, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_core::BOOL);
    unsafe { QueryServiceConfig2A(hservice, dwinfolevel, lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbbytesneeded as _) }
}
#[inline]
pub unsafe fn QueryServiceConfig2W(hservice: SC_HANDLE, dwinfolevel: u32, lpbuffer: Option<&mut [u8]>, pcbbytesneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn QueryServiceConfig2W(hservice : SC_HANDLE, dwinfolevel : u32, lpbuffer : *mut u8, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_core::BOOL);
    unsafe { QueryServiceConfig2W(hservice, dwinfolevel, lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbbytesneeded as _) }
}
#[inline]
pub unsafe fn QueryServiceConfigA(hservice: SC_HANDLE, lpserviceconfig: Option<*mut QUERY_SERVICE_CONFIGA>, cbbufsize: u32, pcbbytesneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn QueryServiceConfigA(hservice : SC_HANDLE, lpserviceconfig : *mut QUERY_SERVICE_CONFIGA, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_core::BOOL);
    unsafe { QueryServiceConfigA(hservice, lpserviceconfig.unwrap_or(core::mem::zeroed()) as _, cbbufsize, pcbbytesneeded as _) }
}
#[inline]
pub unsafe fn QueryServiceConfigW(hservice: SC_HANDLE, lpserviceconfig: Option<*mut QUERY_SERVICE_CONFIGW>, cbbufsize: u32, pcbbytesneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn QueryServiceConfigW(hservice : SC_HANDLE, lpserviceconfig : *mut QUERY_SERVICE_CONFIGW, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_core::BOOL);
    unsafe { QueryServiceConfigW(hservice, lpserviceconfig.unwrap_or(core::mem::zeroed()) as _, cbbufsize, pcbbytesneeded as _) }
}
#[inline]
pub unsafe fn QueryServiceDynamicInformation(hservicestatus: SERVICE_STATUS_HANDLE, dwinfolevel: u32, ppdynamicinfo: *mut *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn QueryServiceDynamicInformation(hservicestatus : SERVICE_STATUS_HANDLE, dwinfolevel : u32, ppdynamicinfo : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { QueryServiceDynamicInformation(hservicestatus, dwinfolevel, ppdynamicinfo as _) }
}
#[inline]
pub unsafe fn QueryServiceLockStatusA(hscmanager: SC_HANDLE, lplockstatus: Option<*mut QUERY_SERVICE_LOCK_STATUSA>, cbbufsize: u32, pcbbytesneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn QueryServiceLockStatusA(hscmanager : SC_HANDLE, lplockstatus : *mut QUERY_SERVICE_LOCK_STATUSA, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_core::BOOL);
    unsafe { QueryServiceLockStatusA(hscmanager, lplockstatus.unwrap_or(core::mem::zeroed()) as _, cbbufsize, pcbbytesneeded as _) }
}
#[inline]
pub unsafe fn QueryServiceLockStatusW(hscmanager: SC_HANDLE, lplockstatus: Option<*mut QUERY_SERVICE_LOCK_STATUSW>, cbbufsize: u32, pcbbytesneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn QueryServiceLockStatusW(hscmanager : SC_HANDLE, lplockstatus : *mut QUERY_SERVICE_LOCK_STATUSW, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_core::BOOL);
    unsafe { QueryServiceLockStatusW(hscmanager, lplockstatus.unwrap_or(core::mem::zeroed()) as _, cbbufsize, pcbbytesneeded as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryServiceObjectSecurity(hservice: SC_HANDLE, dwsecurityinformation: super::winnt::SECURITY_INFORMATION, lpsecuritydescriptor: Option<super::winnt::PSECURITY_DESCRIPTOR>, cbbufsize: u32, pcbbytesneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn QueryServiceObjectSecurity(hservice : SC_HANDLE, dwsecurityinformation : super::winnt::SECURITY_INFORMATION, lpsecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_core::BOOL);
    unsafe { QueryServiceObjectSecurity(hservice, dwsecurityinformation, lpsecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, cbbufsize, pcbbytesneeded as _) }
}
#[inline]
pub unsafe fn QueryServiceStatus(hservice: SC_HANDLE, lpservicestatus: *mut SERVICE_STATUS) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn QueryServiceStatus(hservice : SC_HANDLE, lpservicestatus : *mut SERVICE_STATUS) -> windows_core::BOOL);
    unsafe { QueryServiceStatus(hservice, lpservicestatus as _) }
}
#[inline]
pub unsafe fn QueryServiceStatusEx(hservice: SC_HANDLE, infolevel: SC_STATUS_TYPE, lpbuffer: Option<&mut [u8]>, pcbbytesneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn QueryServiceStatusEx(hservice : SC_HANDLE, infolevel : SC_STATUS_TYPE, lpbuffer : *mut u8, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_core::BOOL);
    unsafe { QueryServiceStatusEx(hservice, infolevel, lpbuffer.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), lpbuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), pcbbytesneeded as _) }
}
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerA<P0>(lpservicename: P0, lphandlerproc: LPHANDLER_FUNCTION) -> SERVICE_STATUS_HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerA(lpservicename : windows_core::PCSTR, lphandlerproc : LPHANDLER_FUNCTION) -> SERVICE_STATUS_HANDLE);
    unsafe { RegisterServiceCtrlHandlerA(lpservicename.param().abi(), lphandlerproc) }
}
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerExA<P0>(lpservicename: P0, lphandlerproc: LPHANDLER_FUNCTION_EX, lpcontext: Option<*const core::ffi::c_void>) -> SERVICE_STATUS_HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerExA(lpservicename : windows_core::PCSTR, lphandlerproc : LPHANDLER_FUNCTION_EX, lpcontext : *const core::ffi::c_void) -> SERVICE_STATUS_HANDLE);
    unsafe { RegisterServiceCtrlHandlerExA(lpservicename.param().abi(), lphandlerproc, lpcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerExW<P0>(lpservicename: P0, lphandlerproc: LPHANDLER_FUNCTION_EX, lpcontext: Option<*const core::ffi::c_void>) -> SERVICE_STATUS_HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerExW(lpservicename : windows_core::PCWSTR, lphandlerproc : LPHANDLER_FUNCTION_EX, lpcontext : *const core::ffi::c_void) -> SERVICE_STATUS_HANDLE);
    unsafe { RegisterServiceCtrlHandlerExW(lpservicename.param().abi(), lphandlerproc, lpcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerW<P0>(lpservicename: P0, lphandlerproc: LPHANDLER_FUNCTION) -> SERVICE_STATUS_HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerW(lpservicename : windows_core::PCWSTR, lphandlerproc : LPHANDLER_FUNCTION) -> SERVICE_STATUS_HANDLE);
    unsafe { RegisterServiceCtrlHandlerW(lpservicename.param().abi(), lphandlerproc) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetServiceObjectSecurity(hservice: SC_HANDLE, dwsecurityinformation: super::winnt::SECURITY_INFORMATION, lpsecuritydescriptor: super::winnt::PSECURITY_DESCRIPTOR) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetServiceObjectSecurity(hservice : SC_HANDLE, dwsecurityinformation : super::winnt::SECURITY_INFORMATION, lpsecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR) -> windows_core::BOOL);
    unsafe { SetServiceObjectSecurity(hservice, dwsecurityinformation, lpsecuritydescriptor) }
}
#[inline]
pub unsafe fn SetServiceStatus(hservicestatus: SERVICE_STATUS_HANDLE, lpservicestatus: *const SERVICE_STATUS) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SetServiceStatus(hservicestatus : SERVICE_STATUS_HANDLE, lpservicestatus : *const SERVICE_STATUS) -> windows_core::BOOL);
    unsafe { SetServiceStatus(hservicestatus, lpservicestatus) }
}
#[inline]
pub unsafe fn StartServiceA(hservice: SC_HANDLE, lpserviceargvectors: Option<&[windows_core::PCSTR]>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn StartServiceA(hservice : SC_HANDLE, dwnumserviceargs : u32, lpserviceargvectors : *const windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { StartServiceA(hservice, lpserviceargvectors.map_or(0, |slice| slice.len().try_into().unwrap()), lpserviceargvectors.map_or(core::ptr::null(), |slice| slice.as_ptr())) }
}
#[inline]
pub unsafe fn StartServiceCtrlDispatcherA(lpservicestarttable: *const SERVICE_TABLE_ENTRYA) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn StartServiceCtrlDispatcherA(lpservicestarttable : *const SERVICE_TABLE_ENTRYA) -> windows_core::BOOL);
    unsafe { StartServiceCtrlDispatcherA(lpservicestarttable) }
}
#[inline]
pub unsafe fn StartServiceCtrlDispatcherW(lpservicestarttable: *const SERVICE_TABLE_ENTRYW) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn StartServiceCtrlDispatcherW(lpservicestarttable : *const SERVICE_TABLE_ENTRYW) -> windows_core::BOOL);
    unsafe { StartServiceCtrlDispatcherW(lpservicestarttable) }
}
#[inline]
pub unsafe fn StartServiceW(hservice: SC_HANDLE, lpserviceargvectors: Option<&[windows_core::PCWSTR]>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn StartServiceW(hservice : SC_HANDLE, dwnumserviceargs : u32, lpserviceargvectors : *const windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { StartServiceW(hservice, lpserviceargvectors.map_or(0, |slice| slice.len().try_into().unwrap()), lpserviceargvectors.map_or(core::ptr::null(), |slice| slice.as_ptr())) }
}
#[inline]
pub unsafe fn UnlockServiceDatabase(sclock: SC_LOCK) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn UnlockServiceDatabase(sclock : SC_LOCK) -> windows_core::BOOL);
    unsafe { UnlockServiceDatabase(sclock) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WaitServiceState(hservice: SC_HANDLE, dwnotify: u32, dwtimeout: Option<u32>, hcancelevent: Option<super::winnt::HANDLE>) -> u32 {
    windows_core::link!("advapi32.dll" "system" fn WaitServiceState(hservice : SC_HANDLE, dwnotify : u32, dwtimeout : u32, hcancelevent : super::winnt::HANDLE) -> u32);
    unsafe { WaitServiceState(hservice, dwnotify, dwtimeout.unwrap_or(core::mem::zeroed()) as _, hcancelevent.unwrap_or(core::mem::zeroed()) as _) }
}
pub const CUSTOM_SYSTEM_STATE_CHANGE_EVENT_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2d7a2816_0c5e_45fc_9ce7_570e5ecde9c9);
pub const DOMAIN_JOIN_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1ce20aba_9851_4421_9430_1ddeb766e809);
pub const DOMAIN_LEAVE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xddaf516e_58c2_4866_9574_c3b615d42ea1);
pub type ENUM_SERVICE_STATUS = ENUM_SERVICE_STATUSA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENUM_SERVICE_STATUSA {
    pub lpServiceName: windows_core::PSTR,
    pub lpDisplayName: windows_core::PSTR,
    pub ServiceStatus: SERVICE_STATUS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENUM_SERVICE_STATUSW {
    pub lpServiceName: windows_core::PWSTR,
    pub lpDisplayName: windows_core::PWSTR,
    pub ServiceStatus: SERVICE_STATUS,
}
pub type ENUM_SERVICE_STATUS_PROCESS = ENUM_SERVICE_STATUS_PROCESSA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENUM_SERVICE_STATUS_PROCESSA {
    pub lpServiceName: windows_core::PSTR,
    pub lpDisplayName: windows_core::PSTR,
    pub ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENUM_SERVICE_STATUS_PROCESSW {
    pub lpServiceName: windows_core::PWSTR,
    pub lpDisplayName: windows_core::PWSTR,
    pub ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}
pub const FIREWALL_PORT_CLOSE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa144ed38_8e12_4de4_9d96_e64740b1a524);
pub const FIREWALL_PORT_OPEN_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb7569e07_8421_4ee0_ad10_86915afdad09);
pub type HANDLER_FUNCTION = Option<unsafe extern "system" fn(dwcontrol: u32)>;
pub type HANDLER_FUNCTION_EX = Option<unsafe extern "system" fn(dwcontrol: u32, dweventtype: u32, lpeventdata: *mut core::ffi::c_void, lpcontext: *mut core::ffi::c_void) -> u32>;
pub type LPENUM_SERVICE_STATUS = LPENUM_SERVICE_STATUSA;
pub type LPENUM_SERVICE_STATUSA = *mut ENUM_SERVICE_STATUSA;
pub type LPENUM_SERVICE_STATUSW = *mut ENUM_SERVICE_STATUSW;
pub type LPENUM_SERVICE_STATUS_PROCESS = LPENUM_SERVICE_STATUS_PROCESSA;
pub type LPENUM_SERVICE_STATUS_PROCESSA = *mut ENUM_SERVICE_STATUS_PROCESSA;
pub type LPENUM_SERVICE_STATUS_PROCESSW = *mut ENUM_SERVICE_STATUS_PROCESSW;
pub type LPHANDLER_FUNCTION = Option<unsafe extern "system" fn(dwcontrol: u32)>;
pub type LPHANDLER_FUNCTION_EX = Option<unsafe extern "system" fn(dwcontrol: u32, dweventtype: u32, lpeventdata: *mut core::ffi::c_void, lpcontext: *mut core::ffi::c_void) -> u32>;
pub type LPQUERY_SERVICE_CONFIG = LPQUERY_SERVICE_CONFIGA;
pub type LPQUERY_SERVICE_CONFIGA = *mut QUERY_SERVICE_CONFIGA;
pub type LPQUERY_SERVICE_CONFIGW = *mut QUERY_SERVICE_CONFIGW;
pub type LPQUERY_SERVICE_LOCK_STATUS = LPQUERY_SERVICE_LOCK_STATUSA;
pub type LPQUERY_SERVICE_LOCK_STATUSA = *mut QUERY_SERVICE_LOCK_STATUSA;
pub type LPQUERY_SERVICE_LOCK_STATUSW = *mut QUERY_SERVICE_LOCK_STATUSW;
pub type LPSC_ACTION = *mut SC_ACTION;
pub type LPSC_HANDLE = *mut SC_HANDLE;
pub type LPSERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM = *mut SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM;
pub type LPSERVICE_DELAYED_AUTO_START_INFO = *mut SERVICE_DELAYED_AUTO_START_INFO;
pub type LPSERVICE_DESCRIPTION = LPSERVICE_DESCRIPTIONA;
pub type LPSERVICE_DESCRIPTIONA = *mut SERVICE_DESCRIPTIONA;
pub type LPSERVICE_DESCRIPTIONW = *mut SERVICE_DESCRIPTIONW;
pub type LPSERVICE_FAILURE_ACTIONS = LPSERVICE_FAILURE_ACTIONSA;
pub type LPSERVICE_FAILURE_ACTIONSA = *mut SERVICE_FAILURE_ACTIONSA;
pub type LPSERVICE_FAILURE_ACTIONSW = *mut SERVICE_FAILURE_ACTIONSW;
pub type LPSERVICE_FAILURE_ACTIONS_FLAG = *mut SERVICE_FAILURE_ACTIONS_FLAG;
pub type LPSERVICE_MAIN_FUNCTIONA = Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut windows_core::PSTR)>;
pub type LPSERVICE_MAIN_FUNCTIONW = Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut windows_core::PWSTR)>;
pub type LPSERVICE_PREFERRED_NODE_INFO = *mut SERVICE_PREFERRED_NODE_INFO;
pub type LPSERVICE_PRESHUTDOWN_INFO = *mut SERVICE_PRESHUTDOWN_INFO;
pub type LPSERVICE_REQUIRED_PRIVILEGES_INFO = LPSERVICE_REQUIRED_PRIVILEGES_INFOA;
pub type LPSERVICE_REQUIRED_PRIVILEGES_INFOA = *mut SERVICE_REQUIRED_PRIVILEGES_INFOA;
pub type LPSERVICE_REQUIRED_PRIVILEGES_INFOW = *mut SERVICE_REQUIRED_PRIVILEGES_INFOW;
pub type LPSERVICE_SID_INFO = *mut SERVICE_SID_INFO;
pub type LPSERVICE_STATUS = *mut SERVICE_STATUS;
pub type LPSERVICE_STATUS_PROCESS = *mut SERVICE_STATUS_PROCESS;
pub type LPSERVICE_TABLE_ENTRY = LPSERVICE_TABLE_ENTRYA;
pub type LPSERVICE_TABLE_ENTRYA = *mut SERVICE_TABLE_ENTRYA;
pub type LPSERVICE_TABLE_ENTRYW = *mut SERVICE_TABLE_ENTRYW;
pub const MACHINE_POLICY_PRESENT_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x659fcae6_5bdb_4da9_b1ff_ca2a178d46e0);
pub const MaxServiceRegistryStateType: SERVICE_REGISTRY_STATE_TYPE = 2;
pub const NAMED_PIPE_EVENT_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1f81d131_3fac_4537_9e0c_7e7b0c2f4b55);
pub const NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4f27f2de_14e2_430b_a549_7cd48cbc8245);
pub const NETWORK_MANAGER_LAST_IP_ADDRESS_REMOVAL_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcc4ba62a_162e_4648_847a_b6bdf993e335);
pub type PFN_SC_NOTIFY_CALLBACK = Option<unsafe extern "system" fn(pparameter: *const core::ffi::c_void)>;
pub type PSC_EVENT_TYPE = *mut SC_EVENT_TYPE;
pub type PSC_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(dwnotify: u32, pcallbackcontext: *const core::ffi::c_void)>;
pub type PSC_NOTIFICATION_REGISTRATION = *mut _SC_NOTIFICATION_REGISTRATION;
pub type PSERVICE_CONTROL_STATUS_REASON_PARAMS = PSERVICE_CONTROL_STATUS_REASON_PARAMSA;
pub type PSERVICE_CONTROL_STATUS_REASON_PARAMSA = *mut SERVICE_CONTROL_STATUS_REASON_PARAMSA;
pub type PSERVICE_CONTROL_STATUS_REASON_PARAMSW = *mut SERVICE_CONTROL_STATUS_REASON_PARAMSW;
pub type PSERVICE_LAUNCH_PROTECTED_INFO = *mut SERVICE_LAUNCH_PROTECTED_INFO;
pub type PSERVICE_NOTIFY = PSERVICE_NOTIFYA;
pub type PSERVICE_NOTIFYA = *mut SERVICE_NOTIFY_2A;
pub type PSERVICE_NOTIFYW = *mut SERVICE_NOTIFY_2W;
pub type PSERVICE_NOTIFY_1 = *mut SERVICE_NOTIFY_1;
pub type PSERVICE_NOTIFY_2 = PSERVICE_NOTIFY_2A;
pub type PSERVICE_NOTIFY_2A = *mut SERVICE_NOTIFY_2A;
pub type PSERVICE_NOTIFY_2W = *mut SERVICE_NOTIFY_2W;
pub type PSERVICE_START_REASON = *mut SERVICE_START_REASON;
pub type PSERVICE_TIMECHANGE_INFO = *mut SERVICE_TIMECHANGE_INFO;
#[cfg(feature = "minwindef")]
pub type PSERVICE_TRIGGER = *mut SERVICE_TRIGGER;
#[cfg(feature = "minwindef")]
pub type PSERVICE_TRIGGER_INFO = *mut SERVICE_TRIGGER_INFO;
#[cfg(feature = "minwindef")]
pub type PSERVICE_TRIGGER_SPECIFIC_DATA_ITEM = *mut SERVICE_TRIGGER_SPECIFIC_DATA_ITEM;
pub type QUERY_SERVICE_CONFIG = QUERY_SERVICE_CONFIGA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUERY_SERVICE_CONFIGA {
    pub dwServiceType: u32,
    pub dwStartType: u32,
    pub dwErrorControl: u32,
    pub lpBinaryPathName: windows_core::PSTR,
    pub lpLoadOrderGroup: windows_core::PSTR,
    pub dwTagId: u32,
    pub lpDependencies: windows_core::PSTR,
    pub lpServiceStartName: windows_core::PSTR,
    pub lpDisplayName: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUERY_SERVICE_CONFIGW {
    pub dwServiceType: u32,
    pub dwStartType: u32,
    pub dwErrorControl: u32,
    pub lpBinaryPathName: windows_core::PWSTR,
    pub lpLoadOrderGroup: windows_core::PWSTR,
    pub dwTagId: u32,
    pub lpDependencies: windows_core::PWSTR,
    pub lpServiceStartName: windows_core::PWSTR,
    pub lpDisplayName: windows_core::PWSTR,
}
pub type QUERY_SERVICE_LOCK_STATUS = QUERY_SERVICE_LOCK_STATUSA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUERY_SERVICE_LOCK_STATUSA {
    pub fIsLocked: u32,
    pub lpLockOwner: windows_core::PSTR,
    pub dwLockDuration: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUERY_SERVICE_LOCK_STATUSW {
    pub fIsLocked: u32,
    pub lpLockOwner: windows_core::PWSTR,
    pub dwLockDuration: u32,
}
pub const RPC_INTERFACE_EVENT_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbc90d167_9470_4139_a9ba_be0bbbf5b74d);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SC_ACTION {
    pub Type: SC_ACTION_TYPE,
    pub Delay: u32,
}
pub const SC_ACTION_NONE: SC_ACTION_TYPE = 0;
pub const SC_ACTION_OWN_RESTART: SC_ACTION_TYPE = 4;
pub const SC_ACTION_REBOOT: SC_ACTION_TYPE = 2;
pub const SC_ACTION_RESTART: SC_ACTION_TYPE = 1;
pub const SC_ACTION_RUN_COMMAND: SC_ACTION_TYPE = 3;
pub type SC_ACTION_TYPE = i32;
pub const SC_AGGREGATE_STORAGE_KEY: windows_core::PCWSTR = windows_core::w!("System\\CurrentControlSet\\Control\\ServiceAggregatedEvents");
pub const SC_ENUM_PROCESS_INFO: SC_ENUM_TYPE = 0;
pub type SC_ENUM_TYPE = i32;
pub const SC_EVENT_DATABASE_CHANGE: SC_EVENT_TYPE = 0;
pub const SC_EVENT_PROPERTY_CHANGE: SC_EVENT_TYPE = 1;
pub const SC_EVENT_STATUS_CHANGE: SC_EVENT_TYPE = 2;
pub type SC_EVENT_TYPE = i32;
pub const SC_GROUP_IDENTIFIER: u32 = 43;
pub const SC_GROUP_IDENTIFIERA: u32 = 43;
pub const SC_GROUP_IDENTIFIERW: u32 = 43;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SC_HANDLE(pub *mut core::ffi::c_void);
impl Default for SC_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SC_LOCK(pub *mut core::ffi::c_void);
impl Default for SC_LOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SC_MANAGER_ALL_ACCESS: u32 = 983103;
pub const SC_MANAGER_CONNECT: u32 = 1;
pub const SC_MANAGER_CREATE_SERVICE: u32 = 2;
pub const SC_MANAGER_ENUMERATE_SERVICE: u32 = 4;
pub const SC_MANAGER_LOCK: u32 = 8;
pub const SC_MANAGER_MODIFY_BOOT_CONFIG: u32 = 32;
pub const SC_MANAGER_QUERY_LOCK_STATUS: u32 = 16;
pub type SC_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(dwnotify: u32, pcallbackcontext: *const core::ffi::c_void)>;
pub const SC_STATUS_PROCESS_INFO: SC_STATUS_TYPE = 0;
pub type SC_STATUS_TYPE = i32;
pub const SERVICES_ACTIVE_DATABASEA: windows_core::PCSTR = windows_core::s!("ServicesActive");
pub const SERVICES_ACTIVE_DATABASEW: windows_core::PCWSTR = windows_core::w!("ServicesActive");
pub const SERVICES_FAILED_DATABASEA: windows_core::PCSTR = windows_core::s!("ServicesFailed");
pub const SERVICES_FAILED_DATABASEW: windows_core::PCWSTR = windows_core::w!("ServicesFailed");
pub const SERVICE_ACCEPT_HARDWAREPROFILECHANGE: u32 = 32;
pub const SERVICE_ACCEPT_LOWRESOURCES: u32 = 8192;
pub const SERVICE_ACCEPT_NETBINDCHANGE: u32 = 16;
pub const SERVICE_ACCEPT_PARAMCHANGE: u32 = 8;
pub const SERVICE_ACCEPT_PAUSE_CONTINUE: u32 = 2;
pub const SERVICE_ACCEPT_POWEREVENT: u32 = 64;
pub const SERVICE_ACCEPT_PRESHUTDOWN: u32 = 256;
pub const SERVICE_ACCEPT_SESSIONCHANGE: u32 = 128;
pub const SERVICE_ACCEPT_SHUTDOWN: u32 = 4;
pub const SERVICE_ACCEPT_STOP: u32 = 1;
pub const SERVICE_ACCEPT_SYSTEMLOWRESOURCES: u32 = 16384;
pub const SERVICE_ACCEPT_TIMECHANGE: u32 = 512;
pub const SERVICE_ACCEPT_TRIGGEREVENT: u32 = 1024;
pub const SERVICE_ACCEPT_USER_LOGOFF: u32 = 2048;
pub const SERVICE_ACTIVE: u32 = 1;
pub const SERVICE_ALL_ACCESS: u32 = 983551;
pub const SERVICE_CHANGE_CONFIG: u32 = 2;
pub const SERVICE_CONFIG_DELAYED_AUTO_START_INFO: u32 = 3;
pub const SERVICE_CONFIG_DESCRIPTION: u32 = 1;
pub const SERVICE_CONFIG_FAILURE_ACTIONS: u32 = 2;
pub const SERVICE_CONFIG_FAILURE_ACTIONS_FLAG: u32 = 4;
pub const SERVICE_CONFIG_LAUNCH_PROTECTED: u32 = 12;
pub const SERVICE_CONFIG_PREFERRED_NODE: u32 = 9;
pub const SERVICE_CONFIG_PRESHUTDOWN_INFO: u32 = 7;
pub const SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO: u32 = 6;
pub const SERVICE_CONFIG_SERVICE_SID_INFO: u32 = 5;
pub const SERVICE_CONFIG_TRIGGER_INFO: u32 = 8;
pub const SERVICE_CONTINUE_PENDING: u32 = 5;
pub const SERVICE_CONTROL_CONTINUE: u32 = 3;
pub const SERVICE_CONTROL_DEVICEEVENT: u32 = 11;
pub const SERVICE_CONTROL_HARDWAREPROFILECHANGE: u32 = 12;
pub const SERVICE_CONTROL_INTERROGATE: u32 = 4;
pub const SERVICE_CONTROL_LOWRESOURCES: u32 = 96;
pub const SERVICE_CONTROL_NETBINDADD: u32 = 7;
pub const SERVICE_CONTROL_NETBINDDISABLE: u32 = 10;
pub const SERVICE_CONTROL_NETBINDENABLE: u32 = 9;
pub const SERVICE_CONTROL_NETBINDREMOVE: u32 = 8;
pub const SERVICE_CONTROL_PARAMCHANGE: u32 = 6;
pub const SERVICE_CONTROL_PAUSE: u32 = 2;
pub const SERVICE_CONTROL_POWEREVENT: u32 = 13;
pub const SERVICE_CONTROL_PRESHUTDOWN: u32 = 15;
pub const SERVICE_CONTROL_SESSIONCHANGE: u32 = 14;
pub const SERVICE_CONTROL_SHUTDOWN: u32 = 5;
pub const SERVICE_CONTROL_STATUS_REASON_INFO: u32 = 1;
pub type SERVICE_CONTROL_STATUS_REASON_PARAMS = SERVICE_CONTROL_STATUS_REASON_PARAMSA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    pub dwReason: u32,
    pub pszComment: windows_core::PSTR,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    pub dwReason: u32,
    pub pszComment: windows_core::PWSTR,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
pub const SERVICE_CONTROL_STOP: u32 = 1;
pub const SERVICE_CONTROL_SYSTEMLOWRESOURCES: u32 = 97;
pub const SERVICE_CONTROL_TIMECHANGE: u32 = 16;
pub const SERVICE_CONTROL_TRIGGEREVENT: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    pub u: SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0,
}
impl Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    pub CustomStateId: SERVICE_TRIGGER_CUSTOM_STATE_ID,
    pub s: SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0,
}
impl Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    pub DataOffset: u32,
    pub Data: [u8; 1],
}
impl Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_DELAYED_AUTO_START_INFO {
    pub fDelayedAutostart: windows_core::BOOL,
}
pub type SERVICE_DESCRIPTION = SERVICE_DESCRIPTIONA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_DESCRIPTIONA {
    pub lpDescription: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_DESCRIPTIONW {
    pub lpDescription: windows_core::PWSTR,
}
pub type SERVICE_DIRECTORY_TYPE = i32;
pub const SERVICE_DYNAMIC_INFORMATION_LEVEL_START_REASON: u32 = 1;
pub const SERVICE_ENUMERATE_DEPENDENTS: u32 = 8;
pub type SERVICE_FAILURE_ACTIONS = SERVICE_FAILURE_ACTIONSA;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SERVICE_FAILURE_ACTIONSA {
    pub dwResetPeriod: u32,
    pub lpRebootMsg: windows_core::PSTR,
    pub lpCommand: windows_core::PSTR,
    pub cActions: u32,
    pub lpsaActions: *mut SC_ACTION,
}
impl Default for SERVICE_FAILURE_ACTIONSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SERVICE_FAILURE_ACTIONSW {
    pub dwResetPeriod: u32,
    pub lpRebootMsg: windows_core::PWSTR,
    pub lpCommand: windows_core::PWSTR,
    pub cActions: u32,
    pub lpsaActions: *mut SC_ACTION,
}
impl Default for SERVICE_FAILURE_ACTIONSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_FAILURE_ACTIONS_FLAG {
    pub fFailureActionsOnNonCrashFailures: windows_core::BOOL,
}
pub const SERVICE_INACTIVE: u32 = 2;
pub const SERVICE_INTERROGATE: u32 = 128;
pub const SERVICE_LAUNCH_PROTECTED_ANTIMALWARE_LIGHT: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_LAUNCH_PROTECTED_INFO {
    pub dwLaunchProtected: u32,
}
pub const SERVICE_LAUNCH_PROTECTED_NONE: u32 = 0;
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS: u32 = 1;
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS_LIGHT: u32 = 2;
#[cfg(feature = "winnt")]
pub type SERVICE_MAIN_FUNCTIONA = Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut super::winnt::LPTSTR)>;
pub type SERVICE_MAIN_FUNCTIONW = Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut windows_core::PWSTR)>;
pub type SERVICE_NOTIFY = SERVICE_NOTIFYA;
pub type SERVICE_NOTIFYA = SERVICE_NOTIFY_2A;
pub type SERVICE_NOTIFYW = SERVICE_NOTIFY_2W;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SERVICE_NOTIFY_1 {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
impl Default for SERVICE_NOTIFY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SERVICE_NOTIFY_2 = SERVICE_NOTIFY_2A;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SERVICE_NOTIFY_2A {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: u32,
    pub pszServiceNames: windows_core::PSTR,
}
impl Default for SERVICE_NOTIFY_2A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SERVICE_NOTIFY_2W {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: u32,
    pub pszServiceNames: windows_core::PWSTR,
}
impl Default for SERVICE_NOTIFY_2W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERVICE_NOTIFY_CONTINUE_PENDING: u32 = 16;
pub const SERVICE_NOTIFY_CREATED: u32 = 128;
pub const SERVICE_NOTIFY_DELETED: u32 = 256;
pub const SERVICE_NOTIFY_DELETE_PENDING: u32 = 512;
pub const SERVICE_NOTIFY_PAUSED: u32 = 64;
pub const SERVICE_NOTIFY_PAUSE_PENDING: u32 = 32;
pub const SERVICE_NOTIFY_RUNNING: u32 = 8;
pub const SERVICE_NOTIFY_START_PENDING: u32 = 2;
pub const SERVICE_NOTIFY_STATUS_CHANGE: u32 = 2;
pub const SERVICE_NOTIFY_STATUS_CHANGE_1: u32 = 1;
pub const SERVICE_NOTIFY_STATUS_CHANGE_2: u32 = 2;
pub const SERVICE_NOTIFY_STOPPED: u32 = 1;
pub const SERVICE_NOTIFY_STOP_PENDING: u32 = 4;
pub const SERVICE_NO_CHANGE: u32 = 4294967295;
pub const SERVICE_PAUSED: u32 = 7;
pub const SERVICE_PAUSE_CONTINUE: u32 = 64;
pub const SERVICE_PAUSE_PENDING: u32 = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_PREFERRED_NODE_INFO {
    pub usPreferredNode: u16,
    pub fDelete: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_PRESHUTDOWN_INFO {
    pub dwPreshutdownTimeout: u32,
}
pub const SERVICE_QUERY_CONFIG: u32 = 1;
pub const SERVICE_QUERY_STATUS: u32 = 4;
pub type SERVICE_REGISTRY_STATE_TYPE = i32;
pub type SERVICE_REQUIRED_PRIVILEGES_INFO = SERVICE_REQUIRED_PRIVILEGES_INFOA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOA {
    pub pmszRequiredPrivileges: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOW {
    pub pmszRequiredPrivileges: windows_core::PWSTR,
}
pub const SERVICE_RUNNING: u32 = 4;
pub const SERVICE_RUNS_IN_SYSTEM_PROCESS: u32 = 1;
pub type SERVICE_SHARED_DIRECTORY_TYPE = i32;
pub type SERVICE_SHARED_REGISTRY_STATE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_SID_INFO {
    pub dwServiceSidType: u32,
}
pub const SERVICE_SID_TYPE_NONE: u32 = 0;
pub const SERVICE_SID_TYPE_RESTRICTED: u32 = 3;
pub const SERVICE_SID_TYPE_UNRESTRICTED: u32 = 1;
pub const SERVICE_START: u32 = 16;
pub const SERVICE_START_PENDING: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_START_REASON {
    pub dwReason: u32,
}
pub const SERVICE_START_REASON_AUTO: u32 = 2;
pub const SERVICE_START_REASON_DELAYEDAUTO: u32 = 16;
pub const SERVICE_START_REASON_DEMAND: u32 = 1;
pub const SERVICE_START_REASON_RESTART_ON_FAILURE: u32 = 8;
pub const SERVICE_START_REASON_TRIGGER: u32 = 4;
pub const SERVICE_STATE_ALL: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_STATUS {
    pub dwServiceType: u32,
    pub dwCurrentState: u32,
    pub dwControlsAccepted: u32,
    pub dwWin32ExitCode: u32,
    pub dwServiceSpecificExitCode: u32,
    pub dwCheckPoint: u32,
    pub dwWaitHint: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SERVICE_STATUS_HANDLE(pub *mut core::ffi::c_void);
impl Default for SERVICE_STATUS_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_STATUS_PROCESS {
    pub dwServiceType: u32,
    pub dwCurrentState: u32,
    pub dwControlsAccepted: u32,
    pub dwWin32ExitCode: u32,
    pub dwServiceSpecificExitCode: u32,
    pub dwCheckPoint: u32,
    pub dwWaitHint: u32,
    pub dwProcessId: u32,
    pub dwServiceFlags: u32,
}
pub const SERVICE_STOP: u32 = 32;
pub const SERVICE_STOPPED: u32 = 1;
pub const SERVICE_STOP_PENDING: u32 = 3;
pub const SERVICE_STOP_REASON_FLAG_CUSTOM: u32 = 536870912;
pub const SERVICE_STOP_REASON_FLAG_MAX: u32 = 2147483648;
pub const SERVICE_STOP_REASON_FLAG_MIN: u32 = 0;
pub const SERVICE_STOP_REASON_FLAG_PLANNED: u32 = 1073741824;
pub const SERVICE_STOP_REASON_FLAG_UNPLANNED: u32 = 268435456;
pub const SERVICE_STOP_REASON_MAJOR_APPLICATION: u32 = 327680;
pub const SERVICE_STOP_REASON_MAJOR_HARDWARE: u32 = 131072;
pub const SERVICE_STOP_REASON_MAJOR_MAX: u32 = 458752;
pub const SERVICE_STOP_REASON_MAJOR_MAX_CUSTOM: u32 = 16711680;
pub const SERVICE_STOP_REASON_MAJOR_MIN: u32 = 0;
pub const SERVICE_STOP_REASON_MAJOR_MIN_CUSTOM: u32 = 4194304;
pub const SERVICE_STOP_REASON_MAJOR_NONE: u32 = 393216;
pub const SERVICE_STOP_REASON_MAJOR_OPERATINGSYSTEM: u32 = 196608;
pub const SERVICE_STOP_REASON_MAJOR_OTHER: u32 = 65536;
pub const SERVICE_STOP_REASON_MAJOR_SOFTWARE: u32 = 262144;
pub const SERVICE_STOP_REASON_MINOR_DISK: u32 = 8;
pub const SERVICE_STOP_REASON_MINOR_ENVIRONMENT: u32 = 10;
pub const SERVICE_STOP_REASON_MINOR_HARDWARE_DRIVER: u32 = 11;
pub const SERVICE_STOP_REASON_MINOR_HUNG: u32 = 6;
pub const SERVICE_STOP_REASON_MINOR_INSTALLATION: u32 = 3;
pub const SERVICE_STOP_REASON_MINOR_MAINTENANCE: u32 = 2;
pub const SERVICE_STOP_REASON_MINOR_MAX: u32 = 25;
pub const SERVICE_STOP_REASON_MINOR_MAX_CUSTOM: u32 = 65535;
pub const SERVICE_STOP_REASON_MINOR_MEMOTYLIMIT: u32 = 24;
pub const SERVICE_STOP_REASON_MINOR_MIN: u32 = 0;
pub const SERVICE_STOP_REASON_MINOR_MIN_CUSTOM: u32 = 256;
pub const SERVICE_STOP_REASON_MINOR_MMC: u32 = 22;
pub const SERVICE_STOP_REASON_MINOR_NETWORKCARD: u32 = 9;
pub const SERVICE_STOP_REASON_MINOR_NETWORK_CONNECTIVITY: u32 = 17;
pub const SERVICE_STOP_REASON_MINOR_NONE: u32 = 23;
pub const SERVICE_STOP_REASON_MINOR_OTHER: u32 = 1;
pub const SERVICE_STOP_REASON_MINOR_OTHERDRIVER: u32 = 12;
pub const SERVICE_STOP_REASON_MINOR_RECONFIG: u32 = 5;
pub const SERVICE_STOP_REASON_MINOR_SECURITY: u32 = 16;
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX: u32 = 15;
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX_UNINSTALL: u32 = 21;
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK: u32 = 13;
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK_UNINSTALL: u32 = 19;
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE: u32 = 14;
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE_UNINSTALL: u32 = 20;
pub const SERVICE_STOP_REASON_MINOR_UNSTABLE: u32 = 7;
pub const SERVICE_STOP_REASON_MINOR_UPGRADE: u32 = 4;
pub const SERVICE_STOP_REASON_MINOR_WMI: u32 = 18;
pub type SERVICE_TABLE_ENTRY = SERVICE_TABLE_ENTRYA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SERVICE_TABLE_ENTRYA {
    pub lpServiceName: windows_core::PSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SERVICE_TABLE_ENTRYW {
    pub lpServiceName: windows_core::PWSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONW,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_TIMECHANGE_INFO {
    pub liNewTime: i64,
    pub liOldTime: i64,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SERVICE_TRIGGER {
    pub dwTriggerType: u32,
    pub dwAction: u32,
    pub pTriggerSubtype: *mut windows_core::GUID,
    pub cDataItems: u32,
    pub pDataItems: PSERVICE_TRIGGER_SPECIFIC_DATA_ITEM,
}
#[cfg(feature = "minwindef")]
impl Default for SERVICE_TRIGGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERVICE_TRIGGER_ACTION_SERVICE_START: u32 = 1;
pub const SERVICE_TRIGGER_ACTION_SERVICE_STOP: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SERVICE_TRIGGER_CUSTOM_STATE_ID {
    pub Data: [u32; 2],
}
impl Default for SERVICE_TRIGGER_CUSTOM_STATE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERVICE_TRIGGER_DATA_TYPE_BINARY: u32 = 1;
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ALL: u32 = 5;
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ANY: u32 = 4;
pub const SERVICE_TRIGGER_DATA_TYPE_LEVEL: u32 = 3;
pub const SERVICE_TRIGGER_DATA_TYPE_STRING: u32 = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_TRIGGER_INFO {
    pub cTriggers: u32,
    pub pTriggers: PSERVICE_TRIGGER,
    pub pReserved: super::minwindef::PBYTE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    pub dwDataType: u32,
    pub cbData: u32,
    pub pData: super::minwindef::PBYTE,
}
pub const SERVICE_TRIGGER_STARTED_ARGUMENT: windows_core::PCWSTR = windows_core::w!("TriggerStarted");
pub const SERVICE_TRIGGER_TYPE_AGGREGATE: u32 = 30;
pub const SERVICE_TRIGGER_TYPE_CUSTOM: u32 = 20;
pub const SERVICE_TRIGGER_TYPE_CUSTOM_SYSTEM_STATE_CHANGE: u32 = 7;
pub const SERVICE_TRIGGER_TYPE_DEVICE_INTERFACE_ARRIVAL: u32 = 1;
pub const SERVICE_TRIGGER_TYPE_DOMAIN_JOIN: u32 = 3;
pub const SERVICE_TRIGGER_TYPE_FIREWALL_PORT_EVENT: u32 = 4;
pub const SERVICE_TRIGGER_TYPE_GROUP_POLICY: u32 = 5;
pub const SERVICE_TRIGGER_TYPE_IP_ADDRESS_AVAILABILITY: u32 = 2;
pub const SERVICE_TRIGGER_TYPE_NETWORK_ENDPOINT: u32 = 6;
pub const SERVICE_USER_DEFINED_CONTROL: u32 = 256;
pub const ServiceDirectoryPersistentState: SERVICE_DIRECTORY_TYPE = 0;
pub const ServiceDirectoryTypeMax: SERVICE_DIRECTORY_TYPE = 1;
pub const ServiceRegistryStateParameters: SERVICE_REGISTRY_STATE_TYPE = 0;
pub const ServiceRegistryStatePersistent: SERVICE_REGISTRY_STATE_TYPE = 1;
pub const ServiceSharedDirectoryPersistentState: SERVICE_SHARED_DIRECTORY_TYPE = 0;
pub const ServiceSharedRegistryPersistentState: SERVICE_SHARED_REGISTRY_STATE_TYPE = 0;
pub const USER_POLICY_PRESENT_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x54fb46c8_f089_464c_b1fd_59d1b62c3b50);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _SC_NOTIFICATION_REGISTRATION(pub u8);
