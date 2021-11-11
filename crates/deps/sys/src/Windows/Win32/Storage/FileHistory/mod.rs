#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_System_WindowsProgramming`*"]
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn FhServiceBlockBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_System_WindowsProgramming`*"]
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn FhServiceClosePipe(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FhServiceOpenPipe(startserviceifstopped: super::super::Foundation::BOOL, pipe: *mut super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_System_WindowsProgramming`*"]
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn FhServiceReloadConfiguration(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FhServiceStartBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE, lowpriorityio: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_Foundation`, `Win32_System_WindowsProgramming`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn FhServiceStopBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE, stoptracking: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Storage_FileHistory`, `Win32_System_WindowsProgramming`*"]
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn FhServiceUnblockBackup(pipe: super::super::System::WindowsProgramming::FH_SERVICE_PIPE_HANDLE) -> ::windows::runtime::HRESULT;
}
