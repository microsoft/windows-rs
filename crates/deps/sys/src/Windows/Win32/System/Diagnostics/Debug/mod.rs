#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_System_Diagnostics_Debug_WebApp")]
pub mod WebApp;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn AddVectoredContinueHandler();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn AddVectoredExceptionHandler();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Beep();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BindImage();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BindImageEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckRemoteDebuggerPresent();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn CheckSumMappedFile();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn CheckSumMappedFile();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn CloseThreadWaitChainSession();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ContinueDebugEvent();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn CopyContext();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn CreateDataModelManager();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DbgHelpCreateUserDump();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DbgHelpCreateUserDumpW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugActiveProcess();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugActiveProcessStop();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn DebugBreak();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugBreakProcess();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugConnect();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugConnectWide();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn DebugCreate();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn DebugCreateEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DebugSetProcessKillOnExit();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn DecodePointer();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DecodeRemotePointer();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn DecodeSystemPointer();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn EncodePointer();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EncodeRemotePointer();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn EncodeSystemPointer();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDirTree();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDirTreeW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModules();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModules64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModulesEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModulesExW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateLoadedModulesW64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FatalAppExitA();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FatalAppExitW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn FatalExit();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindDebugInfoFile();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindDebugInfoFileEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindDebugInfoFileExW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableImage();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableImageEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableImageExW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFileInPath();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindFileInSearchPath();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushInstructionCache();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FormatMessageA();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FormatMessageW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    pub fn GetEnabledXStateFeatures();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn GetErrorMode();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetImageConfigInformation();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetImageConfigInformation();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetImageUnusedHeaderBytes();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn GetSymLoadError();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetThreadContext();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn GetThreadErrorMode();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadSelectorEntry();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThreadWaitChain();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTimestampForLoadedLibrary();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn GetXStateFeaturesMask();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_Security_WinTrust`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_WinTrust"))]
    pub fn ImageAddCertificate();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageDirectoryEntryToData();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageDirectoryEntryToDataEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageEnumerateCertificates();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_Security_WinTrust`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_WinTrust"))]
    pub fn ImageGetCertificateData();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_Security_WinTrust`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_WinTrust"))]
    pub fn ImageGetCertificateHeader();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageGetDigestStream();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn ImageLoad();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn ImageNtHeader();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn ImageNtHeader();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageRemoveCertificate();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn ImageRvaToSection();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn ImageRvaToSection();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn ImageRvaToVa();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86",))]
    pub fn ImageRvaToVa();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn ImageUnload();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn ImagehlpApiVersion();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn ImagehlpApiVersionEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn InitializeContext();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn InitializeContext2();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDebuggerPresent();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn LocateXStateFeature();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MakeSureDirectoryPathExists();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn MapAndLoad();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapFileAndCheckSumA();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapFileAndCheckSumW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MessageBeep();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MiniDumpReadDumpStream();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_Storage_FileSystem`, `Win32_System_Kernel`, `Win32_System_Memory`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_System_Kernel", feature = "Win32_System_Memory"))]
    pub fn MiniDumpWriteDump();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenThreadWaitChainSession();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OutputDebugStringA();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OutputDebugStringW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn RaiseException();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RaiseFailFastException();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RangeMapAddPeImageSections();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn RangeMapCreate();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn RangeMapFree();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RangeMapRead();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RangeMapRemove();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RangeMapWrite();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReBaseImage();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReBaseImage64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadProcessMemory();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn RegisterWaitChainCOMCallback();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveInvalidModuleList();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn RemoveVectoredContinueHandler();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn RemoveVectoredExceptionHandler();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReportSymbolLoadSummary();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlAddFunctionTable();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlAddFunctionTable();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "aarch64",))]
    pub fn RtlAddGrowableFunctionTable();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64",))]
    pub fn RtlAddGrowableFunctionTable();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`*"]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn RtlCaptureContext();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86_64",))]
    #[cfg(feature = "Win32_System_Kernel")]
    pub fn RtlCaptureContext2();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn RtlCaptureStackBackTrace();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlDeleteFunctionTable();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlDeleteFunctionTable();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn RtlDeleteGrowableFunctionTable();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn RtlGrowFunctionTable();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlInstallFunctionTableCallback();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "aarch64",))]
    pub fn RtlLookupFunctionEntry();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    #[cfg(any(target_arch = "x86_64",))]
    pub fn RtlLookupFunctionEntry();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn RtlPcToFileHeader();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlRaiseException();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlRestoreContext();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlUnwind();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlUnwindEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlVirtualUnwind();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86_64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlVirtualUnwind();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SearchTreeForFile();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SearchTreeForFileW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn SetCheckUserInterruptShared();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn SetErrorMode();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetImageConfigInformation();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetImageConfigInformation();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn SetSymLoadError();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetThreadContext();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadErrorMode();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetUnhandledExceptionFilter();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn SetXStateFeaturesMask();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StackWalk();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StackWalk64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StackWalkEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddSourceStream();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddSourceStreamA();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddSourceStreamW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddSymbol();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddSymbolW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymAddrIncludeInlineTrace();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymCleanup();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymCompareInlineTrace();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymDeleteSymbol();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymDeleteSymbolW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumLines();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumLinesW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumProcesses();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceFileTokens();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceFiles();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceFilesW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceLines();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSourceLinesW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSym();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbols();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsExW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsForAddr();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsForAddrW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumSymbolsW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumTypes();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumTypesByName();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumTypesByNameW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumTypesW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateModules();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateModules64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateModulesW64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateSymbols();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateSymbols64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateSymbolsW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymEnumerateSymbolsW64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindDebugInfoFile();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindDebugInfoFileW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindExecutableImage();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindExecutableImageW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindFileInPath();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFindFileInPathW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromAddr();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromAddrW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromIndex();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromIndexW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromInlineContext();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromInlineContextW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromName();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromNameW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromToken();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFromTokenW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFunctionTableAccess();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFunctionTableAccess64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymFunctionTableAccess64AccessRoutines();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetExtendedOption();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetFileLineOffsets64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetHomeDirectory();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetHomeDirectoryW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromAddr();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromAddr64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromAddrW64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromInlineContext();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromInlineContextW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromName();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromName64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineFromNameW64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineNext();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineNext64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLineNextW64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLinePrev();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLinePrev64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetLinePrevW64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleBase();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleBase64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleInfo();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleInfo64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleInfoW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetModuleInfoW64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetOmaps();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn SymGetOptions();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetScope();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetScopeW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSearchPath();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSearchPathW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFile();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileChecksum();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileChecksumW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileFromToken();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileFromTokenByTokenName();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileFromTokenByTokenNameW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileFromTokenW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileToken();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileTokenByTokenName();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileTokenByTokenNameW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileTokenW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceFileW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceVarFromToken();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSourceVarFromTokenW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymFromAddr();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymFromAddr64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymFromName();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymFromName64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymNext();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymNext64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymPrev();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymPrev64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymbolFile();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetSymbolFileW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetTypeFromName();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetTypeFromNameW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetTypeInfo();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetTypeInfoEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymGetUnwindInfo();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymInitialize();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymInitializeW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymLoadModule();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymLoadModule64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymLoadModuleEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymLoadModuleExW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymMatchFileName();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymMatchFileNameW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymMatchString();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymMatchStringA();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymMatchStringW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymNext();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymNextW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymPrev();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymPrevW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymQueryInlineTrace();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRefreshModuleList();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterCallback();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterCallback64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterCallbackW64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterFunctionEntryCallback();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymRegisterFunctionEntryCallback64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSearch();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSearchW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetContext();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetExtendedOption();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetHomeDirectory();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetHomeDirectoryW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn SymSetOptions();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetParentWindow();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetScopeFromAddr();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetScopeFromIndex();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetScopeFromInlineContext();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetSearchPath();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSetSearchPathW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvDeltaName();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvDeltaNameW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexInfo();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexInfoW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexString();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexStringW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexes();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetFileIndexesW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetSupplement();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvGetSupplementW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvIsStore();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvIsStoreW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvStoreFile();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvStoreFileW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvStoreSupplement();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymSrvStoreSupplementW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymUnDName();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymUnDName64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymUnloadModule();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SymUnloadModule64();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`*"]
    pub fn TerminateProcessOnMemoryExhaustion();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TouchFileTimes();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnDecorateSymbolName();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnDecorateSymbolNameW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn UnMapAndLoad();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn UnhandledExceptionFilter();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateDebugInfoFile();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateDebugInfoFileEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn WaitForDebugEvent();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn WaitForDebugEventEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64GetThreadContext();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64GetThreadSelectorEntry();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Wow64SetThreadContext();
    #[doc = "*Required features: `Win32_System_Diagnostics_Debug`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProcessMemory();
}
