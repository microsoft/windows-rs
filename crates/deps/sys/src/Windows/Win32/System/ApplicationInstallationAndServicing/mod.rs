#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn ActivateActCtx(hactctx: super::super::Foundation::HANDLE, lpcookie: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddRefActCtx(hactctx: super::super::Foundation::HANDLE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaA(applyflags: i64, lpsourcename: super::super::Foundation::PSTR, lpdeltaname: super::super::Foundation::PSTR, lptargetname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lptarget: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaGetReverseB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lpreversefiletime: *const super::super::Foundation::FILETIME, lptarget: *mut DELTA_OUTPUT, lptargetreverse: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaProvidedB(applyflags: i64, source: DELTA_INPUT, delta: DELTA_INPUT, lptarget: *mut ::core::ffi::c_void, utargetsize: usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyDeltaW(applyflags: i64, lpsourcename: super::super::Foundation::PWSTR, lpdeltaname: super::super::Foundation::PWSTR, lptargetname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileA(patchfilename: super::super::Foundation::PSTR, oldfilename: super::super::Foundation::PSTR, newfilename: super::super::Foundation::PSTR, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileByBuffers(patchfilemapped: *const u8, patchfilesize: u32, oldfilemapped: *const u8, oldfilesize: u32, newfilebuffer: *mut *mut u8, newfilebuffersize: u32, newfileactualsize: *mut u32, newfiletime: *mut super::super::Foundation::FILETIME, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileByHandles(patchfilehandle: super::super::Foundation::HANDLE, oldfilehandle: super::super::Foundation::HANDLE, newfilehandle: super::super::Foundation::HANDLE, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileByHandlesEx(patchfilehandle: super::super::Foundation::HANDLE, oldfilehandle: super::super::Foundation::HANDLE, newfilehandle: super::super::Foundation::HANDLE, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileExA(patchfilename: super::super::Foundation::PSTR, oldfilename: super::super::Foundation::PSTR, newfilename: super::super::Foundation::PSTR, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileExW(patchfilename: super::super::Foundation::PWSTR, oldfilename: super::super::Foundation::PWSTR, newfilename: super::super::Foundation::PWSTR, applyoptionflags: u32, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPatchToFileW(patchfilename: super::super::Foundation::PWSTR, oldfilename: super::super::Foundation::PWSTR, newfilename: super::super::Foundation::PWSTR, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateActCtxA(pactctx: *const ACTCTXA) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateActCtxW(pactctx: *const ACTCTXW) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeltaA(filetypeset: i64, setflags: i64, resetflags: i64, lpsourcename: super::super::Foundation::PSTR, lptargetname: super::super::Foundation::PSTR, lpsourceoptionsname: super::super::Foundation::PSTR, lptargetoptionsname: super::super::Foundation::PSTR, globaloptions: DELTA_INPUT, lptargetfiletime: *const super::super::Foundation::FILETIME, hashalgid: u32, lpdeltaname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeltaB(filetypeset: i64, setflags: i64, resetflags: i64, source: DELTA_INPUT, target: DELTA_INPUT, sourceoptions: DELTA_INPUT, targetoptions: DELTA_INPUT, globaloptions: DELTA_INPUT, lptargetfiletime: *const super::super::Foundation::FILETIME, hashalgid: u32, lpdelta: *mut DELTA_OUTPUT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDeltaW(filetypeset: i64, setflags: i64, resetflags: i64, lpsourcename: super::super::Foundation::PWSTR, lptargetname: super::super::Foundation::PWSTR, lpsourceoptionsname: super::super::Foundation::PWSTR, lptargetoptionsname: super::super::Foundation::PWSTR, globaloptions: DELTA_INPUT, lptargetfiletime: *const super::super::Foundation::FILETIME, hashalgid: u32, lpdeltaname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileA(oldfilename: super::super::Foundation::PSTR, newfilename: super::super::Foundation::PSTR, patchfilename: super::super::Foundation::PSTR, optionflags: u32, optiondata: *const PATCH_OPTION_DATA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileByHandles(oldfilehandle: super::super::Foundation::HANDLE, newfilehandle: super::super::Foundation::HANDLE, patchfilehandle: super::super::Foundation::HANDLE, optionflags: u32, optiondata: *const PATCH_OPTION_DATA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileByHandlesEx(oldfilecount: u32, oldfileinfoarray: *const PATCH_OLD_FILE_INFO_H, newfilehandle: super::super::Foundation::HANDLE, patchfilehandle: super::super::Foundation::HANDLE, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileExA(oldfilecount: u32, oldfileinfoarray: *const PATCH_OLD_FILE_INFO_A, newfilename: super::super::Foundation::PSTR, patchfilename: super::super::Foundation::PSTR, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileExW(oldfilecount: u32, oldfileinfoarray: *const PATCH_OLD_FILE_INFO_W, newfilename: super::super::Foundation::PWSTR, patchfilename: super::super::Foundation::PWSTR, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, progresscallback: PPATCH_PROGRESS_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePatchFileW(oldfilename: super::super::Foundation::PWSTR, newfilename: super::super::Foundation::PWSTR, patchfilename: super::super::Foundation::PWSTR, optionflags: u32, optiondata: *const PATCH_OPTION_DATA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeactivateActCtx(dwflags: u32, ulcookie: usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeltaFree(lpmemory: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeltaNormalizeProvidedB(filetypeset: i64, normalizeflags: i64, normalizeoptions: DELTA_INPUT, lpsource: *mut ::core::ffi::c_void, usourcesize: usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractPatchHeaderToFileA(patchfilename: super::super::Foundation::PSTR, patchheaderfilename: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractPatchHeaderToFileByHandles(patchfilehandle: super::super::Foundation::HANDLE, patchheaderfilehandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractPatchHeaderToFileW(patchfilename: super::super::Foundation::PWSTR, patchheaderfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FindActCtxSectionGuid(dwflags: u32, lpextensionguid: *const ::windows_sys::core::GUID, ulsectionid: u32, lpguidtofind: *const ::windows_sys::core::GUID, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FindActCtxSectionStringA(dwflags: u32, lpextensionguid: *const ::windows_sys::core::GUID, ulsectionid: u32, lpstringtofind: super::super::Foundation::PSTR, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FindActCtxSectionStringW(dwflags: u32, lpextensionguid: *const ::windows_sys::core::GUID, ulsectionid: u32, lpstringtofind: super::super::Foundation::PWSTR, returneddata: *mut ACTCTX_SECTION_KEYED_DATA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentActCtx(lphactctx: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaInfoA(lpdeltaname: super::super::Foundation::PSTR, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaInfoB(delta: DELTA_INPUT, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaInfoW(lpdeltaname: super::super::Foundation::PWSTR, lpheaderinfo: *mut DELTA_HEADER_INFO) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaSignatureA(filetypeset: i64, hashalgid: u32, lpsourcename: super::super::Foundation::PSTR, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaSignatureB(filetypeset: i64, hashalgid: u32, source: DELTA_INPUT, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeltaSignatureW(filetypeset: i64, hashalgid: u32, lpsourcename: super::super::Foundation::PWSTR, lphash: *mut DELTA_HASH) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilePatchSignatureA(filename: super::super::Foundation::PSTR, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilePatchSignatureByBuffer(filebufferwritable: *mut u8, filesize: u32, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilePatchSignatureByHandle(filehandle: super::super::Foundation::HANDLE, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilePatchSignatureW(filename: super::super::Foundation::PWSTR, optionflags: u32, optiondata: *const ::core::ffi::c_void, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE, signaturebuffersize: u32, signaturebuffer: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiAdvertiseProductA(szpackagepath: super::super::Foundation::PSTR, szscriptfilepath: super::super::Foundation::PSTR, sztransforms: super::super::Foundation::PSTR, lgidlanguage: u16) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiAdvertiseProductExA(szpackagepath: super::super::Foundation::PSTR, szscriptfilepath: super::super::Foundation::PSTR, sztransforms: super::super::Foundation::PSTR, lgidlanguage: u16, dwplatform: u32, dwoptions: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiAdvertiseProductExW(szpackagepath: super::super::Foundation::PWSTR, szscriptfilepath: super::super::Foundation::PWSTR, sztransforms: super::super::Foundation::PWSTR, lgidlanguage: u16, dwplatform: u32, dwoptions: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiAdvertiseProductW(szpackagepath: super::super::Foundation::PWSTR, szscriptfilepath: super::super::Foundation::PWSTR, sztransforms: super::super::Foundation::PWSTR, lgidlanguage: u16) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn MsiAdvertiseScriptA(szscriptfile: super::super::Foundation::PSTR, dwflags: u32, phregdata: *const super::Registry::HKEY, fremoveitems: super::super::Foundation::BOOL) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn MsiAdvertiseScriptW(szscriptfile: super::super::Foundation::PWSTR, dwflags: u32, phregdata: *const super::Registry::HKEY, fremoveitems: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiApplyMultiplePatchesA(szpatchpackages: super::super::Foundation::PSTR, szproductcode: super::super::Foundation::PSTR, szpropertieslist: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiApplyMultiplePatchesW(szpatchpackages: super::super::Foundation::PWSTR, szproductcode: super::super::Foundation::PWSTR, szpropertieslist: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiApplyPatchA(szpatchpackage: super::super::Foundation::PSTR, szinstallpackage: super::super::Foundation::PSTR, einstalltype: INSTALLTYPE, szcommandline: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiApplyPatchW(szpatchpackage: super::super::Foundation::PWSTR, szinstallpackage: super::super::Foundation::PWSTR, einstalltype: INSTALLTYPE, szcommandline: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiBeginTransactionA(szname: super::super::Foundation::PSTR, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiBeginTransactionW(szname: super::super::Foundation::PWSTR, dwtransactionattributes: u32, phtransactionhandle: *mut MSIHANDLE, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32;
    pub fn MsiCloseAllHandles() -> u32;
    pub fn MsiCloseHandle(hany: MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiCollectUserInfoA(szproduct: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiCollectUserInfoW(szproduct: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureFeatureA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR, einstallstate: INSTALLSTATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureFeatureW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR, einstallstate: INSTALLSTATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureProductA(szproduct: super::super::Foundation::PSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureProductExA(szproduct: super::super::Foundation::PSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE, szcommandline: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureProductExW(szproduct: super::super::Foundation::PWSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE, szcommandline: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiConfigureProductW(szproduct: super::super::Foundation::PWSTR, iinstalllevel: INSTALLLEVEL, einstallstate: INSTALLSTATE) -> u32;
    pub fn MsiCreateRecord(cparams: u32) -> MSIHANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiCreateTransformSummaryInfoA(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: super::super::Foundation::PSTR, ierrorconditions: MSITRANSFORM_ERROR, ivalidation: MSITRANSFORM_VALIDATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiCreateTransformSummaryInfoW(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: super::super::Foundation::PWSTR, ierrorconditions: MSITRANSFORM_ERROR, ivalidation: MSITRANSFORM_VALIDATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseApplyTransformA(hdatabase: MSIHANDLE, sztransformfile: super::super::Foundation::PSTR, ierrorconditions: MSITRANSFORM_ERROR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseApplyTransformW(hdatabase: MSIHANDLE, sztransformfile: super::super::Foundation::PWSTR, ierrorconditions: MSITRANSFORM_ERROR) -> u32;
    pub fn MsiDatabaseCommit(hdatabase: MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseExportA(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PSTR, szfolderpath: super::super::Foundation::PSTR, szfilename: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseExportW(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PWSTR, szfolderpath: super::super::Foundation::PWSTR, szfilename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseGenerateTransformA(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: super::super::Foundation::PSTR, ireserved1: i32, ireserved2: i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseGenerateTransformW(hdatabase: MSIHANDLE, hdatabasereference: MSIHANDLE, sztransformfile: super::super::Foundation::PWSTR, ireserved1: i32, ireserved2: i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseGetPrimaryKeysA(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PSTR, phrecord: *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseGetPrimaryKeysW(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PWSTR, phrecord: *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseImportA(hdatabase: MSIHANDLE, szfolderpath: super::super::Foundation::PSTR, szfilename: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseImportW(hdatabase: MSIHANDLE, szfolderpath: super::super::Foundation::PWSTR, szfilename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseIsTablePersistentA(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PSTR) -> MSICONDITION;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseIsTablePersistentW(hdatabase: MSIHANDLE, sztablename: super::super::Foundation::PWSTR) -> MSICONDITION;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseMergeA(hdatabase: MSIHANDLE, hdatabasemerge: MSIHANDLE, sztablename: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseMergeW(hdatabase: MSIHANDLE, hdatabasemerge: MSIHANDLE, sztablename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseOpenViewA(hdatabase: MSIHANDLE, szquery: super::super::Foundation::PSTR, phview: *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDatabaseOpenViewW(hdatabase: MSIHANDLE, szquery: super::super::Foundation::PWSTR, phview: *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDetermineApplicablePatchesA(szproductpackagepath: super::super::Foundation::PSTR, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDetermineApplicablePatchesW(szproductpackagepath: super::super::Foundation::PWSTR, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDeterminePatchSequenceA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDeterminePatchSequenceW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, cpatchinfo: u32, ppatchinfo: *mut MSIPATCHSEQUENCEINFOW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDoActionA(hinstall: MSIHANDLE, szaction: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiDoActionW(hinstall: MSIHANDLE, szaction: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnableLogA(dwlogmode: INSTALLOGMODE, szlogfile: super::super::Foundation::PSTR, dwlogattributes: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnableLogW(dwlogmode: INSTALLOGMODE, szlogfile: super::super::Foundation::PWSTR, dwlogattributes: u32) -> u32;
    pub fn MsiEnableUIPreview(hdatabase: MSIHANDLE, phpreview: *mut MSIHANDLE) -> u32;
    pub fn MsiEndTransaction(dwtransactionstate: MSITRANSACTIONSTATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumClientsA(szcomponent: super::super::Foundation::PSTR, iproductindex: u32, lpproductbuf: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumClientsExA(szcomponent: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwproductindex: u32, szproductbuf: super::super::Foundation::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PSTR, pcchsid: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumClientsExW(szcomponent: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwproductindex: u32, szproductbuf: super::super::Foundation::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PWSTR, pcchsid: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumClientsW(szcomponent: super::super::Foundation::PWSTR, iproductindex: u32, lpproductbuf: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentCostsA(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PSTR, dwindex: u32, istate: INSTALLSTATE, szdrivebuf: super::super::Foundation::PSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentCostsW(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PWSTR, dwindex: u32, istate: INSTALLSTATE, szdrivebuf: super::super::Foundation::PWSTR, pcchdrivebuf: *mut u32, picost: *mut i32, pitempcost: *mut i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentQualifiersA(szcomponent: super::super::Foundation::PSTR, iindex: u32, lpqualifierbuf: super::super::Foundation::PSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: super::super::Foundation::PSTR, pcchapplicationdatabuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentQualifiersW(szcomponent: super::super::Foundation::PWSTR, iindex: u32, lpqualifierbuf: super::super::Foundation::PWSTR, pcchqualifierbuf: *mut u32, lpapplicationdatabuf: super::super::Foundation::PWSTR, pcchapplicationdatabuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentsA(icomponentindex: u32, lpcomponentbuf: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentsExA(szusersid: super::super::Foundation::PSTR, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: super::super::Foundation::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PSTR, pcchsid: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentsExW(szusersid: super::super::Foundation::PWSTR, dwcontext: u32, dwindex: u32, szinstalledcomponentcode: super::super::Foundation::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PWSTR, pcchsid: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumComponentsW(icomponentindex: u32, lpcomponentbuf: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumFeaturesA(szproduct: super::super::Foundation::PSTR, ifeatureindex: u32, lpfeaturebuf: super::super::Foundation::PSTR, lpparentbuf: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumFeaturesW(szproduct: super::super::Foundation::PWSTR, ifeatureindex: u32, lpfeaturebuf: super::super::Foundation::PWSTR, lpparentbuf: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumPatchesA(szproduct: super::super::Foundation::PSTR, ipatchindex: u32, lppatchbuf: super::super::Foundation::PSTR, lptransformsbuf: super::super::Foundation::PSTR, pcchtransformsbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumPatchesExA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: super::super::Foundation::PSTR, sztargetproductcode: super::super::Foundation::PSTR, pdwtargetproductcontext: *mut MSIINSTALLCONTEXT, sztargetusersid: super::super::Foundation::PSTR, pcchtargetusersid: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumPatchesExW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: u32, dwfilter: u32, dwindex: u32, szpatchcode: super::super::Foundation::PWSTR, sztargetproductcode: super::super::Foundation::PWSTR, pdwtargetproductcontext: *mut MSIINSTALLCONTEXT, sztargetusersid: super::super::Foundation::PWSTR, pcchtargetusersid: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumPatchesW(szproduct: super::super::Foundation::PWSTR, ipatchindex: u32, lppatchbuf: super::super::Foundation::PWSTR, lptransformsbuf: super::super::Foundation::PWSTR, pcchtransformsbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumProductsA(iproductindex: u32, lpproductbuf: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumProductsExA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: u32, dwindex: u32, szinstalledproductcode: super::super::Foundation::PSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PSTR, pcchsid: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumProductsExW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: u32, dwindex: u32, szinstalledproductcode: super::super::Foundation::PWSTR, pdwinstalledcontext: *mut MSIINSTALLCONTEXT, szsid: super::super::Foundation::PWSTR, pcchsid: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumProductsW(iproductindex: u32, lpproductbuf: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumRelatedProductsA(lpupgradecode: super::super::Foundation::PSTR, dwreserved: u32, iproductindex: u32, lpproductbuf: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEnumRelatedProductsW(lpupgradecode: super::super::Foundation::PWSTR, dwreserved: u32, iproductindex: u32, lpproductbuf: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEvaluateConditionA(hinstall: MSIHANDLE, szcondition: super::super::Foundation::PSTR) -> MSICONDITION;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiEvaluateConditionW(hinstall: MSIHANDLE, szcondition: super::super::Foundation::PWSTR) -> MSICONDITION;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiExtractPatchXMLDataA(szpatchpath: super::super::Foundation::PSTR, dwreserved: u32, szxmldata: super::super::Foundation::PSTR, pcchxmldata: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiExtractPatchXMLDataW(szpatchpath: super::super::Foundation::PWSTR, dwreserved: u32, szxmldata: super::super::Foundation::PWSTR, pcchxmldata: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiFormatRecordA(hinstall: MSIHANDLE, hrecord: MSIHANDLE, szresultbuf: super::super::Foundation::PSTR, pcchresultbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiFormatRecordW(hinstall: MSIHANDLE, hrecord: MSIHANDLE, szresultbuf: super::super::Foundation::PWSTR, pcchresultbuf: *mut u32) -> u32;
    pub fn MsiGetActiveDatabase(hinstall: MSIHANDLE) -> MSIHANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentPathA(szproduct: super::super::Foundation::PSTR, szcomponent: super::super::Foundation::PSTR, lppathbuf: super::super::Foundation::PSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentPathExA(szproductcode: super::super::Foundation::PSTR, szcomponentcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, lpoutpathbuffer: super::super::Foundation::PSTR, pcchoutpathbuffer: *mut u32) -> INSTALLSTATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentPathExW(szproductcode: super::super::Foundation::PWSTR, szcomponentcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, lpoutpathbuffer: super::super::Foundation::PWSTR, pcchoutpathbuffer: *mut u32) -> INSTALLSTATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentPathW(szproduct: super::super::Foundation::PWSTR, szcomponent: super::super::Foundation::PWSTR, lppathbuf: super::super::Foundation::PWSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentStateA(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetComponentStateW(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PWSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    pub fn MsiGetDatabaseState(hdatabase: MSIHANDLE) -> MSIDBSTATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureCostA(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PSTR, icosttree: MSICOSTTREE, istate: INSTALLSTATE, picost: *mut i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureCostW(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, icosttree: MSICOSTTREE, istate: INSTALLSTATE, picost: *mut i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureInfoA(hproduct: MSIHANDLE, szfeature: super::super::Foundation::PSTR, lpattributes: *mut u32, lptitlebuf: super::super::Foundation::PSTR, pcchtitlebuf: *mut u32, lphelpbuf: super::super::Foundation::PSTR, pcchhelpbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureInfoW(hproduct: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, lpattributes: *mut u32, lptitlebuf: super::super::Foundation::PWSTR, pcchtitlebuf: *mut u32, lphelpbuf: super::super::Foundation::PWSTR, pcchhelpbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureStateA(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureStateW(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, piinstalled: *mut INSTALLSTATE, piaction: *mut INSTALLSTATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureUsageA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR, pdwusecount: *mut u32, pwdateused: *mut u16) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureUsageW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR, pdwusecount: *mut u32, pwdateused: *mut u16) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureValidStatesA(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PSTR, lpinstallstates: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFeatureValidStatesW(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, lpinstallstates: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFileHashA(szfilepath: super::super::Foundation::PSTR, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFileHashW(szfilepath: super::super::Foundation::PWSTR, dwoptions: u32, phash: *mut MSIFILEHASHINFO) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn MsiGetFileSignatureInformationA(szsignedobjectpath: super::super::Foundation::PSTR, dwflags: u32, ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, pbhashdata: *mut u8, pcbhashdata: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub fn MsiGetFileSignatureInformationW(szsignedobjectpath: super::super::Foundation::PWSTR, dwflags: u32, ppccertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT, pbhashdata: *mut u8, pcbhashdata: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFileVersionA(szfilepath: super::super::Foundation::PSTR, lpversionbuf: super::super::Foundation::PSTR, pcchversionbuf: *mut u32, lplangbuf: super::super::Foundation::PSTR, pcchlangbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetFileVersionW(szfilepath: super::super::Foundation::PWSTR, lpversionbuf: super::super::Foundation::PWSTR, pcchversionbuf: *mut u32, lplangbuf: super::super::Foundation::PWSTR, pcchlangbuf: *mut u32) -> u32;
    pub fn MsiGetLanguage(hinstall: MSIHANDLE) -> u16;
    pub fn MsiGetLastErrorRecord() -> MSIHANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetMode(hinstall: MSIHANDLE, erunmode: MSIRUNMODE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchFileListA(szproductcode: super::super::Foundation::PSTR, szpatchpackages: super::super::Foundation::PSTR, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchFileListW(szproductcode: super::super::Foundation::PWSTR, szpatchpackages: super::super::Foundation::PWSTR, pcfiles: *mut u32, pphfilerecords: *mut *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchInfoA(szpatch: super::super::Foundation::PSTR, szattribute: super::super::Foundation::PSTR, lpvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchInfoExA(szpatchcode: super::super::Foundation::PSTR, szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: super::super::Foundation::PSTR, lpvalue: super::super::Foundation::PSTR, pcchvalue: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchInfoExW(szpatchcode: super::super::Foundation::PWSTR, szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: super::super::Foundation::PWSTR, lpvalue: super::super::Foundation::PWSTR, pcchvalue: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPatchInfoW(szpatch: super::super::Foundation::PWSTR, szattribute: super::super::Foundation::PWSTR, lpvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductCodeA(szcomponent: super::super::Foundation::PSTR, lpbuf39: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductCodeW(szcomponent: super::super::Foundation::PWSTR, lpbuf39: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoA(szproduct: super::super::Foundation::PSTR, szattribute: super::super::Foundation::PSTR, lpvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoExA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: super::super::Foundation::PSTR, szvalue: super::super::Foundation::PSTR, pcchvalue: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoExW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, szproperty: super::super::Foundation::PWSTR, szvalue: super::super::Foundation::PWSTR, pcchvalue: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoFromScriptA(szscriptfile: super::super::Foundation::PSTR, lpproductbuf39: super::super::Foundation::PSTR, plgidlanguage: *mut u16, pdwversion: *mut u32, lpnamebuf: super::super::Foundation::PSTR, pcchnamebuf: *mut u32, lppackagebuf: super::super::Foundation::PSTR, pcchpackagebuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoFromScriptW(szscriptfile: super::super::Foundation::PWSTR, lpproductbuf39: super::super::Foundation::PWSTR, plgidlanguage: *mut u16, pdwversion: *mut u32, lpnamebuf: super::super::Foundation::PWSTR, pcchnamebuf: *mut u32, lppackagebuf: super::super::Foundation::PWSTR, pcchpackagebuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductInfoW(szproduct: super::super::Foundation::PWSTR, szattribute: super::super::Foundation::PWSTR, lpvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductPropertyA(hproduct: MSIHANDLE, szproperty: super::super::Foundation::PSTR, lpvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetProductPropertyW(hproduct: MSIHANDLE, szproperty: super::super::Foundation::PWSTR, lpvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPropertyA(hinstall: MSIHANDLE, szname: super::super::Foundation::PSTR, szvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetPropertyW(hinstall: MSIHANDLE, szname: super::super::Foundation::PWSTR, szvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetShortcutTargetA(szshortcutpath: super::super::Foundation::PSTR, szproductcode: super::super::Foundation::PSTR, szfeatureid: super::super::Foundation::PSTR, szcomponentcode: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetShortcutTargetW(szshortcutpath: super::super::Foundation::PWSTR, szproductcode: super::super::Foundation::PWSTR, szfeatureid: super::super::Foundation::PWSTR, szcomponentcode: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetSourcePathA(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PSTR, szpathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetSourcePathW(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PWSTR, szpathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetSummaryInformationA(hdatabase: MSIHANDLE, szdatabasepath: super::super::Foundation::PSTR, uiupdatecount: u32, phsummaryinfo: *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetSummaryInformationW(hdatabase: MSIHANDLE, szdatabasepath: super::super::Foundation::PWSTR, uiupdatecount: u32, phsummaryinfo: *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetTargetPathA(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PSTR, szpathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetTargetPathW(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PWSTR, szpathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetUserInfoA(szproduct: super::super::Foundation::PSTR, lpusernamebuf: super::super::Foundation::PSTR, pcchusernamebuf: *mut u32, lporgnamebuf: super::super::Foundation::PSTR, pcchorgnamebuf: *mut u32, lpserialbuf: super::super::Foundation::PSTR, pcchserialbuf: *mut u32) -> USERINFOSTATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiGetUserInfoW(szproduct: super::super::Foundation::PWSTR, lpusernamebuf: super::super::Foundation::PWSTR, pcchusernamebuf: *mut u32, lporgnamebuf: super::super::Foundation::PWSTR, pcchorgnamebuf: *mut u32, lpserialbuf: super::super::Foundation::PWSTR, pcchserialbuf: *mut u32) -> USERINFOSTATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallMissingComponentA(szproduct: super::super::Foundation::PSTR, szcomponent: super::super::Foundation::PSTR, einstallstate: INSTALLSTATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallMissingComponentW(szproduct: super::super::Foundation::PWSTR, szcomponent: super::super::Foundation::PWSTR, einstallstate: INSTALLSTATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallMissingFileA(szproduct: super::super::Foundation::PSTR, szfile: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallMissingFileW(szproduct: super::super::Foundation::PWSTR, szfile: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallProductA(szpackagepath: super::super::Foundation::PSTR, szcommandline: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiInstallProductW(szpackagepath: super::super::Foundation::PWSTR, szcommandline: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiIsProductElevatedA(szproduct: super::super::Foundation::PSTR, pfelevated: *mut super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiIsProductElevatedW(szproduct: super::super::Foundation::PWSTR, pfelevated: *mut super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiJoinTransaction(htransactionhandle: MSIHANDLE, dwtransactionattributes: u32, phchangeofownerevent: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiLocateComponentA(szcomponent: super::super::Foundation::PSTR, lppathbuf: super::super::Foundation::PSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiLocateComponentW(szcomponent: super::super::Foundation::PWSTR, lppathbuf: super::super::Foundation::PWSTR, pcchbuf: *mut u32) -> INSTALLSTATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiNotifySidChangeA(poldsid: super::super::Foundation::PSTR, pnewsid: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiNotifySidChangeW(poldsid: super::super::Foundation::PWSTR, pnewsid: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenDatabaseA(szdatabasepath: super::super::Foundation::PSTR, szpersist: super::super::Foundation::PSTR, phdatabase: *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenDatabaseW(szdatabasepath: super::super::Foundation::PWSTR, szpersist: super::super::Foundation::PWSTR, phdatabase: *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenPackageA(szpackagepath: super::super::Foundation::PSTR, hproduct: *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenPackageExA(szpackagepath: super::super::Foundation::PSTR, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenPackageExW(szpackagepath: super::super::Foundation::PWSTR, dwoptions: u32, hproduct: *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenPackageW(szpackagepath: super::super::Foundation::PWSTR, hproduct: *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenProductA(szproduct: super::super::Foundation::PSTR, hproduct: *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiOpenProductW(szproduct: super::super::Foundation::PWSTR, hproduct: *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiPreviewBillboardA(hpreview: MSIHANDLE, szcontrolname: super::super::Foundation::PSTR, szbillboard: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiPreviewBillboardW(hpreview: MSIHANDLE, szcontrolname: super::super::Foundation::PWSTR, szbillboard: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiPreviewDialogA(hpreview: MSIHANDLE, szdialogname: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiPreviewDialogW(hpreview: MSIHANDLE, szdialogname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn MsiProcessAdvertiseScriptA(szscriptfile: super::super::Foundation::PSTR, sziconfolder: super::super::Foundation::PSTR, hregdata: super::Registry::HKEY, fshortcuts: super::super::Foundation::BOOL, fremoveitems: super::super::Foundation::BOOL) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn MsiProcessAdvertiseScriptW(szscriptfile: super::super::Foundation::PWSTR, sziconfolder: super::super::Foundation::PWSTR, hregdata: super::Registry::HKEY, fshortcuts: super::super::Foundation::BOOL, fremoveitems: super::super::Foundation::BOOL) -> u32;
    pub fn MsiProcessMessage(hinstall: MSIHANDLE, emessagetype: INSTALLMESSAGE, hrecord: MSIHANDLE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideAssemblyA(szassemblyname: super::super::Foundation::PSTR, szappcontext: super::super::Foundation::PSTR, dwinstallmode: INSTALLMODE, dwassemblyinfo: MSIASSEMBLYINFO, lppathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideAssemblyW(szassemblyname: super::super::Foundation::PWSTR, szappcontext: super::super::Foundation::PWSTR, dwinstallmode: INSTALLMODE, dwassemblyinfo: MSIASSEMBLYINFO, lppathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideComponentA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR, szcomponent: super::super::Foundation::PSTR, dwinstallmode: INSTALLMODE, lppathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideComponentW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR, szcomponent: super::super::Foundation::PWSTR, dwinstallmode: INSTALLMODE, lppathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideQualifiedComponentA(szcategory: super::super::Foundation::PSTR, szqualifier: super::super::Foundation::PSTR, dwinstallmode: INSTALLMODE, lppathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideQualifiedComponentExA(szcategory: super::super::Foundation::PSTR, szqualifier: super::super::Foundation::PSTR, dwinstallmode: INSTALLMODE, szproduct: super::super::Foundation::PSTR, dwunused1: u32, dwunused2: u32, lppathbuf: super::super::Foundation::PSTR, pcchpathbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideQualifiedComponentExW(szcategory: super::super::Foundation::PWSTR, szqualifier: super::super::Foundation::PWSTR, dwinstallmode: INSTALLMODE, szproduct: super::super::Foundation::PWSTR, dwunused1: u32, dwunused2: u32, lppathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiProvideQualifiedComponentW(szcategory: super::super::Foundation::PWSTR, szqualifier: super::super::Foundation::PWSTR, dwinstallmode: INSTALLMODE, lppathbuf: super::super::Foundation::PWSTR, pcchpathbuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryComponentStateA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: super::super::Foundation::PSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryComponentStateW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, szcomponentcode: super::super::Foundation::PWSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryFeatureStateA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR) -> INSTALLSTATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryFeatureStateExA(szproductcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, szfeature: super::super::Foundation::PSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryFeatureStateExW(szproductcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, szfeature: super::super::Foundation::PWSTR, pdwstate: *mut INSTALLSTATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryFeatureStateW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR) -> INSTALLSTATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryProductStateA(szproduct: super::super::Foundation::PSTR) -> INSTALLSTATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiQueryProductStateW(szproduct: super::super::Foundation::PWSTR) -> INSTALLSTATE;
    pub fn MsiRecordClearData(hrecord: MSIHANDLE) -> u32;
    pub fn MsiRecordDataSize(hrecord: MSIHANDLE, ifield: u32) -> u32;
    pub fn MsiRecordGetFieldCount(hrecord: MSIHANDLE) -> u32;
    pub fn MsiRecordGetInteger(hrecord: MSIHANDLE, ifield: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordGetStringA(hrecord: MSIHANDLE, ifield: u32, szvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordGetStringW(hrecord: MSIHANDLE, ifield: u32, szvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordIsNull(hrecord: MSIHANDLE, ifield: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordReadStream(hrecord: MSIHANDLE, ifield: u32, szdatabuf: super::super::Foundation::PSTR, pcbdatabuf: *mut u32) -> u32;
    pub fn MsiRecordSetInteger(hrecord: MSIHANDLE, ifield: u32, ivalue: i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordSetStreamA(hrecord: MSIHANDLE, ifield: u32, szfilepath: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordSetStreamW(hrecord: MSIHANDLE, ifield: u32, szfilepath: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordSetStringA(hrecord: MSIHANDLE, ifield: u32, szvalue: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRecordSetStringW(hrecord: MSIHANDLE, ifield: u32, szvalue: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiReinstallFeatureA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR, dwreinstallmode: REINSTALLMODE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiReinstallFeatureW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR, dwreinstallmode: REINSTALLMODE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiReinstallProductA(szproduct: super::super::Foundation::PSTR, szreinstallmode: REINSTALLMODE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiReinstallProductW(szproduct: super::super::Foundation::PWSTR, szreinstallmode: REINSTALLMODE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRemovePatchesA(szpatchlist: super::super::Foundation::PSTR, szproductcode: super::super::Foundation::PSTR, euninstalltype: INSTALLTYPE, szpropertylist: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiRemovePatchesW(szpatchlist: super::super::Foundation::PWSTR, szproductcode: super::super::Foundation::PWSTR, euninstalltype: INSTALLTYPE, szpropertylist: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSequenceA(hinstall: MSIHANDLE, sztable: super::super::Foundation::PSTR, isequencemode: i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSequenceW(hinstall: MSIHANDLE, sztable: super::super::Foundation::PWSTR, isequencemode: i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetComponentStateA(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PSTR, istate: INSTALLSTATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetComponentStateW(hinstall: MSIHANDLE, szcomponent: super::super::Foundation::PWSTR, istate: INSTALLSTATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetExternalUIA(puihandler: INSTALLUI_HANDLERA, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void) -> ::core::option::Option<INSTALLUI_HANDLERA>;
    pub fn MsiSetExternalUIRecord(puihandler: PINSTALLUI_HANDLER_RECORD, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void, ppuiprevhandler: PINSTALLUI_HANDLER_RECORD) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetExternalUIW(puihandler: INSTALLUI_HANDLERW, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void) -> ::core::option::Option<INSTALLUI_HANDLERW>;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetFeatureAttributesA(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PSTR, dwattributes: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetFeatureAttributesW(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, dwattributes: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetFeatureStateA(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PSTR, istate: INSTALLSTATE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetFeatureStateW(hinstall: MSIHANDLE, szfeature: super::super::Foundation::PWSTR, istate: INSTALLSTATE) -> u32;
    pub fn MsiSetInstallLevel(hinstall: MSIHANDLE, iinstalllevel: i32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetInternalUI(dwuilevel: INSTALLUILEVEL, phwnd: *mut super::super::Foundation::HWND) -> INSTALLUILEVEL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetMode(hinstall: MSIHANDLE, erunmode: MSIRUNMODE, fstate: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetPropertyA(hinstall: MSIHANDLE, szname: super::super::Foundation::PSTR, szvalue: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetPropertyW(hinstall: MSIHANDLE, szname: super::super::Foundation::PWSTR, szvalue: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetTargetPathA(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PSTR, szfolderpath: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetTargetPathW(hinstall: MSIHANDLE, szfolder: super::super::Foundation::PWSTR, szfolderpath: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddMediaDiskA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: super::super::Foundation::PSTR, szdiskprompt: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddMediaDiskW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32, szvolumelabel: super::super::Foundation::PWSTR, szdiskprompt: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddSourceA(szproduct: super::super::Foundation::PSTR, szusername: super::super::Foundation::PSTR, dwreserved: u32, szsource: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddSourceExA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: super::super::Foundation::PSTR, dwindex: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddSourceExW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: super::super::Foundation::PWSTR, dwindex: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListAddSourceW(szproduct: super::super::Foundation::PWSTR, szusername: super::super::Foundation::PWSTR, dwreserved: u32, szsource: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearAllA(szproduct: super::super::Foundation::PSTR, szusername: super::super::Foundation::PSTR, dwreserved: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearAllExA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearAllExW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearAllW(szproduct: super::super::Foundation::PWSTR, szusername: super::super::Foundation::PWSTR, dwreserved: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearMediaDiskA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearMediaDiskW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwdiskid: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearSourceA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListClearSourceW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szsource: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListEnumMediaDisksA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: *mut u32, szvolumelabel: super::super::Foundation::PSTR, pcchvolumelabel: *mut u32, szdiskprompt: super::super::Foundation::PSTR, pcchdiskprompt: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListEnumMediaDisksW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, pdwdiskid: *mut u32, szvolumelabel: super::super::Foundation::PWSTR, pcchvolumelabel: *mut u32, szdiskprompt: super::super::Foundation::PWSTR, pcchdiskprompt: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListEnumSourcesA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: super::super::Foundation::PSTR, pcchsource: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListEnumSourcesW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, dwindex: u32, szsource: super::super::Foundation::PWSTR, pcchsource: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListForceResolutionA(szproduct: super::super::Foundation::PSTR, szusername: super::super::Foundation::PSTR, dwreserved: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListForceResolutionExA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListForceResolutionExW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListForceResolutionW(szproduct: super::super::Foundation::PWSTR, szusername: super::super::Foundation::PWSTR, dwreserved: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListGetInfoA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: super::super::Foundation::PSTR, szvalue: super::super::Foundation::PSTR, pcchvalue: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListGetInfoW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: super::super::Foundation::PWSTR, szvalue: super::super::Foundation::PWSTR, pcchvalue: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListSetInfoA(szproductcodeorpatchcode: super::super::Foundation::PSTR, szusersid: super::super::Foundation::PSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: super::super::Foundation::PSTR, szvalue: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSourceListSetInfoW(szproductcodeorpatchcode: super::super::Foundation::PWSTR, szusersid: super::super::Foundation::PWSTR, dwcontext: MSIINSTALLCONTEXT, dwoptions: u32, szproperty: super::super::Foundation::PWSTR, szvalue: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSummaryInfoGetPropertyA(hsummaryinfo: MSIHANDLE, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: *mut super::super::Foundation::FILETIME, szvaluebuf: super::super::Foundation::PSTR, pcchvaluebuf: *mut u32) -> u32;
    pub fn MsiSummaryInfoGetPropertyCount(hsummaryinfo: MSIHANDLE, puipropertycount: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSummaryInfoGetPropertyW(hsummaryinfo: MSIHANDLE, uiproperty: u32, puidatatype: *mut u32, pivalue: *mut i32, pftvalue: *mut super::super::Foundation::FILETIME, szvaluebuf: super::super::Foundation::PWSTR, pcchvaluebuf: *mut u32) -> u32;
    pub fn MsiSummaryInfoPersist(hsummaryinfo: MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSummaryInfoSetPropertyA(hsummaryinfo: MSIHANDLE, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut super::super::Foundation::FILETIME, szvalue: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSummaryInfoSetPropertyW(hsummaryinfo: MSIHANDLE, uiproperty: u32, uidatatype: u32, ivalue: i32, pftvalue: *mut super::super::Foundation::FILETIME, szvalue: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiUseFeatureA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR) -> INSTALLSTATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiUseFeatureExA(szproduct: super::super::Foundation::PSTR, szfeature: super::super::Foundation::PSTR, dwinstallmode: u32, dwreserved: u32) -> INSTALLSTATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiUseFeatureExW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR, dwinstallmode: u32, dwreserved: u32) -> INSTALLSTATE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiUseFeatureW(szproduct: super::super::Foundation::PWSTR, szfeature: super::super::Foundation::PWSTR) -> INSTALLSTATE;
    pub fn MsiVerifyDiskSpace(hinstall: MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiVerifyPackageA(szpackagepath: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiVerifyPackageW(szpackagepath: super::super::Foundation::PWSTR) -> u32;
    pub fn MsiViewClose(hview: MSIHANDLE) -> u32;
    pub fn MsiViewExecute(hview: MSIHANDLE, hrecord: MSIHANDLE) -> u32;
    pub fn MsiViewFetch(hview: MSIHANDLE, phrecord: *mut MSIHANDLE) -> u32;
    pub fn MsiViewGetColumnInfo(hview: MSIHANDLE, ecolumninfo: MSICOLINFO, phrecord: *mut MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiViewGetErrorA(hview: MSIHANDLE, szcolumnnamebuffer: super::super::Foundation::PSTR, pcchbuf: *mut u32) -> MSIDBERROR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiViewGetErrorW(hview: MSIHANDLE, szcolumnnamebuffer: super::super::Foundation::PWSTR, pcchbuf: *mut u32) -> MSIDBERROR;
    pub fn MsiViewModify(hview: MSIHANDLE, emodifymode: MSIMODIFY, hrecord: MSIHANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NormalizeFileForPatchSignature(filebuffer: *mut ::core::ffi::c_void, filesize: u32, optionflags: u32, optiondata: *const PATCH_OPTION_DATA, newfilecoffbase: u32, newfilecofftime: u32, ignorerangecount: u32, ignorerangearray: *const PATCH_IGNORE_RANGE, retainrangecount: u32, retainrangearray: *const PATCH_RETAIN_RANGE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryActCtxSettingsW(dwflags: u32, hactctx: super::super::Foundation::HANDLE, settingsnamespace: super::super::Foundation::PWSTR, settingname: super::super::Foundation::PWSTR, pvbuffer: super::super::Foundation::PWSTR, dwbuffer: usize, pdwwrittenorrequired: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryActCtxW(dwflags: u32, hactctx: super::super::Foundation::HANDLE, pvsubinstance: *const ::core::ffi::c_void, ulinfoclass: u32, pvbuffer: *mut ::core::ffi::c_void, cbbuffer: usize, pcbwrittenorrequired: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReleaseActCtx(hactctx: super::super::Foundation::HANDLE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SfcGetNextProtectedFile(rpchandle: super::super::Foundation::HANDLE, protfiledata: *mut PROTECTED_FILE_DATA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SfcIsFileProtected(rpchandle: super::super::Foundation::HANDLE, protfilename: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SfcIsKeyProtected(keyhandle: super::Registry::HKEY, subkeyname: super::super::Foundation::PWSTR, keysam: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SfpVerifyFile(pszfilename: super::super::Foundation::PSTR, pszerror: super::super::Foundation::PSTR, dwerrsize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TestApplyPatchToFileA(patchfilename: super::super::Foundation::PSTR, oldfilename: super::super::Foundation::PSTR, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TestApplyPatchToFileByBuffers(patchfilebuffer: *const u8, patchfilesize: u32, oldfilebuffer: *const u8, oldfilesize: u32, newfilesize: *mut u32, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TestApplyPatchToFileByHandles(patchfilehandle: super::super::Foundation::HANDLE, oldfilehandle: super::super::Foundation::HANDLE, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TestApplyPatchToFileW(patchfilename: super::super::Foundation::PWSTR, oldfilename: super::super::Foundation::PWSTR, applyoptionflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ZombifyActCtx(hactctx: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ACTCTXA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ACTCTXW(i32);
#[repr(C)]
pub struct ACTCTX_COMPATIBILITY_ELEMENT_TYPE(i32);
#[repr(C)]
pub struct ACTCTX_REQUESTED_RUN_LEVEL(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
#[repr(C)]
pub struct ACTCTX_SECTION_KEYED_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION(i32);
#[repr(C)]
pub struct ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ACTIVATION_CONTEXT_DETAILED_INFORMATION(i32);
#[repr(C)]
pub struct ACTIVATION_CONTEXT_QUERY_INDEX(i32);
#[repr(C)]
pub struct ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION(i32);
#[repr(C)]
pub struct ADVERTISEFLAGS(i32);
pub const APPLY_OPTION_FAIL_IF_CLOSE: u32 = 2u32;
pub const APPLY_OPTION_FAIL_IF_EXACT: u32 = 1u32;
pub const APPLY_OPTION_TEST_ONLY: u32 = 4u32;
pub const APPLY_OPTION_VALID_FLAGS: u32 = 7u32;
#[repr(C)]
pub struct ASM_BIND_FLAGS(i32);
#[repr(C)]
pub struct ASM_CMP_FLAGS(i32);
#[repr(C)]
pub struct ASM_DISPLAY_FLAGS(i32);
#[repr(C)]
pub struct ASM_NAME(i32);
pub const ASSEMBLYINFO_FLAG_INSTALLED: u32 = 1u32;
pub const ASSEMBLYINFO_FLAG_PAYLOADRESIDENT: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ASSEMBLY_FILE_DETAILED_INFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ASSEMBLY_INFO(i32);
pub const CLSID_EvalCom2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1851660560, data2: 32851, data3: 18016, data4: [183, 149, 107, 97, 46, 41, 188, 88] };
pub const CLSID_MsmMerge2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4182345173, data2: 10745, data3: 18243, data4: [152, 5, 153, 188, 63, 53, 182, 120] };
#[repr(C)]
pub struct COMPATIBILITY_CONTEXT_ELEMENT(i32);
#[repr(C)]
pub struct CREATE_ASM_NAME_OBJ_FLAGS(i32);
pub const DEFAULT_DISK_ID: u32 = 2u32;
pub const DEFAULT_FILE_SEQUENCE_START: u32 = 2u32;
pub const DEFAULT_MINIMUM_REQUIRED_MSI_VERSION: u32 = 100u32;
#[repr(C)]
pub struct DELTA_HASH(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DELTA_HEADER_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DELTA_INPUT(i32);
pub const DELTA_MAX_HASH_SIZE: u32 = 32u32;
#[repr(C)]
pub struct DELTA_OUTPUT(i32);
pub const ERROR_PATCH_BIGGER_THAN_COMPRESSED: u32 = 3222155525u32;
pub const ERROR_PATCH_CORRUPT: u32 = 3222159618u32;
pub const ERROR_PATCH_DECODE_FAILURE: u32 = 3222159617u32;
pub const ERROR_PATCH_ENCODE_FAILURE: u32 = 3222155521u32;
pub const ERROR_PATCH_IMAGEHLP_FAILURE: u32 = 3222155526u32;
pub const ERROR_PATCH_INVALID_OPTIONS: u32 = 3222155522u32;
pub const ERROR_PATCH_NEWER_FORMAT: u32 = 3222159619u32;
pub const ERROR_PATCH_NOT_AVAILABLE: u32 = 3222159622u32;
pub const ERROR_PATCH_NOT_NECESSARY: u32 = 3222159621u32;
pub const ERROR_PATCH_RETAIN_RANGES_DIFFER: u32 = 3222155524u32;
pub const ERROR_PATCH_SAME_FILE: u32 = 3222155523u32;
pub const ERROR_PATCH_WRONG_FILE: u32 = 3222159620u32;
pub const ERROR_PCW_BAD_API_PATCHING_SYMBOL_FLAGS: u32 = 3222163725u32;
pub const ERROR_PCW_BAD_FAMILY_RANGE_NAME: u32 = 3222163801u32;
pub const ERROR_PCW_BAD_FILE_SEQUENCE_START: u32 = 3222163770u32;
pub const ERROR_PCW_BAD_GUIDS_TO_REPLACE: u32 = 3222163721u32;
pub const ERROR_PCW_BAD_IMAGE_FAMILY_DISKID: u32 = 3222163773u32;
pub const ERROR_PCW_BAD_IMAGE_FAMILY_FILESEQSTART: u32 = 3222163774u32;
pub const ERROR_PCW_BAD_IMAGE_FAMILY_NAME: u32 = 3222163748u32;
pub const ERROR_PCW_BAD_IMAGE_FAMILY_SRC_PROP: u32 = 3222163750u32;
pub const ERROR_PCW_BAD_MAJOR_VERSION: u32 = 3222163853u32;
pub const ERROR_PCW_BAD_PATCH_GUID: u32 = 3222163720u32;
pub const ERROR_PCW_BAD_PRODUCTVERSION_VALIDATION: u32 = 3222163844u32;
pub const ERROR_PCW_BAD_SEQUENCE: u32 = 3222163848u32;
pub const ERROR_PCW_BAD_SUPERCEDENCE: u32 = 3222163847u32;
pub const ERROR_PCW_BAD_TARGET: u32 = 3222163849u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_NAME: u32 = 3222163736u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_PRODUCT_CODE: u32 = 3222163834u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_PRODUCT_VERSION: u32 = 3222163835u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_UPGRADED: u32 = 3222163776u32;
pub const ERROR_PCW_BAD_TARGET_IMAGE_UPGRADE_CODE: u32 = 3222163836u32;
pub const ERROR_PCW_BAD_TARGET_PRODUCT_CODE_LIST: u32 = 3222163722u32;
pub const ERROR_PCW_BAD_TGT_UPD_IMAGES: u32 = 3222163846u32;
pub const ERROR_PCW_BAD_TRANSFORMSET: u32 = 3222163845u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_FAMILY: u32 = 3222163775u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_NAME: u32 = 3222163728u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_PRODUCT_CODE: u32 = 3222163831u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_PRODUCT_VERSION: u32 = 3222163832u32;
pub const ERROR_PCW_BAD_UPGRADED_IMAGE_UPGRADE_CODE: u32 = 3222163833u32;
pub const ERROR_PCW_BAD_VERSION_STRING: u32 = 3222163852u32;
pub const ERROR_PCW_BASE: u32 = 3222163713u32;
pub const ERROR_PCW_CANNOT_CREATE_TABLE: u32 = 3222163841u32;
pub const ERROR_PCW_CANNOT_RUN_MAKECAB: u32 = 3222163782u32;
pub const ERROR_PCW_CANNOT_WRITE_DDF: u32 = 3222163781u32;
pub const ERROR_PCW_CANT_COPY_FILE_TO_TEMP_FOLDER: u32 = 3222163771u32;
pub const ERROR_PCW_CANT_CREATE_ONE_PATCH_FILE: u32 = 3222163772u32;
pub const ERROR_PCW_CANT_CREATE_PATCH_FILE: u32 = 3222163718u32;
pub const ERROR_PCW_CANT_CREATE_SUMMARY_INFO: u32 = 3222163828u32;
pub const ERROR_PCW_CANT_CREATE_SUMMARY_INFO_POUND: u32 = 3222163830u32;
pub const ERROR_PCW_CANT_CREATE_TEMP_FOLDER: u32 = 3222163715u32;
pub const ERROR_PCW_CANT_DELETE_TEMP_FOLDER: u32 = 3222163974u32;
pub const ERROR_PCW_CANT_GENERATE_SEQUENCEINFO_MAJORUPGD: u32 = 3222163842u32;
pub const ERROR_PCW_CANT_GENERATE_TRANSFORM: u32 = 3222163827u32;
pub const ERROR_PCW_CANT_GENERATE_TRANSFORM_POUND: u32 = 3222163829u32;
pub const ERROR_PCW_CANT_OVERWRITE_PATCH: u32 = 3222163717u32;
pub const ERROR_PCW_CANT_READ_FILE: u32 = 3222163978u32;
pub const ERROR_PCW_CREATEFILE_LOG_FAILED: u32 = 3222163861u32;
pub const ERROR_PCW_DUPLICATE_SEQUENCE_RECORD: u32 = 3222163858u32;
pub const ERROR_PCW_DUP_IMAGE_FAMILY_NAME: u32 = 3222163749u32;
pub const ERROR_PCW_DUP_TARGET_IMAGE_NAME: u32 = 3222163737u32;
pub const ERROR_PCW_DUP_TARGET_IMAGE_PACKCODE: u32 = 3222163777u32;
pub const ERROR_PCW_DUP_UPGRADED_IMAGE_NAME: u32 = 3222163729u32;
pub const ERROR_PCW_DUP_UPGRADED_IMAGE_PACKCODE: u32 = 3222163795u32;
pub const ERROR_PCW_ERROR_WRITING_TO_LOG: u32 = 3222163864u32;
pub const ERROR_PCW_EXECUTE_VIEW: u32 = 3222163870u32;
pub const ERROR_PCW_EXTFILE_BAD_FAMILY_FIELD: u32 = 3222163756u32;
pub const ERROR_PCW_EXTFILE_BAD_IGNORE_LENGTHS: u32 = 3222163814u32;
pub const ERROR_PCW_EXTFILE_BAD_IGNORE_OFFSETS: u32 = 3222163812u32;
pub const ERROR_PCW_EXTFILE_BAD_RETAIN_OFFSETS: u32 = 3222163817u32;
pub const ERROR_PCW_EXTFILE_BLANK_FILE_TABLE_KEY: u32 = 3222163755u32;
pub const ERROR_PCW_EXTFILE_BLANK_PATH_TO_FILE: u32 = 3222163758u32;
pub const ERROR_PCW_EXTFILE_IGNORE_COUNT_MISMATCH: u32 = 3222163815u32;
pub const ERROR_PCW_EXTFILE_LONG_FILE_TABLE_KEY: u32 = 3222163754u32;
pub const ERROR_PCW_EXTFILE_LONG_IGNORE_LENGTHS: u32 = 3222163813u32;
pub const ERROR_PCW_EXTFILE_LONG_IGNORE_OFFSETS: u32 = 3222163811u32;
pub const ERROR_PCW_EXTFILE_LONG_PATH_TO_FILE: u32 = 3222163757u32;
pub const ERROR_PCW_EXTFILE_LONG_RETAIN_OFFSETS: u32 = 3222163816u32;
pub const ERROR_PCW_EXTFILE_MISSING_FILE: u32 = 3222163759u32;
pub const ERROR_PCW_FAILED_CREATE_TRANSFORM: u32 = 3222163973u32;
pub const ERROR_PCW_FAILED_EXPAND_PATH: u32 = 3222163872u32;
pub const ERROR_PCW_FAMILY_RANGE_BAD_RETAIN_LENGTHS: u32 = 3222163809u32;
pub const ERROR_PCW_FAMILY_RANGE_BAD_RETAIN_OFFSETS: u32 = 3222163806u32;
pub const ERROR_PCW_FAMILY_RANGE_BLANK_FILE_TABLE_KEY: u32 = 3222163803u32;
pub const ERROR_PCW_FAMILY_RANGE_BLANK_RETAIN_LENGTHS: u32 = 3222163808u32;
pub const ERROR_PCW_FAMILY_RANGE_BLANK_RETAIN_OFFSETS: u32 = 3222163805u32;
pub const ERROR_PCW_FAMILY_RANGE_COUNT_MISMATCH: u32 = 3222163810u32;
pub const ERROR_PCW_FAMILY_RANGE_LONG_FILE_TABLE_KEY: u32 = 3222163802u32;
pub const ERROR_PCW_FAMILY_RANGE_LONG_RETAIN_LENGTHS: u32 = 3222163807u32;
pub const ERROR_PCW_FAMILY_RANGE_LONG_RETAIN_OFFSETS: u32 = 3222163804u32;
pub const ERROR_PCW_FAMILY_RANGE_NAME_TOO_LONG: u32 = 3222163800u32;
pub const ERROR_PCW_IMAGE_FAMILY_NAME_TOO_LONG: u32 = 3222163747u32;
pub const ERROR_PCW_IMAGE_PATH_NOT_EXIST: u32 = 3222163988u32;
pub const ERROR_PCW_INTERNAL_ERROR: u32 = 3222163969u32;
pub const ERROR_PCW_INVALID_LOG_LEVEL: u32 = 3222163862u32;
pub const ERROR_PCW_INVALID_MAJOR_VERSION: u32 = 3222163990u32;
pub const ERROR_PCW_INVALID_PARAMETER: u32 = 3222163860u32;
pub const ERROR_PCW_INVALID_PATCHMETADATA_PROP: u32 = 3222163856u32;
pub const ERROR_PCW_INVALID_PATCH_TYPE_SEQUENCING: u32 = 3222163977u32;
pub const ERROR_PCW_INVALID_PCP_EXTERNALFILES: u32 = 3222163982u32;
pub const ERROR_PCW_INVALID_PCP_FAMILYFILERANGES: u32 = 3222163992u32;
pub const ERROR_PCW_INVALID_PCP_IMAGEFAMILIES: u32 = 3222163983u32;
pub const ERROR_PCW_INVALID_PCP_PATCHSEQUENCE: u32 = 3222163984u32;
pub const ERROR_PCW_INVALID_PCP_PROPERTIES: u32 = 3222163991u32;
pub const ERROR_PCW_INVALID_PCP_PROPERTY: u32 = 3222163970u32;
pub const ERROR_PCW_INVALID_PCP_TARGETFILES_OPTIONALDATA: u32 = 3222163985u32;
pub const ERROR_PCW_INVALID_PCP_TARGETIMAGES: u32 = 3222163971u32;
pub const ERROR_PCW_INVALID_PCP_UPGRADEDFILESTOIGNORE: u32 = 3222163980u32;
pub const ERROR_PCW_INVALID_PCP_UPGRADEDFILES_OPTIONALDATA: u32 = 3222163986u32;
pub const ERROR_PCW_INVALID_PCP_UPGRADEDIMAGES: u32 = 3222163981u32;
pub const ERROR_PCW_INVALID_RANGE_ELEMENT: u32 = 3222163989u32;
pub const ERROR_PCW_INVALID_SUPERCEDENCE: u32 = 3222163857u32;
pub const ERROR_PCW_INVALID_SUPERSEDENCE_VALUE: u32 = 3222163976u32;
pub const ERROR_PCW_INVALID_UI_LEVEL: u32 = 3222163863u32;
pub const ERROR_PCW_LAX_VALIDATION_FLAGS: u32 = 3222163972u32;
pub const ERROR_PCW_MAJOR_UPGD_WITHOUT_SEQUENCING: u32 = 3222163843u32;
pub const ERROR_PCW_MATCHED_PRODUCT_VERSIONS: u32 = 3222163837u32;
pub const ERROR_PCW_MISMATCHED_PRODUCT_CODES: u32 = 3222163779u32;
pub const ERROR_PCW_MISMATCHED_PRODUCT_VERSIONS: u32 = 3222163780u32;
pub const ERROR_PCW_MISSING_DIRECTORY_TABLE: u32 = 3222163975u32;
pub const ERROR_PCW_MISSING_PATCHMETADATA: u32 = 3222163987u32;
pub const ERROR_PCW_MISSING_PATCH_GUID: u32 = 3222163719u32;
pub const ERROR_PCW_MISSING_PATCH_PATH: u32 = 3222163716u32;
pub const ERROR_PCW_NO_UPGRADED_IMAGES_TO_PATCH: u32 = 3222163723u32;
pub const ERROR_PCW_NULL_PATCHFAMILY: u32 = 3222163850u32;
pub const ERROR_PCW_NULL_SEQUENCE_NUMBER: u32 = 3222163851u32;
pub const ERROR_PCW_OBSOLETION_WITH_MSI30: u32 = 3222163839u32;
pub const ERROR_PCW_OBSOLETION_WITH_PATCHSEQUENCE: u32 = 3222163840u32;
pub const ERROR_PCW_OBSOLETION_WITH_SEQUENCE_DATA: u32 = 3222163838u32;
pub const ERROR_PCW_OODS_COPYING_MSI: u32 = 3222163726u32;
pub const ERROR_PCW_OPEN_VIEW: u32 = 3222163869u32;
pub const ERROR_PCW_OUT_OF_MEMORY: u32 = 3222163865u32;
pub const ERROR_PCW_PATCHMETADATA_PROP_NOT_SET: u32 = 3222163855u32;
pub const ERROR_PCW_PCP_BAD_FORMAT: u32 = 3222163714u32;
pub const ERROR_PCW_PCP_DOESNT_EXIST: u32 = 3222163713u32;
pub const ERROR_PCW_SEQUENCING_BAD_TARGET: u32 = 3222163854u32;
pub const ERROR_PCW_TARGET_BAD_PROD_CODE_VAL: u32 = 3222163744u32;
pub const ERROR_PCW_TARGET_BAD_PROD_VALIDATE: u32 = 3222163743u32;
pub const ERROR_PCW_TARGET_IMAGE_COMPRESSED: u32 = 3222163742u32;
pub const ERROR_PCW_TARGET_IMAGE_NAME_TOO_LONG: u32 = 3222163735u32;
pub const ERROR_PCW_TARGET_IMAGE_PATH_EMPTY: u32 = 3222163739u32;
pub const ERROR_PCW_TARGET_IMAGE_PATH_NOT_EXIST: u32 = 3222163740u32;
pub const ERROR_PCW_TARGET_IMAGE_PATH_NOT_MSI: u32 = 3222163741u32;
pub const ERROR_PCW_TARGET_IMAGE_PATH_TOO_LONG: u32 = 3222163738u32;
pub const ERROR_PCW_TARGET_MISSING_SRC_FILES: u32 = 3222163746u32;
pub const ERROR_PCW_TARGET_WRONG_PRODUCT_VERSION_COMP: u32 = 3222163979u32;
pub const ERROR_PCW_TFILEDATA_BAD_IGNORE_LENGTHS: u32 = 3222163822u32;
pub const ERROR_PCW_TFILEDATA_BAD_IGNORE_OFFSETS: u32 = 3222163820u32;
pub const ERROR_PCW_TFILEDATA_BAD_RETAIN_OFFSETS: u32 = 3222163825u32;
pub const ERROR_PCW_TFILEDATA_BAD_TARGET_FIELD: u32 = 3222163791u32;
pub const ERROR_PCW_TFILEDATA_BLANK_FILE_TABLE_KEY: u32 = 3222163789u32;
pub const ERROR_PCW_TFILEDATA_IGNORE_COUNT_MISMATCH: u32 = 3222163823u32;
pub const ERROR_PCW_TFILEDATA_LONG_FILE_TABLE_KEY: u32 = 3222163788u32;
pub const ERROR_PCW_TFILEDATA_LONG_IGNORE_LENGTHS: u32 = 3222163821u32;
pub const ERROR_PCW_TFILEDATA_LONG_IGNORE_OFFSETS: u32 = 3222163819u32;
pub const ERROR_PCW_TFILEDATA_LONG_RETAIN_OFFSETS: u32 = 3222163824u32;
pub const ERROR_PCW_TFILEDATA_MISSING_FILE_TABLE_KEY: u32 = 3222163790u32;
pub const ERROR_PCW_UFILEDATA_BAD_UPGRADED_FIELD: u32 = 3222163778u32;
pub const ERROR_PCW_UFILEDATA_BLANK_FILE_TABLE_KEY: u32 = 3222163752u32;
pub const ERROR_PCW_UFILEDATA_LONG_FILE_TABLE_KEY: u32 = 3222163751u32;
pub const ERROR_PCW_UFILEDATA_MISSING_FILE_TABLE_KEY: u32 = 3222163753u32;
pub const ERROR_PCW_UFILEIGNORE_BAD_FILE_TABLE_KEY: u32 = 3222163799u32;
pub const ERROR_PCW_UFILEIGNORE_BAD_UPGRADED_FIELD: u32 = 3222163796u32;
pub const ERROR_PCW_UFILEIGNORE_BLANK_FILE_TABLE_KEY: u32 = 3222163798u32;
pub const ERROR_PCW_UFILEIGNORE_LONG_FILE_TABLE_KEY: u32 = 3222163797u32;
pub const ERROR_PCW_UNKNOWN_ERROR: u32 = 3222163866u32;
pub const ERROR_PCW_UNKNOWN_INFO: u32 = 3222163867u32;
pub const ERROR_PCW_UNKNOWN_WARN: u32 = 3222163868u32;
pub const ERROR_PCW_UPGRADED_IMAGE_COMPRESSED: u32 = 3222163734u32;
pub const ERROR_PCW_UPGRADED_IMAGE_NAME_TOO_LONG: u32 = 3222163727u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_NOT_EXIST: u32 = 3222163793u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_NOT_MSI: u32 = 3222163794u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATCH_PATH_TOO_LONG: u32 = 3222163792u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_EMPTY: u32 = 3222163731u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_NOT_EXIST: u32 = 3222163732u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_NOT_MSI: u32 = 3222163733u32;
pub const ERROR_PCW_UPGRADED_IMAGE_PATH_TOO_LONG: u32 = 3222163730u32;
pub const ERROR_PCW_UPGRADED_MISSING_SRC_FILES: u32 = 3222163745u32;
pub const ERROR_PCW_VIEW_FETCH: u32 = 3222163871u32;
pub const ERROR_PCW_WRITE_SUMMARY_PROPERTIES: u32 = 3222163787u32;
pub const ERROR_PCW_WRONG_PATCHMETADATA_STRD_PROP: u32 = 3222163859u32;
pub const ERROR_ROLLBACK_DISABLED: u32 = 1653u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FUSION_INSTALL_REFERENCE(i32);
pub const FUSION_REFCOUNT_FILEPATH_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2955910501,
    data2: 64375,
    data3: 20346,
    data4: [175, 165, 179, 145, 48, 159, 17, 201],
};
pub const FUSION_REFCOUNT_OPAQUE_STRING_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 784938083,
    data2: 45251,
    data3: 17889,
    data4: [131, 100, 50, 126, 150, 174, 168, 86],
};
pub const FUSION_REFCOUNT_UNINSTALL_SUBKEY_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2364391957,
    data2: 44107,
    data3: 18571,
    data4: [147, 192, 165, 10, 73, 203, 47, 184],
};
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_ALREADY_INSTALLED: u32 = 3u32;
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_INSTALLED: u32 = 1u32;
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_REFRESHED: u32 = 2u32;
pub const IASSEMBLYCACHEITEM_COMMIT_FLAG_REFRESH: u32 = 1u32;
#[repr(C)]
pub struct IASSEMBLYCACHE_UNINSTALL_DISPOSITION(i32);
#[repr(transparent)]
pub struct IAssemblyCache(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAssemblyCacheItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAssemblyName(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumMsmDependency(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumMsmError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumMsmString(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMsmDependencies(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMsmDependency(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMsmError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMsmErrors(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMsmGetFiles(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMsmMerge(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMsmStrings(pub *mut ::core::ffi::c_void);
pub const INFO_BASE: u32 = 3222229249u32;
pub const INFO_ENTERING_PHASE_I: u32 = 3222229251u32;
pub const INFO_ENTERING_PHASE_II: u32 = 3222229256u32;
pub const INFO_ENTERING_PHASE_III: u32 = 3222229257u32;
pub const INFO_ENTERING_PHASE_IV: u32 = 3222229258u32;
pub const INFO_ENTERING_PHASE_I_VALIDATION: u32 = 3222229250u32;
pub const INFO_ENTERING_PHASE_V: u32 = 3222229259u32;
pub const INFO_GENERATING_METADATA: u32 = 3222229265u32;
pub const INFO_PASSED_MAIN_CONTROL: u32 = 3222229249u32;
pub const INFO_PATCHCACHE_FILEINFO_FAILURE: u32 = 3222229267u32;
pub const INFO_PATCHCACHE_PCI_READFAILURE: u32 = 3222229268u32;
pub const INFO_PATCHCACHE_PCI_WRITEFAILURE: u32 = 3222229269u32;
pub const INFO_PCP_PATH: u32 = 3222229252u32;
pub const INFO_PROPERTY: u32 = 3222229255u32;
pub const INFO_SET_OPTIONS: u32 = 3222229254u32;
pub const INFO_SUCCESSFUL_PATCH_CREATION: u32 = 3222229271u32;
pub const INFO_TEMP_DIR: u32 = 3222229253u32;
pub const INFO_TEMP_DIR_CLEANUP: u32 = 3222229266u32;
pub const INFO_USING_USER_MSI_FOR_PATCH_TABLES: u32 = 3222229270u32;
#[repr(C)]
pub struct INSTALLFEATUREATTRIBUTE(i32);
#[repr(C)]
pub struct INSTALLLEVEL(i32);
#[repr(C)]
pub struct INSTALLLOGATTRIBUTES(i32);
#[repr(C)]
pub struct INSTALLMESSAGE(i32);
pub const INSTALLMESSAGE_TYPEMASK: i32 = -16777216i32;
#[repr(C)]
pub struct INSTALLMODE(i32);
#[repr(C)]
pub struct INSTALLOGMODE(i32);
#[repr(C)]
pub struct INSTALLSTATE(i32);
#[repr(C)]
pub struct INSTALLTYPE(i32);
#[repr(C)]
pub struct INSTALLUILEVEL(i32);
#[repr(C)]
pub struct INSTALLUI_HANDLERA(i32);
#[repr(C)]
pub struct INSTALLUI_HANDLERW(i32);
#[repr(transparent)]
pub struct IPMApplicationInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMApplicationInfoEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMBackgroundServiceAgentInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMBackgroundServiceAgentInfoEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMBackgroundWorkerInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMBackgroundWorkerInfoEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMDeploymentManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMEnumerationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMExtensionCachedFileUpdaterInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMExtensionContractInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMExtensionFileExtensionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMExtensionFileOpenPickerInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMExtensionFileSavePickerInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMExtensionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMExtensionInfoEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMExtensionProtocolInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMExtensionShareTargetInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMLiveTileJobInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMLiveTileJobInfoEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMTaskInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMTaskInfoEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMTileInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMTileInfoEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMTilePropertyEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPMTilePropertyInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IValidate(pub *mut ::core::ffi::c_void);
pub const LIBID_MsmMergeTypeLib: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 182298671, data2: 11302, data3: 4562, data4: [173, 101, 0, 160, 201, 175, 17, 166] };
pub const LOGALL: u32 = 15u32;
pub const LOGERR: u32 = 4u32;
pub const LOGINFO: u32 = 1u32;
pub const LOGNONE: u32 = 0u32;
pub const LOGPERFMESSAGES: u32 = 8u32;
pub const LOGTOKEN_NO_LOG: u32 = 1u32;
pub const LOGTOKEN_SETUPAPI_APPLOG: u32 = 2u32;
pub const LOGTOKEN_SETUPAPI_DEVLOG: u32 = 3u32;
pub const LOGTOKEN_TYPE_MASK: u32 = 3u32;
pub const LOGTOKEN_UNSPECIFIED: u32 = 0u32;
pub const LOGWARN: u32 = 2u32;
#[repr(C)]
pub struct LPDISPLAYVAL(i32);
#[repr(C)]
pub struct LPEVALCOMCALLBACK(i32);
pub const MAX_FEATURE_CHARS: u32 = 38u32;
pub const MAX_GUID_CHARS: u32 = 38u32;
#[repr(C)]
pub struct MSIADVERTISEOPTIONFLAGS(i32);
#[repr(C)]
pub struct MSIARCHITECTUREFLAGS(i32);
#[repr(C)]
pub struct MSIASSEMBLYINFO(i32);
#[repr(C)]
pub struct MSICODE(i32);
#[repr(C)]
pub struct MSICOLINFO(i32);
#[repr(C)]
pub struct MSICONDITION(i32);
#[repr(C)]
pub struct MSICOSTTREE(i32);
#[repr(C)]
pub struct MSIDBERROR(i32);
#[repr(C)]
pub struct MSIDBSTATE(i32);
#[repr(C)]
pub struct MSIFILEHASHINFO(i32);
#[repr(C)]
pub struct MSIHANDLE(i32);
#[repr(C)]
pub struct MSIINSTALLCONTEXT(i32);
#[repr(C)]
pub struct MSIMODIFY(i32);
#[repr(C)]
pub struct MSIOPENPACKAGEFLAGS(i32);
#[repr(C)]
pub struct MSIPATCHDATATYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MSIPATCHSEQUENCEINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MSIPATCHSEQUENCEINFOW(i32);
#[repr(C)]
pub struct MSIPATCHSTATE(i32);
#[repr(C)]
pub struct MSIRUNMODE(i32);
#[repr(C)]
pub struct MSISOURCETYPE(i32);
#[repr(C)]
pub struct MSITRANSACTION(i32);
#[repr(C)]
pub struct MSITRANSACTIONSTATE(i32);
#[repr(C)]
pub struct MSITRANSFORM_ERROR(i32);
#[repr(C)]
pub struct MSITRANSFORM_VALIDATE(i32);
pub const MSI_INVALID_HASH_IS_FATAL: u32 = 1u32;
pub const MSI_NULL_INTEGER: u32 = 2147483648u32;
#[repr(C)]
pub struct MsmMerge(i32);
#[repr(C)]
pub struct PACKMAN_RUNTIME(i32);
#[repr(C)]
pub struct PATCH_IGNORE_RANGE(i32);
#[repr(C)]
pub struct PATCH_INTERLEAVE_MAP(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PATCH_OLD_FILE_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PATCH_OLD_FILE_INFO_A(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PATCH_OLD_FILE_INFO_H(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PATCH_OLD_FILE_INFO_W(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PATCH_OPTION_DATA(i32);
pub const PATCH_OPTION_FAIL_IF_BIGGER: u32 = 1048576u32;
pub const PATCH_OPTION_FAIL_IF_SAME_FILE: u32 = 524288u32;
pub const PATCH_OPTION_INTERLEAVE_FILES: u32 = 1073741824u32;
pub const PATCH_OPTION_NO_BINDFIX: u32 = 65536u32;
pub const PATCH_OPTION_NO_CHECKSUM: u32 = 2097152u32;
pub const PATCH_OPTION_NO_LOCKFIX: u32 = 131072u32;
pub const PATCH_OPTION_NO_REBASE: u32 = 262144u32;
pub const PATCH_OPTION_NO_RESTIMEFIX: u32 = 4194304u32;
pub const PATCH_OPTION_NO_TIMESTAMP: u32 = 8388608u32;
pub const PATCH_OPTION_RESERVED1: u32 = 2147483648u32;
pub const PATCH_OPTION_SIGNATURE_MD5: u32 = 16777216u32;
pub const PATCH_OPTION_USE_BEST: u32 = 0u32;
pub const PATCH_OPTION_USE_LZX_A: u32 = 1u32;
pub const PATCH_OPTION_USE_LZX_B: u32 = 2u32;
pub const PATCH_OPTION_USE_LZX_BEST: u32 = 3u32;
pub const PATCH_OPTION_USE_LZX_LARGE: u32 = 4u32;
pub const PATCH_OPTION_VALID_FLAGS: u32 = 3237937159u32;
#[repr(C)]
pub struct PATCH_RETAIN_RANGE(i32);
pub const PATCH_SYMBOL_NO_FAILURES: u32 = 2u32;
pub const PATCH_SYMBOL_NO_IMAGEHLP: u32 = 1u32;
pub const PATCH_SYMBOL_RESERVED1: u32 = 2147483648u32;
pub const PATCH_SYMBOL_UNDECORATED_TOO: u32 = 4u32;
pub const PATCH_TRANSFORM_PE_IRELOC_2: u32 = 512u32;
pub const PATCH_TRANSFORM_PE_RESOURCE_2: u32 = 256u32;
pub const PID_APPNAME: u32 = 18u32;
pub const PID_AUTHOR: u32 = 4u32;
pub const PID_CHARCOUNT: u32 = 16u32;
pub const PID_COMMENTS: u32 = 6u32;
pub const PID_CREATE_DTM: u32 = 12u32;
pub const PID_EDITTIME: u32 = 10u32;
pub const PID_KEYWORDS: u32 = 5u32;
pub const PID_LASTAUTHOR: u32 = 8u32;
pub const PID_LASTPRINTED: u32 = 11u32;
pub const PID_LASTSAVE_DTM: u32 = 13u32;
pub const PID_MSIRESTRICT: u32 = 16u32;
pub const PID_MSISOURCE: u32 = 15u32;
pub const PID_MSIVERSION: u32 = 14u32;
pub const PID_PAGECOUNT: u32 = 14u32;
pub const PID_REVNUMBER: u32 = 9u32;
pub const PID_SUBJECT: u32 = 3u32;
pub const PID_TEMPLATE: u32 = 7u32;
pub const PID_THUMBNAIL: u32 = 17u32;
pub const PID_TITLE: u32 = 2u32;
pub const PID_WORDCOUNT: u32 = 15u32;
#[repr(C)]
pub struct PINSTALLUI_HANDLER_RECORD(i32);
#[repr(C)]
pub struct PMSIHANDLE(i32);
#[repr(C)]
pub struct PMSvc(i32);
#[repr(C)]
pub struct PM_ACTIVATION_POLICY(i32);
#[repr(C)]
pub struct PM_APPLICATION_HUBTYPE(i32);
#[repr(C)]
pub struct PM_APPLICATION_INSTALL_TYPE(i32);
#[repr(C)]
pub struct PM_APPLICATION_STATE(i32);
#[repr(C)]
pub struct PM_APP_GENRE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PM_BSATASKID(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PM_BWTASKID(i32);
#[repr(C)]
pub struct PM_ENUM_APP_FILTER(i32);
#[repr(C)]
pub struct PM_ENUM_BSA_FILTER(i32);
#[repr(C)]
pub struct PM_ENUM_BW_FILTER(i32);
#[repr(C)]
pub struct PM_ENUM_EXTENSION_FILTER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PM_ENUM_FILTER(i32);
#[repr(C)]
pub struct PM_ENUM_TASK_FILTER(i32);
#[repr(C)]
pub struct PM_ENUM_TILE_FILTER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PM_EXTENSIONCONSUMER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PM_INSTALLINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PM_INVOCATIONINFO(i32);
#[repr(C)]
pub struct PM_LIVETILE_RECURRENCE_TYPE(i32);
#[repr(C)]
pub struct PM_LOGO_SIZE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PM_STARTAPPBLOB(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PM_STARTTILEBLOB(i32);
#[repr(C)]
pub struct PM_STARTTILE_TYPE(i32);
#[repr(C)]
pub struct PM_TASK_TRANSITION(i32);
#[repr(C)]
pub struct PM_TASK_TYPE(i32);
#[repr(C)]
pub struct PM_TILE_HUBTYPE(i32);
#[repr(C)]
pub struct PM_TILE_SIZE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PM_UPDATEINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PM_UPDATEINFO_LEGACY(i32);
#[repr(C)]
pub struct PPATCH_PROGRESS_CALLBACK(i32);
#[repr(C)]
pub struct PPATCH_SYMLOAD_CALLBACK(i32);
#[repr(C)]
pub struct PROTECTED_FILE_DATA(i32);
#[repr(C)]
pub struct QUERYASMINFO_FLAGS(i32);
#[repr(C)]
pub struct REINSTALLMODE(i32);
#[repr(C)]
pub struct RESULTTYPES(i32);
#[repr(C)]
pub struct SCRIPTFLAGS(i32);
pub const SFC_DISABLE_ASK: u32 = 1u32;
pub const SFC_DISABLE_NOPOPUPS: u32 = 4u32;
pub const SFC_DISABLE_NORMAL: u32 = 0u32;
pub const SFC_DISABLE_ONCE: u32 = 2u32;
pub const SFC_DISABLE_SETUP: u32 = 3u32;
pub const SFC_QUOTA_DEFAULT: u32 = 50u32;
pub const SFC_SCAN_ALWAYS: u32 = 1u32;
pub const SFC_SCAN_IMMEDIATE: u32 = 3u32;
pub const SFC_SCAN_NORMAL: u32 = 0u32;
pub const SFC_SCAN_ONCE: u32 = 2u32;
#[repr(C)]
pub struct STATUSTYPES(i32);
pub const STREAM_FORMAT_COMPLIB_MANIFEST: u32 = 1u32;
pub const STREAM_FORMAT_COMPLIB_MODULE: u32 = 0u32;
pub const STREAM_FORMAT_WIN32_MANIFEST: u32 = 4u32;
pub const STREAM_FORMAT_WIN32_MODULE: u32 = 2u32;
#[repr(C)]
pub struct TILE_TEMPLATE_TYPE(i32);
pub const TXTLOG_BACKUP: u32 = 128u32;
pub const TXTLOG_CMI: u32 = 268435456u32;
pub const TXTLOG_COPYFILES: u32 = 8u32;
pub const TXTLOG_DEPTH_DECR: u32 = 262144u32;
pub const TXTLOG_DEPTH_INCR: u32 = 131072u32;
pub const TXTLOG_DETAILS: u32 = 5u32;
pub const TXTLOG_DEVINST: u32 = 1u32;
pub const TXTLOG_DEVMGR: u32 = 536870912u32;
pub const TXTLOG_DRIVER_STORE: u32 = 67108864u32;
pub const TXTLOG_DRVSETUP: u32 = 4194304u32;
pub const TXTLOG_ERROR: u32 = 1u32;
pub const TXTLOG_FILEQ: u32 = 4u32;
pub const TXTLOG_FLUSH_FILE: u32 = 1048576u32;
pub const TXTLOG_INF: u32 = 2u32;
pub const TXTLOG_INFDB: u32 = 1024u32;
pub const TXTLOG_INSTALLER: u32 = 1073741824u32;
pub const TXTLOG_NEWDEV: u32 = 16777216u32;
pub const TXTLOG_POLICY: u32 = 8388608u32;
pub const TXTLOG_RESERVED_FLAGS: u32 = 65520u32;
pub const TXTLOG_SETUP: u32 = 134217728u32;
pub const TXTLOG_SETUPAPI_BITS: u32 = 3u32;
pub const TXTLOG_SETUPAPI_CMDLINE: u32 = 2u32;
pub const TXTLOG_SETUPAPI_DEVLOG: u32 = 1u32;
pub const TXTLOG_SIGVERIF: u32 = 32u32;
pub const TXTLOG_SUMMARY: u32 = 4u32;
pub const TXTLOG_SYSTEM_STATE_CHANGE: u32 = 3u32;
pub const TXTLOG_TAB_1: u32 = 524288u32;
pub const TXTLOG_TIMESTAMP: u32 = 65536u32;
pub const TXTLOG_UI: u32 = 256u32;
pub const TXTLOG_UMPNPMGR: u32 = 33554432u32;
pub const TXTLOG_UTIL: u32 = 512u32;
pub const TXTLOG_VENDOR: u32 = 2147483648u32;
pub const TXTLOG_VERBOSE: u32 = 6u32;
pub const TXTLOG_VERY_VERBOSE: u32 = 7u32;
pub const TXTLOG_WARNING: u32 = 2u32;
pub const UIALL: u32 = 32768u32;
pub const UILOGBITS: u32 = 15u32;
pub const UINONE: u32 = 0u32;
#[repr(C)]
pub struct USERINFOSTATE(i32);
pub const WARN_BAD_MAJOR_VERSION: u32 = 3222294792u32;
pub const WARN_BASE: u32 = 3222294785u32;
pub const WARN_EQUAL_FILE_VERSION: u32 = 3222294794u32;
pub const WARN_FILE_VERSION_DOWNREV: u32 = 3222294793u32;
pub const WARN_IMPROPER_TRANSFORM_VALIDATION: u32 = 3222294788u32;
pub const WARN_INVALID_TRANSFORM_VALIDATION: u32 = 3222294791u32;
pub const WARN_MAJOR_UPGRADE_PATCH: u32 = 3222294785u32;
pub const WARN_OBSOLETION_WITH_MSI30: u32 = 3222294801u32;
pub const WARN_OBSOLETION_WITH_PATCHSEQUENCE: u32 = 3222294803u32;
pub const WARN_OBSOLETION_WITH_SEQUENCE_DATA: u32 = 3222294802u32;
pub const WARN_PATCHPROPERTYNOTSET: u32 = 3222294795u32;
pub const WARN_PCW_MISMATCHED_PRODUCT_CODES: u32 = 3222294789u32;
pub const WARN_PCW_MISMATCHED_PRODUCT_VERSIONS: u32 = 3222294790u32;
pub const WARN_SEQUENCE_DATA_GENERATION_DISABLED: u32 = 3222294786u32;
pub const WARN_SEQUENCE_DATA_SUPERSEDENCE_IGNORED: u32 = 3222294787u32;
pub const _WIN32_MSI: u32 = 500u32;
pub const _WIN32_MSM: u32 = 100u32;
#[repr(C)]
pub struct _tagAPPTASKTYPE(i32);
pub const cchMaxInteger: i32 = 12i32;
#[repr(C)]
pub struct msidbAssemblyAttributes(i32);
#[repr(C)]
pub struct msidbClassAttributes(i32);
#[repr(C)]
pub struct msidbComponentAttributes(i32);
#[repr(C)]
pub struct msidbControlAttributes(i32);
#[repr(C)]
pub struct msidbCustomActionType(i32);
#[repr(C)]
pub struct msidbDialogAttributes(i32);
#[repr(C)]
pub struct msidbEmbeddedUIAttributes(i32);
#[repr(C)]
pub struct msidbFeatureAttributes(i32);
#[repr(C)]
pub struct msidbFileAttributes(i32);
#[repr(C)]
pub struct msidbIniFileAction(i32);
#[repr(C)]
pub struct msidbLocatorType(i32);
#[repr(C)]
pub struct msidbMoveFileOptions(i32);
#[repr(C)]
pub struct msidbODBCDataSourceRegistration(i32);
#[repr(C)]
pub struct msidbPatchAttributes(i32);
#[repr(C)]
pub struct msidbRegistryRoot(i32);
#[repr(C)]
pub struct msidbRemoveFileInstallMode(i32);
#[repr(C)]
pub struct msidbServiceConfigEvent(i32);
#[repr(C)]
pub struct msidbServiceControlEvent(i32);
#[repr(C)]
pub struct msidbServiceInstallErrorControl(i32);
#[repr(C)]
pub struct msidbSumInfoSourceType(i32);
#[repr(C)]
pub struct msidbTextStyleStyleBits(i32);
#[repr(C)]
pub struct msidbUpgradeAttributes(i32);
#[repr(C)]
pub struct msifiFastInstallBits(i32);
#[repr(C)]
pub struct msirbRebootReason(i32);
#[repr(C)]
pub struct msirbRebootType(i32);
#[repr(C)]
pub struct msmErrorType(i32);
