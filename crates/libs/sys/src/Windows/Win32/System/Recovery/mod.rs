#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_Recovery\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplicationRecoveryFinished(bsuccess: super::super::Foundation::BOOL);
    #[doc = "*Required features: `\"Win32_System_Recovery\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplicationRecoveryInProgress(pbcancelled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Recovery\"`, `\"Win32_Foundation\"`, `\"Win32_System_WindowsProgramming\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_WindowsProgramming"))]
    pub fn GetApplicationRecoveryCallback(hprocess: super::super::Foundation::HANDLE, precoverycallback: *mut super::WindowsProgramming::APPLICATION_RECOVERY_CALLBACK, ppvparameter: *mut *mut ::core::ffi::c_void, pdwpinginterval: *mut u32, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Recovery\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetApplicationRestartSettings(hprocess: super::super::Foundation::HANDLE, pwzcommandline: ::windows_sys::core::PWSTR, pcchsize: *mut u32, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Recovery\"`, `\"Win32_System_WindowsProgramming\"`*"]
    #[cfg(feature = "Win32_System_WindowsProgramming")]
    pub fn RegisterApplicationRecoveryCallback(precoveycallback: super::WindowsProgramming::APPLICATION_RECOVERY_CALLBACK, pvparameter: *const ::core::ffi::c_void, dwpinginterval: u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Recovery\"`*"]
    pub fn RegisterApplicationRestart(pwzcommandline: ::windows_sys::core::PCWSTR, dwflags: REGISTER_APPLICATION_RESTART_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Recovery\"`*"]
    pub fn UnregisterApplicationRecoveryCallback() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Recovery\"`*"]
    pub fn UnregisterApplicationRestart() -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_System_Recovery\"`*"]
pub type REGISTER_APPLICATION_RESTART_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Recovery\"`*"]
pub const RESTART_NO_CRASH: REGISTER_APPLICATION_RESTART_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Recovery\"`*"]
pub const RESTART_NO_HANG: REGISTER_APPLICATION_RESTART_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Recovery\"`*"]
pub const RESTART_NO_PATCH: REGISTER_APPLICATION_RESTART_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_Recovery\"`*"]
pub const RESTART_NO_REBOOT: REGISTER_APPLICATION_RESTART_FLAGS = 8u32;
