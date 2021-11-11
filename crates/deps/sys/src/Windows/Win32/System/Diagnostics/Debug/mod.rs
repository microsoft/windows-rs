#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_System_Diagnostics_Debug_WebApp")]
pub mod WebApp;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn AddVectoredContinueHandler(first: u32, handler: ::windows::runtime::RawPtr) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn AddVectoredExceptionHandler(first: u32, handler: ::windows::runtime::RawPtr) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Beep(dwfreq: u32, dwduration: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BindImage(imagename: super::super::super::Foundation::PSTR, dllpath: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BindImageEx(flags: u32, imagename: super::super::super::Foundation::PSTR, dllpath: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, statusroutine: ::windows::runtime::RawPtr) -> super::super::super::Foundation::BOOL;
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
    pub fn CreateDataModelManager(debughost: ::windows::runtime::RawPtr, manager: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DbgHelpCreateUserDump(filename: super::super::super::Foundation::PSTR, callback: ::windows::runtime::RawPtr, userdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DbgHelpCreateUserDumpW(filename: super::super::super::Foundation::PWSTR, callback: ::windows::runtime::RawPtr, userdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
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
    pub fn DebugConnect(remoteoptions: super::super::super::Foundation::PSTR, interfaceid: *const ::windows::runtime::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugConnectWide(remoteoptions: super::super::super::Foundation::PWSTR, interfaceid: *const ::windows::runtime::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn DebugCreate(interfaceid: *const ::windows::runtime::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn DebugCreateEx(interfaceid: *const ::windows::runtime::GUID, dbgengoptions: u32, interface: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugSetProcessKillOnExit(killonexit: super::super::super::Foundation::BOOL) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn DecodePointer(ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DecodeRemotePointer(processhandle: super::super::super::Foundation::HANDLE, ptr: *const ::core::ffi::c_void, decodedptr: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn DecodeSystemPointer(ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn EncodePointer(ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EncodeRemotePointer(processhandle: super::super::super::Foundation::HANDLE, ptr: *const ::core::ffi::c_void, encodedptr: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn EncodeSystemPointer(ptr: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDirTree(hprocess: super::super::super::Foundation::HANDLE, rootpath: super::super::super::Foundation::PSTR, inputpathname: super::super::super::Foundation::PSTR, outputpathbuffer: super::super::super::Foundation::PSTR, cb: ::windows::runtime::RawPtr, data: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDirTreeW(hprocess: super::super::super::Foundation::HANDLE, rootpath: super::super::super::Foundation::PWSTR, inputpathname: super::super::super::Foundation::PWSTR, outputpathbuffer: super::super::super::Foundation::PWSTR, cb: ::windows::runtime::RawPtr, data: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModules(hprocess: super::super::super::Foundation::HANDLE, enumloadedmodulescallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModules64(hprocess: super::super::super::Foundation::HANDLE, enumloadedmodulescallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModulesEx(hprocess: super::super::super::Foundation::HANDLE, enumloadedmodulescallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModulesExW(hprocess: super::super::super::Foundation::HANDLE, enumloadedmodulescallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModulesW64(hprocess: super::super::super::Foundation::HANDLE, enumloadedmodulescallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
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
    pub fn FindDebugInfoFileEx(filename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, debugfilepath: super::super::super::Foundation::PSTR, callback: ::windows::runtime::RawPtr, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindDebugInfoFileExW(filename: super::super::super::Foundation::PWSTR, symbolpath: super::super::super::Foundation::PWSTR, debugfilepath: super::super::super::Foundation::PWSTR, callback: ::windows::runtime::RawPtr, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableImage(filename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, imagefilepath: super::super::super::Foundation::PSTR) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableImageEx(filename: super::super::super::Foundation::PSTR, symbolpath: super::super::super::Foundation::PSTR, imagefilepath: super::super::super::Foundation::PSTR, callback: ::windows::runtime::RawPtr, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableImageExW(filename: super::super::super::Foundation::PWSTR, symbolpath: super::super::super::Foundation::PWSTR, imagefilepath: super::super::super::Foundation::PWSTR, callback: ::windows::runtime::RawPtr, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
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
    pub fn ImageGetDigestStream(filehandle: super::super::super::Foundation::HANDLE, digestlevel: u32, digestfunction: ::windows::runtime::RawPtr, digesthandle: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
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
    pub fn MiniDumpWriteDump(hprocess: super::super::super::Foundation::HANDLE, processid: u32, hfile: super::super::super::Foundation::HANDLE, dumptype: MINIDUMP_TYPE, exceptionparam: *const MINIDUMP_EXCEPTION_INFORMATION, userstreamparam: *const MINIDUMP_USER_STREAM_INFORMATION, callbackparam: *const ::core::mem::ManuallyDrop<MINIDUMP_CALLBACK_INFORMATION>) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenThreadWaitChainSession(flags: OPEN_THREAD_WAIT_CHAIN_SESSION_FLAGS, callback: ::windows::runtime::RawPtr) -> *mut ::core::ffi::c_void;
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
    pub fn RegisterWaitChainCOMCallback(callstatecallback: ::windows::runtime::RawPtr, activationstatecallback: ::windows::runtime::RawPtr);
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
    pub fn RtlInstallFunctionTableCallback(tableidentifier: u64, baseaddress: u64, length: u32, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, outofprocesscallbackdll: super::super::super::Foundation::PWSTR) -> super::super::super::Foundation::BOOLEAN;
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
    pub fn SetCheckUserInterruptShared(lpstartaddress: ::windows::runtime::RawPtr);
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
    pub fn SetUnhandledExceptionFilter(lptoplevelexceptionfilter: ::windows::runtime::RawPtr) -> ::core::option::Option<LPTOP_LEVEL_EXCEPTION_FILTER>;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetXStateFeaturesMask(context: *mut CONTEXT, featuremask: u64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StackWalk(machinetype: u32, hprocess: super::super::super::Foundation::HANDLE, hthread: super::super::super::Foundation::HANDLE, stackframe: *mut STACKFRAME, contextrecord: *mut ::core::ffi::c_void, readmemoryroutine: ::windows::runtime::RawPtr, functiontableaccessroutine: ::windows::runtime::RawPtr, getmodulebaseroutine: ::windows::runtime::RawPtr, translateaddress: ::windows::runtime::RawPtr) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StackWalk64(machinetype: u32, hprocess: super::super::super::Foundation::HANDLE, hthread: super::super::super::Foundation::HANDLE, stackframe: *mut STACKFRAME64, contextrecord: *mut ::core::ffi::c_void, readmemoryroutine: ::windows::runtime::RawPtr, functiontableaccessroutine: ::windows::runtime::RawPtr, getmodulebaseroutine: ::windows::runtime::RawPtr, translateaddress: ::windows::runtime::RawPtr) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StackWalkEx(machinetype: u32, hprocess: super::super::super::Foundation::HANDLE, hthread: super::super::super::Foundation::HANDLE, stackframe: *mut STACKFRAME_EX, contextrecord: *mut ::core::ffi::c_void, readmemoryroutine: ::windows::runtime::RawPtr, functiontableaccessroutine: ::windows::runtime::RawPtr, getmodulebaseroutine: ::windows::runtime::RawPtr, translateaddress: ::windows::runtime::RawPtr, flags: u32) -> super::super::super::Foundation::BOOL;
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
    pub fn SymEnumLines(hprocess: super::super::super::Foundation::HANDLE, base: u64, obj: super::super::super::Foundation::PSTR, file: super::super::super::Foundation::PSTR, enumlinescallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumLinesW(hprocess: super::super::super::Foundation::HANDLE, base: u64, obj: super::super::super::Foundation::PWSTR, file: super::super::super::Foundation::PWSTR, enumlinescallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumProcesses(enumprocessescallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceFileTokens(hprocess: super::super::super::Foundation::HANDLE, base: u64, callback: ::windows::runtime::RawPtr) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceFiles(hprocess: super::super::super::Foundation::HANDLE, modbase: u64, mask: super::super::super::Foundation::PSTR, cbsrcfiles: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceFilesW(hprocess: super::super::super::Foundation::HANDLE, modbase: u64, mask: super::super::super::Foundation::PWSTR, cbsrcfiles: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceLines(hprocess: super::super::super::Foundation::HANDLE, base: u64, obj: super::super::super::Foundation::PSTR, file: super::super::super::Foundation::PSTR, line: u32, flags: u32, enumlinescallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceLinesW(hprocess: super::super::super::Foundation::HANDLE, base: u64, obj: super::super::super::Foundation::PWSTR, file: super::super::super::Foundation::PWSTR, line: u32, flags: u32, enumlinescallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSym(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbols(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PSTR, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsEx(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PSTR, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void, options: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsExW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PWSTR, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void, options: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsForAddr(hprocess: super::super::super::Foundation::HANDLE, address: u64, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsForAddrW(hprocess: super::super::super::Foundation::HANDLE, address: u64, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PWSTR, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumTypes(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumTypesByName(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PSTR, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumTypesByNameW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, mask: super::super::super::Foundation::PWSTR, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumTypesW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateModules(hprocess: super::super::super::Foundation::HANDLE, enummodulescallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateModules64(hprocess: super::super::super::Foundation::HANDLE, enummodulescallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateModulesW64(hprocess: super::super::super::Foundation::HANDLE, enummodulescallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateSymbols(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u32, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateSymbols64(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateSymbolsW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u32, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateSymbolsW64(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindDebugInfoFile(hprocess: super::super::super::Foundation::HANDLE, filename: super::super::super::Foundation::PSTR, debugfilepath: super::super::super::Foundation::PSTR, callback: ::windows::runtime::RawPtr, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindDebugInfoFileW(hprocess: super::super::super::Foundation::HANDLE, filename: super::super::super::Foundation::PWSTR, debugfilepath: super::super::super::Foundation::PWSTR, callback: ::windows::runtime::RawPtr, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindExecutableImage(hprocess: super::super::super::Foundation::HANDLE, filename: super::super::super::Foundation::PSTR, imagefilepath: super::super::super::Foundation::PSTR, callback: ::windows::runtime::RawPtr, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindExecutableImageW(hprocess: super::super::super::Foundation::HANDLE, filename: super::super::super::Foundation::PWSTR, imagefilepath: super::super::super::Foundation::PWSTR, callback: ::windows::runtime::RawPtr, callerdata: *const ::core::ffi::c_void) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindFileInPath(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PSTR, filename: super::super::super::Foundation::PSTR, id: *const ::core::ffi::c_void, two: u32, three: u32, flags: SYM_FIND_ID_OPTION, foundfile: super::super::super::Foundation::PSTR, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindFileInPathW(hprocess: super::super::super::Foundation::HANDLE, searchpatha: super::super::super::Foundation::PWSTR, filename: super::super::super::Foundation::PWSTR, id: *const ::core::ffi::c_void, two: u32, three: u32, flags: SYM_FIND_ID_OPTION, foundfile: super::super::super::Foundation::PWSTR, callback: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
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
    pub fn SymFunctionTableAccess64AccessRoutines(hprocess: super::super::super::Foundation::HANDLE, addrbase: u64, readmemoryroutine: ::windows::runtime::RawPtr, getmodulebaseroutine: ::windows::runtime::RawPtr) -> *mut ::core::ffi::c_void;
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
    pub fn SymRegisterCallback(hprocess: super::super::super::Foundation::HANDLE, callbackfunction: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterCallback64(hprocess: super::super::super::Foundation::HANDLE, callbackfunction: ::windows::runtime::RawPtr, usercontext: u64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterCallbackW64(hprocess: super::super::super::Foundation::HANDLE, callbackfunction: ::windows::runtime::RawPtr, usercontext: u64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterFunctionEntryCallback(hprocess: super::super::super::Foundation::HANDLE, callbackfunction: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterFunctionEntryCallback64(hprocess: super::super::super::Foundation::HANDLE, callbackfunction: ::windows::runtime::RawPtr, usercontext: u64) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSearch(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, index: u32, symtag: u32, mask: super::super::super::Foundation::PSTR, address: u64, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void, options: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSearchW(hprocess: super::super::super::Foundation::HANDLE, baseofdll: u64, index: u32, symtag: u32, mask: super::super::super::Foundation::PWSTR, address: u64, enumsymbolscallback: ::windows::runtime::RawPtr, usercontext: *const ::core::ffi::c_void, options: u32) -> super::super::super::Foundation::BOOL;
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
    pub fn SymSrvGetFileIndexes(file: super::super::super::Foundation::PSTR, id: *mut ::windows::runtime::GUID, val1: *mut u32, val2: *mut u32, flags: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexesW(file: super::super::super::Foundation::PWSTR, id: *mut ::windows::runtime::GUID, val1: *mut u32, val2: *mut u32, flags: u32) -> super::super::super::Foundation::BOOL;
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
    pub fn WaitForDebugEvent(lpdebugevent: *mut ::core::mem::ManuallyDrop<DEBUG_EVENT>, dwmilliseconds: u32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn WaitForDebugEventEx(lpdebugevent: *mut ::core::mem::ManuallyDrop<DEBUG_EVENT>, dwmilliseconds: u32) -> super::super::super::Foundation::BOOL;
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
