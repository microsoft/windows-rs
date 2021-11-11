#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_RestartManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmAddFilter(dwsessionhandle: u32, strmodulename: super::super::Foundation::PWSTR, pprocess: *const RM_UNIQUE_PROCESS, strserviceshortname: super::super::Foundation::PWSTR, filteraction: RM_FILTER_ACTION) -> u32;
    #[doc = "*Required features: `Win32_System_RestartManager`*"]
    pub fn RmCancelCurrentTask(dwsessionhandle: u32) -> u32;
    #[doc = "*Required features: `Win32_System_RestartManager`*"]
    pub fn RmEndSession(dwsessionhandle: u32) -> u32;
    #[doc = "*Required features: `Win32_System_RestartManager`*"]
    pub fn RmGetFilterList(dwsessionhandle: u32, pbfilterbuf: *mut u8, cbfilterbuf: u32, cbfilterbufneeded: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_RestartManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmGetList(dwsessionhandle: u32, pnprocinfoneeded: *mut u32, pnprocinfo: *mut u32, rgaffectedapps: *mut RM_PROCESS_INFO, lpdwrebootreasons: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_RestartManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmJoinSession(psessionhandle: *mut u32, strsessionkey: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_RestartManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmRegisterResources(dwsessionhandle: u32, nfiles: u32, rgsfilenames: *const super::super::Foundation::PWSTR, napplications: u32, rgapplications: *const RM_UNIQUE_PROCESS, nservices: u32, rgsservicenames: *const super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_RestartManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmRemoveFilter(dwsessionhandle: u32, strmodulename: super::super::Foundation::PWSTR, pprocess: *const RM_UNIQUE_PROCESS, strserviceshortname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_RestartManager`*"]
    pub fn RmRestart(dwsessionhandle: u32, dwrestartflags: u32, fnstatus: ::windows::runtime::RawPtr) -> u32;
    #[doc = "*Required features: `Win32_System_RestartManager`*"]
    pub fn RmShutdown(dwsessionhandle: u32, lactionflags: u32, fnstatus: ::windows::runtime::RawPtr) -> u32;
    #[doc = "*Required features: `Win32_System_RestartManager`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmStartSession(psessionhandle: *mut u32, dwsessionflags: u32, strsessionkey: super::super::Foundation::PWSTR) -> u32;
}
