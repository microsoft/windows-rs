#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcClose(object: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcDeleteSubscription(subscriptionname: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcEnumNextSubscription(subscriptionenum: isize, subscriptionnamebuffersize: u32, subscriptionnamebuffer: super::super::Foundation::PWSTR, subscriptionnamebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcGetObjectArrayProperty(objectarray: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, arrayindex: u32, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EC_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcGetObjectArraySize(objectarray: isize, objectarraysize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcGetSubscriptionProperty(subscription: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EC_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcGetSubscriptionRunTimeStatus(subscriptionname: super::super::Foundation::PWSTR, statusinfoid: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID, eventsourcename: super::super::Foundation::PWSTR, flags: u32, statusvaluebuffersize: u32, statusvaluebuffer: *mut EC_VARIANT, statusvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcInsertObjectArrayElement(objectarray: isize, arrayindex: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcOpenSubscription(subscriptionname: super::super::Foundation::PWSTR, accessmask: u32, flags: u32) -> isize;
    #[doc = "*Required features: `Win32_System_EventCollector`*"]
    pub fn EcOpenSubscriptionEnum(flags: u32) -> isize;
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcRemoveObjectArrayElement(objectarray: isize, arrayindex: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcRetrySubscription(subscriptionname: super::super::Foundation::PWSTR, eventsourcename: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcSaveSubscription(subscription: isize, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcSetObjectArrayProperty(objectarray: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, arrayindex: u32, flags: u32, propertyvalue: *mut EC_VARIANT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventCollector`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcSetSubscriptionProperty(subscription: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, flags: u32, propertyvalue: *mut EC_VARIANT) -> super::super::Foundation::BOOL;
}
