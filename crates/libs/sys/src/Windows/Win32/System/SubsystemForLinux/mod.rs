#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: 'Win32_System_SubsystemForLinux', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslConfigureDistribution(distributionname: super::super::Foundation::PWSTR, defaultuid: u32, wsldistributionflags: WSL_DISTRIBUTION_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_System_SubsystemForLinux', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslGetDistributionConfiguration(distributionname: super::super::Foundation::PWSTR, distributionversion: *mut u32, defaultuid: *mut u32, wsldistributionflags: *mut WSL_DISTRIBUTION_FLAGS, defaultenvironmentvariables: *mut *mut super::super::Foundation::PSTR, defaultenvironmentvariablecount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_System_SubsystemForLinux', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslIsDistributionRegistered(distributionname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: 'Win32_System_SubsystemForLinux', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslLaunch(distributionname: super::super::Foundation::PWSTR, command: super::super::Foundation::PWSTR, usecurrentworkingdirectory: super::super::Foundation::BOOL, stdin: super::super::Foundation::HANDLE, stdout: super::super::Foundation::HANDLE, stderr: super::super::Foundation::HANDLE, process: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_System_SubsystemForLinux', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslLaunchInteractive(distributionname: super::super::Foundation::PWSTR, command: super::super::Foundation::PWSTR, usecurrentworkingdirectory: super::super::Foundation::BOOL, exitcode: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_System_SubsystemForLinux', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslRegisterDistribution(distributionname: super::super::Foundation::PWSTR, targzfilename: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_System_SubsystemForLinux', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WslUnregisterDistribution(distributionname: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: 'Win32_System_SubsystemForLinux'*"]
pub type WSL_DISTRIBUTION_FLAGS = u32;
#[doc = "*Required features: 'Win32_System_SubsystemForLinux'*"]
pub const WSL_DISTRIBUTION_FLAGS_NONE: WSL_DISTRIBUTION_FLAGS = 0u32;
#[doc = "*Required features: 'Win32_System_SubsystemForLinux'*"]
pub const WSL_DISTRIBUTION_FLAGS_ENABLE_INTEROP: WSL_DISTRIBUTION_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_System_SubsystemForLinux'*"]
pub const WSL_DISTRIBUTION_FLAGS_APPEND_NT_PATH: WSL_DISTRIBUTION_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_System_SubsystemForLinux'*"]
pub const WSL_DISTRIBUTION_FLAGS_ENABLE_DRIVE_MOUNTING: WSL_DISTRIBUTION_FLAGS = 4u32;
