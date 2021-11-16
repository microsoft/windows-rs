#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplicationRecoveryFinished(bsuccess: super::super::Foundation::BOOL);
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplicationRecoveryInProgress(pbcancelled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn GetApplicationRecoveryCallback(hprocess: super::super::Foundation::HANDLE, precoverycallback: *mut super::WindowsProgramming::APPLICATION_RECOVERY_CALLBACK, ppvparameter: *mut *mut ::core::ffi::c_void, pdwpinginterval: *mut u32, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetApplicationRestartSettings(hprocess: super::super::Foundation::HANDLE, pwzcommandline: super::super::Foundation::PWSTR, pcchsize: *mut u32, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn RegisterApplicationRecoveryCallback(precoveycallback: super::WindowsProgramming::APPLICATION_RECOVERY_CALLBACK, pvparameter: *const ::core::ffi::c_void, dwpinginterval: u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterApplicationRestart(pwzcommandline: super::super::Foundation::PWSTR, dwflags: REGISTER_APPLICATION_RESTART_FLAGS) -> ::windows_sys::core::HRESULT;
    pub fn UnregisterApplicationRecoveryCallback() -> ::windows_sys::core::HRESULT;
    pub fn UnregisterApplicationRestart() -> ::windows_sys::core::HRESULT;
}
pub const RESTART_NO_CRASH: u32 = 1u32;
pub const RESTART_NO_HANG: u32 = 2u32;
pub const RESTART_NO_PATCH: u32 = 4u32;
pub const RESTART_NO_REBOOT: u32 = 8u32;
