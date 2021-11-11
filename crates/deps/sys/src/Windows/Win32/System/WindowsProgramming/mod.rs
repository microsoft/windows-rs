#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddDelBackupEntryA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddDelBackupEntryW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdvInstallFileA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdvInstallFileW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApphelpCheckShellObject();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelDeviceWakeupRequest();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelTimerQueueTimer();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn CloseINFEngine();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn ConvertAuxiliaryCounterToPerformanceCounter();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn ConvertPerformanceCounterToAuxiliaryCounter();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWaitableTimerA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWaitableTimerExA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn DCIBeginAccess();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DCICloseProvider();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DCICreateOffscreen();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DCICreateOverlay();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DCICreatePrimary();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn DCIDestroy();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn DCIDraw();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn DCIEndAccess();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DCIEnum();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn DCIOpenProvider();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DCISetClipList();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCISetDestination();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DCISetSrcDestClip();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DelNodeA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DelNodeRunDLL32W();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DelNodeW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsHostnameToComputerNameA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsHostnameToComputerNameW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DosDateTimeToFileTime();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableProcessOptionalXStateFeatures();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExecuteCabA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExecuteCabW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractFilesA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ExtractFilesW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileSaveMarkNotExistA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileSaveMarkNotExistW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileSaveRestoreOnINFA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileSaveRestoreOnINFW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileSaveRestoreW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FileTimeToDosDateTime();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn GdiEntry13();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerNameA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerNameW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentHwProfileA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentHwProfileW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetDCRegionData();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn GetFeatureEnabledState();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFeatureVariant();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFirmwareEnvironmentVariableA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFirmwareEnvironmentVariableExA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFirmwareEnvironmentVariableExW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFirmwareEnvironmentVariableW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileIntA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileIntW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileSectionA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileSectionNamesA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileSectionNamesW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileSectionW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileStringA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileStringW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileStructA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPrivateProfileStructW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileIntA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileIntW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileSectionA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileSectionW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileStringA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileStringW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemRegistryQuota();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64",))]
    pub fn GetThreadEnabledXStateFeatures();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserNameA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserNameW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionFromFileA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionFromFileExA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionFromFileExW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionFromFileW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetWindowRegionData();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn GlobalCompact();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn GlobalFix();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalUnWire();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn GlobalUnfix();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn GlobalWire();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPGetIMEA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPGetIMEW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPQueryIMEA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPQueryIMEW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPSetIMEA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IMPSetIMEW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsApiSetImplemented();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadHugeReadPtr();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadHugeWritePtr();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNTAdmin();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNativeVhdBoot();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsTokenUntrusted();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LaunchINFSectionExW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LaunchINFSectionW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn LocalCompact();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn LocalShrink();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn MulDiv();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NeedReboot();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn NeedRebootInit();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtClose();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtDeviceIoControlFile();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtNotifyChangeMultipleKeys();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtOpenFile();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQueryMultipleValueKey();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQueryObject();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQuerySystemInformation();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQuerySystemTime();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtQueryTimerResolution();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtRenameKey();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtSetInformationKey();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NtWaitForSingleObject();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenINFEngineA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenINFEngineW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenMutexA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenSemaphoreA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenWaitableTimerA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn QueryAuxiliaryCounterFrequency();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryIdleProcessorCycleTime();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryIdleProcessorCycleTimeEx();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn QueryInterruptTime();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn QueryInterruptTimePrecise();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryProcessCycleTime();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryThreadCycleTime();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryUnbiasedInterruptTime();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn QueryUnbiasedInterruptTimePrecise();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RaiseCustomSystemEventTrigger();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RebootCheckOnInstallA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RebootCheckOnInstallW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RecordFeatureError();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RecordFeatureUsage();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegInstallA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegInstallW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegRestoreAllA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegRestoreAllW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegSaveRestoreA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegSaveRestoreOnINFA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegSaveRestoreOnINFW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn RegSaveRestoreW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplacePartitionUnit();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RequestDeviceWakeup();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlAnsiStringToUnicodeString();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlCharToInteger();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlFreeAnsiString();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlFreeOemString();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlFreeUnicodeString();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn RtlGetReturnAddressHijackTarget();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlInitAnsiString();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlInitAnsiStringEx();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlInitString();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlInitStringEx();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlInitUnicodeString();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlIsNameLegalDOS8Dot3();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlLocalTimeToSystemTime();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlRaiseCustomSystemEventTrigger();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlTimeToSecondsSince1970();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlUnicodeStringToAnsiString();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_System_Kernel`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
    pub fn RtlUnicodeStringToOemString();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlUnicodeToMultiByteSize();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn RtlUniform();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RunSetupCommandA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RunSetupCommandW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendIMEMessageExA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendIMEMessageExW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetEnvironmentStringsA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFirmwareEnvironmentVariableA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFirmwareEnvironmentVariableExA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFirmwareEnvironmentVariableExW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFirmwareEnvironmentVariableW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn SetHandleCount();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMessageWaitingIndicator();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPerUserSecValuesA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPerUserSecValuesW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SignalObjectAndWait();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn SubscribeFeatureStateChangeNotification();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateInfStringA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateInfStringExA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateInfStringExW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateInfStringW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn UnsubscribeFeatureStateChangeNotification();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UserInstStubWrapperA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UserInstStubWrapperW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UserUnInstStubWrapperA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UserUnInstStubWrapperW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WINNLSEnableIME();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WINNLSGetEnableStatus();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WINNLSGetIMEHotkey();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn WinWatchClose();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinWatchDidStatusChange();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn WinWatchGetClipList();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinWatchNotify();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinWatchOpen();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpGetLockdownPolicy();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpIsClassInApprovedList();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpIsDynamicCodePolicyEnabled();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpQueryDeviceSecurityInformation();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpQueryDynamicCodeTrust();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WldpSetDynamicCodeTrust();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileSectionA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileSectionW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileStringA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileStringW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileStructA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePrivateProfileStructW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProfileSectionA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProfileSectionW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProfileStringA();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProfileStringW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn _hread();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn _hwrite();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn _lclose();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn _lcreat();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn _llseek();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn _lopen();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    pub fn _lread();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn _lwrite();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_lstrcmpW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_lstrcmpiW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_lstrlenW();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_wcschr();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_wcscpy();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_wcsicmp();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_wcslen();
    #[doc = "*Required features: `Win32_System_WindowsProgramming`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    pub fn uaw_wcsrchr();
}
