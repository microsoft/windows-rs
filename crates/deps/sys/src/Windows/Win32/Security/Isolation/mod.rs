#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateAppContainerProfile(pszappcontainername: super::super::Foundation::PWSTR, pszdisplayname: super::super::Foundation::PWSTR, pszdescription: super::super::Foundation::PWSTR, pcapabilities: *const super::SID_AND_ATTRIBUTES, dwcapabilitycount: u32, ppsidappcontainersid: *mut super::super::Foundation::PSID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteAppContainerProfile(pszappcontainername: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeriveAppContainerSidFromAppContainerName(pszappcontainername: super::super::Foundation::PWSTR, ppsidappcontainersid: *mut super::super::Foundation::PSID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid: super::super::Foundation::PSID, pszrestrictedappcontainername: super::super::Foundation::PWSTR, ppsidrestrictedappcontainersid: *mut super::super::Foundation::PSID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAppContainerFolderPath(pszappcontainersid: super::super::Foundation::PWSTR, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAppContainerNamedObjectPath(token: super::super::Foundation::HANDLE, appcontainersid: super::super::Foundation::PSID, objectpathlength: u32, objectpath: super::super::Foundation::PWSTR, returnlength: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetAppContainerRegistryLocation(desiredaccess: u32, phappcontainerkey: *mut super::super::System::Registry::HKEY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessInIsolatedContainer(isprocessinisolatedcontainer: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessInIsolatedWindowsEnvironment(isprocessinisolatedwindowsenvironment: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessInWDAGContainer(reserved: *const ::core::ffi::c_void, isprocessinwdagcontainer: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
}
#[repr(transparent)]
pub struct IIsolatedAppLauncher(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IsolatedAppLauncher(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IsolatedAppLauncherTelemetryParameters(i32);
