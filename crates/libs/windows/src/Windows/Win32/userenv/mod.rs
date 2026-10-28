#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateAppContainerProfile<P0, P1, P2>(pszappcontainername: P0, pszdisplayname: P1, pszdescription: P2, pcapabilities: Option<&[super::SID_AND_ATTRIBUTES]>, ppsidappcontainersid: *mut super::PSID) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn CreateAppContainerProfile(pszappcontainername : windows_core::PCWSTR, pszdisplayname : windows_core::PCWSTR, pszdescription : windows_core::PCWSTR, pcapabilities : super::PSID_AND_ATTRIBUTES, dwcapabilitycount : super::DWORD, ppsidappcontainersid : *mut super::PSID) -> windows_core::HRESULT);
    unsafe { CreateAppContainerProfile(pszappcontainername.param().abi(), pszdisplayname.param().abi(), pszdescription.param().abi(), core::mem::transmute(pcapabilities.map_or(core::ptr::null(), |slice| slice.as_ptr())), pcapabilities.map_or(0, |slice| slice.len().try_into().unwrap()), ppsidappcontainersid as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn CreateEnvironmentBlock(lpenvironment: *mut super::LPVOID, htoken: Option<super::HANDLE>, binherit: bool) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn CreateEnvironmentBlock(lpenvironment : *mut super::LPVOID, htoken : super::HANDLE, binherit : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CreateEnvironmentBlock(lpenvironment as _, htoken.unwrap_or(core::mem::zeroed()) as _, binherit.into()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CreateProfile<P0, P1>(pszusersid: P0, pszusername: P1, pszprofilepath: windows_core::PWSTR, cchprofilepath: super::DWORD) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn CreateProfile(pszusersid : windows_core::PCWSTR, pszusername : windows_core::PCWSTR, pszprofilepath : windows_core::PWSTR, cchprofilepath : super::DWORD) -> windows_core::HRESULT);
    unsafe { CreateProfile(pszusersid.param().abi(), pszusername.param().abi(), pszprofilepath, cchprofilepath) }
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
pub unsafe fn DeriveAppContainerSidFromAppContainerName<P0>(pszappcontainername: P0, ppsidappcontainersid: *mut super::PSID) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn DeriveAppContainerSidFromAppContainerName(pszappcontainername : windows_core::PCWSTR, ppsidappcontainersid : *mut super::PSID) -> windows_core::HRESULT);
    unsafe { DeriveAppContainerSidFromAppContainerName(pszappcontainername.param().abi(), ppsidappcontainersid as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName<P1>(psidappcontainersid: super::PSID, pszrestrictedappcontainername: P1, ppsidrestrictedappcontainersid: *mut super::PSID) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid : super::PSID, pszrestrictedappcontainername : windows_core::PCWSTR, ppsidrestrictedappcontainersid : *mut super::PSID) -> windows_core::HRESULT);
    unsafe { DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid, pszrestrictedappcontainername.param().abi(), ppsidrestrictedappcontainersid as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DestroyEnvironmentBlock(lpenvironment: super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn DestroyEnvironmentBlock(lpenvironment : super::LPVOID) -> windows_core::BOOL);
    unsafe { DestroyEnvironmentBlock(lpenvironment) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EnterCriticalPolicySection(bmachine: bool) -> super::HANDLE {
    windows_core::link!("userenv.dll" "system" fn EnterCriticalPolicySection(bmachine : windows_core::BOOL) -> super::HANDLE);
    unsafe { EnterCriticalPolicySection(bmachine.into()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ExpandEnvironmentStringsForUserA<P1>(htoken: Option<super::HANDLE>, lpsrc: P1, lpdest: windows_core::PSTR, dwsize: super::DWORD) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("userenv.dll" "system" fn ExpandEnvironmentStringsForUserA(htoken : super::HANDLE, lpsrc : windows_core::PCSTR, lpdest : windows_core::PSTR, dwsize : super::DWORD) -> windows_core::BOOL);
    unsafe { ExpandEnvironmentStringsForUserA(htoken.unwrap_or(core::mem::zeroed()) as _, lpsrc.param().abi(), lpdest, dwsize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ExpandEnvironmentStringsForUserW<P1>(htoken: Option<super::HANDLE>, lpsrc: P1, lpdest: windows_core::PWSTR, dwsize: super::DWORD) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn ExpandEnvironmentStringsForUserW(htoken : super::HANDLE, lpsrc : windows_core::PCWSTR, lpdest : windows_core::PWSTR, dwsize : super::DWORD) -> windows_core::BOOL);
    unsafe { ExpandEnvironmentStringsForUserW(htoken.unwrap_or(core::mem::zeroed()) as _, lpsrc.param().abi(), lpdest, dwsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn FreeGPOListA(pgpolist: PGROUP_POLICY_OBJECTA) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn FreeGPOListA(pgpolist : PGROUP_POLICY_OBJECTA) -> windows_core::BOOL);
    unsafe { FreeGPOListA(pgpolist) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FreeGPOListW(pgpolist: PGROUP_POLICY_OBJECTW) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn FreeGPOListW(pgpolist : PGROUP_POLICY_OBJECTW) -> windows_core::BOOL);
    unsafe { FreeGPOListW(pgpolist) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GenerateGPNotification<P1>(bmachine: bool, lpwszmgmtproduct: P1, dwmgmtproductoptions: super::DWORD) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GenerateGPNotification(bmachine : windows_core::BOOL, lpwszmgmtproduct : windows_core::PCWSTR, dwmgmtproductoptions : super::DWORD) -> super::DWORD);
    unsafe { GenerateGPNotification(bmachine.into(), lpwszmgmtproduct.param().abi(), dwmgmtproductoptions) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetAllUsersProfileDirectoryA(lpprofiledir: Option<windows_core::PSTR>, lpcchsize: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetAllUsersProfileDirectoryA(lpprofiledir : windows_core::PSTR, lpcchsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetAllUsersProfileDirectoryA(lpprofiledir.unwrap_or(core::mem::zeroed()) as _, lpcchsize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetAllUsersProfileDirectoryW(lpprofiledir: Option<windows_core::PWSTR>, lpcchsize: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetAllUsersProfileDirectoryW(lpprofiledir : windows_core::PWSTR, lpcchsize : super::LPDWORD) -> windows_core::BOOL);
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
#[cfg(all(feature = "minwindef", feature = "winreg"))]
#[inline]
pub unsafe fn GetAppContainerRegistryLocation(desiredaccess: super::REGSAM) -> windows_core::Result<*mut super::HKEY__> {
    windows_core::link!("userenv.dll" "system" fn GetAppContainerRegistryLocation(desiredaccess : super::REGSAM, phappcontainerkey : super::PHKEY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetAppContainerRegistryLocation(desiredaccess, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetAppliedGPOListA<P1>(dwflags: super::DWORD, pmachinename: P1, psiduser: Option<super::PSID>, pguidextension: *const windows_core::GUID, ppgpolist: *mut PGROUP_POLICY_OBJECTA) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GetAppliedGPOListA(dwflags : super::DWORD, pmachinename : windows_core::PCSTR, psiduser : super::PSID, pguidextension : *const windows_core::GUID, ppgpolist : *mut PGROUP_POLICY_OBJECTA) -> super::DWORD);
    unsafe { GetAppliedGPOListA(dwflags, pmachinename.param().abi(), psiduser.unwrap_or(core::mem::zeroed()) as _, pguidextension, ppgpolist as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetAppliedGPOListW<P1>(dwflags: super::DWORD, pmachinename: P1, psiduser: Option<super::PSID>, pguidextension: *const windows_core::GUID, ppgpolist: *mut PGROUP_POLICY_OBJECTW) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GetAppliedGPOListW(dwflags : super::DWORD, pmachinename : windows_core::PCWSTR, psiduser : super::PSID, pguidextension : *const windows_core::GUID, ppgpolist : *mut PGROUP_POLICY_OBJECTW) -> super::DWORD);
    unsafe { GetAppliedGPOListW(dwflags, pmachinename.param().abi(), psiduser.unwrap_or(core::mem::zeroed()) as _, pguidextension, ppgpolist as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetDefaultUserProfileDirectoryA(lpprofiledir: Option<windows_core::PSTR>, lpcchsize: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetDefaultUserProfileDirectoryA(lpprofiledir : windows_core::PSTR, lpcchsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetDefaultUserProfileDirectoryA(lpprofiledir.unwrap_or(core::mem::zeroed()) as _, lpcchsize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetDefaultUserProfileDirectoryW(lpprofiledir: Option<windows_core::PWSTR>, lpcchsize: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetDefaultUserProfileDirectoryW(lpprofiledir : windows_core::PWSTR, lpcchsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetDefaultUserProfileDirectoryW(lpprofiledir.unwrap_or(core::mem::zeroed()) as _, lpcchsize as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetGPOListA<P1, P2, P3>(htoken: Option<super::HANDLE>, lpname: P1, lphostname: P2, lpcomputername: P3, dwflags: super::DWORD, pgpolist: *mut PGROUP_POLICY_OBJECTA) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GetGPOListA(htoken : super::HANDLE, lpname : windows_core::PCSTR, lphostname : windows_core::PCSTR, lpcomputername : windows_core::PCSTR, dwflags : super::DWORD, pgpolist : *mut PGROUP_POLICY_OBJECTA) -> windows_core::BOOL);
    unsafe { GetGPOListA(htoken.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi(), lphostname.param().abi(), lpcomputername.param().abi(), dwflags, pgpolist as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetGPOListW<P1, P2, P3>(htoken: Option<super::HANDLE>, lpname: P1, lphostname: P2, lpcomputername: P3, dwflags: super::DWORD, pgpolist: *mut PGROUP_POLICY_OBJECTW) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn GetGPOListW(htoken : super::HANDLE, lpname : windows_core::PCWSTR, lphostname : windows_core::PCWSTR, lpcomputername : windows_core::PCWSTR, dwflags : super::DWORD, pgpolist : *mut PGROUP_POLICY_OBJECTW) -> windows_core::BOOL);
    unsafe { GetGPOListW(htoken.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi(), lphostname.param().abi(), lpcomputername.param().abi(), dwflags, pgpolist as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetProfileType(dwflags: *mut super::DWORD) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetProfileType(dwflags : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { GetProfileType(dwflags as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetProfilesDirectoryA(lpprofiledir: Option<windows_core::PSTR>, lpcchsize: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetProfilesDirectoryA(lpprofiledir : windows_core::PSTR, lpcchsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetProfilesDirectoryA(lpprofiledir.unwrap_or(core::mem::zeroed()) as _, lpcchsize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetProfilesDirectoryW(lpprofiledir: Option<windows_core::PWSTR>, lpcchsize: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetProfilesDirectoryW(lpprofiledir : windows_core::PWSTR, lpcchsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetProfilesDirectoryW(lpprofiledir.unwrap_or(core::mem::zeroed()) as _, lpcchsize as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetUserProfileDirectoryA(htoken: super::HANDLE, lpprofiledir: Option<windows_core::PSTR>, lpcchsize: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetUserProfileDirectoryA(htoken : super::HANDLE, lpprofiledir : windows_core::PSTR, lpcchsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetUserProfileDirectoryA(htoken, lpprofiledir.unwrap_or(core::mem::zeroed()) as _, lpcchsize as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetUserProfileDirectoryW(htoken: super::HANDLE, lpprofiledir: Option<windows_core::PWSTR>, lpcchsize: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn GetUserProfileDirectoryW(htoken : super::HANDLE, lpprofiledir : windows_core::PWSTR, lpcchsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetUserProfileDirectoryW(htoken, lpprofiledir.unwrap_or(core::mem::zeroed()) as _, lpcchsize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn LeaveCriticalPolicySection(hsection: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn LeaveCriticalPolicySection(hsection : super::HANDLE) -> windows_core::BOOL);
    unsafe { LeaveCriticalPolicySection(hsection) }
}
#[cfg(all(feature = "profinfo", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn LoadUserProfileA(htoken: super::HANDLE, lpprofileinfo: super::LPPROFILEINFOA) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn LoadUserProfileA(htoken : super::HANDLE, lpprofileinfo : super::LPPROFILEINFOA) -> windows_core::BOOL);
    unsafe { LoadUserProfileA(htoken, lpprofileinfo as _) }
}
#[cfg(all(feature = "profinfo", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn LoadUserProfileW(htoken: super::HANDLE, lpprofileinfo: super::LPPROFILEINFOW) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn LoadUserProfileW(htoken : super::HANDLE, lpprofileinfo : super::LPPROFILEINFOW) -> windows_core::BOOL);
    unsafe { LoadUserProfileW(htoken, lpprofileinfo as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ProcessGroupPolicyCompleted(extensionid: REFGPEXTENSIONID, pasynchandle: ASYNCCOMPLETIONHANDLE, dwstatus: super::DWORD) -> super::DWORD {
    windows_core::link!("userenv.dll" "system" fn ProcessGroupPolicyCompleted(extensionid : REFGPEXTENSIONID, pasynchandle : ASYNCCOMPLETIONHANDLE, dwstatus : super::DWORD) -> super::DWORD);
    unsafe { ProcessGroupPolicyCompleted(extensionid, pasynchandle, dwstatus) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ProcessGroupPolicyCompletedEx(extensionid: REFGPEXTENSIONID, pasynchandle: ASYNCCOMPLETIONHANDLE, dwstatus: super::DWORD, rsopstatus: windows_core::HRESULT) -> super::DWORD {
    windows_core::link!("userenv.dll" "system" fn ProcessGroupPolicyCompletedEx(extensionid : REFGPEXTENSIONID, pasynchandle : ASYNCCOMPLETIONHANDLE, dwstatus : super::DWORD, rsopstatus : windows_core::HRESULT) -> super::DWORD);
    unsafe { ProcessGroupPolicyCompletedEx(extensionid, pasynchandle, dwstatus, rsopstatus) }
}
#[inline]
pub unsafe fn RefreshPolicy(bmachine: bool) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn RefreshPolicy(bmachine : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { RefreshPolicy(bmachine.into()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RefreshPolicyEx(bmachine: bool, dwoptions: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn RefreshPolicyEx(bmachine : windows_core::BOOL, dwoptions : super::DWORD) -> windows_core::BOOL);
    unsafe { RefreshPolicyEx(bmachine.into(), dwoptions) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RegisterGPNotification(hevent: super::HANDLE, bmachine: bool) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn RegisterGPNotification(hevent : super::HANDLE, bmachine : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { RegisterGPNotification(hevent, bmachine.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RsopAccessCheckByType(psecuritydescriptor: super::PSECURITY_DESCRIPTOR, pprincipalselfsid: Option<super::PSID>, prsoptoken: PRSOPTOKEN, dwdesiredaccessmask: super::DWORD, pobjecttypelist: Option<&[super::OBJECT_TYPE_LIST]>, pgenericmapping: super::PGENERIC_MAPPING, pprivilegeset: Option<super::PPRIVILEGE_SET>, pdwprivilegesetlength: Option<super::LPDWORD>, pdwgrantedaccessmask: super::LPDWORD, pbaccessstatus: super::LPBOOL) -> windows_core::HRESULT {
    windows_core::link!("userenv.dll" "system" fn RsopAccessCheckByType(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, pprincipalselfsid : super::PSID, prsoptoken : PRSOPTOKEN, dwdesiredaccessmask : super::DWORD, pobjecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : super::DWORD, pgenericmapping : super::PGENERIC_MAPPING, pprivilegeset : super::PPRIVILEGE_SET, pdwprivilegesetlength : super::LPDWORD, pdwgrantedaccessmask : super::LPDWORD, pbaccessstatus : super::LPBOOL) -> windows_core::HRESULT);
    unsafe { RsopAccessCheckByType(psecuritydescriptor, pprincipalselfsid.unwrap_or(core::mem::zeroed()) as _, prsoptoken, dwdesiredaccessmask, core::mem::transmute(pobjecttypelist.map_or(core::ptr::null(), |slice| slice.as_ptr())), pobjecttypelist.map_or(0, |slice| slice.len().try_into().unwrap()), pgenericmapping, pprivilegeset.unwrap_or(core::mem::zeroed()) as _, pdwprivilegesetlength.unwrap_or(core::mem::zeroed()) as _, pdwgrantedaccessmask as _, pbaccessstatus as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn RsopFileAccessCheck<P0>(pszfilename: P0, prsoptoken: PRSOPTOKEN, dwdesiredaccessmask: super::DWORD, pdwgrantedaccessmask: super::LPDWORD, pbaccessstatus: super::LPBOOL) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("userenv.dll" "system" fn RsopFileAccessCheck(pszfilename : windows_core::PCWSTR, prsoptoken : PRSOPTOKEN, dwdesiredaccessmask : super::DWORD, pdwgrantedaccessmask : super::LPDWORD, pbaccessstatus : super::LPBOOL) -> windows_core::HRESULT);
    unsafe { RsopFileAccessCheck(pszfilename.param().abi(), prsoptoken, dwdesiredaccessmask, pdwgrantedaccessmask as _, pbaccessstatus as _) }
}
#[cfg(all(feature = "shared", feature = "wbemcli"))]
#[inline]
pub unsafe fn RsopResetPolicySettingStatus<P1, P2>(dwflags: super::DWORD, pservices: P1, psettinginstance: P2) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::IWbemServices>,
    P2: windows_core::Param<super::IWbemClassObject>,
{
    windows_core::link!("userenv.dll" "system" fn RsopResetPolicySettingStatus(dwflags : super::DWORD, pservices : *mut core::ffi::c_void, psettinginstance : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RsopResetPolicySettingStatus(dwflags, pservices.param().abi(), psettinginstance.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "wbemcli"))]
#[inline]
pub unsafe fn RsopSetPolicySettingStatus<P1, P2>(dwflags: super::DWORD, pservices: P1, psettinginstance: P2, pstatus: &[POLICYSETTINGSTATUSINFO]) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::IWbemServices>,
    P2: windows_core::Param<super::IWbemClassObject>,
{
    windows_core::link!("userenv.dll" "system" fn RsopSetPolicySettingStatus(dwflags : super::DWORD, pservices : *mut core::ffi::c_void, psettinginstance : *mut core::ffi::c_void, ninfo : super::DWORD, pstatus : *const POLICYSETTINGSTATUSINFO) -> windows_core::HRESULT);
    unsafe { RsopSetPolicySettingStatus(dwflags, pservices.param().abi(), psettinginstance.param().abi(), pstatus.len().try_into().unwrap(), pstatus.as_ptr()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn UnloadUserProfile(htoken: super::HANDLE, hprofile: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn UnloadUserProfile(htoken : super::HANDLE, hprofile : super::HANDLE) -> windows_core::BOOL);
    unsafe { UnloadUserProfile(htoken, hprofile) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn UnregisterGPNotification(hevent: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("userenv.dll" "system" fn UnregisterGPNotification(hevent : super::HANDLE) -> windows_core::BOOL);
    unsafe { UnregisterGPNotification(hevent) }
}
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
pub const GP_DLLNAME: windows_core::PCSTR = windows_core::s!("DllName");
pub const GP_ENABLEASYNCHRONOUSPROCESSING: windows_core::PCSTR = windows_core::s!("EnableAsynchronousProcessing");
pub const GP_MAXNOGPOLISTCHANGESINTERVAL: windows_core::PCSTR = windows_core::s!("MaxNoGPOListChangesInterval");
pub const GP_NOBACKGROUNDPOLICY: windows_core::PCSTR = windows_core::s!("NoBackgroundPolicy");
pub const GP_NOGPOLISTCHANGES: windows_core::PCSTR = windows_core::s!("NoGPOListChanges");
pub const GP_NOMACHINEPOLICY: windows_core::PCSTR = windows_core::s!("NoMachinePolicy");
pub const GP_NOSLOWLINK: windows_core::PCSTR = windows_core::s!("NoSlowLink");
pub const GP_NOTIFYLINKTRANSITION: windows_core::PCSTR = windows_core::s!("NotifyLinkTransition");
pub const GP_NOUSERPOLICY: windows_core::PCSTR = windows_core::s!("NoUserPolicy");
pub const GP_PERUSERLOCALSETTINGS: windows_core::PCSTR = windows_core::s!("PerUserLocalSettings");
pub const GP_PROCESSGROUPPOLICY: windows_core::PCSTR = windows_core::s!("ProcessGroupPolicy");
pub const GP_REQUIRESSUCCESSFULREGISTRY: windows_core::PCSTR = windows_core::s!("RequiresSuccessfulRegistry");
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type GROUP_POLICY_OBJECT = GROUP_POLICY_OBJECTA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GROUP_POLICY_OBJECTA {
    pub dwOptions: super::DWORD,
    pub dwVersion: super::DWORD,
    pub lpDSPath: windows_core::PSTR,
    pub lpFileSysPath: windows_core::PSTR,
    pub lpDisplayName: windows_core::PSTR,
    pub szGPOName: [super::CHAR; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::LPARAM,
    pub pNext: *mut Self,
    pub pPrev: *mut Self,
    pub lpExtensions: windows_core::PSTR,
    pub lParam2: super::LPARAM,
    pub lpLink: windows_core::PSTR,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for GROUP_POLICY_OBJECTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GROUP_POLICY_OBJECTW {
    pub dwOptions: super::DWORD,
    pub dwVersion: super::DWORD,
    pub lpDSPath: windows_core::PWSTR,
    pub lpFileSysPath: windows_core::PWSTR,
    pub lpDisplayName: windows_core::PWSTR,
    pub szGPOName: [super::WCHAR; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::LPARAM,
    pub pNext: *mut Self,
    pub pPrev: *mut Self,
    pub lpExtensions: windows_core::PWSTR,
    pub lParam2: super::LPARAM,
    pub lpLink: windows_core::PWSTR,
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
pub type PFNGENERATEGROUPPOLICY = Option<unsafe extern "C" fn(dwflags: super::DWORD, pbabort: *mut windows_core::BOOL, pwszsite: *const super::WCHAR, pcomputertarget: PRSOP_TARGET, pusertarget: PRSOP_TARGET) -> super::DWORD>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PFNPROCESSGROUPPOLICY = Option<unsafe extern "C" fn(dwflags: super::DWORD, htoken: super::HANDLE, hkeyroot: super::HKEY, pdeletedgpolist: PGROUP_POLICY_OBJECT, pchangedgpolist: PGROUP_POLICY_OBJECT, phandle: ASYNCCOMPLETIONHANDLE, pbabort: *mut windows_core::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK) -> super::DWORD>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wbemcli", feature = "winnt"))]
pub type PFNPROCESSGROUPPOLICYEX = Option<unsafe extern "C" fn(dwflags: super::DWORD, htoken: super::HANDLE, hkeyroot: super::HKEY, pdeletedgpolist: PGROUP_POLICY_OBJECT, pchangedgpolist: PGROUP_POLICY_OBJECT, phandle: ASYNCCOMPLETIONHANDLE, pbabort: *mut windows_core::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK, pwbemservices: windows_core::Ref<super::IWbemServices>, prsopstatus: *mut windows_core::HRESULT) -> super::DWORD>;
#[cfg(feature = "shared")]
pub type PFNSTATUSMESSAGECALLBACK = Option<unsafe extern "C" fn(bverbose: windows_core::BOOL, lpmessage: windows_core::PCWSTR) -> super::DWORD>;
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POLICYSETTINGSTATUSINFO {
    pub szKey: windows_core::PWSTR,
    pub szEventSource: windows_core::PWSTR,
    pub szEventLogName: windows_core::PWSTR,
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
pub type REFGPEXTENSIONID = *mut windows_core::GUID;
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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RSOP_TARGET {
    pub pwszAccountName: *mut super::WCHAR,
    pub pwszNewSOM: *mut super::WCHAR,
    pub psaSecurityGroups: *mut super::SAFEARRAY,
    pub pRsopToken: PRSOPTOKEN,
    pub pGPOList: PGROUP_POLICY_OBJECT,
    pub pWbemServices: core::mem::ManuallyDrop<Option<super::IWbemServices>>,
}
pub const RSOP_TEMPNAMESPACE_EXISTS: i32 = 4;
pub const RSOP_USER_ACCESS_DENIED: i32 = 1;
pub type SETTINGSTATUS = i32;
