#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BrowseForGPO(lpbrowseinfo: *mut GPOBROWSEINFO) -> ::windows_core::Result<()> {
    ::windows_targets::link!("gpedit.dll" "system" fn BrowseForGPO(lpbrowseinfo : *mut GPOBROWSEINFO) -> ::windows_core::HRESULT);
    BrowseForGPO(lpbrowseinfo).ok()
}
#[inline]
pub unsafe fn CommandLineFromMsiDescriptor<P0>(descriptor: P0, commandline: ::windows_core::PWSTR, commandlinelength: *mut u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn CommandLineFromMsiDescriptor(descriptor : ::windows_core::PCWSTR, commandline : ::windows_core::PWSTR, commandlinelength : *mut u32) -> u32);
    CommandLineFromMsiDescriptor(descriptor.into_param().abi(), ::core::mem::transmute(commandline), commandlinelength)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateGPOLink<P0, P1, P2>(lpgpo: P0, lpcontainer: P1, fhighpriority: P2) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("gpedit.dll" "system" fn CreateGPOLink(lpgpo : ::windows_core::PCWSTR, lpcontainer : ::windows_core::PCWSTR, fhighpriority : super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    CreateGPOLink(lpgpo.into_param().abi(), lpcontainer.into_param().abi(), fhighpriority.into_param().abi()).ok()
}
#[inline]
pub unsafe fn DeleteAllGPOLinks<P0>(lpcontainer: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("gpedit.dll" "system" fn DeleteAllGPOLinks(lpcontainer : ::windows_core::PCWSTR) -> ::windows_core::HRESULT);
    DeleteAllGPOLinks(lpcontainer.into_param().abi()).ok()
}
#[inline]
pub unsafe fn DeleteGPOLink<P0, P1>(lpgpo: P0, lpcontainer: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("gpedit.dll" "system" fn DeleteGPOLink(lpgpo : ::windows_core::PCWSTR, lpcontainer : ::windows_core::PCWSTR) -> ::windows_core::HRESULT);
    DeleteGPOLink(lpgpo.into_param().abi(), lpcontainer.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnterCriticalPolicySection<P0>(bmachine: P0) -> ::windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("userenv.dll" "system" fn EnterCriticalPolicySection(bmachine : super::super::Foundation:: BOOL) -> super::super::Foundation:: HANDLE);
    let result__ = EnterCriticalPolicySection(bmachine.into_param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn ExportRSoPData<P0, P1>(lpnamespace: P0, lpfilename: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("gpedit.dll" "system" fn ExportRSoPData(lpnamespace : ::windows_core::PCWSTR, lpfilename : ::windows_core::PCWSTR) -> ::windows_core::HRESULT);
    ExportRSoPData(lpnamespace.into_param().abi(), lpfilename.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeGPOListA(pgpolist: *const GROUP_POLICY_OBJECTA) -> ::windows_core::Result<()> {
    ::windows_targets::link!("userenv.dll" "system" fn FreeGPOListA(pgpolist : *const GROUP_POLICY_OBJECTA) -> super::super::Foundation:: BOOL);
    FreeGPOListA(pgpolist).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeGPOListW(pgpolist: *const GROUP_POLICY_OBJECTW) -> ::windows_core::Result<()> {
    ::windows_targets::link!("userenv.dll" "system" fn FreeGPOListW(pgpolist : *const GROUP_POLICY_OBJECTW) -> super::super::Foundation:: BOOL);
    FreeGPOListW(pgpolist).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GenerateGPNotification<P0, P1>(bmachine: P0, lpwszmgmtproduct: P1, dwmgmtproductoptions: u32) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("userenv.dll" "system" fn GenerateGPNotification(bmachine : super::super::Foundation:: BOOL, lpwszmgmtproduct : ::windows_core::PCWSTR, dwmgmtproductoptions : u32) -> u32);
    GenerateGPNotification(bmachine.into_param().abi(), lpwszmgmtproduct.into_param().abi(), dwmgmtproductoptions)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAppliedGPOListA<P0, P1>(dwflags: u32, pmachinename: P0, psiduser: P1, pguidextension: *const ::windows_core::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows_targets::link!("userenv.dll" "system" fn GetAppliedGPOListA(dwflags : u32, pmachinename : ::windows_core::PCSTR, psiduser : super::super::Foundation:: PSID, pguidextension : *const ::windows_core::GUID, ppgpolist : *mut *mut GROUP_POLICY_OBJECTA) -> u32);
    GetAppliedGPOListA(dwflags, pmachinename.into_param().abi(), psiduser.into_param().abi(), pguidextension, ppgpolist)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAppliedGPOListW<P0, P1>(dwflags: u32, pmachinename: P0, psiduser: P1, pguidextension: *const ::windows_core::GUID, ppgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows_targets::link!("userenv.dll" "system" fn GetAppliedGPOListW(dwflags : u32, pmachinename : ::windows_core::PCWSTR, psiduser : super::super::Foundation:: PSID, pguidextension : *const ::windows_core::GUID, ppgpolist : *mut *mut GROUP_POLICY_OBJECTW) -> u32);
    GetAppliedGPOListW(dwflags, pmachinename.into_param().abi(), psiduser.into_param().abi(), pguidextension, ppgpolist)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGPOListA<P0, P1, P2, P3>(htoken: P0, lpname: P1, lphostname: P2, lpcomputername: P3, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTA) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("userenv.dll" "system" fn GetGPOListA(htoken : super::super::Foundation:: HANDLE, lpname : ::windows_core::PCSTR, lphostname : ::windows_core::PCSTR, lpcomputername : ::windows_core::PCSTR, dwflags : u32, pgpolist : *mut *mut GROUP_POLICY_OBJECTA) -> super::super::Foundation:: BOOL);
    GetGPOListA(htoken.into_param().abi(), lpname.into_param().abi(), lphostname.into_param().abi(), lpcomputername.into_param().abi(), dwflags, pgpolist).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGPOListW<P0, P1, P2, P3>(htoken: P0, lpname: P1, lphostname: P2, lpcomputername: P3, dwflags: u32, pgpolist: *mut *mut GROUP_POLICY_OBJECTW) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("userenv.dll" "system" fn GetGPOListW(htoken : super::super::Foundation:: HANDLE, lpname : ::windows_core::PCWSTR, lphostname : ::windows_core::PCWSTR, lpcomputername : ::windows_core::PCWSTR, dwflags : u32, pgpolist : *mut *mut GROUP_POLICY_OBJECTW) -> super::super::Foundation:: BOOL);
    GetGPOListW(htoken.into_param().abi(), lpname.into_param().abi(), lphostname.into_param().abi(), lpcomputername.into_param().abi(), dwflags, pgpolist).ok()
}
#[inline]
pub unsafe fn GetLocalManagedApplicationData<P0>(productcode: P0, displayname: *mut ::windows_core::PWSTR, supporturl: *mut ::windows_core::PWSTR)
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn GetLocalManagedApplicationData(productcode : ::windows_core::PCWSTR, displayname : *mut ::windows_core::PWSTR, supporturl : *mut ::windows_core::PWSTR) -> ());
    GetLocalManagedApplicationData(productcode.into_param().abi(), displayname, supporturl)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLocalManagedApplications<P0>(buserapps: P0, pdwapps: *mut u32, prglocalapps: *mut *mut LOCALMANAGEDAPPLICATION) -> u32
where
    P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn GetLocalManagedApplications(buserapps : super::super::Foundation:: BOOL, pdwapps : *mut u32, prglocalapps : *mut *mut LOCALMANAGEDAPPLICATION) -> u32);
    GetLocalManagedApplications(buserapps.into_param().abi(), pdwapps, prglocalapps)
}
#[doc = "Required features: `\"Win32_UI_Shell\"`"]
#[cfg(feature = "Win32_UI_Shell")]
#[inline]
pub unsafe fn GetManagedApplicationCategories(dwreserved: u32, pappcategory: *mut super::super::UI::Shell::APPCATEGORYINFOLIST) -> u32 {
    ::windows_targets::link!("advapi32.dll" "system" fn GetManagedApplicationCategories(dwreserved : u32, pappcategory : *mut super::super::UI::Shell:: APPCATEGORYINFOLIST) -> u32);
    GetManagedApplicationCategories(dwreserved, pappcategory)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetManagedApplications(pcategory: *const ::windows_core::GUID, dwqueryflags: u32, dwinfolevel: u32, pdwapps: *mut u32, prgmanagedapps: *mut *mut MANAGEDAPPLICATION) -> u32 {
    ::windows_targets::link!("advapi32.dll" "system" fn GetManagedApplications(pcategory : *const ::windows_core::GUID, dwqueryflags : u32, dwinfolevel : u32, pdwapps : *mut u32, prgmanagedapps : *mut *mut MANAGEDAPPLICATION) -> u32);
    GetManagedApplications(pcategory, dwqueryflags, dwinfolevel, pdwapps, prgmanagedapps)
}
#[inline]
pub unsafe fn ImportRSoPData<P0, P1>(lpnamespace: P0, lpfilename: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("gpedit.dll" "system" fn ImportRSoPData(lpnamespace : ::windows_core::PCWSTR, lpfilename : ::windows_core::PCWSTR) -> ::windows_core::HRESULT);
    ImportRSoPData(lpnamespace.into_param().abi(), lpfilename.into_param().abi()).ok()
}
#[inline]
pub unsafe fn InstallApplication(pinstallinfo: *const INSTALLDATA) -> u32 {
    ::windows_targets::link!("advapi32.dll" "system" fn InstallApplication(pinstallinfo : *const INSTALLDATA) -> u32);
    InstallApplication(pinstallinfo)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn LeaveCriticalPolicySection<P0>(hsection: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("userenv.dll" "system" fn LeaveCriticalPolicySection(hsection : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    LeaveCriticalPolicySection(hsection.into_param().abi()).ok()
}
#[inline]
pub unsafe fn ProcessGroupPolicyCompleted(extensionid: *const ::windows_core::GUID, pasynchandle: usize, dwstatus: u32) -> u32 {
    ::windows_targets::link!("userenv.dll" "system" fn ProcessGroupPolicyCompleted(extensionid : *const ::windows_core::GUID, pasynchandle : usize, dwstatus : u32) -> u32);
    ProcessGroupPolicyCompleted(extensionid, pasynchandle, dwstatus)
}
#[inline]
pub unsafe fn ProcessGroupPolicyCompletedEx(extensionid: *const ::windows_core::GUID, pasynchandle: usize, dwstatus: u32, rsopstatus: ::windows_core::HRESULT) -> u32 {
    ::windows_targets::link!("userenv.dll" "system" fn ProcessGroupPolicyCompletedEx(extensionid : *const ::windows_core::GUID, pasynchandle : usize, dwstatus : u32, rsopstatus : ::windows_core::HRESULT) -> u32);
    ProcessGroupPolicyCompletedEx(extensionid, pasynchandle, dwstatus, rsopstatus)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RefreshPolicy<P0>(bmachine: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("userenv.dll" "system" fn RefreshPolicy(bmachine : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    RefreshPolicy(bmachine.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RefreshPolicyEx<P0>(bmachine: P0, dwoptions: u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("userenv.dll" "system" fn RefreshPolicyEx(bmachine : super::super::Foundation:: BOOL, dwoptions : u32) -> super::super::Foundation:: BOOL);
    RefreshPolicyEx(bmachine.into_param().abi(), dwoptions).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterGPNotification<P0, P1>(hevent: P0, bmachine: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link!("userenv.dll" "system" fn RegisterGPNotification(hevent : super::super::Foundation:: HANDLE, bmachine : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    RegisterGPNotification(hevent.into_param().abi(), bmachine.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn RsopAccessCheckByType<P0, P1>(psecuritydescriptor: P0, pprincipalselfsid: P1, prsoptoken: *const ::core::ffi::c_void, dwdesiredaccessmask: u32, pobjecttypelist: ::core::option::Option<&[super::super::Security::OBJECT_TYPE_LIST]>, pgenericmapping: *const super::super::Security::GENERIC_MAPPING, pprivilegeset: ::core::option::Option<*const super::super::Security::PRIVILEGE_SET>, pdwprivilegesetlength: ::core::option::Option<*const u32>, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Security::PSECURITY_DESCRIPTOR>,
    P1: ::windows_core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows_targets::link!("userenv.dll" "system" fn RsopAccessCheckByType(psecuritydescriptor : super::super::Security:: PSECURITY_DESCRIPTOR, pprincipalselfsid : super::super::Foundation:: PSID, prsoptoken : *const ::core::ffi::c_void, dwdesiredaccessmask : u32, pobjecttypelist : *const super::super::Security:: OBJECT_TYPE_LIST, objecttypelistlength : u32, pgenericmapping : *const super::super::Security:: GENERIC_MAPPING, pprivilegeset : *const super::super::Security:: PRIVILEGE_SET, pdwprivilegesetlength : *const u32, pdwgrantedaccessmask : *mut u32, pbaccessstatus : *mut super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    RsopAccessCheckByType(
        psecuritydescriptor.into_param().abi(),
        pprincipalselfsid.into_param().abi(),
        prsoptoken,
        dwdesiredaccessmask,
        ::core::mem::transmute(pobjecttypelist.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
        pobjecttypelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        pgenericmapping,
        ::core::mem::transmute(pprivilegeset.unwrap_or(::std::ptr::null())),
        ::core::mem::transmute(pdwprivilegesetlength.unwrap_or(::std::ptr::null())),
        pdwgrantedaccessmask,
        pbaccessstatus,
    )
    .ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RsopFileAccessCheck<P0>(pszfilename: P0, prsoptoken: *const ::core::ffi::c_void, dwdesiredaccessmask: u32, pdwgrantedaccessmask: *mut u32, pbaccessstatus: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("userenv.dll" "system" fn RsopFileAccessCheck(pszfilename : ::windows_core::PCWSTR, prsoptoken : *const ::core::ffi::c_void, dwdesiredaccessmask : u32, pdwgrantedaccessmask : *mut u32, pbaccessstatus : *mut super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    RsopFileAccessCheck(pszfilename.into_param().abi(), prsoptoken, dwdesiredaccessmask, pdwgrantedaccessmask, pbaccessstatus).ok()
}
#[doc = "Required features: `\"Win32_System_Wmi\"`"]
#[cfg(feature = "Win32_System_Wmi")]
#[inline]
pub unsafe fn RsopResetPolicySettingStatus<P0, P1>(dwflags: u32, pservices: P0, psettinginstance: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::Wmi::IWbemServices>,
    P1: ::windows_core::IntoParam<super::Wmi::IWbemClassObject>,
{
    ::windows_targets::link!("userenv.dll" "system" fn RsopResetPolicySettingStatus(dwflags : u32, pservices : * mut::core::ffi::c_void, psettinginstance : * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    RsopResetPolicySettingStatus(dwflags, pservices.into_param().abi(), psettinginstance.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Wmi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Wmi"))]
#[inline]
pub unsafe fn RsopSetPolicySettingStatus<P0, P1>(dwflags: u32, pservices: P0, psettinginstance: P1, pstatus: &[POLICYSETTINGSTATUSINFO]) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::Wmi::IWbemServices>,
    P1: ::windows_core::IntoParam<super::Wmi::IWbemClassObject>,
{
    ::windows_targets::link!("userenv.dll" "system" fn RsopSetPolicySettingStatus(dwflags : u32, pservices : * mut::core::ffi::c_void, psettinginstance : * mut::core::ffi::c_void, ninfo : u32, pstatus : *const POLICYSETTINGSTATUSINFO) -> ::windows_core::HRESULT);
    RsopSetPolicySettingStatus(dwflags, pservices.into_param().abi(), psettinginstance.into_param().abi(), pstatus.len().try_into().unwrap(), ::core::mem::transmute(pstatus.as_ptr())).ok()
}
#[inline]
pub unsafe fn UninstallApplication<P0>(productcode: P0, dwstatus: u32) -> u32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("advapi32.dll" "system" fn UninstallApplication(productcode : ::windows_core::PCWSTR, dwstatus : u32) -> u32);
    UninstallApplication(productcode.into_param().abi(), dwstatus)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterGPNotification<P0>(hevent: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link!("userenv.dll" "system" fn UnregisterGPNotification(hevent : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    UnregisterGPNotification(hevent.into_param().abi()).ok()
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPEInformation(::windows_core::IUnknown);
impl IGPEInformation {
    pub unsafe fn GetName(&self, pszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDisplayName(&self, pszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Registry\"`"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn GetRegistryKey(&self, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRegistryKey)(::windows_core::Interface::as_raw(self), dwsection, hkey).ok()
    }
    pub unsafe fn GetDSPath(&self, dwsection: u32, pszpath: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDSPath)(::windows_core::Interface::as_raw(self), dwsection, ::core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetFileSysPath(&self, dwsection: u32, pszpath: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFileSysPath)(::windows_core::Interface::as_raw(self), dwsection, ::core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetOptions(&self, dwoptions: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOptions)(::windows_core::Interface::as_raw(self), dwoptions).ok()
    }
    pub unsafe fn GetType(&self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetType)(::windows_core::Interface::as_raw(self), gpotype).ok()
    }
    pub unsafe fn GetHint(&self, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetHint)(::windows_core::Interface::as_raw(self), gphint).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PolicyChanged<P0, P1>(&self, bmachine: P0, badd: P1, pguidextension: *mut ::windows_core::GUID, pguidsnapin: *mut ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).PolicyChanged)(::windows_core::Interface::as_raw(self), bmachine.into_param().abi(), badd.into_param().abi(), pguidextension, pguidsnapin).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IGPEInformation, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGPEInformation {
    type Vtable = IGPEInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGPEInformation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fc0b735_a0e1_11d1_a7d3_0000f87571e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGPEInformation_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Registry")]
    pub GetRegistryKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, hkey: *mut super::Registry::HKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    GetRegistryKey: usize,
    pub GetDSPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows_core::PWSTR, cchmaxpath: i32) -> ::windows_core::HRESULT,
    pub GetFileSysPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows_core::PWSTR, cchmaxpath: i32) -> ::windows_core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoptions: *mut u32) -> ::windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows_core::HRESULT,
    pub GetHint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gphint: *mut GROUP_POLICY_HINT_TYPE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PolicyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows_core::GUID, pguidsnapin: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PolicyChanged: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPM(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPM {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDomain<P0, P1>(&self, bstrdomain: P0, bstrdomaincontroller: P1, ldcflags: i32) -> ::windows_core::Result<IGPMDomain>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDomain)(::windows_core::Interface::as_raw(self), bstrdomain.into_param().abi(), bstrdomaincontroller.into_param().abi(), ldcflags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBackupDir<P0>(&self, bstrbackupdir: P0) -> ::windows_core::Result<IGPMBackupDir>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBackupDir)(::windows_core::Interface::as_raw(self), bstrbackupdir.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSitesContainer<P0, P1, P2>(&self, bstrforest: P0, bstrdomain: P1, bstrdomaincontroller: P2, ldcflags: i32) -> ::windows_core::Result<IGPMSitesContainer>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSitesContainer)(::windows_core::Interface::as_raw(self), bstrforest.into_param().abi(), bstrdomain.into_param().abi(), bstrdomaincontroller.into_param().abi(), ldcflags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRSOP<P0>(&self, gpmrsopmode: GPMRSOPMode, bstrnamespace: P0, lflags: i32) -> ::windows_core::Result<IGPMRSOP>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRSOP)(::windows_core::Interface::as_raw(self), gpmrsopmode, bstrnamespace.into_param().abi(), lflags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePermission<P0, P1>(&self, bstrtrustee: P0, perm: GPMPermissionType, binheritable: P1) -> ::windows_core::Result<IGPMPermission>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePermission)(::windows_core::Interface::as_raw(self), bstrtrustee.into_param().abi(), perm, binheritable.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSearchCriteria(&self) -> ::windows_core::Result<IGPMSearchCriteria> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSearchCriteria)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTrustee<P0>(&self, bstrtrustee: P0) -> ::windows_core::Result<IGPMTrustee>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateTrustee)(::windows_core::Interface::as_raw(self), bstrtrustee.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClientSideExtensions(&self) -> ::windows_core::Result<IGPMCSECollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetClientSideExtensions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConstants(&self) -> ::windows_core::Result<IGPMConstants> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetConstants)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMigrationTable<P0>(&self, bstrmigrationtablepath: P0) -> ::windows_core::Result<IGPMMigrationTable>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMigrationTable)(::windows_core::Interface::as_raw(self), bstrmigrationtablepath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateMigrationTable(&self) -> ::windows_core::Result<IGPMMigrationTable> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateMigrationTable)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InitializeReporting<P0>(&self, bstradmpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).InitializeReporting)(::windows_core::Interface::as_raw(self), bstradmpath.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPM, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPM {
    type Vtable = IGPM_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPM {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5fae809_3bd6_4da9_a65e_17665b41d763);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPM_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdomain: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdomaincontroller: ::std::mem::MaybeUninit<::windows_core::BSTR>, ldcflags: i32, pigpmdomain: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDomain: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBackupDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbackupdir: ::std::mem::MaybeUninit<::windows_core::BSTR>, pigpmbackupdir: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBackupDir: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSitesContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrforest: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdomain: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdomaincontroller: ::std::mem::MaybeUninit<::windows_core::BSTR>, ldcflags: i32, ppigpmsitescontainer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSitesContainer: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRSOP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmrsopmode: GPMRSOPMode, bstrnamespace: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: i32, ppigpmrsop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRSOP: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtrustee: ::std::mem::MaybeUninit<::windows_core::BSTR>, perm: GPMPermissionType, binheritable: super::super::Foundation::VARIANT_BOOL, ppperm: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePermission: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSearchCriteria: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmsearchcriteria: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSearchCriteria: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateTrustee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtrustee: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppigpmtrustee: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateTrustee: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetClientSideExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmcsecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetClientSideExtensions: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetConstants: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmconstants: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetConstants: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetMigrationTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmigrationtablepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppmigrationtable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetMigrationTable: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateMigrationTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmigrationtable: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateMigrationTable: usize,
    pub InitializeReporting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPM2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPM2 {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDomain<P0, P1>(&self, bstrdomain: P0, bstrdomaincontroller: P1, ldcflags: i32) -> ::windows_core::Result<IGPMDomain>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDomain)(::windows_core::Interface::as_raw(self), bstrdomain.into_param().abi(), bstrdomaincontroller.into_param().abi(), ldcflags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBackupDir<P0>(&self, bstrbackupdir: P0) -> ::windows_core::Result<IGPMBackupDir>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetBackupDir)(::windows_core::Interface::as_raw(self), bstrbackupdir.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSitesContainer<P0, P1, P2>(&self, bstrforest: P0, bstrdomain: P1, bstrdomaincontroller: P2, ldcflags: i32) -> ::windows_core::Result<IGPMSitesContainer>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
        P2: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSitesContainer)(::windows_core::Interface::as_raw(self), bstrforest.into_param().abi(), bstrdomain.into_param().abi(), bstrdomaincontroller.into_param().abi(), ldcflags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRSOP<P0>(&self, gpmrsopmode: GPMRSOPMode, bstrnamespace: P0, lflags: i32) -> ::windows_core::Result<IGPMRSOP>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRSOP)(::windows_core::Interface::as_raw(self), gpmrsopmode, bstrnamespace.into_param().abi(), lflags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePermission<P0, P1>(&self, bstrtrustee: P0, perm: GPMPermissionType, binheritable: P1) -> ::windows_core::Result<IGPMPermission>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePermission)(::windows_core::Interface::as_raw(self), bstrtrustee.into_param().abi(), perm, binheritable.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSearchCriteria(&self) -> ::windows_core::Result<IGPMSearchCriteria> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateSearchCriteria)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateTrustee<P0>(&self, bstrtrustee: P0) -> ::windows_core::Result<IGPMTrustee>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateTrustee)(::windows_core::Interface::as_raw(self), bstrtrustee.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClientSideExtensions(&self) -> ::windows_core::Result<IGPMCSECollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClientSideExtensions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetConstants(&self) -> ::windows_core::Result<IGPMConstants> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetConstants)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetMigrationTable<P0>(&self, bstrmigrationtablepath: P0) -> ::windows_core::Result<IGPMMigrationTable>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetMigrationTable)(::windows_core::Interface::as_raw(self), bstrmigrationtablepath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateMigrationTable(&self) -> ::windows_core::Result<IGPMMigrationTable> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateMigrationTable)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InitializeReporting<P0>(&self, bstradmpath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.InitializeReporting)(::windows_core::Interface::as_raw(self), bstradmpath.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBackupDirEx<P0>(&self, bstrbackupdir: P0, backupdirtype: GPMBackupType) -> ::windows_core::Result<IGPMBackupDirEx>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBackupDirEx)(::windows_core::Interface::as_raw(self), bstrbackupdir.into_param().abi(), backupdirtype, &mut result__).from_abi(result__)
    }
    pub unsafe fn InitializeReportingEx<P0>(&self, bstradmpath: P0, reportingoptions: i32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).InitializeReportingEx)(::windows_core::Interface::as_raw(self), bstradmpath.into_param().abi(), reportingoptions).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPM2, ::windows_core::IUnknown, super::Com::IDispatch, IGPM);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPM2 {
    type Vtable = IGPM2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPM2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x00238f8a_3d86_41ac_8f5e_06a6638a634a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPM2_Vtbl {
    pub base__: IGPM_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBackupDirEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbackupdir: ::std::mem::MaybeUninit<::windows_core::BSTR>, backupdirtype: GPMBackupType, ppigpmbackupdirex: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBackupDirEx: usize,
    pub InitializeReportingEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstradmpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, reportingoptions: i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMAsyncCancel(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMAsyncCancel {
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMAsyncCancel, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMAsyncCancel {
    type Vtable = IGPMAsyncCancel_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMAsyncCancel {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xddc67754_be67_4541_8166_f48166868c9c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMAsyncCancel_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMAsyncProgress(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMAsyncProgress {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Status<P0>(&self, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows_core::HRESULT, presult: *const super::Variant::VARIANT, ppigpmstatusmsgcollection: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMStatusMsgCollection>,
    {
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), lprogressnumerator, lprogressdenominator, hrstatus, presult, ppigpmstatusmsgcollection.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMAsyncProgress, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMAsyncProgress {
    type Vtable = IGPMAsyncProgress_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMAsyncProgress {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6aac29f8_5948_4324_bf70_423818942dbc);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMAsyncProgress_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lprogressnumerator: i32, lprogressdenominator: i32, hrstatus: ::windows_core::HRESULT, presult: *const super::Variant::VARIANT, ppigpmstatusmsgcollection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Status: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMBackup(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMBackup {
    pub unsafe fn ID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GPOID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GPOID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GPODomain(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GPODomain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GPODisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GPODisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Timestamp(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Timestamp)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Comment(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Comment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BackupDir(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BackupDir)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GenerateReport)(::windows_core::Interface::as_raw(self), gpmreporttype, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GenerateReportToFile<P0>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: P0) -> ::windows_core::Result<IGPMResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GenerateReportToFile)(::windows_core::Interface::as_raw(self), gpmreporttype, bstrtargetfilepath.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMBackup, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMBackup {
    type Vtable = IGPMBackup_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMBackup {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd8a16a35_3b0d_416b_8d02_4df6f95a7119);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackup_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GPOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GPODomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GPODisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub BackupDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GenerateReport: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GenerateReportToFile: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMBackupCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMBackupCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMBackupCollection, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMBackupCollection {
    type Vtable = IGPMBackupCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMBackupCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc786fc0f_26d8_4bab_a745_39ca7e800cac);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackupCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmbackup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMBackupDir(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMBackupDir {
    pub unsafe fn BackupDirectory(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BackupDirectory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBackup<P0>(&self, bstrid: P0) -> ::windows_core::Result<IGPMBackup>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBackup)(::windows_core::Interface::as_raw(self), bstrid.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchBackups<P0>(&self, pigpmsearchcriteria: P0) -> ::windows_core::Result<IGPMBackupCollection>
    where
        P0: ::windows_core::IntoParam<IGPMSearchCriteria>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchBackups)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMBackupDir, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMBackupDir {
    type Vtable = IGPMBackupDir_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMBackupDir {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb1568bed_0a93_4acc_810f_afe7081019b9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackupDir_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub BackupDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBackup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppbackup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBackup: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchBackups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmbackupcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchBackups: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMBackupDirEx(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMBackupDirEx {
    pub unsafe fn BackupDir(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BackupDir)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BackupType(&self) -> ::windows_core::Result<GPMBackupType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BackupType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetBackup<P0>(&self, bstrid: P0) -> ::windows_core::Result<super::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBackup)(::windows_core::Interface::as_raw(self), bstrid.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SearchBackups<P0>(&self, pigpmsearchcriteria: P0) -> ::windows_core::Result<super::Variant::VARIANT>
    where
        P0: ::windows_core::IntoParam<IGPMSearchCriteria>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchBackups)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMBackupDirEx, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMBackupDirEx {
    type Vtable = IGPMBackupDirEx_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMBackupDirEx {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf8dc55ed_3ba0_4864_aad4_d365189ee1d5);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMBackupDirEx_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub BackupDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbackupdir: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub BackupType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgpmbackuptype: *mut GPMBackupType) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetBackup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarbackup: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetBackup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SearchBackups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, pvarbackupcollection: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SearchBackups: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMCSECollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMCSECollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMCSECollection, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMCSECollection {
    type Vtable = IGPMCSECollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMCSECollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e52a97d_0a4a_4a6f_85db_201622455da0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMCSECollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmcses: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMClientSideExtension(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMClientSideExtension {
    pub unsafe fn ID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUserEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsUserEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsComputerEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsComputerEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMClientSideExtension, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMClientSideExtension {
    type Vtable = IGPMClientSideExtension_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMClientSideExtension {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69da7488_b8db_415e_9266_901be4d49928);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMClientSideExtension_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUserEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUserEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsComputerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsComputerEnabled: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMConstants(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMConstants {
    pub unsafe fn PermGPOApply(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermGPOApply)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermGPORead(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermGPORead)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermGPOEdit(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermGPOEdit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermGPOEditSecurityAndDelete(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermGPOEditSecurityAndDelete)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermGPOCustom(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermGPOCustom)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermWMIFilterEdit(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermWMIFilterEdit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermWMIFilterFullControl(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermWMIFilterFullControl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermWMIFilterCustom(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermWMIFilterCustom)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermSOMLink(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermSOMLink)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermSOMLogging(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermSOMLogging)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermSOMPlanning(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermSOMPlanning)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermSOMGPOCreate(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermSOMGPOCreate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermSOMWMICreate(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermSOMWMICreate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermSOMWMIFullControl(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermSOMWMIFullControl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyGPOPermissions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyGPOPermissions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyGPOEffectivePermissions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyGPOEffectivePermissions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyGPODisplayName(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyGPODisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyGPOWMIFilter(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyGPOWMIFilter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyGPOID(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyGPOID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyGPOComputerExtensions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyGPOComputerExtensions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyGPOUserExtensions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyGPOUserExtensions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertySOMLinks(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertySOMLinks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyGPODomain(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyGPODomain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyBackupMostRecent(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyBackupMostRecent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchOpEquals(&self) -> ::windows_core::Result<GPMSearchOperation> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchOpEquals)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchOpContains(&self) -> ::windows_core::Result<GPMSearchOperation> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchOpContains)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchOpNotContains(&self) -> ::windows_core::Result<GPMSearchOperation> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchOpNotContains)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchOpNotEquals(&self) -> ::windows_core::Result<GPMSearchOperation> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchOpNotEquals)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UsePDC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UsePDC)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UseAnyDC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UseAnyDC)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DoNotUseW2KDC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DoNotUseW2KDC)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SOMSite(&self) -> ::windows_core::Result<GPMSOMType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SOMSite)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SOMDomain(&self) -> ::windows_core::Result<GPMSOMType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SOMDomain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SOMOU(&self) -> ::windows_core::Result<GPMSOMType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SOMOU)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_SecurityFlags<P0, P1, P2, P3>(&self, vbowner: P0, vbgroup: P1, vbdacl: P2, vbsacl: P3) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_SecurityFlags)(::windows_core::Interface::as_raw(self), vbowner.into_param().abi(), vbgroup.into_param().abi(), vbdacl.into_param().abi(), vbsacl.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DoNotValidateDC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DoNotValidateDC)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReportHTML(&self) -> ::windows_core::Result<GPMReportType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReportHTML)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReportXML(&self) -> ::windows_core::Result<GPMReportType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReportXML)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RSOPModeUnknown(&self) -> ::windows_core::Result<GPMRSOPMode> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RSOPModeUnknown)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RSOPModePlanning(&self) -> ::windows_core::Result<GPMRSOPMode> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RSOPModePlanning)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RSOPModeLogging(&self) -> ::windows_core::Result<GPMRSOPMode> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RSOPModeLogging)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EntryTypeUser(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EntryTypeUser)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EntryTypeComputer(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EntryTypeComputer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EntryTypeLocalGroup(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EntryTypeLocalGroup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EntryTypeGlobalGroup(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EntryTypeGlobalGroup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EntryTypeUniversalGroup(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EntryTypeUniversalGroup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EntryTypeUNCPath(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EntryTypeUNCPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EntryTypeUnknown(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EntryTypeUnknown)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DestinationOptionSameAsSource(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DestinationOptionSameAsSource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DestinationOptionNone(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DestinationOptionNone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DestinationOptionByRelativeName(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DestinationOptionByRelativeName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DestinationOptionSet(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DestinationOptionSet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MigrationTableOnly(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).MigrationTableOnly)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProcessSecurity(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ProcessSecurity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RsopLoggingNoComputer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RsopLoggingNoComputer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RsopLoggingNoUser(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RsopLoggingNoUser)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RsopPlanningAssumeSlowLink(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RsopPlanningAssumeSlowLink)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_RsopPlanningLoopbackOption<P0>(&self, vbmerge: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_RsopPlanningLoopbackOption)(::windows_core::Interface::as_raw(self), vbmerge.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RsopPlanningAssumeUserWQLFilterTrue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RsopPlanningAssumeUserWQLFilterTrue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RsopPlanningAssumeCompWQLFilterTrue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).RsopPlanningAssumeCompWQLFilterTrue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMConstants, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMConstants {
    type Vtable = IGPMConstants_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMConstants {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50ef73e6_d35c_4c8d_be63_7ea5d2aac5c4);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMConstants_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub PermGPOApply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermGPORead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermGPOEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermGPOEditSecurityAndDelete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermGPOCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermWMIFilterEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermWMIFilterFullControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermWMIFilterCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermSOMLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermSOMLogging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermSOMPlanning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermSOMGPOCreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermSOMWMICreate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermSOMWMIFullControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub SearchPropertyGPOPermissions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyGPOEffectivePermissions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyGPODisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyGPOWMIFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyGPOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyGPOComputerExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyGPOUserExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertySOMLinks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyGPODomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyBackupMostRecent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchOpEquals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows_core::HRESULT,
    pub SearchOpContains: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows_core::HRESULT,
    pub SearchOpNotContains: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows_core::HRESULT,
    pub SearchOpNotEquals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchOperation) -> ::windows_core::HRESULT,
    pub UsePDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub UseAnyDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub DoNotUseW2KDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub SOMSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows_core::HRESULT,
    pub SOMDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows_core::HRESULT,
    pub SOMOU: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_SecurityFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbowner: super::super::Foundation::VARIANT_BOOL, vbgroup: super::super::Foundation::VARIANT_BOOL, vbdacl: super::super::Foundation::VARIANT_BOOL, vbsacl: super::super::Foundation::VARIANT_BOOL, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_SecurityFlags: usize,
    pub DoNotValidateDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub ReportHTML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMReportType) -> ::windows_core::HRESULT,
    pub ReportXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMReportType) -> ::windows_core::HRESULT,
    pub RSOPModeUnknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows_core::HRESULT,
    pub RSOPModePlanning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows_core::HRESULT,
    pub RSOPModeLogging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows_core::HRESULT,
    pub EntryTypeUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT,
    pub EntryTypeComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT,
    pub EntryTypeLocalGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT,
    pub EntryTypeGlobalGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT,
    pub EntryTypeUniversalGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT,
    pub EntryTypeUNCPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT,
    pub EntryTypeUnknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMEntryType) -> ::windows_core::HRESULT,
    pub DestinationOptionSameAsSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows_core::HRESULT,
    pub DestinationOptionNone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows_core::HRESULT,
    pub DestinationOptionByRelativeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows_core::HRESULT,
    pub DestinationOptionSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMDestinationOption) -> ::windows_core::HRESULT,
    pub MigrationTableOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub ProcessSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub RsopLoggingNoComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub RsopLoggingNoUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub RsopPlanningAssumeSlowLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub get_RsopPlanningLoopbackOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbmerge: super::super::Foundation::VARIANT_BOOL, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    get_RsopPlanningLoopbackOption: usize,
    pub RsopPlanningAssumeUserWQLFilterTrue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub RsopPlanningAssumeCompWQLFilterTrue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMConstants2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMConstants2 {
    pub unsafe fn PermGPOApply(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermGPOApply)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermGPORead(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermGPORead)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermGPOEdit(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermGPOEdit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermGPOEditSecurityAndDelete(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermGPOEditSecurityAndDelete)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermGPOCustom(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermGPOCustom)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermWMIFilterEdit(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermWMIFilterEdit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermWMIFilterFullControl(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermWMIFilterFullControl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermWMIFilterCustom(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermWMIFilterCustom)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermSOMLink(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermSOMLink)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermSOMLogging(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermSOMLogging)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermSOMPlanning(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermSOMPlanning)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermSOMGPOCreate(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermSOMGPOCreate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermSOMWMICreate(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermSOMWMICreate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermSOMWMIFullControl(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.PermSOMWMIFullControl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyGPOPermissions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyGPOPermissions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyGPOEffectivePermissions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyGPOEffectivePermissions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyGPODisplayName(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyGPODisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyGPOWMIFilter(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyGPOWMIFilter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyGPOID(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyGPOID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyGPOComputerExtensions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyGPOComputerExtensions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyGPOUserExtensions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyGPOUserExtensions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertySOMLinks(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertySOMLinks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyGPODomain(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyGPODomain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyBackupMostRecent(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchPropertyBackupMostRecent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchOpEquals(&self) -> ::windows_core::Result<GPMSearchOperation> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchOpEquals)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchOpContains(&self) -> ::windows_core::Result<GPMSearchOperation> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchOpContains)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchOpNotContains(&self) -> ::windows_core::Result<GPMSearchOperation> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchOpNotContains)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchOpNotEquals(&self) -> ::windows_core::Result<GPMSearchOperation> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchOpNotEquals)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UsePDC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.UsePDC)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UseAnyDC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.UseAnyDC)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DoNotUseW2KDC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DoNotUseW2KDC)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SOMSite(&self) -> ::windows_core::Result<GPMSOMType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SOMSite)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SOMDomain(&self) -> ::windows_core::Result<GPMSOMType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SOMDomain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SOMOU(&self) -> ::windows_core::Result<GPMSOMType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SOMOU)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_SecurityFlags<P0, P1, P2, P3>(&self, vbowner: P0, vbgroup: P1, vbdacl: P2, vbsacl: P3) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P3: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_SecurityFlags)(::windows_core::Interface::as_raw(self), vbowner.into_param().abi(), vbgroup.into_param().abi(), vbdacl.into_param().abi(), vbsacl.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DoNotValidateDC(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DoNotValidateDC)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReportHTML(&self) -> ::windows_core::Result<GPMReportType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ReportHTML)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReportXML(&self) -> ::windows_core::Result<GPMReportType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ReportXML)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RSOPModeUnknown(&self) -> ::windows_core::Result<GPMRSOPMode> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RSOPModeUnknown)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RSOPModePlanning(&self) -> ::windows_core::Result<GPMRSOPMode> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RSOPModePlanning)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RSOPModeLogging(&self) -> ::windows_core::Result<GPMRSOPMode> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RSOPModeLogging)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EntryTypeUser(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EntryTypeUser)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EntryTypeComputer(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EntryTypeComputer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EntryTypeLocalGroup(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EntryTypeLocalGroup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EntryTypeGlobalGroup(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EntryTypeGlobalGroup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EntryTypeUniversalGroup(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EntryTypeUniversalGroup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EntryTypeUNCPath(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EntryTypeUNCPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EntryTypeUnknown(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.EntryTypeUnknown)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DestinationOptionSameAsSource(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DestinationOptionSameAsSource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DestinationOptionNone(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DestinationOptionNone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DestinationOptionByRelativeName(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DestinationOptionByRelativeName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DestinationOptionSet(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DestinationOptionSet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MigrationTableOnly(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.MigrationTableOnly)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ProcessSecurity(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ProcessSecurity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RsopLoggingNoComputer(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RsopLoggingNoComputer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RsopLoggingNoUser(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RsopLoggingNoUser)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RsopPlanningAssumeSlowLink(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RsopPlanningAssumeSlowLink)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn get_RsopPlanningLoopbackOption<P0>(&self, vbmerge: P0) -> ::windows_core::Result<i32>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.get_RsopPlanningLoopbackOption)(::windows_core::Interface::as_raw(self), vbmerge.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RsopPlanningAssumeUserWQLFilterTrue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RsopPlanningAssumeUserWQLFilterTrue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RsopPlanningAssumeCompWQLFilterTrue(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.RsopPlanningAssumeCompWQLFilterTrue)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BackupTypeGPO(&self) -> ::windows_core::Result<GPMBackupType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BackupTypeGPO)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn BackupTypeStarterGPO(&self) -> ::windows_core::Result<GPMBackupType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BackupTypeStarterGPO)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StarterGPOTypeSystem(&self) -> ::windows_core::Result<GPMStarterGPOType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StarterGPOTypeSystem)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StarterGPOTypeCustom(&self) -> ::windows_core::Result<GPMStarterGPOType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StarterGPOTypeCustom)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyStarterGPOPermissions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyStarterGPOPermissions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyStarterGPOEffectivePermissions(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyStarterGPOEffectivePermissions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyStarterGPODisplayName(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyStarterGPODisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyStarterGPOID(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyStarterGPOID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SearchPropertyStarterGPODomain(&self) -> ::windows_core::Result<GPMSearchProperty> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchPropertyStarterGPODomain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermStarterGPORead(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermStarterGPORead)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermStarterGPOEdit(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermStarterGPOEdit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermStarterGPOFullControl(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermStarterGPOFullControl)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn PermStarterGPOCustom(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PermStarterGPOCustom)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReportLegacy(&self) -> ::windows_core::Result<GPMReportingOptions> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReportLegacy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ReportComments(&self) -> ::windows_core::Result<GPMReportingOptions> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ReportComments)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMConstants2, ::windows_core::IUnknown, super::Com::IDispatch, IGPMConstants);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMConstants2 {
    type Vtable = IGPMConstants2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMConstants2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x05ae21b0_ac09_4032_a26f_9e7da786dc19);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMConstants2_Vtbl {
    pub base__: IGPMConstants_Vtbl,
    pub BackupTypeGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMBackupType) -> ::windows_core::HRESULT,
    pub BackupTypeStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMBackupType) -> ::windows_core::HRESULT,
    pub StarterGPOTypeSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows_core::HRESULT,
    pub StarterGPOTypeCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows_core::HRESULT,
    pub SearchPropertyStarterGPOPermissions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyStarterGPOEffectivePermissions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyStarterGPODisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyStarterGPOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub SearchPropertyStarterGPODomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSearchProperty) -> ::windows_core::HRESULT,
    pub PermStarterGPORead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermStarterGPOEdit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermStarterGPOFullControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub PermStarterGPOCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    pub ReportLegacy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMReportingOptions) -> ::windows_core::HRESULT,
    pub ReportComments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMReportingOptions) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMDomain(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMDomain {
    pub unsafe fn DomainController(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DomainController)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Domain(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Domain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGPO(&self) -> ::windows_core::Result<IGPMGPO> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateGPO)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetGPO<P0>(&self, bstrguid: P0) -> ::windows_core::Result<IGPMGPO>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGPO)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchGPOs<P0>(&self, pigpmsearchcriteria: P0) -> ::windows_core::Result<IGPMGPOCollection>
    where
        P0: ::windows_core::IntoParam<IGPMSearchCriteria>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchGPOs)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RestoreGPO<P0>(&self, pigpmbackup: P0, ldcflags: i32, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMBackup>,
    {
        (::windows_core::Interface::vtable(self).RestoreGPO)(::windows_core::Interface::as_raw(self), pigpmbackup.into_param().abi(), ldcflags, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSOM<P0>(&self, bstrpath: P0) -> ::windows_core::Result<IGPMSOM>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSOM)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchSOMs<P0>(&self, pigpmsearchcriteria: P0) -> ::windows_core::Result<IGPMSOMCollection>
    where
        P0: ::windows_core::IntoParam<IGPMSearchCriteria>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchSOMs)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWMIFilter<P0>(&self, bstrpath: P0) -> ::windows_core::Result<IGPMWMIFilter>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetWMIFilter)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchWMIFilters<P0>(&self, pigpmsearchcriteria: P0) -> ::windows_core::Result<IGPMWMIFilterCollection>
    where
        P0: ::windows_core::IntoParam<IGPMSearchCriteria>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchWMIFilters)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMDomain, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMDomain {
    type Vtable = IGPMDomain_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMDomain {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6b21cc14_5a00_4f44_a738_feec8a94c7e3);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMDomain_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DomainController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnewgpo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateGPO: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppgpo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetGPO: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchGPOs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmgpocollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchGPOs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RestoreGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmbackup: *mut ::core::ffi::c_void, ldcflags: i32, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RestoreGPO: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsom: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSOM: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchSOMs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmsomcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchSOMs: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWMIFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppwmifilter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWMIFilter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchWMIFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmwmifiltercollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchWMIFilters: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMDomain2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMDomain2 {
    pub unsafe fn DomainController(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DomainController)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Domain(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Domain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGPO(&self) -> ::windows_core::Result<IGPMGPO> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateGPO)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetGPO<P0>(&self, bstrguid: P0) -> ::windows_core::Result<IGPMGPO>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetGPO)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchGPOs<P0>(&self, pigpmsearchcriteria: P0) -> ::windows_core::Result<IGPMGPOCollection>
    where
        P0: ::windows_core::IntoParam<IGPMSearchCriteria>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchGPOs)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RestoreGPO<P0>(&self, pigpmbackup: P0, ldcflags: i32, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMBackup>,
    {
        (::windows_core::Interface::vtable(self).base__.RestoreGPO)(::windows_core::Interface::as_raw(self), pigpmbackup.into_param().abi(), ldcflags, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSOM<P0>(&self, bstrpath: P0) -> ::windows_core::Result<IGPMSOM>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSOM)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchSOMs<P0>(&self, pigpmsearchcriteria: P0) -> ::windows_core::Result<IGPMSOMCollection>
    where
        P0: ::windows_core::IntoParam<IGPMSearchCriteria>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchSOMs)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWMIFilter<P0>(&self, bstrpath: P0) -> ::windows_core::Result<IGPMWMIFilter>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetWMIFilter)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchWMIFilters<P0>(&self, pigpmsearchcriteria: P0) -> ::windows_core::Result<IGPMWMIFilterCollection>
    where
        P0: ::windows_core::IntoParam<IGPMSearchCriteria>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchWMIFilters)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStarterGPO(&self) -> ::windows_core::Result<IGPMStarterGPO> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateStarterGPO)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGPOFromStarterGPO<P0>(&self, pgpotemplate: P0) -> ::windows_core::Result<IGPMGPO>
    where
        P0: ::windows_core::IntoParam<IGPMStarterGPO>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateGPOFromStarterGPO)(::windows_core::Interface::as_raw(self), pgpotemplate.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStarterGPO<P0>(&self, bstrguid: P0) -> ::windows_core::Result<IGPMStarterGPO>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStarterGPO)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchStarterGPOs<P0>(&self, pigpmsearchcriteria: P0) -> ::windows_core::Result<IGPMStarterGPOCollection>
    where
        P0: ::windows_core::IntoParam<IGPMSearchCriteria>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchStarterGPOs)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LoadStarterGPO<P0, P1>(&self, bstrloadfile: P0, boverwrite: P1, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).LoadStarterGPO)(::windows_core::Interface::as_raw(self), bstrloadfile.into_param().abi(), boverwrite.into_param().abi(), pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RestoreStarterGPO<P0>(&self, pigpmtmplbackup: P0, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMStarterGPOBackup>,
    {
        (::windows_core::Interface::vtable(self).RestoreStarterGPO)(::windows_core::Interface::as_raw(self), pigpmtmplbackup.into_param().abi(), pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMDomain2, ::windows_core::IUnknown, super::Com::IDispatch, IGPMDomain);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMDomain2 {
    type Vtable = IGPMDomain2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMDomain2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7ca6bb8b_f1eb_490a_938d_3c4e51c768e6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMDomain2_Vtbl {
    pub base__: IGPMDomain_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnewtemplate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStarterGPO: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateGPOFromStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgpotemplate: *mut ::core::ffi::c_void, ppnewgpo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateGPOFromStarterGPO: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptemplate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStarterGPO: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchStarterGPOs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmtemplatecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchStarterGPOs: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LoadStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrloadfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, boverwrite: super::super::Foundation::VARIANT_BOOL, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LoadStarterGPO: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub RestoreStarterGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmtmplbackup: *mut ::core::ffi::c_void, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    RestoreStarterGPO: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMDomain3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMDomain3 {
    pub unsafe fn DomainController(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DomainController)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Domain(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Domain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGPO(&self) -> ::windows_core::Result<IGPMGPO> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreateGPO)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetGPO<P0>(&self, bstrguid: P0) -> ::windows_core::Result<IGPMGPO>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetGPO)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchGPOs<P0>(&self, pigpmsearchcriteria: P0) -> ::windows_core::Result<IGPMGPOCollection>
    where
        P0: ::windows_core::IntoParam<IGPMSearchCriteria>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SearchGPOs)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RestoreGPO<P0>(&self, pigpmbackup: P0, ldcflags: i32, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMBackup>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.RestoreGPO)(::windows_core::Interface::as_raw(self), pigpmbackup.into_param().abi(), ldcflags, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSOM<P0>(&self, bstrpath: P0) -> ::windows_core::Result<IGPMSOM>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSOM)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchSOMs<P0>(&self, pigpmsearchcriteria: P0) -> ::windows_core::Result<IGPMSOMCollection>
    where
        P0: ::windows_core::IntoParam<IGPMSearchCriteria>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SearchSOMs)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWMIFilter<P0>(&self, bstrpath: P0) -> ::windows_core::Result<IGPMWMIFilter>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetWMIFilter)(::windows_core::Interface::as_raw(self), bstrpath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchWMIFilters<P0>(&self, pigpmsearchcriteria: P0) -> ::windows_core::Result<IGPMWMIFilterCollection>
    where
        P0: ::windows_core::IntoParam<IGPMSearchCriteria>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.SearchWMIFilters)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStarterGPO(&self) -> ::windows_core::Result<IGPMStarterGPO> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateStarterGPO)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGPOFromStarterGPO<P0>(&self, pgpotemplate: P0) -> ::windows_core::Result<IGPMGPO>
    where
        P0: ::windows_core::IntoParam<IGPMStarterGPO>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateGPOFromStarterGPO)(::windows_core::Interface::as_raw(self), pgpotemplate.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStarterGPO<P0>(&self, bstrguid: P0) -> ::windows_core::Result<IGPMStarterGPO>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetStarterGPO)(::windows_core::Interface::as_raw(self), bstrguid.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchStarterGPOs<P0>(&self, pigpmsearchcriteria: P0) -> ::windows_core::Result<IGPMStarterGPOCollection>
    where
        P0: ::windows_core::IntoParam<IGPMSearchCriteria>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.SearchStarterGPOs)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LoadStarterGPO<P0, P1>(&self, bstrloadfile: P0, boverwrite: P1, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.LoadStarterGPO)(::windows_core::Interface::as_raw(self), bstrloadfile.into_param().abi(), boverwrite.into_param().abi(), pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn RestoreStarterGPO<P0>(&self, pigpmtmplbackup: P0, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMStarterGPOBackup>,
    {
        (::windows_core::Interface::vtable(self).base__.RestoreStarterGPO)(::windows_core::Interface::as_raw(self), pigpmtmplbackup.into_param().abi(), pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GenerateReport)(::windows_core::Interface::as_raw(self), gpmreporttype, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    pub unsafe fn InfrastructureDC(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InfrastructureDC)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInfrastructureDC<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetInfrastructureDC)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn SetInfrastructureFlags(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInfrastructureFlags)(::windows_core::Interface::as_raw(self), dwflags).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMDomain3, ::windows_core::IUnknown, super::Com::IDispatch, IGPMDomain, IGPMDomain2);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMDomain3 {
    type Vtable = IGPMDomain3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMDomain3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0077fdfe_88c7_4acf_a11d_d10a7c310a03);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMDomain3_Vtbl {
    pub base__: IGPMDomain2_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GenerateReport: usize,
    pub InfrastructureDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetInfrastructureDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetInfrastructureFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMGPO(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPO {
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDisplayName<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDisplayName)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DomainName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DomainName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreationTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ModificationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ModificationTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserDSVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UserDSVersionNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ComputerDSVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ComputerDSVersionNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserSysvolVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UserSysvolVersionNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ComputerSysvolVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ComputerSysvolVersionNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWMIFilter(&self) -> ::windows_core::Result<IGPMWMIFilter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetWMIFilter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetWMIFilter<P0>(&self, pigpmwmifilter: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMWMIFilter>,
    {
        (::windows_core::Interface::vtable(self).SetWMIFilter)(::windows_core::Interface::as_raw(self), pigpmwmifilter.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserEnabled<P0>(&self, vbenabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetUserEnabled)(::windows_core::Interface::as_raw(self), vbenabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetComputerEnabled<P0>(&self, vbenabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetComputerEnabled)(::windows_core::Interface::as_raw(self), vbenabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUserEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsUserEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsComputerEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsComputerEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows_core::Result<IGPMSecurityInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSecurityInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityInfo<P0>(&self, psecurityinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMSecurityInfo>,
    {
        (::windows_core::Interface::vtable(self).SetSecurityInfo)(::windows_core::Interface::as_raw(self), psecurityinfo.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Backup<P0, P1>(&self, bstrbackupdir: P0, bstrcomment: P1, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Backup)(::windows_core::Interface::as_raw(self), bstrbackupdir.into_param().abi(), bstrcomment.into_param().abi(), pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Import<P0>(&self, lflags: i32, pigpmbackup: P0, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMBackup>,
    {
        (::windows_core::Interface::vtable(self).Import)(::windows_core::Interface::as_raw(self), lflags, pigpmbackup.into_param().abi(), pvarmigrationtable, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GenerateReport)(::windows_core::Interface::as_raw(self), gpmreporttype, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GenerateReportToFile<P0>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: P0) -> ::windows_core::Result<IGPMResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GenerateReportToFile)(::windows_core::Interface::as_raw(self), gpmreporttype, bstrtargetfilepath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CopyTo<P0>(&self, lflags: i32, pigpmdomain: P0, pvarnewdisplayname: *const super::Variant::VARIANT, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMDomain>,
    {
        (::windows_core::Interface::vtable(self).CopyTo)(::windows_core::Interface::as_raw(self), lflags, pigpmdomain.into_param().abi(), pvarnewdisplayname, pvarmigrationtable, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityDescriptor<P0>(&self, lflags: i32, psd: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).SetSecurityDescriptor)(::windows_core::Interface::as_raw(self), lflags, psd.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityDescriptor(&self, lflags: i32) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSecurityDescriptor)(::windows_core::Interface::as_raw(self), lflags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsACLConsistent(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsACLConsistent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MakeACLConsistent(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MakeACLConsistent)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMGPO, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMGPO {
    type Vtable = IGPMGPO_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMGPO {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x58cc4352_1ca3_48e5_9864_1da4d6e0d60f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPO_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DomainName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CreationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows_core::HRESULT,
    pub ModificationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows_core::HRESULT,
    pub UserDSVersionNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub ComputerDSVersionNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub UserSysvolVersionNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    pub ComputerSysvolVersionNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetWMIFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmwmifilter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetWMIFilter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetWMIFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmwmifilter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetWMIFilter: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUserEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUserEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetComputerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vbenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetComputerEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsUserEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsUserEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsComputerEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsComputerEnabled: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psecurityinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityInfo: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Backup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbackupdir: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrcomment: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Backup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Import: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pigpmbackup: *mut ::core::ffi::c_void, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Import: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GenerateReport: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GenerateReportToFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, pigpmdomain: *mut ::core::ffi::c_void, pvarnewdisplayname: *const super::Variant::VARIANT, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CopyTo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, psd: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityDescriptor: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityDescriptor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, ppsd: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityDescriptor: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsACLConsistent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvbconsistent: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsACLConsistent: usize,
    pub MakeACLConsistent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMGPO2(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPO2 {
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDisplayName<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDisplayName)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DomainName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.DomainName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreationTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ModificationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ModificationTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserDSVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserDSVersionNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ComputerDSVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ComputerDSVersionNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserSysvolVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.UserSysvolVersionNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ComputerSysvolVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.ComputerSysvolVersionNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWMIFilter(&self) -> ::windows_core::Result<IGPMWMIFilter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetWMIFilter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetWMIFilter<P0>(&self, pigpmwmifilter: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMWMIFilter>,
    {
        (::windows_core::Interface::vtable(self).base__.SetWMIFilter)(::windows_core::Interface::as_raw(self), pigpmwmifilter.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserEnabled<P0>(&self, vbenabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetUserEnabled)(::windows_core::Interface::as_raw(self), vbenabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetComputerEnabled<P0>(&self, vbenabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetComputerEnabled)(::windows_core::Interface::as_raw(self), vbenabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUserEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsUserEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsComputerEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsComputerEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows_core::Result<IGPMSecurityInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSecurityInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityInfo<P0>(&self, psecurityinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMSecurityInfo>,
    {
        (::windows_core::Interface::vtable(self).base__.SetSecurityInfo)(::windows_core::Interface::as_raw(self), psecurityinfo.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Backup<P0, P1>(&self, bstrbackupdir: P0, bstrcomment: P1, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.Backup)(::windows_core::Interface::as_raw(self), bstrbackupdir.into_param().abi(), bstrcomment.into_param().abi(), pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Import<P0>(&self, lflags: i32, pigpmbackup: P0, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMBackup>,
    {
        (::windows_core::Interface::vtable(self).base__.Import)(::windows_core::Interface::as_raw(self), lflags, pigpmbackup.into_param().abi(), pvarmigrationtable, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GenerateReport)(::windows_core::Interface::as_raw(self), gpmreporttype, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GenerateReportToFile<P0>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: P0) -> ::windows_core::Result<IGPMResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GenerateReportToFile)(::windows_core::Interface::as_raw(self), gpmreporttype, bstrtargetfilepath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CopyTo<P0>(&self, lflags: i32, pigpmdomain: P0, pvarnewdisplayname: *const super::Variant::VARIANT, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMDomain>,
    {
        (::windows_core::Interface::vtable(self).base__.CopyTo)(::windows_core::Interface::as_raw(self), lflags, pigpmdomain.into_param().abi(), pvarnewdisplayname, pvarmigrationtable, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityDescriptor<P0>(&self, lflags: i32, psd: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.SetSecurityDescriptor)(::windows_core::Interface::as_raw(self), lflags, psd.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityDescriptor(&self, lflags: i32) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSecurityDescriptor)(::windows_core::Interface::as_raw(self), lflags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsACLConsistent(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsACLConsistent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MakeACLConsistent(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.MakeACLConsistent)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMGPO2, ::windows_core::IUnknown, super::Com::IDispatch, IGPMGPO);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMGPO2 {
    type Vtable = IGPMGPO2_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMGPO2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8a66a210_b78b_4d99_88e2_c306a817c925);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPO2_Vtbl {
    pub base__: IGPMGPO_Vtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMGPO3(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPO3 {
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDisplayName<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetDisplayName)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DomainName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.DomainName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.CreationTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ModificationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ModificationTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserDSVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UserDSVersionNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ComputerDSVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ComputerDSVersionNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserSysvolVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.UserSysvolVersionNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ComputerSysvolVersionNumber(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.ComputerSysvolVersionNumber)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetWMIFilter(&self) -> ::windows_core::Result<IGPMWMIFilter> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetWMIFilter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetWMIFilter<P0>(&self, pigpmwmifilter: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMWMIFilter>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetWMIFilter)(::windows_core::Interface::as_raw(self), pigpmwmifilter.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserEnabled<P0>(&self, vbenabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetUserEnabled)(::windows_core::Interface::as_raw(self), vbenabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetComputerEnabled<P0>(&self, vbenabled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetComputerEnabled)(::windows_core::Interface::as_raw(self), vbenabled.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUserEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsUserEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsComputerEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsComputerEnabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows_core::Result<IGPMSecurityInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSecurityInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityInfo<P0>(&self, psecurityinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMSecurityInfo>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetSecurityInfo)(::windows_core::Interface::as_raw(self), psecurityinfo.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Backup<P0, P1>(&self, bstrbackupdir: P0, bstrcomment: P1, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Backup)(::windows_core::Interface::as_raw(self), bstrbackupdir.into_param().abi(), bstrcomment.into_param().abi(), pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Import<P0>(&self, lflags: i32, pigpmbackup: P0, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMBackup>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.Import)(::windows_core::Interface::as_raw(self), lflags, pigpmbackup.into_param().abi(), pvarmigrationtable, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GenerateReport)(::windows_core::Interface::as_raw(self), gpmreporttype, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GenerateReportToFile<P0>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: P0) -> ::windows_core::Result<IGPMResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GenerateReportToFile)(::windows_core::Interface::as_raw(self), gpmreporttype, bstrtargetfilepath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CopyTo<P0>(&self, lflags: i32, pigpmdomain: P0, pvarnewdisplayname: *const super::Variant::VARIANT, pvarmigrationtable: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMDomain>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.CopyTo)(::windows_core::Interface::as_raw(self), lflags, pigpmdomain.into_param().abi(), pvarnewdisplayname, pvarmigrationtable, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityDescriptor<P0>(&self, lflags: i32, psd: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Com::IDispatch>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetSecurityDescriptor)(::windows_core::Interface::as_raw(self), lflags, psd.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityDescriptor(&self, lflags: i32) -> ::windows_core::Result<super::Com::IDispatch> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetSecurityDescriptor)(::windows_core::Interface::as_raw(self), lflags, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsACLConsistent(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.IsACLConsistent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn MakeACLConsistent(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.MakeACLConsistent)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDescription)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn InfrastructureDC(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).InfrastructureDC)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetInfrastructureDC<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetInfrastructureDC)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn SetInfrastructureFlags(&self, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetInfrastructureFlags)(::windows_core::Interface::as_raw(self), dwflags).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMGPO3, ::windows_core::IUnknown, super::Com::IDispatch, IGPMGPO, IGPMGPO2);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMGPO3 {
    type Vtable = IGPMGPO3_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMGPO3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7cf123a1_f94a_4112_bfae_6aa1db9cb248);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPO3_Vtbl {
    pub base__: IGPMGPO2_Vtbl,
    pub InfrastructureDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetInfrastructureDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetInfrastructureFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMGPOCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPOCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMGPOCollection, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMGPOCollection {
    type Vtable = IGPMGPOCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMGPOCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0f0d5cf_70ca_4c39_9e29_b642f8726c01);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPOCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmgpos: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMGPOLink(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPOLink {
    pub unsafe fn GPOID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GPOID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GPODomain(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GPODomain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enabled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetEnabled)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enforced(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Enforced)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnforced<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetEnforced)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn SOMLinkOrder(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SOMLinkOrder)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SOM(&self) -> ::windows_core::Result<IGPMSOM> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SOM)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMGPOLink, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMGPOLink {
    type Vtable = IGPMGPOLink_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMGPOLink {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x434b99bd_5de7_478a_809c_c251721df70c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPOLink_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub GPOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GPODomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Enforced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enforced: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnforced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnforced: usize,
    pub SOMLinkOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmsom: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SOM: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMGPOLinksCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMGPOLinksCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMGPOLinksCollection, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMGPOLinksCollection {
    type Vtable = IGPMGPOLinksCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMGPOLinksCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x189d7b68_16bd_4d0d_a2ec_2e6aa2288c7f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMGPOLinksCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmlinks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMMapEntry(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMMapEntry {
    pub unsafe fn Source(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Source)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Destination(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Destination)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DestinationOption(&self) -> ::windows_core::Result<GPMDestinationOption> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DestinationOption)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EntryType(&self) -> ::windows_core::Result<GPMEntryType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).EntryType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMMapEntry, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMMapEntry {
    type Vtable = IGPMMapEntry_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMMapEntry {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8e79ad06_2381_4444_be4c_ff693e6e6f2b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMMapEntry_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Destination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdestination: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DestinationOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgpmdestoption: *mut GPMDestinationOption) -> ::windows_core::HRESULT,
    pub EntryType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgpmentrytype: *mut GPMEntryType) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMMapEntryCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMMapEntryCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMMapEntryCollection, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMMapEntryCollection {
    type Vtable = IGPMMapEntryCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMMapEntryCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbb0bf49b_e53f_443f_b807_8be22bfb6d42);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMMapEntryCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMMigrationTable(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMMigrationTable {
    pub unsafe fn Save<P0>(&self, bstrmigrationtablepath: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), bstrmigrationtablepath.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Add(&self, lflags: i32, var: super::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), lflags, ::core::mem::transmute(var)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn AddEntry<P0>(&self, bstrsource: P0, gpmentrytype: GPMEntryType, pvardestination: *const super::Variant::VARIANT) -> ::windows_core::Result<IGPMMapEntry>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AddEntry)(::windows_core::Interface::as_raw(self), bstrsource.into_param().abi(), gpmentrytype, pvardestination, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEntry<P0>(&self, bstrsource: P0) -> ::windows_core::Result<IGPMMapEntry>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEntry)(::windows_core::Interface::as_raw(self), bstrsource.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteEntry<P0>(&self, bstrsource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).DeleteEntry)(::windows_core::Interface::as_raw(self), bstrsource.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn UpdateDestination<P0>(&self, bstrsource: P0, pvardestination: *const super::Variant::VARIANT) -> ::windows_core::Result<IGPMMapEntry>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UpdateDestination)(::windows_core::Interface::as_raw(self), bstrsource.into_param().abi(), pvardestination, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Validate(&self) -> ::windows_core::Result<IGPMResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Validate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEntries(&self) -> ::windows_core::Result<IGPMMapEntryCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEntries)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMMigrationTable, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMMigrationTable {
    type Vtable = IGPMMigrationTable_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMMigrationTable {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x48f823b1_efaf_470b_b6ed_40d14ee1a4ec);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMMigrationTable_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmigrationtablepath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lflags: i32, var: super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Add: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub AddEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsource: ::std::mem::MaybeUninit<::windows_core::BSTR>, gpmentrytype: GPMEntryType, pvardestination: *const super::Variant::VARIANT, ppentry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    AddEntry: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsource: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppentry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEntry: usize,
    pub DeleteEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsource: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub UpdateDestination: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsource: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvardestination: *const super::Variant::VARIANT, ppentry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    UpdateDestination: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Validate: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppentries: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEntries: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMPermission(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMPermission {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Inherited(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Inherited)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Inheritable(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Inheritable)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Denied(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Denied)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Permission(&self) -> ::windows_core::Result<GPMPermissionType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Permission)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Trustee(&self) -> ::windows_core::Result<IGPMTrustee> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Trustee)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMPermission, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMPermission {
    type Vtable = IGPMPermission_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMPermission {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35ebca40_e1a1_4a02_8905_d79416fb464a);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMPermission_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Inherited: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Inherited: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Inheritable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Inheritable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Denied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Denied: usize,
    pub Permission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMPermissionType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Trustee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmtrustee: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Trustee: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMRSOP(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMRSOP {
    pub unsafe fn Mode(&self) -> ::windows_core::Result<GPMRSOPMode> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Mode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Namespace(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Namespace)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLoggingComputer<P0>(&self, bstrval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLoggingComputer)(::windows_core::Interface::as_raw(self), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn LoggingComputer(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LoggingComputer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLoggingUser<P0>(&self, bstrval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLoggingUser)(::windows_core::Interface::as_raw(self), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn LoggingUser(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LoggingUser)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLoggingFlags(&self, lval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLoggingFlags)(::windows_core::Interface::as_raw(self), lval).ok()
    }
    pub unsafe fn LoggingFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LoggingFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPlanningFlags(&self, lval: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlanningFlags)(::windows_core::Interface::as_raw(self), lval).ok()
    }
    pub unsafe fn PlanningFlags(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PlanningFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPlanningDomainController<P0>(&self, bstrval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPlanningDomainController)(::windows_core::Interface::as_raw(self), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn PlanningDomainController(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PlanningDomainController)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPlanningSiteName<P0>(&self, bstrval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPlanningSiteName)(::windows_core::Interface::as_raw(self), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn PlanningSiteName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PlanningSiteName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPlanningUser<P0>(&self, bstrval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPlanningUser)(::windows_core::Interface::as_raw(self), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn PlanningUser(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PlanningUser)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPlanningUserSOM<P0>(&self, bstrval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPlanningUserSOM)(::windows_core::Interface::as_raw(self), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn PlanningUserSOM(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PlanningUserSOM)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetPlanningUserWMIFilters(&self, varval: super::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlanningUserWMIFilters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varval)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PlanningUserWMIFilters(&self) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PlanningUserWMIFilters)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetPlanningUserSecurityGroups(&self, varval: super::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlanningUserSecurityGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varval)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PlanningUserSecurityGroups(&self) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PlanningUserSecurityGroups)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPlanningComputer<P0>(&self, bstrval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPlanningComputer)(::windows_core::Interface::as_raw(self), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn PlanningComputer(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PlanningComputer)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPlanningComputerSOM<P0>(&self, bstrval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPlanningComputerSOM)(::windows_core::Interface::as_raw(self), bstrval.into_param().abi()).ok()
    }
    pub unsafe fn PlanningComputerSOM(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PlanningComputerSOM)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetPlanningComputerWMIFilters(&self, varval: super::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlanningComputerWMIFilters)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varval)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PlanningComputerWMIFilters(&self) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PlanningComputerWMIFilters)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetPlanningComputerSecurityGroups(&self, varval: super::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPlanningComputerSecurityGroups)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(varval)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn PlanningComputerSecurityGroups(&self) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).PlanningComputerSecurityGroups)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn LoggingEnumerateUsers(&self) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LoggingEnumerateUsers)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateQueryResults(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CreateQueryResults)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ReleaseQueryResults(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ReleaseQueryResults)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GenerateReport)(::windows_core::Interface::as_raw(self), gpmreporttype, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GenerateReportToFile<P0>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: P0) -> ::windows_core::Result<IGPMResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GenerateReportToFile)(::windows_core::Interface::as_raw(self), gpmreporttype, bstrtargetfilepath.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMRSOP, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMRSOP {
    type Vtable = IGPMRSOP_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMRSOP {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x49ed785a_3237_4ff2_b1f0_fdf5a8d5a1ee);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMRSOP_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMRSOPMode) -> ::windows_core::HRESULT,
    pub Namespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetLoggingComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LoggingComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetLoggingUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub LoggingUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetLoggingFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows_core::HRESULT,
    pub LoggingFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows_core::HRESULT,
    pub SetPlanningFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: i32) -> ::windows_core::HRESULT,
    pub PlanningFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows_core::HRESULT,
    pub SetPlanningDomainController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PlanningDomainController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPlanningSiteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PlanningSiteName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPlanningUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PlanningUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPlanningUserSOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PlanningUserSOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetPlanningUserWMIFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetPlanningUserWMIFilters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PlanningUserWMIFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PlanningUserWMIFilters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetPlanningUserSecurityGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetPlanningUserSecurityGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PlanningUserSecurityGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PlanningUserSecurityGroups: usize,
    pub SetPlanningComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PlanningComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPlanningComputerSOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub PlanningComputerSOM: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetPlanningComputerWMIFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetPlanningComputerWMIFilters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PlanningComputerWMIFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PlanningComputerWMIFilters: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetPlanningComputerSecurityGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetPlanningComputerSecurityGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub PlanningComputerSecurityGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    PlanningComputerSecurityGroups: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub LoggingEnumerateUsers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, varval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    LoggingEnumerateUsers: usize,
    pub CreateQueryResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReleaseQueryResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GenerateReport: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GenerateReportToFile: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMResult(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMResult {
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Status(&self) -> ::windows_core::Result<IGPMStatusMsgCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Status)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Result(&self) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Result)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OverallStatus(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OverallStatus)(::windows_core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMResult, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMResult {
    type Vtable = IGPMResult_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x86dff7e9_f76f_42ab_9570_cebc6be8a52d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMResult_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmstatusmsgcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Status: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Result: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarresult: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Result: usize,
    pub OverallStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMSOM(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMSOM {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GPOInheritanceBlocked(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GPOInheritanceBlocked)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGPOInheritanceBlocked<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetGPOInheritanceBlocked)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateGPOLink<P0>(&self, llinkpos: i32, pgpo: P0) -> ::windows_core::Result<IGPMGPOLink>
    where
        P0: ::windows_core::IntoParam<IGPMGPO>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateGPOLink)(::windows_core::Interface::as_raw(self), llinkpos, pgpo.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<GPMSOMType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetGPOLinks(&self) -> ::windows_core::Result<IGPMGPOLinksCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGPOLinks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInheritedGPOLinks(&self) -> ::windows_core::Result<IGPMGPOLinksCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetInheritedGPOLinks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows_core::Result<IGPMSecurityInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSecurityInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityInfo<P0>(&self, psecurityinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMSecurityInfo>,
    {
        (::windows_core::Interface::vtable(self).SetSecurityInfo)(::windows_core::Interface::as_raw(self), psecurityinfo.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMSOM, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMSOM {
    type Vtable = IGPMSOM_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMSOM {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc0a7f09e_05a1_4f0c_8158_9e5c33684f6b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSOM_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GPOInheritanceBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GPOInheritanceBlocked: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGPOInheritanceBlocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGPOInheritanceBlocked: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateGPOLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, llinkpos: i32, pgpo: *mut ::core::ffi::c_void, ppnewgpolink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateGPOLink: usize,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMSOMType) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetGPOLinks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgpolinks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetGPOLinks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetInheritedGPOLinks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppgpolinks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetInheritedGPOLinks: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psecurityinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityInfo: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMSOMCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMSOMCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMSOMCollection, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMSOMCollection {
    type Vtable = IGPMSOMCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMSOMCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xadc1688e_00e4_4495_abba_bed200df0cab);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSOMCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmsom: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMSearchCriteria(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMSearchCriteria {
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Add(&self, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: super::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), searchproperty, searchoperation, ::core::mem::transmute(varvalue)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMSearchCriteria, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMSearchCriteria {
    type Vtable = IGPMSearchCriteria_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMSearchCriteria {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd6f11c42_829b_48d4_83f5_3615b67dfc22);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSearchCriteria_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchproperty: GPMSearchProperty, searchoperation: GPMSearchOperation, varvalue: super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Add: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMSecurityInfo(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMSecurityInfo {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, pperm: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMPermission>,
    {
        (::windows_core::Interface::vtable(self).Add)(::windows_core::Interface::as_raw(self), pperm.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Remove<P0>(&self, pperm: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMPermission>,
    {
        (::windows_core::Interface::vtable(self).Remove)(::windows_core::Interface::as_raw(self), pperm.into_param().abi()).ok()
    }
    pub unsafe fn RemoveTrustee<P0>(&self, bstrtrustee: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).RemoveTrustee)(::windows_core::Interface::as_raw(self), bstrtrustee.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMSecurityInfo, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMSecurityInfo {
    type Vtable = IGPMSecurityInfo_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMSecurityInfo {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb6c31ed4_1c93_4d3e_ae84_eb6d61161b60);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSecurityInfo_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperm: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pperm: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Remove: usize,
    pub RemoveTrustee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtrustee: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMSitesContainer(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMSitesContainer {
    pub unsafe fn DomainController(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DomainController)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Domain(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Domain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Forest(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Forest)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSite<P0>(&self, bstrsitename: P0) -> ::windows_core::Result<IGPMSOM>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSite)(::windows_core::Interface::as_raw(self), bstrsitename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SearchSites<P0>(&self, pigpmsearchcriteria: P0) -> ::windows_core::Result<IGPMSOMCollection>
    where
        P0: ::windows_core::IntoParam<IGPMSearchCriteria>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SearchSites)(::windows_core::Interface::as_raw(self), pigpmsearchcriteria.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMSitesContainer, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMSitesContainer {
    type Vtable = IGPMSitesContainer_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMSitesContainer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4725a899_2782_4d27_a6bb_d499246ffd72);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMSitesContainer_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DomainController: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Forest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsitename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsom: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSite: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SearchSites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pigpmsearchcriteria: *mut ::core::ffi::c_void, ppigpmsomcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SearchSites: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMStarterGPO(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStarterGPO {
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDisplayName<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDisplayName)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn Author(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Author)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Product(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Product)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreationTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreationTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ModifiedTime(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ModifiedTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<GPMStarterGPOType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ComputerVersion(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ComputerVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UserVersion(&self) -> ::windows_core::Result<u16> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).UserVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StarterGPOVersion(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StarterGPOVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Save<P0, P1, P2>(&self, bstrsavefile: P0, boverwrite: P1, bsaveassystem: P2, bstrlanguage: *const super::Variant::VARIANT, bstrauthor: *const super::Variant::VARIANT, bstrproduct: *const super::Variant::VARIANT, bstruniqueid: *const super::Variant::VARIANT, bstrversion: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
        P2: ::windows_core::IntoParam<super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), bstrsavefile.into_param().abi(), boverwrite.into_param().abi(), bsaveassystem.into_param().abi(), bstrlanguage, bstrauthor, bstrproduct, bstruniqueid, bstrversion, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn Backup<P0, P1>(&self, bstrbackupdir: P0, bstrcomment: P1, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
        P1: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).Backup)(::windows_core::Interface::as_raw(self), bstrbackupdir.into_param().abi(), bstrcomment.into_param().abi(), pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn CopyTo(&self, pvarnewdisplayname: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *const super::Variant::VARIANT) -> ::windows_core::Result<IGPMResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CopyTo)(::windows_core::Interface::as_raw(self), pvarnewdisplayname, pvargpmprogress, pvargpmcancel, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *const super::Variant::VARIANT) -> ::windows_core::Result<IGPMResult> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GenerateReport)(::windows_core::Interface::as_raw(self), gpmreporttype, pvargpmprogress, pvargpmcancel, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GenerateReportToFile<P0>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: P0) -> ::windows_core::Result<IGPMResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GenerateReportToFile)(::windows_core::Interface::as_raw(self), gpmreporttype, bstrtargetfilepath.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows_core::Result<IGPMSecurityInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSecurityInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityInfo<P0>(&self, psecurityinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMSecurityInfo>,
    {
        (::windows_core::Interface::vtable(self).SetSecurityInfo)(::windows_core::Interface::as_raw(self), psecurityinfo.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMStarterGPO, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMStarterGPO {
    type Vtable = IGPMStarterGPO_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMStarterGPO {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdfc3f61b_8880_4490_9337_d29c7ba8c2f0);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPO_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Product: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub CreationTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT,
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ModifiedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut GPMStarterGPOType) -> ::windows_core::HRESULT,
    pub ComputerVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u16) -> ::windows_core::HRESULT,
    pub UserVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut u16) -> ::windows_core::HRESULT,
    pub StarterGPOVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsavefile: ::std::mem::MaybeUninit<::windows_core::BSTR>, boverwrite: super::super::Foundation::VARIANT_BOOL, bsaveassystem: super::super::Foundation::VARIANT_BOOL, bstrlanguage: *const super::Variant::VARIANT, bstrauthor: *const super::Variant::VARIANT, bstrproduct: *const super::Variant::VARIANT, bstruniqueid: *const super::Variant::VARIANT, bstrversion: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Save: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub Backup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrbackupdir: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrcomment: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    Backup: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarnewdisplayname: *const super::Variant::VARIANT, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *const super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    CopyTo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *const super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GenerateReport: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GenerateReportToFile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psecurityinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityInfo: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMStarterGPOBackup(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStarterGPOBackup {
    pub unsafe fn BackupDir(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).BackupDir)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Comment(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Comment)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).DisplayName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Domain(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Domain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn StarterGPOID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).StarterGPOID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ID(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ID)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Timestamp(&self) -> ::windows_core::Result<f64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Timestamp)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows_core::Result<GPMStarterGPOType> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Type)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GenerateReport(&self, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut ::core::option::Option<IGPMResult>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GenerateReport)(::windows_core::Interface::as_raw(self), gpmreporttype, pvargpmprogress, pvargpmcancel, ::core::mem::transmute(ppigpmresult)).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GenerateReportToFile<P0>(&self, gpmreporttype: GPMReportType, bstrtargetfilepath: P0) -> ::windows_core::Result<IGPMResult>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GenerateReportToFile)(::windows_core::Interface::as_raw(self), gpmreporttype, bstrtargetfilepath.into_param().abi(), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMStarterGPOBackup, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMStarterGPOBackup {
    type Vtable = IGPMStarterGPOBackup_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMStarterGPOBackup {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x51d98eda_a87e_43dd_b80a_0b66ef1938d6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPOBackup_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub BackupDir: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbackupdir: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Comment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcomment: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtemplatedomain: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub StarterGPOID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtemplateid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptimestamp: *mut f64) -> ::windows_core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut GPMStarterGPOType) -> ::windows_core::HRESULT,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GenerateReport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, pvargpmprogress: *const super::Variant::VARIANT, pvargpmcancel: *mut super::Variant::VARIANT, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GenerateReport: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GenerateReportToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpmreporttype: GPMReportType, bstrtargetfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppigpmresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GenerateReportToFile: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMStarterGPOBackupCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStarterGPOBackupCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMStarterGPOBackupCollection, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMStarterGPOBackupCollection {
    type Vtable = IGPMStarterGPOBackupCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMStarterGPOBackupCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc998031d_add0_4bb5_8dea_298505d8423b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPOBackupCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmtmplbackup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMStarterGPOCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStarterGPOCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMStarterGPOCollection, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMStarterGPOCollection {
    type Vtable = IGPMStarterGPOCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMStarterGPOCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e522729_2219_44ad_933a_64dfd650c423);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStarterGPOCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppigpmtemplates: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMStatusMessage(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStatusMessage {
    pub unsafe fn ObjectPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ObjectPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ErrorCode(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ErrorCode)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExtensionName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ExtensionName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SettingsName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SettingsName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn OperationCode(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OperationCode)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Message(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Message)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMStatusMessage, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMStatusMessage {
    type Vtable = IGPMStatusMessage_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMStatusMessage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8496c22f_f3de_4a1f_8f58_603caaa93d7b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStatusMessage_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub ObjectPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ExtensionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SettingsName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub OperationCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMStatusMsgCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMStatusMsgCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMStatusMsgCollection, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMStatusMsgCollection {
    type Vtable = IGPMStatusMsgCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMStatusMsgCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9b6e1af0_1a92_40f3_a59d_f36ac1f728b7);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMStatusMsgCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMTrustee(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMTrustee {
    pub unsafe fn TrusteeSid(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TrusteeSid)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TrusteeName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TrusteeName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TrusteeDomain(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TrusteeDomain)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TrusteeDSPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TrusteeDSPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn TrusteeType(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TrusteeType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMTrustee, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMTrustee {
    type Vtable = IGPMTrustee_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMTrustee {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b466da8_c1a4_4b2a_999a_befcdd56cefb);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMTrustee_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub TrusteeSid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TrusteeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TrusteeDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TrusteeDSPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub TrusteeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lval: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMWMIFilter(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMWMIFilter {
    pub unsafe fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Path)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Name)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, newval: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), newval.into_param().abi()).ok()
    }
    pub unsafe fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Description)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetQueryList(&self) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetQueryList)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSecurityInfo(&self) -> ::windows_core::Result<IGPMSecurityInfo> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSecurityInfo)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSecurityInfo<P0>(&self, psecurityinfo: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IGPMSecurityInfo>,
    {
        (::windows_core::Interface::vtable(self).SetSecurityInfo)(::windows_core::Interface::as_raw(self), psecurityinfo.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMWMIFilter, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMWMIFilter {
    type Vtable = IGPMWMIFilter_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMWMIFilter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xef2ff9b4_3c27_459a_b979_038305cec75d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMWMIFilter_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetQueryList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pqrylist: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetQueryList: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsecurityinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetSecurityInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetSecurityInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psecurityinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetSecurityInfo: usize,
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGPMWMIFilterCollection(::windows_core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IGPMWMIFilterCollection {
    pub unsafe fn Count(&self) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Count)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Variant::VARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).get_Item)(::windows_core::Interface::as_raw(self), lindex, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Ole\"`"]
    #[cfg(feature = "Win32_System_Ole")]
    pub unsafe fn _NewEnum(&self) -> ::windows_core::Result<super::Ole::IEnumVARIANT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self)._NewEnum)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows_core::imp::interface_hierarchy!(IGPMWMIFilterCollection, ::windows_core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::Interface for IGPMWMIFilterCollection {
    type Vtable = IGPMWMIFilterCollection_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows_core::ComInterface for IGPMWMIFilterCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5782d582_1a36_4661_8a94_c3c32551945b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IGPMWMIFilterCollection_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lindex: i32, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    get_Item: usize,
    #[cfg(feature = "Win32_System_Ole")]
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))]
    _NewEnum: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IGroupPolicyObject(::windows_core::IUnknown);
impl IGroupPolicyObject {
    pub unsafe fn New<P0, P1>(&self, pszdomainname: P0, pszdisplayname: P1, dwflags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).New)(::windows_core::Interface::as_raw(self), pszdomainname.into_param().abi(), pszdisplayname.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn OpenDSGPO<P0>(&self, pszpath: P0, dwflags: GPO_OPEN_FLAGS) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).OpenDSGPO)(::windows_core::Interface::as_raw(self), pszpath.into_param().abi(), dwflags).ok()
    }
    pub unsafe fn OpenLocalMachineGPO(&self, dwflags: GPO_OPEN_FLAGS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OpenLocalMachineGPO)(::windows_core::Interface::as_raw(self), dwflags).ok()
    }
    pub unsafe fn OpenRemoteMachineGPO<P0>(&self, pszcomputername: P0, dwflags: GPO_OPEN_FLAGS) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).OpenRemoteMachineGPO)(::windows_core::Interface::as_raw(self), pszcomputername.into_param().abi(), dwflags).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Save<P0, P1>(&self, bmachine: P0, badd: P1, pguidextension: *mut ::windows_core::GUID, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).Save)(::windows_core::Interface::as_raw(self), bmachine.into_param().abi(), badd.into_param().abi(), pguidextension, pguid).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Delete)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetName(&self, pszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDisplayName(&self, pszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDisplayName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn SetDisplayName<P0>(&self, pszname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDisplayName)(::windows_core::Interface::as_raw(self), pszname.into_param().abi()).ok()
    }
    pub unsafe fn GetPath(&self, pszpath: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetDSPath(&self, dwsection: u32, pszpath: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDSPath)(::windows_core::Interface::as_raw(self), dwsection, ::core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetFileSysPath(&self, dwsection: u32, pszpath: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFileSysPath)(::windows_core::Interface::as_raw(self), dwsection, ::core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Registry\"`"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn GetRegistryKey(&self, dwsection: GPO_SECTION, hkey: *mut super::Registry::HKEY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRegistryKey)(::windows_core::Interface::as_raw(self), dwsection, hkey).ok()
    }
    pub unsafe fn GetOptions(&self, dwoptions: *mut GPO_OPTIONS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetOptions)(::windows_core::Interface::as_raw(self), dwoptions).ok()
    }
    pub unsafe fn SetOptions(&self, dwoptions: GPO_OPTIONS, dwmask: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOptions)(::windows_core::Interface::as_raw(self), dwoptions, dwmask).ok()
    }
    pub unsafe fn GetType(&self, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetType)(::windows_core::Interface::as_raw(self), gpotype).ok()
    }
    pub unsafe fn GetMachineName(&self, pszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetMachineName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()).ok()
    }
    #[doc = "Required features: `\"Win32_UI_Controls\"`"]
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn GetPropertySheetPages(&self, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPropertySheetPages)(::windows_core::Interface::as_raw(self), hpages, upagecount).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IGroupPolicyObject, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IGroupPolicyObject {
    type Vtable = IGroupPolicyObject_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IGroupPolicyObject {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea502723_a23d_11d1_a7d3_0000f87571e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGroupPolicyObject_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub New: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszdomainname: ::windows_core::PCWSTR, pszdisplayname: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT,
    pub OpenDSGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR, dwflags: GPO_OPEN_FLAGS) -> ::windows_core::HRESULT,
    pub OpenLocalMachineGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: GPO_OPEN_FLAGS) -> ::windows_core::HRESULT,
    pub OpenRemoteMachineGPO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszcomputername: ::windows_core::PCWSTR, dwflags: GPO_OPEN_FLAGS) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bmachine: super::super::Foundation::BOOL, badd: super::super::Foundation::BOOL, pguidextension: *mut ::windows_core::GUID, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Save: usize,
    pub Delete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT,
    pub GetDSPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows_core::PWSTR, cchmaxpath: i32) -> ::windows_core::HRESULT,
    pub GetFileSysPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, pszpath: ::windows_core::PWSTR, cchmaxpath: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Registry")]
    pub GetRegistryKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: GPO_SECTION, hkey: *mut super::Registry::HKEY) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    GetRegistryKey: usize,
    pub GetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoptions: *mut GPO_OPTIONS) -> ::windows_core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwoptions: GPO_OPTIONS, dwmask: u32) -> ::windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gpotype: *mut GROUP_POLICY_OBJECT_TYPE) -> ::windows_core::HRESULT,
    pub GetMachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Controls")]
    pub GetPropertySheetPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hpages: *mut *mut super::super::UI::Controls::HPROPSHEETPAGE, upagecount: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))]
    GetPropertySheetPages: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRSOPInformation(::windows_core::IUnknown);
impl IRSOPInformation {
    pub unsafe fn GetNamespace(&self, dwsection: u32, pszname: &mut [u16]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetNamespace)(::windows_core::Interface::as_raw(self), dwsection, ::core::mem::transmute(pszname.as_ptr()), pszname.len().try_into().unwrap()).ok()
    }
    pub unsafe fn GetFlags(&self, pdwflags: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn GetEventLogEntryText<P0, P1, P2>(&self, pszeventsource: P0, pszeventlogname: P1, pszeventtime: P2, dweventid: u32) -> ::windows_core::Result<::windows_core::PWSTR>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEventLogEntryText)(::windows_core::Interface::as_raw(self), pszeventsource.into_param().abi(), pszeventlogname.into_param().abi(), pszeventtime.into_param().abi(), dweventid, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IRSOPInformation, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRSOPInformation {
    type Vtable = IRSOPInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRSOPInformation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9a5a81b5_d9c7_49ef_9d11_ddf50968c48d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRSOPInformation_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsection: u32, pszname: ::windows_core::PWSTR, cchmaxlength: i32) -> ::windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT,
    pub GetEventLogEntryText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszeventsource: ::windows_core::PCWSTR, pszeventlogname: ::windows_core::PCWSTR, pszeventtime: ::windows_core::PCWSTR, dweventid: u32, ppsztext: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
pub const ABSENT: APPSTATE = APPSTATE(0i32);
pub const ADMXCOMMENTS_EXTENSION_GUID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6c5a2a86_9eb3_42b9_aa83_a7371ba011b9);
pub const APPNAME: INSTALLSPECTYPE = INSTALLSPECTYPE(1i32);
pub const ASSIGNED: APPSTATE = APPSTATE(1i32);
pub const CLSID_GPESnapIn: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fc0b734_a0e1_11d1_a7d3_0000f87571e3);
pub const CLSID_GroupPolicyObject: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea502722_a23d_11d1_a7d3_0000f87571e3);
pub const CLSID_RSOPSnapIn: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6dc3804b_7212_458d_adb0_9a07e2ae1fa2);
pub const COMCLASS: INSTALLSPECTYPE = INSTALLSPECTYPE(4i32);
pub const FILEEXT: INSTALLSPECTYPE = INSTALLSPECTYPE(2i32);
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
pub const GPHintDomain: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(3i32);
pub const GPHintMachine: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(1i32);
pub const GPHintOrganizationalUnit: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(4i32);
pub const GPHintSite: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(2i32);
pub const GPHintUnknown: GROUP_POLICY_HINT_TYPE = GROUP_POLICY_HINT_TYPE(0i32);
pub const GPLinkDomain: GPO_LINK = GPO_LINK(3i32);
pub const GPLinkMachine: GPO_LINK = GPO_LINK(1i32);
pub const GPLinkOrganizationalUnit: GPO_LINK = GPO_LINK(4i32);
pub const GPLinkSite: GPO_LINK = GPO_LINK(2i32);
pub const GPLinkUnknown: GPO_LINK = GPO_LINK(0i32);
pub const GPM: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf5694708_88fe_4b35_babf_e56162d5fbc8);
pub const GPMAsyncCancel: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x372796a9_76ec_479d_ad6c_556318ed5f9d);
pub const GPMBackup: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed1a54b8_5efa_482a_93c0_8ad86f0d68c3);
pub const GPMBackupCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xeb8f035b_70db_4a9f_9676_37c25994e9dc);
pub const GPMBackupDir: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfce4a59d_0f21_4afa_b859_e6d0c62cd10c);
pub const GPMBackupDirEx: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8c0988a_cf03_4c5b_8be2_2aa9ad32aada);
pub const GPMCSECollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcf92b828_2d44_4b61_b10a_b327afd42da8);
pub const GPMClientSideExtension: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1a2e70e_659c_4b1a_940b_f88b0af9c8a4);
pub const GPMConstants: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3855e880_cd9e_4d0c_9eaf_1579283a1888);
pub const GPMDomain: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x710901be_1050_4cb1_838a_c5cff259e183);
pub const GPMGPO: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2ce2994_59b5_4064_b581_4d68486a16c4);
pub const GPMGPOCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a057325_832d_4de3_a41f_c780436a4e09);
pub const GPMGPOLink: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc1df9880_5303_42c6_8a3c_0488e1bf7364);
pub const GPMGPOLinksCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf6ed581a_49a5_47e2_b771_fd8dc02b6259);
pub const GPMMapEntry: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c975253_5431_4471_b35d_0626c928258a);
pub const GPMMapEntryCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0cf75d5b_a3a1_4c55_b4fe_9e149c41f66d);
pub const GPMMigrationTable: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x55af4043_2a06_4f72_abef_631b44079c76);
pub const GPMPermission: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5871a40a_e9c0_46ec_913e_944ef9225a94);
pub const GPMRSOP: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x489b0caf_9ec2_4eb7_91f5_b6f71d43da8c);
pub const GPMResult: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92101ac0_9287_4206_a3b2_4bdb73d225f6);
pub const GPMSOM: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x32d93fac_450e_44cf_829c_8b22ff6bdae1);
pub const GPMSOMCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x24c1f147_3720_4f5b_a9c3_06b4e4f931d2);
pub const GPMSearchCriteria: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x17aaca26_5ce0_44fa_8cc0_5259e6483566);
pub const GPMSecurityInfo: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x547a5e8f_9162_4516_a4df_9ddb9686d846);
pub const GPMSitesContainer: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x229f5c42_852c_4b30_945f_c522be9bd386);
pub const GPMStarterGPOBackup: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x389e400a_d8ef_455b_a861_5f9ca34a6a02);
pub const GPMStarterGPOBackupCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe75ea59d_1aeb_4cb5_a78a_281daa582406);
pub const GPMStarterGPOCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x82f8aa8b_49ba_43b2_956e_3397f9b94c3a);
pub const GPMStatusMessage: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4b77cc94_d255_409b_bc62_370881715a19);
pub const GPMStatusMsgCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2824e4be_4bcc_4cac_9e60_0e3ed7f12496);
pub const GPMTemplate: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xecf1d454_71da_4e2f_a8c0_8185465911d9);
pub const GPMTrustee: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc54a700d_19b6_4211_bcb0_e8e2475e471e);
pub const GPMWMIFilter: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x626745d8_0dea_4062_bf60_cfc5b1ca1286);
pub const GPMWMIFilterCollection: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74dc6d28_e820_47d6_a0b8_f08d93d7fa33);
pub const GPM_DONOTUSE_W2KDC: u32 = 2u32;
pub const GPM_DONOT_VALIDATEDC: u32 = 1u32;
pub const GPM_MIGRATIONTABLE_ONLY: u32 = 1u32;
pub const GPM_PROCESS_SECURITY: u32 = 2u32;
pub const GPM_USE_ANYDC: u32 = 1u32;
pub const GPM_USE_PDC: u32 = 0u32;
pub const GPOTypeDS: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(2i32);
pub const GPOTypeLocal: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(0i32);
pub const GPOTypeLocalGroup: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(4i32);
pub const GPOTypeLocalUser: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(3i32);
pub const GPOTypeRemote: GROUP_POLICY_OBJECT_TYPE = GROUP_POLICY_OBJECT_TYPE(1i32);
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
pub const GPO_LIST_FLAG_MACHINE: u32 = 1u32;
pub const GPO_LIST_FLAG_NO_SECURITYFILTERS: u32 = 8u32;
pub const GPO_LIST_FLAG_NO_WMIFILTERS: u32 = 4u32;
pub const GPO_LIST_FLAG_SITEONLY: u32 = 2u32;
pub const GPO_OPEN_LOAD_REGISTRY: GPO_OPEN_FLAGS = GPO_OPEN_FLAGS(1u32);
pub const GPO_OPEN_READ_ONLY: GPO_OPEN_FLAGS = GPO_OPEN_FLAGS(2u32);
pub const GPO_OPTION_DISABLE_MACHINE: GPO_OPTIONS = GPO_OPTIONS(2u32);
pub const GPO_OPTION_DISABLE_USER: GPO_OPTIONS = GPO_OPTIONS(1u32);
pub const GPO_SECTION_MACHINE: GPO_SECTION = GPO_SECTION(2u32);
pub const GPO_SECTION_ROOT: GPO_SECTION = GPO_SECTION(0u32);
pub const GPO_SECTION_USER: GPO_SECTION = GPO_SECTION(1u32);
pub const GP_DLLNAME: ::windows_core::PCWSTR = ::windows_core::w!("DllName");
pub const GP_ENABLEASYNCHRONOUSPROCESSING: ::windows_core::PCWSTR = ::windows_core::w!("EnableAsynchronousProcessing");
pub const GP_MAXNOGPOLISTCHANGESINTERVAL: ::windows_core::PCWSTR = ::windows_core::w!("MaxNoGPOListChangesInterval");
pub const GP_NOBACKGROUNDPOLICY: ::windows_core::PCWSTR = ::windows_core::w!("NoBackgroundPolicy");
pub const GP_NOGPOLISTCHANGES: ::windows_core::PCWSTR = ::windows_core::w!("NoGPOListChanges");
pub const GP_NOMACHINEPOLICY: ::windows_core::PCWSTR = ::windows_core::w!("NoMachinePolicy");
pub const GP_NOSLOWLINK: ::windows_core::PCWSTR = ::windows_core::w!("NoSlowLink");
pub const GP_NOTIFYLINKTRANSITION: ::windows_core::PCWSTR = ::windows_core::w!("NotifyLinkTransition");
pub const GP_NOUSERPOLICY: ::windows_core::PCWSTR = ::windows_core::w!("NoUserPolicy");
pub const GP_PERUSERLOCALSETTINGS: ::windows_core::PCWSTR = ::windows_core::w!("PerUserLocalSettings");
pub const GP_PROCESSGROUPPOLICY: ::windows_core::PCWSTR = ::windows_core::w!("ProcessGroupPolicy");
pub const GP_REQUIRESSUCCESSFULREGISTRY: ::windows_core::PCWSTR = ::windows_core::w!("RequiresSuccessfulRegistry");
pub const GROUP_POLICY_TRIGGER_EVENT_PROVIDER_GUID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd2f4252_5e1e_49fc_9a30_f3978ad89ee2);
pub const LOCALSTATE_ASSIGNED: u32 = 1u32;
pub const LOCALSTATE_ORPHANED: u32 = 32u32;
pub const LOCALSTATE_POLICYREMOVE_ORPHAN: u32 = 8u32;
pub const LOCALSTATE_POLICYREMOVE_UNINSTALL: u32 = 16u32;
pub const LOCALSTATE_PUBLISHED: u32 = 2u32;
pub const LOCALSTATE_UNINSTALLED: u32 = 64u32;
pub const LOCALSTATE_UNINSTALL_UNMANAGED: u32 = 4u32;
pub const MACHINE_POLICY_PRESENT_TRIGGER_GUID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x659fcae6_5bdb_4da9_b1ff_ca2a178d46e0);
pub const MANAGED_APPS_FROMCATEGORY: u32 = 2u32;
pub const MANAGED_APPS_INFOLEVEL_DEFAULT: u32 = 65536u32;
pub const MANAGED_APPS_USERAPPLICATIONS: u32 = 1u32;
pub const MANAGED_APPTYPE_SETUPEXE: u32 = 2u32;
pub const MANAGED_APPTYPE_UNSUPPORTED: u32 = 3u32;
pub const MANAGED_APPTYPE_WINDOWSINSTALLER: u32 = 1u32;
pub const NODEID_Machine: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fc0b737_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_MachineSWSettings: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fc0b73a_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_RSOPMachine: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbd4c1a2e_0b7a_4a62_a6b0_c0577539c97e);
pub const NODEID_RSOPMachineSWSettings: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6a76273e_eb8e_45db_94c5_25663a5f2c1a);
pub const NODEID_RSOPUser: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab87364f_0cec_4cd8_9bf8_898f34628fb8);
pub const NODEID_RSOPUserSWSettings: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe52c5ce3_fd27_4402_84de_d9a5f2858910);
pub const NODEID_User: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fc0b738_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_UserSWSettings: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fc0b73c_a0e1_11d1_a7d3_0000f87571e3);
pub const PI_APPLYPOLICY: u32 = 2u32;
pub const PI_NOUI: u32 = 1u32;
pub const PROGID: INSTALLSPECTYPE = INSTALLSPECTYPE(3i32);
pub const PT_MANDATORY: u32 = 4u32;
pub const PT_ROAMING: u32 = 2u32;
pub const PT_ROAMING_PREEXISTING: u32 = 8u32;
pub const PT_TEMPORARY: u32 = 1u32;
pub const PUBLISHED: APPSTATE = APPSTATE(2i32);
pub const REGISTRY_EXTENSION_GUID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x35378eac_683f_11d2_a89a_00c04fbbcfa2);
pub const RP_FORCE: u32 = 1u32;
pub const RP_SYNC: u32 = 2u32;
pub const RSOPApplied: SETTINGSTATUS = SETTINGSTATUS(1i32);
pub const RSOPFailed: SETTINGSTATUS = SETTINGSTATUS(3i32);
pub const RSOPIgnored: SETTINGSTATUS = SETTINGSTATUS(2i32);
pub const RSOPSubsettingFailed: SETTINGSTATUS = SETTINGSTATUS(4i32);
pub const RSOPUnspecified: SETTINGSTATUS = SETTINGSTATUS(0i32);
pub const RSOP_COMPUTER_ACCESS_DENIED: u32 = 2u32;
pub const RSOP_INFO_FLAG_DIAGNOSTIC_MODE: u32 = 1u32;
pub const RSOP_NO_COMPUTER: u32 = 65536u32;
pub const RSOP_NO_USER: u32 = 131072u32;
pub const RSOP_PLANNING_ASSUME_COMP_WQLFILTER_TRUE: u32 = 16u32;
pub const RSOP_PLANNING_ASSUME_LOOPBACK_MERGE: u32 = 2u32;
pub const RSOP_PLANNING_ASSUME_LOOPBACK_REPLACE: u32 = 4u32;
pub const RSOP_PLANNING_ASSUME_SLOW_LINK: u32 = 1u32;
pub const RSOP_PLANNING_ASSUME_USER_WQLFILTER_TRUE: u32 = 8u32;
pub const RSOP_TEMPNAMESPACE_EXISTS: u32 = 4u32;
pub const RSOP_USER_ACCESS_DENIED: u32 = 1u32;
pub const USER_POLICY_PRESENT_TRIGGER_GUID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x54fb46c8_f089_464c_b1fd_59d1b62c3b50);
pub const backupMostRecent: GPMSearchProperty = GPMSearchProperty(9i32);
pub const gpoComputerExtensions: GPMSearchProperty = GPMSearchProperty(5i32);
pub const gpoDisplayName: GPMSearchProperty = GPMSearchProperty(2i32);
pub const gpoDomain: GPMSearchProperty = GPMSearchProperty(8i32);
pub const gpoEffectivePermissions: GPMSearchProperty = GPMSearchProperty(1i32);
pub const gpoID: GPMSearchProperty = GPMSearchProperty(4i32);
pub const gpoPermissions: GPMSearchProperty = GPMSearchProperty(0i32);
pub const gpoUserExtensions: GPMSearchProperty = GPMSearchProperty(6i32);
pub const gpoWMIFilter: GPMSearchProperty = GPMSearchProperty(3i32);
pub const opContains: GPMSearchOperation = GPMSearchOperation(1i32);
pub const opDestinationByRelativeName: GPMDestinationOption = GPMDestinationOption(2i32);
pub const opDestinationNone: GPMDestinationOption = GPMDestinationOption(1i32);
pub const opDestinationSameAsSource: GPMDestinationOption = GPMDestinationOption(0i32);
pub const opDestinationSet: GPMDestinationOption = GPMDestinationOption(3i32);
pub const opEquals: GPMSearchOperation = GPMSearchOperation(0i32);
pub const opNotContains: GPMSearchOperation = GPMSearchOperation(2i32);
pub const opNotEquals: GPMSearchOperation = GPMSearchOperation(3i32);
pub const opReportComments: GPMReportingOptions = GPMReportingOptions(1i32);
pub const opReportLegacy: GPMReportingOptions = GPMReportingOptions(0i32);
pub const permGPOApply: GPMPermissionType = GPMPermissionType(65536i32);
pub const permGPOCustom: GPMPermissionType = GPMPermissionType(65795i32);
pub const permGPOEdit: GPMPermissionType = GPMPermissionType(65793i32);
pub const permGPOEditSecurityAndDelete: GPMPermissionType = GPMPermissionType(65794i32);
pub const permGPORead: GPMPermissionType = GPMPermissionType(65792i32);
pub const permSOMGPOCreate: GPMPermissionType = GPMPermissionType(1049600i32);
pub const permSOMLink: GPMPermissionType = GPMPermissionType(1835008i32);
pub const permSOMLogging: GPMPermissionType = GPMPermissionType(1573120i32);
pub const permSOMPlanning: GPMPermissionType = GPMPermissionType(1573376i32);
pub const permSOMStarterGPOCreate: GPMPermissionType = GPMPermissionType(1049856i32);
pub const permSOMWMICreate: GPMPermissionType = GPMPermissionType(1049344i32);
pub const permSOMWMIFullControl: GPMPermissionType = GPMPermissionType(1049345i32);
pub const permStarterGPOCustom: GPMPermissionType = GPMPermissionType(197891i32);
pub const permStarterGPOEdit: GPMPermissionType = GPMPermissionType(197889i32);
pub const permStarterGPOFullControl: GPMPermissionType = GPMPermissionType(197890i32);
pub const permStarterGPORead: GPMPermissionType = GPMPermissionType(197888i32);
pub const permWMIFilterCustom: GPMPermissionType = GPMPermissionType(131074i32);
pub const permWMIFilterEdit: GPMPermissionType = GPMPermissionType(131072i32);
pub const permWMIFilterFullControl: GPMPermissionType = GPMPermissionType(131073i32);
pub const repClientHealthRefreshXML: GPMReportType = GPMReportType(5i32);
pub const repClientHealthXML: GPMReportType = GPMReportType(4i32);
pub const repHTML: GPMReportType = GPMReportType(1i32);
pub const repInfraRefreshXML: GPMReportType = GPMReportType(3i32);
pub const repInfraXML: GPMReportType = GPMReportType(2i32);
pub const repXML: GPMReportType = GPMReportType(0i32);
pub const rsopLogging: GPMRSOPMode = GPMRSOPMode(2i32);
pub const rsopPlanning: GPMRSOPMode = GPMRSOPMode(1i32);
pub const rsopUnknown: GPMRSOPMode = GPMRSOPMode(0i32);
pub const somDomain: GPMSOMType = GPMSOMType(1i32);
pub const somLinks: GPMSearchProperty = GPMSearchProperty(7i32);
pub const somOU: GPMSOMType = GPMSOMType(2i32);
pub const somSite: GPMSOMType = GPMSOMType(0i32);
pub const starterGPODisplayName: GPMSearchProperty = GPMSearchProperty(12i32);
pub const starterGPODomain: GPMSearchProperty = GPMSearchProperty(14i32);
pub const starterGPOEffectivePermissions: GPMSearchProperty = GPMSearchProperty(11i32);
pub const starterGPOID: GPMSearchProperty = GPMSearchProperty(13i32);
pub const starterGPOPermissions: GPMSearchProperty = GPMSearchProperty(10i32);
pub const typeComputer: GPMEntryType = GPMEntryType(1i32);
pub const typeCustom: GPMStarterGPOType = GPMStarterGPOType(1i32);
pub const typeGPO: GPMBackupType = GPMBackupType(0i32);
pub const typeGlobalGroup: GPMEntryType = GPMEntryType(3i32);
pub const typeLocalGroup: GPMEntryType = GPMEntryType(2i32);
pub const typeStarterGPO: GPMBackupType = GPMBackupType(1i32);
pub const typeSystem: GPMStarterGPOType = GPMStarterGPOType(0i32);
pub const typeUNCPath: GPMEntryType = GPMEntryType(5i32);
pub const typeUniversalGroup: GPMEntryType = GPMEntryType(4i32);
pub const typeUnknown: GPMEntryType = GPMEntryType(6i32);
pub const typeUser: GPMEntryType = GPMEntryType(0i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APPSTATE(pub i32);
impl ::core::marker::Copy for APPSTATE {}
impl ::core::clone::Clone for APPSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for APPSTATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for APPSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPSTATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GPMBackupType(pub i32);
impl ::core::marker::Copy for GPMBackupType {}
impl ::core::clone::Clone for GPMBackupType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMBackupType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GPMBackupType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GPMBackupType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMBackupType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GPMDestinationOption(pub i32);
impl ::core::marker::Copy for GPMDestinationOption {}
impl ::core::clone::Clone for GPMDestinationOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMDestinationOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GPMDestinationOption {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GPMDestinationOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMDestinationOption").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GPMEntryType(pub i32);
impl ::core::marker::Copy for GPMEntryType {}
impl ::core::clone::Clone for GPMEntryType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMEntryType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GPMEntryType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GPMEntryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMEntryType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GPMPermissionType(pub i32);
impl ::core::marker::Copy for GPMPermissionType {}
impl ::core::clone::Clone for GPMPermissionType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMPermissionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GPMPermissionType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GPMPermissionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMPermissionType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GPMRSOPMode(pub i32);
impl ::core::marker::Copy for GPMRSOPMode {}
impl ::core::clone::Clone for GPMRSOPMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMRSOPMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GPMRSOPMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GPMRSOPMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMRSOPMode").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GPMReportType(pub i32);
impl ::core::marker::Copy for GPMReportType {}
impl ::core::clone::Clone for GPMReportType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMReportType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GPMReportType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GPMReportType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMReportType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GPMReportingOptions(pub i32);
impl ::core::marker::Copy for GPMReportingOptions {}
impl ::core::clone::Clone for GPMReportingOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMReportingOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GPMReportingOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GPMReportingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMReportingOptions").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GPMSOMType(pub i32);
impl ::core::marker::Copy for GPMSOMType {}
impl ::core::clone::Clone for GPMSOMType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMSOMType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GPMSOMType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GPMSOMType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMSOMType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GPMSearchOperation(pub i32);
impl ::core::marker::Copy for GPMSearchOperation {}
impl ::core::clone::Clone for GPMSearchOperation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMSearchOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GPMSearchOperation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GPMSearchOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMSearchOperation").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GPMSearchProperty(pub i32);
impl ::core::marker::Copy for GPMSearchProperty {}
impl ::core::clone::Clone for GPMSearchProperty {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMSearchProperty {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GPMSearchProperty {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GPMSearchProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMSearchProperty").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GPMStarterGPOType(pub i32);
impl ::core::marker::Copy for GPMStarterGPOType {}
impl ::core::clone::Clone for GPMStarterGPOType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPMStarterGPOType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GPMStarterGPOType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GPMStarterGPOType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPMStarterGPOType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GPO_LINK(pub i32);
impl ::core::marker::Copy for GPO_LINK {}
impl ::core::clone::Clone for GPO_LINK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPO_LINK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GPO_LINK {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GPO_LINK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPO_LINK").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GPO_OPEN_FLAGS(pub u32);
impl ::core::marker::Copy for GPO_OPEN_FLAGS {}
impl ::core::clone::Clone for GPO_OPEN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPO_OPEN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GPO_OPEN_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GPO_OPEN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPO_OPEN_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GPO_OPTIONS(pub u32);
impl ::core::marker::Copy for GPO_OPTIONS {}
impl ::core::clone::Clone for GPO_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPO_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GPO_OPTIONS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GPO_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPO_OPTIONS").field(&self.0).finish()
    }
}
impl GPO_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for GPO_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GPO_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GPO_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GPO_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GPO_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GPO_SECTION(pub u32);
impl ::core::marker::Copy for GPO_SECTION {}
impl ::core::clone::Clone for GPO_SECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GPO_SECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GPO_SECTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GPO_SECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPO_SECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GROUP_POLICY_HINT_TYPE(pub i32);
impl ::core::marker::Copy for GROUP_POLICY_HINT_TYPE {}
impl ::core::clone::Clone for GROUP_POLICY_HINT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GROUP_POLICY_HINT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GROUP_POLICY_HINT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GROUP_POLICY_HINT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GROUP_POLICY_HINT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GROUP_POLICY_OBJECT_TYPE(pub i32);
impl ::core::marker::Copy for GROUP_POLICY_OBJECT_TYPE {}
impl ::core::clone::Clone for GROUP_POLICY_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GROUP_POLICY_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GROUP_POLICY_OBJECT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GROUP_POLICY_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GROUP_POLICY_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct INSTALLSPECTYPE(pub i32);
impl ::core::marker::Copy for INSTALLSPECTYPE {}
impl ::core::clone::Clone for INSTALLSPECTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTALLSPECTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for INSTALLSPECTYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for INSTALLSPECTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTALLSPECTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SETTINGSTATUS(pub i32);
impl ::core::marker::Copy for SETTINGSTATUS {}
impl ::core::clone::Clone for SETTINGSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SETTINGSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for SETTINGSTATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SETTINGSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SETTINGSTATUS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GPOBROWSEINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpTitle: ::windows_core::PWSTR,
    pub lpInitialOU: ::windows_core::PWSTR,
    pub lpDSPath: ::windows_core::PWSTR,
    pub dwDSPathSize: u32,
    pub lpName: ::windows_core::PWSTR,
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GPOBROWSEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GPOBROWSEINFO").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("hwndOwner", &self.hwndOwner).field("lpTitle", &self.lpTitle).field("lpInitialOU", &self.lpInitialOU).field("lpDSPath", &self.lpDSPath).field("dwDSPathSize", &self.dwDSPathSize).field("lpName", &self.lpName).field("dwNameSize", &self.dwNameSize).field("gpoType", &self.gpoType).field("gpoHint", &self.gpoHint).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GPOBROWSEINFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GPOBROWSEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.hwndOwner == other.hwndOwner && self.lpTitle == other.lpTitle && self.lpInitialOU == other.lpInitialOU && self.lpDSPath == other.lpDSPath && self.dwDSPathSize == other.dwDSPathSize && self.lpName == other.lpName && self.dwNameSize == other.dwNameSize && self.gpoType == other.gpoType && self.gpoHint == other.gpoHint
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GPOBROWSEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GPOBROWSEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUP_POLICY_OBJECTA {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: ::windows_core::PSTR,
    pub lpFileSysPath: ::windows_core::PSTR,
    pub lpDisplayName: ::windows_core::PSTR,
    pub szGPOName: [u8; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::super::Foundation::LPARAM,
    pub pNext: *mut GROUP_POLICY_OBJECTA,
    pub pPrev: *mut GROUP_POLICY_OBJECTA,
    pub lpExtensions: ::windows_core::PSTR,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lpLink: ::windows_core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUP_POLICY_OBJECTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUP_POLICY_OBJECTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GROUP_POLICY_OBJECTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_POLICY_OBJECTA")
            .field("dwOptions", &self.dwOptions)
            .field("dwVersion", &self.dwVersion)
            .field("lpDSPath", &self.lpDSPath)
            .field("lpFileSysPath", &self.lpFileSysPath)
            .field("lpDisplayName", &self.lpDisplayName)
            .field("szGPOName", &self.szGPOName)
            .field("GPOLink", &self.GPOLink)
            .field("lParam", &self.lParam)
            .field("pNext", &self.pNext)
            .field("pPrev", &self.pPrev)
            .field("lpExtensions", &self.lpExtensions)
            .field("lParam2", &self.lParam2)
            .field("lpLink", &self.lpLink)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GROUP_POLICY_OBJECTA {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_POLICY_OBJECTA {
    fn eq(&self, other: &Self) -> bool {
        self.dwOptions == other.dwOptions && self.dwVersion == other.dwVersion && self.lpDSPath == other.lpDSPath && self.lpFileSysPath == other.lpFileSysPath && self.lpDisplayName == other.lpDisplayName && self.szGPOName == other.szGPOName && self.GPOLink == other.GPOLink && self.lParam == other.lParam && self.pNext == other.pNext && self.pPrev == other.pPrev && self.lpExtensions == other.lpExtensions && self.lParam2 == other.lParam2 && self.lpLink == other.lpLink
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_POLICY_OBJECTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_POLICY_OBJECTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct GROUP_POLICY_OBJECTW {
    pub dwOptions: u32,
    pub dwVersion: u32,
    pub lpDSPath: ::windows_core::PWSTR,
    pub lpFileSysPath: ::windows_core::PWSTR,
    pub lpDisplayName: ::windows_core::PWSTR,
    pub szGPOName: [u16; 50],
    pub GPOLink: GPO_LINK,
    pub lParam: super::super::Foundation::LPARAM,
    pub pNext: *mut GROUP_POLICY_OBJECTW,
    pub pPrev: *mut GROUP_POLICY_OBJECTW,
    pub lpExtensions: ::windows_core::PWSTR,
    pub lParam2: super::super::Foundation::LPARAM,
    pub lpLink: ::windows_core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for GROUP_POLICY_OBJECTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for GROUP_POLICY_OBJECTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GROUP_POLICY_OBJECTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_POLICY_OBJECTW")
            .field("dwOptions", &self.dwOptions)
            .field("dwVersion", &self.dwVersion)
            .field("lpDSPath", &self.lpDSPath)
            .field("lpFileSysPath", &self.lpFileSysPath)
            .field("lpDisplayName", &self.lpDisplayName)
            .field("szGPOName", &self.szGPOName)
            .field("GPOLink", &self.GPOLink)
            .field("lParam", &self.lParam)
            .field("pNext", &self.pNext)
            .field("pPrev", &self.pPrev)
            .field("lpExtensions", &self.lpExtensions)
            .field("lParam2", &self.lParam2)
            .field("lpLink", &self.lpLink)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for GROUP_POLICY_OBJECTW {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GROUP_POLICY_OBJECTW {
    fn eq(&self, other: &Self) -> bool {
        self.dwOptions == other.dwOptions && self.dwVersion == other.dwVersion && self.lpDSPath == other.lpDSPath && self.lpFileSysPath == other.lpFileSysPath && self.lpDisplayName == other.lpDisplayName && self.szGPOName == other.szGPOName && self.GPOLink == other.GPOLink && self.lParam == other.lParam && self.pNext == other.pNext && self.pPrev == other.pPrev && self.lpExtensions == other.lpExtensions && self.lParam2 == other.lParam2 && self.lpLink == other.lpLink
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GROUP_POLICY_OBJECTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GROUP_POLICY_OBJECTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct INSTALLDATA {
    pub Type: INSTALLSPECTYPE,
    pub Spec: INSTALLSPEC,
}
impl ::core::marker::Copy for INSTALLDATA {}
impl ::core::clone::Clone for INSTALLDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for INSTALLDATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for INSTALLDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union INSTALLSPEC {
    pub AppName: INSTALLSPEC_0,
    pub FileExt: ::windows_core::PWSTR,
    pub ProgId: ::windows_core::PWSTR,
    pub COMClass: INSTALLSPEC_1,
}
impl ::core::marker::Copy for INSTALLSPEC {}
impl ::core::clone::Clone for INSTALLSPEC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for INSTALLSPEC {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for INSTALLSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct INSTALLSPEC_0 {
    pub Name: ::windows_core::PWSTR,
    pub GPOId: ::windows_core::GUID,
}
impl ::core::marker::Copy for INSTALLSPEC_0 {}
impl ::core::clone::Clone for INSTALLSPEC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INSTALLSPEC_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSTALLSPEC_0").field("Name", &self.Name).field("GPOId", &self.GPOId).finish()
    }
}
impl ::windows_core::TypeKind for INSTALLSPEC_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for INSTALLSPEC_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.GPOId == other.GPOId
    }
}
impl ::core::cmp::Eq for INSTALLSPEC_0 {}
impl ::core::default::Default for INSTALLSPEC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct INSTALLSPEC_1 {
    pub Clsid: ::windows_core::GUID,
    pub ClsCtx: u32,
}
impl ::core::marker::Copy for INSTALLSPEC_1 {}
impl ::core::clone::Clone for INSTALLSPEC_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INSTALLSPEC_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSTALLSPEC_1").field("Clsid", &self.Clsid).field("ClsCtx", &self.ClsCtx).finish()
    }
}
impl ::windows_core::TypeKind for INSTALLSPEC_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for INSTALLSPEC_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Clsid == other.Clsid && self.ClsCtx == other.ClsCtx
    }
}
impl ::core::cmp::Eq for INSTALLSPEC_1 {}
impl ::core::default::Default for INSTALLSPEC_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct LOCALMANAGEDAPPLICATION {
    pub pszDeploymentName: ::windows_core::PWSTR,
    pub pszPolicyName: ::windows_core::PWSTR,
    pub pszProductId: ::windows_core::PWSTR,
    pub dwState: u32,
}
impl ::core::marker::Copy for LOCALMANAGEDAPPLICATION {}
impl ::core::clone::Clone for LOCALMANAGEDAPPLICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOCALMANAGEDAPPLICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCALMANAGEDAPPLICATION").field("pszDeploymentName", &self.pszDeploymentName).field("pszPolicyName", &self.pszPolicyName).field("pszProductId", &self.pszProductId).field("dwState", &self.dwState).finish()
    }
}
impl ::windows_core::TypeKind for LOCALMANAGEDAPPLICATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for LOCALMANAGEDAPPLICATION {
    fn eq(&self, other: &Self) -> bool {
        self.pszDeploymentName == other.pszDeploymentName && self.pszPolicyName == other.pszPolicyName && self.pszProductId == other.pszProductId && self.dwState == other.dwState
    }
}
impl ::core::cmp::Eq for LOCALMANAGEDAPPLICATION {}
impl ::core::default::Default for LOCALMANAGEDAPPLICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct MANAGEDAPPLICATION {
    pub pszPackageName: ::windows_core::PWSTR,
    pub pszPublisher: ::windows_core::PWSTR,
    pub dwVersionHi: u32,
    pub dwVersionLo: u32,
    pub dwRevision: u32,
    pub GpoId: ::windows_core::GUID,
    pub pszPolicyName: ::windows_core::PWSTR,
    pub ProductId: ::windows_core::GUID,
    pub Language: u16,
    pub pszOwner: ::windows_core::PWSTR,
    pub pszCompany: ::windows_core::PWSTR,
    pub pszComments: ::windows_core::PWSTR,
    pub pszContact: ::windows_core::PWSTR,
    pub pszSupportUrl: ::windows_core::PWSTR,
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MANAGEDAPPLICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MANAGEDAPPLICATION")
            .field("pszPackageName", &self.pszPackageName)
            .field("pszPublisher", &self.pszPublisher)
            .field("dwVersionHi", &self.dwVersionHi)
            .field("dwVersionLo", &self.dwVersionLo)
            .field("dwRevision", &self.dwRevision)
            .field("GpoId", &self.GpoId)
            .field("pszPolicyName", &self.pszPolicyName)
            .field("ProductId", &self.ProductId)
            .field("Language", &self.Language)
            .field("pszOwner", &self.pszOwner)
            .field("pszCompany", &self.pszCompany)
            .field("pszComments", &self.pszComments)
            .field("pszContact", &self.pszContact)
            .field("pszSupportUrl", &self.pszSupportUrl)
            .field("dwPathType", &self.dwPathType)
            .field("bInstalled", &self.bInstalled)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for MANAGEDAPPLICATION {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MANAGEDAPPLICATION {
    fn eq(&self, other: &Self) -> bool {
        self.pszPackageName == other.pszPackageName && self.pszPublisher == other.pszPublisher && self.dwVersionHi == other.dwVersionHi && self.dwVersionLo == other.dwVersionLo && self.dwRevision == other.dwRevision && self.GpoId == other.GpoId && self.pszPolicyName == other.pszPolicyName && self.ProductId == other.ProductId && self.Language == other.Language && self.pszOwner == other.pszOwner && self.pszCompany == other.pszCompany && self.pszComments == other.pszComments && self.pszContact == other.pszContact && self.pszSupportUrl == other.pszSupportUrl && self.dwPathType == other.dwPathType && self.bInstalled == other.bInstalled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MANAGEDAPPLICATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MANAGEDAPPLICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct POLICYSETTINGSTATUSINFO {
    pub szKey: ::windows_core::PWSTR,
    pub szEventSource: ::windows_core::PWSTR,
    pub szEventLogName: ::windows_core::PWSTR,
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLICYSETTINGSTATUSINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLICYSETTINGSTATUSINFO").field("szKey", &self.szKey).field("szEventSource", &self.szEventSource).field("szEventLogName", &self.szEventLogName).field("dwEventID", &self.dwEventID).field("dwErrorCode", &self.dwErrorCode).field("status", &self.status).field("timeLogged", &self.timeLogged).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for POLICYSETTINGSTATUSINFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLICYSETTINGSTATUSINFO {
    fn eq(&self, other: &Self) -> bool {
        self.szKey == other.szKey && self.szEventSource == other.szEventSource && self.szEventLogName == other.szEventLogName && self.dwEventID == other.dwEventID && self.dwErrorCode == other.dwErrorCode && self.status == other.status && self.timeLogged == other.timeLogged
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLICYSETTINGSTATUSINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLICYSETTINGSTATUSINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Wmi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
pub struct RSOP_TARGET {
    pub pwszAccountName: ::windows_core::PWSTR,
    pub pwszNewSOM: ::windows_core::PWSTR,
    pub psaSecurityGroups: *mut super::Com::SAFEARRAY,
    pub pRsopToken: *mut ::core::ffi::c_void,
    pub pGPOList: *mut GROUP_POLICY_OBJECTA,
    pub pWbemServices: ::std::mem::ManuallyDrop<::core::option::Option<super::Wmi::IWbemServices>>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl ::core::clone::Clone for RSOP_TARGET {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl ::core::fmt::Debug for RSOP_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RSOP_TARGET").field("pwszAccountName", &self.pwszAccountName).field("pwszNewSOM", &self.pwszNewSOM).field("psaSecurityGroups", &self.psaSecurityGroups).field("pRsopToken", &self.pRsopToken).field("pGPOList", &self.pGPOList).field("pWbemServices", &self.pWbemServices).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl ::windows_core::TypeKind for RSOP_TARGET {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl ::core::cmp::PartialEq for RSOP_TARGET {
    fn eq(&self, other: &Self) -> bool {
        self.pwszAccountName == other.pwszAccountName && self.pwszNewSOM == other.pwszNewSOM && self.psaSecurityGroups == other.psaSecurityGroups && self.pRsopToken == other.pRsopToken && self.pGPOList == other.pGPOList && self.pWbemServices == other.pWbemServices
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl ::core::cmp::Eq for RSOP_TARGET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
impl ::core::default::Default for RSOP_TARGET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Wmi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Wmi"))]
pub type PFNGENERATEGROUPPOLICY = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, pbabort: *mut super::super::Foundation::BOOL, pwszsite: ::windows_core::PCWSTR, pcomputertarget: *const RSOP_TARGET, pusertarget: *const RSOP_TARGET) -> u32>;
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub type PFNPROCESSGROUPPOLICY = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, htoken: super::super::Foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut super::super::Foundation::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK) -> u32>;
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`, `\"Win32_System_Wmi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_System_Wmi"))]
pub type PFNPROCESSGROUPPOLICYEX = ::core::option::Option<unsafe extern "system" fn(dwflags: u32, htoken: super::super::Foundation::HANDLE, hkeyroot: super::Registry::HKEY, pdeletedgpolist: *const GROUP_POLICY_OBJECTA, pchangedgpolist: *const GROUP_POLICY_OBJECTA, phandle: usize, pbabort: *mut super::super::Foundation::BOOL, pstatuscallback: PFNSTATUSMESSAGECALLBACK, pwbemservices: ::core::option::Option<super::Wmi::IWbemServices>, prsopstatus: *mut ::windows_core::HRESULT) -> u32>;
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNSTATUSMESSAGECALLBACK = ::core::option::Option<unsafe extern "system" fn(bverbose: super::super::Foundation::BOOL, lpmessage: ::windows_core::PCWSTR) -> u32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
