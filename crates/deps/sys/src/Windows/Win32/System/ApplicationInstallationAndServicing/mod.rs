#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ActivateActCtx(hactctx: super::super::Foundation::HANDLE, lpcookie: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddRefActCtx(hactctx: super::super::Foundation::HANDLE);
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaA(applyflags: i64, lpsourcename: super::super::Foundation::PSTR, lpdeltaname: super::super::Foundation::PSTR, lptargetname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lptarget: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaGetReverseB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lpreversefiletime: *const super::super::Foundation::FILETIME, lptarget: *mut DELTA_OUTPUT, lptargetreverse: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaProvidedB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lptarget: *mut ::core::ffi::c_void, utargetsize: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaW(applyflags: i64, lpsourcename: super::super::Foundation::PWSTR, lpdeltaname: super::super::Foundation::PWSTR, lptargetname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileA(patchfilename: super::super::Foundation::PSTR, oldfilename: super::super::Foundation::PSTR, newfilename: super::super::Foundation::PSTR, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileByBuffers(patchfilemapped: *const u8, patchfilesize: u32, oldfilemapped: *const u8, oldfilesize: u32, newfilebuffer: *mut *mut u8, newfilebuffersize: u32, newfileactualsize: *mut u32, newfiletime: *mut super::super::Foundation::FILETIME, applyoptionflags: u32, progresscallback: ::windows::runtime::RawPtr, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileByHandles(patchfilehandle: super::super::Foundation::HANDLE, oldfilehandle: super::super::Foundation::HANDLE, newfilehandle: super::super::Foundation::HANDLE, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileByHandlesEx(patchfilehandle: super::super::Foundation::HANDLE, oldfilehandle: super::super::Foundation::HANDLE, newfilehandle: super::super::Foundation::HANDLE, applyoptionflags: u32, progresscallback: ::windows::runtime::RawPtr, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileExA(patchfilename: super::super::Foundation::PSTR, oldfilename: super::super::Foundation::PSTR, newfilename: super::super::Foundation::PSTR, applyoptionflags: u32, progresscallback: ::windows::runtime::RawPtr, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileExW(patchfilename: super::super::Foundation::PWSTR, oldfilename: super::super::Foundation::PWSTR, newfilename: super::super::Foundation::PWSTR, applyoptionflags: u32, progresscallback: ::windows::runtime::RawPtr, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileW(patchfilename: super::super::Foundation::PWSTR, oldfilename: super::super::Foundation::PWSTR, newfilename: super::super::Foundation::PWSTR, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateActCtxA(pactctx: *const ACTCTXA) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateActCtxW(pactctx: *const ACTCTXW) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeltaA(filetypeset: i64, setflags: i64, resetflags: i64, lpsourcename: super::super::Foundation::PSTR, lptargetname: super::super::Foundation::PSTR, lpsourceoptionsname: super::super::Foundation::PSTR, lptargetoptionsname: super::super::Foundation::PSTR, globaloptions: DELTA_INPUT, lptargetfiletime: *const super::super::Foundation::FILETIME, hashalgid: u32, lpdeltaname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeltaB(filetypeset: i64, setflags: i64, resetflags: i64, source: DELTA_INPUT, target: DELTA_INPUT, sourceoptions: DELTA_INPUT, targetoptions: DELTA_INPUT, globaloptions: DELTA_INPUT, lptargetfiletime: *const super::super::Foundation::FILETIME, hashalgid: u32, lpdelta: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeltaW(filetypeset: i64, setflags: i64, resetflags: i64, lpsourcename: super::super::Foundation::PWSTR, lptargetname: super::super::Foundation::PWSTR, lpsourceoptionsname: super::super::Foundation::PWSTR, lptargetoptionsname: super::super::Foundation::PWSTR, globaloptions: DELTA_INPUT, lptargetfiletime: *const super::super::Foundation::FILETIME, hashalgid: u32, lpdeltaname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileA(oldfilename: super::super::Foundation::PSTR, newfilename: super::super::Foundation::PSTR, patchfilename: super::super::Foundation::PSTR, optionflags: u32, optiondata: *const ::core::mem::ManuallyDrop<PATCH_OPTION_DATA>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileByHandles(oldfilehandle: super::super::Foundation::HANDLE, newfilehandle: super::super::Foundation::HANDLE, patchfilehandle: super::super::Foundation::HANDLE, optionflags: u32, optiondata: *const ::core::mem::ManuallyDrop<PATCH_OPTION_DATA>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileByHandlesEx(oldfilecount: u32, oldfileinfoarray: *const PATCH_OLD_FILE_INFO_H, newfilehandle: super::super::Foundation::HANDLE, patchfilehandle: super::super::Foundation::HANDLE, optionflags: u32, optiondata: *const ::core::mem::ManuallyDrop<PATCH_OPTION_DATA>, progresscallback: ::windows::runtime::RawPtr, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileExA(oldfilecount: u32, oldfileinfoarray: *const PATCH_OLD_FILE_INFO_A, newfilename: super::super::Foundation::PSTR, patchfilename: super::super::Foundation::PSTR, optionflags: u32, optiondata: *const ::core::mem::ManuallyDrop<PATCH_OPTION_DATA>, progresscallback: ::windows::runtime::RawPtr, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileExW(oldfilecount: u32, oldfileinfoarray: *const PATCH_OLD_FILE_INFO_W, newfilename: super::super::Foundation::PWSTR, patchfilename: super::super::Foundation::PWSTR, optionflags: u32, optiondata: *const ::core::mem::ManuallyDrop<PATCH_OPTION_DATA>, progresscallback: ::windows::runtime::RawPtr, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileW(oldfilename: super::super::Foundation::PWSTR, newfilename: super::super::Foundation::PWSTR, patchfilename: super::super::Foundation::PWSTR, optionflags: u32, optiondata: *const ::core::mem::ManuallyDrop<PATCH_OPTION_DATA>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeactivateActCtx(dwflags: u32, ulcookie: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeltaFree(lpmemory: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeltaNormalizeProvidedB(filetypeset: i64, normalizeflags: i64, normalizeoptions: DELTA_INPUT, lpsource: *mut ::core::ffi::c_void, usourcesize: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractPatchHeaderToFileA(patchfilename: super::super::Foundation::PSTR, patchheaderfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractPatchHeaderToFileByHandles(patchfilehandle: super::super::Foundation::HANDLE, patchheaderfilehandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractPatchHeaderToFileW(patchfilename: super::super::Foundation::PWSTR, patchheaderfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FindActCtxSectionGuid(dwflags: u32, lpextensionguid: *const ::windows::runtime::GUID, ulsectionid: u32, lpguidtofind: *const ::windows::runtime::GUID, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FindActCtxSectionStringA(dwflags: u32, lpextensionguid: *const ::windows::runtime::GUID, ulsectionid: u32, lpstringtofind: super::super::Foundation::PSTR, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FindActCtxSectionStringW(dwflags: u32, lpextensionguid: *const ::windows::runtime::GUID, ulsectionid: u32, lpstringtofind: super::super::Foundation::PWSTR, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentActCtx(lphactctx: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaInfoA(lpdeltaname: super::super::Foundation::PSTR, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaInfoB(delta: DELTA_INPUT, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaInfoW(lpdeltaname: super::super::Foundation::PWSTR, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaSignatureA(filetypeset: i64, hashalgid: u32, lpsourcename: super::super::Foundation::PSTR, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaSignatureB(filetypeset: i64, hashalgid: u32, source: DELTA_INPUT, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaSignatureW(filetypeset: i64, hashalgid: u32, lpsourcename: super::super::Foundation::PWSTR, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilePatchSignatureA(filename: super::super::Foundation::PSTR, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilePatchSignatureByBuffer(filebufferwritable: *mut u8, filesize: u32, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilePatchSignatureByHandle(filehandle: super::super::Foundation::HANDLE, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilePatchSignatureW(filename: super::super::Foundation::PWSTR, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiAdvertiseProductA(szpackagepath: super::super::Foundation::PSTR, szscriptfilepath: super::super::Foundation::PSTR, sztransforms: super::super::Foundation::PSTR, lgidlanguage: u16) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiAdvertiseProductExA(szpackagepath: super::super::Foundation::PSTR, szscriptfilepath: super::super::Foundation::PSTR, sztransforms: super::super::Foundation::PSTR, lgidlanguage: u16, dwplatform: u32, dwoptions: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiAdvertiseProductExW(szpackagepath: super::super::Foundation::PWSTR, szscriptfilepath: super::super::Foundation::PWSTR, sztransforms: super::super::Foundation::PWSTR, lgidlanguage: u16, dwplatform: u32, dwoptions: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiAdvertiseProductW(szpackagepath: super::super::Foundation::PWSTR, szscriptfilepath: super::super::Foundation::PWSTR, sztransforms: super::super::Foundation::PWSTR, lgidlanguage: u16) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn MsiAdvertiseScriptA(szscriptfile: super::super::Foundation::PSTR, dwflags: u32, phregdata: *const super::Registry::HKEY, fremoveitems: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn MsiAdvertiseScriptW(szscriptfile: super::super::Foundation::PWSTR, dwflags: u32, phregdata: *const super::Registry::HKEY, fremoveitems: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiApplyMultiplePatchesA(szpatchpackages: super::super::Foundation::PSTR, szproductcode: super::super::Foundation::PSTR, szpropertieslist: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiApplyMultiplePatchesW(szpatchpackages: super::super::Foundation::PWSTR, szproductcode: super::super::Foundation::PWSTR, szpropertieslist: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiApplyPatchA(szpatchpackage: super::super::Foundation::PSTR, szinstallpackage: super::super::Foundation::PSTR, einstalltype: INSTALLTYPE, szcommandline: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiApplyPatchW(szpatchpackage: super::super::Foundation::PWSTR, szinstallpackage: super::super::Foundation::PWSTR, einstalltype: INSTALLTYPE, szcommandline: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiBeginTransactionA(szname: super::super::Foundation::PSTR, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiBeginTransactionW(szname: super::super::Foundation::PWSTR, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiCloseAllHandles() -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiCloseHandle(hany: MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiCollectUserInfoA(szproduct: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiCollectUserInfoW(szproduct: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureFeatureA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR, einstallstate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureFeatureW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR, einstallstate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureProductA(szproduct: super::super::Foundation::PSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureProductExA(szproduct: super::super::Foundation::PSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE, szcommandline: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureProductExW(szproduct: super::super::Foundation::PWSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE, szcommandline: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureProductW(szproduct: super::super::Foundation::PWSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiCreateRecord(cparams: u32) -> MSIHANDLE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiCreateTransformSummaryInfoA(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: super::super::Foundation::PSTR, ierrorconditions: MSITRANSFORM_ERROR, ivalidation: MSITRANSFORM_VALIDATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiCreateTransformSummaryInfoW(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: super::super::Foundation::PWSTR, ierrorconditions: MSITRANSFORM_ERROR, ivalidation: MSITRANSFORM_VALIDATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseApplyTransformA(hdatabase: MSIHANDLE, sztransformfile: super::super::Foundation::PSTR, ierrorconditions: MSITRANSFORM_ERROR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseApplyTransformW(hdatabase: MSIHANDLE, sztransformfile: super::super::Foundation::PWSTR, ierrorconditions: MSITRANSFORM_ERROR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiDatabaseCommit(hdatabase: MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseExportA(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PSTR, szfolderpath: super::super::Foundation::PSTR, szfilename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseExportW(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PWSTR, szfolderpath: super::super::Foundation::PWSTR, szfilename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseGenerateTransformA(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: super::super::Foundation::PSTR, ireserved1: i32, ireserved2: i32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseGenerateTransformW(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: super::super::Foundation::PWSTR, ireserved1: i32, ireserved2: i32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseGetPrimaryKeysA(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PSTR, phrecord: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseGetPrimaryKeysW(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PWSTR, phrecord: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseImportA(hdatabase: MSIHANDLE, szfolderpath: super::super::Foundation::PSTR, szfilename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseImportW(hdatabase: MSIHANDLE, szfolderpath: super::super::Foundation::PWSTR, szfilename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseIsTablePersistentA(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PSTR) -> MSICONDITION;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseIsTablePersistentW(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PWSTR) -> MSICONDITION;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseMergeA(hdatabase: MSIHANDLE, hdatabasemerge: MSIHANDLE, sztablename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseMergeW(hdatabase: MSIHANDLE, hdatabasemerge: MSIHANDLE, sztablename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseOpenViewA(hdatabase: MSIHANDLE, szquery: super::super::Foundation::PSTR, phview: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseOpenViewW(hdatabase: MSIHANDLE, szquery: super::super::Foundation::PWSTR, phview: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDetermineApplicablePatchesA(szproductpackagepath: super::super::Foundation::PSTR, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOA) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDetermineApplicablePatchesW(szproductpackagepath: super::super::Foundation::PWSTR, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOW) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDeterminePatchSequenceA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOA) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDeterminePatchSequenceW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOW) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDoActionA(hinstall: MSIHANDLE, szaction: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDoActionW(hinstall: MSIHANDLE, szaction: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnableLogA(dwlogmode: INSTALLOGMODE, szlogfile: super::super::Foundation::PSTR, dwlogattributes: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnableLogW(dwlogmode: INSTALLOGMODE, szlogfile: super::super::Foundation::PWSTR, dwlogattributes: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiEnableUIPreview(hdatabase: MSIHANDLE, phpreview: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiEndTransaction(dwtransactionstate: MSITRANSACTIONSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumClientsA(szcomponent: super::super::Foundation::PSTR, iproductindex: u32, lpproductbuf: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumClientsExA(szcomponent: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwproductindex: u32, szproductbuf: super::super::Foundation::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PSTR, pcchsid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumClientsExW(szcomponent: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwproductindex: u32, szproductbuf: super::super::Foundation::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PWSTR, pcchsid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumClientsW(szcomponent: super::super::Foundation::PWSTR, iproductindex: u32, lpproductbuf: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentCostsA(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PSTR, dwindex: u32, istate: INSTALLSTATE, szdrivebuf: super::super::Foundation::PSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentCostsW(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PWSTR, dwindex: u32, istate: INSTALLSTATE, szdrivebuf: super::super::Foundation::PWSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentQualifiersA(szcomponent: super::super::Foundation::PSTR, iindex: u32, lpqualifierbuf: super::super::Foundation::PSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: super::super::Foundation::PSTR, pcchapplicationdatabuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentQualifiersW(szcomponent: super::super::Foundation::PWSTR, iindex: u32, lpqualifierbuf: super::super::Foundation::PWSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: super::super::Foundation::PWSTR, pcchapplicationdatabuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentsA(icomponentindex: u32, lpcomponentbuf: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentsExA(szusersid: super::super::Foundation::PSTR, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: super::super::Foundation::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PSTR, pcchsid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentsExW(szusersid: super::super::Foundation::PWSTR, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: super::super::Foundation::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PWSTR, pcchsid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentsW(icomponentindex: u32, lpcomponentbuf: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumFeaturesA(szproduct: super::super::Foundation::PSTR, ifeatureindex: u32, lpfeaturebuf: super::super::Foundation::PSTR, lpparentbuf: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumFeaturesW(szproduct: super::super::Foundation::PWSTR, ifeatureindex: u32, lpfeaturebuf: super::super::Foundation::PWSTR, lpparentbuf: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumPatchesA(szproduct: super::super::Foundation::PSTR, ipatchindex: u32, lppatchbuf: super::super::Foundation::PSTR, lptransformsbuf: super::super::Foundation::PSTR, pcchtransformsbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumPatchesExA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: super::super::Foundation::PSTR, sztargetproductcode: super::super::Foundation::PSTR, pdwtargetproductcontext: *mut MSIINSTALLCONTEXT, sztargetusersid: super::super::Foundation::PSTR, pcchtargetusersid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumPatchesExW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: super::super::Foundation::PWSTR, sztargetproductcode: super::super::Foundation::PWSTR, pdwtargetproductcontext: *mut MSIINSTALLCONTEXT, sztargetusersid: super::super::Foundation::PWSTR, pcchtargetusersid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumPatchesW(szproduct: super::super::Foundation::PWSTR, ipatchindex: u32, lppatchbuf: super::super::Foundation::PWSTR, lptransformsbuf: super::super::Foundation::PWSTR, pcchtransformsbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumProductsA(iproductindex: u32, lpproductbuf: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumProductsExA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: u32, dwindex: u32, szinstalledproductcode: super::super::Foundation::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PSTR, pcchsid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumProductsExW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: u32, dwindex: u32, szinstalledproductcode: super::super::Foundation::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PWSTR, pcchsid: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumProductsW(iproductindex: u32, lpproductbuf: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumRelatedProductsA(lpupgradecode: super::super::Foundation::PSTR, dwreserved: u32, iproductindex: u32, lpproductbuf: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumRelatedProductsW(lpupgradecode: super::super::Foundation::PWSTR, dwreserved: u32, iproductindex: u32, lpproductbuf: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEvaluateConditionA(hinstall: MSIHANDLE, szcondition: super::super::Foundation::PSTR) -> MSICONDITION;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEvaluateConditionW(hinstall: MSIHANDLE, szcondition: super::super::Foundation::PWSTR) -> MSICONDITION;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiExtractPatchXMLDataA(szpatchpath: super::super::Foundation::PSTR, dwreserved: u32, szxmldata: super::super::Foundation::PSTR, pcchxmldata: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiExtractPatchXMLDataW(szpatchpath: super::super::Foundation::PWSTR, dwreserved: u32, szxmldata: super::super::Foundation::PWSTR, pcchxmldata: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiFormatRecordA(hinstall: MSIHANDLE, hrecord: MSIHANDLE, szresultbuf: super::super::Foundation::PSTR, pcchresultbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiFormatRecordW(hinstall: MSIHANDLE, hrecord: MSIHANDLE, szresultbuf: super::super::Foundation::PWSTR, pcchresultbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiGetActiveDatabase(hinstall: MSIHANDLE) -> MSIHANDLE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentPathA(szproduct: super::super::Foundation::PSTR, szcomponent: super::super::Foundation::PSTR, lppathbuf: super::super::Foundation::PSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentPathExA(szproductcode: super::super::Foundation::PSTR, szcomponentcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, lpoutpathbuffer: super::super::Foundation::PSTR, pcchoutpathbuffer: *mut u32) -> INSTALLSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentPathExW(szproductcode: super::super::Foundation::PWSTR, szcomponentcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, lpoutpathbuffer: super::super::Foundation::PWSTR, pcchoutpathbuffer: *mut u32) -> INSTALLSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentPathW(szproduct: super::super::Foundation::PWSTR, szcomponent: super::super::Foundation::PWSTR, lppathbuf: super::super::Foundation::PWSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentStateA(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentStateW(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PWSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiGetDatabaseState(hdatabase: MSIHANDLE) -> MSIDBSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureCostA(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PSTR, icosttree: MSICOSTTREE, istate: INSTALLSTATE, picost: *mut i32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureCostW(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, icosttree: MSICOSTTREE, istate: INSTALLSTATE, picost: *mut i32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureInfoA(hproduct: MSIHANDLE, szfeature: super::super::Foundation::PSTR, lpattributes: *mut u32, lptitlebuf: super::super::Foundation::PSTR, pcchtitlebuf: *mut u32, lphelpbuf: super::super::Foundation::PSTR, pcchhelpbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureInfoW(hproduct: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, lpattributes: *mut u32, lptitlebuf: super::super::Foundation::PWSTR, pcchtitlebuf: *mut u32, lphelpbuf: super::super::Foundation::PWSTR, pcchhelpbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureStateA(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureStateW(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureUsageA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR, pdwusecount: *mut u32, pwdateused: *mut u16) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureUsageW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR, pdwusecount: *mut u32, pwdateused: *mut u16) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureValidStatesA(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PSTR, lpinstallstates: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureValidStatesW(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, lpinstallstates: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFileHashA(szfilepath: super::super::Foundation::PSTR, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFileHashW(szfilepath: super::super::Foundation::PWSTR, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn MsiGetFileSignatureInformationA(szsignedobjectpath: super::super::Foundation::PSTR, dwflags: u32, ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, pbhashdata: *mut u8, pcbhashdata: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn MsiGetFileSignatureInformationW(szsignedobjectpath: super::super::Foundation::PWSTR, dwflags: u32, ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, pbhashdata: *mut u8, pcbhashdata: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFileVersionA(szfilepath: super::super::Foundation::PSTR, lpversionbuf: super::super::Foundation::PSTR, pcchversionbuf: *mut u32, lplangbuf: super::super::Foundation::PSTR, pcchlangbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFileVersionW(szfilepath: super::super::Foundation::PWSTR, lpversionbuf: super::super::Foundation::PWSTR, pcchversionbuf: *mut u32, lplangbuf: super::super::Foundation::PWSTR, pcchlangbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiGetLanguage(hinstall: MSIHANDLE) -> u16;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiGetLastErrorRecord() -> MSIHANDLE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetMode(hinstall: MSIHANDLE, erunmode: MSIRUNMODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchFileListA(szproductcode: super::super::Foundation::PSTR, szpatchpackages: super::super::Foundation::PSTR, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchFileListW(szproductcode: super::super::Foundation::PWSTR, szpatchpackages: super::super::Foundation::PWSTR, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchInfoA(szpatch: super::super::Foundation::PSTR, szattribute: super::super::Foundation::PSTR, lpvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchInfoExA(szpatchcode: super::super::Foundation::PSTR, szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: super::super::Foundation::PSTR, lpvalue: super::super::Foundation::PSTR, pcchvalue: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchInfoExW(szpatchcode: super::super::Foundation::PWSTR, szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: super::super::Foundation::PWSTR, lpvalue: super::super::Foundation::PWSTR, pcchvalue: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchInfoW(szpatch: super::super::Foundation::PWSTR, szattribute: super::super::Foundation::PWSTR, lpvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductCodeA(szcomponent: super::super::Foundation::PSTR, lpbuf39: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductCodeW(szcomponent: super::super::Foundation::PWSTR, lpbuf39: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoA(szproduct: super::super::Foundation::PSTR, szattribute: super::super::Foundation::PSTR, lpvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoExA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: super::super::Foundation::PSTR, szvalue: super::super::Foundation::PSTR, pcchvalue: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoExW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: super::super::Foundation::PWSTR, szvalue: super::super::Foundation::PWSTR, pcchvalue: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoFromScriptA(szscriptfile: super::super::Foundation::PSTR, lpproductbuf39: super::super::Foundation::PSTR, plgidlanguage: *mut u16, pdwversion: *mut u32, lpnamebuf: super::super::Foundation::PSTR, pcchnamebuf: *mut u32, lppackagebuf: super::super::Foundation::PSTR, pcchpackagebuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoFromScriptW(szscriptfile: super::super::Foundation::PWSTR, lpproductbuf39: super::super::Foundation::PWSTR, plgidlanguage: *mut u16, pdwversion: *mut u32, lpnamebuf: super::super::Foundation::PWSTR, pcchnamebuf: *mut u32, lppackagebuf: super::super::Foundation::PWSTR, pcchpackagebuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoW(szproduct: super::super::Foundation::PWSTR, szattribute: super::super::Foundation::PWSTR, lpvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductPropertyA(hproduct: MSIHANDLE, szproperty: super::super::Foundation::PSTR, lpvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductPropertyW(hproduct: MSIHANDLE, szproperty: super::super::Foundation::PWSTR, lpvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPropertyA(hinstall: MSIHANDLE, szname: super::super::Foundation::PSTR, szvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPropertyW(hinstall: MSIHANDLE, szname: super::super::Foundation::PWSTR, szvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetShortcutTargetA(szshortcutpath: super::super::Foundation::PSTR, szproductcode: super::super::Foundation::PSTR, szfeatureid: super::super::Foundation::PSTR, szcomponentcode: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetShortcutTargetW(szshortcutpath: super::super::Foundation::PWSTR, szproductcode: super::super::Foundation::PWSTR, szfeatureid: super::super::Foundation::PWSTR, szcomponentcode: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetSourcePathA(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PSTR, szpathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetSourcePathW(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PWSTR, szpathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetSummaryInformationA(hdatabase: MSIHANDLE, szdatabasepath: super::super::Foundation::PSTR, uiupdatecount: u32, phsummaryinfo: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetSummaryInformationW(hdatabase: MSIHANDLE, szdatabasepath: super::super::Foundation::PWSTR, uiupdatecount: u32, phsummaryinfo: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetTargetPathA(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PSTR, szpathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetTargetPathW(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PWSTR, szpathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetUserInfoA(szproduct: super::super::Foundation::PSTR, lpusernamebuf: super::super::Foundation::PSTR, pcchusernamebuf: *mut u32, lporgnamebuf: super::super::Foundation::PSTR, pcchorgnamebuf: *mut u32, lpserialbuf: super::super::Foundation::PSTR, pcchserialbuf: *mut u32) -> USERINFOSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetUserInfoW(szproduct: super::super::Foundation::PWSTR, lpusernamebuf: super::super::Foundation::PWSTR, pcchusernamebuf: *mut u32, lporgnamebuf: super::super::Foundation::PWSTR, pcchorgnamebuf: *mut u32, lpserialbuf: super::super::Foundation::PWSTR, pcchserialbuf: *mut u32) -> USERINFOSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallMissingComponentA(szproduct: super::super::Foundation::PSTR, szcomponent: super::super::Foundation::PSTR, einstallstate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallMissingComponentW(szproduct: super::super::Foundation::PWSTR, szcomponent: super::super::Foundation::PWSTR, einstallstate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallMissingFileA(szproduct: super::super::Foundation::PSTR, szfile: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallMissingFileW(szproduct: super::super::Foundation::PWSTR, szfile: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallProductA(szpackagepath: super::super::Foundation::PSTR, szcommandline: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallProductW(szpackagepath: super::super::Foundation::PWSTR, szcommandline: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiIsProductElevatedA(szproduct: super::super::Foundation::PSTR, pfelevated: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiIsProductElevatedW(szproduct: super::super::Foundation::PWSTR, pfelevated: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiJoinTransaction(htransactionhandle: MSIHANDLE, dwtransactionattributes: u32, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiLocateComponentA(szcomponent: super::super::Foundation::PSTR, lppathbuf: super::super::Foundation::PSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiLocateComponentW(szcomponent: super::super::Foundation::PWSTR, lppathbuf: super::super::Foundation::PWSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiNotifySidChangeA(poldsid: super::super::Foundation::PSTR, pnewsid: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiNotifySidChangeW(poldsid: super::super::Foundation::PWSTR, pnewsid: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenDatabaseA(szdatabasepath: super::super::Foundation::PSTR, szpersist: super::super::Foundation::PSTR, phdatabase: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenDatabaseW(szdatabasepath: super::super::Foundation::PWSTR, szpersist: super::super::Foundation::PWSTR, phdatabase: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenPackageA(szpackagepath: super::super::Foundation::PSTR, hproduct: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenPackageExA(szpackagepath: super::super::Foundation::PSTR, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenPackageExW(szpackagepath: super::super::Foundation::PWSTR, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenPackageW(szpackagepath: super::super::Foundation::PWSTR, hproduct: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenProductA(szproduct: super::super::Foundation::PSTR, hproduct: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenProductW(szproduct: super::super::Foundation::PWSTR, hproduct: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiPreviewBillboardA(hpreview: MSIHANDLE, szcontrolname: super::super::Foundation::PSTR, szbillboard: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiPreviewBillboardW(hpreview: MSIHANDLE, szcontrolname: super::super::Foundation::PWSTR, szbillboard: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiPreviewDialogA(hpreview: MSIHANDLE, szdialogname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiPreviewDialogW(hpreview: MSIHANDLE, szdialogname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn MsiProcessAdvertiseScriptA(szscriptfile: super::super::Foundation::PSTR, sziconfolder: super::super::Foundation::PSTR, hregdata: super::Registry::HKEY, fshortcuts: super::super::Foundation::BOOL, fremoveitems: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn MsiProcessAdvertiseScriptW(szscriptfile: super::super::Foundation::PWSTR, sziconfolder: super::super::Foundation::PWSTR, hregdata: super::Registry::HKEY, fshortcuts: super::super::Foundation::BOOL, fremoveitems: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiProcessMessage(hinstall: MSIHANDLE, emessagetype: INSTALLMESSAGE, hrecord: MSIHANDLE) -> i32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideAssemblyA(szassemblyname: super::super::Foundation::PSTR, szappcontext: super::super::Foundation::PSTR, dwinstallmode: INSTALLMODE, dwassemblyinfo: MSIASSEMBLYINFO, lppathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideAssemblyW(szassemblyname: super::super::Foundation::PWSTR, szappcontext: super::super::Foundation::PWSTR, dwinstallmode: INSTALLMODE, dwassemblyinfo: MSIASSEMBLYINFO, lppathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideComponentA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR, szcomponent: super::super::Foundation::PSTR, dwinstallmode: INSTALLMODE, lppathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideComponentW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR, szcomponent: super::super::Foundation::PWSTR, dwinstallmode: INSTALLMODE, lppathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideQualifiedComponentA(szcategory: super::super::Foundation::PSTR, szqualifier: super::super::Foundation::PSTR, dwinstallmode: INSTALLMODE, lppathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideQualifiedComponentExA(szcategory: super::super::Foundation::PSTR, szqualifier: super::super::Foundation::PSTR, dwinstallmode: INSTALLMODE, szproduct: super::super::Foundation::PSTR, dwunused1: u32, dwunused2: u32, lppathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideQualifiedComponentExW(szcategory: super::super::Foundation::PWSTR, szqualifier: super::super::Foundation::PWSTR, dwinstallmode: INSTALLMODE, szproduct: super::super::Foundation::PWSTR, dwunused1: u32, dwunused2: u32, lppathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideQualifiedComponentW(szcategory: super::super::Foundation::PWSTR, szqualifier: super::super::Foundation::PWSTR, dwinstallmode: INSTALLMODE, lppathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryComponentStateA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: super::super::Foundation::PSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryComponentStateW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: super::super::Foundation::PWSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryFeatureStateA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR) -> INSTALLSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryFeatureStateExA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, szfeature: super::super::Foundation::PSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryFeatureStateExW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, szfeature: super::super::Foundation::PWSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryFeatureStateW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR) -> INSTALLSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryProductStateA(szproduct: super::super::Foundation::PSTR) -> INSTALLSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryProductStateW(szproduct: super::super::Foundation::PWSTR) -> INSTALLSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiRecordClearData(hrecord: MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiRecordDataSize(hrecord: MSIHANDLE, ifield: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiRecordGetFieldCount(hrecord: MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiRecordGetInteger(hrecord: MSIHANDLE, ifield: u32) -> i32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordGetStringA(hrecord: MSIHANDLE, ifield: u32, szvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordGetStringW(hrecord: MSIHANDLE, ifield: u32, szvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordIsNull(hrecord: MSIHANDLE, ifield: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordReadStream(hrecord: MSIHANDLE, ifield: u32, szdatabuf: super::super::Foundation::PSTR, pcbdatabuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiRecordSetInteger(hrecord: MSIHANDLE, ifield: u32, ivalue: i32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordSetStreamA(hrecord: MSIHANDLE, ifield: u32, szfilepath: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordSetStreamW(hrecord: MSIHANDLE, ifield: u32, szfilepath: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordSetStringA(hrecord: MSIHANDLE, ifield: u32, szvalue: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordSetStringW(hrecord: MSIHANDLE, ifield: u32, szvalue: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiReinstallFeatureA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR, dwreinstallmode: REINSTALLMODE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiReinstallFeatureW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR, dwreinstallmode: REINSTALLMODE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiReinstallProductA(szproduct: super::super::Foundation::PSTR, szreinstallmode: REINSTALLMODE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiReinstallProductW(szproduct: super::super::Foundation::PWSTR, szreinstallmode: REINSTALLMODE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRemovePatchesA(szpatchlist: super::super::Foundation::PSTR, szproductcode: super::super::Foundation::PSTR, euninstalltype: INSTALLTYPE, szpropertylist: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRemovePatchesW(szpatchlist: super::super::Foundation::PWSTR, szproductcode: super::super::Foundation::PWSTR, euninstalltype: INSTALLTYPE, szpropertylist: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSequenceA(hinstall: MSIHANDLE, sztable: super::super::Foundation::PSTR, isequencemode: i32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSequenceW(hinstall: MSIHANDLE, sztable: super::super::Foundation::PWSTR, isequencemode: i32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetComponentStateA(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PSTR, istate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetComponentStateW(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PWSTR, istate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetExternalUIA(puihandler: ::windows::runtime::RawPtr, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void) -> ::core::option::Option<INSTALLUI_HANDLERA>;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiSetExternalUIRecord(puihandler: ::windows::runtime::RawPtr, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void, ppuiprevhandler: ::windows::runtime::RawPtr) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetExternalUIW(puihandler: ::windows::runtime::RawPtr, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void) -> ::core::option::Option<INSTALLUI_HANDLERW>;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetFeatureAttributesA(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PSTR, dwattributes: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetFeatureAttributesW(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, dwattributes: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetFeatureStateA(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PSTR, istate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetFeatureStateW(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, istate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiSetInstallLevel(hinstall: MSIHANDLE, iinstalllevel: i32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetInternalUI(dwuilevel: INSTALLUILEVEL, phwnd: *mut super::super::Foundation::HWND) -> INSTALLUILEVEL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetMode(hinstall: MSIHANDLE, erunmode: MSIRUNMODE, fstate: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetPropertyA(hinstall: MSIHANDLE, szname: super::super::Foundation::PSTR, szvalue: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetPropertyW(hinstall: MSIHANDLE, szname: super::super::Foundation::PWSTR, szvalue: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetTargetPathA(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PSTR, szfolderpath: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetTargetPathW(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PWSTR, szfolderpath: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddMediaDiskA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: super::super::Foundation::PSTR, szdiskprompt: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddMediaDiskW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: super::super::Foundation::PWSTR, szdiskprompt: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddSourceA(szproduct: super::super::Foundation::PSTR, szusername: super::super::Foundation::PSTR, dwreserved: u32, szsource: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddSourceExA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: super::super::Foundation::PSTR, dwindex: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddSourceExW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: super::super::Foundation::PWSTR, dwindex: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddSourceW(szproduct: super::super::Foundation::PWSTR, szusername: super::super::Foundation::PWSTR, dwreserved: u32, szsource: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearAllA(szproduct: super::super::Foundation::PSTR, szusername: super::super::Foundation::PSTR, dwreserved: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearAllExA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearAllExW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearAllW(szproduct: super::super::Foundation::PWSTR, szusername: super::super::Foundation::PWSTR, dwreserved: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearMediaDiskA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearMediaDiskW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearSourceA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearSourceW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListEnumMediaDisksA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: *mut u32, szvolumelabel: super::super::Foundation::PSTR, pcchvolumelabel: *mut u32, szdiskprompt: super::super::Foundation::PSTR, pcchdiskprompt: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListEnumMediaDisksW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: *mut u32, szvolumelabel: super::super::Foundation::PWSTR, pcchvolumelabel: *mut u32, szdiskprompt: super::super::Foundation::PWSTR, pcchdiskprompt: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListEnumSourcesA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: super::super::Foundation::PSTR, pcchsource: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListEnumSourcesW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: super::super::Foundation::PWSTR, pcchsource: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListForceResolutionA(szproduct: super::super::Foundation::PSTR, szusername: super::super::Foundation::PSTR, dwreserved: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListForceResolutionExA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListForceResolutionExW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListForceResolutionW(szproduct: super::super::Foundation::PWSTR, szusername: super::super::Foundation::PWSTR, dwreserved: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListGetInfoA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: super::super::Foundation::PSTR, szvalue: super::super::Foundation::PSTR, pcchvalue: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListGetInfoW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: super::super::Foundation::PWSTR, szvalue: super::super::Foundation::PWSTR, pcchvalue: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListSetInfoA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: super::super::Foundation::PSTR, szvalue: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListSetInfoW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: super::super::Foundation::PWSTR, szvalue: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSummaryInfoGetPropertyA(hsummaryinfo: MSIHANDLE, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: *mut super::super::Foundation::FILETIME, szvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiSummaryInfoGetPropertyCount(hsummaryinfo: MSIHANDLE, puipropertycount: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSummaryInfoGetPropertyW(hsummaryinfo: MSIHANDLE, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: *mut super::super::Foundation::FILETIME, szvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiSummaryInfoPersist(hsummaryinfo: MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSummaryInfoSetPropertyA(hsummaryinfo: MSIHANDLE, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut super::super::Foundation::FILETIME, szvalue: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSummaryInfoSetPropertyW(hsummaryinfo: MSIHANDLE, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut super::super::Foundation::FILETIME, szvalue: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiUseFeatureA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR) -> INSTALLSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiUseFeatureExA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR, dwinstallmode: u32, dwreserved: u32) -> INSTALLSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiUseFeatureExW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR, dwinstallmode: u32, dwreserved: u32) -> INSTALLSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiUseFeatureW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR) -> INSTALLSTATE;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiVerifyDiskSpace(hinstall: MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiVerifyPackageA(szpackagepath: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiVerifyPackageW(szpackagepath: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiViewClose(hview: MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiViewExecute(hview: MSIHANDLE, hrecord: MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiViewFetch(hview: MSIHANDLE, phrecord: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiViewGetColumnInfo(hview: MSIHANDLE, ecolumninfo: MSICOLINFO, phrecord: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiViewGetErrorA(hview: MSIHANDLE, szcolumnnamebuffer: super::super::Foundation::PSTR, pcchbuf: *mut u32) -> MSIDBERROR;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiViewGetErrorW(hview: MSIHANDLE, szcolumnnamebuffer: super::super::Foundation::PWSTR, pcchbuf: *mut u32) -> MSIDBERROR;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`*"]
    pub fn MsiViewModify(hview: MSIHANDLE, emodifymode: MSIMODIFY, hrecord: MSIHANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NormalizeFileForPatchSignature(filebuffer: *mut ::core::ffi::c_void, filesize: u32, optionflags: u32, optiondata: *const ::core::mem::ManuallyDrop<PATCH_OPTION_DATA>, newfilecoffbase: u32, newfilecofftime: u32, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE) -> i32;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryActCtxSettingsW(dwflags: u32, hactctx: super::super::Foundation::HANDLE, settingsnamespace: super::super::Foundation::PWSTR, settingname: super::super::Foundation::PWSTR, pvbuffer: super::super::Foundation::PWSTR, dwbuffer: usize, pdwwrittenorrequired: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryActCtxW(dwflags: u32, hactctx: super::super::Foundation::HANDLE, pvsubinstance: *const ::core::ffi::c_void, ulinfoclass: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: usize, pcbwrittenorrequired: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseActCtx(hactctx: super::super::Foundation::HANDLE);
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SfcGetNextProtectedFile(rpchandle: super::super::Foundation::HANDLE, protfiledata: *mut PROTECTED_FILE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SfcIsFileProtected(rpchandle: super::super::Foundation::HANDLE, protfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SfcIsKeyProtected(keyhandle: super::Registry::HKEY, subkeyname: super::super::Foundation::PWSTR, keysam: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SfpVerifyFile(pszfilename: super::super::Foundation::PSTR, pszerror: super::super::Foundation::PSTR, dwerrsize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TestApplyPatchToFileA(patchfilename: super::super::Foundation::PSTR, oldfilename: super::super::Foundation::PSTR, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TestApplyPatchToFileByBuffers(patchfilebuffer: *const u8, patchfilesize: u32, oldfilebuffer: *const u8, oldfilesize: u32, newfilesize: *mut u32, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TestApplyPatchToFileByHandles(patchfilehandle: super::super::Foundation::HANDLE, oldfilehandle: super::super::Foundation::HANDLE, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TestApplyPatchToFileW(patchfilename: super::super::Foundation::PWSTR, oldfilename: super::super::Foundation::PWSTR, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ApplicationInstallationAndServicing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ZombifyActCtx(hactctx: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
}
