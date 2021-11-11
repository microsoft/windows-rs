#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_RestartManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmAddFilter();
    #[doc = "*Required features: `Win32_System_RestartManager`*"]
    pub fn RmCancelCurrentTask();
    #[doc = "*Required features: `Win32_System_RestartManager`*"]
    pub fn RmEndSession();
    #[doc = "*Required features: `Win32_System_RestartManager`*"]
    pub fn RmGetFilterList();
    #[doc = "*Required features: `Win32_System_RestartManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmGetList();
    #[doc = "*Required features: `Win32_System_RestartManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmJoinSession();
    #[doc = "*Required features: `Win32_System_RestartManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmRegisterResources();
    #[doc = "*Required features: `Win32_System_RestartManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmRemoveFilter();
    #[doc = "*Required features: `Win32_System_RestartManager`*"]
    pub fn RmRestart();
    #[doc = "*Required features: `Win32_System_RestartManager`*"]
    pub fn RmShutdown();
    #[doc = "*Required features: `Win32_System_RestartManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmStartSession();
}
