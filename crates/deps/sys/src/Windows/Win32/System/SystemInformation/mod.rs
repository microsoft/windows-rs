#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DnsHostnameToComputerNameExW();
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn EnumSystemFirmwareTables();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerNameExA();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComputerNameExW();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFirmwareType();
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn GetIntegratedDisplaySize();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLocalTime();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogicalProcessorInformation();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLogicalProcessorInformationEx();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn GetNativeSystemInfo();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOsManufacturingMode();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOsSafeBootMode();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPhysicallyInstalledSystemMemory();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessorSystemCycleTime();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProductInfo();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemCpuSetInformation();
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn GetSystemDEPPolicy();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemDirectoryA();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemDirectoryW();
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn GetSystemFirmwareTable();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_System_Diagnostics_Debug`*"]
    #[cfg(feature = "Win32_System_Diagnostics_Debug")]
    pub fn GetSystemInfo();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemLeapSecondInformation();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTime();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimeAdjustment();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimeAdjustmentPrecise();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimeAsFileTime();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemTimePreciseAsFileTime();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWindowsDirectoryA();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWindowsDirectoryW();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWow64Directory2A();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWow64Directory2W();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWow64DirectoryA();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemWow64DirectoryW();
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn GetTickCount();
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn GetTickCount64();
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn GetVersion();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionExA();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetVersionExW();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowsDirectoryA();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowsDirectoryW();
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn GlobalMemoryStatus();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalMemoryStatusEx();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsUserCetAvailableInEnvironment();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWow64GuestMachineSupported();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlConvertDeviceFamilyInfoToString();
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn RtlGetDeviceFamilyInfoEnum();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlGetProductInfo();
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn RtlGetSystemGlobalData();
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn RtlOsDeploymentState();
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn RtlSwitchedVVI();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameA();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameEx2W();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameExA();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameExW();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetComputerNameW();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLocalTime();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemTime();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemTimeAdjustment();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemTimeAdjustmentPrecise();
    #[doc = "*Required features: `Win32_System_SystemInformation`*"]
    pub fn VerSetConditionMask();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyVersionInfoA();
    #[doc = "*Required features: `Win32_System_SystemInformation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyVersionInfoW();
}
