#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiAdvertiseProductA<P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: super::LANGID) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseProductA(szpackagepath : windows_core::PCSTR, szscriptfilepath : windows_core::PCSTR, sztransforms : windows_core::PCSTR, lgidlanguage : super::LANGID) -> super::UINT);
    unsafe { MsiAdvertiseProductA(szpackagepath.param().abi(), szscriptfilepath.param().abi(), sztransforms.param().abi(), lgidlanguage) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiAdvertiseProductExA<P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: super::LANGID, dwplatform: super::DWORD, dwoptions: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseProductExA(szpackagepath : windows_core::PCSTR, szscriptfilepath : windows_core::PCSTR, sztransforms : windows_core::PCSTR, lgidlanguage : super::LANGID, dwplatform : super::DWORD, dwoptions : super::DWORD) -> super::UINT);
    unsafe { MsiAdvertiseProductExA(szpackagepath.param().abi(), szscriptfilepath.param().abi(), sztransforms.param().abi(), lgidlanguage, dwplatform, dwoptions) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiAdvertiseProductExW<P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: super::LANGID, dwplatform: super::DWORD, dwoptions: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseProductExW(szpackagepath : windows_core::PCWSTR, szscriptfilepath : windows_core::PCWSTR, sztransforms : windows_core::PCWSTR, lgidlanguage : super::LANGID, dwplatform : super::DWORD, dwoptions : super::DWORD) -> super::UINT);
    unsafe { MsiAdvertiseProductExW(szpackagepath.param().abi(), szscriptfilepath.param().abi(), sztransforms.param().abi(), lgidlanguage, dwplatform, dwoptions) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiAdvertiseProductW<P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: super::LANGID) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseProductW(szpackagepath : windows_core::PCWSTR, szscriptfilepath : windows_core::PCWSTR, sztransforms : windows_core::PCWSTR, lgidlanguage : super::LANGID) -> super::UINT);
    unsafe { MsiAdvertiseProductW(szpackagepath.param().abi(), szscriptfilepath.param().abi(), sztransforms.param().abi(), lgidlanguage) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiAdvertiseScriptA<P0>(szscriptfile: P0, dwflags: super::DWORD, phregdata: Option<super::PHKEY>, fremoveitems: bool) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseScriptA(szscriptfile : windows_core::PCSTR, dwflags : super::DWORD, phregdata : super::PHKEY, fremoveitems : windows_core::BOOL) -> super::UINT);
    unsafe { MsiAdvertiseScriptA(szscriptfile.param().abi(), dwflags, phregdata.unwrap_or(core::mem::zeroed()) as _, fremoveitems.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiAdvertiseScriptW<P0>(szscriptfile: P0, dwflags: super::DWORD, phregdata: Option<super::PHKEY>, fremoveitems: bool) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseScriptW(szscriptfile : windows_core::PCWSTR, dwflags : super::DWORD, phregdata : super::PHKEY, fremoveitems : windows_core::BOOL) -> super::UINT);
    unsafe { MsiAdvertiseScriptW(szscriptfile.param().abi(), dwflags, phregdata.unwrap_or(core::mem::zeroed()) as _, fremoveitems.into()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiApplyMultiplePatchesA<P0, P1, P2>(szpatchpackages: P0, szproductcode: P1, szpropertieslist: P2) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiApplyMultiplePatchesA(szpatchpackages : windows_core::PCSTR, szproductcode : windows_core::PCSTR, szpropertieslist : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiApplyMultiplePatchesA(szpatchpackages.param().abi(), szproductcode.param().abi(), szpropertieslist.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiApplyMultiplePatchesW<P0, P1, P2>(szpatchpackages: P0, szproductcode: P1, szpropertieslist: P2) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiApplyMultiplePatchesW(szpatchpackages : windows_core::PCWSTR, szproductcode : windows_core::PCWSTR, szpropertieslist : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiApplyMultiplePatchesW(szpatchpackages.param().abi(), szproductcode.param().abi(), szpropertieslist.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiApplyPatchA<P0, P1, P3>(szpatchpackage: P0, szinstallpackage: P1, einstalltype: INSTALLTYPE, szcommandline: P3) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiApplyPatchA(szpatchpackage : windows_core::PCSTR, szinstallpackage : windows_core::PCSTR, einstalltype : INSTALLTYPE, szcommandline : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiApplyPatchA(szpatchpackage.param().abi(), szinstallpackage.param().abi(), einstalltype, szcommandline.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiApplyPatchW<P0, P1, P3>(szpatchpackage: P0, szinstallpackage: P1, einstalltype: INSTALLTYPE, szcommandline: P3) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiApplyPatchW(szpatchpackage : windows_core::PCWSTR, szinstallpackage : windows_core::PCWSTR, einstalltype : INSTALLTYPE, szcommandline : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiApplyPatchW(szpatchpackage.param().abi(), szinstallpackage.param().abi(), einstalltype, szcommandline.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MsiBeginTransactionA<P0>(szname: P0, dwtransactionattributes: super::DWORD, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::HANDLE) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiBeginTransactionA(szname : windows_core::PCSTR, dwtransactionattributes : super::DWORD, phtransactionhandle : *mut MSIHANDLE, phchangeofownerevent : *mut super::HANDLE) -> super::UINT);
    unsafe { MsiBeginTransactionA(szname.param().abi(), dwtransactionattributes, phtransactionhandle as _, phchangeofownerevent as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MsiBeginTransactionW<P0>(szname: P0, dwtransactionattributes: super::DWORD, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::HANDLE) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiBeginTransactionW(szname : windows_core::PCWSTR, dwtransactionattributes : super::DWORD, phtransactionhandle : *mut MSIHANDLE, phchangeofownerevent : *mut super::HANDLE) -> super::UINT);
    unsafe { MsiBeginTransactionW(szname.param().abi(), dwtransactionattributes, phtransactionhandle as _, phchangeofownerevent as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiCloseAllHandles() -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiCloseAllHandles() -> super::UINT);
    unsafe { MsiCloseAllHandles() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiCloseHandle(hany: MSIHANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiCloseHandle(hany : MSIHANDLE) -> super::UINT);
    unsafe { MsiCloseHandle(hany) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiCollectUserInfoA<P0>(szproduct: P0) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiCollectUserInfoA(szproduct : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiCollectUserInfoA(szproduct.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiCollectUserInfoW<P0>(szproduct: P0) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiCollectUserInfoW(szproduct : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiCollectUserInfoW(szproduct.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiConfigureFeatureA<P0, P1>(szproduct: P0, szfeature: P1, einstallstate: INSTALLSTATE) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureFeatureA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR, einstallstate : INSTALLSTATE) -> super::UINT);
    unsafe { MsiConfigureFeatureA(szproduct.param().abi(), szfeature.param().abi(), einstallstate) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiConfigureFeatureW<P0, P1>(szproduct: P0, szfeature: P1, einstallstate: INSTALLSTATE) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureFeatureW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR, einstallstate : INSTALLSTATE) -> super::UINT);
    unsafe { MsiConfigureFeatureW(szproduct.param().abi(), szfeature.param().abi(), einstallstate) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiConfigureProductA<P0>(szproduct: P0, iinstalllevel: i32, einstallstate: INSTALLSTATE) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureProductA(szproduct : windows_core::PCSTR, iinstalllevel : i32, einstallstate : INSTALLSTATE) -> super::UINT);
    unsafe { MsiConfigureProductA(szproduct.param().abi(), iinstalllevel, einstallstate) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiConfigureProductExA<P0, P3>(szproduct: P0, iinstalllevel: i32, einstallstate: INSTALLSTATE, szcommandline: P3) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureProductExA(szproduct : windows_core::PCSTR, iinstalllevel : i32, einstallstate : INSTALLSTATE, szcommandline : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiConfigureProductExA(szproduct.param().abi(), iinstalllevel, einstallstate, szcommandline.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiConfigureProductExW<P0, P3>(szproduct: P0, iinstalllevel: i32, einstallstate: INSTALLSTATE, szcommandline: P3) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureProductExW(szproduct : windows_core::PCWSTR, iinstalllevel : i32, einstallstate : INSTALLSTATE, szcommandline : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiConfigureProductExW(szproduct.param().abi(), iinstalllevel, einstallstate, szcommandline.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiConfigureProductW<P0>(szproduct: P0, iinstalllevel: i32, einstallstate: INSTALLSTATE) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureProductW(szproduct : windows_core::PCWSTR, iinstalllevel : i32, einstallstate : INSTALLSTATE) -> super::UINT);
    unsafe { MsiConfigureProductW(szproduct.param().abi(), iinstalllevel, einstallstate) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiDetermineApplicablePatchesA<P0>(szproductpackagepath: P0, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOA]) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDetermineApplicablePatchesA(szproductpackagepath : windows_core::PCSTR, cpatchinfo : super::DWORD, ppatchinfo : PMSIPATCHSEQUENCEINFOA) -> super::UINT);
    unsafe { MsiDetermineApplicablePatchesA(szproductpackagepath.param().abi(), ppatchinfo.len().try_into().unwrap(), core::mem::transmute(ppatchinfo.as_mut_ptr())) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiDetermineApplicablePatchesW<P0>(szproductpackagepath: P0, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOW]) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDetermineApplicablePatchesW(szproductpackagepath : windows_core::PCWSTR, cpatchinfo : super::DWORD, ppatchinfo : PMSIPATCHSEQUENCEINFOW) -> super::UINT);
    unsafe { MsiDetermineApplicablePatchesW(szproductpackagepath.param().abi(), ppatchinfo.len().try_into().unwrap(), core::mem::transmute(ppatchinfo.as_mut_ptr())) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiDeterminePatchSequenceA<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOA]) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDeterminePatchSequenceA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, cpatchinfo : super::DWORD, ppatchinfo : PMSIPATCHSEQUENCEINFOA) -> super::UINT);
    unsafe { MsiDeterminePatchSequenceA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, ppatchinfo.len().try_into().unwrap(), core::mem::transmute(ppatchinfo.as_mut_ptr())) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiDeterminePatchSequenceW<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOW]) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDeterminePatchSequenceW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, cpatchinfo : super::DWORD, ppatchinfo : PMSIPATCHSEQUENCEINFOW) -> super::UINT);
    unsafe { MsiDeterminePatchSequenceW(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, ppatchinfo.len().try_into().unwrap(), core::mem::transmute(ppatchinfo.as_mut_ptr())) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiEnableLogA<P1>(dwlogmode: super::DWORD, szlogfile: P1, dwlogattributes: super::DWORD) -> super::UINT
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnableLogA(dwlogmode : super::DWORD, szlogfile : windows_core::PCSTR, dwlogattributes : super::DWORD) -> super::UINT);
    unsafe { MsiEnableLogA(dwlogmode, szlogfile.param().abi(), dwlogattributes) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiEnableLogW<P1>(dwlogmode: super::DWORD, szlogfile: P1, dwlogattributes: super::DWORD) -> super::UINT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnableLogW(dwlogmode : super::DWORD, szlogfile : windows_core::PCWSTR, dwlogattributes : super::DWORD) -> super::UINT);
    unsafe { MsiEnableLogW(dwlogmode, szlogfile.param().abi(), dwlogattributes) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiEndTransaction(dwtransactionstate: super::DWORD) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiEndTransaction(dwtransactionstate : super::DWORD) -> super::UINT);
    unsafe { MsiEndTransaction(dwtransactionstate) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiEnumClientsA<P0>(szcomponent: P0, iproductindex: super::DWORD, lpproductbuf: windows_core::PSTR) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumClientsA(szcomponent : windows_core::PCSTR, iproductindex : super::DWORD, lpproductbuf : windows_core::PSTR) -> super::UINT);
    unsafe { MsiEnumClientsA(szcomponent.param().abi(), iproductindex, lpproductbuf) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiEnumClientsExA<P0, P1>(szcomponent: P0, szusersid: P1, dwcontext: super::DWORD, dwproductindex: super::DWORD, szproductbuf: Option<*mut super::CHAR>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PSTR>, pcchsid: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumClientsExA(szcomponent : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : super::DWORD, dwproductindex : super::DWORD, szproductbuf : *mut super::CHAR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PSTR, pcchsid : super::LPDWORD) -> super::UINT);
    unsafe { MsiEnumClientsExA(szcomponent.param().abi(), szusersid.param().abi(), dwcontext, dwproductindex, szproductbuf.unwrap_or(core::mem::zeroed()) as _, pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MsiEnumClientsExW<P0, P1>(szcomponent: P0, szusersid: P1, dwcontext: super::DWORD, dwproductindex: super::DWORD, szproductbuf: Option<*mut super::WCHAR>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PWSTR>, pcchsid: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumClientsExW(szcomponent : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : super::DWORD, dwproductindex : super::DWORD, szproductbuf : *mut super::WCHAR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PWSTR, pcchsid : super::LPDWORD) -> super::UINT);
    unsafe { MsiEnumClientsExW(szcomponent.param().abi(), szusersid.param().abi(), dwcontext, dwproductindex, szproductbuf.unwrap_or(core::mem::zeroed()) as _, pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiEnumClientsW<P0>(szcomponent: P0, iproductindex: super::DWORD, lpproductbuf: windows_core::PWSTR) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumClientsW(szcomponent : windows_core::PCWSTR, iproductindex : super::DWORD, lpproductbuf : windows_core::PWSTR) -> super::UINT);
    unsafe { MsiEnumClientsW(szcomponent.param().abi(), iproductindex, lpproductbuf) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiEnumComponentQualifiersA<P0>(szcomponent: P0, iindex: super::DWORD, lpqualifierbuf: windows_core::PSTR, pcchqualifierbuf: super::LPDWORD, lpapplicationdatabuf: Option<windows_core::PSTR>, pcchapplicationdatabuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentQualifiersA(szcomponent : windows_core::PCSTR, iindex : super::DWORD, lpqualifierbuf : windows_core::PSTR, pcchqualifierbuf : super::LPDWORD, lpapplicationdatabuf : windows_core::PSTR, pcchapplicationdatabuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiEnumComponentQualifiersA(szcomponent.param().abi(), iindex, lpqualifierbuf, pcchqualifierbuf as _, lpapplicationdatabuf.unwrap_or(core::mem::zeroed()) as _, pcchapplicationdatabuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiEnumComponentQualifiersW<P0>(szcomponent: P0, iindex: super::DWORD, lpqualifierbuf: windows_core::PWSTR, pcchqualifierbuf: super::LPDWORD, lpapplicationdatabuf: Option<windows_core::PWSTR>, pcchapplicationdatabuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentQualifiersW(szcomponent : windows_core::PCWSTR, iindex : super::DWORD, lpqualifierbuf : windows_core::PWSTR, pcchqualifierbuf : super::LPDWORD, lpapplicationdatabuf : windows_core::PWSTR, pcchapplicationdatabuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiEnumComponentQualifiersW(szcomponent.param().abi(), iindex, lpqualifierbuf, pcchqualifierbuf as _, lpapplicationdatabuf.unwrap_or(core::mem::zeroed()) as _, pcchapplicationdatabuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiEnumComponentsA(icomponentindex: super::DWORD, lpcomponentbuf: windows_core::PSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentsA(icomponentindex : super::DWORD, lpcomponentbuf : windows_core::PSTR) -> super::UINT);
    unsafe { MsiEnumComponentsA(icomponentindex, lpcomponentbuf) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiEnumComponentsExA<P0>(szusersid: P0, dwcontext: super::DWORD, dwindex: super::DWORD, szinstalledcomponentcode: Option<*mut super::CHAR>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PSTR>, pcchsid: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentsExA(szusersid : windows_core::PCSTR, dwcontext : super::DWORD, dwindex : super::DWORD, szinstalledcomponentcode : *mut super::CHAR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PSTR, pcchsid : super::LPDWORD) -> super::UINT);
    unsafe { MsiEnumComponentsExA(szusersid.param().abi(), dwcontext, dwindex, szinstalledcomponentcode.unwrap_or(core::mem::zeroed()) as _, pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MsiEnumComponentsExW<P0>(szusersid: P0, dwcontext: super::DWORD, dwindex: super::DWORD, szinstalledcomponentcode: Option<*mut super::WCHAR>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PWSTR>, pcchsid: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentsExW(szusersid : windows_core::PCWSTR, dwcontext : super::DWORD, dwindex : super::DWORD, szinstalledcomponentcode : *mut super::WCHAR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PWSTR, pcchsid : super::LPDWORD) -> super::UINT);
    unsafe { MsiEnumComponentsExW(szusersid.param().abi(), dwcontext, dwindex, szinstalledcomponentcode.unwrap_or(core::mem::zeroed()) as _, pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiEnumComponentsW(icomponentindex: super::DWORD, lpcomponentbuf: windows_core::PWSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentsW(icomponentindex : super::DWORD, lpcomponentbuf : windows_core::PWSTR) -> super::UINT);
    unsafe { MsiEnumComponentsW(icomponentindex, lpcomponentbuf) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiEnumFeaturesA<P0>(szproduct: P0, ifeatureindex: super::DWORD, lpfeaturebuf: windows_core::PSTR, lpparentbuf: Option<windows_core::PSTR>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumFeaturesA(szproduct : windows_core::PCSTR, ifeatureindex : super::DWORD, lpfeaturebuf : windows_core::PSTR, lpparentbuf : windows_core::PSTR) -> super::UINT);
    unsafe { MsiEnumFeaturesA(szproduct.param().abi(), ifeatureindex, lpfeaturebuf, lpparentbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiEnumFeaturesW<P0>(szproduct: P0, ifeatureindex: super::DWORD, lpfeaturebuf: windows_core::PWSTR, lpparentbuf: Option<windows_core::PWSTR>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumFeaturesW(szproduct : windows_core::PCWSTR, ifeatureindex : super::DWORD, lpfeaturebuf : windows_core::PWSTR, lpparentbuf : windows_core::PWSTR) -> super::UINT);
    unsafe { MsiEnumFeaturesW(szproduct.param().abi(), ifeatureindex, lpfeaturebuf, lpparentbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiEnumPatchesA<P0>(szproduct: P0, ipatchindex: super::DWORD, lppatchbuf: windows_core::PSTR, lptransformsbuf: windows_core::PSTR, pcchtransformsbuf: super::LPDWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumPatchesA(szproduct : windows_core::PCSTR, ipatchindex : super::DWORD, lppatchbuf : windows_core::PSTR, lptransformsbuf : windows_core::PSTR, pcchtransformsbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiEnumPatchesA(szproduct.param().abi(), ipatchindex, lppatchbuf, lptransformsbuf, pcchtransformsbuf as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiEnumPatchesExA<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: super::DWORD, dwfilter: super::DWORD, dwindex: super::DWORD, szpatchcode: Option<*mut super::CHAR>, sztargetproductcode: Option<*mut super::CHAR>, pdwtargetproductcontext: Option<*mut MSIINSTALLCONTEXT>, sztargetusersid: Option<windows_core::PSTR>, pcchtargetusersid: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumPatchesExA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : super::DWORD, dwfilter : super::DWORD, dwindex : super::DWORD, szpatchcode : *mut super::CHAR, sztargetproductcode : *mut super::CHAR, pdwtargetproductcontext : *mut MSIINSTALLCONTEXT, sztargetusersid : windows_core::PSTR, pcchtargetusersid : super::LPDWORD) -> super::UINT);
    unsafe { MsiEnumPatchesExA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, dwfilter, dwindex, szpatchcode.unwrap_or(core::mem::zeroed()) as _, sztargetproductcode.unwrap_or(core::mem::zeroed()) as _, pdwtargetproductcontext.unwrap_or(core::mem::zeroed()) as _, sztargetusersid.unwrap_or(core::mem::zeroed()) as _, pcchtargetusersid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MsiEnumPatchesExW<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: super::DWORD, dwfilter: super::DWORD, dwindex: super::DWORD, szpatchcode: Option<*mut super::WCHAR>, sztargetproductcode: Option<*mut super::WCHAR>, pdwtargetproductcontext: Option<*mut MSIINSTALLCONTEXT>, sztargetusersid: Option<windows_core::PWSTR>, pcchtargetusersid: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumPatchesExW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : super::DWORD, dwfilter : super::DWORD, dwindex : super::DWORD, szpatchcode : *mut super::WCHAR, sztargetproductcode : *mut super::WCHAR, pdwtargetproductcontext : *mut MSIINSTALLCONTEXT, sztargetusersid : windows_core::PWSTR, pcchtargetusersid : super::LPDWORD) -> super::UINT);
    unsafe { MsiEnumPatchesExW(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, dwfilter, dwindex, szpatchcode.unwrap_or(core::mem::zeroed()) as _, sztargetproductcode.unwrap_or(core::mem::zeroed()) as _, pdwtargetproductcontext.unwrap_or(core::mem::zeroed()) as _, sztargetusersid.unwrap_or(core::mem::zeroed()) as _, pcchtargetusersid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiEnumPatchesW<P0>(szproduct: P0, ipatchindex: super::DWORD, lppatchbuf: windows_core::PWSTR, lptransformsbuf: windows_core::PWSTR, pcchtransformsbuf: super::LPDWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumPatchesW(szproduct : windows_core::PCWSTR, ipatchindex : super::DWORD, lppatchbuf : windows_core::PWSTR, lptransformsbuf : windows_core::PWSTR, pcchtransformsbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiEnumPatchesW(szproduct.param().abi(), ipatchindex, lppatchbuf, lptransformsbuf, pcchtransformsbuf as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiEnumProductsA(iproductindex: super::DWORD, lpproductbuf: windows_core::PSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiEnumProductsA(iproductindex : super::DWORD, lpproductbuf : windows_core::PSTR) -> super::UINT);
    unsafe { MsiEnumProductsA(iproductindex, lpproductbuf) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiEnumProductsExA<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: super::DWORD, dwindex: super::DWORD, szinstalledproductcode: Option<*mut super::CHAR>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PSTR>, pcchsid: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumProductsExA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : super::DWORD, dwindex : super::DWORD, szinstalledproductcode : *mut super::CHAR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PSTR, pcchsid : super::LPDWORD) -> super::UINT);
    unsafe { MsiEnumProductsExA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, dwindex, szinstalledproductcode.unwrap_or(core::mem::zeroed()) as _, pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MsiEnumProductsExW<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: super::DWORD, dwindex: super::DWORD, szinstalledproductcode: Option<*mut super::WCHAR>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PWSTR>, pcchsid: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumProductsExW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : super::DWORD, dwindex : super::DWORD, szinstalledproductcode : *mut super::WCHAR, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PWSTR, pcchsid : super::LPDWORD) -> super::UINT);
    unsafe { MsiEnumProductsExW(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, dwindex, szinstalledproductcode.unwrap_or(core::mem::zeroed()) as _, pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiEnumProductsW(iproductindex: super::DWORD, lpproductbuf: windows_core::PWSTR) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiEnumProductsW(iproductindex : super::DWORD, lpproductbuf : windows_core::PWSTR) -> super::UINT);
    unsafe { MsiEnumProductsW(iproductindex, lpproductbuf) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiEnumRelatedProductsA<P0>(lpupgradecode: P0, dwreserved: super::DWORD, iproductindex: super::DWORD, lpproductbuf: windows_core::PSTR) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumRelatedProductsA(lpupgradecode : windows_core::PCSTR, dwreserved : super::DWORD, iproductindex : super::DWORD, lpproductbuf : windows_core::PSTR) -> super::UINT);
    unsafe { MsiEnumRelatedProductsA(lpupgradecode.param().abi(), dwreserved, iproductindex, lpproductbuf) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiEnumRelatedProductsW<P0>(lpupgradecode: P0, dwreserved: super::DWORD, iproductindex: super::DWORD, lpproductbuf: windows_core::PWSTR) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumRelatedProductsW(lpupgradecode : windows_core::PCWSTR, dwreserved : super::DWORD, iproductindex : super::DWORD, lpproductbuf : windows_core::PWSTR) -> super::UINT);
    unsafe { MsiEnumRelatedProductsW(lpupgradecode.param().abi(), dwreserved, iproductindex, lpproductbuf) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiExtractPatchXMLDataA<P0>(szpatchpath: P0, dwreserved: super::DWORD, szxmldata: Option<windows_core::PSTR>, pcchxmldata: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiExtractPatchXMLDataA(szpatchpath : windows_core::PCSTR, dwreserved : super::DWORD, szxmldata : windows_core::PSTR, pcchxmldata : super::LPDWORD) -> super::UINT);
    unsafe { MsiExtractPatchXMLDataA(szpatchpath.param().abi(), dwreserved, szxmldata.unwrap_or(core::mem::zeroed()) as _, pcchxmldata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiExtractPatchXMLDataW<P0>(szpatchpath: P0, dwreserved: super::DWORD, szxmldata: Option<windows_core::PWSTR>, pcchxmldata: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiExtractPatchXMLDataW(szpatchpath : windows_core::PCWSTR, dwreserved : super::DWORD, szxmldata : windows_core::PWSTR, pcchxmldata : super::LPDWORD) -> super::UINT);
    unsafe { MsiExtractPatchXMLDataW(szpatchpath.param().abi(), dwreserved, szxmldata.unwrap_or(core::mem::zeroed()) as _, pcchxmldata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn MsiGetComponentPathA<P0, P1>(szproduct: P0, szcomponent: P1, lppathbuf: Option<windows_core::PSTR>, pcchbuf: Option<super::LPDWORD>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetComponentPathA(szproduct : windows_core::PCSTR, szcomponent : windows_core::PCSTR, lppathbuf : windows_core::PSTR, pcchbuf : super::LPDWORD) -> INSTALLSTATE);
    unsafe { MsiGetComponentPathA(szproduct.param().abi(), szcomponent.param().abi(), lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn MsiGetComponentPathExA<P0, P1, P2>(szproductcode: P0, szcomponentcode: P1, szusersid: P2, dwcontext: Option<MSIINSTALLCONTEXT>, lpoutpathbuffer: Option<windows_core::PSTR>, pcchoutpathbuffer: Option<super::LPDWORD>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetComponentPathExA(szproductcode : windows_core::PCSTR, szcomponentcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, lpoutpathbuffer : windows_core::PSTR, pcchoutpathbuffer : super::LPDWORD) -> INSTALLSTATE);
    unsafe { MsiGetComponentPathExA(szproductcode.param().abi(), szcomponentcode.param().abi(), szusersid.param().abi(), dwcontext.unwrap_or(core::mem::zeroed()) as _, lpoutpathbuffer.unwrap_or(core::mem::zeroed()) as _, pcchoutpathbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn MsiGetComponentPathExW<P0, P1, P2>(szproductcode: P0, szcomponentcode: P1, szusersid: P2, dwcontext: Option<MSIINSTALLCONTEXT>, lpoutpathbuffer: Option<windows_core::PWSTR>, pcchoutpathbuffer: Option<super::LPDWORD>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetComponentPathExW(szproductcode : windows_core::PCWSTR, szcomponentcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, lpoutpathbuffer : windows_core::PWSTR, pcchoutpathbuffer : super::LPDWORD) -> INSTALLSTATE);
    unsafe { MsiGetComponentPathExW(szproductcode.param().abi(), szcomponentcode.param().abi(), szusersid.param().abi(), dwcontext.unwrap_or(core::mem::zeroed()) as _, lpoutpathbuffer.unwrap_or(core::mem::zeroed()) as _, pcchoutpathbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn MsiGetComponentPathW<P0, P1>(szproduct: P0, szcomponent: P1, lppathbuf: Option<windows_core::PWSTR>, pcchbuf: Option<super::LPDWORD>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetComponentPathW(szproduct : windows_core::PCWSTR, szcomponent : windows_core::PCWSTR, lppathbuf : windows_core::PWSTR, pcchbuf : super::LPDWORD) -> INSTALLSTATE);
    unsafe { MsiGetComponentPathW(szproduct.param().abi(), szcomponent.param().abi(), lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetFeatureInfoA<P1>(hproduct: MSIHANDLE, szfeature: P1, lpattributes: Option<super::LPDWORD>, lptitlebuf: Option<windows_core::PSTR>, pcchtitlebuf: Option<super::LPDWORD>, lphelpbuf: Option<windows_core::PSTR>, pcchhelpbuf: Option<super::LPDWORD>) -> super::UINT
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureInfoA(hproduct : MSIHANDLE, szfeature : windows_core::PCSTR, lpattributes : super::LPDWORD, lptitlebuf : windows_core::PSTR, pcchtitlebuf : super::LPDWORD, lphelpbuf : windows_core::PSTR, pcchhelpbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetFeatureInfoA(hproduct, szfeature.param().abi(), lpattributes.unwrap_or(core::mem::zeroed()) as _, lptitlebuf.unwrap_or(core::mem::zeroed()) as _, pcchtitlebuf.unwrap_or(core::mem::zeroed()) as _, lphelpbuf.unwrap_or(core::mem::zeroed()) as _, pcchhelpbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetFeatureInfoW<P1>(hproduct: MSIHANDLE, szfeature: P1, lpattributes: Option<super::LPDWORD>, lptitlebuf: Option<windows_core::PWSTR>, pcchtitlebuf: Option<super::LPDWORD>, lphelpbuf: Option<windows_core::PWSTR>, pcchhelpbuf: Option<super::LPDWORD>) -> super::UINT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureInfoW(hproduct : MSIHANDLE, szfeature : windows_core::PCWSTR, lpattributes : super::LPDWORD, lptitlebuf : windows_core::PWSTR, pcchtitlebuf : super::LPDWORD, lphelpbuf : windows_core::PWSTR, pcchhelpbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetFeatureInfoW(hproduct, szfeature.param().abi(), lpattributes.unwrap_or(core::mem::zeroed()) as _, lptitlebuf.unwrap_or(core::mem::zeroed()) as _, pcchtitlebuf.unwrap_or(core::mem::zeroed()) as _, lphelpbuf.unwrap_or(core::mem::zeroed()) as _, pcchhelpbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetFeatureUsageA<P0, P1>(szproduct: P0, szfeature: P1, pdwusecount: Option<super::LPDWORD>, pwdateused: Option<super::LPWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureUsageA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR, pdwusecount : super::LPDWORD, pwdateused : super::LPWORD) -> super::UINT);
    unsafe { MsiGetFeatureUsageA(szproduct.param().abi(), szfeature.param().abi(), pdwusecount.unwrap_or(core::mem::zeroed()) as _, pwdateused.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetFeatureUsageW<P0, P1>(szproduct: P0, szfeature: P1, pdwusecount: Option<super::LPDWORD>, pwdateused: Option<super::LPWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureUsageW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR, pdwusecount : super::LPDWORD, pwdateused : super::LPWORD) -> super::UINT);
    unsafe { MsiGetFeatureUsageW(szproduct.param().abi(), szfeature.param().abi(), pdwusecount.unwrap_or(core::mem::zeroed()) as _, pwdateused.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiGetFileHashA<P0>(szfilepath: P0, dwoptions: super::DWORD, phash: PMSIFILEHASHINFO) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileHashA(szfilepath : windows_core::PCSTR, dwoptions : super::DWORD, phash : PMSIFILEHASHINFO) -> super::UINT);
    unsafe { MsiGetFileHashA(szfilepath.param().abi(), dwoptions, phash as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiGetFileHashW<P0>(szfilepath: P0, dwoptions: super::DWORD, phash: PMSIFILEHASHINFO) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileHashW(szfilepath : windows_core::PCWSTR, dwoptions : super::DWORD, phash : PMSIFILEHASHINFO) -> super::UINT);
    unsafe { MsiGetFileHashW(szfilepath.param().abi(), dwoptions, phash as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
#[inline]
pub unsafe fn MsiGetFileSignatureInformationA<P0>(szsignedobjectpath: P0, dwflags: super::DWORD, ppccertcontext: *mut super::PCCERT_CONTEXT, pbhashdata: Option<super::LPBYTE>, pcbhashdata: Option<super::LPDWORD>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileSignatureInformationA(szsignedobjectpath : windows_core::PCSTR, dwflags : super::DWORD, ppccertcontext : *mut super::PCCERT_CONTEXT, pbhashdata : super::LPBYTE, pcbhashdata : super::LPDWORD) -> windows_core::HRESULT);
    unsafe { MsiGetFileSignatureInformationA(szsignedobjectpath.param().abi(), dwflags, ppccertcontext as _, pbhashdata.unwrap_or(core::mem::zeroed()) as _, pcbhashdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
#[inline]
pub unsafe fn MsiGetFileSignatureInformationW<P0>(szsignedobjectpath: P0, dwflags: super::DWORD, ppccertcontext: *mut super::PCCERT_CONTEXT, pbhashdata: Option<super::LPBYTE>, pcbhashdata: Option<super::LPDWORD>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileSignatureInformationW(szsignedobjectpath : windows_core::PCWSTR, dwflags : super::DWORD, ppccertcontext : *mut super::PCCERT_CONTEXT, pbhashdata : super::LPBYTE, pcbhashdata : super::LPDWORD) -> windows_core::HRESULT);
    unsafe { MsiGetFileSignatureInformationW(szsignedobjectpath.param().abi(), dwflags, ppccertcontext as _, pbhashdata.unwrap_or(core::mem::zeroed()) as _, pcbhashdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetFileVersionA<P0>(szfilepath: P0, lpversionbuf: Option<windows_core::PSTR>, pcchversionbuf: Option<super::LPDWORD>, lplangbuf: Option<windows_core::PSTR>, pcchlangbuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileVersionA(szfilepath : windows_core::PCSTR, lpversionbuf : windows_core::PSTR, pcchversionbuf : super::LPDWORD, lplangbuf : windows_core::PSTR, pcchlangbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetFileVersionA(szfilepath.param().abi(), lpversionbuf.unwrap_or(core::mem::zeroed()) as _, pcchversionbuf.unwrap_or(core::mem::zeroed()) as _, lplangbuf.unwrap_or(core::mem::zeroed()) as _, pcchlangbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetFileVersionW<P0>(szfilepath: P0, lpversionbuf: Option<windows_core::PWSTR>, pcchversionbuf: Option<super::LPDWORD>, lplangbuf: Option<windows_core::PWSTR>, pcchlangbuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileVersionW(szfilepath : windows_core::PCWSTR, lpversionbuf : windows_core::PWSTR, pcchversionbuf : super::LPDWORD, lplangbuf : windows_core::PWSTR, pcchlangbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetFileVersionW(szfilepath.param().abi(), lpversionbuf.unwrap_or(core::mem::zeroed()) as _, pcchversionbuf.unwrap_or(core::mem::zeroed()) as _, lplangbuf.unwrap_or(core::mem::zeroed()) as _, pcchlangbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetPatchFileListA<P0, P1>(szproductcode: P0, szpatchpackages: P1, pcfiles: super::LPDWORD, pphfilerecords: *mut *mut MSIHANDLE) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchFileListA(szproductcode : windows_core::PCSTR, szpatchpackages : windows_core::PCSTR, pcfiles : super::LPDWORD, pphfilerecords : *mut *mut MSIHANDLE) -> super::UINT);
    unsafe { MsiGetPatchFileListA(szproductcode.param().abi(), szpatchpackages.param().abi(), pcfiles as _, pphfilerecords as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetPatchFileListW<P0, P1>(szproductcode: P0, szpatchpackages: P1, pcfiles: super::LPDWORD, pphfilerecords: *mut *mut MSIHANDLE) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchFileListW(szproductcode : windows_core::PCWSTR, szpatchpackages : windows_core::PCWSTR, pcfiles : super::LPDWORD, pphfilerecords : *mut *mut MSIHANDLE) -> super::UINT);
    unsafe { MsiGetPatchFileListW(szproductcode.param().abi(), szpatchpackages.param().abi(), pcfiles as _, pphfilerecords as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetPatchInfoA<P0, P1>(szpatch: P0, szattribute: P1, lpvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchInfoA(szpatch : windows_core::PCSTR, szattribute : windows_core::PCSTR, lpvaluebuf : windows_core::PSTR, pcchvaluebuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetPatchInfoA(szpatch.param().abi(), szattribute.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetPatchInfoExA<P0, P1, P2, P4>(szpatchcode: P0, szproductcode: P1, szusersid: P2, dwcontext: MSIINSTALLCONTEXT, szproperty: P4, lpvalue: Option<windows_core::PSTR>, pcchvalue: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchInfoExA(szpatchcode : windows_core::PCSTR, szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : windows_core::PCSTR, lpvalue : windows_core::PSTR, pcchvalue : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetPatchInfoExA(szpatchcode.param().abi(), szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szproperty.param().abi(), lpvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetPatchInfoExW<P0, P1, P2, P4>(szpatchcode: P0, szproductcode: P1, szusersid: P2, dwcontext: MSIINSTALLCONTEXT, szproperty: P4, lpvalue: Option<windows_core::PWSTR>, pcchvalue: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchInfoExW(szpatchcode : windows_core::PCWSTR, szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : windows_core::PCWSTR, lpvalue : windows_core::PWSTR, pcchvalue : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetPatchInfoExW(szpatchcode.param().abi(), szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szproperty.param().abi(), lpvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetPatchInfoW<P0, P1>(szpatch: P0, szattribute: P1, lpvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchInfoW(szpatch : windows_core::PCWSTR, szattribute : windows_core::PCWSTR, lpvaluebuf : windows_core::PWSTR, pcchvaluebuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetPatchInfoW(szpatch.param().abi(), szattribute.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiGetProductCodeA<P0>(szcomponent: P0, lpbuf39: windows_core::PSTR) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductCodeA(szcomponent : windows_core::PCSTR, lpbuf39 : windows_core::PSTR) -> super::UINT);
    unsafe { MsiGetProductCodeA(szcomponent.param().abi(), lpbuf39) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiGetProductCodeW<P0>(szcomponent: P0, lpbuf39: windows_core::PWSTR) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductCodeW(szcomponent : windows_core::PCWSTR, lpbuf39 : windows_core::PWSTR) -> super::UINT);
    unsafe { MsiGetProductCodeW(szcomponent.param().abi(), lpbuf39) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetProductInfoA<P0, P1>(szproduct: P0, szattribute: P1, lpvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoA(szproduct : windows_core::PCSTR, szattribute : windows_core::PCSTR, lpvaluebuf : windows_core::PSTR, pcchvaluebuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetProductInfoA(szproduct.param().abi(), szattribute.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetProductInfoExA<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szproperty: P3, szvalue: Option<windows_core::PSTR>, pcchvalue: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoExA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : windows_core::PCSTR, szvalue : windows_core::PSTR, pcchvalue : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetProductInfoExA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szproperty.param().abi(), szvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetProductInfoExW<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szproperty: P3, szvalue: Option<windows_core::PWSTR>, pcchvalue: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoExW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : windows_core::PCWSTR, szvalue : windows_core::PWSTR, pcchvalue : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetProductInfoExW(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szproperty.param().abi(), szvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetProductInfoFromScriptA<P0>(szscriptfile: P0, lpproductbuf39: Option<windows_core::PSTR>, plgidlanguage: Option<*mut super::LANGID>, pdwversion: Option<super::LPDWORD>, lpnamebuf: Option<windows_core::PSTR>, pcchnamebuf: Option<super::LPDWORD>, lppackagebuf: Option<windows_core::PSTR>, pcchpackagebuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoFromScriptA(szscriptfile : windows_core::PCSTR, lpproductbuf39 : windows_core::PSTR, plgidlanguage : *mut super::LANGID, pdwversion : super::LPDWORD, lpnamebuf : windows_core::PSTR, pcchnamebuf : super::LPDWORD, lppackagebuf : windows_core::PSTR, pcchpackagebuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetProductInfoFromScriptA(szscriptfile.param().abi(), lpproductbuf39.unwrap_or(core::mem::zeroed()) as _, plgidlanguage.unwrap_or(core::mem::zeroed()) as _, pdwversion.unwrap_or(core::mem::zeroed()) as _, lpnamebuf.unwrap_or(core::mem::zeroed()) as _, pcchnamebuf.unwrap_or(core::mem::zeroed()) as _, lppackagebuf.unwrap_or(core::mem::zeroed()) as _, pcchpackagebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetProductInfoFromScriptW<P0>(szscriptfile: P0, lpproductbuf39: Option<windows_core::PWSTR>, plgidlanguage: Option<*mut super::LANGID>, pdwversion: Option<super::LPDWORD>, lpnamebuf: Option<windows_core::PWSTR>, pcchnamebuf: Option<super::LPDWORD>, lppackagebuf: Option<windows_core::PWSTR>, pcchpackagebuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoFromScriptW(szscriptfile : windows_core::PCWSTR, lpproductbuf39 : windows_core::PWSTR, plgidlanguage : *mut super::LANGID, pdwversion : super::LPDWORD, lpnamebuf : windows_core::PWSTR, pcchnamebuf : super::LPDWORD, lppackagebuf : windows_core::PWSTR, pcchpackagebuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetProductInfoFromScriptW(szscriptfile.param().abi(), lpproductbuf39.unwrap_or(core::mem::zeroed()) as _, plgidlanguage.unwrap_or(core::mem::zeroed()) as _, pdwversion.unwrap_or(core::mem::zeroed()) as _, lpnamebuf.unwrap_or(core::mem::zeroed()) as _, pcchnamebuf.unwrap_or(core::mem::zeroed()) as _, lppackagebuf.unwrap_or(core::mem::zeroed()) as _, pcchpackagebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetProductInfoW<P0, P1>(szproduct: P0, szattribute: P1, lpvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoW(szproduct : windows_core::PCWSTR, szattribute : windows_core::PCWSTR, lpvaluebuf : windows_core::PWSTR, pcchvaluebuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetProductInfoW(szproduct.param().abi(), szattribute.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetProductPropertyA<P1>(hproduct: MSIHANDLE, szproperty: P1, lpvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<super::LPDWORD>) -> super::UINT
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductPropertyA(hproduct : MSIHANDLE, szproperty : windows_core::PCSTR, lpvaluebuf : windows_core::PSTR, pcchvaluebuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetProductPropertyA(hproduct, szproperty.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiGetProductPropertyW<P1>(hproduct: MSIHANDLE, szproperty: P1, lpvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<super::LPDWORD>) -> super::UINT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductPropertyW(hproduct : MSIHANDLE, szproperty : windows_core::PCWSTR, lpvaluebuf : windows_core::PWSTR, pcchvaluebuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiGetProductPropertyW(hproduct, szproperty.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiGetShortcutTargetA<P0>(szshortcutpath: P0, szproductcode: Option<windows_core::PSTR>, szfeatureid: Option<windows_core::PSTR>, szcomponentcode: Option<windows_core::PSTR>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetShortcutTargetA(szshortcutpath : windows_core::PCSTR, szproductcode : windows_core::PSTR, szfeatureid : windows_core::PSTR, szcomponentcode : windows_core::PSTR) -> super::UINT);
    unsafe { MsiGetShortcutTargetA(szshortcutpath.param().abi(), szproductcode.unwrap_or(core::mem::zeroed()) as _, szfeatureid.unwrap_or(core::mem::zeroed()) as _, szcomponentcode.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiGetShortcutTargetW<P0>(szshortcutpath: P0, szproductcode: Option<windows_core::PWSTR>, szfeatureid: Option<windows_core::PWSTR>, szcomponentcode: Option<windows_core::PWSTR>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetShortcutTargetW(szshortcutpath : windows_core::PCWSTR, szproductcode : windows_core::PWSTR, szfeatureid : windows_core::PWSTR, szcomponentcode : windows_core::PWSTR) -> super::UINT);
    unsafe { MsiGetShortcutTargetW(szshortcutpath.param().abi(), szproductcode.unwrap_or(core::mem::zeroed()) as _, szfeatureid.unwrap_or(core::mem::zeroed()) as _, szcomponentcode.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn MsiGetUserInfoA<P0>(szproduct: P0, lpusernamebuf: Option<windows_core::PSTR>, pcchusernamebuf: Option<super::LPDWORD>, lporgnamebuf: Option<windows_core::PSTR>, pcchorgnamebuf: Option<super::LPDWORD>, lpserialbuf: Option<windows_core::PSTR>, pcchserialbuf: Option<super::LPDWORD>) -> USERINFOSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetUserInfoA(szproduct : windows_core::PCSTR, lpusernamebuf : windows_core::PSTR, pcchusernamebuf : super::LPDWORD, lporgnamebuf : windows_core::PSTR, pcchorgnamebuf : super::LPDWORD, lpserialbuf : windows_core::PSTR, pcchserialbuf : super::LPDWORD) -> USERINFOSTATE);
    unsafe { MsiGetUserInfoA(szproduct.param().abi(), lpusernamebuf.unwrap_or(core::mem::zeroed()) as _, pcchusernamebuf.unwrap_or(core::mem::zeroed()) as _, lporgnamebuf.unwrap_or(core::mem::zeroed()) as _, pcchorgnamebuf.unwrap_or(core::mem::zeroed()) as _, lpserialbuf.unwrap_or(core::mem::zeroed()) as _, pcchserialbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn MsiGetUserInfoW<P0>(szproduct: P0, lpusernamebuf: Option<windows_core::PWSTR>, pcchusernamebuf: Option<super::LPDWORD>, lporgnamebuf: Option<windows_core::PWSTR>, pcchorgnamebuf: Option<super::LPDWORD>, lpserialbuf: Option<windows_core::PWSTR>, pcchserialbuf: Option<super::LPDWORD>) -> USERINFOSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetUserInfoW(szproduct : windows_core::PCWSTR, lpusernamebuf : windows_core::PWSTR, pcchusernamebuf : super::LPDWORD, lporgnamebuf : windows_core::PWSTR, pcchorgnamebuf : super::LPDWORD, lpserialbuf : windows_core::PWSTR, pcchserialbuf : super::LPDWORD) -> USERINFOSTATE);
    unsafe { MsiGetUserInfoW(szproduct.param().abi(), lpusernamebuf.unwrap_or(core::mem::zeroed()) as _, pcchusernamebuf.unwrap_or(core::mem::zeroed()) as _, lporgnamebuf.unwrap_or(core::mem::zeroed()) as _, pcchorgnamebuf.unwrap_or(core::mem::zeroed()) as _, lpserialbuf.unwrap_or(core::mem::zeroed()) as _, pcchserialbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiInstallMissingComponentA<P0, P1>(szproduct: P0, szcomponent: P1, einstallstate: INSTALLSTATE) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallMissingComponentA(szproduct : windows_core::PCSTR, szcomponent : windows_core::PCSTR, einstallstate : INSTALLSTATE) -> super::UINT);
    unsafe { MsiInstallMissingComponentA(szproduct.param().abi(), szcomponent.param().abi(), einstallstate) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiInstallMissingComponentW<P0, P1>(szproduct: P0, szcomponent: P1, einstallstate: INSTALLSTATE) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallMissingComponentW(szproduct : windows_core::PCWSTR, szcomponent : windows_core::PCWSTR, einstallstate : INSTALLSTATE) -> super::UINT);
    unsafe { MsiInstallMissingComponentW(szproduct.param().abi(), szcomponent.param().abi(), einstallstate) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiInstallMissingFileA<P0, P1>(szproduct: P0, szfile: P1) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallMissingFileA(szproduct : windows_core::PCSTR, szfile : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiInstallMissingFileA(szproduct.param().abi(), szfile.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiInstallMissingFileW<P0, P1>(szproduct: P0, szfile: P1) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallMissingFileW(szproduct : windows_core::PCWSTR, szfile : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiInstallMissingFileW(szproduct.param().abi(), szfile.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiInstallProductA<P0, P1>(szpackagepath: P0, szcommandline: P1) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallProductA(szpackagepath : windows_core::PCSTR, szcommandline : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiInstallProductA(szpackagepath.param().abi(), szcommandline.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiInstallProductW<P0, P1>(szpackagepath: P0, szcommandline: P1) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallProductW(szpackagepath : windows_core::PCWSTR, szcommandline : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiInstallProductW(szpackagepath.param().abi(), szcommandline.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiIsProductElevatedA<P0>(szproduct: P0, pfelevated: *mut windows_core::BOOL) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiIsProductElevatedA(szproduct : windows_core::PCSTR, pfelevated : *mut windows_core::BOOL) -> super::UINT);
    unsafe { MsiIsProductElevatedA(szproduct.param().abi(), pfelevated as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiIsProductElevatedW<P0>(szproduct: P0, pfelevated: *mut windows_core::BOOL) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiIsProductElevatedW(szproduct : windows_core::PCWSTR, pfelevated : *mut windows_core::BOOL) -> super::UINT);
    unsafe { MsiIsProductElevatedW(szproduct.param().abi(), pfelevated as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MsiJoinTransaction(htransactionhandle: MSIHANDLE, dwtransactionattributes: super::DWORD, phchangeofownerevent: *mut super::HANDLE) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiJoinTransaction(htransactionhandle : MSIHANDLE, dwtransactionattributes : super::DWORD, phchangeofownerevent : *mut super::HANDLE) -> super::UINT);
    unsafe { MsiJoinTransaction(htransactionhandle, dwtransactionattributes, phchangeofownerevent as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn MsiLocateComponentA<P0>(szcomponent: P0, lppathbuf: Option<windows_core::PSTR>, pcchbuf: Option<super::LPDWORD>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiLocateComponentA(szcomponent : windows_core::PCSTR, lppathbuf : windows_core::PSTR, pcchbuf : super::LPDWORD) -> INSTALLSTATE);
    unsafe { MsiLocateComponentA(szcomponent.param().abi(), lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn MsiLocateComponentW<P0>(szcomponent: P0, lppathbuf: Option<windows_core::PWSTR>, pcchbuf: Option<super::LPDWORD>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiLocateComponentW(szcomponent : windows_core::PCWSTR, lppathbuf : windows_core::PWSTR, pcchbuf : super::LPDWORD) -> INSTALLSTATE);
    unsafe { MsiLocateComponentW(szcomponent.param().abi(), lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiNotifySidChangeA<P0, P1>(poldsid: P0, pnewsid: P1) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiNotifySidChangeA(poldsid : windows_core::PCSTR, pnewsid : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiNotifySidChangeA(poldsid.param().abi(), pnewsid.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiNotifySidChangeW<P0, P1>(poldsid: P0, pnewsid: P1) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiNotifySidChangeW(poldsid : windows_core::PCWSTR, pnewsid : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiNotifySidChangeW(poldsid.param().abi(), pnewsid.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiOpenPackageA<P0>(szpackagepath: P0, hproduct: *mut MSIHANDLE) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenPackageA(szpackagepath : windows_core::PCSTR, hproduct : *mut MSIHANDLE) -> super::UINT);
    unsafe { MsiOpenPackageA(szpackagepath.param().abi(), hproduct as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiOpenPackageExA<P0>(szpackagepath: P0, dwoptions: super::DWORD, hproduct: *mut MSIHANDLE) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenPackageExA(szpackagepath : windows_core::PCSTR, dwoptions : super::DWORD, hproduct : *mut MSIHANDLE) -> super::UINT);
    unsafe { MsiOpenPackageExA(szpackagepath.param().abi(), dwoptions, hproduct as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiOpenPackageExW<P0>(szpackagepath: P0, dwoptions: super::DWORD, hproduct: *mut MSIHANDLE) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenPackageExW(szpackagepath : windows_core::PCWSTR, dwoptions : super::DWORD, hproduct : *mut MSIHANDLE) -> super::UINT);
    unsafe { MsiOpenPackageExW(szpackagepath.param().abi(), dwoptions, hproduct as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiOpenPackageW<P0>(szpackagepath: P0, hproduct: *mut MSIHANDLE) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenPackageW(szpackagepath : windows_core::PCWSTR, hproduct : *mut MSIHANDLE) -> super::UINT);
    unsafe { MsiOpenPackageW(szpackagepath.param().abi(), hproduct as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiOpenProductA<P0>(szproduct: P0, hproduct: *mut MSIHANDLE) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenProductA(szproduct : windows_core::PCSTR, hproduct : *mut MSIHANDLE) -> super::UINT);
    unsafe { MsiOpenProductA(szproduct.param().abi(), hproduct as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiOpenProductW<P0>(szproduct: P0, hproduct: *mut MSIHANDLE) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenProductW(szproduct : windows_core::PCWSTR, hproduct : *mut MSIHANDLE) -> super::UINT);
    unsafe { MsiOpenProductW(szproduct.param().abi(), hproduct as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiProcessAdvertiseScriptA<P0, P1>(szscriptfile: P0, sziconfolder: P1, hregdata: Option<super::HKEY>, fshortcuts: bool, fremoveitems: bool) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProcessAdvertiseScriptA(szscriptfile : windows_core::PCSTR, sziconfolder : windows_core::PCSTR, hregdata : super::HKEY, fshortcuts : windows_core::BOOL, fremoveitems : windows_core::BOOL) -> super::UINT);
    unsafe { MsiProcessAdvertiseScriptA(szscriptfile.param().abi(), sziconfolder.param().abi(), hregdata.unwrap_or(core::mem::zeroed()) as _, fshortcuts.into(), fremoveitems.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiProcessAdvertiseScriptW<P0, P1>(szscriptfile: P0, sziconfolder: P1, hregdata: Option<super::HKEY>, fshortcuts: bool, fremoveitems: bool) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProcessAdvertiseScriptW(szscriptfile : windows_core::PCWSTR, sziconfolder : windows_core::PCWSTR, hregdata : super::HKEY, fshortcuts : windows_core::BOOL, fremoveitems : windows_core::BOOL) -> super::UINT);
    unsafe { MsiProcessAdvertiseScriptW(szscriptfile.param().abi(), sziconfolder.param().abi(), hregdata.unwrap_or(core::mem::zeroed()) as _, fshortcuts.into(), fremoveitems.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiProvideAssemblyA<P0, P1>(szassemblyname: P0, szappcontext: P1, dwinstallmode: super::DWORD, dwassemblyinfo: super::DWORD, lppathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideAssemblyA(szassemblyname : windows_core::PCSTR, szappcontext : windows_core::PCSTR, dwinstallmode : super::DWORD, dwassemblyinfo : super::DWORD, lppathbuf : windows_core::PSTR, pcchpathbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiProvideAssemblyA(szassemblyname.param().abi(), szappcontext.param().abi(), dwinstallmode, dwassemblyinfo, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiProvideAssemblyW<P0, P1>(szassemblyname: P0, szappcontext: P1, dwinstallmode: super::DWORD, dwassemblyinfo: super::DWORD, lppathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideAssemblyW(szassemblyname : windows_core::PCWSTR, szappcontext : windows_core::PCWSTR, dwinstallmode : super::DWORD, dwassemblyinfo : super::DWORD, lppathbuf : windows_core::PWSTR, pcchpathbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiProvideAssemblyW(szassemblyname.param().abi(), szappcontext.param().abi(), dwinstallmode, dwassemblyinfo, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiProvideComponentA<P0, P1, P2>(szproduct: P0, szfeature: P1, szcomponent: P2, dwinstallmode: super::DWORD, lppathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideComponentA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR, szcomponent : windows_core::PCSTR, dwinstallmode : super::DWORD, lppathbuf : windows_core::PSTR, pcchpathbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiProvideComponentA(szproduct.param().abi(), szfeature.param().abi(), szcomponent.param().abi(), dwinstallmode, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiProvideComponentW<P0, P1, P2>(szproduct: P0, szfeature: P1, szcomponent: P2, dwinstallmode: super::DWORD, lppathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideComponentW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR, szcomponent : windows_core::PCWSTR, dwinstallmode : super::DWORD, lppathbuf : windows_core::PWSTR, pcchpathbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiProvideComponentW(szproduct.param().abi(), szfeature.param().abi(), szcomponent.param().abi(), dwinstallmode, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiProvideQualifiedComponentA<P0, P1>(szcategory: P0, szqualifier: P1, dwinstallmode: super::DWORD, lppathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideQualifiedComponentA(szcategory : windows_core::PCSTR, szqualifier : windows_core::PCSTR, dwinstallmode : super::DWORD, lppathbuf : windows_core::PSTR, pcchpathbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiProvideQualifiedComponentA(szcategory.param().abi(), szqualifier.param().abi(), dwinstallmode, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiProvideQualifiedComponentExA<P0, P1, P3>(szcategory: P0, szqualifier: P1, dwinstallmode: super::DWORD, szproduct: P3, dwunused1: super::DWORD, dwunused2: super::DWORD, lppathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideQualifiedComponentExA(szcategory : windows_core::PCSTR, szqualifier : windows_core::PCSTR, dwinstallmode : super::DWORD, szproduct : windows_core::PCSTR, dwunused1 : super::DWORD, dwunused2 : super::DWORD, lppathbuf : windows_core::PSTR, pcchpathbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiProvideQualifiedComponentExA(szcategory.param().abi(), szqualifier.param().abi(), dwinstallmode, szproduct.param().abi(), dwunused1, dwunused2, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiProvideQualifiedComponentExW<P0, P1, P3>(szcategory: P0, szqualifier: P1, dwinstallmode: super::DWORD, szproduct: P3, dwunused1: super::DWORD, dwunused2: super::DWORD, lppathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideQualifiedComponentExW(szcategory : windows_core::PCWSTR, szqualifier : windows_core::PCWSTR, dwinstallmode : super::DWORD, szproduct : windows_core::PCWSTR, dwunused1 : super::DWORD, dwunused2 : super::DWORD, lppathbuf : windows_core::PWSTR, pcchpathbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiProvideQualifiedComponentExW(szcategory.param().abi(), szqualifier.param().abi(), dwinstallmode, szproduct.param().abi(), dwunused1, dwunused2, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiProvideQualifiedComponentW<P0, P1>(szcategory: P0, szqualifier: P1, dwinstallmode: super::DWORD, lppathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideQualifiedComponentW(szcategory : windows_core::PCWSTR, szqualifier : windows_core::PCWSTR, dwinstallmode : super::DWORD, lppathbuf : windows_core::PWSTR, pcchpathbuf : super::LPDWORD) -> super::UINT);
    unsafe { MsiProvideQualifiedComponentW(szcategory.param().abi(), szqualifier.param().abi(), dwinstallmode, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiQueryComponentStateA<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: P3, pdwstate: Option<*mut INSTALLSTATE>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryComponentStateA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szcomponentcode : windows_core::PCSTR, pdwstate : *mut INSTALLSTATE) -> super::UINT);
    unsafe { MsiQueryComponentStateA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szcomponentcode.param().abi(), pdwstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiQueryComponentStateW<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: P3, pdwstate: Option<*mut INSTALLSTATE>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryComponentStateW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szcomponentcode : windows_core::PCWSTR, pdwstate : *mut INSTALLSTATE) -> super::UINT);
    unsafe { MsiQueryComponentStateW(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szcomponentcode.param().abi(), pdwstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiQueryFeatureStateA<P0, P1>(szproduct: P0, szfeature: P1) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryFeatureStateA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR) -> INSTALLSTATE);
    unsafe { MsiQueryFeatureStateA(szproduct.param().abi(), szfeature.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiQueryFeatureStateExA<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szfeature: P3, pdwstate: Option<*mut INSTALLSTATE>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryFeatureStateExA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szfeature : windows_core::PCSTR, pdwstate : *mut INSTALLSTATE) -> super::UINT);
    unsafe { MsiQueryFeatureStateExA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szfeature.param().abi(), pdwstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiQueryFeatureStateExW<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szfeature: P3, pdwstate: Option<*mut INSTALLSTATE>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryFeatureStateExW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szfeature : windows_core::PCWSTR, pdwstate : *mut INSTALLSTATE) -> super::UINT);
    unsafe { MsiQueryFeatureStateExW(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szfeature.param().abi(), pdwstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiQueryFeatureStateW<P0, P1>(szproduct: P0, szfeature: P1) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryFeatureStateW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR) -> INSTALLSTATE);
    unsafe { MsiQueryFeatureStateW(szproduct.param().abi(), szfeature.param().abi()) }
}
#[inline]
pub unsafe fn MsiQueryProductStateA<P0>(szproduct: P0) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryProductStateA(szproduct : windows_core::PCSTR) -> INSTALLSTATE);
    unsafe { MsiQueryProductStateA(szproduct.param().abi()) }
}
#[inline]
pub unsafe fn MsiQueryProductStateW<P0>(szproduct: P0) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryProductStateW(szproduct : windows_core::PCWSTR) -> INSTALLSTATE);
    unsafe { MsiQueryProductStateW(szproduct.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiReinstallFeatureA<P0, P1>(szproduct: P0, szfeature: P1, dwreinstallmode: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiReinstallFeatureA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR, dwreinstallmode : super::DWORD) -> super::UINT);
    unsafe { MsiReinstallFeatureA(szproduct.param().abi(), szfeature.param().abi(), dwreinstallmode) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiReinstallFeatureW<P0, P1>(szproduct: P0, szfeature: P1, dwreinstallmode: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiReinstallFeatureW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR, dwreinstallmode : super::DWORD) -> super::UINT);
    unsafe { MsiReinstallFeatureW(szproduct.param().abi(), szfeature.param().abi(), dwreinstallmode) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiReinstallProductA<P0>(szproduct: P0, szreinstallmode: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiReinstallProductA(szproduct : windows_core::PCSTR, szreinstallmode : super::DWORD) -> super::UINT);
    unsafe { MsiReinstallProductA(szproduct.param().abi(), szreinstallmode) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiReinstallProductW<P0>(szproduct: P0, szreinstallmode: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiReinstallProductW(szproduct : windows_core::PCWSTR, szreinstallmode : super::DWORD) -> super::UINT);
    unsafe { MsiReinstallProductW(szproduct.param().abi(), szreinstallmode) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiRemovePatchesA<P0, P1, P3>(szpatchlist: P0, szproductcode: P1, euninstalltype: INSTALLTYPE, szpropertylist: P3) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiRemovePatchesA(szpatchlist : windows_core::PCSTR, szproductcode : windows_core::PCSTR, euninstalltype : INSTALLTYPE, szpropertylist : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiRemovePatchesA(szpatchlist.param().abi(), szproductcode.param().abi(), euninstalltype, szpropertylist.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiRemovePatchesW<P0, P1, P3>(szpatchlist: P0, szproductcode: P1, euninstalltype: INSTALLTYPE, szpropertylist: P3) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiRemovePatchesW(szpatchlist : windows_core::PCWSTR, szproductcode : windows_core::PCWSTR, euninstalltype : INSTALLTYPE, szpropertylist : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiRemovePatchesW(szpatchlist.param().abi(), szproductcode.param().abi(), euninstalltype, szpropertylist.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiSetExternalUIA(puihandler: INSTALLUI_HANDLERA, dwmessagefilter: super::DWORD, pvcontext: Option<super::LPVOID>) -> INSTALLUI_HANDLERA {
    windows_core::link!("msi.dll" "system" fn MsiSetExternalUIA(puihandler : INSTALLUI_HANDLERA, dwmessagefilter : super::DWORD, pvcontext : super::LPVOID) -> INSTALLUI_HANDLERA);
    unsafe { MsiSetExternalUIA(puihandler, dwmessagefilter, pvcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiSetExternalUIRecord(puihandler: INSTALLUI_HANDLER_RECORD, dwmessagefilter: super::DWORD, pvcontext: Option<super::LPVOID>, ppuiprevhandler: Option<PINSTALLUI_HANDLER_RECORD>) -> super::UINT {
    windows_core::link!("msi.dll" "system" fn MsiSetExternalUIRecord(puihandler : INSTALLUI_HANDLER_RECORD, dwmessagefilter : super::DWORD, pvcontext : super::LPVOID, ppuiprevhandler : PINSTALLUI_HANDLER_RECORD) -> super::UINT);
    unsafe { MsiSetExternalUIRecord(puihandler, dwmessagefilter, pvcontext.unwrap_or(core::mem::zeroed()) as _, ppuiprevhandler.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiSetExternalUIW(puihandler: INSTALLUI_HANDLERW, dwmessagefilter: super::DWORD, pvcontext: Option<super::LPVOID>) -> INSTALLUI_HANDLERW {
    windows_core::link!("msi.dll" "system" fn MsiSetExternalUIW(puihandler : INSTALLUI_HANDLERW, dwmessagefilter : super::DWORD, pvcontext : super::LPVOID) -> INSTALLUI_HANDLERW);
    unsafe { MsiSetExternalUIW(puihandler, dwmessagefilter, pvcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MsiSetInternalUI(dwuilevel: INSTALLUILEVEL, phwnd: Option<*mut super::HWND>) -> INSTALLUILEVEL {
    windows_core::link!("msi.dll" "system" fn MsiSetInternalUI(dwuilevel : INSTALLUILEVEL, phwnd : *mut super::HWND) -> INSTALLUILEVEL);
    unsafe { MsiSetInternalUI(dwuilevel, phwnd.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListAddMediaDiskA<P0, P1, P5, P6>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD, dwdiskid: super::DWORD, szvolumelabel: P5, szdiskprompt: P6) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddMediaDiskA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD, dwdiskid : super::DWORD, szvolumelabel : windows_core::PCSTR, szdiskprompt : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiSourceListAddMediaDiskA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwdiskid, szvolumelabel.param().abi(), szdiskprompt.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListAddMediaDiskW<P0, P1, P5, P6>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD, dwdiskid: super::DWORD, szvolumelabel: P5, szdiskprompt: P6) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddMediaDiskW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD, dwdiskid : super::DWORD, szvolumelabel : windows_core::PCWSTR, szdiskprompt : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiSourceListAddMediaDiskW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwdiskid, szvolumelabel.param().abi(), szdiskprompt.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListAddSourceA<P0, P1, P3>(szproduct: P0, szusername: P1, dwreserved: super::DWORD, szsource: P3) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddSourceA(szproduct : windows_core::PCSTR, szusername : windows_core::PCSTR, dwreserved : super::DWORD, szsource : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiSourceListAddSourceA(szproduct.param().abi(), szusername.param().abi(), dwreserved, szsource.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListAddSourceExA<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD, szsource: P4, dwindex: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddSourceExA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD, szsource : windows_core::PCSTR, dwindex : super::DWORD) -> super::UINT);
    unsafe { MsiSourceListAddSourceExA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szsource.param().abi(), dwindex) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListAddSourceExW<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD, szsource: P4, dwindex: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddSourceExW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD, szsource : windows_core::PCWSTR, dwindex : super::DWORD) -> super::UINT);
    unsafe { MsiSourceListAddSourceExW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szsource.param().abi(), dwindex) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListAddSourceW<P0, P1, P3>(szproduct: P0, szusername: P1, dwreserved: super::DWORD, szsource: P3) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddSourceW(szproduct : windows_core::PCWSTR, szusername : windows_core::PCWSTR, dwreserved : super::DWORD, szsource : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiSourceListAddSourceW(szproduct.param().abi(), szusername.param().abi(), dwreserved, szsource.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListClearAllA<P0, P1>(szproduct: P0, szusername: P1, dwreserved: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearAllA(szproduct : windows_core::PCSTR, szusername : windows_core::PCSTR, dwreserved : super::DWORD) -> super::UINT);
    unsafe { MsiSourceListClearAllA(szproduct.param().abi(), szusername.param().abi(), dwreserved) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListClearAllExA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearAllExA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD) -> super::UINT);
    unsafe { MsiSourceListClearAllExA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListClearAllExW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearAllExW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD) -> super::UINT);
    unsafe { MsiSourceListClearAllExW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListClearAllW<P0, P1>(szproduct: P0, szusername: P1, dwreserved: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearAllW(szproduct : windows_core::PCWSTR, szusername : windows_core::PCWSTR, dwreserved : super::DWORD) -> super::UINT);
    unsafe { MsiSourceListClearAllW(szproduct.param().abi(), szusername.param().abi(), dwreserved) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListClearMediaDiskA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD, dwdiskid: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearMediaDiskA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD, dwdiskid : super::DWORD) -> super::UINT);
    unsafe { MsiSourceListClearMediaDiskA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwdiskid) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListClearMediaDiskW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD, dwdiskid: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearMediaDiskW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD, dwdiskid : super::DWORD) -> super::UINT);
    unsafe { MsiSourceListClearMediaDiskW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwdiskid) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListClearSourceA<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD, szsource: P4) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearSourceA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD, szsource : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiSourceListClearSourceA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szsource.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListClearSourceW<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD, szsource: P4) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearSourceW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD, szsource : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiSourceListClearSourceW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szsource.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiSourceListEnumMediaDisksA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD, dwindex: super::DWORD, pdwdiskid: Option<super::LPDWORD>, szvolumelabel: Option<windows_core::PSTR>, pcchvolumelabel: Option<super::LPDWORD>, szdiskprompt: Option<windows_core::PSTR>, pcchdiskprompt: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListEnumMediaDisksA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD, dwindex : super::DWORD, pdwdiskid : super::LPDWORD, szvolumelabel : windows_core::PSTR, pcchvolumelabel : super::LPDWORD, szdiskprompt : windows_core::PSTR, pcchdiskprompt : super::LPDWORD) -> super::UINT);
    unsafe { MsiSourceListEnumMediaDisksA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwindex, pdwdiskid.unwrap_or(core::mem::zeroed()) as _, szvolumelabel.unwrap_or(core::mem::zeroed()) as _, pcchvolumelabel.unwrap_or(core::mem::zeroed()) as _, szdiskprompt.unwrap_or(core::mem::zeroed()) as _, pcchdiskprompt.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiSourceListEnumMediaDisksW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD, dwindex: super::DWORD, pdwdiskid: Option<super::LPDWORD>, szvolumelabel: Option<windows_core::PWSTR>, pcchvolumelabel: Option<super::LPDWORD>, szdiskprompt: Option<windows_core::PWSTR>, pcchdiskprompt: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListEnumMediaDisksW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD, dwindex : super::DWORD, pdwdiskid : super::LPDWORD, szvolumelabel : windows_core::PWSTR, pcchvolumelabel : super::LPDWORD, szdiskprompt : windows_core::PWSTR, pcchdiskprompt : super::LPDWORD) -> super::UINT);
    unsafe { MsiSourceListEnumMediaDisksW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwindex, pdwdiskid.unwrap_or(core::mem::zeroed()) as _, szvolumelabel.unwrap_or(core::mem::zeroed()) as _, pcchvolumelabel.unwrap_or(core::mem::zeroed()) as _, szdiskprompt.unwrap_or(core::mem::zeroed()) as _, pcchdiskprompt.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiSourceListEnumSourcesA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD, dwindex: super::DWORD, szsource: Option<windows_core::PSTR>, pcchsource: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListEnumSourcesA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD, dwindex : super::DWORD, szsource : windows_core::PSTR, pcchsource : super::LPDWORD) -> super::UINT);
    unsafe { MsiSourceListEnumSourcesA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwindex, szsource.unwrap_or(core::mem::zeroed()) as _, pcchsource.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiSourceListEnumSourcesW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD, dwindex: super::DWORD, szsource: Option<windows_core::PWSTR>, pcchsource: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListEnumSourcesW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD, dwindex : super::DWORD, szsource : windows_core::PWSTR, pcchsource : super::LPDWORD) -> super::UINT);
    unsafe { MsiSourceListEnumSourcesW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwindex, szsource.unwrap_or(core::mem::zeroed()) as _, pcchsource.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListForceResolutionA<P0, P1>(szproduct: P0, szusername: P1, dwreserved: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListForceResolutionA(szproduct : windows_core::PCSTR, szusername : windows_core::PCSTR, dwreserved : super::DWORD) -> super::UINT);
    unsafe { MsiSourceListForceResolutionA(szproduct.param().abi(), szusername.param().abi(), dwreserved) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListForceResolutionExA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListForceResolutionExA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD) -> super::UINT);
    unsafe { MsiSourceListForceResolutionExA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListForceResolutionExW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListForceResolutionExW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD) -> super::UINT);
    unsafe { MsiSourceListForceResolutionExW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListForceResolutionW<P0, P1>(szproduct: P0, szusername: P1, dwreserved: super::DWORD) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListForceResolutionW(szproduct : windows_core::PCWSTR, szusername : windows_core::PCWSTR, dwreserved : super::DWORD) -> super::UINT);
    unsafe { MsiSourceListForceResolutionW(szproduct.param().abi(), szusername.param().abi(), dwreserved) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiSourceListGetInfoA<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD, szproperty: P4, szvalue: Option<windows_core::PSTR>, pcchvalue: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListGetInfoA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD, szproperty : windows_core::PCSTR, szvalue : windows_core::PSTR, pcchvalue : super::LPDWORD) -> super::UINT);
    unsafe { MsiSourceListGetInfoA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szproperty.param().abi(), szvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn MsiSourceListGetInfoW<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD, szproperty: P4, szvalue: Option<windows_core::PWSTR>, pcchvalue: Option<super::LPDWORD>) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListGetInfoW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD, szproperty : windows_core::PCWSTR, szvalue : windows_core::PWSTR, pcchvalue : super::LPDWORD) -> super::UINT);
    unsafe { MsiSourceListGetInfoW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szproperty.param().abi(), szvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListSetInfoA<P0, P1, P4, P5>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD, szproperty: P4, szvalue: P5) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListSetInfoA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD, szproperty : windows_core::PCSTR, szvalue : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiSourceListSetInfoA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szproperty.param().abi(), szvalue.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiSourceListSetInfoW<P0, P1, P4, P5>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: super::DWORD, szproperty: P4, szvalue: P5) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListSetInfoW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : super::DWORD, szproperty : windows_core::PCWSTR, szvalue : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiSourceListSetInfoW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szproperty.param().abi(), szvalue.param().abi()) }
}
#[inline]
pub unsafe fn MsiUseFeatureA<P0, P1>(szproduct: P0, szfeature: P1) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiUseFeatureA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR) -> INSTALLSTATE);
    unsafe { MsiUseFeatureA(szproduct.param().abi(), szfeature.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiUseFeatureExA<P0, P1>(szproduct: P0, szfeature: P1, dwinstallmode: super::DWORD, dwreserved: super::DWORD) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiUseFeatureExA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR, dwinstallmode : super::DWORD, dwreserved : super::DWORD) -> INSTALLSTATE);
    unsafe { MsiUseFeatureExA(szproduct.param().abi(), szfeature.param().abi(), dwinstallmode, dwreserved) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiUseFeatureExW<P0, P1>(szproduct: P0, szfeature: P1, dwinstallmode: super::DWORD, dwreserved: super::DWORD) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiUseFeatureExW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR, dwinstallmode : super::DWORD, dwreserved : super::DWORD) -> INSTALLSTATE);
    unsafe { MsiUseFeatureExW(szproduct.param().abi(), szfeature.param().abi(), dwinstallmode, dwreserved) }
}
#[inline]
pub unsafe fn MsiUseFeatureW<P0, P1>(szproduct: P0, szfeature: P1) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiUseFeatureW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR) -> INSTALLSTATE);
    unsafe { MsiUseFeatureW(szproduct.param().abi(), szfeature.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiVerifyPackageA<P0>(szpackagepath: P0) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiVerifyPackageA(szpackagepath : windows_core::PCSTR) -> super::UINT);
    unsafe { MsiVerifyPackageA(szpackagepath.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MsiVerifyPackageW<P0>(szpackagepath: P0) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiVerifyPackageW(szpackagepath : windows_core::PCWSTR) -> super::UINT);
    unsafe { MsiVerifyPackageW(szpackagepath.param().abi()) }
}
pub type ADVERTISEFLAGS = i32;
pub const ADVERTISEFLAGS_MACHINEASSIGN: ADVERTISEFLAGS = 0;
pub const ADVERTISEFLAGS_USERASSIGN: ADVERTISEFLAGS = 1;
pub const ERROR_ROLLBACK_DISABLED: i32 = 1653;
pub type INSTALLFEATUREATTRIBUTE = i32;
pub const INSTALLFEATUREATTRIBUTE_DISALLOWADVERTISE: INSTALLFEATUREATTRIBUTE = 16;
pub const INSTALLFEATUREATTRIBUTE_FAVORADVERTISE: INSTALLFEATUREATTRIBUTE = 8;
pub const INSTALLFEATUREATTRIBUTE_FAVORLOCAL: INSTALLFEATUREATTRIBUTE = 1;
pub const INSTALLFEATUREATTRIBUTE_FAVORSOURCE: INSTALLFEATUREATTRIBUTE = 2;
pub const INSTALLFEATUREATTRIBUTE_FOLLOWPARENT: INSTALLFEATUREATTRIBUTE = 4;
pub const INSTALLFEATUREATTRIBUTE_NOUNSUPPORTEDADVERTISE: INSTALLFEATUREATTRIBUTE = 32;
pub type INSTALLLEVEL = i32;
pub const INSTALLLEVEL_DEFAULT: INSTALLLEVEL = 0;
pub const INSTALLLEVEL_MAXIMUM: INSTALLLEVEL = 65535;
pub const INSTALLLEVEL_MINIMUM: INSTALLLEVEL = 1;
pub type INSTALLLOGATTRIBUTES = i32;
pub const INSTALLLOGATTRIBUTES_APPEND: INSTALLLOGATTRIBUTES = 1;
pub const INSTALLLOGATTRIBUTES_FLUSHEACHLINE: INSTALLLOGATTRIBUTES = 2;
pub type INSTALLLOGMODE = i32;
pub const INSTALLLOGMODE_ACTIONDATA: INSTALLLOGMODE = 512;
pub const INSTALLLOGMODE_ACTIONSTART: INSTALLLOGMODE = 256;
pub const INSTALLLOGMODE_COMMONDATA: INSTALLLOGMODE = 2048;
pub const INSTALLLOGMODE_ERROR: INSTALLLOGMODE = 2;
pub const INSTALLLOGMODE_EXTRADEBUG: INSTALLLOGMODE = 8192;
pub const INSTALLLOGMODE_FATALEXIT: INSTALLLOGMODE = 1;
pub const INSTALLLOGMODE_FILESINUSE: INSTALLLOGMODE = 32;
pub const INSTALLLOGMODE_INFO: INSTALLLOGMODE = 16;
pub const INSTALLLOGMODE_INITIALIZE: INSTALLLOGMODE = 4096;
pub const INSTALLLOGMODE_INSTALLEND: INSTALLLOGMODE = 134217728;
pub const INSTALLLOGMODE_INSTALLSTART: INSTALLLOGMODE = 67108864;
pub const INSTALLLOGMODE_LOGONLYONERROR: INSTALLLOGMODE = 16384;
pub const INSTALLLOGMODE_LOGPERFORMANCE: INSTALLLOGMODE = 32768;
pub const INSTALLLOGMODE_OUTOFDISKSPACE: INSTALLLOGMODE = 128;
pub const INSTALLLOGMODE_PROGRESS: INSTALLLOGMODE = 1024;
pub const INSTALLLOGMODE_PROPERTYDUMP: INSTALLLOGMODE = 1024;
pub const INSTALLLOGMODE_RESOLVESOURCE: INSTALLLOGMODE = 64;
pub const INSTALLLOGMODE_RMFILESINUSE: INSTALLLOGMODE = 33554432;
pub const INSTALLLOGMODE_SHOWDIALOG: INSTALLLOGMODE = 16384;
pub const INSTALLLOGMODE_TERMINATE: INSTALLLOGMODE = 8192;
pub const INSTALLLOGMODE_USER: INSTALLLOGMODE = 8;
pub const INSTALLLOGMODE_VERBOSE: INSTALLLOGMODE = 4096;
pub const INSTALLLOGMODE_WARNING: INSTALLLOGMODE = 4;
pub type INSTALLMESSAGE = i32;
pub const INSTALLMESSAGE_ACTIONDATA: INSTALLMESSAGE = 150994944;
pub const INSTALLMESSAGE_ACTIONSTART: INSTALLMESSAGE = 134217728;
pub const INSTALLMESSAGE_COMMONDATA: INSTALLMESSAGE = 184549376;
pub const INSTALLMESSAGE_ERROR: INSTALLMESSAGE = 16777216;
pub const INSTALLMESSAGE_FATALEXIT: INSTALLMESSAGE = 0;
pub const INSTALLMESSAGE_FILESINUSE: INSTALLMESSAGE = 83886080;
pub const INSTALLMESSAGE_INFO: INSTALLMESSAGE = 67108864;
pub const INSTALLMESSAGE_INITIALIZE: INSTALLMESSAGE = 201326592;
pub const INSTALLMESSAGE_INSTALLEND: INSTALLMESSAGE = 452984832;
pub const INSTALLMESSAGE_INSTALLSTART: INSTALLMESSAGE = 436207616;
pub const INSTALLMESSAGE_OUTOFDISKSPACE: INSTALLMESSAGE = 117440512;
pub const INSTALLMESSAGE_PERFORMANCE: INSTALLMESSAGE = 251658240;
pub const INSTALLMESSAGE_PROGRESS: INSTALLMESSAGE = 167772160;
pub const INSTALLMESSAGE_RESOLVESOURCE: INSTALLMESSAGE = 100663296;
pub const INSTALLMESSAGE_RMFILESINUSE: INSTALLMESSAGE = 419430400;
pub const INSTALLMESSAGE_SHOWDIALOG: INSTALLMESSAGE = 234881024;
pub const INSTALLMESSAGE_TERMINATE: INSTALLMESSAGE = 218103808;
pub const INSTALLMESSAGE_USER: INSTALLMESSAGE = 50331648;
pub const INSTALLMESSAGE_WARNING: INSTALLMESSAGE = 33554432;
pub type INSTALLMODE = i32;
pub const INSTALLMODE_DEFAULT: INSTALLMODE = 0;
pub const INSTALLMODE_EXISTING: INSTALLMODE = -1;
pub const INSTALLMODE_NODETECTION: INSTALLMODE = -2;
pub const INSTALLMODE_NODETECTION_ANY: INSTALLMODE = -4;
pub const INSTALLMODE_NOSOURCERESOLUTION: INSTALLMODE = -3;
pub const INSTALLPROPERTY_ASSIGNMENTTYPE: windows_core::PCSTR = windows_core::s!("AssignmentType");
pub const INSTALLPROPERTY_AUTHORIZED_LUA_APP: windows_core::PCSTR = windows_core::s!("AuthorizedLUAApp");
pub const INSTALLPROPERTY_DISKPROMPT: windows_core::PCSTR = windows_core::s!("DiskPrompt");
pub const INSTALLPROPERTY_DISPLAYNAME: windows_core::PCSTR = windows_core::s!("DisplayName");
pub const INSTALLPROPERTY_HELPLINK: windows_core::PCSTR = windows_core::s!("HelpLink");
pub const INSTALLPROPERTY_HELPTELEPHONE: windows_core::PCSTR = windows_core::s!("HelpTelephone");
pub const INSTALLPROPERTY_INSTALLDATE: windows_core::PCSTR = windows_core::s!("InstallDate");
pub const INSTALLPROPERTY_INSTALLEDLANGUAGE: windows_core::PCSTR = windows_core::s!("InstalledLanguage");
pub const INSTALLPROPERTY_INSTALLEDPRODUCTNAME: windows_core::PCSTR = windows_core::s!("InstalledProductName");
pub const INSTALLPROPERTY_INSTALLLOCATION: windows_core::PCSTR = windows_core::s!("InstallLocation");
pub const INSTALLPROPERTY_INSTALLSOURCE: windows_core::PCSTR = windows_core::s!("InstallSource");
pub const INSTALLPROPERTY_INSTANCETYPE: windows_core::PCSTR = windows_core::s!("InstanceType");
pub const INSTALLPROPERTY_LANGUAGE: windows_core::PCSTR = windows_core::s!("Language");
pub const INSTALLPROPERTY_LASTUSEDSOURCE: windows_core::PCSTR = windows_core::s!("LastUsedSource");
pub const INSTALLPROPERTY_LASTUSEDTYPE: windows_core::PCSTR = windows_core::s!("LastUsedType");
pub const INSTALLPROPERTY_LOCALPACKAGE: windows_core::PCSTR = windows_core::s!("LocalPackage");
pub const INSTALLPROPERTY_LUAENABLED: windows_core::PCSTR = windows_core::s!("LUAEnabled");
pub const INSTALLPROPERTY_MEDIAPACKAGEPATH: windows_core::PCSTR = windows_core::s!("MediaPackagePath");
pub const INSTALLPROPERTY_MOREINFOURL: windows_core::PCSTR = windows_core::s!("MoreInfoURL");
pub const INSTALLPROPERTY_PACKAGECODE: windows_core::PCSTR = windows_core::s!("PackageCode");
pub const INSTALLPROPERTY_PACKAGENAME: windows_core::PCSTR = windows_core::s!("PackageName");
pub const INSTALLPROPERTY_PATCHSTATE: windows_core::PCSTR = windows_core::s!("State");
pub const INSTALLPROPERTY_PATCHTYPE: windows_core::PCSTR = windows_core::s!("PatchType");
pub const INSTALLPROPERTY_PRODUCTICON: windows_core::PCSTR = windows_core::s!("ProductIcon");
pub const INSTALLPROPERTY_PRODUCTID: windows_core::PCSTR = windows_core::s!("ProductID");
pub const INSTALLPROPERTY_PRODUCTNAME: windows_core::PCSTR = windows_core::s!("ProductName");
pub const INSTALLPROPERTY_PRODUCTSTATE: windows_core::PCSTR = windows_core::s!("State");
pub const INSTALLPROPERTY_PUBLISHER: windows_core::PCSTR = windows_core::s!("Publisher");
pub const INSTALLPROPERTY_REGCOMPANY: windows_core::PCSTR = windows_core::s!("RegCompany");
pub const INSTALLPROPERTY_REGOWNER: windows_core::PCSTR = windows_core::s!("RegOwner");
pub const INSTALLPROPERTY_TRANSFORMS: windows_core::PCSTR = windows_core::s!("Transforms");
pub const INSTALLPROPERTY_UNINSTALLABLE: windows_core::PCSTR = windows_core::s!("Uninstallable");
pub const INSTALLPROPERTY_URLINFOABOUT: windows_core::PCSTR = windows_core::s!("URLInfoAbout");
pub const INSTALLPROPERTY_URLUPDATEINFO: windows_core::PCSTR = windows_core::s!("URLUpdateInfo");
pub const INSTALLPROPERTY_VERSION: windows_core::PCSTR = windows_core::s!("Version");
pub const INSTALLPROPERTY_VERSIONMAJOR: windows_core::PCSTR = windows_core::s!("VersionMajor");
pub const INSTALLPROPERTY_VERSIONMINOR: windows_core::PCSTR = windows_core::s!("VersionMinor");
pub const INSTALLPROPERTY_VERSIONSTRING: windows_core::PCSTR = windows_core::s!("VersionString");
pub type INSTALLSTATE = i32;
pub const INSTALLSTATE_ABSENT: INSTALLSTATE = 2;
pub const INSTALLSTATE_ADVERTISED: INSTALLSTATE = 1;
pub const INSTALLSTATE_BADCONFIG: INSTALLSTATE = -6;
pub const INSTALLSTATE_BROKEN: INSTALLSTATE = 0;
pub const INSTALLSTATE_DEFAULT: INSTALLSTATE = 5;
pub const INSTALLSTATE_INCOMPLETE: INSTALLSTATE = -5;
pub const INSTALLSTATE_INVALIDARG: INSTALLSTATE = -2;
pub const INSTALLSTATE_LOCAL: INSTALLSTATE = 3;
pub const INSTALLSTATE_MOREDATA: INSTALLSTATE = -3;
pub const INSTALLSTATE_NOTUSED: INSTALLSTATE = -7;
pub const INSTALLSTATE_REMOVED: INSTALLSTATE = 1;
pub const INSTALLSTATE_SOURCE: INSTALLSTATE = 4;
pub const INSTALLSTATE_SOURCEABSENT: INSTALLSTATE = -4;
pub const INSTALLSTATE_UNKNOWN: INSTALLSTATE = -1;
pub type INSTALLTYPE = i32;
pub const INSTALLTYPE_DEFAULT: INSTALLTYPE = 0;
pub const INSTALLTYPE_NETWORK_IMAGE: INSTALLTYPE = 1;
pub const INSTALLTYPE_SINGLE_INSTANCE: INSTALLTYPE = 2;
pub type INSTALLUILEVEL = i32;
pub const INSTALLUILEVEL_BASIC: INSTALLUILEVEL = 3;
pub const INSTALLUILEVEL_DEFAULT: INSTALLUILEVEL = 1;
pub const INSTALLUILEVEL_ENDDIALOG: INSTALLUILEVEL = 128;
pub const INSTALLUILEVEL_FULL: INSTALLUILEVEL = 5;
pub const INSTALLUILEVEL_HIDECANCEL: INSTALLUILEVEL = 32;
pub const INSTALLUILEVEL_NOCHANGE: INSTALLUILEVEL = 0;
pub const INSTALLUILEVEL_NONE: INSTALLUILEVEL = 2;
pub const INSTALLUILEVEL_PROGRESSONLY: INSTALLUILEVEL = 64;
pub const INSTALLUILEVEL_REDUCED: INSTALLUILEVEL = 4;
pub const INSTALLUILEVEL_SOURCERESONLY: INSTALLUILEVEL = 256;
pub const INSTALLUILEVEL_UACONLY: INSTALLUILEVEL = 512;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type INSTALLUI_HANDLERA = Option<unsafe extern "system" fn(pvcontext: super::LPVOID, imessagetype: super::UINT, szmessage: windows_core::PCSTR) -> i32>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type INSTALLUI_HANDLERW = Option<unsafe extern "system" fn(pvcontext: super::LPVOID, imessagetype: super::UINT, szmessage: windows_core::PCWSTR) -> i32>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type INSTALLUI_HANDLER_RECORD = Option<unsafe extern "system" fn(pvcontext: super::LPVOID, imessagetype: super::UINT, hrecord: MSIHANDLE) -> i32>;
pub const MAX_FEATURE_CHARS: i32 = 38;
pub const MAX_GUID_CHARS: i32 = 38;
pub type MSIADVERTISEOPTIONFLAGS = i32;
pub const MSIADVERTISEOPTIONFLAGS_INSTANCE: MSIADVERTISEOPTIONFLAGS = 1;
pub type MSIARCHITECTUREFLAGS = i32;
pub const MSIARCHITECTUREFLAGS_AMD64: MSIARCHITECTUREFLAGS = 4;
pub const MSIARCHITECTUREFLAGS_ARM: MSIARCHITECTUREFLAGS = 8;
pub const MSIARCHITECTUREFLAGS_IA64: MSIARCHITECTUREFLAGS = 2;
pub const MSIARCHITECTUREFLAGS_X86: MSIARCHITECTUREFLAGS = 1;
pub const MSIASSEMBLYINFO_NETASSEMBLY: i32 = 0;
pub const MSIASSEMBLYINFO_WIN32ASSEMBLY: i32 = 1;
pub type MSICODE = i32;
pub const MSICODE_PATCH: MSICODE = 1073741824;
pub const MSICODE_PRODUCT: MSICODE = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MSIFILEHASHINFO {
    pub dwFileHashInfoSize: super::ULONG,
    pub dwData: [super::ULONG; 4],
}
#[cfg(feature = "shared")]
impl Default for MSIFILEHASHINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MSIHANDLE = u32;
pub type MSIINSTALLCONTEXT = i32;
pub const MSIINSTALLCONTEXT_ALL: MSIINSTALLCONTEXT = 7;
pub const MSIINSTALLCONTEXT_ALLUSERMANAGED: MSIINSTALLCONTEXT = 8;
pub const MSIINSTALLCONTEXT_FIRSTVISIBLE: MSIINSTALLCONTEXT = 0;
pub const MSIINSTALLCONTEXT_MACHINE: MSIINSTALLCONTEXT = 4;
pub const MSIINSTALLCONTEXT_NONE: MSIINSTALLCONTEXT = 0;
pub const MSIINSTALLCONTEXT_USERMANAGED: MSIINSTALLCONTEXT = 1;
pub const MSIINSTALLCONTEXT_USERUNMANAGED: MSIINSTALLCONTEXT = 2;
pub type MSIOPENPACKAGEFLAGS = i32;
pub const MSIOPENPACKAGEFLAGS_IGNOREMACHINESTATE: MSIOPENPACKAGEFLAGS = 1;
pub type MSIPATCHDATATYPE = i32;
#[cfg(feature = "shared")]
pub type MSIPATCHSEQUENCEINFO = MSIPATCHSEQUENCEINFOA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSIPATCHSEQUENCEINFOA {
    pub szPatchData: windows_core::PCSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: super::DWORD,
    pub uStatus: super::UINT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MSIPATCHSEQUENCEINFOW {
    pub szPatchData: windows_core::PCWSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: super::DWORD,
    pub uStatus: super::UINT,
}
pub type MSIPATCHSTATE = i32;
pub const MSIPATCHSTATE_ALL: MSIPATCHSTATE = 15;
pub const MSIPATCHSTATE_APPLIED: MSIPATCHSTATE = 1;
pub const MSIPATCHSTATE_INVALID: MSIPATCHSTATE = 0;
pub const MSIPATCHSTATE_OBSOLETED: MSIPATCHSTATE = 4;
pub const MSIPATCHSTATE_REGISTERED: MSIPATCHSTATE = 8;
pub const MSIPATCHSTATE_SUPERSEDED: MSIPATCHSTATE = 2;
pub const MSIPATCH_DATATYPE_PATCHFILE: MSIPATCHDATATYPE = 0;
pub const MSIPATCH_DATATYPE_XMLBLOB: MSIPATCHDATATYPE = 2;
pub const MSIPATCH_DATATYPE_XMLPATH: MSIPATCHDATATYPE = 1;
pub type MSISOURCETYPE = i32;
pub const MSISOURCETYPE_MEDIA: MSISOURCETYPE = 4;
pub const MSISOURCETYPE_NETWORK: MSISOURCETYPE = 1;
pub const MSISOURCETYPE_UNKNOWN: MSISOURCETYPE = 0;
pub const MSISOURCETYPE_URL: MSISOURCETYPE = 2;
pub type MSITRANSACTION = i32;
pub type MSITRANSACTIONSTATE = i32;
pub const MSITRANSACTIONSTATE_COMMIT: MSITRANSACTIONSTATE = 1;
pub const MSITRANSACTIONSTATE_ROLLBACK: MSITRANSACTIONSTATE = 0;
pub const MSITRANSACTION_CHAIN_EMBEDDEDUI: MSITRANSACTION = 1;
pub const MSITRANSACTION_JOIN_EXISTING_EMBEDDEDUI: MSITRANSACTION = 2;
pub const MSI_INVALID_HASH_IS_FATAL: i32 = 1;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PINSTALLUI_HANDLER_RECORD = *mut INSTALLUI_HANDLER_RECORD;
#[cfg(feature = "shared")]
pub type PMSIFILEHASHINFO = *mut MSIFILEHASHINFO;
pub type PMSIPATCHDATATYPE = *mut MSIPATCHDATATYPE;
#[cfg(feature = "shared")]
pub type PMSIPATCHSEQUENCEINFO = *mut MSIPATCHSEQUENCEINFOA;
#[cfg(feature = "shared")]
pub type PMSIPATCHSEQUENCEINFOA = *mut MSIPATCHSEQUENCEINFOA;
#[cfg(feature = "shared")]
pub type PMSIPATCHSEQUENCEINFOW = *mut MSIPATCHSEQUENCEINFOW;
pub type REINSTALLMODE = i32;
pub const REINSTALLMODE_FILEEQUALVERSION: REINSTALLMODE = 8;
pub const REINSTALLMODE_FILEEXACT: REINSTALLMODE = 16;
pub const REINSTALLMODE_FILEMISSING: REINSTALLMODE = 2;
pub const REINSTALLMODE_FILEOLDERVERSION: REINSTALLMODE = 4;
pub const REINSTALLMODE_FILEREPLACE: REINSTALLMODE = 64;
pub const REINSTALLMODE_FILEVERIFY: REINSTALLMODE = 32;
pub const REINSTALLMODE_MACHINEDATA: REINSTALLMODE = 128;
pub const REINSTALLMODE_PACKAGE: REINSTALLMODE = 1024;
pub const REINSTALLMODE_REPAIR: REINSTALLMODE = 1;
pub const REINSTALLMODE_SHORTCUT: REINSTALLMODE = 512;
pub const REINSTALLMODE_USERDATA: REINSTALLMODE = 256;
pub type SCRIPTFLAGS = i32;
pub const SCRIPTFLAGS_CACHEINFO: SCRIPTFLAGS = 1;
pub const SCRIPTFLAGS_MACHINEASSIGN: SCRIPTFLAGS = 8;
pub const SCRIPTFLAGS_REGDATA: SCRIPTFLAGS = 416;
pub const SCRIPTFLAGS_REGDATA_APPINFO: SCRIPTFLAGS = 384;
pub const SCRIPTFLAGS_REGDATA_CLASSINFO: SCRIPTFLAGS = 128;
pub const SCRIPTFLAGS_REGDATA_CNFGINFO: SCRIPTFLAGS = 32;
pub const SCRIPTFLAGS_REGDATA_EXTENSIONINFO: SCRIPTFLAGS = 256;
pub const SCRIPTFLAGS_SHORTCUTS: SCRIPTFLAGS = 4;
pub const SCRIPTFLAGS_VALIDATE_TRANSFORMS_LIST: SCRIPTFLAGS = 64;
pub type USERINFOSTATE = i32;
pub const USERINFOSTATE_ABSENT: USERINFOSTATE = 0;
pub const USERINFOSTATE_INVALIDARG: USERINFOSTATE = -2;
pub const USERINFOSTATE_MOREDATA: USERINFOSTATE = -3;
pub const USERINFOSTATE_PRESENT: USERINFOSTATE = 1;
pub const USERINFOSTATE_UNKNOWN: USERINFOSTATE = -1;
pub const _WIN32_MSI: i32 = 500;
