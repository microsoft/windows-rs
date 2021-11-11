#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssignProcessToJobObject();
    #[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateJobObjectA();
    #[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateJobObjectW();
    #[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateJobSet();
    #[doc = "*Required features: `Win32_System_JobObjects`*"]
    pub fn FreeMemoryJobObject();
    #[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessInJob();
    #[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenJobObjectA();
    #[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenJobObjectW();
    #[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryInformationJobObject();
    #[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryIoRateControlInformationJobObject();
    #[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetInformationJobObject();
    #[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIoRateControlInformationJobObject();
    #[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TerminateJobObject();
    #[doc = "*Required features: `Win32_System_JobObjects`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UserHandleGrantAccess();
}
