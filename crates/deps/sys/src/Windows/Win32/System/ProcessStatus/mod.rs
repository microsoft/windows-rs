#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EmptyWorkingSet();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumDeviceDrivers();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumPageFilesA();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumPageFilesW();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumProcessModules();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumProcessModulesEx();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumProcesses();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetDeviceDriverBaseNameA();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetDeviceDriverBaseNameW();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetDeviceDriverFileNameA();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetDeviceDriverFileNameW();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetMappedFileNameA();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetMappedFileNameW();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetModuleBaseNameA();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetModuleBaseNameW();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetModuleFileNameExA();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetModuleFileNameExW();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetModuleInformation();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetPerformanceInfo();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetProcessImageFileNameA();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetProcessImageFileNameW();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetProcessMemoryInfo();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetWsChanges();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetWsChangesEx();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32InitializeProcessForWsWatch();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32QueryWorkingSet();
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32QueryWorkingSetEx();
}
