#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_SetupAndMigration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OOBEComplete();
    #[doc = "*Required features: `Win32_System_SetupAndMigration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterWaitUntilOOBECompleted();
    #[doc = "*Required features: `Win32_System_SetupAndMigration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterWaitUntilOOBECompleted();
}
