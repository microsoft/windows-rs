#[cfg(feature = "winnt")]
windows_link::link!("msi.dll" "system" fn MsiAdvertiseProductA(szpackagepath : windows_sys::core::PCSTR, szscriptfilepath : windows_sys::core::PCSTR, sztransforms : windows_sys::core::PCSTR, lgidlanguage : super::LANGID) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("msi.dll" "system" fn MsiAdvertiseProductExA(szpackagepath : windows_sys::core::PCSTR, szscriptfilepath : windows_sys::core::PCSTR, sztransforms : windows_sys::core::PCSTR, lgidlanguage : super::LANGID, dwplatform : u32, dwoptions : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("msi.dll" "system" fn MsiAdvertiseProductExW(szpackagepath : windows_sys::core::PCWSTR, szscriptfilepath : windows_sys::core::PCWSTR, sztransforms : windows_sys::core::PCWSTR, lgidlanguage : super::LANGID, dwplatform : u32, dwoptions : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("msi.dll" "system" fn MsiAdvertiseProductW(szpackagepath : windows_sys::core::PCWSTR, szscriptfilepath : windows_sys::core::PCWSTR, sztransforms : windows_sys::core::PCWSTR, lgidlanguage : super::LANGID) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("msi.dll" "system" fn MsiAdvertiseScriptA(szscriptfile : windows_sys::core::PCSTR, dwflags : u32, phregdata : *const super::HKEY, fremoveitems : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("msi.dll" "system" fn MsiAdvertiseScriptW(szscriptfile : windows_sys::core::PCWSTR, dwflags : u32, phregdata : *const super::HKEY, fremoveitems : windows_sys::core::BOOL) -> u32);
windows_link::link!("msi.dll" "system" fn MsiApplyMultiplePatchesA(szpatchpackages : windows_sys::core::PCSTR, szproductcode : windows_sys::core::PCSTR, szpropertieslist : windows_sys::core::PCSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiApplyMultiplePatchesW(szpatchpackages : windows_sys::core::PCWSTR, szproductcode : windows_sys::core::PCWSTR, szpropertieslist : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiApplyPatchA(szpatchpackage : windows_sys::core::PCSTR, szinstallpackage : windows_sys::core::PCSTR, einstalltype : INSTALLTYPE, szcommandline : windows_sys::core::PCSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiApplyPatchW(szpatchpackage : windows_sys::core::PCWSTR, szinstallpackage : windows_sys::core::PCWSTR, einstalltype : INSTALLTYPE, szcommandline : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("msi.dll" "system" fn MsiBeginTransactionA(szname : windows_sys::core::PCSTR, dwtransactionattributes : u32, phtransactionhandle : *mut MSIHANDLE, phchangeofownerevent : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("msi.dll" "system" fn MsiBeginTransactionW(szname : windows_sys::core::PCWSTR, dwtransactionattributes : u32, phtransactionhandle : *mut MSIHANDLE, phchangeofownerevent : *mut super::HANDLE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiCloseAllHandles() -> u32);
windows_link::link!("msi.dll" "system" fn MsiCloseHandle(hany : MSIHANDLE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiCollectUserInfoA(szproduct : windows_sys::core::PCSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiCollectUserInfoW(szproduct : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiConfigureFeatureA(szproduct : windows_sys::core::PCSTR, szfeature : windows_sys::core::PCSTR, einstallstate : INSTALLSTATE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiConfigureFeatureW(szproduct : windows_sys::core::PCWSTR, szfeature : windows_sys::core::PCWSTR, einstallstate : INSTALLSTATE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiConfigureProductA(szproduct : windows_sys::core::PCSTR, iinstalllevel : i32, einstallstate : INSTALLSTATE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiConfigureProductExA(szproduct : windows_sys::core::PCSTR, iinstalllevel : i32, einstallstate : INSTALLSTATE, szcommandline : windows_sys::core::PCSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiConfigureProductExW(szproduct : windows_sys::core::PCWSTR, iinstalllevel : i32, einstallstate : INSTALLSTATE, szcommandline : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiConfigureProductW(szproduct : windows_sys::core::PCWSTR, iinstalllevel : i32, einstallstate : INSTALLSTATE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiDetermineApplicablePatchesA(szproductpackagepath : windows_sys::core::PCSTR, cpatchinfo : u32, ppatchinfo : *mut MSIPATCHSEQUENCEINFOA) -> u32);
windows_link::link!("msi.dll" "system" fn MsiDetermineApplicablePatchesW(szproductpackagepath : windows_sys::core::PCWSTR, cpatchinfo : u32, ppatchinfo : *mut MSIPATCHSEQUENCEINFOW) -> u32);
windows_link::link!("msi.dll" "system" fn MsiDeterminePatchSequenceA(szproductcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : MSIINSTALLCONTEXT, cpatchinfo : u32, ppatchinfo : *mut MSIPATCHSEQUENCEINFOA) -> u32);
windows_link::link!("msi.dll" "system" fn MsiDeterminePatchSequenceW(szproductcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, cpatchinfo : u32, ppatchinfo : *mut MSIPATCHSEQUENCEINFOW) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnableLogA(dwlogmode : u32, szlogfile : windows_sys::core::PCSTR, dwlogattributes : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnableLogW(dwlogmode : u32, szlogfile : windows_sys::core::PCWSTR, dwlogattributes : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEndTransaction(dwtransactionstate : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumClientsA(szcomponent : windows_sys::core::PCSTR, iproductindex : u32, lpproductbuf : windows_sys::core::PSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumClientsExA(szcomponent : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : u32, dwproductindex : u32, szproductbuf : *mut i8, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_sys::core::PSTR, pcchsid : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumClientsExW(szcomponent : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : u32, dwproductindex : u32, szproductbuf : *mut u16, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_sys::core::PWSTR, pcchsid : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumClientsW(szcomponent : windows_sys::core::PCWSTR, iproductindex : u32, lpproductbuf : windows_sys::core::PWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumComponentQualifiersA(szcomponent : windows_sys::core::PCSTR, iindex : u32, lpqualifierbuf : windows_sys::core::PSTR, pcchqualifierbuf : *mut u32, lpapplicationdatabuf : windows_sys::core::PSTR, pcchapplicationdatabuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumComponentQualifiersW(szcomponent : windows_sys::core::PCWSTR, iindex : u32, lpqualifierbuf : windows_sys::core::PWSTR, pcchqualifierbuf : *mut u32, lpapplicationdatabuf : windows_sys::core::PWSTR, pcchapplicationdatabuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumComponentsA(icomponentindex : u32, lpcomponentbuf : windows_sys::core::PSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumComponentsExA(szusersid : windows_sys::core::PCSTR, dwcontext : u32, dwindex : u32, szinstalledcomponentcode : *mut i8, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_sys::core::PSTR, pcchsid : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumComponentsExW(szusersid : windows_sys::core::PCWSTR, dwcontext : u32, dwindex : u32, szinstalledcomponentcode : *mut u16, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_sys::core::PWSTR, pcchsid : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumComponentsW(icomponentindex : u32, lpcomponentbuf : windows_sys::core::PWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumFeaturesA(szproduct : windows_sys::core::PCSTR, ifeatureindex : u32, lpfeaturebuf : windows_sys::core::PSTR, lpparentbuf : windows_sys::core::PSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumFeaturesW(szproduct : windows_sys::core::PCWSTR, ifeatureindex : u32, lpfeaturebuf : windows_sys::core::PWSTR, lpparentbuf : windows_sys::core::PWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumPatchesA(szproduct : windows_sys::core::PCSTR, ipatchindex : u32, lppatchbuf : windows_sys::core::PSTR, lptransformsbuf : windows_sys::core::PSTR, pcchtransformsbuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumPatchesExA(szproductcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : u32, dwfilter : u32, dwindex : u32, szpatchcode : *mut i8, sztargetproductcode : *mut i8, pdwtargetproductcontext : *mut MSIINSTALLCONTEXT, sztargetusersid : windows_sys::core::PSTR, pcchtargetusersid : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumPatchesExW(szproductcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : u32, dwfilter : u32, dwindex : u32, szpatchcode : *mut u16, sztargetproductcode : *mut u16, pdwtargetproductcontext : *mut MSIINSTALLCONTEXT, sztargetusersid : windows_sys::core::PWSTR, pcchtargetusersid : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumPatchesW(szproduct : windows_sys::core::PCWSTR, ipatchindex : u32, lppatchbuf : windows_sys::core::PWSTR, lptransformsbuf : windows_sys::core::PWSTR, pcchtransformsbuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumProductsA(iproductindex : u32, lpproductbuf : windows_sys::core::PSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumProductsExA(szproductcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : u32, dwindex : u32, szinstalledproductcode : *mut i8, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_sys::core::PSTR, pcchsid : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumProductsExW(szproductcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : u32, dwindex : u32, szinstalledproductcode : *mut u16, pdwinstalledcontext : *mut MSIINSTALLCONTEXT, szsid : windows_sys::core::PWSTR, pcchsid : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumProductsW(iproductindex : u32, lpproductbuf : windows_sys::core::PWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumRelatedProductsA(lpupgradecode : windows_sys::core::PCSTR, dwreserved : u32, iproductindex : u32, lpproductbuf : windows_sys::core::PSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiEnumRelatedProductsW(lpupgradecode : windows_sys::core::PCWSTR, dwreserved : u32, iproductindex : u32, lpproductbuf : windows_sys::core::PWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiExtractPatchXMLDataA(szpatchpath : windows_sys::core::PCSTR, dwreserved : u32, szxmldata : windows_sys::core::PSTR, pcchxmldata : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiExtractPatchXMLDataW(szpatchpath : windows_sys::core::PCWSTR, dwreserved : u32, szxmldata : windows_sys::core::PWSTR, pcchxmldata : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetComponentPathA(szproduct : windows_sys::core::PCSTR, szcomponent : windows_sys::core::PCSTR, lppathbuf : windows_sys::core::PSTR, pcchbuf : *mut u32) -> INSTALLSTATE);
windows_link::link!("msi.dll" "system" fn MsiGetComponentPathExA(szproductcode : windows_sys::core::PCSTR, szcomponentcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : MSIINSTALLCONTEXT, lpoutpathbuffer : windows_sys::core::PSTR, pcchoutpathbuffer : *mut u32) -> INSTALLSTATE);
windows_link::link!("msi.dll" "system" fn MsiGetComponentPathExW(szproductcode : windows_sys::core::PCWSTR, szcomponentcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, lpoutpathbuffer : windows_sys::core::PWSTR, pcchoutpathbuffer : *mut u32) -> INSTALLSTATE);
windows_link::link!("msi.dll" "system" fn MsiGetComponentPathW(szproduct : windows_sys::core::PCWSTR, szcomponent : windows_sys::core::PCWSTR, lppathbuf : windows_sys::core::PWSTR, pcchbuf : *mut u32) -> INSTALLSTATE);
windows_link::link!("msi.dll" "system" fn MsiGetFeatureInfoA(hproduct : MSIHANDLE, szfeature : windows_sys::core::PCSTR, lpattributes : *mut u32, lptitlebuf : windows_sys::core::PSTR, pcchtitlebuf : *mut u32, lphelpbuf : windows_sys::core::PSTR, pcchhelpbuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetFeatureInfoW(hproduct : MSIHANDLE, szfeature : windows_sys::core::PCWSTR, lpattributes : *mut u32, lptitlebuf : windows_sys::core::PWSTR, pcchtitlebuf : *mut u32, lphelpbuf : windows_sys::core::PWSTR, pcchhelpbuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetFeatureUsageA(szproduct : windows_sys::core::PCSTR, szfeature : windows_sys::core::PCSTR, pdwusecount : *mut u32, pwdateused : *mut u16) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetFeatureUsageW(szproduct : windows_sys::core::PCWSTR, szfeature : windows_sys::core::PCWSTR, pdwusecount : *mut u32, pwdateused : *mut u16) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetFileHashA(szfilepath : windows_sys::core::PCSTR, dwoptions : u32, phash : *mut MSIFILEHASHINFO) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetFileHashW(szfilepath : windows_sys::core::PCWSTR, dwoptions : u32, phash : *mut MSIFILEHASHINFO) -> u32);
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
windows_link::link!("msi.dll" "system" fn MsiGetFileSignatureInformationA(szsignedobjectpath : windows_sys::core::PCSTR, dwflags : u32, ppccertcontext : *mut super::PCCERT_CONTEXT, pbhashdata : *mut u8, pcbhashdata : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
windows_link::link!("msi.dll" "system" fn MsiGetFileSignatureInformationW(szsignedobjectpath : windows_sys::core::PCWSTR, dwflags : u32, ppccertcontext : *mut super::PCCERT_CONTEXT, pbhashdata : *mut u8, pcbhashdata : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("msi.dll" "system" fn MsiGetFileVersionA(szfilepath : windows_sys::core::PCSTR, lpversionbuf : windows_sys::core::PSTR, pcchversionbuf : *mut u32, lplangbuf : windows_sys::core::PSTR, pcchlangbuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetFileVersionW(szfilepath : windows_sys::core::PCWSTR, lpversionbuf : windows_sys::core::PWSTR, pcchversionbuf : *mut u32, lplangbuf : windows_sys::core::PWSTR, pcchlangbuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetPatchFileListA(szproductcode : windows_sys::core::PCSTR, szpatchpackages : windows_sys::core::PCSTR, pcfiles : *mut u32, pphfilerecords : *mut *mut MSIHANDLE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetPatchFileListW(szproductcode : windows_sys::core::PCWSTR, szpatchpackages : windows_sys::core::PCWSTR, pcfiles : *mut u32, pphfilerecords : *mut *mut MSIHANDLE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetPatchInfoA(szpatch : windows_sys::core::PCSTR, szattribute : windows_sys::core::PCSTR, lpvaluebuf : windows_sys::core::PSTR, pcchvaluebuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetPatchInfoExA(szpatchcode : windows_sys::core::PCSTR, szproductcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : windows_sys::core::PCSTR, lpvalue : windows_sys::core::PSTR, pcchvalue : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetPatchInfoExW(szpatchcode : windows_sys::core::PCWSTR, szproductcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : windows_sys::core::PCWSTR, lpvalue : windows_sys::core::PWSTR, pcchvalue : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetPatchInfoW(szpatch : windows_sys::core::PCWSTR, szattribute : windows_sys::core::PCWSTR, lpvaluebuf : windows_sys::core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetProductCodeA(szcomponent : windows_sys::core::PCSTR, lpbuf39 : windows_sys::core::PSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetProductCodeW(szcomponent : windows_sys::core::PCWSTR, lpbuf39 : windows_sys::core::PWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetProductInfoA(szproduct : windows_sys::core::PCSTR, szattribute : windows_sys::core::PCSTR, lpvaluebuf : windows_sys::core::PSTR, pcchvaluebuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetProductInfoExA(szproductcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : windows_sys::core::PCSTR, szvalue : windows_sys::core::PSTR, pcchvalue : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetProductInfoExW(szproductcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szproperty : windows_sys::core::PCWSTR, szvalue : windows_sys::core::PWSTR, pcchvalue : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("msi.dll" "system" fn MsiGetProductInfoFromScriptA(szscriptfile : windows_sys::core::PCSTR, lpproductbuf39 : windows_sys::core::PSTR, plgidlanguage : *mut super::LANGID, pdwversion : *mut u32, lpnamebuf : windows_sys::core::PSTR, pcchnamebuf : *mut u32, lppackagebuf : windows_sys::core::PSTR, pcchpackagebuf : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("msi.dll" "system" fn MsiGetProductInfoFromScriptW(szscriptfile : windows_sys::core::PCWSTR, lpproductbuf39 : windows_sys::core::PWSTR, plgidlanguage : *mut super::LANGID, pdwversion : *mut u32, lpnamebuf : windows_sys::core::PWSTR, pcchnamebuf : *mut u32, lppackagebuf : windows_sys::core::PWSTR, pcchpackagebuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetProductInfoW(szproduct : windows_sys::core::PCWSTR, szattribute : windows_sys::core::PCWSTR, lpvaluebuf : windows_sys::core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetProductPropertyA(hproduct : MSIHANDLE, szproperty : windows_sys::core::PCSTR, lpvaluebuf : windows_sys::core::PSTR, pcchvaluebuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetProductPropertyW(hproduct : MSIHANDLE, szproperty : windows_sys::core::PCWSTR, lpvaluebuf : windows_sys::core::PWSTR, pcchvaluebuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetShortcutTargetA(szshortcutpath : windows_sys::core::PCSTR, szproductcode : windows_sys::core::PSTR, szfeatureid : windows_sys::core::PSTR, szcomponentcode : windows_sys::core::PSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetShortcutTargetW(szshortcutpath : windows_sys::core::PCWSTR, szproductcode : windows_sys::core::PWSTR, szfeatureid : windows_sys::core::PWSTR, szcomponentcode : windows_sys::core::PWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiGetUserInfoA(szproduct : windows_sys::core::PCSTR, lpusernamebuf : windows_sys::core::PSTR, pcchusernamebuf : *mut u32, lporgnamebuf : windows_sys::core::PSTR, pcchorgnamebuf : *mut u32, lpserialbuf : windows_sys::core::PSTR, pcchserialbuf : *mut u32) -> USERINFOSTATE);
windows_link::link!("msi.dll" "system" fn MsiGetUserInfoW(szproduct : windows_sys::core::PCWSTR, lpusernamebuf : windows_sys::core::PWSTR, pcchusernamebuf : *mut u32, lporgnamebuf : windows_sys::core::PWSTR, pcchorgnamebuf : *mut u32, lpserialbuf : windows_sys::core::PWSTR, pcchserialbuf : *mut u32) -> USERINFOSTATE);
windows_link::link!("msi.dll" "system" fn MsiInstallMissingComponentA(szproduct : windows_sys::core::PCSTR, szcomponent : windows_sys::core::PCSTR, einstallstate : INSTALLSTATE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiInstallMissingComponentW(szproduct : windows_sys::core::PCWSTR, szcomponent : windows_sys::core::PCWSTR, einstallstate : INSTALLSTATE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiInstallMissingFileA(szproduct : windows_sys::core::PCSTR, szfile : windows_sys::core::PCSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiInstallMissingFileW(szproduct : windows_sys::core::PCWSTR, szfile : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiInstallProductA(szpackagepath : windows_sys::core::PCSTR, szcommandline : windows_sys::core::PCSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiInstallProductW(szpackagepath : windows_sys::core::PCWSTR, szcommandline : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiIsProductElevatedA(szproduct : windows_sys::core::PCSTR, pfelevated : *mut windows_sys::core::BOOL) -> u32);
windows_link::link!("msi.dll" "system" fn MsiIsProductElevatedW(szproduct : windows_sys::core::PCWSTR, pfelevated : *mut windows_sys::core::BOOL) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("msi.dll" "system" fn MsiJoinTransaction(htransactionhandle : MSIHANDLE, dwtransactionattributes : u32, phchangeofownerevent : *mut super::HANDLE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiLocateComponentA(szcomponent : windows_sys::core::PCSTR, lppathbuf : windows_sys::core::PSTR, pcchbuf : *mut u32) -> INSTALLSTATE);
windows_link::link!("msi.dll" "system" fn MsiLocateComponentW(szcomponent : windows_sys::core::PCWSTR, lppathbuf : windows_sys::core::PWSTR, pcchbuf : *mut u32) -> INSTALLSTATE);
windows_link::link!("msi.dll" "system" fn MsiNotifySidChangeA(poldsid : windows_sys::core::PCSTR, pnewsid : windows_sys::core::PCSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiNotifySidChangeW(poldsid : windows_sys::core::PCWSTR, pnewsid : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiOpenPackageA(szpackagepath : windows_sys::core::PCSTR, hproduct : *mut MSIHANDLE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiOpenPackageExA(szpackagepath : windows_sys::core::PCSTR, dwoptions : u32, hproduct : *mut MSIHANDLE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiOpenPackageExW(szpackagepath : windows_sys::core::PCWSTR, dwoptions : u32, hproduct : *mut MSIHANDLE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiOpenPackageW(szpackagepath : windows_sys::core::PCWSTR, hproduct : *mut MSIHANDLE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiOpenProductA(szproduct : windows_sys::core::PCSTR, hproduct : *mut MSIHANDLE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiOpenProductW(szproduct : windows_sys::core::PCWSTR, hproduct : *mut MSIHANDLE) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("msi.dll" "system" fn MsiProcessAdvertiseScriptA(szscriptfile : windows_sys::core::PCSTR, sziconfolder : windows_sys::core::PCSTR, hregdata : super::HKEY, fshortcuts : windows_sys::core::BOOL, fremoveitems : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("msi.dll" "system" fn MsiProcessAdvertiseScriptW(szscriptfile : windows_sys::core::PCWSTR, sziconfolder : windows_sys::core::PCWSTR, hregdata : super::HKEY, fshortcuts : windows_sys::core::BOOL, fremoveitems : windows_sys::core::BOOL) -> u32);
windows_link::link!("msi.dll" "system" fn MsiProvideAssemblyA(szassemblyname : windows_sys::core::PCSTR, szappcontext : windows_sys::core::PCSTR, dwinstallmode : u32, dwassemblyinfo : u32, lppathbuf : windows_sys::core::PSTR, pcchpathbuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiProvideAssemblyW(szassemblyname : windows_sys::core::PCWSTR, szappcontext : windows_sys::core::PCWSTR, dwinstallmode : u32, dwassemblyinfo : u32, lppathbuf : windows_sys::core::PWSTR, pcchpathbuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiProvideComponentA(szproduct : windows_sys::core::PCSTR, szfeature : windows_sys::core::PCSTR, szcomponent : windows_sys::core::PCSTR, dwinstallmode : u32, lppathbuf : windows_sys::core::PSTR, pcchpathbuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiProvideComponentW(szproduct : windows_sys::core::PCWSTR, szfeature : windows_sys::core::PCWSTR, szcomponent : windows_sys::core::PCWSTR, dwinstallmode : u32, lppathbuf : windows_sys::core::PWSTR, pcchpathbuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiProvideQualifiedComponentA(szcategory : windows_sys::core::PCSTR, szqualifier : windows_sys::core::PCSTR, dwinstallmode : u32, lppathbuf : windows_sys::core::PSTR, pcchpathbuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiProvideQualifiedComponentExA(szcategory : windows_sys::core::PCSTR, szqualifier : windows_sys::core::PCSTR, dwinstallmode : u32, szproduct : windows_sys::core::PCSTR, dwunused1 : u32, dwunused2 : u32, lppathbuf : windows_sys::core::PSTR, pcchpathbuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiProvideQualifiedComponentExW(szcategory : windows_sys::core::PCWSTR, szqualifier : windows_sys::core::PCWSTR, dwinstallmode : u32, szproduct : windows_sys::core::PCWSTR, dwunused1 : u32, dwunused2 : u32, lppathbuf : windows_sys::core::PWSTR, pcchpathbuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiProvideQualifiedComponentW(szcategory : windows_sys::core::PCWSTR, szqualifier : windows_sys::core::PCWSTR, dwinstallmode : u32, lppathbuf : windows_sys::core::PWSTR, pcchpathbuf : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiQueryComponentStateA(szproductcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szcomponentcode : windows_sys::core::PCSTR, pdwstate : *mut INSTALLSTATE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiQueryComponentStateW(szproductcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szcomponentcode : windows_sys::core::PCWSTR, pdwstate : *mut INSTALLSTATE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiQueryFeatureStateA(szproduct : windows_sys::core::PCSTR, szfeature : windows_sys::core::PCSTR) -> INSTALLSTATE);
windows_link::link!("msi.dll" "system" fn MsiQueryFeatureStateExA(szproductcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : MSIINSTALLCONTEXT, szfeature : windows_sys::core::PCSTR, pdwstate : *mut INSTALLSTATE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiQueryFeatureStateExW(szproductcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, szfeature : windows_sys::core::PCWSTR, pdwstate : *mut INSTALLSTATE) -> u32);
windows_link::link!("msi.dll" "system" fn MsiQueryFeatureStateW(szproduct : windows_sys::core::PCWSTR, szfeature : windows_sys::core::PCWSTR) -> INSTALLSTATE);
windows_link::link!("msi.dll" "system" fn MsiQueryProductStateA(szproduct : windows_sys::core::PCSTR) -> INSTALLSTATE);
windows_link::link!("msi.dll" "system" fn MsiQueryProductStateW(szproduct : windows_sys::core::PCWSTR) -> INSTALLSTATE);
windows_link::link!("msi.dll" "system" fn MsiReinstallFeatureA(szproduct : windows_sys::core::PCSTR, szfeature : windows_sys::core::PCSTR, dwreinstallmode : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiReinstallFeatureW(szproduct : windows_sys::core::PCWSTR, szfeature : windows_sys::core::PCWSTR, dwreinstallmode : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiReinstallProductA(szproduct : windows_sys::core::PCSTR, szreinstallmode : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiReinstallProductW(szproduct : windows_sys::core::PCWSTR, szreinstallmode : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiRemovePatchesA(szpatchlist : windows_sys::core::PCSTR, szproductcode : windows_sys::core::PCSTR, euninstalltype : INSTALLTYPE, szpropertylist : windows_sys::core::PCSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiRemovePatchesW(szpatchlist : windows_sys::core::PCWSTR, szproductcode : windows_sys::core::PCWSTR, euninstalltype : INSTALLTYPE, szpropertylist : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSetExternalUIA(puihandler : INSTALLUI_HANDLERA, dwmessagefilter : u32, pvcontext : *const core::ffi::c_void) -> INSTALLUI_HANDLERA);
windows_link::link!("msi.dll" "system" fn MsiSetExternalUIRecord(puihandler : INSTALLUI_HANDLER_RECORD, dwmessagefilter : u32, pvcontext : *const core::ffi::c_void, ppuiprevhandler : *mut INSTALLUI_HANDLER_RECORD) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSetExternalUIW(puihandler : INSTALLUI_HANDLERW, dwmessagefilter : u32, pvcontext : *const core::ffi::c_void) -> INSTALLUI_HANDLERW);
#[cfg(feature = "windef")]
windows_link::link!("msi.dll" "system" fn MsiSetInternalUI(dwuilevel : INSTALLUILEVEL, phwnd : *mut super::HWND) -> INSTALLUILEVEL);
windows_link::link!("msi.dll" "system" fn MsiSourceListAddMediaDiskA(szproductcodeorpatchcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwdiskid : u32, szvolumelabel : windows_sys::core::PCSTR, szdiskprompt : windows_sys::core::PCSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListAddMediaDiskW(szproductcodeorpatchcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwdiskid : u32, szvolumelabel : windows_sys::core::PCWSTR, szdiskprompt : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListAddSourceA(szproduct : windows_sys::core::PCSTR, szusername : windows_sys::core::PCSTR, dwreserved : u32, szsource : windows_sys::core::PCSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListAddSourceExA(szproductcodeorpatchcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szsource : windows_sys::core::PCSTR, dwindex : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListAddSourceExW(szproductcodeorpatchcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szsource : windows_sys::core::PCWSTR, dwindex : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListAddSourceW(szproduct : windows_sys::core::PCWSTR, szusername : windows_sys::core::PCWSTR, dwreserved : u32, szsource : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListClearAllA(szproduct : windows_sys::core::PCSTR, szusername : windows_sys::core::PCSTR, dwreserved : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListClearAllExA(szproductcodeorpatchcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListClearAllExW(szproductcodeorpatchcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListClearAllW(szproduct : windows_sys::core::PCWSTR, szusername : windows_sys::core::PCWSTR, dwreserved : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListClearMediaDiskA(szproductcodeorpatchcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwdiskid : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListClearMediaDiskW(szproductcodeorpatchcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwdiskid : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListClearSourceA(szproductcodeorpatchcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szsource : windows_sys::core::PCSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListClearSourceW(szproductcodeorpatchcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szsource : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListEnumMediaDisksA(szproductcodeorpatchcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwindex : u32, pdwdiskid : *mut u32, szvolumelabel : windows_sys::core::PSTR, pcchvolumelabel : *mut u32, szdiskprompt : windows_sys::core::PSTR, pcchdiskprompt : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListEnumMediaDisksW(szproductcodeorpatchcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwindex : u32, pdwdiskid : *mut u32, szvolumelabel : windows_sys::core::PWSTR, pcchvolumelabel : *mut u32, szdiskprompt : windows_sys::core::PWSTR, pcchdiskprompt : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListEnumSourcesA(szproductcodeorpatchcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwindex : u32, szsource : windows_sys::core::PSTR, pcchsource : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListEnumSourcesW(szproductcodeorpatchcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, dwindex : u32, szsource : windows_sys::core::PWSTR, pcchsource : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListForceResolutionA(szproduct : windows_sys::core::PCSTR, szusername : windows_sys::core::PCSTR, dwreserved : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListForceResolutionExA(szproductcodeorpatchcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListForceResolutionExW(szproductcodeorpatchcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListForceResolutionW(szproduct : windows_sys::core::PCWSTR, szusername : windows_sys::core::PCWSTR, dwreserved : u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListGetInfoA(szproductcodeorpatchcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szproperty : windows_sys::core::PCSTR, szvalue : windows_sys::core::PSTR, pcchvalue : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListGetInfoW(szproductcodeorpatchcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szproperty : windows_sys::core::PCWSTR, szvalue : windows_sys::core::PWSTR, pcchvalue : *mut u32) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListSetInfoA(szproductcodeorpatchcode : windows_sys::core::PCSTR, szusersid : windows_sys::core::PCSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szproperty : windows_sys::core::PCSTR, szvalue : windows_sys::core::PCSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiSourceListSetInfoW(szproductcodeorpatchcode : windows_sys::core::PCWSTR, szusersid : windows_sys::core::PCWSTR, dwcontext : MSIINSTALLCONTEXT, dwoptions : u32, szproperty : windows_sys::core::PCWSTR, szvalue : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiUseFeatureA(szproduct : windows_sys::core::PCSTR, szfeature : windows_sys::core::PCSTR) -> INSTALLSTATE);
windows_link::link!("msi.dll" "system" fn MsiUseFeatureExA(szproduct : windows_sys::core::PCSTR, szfeature : windows_sys::core::PCSTR, dwinstallmode : u32, dwreserved : u32) -> INSTALLSTATE);
windows_link::link!("msi.dll" "system" fn MsiUseFeatureExW(szproduct : windows_sys::core::PCWSTR, szfeature : windows_sys::core::PCWSTR, dwinstallmode : u32, dwreserved : u32) -> INSTALLSTATE);
windows_link::link!("msi.dll" "system" fn MsiUseFeatureW(szproduct : windows_sys::core::PCWSTR, szfeature : windows_sys::core::PCWSTR) -> INSTALLSTATE);
windows_link::link!("msi.dll" "system" fn MsiVerifyPackageA(szpackagepath : windows_sys::core::PCSTR) -> u32);
windows_link::link!("msi.dll" "system" fn MsiVerifyPackageW(szpackagepath : windows_sys::core::PCWSTR) -> u32);
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
pub type INSTALLUI_HANDLERA = Option<unsafe extern "system" fn(pvcontext: *mut core::ffi::c_void, imessagetype: u32, szmessage: windows_sys::core::PCSTR) -> i32>;
pub type INSTALLUI_HANDLERW = Option<unsafe extern "system" fn(pvcontext: *mut core::ffi::c_void, imessagetype: u32, szmessage: windows_sys::core::PCWSTR) -> i32>;
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
#[derive(Clone, Copy)]
pub struct MSIFILEHASHINFO {
    pub dwFileHashInfoSize: u32,
    pub dwData: [u32; 4],
}
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
pub type MSIPATCHSEQUENCEINFO = MSIPATCHSEQUENCEINFOA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSIPATCHSEQUENCEINFOA {
    pub szPatchData: windows_sys::core::PCSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
impl Default for MSIPATCHSEQUENCEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSIPATCHSEQUENCEINFOW {
    pub szPatchData: windows_sys::core::PCWSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
impl Default for MSIPATCHSEQUENCEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
