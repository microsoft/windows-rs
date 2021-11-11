#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_NetShell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MatchEnumTag(hmodule: super::super::Foundation::HANDLE, pwcarg: super::super::Foundation::PWSTR, dwnumarg: u32, penumtable: *const TOKEN_VALUE, pdwvalue: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetShell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MatchToken(pwszusertoken: super::super::Foundation::PWSTR, pwszcmdtoken: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_NetworkManagement_NetShell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PreprocessCommand(hmodule: super::super::Foundation::HANDLE, ppwcarguments: *mut super::super::Foundation::PWSTR, dwcurrentindex: u32, dwargcount: u32, ptttags: *mut TAG_TYPE, dwtagcount: u32, dwminargs: u32, dwmaxargs: u32, pdwtagtype: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetShell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrintError(hmodule: super::super::Foundation::HANDLE, dwerrid: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetShell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrintMessage(pwszformat: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetShell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrintMessageFromModule(hmodule: super::super::Foundation::HANDLE, dwmsgid: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetShell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterContext(pchildcontext: *const ::core::mem::ManuallyDrop<NS_CONTEXT_ATTRIBUTES>) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_NetShell`*"]
    pub fn RegisterHelper(pguidparentcontext: *const ::windows::runtime::GUID, pfnregistersubcontext: *const ::core::mem::ManuallyDrop<NS_HELPER_ATTRIBUTES>) -> u32;
}
