#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn HlinkClone<P0, P2>(pihl: P0, riid: *const super::IID, pihlsiteforclone: P2, dwsitedata: super::DWORD, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<IHlink>,
    P2: windows_core::Param<IHlinkSite>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkClone(pihl : *mut core::ffi::c_void, riid : *const super::IID, pihlsiteforclone : *mut core::ffi::c_void, dwsitedata : super::DWORD, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkClone(pihl.param().abi(), riid, pihlsiteforclone.param().abi(), dwsitedata, ppvobj as _) }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn HlinkCreateBrowseContext<P0>(piunkouter: P0, riid: *const super::IID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkCreateBrowseContext(piunkouter : *mut core::ffi::c_void, riid : *const super::IID, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkCreateBrowseContext(piunkouter.param().abi(), riid, ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "windef"))]
#[inline]
pub unsafe fn HlinkCreateExtensionServices<P0, P2, P3, P4>(pwzadditionalheaders: P0, phwnd: super::HWND, pszusername: P2, pszpassword: P3, piunkouter: P4, riid: *const super::IID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkCreateExtensionServices(pwzadditionalheaders : windows_core::PCWSTR, phwnd : super::HWND, pszusername : windows_core::PCWSTR, pszpassword : windows_core::PCWSTR, piunkouter : *mut core::ffi::c_void, riid : *const super::IID, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkCreateExtensionServices(pwzadditionalheaders.param().abi(), phwnd as _, pszusername.param().abi(), pszpassword.param().abi(), piunkouter.param().abi(), riid, ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn HlinkCreateFromData<P0, P1, P3>(pidataobj: P0, pihlsite: P1, dwsitedata: super::DWORD, piunkouter: P3, riid: *const super::IID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDataObject>,
    P1: windows_core::Param<IHlinkSite>,
    P3: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkCreateFromData(pidataobj : *mut core::ffi::c_void, pihlsite : *mut core::ffi::c_void, dwsitedata : super::DWORD, piunkouter : *mut core::ffi::c_void, riid : *const super::IID, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkCreateFromData(pidataobj.param().abi(), pihlsite.param().abi(), dwsitedata, piunkouter.param().abi(), riid, ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn HlinkCreateFromMoniker<P0, P1, P2, P3, P5>(pimktrgt: P0, pwzlocation: P1, pwzfriendlyname: P2, pihlsite: P3, dwsitedata: super::DWORD, piunkouter: P5, riid: *const super::IID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IMoniker>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<IHlinkSite>,
    P5: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkCreateFromMoniker(pimktrgt : *mut core::ffi::c_void, pwzlocation : windows_core::PCWSTR, pwzfriendlyname : windows_core::PCWSTR, pihlsite : *mut core::ffi::c_void, dwsitedata : super::DWORD, piunkouter : *mut core::ffi::c_void, riid : *const super::IID, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkCreateFromMoniker(pimktrgt.param().abi(), pwzlocation.param().abi(), pwzfriendlyname.param().abi(), pihlsite.param().abi(), dwsitedata, piunkouter.param().abi(), riid, ppvobj as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn HlinkCreateFromString<P0, P1, P2, P3, P5>(pwztarget: P0, pwzlocation: P1, pwzfriendlyname: P2, pihlsite: P3, dwsitedata: super::DWORD, piunkouter: P5, riid: *const super::IID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<IHlinkSite>,
    P5: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkCreateFromString(pwztarget : windows_core::PCWSTR, pwzlocation : windows_core::PCWSTR, pwzfriendlyname : windows_core::PCWSTR, pihlsite : *mut core::ffi::c_void, dwsitedata : super::DWORD, piunkouter : *mut core::ffi::c_void, riid : *const super::IID, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkCreateFromString(pwztarget.param().abi(), pwzlocation.param().abi(), pwzfriendlyname.param().abi(), pihlsite.param().abi(), dwsitedata, piunkouter.param().abi(), riid, ppvobj as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn HlinkCreateShortcut<P1, P2, P3>(grfhlshortcutf: super::DWORD, pihl: P1, pwzdir: P2, pwzfilename: P3, ppwzshortcutfile: *mut windows_core::PWSTR, dwreserved: super::DWORD) -> windows_core::HRESULT
where
    P1: windows_core::Param<IHlink>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkCreateShortcut(grfhlshortcutf : super::DWORD, pihl : *mut core::ffi::c_void, pwzdir : windows_core::PCWSTR, pwzfilename : windows_core::PCWSTR, ppwzshortcutfile : *mut windows_core::PWSTR, dwreserved : super::DWORD) -> windows_core::HRESULT);
    unsafe { HlinkCreateShortcut(grfhlshortcutf, pihl.param().abi(), pwzdir.param().abi(), pwzfilename.param().abi(), ppwzshortcutfile as _, dwreserved) }
}
#[cfg(all(feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn HlinkCreateShortcutFromMoniker<P1, P2, P3, P4>(grfhlshortcutf: super::DWORD, pimktarget: P1, pwzlocation: P2, pwzdir: P3, pwzfilename: P4, ppwzshortcutfile: *mut windows_core::PWSTR, dwreserved: super::DWORD) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::IMoniker>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkCreateShortcutFromMoniker(grfhlshortcutf : super::DWORD, pimktarget : *mut core::ffi::c_void, pwzlocation : windows_core::PCWSTR, pwzdir : windows_core::PCWSTR, pwzfilename : windows_core::PCWSTR, ppwzshortcutfile : *mut windows_core::PWSTR, dwreserved : super::DWORD) -> windows_core::HRESULT);
    unsafe { HlinkCreateShortcutFromMoniker(grfhlshortcutf, pimktarget.param().abi(), pwzlocation.param().abi(), pwzdir.param().abi(), pwzfilename.param().abi(), ppwzshortcutfile as _, dwreserved) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn HlinkCreateShortcutFromString<P1, P2, P3, P4>(grfhlshortcutf: super::DWORD, pwztarget: P1, pwzlocation: P2, pwzdir: P3, pwzfilename: P4, ppwzshortcutfile: *mut windows_core::PWSTR, dwreserved: super::DWORD) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkCreateShortcutFromString(grfhlshortcutf : super::DWORD, pwztarget : windows_core::PCWSTR, pwzlocation : windows_core::PCWSTR, pwzdir : windows_core::PCWSTR, pwzfilename : windows_core::PCWSTR, ppwzshortcutfile : *mut windows_core::PWSTR, dwreserved : super::DWORD) -> windows_core::HRESULT);
    unsafe { HlinkCreateShortcutFromString(grfhlshortcutf, pwztarget.param().abi(), pwzlocation.param().abi(), pwzdir.param().abi(), pwzfilename.param().abi(), ppwzshortcutfile as _, dwreserved) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn HlinkGetSpecialReference(ureference: super::ULONG, ppwzreference: *mut windows_core::PWSTR) -> windows_core::HRESULT {
    windows_core::link!("hlink.dll" "system" fn HlinkGetSpecialReference(ureference : super::ULONG, ppwzreference : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HlinkGetSpecialReference(ureference, ppwzreference as _) }
}
#[inline]
pub unsafe fn HlinkGetValueFromParams<P0, P1>(pwzparams: P0, pwzname: P1, ppwzvalue: *mut windows_core::PWSTR) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkGetValueFromParams(pwzparams : windows_core::PCWSTR, pwzname : windows_core::PCWSTR, ppwzvalue : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HlinkGetValueFromParams(pwzparams.param().abi(), pwzname.param().abi(), ppwzvalue as _) }
}
#[inline]
pub unsafe fn HlinkIsShortcut<P0>(pwzfilename: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkIsShortcut(pwzfilename : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HlinkIsShortcut(pwzfilename.param().abi()) }
}
#[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon"))]
#[inline]
pub unsafe fn HlinkNavigate<P0, P1, P3, P4, P5>(pihl: P0, pihlframe: P1, grfhlnf: super::DWORD, pbc: P3, pibsc: P4, pihlbc: P5) -> windows_core::HRESULT
where
    P0: windows_core::Param<IHlink>,
    P1: windows_core::Param<IHlinkFrame>,
    P3: windows_core::Param<super::LPBC>,
    P4: windows_core::Param<super::IBindStatusCallback>,
    P5: windows_core::Param<IHlinkBrowseContext>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkNavigate(pihl : *mut core::ffi::c_void, pihlframe : *mut core::ffi::c_void, grfhlnf : super::DWORD, pbc : *mut core::ffi::c_void, pibsc : *mut core::ffi::c_void, pihlbc : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkNavigate(pihl.param().abi(), pihlframe.param().abi(), grfhlnf, pbc.param().abi(), pibsc.param().abi(), pihlbc.param().abi()) }
}
#[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon"))]
#[inline]
pub unsafe fn HlinkNavigateToStringReference<P0, P1, P2, P4, P6, P7, P8>(pwztarget: P0, pwzlocation: P1, pihlsite: P2, dwsitedata: super::DWORD, pihlframe: P4, grfhlnf: super::DWORD, pibc: P6, pibsc: P7, pihlbc: P8) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<IHlinkSite>,
    P4: windows_core::Param<IHlinkFrame>,
    P6: windows_core::Param<super::LPBC>,
    P7: windows_core::Param<super::IBindStatusCallback>,
    P8: windows_core::Param<IHlinkBrowseContext>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkNavigateToStringReference(pwztarget : windows_core::PCWSTR, pwzlocation : windows_core::PCWSTR, pihlsite : *mut core::ffi::c_void, dwsitedata : super::DWORD, pihlframe : *mut core::ffi::c_void, grfhlnf : super::DWORD, pibc : *mut core::ffi::c_void, pibsc : *mut core::ffi::c_void, pihlbc : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkNavigateToStringReference(pwztarget.param().abi(), pwzlocation.param().abi(), pihlsite.param().abi(), dwsitedata, pihlframe.param().abi(), grfhlnf, pibc.param().abi(), pibsc.param().abi(), pihlbc.param().abi()) }
}
#[cfg(all(feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn HlinkOnNavigate<P0, P1, P3, P4, P5>(pihlframe: P0, pihlbc: P1, grfhlnf: super::DWORD, pimktarget: P3, pwzlocation: P4, pwzfriendlyname: P5, puhlid: *mut super::ULONG) -> windows_core::HRESULT
where
    P0: windows_core::Param<IHlinkFrame>,
    P1: windows_core::Param<IHlinkBrowseContext>,
    P3: windows_core::Param<super::IMoniker>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkOnNavigate(pihlframe : *mut core::ffi::c_void, pihlbc : *mut core::ffi::c_void, grfhlnf : super::DWORD, pimktarget : *mut core::ffi::c_void, pwzlocation : windows_core::PCWSTR, pwzfriendlyname : windows_core::PCWSTR, puhlid : *mut super::ULONG) -> windows_core::HRESULT);
    unsafe { HlinkOnNavigate(pihlframe.param().abi(), pihlbc.param().abi(), grfhlnf, pimktarget.param().abi(), pwzlocation.param().abi(), pwzfriendlyname.param().abi(), puhlid as _) }
}
#[cfg(all(feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn HlinkOnRenameDocument<P1, P2, P3>(dwreserved: super::DWORD, pihlbc: P1, pimkold: P2, pimknew: P3) -> windows_core::HRESULT
where
    P1: windows_core::Param<IHlinkBrowseContext>,
    P2: windows_core::Param<super::IMoniker>,
    P3: windows_core::Param<super::IMoniker>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkOnRenameDocument(dwreserved : super::DWORD, pihlbc : *mut core::ffi::c_void, pimkold : *mut core::ffi::c_void, pimknew : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkOnRenameDocument(dwreserved, pihlbc.param().abi(), pimkold.param().abi(), pimknew.param().abi()) }
}
#[cfg(all(feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn HlinkParseDisplayName<P0, P1>(pibc: P0, pwzdisplayname: P1, fnoforceabs: bool, pccheaten: *mut super::ULONG, ppimk: *mut Option<super::IMoniker>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPBC>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkParseDisplayName(pibc : *mut core::ffi::c_void, pwzdisplayname : windows_core::PCWSTR, fnoforceabs : windows_core::BOOL, pccheaten : *mut super::ULONG, ppimk : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkParseDisplayName(pibc.param().abi(), pwzdisplayname.param().abi(), fnoforceabs.into(), pccheaten as _, ppimk as _) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn HlinkPreprocessMoniker<P0, P1>(pibc: P0, pimkin: P1, ppimkout: *mut Option<super::IMoniker>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPBC>,
    P1: windows_core::Param<super::IMoniker>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkPreprocessMoniker(pibc : *mut core::ffi::c_void, pimkin : *mut core::ffi::c_void, ppimkout : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkPreprocessMoniker(pibc.param().abi(), pimkin.param().abi(), ppimkout as _) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn HlinkQueryCreateFromData<P0>(pidataobj: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IDataObject>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkQueryCreateFromData(pidataobj : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkQueryCreateFromData(pidataobj.param().abi()) }
}
#[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon", feature = "wtypes"))]
#[inline]
pub unsafe fn HlinkResolveMonikerForData<P0, P2, P5, P6>(pimkreference: P0, reserved: super::DWORD, pibc: P2, cfmtetc: super::ULONG, rgfmtetc: *mut super::FORMATETC, pibsc: P5, pimkbase: P6) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::LPMONIKER>,
    P2: windows_core::Param<super::LPBC>,
    P5: windows_core::Param<super::IBindStatusCallback>,
    P6: windows_core::Param<super::LPMONIKER>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkResolveMonikerForData(pimkreference : *mut core::ffi::c_void, reserved : super::DWORD, pibc : *mut core::ffi::c_void, cfmtetc : super::ULONG, rgfmtetc : *mut super::FORMATETC, pibsc : *mut core::ffi::c_void, pimkbase : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkResolveMonikerForData(pimkreference.param().abi(), reserved, pibc.param().abi(), cfmtetc, rgfmtetc as _, pibsc.param().abi(), pimkbase.param().abi()) }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
#[inline]
pub unsafe fn HlinkResolveShortcut<P0, P1, P3>(pwzshortcutfilename: P0, pihlsite: P1, dwsitedata: super::DWORD, piunkouter: P3, riid: *const super::IID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<IHlinkSite>,
    P3: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkResolveShortcut(pwzshortcutfilename : windows_core::PCWSTR, pihlsite : *mut core::ffi::c_void, dwsitedata : super::DWORD, piunkouter : *mut core::ffi::c_void, riid : *const super::IID, ppvobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkResolveShortcut(pwzshortcutfilename.param().abi(), pihlsite.param().abi(), dwsitedata, piunkouter.param().abi(), riid, ppvobj as _) }
}
#[cfg(feature = "objidl")]
#[inline]
pub unsafe fn HlinkResolveShortcutToMoniker<P0>(pwzshortcutfilename: P0, ppimktarget: *mut Option<super::IMoniker>, ppwzlocation: *mut windows_core::PWSTR) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkResolveShortcutToMoniker(pwzshortcutfilename : windows_core::PCWSTR, ppimktarget : *mut *mut core::ffi::c_void, ppwzlocation : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HlinkResolveShortcutToMoniker(pwzshortcutfilename.param().abi(), ppimktarget as _, ppwzlocation as _) }
}
#[inline]
pub unsafe fn HlinkResolveShortcutToString<P0>(pwzshortcutfilename: P0, ppwztarget: *mut windows_core::PWSTR, ppwzlocation: *mut windows_core::PWSTR) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkResolveShortcutToString(pwzshortcutfilename : windows_core::PCWSTR, ppwztarget : *mut windows_core::PWSTR, ppwzlocation : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HlinkResolveShortcutToString(pwzshortcutfilename.param().abi(), ppwztarget as _, ppwzlocation as _) }
}
#[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon", feature = "wtypes"))]
#[inline]
pub unsafe fn HlinkResolveStringForData<P0, P2, P5, P6>(pwzreference: P0, reserved: super::DWORD, pibc: P2, cfmtetc: super::ULONG, rgfmtetc: *mut super::FORMATETC, pibsc: P5, pimkbase: P6) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::LPBC>,
    P5: windows_core::Param<super::IBindStatusCallback>,
    P6: windows_core::Param<super::LPMONIKER>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkResolveStringForData(pwzreference : windows_core::PCWSTR, reserved : super::DWORD, pibc : *mut core::ffi::c_void, cfmtetc : super::ULONG, rgfmtetc : *mut super::FORMATETC, pibsc : *mut core::ffi::c_void, pimkbase : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { HlinkResolveStringForData(pwzreference.param().abi(), reserved, pibc.param().abi(), cfmtetc, rgfmtetc as _, pibsc.param().abi(), pimkbase.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn HlinkSetSpecialReference<P1>(ureference: super::ULONG, pwzreference: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkSetSpecialReference(ureference : super::ULONG, pwzreference : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HlinkSetSpecialReference(ureference, pwzreference.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn HlinkTranslateURL<P0>(pwzurl: P0, grfflags: super::DWORD, ppwztranslatedurl: *mut windows_core::PWSTR) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkTranslateURL(pwzurl : windows_core::PCWSTR, grfflags : super::DWORD, ppwztranslatedurl : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { HlinkTranslateURL(pwzurl.param().abi(), grfflags, ppwztranslatedurl as _) }
}
#[cfg(all(feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn HlinkUpdateStackItem<P0, P1, P3, P4, P5>(pihlframe: P0, pihlbc: P1, uhlid: super::ULONG, pimktrgt: P3, pwzlocation: P4, pwzfriendlyname: P5) -> windows_core::HRESULT
where
    P0: windows_core::Param<IHlinkFrame>,
    P1: windows_core::Param<IHlinkBrowseContext>,
    P3: windows_core::Param<super::IMoniker>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("hlink.dll" "system" fn HlinkUpdateStackItem(pihlframe : *mut core::ffi::c_void, pihlbc : *mut core::ffi::c_void, uhlid : super::ULONG, pimktrgt : *mut core::ffi::c_void, pwzlocation : windows_core::PCWSTR, pwzfriendlyname : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { HlinkUpdateStackItem(pihlframe.param().abi(), pihlbc.param().abi(), uhlid, pimktrgt.param().abi(), pwzlocation.param().abi(), pwzfriendlyname.param().abi()) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn OleSaveToStreamEx<P0, P1>(piunk: P0, pistm: P1, fcleardirty: bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::IStream>,
{
    windows_core::link!("hlink.dll" "system" fn OleSaveToStreamEx(piunk : *mut core::ffi::c_void, pistm : *mut core::ffi::c_void, fcleardirty : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { OleSaveToStreamEx(piunk.param().abi(), pistm.param().abi(), fcleardirty.into()) }
}
pub const CFSTR_HYPERLINK: windows_core::PCSTR = windows_core::s!("Hyperlink");
pub const HLBWIF_DOCWNDMAXIMIZED: i32 = 8;
pub const HLBWIF_FRAMEWNDMAXIMIZED: i32 = 4;
pub const HLBWIF_HASDOCWNDINFO: i32 = 2;
pub const HLBWIF_HASFRAMEWNDINFO: i32 = 1;
pub const HLBWIF_HASWEBTOOLBARINFO: i32 = 16;
pub const HLBWIF_WEBTOOLBARHIDDEN: i32 = 32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HLBWINFO {
    pub cbSize: super::ULONG,
    pub grfHLBWIF: super::DWORD,
    pub rcFramePos: super::RECT,
    pub rcDocPos: super::RECT,
    pub hltbinfo: HLTBINFO,
}
pub type HLFNAMEF = i32;
pub const HLFNAMEF_DEFAULT: HLFNAMEF = 0;
pub const HLFNAMEF_TRYCACHE: HLFNAMEF = 1;
pub const HLFNAMEF_TRYFULLTARGET: HLFNAMEF = 4;
pub const HLFNAMEF_TRYPRETTYTARGET: HLFNAMEF = 2;
pub const HLFNAMEF_TRYWIN95SHORTCUT: HLFNAMEF = 8;
pub const HLID_CURRENT: i32 = -3;
pub const HLID_INVALID: i32 = 0;
pub const HLID_NEXT: i32 = -2;
pub const HLID_PREVIOUS: i32 = -1;
pub const HLID_STACKBOTTOM: i32 = -4;
pub const HLID_STACKTOP: i32 = -5;
pub type HLINKGETREF = i32;
pub const HLINKGETREF_ABSOLUTE: HLINKGETREF = 1;
pub const HLINKGETREF_DEFAULT: HLINKGETREF = 0;
pub const HLINKGETREF_RELATIVE: HLINKGETREF = 2;
pub type HLINKMISC = i32;
pub const HLINKMISC_RELATIVE: HLINKMISC = 1;
pub type HLINKSETF = i32;
pub const HLINKSETF_LOCATION: HLINKSETF = 2;
pub const HLINKSETF_TARGET: HLINKSETF = 1;
pub type HLINKWHICHMK = i32;
pub const HLINKWHICHMK_BASE: HLINKWHICHMK = 2;
pub const HLINKWHICHMK_CONTAINER: HLINKWHICHMK = 1;
pub const HLINK_E_FIRST: windows_core::HRESULT = windows_core::HRESULT(0x80040100_u32 as _);
pub const HLINK_S_DONTHIDE: windows_core::HRESULT = windows_core::HRESULT(0x40100_u32 as _);
pub const HLINK_S_FIRST: windows_core::HRESULT = windows_core::HRESULT(0x40100_u32 as _);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HLITEM {
    pub uHLID: super::ULONG,
    pub pwzFriendlyName: windows_core::PWSTR,
}
pub type HLNF = i32;
pub const HLNF_CREATENOHISTORY: HLNF = 32;
pub const HLNF_INTERNALJUMP: HLNF = 1;
pub const HLNF_NAVIGATINGBACK: HLNF = 4;
pub const HLNF_NAVIGATINGFORWARD: HLNF = 8;
pub const HLNF_NAVIGATINGTOSTACKITEM: HLNF = 16;
pub const HLNF_OPENINNEWWINDOW: HLNF = 2;
pub const HLQF_ISCURRENT: i32 = 2;
pub const HLQF_ISVALID: i32 = 1;
pub type HLSHORTCUTF = i32;
pub const HLSHORTCUTF_DEFAULT: HLSHORTCUTF = 0;
pub const HLSHORTCUTF_DONTACTUALLYCREATE: HLSHORTCUTF = 1;
pub const HLSHORTCUTF_MAYUSEEXISTINGSHORTCUT: HLSHORTCUTF = 8;
pub const HLSHORTCUTF_USEFILENAMEFROMFRIENDLYNAME: HLSHORTCUTF = 2;
pub const HLSHORTCUTF_USEUNIQUEFILENAME: HLSHORTCUTF = 4;
pub type HLSR = i32;
pub const HLSR_HISTORYFOLDER: HLSR = 2;
pub const HLSR_HOME: HLSR = 0;
pub const HLSR_SEARCHPAGE: HLSR = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HLTBINFO {
    pub uDockType: super::ULONG,
    pub rcTbPos: super::RECT,
}
pub const HLTB_DOCKEDBOTTOM: i32 = 3;
pub const HLTB_DOCKEDLEFT: i32 = 0;
pub const HLTB_DOCKEDRIGHT: i32 = 2;
pub const HLTB_DOCKEDTOP: i32 = 1;
pub const HLTB_FLOATING: i32 = 4;
pub type HLTRANSLATEF = i32;
pub const HLTRANSLATEF_DEFAULT: HLTRANSLATEF = 0;
pub const HLTRANSLATEF_DONTAPPLYDEFAULTPREFIX: HLTRANSLATEF = 1;
windows_core::imp::define_interface!(IEnumHLITEM, IEnumHLITEM_Vtbl, 0x79eac9c6_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IEnumHLITEM, windows_core::IUnknown);
impl IEnumHLITEM {
    #[cfg(feature = "shared")]
    pub unsafe fn Next(&self, celt: super::ULONG, rgelt: *mut HLITEM, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), celt, rgelt as _, pceltfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Skip(&self, celt: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self, ppienumhlitem: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), core::mem::transmute(ppienumhlitem)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumHLITEM_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut HLITEM, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Next: usize,
    #[cfg(feature = "shared")]
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Skip: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IEnumHLITEM_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: super::ULONG, rgelt: *mut HLITEM, pceltfetched: *mut super::ULONG) -> windows_core::Result<()>;
    fn Skip(&self, celt: super::ULONG) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self, ppienumhlitem: windows_core::OutRef<IEnumHLITEM>) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IEnumHLITEM_Vtbl {
    pub const fn new<Identity: IEnumHLITEM_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumHLITEM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG, rgelt: *mut HLITEM, pceltfetched: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumHLITEM_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumHLITEM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumHLITEM_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumHLITEM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumHLITEM_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumHLITEM_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppienumhlitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumHLITEM_Impl::Clone(this, core::mem::transmute_copy(&ppienumhlitem)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumHLITEM as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IEnumHLITEM {}
windows_core::imp::define_interface!(IExtensionServices, IExtensionServices_Vtbl, 0x79eac9cb_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IExtensionServices, windows_core::IUnknown);
impl IExtensionServices {
    pub unsafe fn SetAdditionalHeaders<P0>(&self, pwzadditionalheaders: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAdditionalHeaders)(windows_core::Interface::as_raw(self), pwzadditionalheaders.param().abi()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetAuthenticateData<P1, P2>(&self, phwnd: super::HWND, pwzusername: P1, pwzpassword: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAuthenticateData)(windows_core::Interface::as_raw(self), phwnd, pwzusername.param().abi(), pwzpassword.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtensionServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAdditionalHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetAuthenticateData: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetAuthenticateData: usize,
}
#[cfg(feature = "windef")]
pub trait IExtensionServices_Impl: windows_core::IUnknownImpl {
    fn SetAdditionalHeaders(&self, pwzadditionalheaders: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetAuthenticateData(&self, phwnd: super::HWND, pwzusername: &windows_core::PCWSTR, pwzpassword: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IExtensionServices_Vtbl {
    pub const fn new<Identity: IExtensionServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAdditionalHeaders<Identity: IExtensionServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzadditionalheaders: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExtensionServices_Impl::SetAdditionalHeaders(this, core::mem::transmute(&pwzadditionalheaders)).into()
            }
        }
        unsafe extern "system" fn SetAuthenticateData<Identity: IExtensionServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: super::HWND, pwzusername: windows_core::PCWSTR, pwzpassword: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExtensionServices_Impl::SetAuthenticateData(this, core::mem::transmute_copy(&phwnd), core::mem::transmute(&pwzusername), core::mem::transmute(&pwzpassword)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAdditionalHeaders: SetAdditionalHeaders::<Identity, OFFSET>,
            SetAuthenticateData: SetAuthenticateData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExtensionServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IExtensionServices {}
windows_core::imp::define_interface!(IHlink, IHlink_Vtbl, 0x79eac9c3_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IHlink, windows_core::IUnknown);
impl IHlink {
    #[cfg(feature = "shared")]
    pub unsafe fn SetHlinkSite<P0>(&self, pihlsite: P0, dwsitedata: super::DWORD) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IHlinkSite>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHlinkSite)(windows_core::Interface::as_raw(self), pihlsite.param().abi(), dwsitedata) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetHlinkSite(&self, ppihlsite: *mut Option<IHlinkSite>, pdwsitedata: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHlinkSite)(windows_core::Interface::as_raw(self), core::mem::transmute(ppihlsite), pdwsitedata as _) }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn SetMonikerReference<P1, P2>(&self, grfhlsetf: super::DWORD, pimktarget: P1, pwzlocation: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMoniker>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMonikerReference)(windows_core::Interface::as_raw(self), grfhlsetf, pimktarget.param().abi(), pwzlocation.param().abi()) }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn GetMonikerReference(&self, dwwhichref: super::DWORD, ppimktarget: Option<*mut Option<super::IMoniker>>, ppwzlocation: Option<*mut windows_core::PWSTR>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMonikerReference)(windows_core::Interface::as_raw(self), dwwhichref, ppimktarget.unwrap_or(core::mem::zeroed()) as _, ppwzlocation.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetStringReference<P1, P2>(&self, grfhlsetf: super::DWORD, pwztarget: P1, pwzlocation: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStringReference)(windows_core::Interface::as_raw(self), grfhlsetf, pwztarget.param().abi(), pwzlocation.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetStringReference(&self, dwwhichref: super::DWORD, ppwztarget: Option<*mut windows_core::PWSTR>, ppwzlocation: Option<*mut windows_core::PWSTR>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStringReference)(windows_core::Interface::as_raw(self), dwwhichref, ppwztarget.unwrap_or(core::mem::zeroed()) as _, ppwzlocation.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetFriendlyName<P0>(&self, pwzfriendlyname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFriendlyName)(windows_core::Interface::as_raw(self), pwzfriendlyname.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFriendlyName(&self, grfhlfnamef: super::DWORD, ppwzfriendlyname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFriendlyName)(windows_core::Interface::as_raw(self), grfhlfnamef, ppwzfriendlyname as _) }
    }
    pub unsafe fn SetTargetFrameName<P0>(&self, pwztargetframename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTargetFrameName)(windows_core::Interface::as_raw(self), pwztargetframename.param().abi()) }
    }
    pub unsafe fn GetTargetFrameName(&self, ppwztargetframename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTargetFrameName)(windows_core::Interface::as_raw(self), ppwztargetframename as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMiscStatus(&self, pdwstatus: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMiscStatus)(windows_core::Interface::as_raw(self), pdwstatus as _) }
    }
    #[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon"))]
    pub unsafe fn Navigate<P1, P2, P3>(&self, grfhlnf: super::DWORD, pibc: P1, pibsc: P2, pihlbc: P3) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::LPBC>,
        P2: windows_core::Param<super::IBindStatusCallback>,
        P3: windows_core::Param<IHlinkBrowseContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).Navigate)(windows_core::Interface::as_raw(self), grfhlnf, pibc.param().abi(), pibsc.param().abi(), pihlbc.param().abi()) }
    }
    pub unsafe fn SetAdditionalParams<P0>(&self, pwzadditionalparams: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAdditionalParams)(windows_core::Interface::as_raw(self), pwzadditionalparams.param().abi()) }
    }
    pub unsafe fn GetAdditionalParams(&self, ppwzadditionalparams: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAdditionalParams)(windows_core::Interface::as_raw(self), ppwzadditionalparams as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHlink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SetHlinkSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetHlinkSite: usize,
    #[cfg(feature = "shared")]
    pub GetHlinkSite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetHlinkSite: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub SetMonikerReference: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    SetMonikerReference: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub GetMonikerReference: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    GetMonikerReference: usize,
    #[cfg(feature = "shared")]
    pub SetStringReference: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetStringReference: usize,
    #[cfg(feature = "shared")]
    pub GetStringReference: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut windows_core::PWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetStringReference: usize,
    pub SetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFriendlyName: usize,
    pub SetTargetFrameName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetTargetFrameName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetMiscStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMiscStatus: usize,
    #[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon"))]
    pub Navigate: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared", feature = "urlmon")))]
    Navigate: usize,
    pub SetAdditionalParams: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetAdditionalParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon"))]
pub trait IHlink_Impl: windows_core::IUnknownImpl {
    fn SetHlinkSite(&self, pihlsite: windows_core::Ref<IHlinkSite>, dwsitedata: super::DWORD) -> windows_core::Result<()>;
    fn GetHlinkSite(&self, ppihlsite: windows_core::OutRef<IHlinkSite>, pdwsitedata: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetMonikerReference(&self, grfhlsetf: super::DWORD, pimktarget: windows_core::Ref<super::IMoniker>, pwzlocation: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetMonikerReference(&self, dwwhichref: super::DWORD, ppimktarget: windows_core::OutRef<super::IMoniker>, ppwzlocation: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetStringReference(&self, grfhlsetf: super::DWORD, pwztarget: &windows_core::PCWSTR, pwzlocation: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetStringReference(&self, dwwhichref: super::DWORD, ppwztarget: *mut windows_core::PWSTR, ppwzlocation: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetFriendlyName(&self, pwzfriendlyname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetFriendlyName(&self, grfhlfnamef: super::DWORD, ppwzfriendlyname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetTargetFrameName(&self, pwztargetframename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetTargetFrameName(&self, ppwztargetframename: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetMiscStatus(&self, pdwstatus: *mut super::DWORD) -> windows_core::Result<()>;
    fn Navigate(&self, grfhlnf: super::DWORD, pibc: windows_core::Ref<super::LPBC>, pibsc: windows_core::Ref<super::IBindStatusCallback>, pihlbc: windows_core::Ref<IHlinkBrowseContext>) -> windows_core::Result<()>;
    fn SetAdditionalParams(&self, pwzadditionalparams: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetAdditionalParams(&self, ppwzadditionalparams: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon"))]
impl IHlink_Vtbl {
    pub const fn new<Identity: IHlink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetHlinkSite<Identity: IHlink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pihlsite: *mut core::ffi::c_void, dwsitedata: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlink_Impl::SetHlinkSite(this, core::mem::transmute_copy(&pihlsite), core::mem::transmute_copy(&dwsitedata)).into()
            }
        }
        unsafe extern "system" fn GetHlinkSite<Identity: IHlink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppihlsite: *mut *mut core::ffi::c_void, pdwsitedata: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlink_Impl::GetHlinkSite(this, core::mem::transmute_copy(&ppihlsite), core::mem::transmute_copy(&pdwsitedata)).into()
            }
        }
        unsafe extern "system" fn SetMonikerReference<Identity: IHlink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfhlsetf: super::DWORD, pimktarget: *mut core::ffi::c_void, pwzlocation: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlink_Impl::SetMonikerReference(this, core::mem::transmute_copy(&grfhlsetf), core::mem::transmute_copy(&pimktarget), core::mem::transmute(&pwzlocation)).into()
            }
        }
        unsafe extern "system" fn GetMonikerReference<Identity: IHlink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwhichref: super::DWORD, ppimktarget: *mut *mut core::ffi::c_void, ppwzlocation: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlink_Impl::GetMonikerReference(this, core::mem::transmute_copy(&dwwhichref), core::mem::transmute_copy(&ppimktarget), core::mem::transmute_copy(&ppwzlocation)).into()
            }
        }
        unsafe extern "system" fn SetStringReference<Identity: IHlink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfhlsetf: super::DWORD, pwztarget: windows_core::PCWSTR, pwzlocation: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlink_Impl::SetStringReference(this, core::mem::transmute_copy(&grfhlsetf), core::mem::transmute(&pwztarget), core::mem::transmute(&pwzlocation)).into()
            }
        }
        unsafe extern "system" fn GetStringReference<Identity: IHlink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwwhichref: super::DWORD, ppwztarget: *mut windows_core::PWSTR, ppwzlocation: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlink_Impl::GetStringReference(this, core::mem::transmute_copy(&dwwhichref), core::mem::transmute_copy(&ppwztarget), core::mem::transmute_copy(&ppwzlocation)).into()
            }
        }
        unsafe extern "system" fn SetFriendlyName<Identity: IHlink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzfriendlyname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlink_Impl::SetFriendlyName(this, core::mem::transmute(&pwzfriendlyname)).into()
            }
        }
        unsafe extern "system" fn GetFriendlyName<Identity: IHlink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfhlfnamef: super::DWORD, ppwzfriendlyname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlink_Impl::GetFriendlyName(this, core::mem::transmute_copy(&grfhlfnamef), core::mem::transmute_copy(&ppwzfriendlyname)).into()
            }
        }
        unsafe extern "system" fn SetTargetFrameName<Identity: IHlink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwztargetframename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlink_Impl::SetTargetFrameName(this, core::mem::transmute(&pwztargetframename)).into()
            }
        }
        unsafe extern "system" fn GetTargetFrameName<Identity: IHlink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwztargetframename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlink_Impl::GetTargetFrameName(this, core::mem::transmute_copy(&ppwztargetframename)).into()
            }
        }
        unsafe extern "system" fn GetMiscStatus<Identity: IHlink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstatus: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlink_Impl::GetMiscStatus(this, core::mem::transmute_copy(&pdwstatus)).into()
            }
        }
        unsafe extern "system" fn Navigate<Identity: IHlink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfhlnf: super::DWORD, pibc: *mut core::ffi::c_void, pibsc: *mut core::ffi::c_void, pihlbc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlink_Impl::Navigate(this, core::mem::transmute_copy(&grfhlnf), core::mem::transmute_copy(&pibc), core::mem::transmute_copy(&pibsc), core::mem::transmute_copy(&pihlbc)).into()
            }
        }
        unsafe extern "system" fn SetAdditionalParams<Identity: IHlink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzadditionalparams: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlink_Impl::SetAdditionalParams(this, core::mem::transmute(&pwzadditionalparams)).into()
            }
        }
        unsafe extern "system" fn GetAdditionalParams<Identity: IHlink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwzadditionalparams: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlink_Impl::GetAdditionalParams(this, core::mem::transmute_copy(&ppwzadditionalparams)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetHlinkSite: SetHlinkSite::<Identity, OFFSET>,
            GetHlinkSite: GetHlinkSite::<Identity, OFFSET>,
            SetMonikerReference: SetMonikerReference::<Identity, OFFSET>,
            GetMonikerReference: GetMonikerReference::<Identity, OFFSET>,
            SetStringReference: SetStringReference::<Identity, OFFSET>,
            GetStringReference: GetStringReference::<Identity, OFFSET>,
            SetFriendlyName: SetFriendlyName::<Identity, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, OFFSET>,
            SetTargetFrameName: SetTargetFrameName::<Identity, OFFSET>,
            GetTargetFrameName: GetTargetFrameName::<Identity, OFFSET>,
            GetMiscStatus: GetMiscStatus::<Identity, OFFSET>,
            Navigate: Navigate::<Identity, OFFSET>,
            SetAdditionalParams: SetAdditionalParams::<Identity, OFFSET>,
            GetAdditionalParams: GetAdditionalParams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHlink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon"))]
impl windows_core::RuntimeName for IHlink {}
windows_core::imp::define_interface!(IHlinkBrowseContext, IHlinkBrowseContext_Vtbl, 0x79eac9c7_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IHlinkBrowseContext, windows_core::IUnknown);
impl IHlinkBrowseContext {
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn Register<P1, P2>(&self, reserved: super::DWORD, piunk: P1, pimk: P2, pdwregister: *mut super::DWORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<super::IMoniker>,
    {
        unsafe { (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self), reserved, piunk.param().abi(), pimk.param().abi(), pdwregister as _) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn GetObject<P0>(&self, pimk: P0, fbindifrootregistered: bool, ppiunk: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMoniker>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), pimk.param().abi(), fbindifrootregistered.into(), core::mem::transmute(ppiunk)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Revoke(&self, dwregister: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Revoke)(windows_core::Interface::as_raw(self), dwregister) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn SetBrowseWindowInfo(&self, phlbwi: *const HLBWINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBrowseWindowInfo)(windows_core::Interface::as_raw(self), phlbwi) }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetBrowseWindowInfo(&self, phlbwi: *mut HLBWINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBrowseWindowInfo)(windows_core::Interface::as_raw(self), phlbwi as _) }
    }
    #[cfg(feature = "objidl")]
    pub unsafe fn SetInitialHlink<P0, P1, P2>(&self, pimktarget: P0, pwzlocation: P1, pwzfriendlyname: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMoniker>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInitialHlink)(windows_core::Interface::as_raw(self), pimktarget.param().abi(), pwzlocation.param().abi(), pwzfriendlyname.param().abi()) }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn OnNavigateHlink<P1, P2, P3>(&self, grfhlnf: super::DWORD, pimktarget: P1, pwzlocation: P2, pwzfriendlyname: P3, puhlid: *mut super::ULONG) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMoniker>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnNavigateHlink)(windows_core::Interface::as_raw(self), grfhlnf, pimktarget.param().abi(), pwzlocation.param().abi(), pwzfriendlyname.param().abi(), puhlid as _) }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn UpdateHlink<P1, P2, P3>(&self, uhlid: super::ULONG, pimktarget: P1, pwzlocation: P2, pwzfriendlyname: P3) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMoniker>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateHlink)(windows_core::Interface::as_raw(self), uhlid, pimktarget.param().abi(), pwzlocation.param().abi(), pwzfriendlyname.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn EnumNavigationStack(&self, dwreserved: super::DWORD, grfhlfnamef: super::DWORD, ppienumhlitem: *mut Option<IEnumHLITEM>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumNavigationStack)(windows_core::Interface::as_raw(self), dwreserved, grfhlfnamef, core::mem::transmute(ppienumhlitem)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn QueryHlink(&self, grfhlqf: super::DWORD, uhlid: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryHlink)(windows_core::Interface::as_raw(self), grfhlqf, uhlid) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetHlink(&self, uhlid: super::ULONG, ppihl: *mut Option<IHlink>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHlink)(windows_core::Interface::as_raw(self), uhlid, core::mem::transmute(ppihl)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetCurrentHlink(&self, uhlid: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentHlink)(windows_core::Interface::as_raw(self), uhlid) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn Clone<P0>(&self, piunkouter: P0, riid: *const super::IID, ppiunkobj: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), piunkouter.param().abi(), riid, core::mem::transmute(ppiunkobj)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Close(&self, reserved: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), reserved) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHlinkBrowseContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    Register: usize,
    #[cfg(feature = "objidl")]
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    GetObject: usize,
    #[cfg(feature = "shared")]
    pub Revoke: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Revoke: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub SetBrowseWindowInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const HLBWINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    SetBrowseWindowInfo: usize,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetBrowseWindowInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HLBWINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetBrowseWindowInfo: usize,
    #[cfg(feature = "objidl")]
    pub SetInitialHlink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidl"))]
    SetInitialHlink: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub OnNavigateHlink: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    OnNavigateHlink: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub UpdateHlink: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    UpdateHlink: usize,
    #[cfg(feature = "shared")]
    pub EnumNavigationStack: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EnumNavigationStack: usize,
    #[cfg(feature = "shared")]
    pub QueryHlink: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    QueryHlink: usize,
    #[cfg(feature = "shared")]
    pub GetHlink: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetHlink: usize,
    #[cfg(feature = "shared")]
    pub SetCurrentHlink: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetCurrentHlink: usize,
    #[cfg(feature = "guiddef")]
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    Clone: usize,
    #[cfg(feature = "shared")]
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Close: usize,
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared", feature = "windef"))]
pub trait IHlinkBrowseContext_Impl: windows_core::IUnknownImpl {
    fn Register(&self, reserved: super::DWORD, piunk: windows_core::Ref<windows_core::IUnknown>, pimk: windows_core::Ref<super::IMoniker>, pdwregister: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetObject(&self, pimk: windows_core::Ref<super::IMoniker>, fbindifrootregistered: windows_core::BOOL, ppiunk: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Revoke(&self, dwregister: super::DWORD) -> windows_core::Result<()>;
    fn SetBrowseWindowInfo(&self, phlbwi: *const HLBWINFO) -> windows_core::Result<()>;
    fn GetBrowseWindowInfo(&self, phlbwi: *mut HLBWINFO) -> windows_core::Result<()>;
    fn SetInitialHlink(&self, pimktarget: windows_core::Ref<super::IMoniker>, pwzlocation: &windows_core::PCWSTR, pwzfriendlyname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnNavigateHlink(&self, grfhlnf: super::DWORD, pimktarget: windows_core::Ref<super::IMoniker>, pwzlocation: &windows_core::PCWSTR, pwzfriendlyname: &windows_core::PCWSTR, puhlid: *mut super::ULONG) -> windows_core::Result<()>;
    fn UpdateHlink(&self, uhlid: super::ULONG, pimktarget: windows_core::Ref<super::IMoniker>, pwzlocation: &windows_core::PCWSTR, pwzfriendlyname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn EnumNavigationStack(&self, dwreserved: super::DWORD, grfhlfnamef: super::DWORD, ppienumhlitem: windows_core::OutRef<IEnumHLITEM>) -> windows_core::Result<()>;
    fn QueryHlink(&self, grfhlqf: super::DWORD, uhlid: super::ULONG) -> windows_core::Result<()>;
    fn GetHlink(&self, uhlid: super::ULONG, ppihl: windows_core::OutRef<IHlink>) -> windows_core::Result<()>;
    fn SetCurrentHlink(&self, uhlid: super::ULONG) -> windows_core::Result<()>;
    fn Clone(&self, piunkouter: windows_core::Ref<windows_core::IUnknown>, riid: *const super::IID, ppiunkobj: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn Close(&self, reserved: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared", feature = "windef"))]
impl IHlinkBrowseContext_Vtbl {
    pub const fn new<Identity: IHlinkBrowseContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Register<Identity: IHlinkBrowseContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: super::DWORD, piunk: *mut core::ffi::c_void, pimk: *mut core::ffi::c_void, pdwregister: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkBrowseContext_Impl::Register(this, core::mem::transmute_copy(&reserved), core::mem::transmute_copy(&piunk), core::mem::transmute_copy(&pimk), core::mem::transmute_copy(&pdwregister)).into()
            }
        }
        unsafe extern "system" fn GetObject<Identity: IHlinkBrowseContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimk: *mut core::ffi::c_void, fbindifrootregistered: windows_core::BOOL, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkBrowseContext_Impl::GetObject(this, core::mem::transmute_copy(&pimk), core::mem::transmute_copy(&fbindifrootregistered), core::mem::transmute_copy(&ppiunk)).into()
            }
        }
        unsafe extern "system" fn Revoke<Identity: IHlinkBrowseContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregister: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkBrowseContext_Impl::Revoke(this, core::mem::transmute_copy(&dwregister)).into()
            }
        }
        unsafe extern "system" fn SetBrowseWindowInfo<Identity: IHlinkBrowseContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phlbwi: *const HLBWINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkBrowseContext_Impl::SetBrowseWindowInfo(this, core::mem::transmute_copy(&phlbwi)).into()
            }
        }
        unsafe extern "system" fn GetBrowseWindowInfo<Identity: IHlinkBrowseContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phlbwi: *mut HLBWINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkBrowseContext_Impl::GetBrowseWindowInfo(this, core::mem::transmute_copy(&phlbwi)).into()
            }
        }
        unsafe extern "system" fn SetInitialHlink<Identity: IHlinkBrowseContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimktarget: *mut core::ffi::c_void, pwzlocation: windows_core::PCWSTR, pwzfriendlyname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkBrowseContext_Impl::SetInitialHlink(this, core::mem::transmute_copy(&pimktarget), core::mem::transmute(&pwzlocation), core::mem::transmute(&pwzfriendlyname)).into()
            }
        }
        unsafe extern "system" fn OnNavigateHlink<Identity: IHlinkBrowseContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfhlnf: super::DWORD, pimktarget: *mut core::ffi::c_void, pwzlocation: windows_core::PCWSTR, pwzfriendlyname: windows_core::PCWSTR, puhlid: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkBrowseContext_Impl::OnNavigateHlink(this, core::mem::transmute_copy(&grfhlnf), core::mem::transmute_copy(&pimktarget), core::mem::transmute(&pwzlocation), core::mem::transmute(&pwzfriendlyname), core::mem::transmute_copy(&puhlid)).into()
            }
        }
        unsafe extern "system" fn UpdateHlink<Identity: IHlinkBrowseContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uhlid: super::ULONG, pimktarget: *mut core::ffi::c_void, pwzlocation: windows_core::PCWSTR, pwzfriendlyname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkBrowseContext_Impl::UpdateHlink(this, core::mem::transmute_copy(&uhlid), core::mem::transmute_copy(&pimktarget), core::mem::transmute(&pwzlocation), core::mem::transmute(&pwzfriendlyname)).into()
            }
        }
        unsafe extern "system" fn EnumNavigationStack<Identity: IHlinkBrowseContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: super::DWORD, grfhlfnamef: super::DWORD, ppienumhlitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkBrowseContext_Impl::EnumNavigationStack(this, core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&grfhlfnamef), core::mem::transmute_copy(&ppienumhlitem)).into()
            }
        }
        unsafe extern "system" fn QueryHlink<Identity: IHlinkBrowseContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfhlqf: super::DWORD, uhlid: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkBrowseContext_Impl::QueryHlink(this, core::mem::transmute_copy(&grfhlqf), core::mem::transmute_copy(&uhlid)).into()
            }
        }
        unsafe extern "system" fn GetHlink<Identity: IHlinkBrowseContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uhlid: super::ULONG, ppihl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkBrowseContext_Impl::GetHlink(this, core::mem::transmute_copy(&uhlid), core::mem::transmute_copy(&ppihl)).into()
            }
        }
        unsafe extern "system" fn SetCurrentHlink<Identity: IHlinkBrowseContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uhlid: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkBrowseContext_Impl::SetCurrentHlink(this, core::mem::transmute_copy(&uhlid)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IHlinkBrowseContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piunkouter: *mut core::ffi::c_void, riid: *const super::IID, ppiunkobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkBrowseContext_Impl::Clone(this, core::mem::transmute_copy(&piunkouter), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppiunkobj)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IHlinkBrowseContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkBrowseContext_Impl::Close(this, core::mem::transmute_copy(&reserved)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            Revoke: Revoke::<Identity, OFFSET>,
            SetBrowseWindowInfo: SetBrowseWindowInfo::<Identity, OFFSET>,
            GetBrowseWindowInfo: GetBrowseWindowInfo::<Identity, OFFSET>,
            SetInitialHlink: SetInitialHlink::<Identity, OFFSET>,
            OnNavigateHlink: OnNavigateHlink::<Identity, OFFSET>,
            UpdateHlink: UpdateHlink::<Identity, OFFSET>,
            EnumNavigationStack: EnumNavigationStack::<Identity, OFFSET>,
            QueryHlink: QueryHlink::<Identity, OFFSET>,
            GetHlink: GetHlink::<Identity, OFFSET>,
            SetCurrentHlink: SetCurrentHlink::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHlinkBrowseContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IHlinkBrowseContext {}
windows_core::imp::define_interface!(IHlinkFrame, IHlinkFrame_Vtbl, 0x79eac9c5_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IHlinkFrame, windows_core::IUnknown);
impl IHlinkFrame {
    pub unsafe fn SetBrowseContext<P0>(&self, pihlbc: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IHlinkBrowseContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBrowseContext)(windows_core::Interface::as_raw(self), pihlbc.param().abi()) }
    }
    pub unsafe fn GetBrowseContext(&self, ppihlbc: *mut Option<IHlinkBrowseContext>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBrowseContext)(windows_core::Interface::as_raw(self), core::mem::transmute(ppihlbc)) }
    }
    #[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon"))]
    pub unsafe fn Navigate<P1, P2, P3>(&self, grfhlnf: super::DWORD, pbc: P1, pibsc: P2, pihlnavigate: P3) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::LPBC>,
        P2: windows_core::Param<super::IBindStatusCallback>,
        P3: windows_core::Param<IHlink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Navigate)(windows_core::Interface::as_raw(self), grfhlnf, pbc.param().abi(), pibsc.param().abi(), pihlnavigate.param().abi()) }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn OnNavigate<P1, P2, P3>(&self, grfhlnf: super::DWORD, pimktarget: P1, pwzlocation: P2, pwzfriendlyname: P3, dwreserved: super::DWORD) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMoniker>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnNavigate)(windows_core::Interface::as_raw(self), grfhlnf, pimktarget.param().abi(), pwzlocation.param().abi(), pwzfriendlyname.param().abi(), dwreserved) }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn UpdateHlink<P1, P2, P3>(&self, uhlid: super::ULONG, pimktarget: P1, pwzlocation: P2, pwzfriendlyname: P3) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMoniker>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateHlink)(windows_core::Interface::as_raw(self), uhlid, pimktarget.param().abi(), pwzlocation.param().abi(), pwzfriendlyname.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHlinkFrame_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetBrowseContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBrowseContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon"))]
    pub Navigate: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared", feature = "urlmon")))]
    Navigate: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub OnNavigate: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    OnNavigate: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub UpdateHlink: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    UpdateHlink: usize,
}
#[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon"))]
pub trait IHlinkFrame_Impl: windows_core::IUnknownImpl {
    fn SetBrowseContext(&self, pihlbc: windows_core::Ref<IHlinkBrowseContext>) -> windows_core::Result<()>;
    fn GetBrowseContext(&self, ppihlbc: windows_core::OutRef<IHlinkBrowseContext>) -> windows_core::Result<()>;
    fn Navigate(&self, grfhlnf: super::DWORD, pbc: windows_core::Ref<super::LPBC>, pibsc: windows_core::Ref<super::IBindStatusCallback>, pihlnavigate: windows_core::Ref<IHlink>) -> windows_core::Result<()>;
    fn OnNavigate(&self, grfhlnf: super::DWORD, pimktarget: windows_core::Ref<super::IMoniker>, pwzlocation: &windows_core::PCWSTR, pwzfriendlyname: &windows_core::PCWSTR, dwreserved: super::DWORD) -> windows_core::Result<()>;
    fn UpdateHlink(&self, uhlid: super::ULONG, pimktarget: windows_core::Ref<super::IMoniker>, pwzlocation: &windows_core::PCWSTR, pwzfriendlyname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon"))]
impl IHlinkFrame_Vtbl {
    pub const fn new<Identity: IHlinkFrame_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetBrowseContext<Identity: IHlinkFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pihlbc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkFrame_Impl::SetBrowseContext(this, core::mem::transmute_copy(&pihlbc)).into()
            }
        }
        unsafe extern "system" fn GetBrowseContext<Identity: IHlinkFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppihlbc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkFrame_Impl::GetBrowseContext(this, core::mem::transmute_copy(&ppihlbc)).into()
            }
        }
        unsafe extern "system" fn Navigate<Identity: IHlinkFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfhlnf: super::DWORD, pbc: *mut core::ffi::c_void, pibsc: *mut core::ffi::c_void, pihlnavigate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkFrame_Impl::Navigate(this, core::mem::transmute_copy(&grfhlnf), core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pibsc), core::mem::transmute_copy(&pihlnavigate)).into()
            }
        }
        unsafe extern "system" fn OnNavigate<Identity: IHlinkFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfhlnf: super::DWORD, pimktarget: *mut core::ffi::c_void, pwzlocation: windows_core::PCWSTR, pwzfriendlyname: windows_core::PCWSTR, dwreserved: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkFrame_Impl::OnNavigate(this, core::mem::transmute_copy(&grfhlnf), core::mem::transmute_copy(&pimktarget), core::mem::transmute(&pwzlocation), core::mem::transmute(&pwzfriendlyname), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn UpdateHlink<Identity: IHlinkFrame_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uhlid: super::ULONG, pimktarget: *mut core::ffi::c_void, pwzlocation: windows_core::PCWSTR, pwzfriendlyname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkFrame_Impl::UpdateHlink(this, core::mem::transmute_copy(&uhlid), core::mem::transmute_copy(&pimktarget), core::mem::transmute(&pwzlocation), core::mem::transmute(&pwzfriendlyname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetBrowseContext: SetBrowseContext::<Identity, OFFSET>,
            GetBrowseContext: GetBrowseContext::<Identity, OFFSET>,
            Navigate: Navigate::<Identity, OFFSET>,
            OnNavigate: OnNavigate::<Identity, OFFSET>,
            UpdateHlink: UpdateHlink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHlinkFrame as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon"))]
impl windows_core::RuntimeName for IHlinkFrame {}
windows_core::imp::define_interface!(IHlinkSite, IHlinkSite_Vtbl, 0x79eac9c2_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IHlinkSite, windows_core::IUnknown);
impl IHlinkSite {
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn QueryService(&self, dwsitedata: super::DWORD, guidservice: *const windows_core::GUID, riid: *const super::IID, ppiunk: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryService)(windows_core::Interface::as_raw(self), dwsitedata, guidservice, riid, core::mem::transmute(ppiunk)) }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn GetMoniker(&self, dwsitedata: super::DWORD, dwassign: super::DWORD, dwwhich: super::DWORD, ppimk: *mut Option<super::IMoniker>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMoniker)(windows_core::Interface::as_raw(self), dwsitedata, dwassign, dwwhich, core::mem::transmute(ppimk)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ReadyToNavigate(&self, dwsitedata: super::DWORD, dwreserved: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadyToNavigate)(windows_core::Interface::as_raw(self), dwsitedata, dwreserved) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn OnNavigationComplete<P3>(&self, dwsitedata: super::DWORD, dwreserved: super::DWORD, hrerror: windows_core::HRESULT, pwzerror: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnNavigationComplete)(windows_core::Interface::as_raw(self), dwsitedata, dwreserved, hrerror, pwzerror.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHlinkSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub QueryService: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const windows_core::GUID, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    QueryService: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub GetMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    GetMoniker: usize,
    #[cfg(feature = "shared")]
    pub ReadyToNavigate: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ReadyToNavigate: usize,
    #[cfg(feature = "shared")]
    pub OnNavigationComplete: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DWORD, windows_core::HRESULT, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    OnNavigationComplete: usize,
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared"))]
pub trait IHlinkSite_Impl: windows_core::IUnknownImpl {
    fn QueryService(&self, dwsitedata: super::DWORD, guidservice: *const windows_core::GUID, riid: *const super::IID, ppiunk: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetMoniker(&self, dwsitedata: super::DWORD, dwassign: super::DWORD, dwwhich: super::DWORD, ppimk: windows_core::OutRef<super::IMoniker>) -> windows_core::Result<()>;
    fn ReadyToNavigate(&self, dwsitedata: super::DWORD, dwreserved: super::DWORD) -> windows_core::Result<()>;
    fn OnNavigationComplete(&self, dwsitedata: super::DWORD, dwreserved: super::DWORD, hrerror: windows_core::HRESULT, pwzerror: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared"))]
impl IHlinkSite_Vtbl {
    pub const fn new<Identity: IHlinkSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryService<Identity: IHlinkSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsitedata: super::DWORD, guidservice: *const windows_core::GUID, riid: *const super::IID, ppiunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkSite_Impl::QueryService(this, core::mem::transmute_copy(&dwsitedata), core::mem::transmute_copy(&guidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppiunk)).into()
            }
        }
        unsafe extern "system" fn GetMoniker<Identity: IHlinkSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsitedata: super::DWORD, dwassign: super::DWORD, dwwhich: super::DWORD, ppimk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkSite_Impl::GetMoniker(this, core::mem::transmute_copy(&dwsitedata), core::mem::transmute_copy(&dwassign), core::mem::transmute_copy(&dwwhich), core::mem::transmute_copy(&ppimk)).into()
            }
        }
        unsafe extern "system" fn ReadyToNavigate<Identity: IHlinkSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsitedata: super::DWORD, dwreserved: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkSite_Impl::ReadyToNavigate(this, core::mem::transmute_copy(&dwsitedata), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn OnNavigationComplete<Identity: IHlinkSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsitedata: super::DWORD, dwreserved: super::DWORD, hrerror: windows_core::HRESULT, pwzerror: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkSite_Impl::OnNavigationComplete(this, core::mem::transmute_copy(&dwsitedata), core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&hrerror), core::mem::transmute(&pwzerror)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryService: QueryService::<Identity, OFFSET>,
            GetMoniker: GetMoniker::<Identity, OFFSET>,
            ReadyToNavigate: ReadyToNavigate::<Identity, OFFSET>,
            OnNavigationComplete: OnNavigationComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHlinkSite as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared"))]
impl windows_core::RuntimeName for IHlinkSite {}
windows_core::imp::define_interface!(IHlinkTarget, IHlinkTarget_Vtbl, 0x79eac9c4_baf9_11ce_8c82_00aa004ba90b);
windows_core::imp::interface_hierarchy!(IHlinkTarget, windows_core::IUnknown);
impl IHlinkTarget {
    pub unsafe fn SetBrowseContext<P0>(&self, pihlbc: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IHlinkBrowseContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBrowseContext)(windows_core::Interface::as_raw(self), pihlbc.param().abi()) }
    }
    pub unsafe fn GetBrowseContext(&self, ppihlbc: *mut Option<IHlinkBrowseContext>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBrowseContext)(windows_core::Interface::as_raw(self), core::mem::transmute(ppihlbc)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Navigate<P1>(&self, grfhlnf: super::DWORD, pwzjumplocation: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Navigate)(windows_core::Interface::as_raw(self), grfhlnf, pwzjumplocation.param().abi()) }
    }
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub unsafe fn GetMoniker<P0>(&self, pwzlocation: P0, dwassign: super::DWORD, ppimklocation: *mut Option<super::IMoniker>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetMoniker)(windows_core::Interface::as_raw(self), pwzlocation.param().abi(), dwassign, core::mem::transmute(ppimklocation)) }
    }
    pub unsafe fn GetFriendlyName<P0>(&self, pwzlocation: P0, ppwzfriendlyname: *mut windows_core::PWSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetFriendlyName)(windows_core::Interface::as_raw(self), pwzlocation.param().abi(), ppwzfriendlyname as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHlinkTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetBrowseContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBrowseContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Navigate: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Navigate: usize,
    #[cfg(all(feature = "objidl", feature = "shared"))]
    pub GetMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidl", feature = "shared")))]
    GetMoniker: usize,
    pub GetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "objidl", feature = "shared"))]
pub trait IHlinkTarget_Impl: windows_core::IUnknownImpl {
    fn SetBrowseContext(&self, pihlbc: windows_core::Ref<IHlinkBrowseContext>) -> windows_core::Result<()>;
    fn GetBrowseContext(&self, ppihlbc: windows_core::OutRef<IHlinkBrowseContext>) -> windows_core::Result<()>;
    fn Navigate(&self, grfhlnf: super::DWORD, pwzjumplocation: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetMoniker(&self, pwzlocation: &windows_core::PCWSTR, dwassign: super::DWORD, ppimklocation: windows_core::OutRef<super::IMoniker>) -> windows_core::Result<()>;
    fn GetFriendlyName(&self, pwzlocation: &windows_core::PCWSTR, ppwzfriendlyname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "objidl", feature = "shared"))]
impl IHlinkTarget_Vtbl {
    pub const fn new<Identity: IHlinkTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetBrowseContext<Identity: IHlinkTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pihlbc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkTarget_Impl::SetBrowseContext(this, core::mem::transmute_copy(&pihlbc)).into()
            }
        }
        unsafe extern "system" fn GetBrowseContext<Identity: IHlinkTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppihlbc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkTarget_Impl::GetBrowseContext(this, core::mem::transmute_copy(&ppihlbc)).into()
            }
        }
        unsafe extern "system" fn Navigate<Identity: IHlinkTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfhlnf: super::DWORD, pwzjumplocation: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkTarget_Impl::Navigate(this, core::mem::transmute_copy(&grfhlnf), core::mem::transmute(&pwzjumplocation)).into()
            }
        }
        unsafe extern "system" fn GetMoniker<Identity: IHlinkTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzlocation: windows_core::PCWSTR, dwassign: super::DWORD, ppimklocation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkTarget_Impl::GetMoniker(this, core::mem::transmute(&pwzlocation), core::mem::transmute_copy(&dwassign), core::mem::transmute_copy(&ppimklocation)).into()
            }
        }
        unsafe extern "system" fn GetFriendlyName<Identity: IHlinkTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwzlocation: windows_core::PCWSTR, ppwzfriendlyname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHlinkTarget_Impl::GetFriendlyName(this, core::mem::transmute(&pwzlocation), core::mem::transmute_copy(&ppwzfriendlyname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetBrowseContext: SetBrowseContext::<Identity, OFFSET>,
            GetBrowseContext: GetBrowseContext::<Identity, OFFSET>,
            Navigate: Navigate::<Identity, OFFSET>,
            GetMoniker: GetMoniker::<Identity, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHlinkTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "objidl", feature = "shared"))]
impl windows_core::RuntimeName for IHlinkTarget {}
pub type LPENUMHLITEM = IEnumHLITEM;
pub type LPEXTENSIONSERVICES = IExtensionServices;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPHLBWINFO = *mut HLBWINFO;
pub type LPHLINK = IHlink;
pub type LPHLINKBROWSECONTEXT = IHlinkBrowseContext;
pub type LPHLINKFRAME = IHlinkFrame;
pub type LPHLINKSITE = IHlinkSite;
pub type LPHLINKTARGET = IHlinkTarget;
#[cfg(feature = "shared")]
pub type LPHLITEM = *mut HLITEM;
