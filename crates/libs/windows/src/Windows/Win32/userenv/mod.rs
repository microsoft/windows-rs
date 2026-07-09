#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateAppContainerProfile<P0, P1, P2>(pszappcontainername: P0, pszdisplayname: P1, pszdescription: P2, pcapabilities: Option<&[super::winnt::SID_AND_ATTRIBUTES]>) -> windows_core::Result<super::winnt::PSID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn CreateAppContainerProfile(pszappcontainername : windows_core::PCWSTR, pszdisplayname : windows_core::PCWSTR, pszdescription : windows_core::PCWSTR, pcapabilities : *const super::winnt::SID_AND_ATTRIBUTES, dwcapabilitycount : u32, ppsidappcontainersid : *mut super::winnt::PSID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateAppContainerProfile(pszappcontainername.param().abi(), pszdisplayname.param().abi(), pszdescription.param().abi(), core::mem::transmute(pcapabilities.map_or(core::ptr::null(), |slice| slice.as_ptr())), pcapabilities.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateEnvironmentBlock(lpenvironment: *mut *mut core::ffi::c_void, htoken: Option<super::winnt::HANDLE>, binherit: bool) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn CreateEnvironmentBlock(lpenvironment : *mut *mut core::ffi::c_void, htoken : super::winnt::HANDLE, binherit : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CreateEnvironmentBlock(lpenvironment as _, htoken.unwrap_or(core::mem::zeroed()) as _, binherit.into()) }
}
#[inline]
pub unsafe fn CreateProfile<P0, P1>(pszusersid: P0, pszusername: P1, pszprofilepath: &mut [u16]) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn CreateProfile(pszusersid : windows_core::PCWSTR, pszusername : windows_core::PCWSTR, pszprofilepath : windows_core::PWSTR, cchprofilepath : u32) -> windows_core::HRESULT);
    unsafe { CreateProfile(pszusersid.param().abi(), pszusername.param().abi(), core::mem::transmute(pszprofilepath.as_ptr()), pszprofilepath.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn DeleteAppContainerProfile<P0>(pszappcontainername: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn DeleteAppContainerProfile(pszappcontainername : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DeleteAppContainerProfile(pszappcontainername.param().abi()) }
}
#[inline]
pub unsafe fn DeleteProfileA<P0, P1, P2>(lpsidstring: P0, lpprofilepath: P1, lpcomputername: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("userenv.dll" "system" fn DeleteProfileA(lpsidstring : windows_core::PCSTR, lpprofilepath : windows_core::PCSTR, lpcomputername : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DeleteProfileA(lpsidstring.param().abi(), lpprofilepath.param().abi(), lpcomputername.param().abi()) }
}
#[inline]
pub unsafe fn DeleteProfileW<P0, P1, P2>(lpsidstring: P0, lpprofilepath: P1, lpcomputername: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn DeleteProfileW(lpsidstring : windows_core::PCWSTR, lpprofilepath : windows_core::PCWSTR, lpcomputername : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { DeleteProfileW(lpsidstring.param().abi(), lpprofilepath.param().abi(), lpcomputername.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeriveAppContainerSidFromAppContainerName<P0>(pszappcontainername: P0) -> windows_core::Result<super::winnt::PSID>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn DeriveAppContainerSidFromAppContainerName(pszappcontainername : windows_core::PCWSTR, ppsidappcontainersid : *mut super::winnt::PSID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DeriveAppContainerSidFromAppContainerName(pszappcontainername.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName<P1>(psidappcontainersid: super::winnt::PSID, pszrestrictedappcontainername: P1) -> windows_core::Result<super::winnt::PSID>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid : super::winnt::PSID, pszrestrictedappcontainername : windows_core::PCWSTR, ppsidrestrictedappcontainersid : *mut super::winnt::PSID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid, pszrestrictedappcontainername.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DestroyEnvironmentBlock(lpenvironment: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn DestroyEnvironmentBlock(lpenvironment : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { DestroyEnvironmentBlock(lpenvironment) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EnterCriticalPolicySection(bmachine: bool) -> super::winnt::HANDLE {
    windows_core::link!("userenv.dll" "system" fn EnterCriticalPolicySection(bmachine : windows_core::BOOL) -> super::winnt::HANDLE);
    unsafe { EnterCriticalPolicySection(bmachine.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ExpandEnvironmentStringsForUserA<P1>(htoken: Option<super::winnt::HANDLE>, lpsrc: P1, lpdest: &mut [u8]) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("userenv.dll" "system" fn ExpandEnvironmentStringsForUserA(htoken : super::winnt::HANDLE, lpsrc : windows_core::PCSTR, lpdest : windows_core::PSTR, dwsize : u32) -> windows_core::BOOL);
    unsafe { ExpandEnvironmentStringsForUserA(htoken.unwrap_or(core::mem::zeroed()) as _, lpsrc.param().abi(), core::mem::transmute(lpdest.as_ptr()), lpdest.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ExpandEnvironmentStringsForUserW<P1>(htoken: Option<super::winnt::HANDLE>, lpsrc: P1, lpdest: &mut [u16]) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn ExpandEnvironmentStringsForUserW(htoken : super::winnt::HANDLE, lpsrc : windows_core::PCWSTR, lpdest : windows_core::PWSTR, dwsize : u32) -> windows_core::BOOL);
    unsafe { ExpandEnvironmentStringsForUserW(htoken.unwrap_or(core::mem::zeroed()) as _, lpsrc.param().abi(), core::mem::transmute(lpdest.as_ptr()), lpdest.len().try_into().unwrap()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn FreeGPOListA(pgpolist: *const GROUP_POLICY_OBJECTA) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn FreeGPOListA(pgpolist : *const GROUP_POLICY_OBJECTA) -> windows_core::BOOL);
    unsafe { FreeGPOListA(pgpolist) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn FreeGPOListW(pgpolist: *const GROUP_POLICY_OBJECTW) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn FreeGPOListW(pgpolist : *const GROUP_POLICY_OBJECTW) -> windows_core::BOOL);
    unsafe { FreeGPOListW(pgpolist) }
}
#[inline]
pub unsafe fn GenerateGPNotification<P1>(bmachine: bool, lpwszmgmtproduct: P1, dwmgmtproductoptions: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GenerateGPNotification(bmachine : windows_core::BOOL, lpwszmgmtproduct : windows_core::PCWSTR, dwmgmtproductoptions : u32) -> u32);
    unsafe { GenerateGPNotification(bmachine.into(), lpwszmgmtproduct.param().abi(), dwmgmtproductoptions) }
}
#[inline]
pub unsafe fn GetAllUsersProfileDirectoryA(lpprofiledir: Option<windows_core::PSTR>, lpcchsize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetAllUsersProfileDirectoryA(lpprofiledir : windows_core::PSTR, lpcchsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetAllUsersProfileDirectoryA(lpprofiledir.unwrap_or(core::mem::zeroed()) as _, lpcchsize as _) }
}
#[inline]
pub unsafe fn GetAllUsersProfileDirectoryW(lpprofiledir: Option<windows_core::PWSTR>, lpcchsize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetAllUsersProfileDirectoryW(lpprofiledir : windows_core::PWSTR, lpcchsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetAllUsersProfileDirectoryW(lpprofiledir.unwrap_or(core::mem::zeroed()) as _, lpcchsize as _) }
}
#[inline]
pub unsafe fn GetAppContainerFolderPath<P0>(pszappcontainersid: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GetAppContainerFolderPath(pszappcontainersid : windows_core::PCWSTR, ppszpath : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetAppContainerFolderPath(pszappcontainersid.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt", feature = "winreg"))]
#[inline]
pub unsafe fn GetAppContainerRegistryLocation(desiredaccess: super::winreg::REGSAM) -> windows_core::Result<super::minwindef::HKEY> {
    windows_core::link!("userenv.dll" "system" fn GetAppContainerRegistryLocation(desiredaccess : super::winreg::REGSAM, phappcontainerkey : *mut super::minwindef::HKEY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetAppContainerRegistryLocation(desiredaccess, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetAppliedGPOListA<P1>(dwflags: u32, pmachinename: P1, psiduser: Option<super::winnt::PSID>, pguidextension: *const windows_core::GUID, ppgpolist: *mut PGROUP_POLICY_OBJECTA) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GetAppliedGPOListA(dwflags : u32, pmachinename : windows_core::PCSTR, psiduser : super::winnt::PSID, pguidextension : *const windows_core::GUID, ppgpolist : *mut PGROUP_POLICY_OBJECTA) -> u32);
    unsafe { GetAppliedGPOListA(dwflags, pmachinename.param().abi(), psiduser.unwrap_or(core::mem::zeroed()) as _, pguidextension, ppgpolist as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetAppliedGPOListW<P1>(dwflags: u32, pmachinename: P1, psiduser: Option<super::winnt::PSID>, pguidextension: *const windows_core::GUID, ppgpolist: *mut PGROUP_POLICY_OBJECTW) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GetAppliedGPOListW(dwflags : u32, pmachinename : windows_core::PCWSTR, psiduser : super::winnt::PSID, pguidextension : *const windows_core::GUID, ppgpolist : *mut PGROUP_POLICY_OBJECTW) -> u32);
    unsafe { GetAppliedGPOListW(dwflags, pmachinename.param().abi(), psiduser.unwrap_or(core::mem::zeroed()) as _, pguidextension, ppgpolist as _) }
}
#[inline]
pub unsafe fn GetDefaultUserProfileDirectoryA(lpprofiledir: Option<windows_core::PSTR>, lpcchsize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetDefaultUserProfileDirectoryA(lpprofiledir : windows_core::PSTR, lpcchsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetDefaultUserProfileDirectoryA(lpprofiledir.unwrap_or(core::mem::zeroed()) as _, lpcchsize as _) }
}
#[inline]
pub unsafe fn GetDefaultUserProfileDirectoryW(lpprofiledir: Option<windows_core::PWSTR>, lpcchsize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetDefaultUserProfileDirectoryW(lpprofiledir : windows_core::PWSTR, lpcchsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetDefaultUserProfileDirectoryW(lpprofiledir.unwrap_or(core::mem::zeroed()) as _, lpcchsize as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetGPOListA<P1, P2, P3>(htoken: Option<super::winnt::HANDLE>, lpname: P1, lphostname: P2, lpcomputername: P3, dwflags: u32, pgpolist: *mut PGROUP_POLICY_OBJECTA) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GetGPOListA(htoken : super::winnt::HANDLE, lpname : windows_core::PCSTR, lphostname : windows_core::PCSTR, lpcomputername : windows_core::PCSTR, dwflags : u32, pgpolist : *mut PGROUP_POLICY_OBJECTA) -> windows_core::BOOL);
    unsafe { GetGPOListA(htoken.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi(), lphostname.param().abi(), lpcomputername.param().abi(), dwflags, pgpolist as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetGPOListW<P1, P2, P3>(htoken: Option<super::winnt::HANDLE>, lpname: P1, lphostname: P2, lpcomputername: P3, dwflags: u32, pgpolist: *mut PGROUP_POLICY_OBJECTW) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GetGPOListW(htoken : super::winnt::HANDLE, lpname : windows_core::PCWSTR, lphostname : windows_core::PCWSTR, lpcomputername : windows_core::PCWSTR, dwflags : u32, pgpolist : *mut PGROUP_POLICY_OBJECTW) -> windows_core::BOOL);
    unsafe { GetGPOListW(htoken.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi(), lphostname.param().abi(), lpcomputername.param().abi(), dwflags, pgpolist as _) }
}
#[inline]
pub unsafe fn GetProfileType(dwflags: *mut u32) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetProfileType(dwflags : *mut u32) -> windows_core::BOOL);
    unsafe { GetProfileType(dwflags as _) }
}
#[inline]
pub unsafe fn GetProfilesDirectoryA(lpprofiledir: Option<windows_core::PSTR>, lpcchsize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetProfilesDirectoryA(lpprofiledir : windows_core::PSTR, lpcchsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetProfilesDirectoryA(lpprofiledir.unwrap_or(core::mem::zeroed()) as _, lpcchsize as _) }
}
#[inline]
pub unsafe fn GetProfilesDirectoryW(lpprofiledir: Option<windows_core::PWSTR>, lpcchsize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetProfilesDirectoryW(lpprofiledir : windows_core::PWSTR, lpcchsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetProfilesDirectoryW(lpprofiledir.unwrap_or(core::mem::zeroed()) as _, lpcchsize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetUserProfileDirectoryA(htoken: super::winnt::HANDLE, lpprofiledir: Option<windows_core::PSTR>, lpcchsize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetUserProfileDirectoryA(htoken : super::winnt::HANDLE, lpprofiledir : windows_core::PSTR, lpcchsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetUserProfileDirectoryA(htoken, lpprofiledir.unwrap_or(core::mem::zeroed()) as _, lpcchsize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetUserProfileDirectoryW(htoken: super::winnt::HANDLE, lpprofiledir: Option<windows_core::PWSTR>, lpcchsize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetUserProfileDirectoryW(htoken : super::winnt::HANDLE, lpprofiledir : windows_core::PWSTR, lpcchsize : *mut u32) -> windows_core::BOOL);
    unsafe { GetUserProfileDirectoryW(htoken, lpprofiledir.unwrap_or(core::mem::zeroed()) as _, lpcchsize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn LeaveCriticalPolicySection(hsection: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn LeaveCriticalPolicySection(hsection : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { LeaveCriticalPolicySection(hsection) }
}
#[cfg(all(feature = "profinfo", feature = "winnt"))]
#[inline]
pub unsafe fn LoadUserProfileA(htoken: super::winnt::HANDLE, lpprofileinfo: *mut super::profinfo::PROFILEINFOA) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn LoadUserProfileA(htoken : super::winnt::HANDLE, lpprofileinfo : *mut super::profinfo::PROFILEINFOA) -> windows_core::BOOL);
    unsafe { LoadUserProfileA(htoken, lpprofileinfo as _) }
}
#[cfg(all(feature = "profinfo", feature = "winnt"))]
#[inline]
pub unsafe fn LoadUserProfileW(htoken: super::winnt::HANDLE, lpprofileinfo: *mut super::profinfo::PROFILEINFOW) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn LoadUserProfileW(htoken : super::winnt::HANDLE, lpprofileinfo : *mut super::profinfo::PROFILEINFOW) -> windows_core::BOOL);
    unsafe { LoadUserProfileW(htoken, lpprofileinfo as _) }
}
#[inline]
pub unsafe fn ProcessGroupPolicyCompleted(extensionid: *const windows_core::GUID, pasynchandle: ASYNCCOMPLETIONHANDLE, dwstatus: u32) -> u32 {
    windows_core::link!("userenv.dll" "system" fn ProcessGroupPolicyCompleted(extensionid : *const windows_core::GUID, pasynchandle : ASYNCCOMPLETIONHANDLE, dwstatus : u32) -> u32);
    unsafe { ProcessGroupPolicyCompleted(extensionid, pasynchandle, dwstatus) }
}
#[inline]
pub unsafe fn ProcessGroupPolicyCompletedEx(extensionid: *const windows_core::GUID, pasynchandle: ASYNCCOMPLETIONHANDLE, dwstatus: u32, rsopstatus: windows_core::HRESULT) -> u32 {
    windows_core::link!("userenv.dll" "system" fn ProcessGroupPolicyCompletedEx(extensionid : *const windows_core::GUID, pasynchandle : ASYNCCOMPLETIONHANDLE, dwstatus : u32, rsopstatus : windows_core::HRESULT) -> u32);
    unsafe { ProcessGroupPolicyCompletedEx(extensionid, pasynchandle, dwstatus, rsopstatus) }
}
#[inline]
pub unsafe fn RefreshPolicy(bmachine: bool) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn RefreshPolicy(bmachine : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { RefreshPolicy(bmachine.into()) }
}
#[inline]
pub unsafe fn RefreshPolicyEx(bmachine: bool, dwoptions: u32) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn RefreshPolicyEx(bmachine : windows_core::BOOL, dwoptions : u32) -> windows_core::BOOL);
    unsafe { RefreshPolicyEx(bmachine.into(), dwoptions) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RegisterGPNotification(hevent: super::winnt::HANDLE, bmachine: bool) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn RegisterGPNotification(hevent : super::winnt::HANDLE, bmachine : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { RegisterGPNotification(hevent, bmachine.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RsopAccessCheckByType(psecuritydescriptor: super::winnt::PSECURITY_DESCRIPTOR, pprincipalselfsid: Option<super::winnt::PSID>, prsoptoken: PRSOPTOKEN, dwdesiredaccessmask: u32, pobjecttypelist: Option<&[super::winnt::OBJECT_TYPE_LIST]>, pgenericmapping: *const super::winnt::GENERIC_MAPPING, pprivilegeset: Option<*const super::winnt::PRIVILEGE_SET>, pdwprivilegesetlength: Option<*const u32>, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("userenv.dll" "system" fn RsopAccessCheckByType(psecuritydescriptor : super::winnt::PSECURITY_DESCRIPTOR, pprincipalselfsid : super::winnt::PSID, prsoptoken : PRSOPTOKEN, dwdesiredaccessmask : u32, pobjecttypelist : *const super::winnt::OBJECT_TYPE_LIST, objecttypelistlength : u32, pgenericmapping : *const super::winnt::GENERIC_MAPPING, pprivilegeset : *const super::winnt::PRIVILEGE_SET, pdwprivilegesetlength : *const u32, pdwgrantedaccessmask : *mut u32, pbaccessstatus : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { RsopAccessCheckByType(psecuritydescriptor, pprincipalselfsid.unwrap_or(core::mem::zeroed()) as _, prsoptoken, dwdesiredaccessmask, core::mem::transmute(pobjecttypelist.map_or(core::ptr::null(), |slice| slice.as_ptr())), pobjecttypelist.map_or(0, |slice| slice.len().try_into().unwrap()), pgenericmapping, pprivilegeset.unwrap_or(core::mem::zeroed()) as _, pdwprivilegesetlength.unwrap_or(core::mem::zeroed()) as _, pdwgrantedaccessmask as _, pbaccessstatus as _) }
}
#[inline]
pub unsafe fn RsopFileAccessCheck<P0>(pszfilename: P0, prsoptoken: PRSOPTOKEN, dwdesiredaccessmask: u32, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut windows_core::BOOL) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn RsopFileAccessCheck(pszfilename : windows_core::PCWSTR, prsoptoken : PRSOPTOKEN, dwdesiredaccessmask : u32, pdwgrantedaccessmask : *mut u32, pbaccessstatus : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { RsopFileAccessCheck(pszfilename.param().abi(), prsoptoken, dwdesiredaccessmask, pdwgrantedaccessmask as _, pbaccessstatus as _) }
}
#[cfg(feature = "wbemcli")]
#[inline]
pub unsafe fn RsopResetPolicySettingStatus<P1, P2>(dwflags: u32, pservices: P1, psettinginstance: P2) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::wbemcli::IWbemServices>,
    P2: windows_core::Param<super::wbemcli::IWbemClassObject>,
{
    windows_core::link!("userenv.dll" "system" fn RsopResetPolicySettingStatus(dwflags : u32, pservices : *mut core::ffi::c_void, psettinginstance : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RsopResetPolicySettingStatus(dwflags, pservices.param().abi(), psettinginstance.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "wbemcli"))]
#[inline]
pub unsafe fn RsopSetPolicySettingStatus<P1, P2>(dwflags: u32, pservices: P1, psettinginstance: P2, pstatus: &[POLICYSETTINGSTATUSINFO]) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::wbemcli::IWbemServices>,
    P2: windows_core::Param<super::wbemcli::IWbemClassObject>,
{
    windows_core::link!("userenv.dll" "system" fn RsopSetPolicySettingStatus(dwflags : u32, pservices : *mut core::ffi::c_void, psettinginstance : *mut core::ffi::c_void, ninfo : u32, pstatus : *const POLICYSETTINGSTATUSINFO) -> windows_core::HRESULT);
    unsafe { RsopSetPolicySettingStatus(dwflags, pservices.param().abi(), psettinginstance.param().abi(), pstatus.len().try_into().unwrap(), core::mem::transmute(pstatus.as_ptr())) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn UnloadUserProfile(htoken: super::winnt::HANDLE, hprofile: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn UnloadUserProfile(htoken : super::winnt::HANDLE, hprofile : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { UnloadUserProfile(htoken, hprofile) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn UnregisterGPNotification(hevent: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn UnregisterGPNotification(hevent : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { UnregisterGPNotification(hevent) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ASYNCCOMPLETIONHANDLE(pub usize);
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GROUP_POLICY_OBJECTA {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: windows_core::PSTR,
    pub lpFileSysPath: windows_core::PSTR,
    pub lpDisplayName: windows_core::PSTR,
    pub szGPOName: [i8; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::minwindef::LPARAM,
    pub pNext: *mut Self,
    pub pPrev: *mut Self,
    pub lpExtensions: windows_core::PSTR,
    pub lParam2: super::minwindef::LPARAM,
    pub lpLink: windows_core::PSTR,
}
#[cfg(feature = "minwindef")]
impl Default for GROUP_POLICY_OBJECTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GROUP_POLICY_OBJECTW {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: windows_core::PWSTR,
    pub lpFileSysPath: windows_core::PWSTR,
    pub lpDisplayName: windows_core::PWSTR,
    pub szGPOName: [u16; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::minwindef::LPARAM,
    pub pNext: *mut Self,
    pub pPrev: *mut Self,
    pub lpExtensions: windows_core::PWSTR,
    pub lParam2: super::minwindef::LPARAM,
    pub lpLink: windows_core::PWSTR,
}
#[cfg(feature = "minwindef")]
impl Default for GROUP_POLICY_OBJECTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwinbase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPOLICYSETTINGSTATUSINFO(pub *mut POLICYSETTINGSTATUSINFO);
#[cfg(feature = "minwinbase")]
impl LPPOLICYSETTINGSTATUSINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwinbase")]
impl Default for LPPOLICYSETTINGSTATUSINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wbemcli"))]
pub type PFNGENERATEGROUPPOLICY = Option<unsafe extern "system" fn(dwflags: u32, pbabort: *mut windows_core::BOOL, pwszsite: *const u16, pcomputertarget: *const RSOP_TARGET, pusertarget: *const RSOP_TARGET) -> u32>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PFNPROCESSGROUPPOLICY = Option<unsafe extern "system" fn(dwflags: u32, htoken: super::winnt::HANDLE, hkeyroot: super::minwindef::HKEY, pdeletedgpolist: PGROUP_POLICY_OBJECT, pchangedgpolist: PGROUP_POLICY_OBJECT, phandle: ASYNCCOMPLETIONHANDLE, pbabort: *mut windows_core::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK) -> u32>;
#[cfg(all(feature = "minwindef", feature = "wbemcli", feature = "winnt"))]
pub type PFNPROCESSGROUPPOLICYEX = Option<unsafe extern "system" fn(dwflags: u32, htoken: super::winnt::HANDLE, hkeyroot: super::minwindef::HKEY, pdeletedgpolist: PGROUP_POLICY_OBJECT, pchangedgpolist: PGROUP_POLICY_OBJECT, phandle: ASYNCCOMPLETIONHANDLE, pbabort: *mut windows_core::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK, pwbemservices: windows_core::Ref<super::wbemcli::IWbemServices>, prsopstatus: *mut windows_core::HRESULT) -> u32>;
pub type PFNSTATUSMESSAGECALLBACK = Option<unsafe extern "system" fn(bverbose: windows_core::BOOL, lpmessage: windows_core::PCWSTR) -> u32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGPO_LINK(pub *mut GPO_LINK);
impl PGPO_LINK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PGPO_LINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PGROUP_POLICY_OBJECT(pub PGROUP_POLICY_OBJECTA);
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGROUP_POLICY_OBJECTA(pub *mut GROUP_POLICY_OBJECTA);
#[cfg(feature = "minwindef")]
impl PGROUP_POLICY_OBJECTA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PGROUP_POLICY_OBJECTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGROUP_POLICY_OBJECTW(pub *mut GROUP_POLICY_OBJECTW);
#[cfg(feature = "minwindef")]
impl PGROUP_POLICY_OBJECTW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PGROUP_POLICY_OBJECTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PI_APPLYPOLICY: u32 = 2;
pub const PI_NOUI: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwinbase")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POLICYSETTINGSTATUSINFO {
    pub szKey: windows_core::PWSTR,
    pub szEventSource: windows_core::PWSTR,
    pub szEventLogName: windows_core::PWSTR,
    pub dwEventID: u32,
    pub dwErrorCode: u32,
    pub status: SETTINGSTATUS,
    pub timeLogged: super::minwinbase::SYSTEMTIME,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRSOPTOKEN(pub *mut core::ffi::c_void);
impl PRSOPTOKEN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRSOPTOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wbemcli"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRSOP_TARGET(pub *mut RSOP_TARGET);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wbemcli"))]
impl PRSOP_TARGET {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wbemcli"))]
impl Default for PRSOP_TARGET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PT_MANDATORY: u32 = 4;
pub const PT_ROAMING: u32 = 2;
pub const PT_ROAMING_PREEXISTING: u32 = 8;
pub const PT_TEMPORARY: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct REFGPEXTENSIONID(pub *mut windows_core::GUID);
impl REFGPEXTENSIONID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for REFGPEXTENSIONID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[derive(Clone, Debug, PartialEq)]
pub struct RSOP_TARGET {
    pub pwszAccountName: *mut u16,
    pub pwszNewSOM: *mut u16,
    pub psaSecurityGroups: *mut super::oaidl::SAFEARRAY,
    pub pRsopToken: PRSOPTOKEN,
    pub pGPOList: PGROUP_POLICY_OBJECT,
    pub pWbemServices: core::mem::ManuallyDrop<Option<super::wbemcli::IWbemServices>>,
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
