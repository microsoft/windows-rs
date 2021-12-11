#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ActivateActCtx(hactctx: super::super::Foundation::HANDLE, lpcookie: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddRefActCtx(hactctx: super::super::Foundation::HANDLE);
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaA(applyflags: i64, lpsourcename: super::super::Foundation::PSTR, lpdeltaname: super::super::Foundation::PSTR, lptargetname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lptarget: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaGetReverseB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lpreversefiletime: *const super::super::Foundation::FILETIME, lptarget: *mut DELTA_OUTPUT, lptargetreverse: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaProvidedB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lptarget: *mut ::core::ffi::c_void, utargetsize: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaW(applyflags: i64, lpsourcename: super::super::Foundation::PWSTR, lpdeltaname: super::super::Foundation::PWSTR, lptargetname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileA(patchfilename: super::super::Foundation::PSTR, oldfilename: super::super::Foundation::PSTR, newfilename: super::super::Foundation::PSTR, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileByBuffers(patchfilemapped: *const u8, patchfilesize: u32, oldfilemapped: *const u8, oldfilesize: u32, newfilebuffer: *mut *mut u8, newfilebuffersize: u32, newfileactualsize: *mut u32, newfiletime: *mut super::super::Foundation::FILETIME, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileByHandles(patchfilehandle: super::super::Foundation::HANDLE, oldfilehandle: super::super::Foundation::HANDLE, newfilehandle: super::super::Foundation::HANDLE, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileByHandlesEx(patchfilehandle: super::super::Foundation::HANDLE, oldfilehandle: super::super::Foundation::HANDLE, newfilehandle: super::super::Foundation::HANDLE, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileExA(patchfilename: super::super::Foundation::PSTR, oldfilename: super::super::Foundation::PSTR, newfilename: super::super::Foundation::PSTR, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileExW(patchfilename: super::super::Foundation::PWSTR, oldfilename: super::super::Foundation::PWSTR, newfilename: super::super::Foundation::PWSTR, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileW(patchfilename: super::super::Foundation::PWSTR, oldfilename: super::super::Foundation::PWSTR, newfilename: super::super::Foundation::PWSTR, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateActCtxA(pactctx: *const ACTCTXA) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateActCtxW(pactctx: *const ACTCTXW) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeltaA(filetypeset: i64, setflags: i64, resetflags: i64, lpsourcename: super::super::Foundation::PSTR, lptargetname: super::super::Foundation::PSTR, lpsourceoptionsname: super::super::Foundation::PSTR, lptargetoptionsname: super::super::Foundation::PSTR, globaloptions: DELTA_INPUT, lptargetfiletime: *const super::super::Foundation::FILETIME, hashalgid: u32, lpdeltaname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeltaB(filetypeset: i64, setflags: i64, resetflags: i64, source: DELTA_INPUT, target: DELTA_INPUT, sourceoptions: DELTA_INPUT, targetoptions: DELTA_INPUT, globaloptions: DELTA_INPUT, lptargetfiletime: *const super::super::Foundation::FILETIME, hashalgid: u32, lpdelta: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeltaW(filetypeset: i64, setflags: i64, resetflags: i64, lpsourcename: super::super::Foundation::PWSTR, lptargetname: super::super::Foundation::PWSTR, lpsourceoptionsname: super::super::Foundation::PWSTR, lptargetoptionsname: super::super::Foundation::PWSTR, globaloptions: DELTA_INPUT, lptargetfiletime: *const super::super::Foundation::FILETIME, hashalgid: u32, lpdeltaname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileA(oldfilename: super::super::Foundation::PSTR, newfilename: super::super::Foundation::PSTR, patchfilename: super::super::Foundation::PSTR, optionflags: u32, optiondata: *const PATCH_OPTION_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileByHandles(oldfilehandle: super::super::Foundation::HANDLE, newfilehandle: super::super::Foundation::HANDLE, patchfilehandle: super::super::Foundation::HANDLE, optionflags: u32, optiondata: *const PATCH_OPTION_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileByHandlesEx(oldfilecount: u32, oldfileinfoarray: *const PATCH_OLD_FILE_INFO_H, newfilehandle: super::super::Foundation::HANDLE, patchfilehandle: super::super::Foundation::HANDLE, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileExA(oldfilecount: u32, oldfileinfoarray: *const PATCH_OLD_FILE_INFO_A, newfilename: super::super::Foundation::PSTR, patchfilename: super::super::Foundation::PSTR, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileExW(oldfilecount: u32, oldfileinfoarray: *const PATCH_OLD_FILE_INFO_W, newfilename: super::super::Foundation::PWSTR, patchfilename: super::super::Foundation::PWSTR, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileW(oldfilename: super::super::Foundation::PWSTR, newfilename: super::super::Foundation::PWSTR, patchfilename: super::super::Foundation::PWSTR, optionflags: u32, optiondata: *const PATCH_OPTION_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeactivateActCtx(dwflags: u32, ulcookie: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeltaFree(lpmemory: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeltaNormalizeProvidedB(filetypeset: i64, normalizeflags: i64, normalizeoptions: DELTA_INPUT, lpsource: *mut ::core::ffi::c_void, usourcesize: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractPatchHeaderToFileA(patchfilename: super::super::Foundation::PSTR, patchheaderfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractPatchHeaderToFileByHandles(patchfilehandle: super::super::Foundation::HANDLE, patchheaderfilehandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractPatchHeaderToFileW(patchfilename: super::super::Foundation::PWSTR, patchheaderfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation', 'Win32_System_WindowsProgramming'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FindActCtxSectionGuid(dwflags: u32, lpextensionguid: *const ::windows_sys::core::GUID, ulsectionid: u32, lpguidtofind: *const ::windows_sys::core::GUID, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation', 'Win32_System_WindowsProgramming'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FindActCtxSectionStringA(dwflags: u32, lpextensionguid: *const ::windows_sys::core::GUID, ulsectionid: u32, lpstringtofind: super::super::Foundation::PSTR, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation', 'Win32_System_WindowsProgramming'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FindActCtxSectionStringW(dwflags: u32, lpextensionguid: *const ::windows_sys::core::GUID, ulsectionid: u32, lpstringtofind: super::super::Foundation::PWSTR, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentActCtx(lphactctx: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaInfoA(lpdeltaname: super::super::Foundation::PSTR, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaInfoB(delta: DELTA_INPUT, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaInfoW(lpdeltaname: super::super::Foundation::PWSTR, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaSignatureA(filetypeset: i64, hashalgid: u32, lpsourcename: super::super::Foundation::PSTR, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaSignatureB(filetypeset: i64, hashalgid: u32, source: DELTA_INPUT, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaSignatureW(filetypeset: i64, hashalgid: u32, lpsourcename: super::super::Foundation::PWSTR, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilePatchSignatureA(filename: super::super::Foundation::PSTR, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilePatchSignatureByBuffer(filebufferwritable: *mut u8, filesize: u32, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilePatchSignatureByHandle(filehandle: super::super::Foundation::HANDLE, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilePatchSignatureW(filename: super::super::Foundation::PWSTR, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiAdvertiseProductA(szpackagepath: super::super::Foundation::PSTR, szscriptfilepath: super::super::Foundation::PSTR, sztransforms: super::super::Foundation::PSTR, lgidlanguage: u16) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiAdvertiseProductExA(szpackagepath: super::super::Foundation::PSTR, szscriptfilepath: super::super::Foundation::PSTR, sztransforms: super::super::Foundation::PSTR, lgidlanguage: u16, dwplatform: u32, dwoptions: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiAdvertiseProductExW(szpackagepath: super::super::Foundation::PWSTR, szscriptfilepath: super::super::Foundation::PWSTR, sztransforms: super::super::Foundation::PWSTR, lgidlanguage: u16, dwplatform: u32, dwoptions: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiAdvertiseProductW(szpackagepath: super::super::Foundation::PWSTR, szscriptfilepath: super::super::Foundation::PWSTR, sztransforms: super::super::Foundation::PWSTR, lgidlanguage: u16) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation', 'Win32_System_Registry'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn MsiAdvertiseScriptA(szscriptfile: super::super::Foundation::PSTR, dwflags: u32, phregdata: *const super::Registry::HKEY, fremoveitems: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation', 'Win32_System_Registry'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn MsiAdvertiseScriptW(szscriptfile: super::super::Foundation::PWSTR, dwflags: u32, phregdata: *const super::Registry::HKEY, fremoveitems: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiApplyMultiplePatchesA(szpatchpackages: super::super::Foundation::PSTR, szproductcode: super::super::Foundation::PSTR, szpropertieslist: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiApplyMultiplePatchesW(szpatchpackages: super::super::Foundation::PWSTR, szproductcode: super::super::Foundation::PWSTR, szpropertieslist: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiApplyPatchA(szpatchpackage: super::super::Foundation::PSTR, szinstallpackage: super::super::Foundation::PSTR, einstalltype: INSTALLTYPE, szcommandline: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiApplyPatchW(szpatchpackage: super::super::Foundation::PWSTR, szinstallpackage: super::super::Foundation::PWSTR, einstalltype: INSTALLTYPE, szcommandline: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiBeginTransactionA(szname: super::super::Foundation::PSTR, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiBeginTransactionW(szname: super::super::Foundation::PWSTR, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiCloseAllHandles() -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiCloseHandle(hany: MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiCollectUserInfoA(szproduct: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiCollectUserInfoW(szproduct: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureFeatureA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR, einstallstate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureFeatureW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR, einstallstate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureProductA(szproduct: super::super::Foundation::PSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureProductExA(szproduct: super::super::Foundation::PSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE, szcommandline: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureProductExW(szproduct: super::super::Foundation::PWSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE, szcommandline: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureProductW(szproduct: super::super::Foundation::PWSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiCreateRecord(cparams: u32) -> MSIHANDLE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiCreateTransformSummaryInfoA(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: super::super::Foundation::PSTR, ierrorconditions: MSITRANSFORM_ERROR, ivalidation: MSITRANSFORM_VALIDATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiCreateTransformSummaryInfoW(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: super::super::Foundation::PWSTR, ierrorconditions: MSITRANSFORM_ERROR, ivalidation: MSITRANSFORM_VALIDATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseApplyTransformA(hdatabase: MSIHANDLE, sztransformfile: super::super::Foundation::PSTR, ierrorconditions: MSITRANSFORM_ERROR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseApplyTransformW(hdatabase: MSIHANDLE, sztransformfile: super::super::Foundation::PWSTR, ierrorconditions: MSITRANSFORM_ERROR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiDatabaseCommit(hdatabase: MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseExportA(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PSTR, szfolderpath: super::super::Foundation::PSTR, szfilename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseExportW(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PWSTR, szfolderpath: super::super::Foundation::PWSTR, szfilename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseGenerateTransformA(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: super::super::Foundation::PSTR, ireserved1: i32, ireserved2: i32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseGenerateTransformW(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: super::super::Foundation::PWSTR, ireserved1: i32, ireserved2: i32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseGetPrimaryKeysA(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PSTR, phrecord: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseGetPrimaryKeysW(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PWSTR, phrecord: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseImportA(hdatabase: MSIHANDLE, szfolderpath: super::super::Foundation::PSTR, szfilename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseImportW(hdatabase: MSIHANDLE, szfolderpath: super::super::Foundation::PWSTR, szfilename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseIsTablePersistentA(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PSTR) -> MSICONDITION;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseIsTablePersistentW(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PWSTR) -> MSICONDITION;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseMergeA(hdatabase: MSIHANDLE, hdatabasemerge: MSIHANDLE, sztablename: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseMergeW(hdatabase: MSIHANDLE, hdatabasemerge: MSIHANDLE, sztablename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseOpenViewA(hdatabase: MSIHANDLE, szquery: super::super::Foundation::PSTR, phview: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseOpenViewW(hdatabase: MSIHANDLE, szquery: super::super::Foundation::PWSTR, phview: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDetermineApplicablePatchesA(szproductpackagepath: super::super::Foundation::PSTR, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOA) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDetermineApplicablePatchesW(szproductpackagepath: super::super::Foundation::PWSTR, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOW) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDeterminePatchSequenceA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOA) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDeterminePatchSequenceW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOW) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDoActionA(hinstall: MSIHANDLE, szaction: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDoActionW(hinstall: MSIHANDLE, szaction: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnableLogA(dwlogmode: INSTALLOGMODE, szlogfile: super::super::Foundation::PSTR, dwlogattributes: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnableLogW(dwlogmode: INSTALLOGMODE, szlogfile: super::super::Foundation::PWSTR, dwlogattributes: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiEnableUIPreview(hdatabase: MSIHANDLE, phpreview: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiEndTransaction(dwtransactionstate: MSITRANSACTIONSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumClientsA(szcomponent: super::super::Foundation::PSTR, iproductindex: u32, lpproductbuf: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumClientsExA(szcomponent: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwproductindex: u32, szproductbuf: super::super::Foundation::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PSTR, pcchsid: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumClientsExW(szcomponent: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwproductindex: u32, szproductbuf: super::super::Foundation::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PWSTR, pcchsid: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumClientsW(szcomponent: super::super::Foundation::PWSTR, iproductindex: u32, lpproductbuf: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentCostsA(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PSTR, dwindex: u32, istate: INSTALLSTATE, szdrivebuf: super::super::Foundation::PSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentCostsW(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PWSTR, dwindex: u32, istate: INSTALLSTATE, szdrivebuf: super::super::Foundation::PWSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentQualifiersA(szcomponent: super::super::Foundation::PSTR, iindex: u32, lpqualifierbuf: super::super::Foundation::PSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: super::super::Foundation::PSTR, pcchapplicationdatabuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentQualifiersW(szcomponent: super::super::Foundation::PWSTR, iindex: u32, lpqualifierbuf: super::super::Foundation::PWSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: super::super::Foundation::PWSTR, pcchapplicationdatabuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentsA(icomponentindex: u32, lpcomponentbuf: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentsExA(szusersid: super::super::Foundation::PSTR, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: super::super::Foundation::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PSTR, pcchsid: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentsExW(szusersid: super::super::Foundation::PWSTR, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: super::super::Foundation::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PWSTR, pcchsid: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentsW(icomponentindex: u32, lpcomponentbuf: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumFeaturesA(szproduct: super::super::Foundation::PSTR, ifeatureindex: u32, lpfeaturebuf: super::super::Foundation::PSTR, lpparentbuf: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumFeaturesW(szproduct: super::super::Foundation::PWSTR, ifeatureindex: u32, lpfeaturebuf: super::super::Foundation::PWSTR, lpparentbuf: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumPatchesA(szproduct: super::super::Foundation::PSTR, ipatchindex: u32, lppatchbuf: super::super::Foundation::PSTR, lptransformsbuf: super::super::Foundation::PSTR, pcchtransformsbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumPatchesExA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: super::super::Foundation::PSTR, sztargetproductcode: super::super::Foundation::PSTR, pdwtargetproductcontext: *mut MSIINSTALLCONTEXT, sztargetusersid: super::super::Foundation::PSTR, pcchtargetusersid: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumPatchesExW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: super::super::Foundation::PWSTR, sztargetproductcode: super::super::Foundation::PWSTR, pdwtargetproductcontext: *mut MSIINSTALLCONTEXT, sztargetusersid: super::super::Foundation::PWSTR, pcchtargetusersid: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumPatchesW(szproduct: super::super::Foundation::PWSTR, ipatchindex: u32, lppatchbuf: super::super::Foundation::PWSTR, lptransformsbuf: super::super::Foundation::PWSTR, pcchtransformsbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumProductsA(iproductindex: u32, lpproductbuf: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumProductsExA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: u32, dwindex: u32, szinstalledproductcode: super::super::Foundation::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PSTR, pcchsid: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumProductsExW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: u32, dwindex: u32, szinstalledproductcode: super::super::Foundation::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PWSTR, pcchsid: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumProductsW(iproductindex: u32, lpproductbuf: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumRelatedProductsA(lpupgradecode: super::super::Foundation::PSTR, dwreserved: u32, iproductindex: u32, lpproductbuf: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumRelatedProductsW(lpupgradecode: super::super::Foundation::PWSTR, dwreserved: u32, iproductindex: u32, lpproductbuf: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEvaluateConditionA(hinstall: MSIHANDLE, szcondition: super::super::Foundation::PSTR) -> MSICONDITION;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEvaluateConditionW(hinstall: MSIHANDLE, szcondition: super::super::Foundation::PWSTR) -> MSICONDITION;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiExtractPatchXMLDataA(szpatchpath: super::super::Foundation::PSTR, dwreserved: u32, szxmldata: super::super::Foundation::PSTR, pcchxmldata: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiExtractPatchXMLDataW(szpatchpath: super::super::Foundation::PWSTR, dwreserved: u32, szxmldata: super::super::Foundation::PWSTR, pcchxmldata: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiFormatRecordA(hinstall: MSIHANDLE, hrecord: MSIHANDLE, szresultbuf: super::super::Foundation::PSTR, pcchresultbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiFormatRecordW(hinstall: MSIHANDLE, hrecord: MSIHANDLE, szresultbuf: super::super::Foundation::PWSTR, pcchresultbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiGetActiveDatabase(hinstall: MSIHANDLE) -> MSIHANDLE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentPathA(szproduct: super::super::Foundation::PSTR, szcomponent: super::super::Foundation::PSTR, lppathbuf: super::super::Foundation::PSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentPathExA(szproductcode: super::super::Foundation::PSTR, szcomponentcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, lpoutpathbuffer: super::super::Foundation::PSTR, pcchoutpathbuffer: *mut u32) -> INSTALLSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentPathExW(szproductcode: super::super::Foundation::PWSTR, szcomponentcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, lpoutpathbuffer: super::super::Foundation::PWSTR, pcchoutpathbuffer: *mut u32) -> INSTALLSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentPathW(szproduct: super::super::Foundation::PWSTR, szcomponent: super::super::Foundation::PWSTR, lppathbuf: super::super::Foundation::PWSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentStateA(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentStateW(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PWSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiGetDatabaseState(hdatabase: MSIHANDLE) -> MSIDBSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureCostA(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PSTR, icosttree: MSICOSTTREE, istate: INSTALLSTATE, picost: *mut i32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureCostW(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, icosttree: MSICOSTTREE, istate: INSTALLSTATE, picost: *mut i32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureInfoA(hproduct: MSIHANDLE, szfeature: super::super::Foundation::PSTR, lpattributes: *mut u32, lptitlebuf: super::super::Foundation::PSTR, pcchtitlebuf: *mut u32, lphelpbuf: super::super::Foundation::PSTR, pcchhelpbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureInfoW(hproduct: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, lpattributes: *mut u32, lptitlebuf: super::super::Foundation::PWSTR, pcchtitlebuf: *mut u32, lphelpbuf: super::super::Foundation::PWSTR, pcchhelpbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureStateA(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureStateW(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureUsageA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR, pdwusecount: *mut u32, pwdateused: *mut u16) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureUsageW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR, pdwusecount: *mut u32, pwdateused: *mut u16) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureValidStatesA(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PSTR, lpinstallstates: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureValidStatesW(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, lpinstallstates: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFileHashA(szfilepath: super::super::Foundation::PSTR, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFileHashW(szfilepath: super::super::Foundation::PWSTR, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation', 'Win32_Security_Cryptography'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn MsiGetFileSignatureInformationA(szsignedobjectpath: super::super::Foundation::PSTR, dwflags: u32, ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, pbhashdata: *mut u8, pcbhashdata: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation', 'Win32_Security_Cryptography'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn MsiGetFileSignatureInformationW(szsignedobjectpath: super::super::Foundation::PWSTR, dwflags: u32, ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, pbhashdata: *mut u8, pcbhashdata: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFileVersionA(szfilepath: super::super::Foundation::PSTR, lpversionbuf: super::super::Foundation::PSTR, pcchversionbuf: *mut u32, lplangbuf: super::super::Foundation::PSTR, pcchlangbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFileVersionW(szfilepath: super::super::Foundation::PWSTR, lpversionbuf: super::super::Foundation::PWSTR, pcchversionbuf: *mut u32, lplangbuf: super::super::Foundation::PWSTR, pcchlangbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiGetLanguage(hinstall: MSIHANDLE) -> u16;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiGetLastErrorRecord() -> MSIHANDLE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetMode(hinstall: MSIHANDLE, erunmode: MSIRUNMODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchFileListA(szproductcode: super::super::Foundation::PSTR, szpatchpackages: super::super::Foundation::PSTR, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchFileListW(szproductcode: super::super::Foundation::PWSTR, szpatchpackages: super::super::Foundation::PWSTR, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchInfoA(szpatch: super::super::Foundation::PSTR, szattribute: super::super::Foundation::PSTR, lpvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchInfoExA(szpatchcode: super::super::Foundation::PSTR, szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: super::super::Foundation::PSTR, lpvalue: super::super::Foundation::PSTR, pcchvalue: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchInfoExW(szpatchcode: super::super::Foundation::PWSTR, szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: super::super::Foundation::PWSTR, lpvalue: super::super::Foundation::PWSTR, pcchvalue: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchInfoW(szpatch: super::super::Foundation::PWSTR, szattribute: super::super::Foundation::PWSTR, lpvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductCodeA(szcomponent: super::super::Foundation::PSTR, lpbuf39: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductCodeW(szcomponent: super::super::Foundation::PWSTR, lpbuf39: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoA(szproduct: super::super::Foundation::PSTR, szattribute: super::super::Foundation::PSTR, lpvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoExA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: super::super::Foundation::PSTR, szvalue: super::super::Foundation::PSTR, pcchvalue: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoExW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: super::super::Foundation::PWSTR, szvalue: super::super::Foundation::PWSTR, pcchvalue: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoFromScriptA(szscriptfile: super::super::Foundation::PSTR, lpproductbuf39: super::super::Foundation::PSTR, plgidlanguage: *mut u16, pdwversion: *mut u32, lpnamebuf: super::super::Foundation::PSTR, pcchnamebuf: *mut u32, lppackagebuf: super::super::Foundation::PSTR, pcchpackagebuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoFromScriptW(szscriptfile: super::super::Foundation::PWSTR, lpproductbuf39: super::super::Foundation::PWSTR, plgidlanguage: *mut u16, pdwversion: *mut u32, lpnamebuf: super::super::Foundation::PWSTR, pcchnamebuf: *mut u32, lppackagebuf: super::super::Foundation::PWSTR, pcchpackagebuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoW(szproduct: super::super::Foundation::PWSTR, szattribute: super::super::Foundation::PWSTR, lpvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductPropertyA(hproduct: MSIHANDLE, szproperty: super::super::Foundation::PSTR, lpvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductPropertyW(hproduct: MSIHANDLE, szproperty: super::super::Foundation::PWSTR, lpvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPropertyA(hinstall: MSIHANDLE, szname: super::super::Foundation::PSTR, szvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPropertyW(hinstall: MSIHANDLE, szname: super::super::Foundation::PWSTR, szvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetShortcutTargetA(szshortcutpath: super::super::Foundation::PSTR, szproductcode: super::super::Foundation::PSTR, szfeatureid: super::super::Foundation::PSTR, szcomponentcode: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetShortcutTargetW(szshortcutpath: super::super::Foundation::PWSTR, szproductcode: super::super::Foundation::PWSTR, szfeatureid: super::super::Foundation::PWSTR, szcomponentcode: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetSourcePathA(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PSTR, szpathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetSourcePathW(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PWSTR, szpathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetSummaryInformationA(hdatabase: MSIHANDLE, szdatabasepath: super::super::Foundation::PSTR, uiupdatecount: u32, phsummaryinfo: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetSummaryInformationW(hdatabase: MSIHANDLE, szdatabasepath: super::super::Foundation::PWSTR, uiupdatecount: u32, phsummaryinfo: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetTargetPathA(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PSTR, szpathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetTargetPathW(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PWSTR, szpathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetUserInfoA(szproduct: super::super::Foundation::PSTR, lpusernamebuf: super::super::Foundation::PSTR, pcchusernamebuf: *mut u32, lporgnamebuf: super::super::Foundation::PSTR, pcchorgnamebuf: *mut u32, lpserialbuf: super::super::Foundation::PSTR, pcchserialbuf: *mut u32) -> USERINFOSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetUserInfoW(szproduct: super::super::Foundation::PWSTR, lpusernamebuf: super::super::Foundation::PWSTR, pcchusernamebuf: *mut u32, lporgnamebuf: super::super::Foundation::PWSTR, pcchorgnamebuf: *mut u32, lpserialbuf: super::super::Foundation::PWSTR, pcchserialbuf: *mut u32) -> USERINFOSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallMissingComponentA(szproduct: super::super::Foundation::PSTR, szcomponent: super::super::Foundation::PSTR, einstallstate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallMissingComponentW(szproduct: super::super::Foundation::PWSTR, szcomponent: super::super::Foundation::PWSTR, einstallstate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallMissingFileA(szproduct: super::super::Foundation::PSTR, szfile: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallMissingFileW(szproduct: super::super::Foundation::PWSTR, szfile: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallProductA(szpackagepath: super::super::Foundation::PSTR, szcommandline: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallProductW(szpackagepath: super::super::Foundation::PWSTR, szcommandline: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiIsProductElevatedA(szproduct: super::super::Foundation::PSTR, pfelevated: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiIsProductElevatedW(szproduct: super::super::Foundation::PWSTR, pfelevated: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiJoinTransaction(htransactionhandle: MSIHANDLE, dwtransactionattributes: u32, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiLocateComponentA(szcomponent: super::super::Foundation::PSTR, lppathbuf: super::super::Foundation::PSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiLocateComponentW(szcomponent: super::super::Foundation::PWSTR, lppathbuf: super::super::Foundation::PWSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiNotifySidChangeA(poldsid: super::super::Foundation::PSTR, pnewsid: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiNotifySidChangeW(poldsid: super::super::Foundation::PWSTR, pnewsid: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenDatabaseA(szdatabasepath: super::super::Foundation::PSTR, szpersist: super::super::Foundation::PSTR, phdatabase: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenDatabaseW(szdatabasepath: super::super::Foundation::PWSTR, szpersist: super::super::Foundation::PWSTR, phdatabase: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenPackageA(szpackagepath: super::super::Foundation::PSTR, hproduct: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenPackageExA(szpackagepath: super::super::Foundation::PSTR, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenPackageExW(szpackagepath: super::super::Foundation::PWSTR, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenPackageW(szpackagepath: super::super::Foundation::PWSTR, hproduct: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenProductA(szproduct: super::super::Foundation::PSTR, hproduct: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenProductW(szproduct: super::super::Foundation::PWSTR, hproduct: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiPreviewBillboardA(hpreview: MSIHANDLE, szcontrolname: super::super::Foundation::PSTR, szbillboard: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiPreviewBillboardW(hpreview: MSIHANDLE, szcontrolname: super::super::Foundation::PWSTR, szbillboard: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiPreviewDialogA(hpreview: MSIHANDLE, szdialogname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiPreviewDialogW(hpreview: MSIHANDLE, szdialogname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation', 'Win32_System_Registry'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn MsiProcessAdvertiseScriptA(szscriptfile: super::super::Foundation::PSTR, sziconfolder: super::super::Foundation::PSTR, hregdata: super::Registry::HKEY, fshortcuts: super::super::Foundation::BOOL, fremoveitems: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation', 'Win32_System_Registry'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn MsiProcessAdvertiseScriptW(szscriptfile: super::super::Foundation::PWSTR, sziconfolder: super::super::Foundation::PWSTR, hregdata: super::Registry::HKEY, fshortcuts: super::super::Foundation::BOOL, fremoveitems: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiProcessMessage(hinstall: MSIHANDLE, emessagetype: INSTALLMESSAGE, hrecord: MSIHANDLE) -> i32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideAssemblyA(szassemblyname: super::super::Foundation::PSTR, szappcontext: super::super::Foundation::PSTR, dwinstallmode: INSTALLMODE, dwassemblyinfo: MSIASSEMBLYINFO, lppathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideAssemblyW(szassemblyname: super::super::Foundation::PWSTR, szappcontext: super::super::Foundation::PWSTR, dwinstallmode: INSTALLMODE, dwassemblyinfo: MSIASSEMBLYINFO, lppathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideComponentA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR, szcomponent: super::super::Foundation::PSTR, dwinstallmode: INSTALLMODE, lppathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideComponentW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR, szcomponent: super::super::Foundation::PWSTR, dwinstallmode: INSTALLMODE, lppathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideQualifiedComponentA(szcategory: super::super::Foundation::PSTR, szqualifier: super::super::Foundation::PSTR, dwinstallmode: INSTALLMODE, lppathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideQualifiedComponentExA(szcategory: super::super::Foundation::PSTR, szqualifier: super::super::Foundation::PSTR, dwinstallmode: INSTALLMODE, szproduct: super::super::Foundation::PSTR, dwunused1: u32, dwunused2: u32, lppathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideQualifiedComponentExW(szcategory: super::super::Foundation::PWSTR, szqualifier: super::super::Foundation::PWSTR, dwinstallmode: INSTALLMODE, szproduct: super::super::Foundation::PWSTR, dwunused1: u32, dwunused2: u32, lppathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideQualifiedComponentW(szcategory: super::super::Foundation::PWSTR, szqualifier: super::super::Foundation::PWSTR, dwinstallmode: INSTALLMODE, lppathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryComponentStateA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: super::super::Foundation::PSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryComponentStateW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: super::super::Foundation::PWSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryFeatureStateA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR) -> INSTALLSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryFeatureStateExA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, szfeature: super::super::Foundation::PSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryFeatureStateExW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, szfeature: super::super::Foundation::PWSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryFeatureStateW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR) -> INSTALLSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryProductStateA(szproduct: super::super::Foundation::PSTR) -> INSTALLSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryProductStateW(szproduct: super::super::Foundation::PWSTR) -> INSTALLSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiRecordClearData(hrecord: MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiRecordDataSize(hrecord: MSIHANDLE, ifield: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiRecordGetFieldCount(hrecord: MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiRecordGetInteger(hrecord: MSIHANDLE, ifield: u32) -> i32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordGetStringA(hrecord: MSIHANDLE, ifield: u32, szvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordGetStringW(hrecord: MSIHANDLE, ifield: u32, szvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordIsNull(hrecord: MSIHANDLE, ifield: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordReadStream(hrecord: MSIHANDLE, ifield: u32, szdatabuf: super::super::Foundation::PSTR, pcbdatabuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiRecordSetInteger(hrecord: MSIHANDLE, ifield: u32, ivalue: i32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordSetStreamA(hrecord: MSIHANDLE, ifield: u32, szfilepath: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordSetStreamW(hrecord: MSIHANDLE, ifield: u32, szfilepath: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordSetStringA(hrecord: MSIHANDLE, ifield: u32, szvalue: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordSetStringW(hrecord: MSIHANDLE, ifield: u32, szvalue: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiReinstallFeatureA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR, dwreinstallmode: REINSTALLMODE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiReinstallFeatureW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR, dwreinstallmode: REINSTALLMODE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiReinstallProductA(szproduct: super::super::Foundation::PSTR, szreinstallmode: REINSTALLMODE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiReinstallProductW(szproduct: super::super::Foundation::PWSTR, szreinstallmode: REINSTALLMODE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRemovePatchesA(szpatchlist: super::super::Foundation::PSTR, szproductcode: super::super::Foundation::PSTR, euninstalltype: INSTALLTYPE, szpropertylist: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRemovePatchesW(szpatchlist: super::super::Foundation::PWSTR, szproductcode: super::super::Foundation::PWSTR, euninstalltype: INSTALLTYPE, szpropertylist: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSequenceA(hinstall: MSIHANDLE, sztable: super::super::Foundation::PSTR, isequencemode: i32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSequenceW(hinstall: MSIHANDLE, sztable: super::super::Foundation::PWSTR, isequencemode: i32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetComponentStateA(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PSTR, istate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetComponentStateW(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PWSTR, istate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetExternalUIA(puihandler: INSTALLUI_HANDLERA, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void) -> INSTALLUI_HANDLERA;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiSetExternalUIRecord(puihandler: PINSTALLUI_HANDLER_RECORD, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void, ppuiprevhandler: PINSTALLUI_HANDLER_RECORD) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetExternalUIW(puihandler: INSTALLUI_HANDLERW, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void) -> INSTALLUI_HANDLERW;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetFeatureAttributesA(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PSTR, dwattributes: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetFeatureAttributesW(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, dwattributes: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetFeatureStateA(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PSTR, istate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetFeatureStateW(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, istate: INSTALLSTATE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiSetInstallLevel(hinstall: MSIHANDLE, iinstalllevel: i32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetInternalUI(dwuilevel: INSTALLUILEVEL, phwnd: *mut super::super::Foundation::HWND) -> INSTALLUILEVEL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetMode(hinstall: MSIHANDLE, erunmode: MSIRUNMODE, fstate: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetPropertyA(hinstall: MSIHANDLE, szname: super::super::Foundation::PSTR, szvalue: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetPropertyW(hinstall: MSIHANDLE, szname: super::super::Foundation::PWSTR, szvalue: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetTargetPathA(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PSTR, szfolderpath: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetTargetPathW(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PWSTR, szfolderpath: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddMediaDiskA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: super::super::Foundation::PSTR, szdiskprompt: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddMediaDiskW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: super::super::Foundation::PWSTR, szdiskprompt: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddSourceA(szproduct: super::super::Foundation::PSTR, szusername: super::super::Foundation::PSTR, dwreserved: u32, szsource: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddSourceExA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: super::super::Foundation::PSTR, dwindex: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddSourceExW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: super::super::Foundation::PWSTR, dwindex: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddSourceW(szproduct: super::super::Foundation::PWSTR, szusername: super::super::Foundation::PWSTR, dwreserved: u32, szsource: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearAllA(szproduct: super::super::Foundation::PSTR, szusername: super::super::Foundation::PSTR, dwreserved: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearAllExA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearAllExW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearAllW(szproduct: super::super::Foundation::PWSTR, szusername: super::super::Foundation::PWSTR, dwreserved: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearMediaDiskA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearMediaDiskW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearSourceA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearSourceW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListEnumMediaDisksA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: *mut u32, szvolumelabel: super::super::Foundation::PSTR, pcchvolumelabel: *mut u32, szdiskprompt: super::super::Foundation::PSTR, pcchdiskprompt: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListEnumMediaDisksW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: *mut u32, szvolumelabel: super::super::Foundation::PWSTR, pcchvolumelabel: *mut u32, szdiskprompt: super::super::Foundation::PWSTR, pcchdiskprompt: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListEnumSourcesA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: super::super::Foundation::PSTR, pcchsource: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListEnumSourcesW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: super::super::Foundation::PWSTR, pcchsource: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListForceResolutionA(szproduct: super::super::Foundation::PSTR, szusername: super::super::Foundation::PSTR, dwreserved: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListForceResolutionExA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListForceResolutionExW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListForceResolutionW(szproduct: super::super::Foundation::PWSTR, szusername: super::super::Foundation::PWSTR, dwreserved: u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListGetInfoA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: super::super::Foundation::PSTR, szvalue: super::super::Foundation::PSTR, pcchvalue: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListGetInfoW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: super::super::Foundation::PWSTR, szvalue: super::super::Foundation::PWSTR, pcchvalue: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListSetInfoA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: super::super::Foundation::PSTR, szvalue: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListSetInfoW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: super::super::Foundation::PWSTR, szvalue: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSummaryInfoGetPropertyA(hsummaryinfo: MSIHANDLE, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: *mut super::super::Foundation::FILETIME, szvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiSummaryInfoGetPropertyCount(hsummaryinfo: MSIHANDLE, puipropertycount: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSummaryInfoGetPropertyW(hsummaryinfo: MSIHANDLE, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: *mut super::super::Foundation::FILETIME, szvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiSummaryInfoPersist(hsummaryinfo: MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSummaryInfoSetPropertyA(hsummaryinfo: MSIHANDLE, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut super::super::Foundation::FILETIME, szvalue: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSummaryInfoSetPropertyW(hsummaryinfo: MSIHANDLE, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut super::super::Foundation::FILETIME, szvalue: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiUseFeatureA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR) -> INSTALLSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiUseFeatureExA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR, dwinstallmode: u32, dwreserved: u32) -> INSTALLSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiUseFeatureExW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR, dwinstallmode: u32, dwreserved: u32) -> INSTALLSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiUseFeatureW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR) -> INSTALLSTATE;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiVerifyDiskSpace(hinstall: MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiVerifyPackageA(szpackagepath: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiVerifyPackageW(szpackagepath: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiViewClose(hview: MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiViewExecute(hview: MSIHANDLE, hrecord: MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiViewFetch(hview: MSIHANDLE, phrecord: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiViewGetColumnInfo(hview: MSIHANDLE, ecolumninfo: MSICOLINFO, phrecord: *mut MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiViewGetErrorA(hview: MSIHANDLE, szcolumnnamebuffer: super::super::Foundation::PSTR, pcchbuf: *mut u32) -> MSIDBERROR;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiViewGetErrorW(hview: MSIHANDLE, szcolumnnamebuffer: super::super::Foundation::PWSTR, pcchbuf: *mut u32) -> MSIDBERROR;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
    pub fn MsiViewModify(hview: MSIHANDLE, emodifymode: MSIMODIFY, hrecord: MSIHANDLE) -> u32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NormalizeFileForPatchSignature(filebuffer: *mut ::core::ffi::c_void, filesize: u32, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, newfilecoffbase: u32, newfilecofftime: u32, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE) -> i32;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryActCtxSettingsW(dwflags: u32, hactctx: super::super::Foundation::HANDLE, settingsnamespace: super::super::Foundation::PWSTR, settingname: super::super::Foundation::PWSTR, pvbuffer: super::super::Foundation::PWSTR, dwbuffer: usize, pdwwrittenorrequired: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryActCtxW(dwflags: u32, hactctx: super::super::Foundation::HANDLE, pvsubinstance: *const ::core::ffi::c_void, ulinfoclass: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: usize, pcbwrittenorrequired: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseActCtx(hactctx: super::super::Foundation::HANDLE);
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SfcGetNextProtectedFile(rpchandle: super::super::Foundation::HANDLE, protfiledata: *mut PROTECTED_FILE_DATA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SfcIsFileProtected(rpchandle: super::super::Foundation::HANDLE, protfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation', 'Win32_System_Registry'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SfcIsKeyProtected(keyhandle: super::Registry::HKEY, subkeyname: super::super::Foundation::PWSTR, keysam: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SfpVerifyFile(pszfilename: super::super::Foundation::PSTR, pszerror: super::super::Foundation::PSTR, dwerrsize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TestApplyPatchToFileA(patchfilename: super::super::Foundation::PSTR, oldfilename: super::super::Foundation::PSTR, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TestApplyPatchToFileByBuffers(patchfilebuffer: *const u8, patchfilesize: u32, oldfilebuffer: *const u8, oldfilesize: u32, newfilesize: *mut u32, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TestApplyPatchToFileByHandles(patchfilehandle: super::super::Foundation::HANDLE, oldfilehandle: super::super::Foundation::HANDLE, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TestApplyPatchToFileW(patchfilename: super::super::Foundation::PWSTR, oldfilename: super::super::Foundation::PWSTR, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ZombifyActCtx(hactctx: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTCTXA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpSource: super::super::Foundation::PSTR,
    pub wProcessorArchitecture: u16,
    pub wLangId: u16,
    pub lpAssemblyDirectory: super::super::Foundation::PSTR,
    pub lpResourceName: super::super::Foundation::PSTR,
    pub lpApplicationName: super::super::Foundation::PSTR,
    pub hModule: super::super::Foundation::HINSTANCE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTCTXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTCTXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTCTXW {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpSource: super::super::Foundation::PWSTR,
    pub wProcessorArchitecture: u16,
    pub wLangId: u16,
    pub lpAssemblyDirectory: super::super::Foundation::PWSTR,
    pub lpResourceName: super::super::Foundation::PWSTR,
    pub lpApplicationName: super::super::Foundation::PWSTR,
    pub hModule: super::super::Foundation::HINSTANCE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTCTXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTCTXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type ACTCTX_COMPATIBILITY_ELEMENT_TYPE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_UNKNOWN: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_OS: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MITIGATION: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MAXVERSIONTESTED: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type ACTCTX_REQUESTED_RUN_LEVEL = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ACTCTX_RUN_LEVEL_UNSPECIFIED: ACTCTX_REQUESTED_RUN_LEVEL = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ACTCTX_RUN_LEVEL_AS_INVOKER: ACTCTX_REQUESTED_RUN_LEVEL = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ACTCTX_RUN_LEVEL_HIGHEST_AVAILABLE: ACTCTX_REQUESTED_RUN_LEVEL = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ACTCTX_RUN_LEVEL_REQUIRE_ADMIN: ACTCTX_REQUESTED_RUN_LEVEL = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ACTCTX_RUN_LEVEL_NUMBERS: ACTCTX_REQUESTED_RUN_LEVEL = 4i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation', 'Win32_System_WindowsProgramming'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
pub struct ACTCTX_SECTION_KEYED_DATA {
    pub cbSize: u32,
    pub ulDataFormatVersion: u32,
    pub lpData: *mut ::core::ffi::c_void,
    pub ulLength: u32,
    pub lpSectionGlobalData: *mut ::core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
    pub lpSectionBase: *mut ::core::ffi::c_void,
    pub ulSectionTotalLength: u32,
    pub hActCtx: super::super::Foundation::HANDLE,
    pub ulAssemblyRosterIndex: u32,
    pub ulFlags: u32,
    pub AssemblyMetadata: super::WindowsProgramming::ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::core::marker::Copy for ACTCTX_SECTION_KEYED_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
impl ::core::clone::Clone for ACTCTX_SECTION_KEYED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    pub ulFlags: u32,
    pub ulEncodedAssemblyIdentityLength: u32,
    pub ulManifestPathType: u32,
    pub ulManifestPathLength: u32,
    pub liManifestLastWriteTime: i64,
    pub ulPolicyPathType: u32,
    pub ulPolicyPathLength: u32,
    pub liPolicyLastWriteTime: i64,
    pub ulMetadataSatelliteRosterIndex: u32,
    pub ulManifestVersionMajor: u32,
    pub ulManifestVersionMinor: u32,
    pub ulPolicyVersionMajor: u32,
    pub ulPolicyVersionMinor: u32,
    pub ulAssemblyDirectoryNameLength: u32,
    pub lpAssemblyEncodedAssemblyIdentity: super::super::Foundation::PWSTR,
    pub lpAssemblyManifestPath: super::super::Foundation::PWSTR,
    pub lpAssemblyPolicyPath: super::super::Foundation::PWSTR,
    pub lpAssemblyDirectoryName: super::super::Foundation::PWSTR,
    pub ulFileCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub struct ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    pub ElementCount: u32,
    pub Elements: [COMPATIBILITY_CONTEXT_ELEMENT; 1],
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    pub dwFlags: u32,
    pub ulFormatVersion: u32,
    pub ulAssemblyCount: u32,
    pub ulRootManifestPathType: u32,
    pub ulRootManifestPathChars: u32,
    pub ulRootConfigurationPathType: u32,
    pub ulRootConfigurationPathChars: u32,
    pub ulAppDirPathType: u32,
    pub ulAppDirPathChars: u32,
    pub lpRootManifestPath: super::super::Foundation::PWSTR,
    pub lpRootConfigurationPath: super::super::Foundation::PWSTR,
    pub lpAppDirPath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ACTIVATION_CONTEXT_DETAILED_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub struct ACTIVATION_CONTEXT_QUERY_INDEX {
    pub ulAssemblyIndex: u32,
    pub ulFileIndexInAssembly: u32,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_QUERY_INDEX {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_QUERY_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub struct ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    pub ulFlags: u32,
    pub RunLevel: ACTCTX_REQUESTED_RUN_LEVEL,
    pub UiAccess: u32,
}
impl ::core::marker::Copy for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {}
impl ::core::clone::Clone for ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type ADVERTISEFLAGS = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ADVERTISEFLAGS_MACHINEASSIGN: ADVERTISEFLAGS = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ADVERTISEFLAGS_USERASSIGN: ADVERTISEFLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const APPLY_OPTION_FAIL_IF_CLOSE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const APPLY_OPTION_FAIL_IF_EXACT: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const APPLY_OPTION_TEST_ONLY: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const APPLY_OPTION_VALID_FLAGS: u32 = 7u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type ASM_BIND_FLAGS = u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_BINDF_FORCE_CACHE_INSTALL: ASM_BIND_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_BINDF_RFS_INTEGRITY_CHECK: ASM_BIND_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_BINDF_RFS_MODULE_CHECK: ASM_BIND_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_BINDF_BINPATH_PROBE_ONLY: ASM_BIND_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_BINDF_SHARED_BINPATH_HINT: ASM_BIND_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_BINDF_PARENT_ASM_HINT: ASM_BIND_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type ASM_CMP_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_CMPF_NAME: ASM_CMP_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_CMPF_MAJOR_VERSION: ASM_CMP_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_CMPF_MINOR_VERSION: ASM_CMP_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_CMPF_BUILD_NUMBER: ASM_CMP_FLAGS = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_CMPF_REVISION_NUMBER: ASM_CMP_FLAGS = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_CMPF_PUBLIC_KEY_TOKEN: ASM_CMP_FLAGS = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_CMPF_CULTURE: ASM_CMP_FLAGS = 64i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_CMPF_CUSTOM: ASM_CMP_FLAGS = 128i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_CMPF_ALL: ASM_CMP_FLAGS = 255i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_CMPF_DEFAULT: ASM_CMP_FLAGS = 256i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type ASM_DISPLAY_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_DISPLAYF_VERSION: ASM_DISPLAY_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_DISPLAYF_CULTURE: ASM_DISPLAY_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_DISPLAYF_PUBLIC_KEY_TOKEN: ASM_DISPLAY_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_DISPLAYF_PUBLIC_KEY: ASM_DISPLAY_FLAGS = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_DISPLAYF_CUSTOM: ASM_DISPLAY_FLAGS = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_DISPLAYF_PROCESSORARCHITECTURE: ASM_DISPLAY_FLAGS = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_DISPLAYF_LANGUAGEID: ASM_DISPLAY_FLAGS = 64i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type ASM_NAME = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_PUBLIC_KEY: ASM_NAME = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_PUBLIC_KEY_TOKEN: ASM_NAME = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_HASH_VALUE: ASM_NAME = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_NAME: ASM_NAME = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_MAJOR_VERSION: ASM_NAME = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_MINOR_VERSION: ASM_NAME = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_BUILD_NUMBER: ASM_NAME = 6i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_REVISION_NUMBER: ASM_NAME = 7i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_CULTURE: ASM_NAME = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_PROCESSOR_ID_ARRAY: ASM_NAME = 9i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_OSINFO_ARRAY: ASM_NAME = 10i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_HASH_ALGID: ASM_NAME = 11i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_ALIAS: ASM_NAME = 12i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_CODEBASE_URL: ASM_NAME = 13i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_CODEBASE_LASTMOD: ASM_NAME = 14i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_NULL_PUBLIC_KEY: ASM_NAME = 15i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_NULL_PUBLIC_KEY_TOKEN: ASM_NAME = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_CUSTOM: ASM_NAME = 17i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_NULL_CUSTOM: ASM_NAME = 18i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_MVID: ASM_NAME = 19i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASM_NAME_MAX_PARAMS: ASM_NAME = 20i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASSEMBLYINFO_FLAG_INSTALLED: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ASSEMBLYINFO_FLAG_PAYLOADRESIDENT: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ASSEMBLY_FILE_DETAILED_INFORMATION {
    pub ulFlags: u32,
    pub ulFilenameLength: u32,
    pub ulPathLength: u32,
    pub lpFileName: super::super::Foundation::PWSTR,
    pub lpFilePath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ASSEMBLY_FILE_DETAILED_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ASSEMBLY_FILE_DETAILED_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ASSEMBLY_INFO {
    pub cbAssemblyInfo: u32,
    pub dwAssemblyFlags: u32,
    pub uliAssemblySizeInKB: u64,
    pub pszCurrentAssemblyPathBuf: super::super::Foundation::PWSTR,
    pub cchBuf: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ASSEMBLY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ASSEMBLY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CLSID_EvalCom2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1851660560, data2: 32851, data3: 18016, data4: [183, 149, 107, 97, 46, 41, 188, 88] };
pub const CLSID_MsmMerge2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4182345173, data2: 10745, data3: 18243, data4: [152, 5, 153, 188, 63, 53, 182, 120] };
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub struct COMPATIBILITY_CONTEXT_ELEMENT {
    pub Id: ::windows_sys::core::GUID,
    pub Type: ACTCTX_COMPATIBILITY_ELEMENT_TYPE,
    pub MaxVersionTested: u64,
}
impl ::core::marker::Copy for COMPATIBILITY_CONTEXT_ELEMENT {}
impl ::core::clone::Clone for COMPATIBILITY_CONTEXT_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type CREATE_ASM_NAME_OBJ_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const CANOF_PARSE_DISPLAY_NAME: CREATE_ASM_NAME_OBJ_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const CANOF_SET_DEFAULT_VALUES: CREATE_ASM_NAME_OBJ_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const DEFAULT_DISK_ID: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const DEFAULT_FILE_SEQUENCE_START: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const DEFAULT_MINIMUM_REQUIRED_MSI_VERSION: u32 = 100u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub struct DELTA_HASH {
    pub HashSize: u32,
    pub HashValue: [u8; 32],
}
impl ::core::marker::Copy for DELTA_HASH {}
impl ::core::clone::Clone for DELTA_HASH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DELTA_HEADER_INFO {
    pub FileTypeSet: i64,
    pub FileType: i64,
    pub Flags: i64,
    pub TargetSize: usize,
    pub TargetFileTime: super::super::Foundation::FILETIME,
    pub TargetHashAlgId: u32,
    pub TargetHash: DELTA_HASH,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELTA_HEADER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELTA_HEADER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DELTA_INPUT {
    pub Anonymous: DELTA_INPUT_0,
    pub uSize: usize,
    pub Editable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELTA_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELTA_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union DELTA_INPUT_0 {
    pub lpcStart: *mut ::core::ffi::c_void,
    pub lpStart: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DELTA_INPUT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DELTA_INPUT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const DELTA_MAX_HASH_SIZE: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub struct DELTA_OUTPUT {
    pub lpStart: *mut ::core::ffi::c_void,
    pub uSize: usize,
}
impl ::core::marker::Copy for DELTA_OUTPUT {}
impl ::core::clone::Clone for DELTA_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PATCH_BIGGER_THAN_COMPRESSED: u32 = 3222155525u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PATCH_CORRUPT: u32 = 3222159618u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PATCH_DECODE_FAILURE: u32 = 3222159617u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PATCH_ENCODE_FAILURE: u32 = 3222155521u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PATCH_IMAGEHLP_FAILURE: u32 = 3222155526u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PATCH_INVALID_OPTIONS: u32 = 3222155522u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PATCH_NEWER_FORMAT: u32 = 3222159619u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PATCH_NOT_AVAILABLE: u32 = 3222159622u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PATCH_NOT_NECESSARY: u32 = 3222159621u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PATCH_RETAIN_RANGES_DIFFER: u32 = 3222155524u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PATCH_SAME_FILE: u32 = 3222155523u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PATCH_WRONG_FILE: u32 = 3222159620u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_API_PATCHING_SYMBOL_FLAGS: u32 = 3222163725u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_FAMILY_RANGE_NAME: u32 = 3222163801u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_FILE_SEQUENCE_START: u32 = 3222163770u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_GUIDS_TO_REPLACE: u32 = 3222163721u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_IMAGE_FAMILY_DISKID: u32 = 3222163773u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_IMAGE_FAMILY_FILESEQSTART: u32 = 3222163774u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_IMAGE_FAMILY_NAME: u32 = 3222163748u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_IMAGE_FAMILY_SRC_PROP: u32 = 3222163750u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_MAJOR_VERSION: u32 = 3222163853u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_PATCH_GUID: u32 = 3222163720u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_PRODUCTVERSION_VALIDATION: u32 = 3222163844u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_SEQUENCE: u32 = 3222163848u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_SUPERCEDENCE: u32 = 3222163847u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_TARGET: u32 = 3222163849u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_NAME: u32 = 3222163736u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_PRODUCT_CODE: u32 = 3222163834u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_PRODUCT_VERSION: u32 = 3222163835u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_UPGRADED: u32 = 3222163776u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_TARGET_IMAGE_UPGRADE_CODE: u32 = 3222163836u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_TARGET_PRODUCT_CODE_LIST: u32 = 3222163722u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_TGT_UPD_IMAGES: u32 = 3222163846u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_TRANSFORMSET: u32 = 3222163845u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_FAMILY: u32 = 3222163775u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_NAME: u32 = 3222163728u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_PRODUCT_CODE: u32 = 3222163831u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_PRODUCT_VERSION: u32 = 3222163832u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_UPGRADE_CODE: u32 = 3222163833u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BAD_VERSION_STRING: u32 = 3222163852u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_BASE: u32 = 3222163713u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_CANNOT_CREATE_TABLE: u32 = 3222163841u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_CANNOT_RUN_MAKECAB: u32 = 3222163782u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_CANNOT_WRITE_DDF: u32 = 3222163781u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_CANT_COPY_FILE_TO_TEMP_FOLDER: u32 = 3222163771u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_CANT_CREATE_ONE_PATCH_FILE: u32 = 3222163772u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_CANT_CREATE_PATCH_FILE: u32 = 3222163718u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_CANT_CREATE_SUMMARY_INFO: u32 = 3222163828u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_CANT_CREATE_SUMMARY_INFO_POUND: u32 = 3222163830u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_CANT_CREATE_TEMP_FOLDER: u32 = 3222163715u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_CANT_DELETE_TEMP_FOLDER: u32 = 3222163974u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_CANT_GENERATE_SEQUENCEINFO_MAJORUPGD: u32 = 3222163842u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_CANT_GENERATE_TRANSFORM: u32 = 3222163827u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_CANT_GENERATE_TRANSFORM_POUND: u32 = 3222163829u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_CANT_OVERWRITE_PATCH: u32 = 3222163717u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_CANT_READ_FILE: u32 = 3222163978u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_CREATEFILE_LOG_FAILED: u32 = 3222163861u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_DUPLICATE_SEQUENCE_RECORD: u32 = 3222163858u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_DUP_IMAGE_FAMILY_NAME: u32 = 3222163749u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_DUP_TARGET_IMAGE_NAME: u32 = 3222163737u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_DUP_TARGET_IMAGE_PACKCODE: u32 = 3222163777u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_DUP_UPGRADED_IMAGE_NAME: u32 = 3222163729u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_DUP_UPGRADED_IMAGE_PACKCODE: u32 = 3222163795u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_ERROR_WRITING_TO_LOG: u32 = 3222163864u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_EXECUTE_VIEW: u32 = 3222163870u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_EXTFILE_BAD_FAMILY_FIELD: u32 = 3222163756u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_EXTFILE_BAD_IGNORE_LENGTHS: u32 = 3222163814u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_EXTFILE_BAD_IGNORE_OFFSETS: u32 = 3222163812u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_EXTFILE_BAD_RETAIN_OFFSETS: u32 = 3222163817u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_EXTFILE_BLANK_FILE_TABLE_KEY: u32 = 3222163755u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_EXTFILE_BLANK_PATH_TO_FILE: u32 = 3222163758u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_EXTFILE_IGNORE_COUNT_MISMATCH: u32 = 3222163815u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_EXTFILE_LONG_FILE_TABLE_KEY: u32 = 3222163754u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_EXTFILE_LONG_IGNORE_LENGTHS: u32 = 3222163813u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_EXTFILE_LONG_IGNORE_OFFSETS: u32 = 3222163811u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_EXTFILE_LONG_PATH_TO_FILE: u32 = 3222163757u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_EXTFILE_LONG_RETAIN_OFFSETS: u32 = 3222163816u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_EXTFILE_MISSING_FILE: u32 = 3222163759u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_FAILED_CREATE_TRANSFORM: u32 = 3222163973u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_FAILED_EXPAND_PATH: u32 = 3222163872u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_FAMILY_RANGE_BAD_RETAIN_LENGTHS: u32 = 3222163809u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_FAMILY_RANGE_BAD_RETAIN_OFFSETS: u32 = 3222163806u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_FAMILY_RANGE_BLANK_FILE_TABLE_KEY: u32 = 3222163803u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_FAMILY_RANGE_BLANK_RETAIN_LENGTHS: u32 = 3222163808u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_FAMILY_RANGE_BLANK_RETAIN_OFFSETS: u32 = 3222163805u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_FAMILY_RANGE_COUNT_MISMATCH: u32 = 3222163810u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_FAMILY_RANGE_LONG_FILE_TABLE_KEY: u32 = 3222163802u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_FAMILY_RANGE_LONG_RETAIN_LENGTHS: u32 = 3222163807u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_FAMILY_RANGE_LONG_RETAIN_OFFSETS: u32 = 3222163804u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_FAMILY_RANGE_NAME_TOO_LONG: u32 = 3222163800u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_IMAGE_FAMILY_NAME_TOO_LONG: u32 = 3222163747u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_IMAGE_PATH_NOT_EXIST: u32 = 3222163988u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INTERNAL_ERROR: u32 = 3222163969u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_LOG_LEVEL: u32 = 3222163862u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_MAJOR_VERSION: u32 = 3222163990u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_PARAMETER: u32 = 3222163860u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_PATCHMETADATA_PROP: u32 = 3222163856u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_PATCH_TYPE_SEQUENCING: u32 = 3222163977u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_PCP_EXTERNALFILES: u32 = 3222163982u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_PCP_FAMILYFILERANGES: u32 = 3222163992u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_PCP_IMAGEFAMILIES: u32 = 3222163983u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_PCP_PATCHSEQUENCE: u32 = 3222163984u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_PCP_PROPERTIES: u32 = 3222163991u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_PCP_PROPERTY: u32 = 3222163970u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_PCP_TARGETFILES_OPTIONALDATA: u32 = 3222163985u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_PCP_TARGETIMAGES: u32 = 3222163971u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_PCP_UPGRADEDFILESTOIGNORE: u32 = 3222163980u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_PCP_UPGRADEDFILES_OPTIONALDATA: u32 = 3222163986u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_PCP_UPGRADEDIMAGES: u32 = 3222163981u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_RANGE_ELEMENT: u32 = 3222163989u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_SUPERCEDENCE: u32 = 3222163857u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_SUPERSEDENCE_VALUE: u32 = 3222163976u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_INVALID_UI_LEVEL: u32 = 3222163863u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_LAX_VALIDATION_FLAGS: u32 = 3222163972u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_MAJOR_UPGD_WITHOUT_SEQUENCING: u32 = 3222163843u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_MATCHED_PRODUCT_VERSIONS: u32 = 3222163837u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_MISMATCHED_PRODUCT_CODES: u32 = 3222163779u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_MISMATCHED_PRODUCT_VERSIONS: u32 = 3222163780u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_MISSING_DIRECTORY_TABLE: u32 = 3222163975u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_MISSING_PATCHMETADATA: u32 = 3222163987u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_MISSING_PATCH_GUID: u32 = 3222163719u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_MISSING_PATCH_PATH: u32 = 3222163716u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_NO_UPGRADED_IMAGES_TO_PATCH: u32 = 3222163723u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_NULL_PATCHFAMILY: u32 = 3222163850u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_NULL_SEQUENCE_NUMBER: u32 = 3222163851u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_OBSOLETION_WITH_MSI30: u32 = 3222163839u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_OBSOLETION_WITH_PATCHSEQUENCE: u32 = 3222163840u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_OBSOLETION_WITH_SEQUENCE_DATA: u32 = 3222163838u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_OODS_COPYING_MSI: u32 = 3222163726u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_OPEN_VIEW: u32 = 3222163869u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_OUT_OF_MEMORY: u32 = 3222163865u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_PATCHMETADATA_PROP_NOT_SET: u32 = 3222163855u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_PCP_BAD_FORMAT: u32 = 3222163714u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_PCP_DOESNT_EXIST: u32 = 3222163713u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_SEQUENCING_BAD_TARGET: u32 = 3222163854u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TARGET_BAD_PROD_CODE_VAL: u32 = 3222163744u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TARGET_BAD_PROD_VALIDATE: u32 = 3222163743u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TARGET_IMAGE_COMPRESSED: u32 = 3222163742u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TARGET_IMAGE_NAME_TOO_LONG: u32 = 3222163735u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TARGET_IMAGE_PATH_EMPTY: u32 = 3222163739u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TARGET_IMAGE_PATH_NOT_EXIST: u32 = 3222163740u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TARGET_IMAGE_PATH_NOT_MSI: u32 = 3222163741u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TARGET_IMAGE_PATH_TOO_LONG: u32 = 3222163738u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TARGET_MISSING_SRC_FILES: u32 = 3222163746u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TARGET_WRONG_PRODUCT_VERSION_COMP: u32 = 3222163979u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TFILEDATA_BAD_IGNORE_LENGTHS: u32 = 3222163822u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TFILEDATA_BAD_IGNORE_OFFSETS: u32 = 3222163820u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TFILEDATA_BAD_RETAIN_OFFSETS: u32 = 3222163825u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TFILEDATA_BAD_TARGET_FIELD: u32 = 3222163791u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TFILEDATA_BLANK_FILE_TABLE_KEY: u32 = 3222163789u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TFILEDATA_IGNORE_COUNT_MISMATCH: u32 = 3222163823u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TFILEDATA_LONG_FILE_TABLE_KEY: u32 = 3222163788u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TFILEDATA_LONG_IGNORE_LENGTHS: u32 = 3222163821u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TFILEDATA_LONG_IGNORE_OFFSETS: u32 = 3222163819u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TFILEDATA_LONG_RETAIN_OFFSETS: u32 = 3222163824u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_TFILEDATA_MISSING_FILE_TABLE_KEY: u32 = 3222163790u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UFILEDATA_BAD_UPGRADED_FIELD: u32 = 3222163778u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UFILEDATA_BLANK_FILE_TABLE_KEY: u32 = 3222163752u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UFILEDATA_LONG_FILE_TABLE_KEY: u32 = 3222163751u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UFILEDATA_MISSING_FILE_TABLE_KEY: u32 = 3222163753u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UFILEIGNORE_BAD_FILE_TABLE_KEY: u32 = 3222163799u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UFILEIGNORE_BAD_UPGRADED_FIELD: u32 = 3222163796u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UFILEIGNORE_BLANK_FILE_TABLE_KEY: u32 = 3222163798u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UFILEIGNORE_LONG_FILE_TABLE_KEY: u32 = 3222163797u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UNKNOWN_ERROR: u32 = 3222163866u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UNKNOWN_INFO: u32 = 3222163867u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UNKNOWN_WARN: u32 = 3222163868u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UPGRADED_IMAGE_COMPRESSED: u32 = 3222163734u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UPGRADED_IMAGE_NAME_TOO_LONG: u32 = 3222163727u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_NOT_EXIST: u32 = 3222163793u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_NOT_MSI: u32 = 3222163794u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_TOO_LONG: u32 = 3222163792u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_EMPTY: u32 = 3222163731u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_NOT_EXIST: u32 = 3222163732u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_NOT_MSI: u32 = 3222163733u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_TOO_LONG: u32 = 3222163730u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_UPGRADED_MISSING_SRC_FILES: u32 = 3222163745u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_VIEW_FETCH: u32 = 3222163871u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_WRITE_SUMMARY_PROPERTIES: u32 = 3222163787u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_PCW_WRONG_PATCHMETADATA_STRD_PROP: u32 = 3222163859u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ERROR_ROLLBACK_DISABLED: u32 = 1653u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FUSION_INSTALL_REFERENCE {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub guidScheme: ::windows_sys::core::GUID,
    pub szIdentifier: super::super::Foundation::PWSTR,
    pub szNonCannonicalData: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FUSION_INSTALL_REFERENCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FUSION_INSTALL_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FUSION_REFCOUNT_FILEPATH_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2955910501, data2: 64375, data3: 20346, data4: [175, 165, 179, 145, 48, 159, 17, 201] };
pub const FUSION_REFCOUNT_OPAQUE_STRING_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 784938083, data2: 45251, data3: 17889, data4: [131, 100, 50, 126, 150, 174, 168, 86] };
pub const FUSION_REFCOUNT_UNINSTALL_SUBKEY_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2364391957, data2: 44107, data3: 18571, data4: [147, 192, 165, 10, 73, 203, 47, 184] };
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_ALREADY_INSTALLED: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_INSTALLED: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_REFRESHED: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const IASSEMBLYCACHEITEM_COMMIT_FLAG_REFRESH: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type IASSEMBLYCACHE_UNINSTALL_DISPOSITION = u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_UNINSTALLED: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_STILL_IN_USE: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_ALREADY_UNINSTALLED: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = 3u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_DELETE_PENDING: IASSEMBLYCACHE_UNINSTALL_DISPOSITION = 4u32;
pub type IAssemblyCache = *mut ::core::ffi::c_void;
pub type IAssemblyCacheItem = *mut ::core::ffi::c_void;
pub type IAssemblyName = *mut ::core::ffi::c_void;
pub type IEnumMsmDependency = *mut ::core::ffi::c_void;
pub type IEnumMsmError = *mut ::core::ffi::c_void;
pub type IEnumMsmString = *mut ::core::ffi::c_void;
pub type IMsmDependencies = *mut ::core::ffi::c_void;
pub type IMsmDependency = *mut ::core::ffi::c_void;
pub type IMsmError = *mut ::core::ffi::c_void;
pub type IMsmErrors = *mut ::core::ffi::c_void;
pub type IMsmGetFiles = *mut ::core::ffi::c_void;
pub type IMsmMerge = *mut ::core::ffi::c_void;
pub type IMsmStrings = *mut ::core::ffi::c_void;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_BASE: u32 = 3222229249u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_ENTERING_PHASE_I: u32 = 3222229251u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_ENTERING_PHASE_II: u32 = 3222229256u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_ENTERING_PHASE_III: u32 = 3222229257u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_ENTERING_PHASE_IV: u32 = 3222229258u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_ENTERING_PHASE_I_VALIDATION: u32 = 3222229250u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_ENTERING_PHASE_V: u32 = 3222229259u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_GENERATING_METADATA: u32 = 3222229265u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_PASSED_MAIN_CONTROL: u32 = 3222229249u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_PATCHCACHE_FILEINFO_FAILURE: u32 = 3222229267u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_PATCHCACHE_PCI_READFAILURE: u32 = 3222229268u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_PATCHCACHE_PCI_WRITEFAILURE: u32 = 3222229269u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_PCP_PATH: u32 = 3222229252u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_PROPERTY: u32 = 3222229255u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_SET_OPTIONS: u32 = 3222229254u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_SUCCESSFUL_PATCH_CREATION: u32 = 3222229271u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_TEMP_DIR: u32 = 3222229253u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_TEMP_DIR_CLEANUP: u32 = 3222229266u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INFO_USING_USER_MSI_FOR_PATCH_TABLES: u32 = 3222229270u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type INSTALLFEATUREATTRIBUTE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLFEATUREATTRIBUTE_FAVORLOCAL: INSTALLFEATUREATTRIBUTE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLFEATUREATTRIBUTE_FAVORSOURCE: INSTALLFEATUREATTRIBUTE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLFEATUREATTRIBUTE_FOLLOWPARENT: INSTALLFEATUREATTRIBUTE = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLFEATUREATTRIBUTE_FAVORADVERTISE: INSTALLFEATUREATTRIBUTE = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLFEATUREATTRIBUTE_DISALLOWADVERTISE: INSTALLFEATUREATTRIBUTE = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLFEATUREATTRIBUTE_NOUNSUPPORTEDADVERTISE: INSTALLFEATUREATTRIBUTE = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type INSTALLLEVEL = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLEVEL_DEFAULT: INSTALLLEVEL = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLEVEL_MINIMUM: INSTALLLEVEL = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLEVEL_MAXIMUM: INSTALLLEVEL = 65535i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type INSTALLLOGATTRIBUTES = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGATTRIBUTES_APPEND: INSTALLLOGATTRIBUTES = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGATTRIBUTES_FLUSHEACHLINE: INSTALLLOGATTRIBUTES = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type INSTALLMESSAGE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_FATALEXIT: INSTALLMESSAGE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_ERROR: INSTALLMESSAGE = 16777216i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_WARNING: INSTALLMESSAGE = 33554432i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_USER: INSTALLMESSAGE = 50331648i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_INFO: INSTALLMESSAGE = 67108864i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_FILESINUSE: INSTALLMESSAGE = 83886080i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_RESOLVESOURCE: INSTALLMESSAGE = 100663296i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_OUTOFDISKSPACE: INSTALLMESSAGE = 117440512i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_ACTIONSTART: INSTALLMESSAGE = 134217728i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_ACTIONDATA: INSTALLMESSAGE = 150994944i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_PROGRESS: INSTALLMESSAGE = 167772160i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_COMMONDATA: INSTALLMESSAGE = 184549376i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_INITIALIZE: INSTALLMESSAGE = 201326592i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_TERMINATE: INSTALLMESSAGE = 218103808i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_SHOWDIALOG: INSTALLMESSAGE = 234881024i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_PERFORMANCE: INSTALLMESSAGE = 251658240i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_RMFILESINUSE: INSTALLMESSAGE = 419430400i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_INSTALLSTART: INSTALLMESSAGE = 436207616i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_INSTALLEND: INSTALLMESSAGE = 452984832i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMESSAGE_TYPEMASK: i32 = -16777216i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type INSTALLMODE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMODE_NODETECTION_ANY: INSTALLMODE = -4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMODE_NOSOURCERESOLUTION: INSTALLMODE = -3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMODE_NODETECTION: INSTALLMODE = -2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMODE_EXISTING: INSTALLMODE = -1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLMODE_DEFAULT: INSTALLMODE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type INSTALLOGMODE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_FATALEXIT: INSTALLOGMODE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_ERROR: INSTALLOGMODE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_WARNING: INSTALLOGMODE = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_USER: INSTALLOGMODE = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_INFO: INSTALLOGMODE = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_RESOLVESOURCE: INSTALLOGMODE = 64i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_OUTOFDISKSPACE: INSTALLOGMODE = 128i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_ACTIONSTART: INSTALLOGMODE = 256i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_ACTIONDATA: INSTALLOGMODE = 512i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_COMMONDATA: INSTALLOGMODE = 2048i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_PROPERTYDUMP: INSTALLOGMODE = 1024i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_VERBOSE: INSTALLOGMODE = 4096i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_EXTRADEBUG: INSTALLOGMODE = 8192i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_LOGONLYONERROR: INSTALLOGMODE = 16384i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_LOGPERFORMANCE: INSTALLOGMODE = 32768i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_PROGRESS: INSTALLOGMODE = 1024i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_INITIALIZE: INSTALLOGMODE = 4096i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_TERMINATE: INSTALLOGMODE = 8192i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_SHOWDIALOG: INSTALLOGMODE = 16384i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_FILESINUSE: INSTALLOGMODE = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_RMFILESINUSE: INSTALLOGMODE = 33554432i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_INSTALLSTART: INSTALLOGMODE = 67108864i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLLOGMODE_INSTALLEND: INSTALLOGMODE = 134217728i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type INSTALLSTATE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLSTATE_NOTUSED: INSTALLSTATE = -7i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLSTATE_BADCONFIG: INSTALLSTATE = -6i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLSTATE_INCOMPLETE: INSTALLSTATE = -5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLSTATE_SOURCEABSENT: INSTALLSTATE = -4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLSTATE_MOREDATA: INSTALLSTATE = -3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLSTATE_INVALIDARG: INSTALLSTATE = -2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLSTATE_UNKNOWN: INSTALLSTATE = -1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLSTATE_BROKEN: INSTALLSTATE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLSTATE_ADVERTISED: INSTALLSTATE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLSTATE_REMOVED: INSTALLSTATE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLSTATE_ABSENT: INSTALLSTATE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLSTATE_LOCAL: INSTALLSTATE = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLSTATE_SOURCE: INSTALLSTATE = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLSTATE_DEFAULT: INSTALLSTATE = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type INSTALLTYPE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLTYPE_DEFAULT: INSTALLTYPE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLTYPE_NETWORK_IMAGE: INSTALLTYPE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLTYPE_SINGLE_INSTANCE: INSTALLTYPE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type INSTALLUILEVEL = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLUILEVEL_NOCHANGE: INSTALLUILEVEL = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLUILEVEL_DEFAULT: INSTALLUILEVEL = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLUILEVEL_NONE: INSTALLUILEVEL = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLUILEVEL_BASIC: INSTALLUILEVEL = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLUILEVEL_REDUCED: INSTALLUILEVEL = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLUILEVEL_FULL: INSTALLUILEVEL = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLUILEVEL_ENDDIALOG: INSTALLUILEVEL = 128i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLUILEVEL_PROGRESSONLY: INSTALLUILEVEL = 64i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLUILEVEL_HIDECANCEL: INSTALLUILEVEL = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLUILEVEL_SOURCERESONLY: INSTALLUILEVEL = 256i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const INSTALLUILEVEL_UACONLY: INSTALLUILEVEL = 512i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type INSTALLUI_HANDLERA = ::core::option::Option<unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, szmessage: super::super::Foundation::PSTR) -> i32>;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type INSTALLUI_HANDLERW = ::core::option::Option<unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, szmessage: super::super::Foundation::PWSTR) -> i32>;
pub type IPMApplicationInfo = *mut ::core::ffi::c_void;
pub type IPMApplicationInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMBackgroundServiceAgentInfo = *mut ::core::ffi::c_void;
pub type IPMBackgroundServiceAgentInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMBackgroundWorkerInfo = *mut ::core::ffi::c_void;
pub type IPMBackgroundWorkerInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMDeploymentManager = *mut ::core::ffi::c_void;
pub type IPMEnumerationManager = *mut ::core::ffi::c_void;
pub type IPMExtensionCachedFileUpdaterInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionContractInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionFileExtensionInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionFileOpenPickerInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionFileSavePickerInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMExtensionProtocolInfo = *mut ::core::ffi::c_void;
pub type IPMExtensionShareTargetInfo = *mut ::core::ffi::c_void;
pub type IPMLiveTileJobInfo = *mut ::core::ffi::c_void;
pub type IPMLiveTileJobInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMTaskInfo = *mut ::core::ffi::c_void;
pub type IPMTaskInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMTileInfo = *mut ::core::ffi::c_void;
pub type IPMTileInfoEnumerator = *mut ::core::ffi::c_void;
pub type IPMTilePropertyEnumerator = *mut ::core::ffi::c_void;
pub type IPMTilePropertyInfo = *mut ::core::ffi::c_void;
pub type IValidate = *mut ::core::ffi::c_void;
pub const LIBID_MsmMergeTypeLib: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 182298671, data2: 11302, data3: 4562, data4: [173, 101, 0, 160, 201, 175, 17, 166] };
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const LOGALL: u32 = 15u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const LOGERR: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const LOGINFO: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const LOGNONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const LOGPERFMESSAGES: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const LOGTOKEN_NO_LOG: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const LOGTOKEN_SETUPAPI_APPLOG: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const LOGTOKEN_SETUPAPI_DEVLOG: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const LOGTOKEN_TYPE_MASK: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const LOGTOKEN_UNSPECIFIED: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const LOGWARN: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDISPLAYVAL = ::core::option::Option<unsafe extern "system" fn(pcontext: *mut ::core::ffi::c_void, uitype: RESULTTYPES, szwval: super::super::Foundation::PWSTR, szwdescription: super::super::Foundation::PWSTR, szwlocation: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPEVALCOMCALLBACK = ::core::option::Option<unsafe extern "system" fn(istatus: STATUSTYPES, szdata: super::super::Foundation::PWSTR, pcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MAX_FEATURE_CHARS: u32 = 38u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MAX_GUID_CHARS: u32 = 38u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSIADVERTISEOPTIONFLAGS = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIADVERTISEOPTIONFLAGS_INSTANCE: MSIADVERTISEOPTIONFLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSIARCHITECTUREFLAGS = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIARCHITECTUREFLAGS_X86: MSIARCHITECTUREFLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIARCHITECTUREFLAGS_IA64: MSIARCHITECTUREFLAGS = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIARCHITECTUREFLAGS_AMD64: MSIARCHITECTUREFLAGS = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIARCHITECTUREFLAGS_ARM: MSIARCHITECTUREFLAGS = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSIASSEMBLYINFO = u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIASSEMBLYINFO_NETASSEMBLY: MSIASSEMBLYINFO = 0u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIASSEMBLYINFO_WIN32ASSEMBLY: MSIASSEMBLYINFO = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSICODE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSICODE_PRODUCT: MSICODE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSICODE_PATCH: MSICODE = 1073741824i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSICOLINFO = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSICOLINFO_NAMES: MSICOLINFO = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSICOLINFO_TYPES: MSICOLINFO = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSICONDITION = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSICONDITION_FALSE: MSICONDITION = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSICONDITION_TRUE: MSICONDITION = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSICONDITION_NONE: MSICONDITION = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSICONDITION_ERROR: MSICONDITION = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSICOSTTREE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSICOSTTREE_SELFONLY: MSICOSTTREE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSICOSTTREE_CHILDREN: MSICOSTTREE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSICOSTTREE_PARENTS: MSICOSTTREE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSICOSTTREE_RESERVED: MSICOSTTREE = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSIDBERROR = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_INVALIDARG: MSIDBERROR = -3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_MOREDATA: MSIDBERROR = -2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_FUNCTIONERROR: MSIDBERROR = -1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_NOERROR: MSIDBERROR = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_DUPLICATEKEY: MSIDBERROR = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_REQUIRED: MSIDBERROR = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADLINK: MSIDBERROR = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_OVERFLOW: MSIDBERROR = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_UNDERFLOW: MSIDBERROR = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_NOTINSET: MSIDBERROR = 6i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADVERSION: MSIDBERROR = 7i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADCASE: MSIDBERROR = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADGUID: MSIDBERROR = 9i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADWILDCARD: MSIDBERROR = 10i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADIDENTIFIER: MSIDBERROR = 11i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADLANGUAGE: MSIDBERROR = 12i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADFILENAME: MSIDBERROR = 13i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADPATH: MSIDBERROR = 14i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADCONDITION: MSIDBERROR = 15i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADFORMATTED: MSIDBERROR = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADTEMPLATE: MSIDBERROR = 17i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADDEFAULTDIR: MSIDBERROR = 18i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADREGPATH: MSIDBERROR = 19i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADCUSTOMSOURCE: MSIDBERROR = 20i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADPROPERTY: MSIDBERROR = 21i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_MISSINGDATA: MSIDBERROR = 22i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADCATEGORY: MSIDBERROR = 23i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADKEYTABLE: MSIDBERROR = 24i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADMAXMINVALUES: MSIDBERROR = 25i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADCABINET: MSIDBERROR = 26i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADSHORTCUT: MSIDBERROR = 27i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_STRINGOVERFLOW: MSIDBERROR = 28i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBERROR_BADLOCALIZEATTRIB: MSIDBERROR = 29i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSIDBSTATE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBSTATE_ERROR: MSIDBSTATE = -1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBSTATE_READ: MSIDBSTATE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIDBSTATE_WRITE: MSIDBSTATE = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub struct MSIFILEHASHINFO {
    pub dwFileHashInfoSize: u32,
    pub dwData: [u32; 4],
}
impl ::core::marker::Copy for MSIFILEHASHINFO {}
impl ::core::clone::Clone for MSIFILEHASHINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type MSIHANDLE = u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSIINSTALLCONTEXT = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIINSTALLCONTEXT_FIRSTVISIBLE: MSIINSTALLCONTEXT = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIINSTALLCONTEXT_NONE: MSIINSTALLCONTEXT = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIINSTALLCONTEXT_USERMANAGED: MSIINSTALLCONTEXT = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIINSTALLCONTEXT_USERUNMANAGED: MSIINSTALLCONTEXT = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIINSTALLCONTEXT_MACHINE: MSIINSTALLCONTEXT = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIINSTALLCONTEXT_ALL: MSIINSTALLCONTEXT = 7i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIINSTALLCONTEXT_ALLUSERMANAGED: MSIINSTALLCONTEXT = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSIMODIFY = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIMODIFY_SEEK: MSIMODIFY = -1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIMODIFY_REFRESH: MSIMODIFY = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIMODIFY_INSERT: MSIMODIFY = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIMODIFY_UPDATE: MSIMODIFY = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIMODIFY_ASSIGN: MSIMODIFY = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIMODIFY_REPLACE: MSIMODIFY = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIMODIFY_MERGE: MSIMODIFY = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIMODIFY_DELETE: MSIMODIFY = 6i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIMODIFY_INSERT_TEMPORARY: MSIMODIFY = 7i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIMODIFY_VALIDATE: MSIMODIFY = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIMODIFY_VALIDATE_NEW: MSIMODIFY = 9i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIMODIFY_VALIDATE_FIELD: MSIMODIFY = 10i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIMODIFY_VALIDATE_DELETE: MSIMODIFY = 11i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSIOPENPACKAGEFLAGS = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIOPENPACKAGEFLAGS_IGNOREMACHINESTATE: MSIOPENPACKAGEFLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSIPATCHDATATYPE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIPATCH_DATATYPE_PATCHFILE: MSIPATCHDATATYPE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIPATCH_DATATYPE_XMLPATH: MSIPATCHDATATYPE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIPATCH_DATATYPE_XMLBLOB: MSIPATCHDATATYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSIPATCHSEQUENCEINFOA {
    pub szPatchData: super::super::Foundation::PSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSIPATCHSEQUENCEINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSIPATCHSEQUENCEINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MSIPATCHSEQUENCEINFOW {
    pub szPatchData: super::super::Foundation::PWSTR,
    pub ePatchDataType: MSIPATCHDATATYPE,
    pub dwOrder: u32,
    pub uStatus: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MSIPATCHSEQUENCEINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MSIPATCHSEQUENCEINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSIPATCHSTATE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIPATCHSTATE_INVALID: MSIPATCHSTATE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIPATCHSTATE_APPLIED: MSIPATCHSTATE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIPATCHSTATE_SUPERSEDED: MSIPATCHSTATE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIPATCHSTATE_OBSOLETED: MSIPATCHSTATE = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIPATCHSTATE_REGISTERED: MSIPATCHSTATE = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIPATCHSTATE_ALL: MSIPATCHSTATE = 15i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSIRUNMODE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_ADMIN: MSIRUNMODE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_ADVERTISE: MSIRUNMODE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_MAINTENANCE: MSIRUNMODE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_ROLLBACKENABLED: MSIRUNMODE = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_LOGENABLED: MSIRUNMODE = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_OPERATIONS: MSIRUNMODE = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_REBOOTATEND: MSIRUNMODE = 6i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_REBOOTNOW: MSIRUNMODE = 7i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_CABINET: MSIRUNMODE = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_SOURCESHORTNAMES: MSIRUNMODE = 9i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_TARGETSHORTNAMES: MSIRUNMODE = 10i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_RESERVED11: MSIRUNMODE = 11i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_WINDOWS9X: MSIRUNMODE = 12i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_ZAWENABLED: MSIRUNMODE = 13i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_RESERVED14: MSIRUNMODE = 14i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_RESERVED15: MSIRUNMODE = 15i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_SCHEDULED: MSIRUNMODE = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_ROLLBACK: MSIRUNMODE = 17i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSIRUNMODE_COMMIT: MSIRUNMODE = 18i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSISOURCETYPE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSISOURCETYPE_UNKNOWN: MSISOURCETYPE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSISOURCETYPE_NETWORK: MSISOURCETYPE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSISOURCETYPE_URL: MSISOURCETYPE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSISOURCETYPE_MEDIA: MSISOURCETYPE = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSITRANSACTION = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSACTION_CHAIN_EMBEDDEDUI: MSITRANSACTION = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSACTION_JOIN_EXISTING_EMBEDDEDUI: MSITRANSACTION = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSITRANSACTIONSTATE = u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSACTIONSTATE_ROLLBACK: MSITRANSACTIONSTATE = 0u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSACTIONSTATE_COMMIT: MSITRANSACTIONSTATE = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSITRANSFORM_ERROR = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_ERROR_ADDEXISTINGROW: MSITRANSFORM_ERROR = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_ERROR_DELMISSINGROW: MSITRANSFORM_ERROR = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_ERROR_ADDEXISTINGTABLE: MSITRANSFORM_ERROR = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_ERROR_DELMISSINGTABLE: MSITRANSFORM_ERROR = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_ERROR_UPDATEMISSINGROW: MSITRANSFORM_ERROR = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_ERROR_CHANGECODEPAGE: MSITRANSFORM_ERROR = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_ERROR_VIEWTRANSFORM: MSITRANSFORM_ERROR = 256i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_ERROR_NONE: MSITRANSFORM_ERROR = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type MSITRANSFORM_VALIDATE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_VALIDATE_LANGUAGE: MSITRANSFORM_VALIDATE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_VALIDATE_PRODUCT: MSITRANSFORM_VALIDATE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_VALIDATE_PLATFORM: MSITRANSFORM_VALIDATE = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_VALIDATE_MAJORVERSION: MSITRANSFORM_VALIDATE = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_VALIDATE_MINORVERSION: MSITRANSFORM_VALIDATE = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_VALIDATE_UPDATEVERSION: MSITRANSFORM_VALIDATE = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_VALIDATE_NEWLESSBASEVERSION: MSITRANSFORM_VALIDATE = 64i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_VALIDATE_NEWLESSEQUALBASEVERSION: MSITRANSFORM_VALIDATE = 128i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_VALIDATE_NEWEQUALBASEVERSION: MSITRANSFORM_VALIDATE = 256i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_VALIDATE_NEWGREATEREQUALBASEVERSION: MSITRANSFORM_VALIDATE = 512i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_VALIDATE_NEWGREATERBASEVERSION: MSITRANSFORM_VALIDATE = 1024i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSITRANSFORM_VALIDATE_UPGRADECODE: MSITRANSFORM_VALIDATE = 2048i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSI_INVALID_HASH_IS_FATAL: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const MSI_NULL_INTEGER: u32 = 2147483648u32;
pub const MsmMerge: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 182298672, data2: 11302, data3: 4562, data4: [173, 101, 0, 160, 201, 175, 17, 166] };
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PACKMAN_RUNTIME = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PACKMAN_RUNTIME_NATIVE: PACKMAN_RUNTIME = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PACKMAN_RUNTIME_SILVERLIGHTMOBILE: PACKMAN_RUNTIME = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PACKMAN_RUNTIME_XNA: PACKMAN_RUNTIME = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PACKMAN_RUNTIME_MODERN_NATIVE: PACKMAN_RUNTIME = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PACKMAN_RUNTIME_JUPITER: PACKMAN_RUNTIME = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PACKMAN_RUNTIME_INVALID: PACKMAN_RUNTIME = 6i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub struct PATCH_IGNORE_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
}
impl ::core::marker::Copy for PATCH_IGNORE_RANGE {}
impl ::core::clone::Clone for PATCH_IGNORE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub struct PATCH_INTERLEAVE_MAP {
    pub CountRanges: u32,
    pub Range: [PATCH_INTERLEAVE_MAP_0; 1],
}
impl ::core::marker::Copy for PATCH_INTERLEAVE_MAP {}
impl ::core::clone::Clone for PATCH_INTERLEAVE_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub struct PATCH_INTERLEAVE_MAP_0 {
    pub OldOffset: u32,
    pub OldLength: u32,
    pub NewLength: u32,
}
impl ::core::marker::Copy for PATCH_INTERLEAVE_MAP_0 {}
impl ::core::clone::Clone for PATCH_INTERLEAVE_MAP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OLD_FILE_INFO {
    pub SizeOfThisStruct: u32,
    pub Anonymous: PATCH_OLD_FILE_INFO_0,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union PATCH_OLD_FILE_INFO_0 {
    pub OldFileNameA: super::super::Foundation::PSTR,
    pub OldFileNameW: super::super::Foundation::PWSTR,
    pub OldFileHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OLD_FILE_INFO_A {
    pub SizeOfThisStruct: u32,
    pub OldFileName: super::super::Foundation::PSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OLD_FILE_INFO_H {
    pub SizeOfThisStruct: u32,
    pub OldFileHandle: super::super::Foundation::HANDLE,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_H {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_H {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OLD_FILE_INFO_W {
    pub SizeOfThisStruct: u32,
    pub OldFileName: super::super::Foundation::PWSTR,
    pub IgnoreRangeCount: u32,
    pub IgnoreRangeArray: *mut PATCH_IGNORE_RANGE,
    pub RetainRangeCount: u32,
    pub RetainRangeArray: *mut PATCH_RETAIN_RANGE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OLD_FILE_INFO_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OLD_FILE_INFO_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PATCH_OPTION_DATA {
    pub SizeOfThisStruct: u32,
    pub SymbolOptionFlags: u32,
    pub NewFileSymbolPath: super::super::Foundation::PSTR,
    pub OldFileSymbolPathArray: *mut super::super::Foundation::PSTR,
    pub ExtendedOptionFlags: u32,
    pub SymLoadCallback: PPATCH_SYMLOAD_CALLBACK,
    pub SymLoadContext: *mut ::core::ffi::c_void,
    pub InterleaveMapArray: *mut *mut PATCH_INTERLEAVE_MAP,
    pub MaxLzxWindowSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PATCH_OPTION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PATCH_OPTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_FAIL_IF_BIGGER: u32 = 1048576u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_FAIL_IF_SAME_FILE: u32 = 524288u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_INTERLEAVE_FILES: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_NO_BINDFIX: u32 = 65536u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_NO_CHECKSUM: u32 = 2097152u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_NO_LOCKFIX: u32 = 131072u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_NO_REBASE: u32 = 262144u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_NO_RESTIMEFIX: u32 = 4194304u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_NO_TIMESTAMP: u32 = 8388608u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_RESERVED1: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_SIGNATURE_MD5: u32 = 16777216u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_USE_BEST: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_USE_LZX_A: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_USE_LZX_B: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_USE_LZX_BEST: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_USE_LZX_LARGE: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_OPTION_VALID_FLAGS: u32 = 3237937159u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub struct PATCH_RETAIN_RANGE {
    pub OffsetInOldFile: u32,
    pub LengthInBytes: u32,
    pub OffsetInNewFile: u32,
}
impl ::core::marker::Copy for PATCH_RETAIN_RANGE {}
impl ::core::clone::Clone for PATCH_RETAIN_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_SYMBOL_NO_FAILURES: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_SYMBOL_NO_IMAGEHLP: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_SYMBOL_RESERVED1: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_SYMBOL_UNDECORATED_TOO: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_TRANSFORM_PE_IRELOC_2: u32 = 512u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PATCH_TRANSFORM_PE_RESOURCE_2: u32 = 256u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_APPNAME: u32 = 18u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_AUTHOR: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_CHARCOUNT: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_COMMENTS: u32 = 6u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_CREATE_DTM: u32 = 12u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_EDITTIME: u32 = 10u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_KEYWORDS: u32 = 5u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_LASTAUTHOR: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_LASTPRINTED: u32 = 11u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_LASTSAVE_DTM: u32 = 13u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_MSIRESTRICT: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_MSISOURCE: u32 = 15u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_MSIVERSION: u32 = 14u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_PAGECOUNT: u32 = 14u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_REVNUMBER: u32 = 9u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_SUBJECT: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_TEMPLATE: u32 = 7u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_THUMBNAIL: u32 = 17u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_TITLE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PID_WORDCOUNT: u32 = 15u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PINSTALLUI_HANDLER_RECORD = ::core::option::Option<unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, hrecord: MSIHANDLE) -> i32>;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub struct PMSIHANDLE {
    pub m_h: MSIHANDLE,
}
impl ::core::marker::Copy for PMSIHANDLE {}
impl ::core::clone::Clone for PMSIHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PMSvc: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3118797308, data2: 58212, data3: 18810, data4: [161, 33, 183, 179, 97, 44, 237, 206] };
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_ACTIVATION_POLICY = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ACTIVATION_POLICY_RESUME: PM_ACTIVATION_POLICY = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ACTIVATION_POLICY_RESUMESAMEPARAMS: PM_ACTIVATION_POLICY = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ACTIVATION_POLICY_REPLACE: PM_ACTIVATION_POLICY = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ACTIVATION_POLICY_REPLACESAMEPARAMS: PM_ACTIVATION_POLICY = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ACTIVATION_POLICY_MULTISESSION: PM_ACTIVATION_POLICY = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ACTIVATION_POLICY_REPLACE_IGNOREFOREGROUND: PM_ACTIVATION_POLICY = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ACTIVATION_POLICY_UNKNOWN: PM_ACTIVATION_POLICY = 6i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ACTIVATION_POLICY_INVALID: PM_ACTIVATION_POLICY = 7i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_APPLICATION_HUBTYPE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_HUBTYPE_NONMUSIC: PM_APPLICATION_HUBTYPE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_HUBTYPE_MUSIC: PM_APPLICATION_HUBTYPE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_HUBTYPE_INVALID: PM_APPLICATION_HUBTYPE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_APPLICATION_INSTALL_TYPE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_INSTALL_NORMAL: PM_APPLICATION_INSTALL_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_INSTALL_IN_ROM: PM_APPLICATION_INSTALL_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_INSTALL_PA: PM_APPLICATION_INSTALL_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_INSTALL_DEBUG: PM_APPLICATION_INSTALL_TYPE = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_INSTALL_ENTERPRISE: PM_APPLICATION_INSTALL_TYPE = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_INSTALL_INVALID: PM_APPLICATION_INSTALL_TYPE = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_APPLICATION_STATE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_STATE_MIN: PM_APPLICATION_STATE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_STATE_INSTALLED: PM_APPLICATION_STATE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_STATE_INSTALLING: PM_APPLICATION_STATE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_STATE_UPDATING: PM_APPLICATION_STATE = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_STATE_UNINSTALLING: PM_APPLICATION_STATE = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_STATE_LICENSE_UPDATING: PM_APPLICATION_STATE = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_STATE_MOVING: PM_APPLICATION_STATE = 6i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_STATE_DISABLED_SD_CARD: PM_APPLICATION_STATE = 7i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_STATE_DISABLED_ENTERPRISE: PM_APPLICATION_STATE = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_STATE_DISABLED_BACKING_UP: PM_APPLICATION_STATE = 9i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_STATE_DISABLED_MDIL_BINDING: PM_APPLICATION_STATE = 10i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_STATE_MAX: PM_APPLICATION_STATE = 10i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APPLICATION_STATE_INVALID: PM_APPLICATION_STATE = 11i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_APP_GENRE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APP_GENRE_GAMES: PM_APP_GENRE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APP_GENRE_OTHER: PM_APP_GENRE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APP_GENRE_INVALID: PM_APP_GENRE = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_BSATASKID {
    pub ProductID: ::windows_sys::core::GUID,
    pub TaskID: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PM_BSATASKID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_BSATASKID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_BWTASKID {
    pub ProductID: ::windows_sys::core::GUID,
    pub TaskID: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PM_BWTASKID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_BWTASKID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_ENUM_APP_FILTER = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APP_FILTER_ALL: PM_ENUM_APP_FILTER = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APP_FILTER_VISIBLE: PM_ENUM_APP_FILTER = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APP_FILTER_GENRE: PM_ENUM_APP_FILTER = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APP_FILTER_NONGAMES: PM_ENUM_APP_FILTER = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APP_FILTER_HUBTYPE: PM_ENUM_APP_FILTER = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APP_FILTER_PINABLEONKIDZONE: PM_ENUM_APP_FILTER = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APP_FILTER_ALL_INCLUDE_MODERN: PM_ENUM_APP_FILTER = 6i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APP_FILTER_FRAMEWORK: PM_ENUM_APP_FILTER = 7i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_APP_FILTER_MAX: PM_ENUM_APP_FILTER = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_ENUM_BSA_FILTER = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_BSA_FILTER_ALL: PM_ENUM_BSA_FILTER = 26i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_BSA_FILTER_BY_TASKID: PM_ENUM_BSA_FILTER = 27i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_BSA_FILTER_BY_PRODUCTID: PM_ENUM_BSA_FILTER = 28i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_BSA_FILTER_BY_PERIODIC: PM_ENUM_BSA_FILTER = 29i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_BSA_FILTER_BY_ALL_LAUNCHONBOOT: PM_ENUM_BSA_FILTER = 30i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_BSA_FILTER_MAX: PM_ENUM_BSA_FILTER = 31i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_ENUM_BW_FILTER = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_BW_FILTER_BOOTWORKER_ALL: PM_ENUM_BW_FILTER = 31i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_BW_FILTER_BY_TASKID: PM_ENUM_BW_FILTER = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_BW_FILTER_MAX: PM_ENUM_BW_FILTER = 33i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_ENUM_EXTENSION_FILTER = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_EXTENSION_FILTER_BY_CONSUMER: PM_ENUM_EXTENSION_FILTER = 17i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_EXTENSION_FILTER_APPCONNECT: PM_ENUM_EXTENSION_FILTER = 17i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_EXTENSION_FILTER_PROTOCOL_ALL: PM_ENUM_EXTENSION_FILTER = 18i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_FILETYPE_ALL: PM_ENUM_EXTENSION_FILTER = 19i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_CONTENTTYPE_ALL: PM_ENUM_EXTENSION_FILTER = 20i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_APPLICATION_ALL: PM_ENUM_EXTENSION_FILTER = 21i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_EXTENSION_FILTER_SHARETARGET_ALL: PM_ENUM_EXTENSION_FILTER = 22i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_EXTENSION_FILTER_FILEOPENPICKER_ALL: PM_ENUM_EXTENSION_FILTER = 23i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_EXTENSION_FILTER_FILESAVEPICKER_ALL: PM_ENUM_EXTENSION_FILTER = 24i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_EXTENSION_FILTER_CACHEDFILEUPDATER_ALL: PM_ENUM_EXTENSION_FILTER = 25i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_ENUM_EXTENSION_FILTER_MAX: PM_ENUM_EXTENSION_FILTER = 26i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_ENUM_FILTER {
    pub FilterType: i32,
    pub FilterParameter: PM_ENUM_FILTER_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PM_ENUM_FILTER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_ENUM_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union PM_ENUM_FILTER_0 {
    pub Dummy: i32,
    pub Genre: PM_APP_GENRE,
    pub AppHubType: PM_APPLICATION_HUBTYPE,
    pub HubType: PM_TILE_HUBTYPE,
    pub Tasktype: PM_TASK_TYPE,
    pub TaskProductID: ::windows_sys::core::GUID,
    pub TileProductID: ::windows_sys::core::GUID,
    pub AppTaskType: _tagAPPTASKTYPE,
    pub Consumer: PM_EXTENSIONCONSUMER,
    pub BSATask: PM_BSATASKID,
    pub BSAProductID: ::windows_sys::core::GUID,
    pub BWTask: PM_BWTASKID,
    pub ProtocolName: super::super::Foundation::BSTR,
    pub FileType: super::super::Foundation::BSTR,
    pub ContentType: super::super::Foundation::BSTR,
    pub AppSupportedFileExtPID: ::windows_sys::core::GUID,
    pub ShareTargetFileType: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PM_ENUM_FILTER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_ENUM_FILTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_ENUM_TASK_FILTER = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_FILTER_APP_ALL: PM_ENUM_TASK_FILTER = 12i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_FILTER_TASK_TYPE: PM_ENUM_TASK_FILTER = 13i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_FILTER_DEHYD_SUPRESSING: PM_ENUM_TASK_FILTER = 14i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_FILTER_APP_TASK_TYPE: PM_ENUM_TASK_FILTER = 15i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_FILTER_BGEXECUTION: PM_ENUM_TASK_FILTER = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_FILTER_MAX: PM_ENUM_TASK_FILTER = 17i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_ENUM_TILE_FILTER = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_FILTER_APPLIST: PM_ENUM_TILE_FILTER = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_FILTER_PINNED: PM_ENUM_TILE_FILTER = 9i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_FILTER_HUBTYPE: PM_ENUM_TILE_FILTER = 10i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_FILTER_APP_ALL: PM_ENUM_TILE_FILTER = 11i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_FILTER_MAX: PM_ENUM_TILE_FILTER = 12i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_EXTENSIONCONSUMER {
    pub ConsumerPID: ::windows_sys::core::GUID,
    pub ExtensionID: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PM_EXTENSIONCONSUMER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_EXTENSIONCONSUMER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_INSTALLINFO {
    pub ProductID: ::windows_sys::core::GUID,
    pub PackagePath: super::super::Foundation::BSTR,
    pub InstanceID: ::windows_sys::core::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub IsUninstallDisabled: super::super::Foundation::BOOL,
    pub DeploymentOptions: u32,
    pub OfferID: ::windows_sys::core::GUID,
    pub MarketplaceAppVersion: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PM_INSTALLINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_INSTALLINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_INVOCATIONINFO {
    pub URIBaseOrAUMID: super::super::Foundation::BSTR,
    pub URIFragmentOrArgs: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PM_INVOCATIONINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_INVOCATIONINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_LIVETILE_RECURRENCE_TYPE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_LIVETILE_RECURRENCE_TYPE_INSTANT: PM_LIVETILE_RECURRENCE_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_LIVETILE_RECURRENCE_TYPE_ONETIME: PM_LIVETILE_RECURRENCE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_LIVETILE_RECURRENCE_TYPE_INTERVAL: PM_LIVETILE_RECURRENCE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_LIVETILE_RECURRENCE_TYPE_MAX: PM_LIVETILE_RECURRENCE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_LOGO_SIZE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_LOGO_SIZE_SMALL: PM_LOGO_SIZE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_LOGO_SIZE_MEDIUM: PM_LOGO_SIZE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_LOGO_SIZE_LARGE: PM_LOGO_SIZE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_LOGO_SIZE_INVALID: PM_LOGO_SIZE = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_STARTAPPBLOB {
    pub cbSize: u32,
    pub ProductID: ::windows_sys::core::GUID,
    pub AppTitle: super::super::Foundation::BSTR,
    pub IconPath: super::super::Foundation::BSTR,
    pub IsUninstallable: super::super::Foundation::BOOL,
    pub AppInstallType: PM_APPLICATION_INSTALL_TYPE,
    pub InstanceID: ::windows_sys::core::GUID,
    pub State: PM_APPLICATION_STATE,
    pub IsModern: super::super::Foundation::BOOL,
    pub IsModernLightUp: super::super::Foundation::BOOL,
    pub LightUpSupportMask: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PM_STARTAPPBLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_STARTAPPBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_STARTTILEBLOB {
    pub cbSize: u32,
    pub ProductID: ::windows_sys::core::GUID,
    pub TileID: super::super::Foundation::BSTR,
    pub TemplateType: TILE_TEMPLATE_TYPE,
    pub HubPosition: [u32; 32],
    pub HubVisibilityBitmask: u32,
    pub IsDefault: super::super::Foundation::BOOL,
    pub TileType: PM_STARTTILE_TYPE,
    pub pbPropBlob: *mut u8,
    pub cbPropBlob: u32,
    pub IsRestoring: super::super::Foundation::BOOL,
    pub IsModern: super::super::Foundation::BOOL,
    pub InvocationInfo: PM_INVOCATIONINFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PM_STARTTILEBLOB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_STARTTILEBLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_STARTTILE_TYPE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_STARTTILE_TYPE_PRIMARY: PM_STARTTILE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_STARTTILE_TYPE_SECONDARY: PM_STARTTILE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_STARTTILE_TYPE_APPLIST: PM_STARTTILE_TYPE = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_STARTTILE_TYPE_APPLISTPRIMARY: PM_STARTTILE_TYPE = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_STARTTILE_TYPE_INVALID: PM_STARTTILE_TYPE = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_TASK_TRANSITION = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_TRANSITION_DEFAULT: PM_TASK_TRANSITION = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_TRANSITION_NONE: PM_TASK_TRANSITION = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_TRANSITION_TURNSTILE: PM_TASK_TRANSITION = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_TRANSITION_SLIDE: PM_TASK_TRANSITION = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_TRANSITION_SWIVEL: PM_TASK_TRANSITION = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_TRANSITION_READERBOARD: PM_TASK_TRANSITION = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_TRANSITION_CUSTOM: PM_TASK_TRANSITION = 6i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_TRANSITION_INVALID: PM_TASK_TRANSITION = 7i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_TASK_TYPE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_TYPE_NORMAL: PM_TASK_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_TYPE_DEFAULT: PM_TASK_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_TYPE_SETTINGS: PM_TASK_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_TYPE_BACKGROUNDSERVICEAGENT: PM_TASK_TYPE = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_TYPE_BACKGROUNDWORKER: PM_TASK_TYPE = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TASK_TYPE_INVALID: PM_TASK_TYPE = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_TILE_HUBTYPE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_HUBTYPE_MUSIC: PM_TILE_HUBTYPE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_HUBTYPE_MOSETTINGS: PM_TILE_HUBTYPE = 268435456i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_HUBTYPE_GAMES: PM_TILE_HUBTYPE = 536870912i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_HUBTYPE_APPLIST: PM_TILE_HUBTYPE = 1073741824i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_HUBTYPE_STARTMENU: PM_TILE_HUBTYPE = -2147483648i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_HUBTYPE_LOCKSCREEN: PM_TILE_HUBTYPE = 16777216i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_HUBTYPE_KIDZONE: PM_TILE_HUBTYPE = 33554432i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_HUBTYPE_CACHED: PM_TILE_HUBTYPE = 67108864i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_HUBTYPE_INVALID: PM_TILE_HUBTYPE = 67108865i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type PM_TILE_SIZE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_SIZE_SMALL: PM_TILE_SIZE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_SIZE_MEDIUM: PM_TILE_SIZE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_SIZE_LARGE: PM_TILE_SIZE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_SIZE_SQUARE310X310: PM_TILE_SIZE = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_SIZE_TALL150X310: PM_TILE_SIZE = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const PM_TILE_SIZE_INVALID: PM_TILE_SIZE = 5i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_UPDATEINFO {
    pub ProductID: ::windows_sys::core::GUID,
    pub PackagePath: super::super::Foundation::BSTR,
    pub InstanceID: ::windows_sys::core::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub MarketplaceAppVersion: super::super::Foundation::BSTR,
    pub DeploymentOptions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PM_UPDATEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_UPDATEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PM_UPDATEINFO_LEGACY {
    pub ProductID: ::windows_sys::core::GUID,
    pub PackagePath: super::super::Foundation::BSTR,
    pub InstanceID: ::windows_sys::core::GUID,
    pub pbLicense: *mut u8,
    pub cbLicense: u32,
    pub MarketplaceAppVersion: super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PM_UPDATEINFO_LEGACY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PM_UPDATEINFO_LEGACY {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PPATCH_PROGRESS_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackcontext: *mut ::core::ffi::c_void, currentposition: u32, maximumposition: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PPATCH_SYMLOAD_CALLBACK = ::core::option::Option<unsafe extern "system" fn(whichfile: u32, symbolfilename: super::super::Foundation::PSTR, symtype: u32, symbolfilechecksum: u32, symbolfiletimedate: u32, imagefilechecksum: u32, imagefiletimedate: u32, callbackcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub struct PROTECTED_FILE_DATA {
    pub FileName: [u16; 260],
    pub FileNumber: u32,
}
impl ::core::marker::Copy for PROTECTED_FILE_DATA {}
impl ::core::clone::Clone for PROTECTED_FILE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type QUERYASMINFO_FLAGS = u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const QUERYASMINFO_FLAG_VALIDATE: QUERYASMINFO_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type REINSTALLMODE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const REINSTALLMODE_REPAIR: REINSTALLMODE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const REINSTALLMODE_FILEMISSING: REINSTALLMODE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const REINSTALLMODE_FILEOLDERVERSION: REINSTALLMODE = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const REINSTALLMODE_FILEEQUALVERSION: REINSTALLMODE = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const REINSTALLMODE_FILEEXACT: REINSTALLMODE = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const REINSTALLMODE_FILEVERIFY: REINSTALLMODE = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const REINSTALLMODE_FILEREPLACE: REINSTALLMODE = 64i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const REINSTALLMODE_MACHINEDATA: REINSTALLMODE = 128i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const REINSTALLMODE_USERDATA: REINSTALLMODE = 256i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const REINSTALLMODE_SHORTCUT: REINSTALLMODE = 512i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const REINSTALLMODE_PACKAGE: REINSTALLMODE = 1024i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type RESULTTYPES = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ieUnknown: RESULTTYPES = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ieError: RESULTTYPES = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ieWarning: RESULTTYPES = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ieInfo: RESULTTYPES = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type SCRIPTFLAGS = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SCRIPTFLAGS_CACHEINFO: SCRIPTFLAGS = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SCRIPTFLAGS_SHORTCUTS: SCRIPTFLAGS = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SCRIPTFLAGS_MACHINEASSIGN: SCRIPTFLAGS = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SCRIPTFLAGS_REGDATA_CNFGINFO: SCRIPTFLAGS = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SCRIPTFLAGS_VALIDATE_TRANSFORMS_LIST: SCRIPTFLAGS = 64i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SCRIPTFLAGS_REGDATA_CLASSINFO: SCRIPTFLAGS = 128i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SCRIPTFLAGS_REGDATA_EXTENSIONINFO: SCRIPTFLAGS = 256i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SCRIPTFLAGS_REGDATA_APPINFO: SCRIPTFLAGS = 384i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SCRIPTFLAGS_REGDATA: SCRIPTFLAGS = 416i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SFC_DISABLE_ASK: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SFC_DISABLE_NOPOPUPS: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SFC_DISABLE_NORMAL: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SFC_DISABLE_ONCE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SFC_DISABLE_SETUP: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SFC_QUOTA_DEFAULT: u32 = 50u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SFC_SCAN_ALWAYS: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SFC_SCAN_IMMEDIATE: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SFC_SCAN_NORMAL: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const SFC_SCAN_ONCE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type STATUSTYPES = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ieStatusGetCUB: STATUSTYPES = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ieStatusICECount: STATUSTYPES = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ieStatusMerge: STATUSTYPES = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ieStatusSummaryInfo: STATUSTYPES = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ieStatusCreateEngine: STATUSTYPES = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ieStatusStarting: STATUSTYPES = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ieStatusRunICE: STATUSTYPES = 6i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ieStatusShutdown: STATUSTYPES = 7i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ieStatusSuccess: STATUSTYPES = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ieStatusFail: STATUSTYPES = 9i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const ieStatusCancel: STATUSTYPES = 10i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const STREAM_FORMAT_COMPLIB_MANIFEST: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const STREAM_FORMAT_COMPLIB_MODULE: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const STREAM_FORMAT_WIN32_MANIFEST: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const STREAM_FORMAT_WIN32_MODULE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type TILE_TEMPLATE_TYPE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_INVALID: TILE_TEMPLATE_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_FLIP: TILE_TEMPLATE_TYPE = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_DEEPLINK: TILE_TEMPLATE_TYPE = 13i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_CYCLE: TILE_TEMPLATE_TYPE = 14i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_METROCOUNT: TILE_TEMPLATE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_AGILESTORE: TILE_TEMPLATE_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_GAMES: TILE_TEMPLATE_TYPE = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_CALENDAR: TILE_TEMPLATE_TYPE = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_MUSICVIDEO: TILE_TEMPLATE_TYPE = 7i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEOPLE: TILE_TEMPLATE_TYPE = 10i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_CONTACT: TILE_TEMPLATE_TYPE = 11i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_GROUP: TILE_TEMPLATE_TYPE = 12i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_DEFAULT: TILE_TEMPLATE_TYPE = 15i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_BADGE: TILE_TEMPLATE_TYPE = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_BLOCK: TILE_TEMPLATE_TYPE = 17i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_TEXT01: TILE_TEMPLATE_TYPE = 18i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_TEXT02: TILE_TEMPLATE_TYPE = 19i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_TEXT03: TILE_TEMPLATE_TYPE = 20i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_TEXT04: TILE_TEMPLATE_TYPE = 21i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_TEXT05: TILE_TEMPLATE_TYPE = 22i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_TEXT06: TILE_TEMPLATE_TYPE = 23i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_TEXT07: TILE_TEMPLATE_TYPE = 24i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_TEXT08: TILE_TEMPLATE_TYPE = 25i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_TEXT09: TILE_TEMPLATE_TYPE = 26i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_TEXT10: TILE_TEMPLATE_TYPE = 27i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_TEXT11: TILE_TEMPLATE_TYPE = 28i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_IMAGE: TILE_TEMPLATE_TYPE = 29i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_IMAGECOLLECTION: TILE_TEMPLATE_TYPE = 30i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_IMAGEANDTEXT01: TILE_TEMPLATE_TYPE = 31i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_IMAGEANDTEXT02: TILE_TEMPLATE_TYPE = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_BLOCKANDTEXT01: TILE_TEMPLATE_TYPE = 33i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_BLOCKANDTEXT02: TILE_TEMPLATE_TYPE = 34i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT01: TILE_TEMPLATE_TYPE = 35i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT02: TILE_TEMPLATE_TYPE = 36i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT03: TILE_TEMPLATE_TYPE = 37i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT04: TILE_TEMPLATE_TYPE = 38i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEEKIMAGE01: TILE_TEMPLATE_TYPE = 39i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEEKIMAGE02: TILE_TEMPLATE_TYPE = 40i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEEKIMAGE03: TILE_TEMPLATE_TYPE = 41i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEEKIMAGE04: TILE_TEMPLATE_TYPE = 42i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEEKIMAGE05: TILE_TEMPLATE_TYPE = 43i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEEKIMAGE06: TILE_TEMPLATE_TYPE = 44i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION01: TILE_TEMPLATE_TYPE = 45i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION02: TILE_TEMPLATE_TYPE = 46i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION03: TILE_TEMPLATE_TYPE = 47i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION04: TILE_TEMPLATE_TYPE = 48i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION05: TILE_TEMPLATE_TYPE = 49i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION06: TILE_TEMPLATE_TYPE = 50i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT01: TILE_TEMPLATE_TYPE = 51i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT02: TILE_TEMPLATE_TYPE = 52i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT03: TILE_TEMPLATE_TYPE = 53i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT04: TILE_TEMPLATE_TYPE = 54i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT05: TILE_TEMPLATE_TYPE = 55i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_METROCOUNTQUEUE: TILE_TEMPLATE_TYPE = 56i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_SEARCH: TILE_TEMPLATE_TYPE = 57i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_TILEFLYOUT01: TILE_TEMPLATE_TYPE = 58i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_FOLDER: TILE_TEMPLATE_TYPE = 59i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TILE_TEMPLATE_ALL: TILE_TEMPLATE_TYPE = 100i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_BACKUP: u32 = 128u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_CMI: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_COPYFILES: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_DEPTH_DECR: u32 = 262144u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_DEPTH_INCR: u32 = 131072u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_DETAILS: u32 = 5u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_DEVINST: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_DEVMGR: u32 = 536870912u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_DRIVER_STORE: u32 = 67108864u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_DRVSETUP: u32 = 4194304u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_ERROR: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_FILEQ: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_FLUSH_FILE: u32 = 1048576u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_INF: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_INFDB: u32 = 1024u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_INSTALLER: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_NEWDEV: u32 = 16777216u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_POLICY: u32 = 8388608u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_RESERVED_FLAGS: u32 = 65520u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_SETUP: u32 = 134217728u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_SETUPAPI_BITS: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_SETUPAPI_CMDLINE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_SETUPAPI_DEVLOG: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_SIGVERIF: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_SUMMARY: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_SYSTEM_STATE_CHANGE: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_TAB_1: u32 = 524288u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_TIMESTAMP: u32 = 65536u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_UI: u32 = 256u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_UMPNPMGR: u32 = 33554432u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_UTIL: u32 = 512u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_VENDOR: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_VERBOSE: u32 = 6u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_VERY_VERBOSE: u32 = 7u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const TXTLOG_WARNING: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const UIALL: u32 = 32768u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const UILOGBITS: u32 = 15u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const UINONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type USERINFOSTATE = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const USERINFOSTATE_MOREDATA: USERINFOSTATE = -3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const USERINFOSTATE_INVALIDARG: USERINFOSTATE = -2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const USERINFOSTATE_UNKNOWN: USERINFOSTATE = -1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const USERINFOSTATE_ABSENT: USERINFOSTATE = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const USERINFOSTATE_PRESENT: USERINFOSTATE = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const WARN_BAD_MAJOR_VERSION: u32 = 3222294792u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const WARN_BASE: u32 = 3222294785u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const WARN_EQUAL_FILE_VERSION: u32 = 3222294794u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const WARN_FILE_VERSION_DOWNREV: u32 = 3222294793u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const WARN_IMPROPER_TRANSFORM_VALIDATION: u32 = 3222294788u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const WARN_INVALID_TRANSFORM_VALIDATION: u32 = 3222294791u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const WARN_MAJOR_UPGRADE_PATCH: u32 = 3222294785u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const WARN_OBSOLETION_WITH_MSI30: u32 = 3222294801u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const WARN_OBSOLETION_WITH_PATCHSEQUENCE: u32 = 3222294803u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const WARN_OBSOLETION_WITH_SEQUENCE_DATA: u32 = 3222294802u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const WARN_PATCHPROPERTYNOTSET: u32 = 3222294795u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const WARN_PCW_MISMATCHED_PRODUCT_CODES: u32 = 3222294789u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const WARN_PCW_MISMATCHED_PRODUCT_VERSIONS: u32 = 3222294790u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const WARN_SEQUENCE_DATA_GENERATION_DISABLED: u32 = 3222294786u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const WARN_SEQUENCE_DATA_SUPERSEDENCE_IGNORED: u32 = 3222294787u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const _WIN32_MSI: u32 = 500u32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const _WIN32_MSM: u32 = 100u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub struct _tagAPPTASKTYPE {
    pub ProductID: ::windows_sys::core::GUID,
    pub TaskType: PM_TASK_TYPE,
}
impl ::core::marker::Copy for _tagAPPTASKTYPE {}
impl ::core::clone::Clone for _tagAPPTASKTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const cchMaxInteger: i32 = 12i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbAssemblyAttributes = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbAssemblyAttributesURT: msidbAssemblyAttributes = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbAssemblyAttributesWin32: msidbAssemblyAttributes = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbClassAttributes = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbClassAttributesRelativePath: msidbClassAttributes = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbComponentAttributes = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbComponentAttributesLocalOnly: msidbComponentAttributes = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbComponentAttributesSourceOnly: msidbComponentAttributes = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbComponentAttributesOptional: msidbComponentAttributes = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbComponentAttributesRegistryKeyPath: msidbComponentAttributes = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbComponentAttributesSharedDllRefCount: msidbComponentAttributes = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbComponentAttributesPermanent: msidbComponentAttributes = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbComponentAttributesODBCDataSource: msidbComponentAttributes = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbComponentAttributesTransitive: msidbComponentAttributes = 64i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbComponentAttributesNeverOverwrite: msidbComponentAttributes = 128i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbComponentAttributes64bit: msidbComponentAttributes = 256i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbComponentAttributesDisableRegistryReflection: msidbComponentAttributes = 512i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbComponentAttributesUninstallOnSupersedence: msidbComponentAttributes = 1024i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbComponentAttributesShared: msidbComponentAttributes = 2048i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbControlAttributes = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesVisible: msidbControlAttributes = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesEnabled: msidbControlAttributes = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesSunken: msidbControlAttributes = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesIndirect: msidbControlAttributes = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesInteger: msidbControlAttributes = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesRTLRO: msidbControlAttributes = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesRightAligned: msidbControlAttributes = 64i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesLeftScroll: msidbControlAttributes = 128i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesBiDi: msidbControlAttributes = 224i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesTransparent: msidbControlAttributes = 65536i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesNoPrefix: msidbControlAttributes = 131072i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesNoWrap: msidbControlAttributes = 262144i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesFormatSize: msidbControlAttributes = 524288i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesUsersLanguage: msidbControlAttributes = 1048576i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesMultiline: msidbControlAttributes = 65536i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesPasswordInput: msidbControlAttributes = 2097152i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesProgress95: msidbControlAttributes = 65536i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesRemovableVolume: msidbControlAttributes = 65536i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesFixedVolume: msidbControlAttributes = 131072i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesRemoteVolume: msidbControlAttributes = 262144i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesCDROMVolume: msidbControlAttributes = 524288i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesRAMDiskVolume: msidbControlAttributes = 1048576i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesFloppyVolume: msidbControlAttributes = 2097152i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlShowRollbackCost: msidbControlAttributes = 4194304i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesSorted: msidbControlAttributes = 65536i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesComboList: msidbControlAttributes = 131072i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesImageHandle: msidbControlAttributes = 65536i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesPushLike: msidbControlAttributes = 131072i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesBitmap: msidbControlAttributes = 262144i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesIcon: msidbControlAttributes = 524288i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesFixedSize: msidbControlAttributes = 1048576i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesIconSize16: msidbControlAttributes = 2097152i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesIconSize32: msidbControlAttributes = 4194304i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesIconSize48: msidbControlAttributes = 6291456i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesElevationShield: msidbControlAttributes = 8388608i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbControlAttributesHasBorder: msidbControlAttributes = 16777216i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbCustomActionType = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeDll: msidbCustomActionType = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeExe: msidbCustomActionType = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeTextData: msidbCustomActionType = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeJScript: msidbCustomActionType = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeVBScript: msidbCustomActionType = 6i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeInstall: msidbCustomActionType = 7i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeBinaryData: msidbCustomActionType = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeSourceFile: msidbCustomActionType = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeDirectory: msidbCustomActionType = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeProperty: msidbCustomActionType = 48i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeContinue: msidbCustomActionType = 64i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeAsync: msidbCustomActionType = 128i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeFirstSequence: msidbCustomActionType = 256i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeOncePerProcess: msidbCustomActionType = 512i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeClientRepeat: msidbCustomActionType = 768i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeInScript: msidbCustomActionType = 1024i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeRollback: msidbCustomActionType = 256i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeCommit: msidbCustomActionType = 512i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeNoImpersonate: msidbCustomActionType = 2048i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeTSAware: msidbCustomActionType = 16384i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionType64BitScript: msidbCustomActionType = 4096i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypeHideTarget: msidbCustomActionType = 8192i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbCustomActionTypePatchUninstall: msidbCustomActionType = 32768i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbDialogAttributes = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbDialogAttributesVisible: msidbDialogAttributes = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbDialogAttributesModal: msidbDialogAttributes = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbDialogAttributesMinimize: msidbDialogAttributes = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbDialogAttributesSysModal: msidbDialogAttributes = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbDialogAttributesKeepModeless: msidbDialogAttributes = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbDialogAttributesTrackDiskSpace: msidbDialogAttributes = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbDialogAttributesUseCustomPalette: msidbDialogAttributes = 64i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbDialogAttributesRTLRO: msidbDialogAttributes = 128i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbDialogAttributesRightAligned: msidbDialogAttributes = 256i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbDialogAttributesLeftScroll: msidbDialogAttributes = 512i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbDialogAttributesBiDi: msidbDialogAttributes = 896i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbDialogAttributesError: msidbDialogAttributes = 65536i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbEmbeddedUIAttributes = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbEmbeddedUI: msidbEmbeddedUIAttributes = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbEmbeddedHandlesBasic: msidbEmbeddedUIAttributes = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbFeatureAttributes = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFeatureAttributesFavorLocal: msidbFeatureAttributes = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFeatureAttributesFavorSource: msidbFeatureAttributes = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFeatureAttributesFollowParent: msidbFeatureAttributes = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFeatureAttributesFavorAdvertise: msidbFeatureAttributes = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFeatureAttributesDisallowAdvertise: msidbFeatureAttributes = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFeatureAttributesUIDisallowAbsent: msidbFeatureAttributes = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFeatureAttributesNoUnsupportedAdvertise: msidbFeatureAttributes = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbFileAttributes = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFileAttributesReadOnly: msidbFileAttributes = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFileAttributesHidden: msidbFileAttributes = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFileAttributesSystem: msidbFileAttributes = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFileAttributesReserved0: msidbFileAttributes = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFileAttributesIsolatedComp: msidbFileAttributes = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFileAttributesReserved1: msidbFileAttributes = 64i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFileAttributesReserved2: msidbFileAttributes = 128i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFileAttributesReserved3: msidbFileAttributes = 256i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFileAttributesVital: msidbFileAttributes = 512i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFileAttributesChecksum: msidbFileAttributes = 1024i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFileAttributesPatchAdded: msidbFileAttributes = 4096i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFileAttributesNoncompressed: msidbFileAttributes = 8192i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFileAttributesCompressed: msidbFileAttributes = 16384i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbFileAttributesReserved4: msidbFileAttributes = 32768i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbIniFileAction = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbIniFileActionAddLine: msidbIniFileAction = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbIniFileActionCreateLine: msidbIniFileAction = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbIniFileActionRemoveLine: msidbIniFileAction = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbIniFileActionAddTag: msidbIniFileAction = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbIniFileActionRemoveTag: msidbIniFileAction = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbLocatorType = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbLocatorTypeDirectory: msidbLocatorType = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbLocatorTypeFileName: msidbLocatorType = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbLocatorTypeRawValue: msidbLocatorType = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbLocatorType64bit: msidbLocatorType = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbMoveFileOptions = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbMoveFileOptionsMove: msidbMoveFileOptions = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbODBCDataSourceRegistration = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbODBCDataSourceRegistrationPerMachine: msidbODBCDataSourceRegistration = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbODBCDataSourceRegistrationPerUser: msidbODBCDataSourceRegistration = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbPatchAttributes = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbPatchAttributesNonVital: msidbPatchAttributes = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbRegistryRoot = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbRegistryRootClassesRoot: msidbRegistryRoot = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbRegistryRootCurrentUser: msidbRegistryRoot = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbRegistryRootLocalMachine: msidbRegistryRoot = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbRegistryRootUsers: msidbRegistryRoot = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbRemoveFileInstallMode = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbRemoveFileInstallModeOnInstall: msidbRemoveFileInstallMode = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbRemoveFileInstallModeOnRemove: msidbRemoveFileInstallMode = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbRemoveFileInstallModeOnBoth: msidbRemoveFileInstallMode = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbServiceConfigEvent = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbServiceConfigEventInstall: msidbServiceConfigEvent = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbServiceConfigEventUninstall: msidbServiceConfigEvent = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbServiceConfigEventReinstall: msidbServiceConfigEvent = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbServiceControlEvent = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbServiceControlEventStart: msidbServiceControlEvent = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbServiceControlEventStop: msidbServiceControlEvent = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbServiceControlEventDelete: msidbServiceControlEvent = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbServiceControlEventUninstallStart: msidbServiceControlEvent = 16i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbServiceControlEventUninstallStop: msidbServiceControlEvent = 32i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbServiceControlEventUninstallDelete: msidbServiceControlEvent = 128i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbServiceInstallErrorControl = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbServiceInstallErrorControlVital: msidbServiceInstallErrorControl = 32768i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbSumInfoSourceType = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbSumInfoSourceTypeSFN: msidbSumInfoSourceType = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbSumInfoSourceTypeCompressed: msidbSumInfoSourceType = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbSumInfoSourceTypeAdminImage: msidbSumInfoSourceType = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbSumInfoSourceTypeLUAPackage: msidbSumInfoSourceType = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbTextStyleStyleBits = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbTextStyleStyleBitsBold: msidbTextStyleStyleBits = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbTextStyleStyleBitsItalic: msidbTextStyleStyleBits = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbTextStyleStyleBitsUnderline: msidbTextStyleStyleBits = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbTextStyleStyleBitsStrike: msidbTextStyleStyleBits = 8i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msidbUpgradeAttributes = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbUpgradeAttributesMigrateFeatures: msidbUpgradeAttributes = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbUpgradeAttributesOnlyDetect: msidbUpgradeAttributes = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbUpgradeAttributesIgnoreRemoveFailure: msidbUpgradeAttributes = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbUpgradeAttributesVersionMinInclusive: msidbUpgradeAttributes = 256i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbUpgradeAttributesVersionMaxInclusive: msidbUpgradeAttributes = 512i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msidbUpgradeAttributesLanguagesExclusive: msidbUpgradeAttributes = 1024i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msifiFastInstallBits = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msifiFastInstallNoSR: msifiFastInstallBits = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msifiFastInstallQuickCosting: msifiFastInstallBits = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msifiFastInstallLessPrgMsg: msifiFastInstallBits = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msirbRebootReason = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msirbRebootUndeterminedReason: msirbRebootReason = 0i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msirbRebootInUseFilesReason: msirbRebootReason = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msirbRebootScheduleRebootReason: msirbRebootReason = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msirbRebootForceRebootReason: msirbRebootReason = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msirbRebootCustomActionReason: msirbRebootReason = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msirbRebootType = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msirbRebootImmediate: msirbRebootType = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msirbRebootDeferred: msirbRebootType = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub type msmErrorType = i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msmErrorLanguageUnsupported: msmErrorType = 1i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msmErrorLanguageFailed: msmErrorType = 2i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msmErrorExclusion: msmErrorType = 3i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msmErrorTableMerge: msmErrorType = 4i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msmErrorResequenceMerge: msmErrorType = 5i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msmErrorFileCreate: msmErrorType = 6i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msmErrorDirCreate: msmErrorType = 7i32;
#[doc = "*Required features: 'Win32_System_ApplicationInstallationAndServicing'*"]
pub const msmErrorFeatureRequired: msmErrorType = 8i32;
