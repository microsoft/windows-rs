#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DtcGetTransactionManager();
    #[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DtcGetTransactionManagerC();
    #[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DtcGetTransactionManagerExA();
    #[doc = "*Required features: `Win32_System_DistributedTransactionCoordinator`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DtcGetTransactionManagerExW();
}
