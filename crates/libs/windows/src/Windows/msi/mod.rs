#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MsiAdvertiseProductA<P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: super::winnt::LANGID) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseProductA(szpackagepath : windows_core::PCSTR, szscriptfilepath : windows_core::PCSTR, sztransforms : windows_core::PCSTR, lgidlanguage : super::winnt::LANGID) -> u32);
    unsafe { MsiAdvertiseProductA(szpackagepath.param().abi(), szscriptfilepath.param().abi(), sztransforms.param().abi(), lgidlanguage) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MsiAdvertiseProductExA<P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: super::winnt::LANGID, dwplatform: u32, dwoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseProductExA(szpackagepath : windows_core::PCSTR, szscriptfilepath : windows_core::PCSTR, sztransforms : windows_core::PCSTR, lgidlanguage : super::winnt::LANGID, dwplatform : u32, dwoptions : u32) -> u32);
    unsafe { MsiAdvertiseProductExA(szpackagepath.param().abi(), szscriptfilepath.param().abi(), sztransforms.param().abi(), lgidlanguage, dwplatform, dwoptions) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MsiAdvertiseProductExW<P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: super::winnt::LANGID, dwplatform: u32, dwoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseProductExW(szpackagepath : windows_core::PCWSTR, szscriptfilepath : windows_core::PCWSTR, sztransforms : windows_core::PCWSTR, lgidlanguage : super::winnt::LANGID, dwplatform : u32, dwoptions : u32) -> u32);
    unsafe { MsiAdvertiseProductExW(szpackagepath.param().abi(), szscriptfilepath.param().abi(), sztransforms.param().abi(), lgidlanguage, dwplatform, dwoptions) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MsiAdvertiseProductW<P0, P1, P2>(szpackagepath: P0, szscriptfilepath: P1, sztransforms: P2, lgidlanguage: super::winnt::LANGID) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseProductW(szpackagepath : windows_core::PCWSTR, szscriptfilepath : windows_core::PCWSTR, sztransforms : windows_core::PCWSTR, lgidlanguage : super::winnt::LANGID) -> u32);
    unsafe { MsiAdvertiseProductW(szpackagepath.param().abi(), szscriptfilepath.param().abi(), sztransforms.param().abi(), lgidlanguage) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn MsiAdvertiseScriptA<P0>(szscriptfile: P0, dwflags: u32, phregdata: Option<*const super::minwindef::HKEY>, fremoveitems: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseScriptA(szscriptfile : windows_core::PCSTR, dwflags : u32, phregdata : *const super::minwindef::HKEY, fremoveitems : windows_core::BOOL) -> u32);
    unsafe { MsiAdvertiseScriptA(szscriptfile.param().abi(), dwflags, phregdata.unwrap_or(core::mem::zeroed()) as _, fremoveitems.into()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn MsiAdvertiseScriptW<P0>(szscriptfile: P0, dwflags: u32, phregdata: Option<*const super::minwindef::HKEY>, fremoveitems: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiAdvertiseScriptW(szscriptfile : windows_core::PCWSTR, dwflags : u32, phregdata : *const super::minwindef::HKEY, fremoveitems : windows_core::BOOL) -> u32);
    unsafe { MsiAdvertiseScriptW(szscriptfile.param().abi(), dwflags, phregdata.unwrap_or(core::mem::zeroed()) as _, fremoveitems.into()) }
}
#[inline]
pub unsafe fn MsiApplyMultiplePatchesA<P0, P1, P2>(szpatchpackages: P0, szproductcode: P1, szpropertieslist: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiApplyMultiplePatchesA(szpatchpackages : windows_core::PCSTR, szproductcode : windows_core::PCSTR, szpropertieslist : windows_core::PCSTR) -> u32);
    unsafe { MsiApplyMultiplePatchesA(szpatchpackages.param().abi(), szproductcode.param().abi(), szpropertieslist.param().abi()) }
}
#[inline]
pub unsafe fn MsiApplyMultiplePatchesW<P0, P1, P2>(szpatchpackages: P0, szproductcode: P1, szpropertieslist: P2) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiApplyMultiplePatchesW(szpatchpackages : windows_core::PCWSTR, szproductcode : windows_core::PCWSTR, szpropertieslist : windows_core::PCWSTR) -> u32);
    unsafe { MsiApplyMultiplePatchesW(szpatchpackages.param().abi(), szproductcode.param().abi(), szpropertieslist.param().abi()) }
}
#[inline]
pub unsafe fn MsiApplyPatchA<P0, P1, P3>(szpatchpackage: P0, szinstallpackage: P1, einstalltype: INSTALLTYPE, szcommandline: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiApplyPatchA(szpatchpackage : windows_core::PCSTR, szinstallpackage : windows_core::PCSTR, einstalltype : INSTALLTYPE, szcommandline : windows_core::PCSTR) -> u32);
    unsafe { MsiApplyPatchA(szpatchpackage.param().abi(), szinstallpackage.param().abi(), einstalltype, szcommandline.param().abi()) }
}
#[inline]
pub unsafe fn MsiApplyPatchW<P0, P1, P3>(szpatchpackage: P0, szinstallpackage: P1, einstalltype: INSTALLTYPE, szcommandline: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiApplyPatchW(szpatchpackage : windows_core::PCWSTR, szinstallpackage : windows_core::PCWSTR, einstalltype : INSTALLTYPE, szcommandline : windows_core::PCWSTR) -> u32);
    unsafe { MsiApplyPatchW(szpatchpackage.param().abi(), szinstallpackage.param().abi(), einstalltype, szcommandline.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MsiBeginTransactionA<P0>(szname: P0, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiBeginTransactionA(szname : windows_core::PCSTR, dwtransactionattributes : u32, phtransactionhandle : *mut MSIHANDLE, phchangeofownerevent : *mut super::winnt::HANDLE) -> u32);
    unsafe { MsiBeginTransactionA(szname.param().abi(), dwtransactionattributes, phtransactionhandle as _, phchangeofownerevent as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MsiBeginTransactionW<P0>(szname: P0, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::winnt::HANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiBeginTransactionW(szname : windows_core::PCWSTR, dwtransactionattributes : u32, phtransactionhandle : *mut MSIHANDLE, phchangeofownerevent : *mut super::winnt::HANDLE) -> u32);
    unsafe { MsiBeginTransactionW(szname.param().abi(), dwtransactionattributes, phtransactionhandle as _, phchangeofownerevent as _) }
}
#[inline]
pub unsafe fn MsiCloseAllHandles() -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiCloseAllHandles() -> u32);
    unsafe { MsiCloseAllHandles() }
}
#[inline]
pub unsafe fn MsiCloseHandle(hany: MSIHANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiCloseHandle(hany : MSIHANDLE) -> u32);
    unsafe { MsiCloseHandle(hany) }
}
#[inline]
pub unsafe fn MsiCollectUserInfoA<P0>(szproduct: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiCollectUserInfoA(szproduct : windows_core::PCSTR) -> u32);
    unsafe { MsiCollectUserInfoA(szproduct.param().abi()) }
}
#[inline]
pub unsafe fn MsiCollectUserInfoW<P0>(szproduct: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiCollectUserInfoW(szproduct : windows_core::PCWSTR) -> u32);
    unsafe { MsiCollectUserInfoW(szproduct.param().abi()) }
}
#[inline]
pub unsafe fn MsiConfigureFeatureA<P0, P1>(szproduct: P0, szfeature: P1, einstallstate: INSTALLSTATE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureFeatureA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR, einstallstate : INSTALLSTATE) -> u32);
    unsafe { MsiConfigureFeatureA(szproduct.param().abi(), szfeature.param().abi(), einstallstate) }
}
#[inline]
pub unsafe fn MsiConfigureFeatureW<P0, P1>(szproduct: P0, szfeature: P1, einstallstate: INSTALLSTATE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureFeatureW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR, einstallstate : INSTALLSTATE) -> u32);
    unsafe { MsiConfigureFeatureW(szproduct.param().abi(), szfeature.param().abi(), einstallstate) }
}
#[inline]
pub unsafe fn MsiConfigureProductA<P0>(szproduct: P0, iinstalllevel: i32, einstallstate: INSTALLSTATE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureProductA(szproduct : windows_core::PCSTR, iinstalllevel : i32, einstallstate : INSTALLSTATE) -> u32);
    unsafe { MsiConfigureProductA(szproduct.param().abi(), iinstalllevel, einstallstate) }
}
#[inline]
pub unsafe fn MsiConfigureProductExA<P0, P3>(szproduct: P0, iinstalllevel: i32, einstallstate: INSTALLSTATE, szcommandline: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureProductExA(szproduct : windows_core::PCSTR, iinstalllevel : i32, einstallstate : INSTALLSTATE, szcommandline : windows_core::PCSTR) -> u32);
    unsafe { MsiConfigureProductExA(szproduct.param().abi(), iinstalllevel, einstallstate, szcommandline.param().abi()) }
}
#[inline]
pub unsafe fn MsiConfigureProductExW<P0, P3>(szproduct: P0, iinstalllevel: i32, einstallstate: INSTALLSTATE, szcommandline: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureProductExW(szproduct : windows_core::PCWSTR, iinstalllevel : i32, einstallstate : INSTALLSTATE, szcommandline : windows_core::PCWSTR) -> u32);
    unsafe { MsiConfigureProductExW(szproduct.param().abi(), iinstalllevel, einstallstate, szcommandline.param().abi()) }
}
#[inline]
pub unsafe fn MsiConfigureProductW<P0>(szproduct: P0, iinstalllevel: i32, einstallstate: INSTALLSTATE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiConfigureProductW(szproduct : windows_core::PCWSTR, iinstalllevel : i32, einstallstate : INSTALLSTATE) -> u32);
    unsafe { MsiConfigureProductW(szproduct.param().abi(), iinstalllevel, einstallstate) }
}
#[inline]
pub unsafe fn MsiDetermineApplicablePatchesA<P0>(szproductpackagepath: P0, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOA]) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDetermineApplicablePatchesA(szproductpackagepath : windows_core::PCSTR, cpatchinfo : u32, ppatchinfo : *mut MSIPATCHSEQUENCEINFOA) -> u32);
    unsafe { MsiDetermineApplicablePatchesA(szproductpackagepath.param().abi(), ppatchinfo.len().try_into().unwrap(), core::mem::transmute(ppatchinfo.as_ptr())) }
}
#[inline]
pub unsafe fn MsiDetermineApplicablePatchesW<P0>(szproductpackagepath: P0, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOW]) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDetermineApplicablePatchesW(szproductpackagepath : windows_core::PCWSTR, cpatchinfo : u32, ppatchinfo : *mut MSIPATCHSEQUENCEINFOW) -> u32);
    unsafe { MsiDetermineApplicablePatchesW(szproductpackagepath.param().abi(), ppatchinfo.len().try_into().unwrap(), core::mem::transmute(ppatchinfo.as_ptr())) }
}
#[inline]
pub unsafe fn MsiDeterminePatchSequenceA<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOA]) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDeterminePatchSequenceA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, cpatchinfo : u32, ppatchinfo : *mut MSIPATCHSEQUENCEINFOA) -> u32);
    unsafe { MsiDeterminePatchSequenceA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, ppatchinfo.len().try_into().unwrap(), core::mem::transmute(ppatchinfo.as_ptr())) }
}
#[inline]
pub unsafe fn MsiDeterminePatchSequenceW<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, ppatchinfo: &mut [MSIPATCHSEQUENCEINFOW]) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiDeterminePatchSequenceW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, cpatchinfo : u32, ppatchinfo : *mut MSIPATCHSEQUENCEINFOW) -> u32);
    unsafe { MsiDeterminePatchSequenceW(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, ppatchinfo.len().try_into().unwrap(), core::mem::transmute(ppatchinfo.as_ptr())) }
}
#[inline]
pub unsafe fn MsiEnableLogA<P1>(dwlogmode: u32, szlogfile: P1, dwlogattributes: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnableLogA(dwlogmode : u32, szlogfile : windows_core::PCSTR, dwlogattributes : u32) -> u32);
    unsafe { MsiEnableLogA(dwlogmode, szlogfile.param().abi(), dwlogattributes) }
}
#[inline]
pub unsafe fn MsiEnableLogW<P1>(dwlogmode: u32, szlogfile: P1, dwlogattributes: u32) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnableLogW(dwlogmode : u32, szlogfile : windows_core::PCWSTR, dwlogattributes : u32) -> u32);
    unsafe { MsiEnableLogW(dwlogmode, szlogfile.param().abi(), dwlogattributes) }
}
#[inline]
pub unsafe fn MsiEndTransaction(dwtransactionstate: u32) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiEndTransaction(dwtransactionstate : u32) -> u32);
    unsafe { MsiEndTransaction(dwtransactionstate) }
}
#[inline]
pub unsafe fn MsiEnumClientsA<P0>(szcomponent: P0, iproductindex: u32, lpproductbuf: windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumClientsA(szcomponent : windows_core::PCSTR, iproductindex : u32, lpproductbuf : windows_core::PSTR) -> u32);
    unsafe { MsiEnumClientsA(szcomponent.param().abi(), iproductindex, core::mem::transmute(lpproductbuf)) }
}
#[inline]
pub unsafe fn MsiEnumClientsExA<P0, P1>(szcomponent: P0, szusersid: P1, dwcontext: u32, dwproductindex: u32, szproductbuf: Option<&mut [i8; 39]>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PSTR>, pcchsid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumClientsExA(szcomponent : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : u32, dwproductindex : u32, szproductbuf : *mut i8, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PSTR, pcchsid : *mut u32) -> u32);
    unsafe { MsiEnumClientsExA(szcomponent.param().abi(), szusersid.param().abi(), dwcontext, dwproductindex, core::mem::transmute(szproductbuf.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumClientsExW<P0, P1>(szcomponent: P0, szusersid: P1, dwcontext: u32, dwproductindex: u32, szproductbuf: Option<&mut [u16; 39]>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PWSTR>, pcchsid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumClientsExW(szcomponent : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : u32, dwproductindex : u32, szproductbuf : *mut u16, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PWSTR, pcchsid : *mut u32) -> u32);
    unsafe { MsiEnumClientsExW(szcomponent.param().abi(), szusersid.param().abi(), dwcontext, dwproductindex, core::mem::transmute(szproductbuf.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumClientsW<P0>(szcomponent: P0, iproductindex: u32, lpproductbuf: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumClientsW(szcomponent : windows_core::PCWSTR, iproductindex : u32, lpproductbuf : windows_core::PWSTR) -> u32);
    unsafe { MsiEnumClientsW(szcomponent.param().abi(), iproductindex, core::mem::transmute(lpproductbuf)) }
}
#[inline]
pub unsafe fn MsiEnumComponentQualifiersA<P0>(szcomponent: P0, iindex: u32, lpqualifierbuf: windows_core::PSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: Option<windows_core::PSTR>, pcchapplicationdatabuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentQualifiersA(szcomponent : windows_core::PCSTR, iindex : u32, lpqualifierbuf : windows_core::PSTR, pcchqualifierbuf : *mut u32, lpapplicationdatabuf : windows_core::PSTR, pcchapplicationdatabuf : *mut u32) -> u32);
    unsafe { MsiEnumComponentQualifiersA(szcomponent.param().abi(), iindex, core::mem::transmute(lpqualifierbuf), pcchqualifierbuf as _, lpapplicationdatabuf.unwrap_or(core::mem::zeroed()) as _, pcchapplicationdatabuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumComponentQualifiersW<P0>(szcomponent: P0, iindex: u32, lpqualifierbuf: windows_core::PWSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: Option<windows_core::PWSTR>, pcchapplicationdatabuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentQualifiersW(szcomponent : windows_core::PCWSTR, iindex : u32, lpqualifierbuf : windows_core::PWSTR, pcchqualifierbuf : *mut u32, lpapplicationdatabuf : windows_core::PWSTR, pcchapplicationdatabuf : *mut u32) -> u32);
    unsafe { MsiEnumComponentQualifiersW(szcomponent.param().abi(), iindex, core::mem::transmute(lpqualifierbuf), pcchqualifierbuf as _, lpapplicationdatabuf.unwrap_or(core::mem::zeroed()) as _, pcchapplicationdatabuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumComponentsA(icomponentindex: u32, lpcomponentbuf: windows_core::PSTR) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentsA(icomponentindex : u32, lpcomponentbuf : windows_core::PSTR) -> u32);
    unsafe { MsiEnumComponentsA(icomponentindex, core::mem::transmute(lpcomponentbuf)) }
}
#[inline]
pub unsafe fn MsiEnumComponentsExA<P0>(szusersid: P0, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: Option<&mut [i8; 39]>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PSTR>, pcchsid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentsExA(szusersid : windows_core::PCSTR, dwcontext : u32, dwindex : u32, szinstalledcomponentcode : *mut i8, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PSTR, pcchsid : *mut u32) -> u32);
    unsafe { MsiEnumComponentsExA(szusersid.param().abi(), dwcontext, dwindex, core::mem::transmute(szinstalledcomponentcode.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumComponentsExW<P0>(szusersid: P0, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: Option<&mut [u16; 39]>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PWSTR>, pcchsid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentsExW(szusersid : windows_core::PCWSTR, dwcontext : u32, dwindex : u32, szinstalledcomponentcode : *mut u16, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PWSTR, pcchsid : *mut u32) -> u32);
    unsafe { MsiEnumComponentsExW(szusersid.param().abi(), dwcontext, dwindex, core::mem::transmute(szinstalledcomponentcode.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumComponentsW(icomponentindex: u32, lpcomponentbuf: windows_core::PWSTR) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiEnumComponentsW(icomponentindex : u32, lpcomponentbuf : windows_core::PWSTR) -> u32);
    unsafe { MsiEnumComponentsW(icomponentindex, core::mem::transmute(lpcomponentbuf)) }
}
#[inline]
pub unsafe fn MsiEnumFeaturesA<P0>(szproduct: P0, ifeatureindex: u32, lpfeaturebuf: windows_core::PSTR, lpparentbuf: Option<windows_core::PSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumFeaturesA(szproduct : windows_core::PCSTR, ifeatureindex : u32, lpfeaturebuf : windows_core::PSTR, lpparentbuf : windows_core::PSTR) -> u32);
    unsafe { MsiEnumFeaturesA(szproduct.param().abi(), ifeatureindex, core::mem::transmute(lpfeaturebuf), lpparentbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumFeaturesW<P0>(szproduct: P0, ifeatureindex: u32, lpfeaturebuf: windows_core::PWSTR, lpparentbuf: Option<windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumFeaturesW(szproduct : windows_core::PCWSTR, ifeatureindex : u32, lpfeaturebuf : windows_core::PWSTR, lpparentbuf : windows_core::PWSTR) -> u32);
    unsafe { MsiEnumFeaturesW(szproduct.param().abi(), ifeatureindex, core::mem::transmute(lpfeaturebuf), lpparentbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumPatchesA<P0>(szproduct: P0, ipatchindex: u32, lppatchbuf: windows_core::PSTR, lptransformsbuf: windows_core::PSTR, pcchtransformsbuf: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumPatchesA(szproduct : windows_core::PCSTR, ipatchindex : u32, lppatchbuf : windows_core::PSTR, lptransformsbuf : windows_core::PSTR, pcchtransformsbuf : *mut u32) -> u32);
    unsafe { MsiEnumPatchesA(szproduct.param().abi(), ipatchindex, core::mem::transmute(lppatchbuf), core::mem::transmute(lptransformsbuf), pcchtransformsbuf as _) }
}
#[inline]
pub unsafe fn MsiEnumPatchesExA<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: Option<&mut [i8; 39]>, sztargetproductcode: Option<&mut [i8; 39]>, pdwtargetproductcontext: Option<*mut MSIINSTALLCONTEXT>, sztargetusersid: Option<windows_core::PSTR>, pcchtargetusersid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumPatchesExA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : u32, dwfilter : u32, dwindex : u32, szpatchcode : *mut i8, sztargetproductcode : *mut i8, pdwtargetproductcontext : *mut MSIINSTALLCONTEXT, sztargetusersid : windows_core::PSTR, pcchtargetusersid : *mut u32) -> u32);
    unsafe { MsiEnumPatchesExA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, dwfilter, dwindex, core::mem::transmute(szpatchcode.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(sztargetproductcode.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwtargetproductcontext.unwrap_or(core::mem::zeroed()) as _, sztargetusersid.unwrap_or(core::mem::zeroed()) as _, pcchtargetusersid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumPatchesExW<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: Option<&mut [u16; 39]>, sztargetproductcode: Option<&mut [u16; 39]>, pdwtargetproductcontext: Option<*mut MSIINSTALLCONTEXT>, sztargetusersid: Option<windows_core::PWSTR>, pcchtargetusersid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumPatchesExW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : u32, dwfilter : u32, dwindex : u32, szpatchcode : *mut u16, sztargetproductcode : *mut u16, pdwtargetproductcontext : *mut MSIINSTALLCONTEXT, sztargetusersid : windows_core::PWSTR, pcchtargetusersid : *mut u32) -> u32);
    unsafe { MsiEnumPatchesExW(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, dwfilter, dwindex, core::mem::transmute(szpatchcode.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), core::mem::transmute(sztargetproductcode.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwtargetproductcontext.unwrap_or(core::mem::zeroed()) as _, sztargetusersid.unwrap_or(core::mem::zeroed()) as _, pcchtargetusersid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumPatchesW<P0>(szproduct: P0, ipatchindex: u32, lppatchbuf: windows_core::PWSTR, lptransformsbuf: windows_core::PWSTR, pcchtransformsbuf: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumPatchesW(szproduct : windows_core::PCWSTR, ipatchindex : u32, lppatchbuf : windows_core::PWSTR, lptransformsbuf : windows_core::PWSTR, pcchtransformsbuf : *mut u32) -> u32);
    unsafe { MsiEnumPatchesW(szproduct.param().abi(), ipatchindex, core::mem::transmute(lppatchbuf), core::mem::transmute(lptransformsbuf), pcchtransformsbuf as _) }
}
#[inline]
pub unsafe fn MsiEnumProductsA(iproductindex: u32, lpproductbuf: windows_core::PSTR) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiEnumProductsA(iproductindex : u32, lpproductbuf : windows_core::PSTR) -> u32);
    unsafe { MsiEnumProductsA(iproductindex, core::mem::transmute(lpproductbuf)) }
}
#[inline]
pub unsafe fn MsiEnumProductsExA<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: u32, dwindex: u32, szinstalledproductcode: Option<&mut [i8; 39]>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PSTR>, pcchsid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumProductsExA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : u32, dwindex : u32, szinstalledproductcode : *mut i8, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PSTR, pcchsid : *mut u32) -> u32);
    unsafe { MsiEnumProductsExA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, dwindex, core::mem::transmute(szinstalledproductcode.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumProductsExW<P0, P1>(szproductcode: P0, szusersid: P1, dwcontext: u32, dwindex: u32, szinstalledproductcode: Option<&mut [u16; 39]>, pdwinstalledcontext: Option<*mut MSIINSTALLCONTEXT>, szsid: Option<windows_core::PWSTR>, pcchsid: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumProductsExW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : u32, dwindex : u32, szinstalledproductcode : *mut u16, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_core::PWSTR, pcchsid : *mut u32) -> u32);
    unsafe { MsiEnumProductsExW(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, dwindex, core::mem::transmute(szinstalledproductcode.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pdwinstalledcontext.unwrap_or(core::mem::zeroed()) as _, szsid.unwrap_or(core::mem::zeroed()) as _, pcchsid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiEnumProductsW(iproductindex: u32, lpproductbuf: windows_core::PWSTR) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiEnumProductsW(iproductindex : u32, lpproductbuf : windows_core::PWSTR) -> u32);
    unsafe { MsiEnumProductsW(iproductindex, core::mem::transmute(lpproductbuf)) }
}
#[inline]
pub unsafe fn MsiEnumRelatedProductsA<P0>(lpupgradecode: P0, dwreserved: Option<u32>, iproductindex: u32, lpproductbuf: windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumRelatedProductsA(lpupgradecode : windows_core::PCSTR, dwreserved : u32, iproductindex : u32, lpproductbuf : windows_core::PSTR) -> u32);
    unsafe { MsiEnumRelatedProductsA(lpupgradecode.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, iproductindex, core::mem::transmute(lpproductbuf)) }
}
#[inline]
pub unsafe fn MsiEnumRelatedProductsW<P0>(lpupgradecode: P0, dwreserved: Option<u32>, iproductindex: u32, lpproductbuf: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiEnumRelatedProductsW(lpupgradecode : windows_core::PCWSTR, dwreserved : u32, iproductindex : u32, lpproductbuf : windows_core::PWSTR) -> u32);
    unsafe { MsiEnumRelatedProductsW(lpupgradecode.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, iproductindex, core::mem::transmute(lpproductbuf)) }
}
#[inline]
pub unsafe fn MsiExtractPatchXMLDataA<P0>(szpatchpath: P0, dwreserved: Option<u32>, szxmldata: Option<windows_core::PSTR>, pcchxmldata: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiExtractPatchXMLDataA(szpatchpath : windows_core::PCSTR, dwreserved : u32, szxmldata : windows_core::PSTR, pcchxmldata : *mut u32) -> u32);
    unsafe { MsiExtractPatchXMLDataA(szpatchpath.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, szxmldata.unwrap_or(core::mem::zeroed()) as _, pcchxmldata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiExtractPatchXMLDataW<P0>(szpatchpath: P0, dwreserved: Option<u32>, szxmldata: Option<windows_core::PWSTR>, pcchxmldata: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiExtractPatchXMLDataW(szpatchpath : windows_core::PCWSTR, dwreserved : u32, szxmldata : windows_core::PWSTR, pcchxmldata : *mut u32) -> u32);
    unsafe { MsiExtractPatchXMLDataW(szpatchpath.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, szxmldata.unwrap_or(core::mem::zeroed()) as _, pcchxmldata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetComponentPathA<P0, P1>(szproduct: P0, szcomponent: P1, lppathbuf: Option<windows_core::PSTR>, pcchbuf: Option<*mut u32>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetComponentPathA(szproduct : windows_core::PCSTR, szcomponent : windows_core::PCSTR, lppathbuf : windows_core::PSTR, pcchbuf : *mut u32) -> INSTALLSTATE);
    unsafe { MsiGetComponentPathA(szproduct.param().abi(), szcomponent.param().abi(), lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetComponentPathExA<P0, P1, P2>(szproductcode: P0, szcomponentcode: P1, szusersid: P2, dwcontext: Option<MSIINSTALLCONTEXT>, lpoutpathbuffer: Option<windows_core::PSTR>, pcchoutpathbuffer: Option<*mut u32>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetComponentPathExA(szproductcode : windows_core::PCSTR, szcomponentcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, lpoutpathbuffer : windows_core::PSTR, pcchoutpathbuffer : *mut u32) -> INSTALLSTATE);
    unsafe { MsiGetComponentPathExA(szproductcode.param().abi(), szcomponentcode.param().abi(), szusersid.param().abi(), dwcontext.unwrap_or(core::mem::zeroed()) as _, lpoutpathbuffer.unwrap_or(core::mem::zeroed()) as _, pcchoutpathbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetComponentPathExW<P0, P1, P2>(szproductcode: P0, szcomponentcode: P1, szusersid: P2, dwcontext: Option<MSIINSTALLCONTEXT>, lpoutpathbuffer: Option<windows_core::PWSTR>, pcchoutpathbuffer: Option<*mut u32>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetComponentPathExW(szproductcode : windows_core::PCWSTR, szcomponentcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, lpoutpathbuffer : windows_core::PWSTR, pcchoutpathbuffer : *mut u32) -> INSTALLSTATE);
    unsafe { MsiGetComponentPathExW(szproductcode.param().abi(), szcomponentcode.param().abi(), szusersid.param().abi(), dwcontext.unwrap_or(core::mem::zeroed()) as _, lpoutpathbuffer.unwrap_or(core::mem::zeroed()) as _, pcchoutpathbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetComponentPathW<P0, P1>(szproduct: P0, szcomponent: P1, lppathbuf: Option<windows_core::PWSTR>, pcchbuf: Option<*mut u32>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetComponentPathW(szproduct : windows_core::PCWSTR, szcomponent : windows_core::PCWSTR, lppathbuf : windows_core::PWSTR, pcchbuf : *mut u32) -> INSTALLSTATE);
    unsafe { MsiGetComponentPathW(szproduct.param().abi(), szcomponent.param().abi(), lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetFeatureInfoA<P1>(hproduct: MSIHANDLE, szfeature: P1, lpattributes: Option<*mut u32>, lptitlebuf: Option<windows_core::PSTR>, pcchtitlebuf: Option<*mut u32>, lphelpbuf: Option<windows_core::PSTR>, pcchhelpbuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureInfoA(hproduct : MSIHANDLE, szfeature : windows_core::PCSTR, lpattributes : *mut u32, lptitlebuf : windows_core::PSTR, pcchtitlebuf : *mut u32, lphelpbuf : windows_core::PSTR, pcchhelpbuf : *mut u32) -> u32);
    unsafe { MsiGetFeatureInfoA(hproduct, szfeature.param().abi(), lpattributes.unwrap_or(core::mem::zeroed()) as _, lptitlebuf.unwrap_or(core::mem::zeroed()) as _, pcchtitlebuf.unwrap_or(core::mem::zeroed()) as _, lphelpbuf.unwrap_or(core::mem::zeroed()) as _, pcchhelpbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetFeatureInfoW<P1>(hproduct: MSIHANDLE, szfeature: P1, lpattributes: Option<*mut u32>, lptitlebuf: Option<windows_core::PWSTR>, pcchtitlebuf: Option<*mut u32>, lphelpbuf: Option<windows_core::PWSTR>, pcchhelpbuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureInfoW(hproduct : MSIHANDLE, szfeature : windows_core::PCWSTR, lpattributes : *mut u32, lptitlebuf : windows_core::PWSTR, pcchtitlebuf : *mut u32, lphelpbuf : windows_core::PWSTR, pcchhelpbuf : *mut u32) -> u32);
    unsafe { MsiGetFeatureInfoW(hproduct, szfeature.param().abi(), lpattributes.unwrap_or(core::mem::zeroed()) as _, lptitlebuf.unwrap_or(core::mem::zeroed()) as _, pcchtitlebuf.unwrap_or(core::mem::zeroed()) as _, lphelpbuf.unwrap_or(core::mem::zeroed()) as _, pcchhelpbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetFeatureUsageA<P0, P1>(szproduct: P0, szfeature: P1, pdwusecount: Option<*mut u32>, pwdateused: Option<*mut u16>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureUsageA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR, pdwusecount : *mut u32, pwdateused : *mut u16) -> u32);
    unsafe { MsiGetFeatureUsageA(szproduct.param().abi(), szfeature.param().abi(), pdwusecount.unwrap_or(core::mem::zeroed()) as _, pwdateused.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetFeatureUsageW<P0, P1>(szproduct: P0, szfeature: P1, pdwusecount: Option<*mut u32>, pwdateused: Option<*mut u16>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFeatureUsageW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR, pdwusecount : *mut u32, pwdateused : *mut u16) -> u32);
    unsafe { MsiGetFeatureUsageW(szproduct.param().abi(), szfeature.param().abi(), pdwusecount.unwrap_or(core::mem::zeroed()) as _, pwdateused.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetFileHashA<P0>(szfilepath: P0, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileHashA(szfilepath : windows_core::PCSTR, dwoptions : u32, phash : *mut MSIFILEHASHINFO) -> u32);
    unsafe { MsiGetFileHashA(szfilepath.param().abi(), dwoptions, phash as _) }
}
#[inline]
pub unsafe fn MsiGetFileHashW<P0>(szfilepath: P0, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileHashW(szfilepath : windows_core::PCWSTR, dwoptions : u32, phash : *mut MSIFILEHASHINFO) -> u32);
    unsafe { MsiGetFileHashW(szfilepath.param().abi(), dwoptions, phash as _) }
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[inline]
pub unsafe fn MsiGetFileSignatureInformationA<P0>(szsignedobjectpath: P0, dwflags: u32, ppccertcontext: *mut super::wincrypt::PCCERT_CONTEXT, pbhashdata: Option<*mut u8>, pcbhashdata: Option<*mut u32>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileSignatureInformationA(szsignedobjectpath : windows_core::PCSTR, dwflags : u32, ppccertcontext : *mut super::wincrypt::PCCERT_CONTEXT, pbhashdata : *mut u8, pcbhashdata : *mut u32) -> windows_core::HRESULT);
    unsafe { MsiGetFileSignatureInformationA(szsignedobjectpath.param().abi(), dwflags, ppccertcontext as _, pbhashdata.unwrap_or(core::mem::zeroed()) as _, pcbhashdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[inline]
pub unsafe fn MsiGetFileSignatureInformationW<P0>(szsignedobjectpath: P0, dwflags: u32, ppccertcontext: *mut super::wincrypt::PCCERT_CONTEXT, pbhashdata: Option<*mut u8>, pcbhashdata: Option<*mut u32>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileSignatureInformationW(szsignedobjectpath : windows_core::PCWSTR, dwflags : u32, ppccertcontext : *mut super::wincrypt::PCCERT_CONTEXT, pbhashdata : *mut u8, pcbhashdata : *mut u32) -> windows_core::HRESULT);
    unsafe { MsiGetFileSignatureInformationW(szsignedobjectpath.param().abi(), dwflags, ppccertcontext as _, pbhashdata.unwrap_or(core::mem::zeroed()) as _, pcbhashdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetFileVersionA<P0>(szfilepath: P0, lpversionbuf: Option<windows_core::PSTR>, pcchversionbuf: Option<*mut u32>, lplangbuf: Option<windows_core::PSTR>, pcchlangbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileVersionA(szfilepath : windows_core::PCSTR, lpversionbuf : windows_core::PSTR, pcchversionbuf : *mut u32, lplangbuf : windows_core::PSTR, pcchlangbuf : *mut u32) -> u32);
    unsafe { MsiGetFileVersionA(szfilepath.param().abi(), lpversionbuf.unwrap_or(core::mem::zeroed()) as _, pcchversionbuf.unwrap_or(core::mem::zeroed()) as _, lplangbuf.unwrap_or(core::mem::zeroed()) as _, pcchlangbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetFileVersionW<P0>(szfilepath: P0, lpversionbuf: Option<windows_core::PWSTR>, pcchversionbuf: Option<*mut u32>, lplangbuf: Option<windows_core::PWSTR>, pcchlangbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetFileVersionW(szfilepath : windows_core::PCWSTR, lpversionbuf : windows_core::PWSTR, pcchversionbuf : *mut u32, lplangbuf : windows_core::PWSTR, pcchlangbuf : *mut u32) -> u32);
    unsafe { MsiGetFileVersionW(szfilepath.param().abi(), lpversionbuf.unwrap_or(core::mem::zeroed()) as _, pcchversionbuf.unwrap_or(core::mem::zeroed()) as _, lplangbuf.unwrap_or(core::mem::zeroed()) as _, pcchlangbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetPatchFileListA<P0, P1>(szproductcode: P0, szpatchpackages: P1, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchFileListA(szproductcode : windows_core::PCSTR, szpatchpackages : windows_core::PCSTR, pcfiles : *mut u32, pphfilerecords : *mut *mut MSIHANDLE) -> u32);
    unsafe { MsiGetPatchFileListA(szproductcode.param().abi(), szpatchpackages.param().abi(), pcfiles as _, pphfilerecords as _) }
}
#[inline]
pub unsafe fn MsiGetPatchFileListW<P0, P1>(szproductcode: P0, szpatchpackages: P1, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchFileListW(szproductcode : windows_core::PCWSTR, szpatchpackages : windows_core::PCWSTR, pcfiles : *mut u32, pphfilerecords : *mut *mut MSIHANDLE) -> u32);
    unsafe { MsiGetPatchFileListW(szproductcode.param().abi(), szpatchpackages.param().abi(), pcfiles as _, pphfilerecords as _) }
}
#[inline]
pub unsafe fn MsiGetPatchInfoA<P0, P1>(szpatch: P0, szattribute: P1, lpvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchInfoA(szpatch : windows_core::PCSTR, szattribute : windows_core::PCSTR, lpvaluebuf : windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiGetPatchInfoA(szpatch.param().abi(), szattribute.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetPatchInfoExA<P0, P1, P2, P4>(szpatchcode: P0, szproductcode: P1, szusersid: P2, dwcontext: MSIINSTALLCONTEXT, szproperty: P4, lpvalue: Option<windows_core::PSTR>, pcchvalue: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchInfoExA(szpatchcode : windows_core::PCSTR, szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : windows_core::PCSTR, lpvalue : windows_core::PSTR, pcchvalue : *mut u32) -> u32);
    unsafe { MsiGetPatchInfoExA(szpatchcode.param().abi(), szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szproperty.param().abi(), lpvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetPatchInfoExW<P0, P1, P2, P4>(szpatchcode: P0, szproductcode: P1, szusersid: P2, dwcontext: MSIINSTALLCONTEXT, szproperty: P4, lpvalue: Option<windows_core::PWSTR>, pcchvalue: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchInfoExW(szpatchcode : windows_core::PCWSTR, szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : windows_core::PCWSTR, lpvalue : windows_core::PWSTR, pcchvalue : *mut u32) -> u32);
    unsafe { MsiGetPatchInfoExW(szpatchcode.param().abi(), szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szproperty.param().abi(), lpvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetPatchInfoW<P0, P1>(szpatch: P0, szattribute: P1, lpvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetPatchInfoW(szpatch : windows_core::PCWSTR, szattribute : windows_core::PCWSTR, lpvaluebuf : windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiGetPatchInfoW(szpatch.param().abi(), szattribute.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetProductCodeA<P0>(szcomponent: P0, lpbuf39: windows_core::PSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductCodeA(szcomponent : windows_core::PCSTR, lpbuf39 : windows_core::PSTR) -> u32);
    unsafe { MsiGetProductCodeA(szcomponent.param().abi(), core::mem::transmute(lpbuf39)) }
}
#[inline]
pub unsafe fn MsiGetProductCodeW<P0>(szcomponent: P0, lpbuf39: windows_core::PWSTR) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductCodeW(szcomponent : windows_core::PCWSTR, lpbuf39 : windows_core::PWSTR) -> u32);
    unsafe { MsiGetProductCodeW(szcomponent.param().abi(), core::mem::transmute(lpbuf39)) }
}
#[inline]
pub unsafe fn MsiGetProductInfoA<P0, P1>(szproduct: P0, szattribute: P1, lpvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoA(szproduct : windows_core::PCSTR, szattribute : windows_core::PCSTR, lpvaluebuf : windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiGetProductInfoA(szproduct.param().abi(), szattribute.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetProductInfoExA<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szproperty: P3, szvalue: Option<windows_core::PSTR>, pcchvalue: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoExA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : windows_core::PCSTR, szvalue : windows_core::PSTR, pcchvalue : *mut u32) -> u32);
    unsafe { MsiGetProductInfoExA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szproperty.param().abi(), szvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetProductInfoExW<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szproperty: P3, szvalue: Option<windows_core::PWSTR>, pcchvalue: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoExW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : windows_core::PCWSTR, szvalue : windows_core::PWSTR, pcchvalue : *mut u32) -> u32);
    unsafe { MsiGetProductInfoExW(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szproperty.param().abi(), szvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MsiGetProductInfoFromScriptA<P0>(szscriptfile: P0, lpproductbuf39: Option<windows_core::PSTR>, plgidlanguage: Option<*mut super::winnt::LANGID>, pdwversion: Option<*mut u32>, lpnamebuf: Option<windows_core::PSTR>, pcchnamebuf: Option<*mut u32>, lppackagebuf: Option<windows_core::PSTR>, pcchpackagebuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoFromScriptA(szscriptfile : windows_core::PCSTR, lpproductbuf39 : windows_core::PSTR, plgidlanguage : *mut super::winnt::LANGID, pdwversion : *mut u32, lpnamebuf : windows_core::PSTR, pcchnamebuf : *mut u32, lppackagebuf : windows_core::PSTR, pcchpackagebuf : *mut u32) -> u32);
    unsafe { MsiGetProductInfoFromScriptA(szscriptfile.param().abi(), lpproductbuf39.unwrap_or(core::mem::zeroed()) as _, plgidlanguage.unwrap_or(core::mem::zeroed()) as _, pdwversion.unwrap_or(core::mem::zeroed()) as _, lpnamebuf.unwrap_or(core::mem::zeroed()) as _, pcchnamebuf.unwrap_or(core::mem::zeroed()) as _, lppackagebuf.unwrap_or(core::mem::zeroed()) as _, pcchpackagebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MsiGetProductInfoFromScriptW<P0>(szscriptfile: P0, lpproductbuf39: Option<windows_core::PWSTR>, plgidlanguage: Option<*mut super::winnt::LANGID>, pdwversion: Option<*mut u32>, lpnamebuf: Option<windows_core::PWSTR>, pcchnamebuf: Option<*mut u32>, lppackagebuf: Option<windows_core::PWSTR>, pcchpackagebuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoFromScriptW(szscriptfile : windows_core::PCWSTR, lpproductbuf39 : windows_core::PWSTR, plgidlanguage : *mut super::winnt::LANGID, pdwversion : *mut u32, lpnamebuf : windows_core::PWSTR, pcchnamebuf : *mut u32, lppackagebuf : windows_core::PWSTR, pcchpackagebuf : *mut u32) -> u32);
    unsafe { MsiGetProductInfoFromScriptW(szscriptfile.param().abi(), lpproductbuf39.unwrap_or(core::mem::zeroed()) as _, plgidlanguage.unwrap_or(core::mem::zeroed()) as _, pdwversion.unwrap_or(core::mem::zeroed()) as _, lpnamebuf.unwrap_or(core::mem::zeroed()) as _, pcchnamebuf.unwrap_or(core::mem::zeroed()) as _, lppackagebuf.unwrap_or(core::mem::zeroed()) as _, pcchpackagebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetProductInfoW<P0, P1>(szproduct: P0, szattribute: P1, lpvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductInfoW(szproduct : windows_core::PCWSTR, szattribute : windows_core::PCWSTR, lpvaluebuf : windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiGetProductInfoW(szproduct.param().abi(), szattribute.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetProductPropertyA<P1>(hproduct: MSIHANDLE, szproperty: P1, lpvaluebuf: Option<windows_core::PSTR>, pcchvaluebuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductPropertyA(hproduct : MSIHANDLE, szproperty : windows_core::PCSTR, lpvaluebuf : windows_core::PSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiGetProductPropertyA(hproduct, szproperty.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetProductPropertyW<P1>(hproduct: MSIHANDLE, szproperty: P1, lpvaluebuf: Option<windows_core::PWSTR>, pcchvaluebuf: Option<*mut u32>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetProductPropertyW(hproduct : MSIHANDLE, szproperty : windows_core::PCWSTR, lpvaluebuf : windows_core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
    unsafe { MsiGetProductPropertyW(hproduct, szproperty.param().abi(), lpvaluebuf.unwrap_or(core::mem::zeroed()) as _, pcchvaluebuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetShortcutTargetA<P0>(szshortcutpath: P0, szproductcode: Option<windows_core::PSTR>, szfeatureid: Option<windows_core::PSTR>, szcomponentcode: Option<windows_core::PSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetShortcutTargetA(szshortcutpath : windows_core::PCSTR, szproductcode : windows_core::PSTR, szfeatureid : windows_core::PSTR, szcomponentcode : windows_core::PSTR) -> u32);
    unsafe { MsiGetShortcutTargetA(szshortcutpath.param().abi(), szproductcode.unwrap_or(core::mem::zeroed()) as _, szfeatureid.unwrap_or(core::mem::zeroed()) as _, szcomponentcode.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetShortcutTargetW<P0>(szshortcutpath: P0, szproductcode: Option<windows_core::PWSTR>, szfeatureid: Option<windows_core::PWSTR>, szcomponentcode: Option<windows_core::PWSTR>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetShortcutTargetW(szshortcutpath : windows_core::PCWSTR, szproductcode : windows_core::PWSTR, szfeatureid : windows_core::PWSTR, szcomponentcode : windows_core::PWSTR) -> u32);
    unsafe { MsiGetShortcutTargetW(szshortcutpath.param().abi(), szproductcode.unwrap_or(core::mem::zeroed()) as _, szfeatureid.unwrap_or(core::mem::zeroed()) as _, szcomponentcode.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetUserInfoA<P0>(szproduct: P0, lpusernamebuf: Option<windows_core::PSTR>, pcchusernamebuf: Option<*mut u32>, lporgnamebuf: Option<windows_core::PSTR>, pcchorgnamebuf: Option<*mut u32>, lpserialbuf: Option<windows_core::PSTR>, pcchserialbuf: Option<*mut u32>) -> USERINFOSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetUserInfoA(szproduct : windows_core::PCSTR, lpusernamebuf : windows_core::PSTR, pcchusernamebuf : *mut u32, lporgnamebuf : windows_core::PSTR, pcchorgnamebuf : *mut u32, lpserialbuf : windows_core::PSTR, pcchserialbuf : *mut u32) -> USERINFOSTATE);
    unsafe { MsiGetUserInfoA(szproduct.param().abi(), lpusernamebuf.unwrap_or(core::mem::zeroed()) as _, pcchusernamebuf.unwrap_or(core::mem::zeroed()) as _, lporgnamebuf.unwrap_or(core::mem::zeroed()) as _, pcchorgnamebuf.unwrap_or(core::mem::zeroed()) as _, lpserialbuf.unwrap_or(core::mem::zeroed()) as _, pcchserialbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiGetUserInfoW<P0>(szproduct: P0, lpusernamebuf: Option<windows_core::PWSTR>, pcchusernamebuf: Option<*mut u32>, lporgnamebuf: Option<windows_core::PWSTR>, pcchorgnamebuf: Option<*mut u32>, lpserialbuf: Option<windows_core::PWSTR>, pcchserialbuf: Option<*mut u32>) -> USERINFOSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiGetUserInfoW(szproduct : windows_core::PCWSTR, lpusernamebuf : windows_core::PWSTR, pcchusernamebuf : *mut u32, lporgnamebuf : windows_core::PWSTR, pcchorgnamebuf : *mut u32, lpserialbuf : windows_core::PWSTR, pcchserialbuf : *mut u32) -> USERINFOSTATE);
    unsafe { MsiGetUserInfoW(szproduct.param().abi(), lpusernamebuf.unwrap_or(core::mem::zeroed()) as _, pcchusernamebuf.unwrap_or(core::mem::zeroed()) as _, lporgnamebuf.unwrap_or(core::mem::zeroed()) as _, pcchorgnamebuf.unwrap_or(core::mem::zeroed()) as _, lpserialbuf.unwrap_or(core::mem::zeroed()) as _, pcchserialbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiInstallMissingComponentA<P0, P1>(szproduct: P0, szcomponent: P1, einstallstate: INSTALLSTATE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallMissingComponentA(szproduct : windows_core::PCSTR, szcomponent : windows_core::PCSTR, einstallstate : INSTALLSTATE) -> u32);
    unsafe { MsiInstallMissingComponentA(szproduct.param().abi(), szcomponent.param().abi(), einstallstate) }
}
#[inline]
pub unsafe fn MsiInstallMissingComponentW<P0, P1>(szproduct: P0, szcomponent: P1, einstallstate: INSTALLSTATE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallMissingComponentW(szproduct : windows_core::PCWSTR, szcomponent : windows_core::PCWSTR, einstallstate : INSTALLSTATE) -> u32);
    unsafe { MsiInstallMissingComponentW(szproduct.param().abi(), szcomponent.param().abi(), einstallstate) }
}
#[inline]
pub unsafe fn MsiInstallMissingFileA<P0, P1>(szproduct: P0, szfile: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallMissingFileA(szproduct : windows_core::PCSTR, szfile : windows_core::PCSTR) -> u32);
    unsafe { MsiInstallMissingFileA(szproduct.param().abi(), szfile.param().abi()) }
}
#[inline]
pub unsafe fn MsiInstallMissingFileW<P0, P1>(szproduct: P0, szfile: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallMissingFileW(szproduct : windows_core::PCWSTR, szfile : windows_core::PCWSTR) -> u32);
    unsafe { MsiInstallMissingFileW(szproduct.param().abi(), szfile.param().abi()) }
}
#[inline]
pub unsafe fn MsiInstallProductA<P0, P1>(szpackagepath: P0, szcommandline: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallProductA(szpackagepath : windows_core::PCSTR, szcommandline : windows_core::PCSTR) -> u32);
    unsafe { MsiInstallProductA(szpackagepath.param().abi(), szcommandline.param().abi()) }
}
#[inline]
pub unsafe fn MsiInstallProductW<P0, P1>(szpackagepath: P0, szcommandline: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiInstallProductW(szpackagepath : windows_core::PCWSTR, szcommandline : windows_core::PCWSTR) -> u32);
    unsafe { MsiInstallProductW(szpackagepath.param().abi(), szcommandline.param().abi()) }
}
#[inline]
pub unsafe fn MsiIsProductElevatedA<P0>(szproduct: P0, pfelevated: *mut windows_core::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiIsProductElevatedA(szproduct : windows_core::PCSTR, pfelevated : *mut windows_core::BOOL) -> u32);
    unsafe { MsiIsProductElevatedA(szproduct.param().abi(), pfelevated as _) }
}
#[inline]
pub unsafe fn MsiIsProductElevatedW<P0>(szproduct: P0, pfelevated: *mut windows_core::BOOL) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiIsProductElevatedW(szproduct : windows_core::PCWSTR, pfelevated : *mut windows_core::BOOL) -> u32);
    unsafe { MsiIsProductElevatedW(szproduct.param().abi(), pfelevated as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MsiJoinTransaction(htransactionhandle: MSIHANDLE, dwtransactionattributes: u32, phchangeofownerevent: *mut super::winnt::HANDLE) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiJoinTransaction(htransactionhandle : MSIHANDLE, dwtransactionattributes : u32, phchangeofownerevent : *mut super::winnt::HANDLE) -> u32);
    unsafe { MsiJoinTransaction(htransactionhandle, dwtransactionattributes, phchangeofownerevent as _) }
}
#[inline]
pub unsafe fn MsiLocateComponentA<P0>(szcomponent: P0, lppathbuf: Option<windows_core::PSTR>, pcchbuf: Option<*mut u32>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiLocateComponentA(szcomponent : windows_core::PCSTR, lppathbuf : windows_core::PSTR, pcchbuf : *mut u32) -> INSTALLSTATE);
    unsafe { MsiLocateComponentA(szcomponent.param().abi(), lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiLocateComponentW<P0>(szcomponent: P0, lppathbuf: Option<windows_core::PWSTR>, pcchbuf: Option<*mut u32>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiLocateComponentW(szcomponent : windows_core::PCWSTR, lppathbuf : windows_core::PWSTR, pcchbuf : *mut u32) -> INSTALLSTATE);
    unsafe { MsiLocateComponentW(szcomponent.param().abi(), lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiNotifySidChangeA<P0, P1>(poldsid: P0, pnewsid: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiNotifySidChangeA(poldsid : windows_core::PCSTR, pnewsid : windows_core::PCSTR) -> u32);
    unsafe { MsiNotifySidChangeA(poldsid.param().abi(), pnewsid.param().abi()) }
}
#[inline]
pub unsafe fn MsiNotifySidChangeW<P0, P1>(poldsid: P0, pnewsid: P1) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiNotifySidChangeW(poldsid : windows_core::PCWSTR, pnewsid : windows_core::PCWSTR) -> u32);
    unsafe { MsiNotifySidChangeW(poldsid.param().abi(), pnewsid.param().abi()) }
}
#[inline]
pub unsafe fn MsiOpenPackageA<P0>(szpackagepath: P0, hproduct: *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenPackageA(szpackagepath : windows_core::PCSTR, hproduct : *mut MSIHANDLE) -> u32);
    unsafe { MsiOpenPackageA(szpackagepath.param().abi(), hproduct as _) }
}
#[inline]
pub unsafe fn MsiOpenPackageExA<P0>(szpackagepath: P0, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenPackageExA(szpackagepath : windows_core::PCSTR, dwoptions : u32, hproduct : *mut MSIHANDLE) -> u32);
    unsafe { MsiOpenPackageExA(szpackagepath.param().abi(), dwoptions, hproduct as _) }
}
#[inline]
pub unsafe fn MsiOpenPackageExW<P0>(szpackagepath: P0, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenPackageExW(szpackagepath : windows_core::PCWSTR, dwoptions : u32, hproduct : *mut MSIHANDLE) -> u32);
    unsafe { MsiOpenPackageExW(szpackagepath.param().abi(), dwoptions, hproduct as _) }
}
#[inline]
pub unsafe fn MsiOpenPackageW<P0>(szpackagepath: P0, hproduct: *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenPackageW(szpackagepath : windows_core::PCWSTR, hproduct : *mut MSIHANDLE) -> u32);
    unsafe { MsiOpenPackageW(szpackagepath.param().abi(), hproduct as _) }
}
#[inline]
pub unsafe fn MsiOpenProductA<P0>(szproduct: P0, hproduct: *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenProductA(szproduct : windows_core::PCSTR, hproduct : *mut MSIHANDLE) -> u32);
    unsafe { MsiOpenProductA(szproduct.param().abi(), hproduct as _) }
}
#[inline]
pub unsafe fn MsiOpenProductW<P0>(szproduct: P0, hproduct: *mut MSIHANDLE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiOpenProductW(szproduct : windows_core::PCWSTR, hproduct : *mut MSIHANDLE) -> u32);
    unsafe { MsiOpenProductW(szproduct.param().abi(), hproduct as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn MsiProcessAdvertiseScriptA<P0, P1>(szscriptfile: P0, sziconfolder: P1, hregdata: Option<super::minwindef::HKEY>, fshortcuts: bool, fremoveitems: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProcessAdvertiseScriptA(szscriptfile : windows_core::PCSTR, sziconfolder : windows_core::PCSTR, hregdata : super::minwindef::HKEY, fshortcuts : windows_core::BOOL, fremoveitems : windows_core::BOOL) -> u32);
    unsafe { MsiProcessAdvertiseScriptA(szscriptfile.param().abi(), sziconfolder.param().abi(), hregdata.unwrap_or(core::mem::zeroed()) as _, fshortcuts.into(), fremoveitems.into()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn MsiProcessAdvertiseScriptW<P0, P1>(szscriptfile: P0, sziconfolder: P1, hregdata: Option<super::minwindef::HKEY>, fshortcuts: bool, fremoveitems: bool) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProcessAdvertiseScriptW(szscriptfile : windows_core::PCWSTR, sziconfolder : windows_core::PCWSTR, hregdata : super::minwindef::HKEY, fshortcuts : windows_core::BOOL, fremoveitems : windows_core::BOOL) -> u32);
    unsafe { MsiProcessAdvertiseScriptW(szscriptfile.param().abi(), sziconfolder.param().abi(), hregdata.unwrap_or(core::mem::zeroed()) as _, fshortcuts.into(), fremoveitems.into()) }
}
#[inline]
pub unsafe fn MsiProvideAssemblyA<P0, P1>(szassemblyname: P0, szappcontext: P1, dwinstallmode: u32, dwassemblyinfo: u32, lppathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideAssemblyA(szassemblyname : windows_core::PCSTR, szappcontext : windows_core::PCSTR, dwinstallmode : u32, dwassemblyinfo : u32, lppathbuf : windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiProvideAssemblyA(szassemblyname.param().abi(), szappcontext.param().abi(), dwinstallmode, dwassemblyinfo, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiProvideAssemblyW<P0, P1>(szassemblyname: P0, szappcontext: P1, dwinstallmode: u32, dwassemblyinfo: u32, lppathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideAssemblyW(szassemblyname : windows_core::PCWSTR, szappcontext : windows_core::PCWSTR, dwinstallmode : u32, dwassemblyinfo : u32, lppathbuf : windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiProvideAssemblyW(szassemblyname.param().abi(), szappcontext.param().abi(), dwinstallmode, dwassemblyinfo, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiProvideComponentA<P0, P1, P2>(szproduct: P0, szfeature: P1, szcomponent: P2, dwinstallmode: u32, lppathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideComponentA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR, szcomponent : windows_core::PCSTR, dwinstallmode : u32, lppathbuf : windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiProvideComponentA(szproduct.param().abi(), szfeature.param().abi(), szcomponent.param().abi(), dwinstallmode, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiProvideComponentW<P0, P1, P2>(szproduct: P0, szfeature: P1, szcomponent: P2, dwinstallmode: u32, lppathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideComponentW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR, szcomponent : windows_core::PCWSTR, dwinstallmode : u32, lppathbuf : windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiProvideComponentW(szproduct.param().abi(), szfeature.param().abi(), szcomponent.param().abi(), dwinstallmode, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiProvideQualifiedComponentA<P0, P1>(szcategory: P0, szqualifier: P1, dwinstallmode: u32, lppathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideQualifiedComponentA(szcategory : windows_core::PCSTR, szqualifier : windows_core::PCSTR, dwinstallmode : u32, lppathbuf : windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiProvideQualifiedComponentA(szcategory.param().abi(), szqualifier.param().abi(), dwinstallmode, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiProvideQualifiedComponentExA<P0, P1, P3>(szcategory: P0, szqualifier: P1, dwinstallmode: u32, szproduct: P3, dwunused1: Option<u32>, dwunused2: Option<u32>, lppathbuf: Option<windows_core::PSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideQualifiedComponentExA(szcategory : windows_core::PCSTR, szqualifier : windows_core::PCSTR, dwinstallmode : u32, szproduct : windows_core::PCSTR, dwunused1 : u32, dwunused2 : u32, lppathbuf : windows_core::PSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiProvideQualifiedComponentExA(szcategory.param().abi(), szqualifier.param().abi(), dwinstallmode, szproduct.param().abi(), dwunused1.unwrap_or(core::mem::zeroed()) as _, dwunused2.unwrap_or(core::mem::zeroed()) as _, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiProvideQualifiedComponentExW<P0, P1, P3>(szcategory: P0, szqualifier: P1, dwinstallmode: u32, szproduct: P3, dwunused1: Option<u32>, dwunused2: Option<u32>, lppathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideQualifiedComponentExW(szcategory : windows_core::PCWSTR, szqualifier : windows_core::PCWSTR, dwinstallmode : u32, szproduct : windows_core::PCWSTR, dwunused1 : u32, dwunused2 : u32, lppathbuf : windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiProvideQualifiedComponentExW(szcategory.param().abi(), szqualifier.param().abi(), dwinstallmode, szproduct.param().abi(), dwunused1.unwrap_or(core::mem::zeroed()) as _, dwunused2.unwrap_or(core::mem::zeroed()) as _, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiProvideQualifiedComponentW<P0, P1>(szcategory: P0, szqualifier: P1, dwinstallmode: u32, lppathbuf: Option<windows_core::PWSTR>, pcchpathbuf: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiProvideQualifiedComponentW(szcategory : windows_core::PCWSTR, szqualifier : windows_core::PCWSTR, dwinstallmode : u32, lppathbuf : windows_core::PWSTR, pcchpathbuf : *mut u32) -> u32);
    unsafe { MsiProvideQualifiedComponentW(szcategory.param().abi(), szqualifier.param().abi(), dwinstallmode, lppathbuf.unwrap_or(core::mem::zeroed()) as _, pcchpathbuf.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiQueryComponentStateA<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: P3, pdwstate: Option<*mut INSTALLSTATE>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryComponentStateA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szcomponentcode : windows_core::PCSTR, pdwstate : *mut INSTALLSTATE) -> u32);
    unsafe { MsiQueryComponentStateA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szcomponentcode.param().abi(), pdwstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiQueryComponentStateW<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: P3, pdwstate: Option<*mut INSTALLSTATE>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryComponentStateW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szcomponentcode : windows_core::PCWSTR, pdwstate : *mut INSTALLSTATE) -> u32);
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
#[inline]
pub unsafe fn MsiQueryFeatureStateExA<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szfeature: P3, pdwstate: Option<*mut INSTALLSTATE>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryFeatureStateExA(szproductcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szfeature : windows_core::PCSTR, pdwstate : *mut INSTALLSTATE) -> u32);
    unsafe { MsiQueryFeatureStateExA(szproductcode.param().abi(), szusersid.param().abi(), dwcontext, szfeature.param().abi(), pdwstate.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiQueryFeatureStateExW<P0, P1, P3>(szproductcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, szfeature: P3, pdwstate: Option<*mut INSTALLSTATE>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiQueryFeatureStateExW(szproductcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szfeature : windows_core::PCWSTR, pdwstate : *mut INSTALLSTATE) -> u32);
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
#[inline]
pub unsafe fn MsiReinstallFeatureA<P0, P1>(szproduct: P0, szfeature: P1, dwreinstallmode: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiReinstallFeatureA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR, dwreinstallmode : u32) -> u32);
    unsafe { MsiReinstallFeatureA(szproduct.param().abi(), szfeature.param().abi(), dwreinstallmode) }
}
#[inline]
pub unsafe fn MsiReinstallFeatureW<P0, P1>(szproduct: P0, szfeature: P1, dwreinstallmode: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiReinstallFeatureW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR, dwreinstallmode : u32) -> u32);
    unsafe { MsiReinstallFeatureW(szproduct.param().abi(), szfeature.param().abi(), dwreinstallmode) }
}
#[inline]
pub unsafe fn MsiReinstallProductA<P0>(szproduct: P0, szreinstallmode: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiReinstallProductA(szproduct : windows_core::PCSTR, szreinstallmode : u32) -> u32);
    unsafe { MsiReinstallProductA(szproduct.param().abi(), szreinstallmode) }
}
#[inline]
pub unsafe fn MsiReinstallProductW<P0>(szproduct: P0, szreinstallmode: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiReinstallProductW(szproduct : windows_core::PCWSTR, szreinstallmode : u32) -> u32);
    unsafe { MsiReinstallProductW(szproduct.param().abi(), szreinstallmode) }
}
#[inline]
pub unsafe fn MsiRemovePatchesA<P0, P1, P3>(szpatchlist: P0, szproductcode: P1, euninstalltype: INSTALLTYPE, szpropertylist: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiRemovePatchesA(szpatchlist : windows_core::PCSTR, szproductcode : windows_core::PCSTR, euninstalltype : INSTALLTYPE, szpropertylist : windows_core::PCSTR) -> u32);
    unsafe { MsiRemovePatchesA(szpatchlist.param().abi(), szproductcode.param().abi(), euninstalltype, szpropertylist.param().abi()) }
}
#[inline]
pub unsafe fn MsiRemovePatchesW<P0, P1, P3>(szpatchlist: P0, szproductcode: P1, euninstalltype: INSTALLTYPE, szpropertylist: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiRemovePatchesW(szpatchlist : windows_core::PCWSTR, szproductcode : windows_core::PCWSTR, euninstalltype : INSTALLTYPE, szpropertylist : windows_core::PCWSTR) -> u32);
    unsafe { MsiRemovePatchesW(szpatchlist.param().abi(), szproductcode.param().abi(), euninstalltype, szpropertylist.param().abi()) }
}
#[inline]
pub unsafe fn MsiSetExternalUIA(puihandler: INSTALLUI_HANDLERA, dwmessagefilter: u32, pvcontext: Option<*const core::ffi::c_void>) -> INSTALLUI_HANDLERA {
    windows_core::link!("msi.dll" "system" fn MsiSetExternalUIA(puihandler : INSTALLUI_HANDLERA, dwmessagefilter : u32, pvcontext : *const core::ffi::c_void) -> INSTALLUI_HANDLERA);
    unsafe { MsiSetExternalUIA(puihandler, dwmessagefilter, pvcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSetExternalUIRecord(puihandler: INSTALLUI_HANDLER_RECORD, dwmessagefilter: u32, pvcontext: Option<*const core::ffi::c_void>, ppuiprevhandler: Option<*mut INSTALLUI_HANDLER_RECORD>) -> u32 {
    windows_core::link!("msi.dll" "system" fn MsiSetExternalUIRecord(puihandler : INSTALLUI_HANDLER_RECORD, dwmessagefilter : u32, pvcontext : *const core::ffi::c_void, ppuiprevhandler : *mut INSTALLUI_HANDLER_RECORD) -> u32);
    unsafe { MsiSetExternalUIRecord(puihandler, dwmessagefilter, pvcontext.unwrap_or(core::mem::zeroed()) as _, ppuiprevhandler.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSetExternalUIW(puihandler: INSTALLUI_HANDLERW, dwmessagefilter: u32, pvcontext: Option<*const core::ffi::c_void>) -> INSTALLUI_HANDLERW {
    windows_core::link!("msi.dll" "system" fn MsiSetExternalUIW(puihandler : INSTALLUI_HANDLERW, dwmessagefilter : u32, pvcontext : *const core::ffi::c_void) -> INSTALLUI_HANDLERW);
    unsafe { MsiSetExternalUIW(puihandler, dwmessagefilter, pvcontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MsiSetInternalUI(dwuilevel: INSTALLUILEVEL, phwnd: Option<*mut super::windef::HWND>) -> INSTALLUILEVEL {
    windows_core::link!("msi.dll" "system" fn MsiSetInternalUI(dwuilevel : INSTALLUILEVEL, phwnd : *mut super::windef::HWND) -> INSTALLUILEVEL);
    unsafe { MsiSetInternalUI(dwuilevel, phwnd.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListAddMediaDiskA<P0, P1, P5, P6>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: P5, szdiskprompt: P6) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
    P6: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddMediaDiskA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwdiskid : u32, szvolumelabel : windows_core::PCSTR, szdiskprompt : windows_core::PCSTR) -> u32);
    unsafe { MsiSourceListAddMediaDiskA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwdiskid, szvolumelabel.param().abi(), szdiskprompt.param().abi()) }
}
#[inline]
pub unsafe fn MsiSourceListAddMediaDiskW<P0, P1, P5, P6>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: P5, szdiskprompt: P6) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddMediaDiskW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwdiskid : u32, szvolumelabel : windows_core::PCWSTR, szdiskprompt : windows_core::PCWSTR) -> u32);
    unsafe { MsiSourceListAddMediaDiskW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwdiskid, szvolumelabel.param().abi(), szdiskprompt.param().abi()) }
}
#[inline]
pub unsafe fn MsiSourceListAddSourceA<P0, P1, P3>(szproduct: P0, szusername: P1, dwreserved: Option<u32>, szsource: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddSourceA(szproduct : windows_core::PCSTR, szusername : windows_core::PCSTR, dwreserved : u32, szsource : windows_core::PCSTR) -> u32);
    unsafe { MsiSourceListAddSourceA(szproduct.param().abi(), szusername.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, szsource.param().abi()) }
}
#[inline]
pub unsafe fn MsiSourceListAddSourceExA<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: P4, dwindex: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddSourceExA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szsource : windows_core::PCSTR, dwindex : u32) -> u32);
    unsafe { MsiSourceListAddSourceExA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szsource.param().abi(), dwindex) }
}
#[inline]
pub unsafe fn MsiSourceListAddSourceExW<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: P4, dwindex: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddSourceExW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szsource : windows_core::PCWSTR, dwindex : u32) -> u32);
    unsafe { MsiSourceListAddSourceExW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szsource.param().abi(), dwindex) }
}
#[inline]
pub unsafe fn MsiSourceListAddSourceW<P0, P1, P3>(szproduct: P0, szusername: P1, dwreserved: Option<u32>, szsource: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListAddSourceW(szproduct : windows_core::PCWSTR, szusername : windows_core::PCWSTR, dwreserved : u32, szsource : windows_core::PCWSTR) -> u32);
    unsafe { MsiSourceListAddSourceW(szproduct.param().abi(), szusername.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, szsource.param().abi()) }
}
#[inline]
pub unsafe fn MsiSourceListClearAllA<P0, P1>(szproduct: P0, szusername: P1, dwreserved: Option<u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearAllA(szproduct : windows_core::PCSTR, szusername : windows_core::PCSTR, dwreserved : u32) -> u32);
    unsafe { MsiSourceListClearAllA(szproduct.param().abi(), szusername.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListClearAllExA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearAllExA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32) -> u32);
    unsafe { MsiSourceListClearAllExA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions) }
}
#[inline]
pub unsafe fn MsiSourceListClearAllExW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearAllExW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32) -> u32);
    unsafe { MsiSourceListClearAllExW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions) }
}
#[inline]
pub unsafe fn MsiSourceListClearAllW<P0, P1>(szproduct: P0, szusername: P1, dwreserved: Option<u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearAllW(szproduct : windows_core::PCWSTR, szusername : windows_core::PCWSTR, dwreserved : u32) -> u32);
    unsafe { MsiSourceListClearAllW(szproduct.param().abi(), szusername.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListClearMediaDiskA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearMediaDiskA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwdiskid : u32) -> u32);
    unsafe { MsiSourceListClearMediaDiskA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwdiskid) }
}
#[inline]
pub unsafe fn MsiSourceListClearMediaDiskW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearMediaDiskW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwdiskid : u32) -> u32);
    unsafe { MsiSourceListClearMediaDiskW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwdiskid) }
}
#[inline]
pub unsafe fn MsiSourceListClearSourceA<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: P4) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearSourceA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szsource : windows_core::PCSTR) -> u32);
    unsafe { MsiSourceListClearSourceA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szsource.param().abi()) }
}
#[inline]
pub unsafe fn MsiSourceListClearSourceW<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: P4) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListClearSourceW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szsource : windows_core::PCWSTR) -> u32);
    unsafe { MsiSourceListClearSourceW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szsource.param().abi()) }
}
#[inline]
pub unsafe fn MsiSourceListEnumMediaDisksA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: Option<*mut u32>, szvolumelabel: Option<windows_core::PSTR>, pcchvolumelabel: Option<*mut u32>, szdiskprompt: Option<windows_core::PSTR>, pcchdiskprompt: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListEnumMediaDisksA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwindex : u32, pdwdiskid : *mut u32, szvolumelabel : windows_core::PSTR, pcchvolumelabel : *mut u32, szdiskprompt : windows_core::PSTR, pcchdiskprompt : *mut u32) -> u32);
    unsafe { MsiSourceListEnumMediaDisksA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwindex, pdwdiskid.unwrap_or(core::mem::zeroed()) as _, szvolumelabel.unwrap_or(core::mem::zeroed()) as _, pcchvolumelabel.unwrap_or(core::mem::zeroed()) as _, szdiskprompt.unwrap_or(core::mem::zeroed()) as _, pcchdiskprompt.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListEnumMediaDisksW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: Option<*mut u32>, szvolumelabel: Option<windows_core::PWSTR>, pcchvolumelabel: Option<*mut u32>, szdiskprompt: Option<windows_core::PWSTR>, pcchdiskprompt: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListEnumMediaDisksW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwindex : u32, pdwdiskid : *mut u32, szvolumelabel : windows_core::PWSTR, pcchvolumelabel : *mut u32, szdiskprompt : windows_core::PWSTR, pcchdiskprompt : *mut u32) -> u32);
    unsafe { MsiSourceListEnumMediaDisksW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwindex, pdwdiskid.unwrap_or(core::mem::zeroed()) as _, szvolumelabel.unwrap_or(core::mem::zeroed()) as _, pcchvolumelabel.unwrap_or(core::mem::zeroed()) as _, szdiskprompt.unwrap_or(core::mem::zeroed()) as _, pcchdiskprompt.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListEnumSourcesA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: Option<windows_core::PSTR>, pcchsource: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListEnumSourcesA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwindex : u32, szsource : windows_core::PSTR, pcchsource : *mut u32) -> u32);
    unsafe { MsiSourceListEnumSourcesA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwindex, szsource.unwrap_or(core::mem::zeroed()) as _, pcchsource.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListEnumSourcesW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: Option<windows_core::PWSTR>, pcchsource: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListEnumSourcesW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwindex : u32, szsource : windows_core::PWSTR, pcchsource : *mut u32) -> u32);
    unsafe { MsiSourceListEnumSourcesW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, dwindex, szsource.unwrap_or(core::mem::zeroed()) as _, pcchsource.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListForceResolutionA<P0, P1>(szproduct: P0, szusername: P1, dwreserved: Option<u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListForceResolutionA(szproduct : windows_core::PCSTR, szusername : windows_core::PCSTR, dwreserved : u32) -> u32);
    unsafe { MsiSourceListForceResolutionA(szproduct.param().abi(), szusername.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListForceResolutionExA<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListForceResolutionExA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32) -> u32);
    unsafe { MsiSourceListForceResolutionExA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions) }
}
#[inline]
pub unsafe fn MsiSourceListForceResolutionExW<P0, P1>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListForceResolutionExW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32) -> u32);
    unsafe { MsiSourceListForceResolutionExW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions) }
}
#[inline]
pub unsafe fn MsiSourceListForceResolutionW<P0, P1>(szproduct: P0, szusername: P1, dwreserved: Option<u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListForceResolutionW(szproduct : windows_core::PCWSTR, szusername : windows_core::PCWSTR, dwreserved : u32) -> u32);
    unsafe { MsiSourceListForceResolutionW(szproduct.param().abi(), szusername.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListGetInfoA<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: P4, szvalue: Option<windows_core::PSTR>, pcchvalue: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListGetInfoA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szproperty : windows_core::PCSTR, szvalue : windows_core::PSTR, pcchvalue : *mut u32) -> u32);
    unsafe { MsiSourceListGetInfoA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szproperty.param().abi(), szvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListGetInfoW<P0, P1, P4>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: P4, szvalue: Option<windows_core::PWSTR>, pcchvalue: Option<*mut u32>) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListGetInfoW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szproperty : windows_core::PCWSTR, szvalue : windows_core::PWSTR, pcchvalue : *mut u32) -> u32);
    unsafe { MsiSourceListGetInfoW(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szproperty.param().abi(), szvalue.unwrap_or(core::mem::zeroed()) as _, pcchvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiSourceListSetInfoA<P0, P1, P4, P5>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: P4, szvalue: P5) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListSetInfoA(szproductcodeorpatchcode : windows_core::PCSTR, szusersid : windows_core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szproperty : windows_core::PCSTR, szvalue : windows_core::PCSTR) -> u32);
    unsafe { MsiSourceListSetInfoA(szproductcodeorpatchcode.param().abi(), szusersid.param().abi(), dwcontext, dwoptions, szproperty.param().abi(), szvalue.param().abi()) }
}
#[inline]
pub unsafe fn MsiSourceListSetInfoW<P0, P1, P4, P5>(szproductcodeorpatchcode: P0, szusersid: P1, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: P4, szvalue: P5) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiSourceListSetInfoW(szproductcodeorpatchcode : windows_core::PCWSTR, szusersid : windows_core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szproperty : windows_core::PCWSTR, szvalue : windows_core::PCWSTR) -> u32);
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
#[inline]
pub unsafe fn MsiUseFeatureExA<P0, P1>(szproduct: P0, szfeature: P1, dwinstallmode: u32, dwreserved: Option<u32>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiUseFeatureExA(szproduct : windows_core::PCSTR, szfeature : windows_core::PCSTR, dwinstallmode : u32, dwreserved : u32) -> INSTALLSTATE);
    unsafe { MsiUseFeatureExA(szproduct.param().abi(), szfeature.param().abi(), dwinstallmode, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn MsiUseFeatureExW<P0, P1>(szproduct: P0, szfeature: P1, dwinstallmode: u32, dwreserved: Option<u32>) -> INSTALLSTATE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiUseFeatureExW(szproduct : windows_core::PCWSTR, szfeature : windows_core::PCWSTR, dwinstallmode : u32, dwreserved : u32) -> INSTALLSTATE);
    unsafe { MsiUseFeatureExW(szproduct.param().abi(), szfeature.param().abi(), dwinstallmode, dwreserved.unwrap_or(core::mem::zeroed()) as _) }
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
#[inline]
pub unsafe fn MsiVerifyPackageA<P0>(szpackagepath: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiVerifyPackageA(szpackagepath : windows_core::PCSTR) -> u32);
    unsafe { MsiVerifyPackageA(szpackagepath.param().abi()) }
}
#[inline]
pub unsafe fn MsiVerifyPackageW<P0>(szpackagepath: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msi.dll" "system" fn MsiVerifyPackageW(szpackagepath : windows_core::PCWSTR) -> u32);
    unsafe { MsiVerifyPackageW(szpackagepath.param().abi()) }
}
pub type ADVERTISEFLAGS = i32;
pub const ADVERTISEFLAGS_MACHINEASSIGN: ADVERTISEFLAGS = 0;
pub const ADVERTISEFLAGS_USERASSIGN: ADVERTISEFLAGS = 1;
pub const ERROR_ROLLBACK_DISABLED: u32 = 1653;
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
pub type INSTALLUI_HANDLERA = Option<unsafe extern "system" fn(pvcontext: *mut core::ffi::c_void, imessagetype: u32, szmessage: windows_core::PCSTR) -> i32>;
pub type INSTALLUI_HANDLERW = Option<unsafe extern "system" fn(pvcontext: *mut core::ffi::c_void, imessagetype: u32, szmessage: windows_core::PCWSTR) -> i32>;
pub type INSTALLUI_HANDLER_RECORD = Option<unsafe extern "system" fn(pvcontext: *mut core::ffi::c_void, imessagetype: u32, hrecord: MSIHANDLE) -> i32>;
pub const MAX_FEATURE_CHARS: u32 = 38;
pub const MAX_GUID_CHARS: u32 = 38;
pub type MSIADVERTISEOPTIONFLAGS = i32;
pub const MSIADVERTISEOPTIONFLAGS_INSTANCE: MSIADVERTISEOPTIONFLAGS = 1;
pub type MSIARCHITECTUREFLAGS = i32;
pub const MSIARCHITECTUREFLAGS_AMD64: MSIARCHITECTUREFLAGS = 4;
pub const MSIARCHITECTUREFLAGS_ARM: MSIARCHITECTUREFLAGS = 8;
pub const MSIARCHITECTUREFLAGS_IA64: MSIARCHITECTUREFLAGS = 2;
pub const MSIARCHITECTUREFLAGS_X86: MSIARCHITECTUREFLAGS = 1;
pub const MSIASSEMBLYINFO_NETASSEMBLY: u32 = 0;
pub const MSIASSEMBLYINFO_WIN32ASSEMBLY: u32 = 1;
pub type MSICODE = i32;
pub const MSICODE_PATCH: MSICODE = 1073741824;
pub const MSICODE_PRODUCT: MSICODE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MSIFILEHASHINFO {
    pub dwFileHashInfoSize: u32,
    pub dwData: [u32; 4],
}
impl Default for MSIFILEHASHINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MSIHANDLE(pub u32);
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
pub type MSIPATCHSEQUENCEINFO = MSIPATCHSEQUENCEINFOA;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSIPATCHSEQUENCEINFOA {
    pub szPatchData: windows_core::PCSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MSIPATCHSEQUENCEINFOW {
    pub szPatchData: windows_core::PCWSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
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
pub const MSI_INVALID_HASH_IS_FATAL: u32 = 1;
pub type PINSTALLUI_HANDLER_RECORD = *mut INSTALLUI_HANDLER_RECORD;
pub type PMSIFILEHASHINFO = *mut MSIFILEHASHINFO;
pub type PMSIPATCHDATATYPE = *mut MSIPATCHDATATYPE;
pub type PMSIPATCHSEQUENCEINFO = PMSIPATCHSEQUENCEINFOA;
pub type PMSIPATCHSEQUENCEINFOA = *mut MSIPATCHSEQUENCEINFOA;
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
