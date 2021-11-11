#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcClose();
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcDeleteSubscription();
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcEnumNextSubscription();
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcGetObjectArrayProperty();
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcGetObjectArraySize();
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcGetSubscriptionProperty();
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcGetSubscriptionRunTimeStatus();
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcInsertObjectArrayElement();
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcOpenSubscription();
    #[doc = "*Required features: `Win32_System_EventCollector`*"]
    pub fn EcOpenSubscriptionEnum();
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcRemoveObjectArrayElement();
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcRetrySubscription();
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcSaveSubscription();
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcSetObjectArrayProperty();
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcSetSubscriptionProperty();
}
