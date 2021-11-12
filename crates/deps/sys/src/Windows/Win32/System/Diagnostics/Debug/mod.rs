#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Win32_System_Diagnostics_Debug_WebApp")]
pub mod WebApp;
#[link(name = "windows")]
extern "system" {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn AddVectoredContinueHandler(first: u32, handler: PVECTORED_EXCEPTION_HANDLER) -> *mut ::core::ffi::c_void;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn AddVectoredExceptionHandler(first: u32, handler: PVECTORED_EXCEPTION_HANDLER) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Beep(dwfreq: u32, dwduration: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BindImage(imagename: super::super::super::Foundation::PSTR, dllpath: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn BindImageEx(flags: u32, imagename: super::super::super::Foundation::PSTR, dllpath: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, statusroutine: PIMAGEHLP_STATUS_ROUTINE) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckRemoteDebuggerPresent(hprocess: super::super::super::Foundation::HANDLE, pbdebuggerpresent: *mut super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn CheckSumMappedFile(baseaddress: *const ::core::ffi::c_void, filelength: u32, headersum: *mut u32, checksum: *mut u32) -> *mut IMAGE_NT_HEADERS64;
    #[cfg(any(target_arch = "x86",))]
    pub fn CheckSumMappedFile(baseaddress: *const ::core::ffi::c_void, filelength: u32, headersum: *mut u32, checksum: *mut u32) -> *mut IMAGE_NT_HEADERS32;
    pub fn CloseThreadWaitChainSession(wcthandle: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ContinueDebugEvent(dwprocessid: u32, dwthreadid: u32, dwcontinuestatus: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn CopyContext(destination: *mut CONTEXT, contextflags: u32, source: *const CONTEXT) -> super::super::super::Foundation::BOOL;
    pub fn CreateDataModelManager(debughost: IDebugHost, manager: *mut IDataModelManager) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DbgHelpCreateUserDump(filename: super::super::super::Foundation::PSTR, callback: PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DbgHelpCreateUserDumpW(filename: super::super::super::Foundation::PWSTR, callback: PDBGHELP_CREATE_USER_DUMP_CALLBACK, userdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugActiveProcess(dwprocessid: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugActiveProcessStop(dwprocessid: u32) -> super::super::super::Foundation::BOOL;
    pub fn DebugBreak();
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugBreakProcess(process: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugConnect(remoteoptions: super::super::super::Foundation::PSTR, interfaceid: *const ::windows_sys::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugConnectWide(remoteoptions: super::super::super::Foundation::PWSTR, interfaceid: *const ::windows_sys::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn DebugCreate(interfaceid: *const ::windows_sys::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn DebugCreateEx(interfaceid: *const ::windows_sys::core::GUID, dbgengoptions: u32, interface: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugSetProcessKillOnExit(killonexit: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    pub fn DecodePointer(ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DecodeRemotePointer(processhandle: super::super::super::Foundation::HANDLE, ptr: *const ::core::ffi::c_void, decodedptr: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn DecodeSystemPointer(ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    pub fn EncodePointer(ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EncodeRemotePointer(processhandle: super::super::super::Foundation::HANDLE, ptr: *const ::core::ffi::c_void, encodedptr: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn EncodeSystemPointer(ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDirTree(hprocess: super::super::super::Foundation::HANDLE, rootpath: super::super::super::Foundation::PSTR, inputpathname: super::super::super::Foundation::PSTR, outputpathbuffer: super::super::super::Foundation::PSTR, cb: PENUMDIRTREE_CALLBACK, data: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDirTreeW(hprocess: super::super::super::Foundation::HANDLE, rootpath: super::super::super::Foundation::PWSTR, inputpathname: super::super::super::Foundation::PWSTR, outputpathbuffer: super::super::super::Foundation::PWSTR, cb: PENUMDIRTREE_CALLBACKW, data: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModules(hprocess: super::super::super::Foundation::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModules64(hprocess: super::super::super::Foundation::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACK64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModulesEx(hprocess: super::super::super::Foundation::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACK64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModulesExW(hprocess: super::super::super::Foundation::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACKW64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModulesW64(hprocess: super::super::super::Foundation::HANDLE, enumloadedmodulescallback: PENUMLOADED_MODULES_CALLBACKW64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FatalAppExitA(uaction: u32, lpmessagetext: super::super::super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn FatalAppExitW(uaction: u32, lpmessagetext: super::super::super::Foundation::PWSTR);
    pub fn FatalExit(exitcode: i32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindDebugInfoFile(filename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, debugfilepath: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindDebugInfoFileEx(filename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, debugfilepath: super::super::super::Foundation::PSTR, callback: PFIND_DEBUG_FILE_CALLBACK, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindDebugInfoFileExW(filename: super::super::super::Foundation::PWSTR, symbolpath: super::super::super::Foundation::PWSTR, debugfilepath: super::super::super::Foundation::PWSTR, callback: PFIND_DEBUG_FILE_CALLBACKW, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableImage(filename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, imagefilepath: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableImageEx(filename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, imagefilepath: super::super::super::Foundation::PSTR, callback: PFIND_EXE_FILE_CALLBACK, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableImageExW(filename: super::super::super::Foundation::PWSTR, symbolpath: super::super::super::Foundation::PWSTR, imagefilepath: super::super::super::Foundation::PWSTR, callback: PFIND_EXE_FILE_CALLBACKW, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFileInPath(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PSTR, filename: super::super::super::Foundation::PSTR, id: *const ::core::ffi::c_void, two: u32, three: u32, flags: u32, filepath: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFileInSearchPath(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PSTR, filename: super::super::super::Foundation::PSTR, one: u32, two: u32, three: u32, filepath: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushInstructionCache(hprocess: super::super::super::Foundation::HANDLE, lpbaseaddress: *const ::core::ffi::c_void, dwsize: usize) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FormatMessageA(dwflags: FORMAT_MESSAGE_OPTIONS, lpsource: *const ::core::ffi::c_void, dwmessageid: u32, dwlanguageid: u32, lpbuffer: super::super::super::Foundation::PSTR, nsize: u32, arguments: *const *const i8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FormatMessageW(dwflags: FORMAT_MESSAGE_OPTIONS, lpsource: *const ::core::ffi::c_void, dwmessageid: u32, dwlanguageid: u32, lpbuffer: super::super::super::Foundation::PWSTR, nsize: u32, arguments: *const *const i8) -> u32;
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    pub fn GetEnabledXStateFeatures() -> u64;
    pub fn GetErrorMode() -> u32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetImageConfigInformation(loadedimage: *const LOADED_IMAGE, imageconfiginformation: *mut IMAGE_LOAD_CONFIG_DIRECTORY64) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetImageConfigInformation(loadedimage: *const LOADED_IMAGE, imageconfiginformation: *mut IMAGE_LOAD_CONFIG_DIRECTORY32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetImageUnusedHeaderBytes(loadedimage: *const LOADED_IMAGE, sizeunusedheaderbytes: *mut u32) -> u32;
    pub fn GetSymLoadError() -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetThreadContext(hthread: super::super::super::Foundation::HANDLE, lpcontext: *mut CONTEXT) -> super::super::super::Foundation::BOOL;
    pub fn GetThreadErrorMode() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadSelectorEntry(hthread: super::super::super::Foundation::HANDLE, dwselector: u32, lpselectorentry: *mut LDT_ENTRY) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadWaitChain(wcthandle: *const ::core::ffi::c_void, context: usize, flags: WAIT_CHAIN_THREAD_OPTIONS, threadid: u32, nodecount: *mut u32, nodeinfoarray: *mut WAITCHAIN_NODE_INFO, iscycle: *mut i32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimestampForLoadedLibrary(module: super::super::super::Foundation::HINSTANCE) -> u32;
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetXStateFeaturesMask(context: *const CONTEXT, featuremask: *mut u64) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_WinTrust"))]
    pub fn ImageAddCertificate(filehandle: super::super::super::Foundation::HANDLE, certificate: *const super::super::super::Security::WinTrust::WIN_CERTIFICATE, index: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageDirectoryEntryToData(base: *const ::core::ffi::c_void, mappedasimage: super::super::super::Foundation::BOOLEAN, directoryentry: IMAGE_DIRECTORY_ENTRY, size: *mut u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageDirectoryEntryToDataEx(base: *const ::core::ffi::c_void, mappedasimage: super::super::super::Foundation::BOOLEAN, directoryentry: IMAGE_DIRECTORY_ENTRY, size: *mut u32, foundheader: *mut *mut IMAGE_SECTION_HEADER) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageEnumerateCertificates(filehandle: super::super::super::Foundation::HANDLE, typefilter: u16, certificatecount: *mut u32, indices: *mut u32, indexcount: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_WinTrust"))]
    pub fn ImageGetCertificateData(filehandle: super::super::super::Foundation::HANDLE, certificateindex: u32, certificate: *mut super::super::super::Security::WinTrust::WIN_CERTIFICATE, requiredlength: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_WinTrust"))]
    pub fn ImageGetCertificateHeader(filehandle: super::super::super::Foundation::HANDLE, certificateindex: u32, certificateheader: *mut super::super::super::Security::WinTrust::WIN_CERTIFICATE) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageGetDigestStream(filehandle: super::super::super::Foundation::HANDLE, digestlevel: u32, digestfunction: DIGEST_FUNCTION, digesthandle: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn ImageLoad(dllname: super::super::super::Foundation::PSTR, dllpath: super::super::super::Foundation::PSTR) -> *mut LOADED_IMAGE;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn ImageNtHeader(base: *const ::core::ffi::c_void) -> *mut IMAGE_NT_HEADERS64;
    #[cfg(any(target_arch = "x86",))]
    pub fn ImageNtHeader(base: *const ::core::ffi::c_void) -> *mut IMAGE_NT_HEADERS32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageRemoveCertificate(filehandle: super::super::super::Foundation::HANDLE, index: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn ImageRvaToSection(ntheaders: *const IMAGE_NT_HEADERS64, base: *const ::core::ffi::c_void, rva: u32) -> *mut IMAGE_SECTION_HEADER;
    #[cfg(any(target_arch = "x86",))]
    pub fn ImageRvaToSection(ntheaders: *const IMAGE_NT_HEADERS32, base: *const ::core::ffi::c_void, rva: u32) -> *mut IMAGE_SECTION_HEADER;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn ImageRvaToVa(ntheaders: *const IMAGE_NT_HEADERS64, base: *const ::core::ffi::c_void, rva: u32, lastrvasection: *const *const IMAGE_SECTION_HEADER) -> *mut ::core::ffi::c_void;
    #[cfg(any(target_arch = "x86",))]
    pub fn ImageRvaToVa(ntheaders: *const IMAGE_NT_HEADERS32, base: *const ::core::ffi::c_void, rva: u32, lastrvasection: *const *const IMAGE_SECTION_HEADER) -> *mut ::core::ffi::c_void;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn ImageUnload(loadedimage: *mut LOADED_IMAGE) -> super::super::super::Foundation::BOOL;
    pub fn ImagehlpApiVersion() -> *mut API_VERSION;
    pub fn ImagehlpApiVersionEx(appversion: *const API_VERSION) -> *mut API_VERSION;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn InitializeContext(buffer: *mut ::core::ffi::c_void, contextflags: u32, context: *mut *mut CONTEXT, contextlength: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn InitializeContext2(buffer: *mut ::core::ffi::c_void, contextflags: u32, context: *mut *mut CONTEXT, contextlength: *mut u32, xstatecompactionmask: u64) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDebuggerPresent() -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn LocateXStateFeature(context: *const CONTEXT, featureid: u32, length: *mut u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MakeSureDirectoryPathExists(dirpath: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn MapAndLoad(imagename: super::super::super::Foundation::PSTR, dllpath: super::super::super::Foundation::PSTR, loadedimage: *mut LOADED_IMAGE, dotdll: super::super::super::Foundation::BOOL, readonly: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapFileAndCheckSumA(filename: super::super::super::Foundation::PSTR, headersum: *mut u32, checksum: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapFileAndCheckSumW(filename: super::super::super::Foundation::PWSTR, headersum: *mut u32, checksum: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MessageBeep(utype: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MiniDumpReadDumpStream(baseofdump: *const ::core::ffi::c_void, streamnumber: u32, dir: *mut *mut MINIDUMP_DIRECTORY, streampointer: *mut *mut ::core::ffi::c_void, streamsize: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Memory"))]
    pub fn MiniDumpWriteDump(hprocess: super::super::super::Foundation::HANDLE, processid: u32, hfile: super::super::super::Foundation::HANDLE, dumptype: MINIDUMP_TYPE, exceptionparam: *const MINIDUMP_EXCEPTION_INFORMATION, userstreamparam: *const MINIDUMP_USER_STREAM_INFORMATION, callbackparam: *const MINIDUMP_CALLBACK_INFORMATION) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenThreadWaitChainSession(flags: OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS, callback: PWAITCHAINCALLBACK) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OutputDebugStringA(lpoutputstring: super::super::super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn OutputDebugStringW(lpoutputstring: super::super::super::Foundation::PWSTR);
    pub fn RaiseException(dwexceptioncode: u32, dwexceptionflags: u32, nnumberofarguments: u32, lparguments: *const usize);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RaiseFailFastException(pexceptionrecord: *const EXCEPTION_RECORD, pcontextrecord: *const CONTEXT, dwflags: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RangeMapAddPeImageSections(rmaphandle: *const ::core::ffi::c_void, imagename: super::super::super::Foundation::PWSTR, mappedimage: *const ::core::ffi::c_void, mappingbytes: u32, imagebase: u64, usertag: u64, mappingflags: u32) -> super::super::super::Foundation::BOOL;
    pub fn RangeMapCreate() -> *mut ::core::ffi::c_void;
    pub fn RangeMapFree(rmaphandle: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RangeMapRead(rmaphandle: *const ::core::ffi::c_void, offset: u64, buffer: *mut ::core::ffi::c_void, requestbytes: u32, flags: u32, donebytes: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RangeMapRemove(rmaphandle: *const ::core::ffi::c_void, usertag: u64) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RangeMapWrite(rmaphandle: *const ::core::ffi::c_void, offset: u64, buffer: *const ::core::ffi::c_void, requestbytes: u32, flags: u32, donebytes: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReBaseImage(currentimagename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, frebase: super::super::super::Foundation::BOOL, frebasesysfileok: super::super::super::Foundation::BOOL, fgoingdown: super::super::super::Foundation::BOOL, checkimagesize: u32, oldimagesize: *mut u32, oldimagebase: *mut usize, newimagesize: *mut u32, newimagebase: *mut usize, timestamp: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReBaseImage64(currentimagename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, frebase: super::super::super::Foundation::BOOL, frebasesysfileok: super::super::super::Foundation::BOOL, fgoingdown: super::super::super::Foundation::BOOL, checkimagesize: u32, oldimagesize: *mut u32, oldimagebase: *mut u64, newimagesize: *mut u32, newimagebase: *mut u64, timestamp: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadProcessMemory(hprocess: super::super::super::Foundation::HANDLE, lpbaseaddress: *const ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, nsize: usize, lpnumberofbytesread: *mut usize) -> super::super::super::Foundation::BOOL;
    pub fn RegisterWaitChainCOMCallback(callstatecallback: PCOGETCALLSTATE, activationstatecallback: PCOGETACTIVATIONSTATE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveInvalidModuleList(hprocess: super::super::super::Foundation::HANDLE);
    pub fn RemoveVectoredContinueHandler(handle: *const ::core::ffi::c_void) -> u32;
    pub fn RemoveVectoredExceptionHandler(handle: *const ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportSymbolLoadSummary(hprocess: super::super::super::Foundation::HANDLE, ploadmodule: super::super::super::Foundation::PWSTR, psymboldata: *const DBGHELP_DATA_REPORT_STRUCT) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlAddFunctionTable(functiontable: *const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY, entrycount: u32, baseaddress: usize) -> super::super::super::Foundation::BOOLEAN;
    #[cfg(any(target_arch = "x86_64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlAddFunctionTable(functiontable: *const IMAGE_RUNTIME_FUNCTION_ENTRY, entrycount: u32, baseaddress: u64) -> super::super::super::Foundation::BOOLEAN;
    #[cfg(any(target_arch = "aarch64",))]
    pub fn RtlAddGrowableFunctionTable(dynamictable: *mut *mut ::core::ffi::c_void, functiontable: *const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY, entrycount: u32, maximumentrycount: u32, rangebase: usize, rangeend: usize) -> u32;
    #[cfg(any(target_arch = "x86_64",))]
    pub fn RtlAddGrowableFunctionTable(dynamictable: *mut *mut ::core::ffi::c_void, functiontable: *const IMAGE_RUNTIME_FUNCTION_ENTRY, entrycount: u32, maximumentrycount: u32, rangebase: usize, rangeend: usize) -> u32;
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn RtlCaptureContext(contextrecord: *mut CONTEXT);
    #[cfg(any(target_arch = "x86_64",))]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn RtlCaptureContext2(contextrecord: *mut CONTEXT);
    pub fn RtlCaptureStackBackTrace(framestoskip: u32, framestocapture: u32, backtrace: *mut *mut ::core::ffi::c_void, backtracehash: *mut u32) -> u16;
    #[cfg(any(target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlDeleteFunctionTable(functiontable: *const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY) -> super::super::super::Foundation::BOOLEAN;
    #[cfg(any(target_arch = "x86_64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlDeleteFunctionTable(functiontable: *const IMAGE_RUNTIME_FUNCTION_ENTRY) -> super::super::super::Foundation::BOOLEAN;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn RtlDeleteGrowableFunctionTable(dynamictable: *const ::core::ffi::c_void);
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn RtlGrowFunctionTable(dynamictable: *mut ::core::ffi::c_void, newentrycount: u32);
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlInstallFunctionTableCallback(tableidentifier: u64, baseaddress: u64, length: u32, callback: PGET_RUNTIME_FUNCTION_CALLBACK, context: *const ::core::ffi::c_void, outofprocesscallbackdll: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOLEAN;
    #[cfg(any(target_arch = "aarch64",))]
    pub fn RtlLookupFunctionEntry(controlpc: usize, imagebase: *mut usize, historytable: *mut UNWIND_HISTORY_TABLE) -> *mut IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY;
    #[cfg(any(target_arch = "x86_64",))]
    pub fn RtlLookupFunctionEntry(controlpc: u64, imagebase: *mut u64, historytable: *mut UNWIND_HISTORY_TABLE) -> *mut IMAGE_RUNTIME_FUNCTION_ENTRY;
    pub fn RtlPcToFileHeader(pcvalue: *const ::core::ffi::c_void, baseofimage: *mut *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlRaiseException(exceptionrecord: *const EXCEPTION_RECORD);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlRestoreContext(contextrecord: *const CONTEXT, exceptionrecord: *const EXCEPTION_RECORD);
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlUnwind(targetframe: *const ::core::ffi::c_void, targetip: *const ::core::ffi::c_void, exceptionrecord: *const EXCEPTION_RECORD, returnvalue: *const ::core::ffi::c_void);
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlUnwindEx(targetframe: *const ::core::ffi::c_void, targetip: *const ::core::ffi::c_void, exceptionrecord: *const EXCEPTION_RECORD, returnvalue: *const ::core::ffi::c_void, contextrecord: *const CONTEXT, historytable: *const UNWIND_HISTORY_TABLE);
    #[cfg(any(target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlVirtualUnwind(handlertype: RTL_VIRTUAL_UNWIND_HANDLER_TYPE, imagebase: usize, controlpc: usize, functionentry: *const IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY, contextrecord: *mut CONTEXT, handlerdata: *mut *mut ::core::ffi::c_void, establisherframe: *mut usize, contextpointers: *mut KNONVOLATILE_CONTEXT_POINTERS_ARM64) -> ::core::option::Option<super::super::Kernel::EXCEPTION_ROUTINE>;
    #[cfg(any(target_arch = "x86_64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlVirtualUnwind(handlertype: RTL_VIRTUAL_UNWIND_HANDLER_TYPE, imagebase: u64, controlpc: u64, functionentry: *const IMAGE_RUNTIME_FUNCTION_ENTRY, contextrecord: *mut CONTEXT, handlerdata: *mut *mut ::core::ffi::c_void, establisherframe: *mut u64, contextpointers: *mut KNONVOLATILE_CONTEXT_POINTERS) -> ::core::option::Option<super::super::Kernel::EXCEPTION_ROUTINE>;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SearchTreeForFile(rootpath: super::super::super::Foundation::PSTR, inputpathname: super::super::super::Foundation::PSTR, outputpathbuffer: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SearchTreeForFileW(rootpath: super::super::super::Foundation::PWSTR, inputpathname: super::super::super::Foundation::PWSTR, outputpathbuffer: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
    pub fn SetCheckUserInterruptShared(lpstartaddress: LPCALL_BACK_USER_INTERRUPT_ROUTINE);
    pub fn SetErrorMode(umode: THREAD_ERROR_MODE) -> u32;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetImageConfigInformation(loadedimage: *mut LOADED_IMAGE, imageconfiginformation: *const IMAGE_LOAD_CONFIG_DIRECTORY64) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetImageConfigInformation(loadedimage: *mut LOADED_IMAGE, imageconfiginformation: *const IMAGE_LOAD_CONFIG_DIRECTORY32) -> super::super::super::Foundation::BOOL;
    pub fn SetSymLoadError(error: u32);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetThreadContext(hthread: super::super::super::Foundation::HANDLE, lpcontext: *const CONTEXT) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadErrorMode(dwnewmode: THREAD_ERROR_MODE, lpoldmode: *const THREAD_ERROR_MODE) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetUnhandledExceptionFilter(lptoplevelexceptionfilter: LPTOP_LEVEL_EXCEPTION_FILTER) -> ::core::option::Option<LPTOP_LEVEL_EXCEPTION_FILTER>;
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetXStateFeaturesMask(context: *mut CONTEXT, featuremask: u64) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StackWalk(machinetype: u32, hprocess: super::super::super::Foundation::HANDLE, hthread: super::super::super::Foundation::HANDLE, stackframe: *mut STACKFRAME, contextrecord: *mut ::core::ffi::c_void, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE, translateaddress: PTRANSLATE_ADDRESS_ROUTINE) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StackWalk64(machinetype: u32, hprocess: super::super::super::Foundation::HANDLE, hthread: super::super::super::Foundation::HANDLE, stackframe: *mut STACKFRAME64, contextrecord: *mut ::core::ffi::c_void, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64, translateaddress: PTRANSLATE_ADDRESS_ROUTINE64) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StackWalkEx(machinetype: u32, hprocess: super::super::super::Foundation::HANDLE, hthread: super::super::super::Foundation::HANDLE, stackframe: *mut STACKFRAME_EX, contextrecord: *mut ::core::ffi::c_void, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, functiontableaccessroutine: PFUNCTION_TABLE_ACCESS_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64, translateaddress: PTRANSLATE_ADDRESS_ROUTINE64, flags: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddSourceStream(hprocess: super::super::super::Foundation::HANDLE, base: u64, streamfile: super::super::super::Foundation::PSTR, buffer: *const u8, size: usize) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddSourceStreamA(hprocess: super::super::super::Foundation::HANDLE, base: u64, streamfile: super::super::super::Foundation::PSTR, buffer: *const u8, size: usize) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddSourceStreamW(hprocess: super::super::super::Foundation::HANDLE, base: u64, filespec: super::super::super::Foundation::PWSTR, buffer: *const u8, size: usize) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddSymbol(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, name: super::super::super::Foundation::PSTR, address: u64, size: u32, flags: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddSymbolW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, name: super::super::super::Foundation::PWSTR, address: u64, size: u32, flags: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddrIncludeInlineTrace(hprocess: super::super::super::Foundation::HANDLE, address: u64) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymCleanup(hprocess: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymCompareInlineTrace(hprocess: super::super::super::Foundation::HANDLE, address1: u64, inlinecontext1: u32, retaddress1: u64, address2: u64, retaddress2: u64) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymDeleteSymbol(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, name: super::super::super::Foundation::PSTR, address: u64, flags: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymDeleteSymbolW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, name: super::super::super::Foundation::PWSTR, address: u64, flags: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumLines(hprocess: super::super::super::Foundation::HANDLE, base: u64, obj: super::super::super::Foundation::PSTR, file: super::super::super::Foundation::PSTR, enumlinescallback: PSYM_ENUMLINES_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumLinesW(hprocess: super::super::super::Foundation::HANDLE, base: u64, obj: super::super::super::Foundation::PWSTR, file: super::super::super::Foundation::PWSTR, enumlinescallback: PSYM_ENUMLINES_CALLBACKW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumProcesses(enumprocessescallback: PSYM_ENUMPROCESSES_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceFileTokens(hprocess: super::super::super::Foundation::HANDLE, base: u64, callback: PENUMSOURCEFILETOKENSCALLBACK) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceFiles(hprocess: super::super::super::Foundation::HANDLE, modbase: u64, mask: super::super::super::Foundation::PSTR, cbsrcfiles: PSYM_ENUMSOURCEFILES_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceFilesW(hprocess: super::super::super::Foundation::HANDLE, modbase: u64, mask: super::super::super::Foundation::PWSTR, cbsrcfiles: PSYM_ENUMSOURCEFILES_CALLBACKW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceLines(hprocess: super::super::super::Foundation::HANDLE, base: u64, obj: super::super::super::Foundation::PSTR, file: super::super::super::Foundation::PSTR, line: u32, flags: u32, enumlinescallback: PSYM_ENUMLINES_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceLinesW(hprocess: super::super::super::Foundation::HANDLE, base: u64, obj: super::super::super::Foundation::PWSTR, file: super::super::super::Foundation::PWSTR, line: u32, flags: u32, enumlinescallback: PSYM_ENUMLINES_CALLBACKW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSym(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbols(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PSTR, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsEx(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PSTR, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: *const ::core::ffi::c_void, options: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsExW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PWSTR, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: *const ::core::ffi::c_void, options: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsForAddr(hprocess: super::super::super::Foundation::HANDLE, address: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsForAddrW(hprocess: super::super::super::Foundation::HANDLE, address: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PWSTR, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumTypes(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumTypesByName(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PSTR, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumTypesByNameW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PWSTR, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumTypesW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateModules(hprocess: super::super::super::Foundation::HANDLE, enummodulescallback: PSYM_ENUMMODULES_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateModules64(hprocess: super::super::super::Foundation::HANDLE, enummodulescallback: PSYM_ENUMMODULES_CALLBACK64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateModulesW64(hprocess: super::super::super::Foundation::HANDLE, enummodulescallback: PSYM_ENUMMODULES_CALLBACKW64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateSymbols(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u32, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateSymbols64(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateSymbolsW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u32, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACKW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateSymbolsW64(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, enumsymbolscallback: PSYM_ENUMSYMBOLS_CALLBACK64W, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindDebugInfoFile(hprocess: super::super::super::Foundation::HANDLE, filename: super::super::super::Foundation::PSTR, debugfilepath: super::super::super::Foundation::PSTR, callback: PFIND_DEBUG_FILE_CALLBACK, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindDebugInfoFileW(hprocess: super::super::super::Foundation::HANDLE, filename: super::super::super::Foundation::PWSTR, debugfilepath: super::super::super::Foundation::PWSTR, callback: PFIND_DEBUG_FILE_CALLBACKW, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindExecutableImage(hprocess: super::super::super::Foundation::HANDLE, filename: super::super::super::Foundation::PSTR, imagefilepath: super::super::super::Foundation::PSTR, callback: PFIND_EXE_FILE_CALLBACK, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindExecutableImageW(hprocess: super::super::super::Foundation::HANDLE, filename: super::super::super::Foundation::PWSTR, imagefilepath: super::super::super::Foundation::PWSTR, callback: PFIND_EXE_FILE_CALLBACKW, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindFileInPath(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PSTR, filename: super::super::super::Foundation::PSTR, id: *const ::core::ffi::c_void, two: u32, three: u32, flags: SYM_FIND_ID_OPTION, foundfile: super::super::super::Foundation::PSTR, callback: PFINDFILEINPATHCALLBACK, context: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindFileInPathW(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PWSTR, filename: super::super::super::Foundation::PWSTR, id: *const ::core::ffi::c_void, two: u32, three: u32, flags: SYM_FIND_ID_OPTION, foundfile: super::super::super::Foundation::PWSTR, callback: PFINDFILEINPATHCALLBACKW, context: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromAddr(hprocess: super::super::super::Foundation::HANDLE, address: u64, displacement: *mut u64, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromAddrW(hprocess: super::super::super::Foundation::HANDLE, address: u64, displacement: *mut u64, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromIndex(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, index: u32, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromIndexW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, index: u32, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromInlineContext(hprocess: super::super::super::Foundation::HANDLE, address: u64, inlinecontext: u32, displacement: *mut u64, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromInlineContextW(hprocess: super::super::super::Foundation::HANDLE, address: u64, inlinecontext: u32, displacement: *mut u64, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromName(hprocess: super::super::super::Foundation::HANDLE, name: super::super::super::Foundation::PSTR, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromNameW(hprocess: super::super::super::Foundation::HANDLE, name: super::super::super::Foundation::PWSTR, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromToken(hprocess: super::super::super::Foundation::HANDLE, base: u64, token: u32, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromTokenW(hprocess: super::super::super::Foundation::HANDLE, base: u64, token: u32, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFunctionTableAccess(hprocess: super::super::super::Foundation::HANDLE, addrbase: u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFunctionTableAccess64(hprocess: super::super::super::Foundation::HANDLE, addrbase: u64) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFunctionTableAccess64AccessRoutines(hprocess: super::super::super::Foundation::HANDLE, addrbase: u64, readmemoryroutine: PREAD_PROCESS_MEMORY_ROUTINE64, getmodulebaseroutine: PGET_MODULE_BASE_ROUTINE64) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetExtendedOption(option: IMAGEHLP_EXTENDED_OPTIONS) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetFileLineOffsets64(hprocess: super::super::super::Foundation::HANDLE, modulename: super::super::super::Foundation::PSTR, filename: super::super::super::Foundation::PSTR, buffer: *mut u64, bufferlines: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetHomeDirectory(r#type: IMAGEHLP_HD_TYPE, dir: super::super::super::Foundation::PSTR, size: usize) -> super::super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetHomeDirectoryW(r#type: IMAGEHLP_HD_TYPE, dir: super::super::super::Foundation::PWSTR, size: usize) -> super::super::super::Foundation::PWSTR;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromAddr(hprocess: super::super::super::Foundation::HANDLE, dwaddr: u32, pdwdisplacement: *mut u32, line: *mut IMAGEHLP_LINE) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromAddr64(hprocess: super::super::super::Foundation::HANDLE, qwaddr: u64, pdwdisplacement: *mut u32, line64: *mut IMAGEHLP_LINE64) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromAddrW64(hprocess: super::super::super::Foundation::HANDLE, dwaddr: u64, pdwdisplacement: *mut u32, line: *mut IMAGEHLP_LINEW64) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromInlineContext(hprocess: super::super::super::Foundation::HANDLE, qwaddr: u64, inlinecontext: u32, qwmodulebaseaddress: u64, pdwdisplacement: *mut u32, line64: *mut IMAGEHLP_LINE64) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromInlineContextW(hprocess: super::super::super::Foundation::HANDLE, dwaddr: u64, inlinecontext: u32, qwmodulebaseaddress: u64, pdwdisplacement: *mut u32, line: *mut IMAGEHLP_LINEW64) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromName(hprocess: super::super::super::Foundation::HANDLE, modulename: super::super::super::Foundation::PSTR, filename: super::super::super::Foundation::PSTR, dwlinenumber: u32, pldisplacement: *mut i32, line: *mut IMAGEHLP_LINE) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromName64(hprocess: super::super::super::Foundation::HANDLE, modulename: super::super::super::Foundation::PSTR, filename: super::super::super::Foundation::PSTR, dwlinenumber: u32, pldisplacement: *mut i32, line: *mut IMAGEHLP_LINE64) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromNameW64(hprocess: super::super::super::Foundation::HANDLE, modulename: super::super::super::Foundation::PWSTR, filename: super::super::super::Foundation::PWSTR, dwlinenumber: u32, pldisplacement: *mut i32, line: *mut IMAGEHLP_LINEW64) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineNext(hprocess: super::super::super::Foundation::HANDLE, line: *mut IMAGEHLP_LINE) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineNext64(hprocess: super::super::super::Foundation::HANDLE, line: *mut IMAGEHLP_LINE64) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineNextW64(hprocess: super::super::super::Foundation::HANDLE, line: *mut IMAGEHLP_LINEW64) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLinePrev(hprocess: super::super::super::Foundation::HANDLE, line: *mut IMAGEHLP_LINE) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLinePrev64(hprocess: super::super::super::Foundation::HANDLE, line: *mut IMAGEHLP_LINE64) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLinePrevW64(hprocess: super::super::super::Foundation::HANDLE, line: *mut IMAGEHLP_LINEW64) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleBase(hprocess: super::super::super::Foundation::HANDLE, dwaddr: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleBase64(hprocess: super::super::super::Foundation::HANDLE, qwaddr: u64) -> u64;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleInfo(hprocess: super::super::super::Foundation::HANDLE, dwaddr: u32, moduleinfo: *mut IMAGEHLP_MODULE) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleInfo64(hprocess: super::super::super::Foundation::HANDLE, qwaddr: u64, moduleinfo: *mut IMAGEHLP_MODULE64) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleInfoW(hprocess: super::super::super::Foundation::HANDLE, dwaddr: u32, moduleinfo: *mut IMAGEHLP_MODULEW) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleInfoW64(hprocess: super::super::super::Foundation::HANDLE, qwaddr: u64, moduleinfo: *mut IMAGEHLP_MODULEW64) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetOmaps(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, omapto: *mut *mut OMAP, comapto: *mut u64, omapfrom: *mut *mut OMAP, comapfrom: *mut u64) -> super::super::super::Foundation::BOOL;
    pub fn SymGetOptions() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetScope(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, index: u32, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetScopeW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, index: u32, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSearchPath(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PSTR, searchpathlength: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSearchPathW(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PWSTR, searchpathlength: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFile(hprocess: super::super::super::Foundation::HANDLE, base: u64, params: super::super::super::Foundation::PSTR, filespec: super::super::super::Foundation::PSTR, filepath: super::super::super::Foundation::PSTR, size: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileChecksum(hprocess: super::super::super::Foundation::HANDLE, base: u64, filespec: super::super::super::Foundation::PSTR, pchecksumtype: *mut u32, pchecksum: *mut u8, checksumsize: u32, pactualbyteswritten: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileChecksumW(hprocess: super::super::super::Foundation::HANDLE, base: u64, filespec: super::super::super::Foundation::PWSTR, pchecksumtype: *mut u32, pchecksum: *mut u8, checksumsize: u32, pactualbyteswritten: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileFromToken(hprocess: super::super::super::Foundation::HANDLE, token: *const ::core::ffi::c_void, params: super::super::super::Foundation::PSTR, filepath: super::super::super::Foundation::PSTR, size: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileFromTokenByTokenName(hprocess: super::super::super::Foundation::HANDLE, token: *const ::core::ffi::c_void, tokenname: super::super::super::Foundation::PSTR, params: super::super::super::Foundation::PSTR, filepath: super::super::super::Foundation::PSTR, size: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileFromTokenByTokenNameW(hprocess: super::super::super::Foundation::HANDLE, token: *const ::core::ffi::c_void, tokenname: super::super::super::Foundation::PWSTR, params: super::super::super::Foundation::PWSTR, filepath: super::super::super::Foundation::PWSTR, size: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileFromTokenW(hprocess: super::super::super::Foundation::HANDLE, token: *const ::core::ffi::c_void, params: super::super::super::Foundation::PWSTR, filepath: super::super::super::Foundation::PWSTR, size: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileToken(hprocess: super::super::super::Foundation::HANDLE, base: u64, filespec: super::super::super::Foundation::PSTR, token: *mut *mut ::core::ffi::c_void, size: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileTokenByTokenName(hprocess: super::super::super::Foundation::HANDLE, base: u64, filespec: super::super::super::Foundation::PSTR, tokenname: super::super::super::Foundation::PSTR, tokenparameters: super::super::super::Foundation::PSTR, token: *mut *mut ::core::ffi::c_void, size: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileTokenByTokenNameW(hprocess: super::super::super::Foundation::HANDLE, base: u64, filespec: super::super::super::Foundation::PWSTR, tokenname: super::super::super::Foundation::PWSTR, tokenparameters: super::super::super::Foundation::PWSTR, token: *mut *mut ::core::ffi::c_void, size: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileTokenW(hprocess: super::super::super::Foundation::HANDLE, base: u64, filespec: super::super::super::Foundation::PWSTR, token: *mut *mut ::core::ffi::c_void, size: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileW(hprocess: super::super::super::Foundation::HANDLE, base: u64, params: super::super::super::Foundation::PWSTR, filespec: super::super::super::Foundation::PWSTR, filepath: super::super::super::Foundation::PWSTR, size: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceVarFromToken(hprocess: super::super::super::Foundation::HANDLE, token: *const ::core::ffi::c_void, params: super::super::super::Foundation::PSTR, varname: super::super::super::Foundation::PSTR, value: super::super::super::Foundation::PSTR, size: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceVarFromTokenW(hprocess: super::super::super::Foundation::HANDLE, token: *const ::core::ffi::c_void, params: super::super::super::Foundation::PWSTR, varname: super::super::super::Foundation::PWSTR, value: super::super::super::Foundation::PWSTR, size: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymFromAddr(hprocess: super::super::super::Foundation::HANDLE, dwaddr: u32, pdwdisplacement: *mut u32, symbol: *mut IMAGEHLP_SYMBOL) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymFromAddr64(hprocess: super::super::super::Foundation::HANDLE, qwaddr: u64, pdwdisplacement: *mut u64, symbol: *mut IMAGEHLP_SYMBOL64) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymFromName(hprocess: super::super::super::Foundation::HANDLE, name: super::super::super::Foundation::PSTR, symbol: *mut IMAGEHLP_SYMBOL) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymFromName64(hprocess: super::super::super::Foundation::HANDLE, name: super::super::super::Foundation::PSTR, symbol: *mut IMAGEHLP_SYMBOL64) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymNext(hprocess: super::super::super::Foundation::HANDLE, symbol: *mut IMAGEHLP_SYMBOL) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymNext64(hprocess: super::super::super::Foundation::HANDLE, symbol: *mut IMAGEHLP_SYMBOL64) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymPrev(hprocess: super::super::super::Foundation::HANDLE, symbol: *mut IMAGEHLP_SYMBOL) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymPrev64(hprocess: super::super::super::Foundation::HANDLE, symbol: *mut IMAGEHLP_SYMBOL64) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymbolFile(hprocess: super::super::super::Foundation::HANDLE, sympath: super::super::super::Foundation::PSTR, imagefile: super::super::super::Foundation::PSTR, r#type: IMAGEHLP_SF_TYPE, symbolfile: super::super::super::Foundation::PSTR, csymbolfile: usize, dbgfile: super::super::super::Foundation::PSTR, cdbgfile: usize) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymbolFileW(hprocess: super::super::super::Foundation::HANDLE, sympath: super::super::super::Foundation::PWSTR, imagefile: super::super::super::Foundation::PWSTR, r#type: IMAGEHLP_SF_TYPE, symbolfile: super::super::super::Foundation::PWSTR, csymbolfile: usize, dbgfile: super::super::super::Foundation::PWSTR, cdbgfile: usize) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetTypeFromName(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, name: super::super::super::Foundation::PSTR, symbol: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetTypeFromNameW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, name: super::super::super::Foundation::PWSTR, symbol: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetTypeInfo(hprocess: super::super::super::Foundation::HANDLE, modbase: u64, typeid: u32, gettype: IMAGEHLP_SYMBOL_TYPE_INFO, pinfo: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetTypeInfoEx(hprocess: super::super::super::Foundation::HANDLE, modbase: u64, params: *mut IMAGEHLP_GET_TYPE_INFO_PARAMS) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetUnwindInfo(hprocess: super::super::super::Foundation::HANDLE, address: u64, buffer: *mut ::core::ffi::c_void, size: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymInitialize(hprocess: super::super::super::Foundation::HANDLE, usersearchpath: super::super::super::Foundation::PSTR, finvadeprocess: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymInitializeW(hprocess: super::super::super::Foundation::HANDLE, usersearchpath: super::super::super::Foundation::PWSTR, finvadeprocess: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymLoadModule(hprocess: super::super::super::Foundation::HANDLE, hfile: super::super::super::Foundation::HANDLE, imagename: super::super::super::Foundation::PSTR, modulename: super::super::super::Foundation::PSTR, baseofdll: u32, sizeofdll: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymLoadModule64(hprocess: super::super::super::Foundation::HANDLE, hfile: super::super::super::Foundation::HANDLE, imagename: super::super::super::Foundation::PSTR, modulename: super::super::super::Foundation::PSTR, baseofdll: u64, sizeofdll: u32) -> u64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymLoadModuleEx(hprocess: super::super::super::Foundation::HANDLE, hfile: super::super::super::Foundation::HANDLE, imagename: super::super::super::Foundation::PSTR, modulename: super::super::super::Foundation::PSTR, baseofdll: u64, dllsize: u32, data: *const MODLOAD_DATA, flags: SYM_LOAD_FLAGS) -> u64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymLoadModuleExW(hprocess: super::super::super::Foundation::HANDLE, hfile: super::super::super::Foundation::HANDLE, imagename: super::super::super::Foundation::PWSTR, modulename: super::super::super::Foundation::PWSTR, baseofdll: u64, dllsize: u32, data: *const MODLOAD_DATA, flags: SYM_LOAD_FLAGS) -> u64;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymMatchFileName(filename: super::super::super::Foundation::PSTR, r#match: super::super::super::Foundation::PSTR, filenamestop: *mut super::super::super::Foundation::PSTR, matchstop: *mut super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymMatchFileNameW(filename: super::super::super::Foundation::PWSTR, r#match: super::super::super::Foundation::PWSTR, filenamestop: *mut super::super::super::Foundation::PWSTR, matchstop: *mut super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymMatchString(string: super::super::super::Foundation::PSTR, expression: super::super::super::Foundation::PSTR, fcase: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymMatchStringA(string: super::super::super::Foundation::PSTR, expression: super::super::super::Foundation::PSTR, fcase: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymMatchStringW(string: super::super::super::Foundation::PWSTR, expression: super::super::super::Foundation::PWSTR, fcase: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymNext(hprocess: super::super::super::Foundation::HANDLE, si: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymNextW(hprocess: super::super::super::Foundation::HANDLE, siw: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymPrev(hprocess: super::super::super::Foundation::HANDLE, si: *mut SYMBOL_INFO) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymPrevW(hprocess: super::super::super::Foundation::HANDLE, siw: *mut SYMBOL_INFOW) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymQueryInlineTrace(hprocess: super::super::super::Foundation::HANDLE, startaddress: u64, startcontext: u32, startretaddress: u64, curaddress: u64, curcontext: *mut u32, curframeindex: *mut u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRefreshModuleList(hprocess: super::super::super::Foundation::HANDLE) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterCallback(hprocess: super::super::super::Foundation::HANDLE, callbackfunction: PSYMBOL_REGISTERED_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterCallback64(hprocess: super::super::super::Foundation::HANDLE, callbackfunction: PSYMBOL_REGISTERED_CALLBACK64, usercontext: u64) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterCallbackW64(hprocess: super::super::super::Foundation::HANDLE, callbackfunction: PSYMBOL_REGISTERED_CALLBACK64, usercontext: u64) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterFunctionEntryCallback(hprocess: super::super::super::Foundation::HANDLE, callbackfunction: PSYMBOL_FUNCENTRY_CALLBACK, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterFunctionEntryCallback64(hprocess: super::super::super::Foundation::HANDLE, callbackfunction: PSYMBOL_FUNCENTRY_CALLBACK64, usercontext: u64) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSearch(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, index: u32, symtag: u32, mask: super::super::super::Foundation::PSTR, address: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACK, usercontext: *const ::core::ffi::c_void, options: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSearchW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, index: u32, symtag: u32, mask: super::super::super::Foundation::PWSTR, address: u64, enumsymbolscallback: PSYM_ENUMERATESYMBOLS_CALLBACKW, usercontext: *const ::core::ffi::c_void, options: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetContext(hprocess: super::super::super::Foundation::HANDLE, stackframe: *const IMAGEHLP_STACK_FRAME, context: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetExtendedOption(option: IMAGEHLP_EXTENDED_OPTIONS, value: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetHomeDirectory(hprocess: super::super::super::Foundation::HANDLE, dir: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetHomeDirectoryW(hprocess: super::super::super::Foundation::HANDLE, dir: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::PWSTR;
    pub fn SymSetOptions(symoptions: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetParentWindow(hwnd: super::super::super::Foundation::HWND) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetScopeFromAddr(hprocess: super::super::super::Foundation::HANDLE, address: u64) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetScopeFromIndex(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, index: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetScopeFromInlineContext(hprocess: super::super::super::Foundation::HANDLE, address: u64, inlinecontext: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetSearchPath(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetSearchPathW(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvDeltaName(hprocess: super::super::super::Foundation::HANDLE, sympath: super::super::super::Foundation::PSTR, r#type: super::super::super::Foundation::PSTR, file1: super::super::super::Foundation::PSTR, file2: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvDeltaNameW(hprocess: super::super::super::Foundation::HANDLE, sympath: super::super::super::Foundation::PWSTR, r#type: super::super::super::Foundation::PWSTR, file1: super::super::super::Foundation::PWSTR, file2: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexInfo(file: super::super::super::Foundation::PSTR, info: *mut SYMSRV_INDEX_INFO, flags: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexInfoW(file: super::super::super::Foundation::PWSTR, info: *mut SYMSRV_INDEX_INFOW, flags: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexString(hprocess: super::super::super::Foundation::HANDLE, srvpath: super::super::super::Foundation::PSTR, file: super::super::super::Foundation::PSTR, index: super::super::super::Foundation::PSTR, size: usize, flags: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexStringW(hprocess: super::super::super::Foundation::HANDLE, srvpath: super::super::super::Foundation::PWSTR, file: super::super::super::Foundation::PWSTR, index: super::super::super::Foundation::PWSTR, size: usize, flags: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexes(file: super::super::super::Foundation::PSTR, id: *mut ::windows_sys::core::GUID, val1: *mut u32, val2: *mut u32, flags: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexesW(file: super::super::super::Foundation::PWSTR, id: *mut ::windows_sys::core::GUID, val1: *mut u32, val2: *mut u32, flags: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetSupplement(hprocess: super::super::super::Foundation::HANDLE, sympath: super::super::super::Foundation::PSTR, node: super::super::super::Foundation::PSTR, file: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetSupplementW(hprocess: super::super::super::Foundation::HANDLE, sympath: super::super::super::Foundation::PWSTR, node: super::super::super::Foundation::PWSTR, file: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvIsStore(hprocess: super::super::super::Foundation::HANDLE, path: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvIsStoreW(hprocess: super::super::super::Foundation::HANDLE, path: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvStoreFile(hprocess: super::super::super::Foundation::HANDLE, srvpath: super::super::super::Foundation::PSTR, file: super::super::super::Foundation::PSTR, flags: SYM_SRV_STORE_FILE_FLAGS) -> super::super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvStoreFileW(hprocess: super::super::super::Foundation::HANDLE, srvpath: super::super::super::Foundation::PWSTR, file: super::super::super::Foundation::PWSTR, flags: SYM_SRV_STORE_FILE_FLAGS) -> super::super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvStoreSupplement(hprocess: super::super::super::Foundation::HANDLE, srvpath: super::super::super::Foundation::PSTR, node: super::super::super::Foundation::PSTR, file: super::super::super::Foundation::PSTR, flags: u32) -> super::super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvStoreSupplementW(hprocess: super::super::super::Foundation::HANDLE, sympath: super::super::super::Foundation::PWSTR, node: super::super::super::Foundation::PWSTR, file: super::super::super::Foundation::PWSTR, flags: u32) -> super::super::super::Foundation::PWSTR;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymUnDName(sym: *const IMAGEHLP_SYMBOL, undecname: super::super::super::Foundation::PSTR, undecnamelength: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymUnDName64(sym: *const IMAGEHLP_SYMBOL64, undecname: super::super::super::Foundation::PSTR, undecnamelength: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymUnloadModule(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymUnloadModule64(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64) -> super::super::super::Foundation::BOOL;
    pub fn TerminateProcessOnMemoryExhaustion(failedallocationsize: usize);
    #[cfg(feature = "Win32_Foundation")]
    pub fn TouchFileTimes(filehandle: super::super::super::Foundation::HANDLE, psystemtime: *const super::super::super::Foundation::SYSTEMTIME) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnDecorateSymbolName(name: super::super::super::Foundation::PSTR, outputstring: super::super::super::Foundation::PSTR, maxstringlength: u32, flags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnDecorateSymbolNameW(name: super::super::super::Foundation::PWSTR, outputstring: super::super::super::Foundation::PWSTR, maxstringlength: u32, flags: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn UnMapAndLoad(loadedimage: *mut LOADED_IMAGE) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn UnhandledExceptionFilter(exceptioninfo: *const EXCEPTION_POINTERS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateDebugInfoFile(imagefilename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, debugfilepath: super::super::super::Foundation::PSTR, ntheaders: *const IMAGE_NT_HEADERS32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateDebugInfoFileEx(imagefilename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, debugfilepath: super::super::super::Foundation::PSTR, ntheaders: *const IMAGE_NT_HEADERS32, oldchecksum: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn WaitForDebugEvent(lpdebugevent: *mut DEBUG_EVENT, dwmilliseconds: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn WaitForDebugEventEx(lpdebugevent: *mut DEBUG_EVENT, dwmilliseconds: u32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64GetThreadContext(hthread: super::super::super::Foundation::HANDLE, lpcontext: *mut WOW64_CONTEXT) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64GetThreadSelectorEntry(hthread: super::super::super::Foundation::HANDLE, dwselector: u32, lpselectorentry: *mut WOW64_LDT_ENTRY) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64SetThreadContext(hthread: super::super::super::Foundation::HANDLE, lpcontext: *const WOW64_CONTEXT) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProcessMemory(hprocess: super::super::super::Foundation::HANDLE, lpbaseaddress: *const ::core::ffi::c_void, lpbuffer: *const ::core::ffi::c_void, nsize: usize, lpnumberofbyteswritten: *mut usize) -> super::super::super::Foundation::BOOL;
}
pub const ACTIVPROF_E_PROFILER_ABSENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220991i32 as _);
pub const ACTIVPROF_E_PROFILER_PRESENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220992i32 as _);
pub const ACTIVPROF_E_UNABLE_TO_APPLY_ACTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147220990i32 as _);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct ADDRESS(i32);
#[repr(C)]
pub struct ADDRESS64(i32);
#[repr(transparent)]
pub struct ADDRESS_MODE(pub i32);
pub const AddrMode1616: ADDRESS_MODE = ADDRESS_MODE(0i32);
pub const AddrMode1632: ADDRESS_MODE = ADDRESS_MODE(1i32);
pub const AddrModeReal: ADDRESS_MODE = ADDRESS_MODE(2i32);
pub const AddrModeFlat: ADDRESS_MODE = ADDRESS_MODE(3i32);
pub const ADDRESS_TYPE_INDEX_NOT_FOUND: u32 = 11u32;
#[repr(C)]
pub struct AER_BRIDGE_DESCRIPTOR_FLAGS(i32);
#[repr(C)]
pub struct AER_ENDPOINT_DESCRIPTOR_FLAGS(i32);
#[repr(C)]
pub struct AER_ROOTPORT_DESCRIPTOR_FLAGS(i32);
#[repr(C)]
pub struct API_VERSION(i32);
pub const API_VERSION_NUMBER: u32 = 12u32;
pub const APPBREAKFLAG_DEBUGGER_BLOCK: u32 = 1u32;
pub const APPBREAKFLAG_DEBUGGER_HALT: u32 = 2u32;
pub const APPBREAKFLAG_IN_BREAKPOINT: u32 = 2147483648u32;
pub const APPBREAKFLAG_NESTED: u32 = 131072u32;
pub const APPBREAKFLAG_STEP: u32 = 65536u32;
pub const APPBREAKFLAG_STEPTYPE_BYTECODE: u32 = 1048576u32;
pub const APPBREAKFLAG_STEPTYPE_MACHINE: u32 = 2097152u32;
pub const APPBREAKFLAG_STEPTYPE_MASK: u32 = 15728640u32;
pub const APPBREAKFLAG_STEPTYPE_SOURCE: u32 = 0u32;
#[repr(transparent)]
pub struct APPLICATION_NODE_EVENT_FILTER(pub i32);
pub const FILTER_EXCLUDE_NOTHING: APPLICATION_NODE_EVENT_FILTER = APPLICATION_NODE_EVENT_FILTER(0i32);
pub const FILTER_EXCLUDE_ANONYMOUS_CODE: APPLICATION_NODE_EVENT_FILTER = APPLICATION_NODE_EVENT_FILTER(1i32);
pub const FILTER_EXCLUDE_EVAL_CODE: APPLICATION_NODE_EVENT_FILTER = APPLICATION_NODE_EVENT_FILTER(2i32);
#[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
#[repr(C)]
pub struct ARM64_NT_CONTEXT(i32);
#[repr(C)]
pub struct ARM64_NT_NEON128(i32);
#[repr(C)]
pub struct ArrayDimension(i32);
#[repr(transparent)]
pub struct AsyncIDebugApplicationNodeEvents(pub *mut ::core::ffi::c_void);
pub const BIND_ALL_IMAGES: u32 = 4u32;
pub const BIND_CACHE_IMPORT_DLLS: u32 = 8u32;
pub const BIND_NO_BOUND_IMPORTS: u32 = 1u32;
pub const BIND_NO_UPDATE: u32 = 2u32;
pub const BIND_REPORT_64BIT_VA: u32 = 16u32;
#[repr(transparent)]
pub struct BREAKPOINT_STATE(pub i32);
pub const BREAKPOINT_DELETED: BREAKPOINT_STATE = BREAKPOINT_STATE(0i32);
pub const BREAKPOINT_DISABLED: BREAKPOINT_STATE = BREAKPOINT_STATE(1i32);
pub const BREAKPOINT_ENABLED: BREAKPOINT_STATE = BREAKPOINT_STATE(2i32);
#[repr(transparent)]
pub struct BREAKREASON(pub i32);
pub const BREAKREASON_STEP: BREAKREASON = BREAKREASON(0i32);
pub const BREAKREASON_BREAKPOINT: BREAKREASON = BREAKREASON(1i32);
pub const BREAKREASON_DEBUGGER_BLOCK: BREAKREASON = BREAKREASON(2i32);
pub const BREAKREASON_HOST_INITIATED: BREAKREASON = BREAKREASON(3i32);
pub const BREAKREASON_LANGUAGE_INITIATED: BREAKREASON = BREAKREASON(4i32);
pub const BREAKREASON_DEBUGGER_HALT: BREAKREASON = BREAKREASON(5i32);
pub const BREAKREASON_ERROR: BREAKREASON = BREAKREASON(6i32);
pub const BREAKREASON_JIT: BREAKREASON = BREAKREASON(7i32);
pub const BREAKREASON_MUTATION_BREAKPOINT: BREAKREASON = BREAKREASON(8i32);
#[repr(transparent)]
pub struct BREAKRESUME_ACTION(pub i32);
pub const BREAKRESUMEACTION_ABORT: BREAKRESUME_ACTION = BREAKRESUME_ACTION(0i32);
pub const BREAKRESUMEACTION_CONTINUE: BREAKRESUME_ACTION = BREAKRESUME_ACTION(1i32);
pub const BREAKRESUMEACTION_STEP_INTO: BREAKRESUME_ACTION = BREAKRESUME_ACTION(2i32);
pub const BREAKRESUMEACTION_STEP_OVER: BREAKRESUME_ACTION = BREAKRESUME_ACTION(3i32);
pub const BREAKRESUMEACTION_STEP_OUT: BREAKRESUME_ACTION = BREAKRESUME_ACTION(4i32);
pub const BREAKRESUMEACTION_IGNORE: BREAKRESUME_ACTION = BREAKRESUME_ACTION(5i32);
pub const BREAKRESUMEACTION_STEP_DOCUMENT: BREAKRESUME_ACTION = BREAKRESUME_ACTION(6i32);
#[repr(transparent)]
pub struct BUGCHECK_ERROR(pub u32);
pub const HARDWARE_PROFILE_UNDOCKED_STRING: BUGCHECK_ERROR = BUGCHECK_ERROR(1073807361u32);
pub const HARDWARE_PROFILE_DOCKED_STRING: BUGCHECK_ERROR = BUGCHECK_ERROR(1073807362u32);
pub const HARDWARE_PROFILE_UNKNOWN_STRING: BUGCHECK_ERROR = BUGCHECK_ERROR(1073807363u32);
pub const WINDOWS_NT_BANNER: BUGCHECK_ERROR = BUGCHECK_ERROR(1073741950u32);
pub const WINDOWS_NT_CSD_STRING: BUGCHECK_ERROR = BUGCHECK_ERROR(1073741959u32);
pub const WINDOWS_NT_INFO_STRING: BUGCHECK_ERROR = BUGCHECK_ERROR(1073741960u32);
pub const WINDOWS_NT_MP_STRING: BUGCHECK_ERROR = BUGCHECK_ERROR(1073741961u32);
pub const THREAD_TERMINATE_HELD_MUTEX: BUGCHECK_ERROR = BUGCHECK_ERROR(1073741962u32);
pub const WINDOWS_NT_INFO_STRING_PLURAL: BUGCHECK_ERROR = BUGCHECK_ERROR(1073741981u32);
pub const WINDOWS_NT_RC_STRING: BUGCHECK_ERROR = BUGCHECK_ERROR(1073741982u32);
pub const APC_INDEX_MISMATCH: BUGCHECK_ERROR = BUGCHECK_ERROR(1u32);
pub const DEVICE_QUEUE_NOT_BUSY: BUGCHECK_ERROR = BUGCHECK_ERROR(2u32);
pub const INVALID_AFFINITY_SET: BUGCHECK_ERROR = BUGCHECK_ERROR(3u32);
pub const INVALID_DATA_ACCESS_TRAP: BUGCHECK_ERROR = BUGCHECK_ERROR(4u32);
pub const INVALID_PROCESS_ATTACH_ATTEMPT: BUGCHECK_ERROR = BUGCHECK_ERROR(5u32);
pub const INVALID_PROCESS_DETACH_ATTEMPT: BUGCHECK_ERROR = BUGCHECK_ERROR(6u32);
pub const INVALID_SOFTWARE_INTERRUPT: BUGCHECK_ERROR = BUGCHECK_ERROR(7u32);
pub const IRQL_NOT_DISPATCH_LEVEL: BUGCHECK_ERROR = BUGCHECK_ERROR(8u32);
pub const IRQL_NOT_GREATER_OR_EQUAL: BUGCHECK_ERROR = BUGCHECK_ERROR(9u32);
pub const IRQL_NOT_LESS_OR_EQUAL: BUGCHECK_ERROR = BUGCHECK_ERROR(10u32);
pub const NO_EXCEPTION_HANDLING_SUPPORT: BUGCHECK_ERROR = BUGCHECK_ERROR(11u32);
pub const MAXIMUM_WAIT_OBJECTS_EXCEEDED: BUGCHECK_ERROR = BUGCHECK_ERROR(12u32);
pub const MUTEX_LEVEL_NUMBER_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(13u32);
pub const NO_USER_MODE_CONTEXT: BUGCHECK_ERROR = BUGCHECK_ERROR(14u32);
pub const SPIN_LOCK_ALREADY_OWNED: BUGCHECK_ERROR = BUGCHECK_ERROR(15u32);
pub const SPIN_LOCK_NOT_OWNED: BUGCHECK_ERROR = BUGCHECK_ERROR(16u32);
pub const THREAD_NOT_MUTEX_OWNER: BUGCHECK_ERROR = BUGCHECK_ERROR(17u32);
pub const TRAP_CAUSE_UNKNOWN: BUGCHECK_ERROR = BUGCHECK_ERROR(18u32);
pub const EMPTY_THREAD_REAPER_LIST: BUGCHECK_ERROR = BUGCHECK_ERROR(19u32);
pub const CREATE_DELETE_LOCK_NOT_LOCKED: BUGCHECK_ERROR = BUGCHECK_ERROR(20u32);
pub const LAST_CHANCE_CALLED_FROM_KMODE: BUGCHECK_ERROR = BUGCHECK_ERROR(21u32);
pub const CID_HANDLE_CREATION: BUGCHECK_ERROR = BUGCHECK_ERROR(22u32);
pub const CID_HANDLE_DELETION: BUGCHECK_ERROR = BUGCHECK_ERROR(23u32);
pub const REFERENCE_BY_POINTER: BUGCHECK_ERROR = BUGCHECK_ERROR(24u32);
pub const BAD_POOL_HEADER: BUGCHECK_ERROR = BUGCHECK_ERROR(25u32);
pub const MEMORY_MANAGEMENT: BUGCHECK_ERROR = BUGCHECK_ERROR(26u32);
pub const PFN_SHARE_COUNT: BUGCHECK_ERROR = BUGCHECK_ERROR(27u32);
pub const PFN_REFERENCE_COUNT: BUGCHECK_ERROR = BUGCHECK_ERROR(28u32);
pub const NO_SPIN_LOCK_AVAILABLE: BUGCHECK_ERROR = BUGCHECK_ERROR(29u32);
pub const KMODE_EXCEPTION_NOT_HANDLED: BUGCHECK_ERROR = BUGCHECK_ERROR(30u32);
pub const SHARED_RESOURCE_CONV_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(31u32);
pub const KERNEL_APC_PENDING_DURING_EXIT: BUGCHECK_ERROR = BUGCHECK_ERROR(32u32);
pub const QUOTA_UNDERFLOW: BUGCHECK_ERROR = BUGCHECK_ERROR(33u32);
pub const FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(34u32);
pub const FAT_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(35u32);
pub const NTFS_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(36u32);
pub const NPFS_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(37u32);
pub const CDFS_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(38u32);
pub const RDR_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(39u32);
pub const CORRUPT_ACCESS_TOKEN: BUGCHECK_ERROR = BUGCHECK_ERROR(40u32);
pub const SECURITY_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(41u32);
pub const INCONSISTENT_IRP: BUGCHECK_ERROR = BUGCHECK_ERROR(42u32);
pub const PANIC_STACK_SWITCH: BUGCHECK_ERROR = BUGCHECK_ERROR(43u32);
pub const PORT_DRIVER_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(44u32);
pub const SCSI_DISK_DRIVER_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(45u32);
pub const DATA_BUS_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(46u32);
pub const INSTRUCTION_BUS_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(47u32);
pub const SET_OF_INVALID_CONTEXT: BUGCHECK_ERROR = BUGCHECK_ERROR(48u32);
pub const PHASE0_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(49u32);
pub const PHASE1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(50u32);
pub const UNEXPECTED_INITIALIZATION_CALL: BUGCHECK_ERROR = BUGCHECK_ERROR(51u32);
pub const CACHE_MANAGER: BUGCHECK_ERROR = BUGCHECK_ERROR(52u32);
pub const NO_MORE_IRP_STACK_LOCATIONS: BUGCHECK_ERROR = BUGCHECK_ERROR(53u32);
pub const DEVICE_REFERENCE_COUNT_NOT_ZERO: BUGCHECK_ERROR = BUGCHECK_ERROR(54u32);
pub const FLOPPY_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(55u32);
pub const SERIAL_DRIVER_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(56u32);
pub const SYSTEM_EXIT_OWNED_MUTEX: BUGCHECK_ERROR = BUGCHECK_ERROR(57u32);
pub const SYSTEM_UNWIND_PREVIOUS_USER: BUGCHECK_ERROR = BUGCHECK_ERROR(58u32);
pub const SYSTEM_SERVICE_EXCEPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(59u32);
pub const INTERRUPT_UNWIND_ATTEMPTED: BUGCHECK_ERROR = BUGCHECK_ERROR(60u32);
pub const INTERRUPT_EXCEPTION_NOT_HANDLED: BUGCHECK_ERROR = BUGCHECK_ERROR(61u32);
pub const MULTIPROCESSOR_CONFIGURATION_NOT_SUPPORTED: BUGCHECK_ERROR = BUGCHECK_ERROR(62u32);
pub const NO_MORE_SYSTEM_PTES: BUGCHECK_ERROR = BUGCHECK_ERROR(63u32);
pub const TARGET_MDL_TOO_SMALL: BUGCHECK_ERROR = BUGCHECK_ERROR(64u32);
pub const MUST_SUCCEED_POOL_EMPTY: BUGCHECK_ERROR = BUGCHECK_ERROR(65u32);
pub const ATDISK_DRIVER_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(66u32);
pub const NO_SUCH_PARTITION: BUGCHECK_ERROR = BUGCHECK_ERROR(67u32);
pub const MULTIPLE_IRP_COMPLETE_REQUESTS: BUGCHECK_ERROR = BUGCHECK_ERROR(68u32);
pub const INSUFFICIENT_SYSTEM_MAP_REGS: BUGCHECK_ERROR = BUGCHECK_ERROR(69u32);
pub const DEREF_UNKNOWN_LOGON_SESSION: BUGCHECK_ERROR = BUGCHECK_ERROR(70u32);
pub const REF_UNKNOWN_LOGON_SESSION: BUGCHECK_ERROR = BUGCHECK_ERROR(71u32);
pub const CANCEL_STATE_IN_COMPLETED_IRP: BUGCHECK_ERROR = BUGCHECK_ERROR(72u32);
pub const PAGE_FAULT_WITH_INTERRUPTS_OFF: BUGCHECK_ERROR = BUGCHECK_ERROR(73u32);
pub const IRQL_GT_ZERO_AT_SYSTEM_SERVICE: BUGCHECK_ERROR = BUGCHECK_ERROR(74u32);
pub const STREAMS_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(75u32);
pub const FATAL_UNHANDLED_HARD_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(76u32);
pub const NO_PAGES_AVAILABLE: BUGCHECK_ERROR = BUGCHECK_ERROR(77u32);
pub const PFN_LIST_CORRUPT: BUGCHECK_ERROR = BUGCHECK_ERROR(78u32);
pub const NDIS_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(79u32);
pub const PAGE_FAULT_IN_NONPAGED_AREA: BUGCHECK_ERROR = BUGCHECK_ERROR(80u32);
pub const PAGE_FAULT_IN_NONPAGED_AREA_M: BUGCHECK_ERROR = BUGCHECK_ERROR(268435536u32);
pub const REGISTRY_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(81u32);
pub const MAILSLOT_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(82u32);
pub const NO_BOOT_DEVICE: BUGCHECK_ERROR = BUGCHECK_ERROR(83u32);
pub const LM_SERVER_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(84u32);
pub const DATA_COHERENCY_EXCEPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(85u32);
pub const INSTRUCTION_COHERENCY_EXCEPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(86u32);
pub const XNS_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(87u32);
pub const VOLMGRX_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(88u32);
pub const PINBALL_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(89u32);
pub const CRITICAL_SERVICE_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(90u32);
pub const SET_ENV_VAR_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(91u32);
pub const HAL_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(92u32);
pub const UNSUPPORTED_PROCESSOR: BUGCHECK_ERROR = BUGCHECK_ERROR(93u32);
pub const OBJECT_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(94u32);
pub const SECURITY_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(95u32);
pub const PROCESS_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(96u32);
pub const HAL1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(97u32);
pub const OBJECT1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(98u32);
pub const SECURITY1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(99u32);
pub const SYMBOLIC_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(100u32);
pub const MEMORY1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(101u32);
pub const CACHE_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(102u32);
pub const CONFIG_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(103u32);
pub const FILE_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(104u32);
pub const IO1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(105u32);
pub const LPC_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(106u32);
pub const PROCESS1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(107u32);
pub const REFMON_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(108u32);
pub const SESSION1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(109u32);
pub const BOOTPROC_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(110u32);
pub const VSL_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(111u32);
pub const SOFT_RESTART_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(112u32);
pub const ASSIGN_DRIVE_LETTERS_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(114u32);
pub const CONFIG_LIST_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(115u32);
pub const BAD_SYSTEM_CONFIG_INFO: BUGCHECK_ERROR = BUGCHECK_ERROR(116u32);
pub const CANNOT_WRITE_CONFIGURATION: BUGCHECK_ERROR = BUGCHECK_ERROR(117u32);
pub const PROCESS_HAS_LOCKED_PAGES: BUGCHECK_ERROR = BUGCHECK_ERROR(118u32);
pub const KERNEL_STACK_INPAGE_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(119u32);
pub const PHASE0_EXCEPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(120u32);
pub const MISMATCHED_HAL: BUGCHECK_ERROR = BUGCHECK_ERROR(121u32);
pub const KERNEL_DATA_INPAGE_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(122u32);
pub const INACCESSIBLE_BOOT_DEVICE: BUGCHECK_ERROR = BUGCHECK_ERROR(123u32);
pub const BUGCODE_NDIS_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(124u32);
pub const INSTALL_MORE_MEMORY: BUGCHECK_ERROR = BUGCHECK_ERROR(125u32);
pub const SYSTEM_THREAD_EXCEPTION_NOT_HANDLED: BUGCHECK_ERROR = BUGCHECK_ERROR(126u32);
pub const SYSTEM_THREAD_EXCEPTION_NOT_HANDLED_M: BUGCHECK_ERROR = BUGCHECK_ERROR(268435582u32);
pub const UNEXPECTED_KERNEL_MODE_TRAP: BUGCHECK_ERROR = BUGCHECK_ERROR(127u32);
pub const UNEXPECTED_KERNEL_MODE_TRAP_M: BUGCHECK_ERROR = BUGCHECK_ERROR(268435583u32);
pub const NMI_HARDWARE_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(128u32);
pub const SPIN_LOCK_INIT_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(129u32);
pub const DFS_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(130u32);
pub const OFS_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(131u32);
pub const RECOM_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(132u32);
pub const SETUP_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(133u32);
pub const AUDIT_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(134u32);
pub const MBR_CHECKSUM_MISMATCH: BUGCHECK_ERROR = BUGCHECK_ERROR(139u32);
pub const KERNEL_MODE_EXCEPTION_NOT_HANDLED: BUGCHECK_ERROR = BUGCHECK_ERROR(142u32);
pub const KERNEL_MODE_EXCEPTION_NOT_HANDLED_M: BUGCHECK_ERROR = BUGCHECK_ERROR(268435598u32);
pub const PP0_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(143u32);
pub const PP1_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(144u32);
pub const WIN32K_INIT_OR_RIT_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(145u32);
pub const UP_DRIVER_ON_MP_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(146u32);
pub const INVALID_KERNEL_HANDLE: BUGCHECK_ERROR = BUGCHECK_ERROR(147u32);
pub const KERNEL_STACK_LOCKED_AT_EXIT: BUGCHECK_ERROR = BUGCHECK_ERROR(148u32);
pub const PNP_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(149u32);
pub const INVALID_WORK_QUEUE_ITEM: BUGCHECK_ERROR = BUGCHECK_ERROR(150u32);
pub const BOUND_IMAGE_UNSUPPORTED: BUGCHECK_ERROR = BUGCHECK_ERROR(151u32);
pub const END_OF_NT_EVALUATION_PERIOD: BUGCHECK_ERROR = BUGCHECK_ERROR(152u32);
pub const INVALID_REGION_OR_SEGMENT: BUGCHECK_ERROR = BUGCHECK_ERROR(153u32);
pub const SYSTEM_LICENSE_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(154u32);
pub const UDFS_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(155u32);
pub const MACHINE_CHECK_EXCEPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(156u32);
pub const USER_MODE_HEALTH_MONITOR: BUGCHECK_ERROR = BUGCHECK_ERROR(158u32);
pub const DRIVER_POWER_STATE_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(159u32);
pub const INTERNAL_POWER_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(160u32);
pub const PCI_BUS_DRIVER_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(161u32);
pub const MEMORY_IMAGE_CORRUPT: BUGCHECK_ERROR = BUGCHECK_ERROR(162u32);
pub const ACPI_DRIVER_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(163u32);
pub const CNSS_FILE_SYSTEM_FILTER: BUGCHECK_ERROR = BUGCHECK_ERROR(164u32);
pub const ACPI_BIOS_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(165u32);
pub const FP_EMULATION_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(166u32);
pub const BAD_EXHANDLE: BUGCHECK_ERROR = BUGCHECK_ERROR(167u32);
pub const BOOTING_IN_SAFEMODE_MINIMAL: BUGCHECK_ERROR = BUGCHECK_ERROR(168u32);
pub const BOOTING_IN_SAFEMODE_NETWORK: BUGCHECK_ERROR = BUGCHECK_ERROR(169u32);
pub const BOOTING_IN_SAFEMODE_DSREPAIR: BUGCHECK_ERROR = BUGCHECK_ERROR(170u32);
pub const SESSION_HAS_VALID_POOL_ON_EXIT: BUGCHECK_ERROR = BUGCHECK_ERROR(171u32);
pub const HAL_MEMORY_ALLOCATION: BUGCHECK_ERROR = BUGCHECK_ERROR(172u32);
pub const VIDEO_DRIVER_DEBUG_REPORT_REQUEST: BUGCHECK_ERROR = BUGCHECK_ERROR(1073741997u32);
pub const BGI_DETECTED_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(177u32);
pub const VIDEO_DRIVER_INIT_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(180u32);
pub const BOOTLOG_LOADED: BUGCHECK_ERROR = BUGCHECK_ERROR(181u32);
pub const BOOTLOG_NOT_LOADED: BUGCHECK_ERROR = BUGCHECK_ERROR(182u32);
pub const BOOTLOG_ENABLED: BUGCHECK_ERROR = BUGCHECK_ERROR(183u32);
pub const ATTEMPTED_SWITCH_FROM_DPC: BUGCHECK_ERROR = BUGCHECK_ERROR(184u32);
pub const CHIPSET_DETECTED_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(185u32);
pub const SESSION_HAS_VALID_VIEWS_ON_EXIT: BUGCHECK_ERROR = BUGCHECK_ERROR(186u32);
pub const NETWORK_BOOT_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(187u32);
pub const NETWORK_BOOT_DUPLICATE_ADDRESS: BUGCHECK_ERROR = BUGCHECK_ERROR(188u32);
pub const INVALID_HIBERNATED_STATE: BUGCHECK_ERROR = BUGCHECK_ERROR(189u32);
pub const ATTEMPTED_WRITE_TO_READONLY_MEMORY: BUGCHECK_ERROR = BUGCHECK_ERROR(190u32);
pub const MUTEX_ALREADY_OWNED: BUGCHECK_ERROR = BUGCHECK_ERROR(191u32);
pub const PCI_CONFIG_SPACE_ACCESS_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(192u32);
pub const SPECIAL_POOL_DETECTED_MEMORY_CORRUPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(193u32);
pub const BAD_POOL_CALLER: BUGCHECK_ERROR = BUGCHECK_ERROR(194u32);
pub const SYSTEM_IMAGE_BAD_SIGNATURE: BUGCHECK_ERROR = BUGCHECK_ERROR(195u32);
pub const DRIVER_VERIFIER_DETECTED_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(196u32);
pub const DRIVER_CORRUPTED_EXPOOL: BUGCHECK_ERROR = BUGCHECK_ERROR(197u32);
pub const DRIVER_CAUGHT_MODIFYING_FREED_POOL: BUGCHECK_ERROR = BUGCHECK_ERROR(198u32);
pub const TIMER_OR_DPC_INVALID: BUGCHECK_ERROR = BUGCHECK_ERROR(199u32);
pub const IRQL_UNEXPECTED_VALUE: BUGCHECK_ERROR = BUGCHECK_ERROR(200u32);
pub const DRIVER_VERIFIER_IOMANAGER_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(201u32);
pub const PNP_DETECTED_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(202u32);
pub const DRIVER_LEFT_LOCKED_PAGES_IN_PROCESS: BUGCHECK_ERROR = BUGCHECK_ERROR(203u32);
pub const PAGE_FAULT_IN_FREED_SPECIAL_POOL: BUGCHECK_ERROR = BUGCHECK_ERROR(204u32);
pub const PAGE_FAULT_BEYOND_END_OF_ALLOCATION: BUGCHECK_ERROR = BUGCHECK_ERROR(205u32);
pub const DRIVER_UNLOADED_WITHOUT_CANCELLING_PENDING_OPERATIONS: BUGCHECK_ERROR = BUGCHECK_ERROR(206u32);
pub const TERMINAL_SERVER_DRIVER_MADE_INCORRECT_MEMORY_REFERENCE: BUGCHECK_ERROR = BUGCHECK_ERROR(207u32);
pub const DRIVER_CORRUPTED_MMPOOL: BUGCHECK_ERROR = BUGCHECK_ERROR(208u32);
pub const DRIVER_IRQL_NOT_LESS_OR_EQUAL: BUGCHECK_ERROR = BUGCHECK_ERROR(209u32);
pub const BUGCODE_ID_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(210u32);
pub const DRIVER_PORTION_MUST_BE_NONPAGED: BUGCHECK_ERROR = BUGCHECK_ERROR(211u32);
pub const SYSTEM_SCAN_AT_RAISED_IRQL_CAUGHT_IMPROPER_DRIVER_UNLOAD: BUGCHECK_ERROR = BUGCHECK_ERROR(212u32);
pub const DRIVER_PAGE_FAULT_IN_FREED_SPECIAL_POOL: BUGCHECK_ERROR = BUGCHECK_ERROR(213u32);
pub const DRIVER_PAGE_FAULT_BEYOND_END_OF_ALLOCATION: BUGCHECK_ERROR = BUGCHECK_ERROR(214u32);
pub const DRIVER_PAGE_FAULT_BEYOND_END_OF_ALLOCATION_M: BUGCHECK_ERROR = BUGCHECK_ERROR(268435670u32);
pub const DRIVER_UNMAPPING_INVALID_VIEW: BUGCHECK_ERROR = BUGCHECK_ERROR(215u32);
pub const DRIVER_USED_EXCESSIVE_PTES: BUGCHECK_ERROR = BUGCHECK_ERROR(216u32);
pub const LOCKED_PAGES_TRACKER_CORRUPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(217u32);
pub const SYSTEM_PTE_MISUSE: BUGCHECK_ERROR = BUGCHECK_ERROR(218u32);
pub const DRIVER_CORRUPTED_SYSPTES: BUGCHECK_ERROR = BUGCHECK_ERROR(219u32);
pub const DRIVER_INVALID_STACK_ACCESS: BUGCHECK_ERROR = BUGCHECK_ERROR(220u32);
pub const POOL_CORRUPTION_IN_FILE_AREA: BUGCHECK_ERROR = BUGCHECK_ERROR(222u32);
pub const IMPERSONATING_WORKER_THREAD: BUGCHECK_ERROR = BUGCHECK_ERROR(223u32);
pub const ACPI_BIOS_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(224u32);
pub const WORKER_THREAD_RETURNED_AT_BAD_IRQL: BUGCHECK_ERROR = BUGCHECK_ERROR(225u32);
pub const MANUALLY_INITIATED_CRASH: BUGCHECK_ERROR = BUGCHECK_ERROR(226u32);
pub const RESOURCE_NOT_OWNED: BUGCHECK_ERROR = BUGCHECK_ERROR(227u32);
pub const WORKER_INVALID: BUGCHECK_ERROR = BUGCHECK_ERROR(228u32);
pub const POWER_FAILURE_SIMULATE: BUGCHECK_ERROR = BUGCHECK_ERROR(229u32);
pub const DRIVER_VERIFIER_DMA_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(230u32);
pub const INVALID_FLOATING_POINT_STATE: BUGCHECK_ERROR = BUGCHECK_ERROR(231u32);
pub const INVALID_CANCEL_OF_FILE_OPEN: BUGCHECK_ERROR = BUGCHECK_ERROR(232u32);
pub const ACTIVE_EX_WORKER_THREAD_TERMINATION: BUGCHECK_ERROR = BUGCHECK_ERROR(233u32);
pub const SAVER_UNSPECIFIED: BUGCHECK_ERROR = BUGCHECK_ERROR(61440u32);
pub const SAVER_BLANKSCREEN: BUGCHECK_ERROR = BUGCHECK_ERROR(61442u32);
pub const SAVER_INPUT: BUGCHECK_ERROR = BUGCHECK_ERROR(61443u32);
pub const SAVER_WATCHDOG: BUGCHECK_ERROR = BUGCHECK_ERROR(61444u32);
pub const SAVER_STARTNOTVISIBLE: BUGCHECK_ERROR = BUGCHECK_ERROR(61445u32);
pub const SAVER_NAVIGATIONMODEL: BUGCHECK_ERROR = BUGCHECK_ERROR(61446u32);
pub const SAVER_OUTOFMEMORY: BUGCHECK_ERROR = BUGCHECK_ERROR(61447u32);
pub const SAVER_GRAPHICS: BUGCHECK_ERROR = BUGCHECK_ERROR(61448u32);
pub const SAVER_NAVSERVERTIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(61449u32);
pub const SAVER_CHROMEPROCESSCRASH: BUGCHECK_ERROR = BUGCHECK_ERROR(61450u32);
pub const SAVER_NOTIFICATIONDISMISSAL: BUGCHECK_ERROR = BUGCHECK_ERROR(61451u32);
pub const SAVER_SPEECHDISMISSAL: BUGCHECK_ERROR = BUGCHECK_ERROR(61452u32);
pub const SAVER_CALLDISMISSAL: BUGCHECK_ERROR = BUGCHECK_ERROR(61453u32);
pub const SAVER_APPBARDISMISSAL: BUGCHECK_ERROR = BUGCHECK_ERROR(61454u32);
pub const SAVER_RILADAPTATIONCRASH: BUGCHECK_ERROR = BUGCHECK_ERROR(61455u32);
pub const SAVER_APPLISTUNREACHABLE: BUGCHECK_ERROR = BUGCHECK_ERROR(61456u32);
pub const SAVER_REPORTNOTIFICATIONFAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(61457u32);
pub const SAVER_UNEXPECTEDSHUTDOWN: BUGCHECK_ERROR = BUGCHECK_ERROR(61458u32);
pub const SAVER_RPCFAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(61459u32);
pub const SAVER_AUXILIARYFULLDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(61460u32);
pub const SAVER_ACCOUNTPROVSVCINITFAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(61461u32);
pub const SAVER_MTBFCOMMANDTIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(789u32);
pub const SAVER_MTBFCOMMANDHANG: BUGCHECK_ERROR = BUGCHECK_ERROR(61697u32);
pub const SAVER_MTBFPASSBUGCHECK: BUGCHECK_ERROR = BUGCHECK_ERROR(61698u32);
pub const SAVER_MTBFIOERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(61699u32);
pub const SAVER_RENDERTHREADHANG: BUGCHECK_ERROR = BUGCHECK_ERROR(61952u32);
pub const SAVER_RENDERMOBILEUIOOM: BUGCHECK_ERROR = BUGCHECK_ERROR(61953u32);
pub const SAVER_DEVICEUPDATEUNSPECIFIED: BUGCHECK_ERROR = BUGCHECK_ERROR(62208u32);
pub const SAVER_AUDIODRIVERHANG: BUGCHECK_ERROR = BUGCHECK_ERROR(62464u32);
pub const SAVER_BATTERYPULLOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(62720u32);
pub const SAVER_MEDIACORETESTHANG: BUGCHECK_ERROR = BUGCHECK_ERROR(62976u32);
pub const SAVER_RESOURCEMANAGEMENT: BUGCHECK_ERROR = BUGCHECK_ERROR(63232u32);
pub const SAVER_CAPTURESERVICE: BUGCHECK_ERROR = BUGCHECK_ERROR(63488u32);
pub const SAVER_WAITFORSHELLREADY: BUGCHECK_ERROR = BUGCHECK_ERROR(63744u32);
pub const SAVER_NONRESPONSIVEPROCESS: BUGCHECK_ERROR = BUGCHECK_ERROR(404u32);
pub const SAVER_SICKAPPLICATION: BUGCHECK_ERROR = BUGCHECK_ERROR(34918u32);
pub const THREAD_STUCK_IN_DEVICE_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(234u32);
pub const THREAD_STUCK_IN_DEVICE_DRIVER_M: BUGCHECK_ERROR = BUGCHECK_ERROR(268435690u32);
pub const DIRTY_MAPPED_PAGES_CONGESTION: BUGCHECK_ERROR = BUGCHECK_ERROR(235u32);
pub const SESSION_HAS_VALID_SPECIAL_POOL_ON_EXIT: BUGCHECK_ERROR = BUGCHECK_ERROR(236u32);
pub const UNMOUNTABLE_BOOT_VOLUME: BUGCHECK_ERROR = BUGCHECK_ERROR(237u32);
pub const CRITICAL_PROCESS_DIED: BUGCHECK_ERROR = BUGCHECK_ERROR(239u32);
pub const STORAGE_MINIPORT_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(240u32);
pub const SCSI_VERIFIER_DETECTED_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(241u32);
pub const HARDWARE_INTERRUPT_STORM: BUGCHECK_ERROR = BUGCHECK_ERROR(242u32);
pub const DISORDERLY_SHUTDOWN: BUGCHECK_ERROR = BUGCHECK_ERROR(243u32);
pub const CRITICAL_OBJECT_TERMINATION: BUGCHECK_ERROR = BUGCHECK_ERROR(244u32);
pub const FLTMGR_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(245u32);
pub const PCI_VERIFIER_DETECTED_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(246u32);
pub const DRIVER_OVERRAN_STACK_BUFFER: BUGCHECK_ERROR = BUGCHECK_ERROR(247u32);
pub const RAMDISK_BOOT_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(248u32);
pub const DRIVER_RETURNED_STATUS_REPARSE_FOR_VOLUME_OPEN: BUGCHECK_ERROR = BUGCHECK_ERROR(249u32);
pub const HTTP_DRIVER_CORRUPTED: BUGCHECK_ERROR = BUGCHECK_ERROR(250u32);
pub const RECURSIVE_MACHINE_CHECK: BUGCHECK_ERROR = BUGCHECK_ERROR(251u32);
pub const ATTEMPTED_EXECUTE_OF_NOEXECUTE_MEMORY: BUGCHECK_ERROR = BUGCHECK_ERROR(252u32);
pub const DIRTY_NOWRITE_PAGES_CONGESTION: BUGCHECK_ERROR = BUGCHECK_ERROR(253u32);
pub const BUGCODE_USB_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(254u32);
pub const BC_BLUETOOTH_VERIFIER_FAULT: BUGCHECK_ERROR = BUGCHECK_ERROR(3070u32);
pub const BC_BTHMINI_VERIFIER_FAULT: BUGCHECK_ERROR = BUGCHECK_ERROR(3071u32);
pub const RESERVE_QUEUE_OVERFLOW: BUGCHECK_ERROR = BUGCHECK_ERROR(255u32);
pub const LOADER_BLOCK_MISMATCH: BUGCHECK_ERROR = BUGCHECK_ERROR(256u32);
pub const CLOCK_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(257u32);
pub const DPC_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(258u32);
pub const MUP_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(259u32);
pub const AGP_INVALID_ACCESS: BUGCHECK_ERROR = BUGCHECK_ERROR(260u32);
pub const AGP_GART_CORRUPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(261u32);
pub const AGP_ILLEGALLY_REPROGRAMMED: BUGCHECK_ERROR = BUGCHECK_ERROR(262u32);
pub const KERNEL_EXPAND_STACK_ACTIVE: BUGCHECK_ERROR = BUGCHECK_ERROR(263u32);
pub const THIRD_PARTY_FILE_SYSTEM_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(264u32);
pub const CRITICAL_STRUCTURE_CORRUPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(265u32);
pub const APP_TAGGING_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(266u32);
pub const DFSC_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(267u32);
pub const FSRTL_EXTRA_CREATE_PARAMETER_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(268u32);
pub const WDF_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(269u32);
pub const VIDEO_MEMORY_MANAGEMENT_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(270u32);
pub const DRIVER_INVALID_CRUNTIME_PARAMETER: BUGCHECK_ERROR = BUGCHECK_ERROR(272u32);
pub const RECURSIVE_NMI: BUGCHECK_ERROR = BUGCHECK_ERROR(273u32);
pub const MSRPC_STATE_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(274u32);
pub const VIDEO_DXGKRNL_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(275u32);
pub const VIDEO_SHADOW_DRIVER_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(276u32);
pub const AGP_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(277u32);
pub const VIDEO_TDR_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(278u32);
pub const VIDEO_TDR_TIMEOUT_DETECTED: BUGCHECK_ERROR = BUGCHECK_ERROR(279u32);
pub const NTHV_GUEST_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(280u32);
pub const VIDEO_SCHEDULER_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(281u32);
pub const EM_INITIALIZATION_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(282u32);
pub const DRIVER_RETURNED_HOLDING_CANCEL_LOCK: BUGCHECK_ERROR = BUGCHECK_ERROR(283u32);
pub const ATTEMPTED_WRITE_TO_CM_PROTECTED_STORAGE: BUGCHECK_ERROR = BUGCHECK_ERROR(284u32);
pub const EVENT_TRACING_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(285u32);
pub const TOO_MANY_RECURSIVE_FAULTS: BUGCHECK_ERROR = BUGCHECK_ERROR(286u32);
pub const INVALID_DRIVER_HANDLE: BUGCHECK_ERROR = BUGCHECK_ERROR(287u32);
pub const BITLOCKER_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(288u32);
pub const DRIVER_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(289u32);
pub const WHEA_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(290u32);
pub const CRYPTO_SELF_TEST_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(291u32);
pub const WHEA_UNCORRECTABLE_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(292u32);
pub const NMR_INVALID_STATE: BUGCHECK_ERROR = BUGCHECK_ERROR(293u32);
pub const NETIO_INVALID_POOL_CALLER: BUGCHECK_ERROR = BUGCHECK_ERROR(294u32);
pub const PAGE_NOT_ZERO: BUGCHECK_ERROR = BUGCHECK_ERROR(295u32);
pub const WORKER_THREAD_RETURNED_WITH_BAD_IO_PRIORITY: BUGCHECK_ERROR = BUGCHECK_ERROR(296u32);
pub const WORKER_THREAD_RETURNED_WITH_BAD_PAGING_IO_PRIORITY: BUGCHECK_ERROR = BUGCHECK_ERROR(297u32);
pub const MUI_NO_VALID_SYSTEM_LANGUAGE: BUGCHECK_ERROR = BUGCHECK_ERROR(298u32);
pub const FAULTY_HARDWARE_CORRUPTED_PAGE: BUGCHECK_ERROR = BUGCHECK_ERROR(299u32);
pub const EXFAT_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(300u32);
pub const VOLSNAP_OVERLAPPED_TABLE_ACCESS: BUGCHECK_ERROR = BUGCHECK_ERROR(301u32);
pub const INVALID_MDL_RANGE: BUGCHECK_ERROR = BUGCHECK_ERROR(302u32);
pub const VHD_BOOT_INITIALIZATION_FAILED: BUGCHECK_ERROR = BUGCHECK_ERROR(303u32);
pub const DYNAMIC_ADD_PROCESSOR_MISMATCH: BUGCHECK_ERROR = BUGCHECK_ERROR(304u32);
pub const INVALID_EXTENDED_PROCESSOR_STATE: BUGCHECK_ERROR = BUGCHECK_ERROR(305u32);
pub const RESOURCE_OWNER_POINTER_INVALID: BUGCHECK_ERROR = BUGCHECK_ERROR(306u32);
pub const DPC_WATCHDOG_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(307u32);
pub const DRIVE_EXTENDER: BUGCHECK_ERROR = BUGCHECK_ERROR(308u32);
pub const REGISTRY_FILTER_DRIVER_EXCEPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(309u32);
pub const VHD_BOOT_HOST_VOLUME_NOT_ENOUGH_SPACE: BUGCHECK_ERROR = BUGCHECK_ERROR(310u32);
pub const WIN32K_HANDLE_MANAGER: BUGCHECK_ERROR = BUGCHECK_ERROR(311u32);
pub const GPIO_CONTROLLER_DRIVER_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(312u32);
pub const KERNEL_SECURITY_CHECK_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(313u32);
pub const KERNEL_MODE_HEAP_CORRUPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(314u32);
pub const PASSIVE_INTERRUPT_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(315u32);
pub const INVALID_IO_BOOST_STATE: BUGCHECK_ERROR = BUGCHECK_ERROR(316u32);
pub const CRITICAL_INITIALIZATION_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(317u32);
pub const ERRATA_WORKAROUND_UNSUCCESSFUL: BUGCHECK_ERROR = BUGCHECK_ERROR(318u32);
pub const REGISTRY_CALLBACK_DRIVER_EXCEPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(319u32);
pub const STORAGE_DEVICE_ABNORMALITY_DETECTED: BUGCHECK_ERROR = BUGCHECK_ERROR(320u32);
pub const VIDEO_ENGINE_TIMEOUT_DETECTED: BUGCHECK_ERROR = BUGCHECK_ERROR(321u32);
pub const VIDEO_TDR_APPLICATION_BLOCKED: BUGCHECK_ERROR = BUGCHECK_ERROR(322u32);
pub const PROCESSOR_DRIVER_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(323u32);
pub const BUGCODE_USB3_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(324u32);
pub const SECURE_BOOT_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(325u32);
pub const NDIS_NET_BUFFER_LIST_INFO_ILLEGALLY_TRANSFERRED: BUGCHECK_ERROR = BUGCHECK_ERROR(326u32);
pub const ABNORMAL_RESET_DETECTED: BUGCHECK_ERROR = BUGCHECK_ERROR(327u32);
pub const IO_OBJECT_INVALID: BUGCHECK_ERROR = BUGCHECK_ERROR(328u32);
pub const REFS_FILE_SYSTEM: BUGCHECK_ERROR = BUGCHECK_ERROR(329u32);
pub const KERNEL_WMI_INTERNAL: BUGCHECK_ERROR = BUGCHECK_ERROR(330u32);
pub const SOC_SUBSYSTEM_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(331u32);
pub const FATAL_ABNORMAL_RESET_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(332u32);
pub const EXCEPTION_SCOPE_INVALID: BUGCHECK_ERROR = BUGCHECK_ERROR(333u32);
pub const SOC_CRITICAL_DEVICE_REMOVED: BUGCHECK_ERROR = BUGCHECK_ERROR(334u32);
pub const PDC_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(335u32);
pub const TCPIP_AOAC_NIC_ACTIVE_REFERENCE_LEAK: BUGCHECK_ERROR = BUGCHECK_ERROR(336u32);
pub const UNSUPPORTED_INSTRUCTION_MODE: BUGCHECK_ERROR = BUGCHECK_ERROR(337u32);
pub const INVALID_PUSH_LOCK_FLAGS: BUGCHECK_ERROR = BUGCHECK_ERROR(338u32);
pub const KERNEL_LOCK_ENTRY_LEAKED_ON_THREAD_TERMINATION: BUGCHECK_ERROR = BUGCHECK_ERROR(339u32);
pub const UNEXPECTED_STORE_EXCEPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(340u32);
pub const OS_DATA_TAMPERING: BUGCHECK_ERROR = BUGCHECK_ERROR(341u32);
pub const WINSOCK_DETECTED_HUNG_CLOSESOCKET_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(342u32);
pub const KERNEL_THREAD_PRIORITY_FLOOR_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(343u32);
pub const ILLEGAL_IOMMU_PAGE_FAULT: BUGCHECK_ERROR = BUGCHECK_ERROR(344u32);
pub const HAL_ILLEGAL_IOMMU_PAGE_FAULT: BUGCHECK_ERROR = BUGCHECK_ERROR(345u32);
pub const SDBUS_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(346u32);
pub const WORKER_THREAD_RETURNED_WITH_SYSTEM_PAGE_PRIORITY_ACTIVE: BUGCHECK_ERROR = BUGCHECK_ERROR(347u32);
pub const PDC_WATCHDOG_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(348u32);
pub const SOC_SUBSYSTEM_FAILURE_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(349u32);
pub const BUGCODE_NDIS_DRIVER_LIVE_DUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(350u32);
pub const CONNECTED_STANDBY_WATCHDOG_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(351u32);
pub const WIN32K_ATOMIC_CHECK_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(352u32);
pub const LIVE_SYSTEM_DUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(353u32);
pub const KERNEL_AUTO_BOOST_INVALID_LOCK_RELEASE: BUGCHECK_ERROR = BUGCHECK_ERROR(354u32);
pub const WORKER_THREAD_TEST_CONDITION: BUGCHECK_ERROR = BUGCHECK_ERROR(355u32);
pub const WIN32K_CRITICAL_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(356u32);
pub const CLUSTER_CSV_STATUS_IO_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(357u32);
pub const CLUSTER_RESOURCE_CALL_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(358u32);
pub const CLUSTER_CSV_SNAPSHOT_DEVICE_INFO_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(359u32);
pub const CLUSTER_CSV_STATE_TRANSITION_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(360u32);
pub const CLUSTER_CSV_VOLUME_ARRIVAL_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(361u32);
pub const CLUSTER_CSV_VOLUME_REMOVAL_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(362u32);
pub const CLUSTER_CSV_CLUSTER_WATCHDOG_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(363u32);
pub const INVALID_RUNDOWN_PROTECTION_FLAGS: BUGCHECK_ERROR = BUGCHECK_ERROR(364u32);
pub const INVALID_SLOT_ALLOCATOR_FLAGS: BUGCHECK_ERROR = BUGCHECK_ERROR(365u32);
pub const ERESOURCE_INVALID_RELEASE: BUGCHECK_ERROR = BUGCHECK_ERROR(366u32);
pub const CLUSTER_CSV_STATE_TRANSITION_INTERVAL_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(367u32);
pub const CLUSTER_CSV_CLUSSVC_DISCONNECT_WATCHDOG: BUGCHECK_ERROR = BUGCHECK_ERROR(368u32);
pub const CRYPTO_LIBRARY_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(369u32);
pub const COREMSGCALL_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(371u32);
pub const COREMSG_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(372u32);
pub const PREVIOUS_FATAL_ABNORMAL_RESET_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(373u32);
pub const ELAM_DRIVER_DETECTED_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(376u32);
pub const CLUSTER_CLUSPORT_STATUS_IO_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(377u32);
pub const PROFILER_CONFIGURATION_ILLEGAL: BUGCHECK_ERROR = BUGCHECK_ERROR(379u32);
pub const PDC_LOCK_WATCHDOG_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(380u32);
pub const PDC_UNEXPECTED_REVOCATION_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(381u32);
pub const MICROCODE_REVISION_MISMATCH: BUGCHECK_ERROR = BUGCHECK_ERROR(382u32);
pub const HYPERGUARD_INITIALIZATION_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(383u32);
pub const WVR_LIVEDUMP_REPLICATION_IOCONTEXT_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(384u32);
pub const WVR_LIVEDUMP_STATE_TRANSITION_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(385u32);
pub const WVR_LIVEDUMP_RECOVERY_IOCONTEXT_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(386u32);
pub const WVR_LIVEDUMP_APP_IO_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(387u32);
pub const WVR_LIVEDUMP_MANUALLY_INITIATED: BUGCHECK_ERROR = BUGCHECK_ERROR(388u32);
pub const WVR_LIVEDUMP_STATE_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(389u32);
pub const WVR_LIVEDUMP_CRITICAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(390u32);
pub const VIDEO_DWMINIT_TIMEOUT_FALLBACK_BDD: BUGCHECK_ERROR = BUGCHECK_ERROR(391u32);
pub const CLUSTER_CSVFS_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(392u32);
pub const BAD_OBJECT_HEADER: BUGCHECK_ERROR = BUGCHECK_ERROR(393u32);
pub const SILO_CORRUPT: BUGCHECK_ERROR = BUGCHECK_ERROR(394u32);
pub const SECURE_KERNEL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(395u32);
pub const HYPERGUARD_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(396u32);
pub const SECURE_FAULT_UNHANDLED: BUGCHECK_ERROR = BUGCHECK_ERROR(397u32);
pub const KERNEL_PARTITION_REFERENCE_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(398u32);
pub const SYNTHETIC_EXCEPTION_UNHANDLED: BUGCHECK_ERROR = BUGCHECK_ERROR(399u32);
pub const WIN32K_CRITICAL_FAILURE_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(400u32);
pub const PF_DETECTED_CORRUPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(401u32);
pub const KERNEL_AUTO_BOOST_LOCK_ACQUISITION_WITH_RAISED_IRQL: BUGCHECK_ERROR = BUGCHECK_ERROR(402u32);
pub const VIDEO_DXGKRNL_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(403u32);
pub const KERNEL_STORAGE_SLOT_IN_USE: BUGCHECK_ERROR = BUGCHECK_ERROR(409u32);
pub const SMB_SERVER_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(405u32);
pub const LOADER_ROLLBACK_DETECTED: BUGCHECK_ERROR = BUGCHECK_ERROR(406u32);
pub const WIN32K_SECURITY_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(407u32);
pub const UFX_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(408u32);
pub const WORKER_THREAD_RETURNED_WHILE_ATTACHED_TO_SILO: BUGCHECK_ERROR = BUGCHECK_ERROR(410u32);
pub const TTM_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(411u32);
pub const WIN32K_POWER_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(412u32);
pub const CLUSTER_SVHDX_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(413u32);
pub const BUGCODE_NETADAPTER_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(414u32);
pub const PDC_PRIVILEGE_CHECK_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(415u32);
pub const TTM_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(416u32);
pub const WIN32K_CALLOUT_WATCHDOG_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(417u32);
pub const WIN32K_CALLOUT_WATCHDOG_BUGCHECK: BUGCHECK_ERROR = BUGCHECK_ERROR(418u32);
pub const CALL_HAS_NOT_RETURNED_WATCHDOG_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(419u32);
pub const DRIPS_SW_HW_DIVERGENCE_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(420u32);
pub const USB_DRIPS_BLOCKER_SURPRISE_REMOVAL_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(421u32);
pub const BLUETOOTH_ERROR_RECOVERY_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(422u32);
pub const SMB_REDIRECTOR_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(423u32);
pub const VIDEO_DXGKRNL_BLACK_SCREEN_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(424u32);
pub const DIRECTED_FX_TRANSITION_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(425u32);
pub const EXCEPTION_ON_INVALID_STACK: BUGCHECK_ERROR = BUGCHECK_ERROR(426u32);
pub const UNWIND_ON_INVALID_STACK: BUGCHECK_ERROR = BUGCHECK_ERROR(427u32);
pub const VIDEO_MINIPORT_FAILED_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(432u32);
pub const VIDEO_MINIPORT_BLACK_SCREEN_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(440u32);
pub const DRIVER_VERIFIER_DETECTED_VIOLATION_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(452u32);
pub const IO_THREADPOOL_DEADLOCK_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(453u32);
pub const FAST_ERESOURCE_PRECONDITION_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(454u32);
pub const STORE_DATA_STRUCTURE_CORRUPTION: BUGCHECK_ERROR = BUGCHECK_ERROR(455u32);
pub const MANUALLY_INITIATED_POWER_BUTTON_HOLD: BUGCHECK_ERROR = BUGCHECK_ERROR(456u32);
pub const USER_MODE_HEALTH_MONITOR_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(457u32);
pub const SYNTHETIC_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(458u32);
pub const INVALID_SILO_DETACH: BUGCHECK_ERROR = BUGCHECK_ERROR(459u32);
pub const EXRESOURCE_TIMEOUT_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(460u32);
pub const INVALID_CALLBACK_STACK_ADDRESS: BUGCHECK_ERROR = BUGCHECK_ERROR(461u32);
pub const INVALID_KERNEL_STACK_ADDRESS: BUGCHECK_ERROR = BUGCHECK_ERROR(462u32);
pub const HARDWARE_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(463u32);
pub const ACPI_FIRMWARE_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(464u32);
pub const TELEMETRY_ASSERTS_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(465u32);
pub const WORKER_THREAD_INVALID_STATE: BUGCHECK_ERROR = BUGCHECK_ERROR(466u32);
pub const WFP_INVALID_OPERATION: BUGCHECK_ERROR = BUGCHECK_ERROR(467u32);
pub const UCMUCSI_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(468u32);
pub const DRIVER_PNP_WATCHDOG: BUGCHECK_ERROR = BUGCHECK_ERROR(469u32);
pub const WORKER_THREAD_RETURNED_WITH_NON_DEFAULT_WORKLOAD_CLASS: BUGCHECK_ERROR = BUGCHECK_ERROR(470u32);
pub const EFS_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(471u32);
pub const UCMUCSI_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(472u32);
pub const HAL_IOMMU_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(473u32);
pub const HAL_BLOCKED_PROCESSOR_INTERNAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(474u32);
pub const IPI_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(475u32);
pub const DMA_COMMON_BUFFER_VECTOR_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(476u32);
pub const BUGCODE_MBBADAPTER_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(477u32);
pub const BUGCODE_WIFIADAPTER_DRIVER: BUGCHECK_ERROR = BUGCHECK_ERROR(478u32);
pub const PROCESSOR_START_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(479u32);
pub const INVALID_ALTERNATE_SYSTEM_CALL_HANDLER_REGISTRATION: BUGCHECK_ERROR = BUGCHECK_ERROR(480u32);
pub const DEVICE_DIAGNOSTIC_LOG_LIVEDUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(481u32);
pub const AZURE_DEVICE_FW_DUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(482u32);
pub const BREAKAWAY_CABLE_TRANSITION: BUGCHECK_ERROR = BUGCHECK_ERROR(483u32);
pub const VIDEO_DXGKRNL_SYSMM_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(484u32);
pub const DRIVER_VERIFIER_TRACKING_LIVE_DUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(485u32);
pub const CRASHDUMP_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(486u32);
pub const REGISTRY_LIVE_DUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(487u32);
pub const INVALID_THREAD_AFFINITY_STATE: BUGCHECK_ERROR = BUGCHECK_ERROR(488u32);
pub const ILLEGAL_ATS_INITIALIZATION: BUGCHECK_ERROR = BUGCHECK_ERROR(489u32);
pub const SECURE_PCI_CONFIG_SPACE_ACCESS_VIOLATION: BUGCHECK_ERROR = BUGCHECK_ERROR(490u32);
pub const DAM_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(491u32);
pub const XBOX_VMCTRL_CS_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(854u32);
pub const XBOX_CORRUPTED_IMAGE: BUGCHECK_ERROR = BUGCHECK_ERROR(855u32);
pub const XBOX_INVERTED_FUNCTION_TABLE_OVERFLOW: BUGCHECK_ERROR = BUGCHECK_ERROR(856u32);
pub const XBOX_CORRUPTED_IMAGE_BASE: BUGCHECK_ERROR = BUGCHECK_ERROR(857u32);
pub const XBOX_XDS_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(858u32);
pub const XBOX_SHUTDOWN_WATCHDOG_TIMEOUT: BUGCHECK_ERROR = BUGCHECK_ERROR(859u32);
pub const XBOX_360_SYSTEM_CRASH: BUGCHECK_ERROR = BUGCHECK_ERROR(864u32);
pub const XBOX_360_SYSTEM_CRASH_RESERVED: BUGCHECK_ERROR = BUGCHECK_ERROR(1056u32);
pub const XBOX_SECURITY_FAILUE: BUGCHECK_ERROR = BUGCHECK_ERROR(1057u32);
pub const KERNEL_CFG_INIT_FAILURE: BUGCHECK_ERROR = BUGCHECK_ERROR(1058u32);
pub const MANUALLY_INITIATED_POWER_BUTTON_HOLD_LIVE_DUMP: BUGCHECK_ERROR = BUGCHECK_ERROR(4552u32);
pub const HYPERVISOR_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(131073u32);
pub const WINLOGON_FATAL_ERROR: BUGCHECK_ERROR = BUGCHECK_ERROR(3221226010u32);
pub const MANUALLY_INITIATED_CRASH1: BUGCHECK_ERROR = BUGCHECK_ERROR(3735936685u32);
pub const BUGCHECK_CONTEXT_MODIFIER: BUGCHECK_ERROR = BUGCHECK_ERROR(2147483648u32);
pub const CANNOT_ALLOCATE_MEMORY: u32 = 9u32;
pub const CATID_ActiveScript: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4038566305, data2: 38983, data3: 4559, data4: [143, 32, 0, 128, 95, 44, 208, 100] };
pub const CATID_ActiveScriptAuthor: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 183380626, data2: 48315, data3: 4560, data4: [140, 114, 0, 192, 79, 194, 176, 133] };
pub const CATID_ActiveScriptEncode: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4038566307, data2: 38983, data3: 4559, data4: [143, 32, 0, 128, 95, 44, 208, 100] };
pub const CATID_ActiveScriptParse: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4038566306, data2: 38983, data3: 4559, data4: [143, 32, 0, 128, 95, 44, 208, 100] };
pub const CBA_CHECK_ARM_MACHINE_THUMB_TYPE_OVERRIDE: u32 = 2147483648u32;
pub const CBA_CHECK_ENGOPT_DISALLOW_NETWORK_PATHS: u32 = 1879048192u32;
pub const CBA_DEBUG_INFO: u32 = 268435456u32;
pub const CBA_DEFERRED_SYMBOL_LOAD_CANCEL: u32 = 7u32;
pub const CBA_DEFERRED_SYMBOL_LOAD_COMPLETE: u32 = 2u32;
pub const CBA_DEFERRED_SYMBOL_LOAD_FAILURE: u32 = 3u32;
pub const CBA_DEFERRED_SYMBOL_LOAD_PARTIAL: u32 = 32u32;
pub const CBA_DEFERRED_SYMBOL_LOAD_START: u32 = 1u32;
pub const CBA_DUPLICATE_SYMBOL: u32 = 5u32;
pub const CBA_ENGINE_PRESENT: u32 = 1610612736u32;
pub const CBA_EVENT: u32 = 16u32;
pub const CBA_MAP_JIT_SYMBOL: u32 = 2684354560u32;
pub const CBA_READ_MEMORY: u32 = 6u32;
pub const CBA_SET_OPTIONS: u32 = 8u32;
pub const CBA_SRCSRV_EVENT: u32 = 1073741824u32;
pub const CBA_SRCSRV_INFO: u32 = 536870912u32;
pub const CBA_SYMBOLS_UNLOADED: u32 = 4u32;
pub const CBA_UPDATE_STATUS_BAR: u32 = 1342177280u32;
pub const CBA_XML_LOG: u32 = 2415919104u32;
pub const CDebugDocumentHelper: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2209922214, data2: 26748, data3: 4560, data4: [164, 5, 0, 170, 0, 96, 39, 92] };
pub const CERT_PE_IMAGE_DIGEST_ALL_IMPORT_INFO: u32 = 4u32;
pub const CERT_PE_IMAGE_DIGEST_DEBUG_INFO: u32 = 1u32;
pub const CERT_PE_IMAGE_DIGEST_NON_PE_INFO: u32 = 8u32;
pub const CERT_PE_IMAGE_DIGEST_RESOURCES: u32 = 2u32;
pub const CERT_SECTION_TYPE_ANY: u32 = 255u32;
pub const CHECKSUM_MAPVIEW_FAILURE: u32 = 3u32;
pub const CHECKSUM_MAP_FAILURE: u32 = 2u32;
pub const CHECKSUM_OPEN_FAILURE: u32 = 1u32;
pub const CHECKSUM_SUCCESS: u32 = 0u32;
pub const CHECKSUM_UNICODE_FAILURE: u32 = 4u32;
#[cfg(any(target_arch = "aarch64",))]
#[cfg(feature = "Win32_System_Kernel")]
#[repr(C)]
pub struct CONTEXT(i32);
#[cfg(any(target_arch = "x86_64",))]
#[cfg(feature = "Win32_System_Kernel")]
#[repr(C)]
pub struct CONTEXT(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_System_Kernel")]
#[repr(C)]
pub struct CONTEXT(i32);
#[repr(C)]
pub struct CPU_INFORMATION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
#[repr(C)]
pub struct CREATE_PROCESS_DEBUG_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
#[repr(C)]
pub struct CREATE_THREAD_DEBUG_INFO(i32);
pub const CROSS_PLATFORM_MAXIMUM_PROCESSORS: u32 = 2048u32;
pub const CURRENT_KD_SECONDARY_VERSION: u32 = 2u32;
#[repr(transparent)]
pub struct CallingConventionKind(pub i32);
pub const CallingConventionUnknown: CallingConventionKind = CallingConventionKind(0i32);
pub const CallingConventionCDecl: CallingConventionKind = CallingConventionKind(1i32);
pub const CallingConventionFastCall: CallingConventionKind = CallingConventionKind(2i32);
pub const CallingConventionStdCall: CallingConventionKind = CallingConventionKind(3i32);
pub const CallingConventionSysCall: CallingConventionKind = CallingConventionKind(4i32);
pub const CallingConventionThisCall: CallingConventionKind = CallingConventionKind(5i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DBGHELP_DATA_REPORT_STRUCT(i32);
#[cfg(feature = "Win32_System_Kernel")]
#[repr(C)]
pub struct DBGKD_DEBUG_DATA_HEADER32(i32);
#[cfg(feature = "Win32_System_Kernel")]
#[repr(C)]
pub struct DBGKD_DEBUG_DATA_HEADER64(i32);
#[repr(C)]
pub struct DBGKD_GET_VERSION32(i32);
#[repr(C)]
pub struct DBGKD_GET_VERSION64(i32);
#[repr(transparent)]
pub struct DBGKD_MAJOR_TYPES(pub i32);
pub const DBGKD_MAJOR_NT: DBGKD_MAJOR_TYPES = DBGKD_MAJOR_TYPES(0i32);
pub const DBGKD_MAJOR_XBOX: DBGKD_MAJOR_TYPES = DBGKD_MAJOR_TYPES(1i32);
pub const DBGKD_MAJOR_BIG: DBGKD_MAJOR_TYPES = DBGKD_MAJOR_TYPES(2i32);
pub const DBGKD_MAJOR_EXDI: DBGKD_MAJOR_TYPES = DBGKD_MAJOR_TYPES(3i32);
pub const DBGKD_MAJOR_NTBD: DBGKD_MAJOR_TYPES = DBGKD_MAJOR_TYPES(4i32);
pub const DBGKD_MAJOR_EFI: DBGKD_MAJOR_TYPES = DBGKD_MAJOR_TYPES(5i32);
pub const DBGKD_MAJOR_TNT: DBGKD_MAJOR_TYPES = DBGKD_MAJOR_TYPES(6i32);
pub const DBGKD_MAJOR_SINGULARITY: DBGKD_MAJOR_TYPES = DBGKD_MAJOR_TYPES(7i32);
pub const DBGKD_MAJOR_HYPERVISOR: DBGKD_MAJOR_TYPES = DBGKD_MAJOR_TYPES(8i32);
pub const DBGKD_MAJOR_MIDORI: DBGKD_MAJOR_TYPES = DBGKD_MAJOR_TYPES(9i32);
pub const DBGKD_MAJOR_CE: DBGKD_MAJOR_TYPES = DBGKD_MAJOR_TYPES(10i32);
pub const DBGKD_MAJOR_COUNT: DBGKD_MAJOR_TYPES = DBGKD_MAJOR_TYPES(11i32);
pub const DBGKD_SIMULATION_EXDI: i32 = 1i32;
pub const DBGKD_SIMULATION_NONE: i32 = 0i32;
pub const DBGKD_VERS_FLAG_DATA: u32 = 2u32;
pub const DBGKD_VERS_FLAG_HAL_IN_NTOS: u32 = 64u32;
pub const DBGKD_VERS_FLAG_HSS: u32 = 16u32;
pub const DBGKD_VERS_FLAG_MP: u32 = 1u32;
pub const DBGKD_VERS_FLAG_NOMM: u32 = 8u32;
pub const DBGKD_VERS_FLAG_PARTITIONS: u32 = 32u32;
pub const DBGKD_VERS_FLAG_PTR64: u32 = 4u32;
#[repr(transparent)]
pub struct DBGPROP_ATTRIB_FLAGS(pub u32);
pub const DBGPROP_ATTRIB_NO_ATTRIB: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(0u32);
pub const DBGPROP_ATTRIB_VALUE_IS_INVALID: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(8u32);
pub const DBGPROP_ATTRIB_VALUE_IS_EXPANDABLE: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(16u32);
pub const DBGPROP_ATTRIB_VALUE_IS_FAKE: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(32u32);
pub const DBGPROP_ATTRIB_VALUE_IS_METHOD: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(256u32);
pub const DBGPROP_ATTRIB_VALUE_IS_EVENT: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(512u32);
pub const DBGPROP_ATTRIB_VALUE_IS_RAW_STRING: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(1024u32);
pub const DBGPROP_ATTRIB_VALUE_READONLY: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(2048u32);
pub const DBGPROP_ATTRIB_ACCESS_PUBLIC: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(4096u32);
pub const DBGPROP_ATTRIB_ACCESS_PRIVATE: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(8192u32);
pub const DBGPROP_ATTRIB_ACCESS_PROTECTED: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(16384u32);
pub const DBGPROP_ATTRIB_ACCESS_FINAL: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(32768u32);
pub const DBGPROP_ATTRIB_STORAGE_GLOBAL: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(65536u32);
pub const DBGPROP_ATTRIB_STORAGE_STATIC: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(131072u32);
pub const DBGPROP_ATTRIB_STORAGE_FIELD: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(262144u32);
pub const DBGPROP_ATTRIB_STORAGE_VIRTUAL: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(524288u32);
pub const DBGPROP_ATTRIB_TYPE_IS_CONSTANT: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(1048576u32);
pub const DBGPROP_ATTRIB_TYPE_IS_SYNCHRONIZED: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(2097152u32);
pub const DBGPROP_ATTRIB_TYPE_IS_VOLATILE: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(4194304u32);
pub const DBGPROP_ATTRIB_HAS_EXTENDED_ATTRIBS: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(8388608u32);
pub const DBGPROP_ATTRIB_FRAME_INTRYBLOCK: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(16777216u32);
pub const DBGPROP_ATTRIB_FRAME_INCATCHBLOCK: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(33554432u32);
pub const DBGPROP_ATTRIB_FRAME_INFINALLYBLOCK: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(67108864u32);
pub const DBGPROP_ATTRIB_VALUE_IS_RETURN_VALUE: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(134217728u32);
pub const DBGPROP_ATTRIB_VALUE_PENDING_MUTATION: DBGPROP_ATTRIB_FLAGS = DBGPROP_ATTRIB_FLAGS(268435456u32);
#[repr(transparent)]
pub struct DBGPROP_INFO(pub u32);
pub const DBGPROP_INFO_NAME: DBGPROP_INFO = DBGPROP_INFO(1u32);
pub const DBGPROP_INFO_TYPE: DBGPROP_INFO = DBGPROP_INFO(2u32);
pub const DBGPROP_INFO_VALUE: DBGPROP_INFO = DBGPROP_INFO(4u32);
pub const DBGPROP_INFO_FULLNAME: DBGPROP_INFO = DBGPROP_INFO(32u32);
pub const DBGPROP_INFO_ATTRIBUTES: DBGPROP_INFO = DBGPROP_INFO(8u32);
pub const DBGPROP_INFO_DEBUGPROP: DBGPROP_INFO = DBGPROP_INFO(16u32);
pub const DBGPROP_INFO_BEAUTIFY: DBGPROP_INFO = DBGPROP_INFO(33554432u32);
pub const DBGPROP_INFO_CALLTOSTRING: DBGPROP_INFO = DBGPROP_INFO(67108864u32);
pub const DBGPROP_INFO_AUTOEXPAND: DBGPROP_INFO = DBGPROP_INFO(134217728u32);
pub const DBG_DUMP_ADDRESS_AT_END: u32 = 131072u32;
pub const DBG_DUMP_ADDRESS_OF_FIELD: u32 = 65536u32;
pub const DBG_DUMP_ARRAY: u32 = 32768u32;
pub const DBG_DUMP_BLOCK_RECURSE: u32 = 2097152u32;
pub const DBG_DUMP_CALL_FOR_EACH: u32 = 8u32;
pub const DBG_DUMP_COMPACT_OUT: u32 = 8192u32;
pub const DBG_DUMP_COPY_TYPE_DATA: u32 = 262144u32;
pub const DBG_DUMP_FIELD_ARRAY: u32 = 16u32;
pub const DBG_DUMP_FIELD_CALL_BEFORE_PRINT: u32 = 1u32;
pub const DBG_DUMP_FIELD_COPY_FIELD_DATA: u32 = 32u32;
pub const DBG_DUMP_FIELD_DEFAULT_STRING: u32 = 65536u32;
pub const DBG_DUMP_FIELD_FULL_NAME: u32 = 8u32;
pub const DBG_DUMP_FIELD_GUID_STRING: u32 = 524288u32;
pub const DBG_DUMP_FIELD_MULTI_STRING: u32 = 262144u32;
pub const DBG_DUMP_FIELD_NO_CALLBACK_REQ: u32 = 2u32;
pub const DBG_DUMP_FIELD_NO_PRINT: u32 = 16384u32;
pub const DBG_DUMP_FIELD_RECUR_ON_THIS: u32 = 4u32;
pub const DBG_DUMP_FIELD_RETURN_ADDRESS: u32 = 4096u32;
pub const DBG_DUMP_FIELD_SIZE_IN_BITS: u32 = 8192u32;
pub const DBG_DUMP_FIELD_UTF32_STRING: u32 = 1048576u32;
pub const DBG_DUMP_FIELD_WCHAR_STRING: u32 = 131072u32;
pub const DBG_DUMP_FUNCTION_FORMAT: u32 = 1048576u32;
pub const DBG_DUMP_GET_SIZE_ONLY: u32 = 128u32;
pub const DBG_DUMP_LIST: u32 = 32u32;
pub const DBG_DUMP_MATCH_SIZE: u32 = 4194304u32;
pub const DBG_DUMP_NO_INDENT: u32 = 1u32;
pub const DBG_DUMP_NO_OFFSET: u32 = 2u32;
pub const DBG_DUMP_NO_PRINT: u32 = 64u32;
pub const DBG_DUMP_READ_PHYSICAL: u32 = 524288u32;
pub const DBG_DUMP_VERBOSE: u32 = 4u32;
pub const DBG_FRAME_DEFAULT: u32 = 0u32;
pub const DBG_FRAME_IGNORE_INLINE: u32 = 4294967295u32;
pub const DBG_RETURN_SUBTYPES: u32 = 0u32;
pub const DBG_RETURN_TYPE: u32 = 0u32;
pub const DBG_RETURN_TYPE_VALUES: u32 = 0u32;
pub const DBHHEADER_PDBGUID: u32 = 3u32;
pub const DEBUG_ADDSYNTHMOD_DEFAULT: u32 = 0u32;
pub const DEBUG_ADDSYNTHMOD_ZEROBASE: u32 = 1u32;
pub const DEBUG_ADDSYNTHSYM_DEFAULT: u32 = 0u32;
pub const DEBUG_ANY_ID: u32 = 4294967295u32;
pub const DEBUG_ASMOPT_DEFAULT: u32 = 0u32;
pub const DEBUG_ASMOPT_IGNORE_OUTPUT_WIDTH: u32 = 4u32;
pub const DEBUG_ASMOPT_NO_CODE_BYTES: u32 = 2u32;
pub const DEBUG_ASMOPT_SOURCE_LINE_NUMBER: u32 = 8u32;
pub const DEBUG_ASMOPT_VERBOSE: u32 = 1u32;
pub const DEBUG_ATTACH_DEFAULT: u32 = 0u32;
pub const DEBUG_ATTACH_EXDI_DRIVER: u32 = 2u32;
pub const DEBUG_ATTACH_EXISTING: u32 = 2u32;
pub const DEBUG_ATTACH_INSTALL_DRIVER: u32 = 4u32;
pub const DEBUG_ATTACH_INVASIVE_NO_INITIAL_BREAK: u32 = 8u32;
pub const DEBUG_ATTACH_INVASIVE_RESUME_PROCESS: u32 = 16u32;
pub const DEBUG_ATTACH_KERNEL_CONNECTION: u32 = 0u32;
pub const DEBUG_ATTACH_LOCAL_KERNEL: u32 = 1u32;
pub const DEBUG_ATTACH_NONINVASIVE: u32 = 1u32;
pub const DEBUG_ATTACH_NONINVASIVE_ALLOW_PARTIAL: u32 = 32u32;
pub const DEBUG_ATTACH_NONINVASIVE_NO_SUSPEND: u32 = 4u32;
pub const DEBUG_BREAKPOINT_ADDER_ONLY: u32 = 8u32;
pub const DEBUG_BREAKPOINT_CODE: u32 = 0u32;
pub const DEBUG_BREAKPOINT_DATA: u32 = 1u32;
pub const DEBUG_BREAKPOINT_DEFERRED: u32 = 2u32;
pub const DEBUG_BREAKPOINT_ENABLED: u32 = 4u32;
pub const DEBUG_BREAKPOINT_GO_ONLY: u32 = 1u32;
pub const DEBUG_BREAKPOINT_INLINE: u32 = 3u32;
pub const DEBUG_BREAKPOINT_ONE_SHOT: u32 = 16u32;
#[repr(C)]
pub struct DEBUG_BREAKPOINT_PARAMETERS(i32);
pub const DEBUG_BREAKPOINT_TIME: u32 = 2u32;
pub const DEBUG_BREAK_EXECUTE: u32 = 4u32;
pub const DEBUG_BREAK_IO: u32 = 8u32;
pub const DEBUG_BREAK_READ: u32 = 1u32;
pub const DEBUG_BREAK_WRITE: u32 = 2u32;
#[repr(C)]
pub struct DEBUG_CACHED_SYMBOL_INFO(i32);
pub const DEBUG_CDS_ALL: u32 = 4294967295u32;
pub const DEBUG_CDS_DATA: u32 = 2u32;
pub const DEBUG_CDS_REFRESH: u32 = 4u32;
pub const DEBUG_CDS_REFRESH_ADDBREAKPOINT: u32 = 4u32;
pub const DEBUG_CDS_REFRESH_EVALUATE: u32 = 1u32;
pub const DEBUG_CDS_REFRESH_EXECUTE: u32 = 2u32;
pub const DEBUG_CDS_REFRESH_EXECUTECOMMANDFILE: u32 = 3u32;
pub const DEBUG_CDS_REFRESH_INLINESTEP: u32 = 16u32;
pub const DEBUG_CDS_REFRESH_INLINESTEP_PSEUDO: u32 = 17u32;
pub const DEBUG_CDS_REFRESH_REMOVEBREAKPOINT: u32 = 5u32;
pub const DEBUG_CDS_REFRESH_SETSCOPE: u32 = 12u32;
pub const DEBUG_CDS_REFRESH_SETSCOPEFRAMEBYINDEX: u32 = 13u32;
pub const DEBUG_CDS_REFRESH_SETSCOPEFROMJITDEBUGINFO: u32 = 14u32;
pub const DEBUG_CDS_REFRESH_SETSCOPEFROMSTOREDEVENT: u32 = 15u32;
pub const DEBUG_CDS_REFRESH_SETVALUE: u32 = 10u32;
pub const DEBUG_CDS_REFRESH_SETVALUE2: u32 = 11u32;
pub const DEBUG_CDS_REFRESH_WRITEPHYSICAL: u32 = 8u32;
pub const DEBUG_CDS_REFRESH_WRITEPHYSICAL2: u32 = 9u32;
pub const DEBUG_CDS_REFRESH_WRITEVIRTUAL: u32 = 6u32;
pub const DEBUG_CDS_REFRESH_WRITEVIRTUALUNCACHED: u32 = 7u32;
pub const DEBUG_CDS_REGISTERS: u32 = 1u32;
pub const DEBUG_CES_ALL: u32 = 4294967295u32;
pub const DEBUG_CES_ASSEMBLY_OPTIONS: u32 = 4096u32;
pub const DEBUG_CES_BREAKPOINTS: u32 = 4u32;
pub const DEBUG_CES_CODE_LEVEL: u32 = 8u32;
pub const DEBUG_CES_CURRENT_THREAD: u32 = 1u32;
pub const DEBUG_CES_EFFECTIVE_PROCESSOR: u32 = 2u32;
pub const DEBUG_CES_ENGINE_OPTIONS: u32 = 32u32;
pub const DEBUG_CES_EVENT_FILTERS: u32 = 256u32;
pub const DEBUG_CES_EXECUTION_STATUS: u32 = 16u32;
pub const DEBUG_CES_EXPRESSION_SYNTAX: u32 = 8192u32;
pub const DEBUG_CES_EXTENSIONS: u32 = 1024u32;
pub const DEBUG_CES_LOG_FILE: u32 = 64u32;
pub const DEBUG_CES_PROCESS_OPTIONS: u32 = 512u32;
pub const DEBUG_CES_RADIX: u32 = 128u32;
pub const DEBUG_CES_SYSTEMS: u32 = 2048u32;
pub const DEBUG_CES_TEXT_REPLACEMENTS: u32 = 16384u32;
pub const DEBUG_CLASS_IMAGE_FILE: u32 = 3u32;
pub const DEBUG_CLASS_KERNEL: u32 = 1u32;
pub const DEBUG_CLASS_UNINITIALIZED: u32 = 0u32;
pub const DEBUG_CLASS_USER_WINDOWS: u32 = 2u32;
pub const DEBUG_CLIENT_CDB: u32 = 4u32;
#[repr(C)]
pub struct DEBUG_CLIENT_CONTEXT(i32);
pub const DEBUG_CLIENT_KD: u32 = 5u32;
pub const DEBUG_CLIENT_NTKD: u32 = 3u32;
pub const DEBUG_CLIENT_NTSD: u32 = 2u32;
pub const DEBUG_CLIENT_UNKNOWN: u32 = 0u32;
pub const DEBUG_CLIENT_VSINT: u32 = 1u32;
pub const DEBUG_CLIENT_WINDBG: u32 = 6u32;
pub const DEBUG_CLIENT_WINIDE: u32 = 7u32;
pub const DEBUG_CMDEX_ADD_EVENT_STRING: u32 = 1u32;
pub const DEBUG_CMDEX_INVALID: u32 = 0u32;
pub const DEBUG_CMDEX_RESET_EVENT_STRINGS: u32 = 2u32;
pub const DEBUG_COMMAND_EXCEPTION_ID: u32 = 3688893886u32;
pub const DEBUG_CONNECT_SESSION_DEFAULT: u32 = 0u32;
pub const DEBUG_CONNECT_SESSION_NO_ANNOUNCE: u32 = 2u32;
pub const DEBUG_CONNECT_SESSION_NO_VERSION: u32 = 1u32;
#[repr(C)]
pub struct DEBUG_CREATE_PROCESS_OPTIONS(i32);
pub const DEBUG_CSS_ALL: u32 = 4294967295u32;
pub const DEBUG_CSS_COLLAPSE_CHILDREN: u32 = 64u32;
pub const DEBUG_CSS_LOADS: u32 = 1u32;
pub const DEBUG_CSS_PATHS: u32 = 8u32;
pub const DEBUG_CSS_SCOPE: u32 = 4u32;
pub const DEBUG_CSS_SYMBOL_OPTIONS: u32 = 16u32;
pub const DEBUG_CSS_TYPE_OPTIONS: u32 = 32u32;
pub const DEBUG_CSS_UNLOADS: u32 = 2u32;
pub const DEBUG_CURRENT_DEFAULT: u32 = 15u32;
pub const DEBUG_CURRENT_DISASM: u32 = 2u32;
pub const DEBUG_CURRENT_REGISTERS: u32 = 4u32;
pub const DEBUG_CURRENT_SOURCE_LINE: u32 = 8u32;
pub const DEBUG_CURRENT_SYMBOL: u32 = 1u32;
pub const DEBUG_DATA_BASE_TRANSLATION_VIRTUAL_OFFSET: u32 = 3u32;
pub const DEBUG_DATA_BreakpointWithStatusAddr: u32 = 32u32;
pub const DEBUG_DATA_CmNtCSDVersionAddr: u32 = 616u32;
pub const DEBUG_DATA_DumpAttributes: u32 = 100072u32;
pub const DEBUG_DATA_DumpFormatVersion: u32 = 100040u32;
pub const DEBUG_DATA_DumpMmStorage: u32 = 100064u32;
pub const DEBUG_DATA_DumpPowerState: u32 = 100056u32;
pub const DEBUG_DATA_DumpWriterStatus: u32 = 100032u32;
pub const DEBUG_DATA_DumpWriterVersion: u32 = 100048u32;
pub const DEBUG_DATA_EtwpDebuggerData: u32 = 816u32;
pub const DEBUG_DATA_ExpNumberOfPagedPoolsAddr: u32 = 112u32;
pub const DEBUG_DATA_ExpPagedPoolDescriptorAddr: u32 = 104u32;
pub const DEBUG_DATA_ExpSystemResourcesListAddr: u32 = 96u32;
pub const DEBUG_DATA_IopErrorLogListHeadAddr: u32 = 144u32;
pub const DEBUG_DATA_KPCR_OFFSET: u32 = 0u32;
pub const DEBUG_DATA_KPRCB_OFFSET: u32 = 1u32;
pub const DEBUG_DATA_KTHREAD_OFFSET: u32 = 2u32;
pub const DEBUG_DATA_KdPrintBufferSizeAddr: u32 = 720u32;
pub const DEBUG_DATA_KdPrintCircularBufferAddr: u32 = 480u32;
pub const DEBUG_DATA_KdPrintCircularBufferEndAddr: u32 = 488u32;
pub const DEBUG_DATA_KdPrintCircularBufferPtrAddr: u32 = 712u32;
pub const DEBUG_DATA_KdPrintRolloverCountAddr: u32 = 504u32;
pub const DEBUG_DATA_KdPrintWritePointerAddr: u32 = 496u32;
pub const DEBUG_DATA_KeBugCheckCallbackListHeadAddr: u32 = 128u32;
pub const DEBUG_DATA_KeTimeIncrementAddr: u32 = 120u32;
pub const DEBUG_DATA_KeUserCallbackDispatcherAddr: u32 = 64u32;
pub const DEBUG_DATA_KernBase: u32 = 24u32;
pub const DEBUG_DATA_KernelVerifierAddr: u32 = 576u32;
pub const DEBUG_DATA_KiBugcheckDataAddr: u32 = 136u32;
pub const DEBUG_DATA_KiCallUserModeAddr: u32 = 56u32;
pub const DEBUG_DATA_KiNormalSystemCall: u32 = 528u32;
pub const DEBUG_DATA_KiProcessorBlockAddr: u32 = 536u32;
pub const DEBUG_DATA_MmAllocatedNonPagedPoolAddr: u32 = 592u32;
pub const DEBUG_DATA_MmAvailablePagesAddr: u32 = 424u32;
pub const DEBUG_DATA_MmBadPagesDetected: u32 = 800u32;
pub const DEBUG_DATA_MmDriverCommitAddr: u32 = 352u32;
pub const DEBUG_DATA_MmExtendedCommitAddr: u32 = 376u32;
pub const DEBUG_DATA_MmFreePageListHeadAddr: u32 = 392u32;
pub const DEBUG_DATA_MmHighestPhysicalPageAddr: u32 = 240u32;
pub const DEBUG_DATA_MmHighestUserAddressAddr: u32 = 456u32;
pub const DEBUG_DATA_MmLastUnloadedDriverAddr: u32 = 552u32;
pub const DEBUG_DATA_MmLoadedUserImageListAddr: u32 = 512u32;
pub const DEBUG_DATA_MmLowestPhysicalPageAddr: u32 = 232u32;
pub const DEBUG_DATA_MmMaximumNonPagedPoolInBytesAddr: u32 = 256u32;
pub const DEBUG_DATA_MmModifiedNoWritePageListHeadAddr: u32 = 416u32;
pub const DEBUG_DATA_MmModifiedPageListHeadAddr: u32 = 408u32;
pub const DEBUG_DATA_MmNonPagedPoolEndAddr: u32 = 280u32;
pub const DEBUG_DATA_MmNonPagedPoolStartAddr: u32 = 272u32;
pub const DEBUG_DATA_MmNonPagedSystemStartAddr: u32 = 264u32;
pub const DEBUG_DATA_MmNumberOfPagingFilesAddr: u32 = 224u32;
pub const DEBUG_DATA_MmNumberOfPhysicalPagesAddr: u32 = 248u32;
pub const DEBUG_DATA_MmPageSize: u32 = 312u32;
pub const DEBUG_DATA_MmPagedPoolCommitAddr: u32 = 368u32;
pub const DEBUG_DATA_MmPagedPoolEndAddr: u32 = 296u32;
pub const DEBUG_DATA_MmPagedPoolInformationAddr: u32 = 304u32;
pub const DEBUG_DATA_MmPagedPoolStartAddr: u32 = 288u32;
pub const DEBUG_DATA_MmPeakCommitmentAddr: u32 = 600u32;
pub const DEBUG_DATA_MmPfnDatabaseAddr: u32 = 192u32;
pub const DEBUG_DATA_MmPhysicalMemoryBlockAddr: u32 = 624u32;
pub const DEBUG_DATA_MmProcessCommitAddr: u32 = 360u32;
pub const DEBUG_DATA_MmResidentAvailablePagesAddr: u32 = 432u32;
pub const DEBUG_DATA_MmSessionBase: u32 = 632u32;
pub const DEBUG_DATA_MmSessionSize: u32 = 640u32;
pub const DEBUG_DATA_MmSharedCommitAddr: u32 = 344u32;
pub const DEBUG_DATA_MmSizeOfPagedPoolInBytesAddr: u32 = 320u32;
pub const DEBUG_DATA_MmSpecialPoolTagAddr: u32 = 568u32;
pub const DEBUG_DATA_MmStandbyPageListHeadAddr: u32 = 400u32;
pub const DEBUG_DATA_MmSubsectionBaseAddr: u32 = 216u32;
pub const DEBUG_DATA_MmSystemCacheEndAddr: u32 = 176u32;
pub const DEBUG_DATA_MmSystemCacheStartAddr: u32 = 168u32;
pub const DEBUG_DATA_MmSystemCacheWsAddr: u32 = 184u32;
pub const DEBUG_DATA_MmSystemParentTablePage: u32 = 648u32;
pub const DEBUG_DATA_MmSystemPtesEndAddr: u32 = 208u32;
pub const DEBUG_DATA_MmSystemPtesStartAddr: u32 = 200u32;
pub const DEBUG_DATA_MmSystemRangeStartAddr: u32 = 464u32;
pub const DEBUG_DATA_MmTotalCommitLimitAddr: u32 = 328u32;
pub const DEBUG_DATA_MmTotalCommitLimitMaximumAddr: u32 = 608u32;
pub const DEBUG_DATA_MmTotalCommittedPagesAddr: u32 = 336u32;
pub const DEBUG_DATA_MmTriageActionTakenAddr: u32 = 560u32;
pub const DEBUG_DATA_MmUnloadedDriversAddr: u32 = 544u32;
pub const DEBUG_DATA_MmUserProbeAddressAddr: u32 = 472u32;
pub const DEBUG_DATA_MmVerifierDataAddr: u32 = 584u32;
pub const DEBUG_DATA_MmVirtualTranslationBase: u32 = 656u32;
pub const DEBUG_DATA_MmZeroedPageListHeadAddr: u32 = 384u32;
pub const DEBUG_DATA_NonPagedPoolDescriptorAddr: u32 = 448u32;
pub const DEBUG_DATA_NtBuildLabAddr: u32 = 520u32;
pub const DEBUG_DATA_ObpRootDirectoryObjectAddr: u32 = 152u32;
pub const DEBUG_DATA_ObpTypeObjectTypeAddr: u32 = 160u32;
pub const DEBUG_DATA_OffsetEprocessDirectoryTableBase: u32 = 686u32;
pub const DEBUG_DATA_OffsetEprocessParentCID: u32 = 684u32;
pub const DEBUG_DATA_OffsetEprocessPeb: u32 = 682u32;
pub const DEBUG_DATA_OffsetKThreadApcProcess: u32 = 672u32;
pub const DEBUG_DATA_OffsetKThreadBStore: u32 = 676u32;
pub const DEBUG_DATA_OffsetKThreadBStoreLimit: u32 = 678u32;
pub const DEBUG_DATA_OffsetKThreadInitialStack: u32 = 670u32;
pub const DEBUG_DATA_OffsetKThreadKernelStack: u32 = 668u32;
pub const DEBUG_DATA_OffsetKThreadNextProcessor: u32 = 664u32;
pub const DEBUG_DATA_OffsetKThreadState: u32 = 674u32;
pub const DEBUG_DATA_OffsetKThreadTeb: u32 = 666u32;
pub const DEBUG_DATA_OffsetPrcbCpuType: u32 = 696u32;
pub const DEBUG_DATA_OffsetPrcbCurrentThread: u32 = 692u32;
pub const DEBUG_DATA_OffsetPrcbDpcRoutine: u32 = 690u32;
pub const DEBUG_DATA_OffsetPrcbMhz: u32 = 694u32;
pub const DEBUG_DATA_OffsetPrcbNumber: u32 = 702u32;
pub const DEBUG_DATA_OffsetPrcbProcessorState: u32 = 700u32;
pub const DEBUG_DATA_OffsetPrcbVendorString: u32 = 698u32;
pub const DEBUG_DATA_PROCESSOR_IDENTIFICATION: u32 = 4u32;
pub const DEBUG_DATA_PROCESSOR_SPEED: u32 = 5u32;
pub const DEBUG_DATA_PaeEnabled: u32 = 100000u32;
pub const DEBUG_DATA_PoolTrackTableAddr: u32 = 440u32;
pub const DEBUG_DATA_ProductType: u32 = 100016u32;
pub const DEBUG_DATA_PsActiveProcessHeadAddr: u32 = 80u32;
pub const DEBUG_DATA_PsLoadedModuleListAddr: u32 = 72u32;
pub const DEBUG_DATA_PspCidTableAddr: u32 = 88u32;
pub const DEBUG_DATA_PteBase: u32 = 864u32;
pub const DEBUG_DATA_SPACE_BUS_DATA: u32 = 5u32;
pub const DEBUG_DATA_SPACE_CONTROL: u32 = 2u32;
pub const DEBUG_DATA_SPACE_COUNT: u32 = 7u32;
pub const DEBUG_DATA_SPACE_DEBUGGER_DATA: u32 = 6u32;
pub const DEBUG_DATA_SPACE_IO: u32 = 3u32;
pub const DEBUG_DATA_SPACE_MSR: u32 = 4u32;
pub const DEBUG_DATA_SPACE_PHYSICAL: u32 = 1u32;
pub const DEBUG_DATA_SPACE_VIRTUAL: u32 = 0u32;
pub const DEBUG_DATA_SavedContextAddr: u32 = 40u32;
pub const DEBUG_DATA_SharedUserData: u32 = 100008u32;
pub const DEBUG_DATA_SizeEProcess: u32 = 680u32;
pub const DEBUG_DATA_SizeEThread: u32 = 704u32;
pub const DEBUG_DATA_SizePrcb: u32 = 688u32;
pub const DEBUG_DATA_SuiteMask: u32 = 100024u32;
pub const DEBUG_DISASM_EFFECTIVE_ADDRESS: u32 = 1u32;
pub const DEBUG_DISASM_MATCHING_SYMBOLS: u32 = 2u32;
pub const DEBUG_DISASM_SOURCE_FILE_NAME: u32 = 8u32;
pub const DEBUG_DISASM_SOURCE_LINE_NUMBER: u32 = 4u32;
pub const DEBUG_DUMP_ACTIVE: u32 = 1030u32;
pub const DEBUG_DUMP_DEFAULT: u32 = 1025u32;
pub const DEBUG_DUMP_FILE_BASE: u32 = 4294967295u32;
pub const DEBUG_DUMP_FILE_LOAD_FAILED_INDEX: u32 = 4294967295u32;
pub const DEBUG_DUMP_FILE_ORIGINAL_CAB_INDEX: u32 = 4294967294u32;
pub const DEBUG_DUMP_FILE_PAGE_FILE_DUMP: u32 = 0u32;
pub const DEBUG_DUMP_FULL: u32 = 1026u32;
pub const DEBUG_DUMP_IMAGE_FILE: u32 = 1027u32;
pub const DEBUG_DUMP_SMALL: u32 = 1024u32;
pub const DEBUG_DUMP_TRACE_LOG: u32 = 1028u32;
pub const DEBUG_DUMP_WINDOWS_CE: u32 = 1029u32;
pub const DEBUG_ECREATE_PROCESS_DEFAULT: u32 = 0u32;
pub const DEBUG_ECREATE_PROCESS_INHERIT_HANDLES: u32 = 1u32;
pub const DEBUG_ECREATE_PROCESS_USE_IMPLICIT_COMMAND_LINE: u32 = 4u32;
pub const DEBUG_ECREATE_PROCESS_USE_VERIFIER_FLAGS: u32 = 2u32;
pub const DEBUG_EINDEX_FROM_CURRENT: u32 = 2u32;
pub const DEBUG_EINDEX_FROM_END: u32 = 1u32;
pub const DEBUG_EINDEX_FROM_START: u32 = 0u32;
pub const DEBUG_EINDEX_NAME: u32 = 0u32;
pub const DEBUG_END_ACTIVE_DETACH: u32 = 2u32;
pub const DEBUG_END_ACTIVE_TERMINATE: u32 = 1u32;
pub const DEBUG_END_DISCONNECT: u32 = 4u32;
pub const DEBUG_END_PASSIVE: u32 = 0u32;
pub const DEBUG_END_REENTRANT: u32 = 3u32;
pub const DEBUG_ENGOPT_ALL: u32 = 15728639u32;
pub const DEBUG_ENGOPT_ALLOW_NETWORK_PATHS: u32 = 4u32;
pub const DEBUG_ENGOPT_ALLOW_READ_ONLY_BREAKPOINTS: u32 = 1024u32;
pub const DEBUG_ENGOPT_DEBUGGING_SENSITIVE_DATA: u32 = 4194304u32;
pub const DEBUG_ENGOPT_DISABLESQM: u32 = 524288u32;
pub const DEBUG_ENGOPT_DISABLE_EXECUTION_COMMANDS: u32 = 65536u32;
pub const DEBUG_ENGOPT_DISABLE_MANAGED_SUPPORT: u32 = 16384u32;
pub const DEBUG_ENGOPT_DISABLE_MODULE_SYMBOL_LOAD: u32 = 32768u32;
pub const DEBUG_ENGOPT_DISABLE_STEPLINES_OPTIONS: u32 = 2097152u32;
pub const DEBUG_ENGOPT_DISALLOW_IMAGE_FILE_MAPPING: u32 = 131072u32;
pub const DEBUG_ENGOPT_DISALLOW_NETWORK_PATHS: u32 = 8u32;
pub const DEBUG_ENGOPT_DISALLOW_SHELL_COMMANDS: u32 = 4096u32;
pub const DEBUG_ENGOPT_FAIL_INCOMPLETE_INFORMATION: u32 = 512u32;
pub const DEBUG_ENGOPT_FINAL_BREAK: u32 = 128u32;
pub const DEBUG_ENGOPT_IGNORE_DBGHELP_VERSION: u32 = 1u32;
pub const DEBUG_ENGOPT_IGNORE_EXTENSION_VERSIONS: u32 = 2u32;
pub const DEBUG_ENGOPT_IGNORE_LOADER_EXCEPTIONS: u32 = 16u32;
pub const DEBUG_ENGOPT_INITIAL_BREAK: u32 = 32u32;
pub const DEBUG_ENGOPT_INITIAL_MODULE_BREAK: u32 = 64u32;
pub const DEBUG_ENGOPT_KD_QUIET_MODE: u32 = 8192u32;
pub const DEBUG_ENGOPT_NO_EXECUTE_REPEAT: u32 = 256u32;
pub const DEBUG_ENGOPT_PREFER_DML: u32 = 262144u32;
pub const DEBUG_ENGOPT_PREFER_TRACE_FILES: u32 = 8388608u32;
pub const DEBUG_ENGOPT_SYNCHRONIZE_BREAKPOINTS: u32 = 2048u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
#[repr(C)]
pub struct DEBUG_EVENT(i32);
pub const DEBUG_EVENT_BREAKPOINT: u32 = 1u32;
pub const DEBUG_EVENT_CHANGE_DEBUGGEE_STATE: u32 = 1024u32;
pub const DEBUG_EVENT_CHANGE_ENGINE_STATE: u32 = 2048u32;
pub const DEBUG_EVENT_CHANGE_SYMBOL_STATE: u32 = 4096u32;
#[repr(transparent)]
pub struct DEBUG_EVENT_CODE(pub u32);
pub const CREATE_PROCESS_DEBUG_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(3u32);
pub const CREATE_THREAD_DEBUG_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(2u32);
pub const EXCEPTION_DEBUG_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(1u32);
pub const EXIT_PROCESS_DEBUG_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(5u32);
pub const EXIT_THREAD_DEBUG_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(4u32);
pub const LOAD_DLL_DEBUG_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(6u32);
pub const OUTPUT_DEBUG_STRING_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(8u32);
pub const RIP_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(9u32);
pub const UNLOAD_DLL_DEBUG_EVENT: DEBUG_EVENT_CODE = DEBUG_EVENT_CODE(7u32);
#[repr(C)]
pub struct DEBUG_EVENT_CONTEXT(i32);
pub const DEBUG_EVENT_CREATE_PROCESS: u32 = 16u32;
pub const DEBUG_EVENT_CREATE_THREAD: u32 = 4u32;
pub const DEBUG_EVENT_EXCEPTION: u32 = 2u32;
pub const DEBUG_EVENT_EXIT_PROCESS: u32 = 32u32;
pub const DEBUG_EVENT_EXIT_THREAD: u32 = 8u32;
#[repr(transparent)]
pub struct DEBUG_EVENT_INFO_TYPE(pub i32);
pub const DEIT_GENERAL: DEBUG_EVENT_INFO_TYPE = DEBUG_EVENT_INFO_TYPE(0i32);
pub const DEIT_ASMJS_IN_DEBUGGING: DEBUG_EVENT_INFO_TYPE = DEBUG_EVENT_INFO_TYPE(1i32);
pub const DEIT_ASMJS_SUCCEEDED: DEBUG_EVENT_INFO_TYPE = DEBUG_EVENT_INFO_TYPE(2i32);
pub const DEIT_ASMJS_FAILED: DEBUG_EVENT_INFO_TYPE = DEBUG_EVENT_INFO_TYPE(3i32);
pub const DEBUG_EVENT_LOAD_MODULE: u32 = 64u32;
pub const DEBUG_EVENT_SERVICE_EXCEPTION: u32 = 8192u32;
pub const DEBUG_EVENT_SESSION_STATUS: u32 = 512u32;
pub const DEBUG_EVENT_SYSTEM_ERROR: u32 = 256u32;
pub const DEBUG_EVENT_UNLOAD_MODULE: u32 = 128u32;
#[repr(C)]
pub struct DEBUG_EXCEPTION_FILTER_PARAMETERS(i32);
pub const DEBUG_EXECUTE_DEFAULT: u32 = 0u32;
pub const DEBUG_EXECUTE_ECHO: u32 = 1u32;
pub const DEBUG_EXECUTE_EVENT: u32 = 2048u32;
pub const DEBUG_EXECUTE_EXTENSION: u32 = 32u32;
pub const DEBUG_EXECUTE_HOTKEY: u32 = 1024u32;
pub const DEBUG_EXECUTE_INTERNAL: u32 = 64u32;
pub const DEBUG_EXECUTE_MENU: u32 = 512u32;
pub const DEBUG_EXECUTE_NOT_LOGGED: u32 = 2u32;
pub const DEBUG_EXECUTE_NO_REPEAT: u32 = 4u32;
pub const DEBUG_EXECUTE_SCRIPT: u32 = 128u32;
pub const DEBUG_EXECUTE_TOOLBAR: u32 = 256u32;
pub const DEBUG_EXECUTE_USER_CLICKED: u32 = 16u32;
pub const DEBUG_EXECUTE_USER_TYPED: u32 = 8u32;
pub const DEBUG_EXEC_FLAGS_NONBLOCK: u32 = 1u32;
pub const DEBUG_EXPR_CPLUSPLUS: u32 = 1u32;
pub const DEBUG_EXPR_MASM: u32 = 0u32;
pub const DEBUG_EXTENSION_AT_ENGINE: u32 = 0u32;
pub const DEBUG_EXTINIT_HAS_COMMAND_HELP: u32 = 1u32;
pub const DEBUG_EXT_PVALUE_DEFAULT: u32 = 0u32;
pub const DEBUG_EXT_PVTYPE_IS_POINTER: u32 = 1u32;
pub const DEBUG_EXT_PVTYPE_IS_VALUE: u32 = 0u32;
pub const DEBUG_EXT_QVALUE_DEFAULT: u32 = 0u32;
pub const DEBUG_FILTER_BREAK: u32 = 0u32;
pub const DEBUG_FILTER_CREATE_PROCESS: u32 = 2u32;
pub const DEBUG_FILTER_CREATE_THREAD: u32 = 0u32;
pub const DEBUG_FILTER_DEBUGGEE_OUTPUT: u32 = 9u32;
pub const DEBUG_FILTER_EXIT_PROCESS: u32 = 3u32;
pub const DEBUG_FILTER_EXIT_THREAD: u32 = 1u32;
pub const DEBUG_FILTER_GO_HANDLED: u32 = 0u32;
pub const DEBUG_FILTER_GO_NOT_HANDLED: u32 = 1u32;
pub const DEBUG_FILTER_IGNORE: u32 = 3u32;
pub const DEBUG_FILTER_INITIAL_BREAKPOINT: u32 = 7u32;
pub const DEBUG_FILTER_INITIAL_MODULE_LOAD: u32 = 8u32;
pub const DEBUG_FILTER_LOAD_MODULE: u32 = 4u32;
pub const DEBUG_FILTER_OUTPUT: u32 = 2u32;
pub const DEBUG_FILTER_REMOVE: u32 = 4u32;
pub const DEBUG_FILTER_SECOND_CHANCE_BREAK: u32 = 1u32;
pub const DEBUG_FILTER_SYSTEM_ERROR: u32 = 6u32;
pub const DEBUG_FILTER_UNLOAD_MODULE: u32 = 5u32;
pub const DEBUG_FIND_SOURCE_BEST_MATCH: u32 = 2u32;
pub const DEBUG_FIND_SOURCE_DEFAULT: u32 = 0u32;
pub const DEBUG_FIND_SOURCE_FULL_PATH: u32 = 1u32;
pub const DEBUG_FIND_SOURCE_NO_SRCSRV: u32 = 4u32;
pub const DEBUG_FIND_SOURCE_TOKEN_LOOKUP: u32 = 8u32;
pub const DEBUG_FIND_SOURCE_WITH_CHECKSUM: u32 = 16u32;
pub const DEBUG_FIND_SOURCE_WITH_CHECKSUM_STRICT: u32 = 32u32;
pub const DEBUG_FORMAT_CAB_SECONDARY_ALL_IMAGES: u32 = 268435456u32;
pub const DEBUG_FORMAT_CAB_SECONDARY_FILES: u32 = 1073741824u32;
pub const DEBUG_FORMAT_DEFAULT: u32 = 0u32;
pub const DEBUG_FORMAT_NO_OVERWRITE: u32 = 2147483648u32;
pub const DEBUG_FORMAT_USER_SMALL_ADD_AVX_XSTATE_CONTEXT: u32 = 131072u32;
pub const DEBUG_FORMAT_USER_SMALL_CODE_SEGMENTS: u32 = 4096u32;
pub const DEBUG_FORMAT_USER_SMALL_DATA_SEGMENTS: u32 = 16u32;
pub const DEBUG_FORMAT_USER_SMALL_FILTER_MEMORY: u32 = 32u32;
pub const DEBUG_FORMAT_USER_SMALL_FILTER_PATHS: u32 = 64u32;
pub const DEBUG_FORMAT_USER_SMALL_FILTER_TRIAGE: u32 = 65536u32;
pub const DEBUG_FORMAT_USER_SMALL_FULL_AUXILIARY_STATE: u32 = 16384u32;
pub const DEBUG_FORMAT_USER_SMALL_FULL_MEMORY: u32 = 1u32;
pub const DEBUG_FORMAT_USER_SMALL_FULL_MEMORY_INFO: u32 = 1024u32;
pub const DEBUG_FORMAT_USER_SMALL_HANDLE_DATA: u32 = 2u32;
pub const DEBUG_FORMAT_USER_SMALL_IGNORE_INACCESSIBLE_MEM: u32 = 134217728u32;
pub const DEBUG_FORMAT_USER_SMALL_INDIRECT_MEMORY: u32 = 8u32;
pub const DEBUG_FORMAT_USER_SMALL_IPT_TRACE: u32 = 262144u32;
pub const DEBUG_FORMAT_USER_SMALL_MODULE_HEADERS: u32 = 32768u32;
pub const DEBUG_FORMAT_USER_SMALL_NO_AUXILIARY_STATE: u32 = 8192u32;
pub const DEBUG_FORMAT_USER_SMALL_NO_OPTIONAL_DATA: u32 = 512u32;
pub const DEBUG_FORMAT_USER_SMALL_PRIVATE_READ_WRITE_MEMORY: u32 = 256u32;
pub const DEBUG_FORMAT_USER_SMALL_PROCESS_THREAD_DATA: u32 = 128u32;
pub const DEBUG_FORMAT_USER_SMALL_SCAN_PARTIAL_PAGES: u32 = 268435456u32;
pub const DEBUG_FORMAT_USER_SMALL_THREAD_INFO: u32 = 2048u32;
pub const DEBUG_FORMAT_USER_SMALL_UNLOADED_MODULES: u32 = 4u32;
pub const DEBUG_FORMAT_WRITE_CAB: u32 = 536870912u32;
pub const DEBUG_FRAME_DEFAULT: u32 = 0u32;
pub const DEBUG_FRAME_IGNORE_INLINE: u32 = 1u32;
pub const DEBUG_GETFNENT_DEFAULT: u32 = 0u32;
pub const DEBUG_GETFNENT_RAW_ENTRY_ONLY: u32 = 1u32;
pub const DEBUG_GETMOD_DEFAULT: u32 = 0u32;
pub const DEBUG_GETMOD_NO_LOADED_MODULES: u32 = 1u32;
pub const DEBUG_GETMOD_NO_UNLOADED_MODULES: u32 = 2u32;
pub const DEBUG_GET_PROC_DEFAULT: u32 = 0u32;
pub const DEBUG_GET_PROC_FULL_MATCH: u32 = 1u32;
pub const DEBUG_GET_PROC_ONLY_MATCH: u32 = 2u32;
pub const DEBUG_GET_PROC_SERVICE_NAME: u32 = 4u32;
#[repr(C)]
pub struct DEBUG_GET_TEXT_COMPLETIONS_IN(i32);
pub const DEBUG_GET_TEXT_COMPLETIONS_IS_DOT_COMMAND: u32 = 1u32;
pub const DEBUG_GET_TEXT_COMPLETIONS_IS_EXTENSION_COMMAND: u32 = 2u32;
pub const DEBUG_GET_TEXT_COMPLETIONS_IS_SYMBOL: u32 = 4u32;
pub const DEBUG_GET_TEXT_COMPLETIONS_NO_DOT_COMMANDS: u32 = 1u32;
pub const DEBUG_GET_TEXT_COMPLETIONS_NO_EXTENSION_COMMANDS: u32 = 2u32;
pub const DEBUG_GET_TEXT_COMPLETIONS_NO_SYMBOLS: u32 = 4u32;
#[repr(C)]
pub struct DEBUG_GET_TEXT_COMPLETIONS_OUT(i32);
pub const DEBUG_GSEL_ALLOW_HIGHER: u32 = 4u32;
pub const DEBUG_GSEL_ALLOW_LOWER: u32 = 2u32;
pub const DEBUG_GSEL_DEFAULT: u32 = 0u32;
pub const DEBUG_GSEL_INLINE_CALLSITE: u32 = 16u32;
pub const DEBUG_GSEL_NEAREST_ONLY: u32 = 8u32;
pub const DEBUG_GSEL_NO_SYMBOL_LOADS: u32 = 1u32;
#[repr(C)]
pub struct DEBUG_HANDLE_DATA_BASIC(i32);
pub const DEBUG_HANDLE_DATA_TYPE_ALL_HANDLE_OPERATIONS: u32 = 10u32;
pub const DEBUG_HANDLE_DATA_TYPE_BASIC: u32 = 0u32;
pub const DEBUG_HANDLE_DATA_TYPE_HANDLE_COUNT: u32 = 3u32;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_EVENT_1: u32 = 13u32;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_MUTANT_1: u32 = 7u32;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_MUTANT_2: u32 = 8u32;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_PROCESS_1: u32 = 11u32;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_PROCESS_2: u32 = 12u32;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_SECTION_1: u32 = 14u32;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_SEMAPHORE_1: u32 = 15u32;
pub const DEBUG_HANDLE_DATA_TYPE_MINI_THREAD_1: u32 = 6u32;
pub const DEBUG_HANDLE_DATA_TYPE_OBJECT_NAME: u32 = 2u32;
pub const DEBUG_HANDLE_DATA_TYPE_OBJECT_NAME_WIDE: u32 = 5u32;
pub const DEBUG_HANDLE_DATA_TYPE_PER_HANDLE_OPERATIONS: u32 = 9u32;
pub const DEBUG_HANDLE_DATA_TYPE_TYPE_NAME: u32 = 1u32;
pub const DEBUG_HANDLE_DATA_TYPE_TYPE_NAME_WIDE: u32 = 4u32;
pub const DEBUG_INTERRUPT_ACTIVE: u32 = 0u32;
pub const DEBUG_INTERRUPT_EXIT: u32 = 2u32;
pub const DEBUG_INTERRUPT_PASSIVE: u32 = 1u32;
pub const DEBUG_IOUTPUT_ADDR_TRANSLATE: u32 = 134217728u32;
pub const DEBUG_IOUTPUT_BREAKPOINT: u32 = 536870912u32;
pub const DEBUG_IOUTPUT_EVENT: u32 = 268435456u32;
pub const DEBUG_IOUTPUT_KD_PROTOCOL: u32 = 2147483648u32;
pub const DEBUG_IOUTPUT_REMOTING: u32 = 1073741824u32;
pub const DEBUG_KERNEL_ACTIVE_DUMP: u32 = 1030u32;
pub const DEBUG_KERNEL_CONNECTION: u32 = 0u32;
pub const DEBUG_KERNEL_DUMP: u32 = 1025u32;
pub const DEBUG_KERNEL_EXDI_DRIVER: u32 = 2u32;
pub const DEBUG_KERNEL_FULL_DUMP: u32 = 1026u32;
pub const DEBUG_KERNEL_IDNA: u32 = 3u32;
pub const DEBUG_KERNEL_INSTALL_DRIVER: u32 = 4u32;
pub const DEBUG_KERNEL_LOCAL: u32 = 1u32;
pub const DEBUG_KERNEL_REPT: u32 = 5u32;
pub const DEBUG_KERNEL_SMALL_DUMP: u32 = 1024u32;
pub const DEBUG_KERNEL_TRACE_LOG: u32 = 1028u32;
pub const DEBUG_KNOWN_STRUCT_GET_NAMES: u32 = 1u32;
pub const DEBUG_KNOWN_STRUCT_GET_SINGLE_LINE_OUTPUT: u32 = 2u32;
pub const DEBUG_KNOWN_STRUCT_SUPPRESS_TYPE_NAME: u32 = 3u32;
#[repr(C)]
pub struct DEBUG_LAST_EVENT_INFO_BREAKPOINT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DEBUG_LAST_EVENT_INFO_EXCEPTION(i32);
#[repr(C)]
pub struct DEBUG_LAST_EVENT_INFO_EXIT_PROCESS(i32);
#[repr(C)]
pub struct DEBUG_LAST_EVENT_INFO_EXIT_THREAD(i32);
#[repr(C)]
pub struct DEBUG_LAST_EVENT_INFO_LOAD_MODULE(i32);
#[repr(C)]
pub struct DEBUG_LAST_EVENT_INFO_SERVICE_EXCEPTION(i32);
#[repr(C)]
pub struct DEBUG_LAST_EVENT_INFO_SYSTEM_ERROR(i32);
#[repr(C)]
pub struct DEBUG_LAST_EVENT_INFO_UNLOAD_MODULE(i32);
pub const DEBUG_LEVEL_ASSEMBLY: u32 = 1u32;
pub const DEBUG_LEVEL_SOURCE: u32 = 0u32;
pub const DEBUG_LIVE_USER_NON_INVASIVE: u32 = 33u32;
pub const DEBUG_LOG_APPEND: u32 = 1u32;
pub const DEBUG_LOG_DEFAULT: u32 = 0u32;
pub const DEBUG_LOG_DML: u32 = 4u32;
pub const DEBUG_LOG_UNICODE: u32 = 2u32;
pub const DEBUG_MANAGED_ALLOWED: u32 = 1u32;
pub const DEBUG_MANAGED_DISABLED: u32 = 0u32;
pub const DEBUG_MANAGED_DLL_LOADED: u32 = 2u32;
pub const DEBUG_MANRESET_DEFAULT: u32 = 0u32;
pub const DEBUG_MANRESET_LOAD_DLL: u32 = 1u32;
pub const DEBUG_MANSTR_LOADED_SUPPORT_DLL: u32 = 1u32;
pub const DEBUG_MANSTR_LOAD_STATUS: u32 = 2u32;
pub const DEBUG_MANSTR_NONE: u32 = 0u32;
pub const DEBUG_MODNAME_IMAGE: u32 = 0u32;
pub const DEBUG_MODNAME_LOADED_IMAGE: u32 = 2u32;
pub const DEBUG_MODNAME_MAPPED_IMAGE: u32 = 4u32;
pub const DEBUG_MODNAME_MODULE: u32 = 1u32;
pub const DEBUG_MODNAME_SYMBOL_FILE: u32 = 3u32;
#[repr(C)]
pub struct DEBUG_MODULE_AND_ID(i32);
pub const DEBUG_MODULE_EXE_MODULE: u32 = 4u32;
pub const DEBUG_MODULE_EXPLICIT: u32 = 8u32;
pub const DEBUG_MODULE_LOADED: u32 = 0u32;
#[repr(C)]
pub struct DEBUG_MODULE_PARAMETERS(i32);
pub const DEBUG_MODULE_SECONDARY: u32 = 16u32;
pub const DEBUG_MODULE_SYM_BAD_CHECKSUM: u32 = 65536u32;
pub const DEBUG_MODULE_SYNTHETIC: u32 = 32u32;
pub const DEBUG_MODULE_UNLOADED: u32 = 1u32;
pub const DEBUG_MODULE_USER_MODE: u32 = 2u32;
pub const DEBUG_NOTIFY_SESSION_ACCESSIBLE: u32 = 2u32;
pub const DEBUG_NOTIFY_SESSION_ACTIVE: u32 = 0u32;
pub const DEBUG_NOTIFY_SESSION_INACCESSIBLE: u32 = 3u32;
pub const DEBUG_NOTIFY_SESSION_INACTIVE: u32 = 1u32;
#[repr(C)]
pub struct DEBUG_OFFSET_REGION(i32);
pub const DEBUG_OFFSINFO_VIRTUAL_SOURCE: u32 = 1u32;
pub const DEBUG_OUTCBF_COMBINED_EXPLICIT_FLUSH: u32 = 1u32;
pub const DEBUG_OUTCBF_DML_HAS_SPECIAL_CHARACTERS: u32 = 4u32;
pub const DEBUG_OUTCBF_DML_HAS_TAGS: u32 = 2u32;
pub const DEBUG_OUTCBI_ANY_FORMAT: u32 = 6u32;
pub const DEBUG_OUTCBI_DML: u32 = 4u32;
pub const DEBUG_OUTCBI_EXPLICIT_FLUSH: u32 = 1u32;
pub const DEBUG_OUTCBI_TEXT: u32 = 2u32;
pub const DEBUG_OUTCB_DML: u32 = 1u32;
pub const DEBUG_OUTCB_EXPLICIT_FLUSH: u32 = 2u32;
pub const DEBUG_OUTCB_TEXT: u32 = 0u32;
pub const DEBUG_OUTCTL_ALL_CLIENTS: u32 = 1u32;
pub const DEBUG_OUTCTL_ALL_OTHER_CLIENTS: u32 = 2u32;
pub const DEBUG_OUTCTL_AMBIENT: u32 = 4294967295u32;
pub const DEBUG_OUTCTL_AMBIENT_DML: u32 = 4294967294u32;
pub const DEBUG_OUTCTL_AMBIENT_TEXT: u32 = 4294967295u32;
pub const DEBUG_OUTCTL_DML: u32 = 32u32;
pub const DEBUG_OUTCTL_IGNORE: u32 = 3u32;
pub const DEBUG_OUTCTL_LOG_ONLY: u32 = 4u32;
pub const DEBUG_OUTCTL_NOT_LOGGED: u32 = 8u32;
pub const DEBUG_OUTCTL_OVERRIDE_MASK: u32 = 16u32;
pub const DEBUG_OUTCTL_SEND_MASK: u32 = 7u32;
pub const DEBUG_OUTCTL_THIS_CLIENT: u32 = 0u32;
pub const DEBUG_OUTPUT_DEBUGGEE: u32 = 128u32;
pub const DEBUG_OUTPUT_DEBUGGEE_PROMPT: u32 = 256u32;
pub const DEBUG_OUTPUT_ERROR: u32 = 2u32;
pub const DEBUG_OUTPUT_EXTENSION_WARNING: u32 = 64u32;
pub const DEBUG_OUTPUT_IDENTITY_DEFAULT: u32 = 0u32;
pub const DEBUG_OUTPUT_NORMAL: u32 = 1u32;
pub const DEBUG_OUTPUT_PROMPT: u32 = 16u32;
pub const DEBUG_OUTPUT_PROMPT_REGISTERS: u32 = 32u32;
pub const DEBUG_OUTPUT_STATUS: u32 = 1024u32;
pub const DEBUG_OUTPUT_SYMBOLS: u32 = 512u32;
pub const DEBUG_OUTPUT_SYMBOLS_DEFAULT: u32 = 0u32;
pub const DEBUG_OUTPUT_SYMBOLS_NO_NAMES: u32 = 1u32;
pub const DEBUG_OUTPUT_SYMBOLS_NO_OFFSETS: u32 = 2u32;
pub const DEBUG_OUTPUT_SYMBOLS_NO_TYPES: u32 = 16u32;
pub const DEBUG_OUTPUT_SYMBOLS_NO_VALUES: u32 = 4u32;
pub const DEBUG_OUTPUT_VERBOSE: u32 = 8u32;
pub const DEBUG_OUTPUT_WARNING: u32 = 4u32;
pub const DEBUG_OUTPUT_XML: u32 = 2048u32;
pub const DEBUG_OUTSYM_ALLOW_DISPLACEMENT: u32 = 4u32;
pub const DEBUG_OUTSYM_DEFAULT: u32 = 0u32;
pub const DEBUG_OUTSYM_FORCE_OFFSET: u32 = 1u32;
pub const DEBUG_OUTSYM_SOURCE_LINE: u32 = 2u32;
pub const DEBUG_OUTTYPE_ADDRESS_AT_END: u32 = 131072u32;
pub const DEBUG_OUTTYPE_ADDRESS_OF_FIELD: u32 = 65536u32;
pub const DEBUG_OUTTYPE_BLOCK_RECURSE: u32 = 2097152u32;
pub const DEBUG_OUTTYPE_COMPACT_OUTPUT: u32 = 8u32;
pub const DEBUG_OUTTYPE_DEFAULT: u32 = 0u32;
pub const DEBUG_OUTTYPE_NO_INDENT: u32 = 1u32;
pub const DEBUG_OUTTYPE_NO_OFFSET: u32 = 2u32;
pub const DEBUG_OUTTYPE_VERBOSE: u32 = 4u32;
pub const DEBUG_OUT_TEXT_REPL_DEFAULT: u32 = 0u32;
pub const DEBUG_PHYSICAL_CACHED: u32 = 1u32;
pub const DEBUG_PHYSICAL_DEFAULT: u32 = 0u32;
pub const DEBUG_PHYSICAL_UNCACHED: u32 = 2u32;
pub const DEBUG_PHYSICAL_WRITE_COMBINED: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DEBUG_PROCESSOR_IDENTIFICATION_ALL(i32);
#[repr(C)]
pub struct DEBUG_PROCESSOR_IDENTIFICATION_ALPHA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DEBUG_PROCESSOR_IDENTIFICATION_AMD64(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DEBUG_PROCESSOR_IDENTIFICATION_ARM(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DEBUG_PROCESSOR_IDENTIFICATION_ARM64(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DEBUG_PROCESSOR_IDENTIFICATION_IA64(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DEBUG_PROCESSOR_IDENTIFICATION_X86(i32);
pub const DEBUG_PROCESS_DETACH_ON_EXIT: u32 = 1u32;
pub const DEBUG_PROCESS_ONLY_THIS_PROCESS: u32 = 2u32;
pub const DEBUG_PROC_DESC_DEFAULT: u32 = 0u32;
pub const DEBUG_PROC_DESC_NO_COMMAND_LINE: u32 = 8u32;
pub const DEBUG_PROC_DESC_NO_MTS_PACKAGES: u32 = 4u32;
pub const DEBUG_PROC_DESC_NO_PATHS: u32 = 1u32;
pub const DEBUG_PROC_DESC_NO_SERVICES: u32 = 2u32;
pub const DEBUG_PROC_DESC_NO_SESSION_ID: u32 = 16u32;
pub const DEBUG_PROC_DESC_NO_USER_NAME: u32 = 32u32;
pub const DEBUG_PROC_DESC_WITH_PACKAGEFAMILY: u32 = 64u32;
#[repr(C)]
pub struct DEBUG_READ_USER_MINIDUMP_STREAM(i32);
pub const DEBUG_REGISTERS_ALL: u32 = 7u32;
pub const DEBUG_REGISTERS_DEFAULT: u32 = 0u32;
pub const DEBUG_REGISTERS_FLOAT: u32 = 4u32;
pub const DEBUG_REGISTERS_INT32: u32 = 1u32;
pub const DEBUG_REGISTERS_INT64: u32 = 2u32;
#[repr(C)]
pub struct DEBUG_REGISTER_DESCRIPTION(i32);
pub const DEBUG_REGISTER_SUB_REGISTER: u32 = 1u32;
pub const DEBUG_REGSRC_DEBUGGEE: u32 = 0u32;
pub const DEBUG_REGSRC_EXPLICIT: u32 = 1u32;
pub const DEBUG_REGSRC_FRAME: u32 = 2u32;
pub const DEBUG_REQUEST_ADD_CACHED_SYMBOL_INFO: u32 = 16u32;
pub const DEBUG_REQUEST_CLOSE_TOKEN: u32 = 30u32;
pub const DEBUG_REQUEST_CURRENT_OUTPUT_CALLBACKS_ARE_DML_AWARE: u32 = 19u32;
pub const DEBUG_REQUEST_DUPLICATE_TOKEN: u32 = 28u32;
pub const DEBUG_REQUEST_EXT_TYPED_DATA_ANSI: u32 = 12u32;
pub const DEBUG_REQUEST_GET_ADDITIONAL_CREATE_OPTIONS: u32 = 4u32;
pub const DEBUG_REQUEST_GET_CACHED_SYMBOL_INFO: u32 = 15u32;
pub const DEBUG_REQUEST_GET_CAPTURED_EVENT_CODE_OFFSET: u32 = 10u32;
pub const DEBUG_REQUEST_GET_DUMP_HEADER: u32 = 21u32;
pub const DEBUG_REQUEST_GET_EXTENSION_SEARCH_PATH_WIDE: u32 = 13u32;
pub const DEBUG_REQUEST_GET_INSTRUMENTATION_VERSION: u32 = 37u32;
pub const DEBUG_REQUEST_GET_MODULE_ARCHITECTURE: u32 = 38u32;
pub const DEBUG_REQUEST_GET_OFFSET_UNWIND_INFORMATION: u32 = 20u32;
pub const DEBUG_REQUEST_GET_TEXT_COMPLETIONS_ANSI: u32 = 18u32;
pub const DEBUG_REQUEST_GET_TEXT_COMPLETIONS_WIDE: u32 = 14u32;
pub const DEBUG_REQUEST_GET_WIN32_MAJOR_MINOR_VERSIONS: u32 = 6u32;
pub const DEBUG_REQUEST_INLINE_QUERY: u32 = 35u32;
pub const DEBUG_REQUEST_MIDORI: u32 = 23u32;
pub const DEBUG_REQUEST_MISC_INFORMATION: u32 = 25u32;
pub const DEBUG_REQUEST_OPEN_PROCESS_TOKEN: u32 = 26u32;
pub const DEBUG_REQUEST_OPEN_THREAD_TOKEN: u32 = 27u32;
pub const DEBUG_REQUEST_PROCESS_DESCRIPTORS: u32 = 24u32;
pub const DEBUG_REQUEST_QUERY_INFO_TOKEN: u32 = 29u32;
pub const DEBUG_REQUEST_READ_CAPTURED_EVENT_CODE_STREAM: u32 = 11u32;
pub const DEBUG_REQUEST_READ_USER_MINIDUMP_STREAM: u32 = 7u32;
pub const DEBUG_REQUEST_REMOVE_CACHED_SYMBOL_INFO: u32 = 17u32;
pub const DEBUG_REQUEST_RESUME_THREAD: u32 = 34u32;
pub const DEBUG_REQUEST_SET_ADDITIONAL_CREATE_OPTIONS: u32 = 5u32;
pub const DEBUG_REQUEST_SET_DUMP_HEADER: u32 = 22u32;
pub const DEBUG_REQUEST_SET_LOCAL_IMPLICIT_COMMAND_LINE: u32 = 9u32;
pub const DEBUG_REQUEST_SOURCE_PATH_HAS_SOURCE_SERVER: u32 = 0u32;
pub const DEBUG_REQUEST_TARGET_CAN_DETACH: u32 = 8u32;
pub const DEBUG_REQUEST_TARGET_EXCEPTION_CONTEXT: u32 = 1u32;
pub const DEBUG_REQUEST_TARGET_EXCEPTION_RECORD: u32 = 3u32;
pub const DEBUG_REQUEST_TARGET_EXCEPTION_THREAD: u32 = 2u32;
pub const DEBUG_REQUEST_TL_INSTRUMENTATION_AWARE: u32 = 36u32;
pub const DEBUG_REQUEST_WOW_MODULE: u32 = 32u32;
pub const DEBUG_REQUEST_WOW_PROCESS: u32 = 31u32;
pub const DEBUG_SCOPE_GROUP_ALL: u32 = 3u32;
pub const DEBUG_SCOPE_GROUP_ARGUMENTS: u32 = 1u32;
pub const DEBUG_SCOPE_GROUP_BY_DATAMODEL: u32 = 4u32;
pub const DEBUG_SCOPE_GROUP_LOCALS: u32 = 2u32;
pub const DEBUG_SERVERS_ALL: u32 = 3u32;
pub const DEBUG_SERVERS_DEBUGGER: u32 = 1u32;
pub const DEBUG_SERVERS_PROCESS: u32 = 2u32;
pub const DEBUG_SESSION_ACTIVE: u32 = 0u32;
pub const DEBUG_SESSION_END: u32 = 4u32;
pub const DEBUG_SESSION_END_SESSION_ACTIVE_DETACH: u32 = 2u32;
pub const DEBUG_SESSION_END_SESSION_ACTIVE_TERMINATE: u32 = 1u32;
pub const DEBUG_SESSION_END_SESSION_PASSIVE: u32 = 3u32;
pub const DEBUG_SESSION_FAILURE: u32 = 7u32;
pub const DEBUG_SESSION_HIBERNATE: u32 = 6u32;
pub const DEBUG_SESSION_REBOOT: u32 = 5u32;
pub const DEBUG_SOURCE_IS_STATEMENT: u32 = 1u32;
#[repr(C)]
pub struct DEBUG_SPECIFIC_FILTER_PARAMETERS(i32);
pub const DEBUG_SRCFILE_SYMBOL_CHECKSUMINFO: u32 = 2u32;
pub const DEBUG_SRCFILE_SYMBOL_TOKEN: u32 = 0u32;
pub const DEBUG_SRCFILE_SYMBOL_TOKEN_SOURCE_COMMAND_WIDE: u32 = 1u32;
#[repr(transparent)]
pub struct DEBUG_STACKFRAME_TYPE(pub i32);
pub const DST_SCRIPT_FRAME: DEBUG_STACKFRAME_TYPE = DEBUG_STACKFRAME_TYPE(0i32);
pub const DST_INTERNAL_FRAME: DEBUG_STACKFRAME_TYPE = DEBUG_STACKFRAME_TYPE(1i32);
pub const DST_INVOCATION_FRAME: DEBUG_STACKFRAME_TYPE = DEBUG_STACKFRAME_TYPE(2i32);
pub const DEBUG_STACK_ARGUMENTS: u32 = 1u32;
pub const DEBUG_STACK_COLUMN_NAMES: u32 = 16u32;
pub const DEBUG_STACK_DML: u32 = 2048u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DEBUG_STACK_FRAME(i32);
pub const DEBUG_STACK_FRAME_ADDRESSES: u32 = 8u32;
pub const DEBUG_STACK_FRAME_ADDRESSES_RA_ONLY: u32 = 256u32;
pub const DEBUG_STACK_FRAME_ARCH: u32 = 16384u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DEBUG_STACK_FRAME_EX(i32);
pub const DEBUG_STACK_FRAME_MEMORY_USAGE: u32 = 512u32;
pub const DEBUG_STACK_FRAME_NUMBERS: u32 = 64u32;
pub const DEBUG_STACK_FRAME_OFFSETS: u32 = 4096u32;
pub const DEBUG_STACK_FUNCTION_INFO: u32 = 2u32;
pub const DEBUG_STACK_NONVOLATILE_REGISTERS: u32 = 32u32;
pub const DEBUG_STACK_PARAMETERS: u32 = 128u32;
pub const DEBUG_STACK_PARAMETERS_NEWLINE: u32 = 1024u32;
pub const DEBUG_STACK_PROVIDER: u32 = 8192u32;
pub const DEBUG_STACK_SOURCE_LINE: u32 = 4u32;
pub const DEBUG_STATUS_BREAK: u32 = 6u32;
pub const DEBUG_STATUS_GO: u32 = 1u32;
pub const DEBUG_STATUS_GO_HANDLED: u32 = 2u32;
pub const DEBUG_STATUS_GO_NOT_HANDLED: u32 = 3u32;
pub const DEBUG_STATUS_IGNORE_EVENT: u32 = 9u32;
pub const DEBUG_STATUS_INSIDE_WAIT: u64 = 4294967296u64;
pub const DEBUG_STATUS_MASK: u32 = 31u32;
pub const DEBUG_STATUS_NO_CHANGE: u32 = 0u32;
pub const DEBUG_STATUS_NO_DEBUGGEE: u32 = 7u32;
pub const DEBUG_STATUS_OUT_OF_SYNC: u32 = 15u32;
pub const DEBUG_STATUS_RESTART_REQUESTED: u32 = 10u32;
pub const DEBUG_STATUS_REVERSE_GO: u32 = 11u32;
pub const DEBUG_STATUS_REVERSE_STEP_BRANCH: u32 = 12u32;
pub const DEBUG_STATUS_REVERSE_STEP_INTO: u32 = 14u32;
pub const DEBUG_STATUS_REVERSE_STEP_OVER: u32 = 13u32;
pub const DEBUG_STATUS_STEP_BRANCH: u32 = 8u32;
pub const DEBUG_STATUS_STEP_INTO: u32 = 5u32;
pub const DEBUG_STATUS_STEP_OVER: u32 = 4u32;
pub const DEBUG_STATUS_TIMEOUT: u32 = 17u32;
pub const DEBUG_STATUS_WAIT_INPUT: u32 = 16u32;
pub const DEBUG_STATUS_WAIT_TIMEOUT: u64 = 8589934592u64;
#[repr(C)]
pub struct DEBUG_SYMBOL_ENTRY(i32);
pub const DEBUG_SYMBOL_EXPANDED: u32 = 16u32;
pub const DEBUG_SYMBOL_EXPANSION_LEVEL_MASK: u32 = 15u32;
pub const DEBUG_SYMBOL_IS_ARGUMENT: u32 = 256u32;
pub const DEBUG_SYMBOL_IS_ARRAY: u32 = 64u32;
pub const DEBUG_SYMBOL_IS_FLOAT: u32 = 128u32;
pub const DEBUG_SYMBOL_IS_LOCAL: u32 = 512u32;
#[repr(C)]
pub struct DEBUG_SYMBOL_PARAMETERS(i32);
pub const DEBUG_SYMBOL_READ_ONLY: u32 = 32u32;
#[repr(C)]
pub struct DEBUG_SYMBOL_SOURCE_ENTRY(i32);
pub const DEBUG_SYMENT_IS_CODE: u32 = 1u32;
pub const DEBUG_SYMENT_IS_DATA: u32 = 2u32;
pub const DEBUG_SYMENT_IS_LOCAL: u32 = 8u32;
pub const DEBUG_SYMENT_IS_MANAGED: u32 = 16u32;
pub const DEBUG_SYMENT_IS_PARAMETER: u32 = 4u32;
pub const DEBUG_SYMENT_IS_SYNTHETIC: u32 = 32u32;
pub const DEBUG_SYMINFO_BREAKPOINT_SOURCE_LINE: u32 = 0u32;
pub const DEBUG_SYMINFO_GET_MODULE_SYMBOL_NAMES_AND_OFFSETS: u32 = 3u32;
pub const DEBUG_SYMINFO_GET_SYMBOL_NAME_BY_OFFSET_AND_TAG_WIDE: u32 = 2u32;
pub const DEBUG_SYMINFO_IMAGEHLP_MODULEW64: u32 = 1u32;
pub const DEBUG_SYMTYPE_CODEVIEW: u32 = 2u32;
pub const DEBUG_SYMTYPE_COFF: u32 = 1u32;
pub const DEBUG_SYMTYPE_DEFERRED: u32 = 5u32;
pub const DEBUG_SYMTYPE_DIA: u32 = 7u32;
pub const DEBUG_SYMTYPE_EXPORT: u32 = 4u32;
pub const DEBUG_SYMTYPE_NONE: u32 = 0u32;
pub const DEBUG_SYMTYPE_PDB: u32 = 3u32;
pub const DEBUG_SYMTYPE_SYM: u32 = 6u32;
pub const DEBUG_SYSOBJINFO_CURRENT_PROCESS_COOKIE: u32 = 2u32;
pub const DEBUG_SYSOBJINFO_THREAD_BASIC_INFORMATION: u32 = 0u32;
pub const DEBUG_SYSOBJINFO_THREAD_NAME_WIDE: u32 = 1u32;
pub const DEBUG_SYSVERSTR_BUILD: u32 = 1u32;
pub const DEBUG_SYSVERSTR_SERVICE_PACK: u32 = 0u32;
pub const DEBUG_TBINFO_AFFINITY: u32 = 32u32;
pub const DEBUG_TBINFO_ALL: u32 = 63u32;
pub const DEBUG_TBINFO_EXIT_STATUS: u32 = 1u32;
pub const DEBUG_TBINFO_PRIORITY: u32 = 4u32;
pub const DEBUG_TBINFO_PRIORITY_CLASS: u32 = 2u32;
pub const DEBUG_TBINFO_START_OFFSET: u32 = 16u32;
pub const DEBUG_TBINFO_TIMES: u32 = 8u32;
pub const DEBUG_TEXT_ALLOWBREAKPOINTS: u32 = 8u32;
pub const DEBUG_TEXT_ALLOWERRORREPORT: u32 = 16u32;
pub const DEBUG_TEXT_EVALUATETOCODECONTEXT: u32 = 32u32;
pub const DEBUG_TEXT_ISEXPRESSION: u32 = 1u32;
pub const DEBUG_TEXT_ISNONUSERCODE: u32 = 64u32;
pub const DEBUG_TEXT_NOSIDEEFFECTS: u32 = 4u32;
pub const DEBUG_TEXT_RETURNVALUE: u32 = 2u32;
#[repr(C)]
pub struct DEBUG_THREAD_BASIC_INFORMATION(i32);
#[repr(C)]
pub struct DEBUG_TYPED_DATA(i32);
pub const DEBUG_TYPED_DATA_IS_IN_MEMORY: u32 = 1u32;
pub const DEBUG_TYPED_DATA_PHYSICAL_CACHED: u32 = 4u32;
pub const DEBUG_TYPED_DATA_PHYSICAL_DEFAULT: u32 = 2u32;
pub const DEBUG_TYPED_DATA_PHYSICAL_MEMORY: u32 = 14u32;
pub const DEBUG_TYPED_DATA_PHYSICAL_UNCACHED: u32 = 6u32;
pub const DEBUG_TYPED_DATA_PHYSICAL_WRITE_COMBINED: u32 = 8u32;
pub const DEBUG_TYPEOPTS_FORCERADIX_OUTPUT: u32 = 4u32;
pub const DEBUG_TYPEOPTS_LONGSTATUS_DISPLAY: u32 = 2u32;
pub const DEBUG_TYPEOPTS_MATCH_MAXSIZE: u32 = 8u32;
pub const DEBUG_TYPEOPTS_UNICODE_DISPLAY: u32 = 1u32;
pub const DEBUG_USER_WINDOWS_DUMP: u32 = 1025u32;
pub const DEBUG_USER_WINDOWS_DUMP_WINDOWS_CE: u32 = 1029u32;
pub const DEBUG_USER_WINDOWS_IDNA: u32 = 2u32;
pub const DEBUG_USER_WINDOWS_PROCESS: u32 = 0u32;
pub const DEBUG_USER_WINDOWS_PROCESS_SERVER: u32 = 1u32;
pub const DEBUG_USER_WINDOWS_REPT: u32 = 3u32;
pub const DEBUG_USER_WINDOWS_SMALL_DUMP: u32 = 1024u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DEBUG_VALUE(i32);
pub const DEBUG_VALUE_FLOAT128: u32 = 9u32;
pub const DEBUG_VALUE_FLOAT32: u32 = 5u32;
pub const DEBUG_VALUE_FLOAT64: u32 = 6u32;
pub const DEBUG_VALUE_FLOAT80: u32 = 7u32;
pub const DEBUG_VALUE_FLOAT82: u32 = 8u32;
pub const DEBUG_VALUE_INT16: u32 = 2u32;
pub const DEBUG_VALUE_INT32: u32 = 3u32;
pub const DEBUG_VALUE_INT64: u32 = 4u32;
pub const DEBUG_VALUE_INT8: u32 = 1u32;
pub const DEBUG_VALUE_INVALID: u32 = 0u32;
pub const DEBUG_VALUE_TYPES: u32 = 12u32;
pub const DEBUG_VALUE_VECTOR128: u32 = 11u32;
pub const DEBUG_VALUE_VECTOR64: u32 = 10u32;
pub const DEBUG_VSEARCH_DEFAULT: u32 = 0u32;
pub const DEBUG_VSEARCH_WRITABLE_ONLY: u32 = 1u32;
pub const DEBUG_VSOURCE_DEBUGGEE: u32 = 1u32;
pub const DEBUG_VSOURCE_DUMP_WITHOUT_MEMINFO: u32 = 3u32;
pub const DEBUG_VSOURCE_INVALID: u32 = 0u32;
pub const DEBUG_VSOURCE_MAPPED_IMAGE: u32 = 2u32;
pub const DEBUG_WAIT_DEFAULT: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
pub type DIGEST_FUNCTION = unsafe extern "system" fn(refdata: *mut ::core::ffi::c_void, pdata: *mut u8, dwlength: u32) -> super::super::super::Foundation::BOOL;
#[cfg(any(target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct DISPATCHER_CONTEXT(i32);
#[cfg(any(target_arch = "x86_64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct DISPATCHER_CONTEXT(i32);
pub const DMP_CONTEXT_RECORD_SIZE_32: u32 = 1200u32;
pub const DMP_CONTEXT_RECORD_SIZE_64: u32 = 3000u32;
pub const DMP_HEADER_COMMENT_SIZE: u32 = 128u32;
pub const DMP_PHYSICAL_MEMORY_BLOCK_SIZE_32: u32 = 700u32;
pub const DMP_PHYSICAL_MEMORY_BLOCK_SIZE_64: u32 = 700u32;
pub const DMP_RESERVED_0_SIZE_32: u32 = 1760u32;
pub const DMP_RESERVED_0_SIZE_64: u32 = 4008u32;
pub const DMP_RESERVED_2_SIZE_32: u32 = 16u32;
pub const DMP_RESERVED_3_SIZE_32: u32 = 56u32;
#[repr(transparent)]
pub struct DOCUMENTNAMETYPE(pub i32);
pub const DOCUMENTNAMETYPE_APPNODE: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(0i32);
pub const DOCUMENTNAMETYPE_TITLE: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(1i32);
pub const DOCUMENTNAMETYPE_FILE_TAIL: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(2i32);
pub const DOCUMENTNAMETYPE_URL: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(3i32);
pub const DOCUMENTNAMETYPE_UNIQUE_TITLE: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(4i32);
pub const DOCUMENTNAMETYPE_SOURCE_MAP_URL: DOCUMENTNAMETYPE = DOCUMENTNAMETYPE(5i32);
pub const DSLFLAG_MISMATCHED_DBG: u32 = 2u32;
pub const DSLFLAG_MISMATCHED_PDB: u32 = 1u32;
#[repr(C)]
pub struct DUMP_FILE_ATTRIBUTES(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DUMP_HEADER32(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DUMP_HEADER64(i32);
pub const DUMP_SUMMARY_VALID_CURRENT_USER_VA: u32 = 2u32;
pub const DUMP_SUMMARY_VALID_KERNEL_VA: u32 = 1u32;
#[repr(transparent)]
pub struct DebugBaseEventCallbacks(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DebugBaseEventCallbacksWide(pub *mut ::core::ffi::c_void);
pub const DebugHelper: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 201113696, data2: 35869, data3: 4560, data4: [172, 205, 0, 170, 0, 96, 39, 92] };
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DebugPropertyInfo(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DebugStackFrameDescriptor(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DebugStackFrameDescriptor64(i32);
pub const DefaultDebugSessionProvider: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2202085538, data2: 20980, data3: 4560, data4: [143, 32, 0, 128, 95, 44, 208, 100] };
#[repr(transparent)]
pub struct ERRORRESUMEACTION(pub i32);
pub const ERRORRESUMEACTION_ReexecuteErrorStatement: ERRORRESUMEACTION = ERRORRESUMEACTION(0i32);
pub const ERRORRESUMEACTION_AbortCallAndReturnErrorToCaller: ERRORRESUMEACTION = ERRORRESUMEACTION(1i32);
pub const ERRORRESUMEACTION_SkipErrorStatement: ERRORRESUMEACTION = ERRORRESUMEACTION(2i32);
pub const ERROR_DBG_CANCELLED: u32 = 3221226695u32;
pub const ERROR_DBG_TIMEOUT: u32 = 3221226932u32;
pub const ERROR_IMAGE_NOT_STRIPPED: u32 = 34816u32;
pub const ERROR_NO_DBG_POINTER: u32 = 34817u32;
pub const ERROR_NO_PDB_POINTER: u32 = 34818u32;
pub const ESLFLAG_FULLPATH: u32 = 1u32;
pub const ESLFLAG_INLINE_SITE: u32 = 16u32;
pub const ESLFLAG_NEAREST: u32 = 2u32;
pub const ESLFLAG_NEXT: u32 = 8u32;
pub const ESLFLAG_PREV: u32 = 4u32;
pub const EVENT_SRCSPEW: u32 = 100u32;
pub const EVENT_SRCSPEW_END: u32 = 199u32;
pub const EVENT_SRCSPEW_START: u32 = 100u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct EXCEPTION_DEBUG_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct EXCEPTION_POINTERS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct EXCEPTION_RECORD(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct EXCEPTION_RECORD32(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct EXCEPTION_RECORD64(i32);
pub const EXIT_ON_CONTROLC: u32 = 8u32;
#[repr(C)]
pub struct EXIT_PROCESS_DEBUG_INFO(i32);
#[repr(C)]
pub struct EXIT_THREAD_DEBUG_INFO(i32);
#[repr(C)]
pub struct EXTSTACKTRACE(i32);
#[repr(C)]
pub struct EXTSTACKTRACE32(i32);
#[repr(C)]
pub struct EXTSTACKTRACE64(i32);
#[repr(C)]
pub struct EXT_API_VERSION(i32);
pub const EXT_API_VERSION_NUMBER: u32 = 5u32;
pub const EXT_API_VERSION_NUMBER32: u32 = 5u32;
pub const EXT_API_VERSION_NUMBER64: u32 = 6u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct EXT_FIND_FILE(i32);
pub const EXT_FIND_FILE_ALLOW_GIVEN_PATH: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct EXT_MATCH_PATTERN_A(i32);
pub const EXT_OUTPUT_VER: u32 = 1u32;
pub const EXT_TDF_PHYSICAL_CACHED: u32 = 4u32;
pub const EXT_TDF_PHYSICAL_DEFAULT: u32 = 2u32;
pub const EXT_TDF_PHYSICAL_MEMORY: u32 = 14u32;
pub const EXT_TDF_PHYSICAL_UNCACHED: u32 = 6u32;
pub const EXT_TDF_PHYSICAL_WRITE_COMBINED: u32 = 8u32;
#[repr(transparent)]
pub struct EXT_TDOP(pub i32);
pub const EXT_TDOP_COPY: EXT_TDOP = EXT_TDOP(0i32);
pub const EXT_TDOP_RELEASE: EXT_TDOP = EXT_TDOP(1i32);
pub const EXT_TDOP_SET_FROM_EXPR: EXT_TDOP = EXT_TDOP(2i32);
pub const EXT_TDOP_SET_FROM_U64_EXPR: EXT_TDOP = EXT_TDOP(3i32);
pub const EXT_TDOP_GET_FIELD: EXT_TDOP = EXT_TDOP(4i32);
pub const EXT_TDOP_EVALUATE: EXT_TDOP = EXT_TDOP(5i32);
pub const EXT_TDOP_GET_TYPE_NAME: EXT_TDOP = EXT_TDOP(6i32);
pub const EXT_TDOP_OUTPUT_TYPE_NAME: EXT_TDOP = EXT_TDOP(7i32);
pub const EXT_TDOP_OUTPUT_SIMPLE_VALUE: EXT_TDOP = EXT_TDOP(8i32);
pub const EXT_TDOP_OUTPUT_FULL_VALUE: EXT_TDOP = EXT_TDOP(9i32);
pub const EXT_TDOP_HAS_FIELD: EXT_TDOP = EXT_TDOP(10i32);
pub const EXT_TDOP_GET_FIELD_OFFSET: EXT_TDOP = EXT_TDOP(11i32);
pub const EXT_TDOP_GET_ARRAY_ELEMENT: EXT_TDOP = EXT_TDOP(12i32);
pub const EXT_TDOP_GET_DEREFERENCE: EXT_TDOP = EXT_TDOP(13i32);
pub const EXT_TDOP_GET_TYPE_SIZE: EXT_TDOP = EXT_TDOP(14i32);
pub const EXT_TDOP_OUTPUT_TYPE_DEFINITION: EXT_TDOP = EXT_TDOP(15i32);
pub const EXT_TDOP_GET_POINTER_TO: EXT_TDOP = EXT_TDOP(16i32);
pub const EXT_TDOP_SET_FROM_TYPE_ID_AND_U64: EXT_TDOP = EXT_TDOP(17i32);
pub const EXT_TDOP_SET_PTR_FROM_TYPE_ID_AND_U64: EXT_TDOP = EXT_TDOP(18i32);
pub const EXT_TDOP_COUNT: EXT_TDOP = EXT_TDOP(19i32);
#[repr(C)]
pub struct EXT_TYPED_DATA(i32);
#[repr(transparent)]
pub struct EX_PROP_INFO_FLAGS(pub i32);
pub const EX_PROP_INFO_ID: EX_PROP_INFO_FLAGS = EX_PROP_INFO_FLAGS(256i32);
pub const EX_PROP_INFO_NTYPE: EX_PROP_INFO_FLAGS = EX_PROP_INFO_FLAGS(512i32);
pub const EX_PROP_INFO_NVALUE: EX_PROP_INFO_FLAGS = EX_PROP_INFO_FLAGS(1024i32);
pub const EX_PROP_INFO_LOCKBYTES: EX_PROP_INFO_FLAGS = EX_PROP_INFO_FLAGS(2048i32);
pub const EX_PROP_INFO_DEBUGEXTPROP: EX_PROP_INFO_FLAGS = EX_PROP_INFO_FLAGS(4096i32);
pub const E_JsDEBUG_INVALID_MEMORY_ADDRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1916338171i32 as _);
pub const E_JsDEBUG_MISMATCHED_RUNTIME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1916338175i32 as _);
pub const E_JsDEBUG_OUTSIDE_OF_VM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1916338172i32 as _);
pub const E_JsDEBUG_RUNTIME_NOT_IN_DEBUG_MODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1916338169i32 as _);
pub const E_JsDEBUG_SOURCE_LOCATION_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1916338170i32 as _);
pub const E_JsDEBUG_UNKNOWN_THREAD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1916338174i32 as _);
#[repr(transparent)]
pub struct ErrorClass(pub i32);
pub const ErrorClassWarning: ErrorClass = ErrorClass(0i32);
pub const ErrorClassError: ErrorClass = ErrorClass(1i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
#[repr(C)]
pub struct ExtendedDebugPropertyInfo(i32);
#[repr(transparent)]
pub struct FACILITY_CODE(pub u32);
pub const FACILITY_NULL: FACILITY_CODE = FACILITY_CODE(0u32);
pub const FACILITY_RPC: FACILITY_CODE = FACILITY_CODE(1u32);
pub const FACILITY_DISPATCH: FACILITY_CODE = FACILITY_CODE(2u32);
pub const FACILITY_STORAGE: FACILITY_CODE = FACILITY_CODE(3u32);
pub const FACILITY_ITF: FACILITY_CODE = FACILITY_CODE(4u32);
pub const FACILITY_WIN32: FACILITY_CODE = FACILITY_CODE(7u32);
pub const FACILITY_WINDOWS: FACILITY_CODE = FACILITY_CODE(8u32);
pub const FACILITY_SSPI: FACILITY_CODE = FACILITY_CODE(9u32);
pub const FACILITY_SECURITY: FACILITY_CODE = FACILITY_CODE(9u32);
pub const FACILITY_CONTROL: FACILITY_CODE = FACILITY_CODE(10u32);
pub const FACILITY_CERT: FACILITY_CODE = FACILITY_CODE(11u32);
pub const FACILITY_INTERNET: FACILITY_CODE = FACILITY_CODE(12u32);
pub const FACILITY_MEDIASERVER: FACILITY_CODE = FACILITY_CODE(13u32);
pub const FACILITY_MSMQ: FACILITY_CODE = FACILITY_CODE(14u32);
pub const FACILITY_SETUPAPI: FACILITY_CODE = FACILITY_CODE(15u32);
pub const FACILITY_SCARD: FACILITY_CODE = FACILITY_CODE(16u32);
pub const FACILITY_COMPLUS: FACILITY_CODE = FACILITY_CODE(17u32);
pub const FACILITY_AAF: FACILITY_CODE = FACILITY_CODE(18u32);
pub const FACILITY_URT: FACILITY_CODE = FACILITY_CODE(19u32);
pub const FACILITY_ACS: FACILITY_CODE = FACILITY_CODE(20u32);
pub const FACILITY_DPLAY: FACILITY_CODE = FACILITY_CODE(21u32);
pub const FACILITY_UMI: FACILITY_CODE = FACILITY_CODE(22u32);
pub const FACILITY_SXS: FACILITY_CODE = FACILITY_CODE(23u32);
pub const FACILITY_WINDOWS_CE: FACILITY_CODE = FACILITY_CODE(24u32);
pub const FACILITY_HTTP: FACILITY_CODE = FACILITY_CODE(25u32);
pub const FACILITY_USERMODE_COMMONLOG: FACILITY_CODE = FACILITY_CODE(26u32);
pub const FACILITY_WER: FACILITY_CODE = FACILITY_CODE(27u32);
pub const FACILITY_USERMODE_FILTER_MANAGER: FACILITY_CODE = FACILITY_CODE(31u32);
pub const FACILITY_BACKGROUNDCOPY: FACILITY_CODE = FACILITY_CODE(32u32);
pub const FACILITY_CONFIGURATION: FACILITY_CODE = FACILITY_CODE(33u32);
pub const FACILITY_WIA: FACILITY_CODE = FACILITY_CODE(33u32);
pub const FACILITY_STATE_MANAGEMENT: FACILITY_CODE = FACILITY_CODE(34u32);
pub const FACILITY_METADIRECTORY: FACILITY_CODE = FACILITY_CODE(35u32);
pub const FACILITY_WINDOWSUPDATE: FACILITY_CODE = FACILITY_CODE(36u32);
pub const FACILITY_DIRECTORYSERVICE: FACILITY_CODE = FACILITY_CODE(37u32);
pub const FACILITY_GRAPHICS: FACILITY_CODE = FACILITY_CODE(38u32);
pub const FACILITY_SHELL: FACILITY_CODE = FACILITY_CODE(39u32);
pub const FACILITY_NAP: FACILITY_CODE = FACILITY_CODE(39u32);
pub const FACILITY_TPM_SERVICES: FACILITY_CODE = FACILITY_CODE(40u32);
pub const FACILITY_TPM_SOFTWARE: FACILITY_CODE = FACILITY_CODE(41u32);
pub const FACILITY_UI: FACILITY_CODE = FACILITY_CODE(42u32);
pub const FACILITY_XAML: FACILITY_CODE = FACILITY_CODE(43u32);
pub const FACILITY_ACTION_QUEUE: FACILITY_CODE = FACILITY_CODE(44u32);
pub const FACILITY_PLA: FACILITY_CODE = FACILITY_CODE(48u32);
pub const FACILITY_WINDOWS_SETUP: FACILITY_CODE = FACILITY_CODE(48u32);
pub const FACILITY_FVE: FACILITY_CODE = FACILITY_CODE(49u32);
pub const FACILITY_FWP: FACILITY_CODE = FACILITY_CODE(50u32);
pub const FACILITY_WINRM: FACILITY_CODE = FACILITY_CODE(51u32);
pub const FACILITY_NDIS: FACILITY_CODE = FACILITY_CODE(52u32);
pub const FACILITY_USERMODE_HYPERVISOR: FACILITY_CODE = FACILITY_CODE(53u32);
pub const FACILITY_CMI: FACILITY_CODE = FACILITY_CODE(54u32);
pub const FACILITY_USERMODE_VIRTUALIZATION: FACILITY_CODE = FACILITY_CODE(55u32);
pub const FACILITY_USERMODE_VOLMGR: FACILITY_CODE = FACILITY_CODE(56u32);
pub const FACILITY_BCD: FACILITY_CODE = FACILITY_CODE(57u32);
pub const FACILITY_USERMODE_VHD: FACILITY_CODE = FACILITY_CODE(58u32);
pub const FACILITY_USERMODE_HNS: FACILITY_CODE = FACILITY_CODE(59u32);
pub const FACILITY_SDIAG: FACILITY_CODE = FACILITY_CODE(60u32);
pub const FACILITY_WEBSERVICES: FACILITY_CODE = FACILITY_CODE(61u32);
pub const FACILITY_WINPE: FACILITY_CODE = FACILITY_CODE(61u32);
pub const FACILITY_WPN: FACILITY_CODE = FACILITY_CODE(62u32);
pub const FACILITY_WINDOWS_STORE: FACILITY_CODE = FACILITY_CODE(63u32);
pub const FACILITY_INPUT: FACILITY_CODE = FACILITY_CODE(64u32);
pub const FACILITY_QUIC: FACILITY_CODE = FACILITY_CODE(65u32);
pub const FACILITY_EAP: FACILITY_CODE = FACILITY_CODE(66u32);
pub const FACILITY_IORING: FACILITY_CODE = FACILITY_CODE(70u32);
pub const FACILITY_WINDOWS_DEFENDER: FACILITY_CODE = FACILITY_CODE(80u32);
pub const FACILITY_OPC: FACILITY_CODE = FACILITY_CODE(81u32);
pub const FACILITY_XPS: FACILITY_CODE = FACILITY_CODE(82u32);
pub const FACILITY_MBN: FACILITY_CODE = FACILITY_CODE(84u32);
pub const FACILITY_POWERSHELL: FACILITY_CODE = FACILITY_CODE(84u32);
pub const FACILITY_RAS: FACILITY_CODE = FACILITY_CODE(83u32);
pub const FACILITY_P2P_INT: FACILITY_CODE = FACILITY_CODE(98u32);
pub const FACILITY_P2P: FACILITY_CODE = FACILITY_CODE(99u32);
pub const FACILITY_DAF: FACILITY_CODE = FACILITY_CODE(100u32);
pub const FACILITY_BLUETOOTH_ATT: FACILITY_CODE = FACILITY_CODE(101u32);
pub const FACILITY_AUDIO: FACILITY_CODE = FACILITY_CODE(102u32);
pub const FACILITY_STATEREPOSITORY: FACILITY_CODE = FACILITY_CODE(103u32);
pub const FACILITY_VISUALCPP: FACILITY_CODE = FACILITY_CODE(109u32);
pub const FACILITY_SCRIPT: FACILITY_CODE = FACILITY_CODE(112u32);
pub const FACILITY_PARSE: FACILITY_CODE = FACILITY_CODE(113u32);
pub const FACILITY_BLB: FACILITY_CODE = FACILITY_CODE(120u32);
pub const FACILITY_BLB_CLI: FACILITY_CODE = FACILITY_CODE(121u32);
pub const FACILITY_WSBAPP: FACILITY_CODE = FACILITY_CODE(122u32);
pub const FACILITY_BLBUI: FACILITY_CODE = FACILITY_CODE(128u32);
pub const FACILITY_USN: FACILITY_CODE = FACILITY_CODE(129u32);
pub const FACILITY_USERMODE_VOLSNAP: FACILITY_CODE = FACILITY_CODE(130u32);
pub const FACILITY_TIERING: FACILITY_CODE = FACILITY_CODE(131u32);
pub const FACILITY_WSB_ONLINE: FACILITY_CODE = FACILITY_CODE(133u32);
pub const FACILITY_ONLINE_ID: FACILITY_CODE = FACILITY_CODE(134u32);
pub const FACILITY_DEVICE_UPDATE_AGENT: FACILITY_CODE = FACILITY_CODE(135u32);
pub const FACILITY_DRVSERVICING: FACILITY_CODE = FACILITY_CODE(136u32);
pub const FACILITY_DLS: FACILITY_CODE = FACILITY_CODE(153u32);
pub const FACILITY_DELIVERY_OPTIMIZATION: FACILITY_CODE = FACILITY_CODE(208u32);
pub const FACILITY_USERMODE_SPACES: FACILITY_CODE = FACILITY_CODE(231u32);
pub const FACILITY_USER_MODE_SECURITY_CORE: FACILITY_CODE = FACILITY_CODE(232u32);
pub const FACILITY_USERMODE_LICENSING: FACILITY_CODE = FACILITY_CODE(234u32);
pub const FACILITY_SOS: FACILITY_CODE = FACILITY_CODE(160u32);
pub const FACILITY_OCP_UPDATE_AGENT: FACILITY_CODE = FACILITY_CODE(173u32);
pub const FACILITY_DEBUGGERS: FACILITY_CODE = FACILITY_CODE(176u32);
pub const FACILITY_SPP: FACILITY_CODE = FACILITY_CODE(256u32);
pub const FACILITY_RESTORE: FACILITY_CODE = FACILITY_CODE(256u32);
pub const FACILITY_DMSERVER: FACILITY_CODE = FACILITY_CODE(256u32);
pub const FACILITY_DEPLOYMENT_SERVICES_SERVER: FACILITY_CODE = FACILITY_CODE(257u32);
pub const FACILITY_DEPLOYMENT_SERVICES_IMAGING: FACILITY_CODE = FACILITY_CODE(258u32);
pub const FACILITY_DEPLOYMENT_SERVICES_MANAGEMENT: FACILITY_CODE = FACILITY_CODE(259u32);
pub const FACILITY_DEPLOYMENT_SERVICES_UTIL: FACILITY_CODE = FACILITY_CODE(260u32);
pub const FACILITY_DEPLOYMENT_SERVICES_BINLSVC: FACILITY_CODE = FACILITY_CODE(261u32);
pub const FACILITY_DEPLOYMENT_SERVICES_PXE: FACILITY_CODE = FACILITY_CODE(263u32);
pub const FACILITY_DEPLOYMENT_SERVICES_TFTP: FACILITY_CODE = FACILITY_CODE(264u32);
pub const FACILITY_DEPLOYMENT_SERVICES_TRANSPORT_MANAGEMENT: FACILITY_CODE = FACILITY_CODE(272u32);
pub const FACILITY_DEPLOYMENT_SERVICES_DRIVER_PROVISIONING: FACILITY_CODE = FACILITY_CODE(278u32);
pub const FACILITY_DEPLOYMENT_SERVICES_MULTICAST_SERVER: FACILITY_CODE = FACILITY_CODE(289u32);
pub const FACILITY_DEPLOYMENT_SERVICES_MULTICAST_CLIENT: FACILITY_CODE = FACILITY_CODE(290u32);
pub const FACILITY_DEPLOYMENT_SERVICES_CONTENT_PROVIDER: FACILITY_CODE = FACILITY_CODE(293u32);
pub const FACILITY_HSP_SERVICES: FACILITY_CODE = FACILITY_CODE(296u32);
pub const FACILITY_HSP_SOFTWARE: FACILITY_CODE = FACILITY_CODE(297u32);
pub const FACILITY_LINGUISTIC_SERVICES: FACILITY_CODE = FACILITY_CODE(305u32);
pub const FACILITY_AUDIOSTREAMING: FACILITY_CODE = FACILITY_CODE(1094u32);
pub const FACILITY_TTD: FACILITY_CODE = FACILITY_CODE(1490u32);
pub const FACILITY_ACCELERATOR: FACILITY_CODE = FACILITY_CODE(1536u32);
pub const FACILITY_WMAAECMA: FACILITY_CODE = FACILITY_CODE(1996u32);
pub const FACILITY_DIRECTMUSIC: FACILITY_CODE = FACILITY_CODE(2168u32);
pub const FACILITY_DIRECT3D10: FACILITY_CODE = FACILITY_CODE(2169u32);
pub const FACILITY_DXGI: FACILITY_CODE = FACILITY_CODE(2170u32);
pub const FACILITY_DXGI_DDI: FACILITY_CODE = FACILITY_CODE(2171u32);
pub const FACILITY_DIRECT3D11: FACILITY_CODE = FACILITY_CODE(2172u32);
pub const FACILITY_DIRECT3D11_DEBUG: FACILITY_CODE = FACILITY_CODE(2173u32);
pub const FACILITY_DIRECT3D12: FACILITY_CODE = FACILITY_CODE(2174u32);
pub const FACILITY_DIRECT3D12_DEBUG: FACILITY_CODE = FACILITY_CODE(2175u32);
pub const FACILITY_DXCORE: FACILITY_CODE = FACILITY_CODE(2176u32);
pub const FACILITY_PRESENTATION: FACILITY_CODE = FACILITY_CODE(2177u32);
pub const FACILITY_LEAP: FACILITY_CODE = FACILITY_CODE(2184u32);
pub const FACILITY_AUDCLNT: FACILITY_CODE = FACILITY_CODE(2185u32);
pub const FACILITY_WINCODEC_DWRITE_DWM: FACILITY_CODE = FACILITY_CODE(2200u32);
pub const FACILITY_WINML: FACILITY_CODE = FACILITY_CODE(2192u32);
pub const FACILITY_DIRECT2D: FACILITY_CODE = FACILITY_CODE(2201u32);
pub const FACILITY_DEFRAG: FACILITY_CODE = FACILITY_CODE(2304u32);
pub const FACILITY_USERMODE_SDBUS: FACILITY_CODE = FACILITY_CODE(2305u32);
pub const FACILITY_JSCRIPT: FACILITY_CODE = FACILITY_CODE(2306u32);
pub const FACILITY_PIDGENX: FACILITY_CODE = FACILITY_CODE(2561u32);
pub const FACILITY_EAS: FACILITY_CODE = FACILITY_CODE(85u32);
pub const FACILITY_WEB: FACILITY_CODE = FACILITY_CODE(885u32);
pub const FACILITY_WEB_SOCKET: FACILITY_CODE = FACILITY_CODE(886u32);
pub const FACILITY_MOBILE: FACILITY_CODE = FACILITY_CODE(1793u32);
pub const FACILITY_SQLITE: FACILITY_CODE = FACILITY_CODE(1967u32);
pub const FACILITY_SERVICE_FABRIC: FACILITY_CODE = FACILITY_CODE(1968u32);
pub const FACILITY_UTC: FACILITY_CODE = FACILITY_CODE(1989u32);
pub const FACILITY_WEP: FACILITY_CODE = FACILITY_CODE(2049u32);
pub const FACILITY_SYNCENGINE: FACILITY_CODE = FACILITY_CODE(2050u32);
pub const FACILITY_XBOX: FACILITY_CODE = FACILITY_CODE(2339u32);
pub const FACILITY_GAME: FACILITY_CODE = FACILITY_CODE(2340u32);
pub const FACILITY_PIX: FACILITY_CODE = FACILITY_CODE(2748u32);
pub const FACILITY_NT_BIT: FACILITY_CODE = FACILITY_CODE(268435456u32);
pub const FACILITY_JsDEBUG: u32 = 3527u32;
pub const FIELDS_DID_NOT_MATCH: u32 = 4u32;
#[repr(C)]
pub struct FIELD_INFO(i32);
pub const FLAG_ENGINE_PRESENT: u32 = 4u32;
pub const FLAG_ENGOPT_DISALLOW_NETWORK_PATHS: u32 = 8u32;
pub const FLAG_OVERRIDE_ARM_MACHINE_TYPE: u32 = 16u32;
#[repr(transparent)]
pub struct FORMAT_MESSAGE_OPTIONS(pub u32);
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(256u32);
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(8192u32);
pub const FORMAT_MESSAGE_FROM_HMODULE: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(2048u32);
pub const FORMAT_MESSAGE_FROM_STRING: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(1024u32);
pub const FORMAT_MESSAGE_FROM_SYSTEM: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(4096u32);
pub const FORMAT_MESSAGE_IGNORE_INSERTS: FORMAT_MESSAGE_OPTIONS = FORMAT_MESSAGE_OPTIONS(512u32);
#[repr(C)]
pub struct FPO_DATA(i32);
pub const GETATTRFLAG_HUMANTEXT: u32 = 32768u32;
pub const GETATTRFLAG_THIS: u32 = 256u32;
pub const GETATTRTYPE_DEPSCAN: u32 = 1u32;
pub const GETATTRTYPE_NORMAL: u32 = 0u32;
#[repr(C)]
pub struct GET_CONTEXT_EX(i32);
#[repr(C)]
pub struct GET_CURRENT_PROCESS_ADDRESS(i32);
#[repr(C)]
pub struct GET_CURRENT_THREAD_ADDRESS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GET_EXPRESSION_EX(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GET_INPUT_LINE(i32);
#[repr(C)]
pub struct GET_PEB_ADDRESS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct GET_SET_SYMPATH(i32);
#[repr(C)]
pub struct GET_TEB_ADDRESS(i32);
#[repr(transparent)]
pub struct IActiveScript(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptAuthor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptAuthorProcedure(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptDebug32(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptDebug64(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptEncode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptError(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptError64(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptErrorDebug(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptErrorDebug110(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptGarbageCollector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptHostEncode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptParse32(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptParse64(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptParseProcedure2_32(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptParseProcedure2_64(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptParseProcedure32(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptParseProcedure64(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptParseProcedureOld32(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptParseProcedureOld64(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptProfilerCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptProfilerCallback2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptProfilerCallback3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptProfilerControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptProfilerControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptProfilerControl3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptProfilerControl4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptProfilerControl5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptProfilerHeapEnum(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptSIPInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptSiteDebug32(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptSiteDebug64(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptSiteDebugEx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptSiteInterruptPoll(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptSiteTraceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptSiteUIControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptSiteWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptStats(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptStringCompare(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptTraceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActiveScriptWinRTErrorDebug(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationDebugger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationDebuggerUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBindEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICodeAddressConcept(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComparableConcept(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelConcept(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelNameBinder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelScript(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelScriptClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelScriptDebug(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelScriptDebug2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelScriptDebugBreakpoint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelScriptDebugBreakpointEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelScriptDebugClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelScriptDebugStack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelScriptDebugStackFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelScriptDebugVariableSetEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelScriptHostContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelScriptManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelScriptProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelScriptProviderEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelScriptTemplate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataModelScriptTemplateEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugAdvanced(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugAdvanced2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugAdvanced3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugAdvanced4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugApplication11032(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugApplication11064(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugApplication32(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugApplication64(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugApplicationNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugApplicationNode100(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugApplicationNodeEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugApplicationThread(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugApplicationThread11032(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugApplicationThread11064(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugApplicationThread64(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugApplicationThreadEvents110(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugAsyncOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugAsyncOperationCallBack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugBreakpoint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugBreakpoint2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugBreakpoint3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugClient2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugClient3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugClient4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugClient5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugClient6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugClient7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugClient8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugCodeContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugControl3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugControl4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugControl5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugControl6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugControl7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugCookie(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugDataSpaces(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugDataSpaces2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugDataSpaces3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugDataSpaces4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugDocumentContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugDocumentHelper32(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugDocumentHelper64(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugDocumentHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugDocumentInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugDocumentProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugDocumentText(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugDocumentTextAuthor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugDocumentTextEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugDocumentTextExternalAuthor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugEventCallbacks(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugEventCallbacksWide(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugEventContextCallbacks(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugExpression(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugExpressionCallBack(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugExpressionContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugExtendedProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugFormatter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostBaseClass(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostConstant(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostErrorSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostEvaluator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostEvaluator2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostExtensibility(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostField(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostMemory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostMemory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostModule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostModule2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostModuleSignature(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostPublic(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostScriptHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostSymbol(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostSymbol2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostSymbolEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostSymbols(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostType2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugHostTypeSignature(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugInputCallbacks(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugOutputCallbacks(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugOutputCallbacks2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugOutputCallbacksWide(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugOutputStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugPlmClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugPlmClient2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugPlmClient3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugPropertyEnumType_All(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugPropertyEnumType_Arguments(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugPropertyEnumType_Locals(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugPropertyEnumType_LocalsPlusArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugPropertyEnumType_Registers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugRegisters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugRegisters2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSessionProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugStackFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugStackFrame110(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugStackFrameSniffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugStackFrameSnifferEx32(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugStackFrameSnifferEx64(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSymbolGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSymbolGroup2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSymbols(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSymbols2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSymbols3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSymbols4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSymbols5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSyncOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSystemObjects(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSystemObjects2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSystemObjects3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugSystemObjects4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugThreadCall32(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDebugThreadCall64(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDynamicConceptProviderConcept(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDynamicKeyProviderConcept(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumDebugApplicationNodes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumDebugCodeContexts(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumDebugExpressionContexts(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumDebugExtendedPropertyInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumDebugPropertyInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumDebugStackFrames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumDebugStackFrames64(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumJsStackFrames(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumRemoteDebugApplicationThreads(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumRemoteDebugApplications(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEquatableConcept(pub *mut ::core::ffi::c_void);
pub const IG_DISASSEMBLE_BUFFER: u32 = 44u32;
pub const IG_DUMP_SYMBOL_INFO: u32 = 22u32;
pub const IG_FIND_FILE: u32 = 40u32;
pub const IG_GET_ANY_MODULE_IN_RANGE: u32 = 45u32;
pub const IG_GET_BUS_DATA: u32 = 20u32;
pub const IG_GET_CACHE_SIZE: u32 = 32u32;
pub const IG_GET_CLR_DATA_INTERFACE: u32 = 38u32;
pub const IG_GET_CONTEXT_EX: u32 = 48u32;
pub const IG_GET_CURRENT_PROCESS: u32 = 26u32;
pub const IG_GET_CURRENT_PROCESS_HANDLE: u32 = 28u32;
pub const IG_GET_CURRENT_THREAD: u32 = 25u32;
pub const IG_GET_DEBUGGER_DATA: u32 = 14u32;
pub const IG_GET_EXCEPTION_RECORD: u32 = 18u32;
pub const IG_GET_EXPRESSION_EX: u32 = 30u32;
pub const IG_GET_INPUT_LINE: u32 = 29u32;
pub const IG_GET_KERNEL_VERSION: u32 = 15u32;
pub const IG_GET_PEB_ADDRESS: u32 = 129u32;
pub const IG_GET_SET_SYMPATH: u32 = 17u32;
pub const IG_GET_TEB_ADDRESS: u32 = 128u32;
pub const IG_GET_THREAD_OS_INFO: u32 = 37u32;
pub const IG_GET_TYPE_SIZE: u32 = 27u32;
pub const IG_IS_PTR64: u32 = 19u32;
pub const IG_KD_CONTEXT: u32 = 1u32;
pub const IG_KSTACK_HELP: u32 = 10u32;
pub const IG_LOWMEM_CHECK: u32 = 23u32;
pub const IG_MATCH_PATTERN_A: u32 = 39u32;
pub const IG_OBSOLETE_PLACEHOLDER_36: u32 = 36u32;
pub const IG_PHYSICAL_TO_VIRTUAL: u32 = 47u32;
pub const IG_POINTER_SEARCH_PHYSICAL: u32 = 35u32;
pub const IG_QUERY_TARGET_INTERFACE: u32 = 42u32;
pub const IG_READ_CONTROL_SPACE: u32 = 2u32;
pub const IG_READ_IO_SPACE: u32 = 4u32;
pub const IG_READ_IO_SPACE_EX: u32 = 8u32;
pub const IG_READ_MSR: u32 = 12u32;
pub const IG_READ_PHYSICAL: u32 = 6u32;
pub const IG_READ_PHYSICAL_WITH_FLAGS: u32 = 33u32;
pub const IG_RELOAD_SYMBOLS: u32 = 16u32;
pub const IG_SEARCH_MEMORY: u32 = 24u32;
pub const IG_SET_BUS_DATA: u32 = 21u32;
pub const IG_SET_THREAD: u32 = 11u32;
pub const IG_TRANSLATE_VIRTUAL_TO_PHYSICAL: u32 = 31u32;
pub const IG_TYPED_DATA: u32 = 43u32;
pub const IG_TYPED_DATA_OBSOLETE: u32 = 41u32;
pub const IG_VIRTUAL_TO_PHYSICAL: u32 = 46u32;
pub const IG_WRITE_CONTROL_SPACE: u32 = 3u32;
pub const IG_WRITE_IO_SPACE: u32 = 5u32;
pub const IG_WRITE_IO_SPACE_EX: u32 = 9u32;
pub const IG_WRITE_MSR: u32 = 13u32;
pub const IG_WRITE_PHYSICAL: u32 = 7u32;
pub const IG_WRITE_PHYSICAL_WITH_FLAGS: u32 = 34u32;
#[repr(transparent)]
pub struct IHostDataModelAccess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIndexableConcept(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIterableConcept(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJsDebug(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJsDebugBreakPoint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJsDebugDataTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJsDebugFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJsDebugProcess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJsDebugProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJsDebugStackWalker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJsEnumDebugProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKeyStore(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_CBA_EVENT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_CBA_EVENTW(i32);
#[repr(transparent)]
pub struct IMAGEHLP_CBA_EVENT_SEVERITY(pub u32);
pub const sevInfo: IMAGEHLP_CBA_EVENT_SEVERITY = IMAGEHLP_CBA_EVENT_SEVERITY(0u32);
pub const sevProblem: IMAGEHLP_CBA_EVENT_SEVERITY = IMAGEHLP_CBA_EVENT_SEVERITY(1u32);
pub const sevAttn: IMAGEHLP_CBA_EVENT_SEVERITY = IMAGEHLP_CBA_EVENT_SEVERITY(2u32);
pub const sevFatal: IMAGEHLP_CBA_EVENT_SEVERITY = IMAGEHLP_CBA_EVENT_SEVERITY(3u32);
#[repr(C)]
pub struct IMAGEHLP_CBA_READ_MEMORY(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOAD(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOAD64(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_DEFERRED_SYMBOL_LOADW64(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_DUPLICATE_SYMBOL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_DUPLICATE_SYMBOL64(i32);
#[repr(transparent)]
pub struct IMAGEHLP_EXTENDED_OPTIONS(pub i32);
pub const SYMOPT_EX_DISABLEACCESSTIMEUPDATE: IMAGEHLP_EXTENDED_OPTIONS = IMAGEHLP_EXTENDED_OPTIONS(0i32);
pub const SYMOPT_EX_LASTVALIDDEBUGDIRECTORY: IMAGEHLP_EXTENDED_OPTIONS = IMAGEHLP_EXTENDED_OPTIONS(1i32);
pub const SYMOPT_EX_NOIMPLICITPATTERNSEARCH: IMAGEHLP_EXTENDED_OPTIONS = IMAGEHLP_EXTENDED_OPTIONS(2i32);
pub const SYMOPT_EX_NEVERLOADSYMBOLS: IMAGEHLP_EXTENDED_OPTIONS = IMAGEHLP_EXTENDED_OPTIONS(3i32);
pub const SYMOPT_EX_MAX: IMAGEHLP_EXTENDED_OPTIONS = IMAGEHLP_EXTENDED_OPTIONS(4i32);
#[repr(transparent)]
pub struct IMAGEHLP_GET_TYPE_INFO_FLAGS(pub u32);
pub const IMAGEHLP_GET_TYPE_INFO_CHILDREN: IMAGEHLP_GET_TYPE_INFO_FLAGS = IMAGEHLP_GET_TYPE_INFO_FLAGS(2u32);
pub const IMAGEHLP_GET_TYPE_INFO_UNCACHED: IMAGEHLP_GET_TYPE_INFO_FLAGS = IMAGEHLP_GET_TYPE_INFO_FLAGS(1u32);
#[repr(C)]
pub struct IMAGEHLP_GET_TYPE_INFO_PARAMS(i32);
#[repr(transparent)]
pub struct IMAGEHLP_HD_TYPE(pub i32);
pub const hdBase: IMAGEHLP_HD_TYPE = IMAGEHLP_HD_TYPE(0i32);
pub const hdSym: IMAGEHLP_HD_TYPE = IMAGEHLP_HD_TYPE(1i32);
pub const hdSrc: IMAGEHLP_HD_TYPE = IMAGEHLP_HD_TYPE(2i32);
pub const hdMax: IMAGEHLP_HD_TYPE = IMAGEHLP_HD_TYPE(3i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_LINE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_LINE64(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_LINEW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_LINEW64(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_MODULE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_MODULE64(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_MODULE64_EX(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct IMAGEHLP_MODULEW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_MODULEW64(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_MODULEW64_EX(i32);
pub const IMAGEHLP_MODULE_REGION_ADDITIONAL: u32 = 4u32;
pub const IMAGEHLP_MODULE_REGION_ALL: u32 = 255u32;
pub const IMAGEHLP_MODULE_REGION_DLLBASE: u32 = 1u32;
pub const IMAGEHLP_MODULE_REGION_DLLRANGE: u32 = 2u32;
pub const IMAGEHLP_MODULE_REGION_JIT: u32 = 8u32;
pub const IMAGEHLP_RMAP_BIG_ENDIAN: u32 = 2u32;
pub const IMAGEHLP_RMAP_FIXUP_ARM64X: u32 = 268435456u32;
pub const IMAGEHLP_RMAP_FIXUP_IMAGEBASE: u32 = 2147483648u32;
pub const IMAGEHLP_RMAP_IGNORE_MISCOMPARE: u32 = 4u32;
pub const IMAGEHLP_RMAP_LOAD_RW_DATA_SECTIONS: u32 = 536870912u32;
pub const IMAGEHLP_RMAP_MAPPED_FLAT: u32 = 1u32;
pub const IMAGEHLP_RMAP_OMIT_SHARED_RW_DATA_SECTIONS: u32 = 1073741824u32;
#[repr(transparent)]
pub struct IMAGEHLP_SF_TYPE(pub i32);
pub const sfImage: IMAGEHLP_SF_TYPE = IMAGEHLP_SF_TYPE(0i32);
pub const sfDbg: IMAGEHLP_SF_TYPE = IMAGEHLP_SF_TYPE(1i32);
pub const sfPdb: IMAGEHLP_SF_TYPE = IMAGEHLP_SF_TYPE(2i32);
pub const sfMpd: IMAGEHLP_SF_TYPE = IMAGEHLP_SF_TYPE(3i32);
pub const sfMax: IMAGEHLP_SF_TYPE = IMAGEHLP_SF_TYPE(4i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_STACK_FRAME(i32);
#[repr(transparent)]
pub struct IMAGEHLP_STATUS_REASON(pub i32);
pub const BindOutOfMemory: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(0i32);
pub const BindRvaToVaFailed: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(1i32);
pub const BindNoRoomInImage: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(2i32);
pub const BindImportModuleFailed: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(3i32);
pub const BindImportProcedureFailed: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(4i32);
pub const BindImportModule: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(5i32);
pub const BindImportProcedure: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(6i32);
pub const BindForwarder: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(7i32);
pub const BindForwarderNOT: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(8i32);
pub const BindImageModified: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(9i32);
pub const BindExpandFileHeaders: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(10i32);
pub const BindImageComplete: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(11i32);
pub const BindMismatchedSymbols: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(12i32);
pub const BindSymbolsNotUpdated: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(13i32);
pub const BindImportProcedure32: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(14i32);
pub const BindImportProcedure64: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(15i32);
pub const BindForwarder32: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(16i32);
pub const BindForwarder64: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(17i32);
pub const BindForwarderNOT32: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(18i32);
pub const BindForwarderNOT64: IMAGEHLP_STATUS_REASON = IMAGEHLP_STATUS_REASON(19i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_SYMBOL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_SYMBOL64(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_SYMBOL64_PACKAGE(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct IMAGEHLP_SYMBOLW(i32);
#[repr(C)]
pub struct IMAGEHLP_SYMBOLW64(i32);
#[repr(C)]
pub struct IMAGEHLP_SYMBOLW64_PACKAGE(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct IMAGEHLP_SYMBOLW_PACKAGE(i32);
pub const IMAGEHLP_SYMBOL_FUNCTION: u32 = 2048u32;
pub const IMAGEHLP_SYMBOL_INFO_CONSTANT: u32 = 256u32;
pub const IMAGEHLP_SYMBOL_INFO_FRAMERELATIVE: u32 = 32u32;
pub const IMAGEHLP_SYMBOL_INFO_LOCAL: u32 = 128u32;
pub const IMAGEHLP_SYMBOL_INFO_PARAMETER: u32 = 64u32;
pub const IMAGEHLP_SYMBOL_INFO_REGISTER: u32 = 8u32;
pub const IMAGEHLP_SYMBOL_INFO_REGRELATIVE: u32 = 16u32;
pub const IMAGEHLP_SYMBOL_INFO_TLSRELATIVE: u32 = 16384u32;
pub const IMAGEHLP_SYMBOL_INFO_VALUEPRESENT: u32 = 1u32;
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_SYMBOL_PACKAGE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IMAGEHLP_SYMBOL_SRC(i32);
pub const IMAGEHLP_SYMBOL_THUNK: u32 = 8192u32;
#[repr(transparent)]
pub struct IMAGEHLP_SYMBOL_TYPE_INFO(pub i32);
pub const TI_GET_SYMTAG: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(0i32);
pub const TI_GET_SYMNAME: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(1i32);
pub const TI_GET_LENGTH: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(2i32);
pub const TI_GET_TYPE: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(3i32);
pub const TI_GET_TYPEID: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(4i32);
pub const TI_GET_BASETYPE: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(5i32);
pub const TI_GET_ARRAYINDEXTYPEID: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(6i32);
pub const TI_FINDCHILDREN: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(7i32);
pub const TI_GET_DATAKIND: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(8i32);
pub const TI_GET_ADDRESSOFFSET: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(9i32);
pub const TI_GET_OFFSET: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(10i32);
pub const TI_GET_VALUE: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(11i32);
pub const TI_GET_COUNT: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(12i32);
pub const TI_GET_CHILDRENCOUNT: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(13i32);
pub const TI_GET_BITPOSITION: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(14i32);
pub const TI_GET_VIRTUALBASECLASS: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(15i32);
pub const TI_GET_VIRTUALTABLESHAPEID: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(16i32);
pub const TI_GET_VIRTUALBASEPOINTEROFFSET: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(17i32);
pub const TI_GET_CLASSPARENTID: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(18i32);
pub const TI_GET_NESTED: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(19i32);
pub const TI_GET_SYMINDEX: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(20i32);
pub const TI_GET_LEXICALPARENT: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(21i32);
pub const TI_GET_ADDRESS: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(22i32);
pub const TI_GET_THISADJUST: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(23i32);
pub const TI_GET_UDTKIND: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(24i32);
pub const TI_IS_EQUIV_TO: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(25i32);
pub const TI_GET_CALLING_CONVENTION: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(26i32);
pub const TI_IS_CLOSE_EQUIV_TO: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(27i32);
pub const TI_GTIEX_REQS_VALID: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(28i32);
pub const TI_GET_VIRTUALBASEOFFSET: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(29i32);
pub const TI_GET_VIRTUALBASEDISPINDEX: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(30i32);
pub const TI_GET_IS_REFERENCE: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(31i32);
pub const TI_GET_INDIRECTVIRTUALBASECLASS: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(32i32);
pub const TI_GET_VIRTUALBASETABLETYPE: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(33i32);
pub const TI_GET_OBJECTPOINTERTYPE: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(34i32);
pub const IMAGEHLP_SYMBOL_TYPE_INFO_MAX: IMAGEHLP_SYMBOL_TYPE_INFO = IMAGEHLP_SYMBOL_TYPE_INFO(35i32);
pub const IMAGEHLP_SYMBOL_VIRTUAL: u32 = 4096u32;
#[repr(C)]
pub struct IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY(i32);
#[repr(C)]
pub struct IMAGE_COFF_SYMBOLS_HEADER(i32);
#[repr(C)]
pub struct IMAGE_COR20_HEADER(i32);
#[repr(C)]
pub struct IMAGE_DATA_DIRECTORY(i32);
#[repr(C)]
pub struct IMAGE_DEBUG_DIRECTORY(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct IMAGE_DEBUG_INFORMATION(i32);
#[repr(transparent)]
pub struct IMAGE_DEBUG_TYPE(pub u32);
pub const IMAGE_DEBUG_TYPE_UNKNOWN: IMAGE_DEBUG_TYPE = IMAGE_DEBUG_TYPE(0u32);
pub const IMAGE_DEBUG_TYPE_COFF: IMAGE_DEBUG_TYPE = IMAGE_DEBUG_TYPE(1u32);
pub const IMAGE_DEBUG_TYPE_CODEVIEW: IMAGE_DEBUG_TYPE = IMAGE_DEBUG_TYPE(2u32);
pub const IMAGE_DEBUG_TYPE_FPO: IMAGE_DEBUG_TYPE = IMAGE_DEBUG_TYPE(3u32);
pub const IMAGE_DEBUG_TYPE_MISC: IMAGE_DEBUG_TYPE = IMAGE_DEBUG_TYPE(4u32);
pub const IMAGE_DEBUG_TYPE_EXCEPTION: IMAGE_DEBUG_TYPE = IMAGE_DEBUG_TYPE(5u32);
pub const IMAGE_DEBUG_TYPE_FIXUP: IMAGE_DEBUG_TYPE = IMAGE_DEBUG_TYPE(6u32);
pub const IMAGE_DEBUG_TYPE_BORLAND: IMAGE_DEBUG_TYPE = IMAGE_DEBUG_TYPE(9u32);
#[repr(transparent)]
pub struct IMAGE_DIRECTORY_ENTRY(pub u32);
pub const IMAGE_DIRECTORY_ENTRY_ARCHITECTURE: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(7u32);
pub const IMAGE_DIRECTORY_ENTRY_BASERELOC: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(5u32);
pub const IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(11u32);
pub const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(14u32);
pub const IMAGE_DIRECTORY_ENTRY_DEBUG: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(6u32);
pub const IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(13u32);
pub const IMAGE_DIRECTORY_ENTRY_EXCEPTION: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(3u32);
pub const IMAGE_DIRECTORY_ENTRY_EXPORT: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(0u32);
pub const IMAGE_DIRECTORY_ENTRY_GLOBALPTR: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(8u32);
pub const IMAGE_DIRECTORY_ENTRY_IAT: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(12u32);
pub const IMAGE_DIRECTORY_ENTRY_IMPORT: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(1u32);
pub const IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(10u32);
pub const IMAGE_DIRECTORY_ENTRY_RESOURCE: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(2u32);
pub const IMAGE_DIRECTORY_ENTRY_SECURITY: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(4u32);
pub const IMAGE_DIRECTORY_ENTRY_TLS: IMAGE_DIRECTORY_ENTRY = IMAGE_DIRECTORY_ENTRY(9u32);
#[repr(transparent)]
pub struct IMAGE_DLL_CHARACTERISTICS(pub u16);
pub const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(32u16);
pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(64u16);
pub const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(128u16);
pub const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(256u16);
pub const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(512u16);
pub const IMAGE_DLLCHARACTERISTICS_NO_SEH: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(1024u16);
pub const IMAGE_DLLCHARACTERISTICS_NO_BIND: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(2048u16);
pub const IMAGE_DLLCHARACTERISTICS_APPCONTAINER: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(4096u16);
pub const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(8192u16);
pub const IMAGE_DLLCHARACTERISTICS_GUARD_CF: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(16384u16);
pub const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(32768u16);
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_COMPAT: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(1u16);
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_COMPAT_STRICT_MODE: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(2u16);
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_SET_CONTEXT_IP_VALIDATION_RELAXED_MODE: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(4u16);
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_DYNAMIC_APIS_ALLOW_IN_PROC: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(8u16);
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_RESERVED_1: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(16u16);
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_RESERVED_2: IMAGE_DLL_CHARACTERISTICS = IMAGE_DLL_CHARACTERISTICS(32u16);
#[repr(transparent)]
pub struct IMAGE_FILE_CHARACTERISTICS(pub u16);
pub const IMAGE_FILE_RELOCS_STRIPPED: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(1u16);
pub const IMAGE_FILE_EXECUTABLE_IMAGE: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(2u16);
pub const IMAGE_FILE_LINE_NUMS_STRIPPED: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(4u16);
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(8u16);
pub const IMAGE_FILE_AGGRESIVE_WS_TRIM: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(16u16);
pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(32u16);
pub const IMAGE_FILE_BYTES_REVERSED_LO: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(128u16);
pub const IMAGE_FILE_32BIT_MACHINE: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(256u16);
pub const IMAGE_FILE_DEBUG_STRIPPED: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(512u16);
pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(1024u16);
pub const IMAGE_FILE_NET_RUN_FROM_SWAP: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(2048u16);
pub const IMAGE_FILE_SYSTEM: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(4096u16);
pub const IMAGE_FILE_DLL: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(8192u16);
pub const IMAGE_FILE_UP_SYSTEM_ONLY: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(16384u16);
pub const IMAGE_FILE_BYTES_REVERSED_HI: IMAGE_FILE_CHARACTERISTICS = IMAGE_FILE_CHARACTERISTICS(32768u16);
#[repr(transparent)]
pub struct IMAGE_FILE_CHARACTERISTICS2(pub u32);
pub const IMAGE_FILE_RELOCS_STRIPPED2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(1u32);
pub const IMAGE_FILE_EXECUTABLE_IMAGE2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(2u32);
pub const IMAGE_FILE_LINE_NUMS_STRIPPED2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(4u32);
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(8u32);
pub const IMAGE_FILE_AGGRESIVE_WS_TRIM2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(16u32);
pub const IMAGE_FILE_LARGE_ADDRESS_AWARE2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(32u32);
pub const IMAGE_FILE_BYTES_REVERSED_LO2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(128u32);
pub const IMAGE_FILE_32BIT_MACHINE2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(256u32);
pub const IMAGE_FILE_DEBUG_STRIPPED2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(512u32);
pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(1024u32);
pub const IMAGE_FILE_NET_RUN_FROM_SWAP2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(2048u32);
pub const IMAGE_FILE_SYSTEM_2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(4096u32);
pub const IMAGE_FILE_DLL_2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(8192u32);
pub const IMAGE_FILE_UP_SYSTEM_ONLY_2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(16384u32);
pub const IMAGE_FILE_BYTES_REVERSED_HI_2: IMAGE_FILE_CHARACTERISTICS2 = IMAGE_FILE_CHARACTERISTICS2(32768u32);
#[repr(C)]
pub struct IMAGE_FILE_HEADER(i32);
#[repr(transparent)]
pub struct IMAGE_FILE_MACHINE(pub u16);
pub const IMAGE_FILE_MACHINE_AXP64: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(644u16);
pub const IMAGE_FILE_MACHINE_I386: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(332u16);
pub const IMAGE_FILE_MACHINE_IA64: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(512u16);
pub const IMAGE_FILE_MACHINE_AMD64: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(34404u16);
pub const IMAGE_FILE_MACHINE_UNKNOWN: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(0u16);
pub const IMAGE_FILE_MACHINE_TARGET_HOST: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(1u16);
pub const IMAGE_FILE_MACHINE_R3000: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(354u16);
pub const IMAGE_FILE_MACHINE_R4000: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(358u16);
pub const IMAGE_FILE_MACHINE_R10000: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(360u16);
pub const IMAGE_FILE_MACHINE_WCEMIPSV2: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(361u16);
pub const IMAGE_FILE_MACHINE_ALPHA: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(388u16);
pub const IMAGE_FILE_MACHINE_SH3: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(418u16);
pub const IMAGE_FILE_MACHINE_SH3DSP: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(419u16);
pub const IMAGE_FILE_MACHINE_SH3E: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(420u16);
pub const IMAGE_FILE_MACHINE_SH4: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(422u16);
pub const IMAGE_FILE_MACHINE_SH5: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(424u16);
pub const IMAGE_FILE_MACHINE_ARM: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(448u16);
pub const IMAGE_FILE_MACHINE_THUMB: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(450u16);
pub const IMAGE_FILE_MACHINE_ARMNT: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(452u16);
pub const IMAGE_FILE_MACHINE_AM33: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(467u16);
pub const IMAGE_FILE_MACHINE_POWERPC: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(496u16);
pub const IMAGE_FILE_MACHINE_POWERPCFP: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(497u16);
pub const IMAGE_FILE_MACHINE_MIPS16: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(614u16);
pub const IMAGE_FILE_MACHINE_ALPHA64: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(644u16);
pub const IMAGE_FILE_MACHINE_MIPSFPU: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(870u16);
pub const IMAGE_FILE_MACHINE_MIPSFPU16: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(1126u16);
pub const IMAGE_FILE_MACHINE_TRICORE: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(1312u16);
pub const IMAGE_FILE_MACHINE_CEF: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(3311u16);
pub const IMAGE_FILE_MACHINE_EBC: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(3772u16);
pub const IMAGE_FILE_MACHINE_M32R: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(36929u16);
pub const IMAGE_FILE_MACHINE_ARM64: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(43620u16);
pub const IMAGE_FILE_MACHINE_CEE: IMAGE_FILE_MACHINE = IMAGE_FILE_MACHINE(49390u16);
#[repr(C)]
pub struct IMAGE_FUNCTION_ENTRY(i32);
#[repr(C)]
pub struct IMAGE_FUNCTION_ENTRY64(i32);
#[repr(C)]
pub struct IMAGE_LOAD_CONFIG_CODE_INTEGRITY(i32);
#[repr(C)]
pub struct IMAGE_LOAD_CONFIG_DIRECTORY32(i32);
#[repr(C)]
pub struct IMAGE_LOAD_CONFIG_DIRECTORY64(i32);
#[repr(C)]
pub struct IMAGE_NT_HEADERS32(i32);
#[repr(C)]
pub struct IMAGE_NT_HEADERS64(i32);
#[repr(C)]
pub struct IMAGE_OPTIONAL_HEADER32(i32);
#[repr(C)]
pub struct IMAGE_OPTIONAL_HEADER64(i32);
#[repr(transparent)]
pub struct IMAGE_OPTIONAL_HEADER_MAGIC(pub u16);
pub const IMAGE_NT_OPTIONAL_HDR_MAGIC: IMAGE_OPTIONAL_HEADER_MAGIC = IMAGE_OPTIONAL_HEADER_MAGIC(523u16);
pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: IMAGE_OPTIONAL_HEADER_MAGIC = IMAGE_OPTIONAL_HEADER_MAGIC(267u16);
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: IMAGE_OPTIONAL_HEADER_MAGIC = IMAGE_OPTIONAL_HEADER_MAGIC(523u16);
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: IMAGE_OPTIONAL_HEADER_MAGIC = IMAGE_OPTIONAL_HEADER_MAGIC(263u16);
#[repr(C)]
pub struct IMAGE_ROM_HEADERS(i32);
#[repr(C)]
pub struct IMAGE_ROM_OPTIONAL_HEADER(i32);
#[repr(C)]
pub struct IMAGE_RUNTIME_FUNCTION_ENTRY(i32);
#[repr(transparent)]
pub struct IMAGE_SECTION_CHARACTERISTICS(pub u32);
pub const IMAGE_SCN_TYPE_NO_PAD: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(8u32);
pub const IMAGE_SCN_CNT_CODE: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(32u32);
pub const IMAGE_SCN_CNT_INITIALIZED_DATA: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(64u32);
pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(128u32);
pub const IMAGE_SCN_LNK_OTHER: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(256u32);
pub const IMAGE_SCN_LNK_INFO: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(512u32);
pub const IMAGE_SCN_LNK_REMOVE: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(2048u32);
pub const IMAGE_SCN_LNK_COMDAT: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(4096u32);
pub const IMAGE_SCN_NO_DEFER_SPEC_EXC: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(16384u32);
pub const IMAGE_SCN_GPREL: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(32768u32);
pub const IMAGE_SCN_MEM_FARDATA: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(32768u32);
pub const IMAGE_SCN_MEM_PURGEABLE: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(131072u32);
pub const IMAGE_SCN_MEM_16BIT: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(131072u32);
pub const IMAGE_SCN_MEM_LOCKED: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(262144u32);
pub const IMAGE_SCN_MEM_PRELOAD: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(524288u32);
pub const IMAGE_SCN_ALIGN_1BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(1048576u32);
pub const IMAGE_SCN_ALIGN_2BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(2097152u32);
pub const IMAGE_SCN_ALIGN_4BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(3145728u32);
pub const IMAGE_SCN_ALIGN_8BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(4194304u32);
pub const IMAGE_SCN_ALIGN_16BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(5242880u32);
pub const IMAGE_SCN_ALIGN_32BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(6291456u32);
pub const IMAGE_SCN_ALIGN_64BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(7340032u32);
pub const IMAGE_SCN_ALIGN_128BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(8388608u32);
pub const IMAGE_SCN_ALIGN_256BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(9437184u32);
pub const IMAGE_SCN_ALIGN_512BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(10485760u32);
pub const IMAGE_SCN_ALIGN_1024BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(11534336u32);
pub const IMAGE_SCN_ALIGN_2048BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(12582912u32);
pub const IMAGE_SCN_ALIGN_4096BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(13631488u32);
pub const IMAGE_SCN_ALIGN_8192BYTES: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(14680064u32);
pub const IMAGE_SCN_ALIGN_MASK: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(15728640u32);
pub const IMAGE_SCN_LNK_NRELOC_OVFL: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(16777216u32);
pub const IMAGE_SCN_MEM_DISCARDABLE: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(33554432u32);
pub const IMAGE_SCN_MEM_NOT_CACHED: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(67108864u32);
pub const IMAGE_SCN_MEM_NOT_PAGED: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(134217728u32);
pub const IMAGE_SCN_MEM_SHARED: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(268435456u32);
pub const IMAGE_SCN_MEM_EXECUTE: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(536870912u32);
pub const IMAGE_SCN_MEM_READ: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(1073741824u32);
pub const IMAGE_SCN_MEM_WRITE: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(2147483648u32);
pub const IMAGE_SCN_SCALE_INDEX: IMAGE_SECTION_CHARACTERISTICS = IMAGE_SECTION_CHARACTERISTICS(1u32);
#[repr(C)]
pub struct IMAGE_SECTION_HEADER(i32);
#[repr(transparent)]
pub struct IMAGE_SUBSYSTEM(pub u16);
pub const IMAGE_SUBSYSTEM_UNKNOWN: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(0u16);
pub const IMAGE_SUBSYSTEM_NATIVE: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(1u16);
pub const IMAGE_SUBSYSTEM_WINDOWS_GUI: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(2u16);
pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(3u16);
pub const IMAGE_SUBSYSTEM_OS2_CUI: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(5u16);
pub const IMAGE_SUBSYSTEM_POSIX_CUI: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(7u16);
pub const IMAGE_SUBSYSTEM_NATIVE_WINDOWS: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(8u16);
pub const IMAGE_SUBSYSTEM_WINDOWS_CE_GUI: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(9u16);
pub const IMAGE_SUBSYSTEM_EFI_APPLICATION: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(10u16);
pub const IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(11u16);
pub const IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(12u16);
pub const IMAGE_SUBSYSTEM_EFI_ROM: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(13u16);
pub const IMAGE_SUBSYSTEM_XBOX: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(14u16);
pub const IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(16u16);
pub const IMAGE_SUBSYSTEM_XBOX_CODE_CATALOG: IMAGE_SUBSYSTEM = IMAGE_SUBSYSTEM(17u16);
#[repr(transparent)]
pub struct IMachineDebugManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMachineDebugManagerCookie(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMachineDebugManagerEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IModelIterator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IModelKeyReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IModelKeyReference2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IModelMethod(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IModelObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IModelPropertyAccessor(pub *mut ::core::ffi::c_void);
pub const INCORRECT_VERSION_INFO: u32 = 7u32;
#[repr(C)]
pub struct INLINE_FRAME_CONTEXT(i32);
pub const INLINE_FRAME_CONTEXT_IGNORE: u32 = 4294967295u32;
pub const INLINE_FRAME_CONTEXT_INIT: u32 = 0u32;
pub const INSUFFICIENT_SPACE_TO_COPY: u32 = 10u32;
pub const INTERFACESAFE_FOR_UNTRUSTED_CALLER: u32 = 1u32;
pub const INTERFACESAFE_FOR_UNTRUSTED_DATA: u32 = 2u32;
pub const INTERFACE_USES_DISPEX: u32 = 4u32;
pub const INTERFACE_USES_SECURITY_MANAGER: u32 = 8u32;
pub const IOCTL_IPMI_INTERNAL_RECORD_SEL_EVENT: u32 = 2232320u32;
#[repr(C)]
pub struct IOSPACE(i32);
#[repr(C)]
pub struct IOSPACE32(i32);
#[repr(C)]
pub struct IOSPACE64(i32);
#[repr(C)]
pub struct IOSPACE_EX(i32);
#[repr(C)]
pub struct IOSPACE_EX32(i32);
#[repr(C)]
pub struct IOSPACE_EX64(i32);
#[repr(transparent)]
pub struct IObjectSafety(pub *mut ::core::ffi::c_void);
pub const IPMI_IOCTL_INDEX: u32 = 1024u32;
#[repr(C)]
pub struct IPMI_OS_SEL_RECORD(i32);
pub const IPMI_OS_SEL_RECORD_MASK: u32 = 65535u32;
#[repr(transparent)]
pub struct IPMI_OS_SEL_RECORD_TYPE(pub i32);
pub const IpmiOsSelRecordTypeWhea: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(0i32);
pub const IpmiOsSelRecordTypeOther: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(1i32);
pub const IpmiOsSelRecordTypeWheaErrorXpfMca: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(2i32);
pub const IpmiOsSelRecordTypeWheaErrorPci: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(3i32);
pub const IpmiOsSelRecordTypeWheaErrorNmi: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(4i32);
pub const IpmiOsSelRecordTypeWheaErrorOther: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(5i32);
pub const IpmiOsSelRecordTypeRaw: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(6i32);
pub const IpmiOsSelRecordTypeDriver: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(7i32);
pub const IpmiOsSelRecordTypeBugcheckRecovery: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(8i32);
pub const IpmiOsSelRecordTypeBugcheckData: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(9i32);
pub const IpmiOsSelRecordTypeMax: IPMI_OS_SEL_RECORD_TYPE = IPMI_OS_SEL_RECORD_TYPE(10i32);
pub const IPMI_OS_SEL_RECORD_VERSION: u32 = 1u32;
pub const IPMI_OS_SEL_RECORD_VERSION_1: u32 = 1u32;
#[repr(transparent)]
pub struct IPerPropertyBrowsing2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPreferredRuntimeTypeConcept(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessDebugManager32(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessDebugManager64(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProvideExpressionContexts(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRawEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteDebugApplication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteDebugApplication110(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteDebugApplicationEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteDebugApplicationThread(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteDebugCriticalErrorEvent110(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteDebugInfoEvent110(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScriptEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScriptInvocationContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScriptNode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScriptScriptlet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISimpleConnectionPoint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStringDisplayableConcept(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITridentEventSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAppDiagnosticsObjectInitialization(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebAppDiagnosticsSetup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IntrinsicKind(pub i32);
pub const IntrinsicVoid: IntrinsicKind = IntrinsicKind(0i32);
pub const IntrinsicBool: IntrinsicKind = IntrinsicKind(1i32);
pub const IntrinsicChar: IntrinsicKind = IntrinsicKind(2i32);
pub const IntrinsicWChar: IntrinsicKind = IntrinsicKind(3i32);
pub const IntrinsicInt: IntrinsicKind = IntrinsicKind(4i32);
pub const IntrinsicUInt: IntrinsicKind = IntrinsicKind(5i32);
pub const IntrinsicLong: IntrinsicKind = IntrinsicKind(6i32);
pub const IntrinsicULong: IntrinsicKind = IntrinsicKind(7i32);
pub const IntrinsicFloat: IntrinsicKind = IntrinsicKind(8i32);
pub const IntrinsicHRESULT: IntrinsicKind = IntrinsicKind(9i32);
pub const IntrinsicChar16: IntrinsicKind = IntrinsicKind(10i32);
pub const IntrinsicChar32: IntrinsicKind = IntrinsicKind(11i32);
#[repr(transparent)]
pub struct JS_PROPERTY_ATTRIBUTES(pub i32);
pub const JS_PROPERTY_ATTRIBUTE_NONE: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(0i32);
pub const JS_PROPERTY_HAS_CHILDREN: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(1i32);
pub const JS_PROPERTY_FAKE: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(2i32);
pub const JS_PROPERTY_METHOD: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(4i32);
pub const JS_PROPERTY_READONLY: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(8i32);
pub const JS_PROPERTY_NATIVE_WINRT_POINTER: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(16i32);
pub const JS_PROPERTY_FRAME_INTRYBLOCK: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(32i32);
pub const JS_PROPERTY_FRAME_INCATCHBLOCK: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(64i32);
pub const JS_PROPERTY_FRAME_INFINALLYBLOCK: JS_PROPERTY_ATTRIBUTES = JS_PROPERTY_ATTRIBUTES(128i32);
#[repr(transparent)]
pub struct JS_PROPERTY_MEMBERS(pub i32);
pub const JS_PROPERTY_MEMBERS_ALL: JS_PROPERTY_MEMBERS = JS_PROPERTY_MEMBERS(0i32);
pub const JS_PROPERTY_MEMBERS_ARGUMENTS: JS_PROPERTY_MEMBERS = JS_PROPERTY_MEMBERS(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct JsDebugPropertyInfo(i32);
#[repr(transparent)]
pub struct JsDebugReadMemoryFlags(pub i32);
pub const None: JsDebugReadMemoryFlags = JsDebugReadMemoryFlags(0i32);
pub const JsDebugAllowPartialRead: JsDebugReadMemoryFlags = JsDebugReadMemoryFlags(1i32);
#[cfg(feature = "Win32_System_Kernel")]
#[repr(C)]
pub struct KDDEBUGGER_DATA32(i32);
#[cfg(feature = "Win32_System_Kernel")]
#[repr(C)]
pub struct KDDEBUGGER_DATA64(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct KDHELP(i32);
#[repr(C)]
pub struct KDHELP64(i32);
pub const KD_SECONDARY_VERSION_AMD64_CONTEXT: u32 = 2u32;
pub const KD_SECONDARY_VERSION_AMD64_OBSOLETE_CONTEXT_1: u32 = 0u32;
pub const KD_SECONDARY_VERSION_AMD64_OBSOLETE_CONTEXT_2: u32 = 1u32;
pub const KD_SECONDARY_VERSION_DEFAULT: u32 = 0u32;
#[cfg(any(target_arch = "x86_64",))]
#[repr(C)]
pub struct KNONVOLATILE_CONTEXT_POINTERS(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct KNONVOLATILE_CONTEXT_POINTERS(i32);
#[cfg(any(target_arch = "aarch64",))]
#[repr(C)]
pub struct KNONVOLATILE_CONTEXT_POINTERS_ARM64(i32);
#[repr(C)]
pub struct LDT_ENTRY(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct LOADED_IMAGE(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct LOADED_IMAGE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct LOAD_DLL_DEBUG_INFO(i32);
pub type LPCALL_BACK_USER_INTERRUPT_ROUTINE = unsafe extern "system" fn() -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type LPTOP_LEVEL_EXCEPTION_FILTER = unsafe extern "system" fn(exceptioninfo: *const EXCEPTION_POINTERS) -> i32;
#[repr(transparent)]
pub struct LanguageKind(pub i32);
pub const LanguageUnknown: LanguageKind = LanguageKind(0i32);
pub const LanguageC: LanguageKind = LanguageKind(1i32);
pub const LanguageCPP: LanguageKind = LanguageKind(2i32);
pub const LanguageAssembly: LanguageKind = LanguageKind(3i32);
#[repr(C)]
pub struct Location(i32);
#[repr(transparent)]
pub struct LocationKind(pub i32);
pub const LocationMember: LocationKind = LocationKind(0i32);
pub const LocationStatic: LocationKind = LocationKind(1i32);
pub const LocationConstant: LocationKind = LocationKind(2i32);
pub const LocationNone: LocationKind = LocationKind(3i32);
#[repr(C)]
pub struct M128A(i32);
pub const MAX_SYM_NAME: u32 = 2000u32;
pub const MEMORY_READ_ERROR: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Memory"))]
#[repr(C)]
pub struct MINIDUMP_CALLBACK_INFORMATION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct MINIDUMP_CALLBACK_INPUT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
#[repr(C)]
pub struct MINIDUMP_CALLBACK_OUTPUT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Memory"))]
pub type MINIDUMP_CALLBACK_ROUTINE = unsafe extern "system" fn(callbackparam: *mut ::core::ffi::c_void, callbackinput: *const MINIDUMP_CALLBACK_INPUT, callbackoutput: *mut MINIDUMP_CALLBACK_OUTPUT) -> super::super::super::Foundation::BOOL;
#[repr(transparent)]
pub struct MINIDUMP_CALLBACK_TYPE(pub i32);
pub const ModuleCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(0i32);
pub const ThreadCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(1i32);
pub const ThreadExCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(2i32);
pub const IncludeThreadCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(3i32);
pub const IncludeModuleCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(4i32);
pub const MemoryCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(5i32);
pub const CancelCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(6i32);
pub const WriteKernelMinidumpCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(7i32);
pub const KernelMinidumpStatusCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(8i32);
pub const RemoveMemoryCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(9i32);
pub const IncludeVmRegionCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(10i32);
pub const IoStartCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(11i32);
pub const IoWriteAllCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(12i32);
pub const IoFinishCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(13i32);
pub const ReadMemoryFailureCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(14i32);
pub const SecondaryFlagsCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(15i32);
pub const IsProcessSnapshotCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(16i32);
pub const VmStartCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(17i32);
pub const VmQueryCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(18i32);
pub const VmPreReadCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(19i32);
pub const VmPostReadCallback: MINIDUMP_CALLBACK_TYPE = MINIDUMP_CALLBACK_TYPE(20i32);
#[repr(C)]
pub struct MINIDUMP_DIRECTORY(i32);
#[repr(C)]
pub struct MINIDUMP_EXCEPTION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct MINIDUMP_EXCEPTION_INFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MINIDUMP_EXCEPTION_INFORMATION64(i32);
#[repr(C)]
pub struct MINIDUMP_EXCEPTION_STREAM(i32);
#[repr(C)]
pub struct MINIDUMP_FUNCTION_TABLE_DESCRIPTOR(i32);
#[repr(C)]
pub struct MINIDUMP_FUNCTION_TABLE_STREAM(i32);
#[repr(C)]
pub struct MINIDUMP_HANDLE_DATA_STREAM(i32);
#[repr(C)]
pub struct MINIDUMP_HANDLE_DESCRIPTOR(i32);
#[repr(C)]
pub struct MINIDUMP_HANDLE_DESCRIPTOR_2(i32);
#[repr(C)]
pub struct MINIDUMP_HANDLE_OBJECT_INFORMATION(i32);
#[repr(transparent)]
pub struct MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(pub i32);
pub const MiniHandleObjectInformationNone: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(0i32);
pub const MiniThreadInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(1i32);
pub const MiniMutantInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(2i32);
pub const MiniMutantInformation2: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(3i32);
pub const MiniProcessInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(4i32);
pub const MiniProcessInformation2: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(5i32);
pub const MiniEventInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(6i32);
pub const MiniSectionInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(7i32);
pub const MiniSemaphoreInformation1: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(8i32);
pub const MiniHandleObjectInformationTypeMax: MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE = MINIDUMP_HANDLE_OBJECT_INFORMATION_TYPE(9i32);
#[repr(C)]
pub struct MINIDUMP_HANDLE_OPERATION_LIST(i32);
#[repr(C)]
pub struct MINIDUMP_HEADER(i32);
#[repr(C)]
pub struct MINIDUMP_INCLUDE_MODULE_CALLBACK(i32);
#[repr(C)]
pub struct MINIDUMP_INCLUDE_THREAD_CALLBACK(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MINIDUMP_IO_CALLBACK(i32);
#[repr(C)]
pub struct MINIDUMP_LOCATION_DESCRIPTOR(i32);
#[repr(C)]
pub struct MINIDUMP_LOCATION_DESCRIPTOR64(i32);
#[repr(C)]
pub struct MINIDUMP_MEMORY64_LIST(i32);
#[repr(C)]
pub struct MINIDUMP_MEMORY_DESCRIPTOR(i32);
#[repr(C)]
pub struct MINIDUMP_MEMORY_DESCRIPTOR64(i32);
#[cfg(feature = "Win32_System_Memory")]
#[repr(C)]
pub struct MINIDUMP_MEMORY_INFO(i32);
#[repr(C)]
pub struct MINIDUMP_MEMORY_INFO_LIST(i32);
#[repr(C)]
pub struct MINIDUMP_MEMORY_LIST(i32);
pub const MINIDUMP_MISC1_PROCESSOR_POWER_INFO: u32 = 4u32;
pub const MINIDUMP_MISC3_PROCESS_EXECUTE_FLAGS: u32 = 32u32;
pub const MINIDUMP_MISC3_PROCESS_INTEGRITY: u32 = 16u32;
pub const MINIDUMP_MISC3_PROTECTED_PROCESS: u32 = 128u32;
pub const MINIDUMP_MISC3_TIMEZONE: u32 = 64u32;
pub const MINIDUMP_MISC4_BUILDSTRING: u32 = 256u32;
pub const MINIDUMP_MISC5_PROCESS_COOKIE: u32 = 512u32;
#[repr(C)]
pub struct MINIDUMP_MISC_INFO(i32);
#[repr(C)]
pub struct MINIDUMP_MISC_INFO_2(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
#[repr(C)]
pub struct MINIDUMP_MISC_INFO_3(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
#[repr(C)]
pub struct MINIDUMP_MISC_INFO_4(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
#[repr(C)]
pub struct MINIDUMP_MISC_INFO_5(i32);
#[repr(transparent)]
pub struct MINIDUMP_MISC_INFO_FLAGS(pub u32);
pub const MINIDUMP_MISC1_PROCESS_ID: MINIDUMP_MISC_INFO_FLAGS = MINIDUMP_MISC_INFO_FLAGS(1u32);
pub const MINIDUMP_MISC1_PROCESS_TIMES: MINIDUMP_MISC_INFO_FLAGS = MINIDUMP_MISC_INFO_FLAGS(2u32);
#[cfg(feature = "Win32_Storage_FileSystem")]
#[repr(C)]
pub struct MINIDUMP_MODULE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
#[repr(C)]
pub struct MINIDUMP_MODULE_CALLBACK(i32);
#[cfg(feature = "Win32_Storage_FileSystem")]
#[repr(C)]
pub struct MINIDUMP_MODULE_LIST(i32);
pub const MINIDUMP_PROCESS_VM_COUNTERS: u32 = 1u32;
#[repr(C)]
pub struct MINIDUMP_PROCESS_VM_COUNTERS_1(i32);
#[repr(C)]
pub struct MINIDUMP_PROCESS_VM_COUNTERS_2(i32);
pub const MINIDUMP_PROCESS_VM_COUNTERS_EX: u32 = 4u32;
pub const MINIDUMP_PROCESS_VM_COUNTERS_EX2: u32 = 8u32;
pub const MINIDUMP_PROCESS_VM_COUNTERS_JOB: u32 = 16u32;
pub const MINIDUMP_PROCESS_VM_COUNTERS_VIRTUALSIZE: u32 = 2u32;
#[repr(C)]
pub struct MINIDUMP_READ_MEMORY_FAILURE_CALLBACK(i32);
#[repr(transparent)]
pub struct MINIDUMP_SECONDARY_FLAGS(pub i32);
pub const MiniSecondaryWithoutPowerInfo: MINIDUMP_SECONDARY_FLAGS = MINIDUMP_SECONDARY_FLAGS(1i32);
pub const MiniSecondaryValidFlags: MINIDUMP_SECONDARY_FLAGS = MINIDUMP_SECONDARY_FLAGS(1i32);
#[repr(transparent)]
pub struct MINIDUMP_STREAM_TYPE(pub i32);
pub const UnusedStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(0i32);
pub const ReservedStream0: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(1i32);
pub const ReservedStream1: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(2i32);
pub const ThreadListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(3i32);
pub const ModuleListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(4i32);
pub const MemoryListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(5i32);
pub const ExceptionStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(6i32);
pub const SystemInfoStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(7i32);
pub const ThreadExListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(8i32);
pub const Memory64ListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(9i32);
pub const CommentStreamA: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(10i32);
pub const CommentStreamW: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(11i32);
pub const HandleDataStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(12i32);
pub const FunctionTableStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(13i32);
pub const UnloadedModuleListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(14i32);
pub const MiscInfoStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(15i32);
pub const MemoryInfoListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(16i32);
pub const ThreadInfoListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(17i32);
pub const HandleOperationListStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(18i32);
pub const TokenStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(19i32);
pub const JavaScriptDataStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(20i32);
pub const SystemMemoryInfoStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(21i32);
pub const ProcessVmCountersStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(22i32);
pub const IptTraceStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(23i32);
pub const ThreadNamesStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(24i32);
pub const ceStreamNull: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32768i32);
pub const ceStreamSystemInfo: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32769i32);
pub const ceStreamException: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32770i32);
pub const ceStreamModuleList: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32771i32);
pub const ceStreamProcessList: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32772i32);
pub const ceStreamThreadList: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32773i32);
pub const ceStreamThreadContextList: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32774i32);
pub const ceStreamThreadCallStackList: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32775i32);
pub const ceStreamMemoryVirtualList: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32776i32);
pub const ceStreamMemoryPhysicalList: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32777i32);
pub const ceStreamBucketParameters: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32778i32);
pub const ceStreamProcessModuleMap: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32779i32);
pub const ceStreamDiagnosisList: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(32780i32);
pub const LastReservedStream: MINIDUMP_STREAM_TYPE = MINIDUMP_STREAM_TYPE(65535i32);
#[repr(C)]
pub struct MINIDUMP_STRING(i32);
pub const MINIDUMP_SYSMEMINFO1_BASICPERF: u32 = 2u32;
pub const MINIDUMP_SYSMEMINFO1_FILECACHE_TRANSITIONREPURPOSECOUNT_FLAGS: u32 = 1u32;
pub const MINIDUMP_SYSMEMINFO1_PERF_CCTOTALDIRTYPAGES_CCDIRTYPAGETHRESHOLD: u32 = 4u32;
pub const MINIDUMP_SYSMEMINFO1_PERF_RESIDENTAVAILABLEPAGES_SHAREDCOMMITPAGES: u32 = 8u32;
#[repr(C)]
pub struct MINIDUMP_SYSTEM_BASIC_INFORMATION(i32);
#[repr(C)]
pub struct MINIDUMP_SYSTEM_BASIC_PERFORMANCE_INFORMATION(i32);
#[repr(C)]
pub struct MINIDUMP_SYSTEM_FILECACHE_INFORMATION(i32);
#[repr(C)]
pub struct MINIDUMP_SYSTEM_INFO(i32);
#[repr(C)]
pub struct MINIDUMP_SYSTEM_MEMORY_INFO_1(i32);
#[repr(C)]
pub struct MINIDUMP_SYSTEM_PERFORMANCE_INFORMATION(i32);
#[repr(C)]
pub struct MINIDUMP_THREAD(i32);
#[cfg(any(target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct MINIDUMP_THREAD_CALLBACK(i32);
#[cfg(any(target_arch = "x86_64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct MINIDUMP_THREAD_CALLBACK(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct MINIDUMP_THREAD_CALLBACK(i32);
#[repr(C)]
pub struct MINIDUMP_THREAD_EX(i32);
#[cfg(any(target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct MINIDUMP_THREAD_EX_CALLBACK(i32);
#[cfg(any(target_arch = "x86_64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct MINIDUMP_THREAD_EX_CALLBACK(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct MINIDUMP_THREAD_EX_CALLBACK(i32);
#[repr(C)]
pub struct MINIDUMP_THREAD_EX_LIST(i32);
#[repr(C)]
pub struct MINIDUMP_THREAD_INFO(i32);
#[repr(transparent)]
pub struct MINIDUMP_THREAD_INFO_DUMP_FLAGS(pub u32);
pub const MINIDUMP_THREAD_INFO_ERROR_THREAD: MINIDUMP_THREAD_INFO_DUMP_FLAGS = MINIDUMP_THREAD_INFO_DUMP_FLAGS(1u32);
pub const MINIDUMP_THREAD_INFO_EXITED_THREAD: MINIDUMP_THREAD_INFO_DUMP_FLAGS = MINIDUMP_THREAD_INFO_DUMP_FLAGS(4u32);
pub const MINIDUMP_THREAD_INFO_INVALID_CONTEXT: MINIDUMP_THREAD_INFO_DUMP_FLAGS = MINIDUMP_THREAD_INFO_DUMP_FLAGS(16u32);
pub const MINIDUMP_THREAD_INFO_INVALID_INFO: MINIDUMP_THREAD_INFO_DUMP_FLAGS = MINIDUMP_THREAD_INFO_DUMP_FLAGS(8u32);
pub const MINIDUMP_THREAD_INFO_INVALID_TEB: MINIDUMP_THREAD_INFO_DUMP_FLAGS = MINIDUMP_THREAD_INFO_DUMP_FLAGS(32u32);
pub const MINIDUMP_THREAD_INFO_WRITING_THREAD: MINIDUMP_THREAD_INFO_DUMP_FLAGS = MINIDUMP_THREAD_INFO_DUMP_FLAGS(2u32);
#[repr(C)]
pub struct MINIDUMP_THREAD_INFO_LIST(i32);
#[repr(C)]
pub struct MINIDUMP_THREAD_LIST(i32);
#[repr(C)]
pub struct MINIDUMP_THREAD_NAME(i32);
#[repr(C)]
pub struct MINIDUMP_THREAD_NAME_LIST(i32);
#[repr(C)]
pub struct MINIDUMP_TOKEN_INFO_HEADER(i32);
#[repr(C)]
pub struct MINIDUMP_TOKEN_INFO_LIST(i32);
#[repr(transparent)]
pub struct MINIDUMP_TYPE(pub u32);
pub const MiniDumpNormal: MINIDUMP_TYPE = MINIDUMP_TYPE(0u32);
pub const MiniDumpWithDataSegs: MINIDUMP_TYPE = MINIDUMP_TYPE(1u32);
pub const MiniDumpWithFullMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(2u32);
pub const MiniDumpWithHandleData: MINIDUMP_TYPE = MINIDUMP_TYPE(4u32);
pub const MiniDumpFilterMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(8u32);
pub const MiniDumpScanMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(16u32);
pub const MiniDumpWithUnloadedModules: MINIDUMP_TYPE = MINIDUMP_TYPE(32u32);
pub const MiniDumpWithIndirectlyReferencedMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(64u32);
pub const MiniDumpFilterModulePaths: MINIDUMP_TYPE = MINIDUMP_TYPE(128u32);
pub const MiniDumpWithProcessThreadData: MINIDUMP_TYPE = MINIDUMP_TYPE(256u32);
pub const MiniDumpWithPrivateReadWriteMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(512u32);
pub const MiniDumpWithoutOptionalData: MINIDUMP_TYPE = MINIDUMP_TYPE(1024u32);
pub const MiniDumpWithFullMemoryInfo: MINIDUMP_TYPE = MINIDUMP_TYPE(2048u32);
pub const MiniDumpWithThreadInfo: MINIDUMP_TYPE = MINIDUMP_TYPE(4096u32);
pub const MiniDumpWithCodeSegs: MINIDUMP_TYPE = MINIDUMP_TYPE(8192u32);
pub const MiniDumpWithoutAuxiliaryState: MINIDUMP_TYPE = MINIDUMP_TYPE(16384u32);
pub const MiniDumpWithFullAuxiliaryState: MINIDUMP_TYPE = MINIDUMP_TYPE(32768u32);
pub const MiniDumpWithPrivateWriteCopyMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(65536u32);
pub const MiniDumpIgnoreInaccessibleMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(131072u32);
pub const MiniDumpWithTokenInformation: MINIDUMP_TYPE = MINIDUMP_TYPE(262144u32);
pub const MiniDumpWithModuleHeaders: MINIDUMP_TYPE = MINIDUMP_TYPE(524288u32);
pub const MiniDumpFilterTriage: MINIDUMP_TYPE = MINIDUMP_TYPE(1048576u32);
pub const MiniDumpWithAvxXStateContext: MINIDUMP_TYPE = MINIDUMP_TYPE(2097152u32);
pub const MiniDumpWithIptTrace: MINIDUMP_TYPE = MINIDUMP_TYPE(4194304u32);
pub const MiniDumpScanInaccessiblePartialPages: MINIDUMP_TYPE = MINIDUMP_TYPE(8388608u32);
pub const MiniDumpFilterWriteCombinedMemory: MINIDUMP_TYPE = MINIDUMP_TYPE(16777216u32);
pub const MiniDumpValidTypeFlags: MINIDUMP_TYPE = MINIDUMP_TYPE(33554431u32);
#[repr(C)]
pub struct MINIDUMP_UNLOADED_MODULE(i32);
#[repr(C)]
pub struct MINIDUMP_UNLOADED_MODULE_LIST(i32);
#[repr(C)]
pub struct MINIDUMP_USER_RECORD(i32);
#[repr(C)]
pub struct MINIDUMP_USER_STREAM(i32);
#[repr(C)]
pub struct MINIDUMP_USER_STREAM_INFORMATION(i32);
pub const MINIDUMP_VERSION: u32 = 42899u32;
#[repr(C)]
pub struct MINIDUMP_VM_POST_READ_CALLBACK(i32);
#[repr(C)]
pub struct MINIDUMP_VM_PRE_READ_CALLBACK(i32);
#[repr(C)]
pub struct MINIDUMP_VM_QUERY_CALLBACK(i32);
#[repr(C)]
pub struct MODLOAD_CVMISC(i32);
#[repr(C)]
pub struct MODLOAD_DATA(i32);
#[repr(transparent)]
pub struct MODLOAD_DATA_TYPE(pub u32);
pub const DBHHEADER_DEBUGDIRS: MODLOAD_DATA_TYPE = MODLOAD_DATA_TYPE(1u32);
pub const DBHHEADER_CVMISC: MODLOAD_DATA_TYPE = MODLOAD_DATA_TYPE(2u32);
#[repr(C)]
pub struct MODLOAD_PDBGUID_PDBAGE(i32);
pub const MODULE_ORDERS_LOADTIME: u32 = 268435456u32;
pub const MODULE_ORDERS_MASK: u32 = 4026531840u32;
pub const MODULE_ORDERS_MODULENAME: u32 = 536870912u32;
#[repr(C)]
pub struct MODULE_TYPE_INFO(i32);
#[repr(transparent)]
pub struct MODULE_WRITE_FLAGS(pub i32);
pub const ModuleWriteModule: MODULE_WRITE_FLAGS = MODULE_WRITE_FLAGS(1i32);
pub const ModuleWriteDataSeg: MODULE_WRITE_FLAGS = MODULE_WRITE_FLAGS(2i32);
pub const ModuleWriteMiscRecord: MODULE_WRITE_FLAGS = MODULE_WRITE_FLAGS(4i32);
pub const ModuleWriteCvRecord: MODULE_WRITE_FLAGS = MODULE_WRITE_FLAGS(8i32);
pub const ModuleReferencedByMemory: MODULE_WRITE_FLAGS = MODULE_WRITE_FLAGS(16i32);
pub const ModuleWriteTlsData: MODULE_WRITE_FLAGS = MODULE_WRITE_FLAGS(32i32);
pub const ModuleWriteCodeSegs: MODULE_WRITE_FLAGS = MODULE_WRITE_FLAGS(64i32);
pub const MachineDebugManager_DEBUG: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1232510188,
    data2: 14933,
    data3: 19376,
    data4: [182, 151, 136, 254, 222, 119, 232, 234],
};
pub const MachineDebugManager_RETAIL: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 201995878, data2: 12489, data3: 4560, data4: [143, 32, 0, 128, 95, 44, 208, 100] };
#[repr(transparent)]
pub struct ModelObjectKind(pub i32);
pub const ObjectPropertyAccessor: ModelObjectKind = ModelObjectKind(0i32);
pub const ObjectContext: ModelObjectKind = ModelObjectKind(1i32);
pub const ObjectTargetObject: ModelObjectKind = ModelObjectKind(2i32);
pub const ObjectTargetObjectReference: ModelObjectKind = ModelObjectKind(3i32);
pub const ObjectSynthetic: ModelObjectKind = ModelObjectKind(4i32);
pub const ObjectNoValue: ModelObjectKind = ModelObjectKind(5i32);
pub const ObjectError: ModelObjectKind = ModelObjectKind(6i32);
pub const ObjectIntrinsic: ModelObjectKind = ModelObjectKind(7i32);
pub const ObjectMethod: ModelObjectKind = ModelObjectKind(8i32);
pub const ObjectKeyReference: ModelObjectKind = ModelObjectKind(9i32);
pub const NULL_FIELD_NAME: u32 = 6u32;
pub const NULL_SYM_DUMP_PARAM: u32 = 5u32;
pub const NUM_SSRVOPTS: u32 = 32u32;
#[repr(transparent)]
pub struct OBJECT_ATTRIB_FLAG(pub u32);
pub const OBJECT_ATTRIB_NO_ATTRIB: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(0u32);
pub const OBJECT_ATTRIB_NO_NAME: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(1u32);
pub const OBJECT_ATTRIB_NO_TYPE: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(2u32);
pub const OBJECT_ATTRIB_NO_VALUE: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(4u32);
pub const OBJECT_ATTRIB_VALUE_IS_INVALID: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(8u32);
pub const OBJECT_ATTRIB_VALUE_IS_OBJECT: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(16u32);
pub const OBJECT_ATTRIB_VALUE_IS_ENUM: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(32u32);
pub const OBJECT_ATTRIB_VALUE_IS_CUSTOM: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(64u32);
pub const OBJECT_ATTRIB_OBJECT_IS_EXPANDABLE: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(112u32);
pub const OBJECT_ATTRIB_VALUE_HAS_CODE: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(128u32);
pub const OBJECT_ATTRIB_TYPE_IS_OBJECT: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(256u32);
pub const OBJECT_ATTRIB_TYPE_HAS_CODE: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(512u32);
pub const OBJECT_ATTRIB_TYPE_IS_EXPANDABLE: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(256u32);
pub const OBJECT_ATTRIB_SLOT_IS_CATEGORY: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(1024u32);
pub const OBJECT_ATTRIB_VALUE_READONLY: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(2048u32);
pub const OBJECT_ATTRIB_ACCESS_PUBLIC: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(4096u32);
pub const OBJECT_ATTRIB_ACCESS_PRIVATE: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(8192u32);
pub const OBJECT_ATTRIB_ACCESS_PROTECTED: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(16384u32);
pub const OBJECT_ATTRIB_ACCESS_FINAL: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(32768u32);
pub const OBJECT_ATTRIB_STORAGE_GLOBAL: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(65536u32);
pub const OBJECT_ATTRIB_STORAGE_STATIC: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(131072u32);
pub const OBJECT_ATTRIB_STORAGE_FIELD: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(262144u32);
pub const OBJECT_ATTRIB_STORAGE_VIRTUAL: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(524288u32);
pub const OBJECT_ATTRIB_TYPE_IS_CONSTANT: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(1048576u32);
pub const OBJECT_ATTRIB_TYPE_IS_SYNCHRONIZED: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(2097152u32);
pub const OBJECT_ATTRIB_TYPE_IS_VOLATILE: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(4194304u32);
pub const OBJECT_ATTRIB_HAS_EXTENDED_ATTRIBS: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(8388608u32);
pub const OBJECT_ATTRIB_IS_CLASS: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(16777216u32);
pub const OBJECT_ATTRIB_IS_FUNCTION: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(33554432u32);
pub const OBJECT_ATTRIB_IS_VARIABLE: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(67108864u32);
pub const OBJECT_ATTRIB_IS_PROPERTY: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(134217728u32);
pub const OBJECT_ATTRIB_IS_MACRO: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(268435456u32);
pub const OBJECT_ATTRIB_IS_TYPE: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(536870912u32);
pub const OBJECT_ATTRIB_IS_INHERITED: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(1073741824u32);
pub const OBJECT_ATTRIB_IS_INTERFACE: OBJECT_ATTRIB_FLAG = OBJECT_ATTRIB_FLAG(2147483648u32);
pub const OID_JSSIP: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 113893392, data2: 14542, data3: 4564, data4: [162, 163, 0, 16, 75, 211, 80, 144] };
pub const OID_VBSSIP: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 371847246,
    data2: 10137,
    data3: 19893,
    data4: [143, 229, 172, 225, 15, 23, 235, 171],
};
pub const OID_WSFSIP: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 442566000, data2: 14542, data3: 4564, data4: [162, 163, 0, 16, 75, 211, 80, 144] };
#[repr(C)]
pub struct OMAP(i32);
#[repr(transparent)]
pub struct OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS(pub u32);
pub const WCT_ASYNC_OPEN_FLAG: OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS = OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS(1u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OUTPUT_DEBUG_STRING_INFO(i32);
pub type PCOGETACTIVATIONSTATE = unsafe extern "system" fn(param0: ::windows_sys::core::GUID, param1: u32, param2: *mut u32) -> ::windows_sys::core::HRESULT;
pub type PCOGETCALLSTATE = unsafe extern "system" fn(param0: i32, param1: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PDBGHELP_CREATE_USER_DUMP_CALLBACK = unsafe extern "system" fn(datatype: u32, data: *const *const ::core::ffi::c_void, datalength: *mut u32, userdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PDEBUG_EXTENSION_CALL = unsafe extern "system" fn(client: IDebugClient, args: super::super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
pub type PDEBUG_EXTENSION_CANUNLOAD = unsafe extern "system" fn() -> ::windows_sys::core::HRESULT;
pub type PDEBUG_EXTENSION_INITIALIZE = unsafe extern "system" fn(version: *mut u32, flags: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PDEBUG_EXTENSION_KNOWN_STRUCT = unsafe extern "system" fn(flags: u32, offset: u64, typename: super::super::super::Foundation::PSTR, buffer: super::super::super::Foundation::PSTR, bufferchars: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PDEBUG_EXTENSION_KNOWN_STRUCT_EX = unsafe extern "system" fn(client: IDebugClient, flags: u32, offset: u64, typename: super::super::super::Foundation::PSTR, buffer: super::super::super::Foundation::PSTR, bufferchars: *mut u32) -> ::windows_sys::core::HRESULT;
pub type PDEBUG_EXTENSION_NOTIFY = unsafe extern "system" fn(notify: u32, argument: u64);
#[cfg(feature = "Win32_Foundation")]
pub type PDEBUG_EXTENSION_PROVIDE_VALUE = unsafe extern "system" fn(client: IDebugClient, flags: u32, name: super::super::super::Foundation::PWSTR, value: *mut u64, typemodbase: *mut u64, typeid: *mut u32, typeflags: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PDEBUG_EXTENSION_QUERY_VALUE_NAMES = unsafe extern "system" fn(client: IDebugClient, flags: u32, buffer: super::super::super::Foundation::PWSTR, bufferchars: u32, bufferneeded: *mut u32) -> ::windows_sys::core::HRESULT;
pub type PDEBUG_EXTENSION_UNINITIALIZE = unsafe extern "system" fn();
pub type PDEBUG_EXTENSION_UNLOAD = unsafe extern "system" fn();
pub type PDEBUG_STACK_PROVIDER_BEGINTHREADSTACKRECONSTRUCTION = unsafe extern "system" fn(streamtype: u32, minidumpstreambuffer: *const ::core::ffi::c_void, buffersize: u32) -> ::windows_sys::core::HRESULT;
pub type PDEBUG_STACK_PROVIDER_ENDTHREADSTACKRECONSTRUCTION = unsafe extern "system" fn() -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PDEBUG_STACK_PROVIDER_FREESTACKSYMFRAMES = unsafe extern "system" fn(stacksymframes: *const STACK_SYM_FRAME_INFO) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PDEBUG_STACK_PROVIDER_RECONSTRUCTSTACK = unsafe extern "system" fn(systemthreadid: u32, nativeframes: *const DEBUG_STACK_FRAME_EX, countnativeframes: u32, stacksymframes: *mut *mut STACK_SYM_FRAME_INFO, stacksymframesfilled: *mut u32) -> ::windows_sys::core::HRESULT;
#[cfg(feature = "Win32_Foundation")]
pub type PENUMDIRTREE_CALLBACK = unsafe extern "system" fn(filepath: super::super::super::Foundation::PSTR, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PENUMDIRTREE_CALLBACKW = unsafe extern "system" fn(filepath: super::super::super::Foundation::PWSTR, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub type PENUMLOADED_MODULES_CALLBACK = unsafe extern "system" fn(modulename: super::super::super::Foundation::PSTR, modulebase: u32, modulesize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PENUMLOADED_MODULES_CALLBACK64 = unsafe extern "system" fn(modulename: super::super::super::Foundation::PSTR, modulebase: u64, modulesize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PENUMLOADED_MODULES_CALLBACKW64 = unsafe extern "system" fn(modulename: super::super::super::Foundation::PWSTR, modulebase: u64, modulesize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PENUMSOURCEFILETOKENSCALLBACK = unsafe extern "system" fn(token: *const ::core::ffi::c_void, size: usize) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFINDFILEINPATHCALLBACK = unsafe extern "system" fn(filename: super::super::super::Foundation::PSTR, context: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFINDFILEINPATHCALLBACKW = unsafe extern "system" fn(filename: super::super::super::Foundation::PWSTR, context: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFIND_DEBUG_FILE_CALLBACK = unsafe extern "system" fn(filehandle: super::super::super::Foundation::HANDLE, filename: super::super::super::Foundation::PSTR, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFIND_DEBUG_FILE_CALLBACKW = unsafe extern "system" fn(filehandle: super::super::super::Foundation::HANDLE, filename: super::super::super::Foundation::PWSTR, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFIND_EXE_FILE_CALLBACK = unsafe extern "system" fn(filehandle: super::super::super::Foundation::HANDLE, filename: super::super::super::Foundation::PSTR, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PFIND_EXE_FILE_CALLBACKW = unsafe extern "system" fn(filehandle: super::super::super::Foundation::HANDLE, filename: super::super::super::Foundation::PWSTR, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub type PFUNCTION_TABLE_ACCESS_ROUTINE = unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, addrbase: u32) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type PFUNCTION_TABLE_ACCESS_ROUTINE64 = unsafe extern "system" fn(ahprocess: super::super::super::Foundation::HANDLE, addrbase: u64) -> *mut ::core::ffi::c_void;
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub type PGET_MODULE_BASE_ROUTINE = unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, address: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PGET_MODULE_BASE_ROUTINE64 = unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, address: u64) -> u64;
#[cfg(any(target_arch = "aarch64",))]
pub type PGET_RUNTIME_FUNCTION_CALLBACK = unsafe extern "system" fn(controlpc: u64, context: *const ::core::ffi::c_void) -> *mut IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY;
#[cfg(any(target_arch = "x86_64",))]
pub type PGET_RUNTIME_FUNCTION_CALLBACK = unsafe extern "system" fn(controlpc: u64, context: *const ::core::ffi::c_void) -> *mut IMAGE_RUNTIME_FUNCTION_ENTRY;
#[repr(C)]
pub struct PHYSICAL(i32);
#[repr(C)]
pub struct PHYSICAL_MEMORY_DESCRIPTOR32(i32);
#[repr(C)]
pub struct PHYSICAL_MEMORY_DESCRIPTOR64(i32);
#[repr(C)]
pub struct PHYSICAL_MEMORY_RUN32(i32);
#[repr(C)]
pub struct PHYSICAL_MEMORY_RUN64(i32);
#[repr(C)]
pub struct PHYSICAL_TO_VIRTUAL(i32);
#[repr(C)]
pub struct PHYSICAL_WITH_FLAGS(i32);
pub const PHYS_FLAG_CACHED: u32 = 1u32;
pub const PHYS_FLAG_DEFAULT: u32 = 0u32;
pub const PHYS_FLAG_UNCACHED: u32 = 2u32;
pub const PHYS_FLAG_WRITE_COMBINED: u32 = 3u32;
#[cfg(feature = "Win32_Foundation")]
pub type PIMAGEHLP_STATUS_ROUTINE = unsafe extern "system" fn(reason: IMAGEHLP_STATUS_REASON, imagename: super::super::super::Foundation::PSTR, dllname: super::super::super::Foundation::PSTR, va: usize, parameter: usize) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PIMAGEHLP_STATUS_ROUTINE32 = unsafe extern "system" fn(reason: IMAGEHLP_STATUS_REASON, imagename: super::super::super::Foundation::PSTR, dllname: super::super::super::Foundation::PSTR, va: u32, parameter: usize) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PIMAGEHLP_STATUS_ROUTINE64 = unsafe extern "system" fn(reason: IMAGEHLP_STATUS_REASON, imagename: super::super::super::Foundation::PSTR, dllname: super::super::super::Foundation::PSTR, va: u64, parameter: usize) -> super::super::super::Foundation::BOOL;
#[repr(C)]
pub struct POINTER_SEARCH_PHYSICAL(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub type PREAD_PROCESS_MEMORY_ROUTINE = unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, lpbaseaddress: u32, lpbuffer: *mut ::core::ffi::c_void, nsize: u32, lpnumberofbytesread: *mut u32) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PREAD_PROCESS_MEMORY_ROUTINE64 = unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, qwbaseaddress: u64, lpbuffer: *mut ::core::ffi::c_void, nsize: u32, lpnumberofbytesread: *mut u32) -> super::super::super::Foundation::BOOL;
#[repr(C)]
pub struct PROCESSORINFO(i32);
#[repr(transparent)]
pub struct PROCESSOR_ARCHITECTURE(pub u16);
pub const PROCESSOR_ARCHITECTURE_AMD64: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(9u16);
pub const PROCESSOR_ARCHITECTURE_IA64: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(6u16);
pub const PROCESSOR_ARCHITECTURE_INTEL: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(0u16);
pub const PROCESSOR_ARCHITECTURE_ARM: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(5u16);
pub const PROCESSOR_ARCHITECTURE_UNKNOWN: PROCESSOR_ARCHITECTURE = PROCESSOR_ARCHITECTURE(65535u16);
#[repr(C)]
pub struct PROCESS_NAME_ENTRY(i32);
#[repr(transparent)]
pub struct PROFILER_EVENT_MASK(pub u32);
pub const PROFILER_EVENT_MASK_TRACE_SCRIPT_FUNCTION_CALL: PROFILER_EVENT_MASK = PROFILER_EVENT_MASK(1u32);
pub const PROFILER_EVENT_MASK_TRACE_NATIVE_FUNCTION_CALL: PROFILER_EVENT_MASK = PROFILER_EVENT_MASK(2u32);
pub const PROFILER_EVENT_MASK_TRACE_DOM_FUNCTION_CALL: PROFILER_EVENT_MASK = PROFILER_EVENT_MASK(4u32);
pub const PROFILER_EVENT_MASK_TRACE_ALL: PROFILER_EVENT_MASK = PROFILER_EVENT_MASK(3u32);
pub const PROFILER_EVENT_MASK_TRACE_ALL_WITH_DOM: PROFILER_EVENT_MASK = PROFILER_EVENT_MASK(7u32);
#[repr(transparent)]
pub struct PROFILER_HEAP_ENUM_FLAGS(pub u32);
pub const PROFILER_HEAP_ENUM_FLAGS_NONE: PROFILER_HEAP_ENUM_FLAGS = PROFILER_HEAP_ENUM_FLAGS(0u32);
pub const PROFILER_HEAP_ENUM_FLAGS_STORE_RELATIONSHIP_FLAGS: PROFILER_HEAP_ENUM_FLAGS = PROFILER_HEAP_ENUM_FLAGS(1u32);
pub const PROFILER_HEAP_ENUM_FLAGS_SUBSTRINGS: PROFILER_HEAP_ENUM_FLAGS = PROFILER_HEAP_ENUM_FLAGS(2u32);
pub const PROFILER_HEAP_ENUM_FLAGS_RELATIONSHIP_SUBSTRINGS: PROFILER_HEAP_ENUM_FLAGS = PROFILER_HEAP_ENUM_FLAGS(3u32);
#[repr(C)]
pub struct PROFILER_HEAP_OBJECT(i32);
#[repr(transparent)]
pub struct PROFILER_HEAP_OBJECT_FLAGS(pub u32);
pub const PROFILER_HEAP_OBJECT_FLAGS_NEW_OBJECT: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(1u32);
pub const PROFILER_HEAP_OBJECT_FLAGS_IS_ROOT: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(2u32);
pub const PROFILER_HEAP_OBJECT_FLAGS_SITE_CLOSED: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(4u32);
pub const PROFILER_HEAP_OBJECT_FLAGS_EXTERNAL: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(8u32);
pub const PROFILER_HEAP_OBJECT_FLAGS_EXTERNAL_UNKNOWN: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(16u32);
pub const PROFILER_HEAP_OBJECT_FLAGS_EXTERNAL_DISPATCH: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(32u32);
pub const PROFILER_HEAP_OBJECT_FLAGS_SIZE_APPROXIMATE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(64u32);
pub const PROFILER_HEAP_OBJECT_FLAGS_SIZE_UNAVAILABLE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(128u32);
pub const PROFILER_HEAP_OBJECT_FLAGS_NEW_STATE_UNAVAILABLE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(256u32);
pub const PROFILER_HEAP_OBJECT_FLAGS_WINRT_INSTANCE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(512u32);
pub const PROFILER_HEAP_OBJECT_FLAGS_WINRT_RUNTIMECLASS: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(1024u32);
pub const PROFILER_HEAP_OBJECT_FLAGS_WINRT_DELEGATE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(2048u32);
pub const PROFILER_HEAP_OBJECT_FLAGS_WINRT_NAMESPACE: PROFILER_HEAP_OBJECT_FLAGS = PROFILER_HEAP_OBJECT_FLAGS(4096u32);
pub const PROFILER_HEAP_OBJECT_NAME_ID_UNAVAILABLE: u32 = 4294967295u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PROFILER_HEAP_OBJECT_OPTIONAL_INFO(i32);
#[repr(transparent)]
pub struct PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(pub i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_PROTOTYPE: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(1i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_FUNCTION_NAME: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(2i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_SCOPE_LIST: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(3i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_INTERNAL_PROPERTY: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(4i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_NAME_PROPERTIES: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(5i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_INDEX_PROPERTIES: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(6i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_ELEMENT_ATTRIBUTES_SIZE: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(7i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_ELEMENT_TEXT_CHILDREN_SIZE: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(8i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_RELATIONSHIPS: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(9i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_WINRTEVENTS: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(10i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_WEAKMAP_COLLECTION_LIST: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(11i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_MAP_COLLECTION_LIST: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(12i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_SET_COLLECTION_LIST: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(13i32);
pub const PROFILER_HEAP_OBJECT_OPTIONAL_INFO_MAX_VALUE: PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE = PROFILER_HEAP_OBJECT_OPTIONAL_INFO_TYPE(13i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PROFILER_HEAP_OBJECT_RELATIONSHIP(i32);
#[repr(transparent)]
pub struct PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(pub u32);
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_NONE: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(0u32);
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_IS_GET_ACCESSOR: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(65536u32);
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_IS_SET_ACCESSOR: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(131072u32);
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_LET_VARIABLE: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(262144u32);
pub const PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS_CONST_VARIABLE: PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS = PROFILER_HEAP_OBJECT_RELATIONSHIP_FLAGS(524288u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PROFILER_HEAP_OBJECT_RELATIONSHIP_LIST(i32);
#[repr(C)]
pub struct PROFILER_HEAP_OBJECT_SCOPE_LIST(i32);
#[repr(C)]
pub struct PROFILER_HEAP_SUMMARY(i32);
#[repr(transparent)]
pub struct PROFILER_HEAP_SUMMARY_VERSION(pub i32);
pub const PROFILER_HEAP_SUMMARY_VERSION_1: PROFILER_HEAP_SUMMARY_VERSION = PROFILER_HEAP_SUMMARY_VERSION(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PROFILER_PROPERTY_TYPE_SUBSTRING_INFO(i32);
#[repr(transparent)]
pub struct PROFILER_RELATIONSHIP_INFO(pub i32);
pub const PROFILER_PROPERTY_TYPE_NUMBER: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(1i32);
pub const PROFILER_PROPERTY_TYPE_STRING: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(2i32);
pub const PROFILER_PROPERTY_TYPE_HEAP_OBJECT: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(3i32);
pub const PROFILER_PROPERTY_TYPE_EXTERNAL_OBJECT: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(4i32);
pub const PROFILER_PROPERTY_TYPE_BSTR: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(5i32);
pub const PROFILER_PROPERTY_TYPE_SUBSTRING: PROFILER_RELATIONSHIP_INFO = PROFILER_RELATIONSHIP_INFO(6i32);
#[repr(transparent)]
pub struct PROFILER_SCRIPT_TYPE(pub i32);
pub const PROFILER_SCRIPT_TYPE_USER: PROFILER_SCRIPT_TYPE = PROFILER_SCRIPT_TYPE(0i32);
pub const PROFILER_SCRIPT_TYPE_DYNAMIC: PROFILER_SCRIPT_TYPE = PROFILER_SCRIPT_TYPE(1i32);
pub const PROFILER_SCRIPT_TYPE_NATIVE: PROFILER_SCRIPT_TYPE = PROFILER_SCRIPT_TYPE(2i32);
pub const PROFILER_SCRIPT_TYPE_DOM: PROFILER_SCRIPT_TYPE = PROFILER_SCRIPT_TYPE(3i32);
#[repr(transparent)]
pub struct PROP_INFO_FLAGS(pub i32);
pub const PROP_INFO_NAME: PROP_INFO_FLAGS = PROP_INFO_FLAGS(1i32);
pub const PROP_INFO_TYPE: PROP_INFO_FLAGS = PROP_INFO_FLAGS(2i32);
pub const PROP_INFO_VALUE: PROP_INFO_FLAGS = PROP_INFO_FLAGS(4i32);
pub const PROP_INFO_FULLNAME: PROP_INFO_FLAGS = PROP_INFO_FLAGS(32i32);
pub const PROP_INFO_ATTRIBUTES: PROP_INFO_FLAGS = PROP_INFO_FLAGS(8i32);
pub const PROP_INFO_DEBUGPROP: PROP_INFO_FLAGS = PROP_INFO_FLAGS(16i32);
pub const PROP_INFO_AUTOEXPAND: PROP_INFO_FLAGS = PROP_INFO_FLAGS(134217728i32);
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERBYINDEXPROC = unsafe extern "system" fn(param0: super::super::super::Foundation::PSTR, param1: super::super::super::Foundation::PSTR, param2: super::super::super::Foundation::PSTR, param3: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERBYINDEXPROCA = unsafe extern "system" fn(param0: super::super::super::Foundation::PSTR, param1: super::super::super::Foundation::PSTR, param2: super::super::super::Foundation::PSTR, param3: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERBYINDEXPROCW = unsafe extern "system" fn(param0: super::super::super::Foundation::PWSTR, param1: super::super::super::Foundation::PWSTR, param2: super::super::super::Foundation::PWSTR, param3: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERCALLBACKPROC = unsafe extern "system" fn(action: usize, data: u64, context: u64) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERCLOSEPROC = unsafe extern "system" fn() -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERDELTANAME = unsafe extern "system" fn(param0: super::super::super::Foundation::PSTR, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void, param5: u32, param6: u32, param7: super::super::super::Foundation::PSTR, param8: usize) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERDELTANAMEW = unsafe extern "system" fn(param0: super::super::super::Foundation::PWSTR, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32, param4: *mut ::core::ffi::c_void, param5: u32, param6: u32, param7: super::super::super::Foundation::PWSTR, param8: usize) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERGETINDEXSTRING = unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: u32, param2: u32, param3: super::super::super::Foundation::PSTR, param4: usize) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERGETINDEXSTRINGW = unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: u32, param2: u32, param3: super::super::super::Foundation::PWSTR, param4: usize) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERGETOPTIONDATAPROC = unsafe extern "system" fn(param0: usize, param1: *mut u64) -> super::super::super::Foundation::BOOL;
pub type PSYMBOLSERVERGETOPTIONSPROC = unsafe extern "system" fn() -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERGETSUPPLEMENT = unsafe extern "system" fn(param0: super::super::super::Foundation::PSTR, param1: super::super::super::Foundation::PSTR, param2: super::super::super::Foundation::PSTR, param3: super::super::super::Foundation::PSTR, param4: usize) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERGETSUPPLEMENTW = unsafe extern "system" fn(param0: super::super::super::Foundation::PWSTR, param1: super::super::super::Foundation::PWSTR, param2: super::super::super::Foundation::PWSTR, param3: super::super::super::Foundation::PWSTR, param4: usize) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERGETVERSION = unsafe extern "system" fn(param0: *mut API_VERSION) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERISSTORE = unsafe extern "system" fn(param0: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERISSTOREW = unsafe extern "system" fn(param0: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERMESSAGEPROC = unsafe extern "system" fn(action: usize, data: u64, context: u64) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVEROPENPROC = unsafe extern "system" fn() -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERPINGPROC = unsafe extern "system" fn(param0: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERPINGPROCA = unsafe extern "system" fn(param0: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERPINGPROCW = unsafe extern "system" fn(param0: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERPINGPROCWEX = unsafe extern "system" fn(param0: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERPROC = unsafe extern "system" fn(param0: super::super::super::Foundation::PSTR, param1: super::super::super::Foundation::PSTR, param2: *mut ::core::ffi::c_void, param3: u32, param4: u32, param5: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERPROCA = unsafe extern "system" fn(param0: super::super::super::Foundation::PSTR, param1: super::super::super::Foundation::PSTR, param2: *mut ::core::ffi::c_void, param3: u32, param4: u32, param5: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERPROCW = unsafe extern "system" fn(param0: super::super::super::Foundation::PWSTR, param1: super::super::super::Foundation::PWSTR, param2: *mut ::core::ffi::c_void, param3: u32, param4: u32, param5: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERSETHTTPAUTHHEADER = unsafe extern "system" fn(pszauthheader: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERSETOPTIONSPROC = unsafe extern "system" fn(param0: usize, param1: u64) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERSETOPTIONSWPROC = unsafe extern "system" fn(param0: usize, param1: u64) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERSTOREFILE = unsafe extern "system" fn(param0: super::super::super::Foundation::PSTR, param1: super::super::super::Foundation::PSTR, param2: *mut ::core::ffi::c_void, param3: u32, param4: u32, param5: super::super::super::Foundation::PSTR, param6: usize, param7: u32) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERSTOREFILEW = unsafe extern "system" fn(param0: super::super::super::Foundation::PWSTR, param1: super::super::super::Foundation::PWSTR, param2: *mut ::core::ffi::c_void, param3: u32, param4: u32, param5: super::super::super::Foundation::PWSTR, param6: usize, param7: u32) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERSTORESUPPLEMENT = unsafe extern "system" fn(param0: super::super::super::Foundation::PSTR, param1: super::super::super::Foundation::PSTR, param2: super::super::super::Foundation::PSTR, param3: super::super::super::Foundation::PSTR, param4: usize, param5: u32) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERSTORESUPPLEMENTW = unsafe extern "system" fn(param0: super::super::super::Foundation::PWSTR, param1: super::super::super::Foundation::PWSTR, param2: super::super::super::Foundation::PWSTR, param3: super::super::super::Foundation::PWSTR, param4: usize, param5: u32) -> super::super::super::Foundation::BOOL;
pub type PSYMBOLSERVERVERSION = unsafe extern "system" fn() -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOLSERVERWEXPROC = unsafe extern "system" fn(param0: super::super::super::Foundation::PWSTR, param1: super::super::super::Foundation::PWSTR, param2: *mut ::core::ffi::c_void, param3: u32, param4: u32, param5: super::super::super::Foundation::PWSTR, param6: *mut SYMSRV_EXTENDED_OUTPUT_DATA) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOL_FUNCENTRY_CALLBACK = unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, addrbase: u32, usercontext: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOL_FUNCENTRY_CALLBACK64 = unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, addrbase: u64, usercontext: u64) -> *mut ::core::ffi::c_void;
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOL_REGISTERED_CALLBACK = unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, actioncode: u32, callbackdata: *const ::core::ffi::c_void, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYMBOL_REGISTERED_CALLBACK64 = unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, actioncode: u32, callbackdata: u64, usercontext: u64) -> super::super::super::Foundation::BOOL;
pub type PSYM_DUMP_FIELD_CALLBACK = unsafe extern "system" fn(pfield: *mut FIELD_INFO, usercontext: *mut ::core::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMERATESYMBOLS_CALLBACK = unsafe extern "system" fn(psyminfo: *const SYMBOL_INFO, symbolsize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMERATESYMBOLS_CALLBACKW = unsafe extern "system" fn(psyminfo: *const SYMBOL_INFOW, symbolsize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMLINES_CALLBACK = unsafe extern "system" fn(lineinfo: *const SRCCODEINFO, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMLINES_CALLBACKW = unsafe extern "system" fn(lineinfo: *const SRCCODEINFOW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMMODULES_CALLBACK = unsafe extern "system" fn(modulename: super::super::super::Foundation::PSTR, baseofdll: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMMODULES_CALLBACK64 = unsafe extern "system" fn(modulename: super::super::super::Foundation::PSTR, baseofdll: u64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMMODULES_CALLBACKW64 = unsafe extern "system" fn(modulename: super::super::super::Foundation::PWSTR, baseofdll: u64, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMPROCESSES_CALLBACK = unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMSOURCEFILES_CALLBACK = unsafe extern "system" fn(psourcefile: *const SOURCEFILE, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMSOURCEFILES_CALLBACKW = unsafe extern "system" fn(psourcefile: *const SOURCEFILEW, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMSYMBOLS_CALLBACK = unsafe extern "system" fn(symbolname: super::super::super::Foundation::PSTR, symboladdress: u32, symbolsize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMSYMBOLS_CALLBACK64 = unsafe extern "system" fn(symbolname: super::super::super::Foundation::PSTR, symboladdress: u64, symbolsize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMSYMBOLS_CALLBACK64W = unsafe extern "system" fn(symbolname: super::super::super::Foundation::PWSTR, symboladdress: u64, symbolsize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub type PSYM_ENUMSYMBOLS_CALLBACKW = unsafe extern "system" fn(symbolname: super::super::super::Foundation::PWSTR, symboladdress: u32, symbolsize: u32, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub type PTRANSLATE_ADDRESS_ROUTINE = unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, hthread: super::super::super::Foundation::HANDLE, lpaddr: *mut ADDRESS) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PTRANSLATE_ADDRESS_ROUTINE64 = unsafe extern "system" fn(hprocess: super::super::super::Foundation::HANDLE, hthread: super::super::super::Foundation::HANDLE, lpaddr: *const ADDRESS64) -> u64;
pub const PTR_SEARCH_NO_SYMBOL_CHECK: u32 = 2147483648u32;
pub const PTR_SEARCH_PHYS_ALL_HITS: u32 = 1u32;
pub const PTR_SEARCH_PHYS_PTE: u32 = 2u32;
pub const PTR_SEARCH_PHYS_RANGE_CHECK_ONLY: u32 = 4u32;
pub const PTR_SEARCH_PHYS_SIZE_SHIFT: u32 = 3u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type PVECTORED_EXCEPTION_HANDLER = unsafe extern "system" fn(exceptioninfo: *mut EXCEPTION_POINTERS) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PWAITCHAINCALLBACK = unsafe extern "system" fn(wcthandle: *mut ::core::ffi::c_void, context: usize, callbackstatus: u32, nodecount: *mut u32, nodeinfoarray: *mut WAITCHAIN_NODE_INFO, iscycle: *mut i32);
pub type PWINDBG_CHECK_CONTROL_C = unsafe extern "system" fn() -> u32;
pub type PWINDBG_CHECK_VERSION = unsafe extern "system" fn() -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PWINDBG_DISASM = unsafe extern "system" fn(lpoffset: *mut usize, lpbuffer: super::super::super::Foundation::PSTR, fshoweffectiveaddress: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PWINDBG_DISASM32 = unsafe extern "system" fn(lpoffset: *mut u32, lpbuffer: super::super::super::Foundation::PSTR, fshoweffectiveaddress: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PWINDBG_DISASM64 = unsafe extern "system" fn(lpoffset: *mut u64, lpbuffer: super::super::super::Foundation::PSTR, fshoweffectiveaddress: u32) -> u32;
pub type PWINDBG_EXTENSION_API_VERSION = unsafe extern "system" fn() -> *mut EXT_API_VERSION;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type PWINDBG_EXTENSION_DLL_INIT = unsafe extern "system" fn(lpextensionapis: *mut WINDBG_EXTENSION_APIS, majorversion: u16, minorversion: u16);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type PWINDBG_EXTENSION_DLL_INIT32 = unsafe extern "system" fn(lpextensionapis: *mut WINDBG_EXTENSION_APIS32, majorversion: u16, minorversion: u16);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type PWINDBG_EXTENSION_DLL_INIT64 = unsafe extern "system" fn(lpextensionapis: *mut WINDBG_EXTENSION_APIS64, majorversion: u16, minorversion: u16);
#[cfg(feature = "Win32_Foundation")]
pub type PWINDBG_EXTENSION_ROUTINE = unsafe extern "system" fn(hcurrentprocess: super::super::super::Foundation::HANDLE, hcurrentthread: super::super::super::Foundation::HANDLE, dwcurrentpc: u32, dwprocessor: u32, lpargumentstring: super::super::super::Foundation::PSTR);
#[cfg(feature = "Win32_Foundation")]
pub type PWINDBG_EXTENSION_ROUTINE32 = unsafe extern "system" fn(hcurrentprocess: super::super::super::Foundation::HANDLE, hcurrentthread: super::super::super::Foundation::HANDLE, dwcurrentpc: u32, dwprocessor: u32, lpargumentstring: super::super::super::Foundation::PSTR);
#[cfg(feature = "Win32_Foundation")]
pub type PWINDBG_EXTENSION_ROUTINE64 = unsafe extern "system" fn(hcurrentprocess: super::super::super::Foundation::HANDLE, hcurrentthread: super::super::super::Foundation::HANDLE, dwcurrentpc: u64, dwprocessor: u32, lpargumentstring: super::super::super::Foundation::PSTR);
#[cfg(feature = "Win32_Foundation")]
pub type PWINDBG_GET_EXPRESSION = unsafe extern "system" fn(lpexpression: super::super::super::Foundation::PSTR) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type PWINDBG_GET_EXPRESSION32 = unsafe extern "system" fn(lpexpression: super::super::super::Foundation::PSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PWINDBG_GET_EXPRESSION64 = unsafe extern "system" fn(lpexpression: super::super::super::Foundation::PSTR) -> u64;
#[cfg(feature = "Win32_Foundation")]
pub type PWINDBG_GET_SYMBOL = unsafe extern "system" fn(offset: *mut ::core::ffi::c_void, pchbuffer: super::super::super::Foundation::PSTR, pdisplacement: *mut usize);
#[cfg(feature = "Win32_Foundation")]
pub type PWINDBG_GET_SYMBOL32 = unsafe extern "system" fn(offset: u32, pchbuffer: super::super::super::Foundation::PSTR, pdisplacement: *mut u32);
#[cfg(feature = "Win32_Foundation")]
pub type PWINDBG_GET_SYMBOL64 = unsafe extern "system" fn(offset: u64, pchbuffer: super::super::super::Foundation::PSTR, pdisplacement: *mut u64);
#[cfg(feature = "Win32_System_Kernel")]
pub type PWINDBG_GET_THREAD_CONTEXT_ROUTINE = unsafe extern "system" fn(processor: u32, lpcontext: *mut CONTEXT, cbsizeofcontext: u32) -> u32;
pub type PWINDBG_IOCTL_ROUTINE = unsafe extern "system" fn(ioctltype: u16, lpvdata: *mut ::core::ffi::c_void, cbsize: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PWINDBG_OLDKD_EXTENSION_ROUTINE = unsafe extern "system" fn(dwcurrentpc: u32, lpextensionapis: *mut WINDBG_OLDKD_EXTENSION_APIS, lpargumentstring: super::super::super::Foundation::PSTR);
pub type PWINDBG_OLDKD_READ_PHYSICAL_MEMORY = unsafe extern "system" fn(address: u64, buffer: *mut ::core::ffi::c_void, count: u32, bytesread: *mut u32) -> u32;
pub type PWINDBG_OLDKD_WRITE_PHYSICAL_MEMORY = unsafe extern "system" fn(address: u64, buffer: *mut ::core::ffi::c_void, length: u32, byteswritten: *mut u32) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub type PWINDBG_OLD_EXTENSION_ROUTINE = unsafe extern "system" fn(dwcurrentpc: u32, lpextensionapis: *mut WINDBG_EXTENSION_APIS, lpargumentstring: super::super::super::Foundation::PSTR);
#[cfg(feature = "Win32_Foundation")]
pub type PWINDBG_OUTPUT_ROUTINE = unsafe extern "system" fn(lpformat: super::super::super::Foundation::PSTR);
pub type PWINDBG_READ_PROCESS_MEMORY_ROUTINE = unsafe extern "system" fn(offset: usize, lpbuffer: *mut ::core::ffi::c_void, cb: u32, lpcbbytesread: *mut u32) -> u32;
pub type PWINDBG_READ_PROCESS_MEMORY_ROUTINE32 = unsafe extern "system" fn(offset: u32, lpbuffer: *mut ::core::ffi::c_void, cb: u32, lpcbbytesread: *mut u32) -> u32;
pub type PWINDBG_READ_PROCESS_MEMORY_ROUTINE64 = unsafe extern "system" fn(offset: u64, lpbuffer: *mut ::core::ffi::c_void, cb: u32, lpcbbytesread: *mut u32) -> u32;
#[cfg(feature = "Win32_System_Kernel")]
pub type PWINDBG_SET_THREAD_CONTEXT_ROUTINE = unsafe extern "system" fn(processor: u32, lpcontext: *mut CONTEXT, cbsizeofcontext: u32) -> u32;
pub type PWINDBG_STACKTRACE_ROUTINE = unsafe extern "system" fn(framepointer: u32, stackpointer: u32, programcounter: u32, stackframes: *mut EXTSTACKTRACE, frames: u32) -> u32;
pub type PWINDBG_STACKTRACE_ROUTINE32 = unsafe extern "system" fn(framepointer: u32, stackpointer: u32, programcounter: u32, stackframes: *mut EXTSTACKTRACE32, frames: u32) -> u32;
pub type PWINDBG_STACKTRACE_ROUTINE64 = unsafe extern "system" fn(framepointer: u64, stackpointer: u64, programcounter: u64, stackframes: *mut EXTSTACKTRACE64, frames: u32) -> u32;
pub type PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE = unsafe extern "system" fn(offset: usize, lpbuffer: *const ::core::ffi::c_void, cb: u32, lpcbbyteswritten: *mut u32) -> u32;
pub type PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE32 = unsafe extern "system" fn(offset: u32, lpbuffer: *const ::core::ffi::c_void, cb: u32, lpcbbyteswritten: *mut u32) -> u32;
pub type PWINDBG_WRITE_PROCESS_MEMORY_ROUTINE64 = unsafe extern "system" fn(offset: u64, lpbuffer: *const ::core::ffi::c_void, cb: u32, lpcbbyteswritten: *mut u32) -> u32;
#[repr(transparent)]
pub struct PointerKind(pub i32);
pub const PointerStandard: PointerKind = PointerKind(0i32);
pub const PointerReference: PointerKind = PointerKind(1i32);
pub const PointerRValueReference: PointerKind = PointerKind(2i32);
pub const PointerCXHat: PointerKind = PointerKind(3i32);
pub const PointerManagedReference: PointerKind = PointerKind(4i32);
#[repr(transparent)]
pub struct PreferredFormat(pub i32);
pub const FormatNone: PreferredFormat = PreferredFormat(0i32);
pub const FormatSingleCharacter: PreferredFormat = PreferredFormat(1i32);
pub const FormatQuotedString: PreferredFormat = PreferredFormat(2i32);
pub const FormatString: PreferredFormat = PreferredFormat(3i32);
pub const FormatQuotedUnicodeString: PreferredFormat = PreferredFormat(4i32);
pub const FormatUnicodeString: PreferredFormat = PreferredFormat(5i32);
pub const FormatQuotedUTF8String: PreferredFormat = PreferredFormat(6i32);
pub const FormatUTF8String: PreferredFormat = PreferredFormat(7i32);
pub const FormatBSTRString: PreferredFormat = PreferredFormat(8i32);
pub const FormatQuotedHString: PreferredFormat = PreferredFormat(9i32);
pub const FormatHString: PreferredFormat = PreferredFormat(10i32);
pub const FormatRaw: PreferredFormat = PreferredFormat(11i32);
pub const FormatEnumNameOnly: PreferredFormat = PreferredFormat(12i32);
pub const FormatEscapedStringWithQuote: PreferredFormat = PreferredFormat(13i32);
pub const FormatUTF32String: PreferredFormat = PreferredFormat(14i32);
pub const FormatQuotedUTF32String: PreferredFormat = PreferredFormat(15i32);
pub const ProcessDebugManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2024085538, data2: 20980, data3: 4560, data4: [143, 32, 0, 128, 95, 44, 208, 100] };
#[repr(C)]
pub struct READCONTROLSPACE(i32);
#[repr(C)]
pub struct READCONTROLSPACE32(i32);
#[repr(C)]
pub struct READCONTROLSPACE64(i32);
#[repr(C)]
pub struct READ_WRITE_MSR(i32);
#[repr(C)]
pub struct RIP_INFO(i32);
#[repr(transparent)]
pub struct RIP_INFO_TYPE(pub u32);
pub const SLE_ERROR: RIP_INFO_TYPE = RIP_INFO_TYPE(1u32);
pub const SLE_MINORERROR: RIP_INFO_TYPE = RIP_INFO_TYPE(2u32);
pub const SLE_WARNING: RIP_INFO_TYPE = RIP_INFO_TYPE(3u32);
#[repr(transparent)]
pub struct RTL_VIRTUAL_UNWIND_HANDLER_TYPE(pub u32);
pub const UNW_FLAG_NHANDLER: RTL_VIRTUAL_UNWIND_HANDLER_TYPE = RTL_VIRTUAL_UNWIND_HANDLER_TYPE(0u32);
pub const UNW_FLAG_EHANDLER: RTL_VIRTUAL_UNWIND_HANDLER_TYPE = RTL_VIRTUAL_UNWIND_HANDLER_TYPE(1u32);
pub const UNW_FLAG_UHANDLER: RTL_VIRTUAL_UNWIND_HANDLER_TYPE = RTL_VIRTUAL_UNWIND_HANDLER_TYPE(2u32);
pub const UNW_FLAG_CHAININFO: RTL_VIRTUAL_UNWIND_HANDLER_TYPE = RTL_VIRTUAL_UNWIND_HANDLER_TYPE(4u32);
#[repr(transparent)]
pub struct RawSearchFlags(pub i32);
pub const RawSearchNone: RawSearchFlags = RawSearchFlags(0i32);
pub const RawSearchNoBases: RawSearchFlags = RawSearchFlags(1i32);
#[repr(transparent)]
pub struct SCRIPTGCTYPE(pub i32);
pub const SCRIPTGCTYPE_NORMAL: SCRIPTGCTYPE = SCRIPTGCTYPE(0i32);
pub const SCRIPTGCTYPE_EXHAUSTIVE: SCRIPTGCTYPE = SCRIPTGCTYPE(1i32);
pub const SCRIPTINFO_ITYPEINFO: u32 = 2u32;
pub const SCRIPTINFO_IUNKNOWN: u32 = 1u32;
pub const SCRIPTINTERRUPT_DEBUG: u32 = 1u32;
pub const SCRIPTINTERRUPT_RAISEEXCEPTION: u32 = 2u32;
pub const SCRIPTITEM_CODEONLY: u32 = 512u32;
pub const SCRIPTITEM_GLOBALMEMBERS: u32 = 8u32;
pub const SCRIPTITEM_ISPERSISTENT: u32 = 64u32;
pub const SCRIPTITEM_ISSOURCE: u32 = 4u32;
pub const SCRIPTITEM_ISVISIBLE: u32 = 2u32;
pub const SCRIPTITEM_NOCODE: u32 = 1024u32;
#[repr(transparent)]
pub struct SCRIPTLANGUAGEVERSION(pub i32);
pub const SCRIPTLANGUAGEVERSION_DEFAULT: SCRIPTLANGUAGEVERSION = SCRIPTLANGUAGEVERSION(0i32);
pub const SCRIPTLANGUAGEVERSION_5_7: SCRIPTLANGUAGEVERSION = SCRIPTLANGUAGEVERSION(1i32);
pub const SCRIPTLANGUAGEVERSION_5_8: SCRIPTLANGUAGEVERSION = SCRIPTLANGUAGEVERSION(2i32);
pub const SCRIPTLANGUAGEVERSION_MAX: SCRIPTLANGUAGEVERSION = SCRIPTLANGUAGEVERSION(255i32);
pub const SCRIPTPROC_HOSTMANAGESSOURCE: u32 = 128u32;
pub const SCRIPTPROC_IMPLICIT_PARENTS: u32 = 512u32;
pub const SCRIPTPROC_IMPLICIT_THIS: u32 = 256u32;
pub const SCRIPTPROC_ISEXPRESSION: u32 = 32u32;
pub const SCRIPTPROC_ISXDOMAIN: u32 = 1024u32;
pub const SCRIPTPROP_ABBREVIATE_GLOBALNAME_RESOLUTION: u32 = 1879048194u32;
pub const SCRIPTPROP_BUILDNUMBER: u32 = 3u32;
pub const SCRIPTPROP_CATCHEXCEPTION: u32 = 4097u32;
pub const SCRIPTPROP_CONVERSIONLCID: u32 = 4098u32;
pub const SCRIPTPROP_DEBUGGER: u32 = 4352u32;
pub const SCRIPTPROP_DELAYEDEVENTSINKING: u32 = 4096u32;
pub const SCRIPTPROP_GCCONTROLSOFTCLOSE: u32 = 8192u32;
pub const SCRIPTPROP_HACK_FIBERSUPPORT: u32 = 1879048192u32;
pub const SCRIPTPROP_HACK_TRIDENTEVENTSINK: u32 = 1879048193u32;
pub const SCRIPTPROP_HOSTKEEPALIVE: u32 = 1879048196u32;
pub const SCRIPTPROP_HOSTSTACKREQUIRED: u32 = 4099u32;
pub const SCRIPTPROP_INTEGERMODE: u32 = 12288u32;
pub const SCRIPTPROP_INVOKEVERSIONING: u32 = 16384u32;
pub const SCRIPTPROP_JITDEBUG: u32 = 4353u32;
pub const SCRIPTPROP_MAJORVERSION: u32 = 1u32;
pub const SCRIPTPROP_MINORVERSION: u32 = 2u32;
pub const SCRIPTPROP_NAME: u32 = 0u32;
pub const SCRIPTPROP_SCRIPTSAREFULLYTRUSTED: u32 = 4100u32;
pub const SCRIPTPROP_STRINGCOMPAREINSTANCE: u32 = 12289u32;
#[repr(transparent)]
pub struct SCRIPTSTATE(pub i32);
pub const SCRIPTSTATE_UNINITIALIZED: SCRIPTSTATE = SCRIPTSTATE(0i32);
pub const SCRIPTSTATE_INITIALIZED: SCRIPTSTATE = SCRIPTSTATE(5i32);
pub const SCRIPTSTATE_STARTED: SCRIPTSTATE = SCRIPTSTATE(1i32);
pub const SCRIPTSTATE_CONNECTED: SCRIPTSTATE = SCRIPTSTATE(2i32);
pub const SCRIPTSTATE_DISCONNECTED: SCRIPTSTATE = SCRIPTSTATE(3i32);
pub const SCRIPTSTATE_CLOSED: SCRIPTSTATE = SCRIPTSTATE(4i32);
pub const SCRIPTSTAT_INSTRUCTION_COUNT: u32 = 2u32;
pub const SCRIPTSTAT_INTSTRUCTION_TIME: u32 = 3u32;
pub const SCRIPTSTAT_STATEMENT_COUNT: u32 = 1u32;
pub const SCRIPTSTAT_TOTAL_TIME: u32 = 4u32;
pub const SCRIPTTEXT_DELAYEXECUTION: u32 = 1u32;
pub const SCRIPTTEXT_HOSTMANAGESSOURCE: u32 = 128u32;
pub const SCRIPTTEXT_ISEXPRESSION: u32 = 32u32;
pub const SCRIPTTEXT_ISNONUSERCODE: u32 = 512u32;
pub const SCRIPTTEXT_ISPERSISTENT: u32 = 64u32;
pub const SCRIPTTEXT_ISVISIBLE: u32 = 2u32;
pub const SCRIPTTEXT_ISXDOMAIN: u32 = 256u32;
#[repr(transparent)]
pub struct SCRIPTTHREADSTATE(pub i32);
pub const SCRIPTTHREADSTATE_NOTINSCRIPT: SCRIPTTHREADSTATE = SCRIPTTHREADSTATE(0i32);
pub const SCRIPTTHREADSTATE_RUNNING: SCRIPTTHREADSTATE = SCRIPTTHREADSTATE(1i32);
#[repr(transparent)]
pub struct SCRIPTTRACEINFO(pub i32);
pub const SCRIPTTRACEINFO_SCRIPTSTART: SCRIPTTRACEINFO = SCRIPTTRACEINFO(0i32);
pub const SCRIPTTRACEINFO_SCRIPTEND: SCRIPTTRACEINFO = SCRIPTTRACEINFO(1i32);
pub const SCRIPTTRACEINFO_COMCALLSTART: SCRIPTTRACEINFO = SCRIPTTRACEINFO(2i32);
pub const SCRIPTTRACEINFO_COMCALLEND: SCRIPTTRACEINFO = SCRIPTTRACEINFO(3i32);
pub const SCRIPTTRACEINFO_CREATEOBJSTART: SCRIPTTRACEINFO = SCRIPTTRACEINFO(4i32);
pub const SCRIPTTRACEINFO_CREATEOBJEND: SCRIPTTRACEINFO = SCRIPTTRACEINFO(5i32);
pub const SCRIPTTRACEINFO_GETOBJSTART: SCRIPTTRACEINFO = SCRIPTTRACEINFO(6i32);
pub const SCRIPTTRACEINFO_GETOBJEND: SCRIPTTRACEINFO = SCRIPTTRACEINFO(7i32);
pub const SCRIPTTYPELIB_ISCONTROL: u32 = 16u32;
pub const SCRIPTTYPELIB_ISPERSISTENT: u32 = 64u32;
#[repr(transparent)]
pub struct SCRIPTUICHANDLING(pub i32);
pub const SCRIPTUICHANDLING_ALLOW: SCRIPTUICHANDLING = SCRIPTUICHANDLING(0i32);
pub const SCRIPTUICHANDLING_NOUIERROR: SCRIPTUICHANDLING = SCRIPTUICHANDLING(1i32);
pub const SCRIPTUICHANDLING_NOUIDEFAULT: SCRIPTUICHANDLING = SCRIPTUICHANDLING(2i32);
#[repr(transparent)]
pub struct SCRIPTUICITEM(pub i32);
pub const SCRIPTUICITEM_INPUTBOX: SCRIPTUICITEM = SCRIPTUICITEM(1i32);
pub const SCRIPTUICITEM_MSGBOX: SCRIPTUICITEM = SCRIPTUICITEM(2i32);
pub const SCRIPT_CMPL_COMMIT: u32 = 4u32;
pub const SCRIPT_CMPL_ENUMLIST: u32 = 2u32;
pub const SCRIPT_CMPL_ENUM_TRIGGER: u32 = 1u32;
pub const SCRIPT_CMPL_GLOBALLIST: u32 = 8u32;
pub const SCRIPT_CMPL_MEMBERLIST: u32 = 1u32;
pub const SCRIPT_CMPL_MEMBER_TRIGGER: u32 = 2u32;
pub const SCRIPT_CMPL_NOLIST: u32 = 0u32;
pub const SCRIPT_CMPL_PARAMTIP: u32 = 4u32;
pub const SCRIPT_CMPL_PARAM_TRIGGER: u32 = 3u32;
#[repr(transparent)]
pub struct SCRIPT_DEBUGGER_OPTIONS(pub i32);
pub const SDO_NONE: SCRIPT_DEBUGGER_OPTIONS = SCRIPT_DEBUGGER_OPTIONS(0i32);
pub const SDO_ENABLE_FIRST_CHANCE_EXCEPTIONS: SCRIPT_DEBUGGER_OPTIONS = SCRIPT_DEBUGGER_OPTIONS(1i32);
pub const SDO_ENABLE_WEB_WORKER_SUPPORT: SCRIPT_DEBUGGER_OPTIONS = SCRIPT_DEBUGGER_OPTIONS(2i32);
pub const SDO_ENABLE_NONUSER_CODE_SUPPORT: SCRIPT_DEBUGGER_OPTIONS = SCRIPT_DEBUGGER_OPTIONS(4i32);
pub const SDO_ENABLE_LIBRARY_STACK_FRAME: SCRIPT_DEBUGGER_OPTIONS = SCRIPT_DEBUGGER_OPTIONS(8i32);
pub const SCRIPT_ENCODE_DEFAULT_LANGUAGE: u32 = 1u32;
pub const SCRIPT_ENCODE_NO_ASP_LANGUAGE: u32 = 2u32;
pub const SCRIPT_ENCODE_SECTION: u32 = 1u32;
#[repr(transparent)]
pub struct SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND(pub i32);
pub const ETK_FIRST_CHANCE: SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND = SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND(0i32);
pub const ETK_USER_UNHANDLED: SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND = SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND(1i32);
pub const ETK_UNHANDLED: SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND = SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND(2i32);
pub const SCRIPT_E_PROPAGATE: i32 = -2147352318i32;
pub const SCRIPT_E_RECORDED: i32 = -2040119292i32;
pub const SCRIPT_E_REPORTED: i32 = -2147352319i32;
#[repr(transparent)]
pub struct SCRIPT_INVOCATION_CONTEXT_TYPE(pub i32);
pub const SICT_Event: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(0i32);
pub const SICT_SetTimeout: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(1i32);
pub const SICT_SetInterval: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(2i32);
pub const SICT_SetImmediate: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(3i32);
pub const SICT_RequestAnimationFrame: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(4i32);
pub const SICT_ToString: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(5i32);
pub const SICT_MutationObserverCheckpoint: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(6i32);
pub const SICT_WWAExecUnsafeLocalFunction: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(7i32);
pub const SICT_WWAExecAtPriority: SCRIPT_INVOCATION_CONTEXT_TYPE = SCRIPT_INVOCATION_CONTEXT_TYPE(8i32);
#[repr(C)]
pub struct SEARCHMEMORY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SOURCEFILE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SOURCEFILEW(i32);
pub const SOURCETEXT_ATTR_COMMENT: u32 = 2u32;
pub const SOURCETEXT_ATTR_FUNCTION_START: u32 = 64u32;
pub const SOURCETEXT_ATTR_HUMANTEXT: u32 = 32768u32;
pub const SOURCETEXT_ATTR_IDENTIFIER: u32 = 256u32;
pub const SOURCETEXT_ATTR_KEYWORD: u32 = 1u32;
pub const SOURCETEXT_ATTR_MEMBERLOOKUP: u32 = 512u32;
pub const SOURCETEXT_ATTR_NONSOURCE: u32 = 4u32;
pub const SOURCETEXT_ATTR_NUMBER: u32 = 16u32;
pub const SOURCETEXT_ATTR_OPERATOR: u32 = 8u32;
pub const SOURCETEXT_ATTR_STRING: u32 = 32u32;
pub const SOURCETEXT_ATTR_THIS: u32 = 1024u32;
pub const SPLITSYM_EXTRACT_ALL: u32 = 2u32;
pub const SPLITSYM_REMOVE_PRIVATE: u32 = 1u32;
pub const SPLITSYM_SYMBOLPATH_IS_SRC: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SRCCODEINFO(i32);
#[repr(C)]
pub struct SRCCODEINFOW(i32);
pub const SSRVACTION_CHECKSUMSTATUS: u32 = 8u32;
pub const SSRVACTION_EVENT: u32 = 3u32;
pub const SSRVACTION_EVENTW: u32 = 4u32;
pub const SSRVACTION_HTTPSTATUS: u32 = 6u32;
pub const SSRVACTION_QUERYCANCEL: u32 = 2u32;
pub const SSRVACTION_SIZE: u32 = 5u32;
pub const SSRVACTION_TRACE: u32 = 1u32;
pub const SSRVACTION_XMLOUTPUT: u32 = 7u32;
pub const SSRVOPT_CALLBACK: u32 = 1u32;
pub const SSRVOPT_CALLBACKW: u32 = 65536u32;
pub const SSRVOPT_DISABLE_PING_HOST: u32 = 67108864u32;
pub const SSRVOPT_DISABLE_TIMEOUT: u32 = 134217728u32;
pub const SSRVOPT_DONT_UNCOMPRESS: u32 = 33554432u32;
pub const SSRVOPT_DOWNSTREAM_STORE: u32 = 8192u32;
pub const SSRVOPT_ENABLE_COMM_MSG: u32 = 268435456u32;
pub const SSRVOPT_FAVOR_COMPRESSED: u32 = 2097152u32;
pub const SSRVOPT_FLAT_DEFAULT_STORE: u32 = 131072u32;
pub const SSRVOPT_GETPATH: u32 = 64u32;
pub const SSRVOPT_MAX: u32 = 2147483648u32;
pub const SSRVOPT_MESSAGE: u32 = 524288u32;
pub const SSRVOPT_NOCOPY: u32 = 64u32;
pub const SSRVOPT_OLDGUIDPTR: u32 = 16u32;
pub const SSRVOPT_OVERWRITE: u32 = 16384u32;
pub const SSRVOPT_PARAMTYPE: u32 = 256u32;
pub const SSRVOPT_PARENTWIN: u32 = 128u32;
pub const SSRVOPT_PROXY: u32 = 4096u32;
pub const SSRVOPT_PROXYW: u32 = 262144u32;
pub const SSRVOPT_RESETTOU: u32 = 32768u32;
pub const SSRVOPT_RETRY_APP_HANG: u32 = 2147483648u32;
pub const SSRVOPT_SECURE: u32 = 512u32;
pub const SSRVOPT_SERVICE: u32 = 1048576u32;
pub const SSRVOPT_SETCONTEXT: u32 = 2048u32;
pub const SSRVOPT_STRING: u32 = 4194304u32;
pub const SSRVOPT_TRACE: u32 = 1024u32;
pub const SSRVOPT_UNATTENDED: u32 = 32u32;
pub const SSRVOPT_URI_FILTER: u32 = 536870912u32;
pub const SSRVOPT_URI_TIERS: u32 = 1073741824u32;
pub const SSRVOPT_WINHTTP: u32 = 8388608u32;
pub const SSRVOPT_WININET: u32 = 16777216u32;
pub const SSRVURI_ALL: u32 = 255u32;
pub const SSRVURI_COMPRESSED: u32 = 2u32;
pub const SSRVURI_FILEPTR: u32 = 4u32;
pub const SSRVURI_HTTP_COMPRESSED: u32 = 2u32;
pub const SSRVURI_HTTP_FILEPTR: u32 = 4u32;
pub const SSRVURI_HTTP_MASK: u32 = 15u32;
pub const SSRVURI_HTTP_NORMAL: u32 = 1u32;
pub const SSRVURI_NORMAL: u32 = 1u32;
pub const SSRVURI_UNC_COMPRESSED: u32 = 32u32;
pub const SSRVURI_UNC_FILEPTR: u32 = 64u32;
pub const SSRVURI_UNC_MASK: u32 = 240u32;
pub const SSRVURI_UNC_NORMAL: u32 = 16u32;
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STACKFRAME(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STACKFRAME64(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STACKFRAME_EX(i32);
pub const STACK_FRAME_TYPE_IGNORE: u32 = 255u32;
pub const STACK_FRAME_TYPE_INIT: u32 = 0u32;
pub const STACK_FRAME_TYPE_INLINE: u32 = 2u32;
pub const STACK_FRAME_TYPE_RA: u32 = 128u32;
pub const STACK_FRAME_TYPE_STACK: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STACK_SRC_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STACK_SYM_FRAME_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub type SYMADDSOURCESTREAM = unsafe extern "system" fn(param0: super::super::super::Foundation::HANDLE, param1: u64, param2: super::super::super::Foundation::PSTR, param3: *mut u8, param4: usize) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type SYMADDSOURCESTREAMA = unsafe extern "system" fn(param0: super::super::super::Foundation::HANDLE, param1: u64, param2: super::super::super::Foundation::PSTR, param3: *mut u8, param4: usize) -> super::super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SYMBOL_INFO(i32);
#[repr(C)]
pub struct SYMBOL_INFOW(i32);
#[repr(C)]
pub struct SYMBOL_INFO_EX(i32);
#[repr(transparent)]
pub struct SYMBOL_INFO_FLAGS(pub u32);
pub const SYMFLAG_CLR_TOKEN: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(262144u32);
pub const SYMFLAG_CONSTANT: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(256u32);
pub const SYMFLAG_EXPORT: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(512u32);
pub const SYMFLAG_FORWARDER: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(1024u32);
pub const SYMFLAG_FRAMEREL: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(32u32);
pub const SYMFLAG_FUNCTION: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(2048u32);
pub const SYMFLAG_ILREL: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(65536u32);
pub const SYMFLAG_LOCAL: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(128u32);
pub const SYMFLAG_METADATA: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(131072u32);
pub const SYMFLAG_PARAMETER: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(64u32);
pub const SYMFLAG_REGISTER: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(8u32);
pub const SYMFLAG_REGREL: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(16u32);
pub const SYMFLAG_SLOT: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(32768u32);
pub const SYMFLAG_THUNK: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(8192u32);
pub const SYMFLAG_TLSREL: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(16384u32);
pub const SYMFLAG_VALUEPRESENT: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(1u32);
pub const SYMFLAG_VIRTUAL: SYMBOL_INFO_FLAGS = SYMBOL_INFO_FLAGS(4096u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SYMBOL_INFO_PACKAGE(i32);
#[repr(C)]
pub struct SYMBOL_INFO_PACKAGEW(i32);
pub const SYMBOL_TYPE_INDEX_NOT_FOUND: u32 = 2u32;
pub const SYMBOL_TYPE_INFO_NOT_FOUND: u32 = 3u32;
pub const SYMENUM_OPTIONS_DEFAULT: u32 = 1u32;
pub const SYMENUM_OPTIONS_INLINE: u32 = 2u32;
pub const SYMFLAG_FIXUP_ARM64X: u32 = 16777216u32;
pub const SYMFLAG_FUNC_NO_RETURN: u32 = 1048576u32;
pub const SYMFLAG_GLOBAL: u32 = 33554432u32;
pub const SYMFLAG_NULL: u32 = 524288u32;
pub const SYMFLAG_PUBLIC_CODE: u32 = 4194304u32;
pub const SYMFLAG_REGREL_ALIASINDIR: u32 = 8388608u32;
pub const SYMFLAG_RESET: u32 = 2147483648u32;
pub const SYMFLAG_SYNTHETIC_ZEROBASE: u32 = 2097152u32;
pub const SYMF_CONSTANT: u32 = 256u32;
pub const SYMF_EXPORT: u32 = 512u32;
pub const SYMF_FORWARDER: u32 = 1024u32;
pub const SYMF_FRAMEREL: u32 = 32u32;
pub const SYMF_FUNCTION: u32 = 2048u32;
pub const SYMF_LOCAL: u32 = 128u32;
pub const SYMF_OMAP_GENERATED: u32 = 1u32;
pub const SYMF_OMAP_MODIFIED: u32 = 2u32;
pub const SYMF_PARAMETER: u32 = 64u32;
pub const SYMF_REGISTER: u32 = 8u32;
pub const SYMF_REGREL: u32 = 16u32;
pub const SYMF_THUNK: u32 = 8192u32;
pub const SYMF_TLSREL: u32 = 16384u32;
pub const SYMF_VIRTUAL: u32 = 4096u32;
pub const SYMOPT_ALLOW_ABSOLUTE_SYMBOLS: u32 = 2048u32;
pub const SYMOPT_ALLOW_ZERO_ADDRESS: u32 = 16777216u32;
pub const SYMOPT_AUTO_PUBLICS: u32 = 65536u32;
pub const SYMOPT_CASE_INSENSITIVE: u32 = 1u32;
pub const SYMOPT_DEBUG: u32 = 2147483648u32;
pub const SYMOPT_DEFERRED_LOADS: u32 = 4u32;
pub const SYMOPT_DISABLE_FAST_SYMBOLS: u32 = 268435456u32;
pub const SYMOPT_DISABLE_SRVSTAR_ON_STARTUP: u32 = 1073741824u32;
pub const SYMOPT_DISABLE_SYMSRV_AUTODETECT: u32 = 33554432u32;
pub const SYMOPT_DISABLE_SYMSRV_TIMEOUT: u32 = 536870912u32;
pub const SYMOPT_EXACT_SYMBOLS: u32 = 1024u32;
pub const SYMOPT_FAIL_CRITICAL_ERRORS: u32 = 512u32;
pub const SYMOPT_FAVOR_COMPRESSED: u32 = 8388608u32;
pub const SYMOPT_FLAT_DIRECTORY: u32 = 4194304u32;
pub const SYMOPT_IGNORE_CVREC: u32 = 128u32;
pub const SYMOPT_IGNORE_IMAGEDIR: u32 = 2097152u32;
pub const SYMOPT_IGNORE_NT_SYMPATH: u32 = 4096u32;
pub const SYMOPT_INCLUDE_32BIT_MODULES: u32 = 8192u32;
pub const SYMOPT_LOAD_ANYTHING: u32 = 64u32;
pub const SYMOPT_LOAD_LINES: u32 = 16u32;
pub const SYMOPT_NO_CPP: u32 = 8u32;
pub const SYMOPT_NO_IMAGE_SEARCH: u32 = 131072u32;
pub const SYMOPT_NO_PROMPTS: u32 = 524288u32;
pub const SYMOPT_NO_PUBLICS: u32 = 32768u32;
pub const SYMOPT_NO_UNQUALIFIED_LOADS: u32 = 256u32;
pub const SYMOPT_OMAP_FIND_NEAREST: u32 = 32u32;
pub const SYMOPT_OVERWRITE: u32 = 1048576u32;
pub const SYMOPT_PUBLICS_ONLY: u32 = 16384u32;
pub const SYMOPT_READONLY_CACHE: u32 = 67108864u32;
pub const SYMOPT_SECURE: u32 = 262144u32;
pub const SYMOPT_SYMPATH_LAST: u32 = 134217728u32;
pub const SYMOPT_UNDNAME: u32 = 2u32;
pub const SYMSEARCH_ALLITEMS: u32 = 8u32;
pub const SYMSEARCH_GLOBALSONLY: u32 = 4u32;
pub const SYMSEARCH_MASKOBJS: u32 = 1u32;
pub const SYMSEARCH_RECURSE: u32 = 2u32;
#[repr(C)]
pub struct SYMSRV_EXTENDED_OUTPUT_DATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SYMSRV_INDEX_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SYMSRV_INDEX_INFOW(i32);
pub const SYMSRV_VERSION: u32 = 2u32;
pub const SYMSTOREOPT_ALT_INDEX: u32 = 16u32;
pub const SYMSTOREOPT_UNICODE: u32 = 32u32;
#[repr(C)]
pub struct SYM_DUMP_PARAM(i32);
#[repr(transparent)]
pub struct SYM_FIND_ID_OPTION(pub u32);
pub const SSRVOPT_DWORD: SYM_FIND_ID_OPTION = SYM_FIND_ID_OPTION(2u32);
pub const SSRVOPT_DWORDPTR: SYM_FIND_ID_OPTION = SYM_FIND_ID_OPTION(4u32);
pub const SSRVOPT_GUIDPTR: SYM_FIND_ID_OPTION = SYM_FIND_ID_OPTION(8u32);
pub const SYM_INLINE_COMP_DIFFERENT: u32 = 5u32;
pub const SYM_INLINE_COMP_ERROR: u32 = 0u32;
pub const SYM_INLINE_COMP_IDENTICAL: u32 = 1u32;
pub const SYM_INLINE_COMP_STEPIN: u32 = 2u32;
pub const SYM_INLINE_COMP_STEPOUT: u32 = 3u32;
pub const SYM_INLINE_COMP_STEPOVER: u32 = 4u32;
#[repr(transparent)]
pub struct SYM_LOAD_FLAGS(pub u32);
pub const SLMFLAG_NONE: SYM_LOAD_FLAGS = SYM_LOAD_FLAGS(0u32);
pub const SLMFLAG_VIRTUAL: SYM_LOAD_FLAGS = SYM_LOAD_FLAGS(1u32);
pub const SLMFLAG_ALT_INDEX: SYM_LOAD_FLAGS = SYM_LOAD_FLAGS(2u32);
pub const SLMFLAG_NO_SYMBOLS: SYM_LOAD_FLAGS = SYM_LOAD_FLAGS(4u32);
#[repr(transparent)]
pub struct SYM_SRV_STORE_FILE_FLAGS(pub u32);
pub const SYMSTOREOPT_COMPRESS: SYM_SRV_STORE_FILE_FLAGS = SYM_SRV_STORE_FILE_FLAGS(1u32);
pub const SYMSTOREOPT_OVERWRITE: SYM_SRV_STORE_FILE_FLAGS = SYM_SRV_STORE_FILE_FLAGS(2u32);
pub const SYMSTOREOPT_PASS_IF_EXISTS: SYM_SRV_STORE_FILE_FLAGS = SYM_SRV_STORE_FILE_FLAGS(64u32);
pub const SYMSTOREOPT_POINTER: SYM_SRV_STORE_FILE_FLAGS = SYM_SRV_STORE_FILE_FLAGS(8u32);
pub const SYMSTOREOPT_RETURNINDEX: SYM_SRV_STORE_FILE_FLAGS = SYM_SRV_STORE_FILE_FLAGS(4u32);
pub const SYM_STKWALK_DEFAULT: u32 = 0u32;
pub const SYM_STKWALK_FORCE_FRAMEPTR: u32 = 1u32;
pub const SYM_STKWALK_ZEROEXTEND_PTRS: u32 = 2u32;
#[repr(transparent)]
pub struct SYM_TYPE(pub i32);
pub const SymNone: SYM_TYPE = SYM_TYPE(0i32);
pub const SymCoff: SYM_TYPE = SYM_TYPE(1i32);
pub const SymCv: SYM_TYPE = SYM_TYPE(2i32);
pub const SymPdb: SYM_TYPE = SYM_TYPE(3i32);
pub const SymExport: SYM_TYPE = SYM_TYPE(4i32);
pub const SymDeferred: SYM_TYPE = SYM_TYPE(5i32);
pub const SymSym: SYM_TYPE = SYM_TYPE(6i32);
pub const SymDia: SYM_TYPE = SYM_TYPE(7i32);
pub const SymVirtual: SYM_TYPE = SYM_TYPE(8i32);
pub const NumSymTypes: SYM_TYPE = SYM_TYPE(9i32);
#[repr(transparent)]
pub struct ScriptChangeKind(pub i32);
pub const ScriptRename: ScriptChangeKind = ScriptChangeKind(0i32);
#[repr(transparent)]
pub struct ScriptDebugEvent(pub i32);
pub const ScriptDebugBreakpoint: ScriptDebugEvent = ScriptDebugEvent(0i32);
pub const ScriptDebugStep: ScriptDebugEvent = ScriptDebugEvent(1i32);
pub const ScriptDebugException: ScriptDebugEvent = ScriptDebugEvent(2i32);
pub const ScriptDebugAsyncBreak: ScriptDebugEvent = ScriptDebugEvent(3i32);
#[repr(transparent)]
pub struct ScriptDebugEventFilter(pub i32);
pub const ScriptDebugEventFilterEntry: ScriptDebugEventFilter = ScriptDebugEventFilter(0i32);
pub const ScriptDebugEventFilterException: ScriptDebugEventFilter = ScriptDebugEventFilter(1i32);
pub const ScriptDebugEventFilterUnhandledException: ScriptDebugEventFilter = ScriptDebugEventFilter(2i32);
pub const ScriptDebugEventFilterAbort: ScriptDebugEventFilter = ScriptDebugEventFilter(3i32);
#[repr(C)]
pub struct ScriptDebugEventInformation(i32);
#[repr(C)]
pub struct ScriptDebugPosition(i32);
#[repr(transparent)]
pub struct ScriptDebugState(pub i32);
pub const ScriptDebugNoDebugger: ScriptDebugState = ScriptDebugState(0i32);
pub const ScriptDebugNotExecuting: ScriptDebugState = ScriptDebugState(1i32);
pub const ScriptDebugExecuting: ScriptDebugState = ScriptDebugState(2i32);
pub const ScriptDebugBreak: ScriptDebugState = ScriptDebugState(3i32);
#[repr(transparent)]
pub struct ScriptExecutionKind(pub i32);
pub const ScriptExecutionNormal: ScriptExecutionKind = ScriptExecutionKind(0i32);
pub const ScriptExecutionStepIn: ScriptExecutionKind = ScriptExecutionKind(1i32);
pub const ScriptExecutionStepOut: ScriptExecutionKind = ScriptExecutionKind(2i32);
pub const ScriptExecutionStepOver: ScriptExecutionKind = ScriptExecutionKind(3i32);
#[repr(transparent)]
pub struct SignatureComparison(pub i32);
pub const Unrelated: SignatureComparison = SignatureComparison(0i32);
pub const Ambiguous: SignatureComparison = SignatureComparison(1i32);
pub const LessSpecific: SignatureComparison = SignatureComparison(2i32);
pub const MoreSpecific: SignatureComparison = SignatureComparison(3i32);
pub const Identical: SignatureComparison = SignatureComparison(4i32);
#[repr(transparent)]
pub struct SymbolKind(pub i32);
pub const Symbol: SymbolKind = SymbolKind(0i32);
pub const SymbolModule: SymbolKind = SymbolKind(1i32);
pub const SymbolType: SymbolKind = SymbolKind(2i32);
pub const SymbolField: SymbolKind = SymbolKind(3i32);
pub const SymbolConstant: SymbolKind = SymbolKind(4i32);
pub const SymbolData: SymbolKind = SymbolKind(5i32);
pub const SymbolBaseClass: SymbolKind = SymbolKind(6i32);
pub const SymbolPublic: SymbolKind = SymbolKind(7i32);
pub const SymbolFunction: SymbolKind = SymbolKind(8i32);
#[repr(transparent)]
pub struct SymbolSearchOptions(pub i32);
pub const SymbolSearchNone: SymbolSearchOptions = SymbolSearchOptions(0i32);
pub const SymbolSearchCompletion: SymbolSearchOptions = SymbolSearchOptions(1i32);
pub const SymbolSearchCaseInsensitive: SymbolSearchOptions = SymbolSearchOptions(2i32);
#[repr(C)]
pub struct TEXT_DOCUMENT_ARRAY(i32);
pub const TEXT_DOC_ATTR_READONLY: u32 = 1u32;
pub const TEXT_DOC_ATTR_TYPE_PRIMARY: u32 = 2u32;
pub const TEXT_DOC_ATTR_TYPE_SCRIPT: u32 = 8u32;
pub const TEXT_DOC_ATTR_TYPE_WORKER: u32 = 4u32;
pub const THREAD_BLOCKED: u32 = 4u32;
#[repr(transparent)]
pub struct THREAD_ERROR_MODE(pub u32);
pub const SEM_ALL_ERRORS: THREAD_ERROR_MODE = THREAD_ERROR_MODE(0u32);
pub const SEM_FAILCRITICALERRORS: THREAD_ERROR_MODE = THREAD_ERROR_MODE(1u32);
pub const SEM_NOGPFAULTERRORBOX: THREAD_ERROR_MODE = THREAD_ERROR_MODE(2u32);
pub const SEM_NOOPENFILEERRORBOX: THREAD_ERROR_MODE = THREAD_ERROR_MODE(32768u32);
pub const SEM_NOALIGNMENTFAULTEXCEPT: THREAD_ERROR_MODE = THREAD_ERROR_MODE(4u32);
pub const THREAD_OUT_OF_CONTEXT: u32 = 8u32;
pub const THREAD_STATE_RUNNING: u32 = 1u32;
pub const THREAD_STATE_SUSPENDED: u32 = 2u32;
#[repr(transparent)]
pub struct THREAD_WRITE_FLAGS(pub i32);
pub const ThreadWriteThread: THREAD_WRITE_FLAGS = THREAD_WRITE_FLAGS(1i32);
pub const ThreadWriteStack: THREAD_WRITE_FLAGS = THREAD_WRITE_FLAGS(2i32);
pub const ThreadWriteContext: THREAD_WRITE_FLAGS = THREAD_WRITE_FLAGS(4i32);
pub const ThreadWriteBackingStore: THREAD_WRITE_FLAGS = THREAD_WRITE_FLAGS(8i32);
pub const ThreadWriteInstructionWindow: THREAD_WRITE_FLAGS = THREAD_WRITE_FLAGS(16i32);
pub const ThreadWriteThreadData: THREAD_WRITE_FLAGS = THREAD_WRITE_FLAGS(32i32);
pub const ThreadWriteThreadInfo: THREAD_WRITE_FLAGS = THREAD_WRITE_FLAGS(64i32);
#[repr(C)]
pub struct TI_FINDCHILDREN_PARAMS(i32);
#[repr(C)]
pub struct TRANSLATE_VIRTUAL_TO_PHYSICAL(i32);
#[repr(transparent)]
pub struct TypeKind(pub i32);
pub const TypeUDT: TypeKind = TypeKind(0i32);
pub const TypePointer: TypeKind = TypeKind(1i32);
pub const TypeMemberPointer: TypeKind = TypeKind(2i32);
pub const TypeArray: TypeKind = TypeKind(3i32);
pub const TypeFunction: TypeKind = TypeKind(4i32);
pub const TypeTypedef: TypeKind = TypeKind(5i32);
pub const TypeEnum: TypeKind = TypeKind(6i32);
pub const TypeIntrinsic: TypeKind = TypeKind(7i32);
pub const TypeExtendedArray: TypeKind = TypeKind(8i32);
pub const UNAVAILABLE_ERROR: u32 = 12u32;
pub const UNDNAME_32_BIT_DECODE: u32 = 2048u32;
pub const UNDNAME_COMPLETE: u32 = 0u32;
pub const UNDNAME_NAME_ONLY: u32 = 4096u32;
pub const UNDNAME_NO_ACCESS_SPECIFIERS: u32 = 128u32;
pub const UNDNAME_NO_ALLOCATION_LANGUAGE: u32 = 16u32;
pub const UNDNAME_NO_ALLOCATION_MODEL: u32 = 8u32;
pub const UNDNAME_NO_ARGUMENTS: u32 = 8192u32;
pub const UNDNAME_NO_CV_THISTYPE: u32 = 64u32;
pub const UNDNAME_NO_FUNCTION_RETURNS: u32 = 4u32;
pub const UNDNAME_NO_LEADING_UNDERSCORES: u32 = 1u32;
pub const UNDNAME_NO_MEMBER_TYPE: u32 = 512u32;
pub const UNDNAME_NO_MS_KEYWORDS: u32 = 2u32;
pub const UNDNAME_NO_MS_THISTYPE: u32 = 32u32;
pub const UNDNAME_NO_RETURN_UDT_MODEL: u32 = 1024u32;
pub const UNDNAME_NO_SPECIAL_SYMS: u32 = 16384u32;
pub const UNDNAME_NO_THISTYPE: u32 = 96u32;
pub const UNDNAME_NO_THROW_SIGNATURES: u32 = 256u32;
#[repr(C)]
pub struct UNLOAD_DLL_DEBUG_INFO(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct UNWIND_HISTORY_TABLE(i32);
#[cfg(any(target_arch = "aarch64",))]
#[repr(C)]
pub struct UNWIND_HISTORY_TABLE_ENTRY(i32);
#[cfg(any(target_arch = "x86_64",))]
#[repr(C)]
pub struct UNWIND_HISTORY_TABLE_ENTRY(i32);
#[repr(transparent)]
pub struct VER_PLATFORM(pub u32);
pub const VER_PLATFORM_WIN32s: VER_PLATFORM = VER_PLATFORM(0u32);
pub const VER_PLATFORM_WIN32_WINDOWS: VER_PLATFORM = VER_PLATFORM(1u32);
pub const VER_PLATFORM_WIN32_NT: VER_PLATFORM = VER_PLATFORM(2u32);
#[repr(C)]
pub struct VIRTUAL_TO_PHYSICAL(i32);
#[repr(transparent)]
pub struct VarArgsKind(pub i32);
pub const VarArgsNone: VarArgsKind = VarArgsKind(0i32);
pub const VarArgsCStyle: VarArgsKind = VarArgsKind(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WAITCHAIN_NODE_INFO(i32);
#[repr(transparent)]
pub struct WAIT_CHAIN_THREAD_OPTIONS(pub u32);
pub const WCT_OUT_OF_PROC_COM_FLAG: WAIT_CHAIN_THREAD_OPTIONS = WAIT_CHAIN_THREAD_OPTIONS(2u32);
pub const WCT_OUT_OF_PROC_CS_FLAG: WAIT_CHAIN_THREAD_OPTIONS = WAIT_CHAIN_THREAD_OPTIONS(4u32);
pub const WCT_OUT_OF_PROC_FLAG: WAIT_CHAIN_THREAD_OPTIONS = WAIT_CHAIN_THREAD_OPTIONS(1u32);
pub const WCT_MAX_NODE_COUNT: u32 = 16u32;
pub const WCT_NETWORK_IO_FLAG: u32 = 8u32;
#[repr(transparent)]
pub struct WCT_OBJECT_STATUS(pub i32);
pub const WctStatusNoAccess: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(1i32);
pub const WctStatusRunning: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(2i32);
pub const WctStatusBlocked: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(3i32);
pub const WctStatusPidOnly: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(4i32);
pub const WctStatusPidOnlyRpcss: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(5i32);
pub const WctStatusOwned: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(6i32);
pub const WctStatusNotOwned: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(7i32);
pub const WctStatusAbandoned: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(8i32);
pub const WctStatusUnknown: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(9i32);
pub const WctStatusError: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(10i32);
pub const WctStatusMax: WCT_OBJECT_STATUS = WCT_OBJECT_STATUS(11i32);
#[repr(transparent)]
pub struct WCT_OBJECT_TYPE(pub i32);
pub const WctCriticalSectionType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(1i32);
pub const WctSendMessageType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(2i32);
pub const WctMutexType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(3i32);
pub const WctAlpcType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(4i32);
pub const WctComType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(5i32);
pub const WctThreadWaitType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(6i32);
pub const WctProcessWaitType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(7i32);
pub const WctThreadType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(8i32);
pub const WctComActivationType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(9i32);
pub const WctUnknownType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(10i32);
pub const WctSocketIoType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(11i32);
pub const WctSmbIoType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(12i32);
pub const WctMaxType: WCT_OBJECT_TYPE = WCT_OBJECT_TYPE(13i32);
pub const WCT_OBJNAME_LENGTH: u32 = 128u32;
pub const WDBGEXTS_ADDRESS_DEFAULT: u32 = 0u32;
pub const WDBGEXTS_ADDRESS_RESERVED0: u32 = 2147483648u32;
pub const WDBGEXTS_ADDRESS_SEG16: u32 = 1u32;
pub const WDBGEXTS_ADDRESS_SEG32: u32 = 2u32;
#[repr(C)]
pub struct WDBGEXTS_CLR_DATA_INTERFACE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WDBGEXTS_DISASSEMBLE_BUFFER(i32);
#[repr(C)]
pub struct WDBGEXTS_MODULE_IN_RANGE(i32);
#[repr(C)]
pub struct WDBGEXTS_QUERY_INTERFACE(i32);
#[repr(C)]
pub struct WDBGEXTS_THREAD_OS_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WHEA_AER_BRIDGE_DESCRIPTOR(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WHEA_AER_ENDPOINT_DESCRIPTOR(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WHEA_AER_ROOTPORT_DESCRIPTOR(i32);
pub const WHEA_BAD_PAGE_LIST_LOCATION: u32 = 15u32;
pub const WHEA_BAD_PAGE_LIST_MAX_SIZE: u32 = 14u32;
pub const WHEA_CMCI_THRESHOLD_COUNT: u32 = 10u32;
pub const WHEA_CMCI_THRESHOLD_POLL_COUNT: u32 = 12u32;
pub const WHEA_CMCI_THRESHOLD_TIME: u32 = 11u32;
pub const WHEA_DEVICE_DRIVER_BUFFER_SET_MAX: u32 = 1u32;
pub const WHEA_DEVICE_DRIVER_BUFFER_SET_MIN: u32 = 1u32;
pub const WHEA_DEVICE_DRIVER_BUFFER_SET_V1: u32 = 1u32;
pub const WHEA_DEVICE_DRIVER_CONFIG_MAX: u32 = 2u32;
pub const WHEA_DEVICE_DRIVER_CONFIG_MIN: u32 = 1u32;
pub const WHEA_DEVICE_DRIVER_CONFIG_V1: u32 = 1u32;
pub const WHEA_DEVICE_DRIVER_CONFIG_V2: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WHEA_DEVICE_DRIVER_DESCRIPTOR(i32);
pub const WHEA_DISABLE_DUMMY_WRITE: u32 = 6u32;
pub const WHEA_DISABLE_OFFLINE: u32 = 0u32;
#[repr(C)]
pub struct WHEA_DRIVER_BUFFER_SET(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WHEA_ERROR_SOURCE_CONFIGURATION_DD(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1(i32);
#[cfg(feature = "Win32_Foundation")]
pub type WHEA_ERROR_SOURCE_CORRECT_DEVICE_DRIVER = unsafe extern "system" fn(errorsourcedesc: *mut ::core::ffi::c_void, maximumsectionlength: *mut u32) -> super::super::super::Foundation::NTSTATUS;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WHEA_ERROR_SOURCE_DESCRIPTOR(i32);
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_AERBRIDGE: u32 = 8u32;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_AERENDPOINT: u32 = 7u32;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_AERROOTPORT: u32 = 6u32;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_GENERIC: u32 = 9u32;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_GENERIC_V2: u32 = 10u32;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_IPFCMC: u32 = 4u32;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_IPFCPE: u32 = 5u32;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_IPFMCA: u32 = 3u32;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_XPFCMC: u32 = 1u32;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_XPFMCE: u32 = 0u32;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_XPFNMI: u32 = 2u32;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_VERSION_10: u32 = 10u32;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_VERSION_11: u32 = 11u32;
pub const WHEA_ERROR_SOURCE_FLAG_DEFAULTSOURCE: u32 = 2147483648u32;
pub const WHEA_ERROR_SOURCE_FLAG_FIRMWAREFIRST: u32 = 1u32;
pub const WHEA_ERROR_SOURCE_FLAG_GHES_ASSIST: u32 = 4u32;
pub const WHEA_ERROR_SOURCE_FLAG_GLOBAL: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub type WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER = unsafe extern "system" fn(context: *mut ::core::ffi::c_void, errorsourceid: u32) -> super::super::super::Foundation::NTSTATUS;
pub const WHEA_ERROR_SOURCE_INVALID_RELATED_SOURCE: u32 = 65535u32;
#[repr(transparent)]
pub struct WHEA_ERROR_SOURCE_STATE(pub i32);
pub const WheaErrSrcStateStopped: WHEA_ERROR_SOURCE_STATE = WHEA_ERROR_SOURCE_STATE(1i32);
pub const WheaErrSrcStateStarted: WHEA_ERROR_SOURCE_STATE = WHEA_ERROR_SOURCE_STATE(2i32);
pub const WheaErrSrcStateRemoved: WHEA_ERROR_SOURCE_STATE = WHEA_ERROR_SOURCE_STATE(3i32);
pub const WheaErrSrcStateRemovePending: WHEA_ERROR_SOURCE_STATE = WHEA_ERROR_SOURCE_STATE(4i32);
#[repr(transparent)]
pub struct WHEA_ERROR_SOURCE_TYPE(pub i32);
pub const WheaErrSrcTypeMCE: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(0i32);
pub const WheaErrSrcTypeCMC: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(1i32);
pub const WheaErrSrcTypeCPE: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(2i32);
pub const WheaErrSrcTypeNMI: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(3i32);
pub const WheaErrSrcTypePCIe: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(4i32);
pub const WheaErrSrcTypeGeneric: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(5i32);
pub const WheaErrSrcTypeINIT: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(6i32);
pub const WheaErrSrcTypeBOOT: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(7i32);
pub const WheaErrSrcTypeSCIGeneric: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(8i32);
pub const WheaErrSrcTypeIPFMCA: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(9i32);
pub const WheaErrSrcTypeIPFCMC: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(10i32);
pub const WheaErrSrcTypeIPFCPE: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(11i32);
pub const WheaErrSrcTypeGenericV2: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(12i32);
pub const WheaErrSrcTypeSCIGenericV2: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(13i32);
pub const WheaErrSrcTypeBMC: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(14i32);
pub const WheaErrSrcTypePMEM: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(15i32);
pub const WheaErrSrcTypeDeviceDriver: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(16i32);
pub const WheaErrSrcTypeMax: WHEA_ERROR_SOURCE_TYPE = WHEA_ERROR_SOURCE_TYPE(17i32);
pub type WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER = unsafe extern "system" fn(context: *mut ::core::ffi::c_void);
#[repr(C)]
pub struct WHEA_GENERIC_ERROR_DESCRIPTOR(i32);
#[repr(C)]
pub struct WHEA_GENERIC_ERROR_DESCRIPTOR_V2(i32);
#[repr(C)]
pub struct WHEA_IPF_CMC_DESCRIPTOR(i32);
#[repr(C)]
pub struct WHEA_IPF_CPE_DESCRIPTOR(i32);
#[repr(C)]
pub struct WHEA_IPF_MCA_DESCRIPTOR(i32);
pub const WHEA_MAX_MC_BANKS: u32 = 32u32;
pub const WHEA_MEM_PERSISTOFFLINE: u32 = 1u32;
pub const WHEA_MEM_PFA_DISABLE: u32 = 2u32;
pub const WHEA_MEM_PFA_PAGECOUNT: u32 = 3u32;
pub const WHEA_MEM_PFA_THRESHOLD: u32 = 4u32;
pub const WHEA_MEM_PFA_TIMEOUT: u32 = 5u32;
#[repr(C)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR(i32);
#[repr(C)]
pub struct WHEA_NOTIFICATION_FLAGS(i32);
pub const WHEA_NOTIFICATION_TYPE_ARMV8_SEA: u32 = 8u32;
pub const WHEA_NOTIFICATION_TYPE_ARMV8_SEI: u32 = 9u32;
pub const WHEA_NOTIFICATION_TYPE_CMCI: u32 = 5u32;
pub const WHEA_NOTIFICATION_TYPE_EXTERNALINTERRUPT: u32 = 1u32;
pub const WHEA_NOTIFICATION_TYPE_EXTERNALINTERRUPT_GSIV: u32 = 10u32;
pub const WHEA_NOTIFICATION_TYPE_GPIO_SIGNAL: u32 = 7u32;
pub const WHEA_NOTIFICATION_TYPE_LOCALINTERRUPT: u32 = 2u32;
pub const WHEA_NOTIFICATION_TYPE_MCE: u32 = 6u32;
pub const WHEA_NOTIFICATION_TYPE_NMI: u32 = 4u32;
pub const WHEA_NOTIFICATION_TYPE_POLLED: u32 = 0u32;
pub const WHEA_NOTIFICATION_TYPE_SCI: u32 = 3u32;
pub const WHEA_NOTIFICATION_TYPE_SDEI: u32 = 11u32;
pub const WHEA_NOTIFY_ALL_OFFLINES: u32 = 16u32;
#[repr(C)]
pub struct WHEA_PCI_SLOT_NUMBER(i32);
pub const WHEA_PENDING_PAGE_LIST_SZ: u32 = 13u32;
pub const WHEA_RESTORE_CMCI_ATTEMPTS: u32 = 8u32;
pub const WHEA_RESTORE_CMCI_ENABLED: u32 = 7u32;
pub const WHEA_RESTORE_CMCI_ERR_LIMIT: u32 = 9u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WHEA_XPF_CMC_DESCRIPTOR(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WHEA_XPF_MCE_DESCRIPTOR(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WHEA_XPF_MC_BANK_DESCRIPTOR(i32);
pub const WHEA_XPF_MC_BANK_STATUSFORMAT_AMD64MCA: u32 = 2u32;
pub const WHEA_XPF_MC_BANK_STATUSFORMAT_IA32MCA: u32 = 0u32;
pub const WHEA_XPF_MC_BANK_STATUSFORMAT_Intel64MCA: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WHEA_XPF_NMI_DESCRIPTOR(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct WINDBG_EXTENSION_APIS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct WINDBG_EXTENSION_APIS32(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[repr(C)]
pub struct WINDBG_EXTENSION_APIS64(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINDBG_OLDKD_EXTENSION_APIS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WINDBG_OLD_EXTENSION_APIS(i32);
#[repr(C)]
pub struct WOW64_CONTEXT(i32);
pub const WOW64_CONTEXT_EXCEPTION_ACTIVE: u32 = 134217728u32;
pub const WOW64_CONTEXT_EXCEPTION_REPORTING: u32 = 2147483648u32;
pub const WOW64_CONTEXT_EXCEPTION_REQUEST: u32 = 1073741824u32;
pub const WOW64_CONTEXT_SERVICE_ACTIVE: u32 = 268435456u32;
pub const WOW64_CONTEXT_i386: u32 = 65536u32;
pub const WOW64_CONTEXT_i486: u32 = 65536u32;
#[repr(C)]
pub struct WOW64_DESCRIPTOR_TABLE_ENTRY(i32);
#[repr(C)]
pub struct WOW64_FLOATING_SAVE_AREA(i32);
#[repr(C)]
pub struct WOW64_LDT_ENTRY(i32);
pub const WOW64_MAXIMUM_SUPPORTED_EXTENSION: u32 = 512u32;
pub const WOW64_SIZE_OF_80387_REGISTERS: u32 = 80u32;
#[repr(C)]
pub struct XPF_MCE_FLAGS(i32);
#[repr(C)]
pub struct XPF_MC_BANK_FLAGS(i32);
#[repr(C)]
pub struct XSAVE_AREA(i32);
#[repr(C)]
pub struct XSAVE_AREA_HEADER(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct XSAVE_FORMAT(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct XSAVE_FORMAT(i32);
#[repr(C)]
pub struct XSTATE_CONFIGURATION(i32);
#[repr(C)]
pub struct XSTATE_CONFIG_FEATURE_MSC_INFO(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct XSTATE_CONTEXT(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct XSTATE_CONTEXT(i32);
#[repr(C)]
pub struct XSTATE_FEATURE(i32);
#[repr(transparent)]
pub struct _DUMP_TYPES(pub i32);
pub const DUMP_TYPE_INVALID: _DUMP_TYPES = _DUMP_TYPES(-1i32);
pub const DUMP_TYPE_UNKNOWN: _DUMP_TYPES = _DUMP_TYPES(0i32);
pub const DUMP_TYPE_FULL: _DUMP_TYPES = _DUMP_TYPES(1i32);
pub const DUMP_TYPE_SUMMARY: _DUMP_TYPES = _DUMP_TYPES(2i32);
pub const DUMP_TYPE_HEADER: _DUMP_TYPES = _DUMP_TYPES(3i32);
pub const DUMP_TYPE_TRIAGE: _DUMP_TYPES = _DUMP_TYPES(4i32);
pub const DUMP_TYPE_BITMAP_FULL: _DUMP_TYPES = _DUMP_TYPES(5i32);
pub const DUMP_TYPE_BITMAP_KERNEL: _DUMP_TYPES = _DUMP_TYPES(6i32);
pub const DUMP_TYPE_AUTOMATIC: _DUMP_TYPES = _DUMP_TYPES(7i32);
#[repr(C)]
pub struct _GETSETBUSDATA(i32);
#[repr(C)]
pub struct _IMAGEHLP_JIT_SYMBOL_MAP(i32);
#[repr(C)]
pub struct __MIDL___MIDL_itf_jscript9diag_0000_0007_0001(i32);
pub const fasaCaseSensitive: u32 = 4u32;
pub const fasaPreferInternalHandler: u32 = 1u32;
pub const fasaSupportInternalHandler: u32 = 2u32;
pub const sevMax: i32 = 4i32;
