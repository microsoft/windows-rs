#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
    pub fn MsiSetExternalUIA(puihandler: INSTALLUI_HANDLERA, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void) -> INSTALLUI_HANDLERA;
    pub fn MsiSetExternalUIRecord(puihandler: PINSTALLUI_HANDLER_RECORD, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void, ppuiprevhandler: PINSTALLUI_HANDLER_RECORD) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsiSetExternalUIW(puihandler: INSTALLUI_HANDLERW, dwmessagefilter: u32, pvcontext: *const ::core::ffi::c_void) -> INSTALLUI_HANDLERW;
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
#[repr(C)]
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
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_UNKNOWN: i32 = 0i32;
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_OS: i32 = 1i32;
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MITIGATION: i32 = 2i32;
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MAXVERSIONTESTED: i32 = 3i32;
pub const ACTCTX_RUN_LEVEL_UNSPECIFIED: i32 = 0i32;
pub const ACTCTX_RUN_LEVEL_AS_INVOKER: i32 = 1i32;
pub const ACTCTX_RUN_LEVEL_HIGHEST_AVAILABLE: i32 = 2i32;
pub const ACTCTX_RUN_LEVEL_REQUIRE_ADMIN: i32 = 3i32;
pub const ACTCTX_RUN_LEVEL_NUMBERS: i32 = 4i32;
#[repr(C)]
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
pub const ADVERTISEFLAGS_MACHINEASSIGN: i32 = 0i32;
pub const ADVERTISEFLAGS_USERASSIGN: i32 = 1i32;
pub const APPLY_OPTION_FAIL_IF_CLOSE: u32 = 2u32;
pub const APPLY_OPTION_FAIL_IF_EXACT: u32 = 1u32;
pub const APPLY_OPTION_TEST_ONLY: u32 = 4u32;
pub const APPLY_OPTION_VALID_FLAGS: u32 = 7u32;
pub const ASM_BINDF_FORCE_CACHE_INSTALL: u32 = 1u32;
pub const ASM_BINDF_RFS_INTEGRITY_CHECK: u32 = 2u32;
pub const ASM_BINDF_RFS_MODULE_CHECK: u32 = 4u32;
pub const ASM_BINDF_BINPATH_PROBE_ONLY: u32 = 8u32;
pub const ASM_BINDF_SHARED_BINPATH_HINT: u32 = 16u32;
pub const ASM_BINDF_PARENT_ASM_HINT: u32 = 32u32;
pub const ASM_CMPF_NAME: i32 = 1i32;
pub const ASM_CMPF_MAJOR_VERSION: i32 = 2i32;
pub const ASM_CMPF_MINOR_VERSION: i32 = 4i32;
pub const ASM_CMPF_BUILD_NUMBER: i32 = 8i32;
pub const ASM_CMPF_REVISION_NUMBER: i32 = 16i32;
pub const ASM_CMPF_PUBLIC_KEY_TOKEN: i32 = 32i32;
pub const ASM_CMPF_CULTURE: i32 = 64i32;
pub const ASM_CMPF_CUSTOM: i32 = 128i32;
pub const ASM_CMPF_ALL: i32 = 255i32;
pub const ASM_CMPF_DEFAULT: i32 = 256i32;
pub const ASM_DISPLAYF_VERSION: i32 = 1i32;
pub const ASM_DISPLAYF_CULTURE: i32 = 2i32;
pub const ASM_DISPLAYF_PUBLIC_KEY_TOKEN: i32 = 4i32;
pub const ASM_DISPLAYF_PUBLIC_KEY: i32 = 8i32;
pub const ASM_DISPLAYF_CUSTOM: i32 = 16i32;
pub const ASM_DISPLAYF_PROCESSORARCHITECTURE: i32 = 32i32;
pub const ASM_DISPLAYF_LANGUAGEID: i32 = 64i32;
pub const ASM_NAME_PUBLIC_KEY: i32 = 0i32;
pub const ASM_NAME_PUBLIC_KEY_TOKEN: i32 = 1i32;
pub const ASM_NAME_HASH_VALUE: i32 = 2i32;
pub const ASM_NAME_NAME: i32 = 3i32;
pub const ASM_NAME_MAJOR_VERSION: i32 = 4i32;
pub const ASM_NAME_MINOR_VERSION: i32 = 5i32;
pub const ASM_NAME_BUILD_NUMBER: i32 = 6i32;
pub const ASM_NAME_REVISION_NUMBER: i32 = 7i32;
pub const ASM_NAME_CULTURE: i32 = 8i32;
pub const ASM_NAME_PROCESSOR_ID_ARRAY: i32 = 9i32;
pub const ASM_NAME_OSINFO_ARRAY: i32 = 10i32;
pub const ASM_NAME_HASH_ALGID: i32 = 11i32;
pub const ASM_NAME_ALIAS: i32 = 12i32;
pub const ASM_NAME_CODEBASE_URL: i32 = 13i32;
pub const ASM_NAME_CODEBASE_LASTMOD: i32 = 14i32;
pub const ASM_NAME_NULL_PUBLIC_KEY: i32 = 15i32;
pub const ASM_NAME_NULL_PUBLIC_KEY_TOKEN: i32 = 16i32;
pub const ASM_NAME_CUSTOM: i32 = 17i32;
pub const ASM_NAME_NULL_CUSTOM: i32 = 18i32;
pub const ASM_NAME_MVID: i32 = 19i32;
pub const ASM_NAME_MAX_PARAMS: i32 = 20i32;
pub const ASSEMBLYINFO_FLAG_INSTALLED: u32 = 1u32;
pub const ASSEMBLYINFO_FLAG_PAYLOADRESIDENT: u32 = 2u32;
#[repr(C)]
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
pub const CANOF_PARSE_DISPLAY_NAME: i32 = 1i32;
pub const CANOF_SET_DEFAULT_VALUES: i32 = 2i32;
pub const DEFAULT_DISK_ID: u32 = 2u32;
pub const DEFAULT_FILE_SEQUENCE_START: u32 = 2u32;
pub const DEFAULT_MINIMUM_REQUIRED_MSI_VERSION: u32 = 100u32;
#[repr(C)]
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
pub const DELTA_MAX_HASH_SIZE: u32 = 32u32;
#[repr(C)]
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
#[repr(C)]
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
pub const FUSION_REFCOUNT_FILEPATH_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2955910501,
    data2: 64375,
    data3: 20346,
    data4: [175, 165, 179, 145, 48, 159, 17, 201],
};
pub const FUSION_REFCOUNT_OPAQUE_STRING_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 784938083,
    data2: 45251,
    data3: 17889,
    data4: [131, 100, 50, 126, 150, 174, 168, 86],
};
pub const FUSION_REFCOUNT_UNINSTALL_SUBKEY_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2364391957,
    data2: 44107,
    data3: 18571,
    data4: [147, 192, 165, 10, 73, 203, 47, 184],
};
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_ALREADY_INSTALLED: u32 = 3u32;
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_INSTALLED: u32 = 1u32;
pub const IASSEMBLYCACHEITEM_COMMIT_DISPOSITION_REFRESHED: u32 = 2u32;
pub const IASSEMBLYCACHEITEM_COMMIT_FLAG_REFRESH: u32 = 1u32;
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_UNINSTALLED: u32 = 1u32;
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_STILL_IN_USE: u32 = 2u32;
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_ALREADY_UNINSTALLED: u32 = 3u32;
pub const IASSEMBLYCACHE_UNINSTALL_DISPOSITION_DELETE_PENDING: u32 = 4u32;
#[repr(transparent)]
pub struct IAssemblyCache(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAssemblyCache {}
impl ::core::clone::Clone for IAssemblyCache {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAssemblyCacheItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAssemblyCacheItem {}
impl ::core::clone::Clone for IAssemblyCacheItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAssemblyName(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAssemblyName {}
impl ::core::clone::Clone for IAssemblyName {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumMsmDependency(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumMsmDependency {}
impl ::core::clone::Clone for IEnumMsmDependency {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumMsmError(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumMsmError {}
impl ::core::clone::Clone for IEnumMsmError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumMsmString(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumMsmString {}
impl ::core::clone::Clone for IEnumMsmString {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMsmDependencies(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMsmDependencies {}
impl ::core::clone::Clone for IMsmDependencies {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMsmDependency(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMsmDependency {}
impl ::core::clone::Clone for IMsmDependency {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMsmError(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMsmError {}
impl ::core::clone::Clone for IMsmError {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMsmErrors(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMsmErrors {}
impl ::core::clone::Clone for IMsmErrors {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMsmGetFiles(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMsmGetFiles {}
impl ::core::clone::Clone for IMsmGetFiles {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMsmMerge(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMsmMerge {}
impl ::core::clone::Clone for IMsmMerge {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMsmStrings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMsmStrings {}
impl ::core::clone::Clone for IMsmStrings {
    fn clone(&self) -> Self {
        *self
    }
}
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
pub const INSTALLFEATUREATTRIBUTE_FAVORLOCAL: i32 = 1i32;
pub const INSTALLFEATUREATTRIBUTE_FAVORSOURCE: i32 = 2i32;
pub const INSTALLFEATUREATTRIBUTE_FOLLOWPARENT: i32 = 4i32;
pub const INSTALLFEATUREATTRIBUTE_FAVORADVERTISE: i32 = 8i32;
pub const INSTALLFEATUREATTRIBUTE_DISALLOWADVERTISE: i32 = 16i32;
pub const INSTALLFEATUREATTRIBUTE_NOUNSUPPORTEDADVERTISE: i32 = 32i32;
pub const INSTALLLEVEL_DEFAULT: i32 = 0i32;
pub const INSTALLLEVEL_MINIMUM: i32 = 1i32;
pub const INSTALLLEVEL_MAXIMUM: i32 = 65535i32;
pub const INSTALLLOGATTRIBUTES_APPEND: i32 = 1i32;
pub const INSTALLLOGATTRIBUTES_FLUSHEACHLINE: i32 = 2i32;
pub const INSTALLMESSAGE_FATALEXIT: i32 = 0i32;
pub const INSTALLMESSAGE_ERROR: i32 = 16777216i32;
pub const INSTALLMESSAGE_WARNING: i32 = 33554432i32;
pub const INSTALLMESSAGE_USER: i32 = 50331648i32;
pub const INSTALLMESSAGE_INFO: i32 = 67108864i32;
pub const INSTALLMESSAGE_FILESINUSE: i32 = 83886080i32;
pub const INSTALLMESSAGE_RESOLVESOURCE: i32 = 100663296i32;
pub const INSTALLMESSAGE_OUTOFDISKSPACE: i32 = 117440512i32;
pub const INSTALLMESSAGE_ACTIONSTART: i32 = 134217728i32;
pub const INSTALLMESSAGE_ACTIONDATA: i32 = 150994944i32;
pub const INSTALLMESSAGE_PROGRESS: i32 = 167772160i32;
pub const INSTALLMESSAGE_COMMONDATA: i32 = 184549376i32;
pub const INSTALLMESSAGE_INITIALIZE: i32 = 201326592i32;
pub const INSTALLMESSAGE_TERMINATE: i32 = 218103808i32;
pub const INSTALLMESSAGE_SHOWDIALOG: i32 = 234881024i32;
pub const INSTALLMESSAGE_PERFORMANCE: i32 = 251658240i32;
pub const INSTALLMESSAGE_RMFILESINUSE: i32 = 419430400i32;
pub const INSTALLMESSAGE_INSTALLSTART: i32 = 436207616i32;
pub const INSTALLMESSAGE_INSTALLEND: i32 = 452984832i32;
pub const INSTALLMESSAGE_TYPEMASK: i32 = -16777216i32;
pub const INSTALLMODE_NODETECTION_ANY: i32 = -4i32;
pub const INSTALLMODE_NOSOURCERESOLUTION: i32 = -3i32;
pub const INSTALLMODE_NODETECTION: i32 = -2i32;
pub const INSTALLMODE_EXISTING: i32 = -1i32;
pub const INSTALLMODE_DEFAULT: i32 = 0i32;
pub const INSTALLLOGMODE_FATALEXIT: i32 = 1i32;
pub const INSTALLLOGMODE_ERROR: i32 = 2i32;
pub const INSTALLLOGMODE_WARNING: i32 = 4i32;
pub const INSTALLLOGMODE_USER: i32 = 8i32;
pub const INSTALLLOGMODE_INFO: i32 = 16i32;
pub const INSTALLLOGMODE_RESOLVESOURCE: i32 = 64i32;
pub const INSTALLLOGMODE_OUTOFDISKSPACE: i32 = 128i32;
pub const INSTALLLOGMODE_ACTIONSTART: i32 = 256i32;
pub const INSTALLLOGMODE_ACTIONDATA: i32 = 512i32;
pub const INSTALLLOGMODE_COMMONDATA: i32 = 2048i32;
pub const INSTALLLOGMODE_PROPERTYDUMP: i32 = 1024i32;
pub const INSTALLLOGMODE_VERBOSE: i32 = 4096i32;
pub const INSTALLLOGMODE_EXTRADEBUG: i32 = 8192i32;
pub const INSTALLLOGMODE_LOGONLYONERROR: i32 = 16384i32;
pub const INSTALLLOGMODE_LOGPERFORMANCE: i32 = 32768i32;
pub const INSTALLLOGMODE_PROGRESS: i32 = 1024i32;
pub const INSTALLLOGMODE_INITIALIZE: i32 = 4096i32;
pub const INSTALLLOGMODE_TERMINATE: i32 = 8192i32;
pub const INSTALLLOGMODE_SHOWDIALOG: i32 = 16384i32;
pub const INSTALLLOGMODE_FILESINUSE: i32 = 32i32;
pub const INSTALLLOGMODE_RMFILESINUSE: i32 = 33554432i32;
pub const INSTALLLOGMODE_INSTALLSTART: i32 = 67108864i32;
pub const INSTALLLOGMODE_INSTALLEND: i32 = 134217728i32;
pub const INSTALLSTATE_NOTUSED: i32 = -7i32;
pub const INSTALLSTATE_BADCONFIG: i32 = -6i32;
pub const INSTALLSTATE_INCOMPLETE: i32 = -5i32;
pub const INSTALLSTATE_SOURCEABSENT: i32 = -4i32;
pub const INSTALLSTATE_MOREDATA: i32 = -3i32;
pub const INSTALLSTATE_INVALIDARG: i32 = -2i32;
pub const INSTALLSTATE_UNKNOWN: i32 = -1i32;
pub const INSTALLSTATE_BROKEN: i32 = 0i32;
pub const INSTALLSTATE_ADVERTISED: i32 = 1i32;
pub const INSTALLSTATE_REMOVED: i32 = 1i32;
pub const INSTALLSTATE_ABSENT: i32 = 2i32;
pub const INSTALLSTATE_LOCAL: i32 = 3i32;
pub const INSTALLSTATE_SOURCE: i32 = 4i32;
pub const INSTALLSTATE_DEFAULT: i32 = 5i32;
pub const INSTALLTYPE_DEFAULT: i32 = 0i32;
pub const INSTALLTYPE_NETWORK_IMAGE: i32 = 1i32;
pub const INSTALLTYPE_SINGLE_INSTANCE: i32 = 2i32;
pub const INSTALLUILEVEL_NOCHANGE: i32 = 0i32;
pub const INSTALLUILEVEL_DEFAULT: i32 = 1i32;
pub const INSTALLUILEVEL_NONE: i32 = 2i32;
pub const INSTALLUILEVEL_BASIC: i32 = 3i32;
pub const INSTALLUILEVEL_REDUCED: i32 = 4i32;
pub const INSTALLUILEVEL_FULL: i32 = 5i32;
pub const INSTALLUILEVEL_ENDDIALOG: i32 = 128i32;
pub const INSTALLUILEVEL_PROGRESSONLY: i32 = 64i32;
pub const INSTALLUILEVEL_HIDECANCEL: i32 = 32i32;
pub const INSTALLUILEVEL_SOURCERESONLY: i32 = 256i32;
pub const INSTALLUILEVEL_UACONLY: i32 = 512i32;
#[cfg(feature = "Win32_Foundation")]
pub type INSTALLUI_HANDLERA = unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, szmessage: super::super::Foundation::PSTR) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type INSTALLUI_HANDLERW = unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, szmessage: super::super::Foundation::PWSTR) -> i32;
#[repr(transparent)]
pub struct IPMApplicationInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMApplicationInfo {}
impl ::core::clone::Clone for IPMApplicationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMApplicationInfoEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMApplicationInfoEnumerator {}
impl ::core::clone::Clone for IPMApplicationInfoEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMBackgroundServiceAgentInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMBackgroundServiceAgentInfo {}
impl ::core::clone::Clone for IPMBackgroundServiceAgentInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMBackgroundServiceAgentInfoEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMBackgroundServiceAgentInfoEnumerator {}
impl ::core::clone::Clone for IPMBackgroundServiceAgentInfoEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMBackgroundWorkerInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMBackgroundWorkerInfo {}
impl ::core::clone::Clone for IPMBackgroundWorkerInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMBackgroundWorkerInfoEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMBackgroundWorkerInfoEnumerator {}
impl ::core::clone::Clone for IPMBackgroundWorkerInfoEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMDeploymentManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMDeploymentManager {}
impl ::core::clone::Clone for IPMDeploymentManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMEnumerationManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMEnumerationManager {}
impl ::core::clone::Clone for IPMEnumerationManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMExtensionCachedFileUpdaterInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMExtensionCachedFileUpdaterInfo {}
impl ::core::clone::Clone for IPMExtensionCachedFileUpdaterInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMExtensionContractInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMExtensionContractInfo {}
impl ::core::clone::Clone for IPMExtensionContractInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMExtensionFileExtensionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMExtensionFileExtensionInfo {}
impl ::core::clone::Clone for IPMExtensionFileExtensionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMExtensionFileOpenPickerInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMExtensionFileOpenPickerInfo {}
impl ::core::clone::Clone for IPMExtensionFileOpenPickerInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMExtensionFileSavePickerInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMExtensionFileSavePickerInfo {}
impl ::core::clone::Clone for IPMExtensionFileSavePickerInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMExtensionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMExtensionInfo {}
impl ::core::clone::Clone for IPMExtensionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMExtensionInfoEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMExtensionInfoEnumerator {}
impl ::core::clone::Clone for IPMExtensionInfoEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMExtensionProtocolInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMExtensionProtocolInfo {}
impl ::core::clone::Clone for IPMExtensionProtocolInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMExtensionShareTargetInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMExtensionShareTargetInfo {}
impl ::core::clone::Clone for IPMExtensionShareTargetInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMLiveTileJobInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMLiveTileJobInfo {}
impl ::core::clone::Clone for IPMLiveTileJobInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMLiveTileJobInfoEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMLiveTileJobInfoEnumerator {}
impl ::core::clone::Clone for IPMLiveTileJobInfoEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMTaskInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMTaskInfo {}
impl ::core::clone::Clone for IPMTaskInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMTaskInfoEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMTaskInfoEnumerator {}
impl ::core::clone::Clone for IPMTaskInfoEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMTileInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMTileInfo {}
impl ::core::clone::Clone for IPMTileInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMTileInfoEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMTileInfoEnumerator {}
impl ::core::clone::Clone for IPMTileInfoEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMTilePropertyEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMTilePropertyEnumerator {}
impl ::core::clone::Clone for IPMTilePropertyEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPMTilePropertyInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPMTilePropertyInfo {}
impl ::core::clone::Clone for IPMTilePropertyInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IValidate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IValidate {}
impl ::core::clone::Clone for IValidate {
    fn clone(&self) -> Self {
        *self
    }
}
pub const LIBID_MsmMergeTypeLib: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 182298671, data2: 11302, data3: 4562, data4: [173, 101, 0, 160, 201, 175, 17, 166] };
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
#[cfg(feature = "Win32_Foundation")]
pub type LPDISPLAYVAL = unsafe extern "system" fn(pcontext: *mut ::core::ffi::c_void, uitype: RESULTTYPES, szwval: super::super::Foundation::PWSTR, szwdescription: super::super::Foundation::PWSTR, szwlocation: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type LPEVALCOMCALLBACK = unsafe extern "system" fn(istatus: STATUSTYPES, szdata: super::super::Foundation::PWSTR, pcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
pub const MAX_FEATURE_CHARS: u32 = 38u32;
pub const MAX_GUID_CHARS: u32 = 38u32;
pub const MSIADVERTISEOPTIONFLAGS_INSTANCE: i32 = 1i32;
pub const MSIARCHITECTUREFLAGS_X86: i32 = 1i32;
pub const MSIARCHITECTUREFLAGS_IA64: i32 = 2i32;
pub const MSIARCHITECTUREFLAGS_AMD64: i32 = 4i32;
pub const MSIARCHITECTUREFLAGS_ARM: i32 = 8i32;
pub const MSIASSEMBLYINFO_NETASSEMBLY: u32 = 0u32;
pub const MSIASSEMBLYINFO_WIN32ASSEMBLY: u32 = 1u32;
pub const MSICODE_PRODUCT: i32 = 0i32;
pub const MSICODE_PATCH: i32 = 1073741824i32;
pub const MSICOLINFO_NAMES: i32 = 0i32;
pub const MSICOLINFO_TYPES: i32 = 1i32;
pub const MSICONDITION_FALSE: i32 = 0i32;
pub const MSICONDITION_TRUE: i32 = 1i32;
pub const MSICONDITION_NONE: i32 = 2i32;
pub const MSICONDITION_ERROR: i32 = 3i32;
pub const MSICOSTTREE_SELFONLY: i32 = 0i32;
pub const MSICOSTTREE_CHILDREN: i32 = 1i32;
pub const MSICOSTTREE_PARENTS: i32 = 2i32;
pub const MSICOSTTREE_RESERVED: i32 = 3i32;
pub const MSIDBERROR_INVALIDARG: i32 = -3i32;
pub const MSIDBERROR_MOREDATA: i32 = -2i32;
pub const MSIDBERROR_FUNCTIONERROR: i32 = -1i32;
pub const MSIDBERROR_NOERROR: i32 = 0i32;
pub const MSIDBERROR_DUPLICATEKEY: i32 = 1i32;
pub const MSIDBERROR_REQUIRED: i32 = 2i32;
pub const MSIDBERROR_BADLINK: i32 = 3i32;
pub const MSIDBERROR_OVERFLOW: i32 = 4i32;
pub const MSIDBERROR_UNDERFLOW: i32 = 5i32;
pub const MSIDBERROR_NOTINSET: i32 = 6i32;
pub const MSIDBERROR_BADVERSION: i32 = 7i32;
pub const MSIDBERROR_BADCASE: i32 = 8i32;
pub const MSIDBERROR_BADGUID: i32 = 9i32;
pub const MSIDBERROR_BADWILDCARD: i32 = 10i32;
pub const MSIDBERROR_BADIDENTIFIER: i32 = 11i32;
pub const MSIDBERROR_BADLANGUAGE: i32 = 12i32;
pub const MSIDBERROR_BADFILENAME: i32 = 13i32;
pub const MSIDBERROR_BADPATH: i32 = 14i32;
pub const MSIDBERROR_BADCONDITION: i32 = 15i32;
pub const MSIDBERROR_BADFORMATTED: i32 = 16i32;
pub const MSIDBERROR_BADTEMPLATE: i32 = 17i32;
pub const MSIDBERROR_BADDEFAULTDIR: i32 = 18i32;
pub const MSIDBERROR_BADREGPATH: i32 = 19i32;
pub const MSIDBERROR_BADCUSTOMSOURCE: i32 = 20i32;
pub const MSIDBERROR_BADPROPERTY: i32 = 21i32;
pub const MSIDBERROR_MISSINGDATA: i32 = 22i32;
pub const MSIDBERROR_BADCATEGORY: i32 = 23i32;
pub const MSIDBERROR_BADKEYTABLE: i32 = 24i32;
pub const MSIDBERROR_BADMAXMINVALUES: i32 = 25i32;
pub const MSIDBERROR_BADCABINET: i32 = 26i32;
pub const MSIDBERROR_BADSHORTCUT: i32 = 27i32;
pub const MSIDBERROR_STRINGOVERFLOW: i32 = 28i32;
pub const MSIDBERROR_BADLOCALIZEATTRIB: i32 = 29i32;
pub const MSIDBSTATE_ERROR: i32 = -1i32;
pub const MSIDBSTATE_READ: i32 = 0i32;
pub const MSIDBSTATE_WRITE: i32 = 1i32;
#[repr(C)]
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
pub const MSIINSTALLCONTEXT_FIRSTVISIBLE: i32 = 0i32;
pub const MSIINSTALLCONTEXT_NONE: i32 = 0i32;
pub const MSIINSTALLCONTEXT_USERMANAGED: i32 = 1i32;
pub const MSIINSTALLCONTEXT_USERUNMANAGED: i32 = 2i32;
pub const MSIINSTALLCONTEXT_MACHINE: i32 = 4i32;
pub const MSIINSTALLCONTEXT_ALL: i32 = 7i32;
pub const MSIINSTALLCONTEXT_ALLUSERMANAGED: i32 = 8i32;
pub const MSIMODIFY_SEEK: i32 = -1i32;
pub const MSIMODIFY_REFRESH: i32 = 0i32;
pub const MSIMODIFY_INSERT: i32 = 1i32;
pub const MSIMODIFY_UPDATE: i32 = 2i32;
pub const MSIMODIFY_ASSIGN: i32 = 3i32;
pub const MSIMODIFY_REPLACE: i32 = 4i32;
pub const MSIMODIFY_MERGE: i32 = 5i32;
pub const MSIMODIFY_DELETE: i32 = 6i32;
pub const MSIMODIFY_INSERT_TEMPORARY: i32 = 7i32;
pub const MSIMODIFY_VALIDATE: i32 = 8i32;
pub const MSIMODIFY_VALIDATE_NEW: i32 = 9i32;
pub const MSIMODIFY_VALIDATE_FIELD: i32 = 10i32;
pub const MSIMODIFY_VALIDATE_DELETE: i32 = 11i32;
pub const MSIOPENPACKAGEFLAGS_IGNOREMACHINESTATE: i32 = 1i32;
pub const MSIPATCH_DATATYPE_PATCHFILE: i32 = 0i32;
pub const MSIPATCH_DATATYPE_XMLPATH: i32 = 1i32;
pub const MSIPATCH_DATATYPE_XMLBLOB: i32 = 2i32;
#[repr(C)]
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
pub const MSIPATCHSTATE_INVALID: i32 = 0i32;
pub const MSIPATCHSTATE_APPLIED: i32 = 1i32;
pub const MSIPATCHSTATE_SUPERSEDED: i32 = 2i32;
pub const MSIPATCHSTATE_OBSOLETED: i32 = 4i32;
pub const MSIPATCHSTATE_REGISTERED: i32 = 8i32;
pub const MSIPATCHSTATE_ALL: i32 = 15i32;
pub const MSIRUNMODE_ADMIN: i32 = 0i32;
pub const MSIRUNMODE_ADVERTISE: i32 = 1i32;
pub const MSIRUNMODE_MAINTENANCE: i32 = 2i32;
pub const MSIRUNMODE_ROLLBACKENABLED: i32 = 3i32;
pub const MSIRUNMODE_LOGENABLED: i32 = 4i32;
pub const MSIRUNMODE_OPERATIONS: i32 = 5i32;
pub const MSIRUNMODE_REBOOTATEND: i32 = 6i32;
pub const MSIRUNMODE_REBOOTNOW: i32 = 7i32;
pub const MSIRUNMODE_CABINET: i32 = 8i32;
pub const MSIRUNMODE_SOURCESHORTNAMES: i32 = 9i32;
pub const MSIRUNMODE_TARGETSHORTNAMES: i32 = 10i32;
pub const MSIRUNMODE_RESERVED11: i32 = 11i32;
pub const MSIRUNMODE_WINDOWS9X: i32 = 12i32;
pub const MSIRUNMODE_ZAWENABLED: i32 = 13i32;
pub const MSIRUNMODE_RESERVED14: i32 = 14i32;
pub const MSIRUNMODE_RESERVED15: i32 = 15i32;
pub const MSIRUNMODE_SCHEDULED: i32 = 16i32;
pub const MSIRUNMODE_ROLLBACK: i32 = 17i32;
pub const MSIRUNMODE_COMMIT: i32 = 18i32;
pub const MSISOURCETYPE_UNKNOWN: i32 = 0i32;
pub const MSISOURCETYPE_NETWORK: i32 = 1i32;
pub const MSISOURCETYPE_URL: i32 = 2i32;
pub const MSISOURCETYPE_MEDIA: i32 = 4i32;
pub const MSITRANSACTION_CHAIN_EMBEDDEDUI: i32 = 1i32;
pub const MSITRANSACTION_JOIN_EXISTING_EMBEDDEDUI: i32 = 2i32;
pub const MSITRANSACTIONSTATE_ROLLBACK: u32 = 0u32;
pub const MSITRANSACTIONSTATE_COMMIT: u32 = 1u32;
pub const MSITRANSFORM_ERROR_ADDEXISTINGROW: i32 = 1i32;
pub const MSITRANSFORM_ERROR_DELMISSINGROW: i32 = 2i32;
pub const MSITRANSFORM_ERROR_ADDEXISTINGTABLE: i32 = 4i32;
pub const MSITRANSFORM_ERROR_DELMISSINGTABLE: i32 = 8i32;
pub const MSITRANSFORM_ERROR_UPDATEMISSINGROW: i32 = 16i32;
pub const MSITRANSFORM_ERROR_CHANGECODEPAGE: i32 = 32i32;
pub const MSITRANSFORM_ERROR_VIEWTRANSFORM: i32 = 256i32;
pub const MSITRANSFORM_ERROR_NONE: i32 = 0i32;
pub const MSITRANSFORM_VALIDATE_LANGUAGE: i32 = 1i32;
pub const MSITRANSFORM_VALIDATE_PRODUCT: i32 = 2i32;
pub const MSITRANSFORM_VALIDATE_PLATFORM: i32 = 4i32;
pub const MSITRANSFORM_VALIDATE_MAJORVERSION: i32 = 8i32;
pub const MSITRANSFORM_VALIDATE_MINORVERSION: i32 = 16i32;
pub const MSITRANSFORM_VALIDATE_UPDATEVERSION: i32 = 32i32;
pub const MSITRANSFORM_VALIDATE_NEWLESSBASEVERSION: i32 = 64i32;
pub const MSITRANSFORM_VALIDATE_NEWLESSEQUALBASEVERSION: i32 = 128i32;
pub const MSITRANSFORM_VALIDATE_NEWEQUALBASEVERSION: i32 = 256i32;
pub const MSITRANSFORM_VALIDATE_NEWGREATEREQUALBASEVERSION: i32 = 512i32;
pub const MSITRANSFORM_VALIDATE_NEWGREATERBASEVERSION: i32 = 1024i32;
pub const MSITRANSFORM_VALIDATE_UPGRADECODE: i32 = 2048i32;
pub const MSI_INVALID_HASH_IS_FATAL: u32 = 1u32;
pub const MSI_NULL_INTEGER: u32 = 2147483648u32;
pub const MsmMerge: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 182298672, data2: 11302, data3: 4562, data4: [173, 101, 0, 160, 201, 175, 17, 166] };
pub const PACKMAN_RUNTIME_NATIVE: i32 = 1i32;
pub const PACKMAN_RUNTIME_SILVERLIGHTMOBILE: i32 = 2i32;
pub const PACKMAN_RUNTIME_XNA: i32 = 3i32;
pub const PACKMAN_RUNTIME_MODERN_NATIVE: i32 = 4i32;
pub const PACKMAN_RUNTIME_JUPITER: i32 = 5i32;
pub const PACKMAN_RUNTIME_INVALID: i32 = 6i32;
#[repr(C)]
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
pub type PINSTALLUI_HANDLER_RECORD = unsafe extern "system" fn(pvcontext: *mut ::core::ffi::c_void, imessagetype: u32, hrecord: MSIHANDLE) -> i32;
#[repr(C)]
pub struct PMSIHANDLE {
    pub m_h: MSIHANDLE,
}
impl ::core::marker::Copy for PMSIHANDLE {}
impl ::core::clone::Clone for PMSIHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PMSvc: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3118797308,
    data2: 58212,
    data3: 18810,
    data4: [161, 33, 183, 179, 97, 44, 237, 206],
};
pub const PM_ACTIVATION_POLICY_RESUME: i32 = 0i32;
pub const PM_ACTIVATION_POLICY_RESUMESAMEPARAMS: i32 = 1i32;
pub const PM_ACTIVATION_POLICY_REPLACE: i32 = 2i32;
pub const PM_ACTIVATION_POLICY_REPLACESAMEPARAMS: i32 = 3i32;
pub const PM_ACTIVATION_POLICY_MULTISESSION: i32 = 4i32;
pub const PM_ACTIVATION_POLICY_REPLACE_IGNOREFOREGROUND: i32 = 5i32;
pub const PM_ACTIVATION_POLICY_UNKNOWN: i32 = 6i32;
pub const PM_ACTIVATION_POLICY_INVALID: i32 = 7i32;
pub const PM_APPLICATION_HUBTYPE_NONMUSIC: i32 = 0i32;
pub const PM_APPLICATION_HUBTYPE_MUSIC: i32 = 1i32;
pub const PM_APPLICATION_HUBTYPE_INVALID: i32 = 2i32;
pub const PM_APPLICATION_INSTALL_NORMAL: i32 = 0i32;
pub const PM_APPLICATION_INSTALL_IN_ROM: i32 = 1i32;
pub const PM_APPLICATION_INSTALL_PA: i32 = 2i32;
pub const PM_APPLICATION_INSTALL_DEBUG: i32 = 3i32;
pub const PM_APPLICATION_INSTALL_ENTERPRISE: i32 = 4i32;
pub const PM_APPLICATION_INSTALL_INVALID: i32 = 5i32;
pub const PM_APPLICATION_STATE_MIN: i32 = 0i32;
pub const PM_APPLICATION_STATE_INSTALLED: i32 = 1i32;
pub const PM_APPLICATION_STATE_INSTALLING: i32 = 2i32;
pub const PM_APPLICATION_STATE_UPDATING: i32 = 3i32;
pub const PM_APPLICATION_STATE_UNINSTALLING: i32 = 4i32;
pub const PM_APPLICATION_STATE_LICENSE_UPDATING: i32 = 5i32;
pub const PM_APPLICATION_STATE_MOVING: i32 = 6i32;
pub const PM_APPLICATION_STATE_DISABLED_SD_CARD: i32 = 7i32;
pub const PM_APPLICATION_STATE_DISABLED_ENTERPRISE: i32 = 8i32;
pub const PM_APPLICATION_STATE_DISABLED_BACKING_UP: i32 = 9i32;
pub const PM_APPLICATION_STATE_DISABLED_MDIL_BINDING: i32 = 10i32;
pub const PM_APPLICATION_STATE_MAX: i32 = 10i32;
pub const PM_APPLICATION_STATE_INVALID: i32 = 11i32;
pub const PM_APP_GENRE_GAMES: i32 = 0i32;
pub const PM_APP_GENRE_OTHER: i32 = 1i32;
pub const PM_APP_GENRE_INVALID: i32 = 2i32;
#[repr(C)]
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
pub const PM_APP_FILTER_ALL: i32 = 0i32;
pub const PM_APP_FILTER_VISIBLE: i32 = 1i32;
pub const PM_APP_FILTER_GENRE: i32 = 2i32;
pub const PM_APP_FILTER_NONGAMES: i32 = 3i32;
pub const PM_APP_FILTER_HUBTYPE: i32 = 4i32;
pub const PM_APP_FILTER_PINABLEONKIDZONE: i32 = 5i32;
pub const PM_APP_FILTER_ALL_INCLUDE_MODERN: i32 = 6i32;
pub const PM_APP_FILTER_FRAMEWORK: i32 = 7i32;
pub const PM_APP_FILTER_MAX: i32 = 8i32;
pub const PM_ENUM_BSA_FILTER_ALL: i32 = 26i32;
pub const PM_ENUM_BSA_FILTER_BY_TASKID: i32 = 27i32;
pub const PM_ENUM_BSA_FILTER_BY_PRODUCTID: i32 = 28i32;
pub const PM_ENUM_BSA_FILTER_BY_PERIODIC: i32 = 29i32;
pub const PM_ENUM_BSA_FILTER_BY_ALL_LAUNCHONBOOT: i32 = 30i32;
pub const PM_ENUM_BSA_FILTER_MAX: i32 = 31i32;
pub const PM_ENUM_BW_FILTER_BOOTWORKER_ALL: i32 = 31i32;
pub const PM_ENUM_BW_FILTER_BY_TASKID: i32 = 32i32;
pub const PM_ENUM_BW_FILTER_MAX: i32 = 33i32;
pub const PM_ENUM_EXTENSION_FILTER_BY_CONSUMER: i32 = 17i32;
pub const PM_ENUM_EXTENSION_FILTER_APPCONNECT: i32 = 17i32;
pub const PM_ENUM_EXTENSION_FILTER_PROTOCOL_ALL: i32 = 18i32;
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_FILETYPE_ALL: i32 = 19i32;
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_CONTENTTYPE_ALL: i32 = 20i32;
pub const PM_ENUM_EXTENSION_FILTER_FTASSOC_APPLICATION_ALL: i32 = 21i32;
pub const PM_ENUM_EXTENSION_FILTER_SHARETARGET_ALL: i32 = 22i32;
pub const PM_ENUM_EXTENSION_FILTER_FILEOPENPICKER_ALL: i32 = 23i32;
pub const PM_ENUM_EXTENSION_FILTER_FILESAVEPICKER_ALL: i32 = 24i32;
pub const PM_ENUM_EXTENSION_FILTER_CACHEDFILEUPDATER_ALL: i32 = 25i32;
pub const PM_ENUM_EXTENSION_FILTER_MAX: i32 = 26i32;
#[repr(C)]
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
pub const PM_TASK_FILTER_APP_ALL: i32 = 12i32;
pub const PM_TASK_FILTER_TASK_TYPE: i32 = 13i32;
pub const PM_TASK_FILTER_DEHYD_SUPRESSING: i32 = 14i32;
pub const PM_TASK_FILTER_APP_TASK_TYPE: i32 = 15i32;
pub const PM_TASK_FILTER_BGEXECUTION: i32 = 16i32;
pub const PM_TASK_FILTER_MAX: i32 = 17i32;
pub const PM_TILE_FILTER_APPLIST: i32 = 8i32;
pub const PM_TILE_FILTER_PINNED: i32 = 9i32;
pub const PM_TILE_FILTER_HUBTYPE: i32 = 10i32;
pub const PM_TILE_FILTER_APP_ALL: i32 = 11i32;
pub const PM_TILE_FILTER_MAX: i32 = 12i32;
#[repr(C)]
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
pub const PM_LIVETILE_RECURRENCE_TYPE_INSTANT: i32 = 0i32;
pub const PM_LIVETILE_RECURRENCE_TYPE_ONETIME: i32 = 1i32;
pub const PM_LIVETILE_RECURRENCE_TYPE_INTERVAL: i32 = 2i32;
pub const PM_LIVETILE_RECURRENCE_TYPE_MAX: i32 = 2i32;
pub const PM_LOGO_SIZE_SMALL: i32 = 0i32;
pub const PM_LOGO_SIZE_MEDIUM: i32 = 1i32;
pub const PM_LOGO_SIZE_LARGE: i32 = 2i32;
pub const PM_LOGO_SIZE_INVALID: i32 = 3i32;
#[repr(C)]
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
pub const PM_STARTTILE_TYPE_PRIMARY: i32 = 1i32;
pub const PM_STARTTILE_TYPE_SECONDARY: i32 = 2i32;
pub const PM_STARTTILE_TYPE_APPLIST: i32 = 3i32;
pub const PM_STARTTILE_TYPE_APPLISTPRIMARY: i32 = 4i32;
pub const PM_STARTTILE_TYPE_INVALID: i32 = 5i32;
pub const PM_TASK_TRANSITION_DEFAULT: i32 = 0i32;
pub const PM_TASK_TRANSITION_NONE: i32 = 1i32;
pub const PM_TASK_TRANSITION_TURNSTILE: i32 = 2i32;
pub const PM_TASK_TRANSITION_SLIDE: i32 = 3i32;
pub const PM_TASK_TRANSITION_SWIVEL: i32 = 4i32;
pub const PM_TASK_TRANSITION_READERBOARD: i32 = 5i32;
pub const PM_TASK_TRANSITION_CUSTOM: i32 = 6i32;
pub const PM_TASK_TRANSITION_INVALID: i32 = 7i32;
pub const PM_TASK_TYPE_NORMAL: i32 = 0i32;
pub const PM_TASK_TYPE_DEFAULT: i32 = 1i32;
pub const PM_TASK_TYPE_SETTINGS: i32 = 2i32;
pub const PM_TASK_TYPE_BACKGROUNDSERVICEAGENT: i32 = 3i32;
pub const PM_TASK_TYPE_BACKGROUNDWORKER: i32 = 4i32;
pub const PM_TASK_TYPE_INVALID: i32 = 5i32;
pub const PM_TILE_HUBTYPE_MUSIC: i32 = 1i32;
pub const PM_TILE_HUBTYPE_MOSETTINGS: i32 = 268435456i32;
pub const PM_TILE_HUBTYPE_GAMES: i32 = 536870912i32;
pub const PM_TILE_HUBTYPE_APPLIST: i32 = 1073741824i32;
pub const PM_TILE_HUBTYPE_STARTMENU: i32 = -2147483648i32;
pub const PM_TILE_HUBTYPE_LOCKSCREEN: i32 = 16777216i32;
pub const PM_TILE_HUBTYPE_KIDZONE: i32 = 33554432i32;
pub const PM_TILE_HUBTYPE_CACHED: i32 = 67108864i32;
pub const PM_TILE_HUBTYPE_INVALID: i32 = 67108865i32;
pub const PM_TILE_SIZE_SMALL: i32 = 0i32;
pub const PM_TILE_SIZE_MEDIUM: i32 = 1i32;
pub const PM_TILE_SIZE_LARGE: i32 = 2i32;
pub const PM_TILE_SIZE_SQUARE310X310: i32 = 3i32;
pub const PM_TILE_SIZE_TALL150X310: i32 = 4i32;
pub const PM_TILE_SIZE_INVALID: i32 = 5i32;
#[repr(C)]
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
#[cfg(feature = "Win32_Foundation")]
pub type PPATCH_PROGRESS_CALLBACK = unsafe extern "system" fn(callbackcontext: *mut ::core::ffi::c_void, currentposition: u32, maximumposition: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PPATCH_SYMLOAD_CALLBACK = unsafe extern "system" fn(whichfile: u32, symbolfilename: super::super::Foundation::PSTR, symtype: u32, symbolfilechecksum: u32, symbolfiletimedate: u32, imagefilechecksum: u32, imagefiletimedate: u32, callbackcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
#[repr(C)]
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
pub const QUERYASMINFO_FLAG_VALIDATE: u32 = 1u32;
pub const REINSTALLMODE_REPAIR: i32 = 1i32;
pub const REINSTALLMODE_FILEMISSING: i32 = 2i32;
pub const REINSTALLMODE_FILEOLDERVERSION: i32 = 4i32;
pub const REINSTALLMODE_FILEEQUALVERSION: i32 = 8i32;
pub const REINSTALLMODE_FILEEXACT: i32 = 16i32;
pub const REINSTALLMODE_FILEVERIFY: i32 = 32i32;
pub const REINSTALLMODE_FILEREPLACE: i32 = 64i32;
pub const REINSTALLMODE_MACHINEDATA: i32 = 128i32;
pub const REINSTALLMODE_USERDATA: i32 = 256i32;
pub const REINSTALLMODE_SHORTCUT: i32 = 512i32;
pub const REINSTALLMODE_PACKAGE: i32 = 1024i32;
pub const ieUnknown: i32 = 0i32;
pub const ieError: i32 = 1i32;
pub const ieWarning: i32 = 2i32;
pub const ieInfo: i32 = 3i32;
pub const SCRIPTFLAGS_CACHEINFO: i32 = 1i32;
pub const SCRIPTFLAGS_SHORTCUTS: i32 = 4i32;
pub const SCRIPTFLAGS_MACHINEASSIGN: i32 = 8i32;
pub const SCRIPTFLAGS_REGDATA_CNFGINFO: i32 = 32i32;
pub const SCRIPTFLAGS_VALIDATE_TRANSFORMS_LIST: i32 = 64i32;
pub const SCRIPTFLAGS_REGDATA_CLASSINFO: i32 = 128i32;
pub const SCRIPTFLAGS_REGDATA_EXTENSIONINFO: i32 = 256i32;
pub const SCRIPTFLAGS_REGDATA_APPINFO: i32 = 384i32;
pub const SCRIPTFLAGS_REGDATA: i32 = 416i32;
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
pub const ieStatusGetCUB: i32 = 0i32;
pub const ieStatusICECount: i32 = 1i32;
pub const ieStatusMerge: i32 = 2i32;
pub const ieStatusSummaryInfo: i32 = 3i32;
pub const ieStatusCreateEngine: i32 = 4i32;
pub const ieStatusStarting: i32 = 5i32;
pub const ieStatusRunICE: i32 = 6i32;
pub const ieStatusShutdown: i32 = 7i32;
pub const ieStatusSuccess: i32 = 8i32;
pub const ieStatusFail: i32 = 9i32;
pub const ieStatusCancel: i32 = 10i32;
pub const STREAM_FORMAT_COMPLIB_MANIFEST: u32 = 1u32;
pub const STREAM_FORMAT_COMPLIB_MODULE: u32 = 0u32;
pub const STREAM_FORMAT_WIN32_MANIFEST: u32 = 4u32;
pub const STREAM_FORMAT_WIN32_MODULE: u32 = 2u32;
pub const TILE_TEMPLATE_INVALID: i32 = 0i32;
pub const TILE_TEMPLATE_FLIP: i32 = 5i32;
pub const TILE_TEMPLATE_DEEPLINK: i32 = 13i32;
pub const TILE_TEMPLATE_CYCLE: i32 = 14i32;
pub const TILE_TEMPLATE_METROCOUNT: i32 = 1i32;
pub const TILE_TEMPLATE_AGILESTORE: i32 = 2i32;
pub const TILE_TEMPLATE_GAMES: i32 = 3i32;
pub const TILE_TEMPLATE_CALENDAR: i32 = 4i32;
pub const TILE_TEMPLATE_MUSICVIDEO: i32 = 7i32;
pub const TILE_TEMPLATE_PEOPLE: i32 = 10i32;
pub const TILE_TEMPLATE_CONTACT: i32 = 11i32;
pub const TILE_TEMPLATE_GROUP: i32 = 12i32;
pub const TILE_TEMPLATE_DEFAULT: i32 = 15i32;
pub const TILE_TEMPLATE_BADGE: i32 = 16i32;
pub const TILE_TEMPLATE_BLOCK: i32 = 17i32;
pub const TILE_TEMPLATE_TEXT01: i32 = 18i32;
pub const TILE_TEMPLATE_TEXT02: i32 = 19i32;
pub const TILE_TEMPLATE_TEXT03: i32 = 20i32;
pub const TILE_TEMPLATE_TEXT04: i32 = 21i32;
pub const TILE_TEMPLATE_TEXT05: i32 = 22i32;
pub const TILE_TEMPLATE_TEXT06: i32 = 23i32;
pub const TILE_TEMPLATE_TEXT07: i32 = 24i32;
pub const TILE_TEMPLATE_TEXT08: i32 = 25i32;
pub const TILE_TEMPLATE_TEXT09: i32 = 26i32;
pub const TILE_TEMPLATE_TEXT10: i32 = 27i32;
pub const TILE_TEMPLATE_TEXT11: i32 = 28i32;
pub const TILE_TEMPLATE_IMAGE: i32 = 29i32;
pub const TILE_TEMPLATE_IMAGECOLLECTION: i32 = 30i32;
pub const TILE_TEMPLATE_IMAGEANDTEXT01: i32 = 31i32;
pub const TILE_TEMPLATE_IMAGEANDTEXT02: i32 = 32i32;
pub const TILE_TEMPLATE_BLOCKANDTEXT01: i32 = 33i32;
pub const TILE_TEMPLATE_BLOCKANDTEXT02: i32 = 34i32;
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT01: i32 = 35i32;
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT02: i32 = 36i32;
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT03: i32 = 37i32;
pub const TILE_TEMPLATE_PEEKIMAGEANDTEXT04: i32 = 38i32;
pub const TILE_TEMPLATE_PEEKIMAGE01: i32 = 39i32;
pub const TILE_TEMPLATE_PEEKIMAGE02: i32 = 40i32;
pub const TILE_TEMPLATE_PEEKIMAGE03: i32 = 41i32;
pub const TILE_TEMPLATE_PEEKIMAGE04: i32 = 42i32;
pub const TILE_TEMPLATE_PEEKIMAGE05: i32 = 43i32;
pub const TILE_TEMPLATE_PEEKIMAGE06: i32 = 44i32;
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION01: i32 = 45i32;
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION02: i32 = 46i32;
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION03: i32 = 47i32;
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION04: i32 = 48i32;
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION05: i32 = 49i32;
pub const TILE_TEMPLATE_PEEKIMAGECOLLECTION06: i32 = 50i32;
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT01: i32 = 51i32;
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT02: i32 = 52i32;
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT03: i32 = 53i32;
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT04: i32 = 54i32;
pub const TILE_TEMPLATE_SMALLIMAGEANDTEXT05: i32 = 55i32;
pub const TILE_TEMPLATE_METROCOUNTQUEUE: i32 = 56i32;
pub const TILE_TEMPLATE_SEARCH: i32 = 57i32;
pub const TILE_TEMPLATE_TILEFLYOUT01: i32 = 58i32;
pub const TILE_TEMPLATE_FOLDER: i32 = 59i32;
pub const TILE_TEMPLATE_ALL: i32 = 100i32;
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
pub const USERINFOSTATE_MOREDATA: i32 = -3i32;
pub const USERINFOSTATE_INVALIDARG: i32 = -2i32;
pub const USERINFOSTATE_UNKNOWN: i32 = -1i32;
pub const USERINFOSTATE_ABSENT: i32 = 0i32;
pub const USERINFOSTATE_PRESENT: i32 = 1i32;
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
pub const cchMaxInteger: i32 = 12i32;
pub const msidbAssemblyAttributesURT: i32 = 0i32;
pub const msidbAssemblyAttributesWin32: i32 = 1i32;
pub const msidbClassAttributesRelativePath: i32 = 1i32;
pub const msidbComponentAttributesLocalOnly: i32 = 0i32;
pub const msidbComponentAttributesSourceOnly: i32 = 1i32;
pub const msidbComponentAttributesOptional: i32 = 2i32;
pub const msidbComponentAttributesRegistryKeyPath: i32 = 4i32;
pub const msidbComponentAttributesSharedDllRefCount: i32 = 8i32;
pub const msidbComponentAttributesPermanent: i32 = 16i32;
pub const msidbComponentAttributesODBCDataSource: i32 = 32i32;
pub const msidbComponentAttributesTransitive: i32 = 64i32;
pub const msidbComponentAttributesNeverOverwrite: i32 = 128i32;
pub const msidbComponentAttributes64bit: i32 = 256i32;
pub const msidbComponentAttributesDisableRegistryReflection: i32 = 512i32;
pub const msidbComponentAttributesUninstallOnSupersedence: i32 = 1024i32;
pub const msidbComponentAttributesShared: i32 = 2048i32;
pub const msidbControlAttributesVisible: i32 = 1i32;
pub const msidbControlAttributesEnabled: i32 = 2i32;
pub const msidbControlAttributesSunken: i32 = 4i32;
pub const msidbControlAttributesIndirect: i32 = 8i32;
pub const msidbControlAttributesInteger: i32 = 16i32;
pub const msidbControlAttributesRTLRO: i32 = 32i32;
pub const msidbControlAttributesRightAligned: i32 = 64i32;
pub const msidbControlAttributesLeftScroll: i32 = 128i32;
pub const msidbControlAttributesBiDi: i32 = 224i32;
pub const msidbControlAttributesTransparent: i32 = 65536i32;
pub const msidbControlAttributesNoPrefix: i32 = 131072i32;
pub const msidbControlAttributesNoWrap: i32 = 262144i32;
pub const msidbControlAttributesFormatSize: i32 = 524288i32;
pub const msidbControlAttributesUsersLanguage: i32 = 1048576i32;
pub const msidbControlAttributesMultiline: i32 = 65536i32;
pub const msidbControlAttributesPasswordInput: i32 = 2097152i32;
pub const msidbControlAttributesProgress95: i32 = 65536i32;
pub const msidbControlAttributesRemovableVolume: i32 = 65536i32;
pub const msidbControlAttributesFixedVolume: i32 = 131072i32;
pub const msidbControlAttributesRemoteVolume: i32 = 262144i32;
pub const msidbControlAttributesCDROMVolume: i32 = 524288i32;
pub const msidbControlAttributesRAMDiskVolume: i32 = 1048576i32;
pub const msidbControlAttributesFloppyVolume: i32 = 2097152i32;
pub const msidbControlShowRollbackCost: i32 = 4194304i32;
pub const msidbControlAttributesSorted: i32 = 65536i32;
pub const msidbControlAttributesComboList: i32 = 131072i32;
pub const msidbControlAttributesImageHandle: i32 = 65536i32;
pub const msidbControlAttributesPushLike: i32 = 131072i32;
pub const msidbControlAttributesBitmap: i32 = 262144i32;
pub const msidbControlAttributesIcon: i32 = 524288i32;
pub const msidbControlAttributesFixedSize: i32 = 1048576i32;
pub const msidbControlAttributesIconSize16: i32 = 2097152i32;
pub const msidbControlAttributesIconSize32: i32 = 4194304i32;
pub const msidbControlAttributesIconSize48: i32 = 6291456i32;
pub const msidbControlAttributesElevationShield: i32 = 8388608i32;
pub const msidbControlAttributesHasBorder: i32 = 16777216i32;
pub const msidbCustomActionTypeDll: i32 = 1i32;
pub const msidbCustomActionTypeExe: i32 = 2i32;
pub const msidbCustomActionTypeTextData: i32 = 3i32;
pub const msidbCustomActionTypeJScript: i32 = 5i32;
pub const msidbCustomActionTypeVBScript: i32 = 6i32;
pub const msidbCustomActionTypeInstall: i32 = 7i32;
pub const msidbCustomActionTypeBinaryData: i32 = 0i32;
pub const msidbCustomActionTypeSourceFile: i32 = 16i32;
pub const msidbCustomActionTypeDirectory: i32 = 32i32;
pub const msidbCustomActionTypeProperty: i32 = 48i32;
pub const msidbCustomActionTypeContinue: i32 = 64i32;
pub const msidbCustomActionTypeAsync: i32 = 128i32;
pub const msidbCustomActionTypeFirstSequence: i32 = 256i32;
pub const msidbCustomActionTypeOncePerProcess: i32 = 512i32;
pub const msidbCustomActionTypeClientRepeat: i32 = 768i32;
pub const msidbCustomActionTypeInScript: i32 = 1024i32;
pub const msidbCustomActionTypeRollback: i32 = 256i32;
pub const msidbCustomActionTypeCommit: i32 = 512i32;
pub const msidbCustomActionTypeNoImpersonate: i32 = 2048i32;
pub const msidbCustomActionTypeTSAware: i32 = 16384i32;
pub const msidbCustomActionType64BitScript: i32 = 4096i32;
pub const msidbCustomActionTypeHideTarget: i32 = 8192i32;
pub const msidbCustomActionTypePatchUninstall: i32 = 32768i32;
pub const msidbDialogAttributesVisible: i32 = 1i32;
pub const msidbDialogAttributesModal: i32 = 2i32;
pub const msidbDialogAttributesMinimize: i32 = 4i32;
pub const msidbDialogAttributesSysModal: i32 = 8i32;
pub const msidbDialogAttributesKeepModeless: i32 = 16i32;
pub const msidbDialogAttributesTrackDiskSpace: i32 = 32i32;
pub const msidbDialogAttributesUseCustomPalette: i32 = 64i32;
pub const msidbDialogAttributesRTLRO: i32 = 128i32;
pub const msidbDialogAttributesRightAligned: i32 = 256i32;
pub const msidbDialogAttributesLeftScroll: i32 = 512i32;
pub const msidbDialogAttributesBiDi: i32 = 896i32;
pub const msidbDialogAttributesError: i32 = 65536i32;
pub const msidbEmbeddedUI: i32 = 1i32;
pub const msidbEmbeddedHandlesBasic: i32 = 2i32;
pub const msidbFeatureAttributesFavorLocal: i32 = 0i32;
pub const msidbFeatureAttributesFavorSource: i32 = 1i32;
pub const msidbFeatureAttributesFollowParent: i32 = 2i32;
pub const msidbFeatureAttributesFavorAdvertise: i32 = 4i32;
pub const msidbFeatureAttributesDisallowAdvertise: i32 = 8i32;
pub const msidbFeatureAttributesUIDisallowAbsent: i32 = 16i32;
pub const msidbFeatureAttributesNoUnsupportedAdvertise: i32 = 32i32;
pub const msidbFileAttributesReadOnly: i32 = 1i32;
pub const msidbFileAttributesHidden: i32 = 2i32;
pub const msidbFileAttributesSystem: i32 = 4i32;
pub const msidbFileAttributesReserved0: i32 = 8i32;
pub const msidbFileAttributesIsolatedComp: i32 = 16i32;
pub const msidbFileAttributesReserved1: i32 = 64i32;
pub const msidbFileAttributesReserved2: i32 = 128i32;
pub const msidbFileAttributesReserved3: i32 = 256i32;
pub const msidbFileAttributesVital: i32 = 512i32;
pub const msidbFileAttributesChecksum: i32 = 1024i32;
pub const msidbFileAttributesPatchAdded: i32 = 4096i32;
pub const msidbFileAttributesNoncompressed: i32 = 8192i32;
pub const msidbFileAttributesCompressed: i32 = 16384i32;
pub const msidbFileAttributesReserved4: i32 = 32768i32;
pub const msidbIniFileActionAddLine: i32 = 0i32;
pub const msidbIniFileActionCreateLine: i32 = 1i32;
pub const msidbIniFileActionRemoveLine: i32 = 2i32;
pub const msidbIniFileActionAddTag: i32 = 3i32;
pub const msidbIniFileActionRemoveTag: i32 = 4i32;
pub const msidbLocatorTypeDirectory: i32 = 0i32;
pub const msidbLocatorTypeFileName: i32 = 1i32;
pub const msidbLocatorTypeRawValue: i32 = 2i32;
pub const msidbLocatorType64bit: i32 = 16i32;
pub const msidbMoveFileOptionsMove: i32 = 1i32;
pub const msidbODBCDataSourceRegistrationPerMachine: i32 = 0i32;
pub const msidbODBCDataSourceRegistrationPerUser: i32 = 1i32;
pub const msidbPatchAttributesNonVital: i32 = 1i32;
pub const msidbRegistryRootClassesRoot: i32 = 0i32;
pub const msidbRegistryRootCurrentUser: i32 = 1i32;
pub const msidbRegistryRootLocalMachine: i32 = 2i32;
pub const msidbRegistryRootUsers: i32 = 3i32;
pub const msidbRemoveFileInstallModeOnInstall: i32 = 1i32;
pub const msidbRemoveFileInstallModeOnRemove: i32 = 2i32;
pub const msidbRemoveFileInstallModeOnBoth: i32 = 3i32;
pub const msidbServiceConfigEventInstall: i32 = 1i32;
pub const msidbServiceConfigEventUninstall: i32 = 2i32;
pub const msidbServiceConfigEventReinstall: i32 = 4i32;
pub const msidbServiceControlEventStart: i32 = 1i32;
pub const msidbServiceControlEventStop: i32 = 2i32;
pub const msidbServiceControlEventDelete: i32 = 8i32;
pub const msidbServiceControlEventUninstallStart: i32 = 16i32;
pub const msidbServiceControlEventUninstallStop: i32 = 32i32;
pub const msidbServiceControlEventUninstallDelete: i32 = 128i32;
pub const msidbServiceInstallErrorControlVital: i32 = 32768i32;
pub const msidbSumInfoSourceTypeSFN: i32 = 1i32;
pub const msidbSumInfoSourceTypeCompressed: i32 = 2i32;
pub const msidbSumInfoSourceTypeAdminImage: i32 = 4i32;
pub const msidbSumInfoSourceTypeLUAPackage: i32 = 8i32;
pub const msidbTextStyleStyleBitsBold: i32 = 1i32;
pub const msidbTextStyleStyleBitsItalic: i32 = 2i32;
pub const msidbTextStyleStyleBitsUnderline: i32 = 4i32;
pub const msidbTextStyleStyleBitsStrike: i32 = 8i32;
pub const msidbUpgradeAttributesMigrateFeatures: i32 = 1i32;
pub const msidbUpgradeAttributesOnlyDetect: i32 = 2i32;
pub const msidbUpgradeAttributesIgnoreRemoveFailure: i32 = 4i32;
pub const msidbUpgradeAttributesVersionMinInclusive: i32 = 256i32;
pub const msidbUpgradeAttributesVersionMaxInclusive: i32 = 512i32;
pub const msidbUpgradeAttributesLanguagesExclusive: i32 = 1024i32;
pub const msifiFastInstallNoSR: i32 = 1i32;
pub const msifiFastInstallQuickCosting: i32 = 2i32;
pub const msifiFastInstallLessPrgMsg: i32 = 4i32;
pub const msirbRebootUndeterminedReason: i32 = 0i32;
pub const msirbRebootInUseFilesReason: i32 = 1i32;
pub const msirbRebootScheduleRebootReason: i32 = 2i32;
pub const msirbRebootForceRebootReason: i32 = 3i32;
pub const msirbRebootCustomActionReason: i32 = 4i32;
pub const msirbRebootImmediate: i32 = 1i32;
pub const msirbRebootDeferred: i32 = 2i32;
pub const msmErrorLanguageUnsupported: i32 = 1i32;
pub const msmErrorLanguageFailed: i32 = 2i32;
pub const msmErrorExclusion: i32 = 3i32;
pub const msmErrorTableMerge: i32 = 4i32;
pub const msmErrorResequenceMerge: i32 = 5i32;
pub const msmErrorFileCreate: i32 = 6i32;
pub const msmErrorDirCreate: i32 = 7i32;
pub const msmErrorFeatureRequired: i32 = 8i32;
