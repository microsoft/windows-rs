#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateToolhelp32Snapshot();
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Heap32First();
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Heap32ListFirst();
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Heap32ListNext();
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Heap32Next();
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Module32First();
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Module32FirstW();
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Module32Next();
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Module32NextW();
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process32First();
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process32FirstW();
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process32Next();
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process32NextW();
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Thread32First();
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Thread32Next();
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Toolhelp32ReadProcessMemory();
}
