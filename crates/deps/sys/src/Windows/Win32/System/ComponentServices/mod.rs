#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn CoCreateActivity();
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn CoEnterServiceDomain();
    #[doc = "*Required features: `Win32_System_ComponentServices`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CoGetDefaultContext();
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn CoLeaveServiceDomain();
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn GetDispenserManager();
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn GetManagedExtensions();
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn MTSCreateActivity();
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn RecycleSurrogate();
    #[doc = "*Required features: `Win32_System_ComponentServices`*"]
    pub fn SafeRef();
}
