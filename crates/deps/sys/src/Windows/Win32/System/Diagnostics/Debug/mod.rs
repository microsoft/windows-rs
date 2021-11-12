#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_System_Diagnostics_Debug_WebApp")]
pub mod WebApp;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn AddVectoredContinueHandler(first: u32, handler: PVECTORED_EXCEPTION_HANDLER) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn AddVectoredExceptionHandler(first: u32, handler: PVECTORED_EXCEPTION_HANDLER) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Beep(dwfreq: u32, dwduration: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BindImage(imagename: super::super::super::Foundation::PSTR, dllpath: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BindImageEx(flags: u32, imagename: super::super::super::Foundation::PSTR, dllpath: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, statusroutine: PIMAGEHLP_STATUS_ROUTINE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckRemoteDebuggerPresent(hprocess: super::super::super::Foundation::HANDLE, pbdebuggerpresent: *mut super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn CheckSumMappedFile(baseaddress: *const ::core::ffi::c_void, filelength: u32, headersum: *mut u32, checksum: *mut u32) -> *mut IMAGE_NT_HEADERS64;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn CheckSumMappedFile(baseaddress: *const ::core::ffi::c_void, filelength: u32, headersum: *mut u32, checksum: *mut u32) -> *mut IMAGE_NT_HEADERS32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn CloseThreadWaitChainSession(wcthandle: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ContinueDebugEvent(dwprocessid: u32, dwthreadid: u32, dwcontinuestatus: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn CopyContext(destination: *mut CONTEXT, contextflags: u32, source: *const CONTEXT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn CreateDataModelManager(debughost: IDebugHost, manager: *mut IDataModelManager) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DbgHelpCreateUserDump(filename: super::super::super::Foundation::PSTR, callback: PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DbgHelpCreateUserDumpW(filename: super::super::super::Foundation::PWSTR, callback: PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugActiveProcess(dwprocessid: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugActiveProcessStop(dwprocessid: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn DebugBreak();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugBreakProcess(process: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugConnect(remoteoptions: super::super::super::Foundation::PSTR, interfaceid: *const ::windows_sys::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugConnectWide(remoteoptions: super::super::super::Foundation::PWSTR, interfaceid: *const ::windows_sys::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn DebugCreate(interfaceid: *const ::windows_sys::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn DebugCreateEx(interfaceid: *const ::windows_sys::core::GUID, dbgengoptions: u32, interface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugSetProcessKillOnExit(killonexit: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn DecodePointer(ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DecodeRemotePointer(processhandle: super::super::super::Foundation::HANDLE, ptr: *const ::core::ffi::c_void, decodedptr: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn DecodeSystemPointer(ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn EncodePointer(ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EncodeRemotePointer(processhandle: super::super::super::Foundation::HANDLE, ptr: *const ::core::ffi::c_void, encodedptr: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn EncodeSystemPointer(ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDirTree(hprocess: super::super::super::Foundation::HANDLE, rootpath: super::super::super::Foundation::PSTR, inputpathname: super::super::super::Foundation::PSTR, outputpathbuffer: super::super::super::Foundation::PSTR, cb: PENUMDIRTREE_CALLBACK, data: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDirTreeW(hprocess: super::super::super::Foundation::HANDLE, rootpath: super::super::super::Foundation::PWSTR, inputpathname: super::super::super::Foundation::PWSTR, outputpathbuffer: super::super::super::Foundation::PWSTR, cb: PENUMDIRTREE_CALLBACKW, data: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModules(hprocess: super::super::super::Foundation::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModules64(hprocess: super::super::super::Foundation::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACK64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModulesEx(hprocess: super::super::super::Foundation::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACK64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModulesExW(hprocess: super::super::super::Foundation::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACKW64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModulesW64(hprocess: super::super::super::Foundation::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACKW64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FatalAppExitA(uaction: u32, lpmessagetext: super::super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FatalAppExitW(uaction: u32, lpmessagetext: super::super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn FatalExit(exitcode: i32);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindDebugInfoFile(filename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, debugfilepath: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindDebugInfoFileEx(filename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, debugfilepath: super::super::super::Foundation::PSTR, callback: PFIND_DEBUG_FILE_CALLBACK, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindDebugInfoFileExW(filename: super::super::super::Foundation::PWSTR, symbolpath: super::super::super::Foundation::PWSTR, debugfilepath: super::super::super::Foundation::PWSTR, callback: PFIND_DEBUG_FILE_CALLBACKW, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableImage(filename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, imagefilepath: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableImageEx(filename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, imagefilepath: super::super::super::Foundation::PSTR, callback: PFIND_EXE_FILE_CALLBACK, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableImageExW(filename: super::super::super::Foundation::PWSTR, symbolpath: super::super::super::Foundation::PWSTR, imagefilepath: super::super::super::Foundation::PWSTR, callback: PFIND_EXE_FILE_CALLBACKW, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFileInPath(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PSTR, filename: super::super::super::Foundation::PSTR, id: *const ::core::ffi::c_void, two: u32, three: u32, flags: u32, filepath: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFileInSearchPath(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PSTR, filename: super::super::super::Foundation::PSTR, one: u32, two: u32, three: u32, filepath: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushInstructionCache(hprocess: super::super::super::Foundation::HANDLE, lpbaseaddress: *const ::core::ffi::c_void, dwsize: usize) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FormatMessageA(dwflags: FORMAT_MESSAGE_OPTIONS, lpsource: *const ::core::ffi::c_void, dwmessageid: u32, dwlanguageid: u32, lpbuffer: super::super::super::Foundation::PSTR, nsize: u32, arguments: *const *const i8) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FormatMessageW(dwflags: FORMAT_MESSAGE_OPTIONS, lpsource: *const ::core::ffi::c_void, dwmessageid: u32, dwlanguageid: u32, lpbuffer: super::super::super::Foundation::PWSTR, nsize: u32, arguments: *const *const i8) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    pub fn GetEnabledXStateFeatures() -> u64;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn GetErrorMode() -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetImageConfigInformation(loadedimage: *const LOADED_IMAGE, imageconfiginformation: *mut IMAGE_LOAD_CONFIG_DIRECTORY64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetImageConfigInformation(loadedimage: *const LOADED_IMAGE, imageconfiginformation: *mut IMAGE_LOAD_CONFIG_DIRECTORY32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetImageUnusedHeaderBytes(loadedimage: *const LOADED_IMAGE, sizeunusedheaderbytes: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn GetSymLoadError() -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetThreadContext(hthread: super::super::super::Foundation::HANDLE, lpcontext: *mut CONTEXT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn GetThreadErrorMode() -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadSelectorEntry(hthread: super::super::super::Foundation::HANDLE, dwselector: u32, lpselectorentry: *mut LDT_ENTRY) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadWaitChain(wcthandle: *const ::core::ffi::c_void, context: usize, flags: WAIT_CHAIN_THREAD_OPTIONS, threadid: u32, nodecount: *mut u32, nodeinfoarray: *mut WAITCHAIN_NODE_INFO, iscycle: *mut i32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimestampForLoadedLibrary(module: super::super::super::Foundation::HINSTANCE) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetXStateFeaturesMask(context: *const CONTEXT, featuremask: *mut u64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_Security_WinTrust`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_WinTrust"))]
    pub fn ImageAddCertificate(filehandle: super::super::super::Foundation::HANDLE, certificate: *const super::super::super::Security::WinTrust::WIN_CERTIFICATE, index: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageDirectoryEntryToData(base: *const ::core::ffi::c_void, mappedasimage: super::super::super::Foundation::BOOLEAN, directoryentry: IMAGE_DIRECTORY_ENTRY, size: *mut u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageDirectoryEntryToDataEx(base: *const ::core::ffi::c_void, mappedasimage: super::super::super::Foundation::BOOLEAN, directoryentry: IMAGE_DIRECTORY_ENTRY, size: *mut u32, foundheader: *mut *mut IMAGE_SECTION_HEADER) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageEnumerateCertificates(filehandle: super::super::super::Foundation::HANDLE, typefilter: u16, certificatecount: *mut u32, indices: *mut u32, indexcount: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_Security_WinTrust`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_WinTrust"))]
    pub fn ImageGetCertificateData(filehandle: super::super::super::Foundation::HANDLE, certificateindex: u32, certificate: *mut super::super::super::Security::WinTrust::WIN_CERTIFICATE, requiredlength: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_Security_WinTrust`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_WinTrust"))]
    pub fn ImageGetCertificateHeader(filehandle: super::super::super::Foundation::HANDLE, certificateindex: u32, certificateheader: *mut super::super::super::Security::WinTrust::WIN_CERTIFICATE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageGetDigestStream(filehandle: super::super::super::Foundation::HANDLE, digestlevel: u32, digestfunction: DIGEST_FUNCTION, digesthandle: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn ImageLoad(dllname: super::super::super::Foundation::PSTR, dllpath: super::super::super::Foundation::PSTR) -> *mut LOADED_IMAGE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn ImageNtHeader(base: *const ::core::ffi::c_void) -> *mut IMAGE_NT_HEADERS64;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn ImageNtHeader(base: *const ::core::ffi::c_void) -> *mut IMAGE_NT_HEADERS32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageRemoveCertificate(filehandle: super::super::super::Foundation::HANDLE, index: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn ImageRvaToSection(ntheaders: *const IMAGE_NT_HEADERS64, base: *const ::core::ffi::c_void, rva: u32) -> *mut IMAGE_SECTION_HEADER;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn ImageRvaToSection(ntheaders: *const IMAGE_NT_HEADERS32, base: *const ::core::ffi::c_void, rva: u32) -> *mut IMAGE_SECTION_HEADER;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn ImageRvaToVa(ntheaders: *const IMAGE_NT_HEADERS64, base: *const ::core::ffi::c_void, rva: u32, lastrvasection: *const *const IMAGE_SECTION_HEADER) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn ImageRvaToVa(ntheaders: *const IMAGE_NT_HEADERS32, base: *const ::core::ffi::c_void, rva: u32, lastrvasection: *const *const IMAGE_SECTION_HEADER) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn ImageUnload(loadedimage: *mut LOADED_IMAGE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn ImagehlpApiVersion() -> *mut API_VERSION;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn ImagehlpApiVersionEx(appversion: *const API_VERSION) -> *mut API_VERSION;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn InitializeContext(buffer: *mut ::core::ffi::c_void, contextflags: u32, context: *mut *mut CONTEXT, contextlength: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn InitializeContext2(buffer: *mut ::core::ffi::c_void, contextflags: u32, context: *mut *mut CONTEXT, contextlength: *mut u32, xstatecompactionmask: u64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDebuggerPresent() -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn LocateXStateFeature(context: *const CONTEXT, featureid: u32, length: *mut u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MakeSureDirectoryPathExists(dirpath: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn MapAndLoad(imagename: super::super::super::Foundation::PSTR, dllpath: super::super::super::Foundation::PSTR, loadedimage: *mut LOADED_IMAGE, dotdll: super::super::super::Foundation::BOOL, readonly: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapFileAndCheckSumA(filename: super::super::super::Foundation::PSTR, headersum: *mut u32, checksum: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapFileAndCheckSumW(filename: super::super::super::Foundation::PWSTR, headersum: *mut u32, checksum: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MessageBeep(utype: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MiniDumpReadDumpStream(baseofdump: *const ::core::ffi::c_void, streamnumber: u32, dir: *mut *mut MINIDUMP_DIRECTORY, streampointer: *mut *mut ::core::ffi::c_void, streamsize: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_Storage_FileSystem`, `Win32_System_Kernel`, `Win32_System_Memory`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Memory"))]
    pub fn MiniDumpWriteDump(hprocess: super::super::super::Foundation::HANDLE, processid: u32, hfile: super::super::super::Foundation::HANDLE, dumptype: MINIDUMP_TYPE, exceptionparam: *const MINIDUMP_EXCEPTION_INFORMATION, userstreamparam: *const MINIDUMP_USER_STREAM_INFORMATION, callbackparam: *const MINIDUMP_CALLBACK_INFORMATION) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenThreadWaitChainSession(flags: OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS, callback: PWAITCHAINCALLBACK) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OutputDebugStringA(lpoutputstring: super::super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OutputDebugStringW(lpoutputstring: super::super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn RaiseException(dwexceptioncode: u32, dwexceptionflags: u32, nnumberofarguments: u32, lparguments: *const usize);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RaiseFailFastException(pexceptionrecord: *const EXCEPTION_RECORD, pcontextrecord: *const CONTEXT, dwflags: u32);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RangeMapAddPeImageSections(rmaphandle: *const ::core::ffi::c_void, imagename: super::super::super::Foundation::PWSTR, mappedimage: *const ::core::ffi::c_void, mappingbytes: u32, imagebase: u64, usertag: u64, mappingflags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn RangeMapCreate() -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn RangeMapFree(rmaphandle: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RangeMapRead(rmaphandle: *const ::core::ffi::c_void, offset: u64, buffer: *mut ::core::ffi::c_void, requestbytes: u32, flags: u32, donebytes: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RangeMapRemove(rmaphandle: *const ::core::ffi::c_void, usertag: u64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RangeMapWrite(rmaphandle: *const ::core::ffi::c_void, offset: u64, buffer: *const ::core::ffi::c_void, requestbytes: u32, flags: u32, donebytes: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReBaseImage(currentimagename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, frebase: super::super::super::Foundation::BOOL, frebasesysfileok: super::super::super::Foundation::BOOL, fgoingdown: super::super::super::Foundation::BOOL, checkimagesize: u32, oldimagesize: *mut u32, oldimagebase: *mut usize, newimagesize: *mut u32, newimagebase: *mut usize, timestamp: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReBaseImage64(currentimagename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, frebase: super::super::super::Foundation::BOOL, frebasesysfileok: super::super::super::Foundation::BOOL, fgoingdown: super::super::super::Foundation::BOOL, checkimagesize: u32, oldimagesize: *mut u32, oldimagebase: *mut u64, newimagesize: *mut u32, newimagebase: *mut u64, timestamp: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadProcessMemory(hprocess: super::super::super::Foundation::HANDLE, lpbaseaddress: *const ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, nsize: usize, lpnumberofbytesread: *mut usize) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn RegisterWaitChainCOMCallback(callstatecallback: PCOGETCALLSTATE, activationstatecallback: PCOGETACTIVATIONSTATE);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveInvalidModuleList(hprocess: super::super::super::Foundation::HANDLE);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn RemoveVectoredContinueHandler(handle: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn RemoveVectoredExceptionHandler(handle: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportSymbolLoadSummary(hprocess: super::super::super::Foundation::HANDLE, ploadmodule: super::super::super::Foundation::PWSTR, psymboldata: *const DBGHELP_DATA_REPORT_STRUCT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlAddFunctionTable(functiontable: *const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY, entrycount: u32, baseaddress: usize) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlAddFunctionTable(functiontable: *const IMAGE_RUNTIME_FUNCTION_ENTRY, entrycount: u32, baseaddress: u64) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "aarch64",))]
    pub fn RtlAddGrowableFunctionTable(dynamictable: *mut *mut ::core::ffi::c_void, functiontable: *const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY, entrycount: u32, maximumentrycount: u32, rangebase: usize, rangeend: usize) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64",))]
    pub fn RtlAddGrowableFunctionTable(dynamictable: *mut *mut ::core::ffi::c_void, functiontable: *const IMAGE_RUNTIME_FUNCTION_ENTRY, entrycount: u32, maximumentrycount: u32, rangebase: usize, rangeend: usize) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`*"]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn RtlCaptureContext(contextrecord: *mut CONTEXT);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86_64",))]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn RtlCaptureContext2(contextrecord: *mut CONTEXT);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn RtlCaptureStackBackTrace(framestoskip: u32, framestocapture: u32, backtrace: *mut *mut ::core::ffi::c_void, backtracehash: *mut u32) -> u16;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlDeleteFunctionTable(functiontable: *const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlDeleteFunctionTable(functiontable: *const IMAGE_RUNTIME_FUNCTION_ENTRY) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn RtlDeleteGrowableFunctionTable(dynamictable: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn RtlGrowFunctionTable(dynamictable: *mut ::core::ffi::c_void, newentrycount: u32);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlInstallFunctionTableCallback(tableidentifier: u64, baseaddress: u64, length: u32, callback: PGET_RUNTIME_FUNCTION_CALLBACK, context: *const ::core::ffi::c_void, outofprocesscallbackdll: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "aarch64",))]
    pub fn RtlLookupFunctionEntry(controlpc: usize, imagebase: *mut usize, historytable: *mut UNWIND_HISTORY_TABLE) -> *mut IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64",))]
    pub fn RtlLookupFunctionEntry(controlpc: u64, imagebase: *mut u64, historytable: *mut UNWIND_HISTORY_TABLE) -> *mut IMAGE_RUNTIME_FUNCTION_ENTRY;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn RtlPcToFileHeader(pcvalue: *const ::core::ffi::c_void, baseofimage: *mut *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlRaiseException(exceptionrecord: *const EXCEPTION_RECORD);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlRestoreContext(contextrecord: *const CONTEXT, exceptionrecord: *const EXCEPTION_RECORD);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlUnwind(targetframe: *const ::core::ffi::c_void, targetip: *const ::core::ffi::c_void, exceptionrecord: *const EXCEPTION_RECORD, returnvalue: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlUnwindEx(targetframe: *const ::core::ffi::c_void, targetip: *const ::core::ffi::c_void, exceptionrecord: *const EXCEPTION_RECORD, returnvalue: *const ::core::ffi::c_void, contextrecord: *const CONTEXT, historytable: *const UNWIND_HISTORY_TABLE);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlVirtualUnwind(handlertype: RTL_VIRTUAL_UNWIND_HANDLER_TYPE, imagebase: usize, controlpc: usize, functionentry: *const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY, contextrecord: *mut CONTEXT, handlerdata: *mut *mut ::core::ffi::c_void, establisherframe: *mut usize, contextpointers: *mut KNONVOLATILE_CONTEXT_POINTERS_ARM64) -> ::core::option::Option<super::super::Kernel::EXCEPTION_ROUTINE>;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86_64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlVirtualUnwind(handlertype: RTL_VIRTUAL_UNWIND_HANDLER_TYPE, imagebase: u64, controlpc: u64, functionentry: *const IMAGE_RUNTIME_FUNCTION_ENTRY, contextrecord: *mut CONTEXT, handlerdata: *mut *mut ::core::ffi::c_void, establisherframe: *mut u64, contextpointers: *mut KNONVOLATILE_CONTEXT_POINTERS) -> ::core::option::Option<super::super::Kernel::EXCEPTION_ROUTINE>;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SearchTreeForFile(rootpath: super::super::super::Foundation::PSTR, inputpathname: super::super::super::Foundation::PSTR, outputpathbuffer: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SearchTreeForFileW(rootpath: super::super::super::Foundation::PWSTR, inputpathname: super::super::super::Foundation::PWSTR, outputpathbuffer: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn SetCheckUserInterruptShared(lpstartaddress: LPCALL_BACK_USER_INTERRUPT_ROUTINE);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn SetErrorMode(umode: THREAD_ERROR_MODE) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetImageConfigInformation(loadedimage: *mut LOADED_IMAGE, imageconfiginformation: *const IMAGE_LOAD_CONFIG_DIRECTORY64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetImageConfigInformation(loadedimage: *mut LOADED_IMAGE, imageconfiginformation: *const IMAGE_LOAD_CONFIG_DIRECTORY32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn SetSymLoadError(error: u32);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetThreadContext(hthread: super::super::super::Foundation::HANDLE, lpcontext: *const CONTEXT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadErrorMode(dwnewmode: THREAD_ERROR_MODE, lpoldmode: *const THREAD_ERROR_MODE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetUnhandledExceptionFilter(lptoplevelexceptionfilter: LPTOP_LEVEL_EXCEPTION_FILTER) -> ::core::option::Option<LPTOP_LEVEL_EXCEPTION_FILTER>;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetXStateFeaturesMask(context: *mut CONTEXT, featuremask: u64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StackWalk(machinetype: u32, hprocess: super::super::super::Foundation::HANDLE, hthread: super::super::super::Foundation::HANDLE, stackframe: *mut STACKFRAME, contextrecord: *mut ::core::ffi::c_void, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE, translateaddress: PTRANSLATE_ADDRESS_ROUTINE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StackWalk64(machinetype: u32, hprocess: super::super::super::Foundation::HANDLE, hthread: super::super::super::Foundation::HANDLE, stackframe: *mut STACKFRAME64, contextrecord: *mut ::core::ffi::c_void, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64, translateaddress: PTRANSLATE_ADDRESS_ROUTINE64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StackWalkEx(machinetype: u32, hprocess: super::super::super::Foundation::HANDLE, hthread: super::super::super::Foundation::HANDLE, stackframe: *mut STACKFRAME_EX, contextrecord: *mut ::core::ffi::c_void, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64, translateaddress: PTRANSLATE_ADDRESS_ROUTINE64, flags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddSourceStream(hprocess: super::super::super::Foundation::HANDLE, base: u64, streamfile: super::super::super::Foundation::PSTR, buffer: *const u8, size: usize) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddSourceStreamA(hprocess: super::super::super::Foundation::HANDLE, base: u64, streamfile: super::super::super::Foundation::PSTR, buffer: *const u8, size: usize) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddSourceStreamW(hprocess: super::super::super::Foundation::HANDLE, base: u64, filespec: super::super::super::Foundation::PWSTR, buffer: *const u8, size: usize) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddSymbol(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, name: super::super::super::Foundation::PSTR, address: u64, size: u32, flags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddSymbolW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, name: super::super::super::Foundation::PWSTR, address: u64, size: u32, flags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddrIncludeInlineTrace(hprocess: super::super::super::Foundation::HANDLE, address: u64) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymCleanup(hprocess: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymCompareInlineTrace(hprocess: super::super::super::Foundation::HANDLE, address1: u64, inlinecontext1: u32, retaddress1: u64, address2: u64, retaddress2: u64) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymDeleteSymbol(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, name: super::super::super::Foundation::PSTR, address: u64, flags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymDeleteSymbolW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, name: super::super::super::Foundation::PWSTR, address: u64, flags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumLines(hprocess: super::super::super::Foundation::HANDLE, base: u64, obj: super::super::super::Foundation::PSTR, file: super::super::super::Foundation::PSTR, enumlinescallback: PSYM_ENUMLINES_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumLinesW(hprocess: super::super::super::Foundation::HANDLE, base: u64, obj: super::super::super::Foundation::PWSTR, file: super::super::super::Foundation::PWSTR, enumlinescallback: PSYM_ENUMLINES_CALLBACKW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumProcesses(enumprocessescallback: PSYM_ENUMPROCESSES_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceFileTokens(hprocess: super::super::super::Foundation::HANDLE, base: u64, callback: PENUMSOURCEFILETOKENSCALLBACK) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceFiles(hprocess: super::super::super::Foundation::HANDLE, modbase: u64, mask: super::super::super::Foundation::PSTR, cbsrcfiles: PSYM_ENUMSOURCEFILES_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceFilesW(hprocess: super::super::super::Foundation::HANDLE, modbase: u64, mask: super::super::super::Foundation::PWSTR, cbsrcfiles: PSYM_ENUMSOURCEFILES_CALLBACKW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceLines(hprocess: super::super::super::Foundation::HANDLE, base: u64, obj: super::super::super::Foundation::PSTR, file: super::super::super::Foundation::PSTR, line: u32, flags: u32, enumlinescallback: PSYM_ENUMLINES_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceLinesW(hprocess: super::super::super::Foundation::HANDLE, base: u64, obj: super::super::super::Foundation::PWSTR, file: super::super::super::Foundation::PWSTR, line: u32, flags: u32, enumlinescallback: PSYM_ENUMLINES_CALLBACKW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSym(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbols(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PSTR, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsEx(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PSTR, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: *const ::core::ffi::c_void, options: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsExW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PWSTR, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: *const ::core::ffi::c_void, options: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsForAddr(hprocess: super::super::super::Foundation::HANDLE, address: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsForAddrW(hprocess: super::super::super::Foundation::HANDLE, address: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PWSTR, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumTypes(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumTypesByName(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PSTR, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumTypesByNameW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PWSTR, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumTypesW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateModules(hprocess: super::super::super::Foundation::HANDLE, enummodulescallback: PSYM_ENUMMODULES_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateModules64(hprocess: super::super::super::Foundation::HANDLE, enummodulescallback: PSYM_ENUMMODULES_CALLBACK64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateModulesW64(hprocess: super::super::super::Foundation::HANDLE, enummodulescallback: PSYM_ENUMMODULES_CALLBACKW64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateSymbols(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u32, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateSymbols64(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateSymbolsW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u32, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACKW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateSymbolsW64(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK64W, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindDebugInfoFile(hprocess: super::super::super::Foundation::HANDLE, filename: super::super::super::Foundation::PSTR, debugfilepath: super::super::super::Foundation::PSTR, callback: PFIND_DEBUG_FILE_CALLBACK, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindDebugInfoFileW(hprocess: super::super::super::Foundation::HANDLE, filename: super::super::super::Foundation::PWSTR, debugfilepath: super::super::super::Foundation::PWSTR, callback: PFIND_DEBUG_FILE_CALLBACKW, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindExecutableImage(hprocess: super::super::super::Foundation::HANDLE, filename: super::super::super::Foundation::PSTR, imagefilepath: super::super::super::Foundation::PSTR, callback: PFIND_EXE_FILE_CALLBACK, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindExecutableImageW(hprocess: super::super::super::Foundation::HANDLE, filename: super::super::super::Foundation::PWSTR, imagefilepath: super::super::super::Foundation::PWSTR, callback: PFIND_EXE_FILE_CALLBACKW, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindFileInPath(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PSTR, filename: super::super::super::Foundation::PSTR, id: *const ::core::ffi::c_void, two: u32, three: u32, flags: SYM_FIND_ID_OPTION, foundfile: super::super::super::Foundation::PSTR, callback: PFINDFILEINPATHCALLBACK, context: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindFileInPathW(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PWSTR, filename: super::super::super::Foundation::PWSTR, id: *const ::core::ffi::c_void, two: u32, three: u32, flags: SYM_FIND_ID_OPTION, foundfile: super::super::super::Foundation::PWSTR, callback: PFINDFILEINPATHCALLBACKW, context: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromAddr(hprocess: super::super::super::Foundation::HANDLE, address: u64, displacement: *mut u64, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromAddrW(hprocess: super::super::super::Foundation::HANDLE, address: u64, displacement: *mut u64, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromIndex(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, index: u32, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromIndexW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, index: u32, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromInlineContext(hprocess: super::super::super::Foundation::HANDLE, address: u64, inlinecontext: u32, displacement: *mut u64, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromInlineContextW(hprocess: super::super::super::Foundation::HANDLE, address: u64, inlinecontext: u32, displacement: *mut u64, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromName(hprocess: super::super::super::Foundation::HANDLE, name: super::super::super::Foundation::PSTR, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromNameW(hprocess: super::super::super::Foundation::HANDLE, name: super::super::super::Foundation::PWSTR, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromToken(hprocess: super::super::super::Foundation::HANDLE, base: u64, token: u32, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromTokenW(hprocess: super::super::super::Foundation::HANDLE, base: u64, token: u32, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFunctionTableAccess(hprocess: super::super::super::Foundation::HANDLE, addrbase: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFunctionTableAccess64(hprocess: super::super::super::Foundation::HANDLE, addrbase: u64) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFunctionTableAccess64AccessRoutines(hprocess: super::super::super::Foundation::HANDLE, addrbase: u64, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetExtendedOption(option: IMAGEHLP_EXTENDED_OPTIONS) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetFileLineOffsets64(hprocess: super::super::super::Foundation::HANDLE, modulename: super::super::super::Foundation::PSTR, filename: super::super::super::Foundation::PSTR, buffer: *mut u64, bufferlines: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetHomeDirectory(r#type: IMAGEHLP_HD_TYPE, dir: super::super::super::Foundation::PSTR, size: usize) -> super::super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetHomeDirectoryW(r#type: IMAGEHLP_HD_TYPE, dir: super::super::super::Foundation::PWSTR, size: usize) -> super::super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromAddr(hprocess: super::super::super::Foundation::HANDLE, dwaddr: u32, pdwdisplacement: *mut u32, line: *mut IMAGEHLP_LINE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromAddr64(hprocess: super::super::super::Foundation::HANDLE, qwaddr: u64, pdwdisplacement: *mut u32, line64: *mut IMAGEHLP_LINE64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromAddrW64(hprocess: super::super::super::Foundation::HANDLE, dwaddr: u64, pdwdisplacement: *mut u32, line: *mut IMAGEHLP_LINEW64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromInlineContext(hprocess: super::super::super::Foundation::HANDLE, qwaddr: u64, inlinecontext: u32, qwmodulebaseaddress: u64, pdwdisplacement: *mut u32, line64: *mut IMAGEHLP_LINE64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromInlineContextW(hprocess: super::super::super::Foundation::HANDLE, dwaddr: u64, inlinecontext: u32, qwmodulebaseaddress: u64, pdwdisplacement: *mut u32, line: *mut IMAGEHLP_LINEW64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromName(hprocess: super::super::super::Foundation::HANDLE, modulename: super::super::super::Foundation::PSTR, filename: super::super::super::Foundation::PSTR, dwlinenumber: u32, pldisplacement: *mut i32, line: *mut IMAGEHLP_LINE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromName64(hprocess: super::super::super::Foundation::HANDLE, modulename: super::super::super::Foundation::PSTR, filename: super::super::super::Foundation::PSTR, dwlinenumber: u32, pldisplacement: *mut i32, line: *mut IMAGEHLP_LINE64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromNameW64(hprocess: super::super::super::Foundation::HANDLE, modulename: super::super::super::Foundation::PWSTR, filename: super::super::super::Foundation::PWSTR, dwlinenumber: u32, pldisplacement: *mut i32, line: *mut IMAGEHLP_LINEW64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineNext(hprocess: super::super::super::Foundation::HANDLE, line: *mut IMAGEHLP_LINE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineNext64(hprocess: super::super::super::Foundation::HANDLE, line: *mut IMAGEHLP_LINE64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineNextW64(hprocess: super::super::super::Foundation::HANDLE, line: *mut IMAGEHLP_LINEW64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLinePrev(hprocess: super::super::super::Foundation::HANDLE, line: *mut IMAGEHLP_LINE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLinePrev64(hprocess: super::super::super::Foundation::HANDLE, line: *mut IMAGEHLP_LINE64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLinePrevW64(hprocess: super::super::super::Foundation::HANDLE, line: *mut IMAGEHLP_LINEW64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleBase(hprocess: super::super::super::Foundation::HANDLE, dwaddr: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleBase64(hprocess: super::super::super::Foundation::HANDLE, qwaddr: u64) -> u64;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleInfo(hprocess: super::super::super::Foundation::HANDLE, dwaddr: u32, moduleinfo: *mut IMAGEHLP_MODULE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleInfo64(hprocess: super::super::super::Foundation::HANDLE, qwaddr: u64, moduleinfo: *mut IMAGEHLP_MODULE64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleInfoW(hprocess: super::super::super::Foundation::HANDLE, dwaddr: u32, moduleinfo: *mut IMAGEHLP_MODULEW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleInfoW64(hprocess: super::super::super::Foundation::HANDLE, qwaddr: u64, moduleinfo: *mut IMAGEHLP_MODULEW64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetOmaps(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, omapto: *mut *mut OMAP, comapto: *mut u64, omapfrom: *mut *mut OMAP, comapfrom: *mut u64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn SymGetOptions() -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetScope(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, index: u32, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetScopeW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, index: u32, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSearchPath(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PSTR, searchpathlength: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSearchPathW(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PWSTR, searchpathlength: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFile(hprocess: super::super::super::Foundation::HANDLE, base: u64, params: super::super::super::Foundation::PSTR, filespec: super::super::super::Foundation::PSTR, filepath: super::super::super::Foundation::PSTR, size: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileChecksum(hprocess: super::super::super::Foundation::HANDLE, base: u64, filespec: super::super::super::Foundation::PSTR, pchecksumtype: *mut u32, pchecksum: *mut u8, checksumsize: u32, pactualbyteswritten: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileChecksumW(hprocess: super::super::super::Foundation::HANDLE, base: u64, filespec: super::super::super::Foundation::PWSTR, pchecksumtype: *mut u32, pchecksum: *mut u8, checksumsize: u32, pactualbyteswritten: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileFromToken(hprocess: super::super::super::Foundation::HANDLE, token: *const ::core::ffi::c_void, params: super::super::super::Foundation::PSTR, filepath: super::super::super::Foundation::PSTR, size: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileFromTokenByTokenName(hprocess: super::super::super::Foundation::HANDLE, token: *const ::core::ffi::c_void, tokenname: super::super::super::Foundation::PSTR, params: super::super::super::Foundation::PSTR, filepath: super::super::super::Foundation::PSTR, size: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileFromTokenByTokenNameW(hprocess: super::super::super::Foundation::HANDLE, token: *const ::core::ffi::c_void, tokenname: super::super::super::Foundation::PWSTR, params: super::super::super::Foundation::PWSTR, filepath: super::super::super::Foundation::PWSTR, size: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileFromTokenW(hprocess: super::super::super::Foundation::HANDLE, token: *const ::core::ffi::c_void, params: super::super::super::Foundation::PWSTR, filepath: super::super::super::Foundation::PWSTR, size: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileToken(hprocess: super::super::super::Foundation::HANDLE, base: u64, filespec: super::super::super::Foundation::PSTR, token: *mut *mut ::core::ffi::c_void, size: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileTokenByTokenName(hprocess: super::super::super::Foundation::HANDLE, base: u64, filespec: super::super::super::Foundation::PSTR, tokenname: super::super::super::Foundation::PSTR, tokenparameters: super::super::super::Foundation::PSTR, token: *mut *mut ::core::ffi::c_void, size: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileTokenByTokenNameW(hprocess: super::super::super::Foundation::HANDLE, base: u64, filespec: super::super::super::Foundation::PWSTR, tokenname: super::super::super::Foundation::PWSTR, tokenparameters: super::super::super::Foundation::PWSTR, token: *mut *mut ::core::ffi::c_void, size: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileTokenW(hprocess: super::super::super::Foundation::HANDLE, base: u64, filespec: super::super::super::Foundation::PWSTR, token: *mut *mut ::core::ffi::c_void, size: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileW(hprocess: super::super::super::Foundation::HANDLE, base: u64, params: super::super::super::Foundation::PWSTR, filespec: super::super::super::Foundation::PWSTR, filepath: super::super::super::Foundation::PWSTR, size: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceVarFromToken(hprocess: super::super::super::Foundation::HANDLE, token: *const ::core::ffi::c_void, params: super::super::super::Foundation::PSTR, varname: super::super::super::Foundation::PSTR, value: super::super::super::Foundation::PSTR, size: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceVarFromTokenW(hprocess: super::super::super::Foundation::HANDLE, token: *const ::core::ffi::c_void, params: super::super::super::Foundation::PWSTR, varname: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::PWSTR, size: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymFromAddr(hprocess: super::super::super::Foundation::HANDLE, dwaddr: u32, pdwdisplacement: *mut u32, symbol: *mut IMAGEHLP_SYMBOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymFromAddr64(hprocess: super::super::super::Foundation::HANDLE, qwaddr: u64, pdwdisplacement: *mut u64, symbol: *mut IMAGEHLP_SYMBOL64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymFromName(hprocess: super::super::super::Foundation::HANDLE, name: super::super::super::Foundation::PSTR, symbol: *mut IMAGEHLP_SYMBOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymFromName64(hprocess: super::super::super::Foundation::HANDLE, name: super::super::super::Foundation::PSTR, symbol: *mut IMAGEHLP_SYMBOL64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymNext(hprocess: super::super::super::Foundation::HANDLE, symbol: *mut IMAGEHLP_SYMBOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymNext64(hprocess: super::super::super::Foundation::HANDLE, symbol: *mut IMAGEHLP_SYMBOL64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymPrev(hprocess: super::super::super::Foundation::HANDLE, symbol: *mut IMAGEHLP_SYMBOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymPrev64(hprocess: super::super::super::Foundation::HANDLE, symbol: *mut IMAGEHLP_SYMBOL64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymbolFile(hprocess: super::super::super::Foundation::HANDLE, sympath: super::super::super::Foundation::PSTR, imagefile: super::super::super::Foundation::PSTR, r#type: IMAGEHLP_SF_TYPE, symbolfile: super::super::super::Foundation::PSTR, csymbolfile: usize, dbgfile: super::super::super::Foundation::PSTR, cdbgfile: usize) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymbolFileW(hprocess: super::super::super::Foundation::HANDLE, sympath: super::super::super::Foundation::PWSTR, imagefile: super::super::super::Foundation::PWSTR, r#type: IMAGEHLP_SF_TYPE, symbolfile: super::super::super::Foundation::PWSTR, csymbolfile: usize, dbgfile: super::super::super::Foundation::PWSTR, cdbgfile: usize) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetTypeFromName(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, name: super::super::super::Foundation::PSTR, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetTypeFromNameW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, name: super::super::super::Foundation::PWSTR, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetTypeInfo(hprocess: super::super::super::Foundation::HANDLE, modbase: u64, typeid: u32, gettype: IMAGEHLP_SYMBOL_TYPE_INFO, pinfo: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetTypeInfoEx(hprocess: super::super::super::Foundation::HANDLE, modbase: u64, params: *mut IMAGEHLP_GET_TYPE_INFO_PARAMS) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetUnwindInfo(hprocess: super::super::super::Foundation::HANDLE, address: u64, buffer: *mut ::core::ffi::c_void, size: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymInitialize(hprocess: super::super::super::Foundation::HANDLE, usersearchpath: super::super::super::Foundation::PSTR, finvadeprocess: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymInitializeW(hprocess: super::super::super::Foundation::HANDLE, usersearchpath: super::super::super::Foundation::PWSTR, finvadeprocess: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymLoadModule(hprocess: super::super::super::Foundation::HANDLE, hfile: super::super::super::Foundation::HANDLE, imagename: super::super::super::Foundation::PSTR, modulename: super::super::super::Foundation::PSTR, baseofdll: u32, sizeofdll: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymLoadModule64(hprocess: super::super::super::Foundation::HANDLE, hfile: super::super::super::Foundation::HANDLE, imagename: super::super::super::Foundation::PSTR, modulename: super::super::super::Foundation::PSTR, baseofdll: u64, sizeofdll: u32) -> u64;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymLoadModuleEx(hprocess: super::super::super::Foundation::HANDLE, hfile: super::super::super::Foundation::HANDLE, imagename: super::super::super::Foundation::PSTR, modulename: super::super::super::Foundation::PSTR, baseofdll: u64, dllsize: u32, data: *const MODLOAD_DATA, flags: SYM_LOAD_FLAGS) -> u64;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymLoadModuleExW(hprocess: super::super::super::Foundation::HANDLE, hfile: super::super::super::Foundation::HANDLE, imagename: super::super::super::Foundation::PWSTR, modulename: super::super::super::Foundation::PWSTR, baseofdll: u64, dllsize: u32, data: *const MODLOAD_DATA, flags: SYM_LOAD_FLAGS) -> u64;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymMatchFileName(filename: super::super::super::Foundation::PSTR, r#match: super::super::super::Foundation::PSTR, filenamestop: *mut super::super::super::Foundation::PSTR, matchstop: *mut super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymMatchFileNameW(filename: super::super::super::Foundation::PWSTR, r#match: super::super::super::Foundation::PWSTR, filenamestop: *mut super::super::super::Foundation::PWSTR, matchstop: *mut super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymMatchString(string: super::super::super::Foundation::PSTR, expression: super::super::super::Foundation::PSTR, fcase: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymMatchStringA(string: super::super::super::Foundation::PSTR, expression: super::super::super::Foundation::PSTR, fcase: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymMatchStringW(string: super::super::super::Foundation::PWSTR, expression: super::super::super::Foundation::PWSTR, fcase: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymNext(hprocess: super::super::super::Foundation::HANDLE, si: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymNextW(hprocess: super::super::super::Foundation::HANDLE, siw: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymPrev(hprocess: super::super::super::Foundation::HANDLE, si: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymPrevW(hprocess: super::super::super::Foundation::HANDLE, siw: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymQueryInlineTrace(hprocess: super::super::super::Foundation::HANDLE, startaddress: u64, startcontext: u32, startretaddress: u64, curaddress: u64, curcontext: *mut u32, curframeindex: *mut u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRefreshModuleList(hprocess: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterCallback(hprocess: super::super::super::Foundation::HANDLE, callbackfunction: PSYMBOL_REGISTERED_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterCallback64(hprocess: super::super::super::Foundation::HANDLE, callbackfunction: PSYMBOL_REGISTERED_CALLBACK64, usercontext: u64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterCallbackW64(hprocess: super::super::super::Foundation::HANDLE, callbackfunction: PSYMBOL_REGISTERED_CALLBACK64, usercontext: u64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterFunctionEntryCallback(hprocess: super::super::super::Foundation::HANDLE, callbackfunction: PSYMBOL_FUNCENTRY_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterFunctionEntryCallback64(hprocess: super::super::super::Foundation::HANDLE, callbackfunction: PSYMBOL_FUNCENTRY_CALLBACK64, usercontext: u64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSearch(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, index: u32, symtag: u32, mask: super::super::super::Foundation::PSTR, address: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: *const ::core::ffi::c_void, options: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSearchW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, index: u32, symtag: u32, mask: super::super::super::Foundation::PWSTR, address: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: *const ::core::ffi::c_void, options: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetContext(hprocess: super::super::super::Foundation::HANDLE, stackframe: *const IMAGEHLP_STACK_FRAME, context: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetExtendedOption(option: IMAGEHLP_EXTENDED_OPTIONS, value: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetHomeDirectory(hprocess: super::super::super::Foundation::HANDLE, dir: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetHomeDirectoryW(hprocess: super::super::super::Foundation::HANDLE, dir: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn SymSetOptions(symoptions: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetParentWindow(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetScopeFromAddr(hprocess: super::super::super::Foundation::HANDLE, address: u64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetScopeFromIndex(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, index: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetScopeFromInlineContext(hprocess: super::super::super::Foundation::HANDLE, address: u64, inlinecontext: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetSearchPath(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetSearchPathW(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvDeltaName(hprocess: super::super::super::Foundation::HANDLE, sympath: super::super::super::Foundation::PSTR, r#type: super::super::super::Foundation::PSTR, file1: super::super::super::Foundation::PSTR, file2: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvDeltaNameW(hprocess: super::super::super::Foundation::HANDLE, sympath: super::super::super::Foundation::PWSTR, r#type: super::super::super::Foundation::PWSTR, file1: super::super::super::Foundation::PWSTR, file2: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexInfo(file: super::super::super::Foundation::PSTR, info: *mut SYMSRV_INDEX_INFO, flags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexInfoW(file: super::super::super::Foundation::PWSTR, info: *mut SYMSRV_INDEX_INFOW, flags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexString(hprocess: super::super::super::Foundation::HANDLE, srvpath: super::super::super::Foundation::PSTR, file: super::super::super::Foundation::PSTR, index: super::super::super::Foundation::PSTR, size: usize, flags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexStringW(hprocess: super::super::super::Foundation::HANDLE, srvpath: super::super::super::Foundation::PWSTR, file: super::super::super::Foundation::PWSTR, index: super::super::super::Foundation::PWSTR, size: usize, flags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexes(file: super::super::super::Foundation::PSTR, id: *mut ::windows_sys::core::GUID, val1: *mut u32, val2: *mut u32, flags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexesW(file: super::super::super::Foundation::PWSTR, id: *mut ::windows_sys::core::GUID, val1: *mut u32, val2: *mut u32, flags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetSupplement(hprocess: super::super::super::Foundation::HANDLE, sympath: super::super::super::Foundation::PSTR, node: super::super::super::Foundation::PSTR, file: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetSupplementW(hprocess: super::super::super::Foundation::HANDLE, sympath: super::super::super::Foundation::PWSTR, node: super::super::super::Foundation::PWSTR, file: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvIsStore(hprocess: super::super::super::Foundation::HANDLE, path: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvIsStoreW(hprocess: super::super::super::Foundation::HANDLE, path: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvStoreFile(hprocess: super::super::super::Foundation::HANDLE, srvpath: super::super::super::Foundation::PSTR, file: super::super::super::Foundation::PSTR, flags: SYM_SRV_STORE_FILE_FLAGS) -> super::super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvStoreFileW(hprocess: super::super::super::Foundation::HANDLE, srvpath: super::super::super::Foundation::PWSTR, file: super::super::super::Foundation::PWSTR, flags: SYM_SRV_STORE_FILE_FLAGS) -> super::super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvStoreSupplement(hprocess: super::super::super::Foundation::HANDLE, srvpath: super::super::super::Foundation::PSTR, node: super::super::super::Foundation::PSTR, file: super::super::super::Foundation::PSTR, flags: u32) -> super::super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvStoreSupplementW(hprocess: super::super::super::Foundation::HANDLE, sympath: super::super::super::Foundation::PWSTR, node: super::super::super::Foundation::PWSTR, file: super::super::super::Foundation::PWSTR, flags: u32) -> super::super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymUnDName(sym: *const IMAGEHLP_SYMBOL, undecname: super::super::super::Foundation::PSTR, undecnamelength: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymUnDName64(sym: *const IMAGEHLP_SYMBOL64, undecname: super::super::super::Foundation::PSTR, undecnamelength: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymUnloadModule(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymUnloadModule64(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn TerminateProcessOnMemoryExhaustion(failedallocationsize: usize);
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TouchFileTimes(filehandle: super::super::super::Foundation::HANDLE, psystemtime: *const super::super::super::Foundation::SYSTEMTIME) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnDecorateSymbolName(name: super::super::super::Foundation::PSTR, outputstring: super::super::super::Foundation::PSTR, maxstringlength: u32, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnDecorateSymbolNameW(name: super::super::super::Foundation::PWSTR, outputstring: super::super::super::Foundation::PWSTR, maxstringlength: u32, flags: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn UnMapAndLoad(loadedimage: *mut LOADED_IMAGE) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn UnhandledExceptionFilter(exceptioninfo: *const EXCEPTION_POINTERS) -> i32;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateDebugInfoFile(imagefilename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, debugfilepath: super::super::super::Foundation::PSTR, ntheaders: *const IMAGE_NT_HEADERS32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateDebugInfoFileEx(imagefilename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, debugfilepath: super::super::super::Foundation::PSTR, ntheaders: *const IMAGE_NT_HEADERS32, oldchecksum: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn WaitForDebugEvent(lpdebugevent: *mut DEBUG_EVENT, dwmilliseconds: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn WaitForDebugEventEx(lpdebugevent: *mut DEBUG_EVENT, dwmilliseconds: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64GetThreadContext(hthread: super::super::super::Foundation::HANDLE, lpcontext: *mut WOW64_CONTEXT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64GetThreadSelectorEntry(hthread: super::super::super::Foundation::HANDLE, dwselector: u32, lpselectorentry: *mut WOW64_LDT_ENTRY) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64SetThreadContext(hthread: super::super::super::Foundation::HANDLE, lpcontext: *const WOW64_CONTEXT) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProcessMemory(hprocess: super::super::super::Foundation::HANDLE, lpbaseaddress: *const ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nsize: usize, lpnumberofbyteswritten: *mut usize) -> super::super::super::Foundation::BOOL;
}
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const ACTIVPROF_E_PROFILER_ABSENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220991i32 as _);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const ACTIVPROF_E_PROFILER_PRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220992i32 as _);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const ACTIVPROF_E_UNABLE_TO_APPLY_ACTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220990i32 as _);
pub struct ADDRESS(i32);
pub struct ADDRESS64(i32);
pub struct ADDRESS_MODE(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const ADDRESS_TYPE_INDEX_NOT_FOUND: u32 = 11u32;
pub struct AER_BRIDGE_DESCRIPTOR_FLAGS(i32);
pub struct AER_ENDPOINT_DESCRIPTOR_FLAGS(i32);
pub struct AER_ROOTPORT_DESCRIPTOR_FLAGS(i32);
pub struct API_VERSION(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const API_VERSION_NUMBER: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const APPBREAKFLAG_DEBUGGER_BLOCK: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const APPBREAKFLAG_DEBUGGER_HALT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const APPBREAKFLAG_IN_BREAKPOINT: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const APPBREAKFLAG_NESTED: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const APPBREAKFLAG_STEP: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const APPBREAKFLAG_STEPTYPE_BYTECODE: u32 = 1048576u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const APPBREAKFLAG_STEPTYPE_MACHINE: u32 = 2097152u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const APPBREAKFLAG_STEPTYPE_MASK: u32 = 15728640u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const APPBREAKFLAG_STEPTYPE_SOURCE: u32 = 0u32;
pub struct APPLICATION_NODE_EVENT_FILTER(i32);
pub struct ARM64_NT_CONTEXT(i32);
pub struct ARM64_NT_NEON128(i32);
pub struct ArrayDimension(i32);
pub struct AsyncIDebugApplicationNodeEvents(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const BIND_ALL_IMAGES: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const BIND_CACHE_IMPORT_DLLS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const BIND_NO_BOUND_IMPORTS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const BIND_NO_UPDATE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const BIND_REPORT_64BIT_VA: u32 = 16u32;
pub struct BREAKPOINT_STATE(i32);
pub struct BREAKREASON(i32);
pub struct BREAKRESUME_ACTION(i32);
pub struct BUGCHECK_ERROR(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CANNOT_ALLOCATE_MEMORY: u32 = 9u32;
pub const CATID_ActiveScript: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4038566305, data2: 38983, data3: 4559, data4: [143, 32, 0, 128, 95, 44, 208, 100] };
pub const CATID_ActiveScriptAuthor: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 183380626, data2: 48315, data3: 4560, data4: [140, 114, 0, 192, 79, 194, 176, 133] };
pub const CATID_ActiveScriptEncode: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4038566307, data2: 38983, data3: 4559, data4: [143, 32, 0, 128, 95, 44, 208, 100] };
pub const CATID_ActiveScriptParse: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4038566306, data2: 38983, data3: 4559, data4: [143, 32, 0, 128, 95, 44, 208, 100] };
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_CHECK_ARM_MACHINE_THUMB_TYPE_OVERRIDE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_CHECK_ENGOPT_DISALLOW_NETWORK_PATHS: u32 = 1879048192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_DEBUG_INFO: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_DEFERRED_SYMBOL_LOAD_CANCEL: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_DEFERRED_SYMBOL_LOAD_COMPLETE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_DEFERRED_SYMBOL_LOAD_FAILURE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_DEFERRED_SYMBOL_LOAD_PARTIAL: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_DEFERRED_SYMBOL_LOAD_START: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_DUPLICATE_SYMBOL: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_ENGINE_PRESENT: u32 = 1610612736u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_EVENT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_MAP_JIT_SYMBOL: u32 = 2684354560u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_READ_MEMORY: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_SET_OPTIONS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_SRCSRV_EVENT: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_SRCSRV_INFO: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_SYMBOLS_UNLOADED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_UPDATE_STATUS_BAR: u32 = 1342177280u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CBA_XML_LOG: u32 = 2415919104u32;
pub struct CDebugDocumentHelper(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CERT_PE_IMAGE_DIGEST_ALL_IMPORT_INFO: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CERT_PE_IMAGE_DIGEST_DEBUG_INFO: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CERT_PE_IMAGE_DIGEST_NON_PE_INFO: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CERT_PE_IMAGE_DIGEST_RESOURCES: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CERT_SECTION_TYPE_ANY: u32 = 255u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CHECKSUM_MAPVIEW_FAILURE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CHECKSUM_MAP_FAILURE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CHECKSUM_OPEN_FAILURE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CHECKSUM_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CHECKSUM_UNICODE_FAILURE: u32 = 4u32;
pub struct CONTEXT(i32);
pub struct CONTEXT(i32);
pub struct CONTEXT(i32);
pub struct CPU_INFORMATION(i32);
pub struct CREATE_PROCESS_DEBUG_INFO(i32);
pub struct CREATE_THREAD_DEBUG_INFO(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CROSS_PLATFORM_MAXIMUM_PROCESSORS: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const CURRENT_KD_SECONDARY_VERSION: u32 = 2u32;
pub struct CallingConventionKind(i32);
pub struct DBGHELP_DATA_REPORT_STRUCT(i32);
pub struct DBGKD_DEBUG_DATA_HEADER32(i32);
pub struct DBGKD_DEBUG_DATA_HEADER64(i32);
pub struct DBGKD_GET_VERSION32(i32);
pub struct DBGKD_GET_VERSION64(i32);
pub struct DBGKD_MAJOR_TYPES(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBGKD_SIMULATION_EXDI: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBGKD_SIMULATION_NONE: i32 = 0i32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBGKD_VERS_FLAG_DATA: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBGKD_VERS_FLAG_HAL_IN_NTOS: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBGKD_VERS_FLAG_HSS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBGKD_VERS_FLAG_MP: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBGKD_VERS_FLAG_NOMM: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBGKD_VERS_FLAG_PARTITIONS: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBGKD_VERS_FLAG_PTR64: u32 = 4u32;
pub struct DBGPROP_ATTRIB_FLAGS(i32);
pub struct DBGPROP_INFO(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_ADDRESS_AT_END: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_ADDRESS_OF_FIELD: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_ARRAY: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_BLOCK_RECURSE: u32 = 2097152u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_CALL_FOR_EACH: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_COMPACT_OUT: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_COPY_TYPE_DATA: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_FIELD_ARRAY: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_FIELD_CALL_BEFORE_PRINT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_FIELD_COPY_FIELD_DATA: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_FIELD_DEFAULT_STRING: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_FIELD_FULL_NAME: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_FIELD_GUID_STRING: u32 = 524288u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_FIELD_MULTI_STRING: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_FIELD_NO_CALLBACK_REQ: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_FIELD_NO_PRINT: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_FIELD_RECUR_ON_THIS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_FIELD_RETURN_ADDRESS: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_FIELD_SIZE_IN_BITS: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_FIELD_UTF32_STRING: u32 = 1048576u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_FIELD_WCHAR_STRING: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_FUNCTION_FORMAT: u32 = 1048576u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_GET_SIZE_ONLY: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_LIST: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_MATCH_SIZE: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_NO_INDENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_NO_OFFSET: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_NO_PRINT: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_READ_PHYSICAL: u32 = 524288u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_DUMP_VERBOSE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_FRAME_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_FRAME_IGNORE_INLINE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_RETURN_SUBTYPES: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_RETURN_TYPE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBG_RETURN_TYPE_VALUES: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DBHHEADER_PDBGUID: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ADDSYNTHMOD_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ADDSYNTHMOD_ZEROBASE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ADDSYNTHSYM_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ANY_ID: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ASMOPT_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ASMOPT_IGNORE_OUTPUT_WIDTH: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ASMOPT_NO_CODE_BYTES: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ASMOPT_SOURCE_LINE_NUMBER: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ASMOPT_VERBOSE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ATTACH_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ATTACH_EXDI_DRIVER: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ATTACH_EXISTING: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ATTACH_INSTALL_DRIVER: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ATTACH_INVASIVE_NO_INITIAL_BREAK: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ATTACH_INVASIVE_RESUME_PROCESS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ATTACH_KERNEL_CONNECTION: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ATTACH_LOCAL_KERNEL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ATTACH_NONINVASIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ATTACH_NONINVASIVE_ALLOW_PARTIAL: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ATTACH_NONINVASIVE_NO_SUSPEND: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_BREAKPOINT_ADDER_ONLY: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_BREAKPOINT_CODE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_BREAKPOINT_DATA: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_BREAKPOINT_DEFERRED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_BREAKPOINT_ENABLED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_BREAKPOINT_GO_ONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_BREAKPOINT_INLINE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_BREAKPOINT_ONE_SHOT: u32 = 16u32;
pub struct DEBUG_BREAKPOINT_PARAMETERS(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_BREAKPOINT_TIME: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_BREAK_EXECUTE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_BREAK_IO: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_BREAK_READ: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_BREAK_WRITE: u32 = 2u32;
pub struct DEBUG_CACHED_SYMBOL_INFO(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_ALL: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_DATA: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_ADDBREAKPOINT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_EVALUATE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_EXECUTE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_EXECUTECOMMANDFILE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_INLINESTEP: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_INLINESTEP_PSEUDO: u32 = 17u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_REMOVEBREAKPOINT: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_SETSCOPE: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_SETSCOPEFRAMEBYINDEX: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_SETSCOPEFROMJITDEBUGINFO: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_SETSCOPEFROMSTOREDEVENT: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_SETVALUE: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_SETVALUE2: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_WRITEPHYSICAL: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_WRITEPHYSICAL2: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_WRITEVIRTUAL: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REFRESH_WRITEVIRTUALUNCACHED: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CDS_REGISTERS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CES_ALL: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CES_ASSEMBLY_OPTIONS: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CES_BREAKPOINTS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CES_CODE_LEVEL: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CES_CURRENT_THREAD: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CES_EFFECTIVE_PROCESSOR: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CES_ENGINE_OPTIONS: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CES_EVENT_FILTERS: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CES_EXECUTION_STATUS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CES_EXPRESSION_SYNTAX: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CES_EXTENSIONS: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CES_LOG_FILE: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CES_PROCESS_OPTIONS: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CES_RADIX: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CES_SYSTEMS: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CES_TEXT_REPLACEMENTS: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CLASS_IMAGE_FILE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CLASS_KERNEL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CLASS_UNINITIALIZED: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CLASS_USER_WINDOWS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CLIENT_CDB: u32 = 4u32;
pub struct DEBUG_CLIENT_CONTEXT(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CLIENT_KD: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CLIENT_NTKD: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CLIENT_NTSD: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CLIENT_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CLIENT_VSINT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CLIENT_WINDBG: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CLIENT_WINIDE: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CMDEX_ADD_EVENT_STRING: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CMDEX_INVALID: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CMDEX_RESET_EVENT_STRINGS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_COMMAND_EXCEPTION_ID: u32 = 3688893886u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CONNECT_SESSION_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CONNECT_SESSION_NO_ANNOUNCE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CONNECT_SESSION_NO_VERSION: u32 = 1u32;
pub struct DEBUG_CREATE_PROCESS_OPTIONS(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CSS_ALL: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CSS_COLLAPSE_CHILDREN: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CSS_LOADS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CSS_PATHS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CSS_SCOPE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CSS_SYMBOL_OPTIONS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CSS_TYPE_OPTIONS: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CSS_UNLOADS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CURRENT_DEFAULT: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CURRENT_DISASM: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CURRENT_REGISTERS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CURRENT_SOURCE_LINE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_CURRENT_SYMBOL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_BASE_TRANSLATION_VIRTUAL_OFFSET: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_BreakpointWithStatusAddr: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_CmNtCSDVersionAddr: u32 = 616u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_DumpAttributes: u32 = 100072u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_DumpFormatVersion: u32 = 100040u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_DumpMmStorage: u32 = 100064u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_DumpPowerState: u32 = 100056u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_DumpWriterStatus: u32 = 100032u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_DumpWriterVersion: u32 = 100048u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_EtwpDebuggerData: u32 = 816u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_ExpNumberOfPagedPoolsAddr: u32 = 112u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_ExpPagedPoolDescriptorAddr: u32 = 104u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_ExpSystemResourcesListAddr: u32 = 96u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_IopErrorLogListHeadAddr: u32 = 144u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KPCR_OFFSET: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KPRCB_OFFSET: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KTHREAD_OFFSET: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KdPrintBufferSizeAddr: u32 = 720u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KdPrintCircularBufferAddr: u32 = 480u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KdPrintCircularBufferEndAddr: u32 = 488u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KdPrintCircularBufferPtrAddr: u32 = 712u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KdPrintRolloverCountAddr: u32 = 504u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KdPrintWritePointerAddr: u32 = 496u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KeBugCheckCallbackListHeadAddr: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KeTimeIncrementAddr: u32 = 120u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KeUserCallbackDispatcherAddr: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KernBase: u32 = 24u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KernelVerifierAddr: u32 = 576u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KiBugcheckDataAddr: u32 = 136u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KiCallUserModeAddr: u32 = 56u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KiNormalSystemCall: u32 = 528u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_KiProcessorBlockAddr: u32 = 536u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmAllocatedNonPagedPoolAddr: u32 = 592u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmAvailablePagesAddr: u32 = 424u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmBadPagesDetected: u32 = 800u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmDriverCommitAddr: u32 = 352u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmExtendedCommitAddr: u32 = 376u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmFreePageListHeadAddr: u32 = 392u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmHighestPhysicalPageAddr: u32 = 240u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmHighestUserAddressAddr: u32 = 456u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmLastUnloadedDriverAddr: u32 = 552u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmLoadedUserImageListAddr: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmLowestPhysicalPageAddr: u32 = 232u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmMaximumNonPagedPoolInBytesAddr: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmModifiedNoWritePageListHeadAddr: u32 = 416u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmModifiedPageListHeadAddr: u32 = 408u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmNonPagedPoolEndAddr: u32 = 280u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmNonPagedPoolStartAddr: u32 = 272u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmNonPagedSystemStartAddr: u32 = 264u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmNumberOfPagingFilesAddr: u32 = 224u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmNumberOfPhysicalPagesAddr: u32 = 248u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmPageSize: u32 = 312u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmPagedPoolCommitAddr: u32 = 368u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmPagedPoolEndAddr: u32 = 296u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmPagedPoolInformationAddr: u32 = 304u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmPagedPoolStartAddr: u32 = 288u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmPeakCommitmentAddr: u32 = 600u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmPfnDatabaseAddr: u32 = 192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmPhysicalMemoryBlockAddr: u32 = 624u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmProcessCommitAddr: u32 = 360u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmResidentAvailablePagesAddr: u32 = 432u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmSessionBase: u32 = 632u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmSessionSize: u32 = 640u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmSharedCommitAddr: u32 = 344u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmSizeOfPagedPoolInBytesAddr: u32 = 320u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmSpecialPoolTagAddr: u32 = 568u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmStandbyPageListHeadAddr: u32 = 400u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmSubsectionBaseAddr: u32 = 216u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmSystemCacheEndAddr: u32 = 176u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmSystemCacheStartAddr: u32 = 168u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmSystemCacheWsAddr: u32 = 184u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmSystemParentTablePage: u32 = 648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmSystemPtesEndAddr: u32 = 208u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmSystemPtesStartAddr: u32 = 200u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmSystemRangeStartAddr: u32 = 464u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmTotalCommitLimitAddr: u32 = 328u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmTotalCommitLimitMaximumAddr: u32 = 608u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmTotalCommittedPagesAddr: u32 = 336u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmTriageActionTakenAddr: u32 = 560u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmUnloadedDriversAddr: u32 = 544u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmUserProbeAddressAddr: u32 = 472u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmVerifierDataAddr: u32 = 584u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmVirtualTranslationBase: u32 = 656u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_MmZeroedPageListHeadAddr: u32 = 384u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_NonPagedPoolDescriptorAddr: u32 = 448u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_NtBuildLabAddr: u32 = 520u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_ObpRootDirectoryObjectAddr: u32 = 152u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_ObpTypeObjectTypeAddr: u32 = 160u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetEprocessDirectoryTableBase: u32 = 686u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetEprocessParentCID: u32 = 684u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetEprocessPeb: u32 = 682u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetKThreadApcProcess: u32 = 672u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetKThreadBStore: u32 = 676u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetKThreadBStoreLimit: u32 = 678u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetKThreadInitialStack: u32 = 670u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetKThreadKernelStack: u32 = 668u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetKThreadNextProcessor: u32 = 664u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetKThreadState: u32 = 674u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetKThreadTeb: u32 = 666u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetPrcbCpuType: u32 = 696u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetPrcbCurrentThread: u32 = 692u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetPrcbDpcRoutine: u32 = 690u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetPrcbMhz: u32 = 694u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetPrcbNumber: u32 = 702u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetPrcbProcessorState: u32 = 700u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_OffsetPrcbVendorString: u32 = 698u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_PROCESSOR_IDENTIFICATION: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_PROCESSOR_SPEED: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_PaeEnabled: u32 = 100000u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_PoolTrackTableAddr: u32 = 440u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_ProductType: u32 = 100016u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_PsActiveProcessHeadAddr: u32 = 80u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_PsLoadedModuleListAddr: u32 = 72u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_PspCidTableAddr: u32 = 88u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_PteBase: u32 = 864u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_SPACE_BUS_DATA: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_SPACE_CONTROL: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_SPACE_COUNT: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_SPACE_DEBUGGER_DATA: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_SPACE_IO: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_SPACE_MSR: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_SPACE_PHYSICAL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_SPACE_VIRTUAL: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_SavedContextAddr: u32 = 40u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_SharedUserData: u32 = 100008u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_SizeEProcess: u32 = 680u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_SizeEThread: u32 = 704u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_SizePrcb: u32 = 688u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DATA_SuiteMask: u32 = 100024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DISASM_EFFECTIVE_ADDRESS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DISASM_MATCHING_SYMBOLS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DISASM_SOURCE_FILE_NAME: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DISASM_SOURCE_LINE_NUMBER: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DUMP_ACTIVE: u32 = 1030u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DUMP_DEFAULT: u32 = 1025u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DUMP_FILE_BASE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DUMP_FILE_LOAD_FAILED_INDEX: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DUMP_FILE_ORIGINAL_CAB_INDEX: u32 = 4294967294u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DUMP_FILE_PAGE_FILE_DUMP: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DUMP_FULL: u32 = 1026u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DUMP_IMAGE_FILE: u32 = 1027u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DUMP_SMALL: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DUMP_TRACE_LOG: u32 = 1028u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_DUMP_WINDOWS_CE: u32 = 1029u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ECREATE_PROCESS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ECREATE_PROCESS_INHERIT_HANDLES: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ECREATE_PROCESS_USE_IMPLICIT_COMMAND_LINE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ECREATE_PROCESS_USE_VERIFIER_FLAGS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EINDEX_FROM_CURRENT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EINDEX_FROM_END: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EINDEX_FROM_START: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EINDEX_NAME: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_END_ACTIVE_DETACH: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_END_ACTIVE_TERMINATE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_END_DISCONNECT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_END_PASSIVE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_END_REENTRANT: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_ALL: u32 = 15728639u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_ALLOW_NETWORK_PATHS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_ALLOW_READ_ONLY_BREAKPOINTS: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_DEBUGGING_SENSITIVE_DATA: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_DISABLESQM: u32 = 524288u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_DISABLE_EXECUTION_COMMANDS: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_DISABLE_MANAGED_SUPPORT: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_DISABLE_MODULE_SYMBOL_LOAD: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_DISABLE_STEPLINES_OPTIONS: u32 = 2097152u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_DISALLOW_IMAGE_FILE_MAPPING: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_DISALLOW_NETWORK_PATHS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_DISALLOW_SHELL_COMMANDS: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_FAIL_INCOMPLETE_INFORMATION: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_FINAL_BREAK: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_IGNORE_DBGHELP_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_IGNORE_EXTENSION_VERSIONS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_IGNORE_LOADER_EXCEPTIONS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_INITIAL_BREAK: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_INITIAL_MODULE_BREAK: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_KD_QUIET_MODE: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_NO_EXECUTE_REPEAT: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_PREFER_DML: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_PREFER_TRACE_FILES: u32 = 8388608u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_ENGOPT_SYNCHRONIZE_BREAKPOINTS: u32 = 2048u32;
pub struct DEBUG_EVENT(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EVENT_BREAKPOINT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EVENT_CHANGE_DEBUGGEE_STATE: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EVENT_CHANGE_ENGINE_STATE: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EVENT_CHANGE_SYMBOL_STATE: u32 = 4096u32;
pub struct DEBUG_EVENT_CODE(i32);
pub struct DEBUG_EVENT_CONTEXT(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EVENT_CREATE_PROCESS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EVENT_CREATE_THREAD: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EVENT_EXCEPTION: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EVENT_EXIT_PROCESS: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EVENT_EXIT_THREAD: u32 = 8u32;
pub struct DEBUG_EVENT_INFO_TYPE(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EVENT_LOAD_MODULE: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EVENT_SERVICE_EXCEPTION: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EVENT_SESSION_STATUS: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EVENT_SYSTEM_ERROR: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EVENT_UNLOAD_MODULE: u32 = 128u32;
pub struct DEBUG_EXCEPTION_FILTER_PARAMETERS(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXECUTE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXECUTE_ECHO: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXECUTE_EVENT: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXECUTE_EXTENSION: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXECUTE_HOTKEY: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXECUTE_INTERNAL: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXECUTE_MENU: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXECUTE_NOT_LOGGED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXECUTE_NO_REPEAT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXECUTE_SCRIPT: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXECUTE_TOOLBAR: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXECUTE_USER_CLICKED: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXECUTE_USER_TYPED: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXEC_FLAGS_NONBLOCK: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXPR_CPLUSPLUS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXPR_MASM: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXTENSION_AT_ENGINE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXTINIT_HAS_COMMAND_HELP: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXT_PVALUE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXT_PVTYPE_IS_POINTER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXT_PVTYPE_IS_VALUE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_EXT_QVALUE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_BREAK: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_CREATE_PROCESS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_CREATE_THREAD: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_DEBUGGEE_OUTPUT: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_EXIT_PROCESS: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_EXIT_THREAD: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_GO_HANDLED: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_GO_NOT_HANDLED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_IGNORE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_INITIAL_BREAKPOINT: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_INITIAL_MODULE_LOAD: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_LOAD_MODULE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_OUTPUT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_REMOVE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_SECOND_CHANCE_BREAK: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_SYSTEM_ERROR: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FILTER_UNLOAD_MODULE: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FIND_SOURCE_BEST_MATCH: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FIND_SOURCE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FIND_SOURCE_FULL_PATH: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FIND_SOURCE_NO_SRCSRV: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FIND_SOURCE_TOKEN_LOOKUP: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FIND_SOURCE_WITH_CHECKSUM: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FIND_SOURCE_WITH_CHECKSUM_STRICT: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_CAB_SECONDARY_ALL_IMAGES: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_CAB_SECONDARY_FILES: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_NO_OVERWRITE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_ADD_AVX_XSTATE_CONTEXT: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_CODE_SEGMENTS: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_DATA_SEGMENTS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_FILTER_MEMORY: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_FILTER_PATHS: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_FILTER_TRIAGE: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_FULL_AUXILIARY_STATE: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_FULL_MEMORY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_FULL_MEMORY_INFO: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_HANDLE_DATA: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_IGNORE_INACCESSIBLE_MEM: u32 = 134217728u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_INDIRECT_MEMORY: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_IPT_TRACE: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_MODULE_HEADERS: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_NO_AUXILIARY_STATE: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_NO_OPTIONAL_DATA: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_PRIVATE_READ_WRITE_MEMORY: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_PROCESS_THREAD_DATA: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_SCAN_PARTIAL_PAGES: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_THREAD_INFO: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_USER_SMALL_UNLOADED_MODULES: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FORMAT_WRITE_CAB: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FRAME_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_FRAME_IGNORE_INLINE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GETFNENT_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GETFNENT_RAW_ENTRY_ONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GETMOD_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GETMOD_NO_LOADED_MODULES: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GETMOD_NO_UNLOADED_MODULES: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GET_PROC_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GET_PROC_FULL_MATCH: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GET_PROC_ONLY_MATCH: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GET_PROC_SERVICE_NAME: u32 = 4u32;
pub struct DEBUG_GET_TEXT_COMPLETIONS_IN(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GET_TEXT_COMPLETIONS_IS_DOT_COMMAND: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GET_TEXT_COMPLETIONS_IS_EXTENSION_COMMAND: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GET_TEXT_COMPLETIONS_IS_SYMBOL: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GET_TEXT_COMPLETIONS_NO_DOT_COMMANDS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GET_TEXT_COMPLETIONS_NO_EXTENSION_COMMANDS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GET_TEXT_COMPLETIONS_NO_SYMBOLS: u32 = 4u32;
pub struct DEBUG_GET_TEXT_COMPLETIONS_OUT(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GSEL_ALLOW_HIGHER: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GSEL_ALLOW_LOWER: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GSEL_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GSEL_INLINE_CALLSITE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GSEL_NEAREST_ONLY: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_GSEL_NO_SYMBOL_LOADS: u32 = 1u32;
pub struct DEBUG_HANDLE_DATA_BASIC(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_HANDLE_DATA_TYPE_ALL_HANDLE_OPERATIONS: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_HANDLE_DATA_TYPE_BASIC: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_HANDLE_DATA_TYPE_HANDLE_COUNT: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_HANDLE_DATA_TYPE_MINI_EVENT_1: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_HANDLE_DATA_TYPE_MINI_MUTANT_1: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_HANDLE_DATA_TYPE_MINI_MUTANT_2: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_HANDLE_DATA_TYPE_MINI_PROCESS_1: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_HANDLE_DATA_TYPE_MINI_PROCESS_2: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_HANDLE_DATA_TYPE_MINI_SECTION_1: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_HANDLE_DATA_TYPE_MINI_SEMAPHORE_1: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_HANDLE_DATA_TYPE_MINI_THREAD_1: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_HANDLE_DATA_TYPE_OBJECT_NAME: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_HANDLE_DATA_TYPE_OBJECT_NAME_WIDE: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_HANDLE_DATA_TYPE_PER_HANDLE_OPERATIONS: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_HANDLE_DATA_TYPE_TYPE_NAME: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_HANDLE_DATA_TYPE_TYPE_NAME_WIDE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_INTERRUPT_ACTIVE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_INTERRUPT_EXIT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_INTERRUPT_PASSIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_IOUTPUT_ADDR_TRANSLATE: u32 = 134217728u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_IOUTPUT_BREAKPOINT: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_IOUTPUT_EVENT: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_IOUTPUT_KD_PROTOCOL: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_IOUTPUT_REMOTING: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_KERNEL_ACTIVE_DUMP: u32 = 1030u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_KERNEL_CONNECTION: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_KERNEL_DUMP: u32 = 1025u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_KERNEL_EXDI_DRIVER: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_KERNEL_FULL_DUMP: u32 = 1026u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_KERNEL_IDNA: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_KERNEL_INSTALL_DRIVER: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_KERNEL_LOCAL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_KERNEL_REPT: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_KERNEL_SMALL_DUMP: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_KERNEL_TRACE_LOG: u32 = 1028u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_KNOWN_STRUCT_GET_NAMES: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_KNOWN_STRUCT_GET_SINGLE_LINE_OUTPUT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_KNOWN_STRUCT_SUPPRESS_TYPE_NAME: u32 = 3u32;
pub struct DEBUG_LAST_EVENT_INFO_BREAKPOINT(i32);
pub struct DEBUG_LAST_EVENT_INFO_EXCEPTION(i32);
pub struct DEBUG_LAST_EVENT_INFO_EXIT_PROCESS(i32);
pub struct DEBUG_LAST_EVENT_INFO_EXIT_THREAD(i32);
pub struct DEBUG_LAST_EVENT_INFO_LOAD_MODULE(i32);
pub struct DEBUG_LAST_EVENT_INFO_SERVICE_EXCEPTION(i32);
pub struct DEBUG_LAST_EVENT_INFO_SYSTEM_ERROR(i32);
pub struct DEBUG_LAST_EVENT_INFO_UNLOAD_MODULE(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_LEVEL_ASSEMBLY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_LEVEL_SOURCE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_LIVE_USER_NON_INVASIVE: u32 = 33u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_LOG_APPEND: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_LOG_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_LOG_DML: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_LOG_UNICODE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MANAGED_ALLOWED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MANAGED_DISABLED: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MANAGED_DLL_LOADED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MANRESET_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MANRESET_LOAD_DLL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MANSTR_LOADED_SUPPORT_DLL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MANSTR_LOAD_STATUS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MANSTR_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MODNAME_IMAGE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MODNAME_LOADED_IMAGE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MODNAME_MAPPED_IMAGE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MODNAME_MODULE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MODNAME_SYMBOL_FILE: u32 = 3u32;
pub struct DEBUG_MODULE_AND_ID(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MODULE_EXE_MODULE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MODULE_EXPLICIT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MODULE_LOADED: u32 = 0u32;
pub struct DEBUG_MODULE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MODULE_SECONDARY: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MODULE_SYM_BAD_CHECKSUM: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MODULE_SYNTHETIC: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MODULE_UNLOADED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_MODULE_USER_MODE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_NOTIFY_SESSION_ACCESSIBLE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_NOTIFY_SESSION_ACTIVE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_NOTIFY_SESSION_INACCESSIBLE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_NOTIFY_SESSION_INACTIVE: u32 = 1u32;
pub struct DEBUG_OFFSET_REGION(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OFFSINFO_VIRTUAL_SOURCE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCBF_COMBINED_EXPLICIT_FLUSH: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCBF_DML_HAS_SPECIAL_CHARACTERS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCBF_DML_HAS_TAGS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCBI_ANY_FORMAT: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCBI_DML: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCBI_EXPLICIT_FLUSH: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCBI_TEXT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCB_DML: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCB_EXPLICIT_FLUSH: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCB_TEXT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCTL_ALL_CLIENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCTL_ALL_OTHER_CLIENTS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCTL_AMBIENT: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCTL_AMBIENT_DML: u32 = 4294967294u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCTL_AMBIENT_TEXT: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCTL_DML: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCTL_IGNORE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCTL_LOG_ONLY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCTL_NOT_LOGGED: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCTL_OVERRIDE_MASK: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCTL_SEND_MASK: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTCTL_THIS_CLIENT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_DEBUGGEE: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_DEBUGGEE_PROMPT: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_ERROR: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_EXTENSION_WARNING: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_IDENTITY_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_NORMAL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_PROMPT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_PROMPT_REGISTERS: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_STATUS: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_SYMBOLS: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_SYMBOLS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_SYMBOLS_NO_NAMES: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_SYMBOLS_NO_OFFSETS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_SYMBOLS_NO_TYPES: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_SYMBOLS_NO_VALUES: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_VERBOSE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_WARNING: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTPUT_XML: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTSYM_ALLOW_DISPLACEMENT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTSYM_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTSYM_FORCE_OFFSET: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTSYM_SOURCE_LINE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTTYPE_ADDRESS_AT_END: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTTYPE_ADDRESS_OF_FIELD: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTTYPE_BLOCK_RECURSE: u32 = 2097152u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTTYPE_COMPACT_OUTPUT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTTYPE_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTTYPE_NO_INDENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTTYPE_NO_OFFSET: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUTTYPE_VERBOSE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_OUT_TEXT_REPL_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_PHYSICAL_CACHED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_PHYSICAL_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_PHYSICAL_UNCACHED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_PHYSICAL_WRITE_COMBINED: u32 = 3u32;
pub struct DEBUG_PROCESSOR_IDENTIFICATION_ALL(i32);
pub struct DEBUG_PROCESSOR_IDENTIFICATION_ALPHA(i32);
pub struct DEBUG_PROCESSOR_IDENTIFICATION_AMD64(i32);
pub struct DEBUG_PROCESSOR_IDENTIFICATION_ARM(i32);
pub struct DEBUG_PROCESSOR_IDENTIFICATION_ARM64(i32);
pub struct DEBUG_PROCESSOR_IDENTIFICATION_IA64(i32);
pub struct DEBUG_PROCESSOR_IDENTIFICATION_X86(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_PROCESS_DETACH_ON_EXIT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_PROCESS_ONLY_THIS_PROCESS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_PROC_DESC_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_PROC_DESC_NO_COMMAND_LINE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_PROC_DESC_NO_MTS_PACKAGES: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_PROC_DESC_NO_PATHS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_PROC_DESC_NO_SERVICES: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_PROC_DESC_NO_SESSION_ID: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_PROC_DESC_NO_USER_NAME: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_PROC_DESC_WITH_PACKAGEFAMILY: u32 = 64u32;
pub struct DEBUG_READ_USER_MINIDUMP_STREAM(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REGISTERS_ALL: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REGISTERS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REGISTERS_FLOAT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REGISTERS_INT32: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REGISTERS_INT64: u32 = 2u32;
pub struct DEBUG_REGISTER_DESCRIPTION(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REGISTER_SUB_REGISTER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REGSRC_DEBUGGEE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REGSRC_EXPLICIT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REGSRC_FRAME: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_ADD_CACHED_SYMBOL_INFO: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_CLOSE_TOKEN: u32 = 30u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_CURRENT_OUTPUT_CALLBACKS_ARE_DML_AWARE: u32 = 19u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_DUPLICATE_TOKEN: u32 = 28u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_EXT_TYPED_DATA_ANSI: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_GET_ADDITIONAL_CREATE_OPTIONS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_GET_CACHED_SYMBOL_INFO: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_GET_CAPTURED_EVENT_CODE_OFFSET: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_GET_DUMP_HEADER: u32 = 21u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_GET_EXTENSION_SEARCH_PATH_WIDE: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_GET_INSTRUMENTATION_VERSION: u32 = 37u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_GET_MODULE_ARCHITECTURE: u32 = 38u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_GET_OFFSET_UNWIND_INFORMATION: u32 = 20u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_GET_TEXT_COMPLETIONS_ANSI: u32 = 18u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_GET_TEXT_COMPLETIONS_WIDE: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_GET_WIN32_MAJOR_MINOR_VERSIONS: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_INLINE_QUERY: u32 = 35u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_MIDORI: u32 = 23u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_MISC_INFORMATION: u32 = 25u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_OPEN_PROCESS_TOKEN: u32 = 26u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_OPEN_THREAD_TOKEN: u32 = 27u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_PROCESS_DESCRIPTORS: u32 = 24u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_QUERY_INFO_TOKEN: u32 = 29u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_READ_CAPTURED_EVENT_CODE_STREAM: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_READ_USER_MINIDUMP_STREAM: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_REMOVE_CACHED_SYMBOL_INFO: u32 = 17u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_RESUME_THREAD: u32 = 34u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_SET_ADDITIONAL_CREATE_OPTIONS: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_SET_DUMP_HEADER: u32 = 22u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_SET_LOCAL_IMPLICIT_COMMAND_LINE: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_SOURCE_PATH_HAS_SOURCE_SERVER: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_TARGET_CAN_DETACH: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_TARGET_EXCEPTION_CONTEXT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_TARGET_EXCEPTION_RECORD: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_TARGET_EXCEPTION_THREAD: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_TL_INSTRUMENTATION_AWARE: u32 = 36u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_WOW_MODULE: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_REQUEST_WOW_PROCESS: u32 = 31u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SCOPE_GROUP_ALL: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SCOPE_GROUP_ARGUMENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SCOPE_GROUP_BY_DATAMODEL: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SCOPE_GROUP_LOCALS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SERVERS_ALL: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SERVERS_DEBUGGER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SERVERS_PROCESS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SESSION_ACTIVE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SESSION_END: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SESSION_END_SESSION_ACTIVE_DETACH: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SESSION_END_SESSION_ACTIVE_TERMINATE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SESSION_END_SESSION_PASSIVE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SESSION_FAILURE: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SESSION_HIBERNATE: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SESSION_REBOOT: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SOURCE_IS_STATEMENT: u32 = 1u32;
pub struct DEBUG_SPECIFIC_FILTER_PARAMETERS(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SRCFILE_SYMBOL_CHECKSUMINFO: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SRCFILE_SYMBOL_TOKEN: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SRCFILE_SYMBOL_TOKEN_SOURCE_COMMAND_WIDE: u32 = 1u32;
pub struct DEBUG_STACKFRAME_TYPE(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STACK_ARGUMENTS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STACK_COLUMN_NAMES: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STACK_DML: u32 = 2048u32;
pub struct DEBUG_STACK_FRAME(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STACK_FRAME_ADDRESSES: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STACK_FRAME_ADDRESSES_RA_ONLY: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STACK_FRAME_ARCH: u32 = 16384u32;
pub struct DEBUG_STACK_FRAME_EX(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STACK_FRAME_MEMORY_USAGE: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STACK_FRAME_NUMBERS: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STACK_FRAME_OFFSETS: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STACK_FUNCTION_INFO: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STACK_NONVOLATILE_REGISTERS: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STACK_PARAMETERS: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STACK_PARAMETERS_NEWLINE: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STACK_PROVIDER: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STACK_SOURCE_LINE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_BREAK: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_GO: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_GO_HANDLED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_GO_NOT_HANDLED: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_IGNORE_EVENT: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_INSIDE_WAIT: u64 = 4294967296u64;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_MASK: u32 = 31u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_NO_CHANGE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_NO_DEBUGGEE: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_OUT_OF_SYNC: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_RESTART_REQUESTED: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_REVERSE_GO: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_REVERSE_STEP_BRANCH: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_REVERSE_STEP_INTO: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_REVERSE_STEP_OVER: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_STEP_BRANCH: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_STEP_INTO: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_STEP_OVER: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_TIMEOUT: u32 = 17u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_WAIT_INPUT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_STATUS_WAIT_TIMEOUT: u64 = 8589934592u64;
pub struct DEBUG_SYMBOL_ENTRY(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMBOL_EXPANDED: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMBOL_EXPANSION_LEVEL_MASK: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMBOL_IS_ARGUMENT: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMBOL_IS_ARRAY: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMBOL_IS_FLOAT: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMBOL_IS_LOCAL: u32 = 512u32;
pub struct DEBUG_SYMBOL_PARAMETERS(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMBOL_READ_ONLY: u32 = 32u32;
pub struct DEBUG_SYMBOL_SOURCE_ENTRY(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMENT_IS_CODE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMENT_IS_DATA: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMENT_IS_LOCAL: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMENT_IS_MANAGED: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMENT_IS_PARAMETER: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMENT_IS_SYNTHETIC: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMINFO_BREAKPOINT_SOURCE_LINE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMINFO_GET_MODULE_SYMBOL_NAMES_AND_OFFSETS: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMINFO_GET_SYMBOL_NAME_BY_OFFSET_AND_TAG_WIDE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMINFO_IMAGEHLP_MODULEW64: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMTYPE_CODEVIEW: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMTYPE_COFF: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMTYPE_DEFERRED: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMTYPE_DIA: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMTYPE_EXPORT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMTYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMTYPE_PDB: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYMTYPE_SYM: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYSOBJINFO_CURRENT_PROCESS_COOKIE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYSOBJINFO_THREAD_BASIC_INFORMATION: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYSOBJINFO_THREAD_NAME_WIDE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYSVERSTR_BUILD: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_SYSVERSTR_SERVICE_PACK: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TBINFO_AFFINITY: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TBINFO_ALL: u32 = 63u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TBINFO_EXIT_STATUS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TBINFO_PRIORITY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TBINFO_PRIORITY_CLASS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TBINFO_START_OFFSET: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TBINFO_TIMES: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TEXT_ALLOWBREAKPOINTS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TEXT_ALLOWERRORREPORT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TEXT_EVALUATETOCODECONTEXT: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TEXT_ISEXPRESSION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TEXT_ISNONUSERCODE: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TEXT_NOSIDEEFFECTS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TEXT_RETURNVALUE: u32 = 2u32;
pub struct DEBUG_THREAD_BASIC_INFORMATION(i32);
pub struct DEBUG_TYPED_DATA(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TYPED_DATA_IS_IN_MEMORY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TYPED_DATA_PHYSICAL_CACHED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TYPED_DATA_PHYSICAL_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TYPED_DATA_PHYSICAL_MEMORY: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TYPED_DATA_PHYSICAL_UNCACHED: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TYPED_DATA_PHYSICAL_WRITE_COMBINED: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TYPEOPTS_FORCERADIX_OUTPUT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TYPEOPTS_LONGSTATUS_DISPLAY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TYPEOPTS_MATCH_MAXSIZE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_TYPEOPTS_UNICODE_DISPLAY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_USER_WINDOWS_DUMP: u32 = 1025u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_USER_WINDOWS_DUMP_WINDOWS_CE: u32 = 1029u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_USER_WINDOWS_IDNA: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_USER_WINDOWS_PROCESS: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_USER_WINDOWS_PROCESS_SERVER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_USER_WINDOWS_REPT: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_USER_WINDOWS_SMALL_DUMP: u32 = 1024u32;
pub struct DEBUG_VALUE(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VALUE_FLOAT128: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VALUE_FLOAT32: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VALUE_FLOAT64: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VALUE_FLOAT80: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VALUE_FLOAT82: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VALUE_INT16: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VALUE_INT32: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VALUE_INT64: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VALUE_INT8: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VALUE_INVALID: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VALUE_TYPES: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VALUE_VECTOR128: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VALUE_VECTOR64: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VSEARCH_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VSEARCH_WRITABLE_ONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VSOURCE_DEBUGGEE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VSOURCE_DUMP_WITHOUT_MEMINFO: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VSOURCE_INVALID: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_VSOURCE_MAPPED_IMAGE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DEBUG_WAIT_DEFAULT: u32 = 0u32;
pub struct DIGEST_FUNCTION(i32);
pub struct DISPATCHER_CONTEXT(i32);
pub struct DISPATCHER_CONTEXT(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DMP_CONTEXT_RECORD_SIZE_32: u32 = 1200u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DMP_CONTEXT_RECORD_SIZE_64: u32 = 3000u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DMP_HEADER_COMMENT_SIZE: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DMP_PHYSICAL_MEMORY_BLOCK_SIZE_32: u32 = 700u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DMP_PHYSICAL_MEMORY_BLOCK_SIZE_64: u32 = 700u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DMP_RESERVED_0_SIZE_32: u32 = 1760u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DMP_RESERVED_0_SIZE_64: u32 = 4008u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DMP_RESERVED_2_SIZE_32: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DMP_RESERVED_3_SIZE_32: u32 = 56u32;
pub struct DOCUMENTNAMETYPE(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DSLFLAG_MISMATCHED_DBG: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DSLFLAG_MISMATCHED_PDB: u32 = 1u32;
pub struct DUMP_FILE_ATTRIBUTES(i32);
pub struct DUMP_HEADER32(i32);
pub struct DUMP_HEADER64(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DUMP_SUMMARY_VALID_CURRENT_USER_VA: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const DUMP_SUMMARY_VALID_KERNEL_VA: u32 = 1u32;
pub struct DebugBaseEventCallbacks(i32);
pub struct DebugBaseEventCallbacksWide(i32);
pub struct DebugHelper(i32);
pub struct DebugPropertyInfo(i32);
pub struct DebugStackFrameDescriptor(i32);
pub struct DebugStackFrameDescriptor64(i32);
pub struct DefaultDebugSessionProvider(i32);
pub struct ERRORRESUMEACTION(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const ERROR_DBG_CANCELLED: u32 = 3221226695u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const ERROR_DBG_TIMEOUT: u32 = 3221226932u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const ERROR_IMAGE_NOT_STRIPPED: u32 = 34816u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const ERROR_NO_DBG_POINTER: u32 = 34817u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const ERROR_NO_PDB_POINTER: u32 = 34818u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const ESLFLAG_FULLPATH: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const ESLFLAG_INLINE_SITE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const ESLFLAG_NEAREST: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const ESLFLAG_NEXT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const ESLFLAG_PREV: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const EVENT_SRCSPEW: u32 = 100u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const EVENT_SRCSPEW_END: u32 = 199u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const EVENT_SRCSPEW_START: u32 = 100u32;
pub struct EXCEPTION_DEBUG_INFO(i32);
pub struct EXCEPTION_POINTERS(i32);
pub struct EXCEPTION_RECORD(i32);
pub struct EXCEPTION_RECORD32(i32);
pub struct EXCEPTION_RECORD64(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const EXIT_ON_CONTROLC: u32 = 8u32;
pub struct EXIT_PROCESS_DEBUG_INFO(i32);
pub struct EXIT_THREAD_DEBUG_INFO(i32);
pub struct EXTSTACKTRACE(i32);
pub struct EXTSTACKTRACE32(i32);
pub struct EXTSTACKTRACE64(i32);
pub struct EXT_API_VERSION(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const EXT_API_VERSION_NUMBER: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const EXT_API_VERSION_NUMBER32: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const EXT_API_VERSION_NUMBER64: u32 = 6u32;
pub struct EXT_FIND_FILE(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const EXT_FIND_FILE_ALLOW_GIVEN_PATH: u32 = 1u32;
pub struct EXT_MATCH_PATTERN_A(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const EXT_OUTPUT_VER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const EXT_TDF_PHYSICAL_CACHED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const EXT_TDF_PHYSICAL_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const EXT_TDF_PHYSICAL_MEMORY: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const EXT_TDF_PHYSICAL_UNCACHED: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const EXT_TDF_PHYSICAL_WRITE_COMBINED: u32 = 8u32;
pub struct EXT_TDOP(i32);
pub struct EXT_TYPED_DATA(i32);
pub struct EX_PROP_INFO_FLAGS(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const E_JsDEBUG_INVALID_MEMORY_ADDRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1916338171i32 as _);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const E_JsDEBUG_MISMATCHED_RUNTIME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1916338175i32 as _);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const E_JsDEBUG_OUTSIDE_OF_VM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1916338172i32 as _);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const E_JsDEBUG_RUNTIME_NOT_IN_DEBUG_MODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1916338169i32 as _);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const E_JsDEBUG_SOURCE_LOCATION_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1916338170i32 as _);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const E_JsDEBUG_UNKNOWN_THREAD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1916338174i32 as _);
pub struct ErrorClass(i32);
pub struct ExtendedDebugPropertyInfo(i32);
pub struct FACILITY_CODE(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const FACILITY_JsDEBUG: u32 = 3527u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const FIELDS_DID_NOT_MATCH: u32 = 4u32;
pub struct FIELD_INFO(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const FLAG_ENGINE_PRESENT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const FLAG_ENGOPT_DISALLOW_NETWORK_PATHS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const FLAG_OVERRIDE_ARM_MACHINE_TYPE: u32 = 16u32;
pub struct FORMAT_MESSAGE_OPTIONS(i32);
pub struct FPO_DATA(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const GETATTRFLAG_HUMANTEXT: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const GETATTRFLAG_THIS: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const GETATTRTYPE_DEPSCAN: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const GETATTRTYPE_NORMAL: u32 = 0u32;
pub struct GET_CONTEXT_EX(i32);
pub struct GET_CURRENT_PROCESS_ADDRESS(i32);
pub struct GET_CURRENT_THREAD_ADDRESS(i32);
pub struct GET_EXPRESSION_EX(i32);
pub struct GET_INPUT_LINE(i32);
pub struct GET_PEB_ADDRESS(i32);
pub struct GET_SET_SYMPATH(i32);
pub struct GET_TEB_ADDRESS(i32);
pub struct IActiveScript(i32);
pub struct IActiveScriptAuthor(i32);
pub struct IActiveScriptAuthorProcedure(i32);
pub struct IActiveScriptDebug32(i32);
pub struct IActiveScriptDebug64(i32);
pub struct IActiveScriptEncode(i32);
pub struct IActiveScriptError(i32);
pub struct IActiveScriptError64(i32);
pub struct IActiveScriptErrorDebug(i32);
pub struct IActiveScriptErrorDebug110(i32);
pub struct IActiveScriptGarbageCollector(i32);
pub struct IActiveScriptHostEncode(i32);
pub struct IActiveScriptParse32(i32);
pub struct IActiveScriptParse64(i32);
pub struct IActiveScriptParseProcedure2_32(i32);
pub struct IActiveScriptParseProcedure2_64(i32);
pub struct IActiveScriptParseProcedure32(i32);
pub struct IActiveScriptParseProcedure64(i32);
pub struct IActiveScriptParseProcedureOld32(i32);
pub struct IActiveScriptParseProcedureOld64(i32);
pub struct IActiveScriptProfilerCallback(i32);
pub struct IActiveScriptProfilerCallback2(i32);
pub struct IActiveScriptProfilerCallback3(i32);
pub struct IActiveScriptProfilerControl(i32);
pub struct IActiveScriptProfilerControl2(i32);
pub struct IActiveScriptProfilerControl3(i32);
pub struct IActiveScriptProfilerControl4(i32);
pub struct IActiveScriptProfilerControl5(i32);
pub struct IActiveScriptProfilerHeapEnum(i32);
pub struct IActiveScriptProperty(i32);
pub struct IActiveScriptSIPInfo(i32);
pub struct IActiveScriptSite(i32);
pub struct IActiveScriptSiteDebug32(i32);
pub struct IActiveScriptSiteDebug64(i32);
pub struct IActiveScriptSiteDebugEx(i32);
pub struct IActiveScriptSiteInterruptPoll(i32);
pub struct IActiveScriptSiteTraceInfo(i32);
pub struct IActiveScriptSiteUIControl(i32);
pub struct IActiveScriptSiteWindow(i32);
pub struct IActiveScriptStats(i32);
pub struct IActiveScriptStringCompare(i32);
pub struct IActiveScriptTraceInfo(i32);
pub struct IActiveScriptWinRTErrorDebug(i32);
pub struct IApplicationDebugger(i32);
pub struct IApplicationDebuggerUI(i32);
pub struct IBindEventHandler(i32);
pub struct ICodeAddressConcept(i32);
pub struct IComparableConcept(i32);
pub struct IDataModelConcept(i32);
pub struct IDataModelManager(i32);
pub struct IDataModelManager2(i32);
pub struct IDataModelNameBinder(i32);
pub struct IDataModelScript(i32);
pub struct IDataModelScriptClient(i32);
pub struct IDataModelScriptDebug(i32);
pub struct IDataModelScriptDebug2(i32);
pub struct IDataModelScriptDebugBreakpoint(i32);
pub struct IDataModelScriptDebugBreakpointEnumerator(i32);
pub struct IDataModelScriptDebugClient(i32);
pub struct IDataModelScriptDebugStack(i32);
pub struct IDataModelScriptDebugStackFrame(i32);
pub struct IDataModelScriptDebugVariableSetEnumerator(i32);
pub struct IDataModelScriptHostContext(i32);
pub struct IDataModelScriptManager(i32);
pub struct IDataModelScriptProvider(i32);
pub struct IDataModelScriptProviderEnumerator(i32);
pub struct IDataModelScriptTemplate(i32);
pub struct IDataModelScriptTemplateEnumerator(i32);
pub struct IDebugAdvanced(i32);
pub struct IDebugAdvanced2(i32);
pub struct IDebugAdvanced3(i32);
pub struct IDebugAdvanced4(i32);
pub struct IDebugApplication11032(i32);
pub struct IDebugApplication11064(i32);
pub struct IDebugApplication32(i32);
pub struct IDebugApplication64(i32);
pub struct IDebugApplicationNode(i32);
pub struct IDebugApplicationNode100(i32);
pub struct IDebugApplicationNodeEvents(i32);
pub struct IDebugApplicationThread(i32);
pub struct IDebugApplicationThread11032(i32);
pub struct IDebugApplicationThread11064(i32);
pub struct IDebugApplicationThread64(i32);
pub struct IDebugApplicationThreadEvents110(i32);
pub struct IDebugAsyncOperation(i32);
pub struct IDebugAsyncOperationCallBack(i32);
pub struct IDebugBreakpoint(i32);
pub struct IDebugBreakpoint2(i32);
pub struct IDebugBreakpoint3(i32);
pub struct IDebugClient(i32);
pub struct IDebugClient2(i32);
pub struct IDebugClient3(i32);
pub struct IDebugClient4(i32);
pub struct IDebugClient5(i32);
pub struct IDebugClient6(i32);
pub struct IDebugClient7(i32);
pub struct IDebugClient8(i32);
pub struct IDebugCodeContext(i32);
pub struct IDebugControl(i32);
pub struct IDebugControl2(i32);
pub struct IDebugControl3(i32);
pub struct IDebugControl4(i32);
pub struct IDebugControl5(i32);
pub struct IDebugControl6(i32);
pub struct IDebugControl7(i32);
pub struct IDebugCookie(i32);
pub struct IDebugDataSpaces(i32);
pub struct IDebugDataSpaces2(i32);
pub struct IDebugDataSpaces3(i32);
pub struct IDebugDataSpaces4(i32);
pub struct IDebugDocument(i32);
pub struct IDebugDocumentContext(i32);
pub struct IDebugDocumentHelper32(i32);
pub struct IDebugDocumentHelper64(i32);
pub struct IDebugDocumentHost(i32);
pub struct IDebugDocumentInfo(i32);
pub struct IDebugDocumentProvider(i32);
pub struct IDebugDocumentText(i32);
pub struct IDebugDocumentTextAuthor(i32);
pub struct IDebugDocumentTextEvents(i32);
pub struct IDebugDocumentTextExternalAuthor(i32);
pub struct IDebugEventCallbacks(i32);
pub struct IDebugEventCallbacksWide(i32);
pub struct IDebugEventContextCallbacks(i32);
pub struct IDebugExpression(i32);
pub struct IDebugExpressionCallBack(i32);
pub struct IDebugExpressionContext(i32);
pub struct IDebugExtendedProperty(i32);
pub struct IDebugFormatter(i32);
pub struct IDebugHelper(i32);
pub struct IDebugHost(i32);
pub struct IDebugHostBaseClass(i32);
pub struct IDebugHostConstant(i32);
pub struct IDebugHostContext(i32);
pub struct IDebugHostData(i32);
pub struct IDebugHostErrorSink(i32);
pub struct IDebugHostEvaluator(i32);
pub struct IDebugHostEvaluator2(i32);
pub struct IDebugHostExtensibility(i32);
pub struct IDebugHostField(i32);
pub struct IDebugHostMemory(i32);
pub struct IDebugHostMemory2(i32);
pub struct IDebugHostModule(i32);
pub struct IDebugHostModule2(i32);
pub struct IDebugHostModuleSignature(i32);
pub struct IDebugHostPublic(i32);
pub struct IDebugHostScriptHost(i32);
pub struct IDebugHostStatus(i32);
pub struct IDebugHostSymbol(i32);
pub struct IDebugHostSymbol2(i32);
pub struct IDebugHostSymbolEnumerator(i32);
pub struct IDebugHostSymbols(i32);
pub struct IDebugHostType(i32);
pub struct IDebugHostType2(i32);
pub struct IDebugHostTypeSignature(i32);
pub struct IDebugInputCallbacks(i32);
pub struct IDebugOutputCallbacks(i32);
pub struct IDebugOutputCallbacks2(i32);
pub struct IDebugOutputCallbacksWide(i32);
pub struct IDebugOutputStream(i32);
pub struct IDebugPlmClient(i32);
pub struct IDebugPlmClient2(i32);
pub struct IDebugPlmClient3(i32);
pub struct IDebugProperty(i32);
pub struct IDebugPropertyEnumType_All(i32);
pub struct IDebugPropertyEnumType_Arguments(i32);
pub struct IDebugPropertyEnumType_Locals(i32);
pub struct IDebugPropertyEnumType_LocalsPlusArgs(i32);
pub struct IDebugPropertyEnumType_Registers(i32);
pub struct IDebugRegisters(i32);
pub struct IDebugRegisters2(i32);
pub struct IDebugSessionProvider(i32);
pub struct IDebugStackFrame(i32);
pub struct IDebugStackFrame110(i32);
pub struct IDebugStackFrameSniffer(i32);
pub struct IDebugStackFrameSnifferEx32(i32);
pub struct IDebugStackFrameSnifferEx64(i32);
pub struct IDebugSymbolGroup(i32);
pub struct IDebugSymbolGroup2(i32);
pub struct IDebugSymbols(i32);
pub struct IDebugSymbols2(i32);
pub struct IDebugSymbols3(i32);
pub struct IDebugSymbols4(i32);
pub struct IDebugSymbols5(i32);
pub struct IDebugSyncOperation(i32);
pub struct IDebugSystemObjects(i32);
pub struct IDebugSystemObjects2(i32);
pub struct IDebugSystemObjects3(i32);
pub struct IDebugSystemObjects4(i32);
pub struct IDebugThreadCall32(i32);
pub struct IDebugThreadCall64(i32);
pub struct IDynamicConceptProviderConcept(i32);
pub struct IDynamicKeyProviderConcept(i32);
pub struct IEnumDebugApplicationNodes(i32);
pub struct IEnumDebugCodeContexts(i32);
pub struct IEnumDebugExpressionContexts(i32);
pub struct IEnumDebugExtendedPropertyInfo(i32);
pub struct IEnumDebugPropertyInfo(i32);
pub struct IEnumDebugStackFrames(i32);
pub struct IEnumDebugStackFrames64(i32);
pub struct IEnumJsStackFrames(i32);
pub struct IEnumRemoteDebugApplicationThreads(i32);
pub struct IEnumRemoteDebugApplications(i32);
pub struct IEquatableConcept(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_DISASSEMBLE_BUFFER: u32 = 44u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_DUMP_SYMBOL_INFO: u32 = 22u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_FIND_FILE: u32 = 40u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_ANY_MODULE_IN_RANGE: u32 = 45u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_BUS_DATA: u32 = 20u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_CACHE_SIZE: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_CLR_DATA_INTERFACE: u32 = 38u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_CONTEXT_EX: u32 = 48u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_CURRENT_PROCESS: u32 = 26u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_CURRENT_PROCESS_HANDLE: u32 = 28u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_CURRENT_THREAD: u32 = 25u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_DEBUGGER_DATA: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_EXCEPTION_RECORD: u32 = 18u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_EXPRESSION_EX: u32 = 30u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_INPUT_LINE: u32 = 29u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_KERNEL_VERSION: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_PEB_ADDRESS: u32 = 129u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_SET_SYMPATH: u32 = 17u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_TEB_ADDRESS: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_THREAD_OS_INFO: u32 = 37u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_GET_TYPE_SIZE: u32 = 27u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_IS_PTR64: u32 = 19u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_KD_CONTEXT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_KSTACK_HELP: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_LOWMEM_CHECK: u32 = 23u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_MATCH_PATTERN_A: u32 = 39u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_OBSOLETE_PLACEHOLDER_36: u32 = 36u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_PHYSICAL_TO_VIRTUAL: u32 = 47u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_POINTER_SEARCH_PHYSICAL: u32 = 35u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_QUERY_TARGET_INTERFACE: u32 = 42u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_READ_CONTROL_SPACE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_READ_IO_SPACE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_READ_IO_SPACE_EX: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_READ_MSR: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_READ_PHYSICAL: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_READ_PHYSICAL_WITH_FLAGS: u32 = 33u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_RELOAD_SYMBOLS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_SEARCH_MEMORY: u32 = 24u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_SET_BUS_DATA: u32 = 21u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_SET_THREAD: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_TRANSLATE_VIRTUAL_TO_PHYSICAL: u32 = 31u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_TYPED_DATA: u32 = 43u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_TYPED_DATA_OBSOLETE: u32 = 41u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_VIRTUAL_TO_PHYSICAL: u32 = 46u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_WRITE_CONTROL_SPACE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_WRITE_IO_SPACE: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_WRITE_IO_SPACE_EX: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_WRITE_MSR: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_WRITE_PHYSICAL: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IG_WRITE_PHYSICAL_WITH_FLAGS: u32 = 34u32;
pub struct IHostDataModelAccess(i32);
pub struct IIndexableConcept(i32);
pub struct IIterableConcept(i32);
pub struct IJsDebug(i32);
pub struct IJsDebugBreakPoint(i32);
pub struct IJsDebugDataTarget(i32);
pub struct IJsDebugFrame(i32);
pub struct IJsDebugProcess(i32);
pub struct IJsDebugProperty(i32);
pub struct IJsDebugStackWalker(i32);
pub struct IJsEnumDebugProperty(i32);
pub struct IKeyEnumerator(i32);
pub struct IKeyStore(i32);
pub struct IMAGEHLP_CBA_EVENT(i32);
pub struct IMAGEHLP_CBA_EVENTW(i32);
pub struct IMAGEHLP_CBA_EVENT_SEVERITY(i32);
pub struct IMAGEHLP_CBA_READ_MEMORY(i32);
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOAD(i32);
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOAD64(i32);
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOADW64(i32);
pub struct IMAGEHLP_DUPLICATE_SYMBOL(i32);
pub struct IMAGEHLP_DUPLICATE_SYMBOL64(i32);
pub struct IMAGEHLP_EXTENDED_OPTIONS(i32);
pub struct IMAGEHLP_GET_TYPE_INFO_FLAGS(i32);
pub struct IMAGEHLP_GET_TYPE_INFO_PARAMS(i32);
pub struct IMAGEHLP_HD_TYPE(i32);
pub struct IMAGEHLP_LINE(i32);
pub struct IMAGEHLP_LINE64(i32);
pub struct IMAGEHLP_LINEW(i32);
pub struct IMAGEHLP_LINEW64(i32);
pub struct IMAGEHLP_MODULE(i32);
pub struct IMAGEHLP_MODULE64(i32);
pub struct IMAGEHLP_MODULE64_EX(i32);
pub struct IMAGEHLP_MODULEW(i32);
pub struct IMAGEHLP_MODULEW64(i32);
pub struct IMAGEHLP_MODULEW64_EX(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_MODULE_REGION_ADDITIONAL: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_MODULE_REGION_ALL: u32 = 255u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_MODULE_REGION_DLLBASE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_MODULE_REGION_DLLRANGE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_MODULE_REGION_JIT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_RMAP_BIG_ENDIAN: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_RMAP_FIXUP_ARM64X: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_RMAP_FIXUP_IMAGEBASE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_RMAP_IGNORE_MISCOMPARE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_RMAP_LOAD_RW_DATA_SECTIONS: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_RMAP_MAPPED_FLAT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_RMAP_OMIT_SHARED_RW_DATA_SECTIONS: u32 = 1073741824u32;
pub struct IMAGEHLP_SF_TYPE(i32);
pub struct IMAGEHLP_STACK_FRAME(i32);
pub struct IMAGEHLP_STATUS_REASON(i32);
pub struct IMAGEHLP_SYMBOL(i32);
pub struct IMAGEHLP_SYMBOL64(i32);
pub struct IMAGEHLP_SYMBOL64_PACKAGE(i32);
pub struct IMAGEHLP_SYMBOLW(i32);
pub struct IMAGEHLP_SYMBOLW64(i32);
pub struct IMAGEHLP_SYMBOLW64_PACKAGE(i32);
pub struct IMAGEHLP_SYMBOLW_PACKAGE(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_SYMBOL_FUNCTION: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_SYMBOL_INFO_CONSTANT: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_SYMBOL_INFO_FRAMERELATIVE: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_SYMBOL_INFO_LOCAL: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_SYMBOL_INFO_PARAMETER: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_SYMBOL_INFO_REGISTER: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_SYMBOL_INFO_REGRELATIVE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_SYMBOL_INFO_TLSRELATIVE: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_SYMBOL_INFO_VALUEPRESENT: u32 = 1u32;
pub struct IMAGEHLP_SYMBOL_PACKAGE(i32);
pub struct IMAGEHLP_SYMBOL_SRC(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_SYMBOL_THUNK: u32 = 8192u32;
pub struct IMAGEHLP_SYMBOL_TYPE_INFO(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IMAGEHLP_SYMBOL_VIRTUAL: u32 = 4096u32;
pub struct IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY(i32);
pub struct IMAGE_COFF_SYMBOLS_HEADER(i32);
pub struct IMAGE_COR20_HEADER(i32);
pub struct IMAGE_DATA_DIRECTORY(i32);
pub struct IMAGE_DEBUG_DIRECTORY(i32);
pub struct IMAGE_DEBUG_INFORMATION(i32);
pub struct IMAGE_DEBUG_TYPE(i32);
pub struct IMAGE_DIRECTORY_ENTRY(i32);
pub struct IMAGE_DLL_CHARACTERISTICS(i32);
pub struct IMAGE_FILE_CHARACTERISTICS(i32);
pub struct IMAGE_FILE_CHARACTERISTICS2(i32);
pub struct IMAGE_FILE_HEADER(i32);
pub struct IMAGE_FILE_MACHINE(i32);
pub struct IMAGE_FUNCTION_ENTRY(i32);
pub struct IMAGE_FUNCTION_ENTRY64(i32);
pub struct IMAGE_LOAD_CONFIG_CODE_INTEGRITY(i32);
pub struct IMAGE_LOAD_CONFIG_DIRECTORY32(i32);
pub struct IMAGE_LOAD_CONFIG_DIRECTORY64(i32);
pub struct IMAGE_NT_HEADERS32(i32);
pub struct IMAGE_NT_HEADERS64(i32);
pub struct IMAGE_OPTIONAL_HEADER32(i32);
pub struct IMAGE_OPTIONAL_HEADER64(i32);
pub struct IMAGE_OPTIONAL_HEADER_MAGIC(i32);
pub struct IMAGE_ROM_HEADERS(i32);
pub struct IMAGE_ROM_OPTIONAL_HEADER(i32);
pub struct IMAGE_RUNTIME_FUNCTION_ENTRY(i32);
pub struct IMAGE_SECTION_CHARACTERISTICS(i32);
pub struct IMAGE_SECTION_HEADER(i32);
pub struct IMAGE_SUBSYSTEM(i32);
pub struct IMachineDebugManager(i32);
pub struct IMachineDebugManagerCookie(i32);
pub struct IMachineDebugManagerEvents(i32);
pub struct IModelIterator(i32);
pub struct IModelKeyReference(i32);
pub struct IModelKeyReference2(i32);
pub struct IModelMethod(i32);
pub struct IModelObject(i32);
pub struct IModelPropertyAccessor(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const INCORRECT_VERSION_INFO: u32 = 7u32;
pub struct INLINE_FRAME_CONTEXT(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const INLINE_FRAME_CONTEXT_IGNORE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const INLINE_FRAME_CONTEXT_INIT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const INSUFFICIENT_SPACE_TO_COPY: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const INTERFACESAFE_FOR_UNTRUSTED_CALLER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const INTERFACESAFE_FOR_UNTRUSTED_DATA: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const INTERFACE_USES_DISPEX: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const INTERFACE_USES_SECURITY_MANAGER: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IOCTL_IPMI_INTERNAL_RECORD_SEL_EVENT: u32 = 2232320u32;
pub struct IOSPACE(i32);
pub struct IOSPACE32(i32);
pub struct IOSPACE64(i32);
pub struct IOSPACE_EX(i32);
pub struct IOSPACE_EX32(i32);
pub struct IOSPACE_EX64(i32);
pub struct IObjectSafety(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IPMI_IOCTL_INDEX: u32 = 1024u32;
pub struct IPMI_OS_SEL_RECORD(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IPMI_OS_SEL_RECORD_MASK: u32 = 65535u32;
pub struct IPMI_OS_SEL_RECORD_TYPE(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IPMI_OS_SEL_RECORD_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const IPMI_OS_SEL_RECORD_VERSION_1: u32 = 1u32;
pub struct IPerPropertyBrowsing2(i32);
pub struct IPreferredRuntimeTypeConcept(i32);
pub struct IProcessDebugManager32(i32);
pub struct IProcessDebugManager64(i32);
pub struct IProvideExpressionContexts(i32);
pub struct IRawEnumerator(i32);
pub struct IRemoteDebugApplication(i32);
pub struct IRemoteDebugApplication110(i32);
pub struct IRemoteDebugApplicationEvents(i32);
pub struct IRemoteDebugApplicationThread(i32);
pub struct IRemoteDebugCriticalErrorEvent110(i32);
pub struct IRemoteDebugInfoEvent110(i32);
pub struct IScriptEntry(i32);
pub struct IScriptInvocationContext(i32);
pub struct IScriptNode(i32);
pub struct IScriptScriptlet(i32);
pub struct ISimpleConnectionPoint(i32);
pub struct IStringDisplayableConcept(i32);
pub struct ITridentEventSink(i32);
pub struct IWebAppDiagnosticsObjectInitialization(i32);
pub struct IWebAppDiagnosticsSetup(i32);
pub struct IntrinsicKind(i32);
pub struct JS_PROPERTY_ATTRIBUTES(i32);
pub struct JS_PROPERTY_MEMBERS(i32);
pub struct JsDebugPropertyInfo(i32);
pub struct JsDebugReadMemoryFlags(i32);
pub struct KDDEBUGGER_DATA32(i32);
pub struct KDDEBUGGER_DATA64(i32);
pub struct KDHELP(i32);
pub struct KDHELP64(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const KD_SECONDARY_VERSION_AMD64_CONTEXT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const KD_SECONDARY_VERSION_AMD64_OBSOLETE_CONTEXT_1: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const KD_SECONDARY_VERSION_AMD64_OBSOLETE_CONTEXT_2: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const KD_SECONDARY_VERSION_DEFAULT: u32 = 0u32;
pub struct KNONVOLATILE_CONTEXT_POINTERS(i32);
pub struct KNONVOLATILE_CONTEXT_POINTERS(i32);
pub struct KNONVOLATILE_CONTEXT_POINTERS_ARM64(i32);
pub struct LDT_ENTRY(i32);
pub struct LOADED_IMAGE(i32);
pub struct LOADED_IMAGE(i32);
pub struct LOAD_DLL_DEBUG_INFO(i32);
pub struct LPCALL_BACK_USER_INTERRUPT_ROUTINE(i32);
pub struct LPTOP_LEVEL_EXCEPTION_FILTER(i32);
pub struct LanguageKind(i32);
pub struct Location(i32);
pub struct LocationKind(i32);
pub struct M128A(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MAX_SYM_NAME: u32 = 2000u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MEMORY_READ_ERROR: u32 = 1u32;
pub struct MINIDUMP_CALLBACK_INFORMATION(i32);
pub struct MINIDUMP_CALLBACK_INPUT(i32);
pub struct MINIDUMP_CALLBACK_OUTPUT(i32);
pub struct MINIDUMP_CALLBACK_ROUTINE(i32);
pub struct MINIDUMP_CALLBACK_TYPE(i32);
pub struct MINIDUMP_DIRECTORY(i32);
pub struct MINIDUMP_EXCEPTION(i32);
pub struct MINIDUMP_EXCEPTION_INFORMATION(i32);
pub struct MINIDUMP_EXCEPTION_INFORMATION64(i32);
pub struct MINIDUMP_EXCEPTION_STREAM(i32);
pub struct MINIDUMP_FUNCTION_TABLE_DESCRIPTOR(i32);
pub struct MINIDUMP_FUNCTION_TABLE_STREAM(i32);
pub struct MINIDUMP_HANDLE_DATA_STREAM(i32);
pub struct MINIDUMP_HANDLE_DESCRIPTOR(i32);
pub struct MINIDUMP_HANDLE_DESCRIPTOR_2(i32);
pub struct MINIDUMP_HANDLE_OBJECT_INFORMATION(i32);
pub struct MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(i32);
pub struct MINIDUMP_HANDLE_OPERATION_LIST(i32);
pub struct MINIDUMP_HEADER(i32);
pub struct MINIDUMP_INCLUDE_MODULE_CALLBACK(i32);
pub struct MINIDUMP_INCLUDE_THREAD_CALLBACK(i32);
pub struct MINIDUMP_IO_CALLBACK(i32);
pub struct MINIDUMP_LOCATION_DESCRIPTOR(i32);
pub struct MINIDUMP_LOCATION_DESCRIPTOR64(i32);
pub struct MINIDUMP_MEMORY64_LIST(i32);
pub struct MINIDUMP_MEMORY_DESCRIPTOR(i32);
pub struct MINIDUMP_MEMORY_DESCRIPTOR64(i32);
pub struct MINIDUMP_MEMORY_INFO(i32);
pub struct MINIDUMP_MEMORY_INFO_LIST(i32);
pub struct MINIDUMP_MEMORY_LIST(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_MISC1_PROCESSOR_POWER_INFO: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_MISC3_PROCESS_EXECUTE_FLAGS: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_MISC3_PROCESS_INTEGRITY: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_MISC3_PROTECTED_PROCESS: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_MISC3_TIMEZONE: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_MISC4_BUILDSTRING: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_MISC5_PROCESS_COOKIE: u32 = 512u32;
pub struct MINIDUMP_MISC_INFO(i32);
pub struct MINIDUMP_MISC_INFO_2(i32);
pub struct MINIDUMP_MISC_INFO_3(i32);
pub struct MINIDUMP_MISC_INFO_4(i32);
pub struct MINIDUMP_MISC_INFO_5(i32);
pub struct MINIDUMP_MISC_INFO_FLAGS(i32);
pub struct MINIDUMP_MODULE(i32);
pub struct MINIDUMP_MODULE_CALLBACK(i32);
pub struct MINIDUMP_MODULE_LIST(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_PROCESS_VM_COUNTERS: u32 = 1u32;
pub struct MINIDUMP_PROCESS_VM_COUNTERS_1(i32);
pub struct MINIDUMP_PROCESS_VM_COUNTERS_2(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_PROCESS_VM_COUNTERS_EX: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_PROCESS_VM_COUNTERS_EX2: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_PROCESS_VM_COUNTERS_JOB: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_PROCESS_VM_COUNTERS_VIRTUALSIZE: u32 = 2u32;
pub struct MINIDUMP_READ_MEMORY_FAILURE_CALLBACK(i32);
pub struct MINIDUMP_SECONDARY_FLAGS(i32);
pub struct MINIDUMP_STREAM_TYPE(i32);
pub struct MINIDUMP_STRING(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_SYSMEMINFO1_BASICPERF: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_SYSMEMINFO1_FILECACHE_TRANSITIONREPURPOSECOUNT_FLAGS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_SYSMEMINFO1_PERF_CCTOTALDIRTYPAGES_CCDIRTYPAGETHRESHOLD: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_SYSMEMINFO1_PERF_RESIDENTAVAILABLEPAGES_SHAREDCOMMITPAGES: u32 = 8u32;
pub struct MINIDUMP_SYSTEM_BASIC_INFORMATION(i32);
pub struct MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION(i32);
pub struct MINIDUMP_SYSTEM_FILECACHE_INFORMATION(i32);
pub struct MINIDUMP_SYSTEM_INFO(i32);
pub struct MINIDUMP_SYSTEM_MEMORY_INFO_1(i32);
pub struct MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION(i32);
pub struct MINIDUMP_THREAD(i32);
pub struct MINIDUMP_THREAD_CALLBACK(i32);
pub struct MINIDUMP_THREAD_CALLBACK(i32);
pub struct MINIDUMP_THREAD_CALLBACK(i32);
pub struct MINIDUMP_THREAD_EX(i32);
pub struct MINIDUMP_THREAD_EX_CALLBACK(i32);
pub struct MINIDUMP_THREAD_EX_CALLBACK(i32);
pub struct MINIDUMP_THREAD_EX_CALLBACK(i32);
pub struct MINIDUMP_THREAD_EX_LIST(i32);
pub struct MINIDUMP_THREAD_INFO(i32);
pub struct MINIDUMP_THREAD_INFO_DUMP_FLAGS(i32);
pub struct MINIDUMP_THREAD_INFO_LIST(i32);
pub struct MINIDUMP_THREAD_LIST(i32);
pub struct MINIDUMP_THREAD_NAME(i32);
pub struct MINIDUMP_THREAD_NAME_LIST(i32);
pub struct MINIDUMP_TOKEN_INFO_HEADER(i32);
pub struct MINIDUMP_TOKEN_INFO_LIST(i32);
pub struct MINIDUMP_TYPE(i32);
pub struct MINIDUMP_UNLOADED_MODULE(i32);
pub struct MINIDUMP_UNLOADED_MODULE_LIST(i32);
pub struct MINIDUMP_USER_RECORD(i32);
pub struct MINIDUMP_USER_STREAM(i32);
pub struct MINIDUMP_USER_STREAM_INFORMATION(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MINIDUMP_VERSION: u32 = 42899u32;
pub struct MINIDUMP_VM_POST_READ_CALLBACK(i32);
pub struct MINIDUMP_VM_PRE_READ_CALLBACK(i32);
pub struct MINIDUMP_VM_QUERY_CALLBACK(i32);
pub struct MODLOAD_CVMISC(i32);
pub struct MODLOAD_DATA(i32);
pub struct MODLOAD_DATA_TYPE(i32);
pub struct MODLOAD_PDBGUID_PDBAGE(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MODULE_ORDERS_LOADTIME: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MODULE_ORDERS_MASK: u32 = 4026531840u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const MODULE_ORDERS_MODULENAME: u32 = 536870912u32;
pub struct MODULE_TYPE_INFO(i32);
pub struct MODULE_WRITE_FLAGS(i32);
pub struct MachineDebugManager_DEBUG(i32);
pub struct MachineDebugManager_RETAIL(i32);
pub struct ModelObjectKind(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const NULL_FIELD_NAME: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const NULL_SYM_DUMP_PARAM: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const NUM_SSRVOPTS: u32 = 32u32;
pub struct OBJECT_ATTRIB_FLAG(i32);
pub const OID_JSSIP: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 113893392, data2: 14542, data3: 4564, data4: [162, 163, 0, 16, 75, 211, 80, 144] };
pub const OID_VBSSIP: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 371847246,
    data2: 10137,
    data3: 19893,
    data4: [143, 229, 172, 225, 15, 23, 235, 171],
};
pub const OID_WSFSIP: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 442566000, data2: 14542, data3: 4564, data4: [162, 163, 0, 16, 75, 211, 80, 144] };
pub struct OMAP(i32);
pub struct OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS(i32);
pub struct OUTPUT_DEBUG_STRING_INFO(i32);
pub struct PCOGETACTIVATIONSTATE(i32);
pub struct PCOGETCALLSTATE(i32);
pub struct PDBGHELP_CREATE_USER_DUMP_CALLBACK(i32);
pub struct PDEBUG_EXTENSION_CALL(i32);
pub struct PDEBUG_EXTENSION_CANUNLOAD(i32);
pub struct PDEBUG_EXTENSION_INITIALIZE(i32);
pub struct PDEBUG_EXTENSION_KNOWN_STRUCT(i32);
pub struct PDEBUG_EXTENSION_KNOWN_STRUCT_EX(i32);
pub struct PDEBUG_EXTENSION_NOTIFY(i32);
pub struct PDEBUG_EXTENSION_PROVIDE_VALUE(i32);
pub struct PDEBUG_EXTENSION_QUERY_VALUE_NAMES(i32);
pub struct PDEBUG_EXTENSION_UNINITIALIZE(i32);
pub struct PDEBUG_EXTENSION_UNLOAD(i32);
pub struct PDEBUG_STACK_PROVIDER_BEGINTHREADSTACKRECONSTRUCTION(i32);
pub struct PDEBUG_STACK_PROVIDER_ENDTHREADSTACKRECONSTRUCTION(i32);
pub struct PDEBUG_STACK_PROVIDER_FREESTACKSYMFRAMES(i32);
pub struct PDEBUG_STACK_PROVIDER_RECONSTRUCTSTACK(i32);
pub struct PENUMDIRTREE_CALLBACK(i32);
pub struct PENUMDIRTREE_CALLBACKW(i32);
pub struct PENUMLOADED_MODULES_CALLBACK(i32);
pub struct PENUMLOADED_MODULES_CALLBACK64(i32);
pub struct PENUMLOADED_MODULES_CALLBACKW64(i32);
pub struct PENUMSOURCEFILETOKENSCALLBACK(i32);
pub struct PFINDFILEINPATHCALLBACK(i32);
pub struct PFINDFILEINPATHCALLBACKW(i32);
pub struct PFIND_DEBUG_FILE_CALLBACK(i32);
pub struct PFIND_DEBUG_FILE_CALLBACKW(i32);
pub struct PFIND_EXE_FILE_CALLBACK(i32);
pub struct PFIND_EXE_FILE_CALLBACKW(i32);
pub struct PFUNCTION_TABLE_ACCESS_ROUTINE(i32);
pub struct PFUNCTION_TABLE_ACCESS_ROUTINE64(i32);
pub struct PGET_MODULE_BASE_ROUTINE(i32);
pub struct PGET_MODULE_BASE_ROUTINE64(i32);
pub struct PGET_RUNTIME_FUNCTION_CALLBACK(i32);
pub struct PGET_RUNTIME_FUNCTION_CALLBACK(i32);
pub struct PHYSICAL(i32);
pub struct PHYSICAL_MEMORY_DESCRIPTOR32(i32);
pub struct PHYSICAL_MEMORY_DESCRIPTOR64(i32);
pub struct PHYSICAL_MEMORY_RUN32(i32);
pub struct PHYSICAL_MEMORY_RUN64(i32);
pub struct PHYSICAL_TO_VIRTUAL(i32);
pub struct PHYSICAL_WITH_FLAGS(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const PHYS_FLAG_CACHED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const PHYS_FLAG_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const PHYS_FLAG_UNCACHED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const PHYS_FLAG_WRITE_COMBINED: u32 = 3u32;
pub struct PIMAGEHLP_STATUS_ROUTINE(i32);
pub struct PIMAGEHLP_STATUS_ROUTINE32(i32);
pub struct PIMAGEHLP_STATUS_ROUTINE64(i32);
pub struct POINTER_SEARCH_PHYSICAL(i32);
pub struct PREAD_PROCESS_MEMORY_ROUTINE(i32);
pub struct PREAD_PROCESS_MEMORY_ROUTINE64(i32);
pub struct PROCESSORINFO(i32);
pub struct PROCESSOR_ARCHITECTURE(i32);
pub struct PROCESS_NAME_ENTRY(i32);
pub struct PROFILER_EVENT_MASK(i32);
pub struct PROFILER_HEAP_ENUM_FLAGS(i32);
pub struct PROFILER_HEAP_OBJECT(i32);
pub struct PROFILER_HEAP_OBJECT_FLAGS(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const PROFILER_HEAP_OBJECT_NAME_ID_UNAVAILABLE: u32 = 4294967295u32;
pub struct PROFILER_HEAP_OBJECT_OPTIONAL_INFO(i32);
pub struct PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(i32);
pub struct PROFILER_HEAP_OBJECT_RELATIONSHIP(i32);
pub struct PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(i32);
pub struct PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST(i32);
pub struct PROFILER_HEAP_OBJECT_SCOPE_LIST(i32);
pub struct PROFILER_HEAP_SUMMARY(i32);
pub struct PROFILER_HEAP_SUMMARY_VERSION(i32);
pub struct PROFILER_PROPERTY_TYPE_SUBSTRING_INFO(i32);
pub struct PROFILER_RELATIONSHIP_INFO(i32);
pub struct PROFILER_SCRIPT_TYPE(i32);
pub struct PROP_INFO_FLAGS(i32);
pub struct PSYMBOLSERVERBYINDEXPROC(i32);
pub struct PSYMBOLSERVERBYINDEXPROCA(i32);
pub struct PSYMBOLSERVERBYINDEXPROCW(i32);
pub struct PSYMBOLSERVERCALLBACKPROC(i32);
pub struct PSYMBOLSERVERCLOSEPROC(i32);
pub struct PSYMBOLSERVERDELTANAME(i32);
pub struct PSYMBOLSERVERDELTANAMEW(i32);
pub struct PSYMBOLSERVERGETINDEXSTRING(i32);
pub struct PSYMBOLSERVERGETINDEXSTRINGW(i32);
pub struct PSYMBOLSERVERGETOPTIONDATAPROC(i32);
pub struct PSYMBOLSERVERGETOPTIONSPROC(i32);
pub struct PSYMBOLSERVERGETSUPPLEMENT(i32);
pub struct PSYMBOLSERVERGETSUPPLEMENTW(i32);
pub struct PSYMBOLSERVERGETVERSION(i32);
pub struct PSYMBOLSERVERISSTORE(i32);
pub struct PSYMBOLSERVERISSTOREW(i32);
pub struct PSYMBOLSERVERMESSAGEPROC(i32);
pub struct PSYMBOLSERVEROPENPROC(i32);
pub struct PSYMBOLSERVERPINGPROC(i32);
pub struct PSYMBOLSERVERPINGPROCA(i32);
pub struct PSYMBOLSERVERPINGPROCW(i32);
pub struct PSYMBOLSERVERPINGPROCWEX(i32);
pub struct PSYMBOLSERVERPROC(i32);
pub struct PSYMBOLSERVERPROCA(i32);
pub struct PSYMBOLSERVERPROCW(i32);
pub struct PSYMBOLSERVERSETHTTPAUTHHEADER(i32);
pub struct PSYMBOLSERVERSETOPTIONSPROC(i32);
pub struct PSYMBOLSERVERSETOPTIONSWPROC(i32);
pub struct PSYMBOLSERVERSTOREFILE(i32);
pub struct PSYMBOLSERVERSTOREFILEW(i32);
pub struct PSYMBOLSERVERSTORESUPPLEMENT(i32);
pub struct PSYMBOLSERVERSTORESUPPLEMENTW(i32);
pub struct PSYMBOLSERVERVERSION(i32);
pub struct PSYMBOLSERVERWEXPROC(i32);
pub struct PSYMBOL_FUNCENTRY_CALLBACK(i32);
pub struct PSYMBOL_FUNCENTRY_CALLBACK64(i32);
pub struct PSYMBOL_REGISTERED_CALLBACK(i32);
pub struct PSYMBOL_REGISTERED_CALLBACK64(i32);
pub struct PSYM_DUMP_FIELD_CALLBACK(i32);
pub struct PSYM_ENUMERATESYMBOLS_CALLBACK(i32);
pub struct PSYM_ENUMERATESYMBOLS_CALLBACKW(i32);
pub struct PSYM_ENUMLINES_CALLBACK(i32);
pub struct PSYM_ENUMLINES_CALLBACKW(i32);
pub struct PSYM_ENUMMODULES_CALLBACK(i32);
pub struct PSYM_ENUMMODULES_CALLBACK64(i32);
pub struct PSYM_ENUMMODULES_CALLBACKW64(i32);
pub struct PSYM_ENUMPROCESSES_CALLBACK(i32);
pub struct PSYM_ENUMSOURCEFILES_CALLBACK(i32);
pub struct PSYM_ENUMSOURCEFILES_CALLBACKW(i32);
pub struct PSYM_ENUMSYMBOLS_CALLBACK(i32);
pub struct PSYM_ENUMSYMBOLS_CALLBACK64(i32);
pub struct PSYM_ENUMSYMBOLS_CALLBACK64W(i32);
pub struct PSYM_ENUMSYMBOLS_CALLBACKW(i32);
pub struct PTRANSLATE_ADDRESS_ROUTINE(i32);
pub struct PTRANSLATE_ADDRESS_ROUTINE64(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const PTR_SEARCH_NO_SYMBOL_CHECK: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const PTR_SEARCH_PHYS_ALL_HITS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const PTR_SEARCH_PHYS_PTE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const PTR_SEARCH_PHYS_RANGE_CHECK_ONLY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const PTR_SEARCH_PHYS_SIZE_SHIFT: u32 = 3u32;
pub struct PVECTORED_EXCEPTION_HANDLER(i32);
pub struct PWAITCHAINCALLBACK(i32);
pub struct PWINDBG_CHECK_CONTROL_C(i32);
pub struct PWINDBG_CHECK_VERSION(i32);
pub struct PWINDBG_DISASM(i32);
pub struct PWINDBG_DISASM32(i32);
pub struct PWINDBG_DISASM64(i32);
pub struct PWINDBG_EXTENSION_API_VERSION(i32);
pub struct PWINDBG_EXTENSION_DLL_INIT(i32);
pub struct PWINDBG_EXTENSION_DLL_INIT32(i32);
pub struct PWINDBG_EXTENSION_DLL_INIT64(i32);
pub struct PWINDBG_EXTENSION_ROUTINE(i32);
pub struct PWINDBG_EXTENSION_ROUTINE32(i32);
pub struct PWINDBG_EXTENSION_ROUTINE64(i32);
pub struct PWINDBG_GET_EXPRESSION(i32);
pub struct PWINDBG_GET_EXPRESSION32(i32);
pub struct PWINDBG_GET_EXPRESSION64(i32);
pub struct PWINDBG_GET_SYMBOL(i32);
pub struct PWINDBG_GET_SYMBOL32(i32);
pub struct PWINDBG_GET_SYMBOL64(i32);
pub struct PWINDBG_GET_THREAD_CONTEXT_ROUTINE(i32);
pub struct PWINDBG_IOCTL_ROUTINE(i32);
pub struct PWINDBG_OLDKD_EXTENSION_ROUTINE(i32);
pub struct PWINDBG_OLDKD_READ_PHYSICAL_MEMORY(i32);
pub struct PWINDBG_OLDKD_WRITE_PHYSICAL_MEMORY(i32);
pub struct PWINDBG_OLD_EXTENSION_ROUTINE(i32);
pub struct PWINDBG_OUTPUT_ROUTINE(i32);
pub struct PWINDBG_READ_PROCESS_MEMORY_ROUTINE(i32);
pub struct PWINDBG_READ_PROCESS_MEMORY_ROUTINE32(i32);
pub struct PWINDBG_READ_PROCESS_MEMORY_ROUTINE64(i32);
pub struct PWINDBG_SET_THREAD_CONTEXT_ROUTINE(i32);
pub struct PWINDBG_STACKTRACE_ROUTINE(i32);
pub struct PWINDBG_STACKTRACE_ROUTINE32(i32);
pub struct PWINDBG_STACKTRACE_ROUTINE64(i32);
pub struct PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE(i32);
pub struct PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE32(i32);
pub struct PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE64(i32);
pub struct PointerKind(i32);
pub struct PreferredFormat(i32);
pub struct ProcessDebugManager(i32);
pub struct READCONTROLSPACE(i32);
pub struct READCONTROLSPACE32(i32);
pub struct READCONTROLSPACE64(i32);
pub struct READ_WRITE_MSR(i32);
pub struct RIP_INFO(i32);
pub struct RIP_INFO_TYPE(i32);
pub struct RTL_VIRTUAL_UNWIND_HANDLER_TYPE(i32);
pub struct RawSearchFlags(i32);
pub struct SCRIPTGCTYPE(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTINFO_ITYPEINFO: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTINFO_IUNKNOWN: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTINTERRUPT_DEBUG: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTINTERRUPT_RAISEEXCEPTION: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTITEM_CODEONLY: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTITEM_GLOBALMEMBERS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTITEM_ISPERSISTENT: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTITEM_ISSOURCE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTITEM_ISVISIBLE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTITEM_NOCODE: u32 = 1024u32;
pub struct SCRIPTLANGUAGEVERSION(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROC_HOSTMANAGESSOURCE: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROC_IMPLICIT_PARENTS: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROC_IMPLICIT_THIS: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROC_ISEXPRESSION: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROC_ISXDOMAIN: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_ABBREVIATE_GLOBALNAME_RESOLUTION: u32 = 1879048194u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_BUILDNUMBER: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_CATCHEXCEPTION: u32 = 4097u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_CONVERSIONLCID: u32 = 4098u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_DEBUGGER: u32 = 4352u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_DELAYEDEVENTSINKING: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_GCCONTROLSOFTCLOSE: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_HACK_FIBERSUPPORT: u32 = 1879048192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_HACK_TRIDENTEVENTSINK: u32 = 1879048193u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_HOSTKEEPALIVE: u32 = 1879048196u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_HOSTSTACKREQUIRED: u32 = 4099u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_INTEGERMODE: u32 = 12288u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_INVOKEVERSIONING: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_JITDEBUG: u32 = 4353u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_MAJORVERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_MINORVERSION: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_NAME: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_SCRIPTSAREFULLYTRUSTED: u32 = 4100u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTPROP_STRINGCOMPAREINSTANCE: u32 = 12289u32;
pub struct SCRIPTSTATE(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTSTAT_INSTRUCTION_COUNT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTSTAT_INTSTRUCTION_TIME: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTSTAT_STATEMENT_COUNT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTSTAT_TOTAL_TIME: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTTEXT_DELAYEXECUTION: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTTEXT_HOSTMANAGESSOURCE: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTTEXT_ISEXPRESSION: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTTEXT_ISNONUSERCODE: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTTEXT_ISPERSISTENT: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTTEXT_ISVISIBLE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTTEXT_ISXDOMAIN: u32 = 256u32;
pub struct SCRIPTTHREADSTATE(i32);
pub struct SCRIPTTRACEINFO(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTTYPELIB_ISCONTROL: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPTTYPELIB_ISPERSISTENT: u32 = 64u32;
pub struct SCRIPTUICHANDLING(i32);
pub struct SCRIPTUICITEM(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPT_CMPL_COMMIT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPT_CMPL_ENUMLIST: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPT_CMPL_ENUM_TRIGGER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPT_CMPL_GLOBALLIST: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPT_CMPL_MEMBERLIST: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPT_CMPL_MEMBER_TRIGGER: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPT_CMPL_NOLIST: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPT_CMPL_PARAMTIP: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPT_CMPL_PARAM_TRIGGER: u32 = 3u32;
pub struct SCRIPT_DEBUGGER_OPTIONS(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPT_ENCODE_DEFAULT_LANGUAGE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPT_ENCODE_NO_ASP_LANGUAGE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPT_ENCODE_SECTION: u32 = 1u32;
pub struct SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPT_E_PROPAGATE: i32 = -2147352318i32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPT_E_RECORDED: i32 = -2040119292i32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SCRIPT_E_REPORTED: i32 = -2147352319i32;
pub struct SCRIPT_INVOCATION_CONTEXT_TYPE(i32);
pub struct SEARCHMEMORY(i32);
pub struct SOURCEFILE(i32);
pub struct SOURCEFILEW(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SOURCETEXT_ATTR_COMMENT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SOURCETEXT_ATTR_FUNCTION_START: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SOURCETEXT_ATTR_HUMANTEXT: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SOURCETEXT_ATTR_IDENTIFIER: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SOURCETEXT_ATTR_KEYWORD: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SOURCETEXT_ATTR_MEMBERLOOKUP: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SOURCETEXT_ATTR_NONSOURCE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SOURCETEXT_ATTR_NUMBER: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SOURCETEXT_ATTR_OPERATOR: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SOURCETEXT_ATTR_STRING: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SOURCETEXT_ATTR_THIS: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SPLITSYM_EXTRACT_ALL: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SPLITSYM_REMOVE_PRIVATE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SPLITSYM_SYMBOLPATH_IS_SRC: u32 = 4u32;
pub struct SRCCODEINFO(i32);
pub struct SRCCODEINFOW(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVACTION_CHECKSUMSTATUS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVACTION_EVENT: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVACTION_EVENTW: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVACTION_HTTPSTATUS: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVACTION_QUERYCANCEL: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVACTION_SIZE: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVACTION_TRACE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVACTION_XMLOUTPUT: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_CALLBACK: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_CALLBACKW: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_DISABLE_PING_HOST: u32 = 67108864u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_DISABLE_TIMEOUT: u32 = 134217728u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_DONT_UNCOMPRESS: u32 = 33554432u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_DOWNSTREAM_STORE: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_ENABLE_COMM_MSG: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_FAVOR_COMPRESSED: u32 = 2097152u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_FLAT_DEFAULT_STORE: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_GETPATH: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_MAX: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_MESSAGE: u32 = 524288u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_NOCOPY: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_OLDGUIDPTR: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_OVERWRITE: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_PARAMTYPE: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_PARENTWIN: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_PROXY: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_PROXYW: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_RESETTOU: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_RETRY_APP_HANG: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_SECURE: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_SERVICE: u32 = 1048576u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_SETCONTEXT: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_STRING: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_TRACE: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_UNATTENDED: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_URI_FILTER: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_URI_TIERS: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_WINHTTP: u32 = 8388608u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVOPT_WININET: u32 = 16777216u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVURI_ALL: u32 = 255u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVURI_COMPRESSED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVURI_FILEPTR: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVURI_HTTP_COMPRESSED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVURI_HTTP_FILEPTR: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVURI_HTTP_MASK: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVURI_HTTP_NORMAL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVURI_NORMAL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVURI_UNC_COMPRESSED: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVURI_UNC_FILEPTR: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVURI_UNC_MASK: u32 = 240u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SSRVURI_UNC_NORMAL: u32 = 16u32;
pub struct STACKFRAME(i32);
pub struct STACKFRAME64(i32);
pub struct STACKFRAME_EX(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const STACK_FRAME_TYPE_IGNORE: u32 = 255u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const STACK_FRAME_TYPE_INIT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const STACK_FRAME_TYPE_INLINE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const STACK_FRAME_TYPE_RA: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const STACK_FRAME_TYPE_STACK: u32 = 1u32;
pub struct STACK_SRC_INFO(i32);
pub struct STACK_SYM_FRAME_INFO(i32);
pub struct SYMADDSOURCESTREAM(i32);
pub struct SYMADDSOURCESTREAMA(i32);
pub struct SYMBOL_INFO(i32);
pub struct SYMBOL_INFOW(i32);
pub struct SYMBOL_INFO_EX(i32);
pub struct SYMBOL_INFO_FLAGS(i32);
pub struct SYMBOL_INFO_PACKAGE(i32);
pub struct SYMBOL_INFO_PACKAGEW(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMBOL_TYPE_INDEX_NOT_FOUND: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMBOL_TYPE_INFO_NOT_FOUND: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMENUM_OPTIONS_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMENUM_OPTIONS_INLINE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMFLAG_FIXUP_ARM64X: u32 = 16777216u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMFLAG_FUNC_NO_RETURN: u32 = 1048576u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMFLAG_GLOBAL: u32 = 33554432u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMFLAG_NULL: u32 = 524288u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMFLAG_PUBLIC_CODE: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMFLAG_REGREL_ALIASINDIR: u32 = 8388608u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMFLAG_RESET: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMFLAG_SYNTHETIC_ZEROBASE: u32 = 2097152u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMF_CONSTANT: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMF_EXPORT: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMF_FORWARDER: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMF_FRAMEREL: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMF_FUNCTION: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMF_LOCAL: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMF_OMAP_GENERATED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMF_OMAP_MODIFIED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMF_PARAMETER: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMF_REGISTER: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMF_REGREL: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMF_THUNK: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMF_TLSREL: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMF_VIRTUAL: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_ALLOW_ABSOLUTE_SYMBOLS: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_ALLOW_ZERO_ADDRESS: u32 = 16777216u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_AUTO_PUBLICS: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_CASE_INSENSITIVE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_DEBUG: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_DEFERRED_LOADS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_DISABLE_FAST_SYMBOLS: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_DISABLE_SRVSTAR_ON_STARTUP: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_DISABLE_SYMSRV_AUTODETECT: u32 = 33554432u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_DISABLE_SYMSRV_TIMEOUT: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_EXACT_SYMBOLS: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_FAIL_CRITICAL_ERRORS: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_FAVOR_COMPRESSED: u32 = 8388608u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_FLAT_DIRECTORY: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_IGNORE_CVREC: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_IGNORE_IMAGEDIR: u32 = 2097152u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_IGNORE_NT_SYMPATH: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_INCLUDE_32BIT_MODULES: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_LOAD_ANYTHING: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_LOAD_LINES: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_NO_CPP: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_NO_IMAGE_SEARCH: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_NO_PROMPTS: u32 = 524288u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_NO_PUBLICS: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_NO_UNQUALIFIED_LOADS: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_OMAP_FIND_NEAREST: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_OVERWRITE: u32 = 1048576u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_PUBLICS_ONLY: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_READONLY_CACHE: u32 = 67108864u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_SECURE: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_SYMPATH_LAST: u32 = 134217728u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMOPT_UNDNAME: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMSEARCH_ALLITEMS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMSEARCH_GLOBALSONLY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMSEARCH_MASKOBJS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMSEARCH_RECURSE: u32 = 2u32;
pub struct SYMSRV_EXTENDED_OUTPUT_DATA(i32);
pub struct SYMSRV_INDEX_INFO(i32);
pub struct SYMSRV_INDEX_INFOW(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMSRV_VERSION: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMSTOREOPT_ALT_INDEX: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYMSTOREOPT_UNICODE: u32 = 32u32;
pub struct SYM_DUMP_PARAM(i32);
pub struct SYM_FIND_ID_OPTION(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYM_INLINE_COMP_DIFFERENT: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYM_INLINE_COMP_ERROR: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYM_INLINE_COMP_IDENTICAL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYM_INLINE_COMP_STEPIN: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYM_INLINE_COMP_STEPOUT: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYM_INLINE_COMP_STEPOVER: u32 = 4u32;
pub struct SYM_LOAD_FLAGS(i32);
pub struct SYM_SRV_STORE_FILE_FLAGS(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYM_STKWALK_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYM_STKWALK_FORCE_FRAMEPTR: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const SYM_STKWALK_ZEROEXTEND_PTRS: u32 = 2u32;
pub struct SYM_TYPE(i32);
pub struct ScriptChangeKind(i32);
pub struct ScriptDebugEvent(i32);
pub struct ScriptDebugEventFilter(i32);
pub struct ScriptDebugEventInformation(i32);
pub struct ScriptDebugPosition(i32);
pub struct ScriptDebugState(i32);
pub struct ScriptExecutionKind(i32);
pub struct SignatureComparison(i32);
pub struct SymbolKind(i32);
pub struct SymbolSearchOptions(i32);
pub struct TEXT_DOCUMENT_ARRAY(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const TEXT_DOC_ATTR_READONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const TEXT_DOC_ATTR_TYPE_PRIMARY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const TEXT_DOC_ATTR_TYPE_SCRIPT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const TEXT_DOC_ATTR_TYPE_WORKER: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const THREAD_BLOCKED: u32 = 4u32;
pub struct THREAD_ERROR_MODE(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const THREAD_OUT_OF_CONTEXT: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const THREAD_STATE_RUNNING: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const THREAD_STATE_SUSPENDED: u32 = 2u32;
pub struct THREAD_WRITE_FLAGS(i32);
pub struct TI_FINDCHILDREN_PARAMS(i32);
pub struct TRANSLATE_VIRTUAL_TO_PHYSICAL(i32);
pub struct TypeKind(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNAVAILABLE_ERROR: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_32_BIT_DECODE: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_COMPLETE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_NAME_ONLY: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_NO_ACCESS_SPECIFIERS: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_NO_ALLOCATION_LANGUAGE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_NO_ALLOCATION_MODEL: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_NO_ARGUMENTS: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_NO_CV_THISTYPE: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_NO_FUNCTION_RETURNS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_NO_LEADING_UNDERSCORES: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_NO_MEMBER_TYPE: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_NO_MS_KEYWORDS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_NO_MS_THISTYPE: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_NO_RETURN_UDT_MODEL: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_NO_SPECIAL_SYMS: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_NO_THISTYPE: u32 = 96u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const UNDNAME_NO_THROW_SIGNATURES: u32 = 256u32;
pub struct UNLOAD_DLL_DEBUG_INFO(i32);
pub struct UNWIND_HISTORY_TABLE(i32);
pub struct UNWIND_HISTORY_TABLE_ENTRY(i32);
pub struct UNWIND_HISTORY_TABLE_ENTRY(i32);
pub struct VER_PLATFORM(i32);
pub struct VIRTUAL_TO_PHYSICAL(i32);
pub struct VarArgsKind(i32);
pub struct WAITCHAIN_NODE_INFO(i32);
pub struct WAIT_CHAIN_THREAD_OPTIONS(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WCT_MAX_NODE_COUNT: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WCT_NETWORK_IO_FLAG: u32 = 8u32;
pub struct WCT_OBJECT_STATUS(i32);
pub struct WCT_OBJECT_TYPE(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WCT_OBJNAME_LENGTH: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WDBGEXTS_ADDRESS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WDBGEXTS_ADDRESS_RESERVED0: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WDBGEXTS_ADDRESS_SEG16: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WDBGEXTS_ADDRESS_SEG32: u32 = 2u32;
pub struct WDBGEXTS_CLR_DATA_INTERFACE(i32);
pub struct WDBGEXTS_DISASSEMBLE_BUFFER(i32);
pub struct WDBGEXTS_MODULE_IN_RANGE(i32);
pub struct WDBGEXTS_QUERY_INTERFACE(i32);
pub struct WDBGEXTS_THREAD_OS_INFO(i32);
pub struct WHEA_AER_BRIDGE_DESCRIPTOR(i32);
pub struct WHEA_AER_ENDPOINT_DESCRIPTOR(i32);
pub struct WHEA_AER_ROOTPORT_DESCRIPTOR(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_BAD_PAGE_LIST_LOCATION: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_BAD_PAGE_LIST_MAX_SIZE: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_CMCI_THRESHOLD_COUNT: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_CMCI_THRESHOLD_POLL_COUNT: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_CMCI_THRESHOLD_TIME: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_DEVICE_DRIVER_BUFFER_SET_MAX: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_DEVICE_DRIVER_BUFFER_SET_MIN: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_DEVICE_DRIVER_BUFFER_SET_V1: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_DEVICE_DRIVER_CONFIG_MAX: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_DEVICE_DRIVER_CONFIG_MIN: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_DEVICE_DRIVER_CONFIG_V1: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_DEVICE_DRIVER_CONFIG_V2: u32 = 2u32;
pub struct WHEA_DEVICE_DRIVER_DESCRIPTOR(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_DISABLE_DUMMY_WRITE: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_DISABLE_OFFLINE: u32 = 0u32;
pub struct WHEA_DRIVER_BUFFER_SET(i32);
pub struct WHEA_ERROR_SOURCE_CONFIGURATION_DD(i32);
pub struct WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER(i32);
pub struct WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1(i32);
pub struct WHEA_ERROR_SOURCE_CORRECT_DEVICE_DRIVER(i32);
pub struct WHEA_ERROR_SOURCE_DESCRIPTOR(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_AERBRIDGE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_AERENDPOINT: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_AERROOTPORT: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_GENERIC: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_GENERIC_V2: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_IPFCMC: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_IPFCPE: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_IPFMCA: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_XPFCMC: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_XPFMCE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_XPFNMI: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_VERSION_10: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_VERSION_11: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_FLAG_DEFAULTSOURCE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_FLAG_FIRMWAREFIRST: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_FLAG_GHES_ASSIST: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_FLAG_GLOBAL: u32 = 2u32;
pub struct WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_ERROR_SOURCE_INVALID_RELATED_SOURCE: u32 = 65535u32;
pub struct WHEA_ERROR_SOURCE_STATE(i32);
pub struct WHEA_ERROR_SOURCE_TYPE(i32);
pub struct WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER(i32);
pub struct WHEA_GENERIC_ERROR_DESCRIPTOR(i32);
pub struct WHEA_GENERIC_ERROR_DESCRIPTOR_V2(i32);
pub struct WHEA_IPF_CMC_DESCRIPTOR(i32);
pub struct WHEA_IPF_CPE_DESCRIPTOR(i32);
pub struct WHEA_IPF_MCA_DESCRIPTOR(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_MAX_MC_BANKS: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_MEM_PERSISTOFFLINE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_MEM_PFA_DISABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_MEM_PFA_PAGECOUNT: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_MEM_PFA_THRESHOLD: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_MEM_PFA_TIMEOUT: u32 = 5u32;
pub struct WHEA_NOTIFICATION_DESCRIPTOR(i32);
pub struct WHEA_NOTIFICATION_FLAGS(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_NOTIFICATION_TYPE_ARMV8_SEA: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_NOTIFICATION_TYPE_ARMV8_SEI: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_NOTIFICATION_TYPE_CMCI: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_NOTIFICATION_TYPE_EXTERNALINTERRUPT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_NOTIFICATION_TYPE_EXTERNALINTERRUPT_GSIV: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_NOTIFICATION_TYPE_GPIO_SIGNAL: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_NOTIFICATION_TYPE_LOCALINTERRUPT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_NOTIFICATION_TYPE_MCE: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_NOTIFICATION_TYPE_NMI: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_NOTIFICATION_TYPE_POLLED: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_NOTIFICATION_TYPE_SCI: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_NOTIFICATION_TYPE_SDEI: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_NOTIFY_ALL_OFFLINES: u32 = 16u32;
pub struct WHEA_PCI_SLOT_NUMBER(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_PENDING_PAGE_LIST_SZ: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_RESTORE_CMCI_ATTEMPTS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_RESTORE_CMCI_ENABLED: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_RESTORE_CMCI_ERR_LIMIT: u32 = 9u32;
pub struct WHEA_XPF_CMC_DESCRIPTOR(i32);
pub struct WHEA_XPF_MCE_DESCRIPTOR(i32);
pub struct WHEA_XPF_MC_BANK_DESCRIPTOR(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_XPF_MC_BANK_STATUSFORMAT_AMD64MCA: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_XPF_MC_BANK_STATUSFORMAT_IA32MCA: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WHEA_XPF_MC_BANK_STATUSFORMAT_Intel64MCA: u32 = 1u32;
pub struct WHEA_XPF_NMI_DESCRIPTOR(i32);
pub struct WINDBG_EXTENSION_APIS(i32);
pub struct WINDBG_EXTENSION_APIS32(i32);
pub struct WINDBG_EXTENSION_APIS64(i32);
pub struct WINDBG_OLDKD_EXTENSION_APIS(i32);
pub struct WINDBG_OLD_EXTENSION_APIS(i32);
pub struct WOW64_CONTEXT(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WOW64_CONTEXT_EXCEPTION_ACTIVE: u32 = 134217728u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WOW64_CONTEXT_EXCEPTION_REPORTING: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WOW64_CONTEXT_EXCEPTION_REQUEST: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WOW64_CONTEXT_SERVICE_ACTIVE: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WOW64_CONTEXT_i386: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WOW64_CONTEXT_i486: u32 = 65536u32;
pub struct WOW64_DESCRIPTOR_TABLE_ENTRY(i32);
pub struct WOW64_FLOATING_SAVE_AREA(i32);
pub struct WOW64_LDT_ENTRY(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WOW64_MAXIMUM_SUPPORTED_EXTENSION: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const WOW64_SIZE_OF_80387_REGISTERS: u32 = 80u32;
pub struct XPF_MCE_FLAGS(i32);
pub struct XPF_MC_BANK_FLAGS(i32);
pub struct XSAVE_AREA(i32);
pub struct XSAVE_AREA_HEADER(i32);
pub struct XSAVE_FORMAT(i32);
pub struct XSAVE_FORMAT(i32);
pub struct XSTATE_CONFIGURATION(i32);
pub struct XSTATE_CONFIG_FEATURE_MSC_INFO(i32);
pub struct XSTATE_CONTEXT(i32);
pub struct XSTATE_CONTEXT(i32);
pub struct XSTATE_FEATURE(i32);
pub struct _DUMP_TYPES(i32);
pub struct _GETSETBUSDATA(i32);
pub struct _IMAGEHLP_JIT_SYMBOL_MAP(i32);
pub struct __MIDL___MIDL_itf_jscript9diag_0000_0007_0001(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const fasaCaseSensitive: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const fasaPreferInternalHandler: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const fasaSupportInternalHandler: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
pub const sevMax: i32 = 4i32;
