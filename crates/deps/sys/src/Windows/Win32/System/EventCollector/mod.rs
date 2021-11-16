#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcClose(object: isize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcDeleteSubscription(subscriptionname: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcEnumNextSubscription(subscriptionenum: isize, subscriptionnamebuffersize: u32, subscriptionnamebuffer: super::super::Foundation::PWSTR, subscriptionnamebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcGetObjectArrayProperty(objectarray: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, arrayindex: u32, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EC_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcGetObjectArraySize(objectarray: isize, objectarraysize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcGetSubscriptionProperty(subscription: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EC_VARIANT, propertyvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcGetSubscriptionRunTimeStatus(subscriptionname: super::super::Foundation::PWSTR, statusinfoid: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID, eventsourcename: super::super::Foundation::PWSTR, flags: u32, statusvaluebuffersize: u32, statusvaluebuffer: *mut EC_VARIANT, statusvaluebufferused: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcInsertObjectArrayElement(objectarray: isize, arrayindex: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcOpenSubscription(subscriptionname: super::super::Foundation::PWSTR, accessmask: u32, flags: u32) -> isize;
    pub fn EcOpenSubscriptionEnum(flags: u32) -> isize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcRemoveObjectArrayElement(objectarray: isize, arrayindex: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcRetrySubscription(subscriptionname: super::super::Foundation::PWSTR, eventsourcename: super::super::Foundation::PWSTR, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcSaveSubscription(subscription: isize, flags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcSetObjectArrayProperty(objectarray: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, arrayindex: u32, flags: u32, propertyvalue: *mut EC_VARIANT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EcSetSubscriptionProperty(subscription: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, flags: u32, propertyvalue: *mut EC_VARIANT) -> super::super::Foundation::BOOL;
}
pub const EC_CREATE_NEW: u32 = 1u32;
pub const EC_OPEN_ALWAYS: u32 = 0u32;
pub const EC_OPEN_EXISTING: u32 = 2u32;
pub const EC_READ_ACCESS: u32 = 1u32;
pub const EcConfigurationModeNormal: i32 = 0i32;
pub const EcConfigurationModeCustom: i32 = 1i32;
pub const EcConfigurationModeMinLatency: i32 = 2i32;
pub const EcConfigurationModeMinBandwidth: i32 = 3i32;
pub const EcContentFormatEvents: i32 = 1i32;
pub const EcContentFormatRenderedText: i32 = 2i32;
pub const EcSubscriptionCredDefault: i32 = 0i32;
pub const EcSubscriptionCredNegotiate: i32 = 1i32;
pub const EcSubscriptionCredDigest: i32 = 2i32;
pub const EcSubscriptionCredBasic: i32 = 3i32;
pub const EcSubscriptionCredLocalMachine: i32 = 4i32;
pub const EcDeliveryModePull: i32 = 1i32;
pub const EcDeliveryModePush: i32 = 2i32;
pub const EcSubscriptionEnabled: i32 = 0i32;
pub const EcSubscriptionEventSources: i32 = 1i32;
pub const EcSubscriptionEventSourceAddress: i32 = 2i32;
pub const EcSubscriptionEventSourceEnabled: i32 = 3i32;
pub const EcSubscriptionEventSourceUserName: i32 = 4i32;
pub const EcSubscriptionEventSourcePassword: i32 = 5i32;
pub const EcSubscriptionDescription: i32 = 6i32;
pub const EcSubscriptionURI: i32 = 7i32;
pub const EcSubscriptionConfigurationMode: i32 = 8i32;
pub const EcSubscriptionExpires: i32 = 9i32;
pub const EcSubscriptionQuery: i32 = 10i32;
pub const EcSubscriptionTransportName: i32 = 11i32;
pub const EcSubscriptionTransportPort: i32 = 12i32;
pub const EcSubscriptionDeliveryMode: i32 = 13i32;
pub const EcSubscriptionDeliveryMaxItems: i32 = 14i32;
pub const EcSubscriptionDeliveryMaxLatencyTime: i32 = 15i32;
pub const EcSubscriptionHeartbeatInterval: i32 = 16i32;
pub const EcSubscriptionLocale: i32 = 17i32;
pub const EcSubscriptionContentFormat: i32 = 18i32;
pub const EcSubscriptionLogFile: i32 = 19i32;
pub const EcSubscriptionPublisherName: i32 = 20i32;
pub const EcSubscriptionCredentialsType: i32 = 21i32;
pub const EcSubscriptionCommonUserName: i32 = 22i32;
pub const EcSubscriptionCommonPassword: i32 = 23i32;
pub const EcSubscriptionHostName: i32 = 24i32;
pub const EcSubscriptionReadExistingEvents: i32 = 25i32;
pub const EcSubscriptionDialect: i32 = 26i32;
pub const EcSubscriptionType: i32 = 27i32;
pub const EcSubscriptionAllowedIssuerCAs: i32 = 28i32;
pub const EcSubscriptionAllowedSubjects: i32 = 29i32;
pub const EcSubscriptionDeniedSubjects: i32 = 30i32;
pub const EcSubscriptionAllowedSourceDomainComputers: i32 = 31i32;
pub const EcSubscriptionPropertyIdEND: i32 = 32i32;
pub const EcRuntimeStatusActiveStatusDisabled: i32 = 1i32;
pub const EcRuntimeStatusActiveStatusActive: i32 = 2i32;
pub const EcRuntimeStatusActiveStatusInactive: i32 = 3i32;
pub const EcRuntimeStatusActiveStatusTrying: i32 = 4i32;
pub const EcSubscriptionRunTimeStatusActive: i32 = 0i32;
pub const EcSubscriptionRunTimeStatusLastError: i32 = 1i32;
pub const EcSubscriptionRunTimeStatusLastErrorMessage: i32 = 2i32;
pub const EcSubscriptionRunTimeStatusLastErrorTime: i32 = 3i32;
pub const EcSubscriptionRunTimeStatusNextRetryTime: i32 = 4i32;
pub const EcSubscriptionRunTimeStatusEventSources: i32 = 5i32;
pub const EcSubscriptionRunTimeStatusLastHeartbeatTime: i32 = 6i32;
pub const EcSubscriptionRunTimeStatusInfoIdEND: i32 = 7i32;
pub const EcSubscriptionTypeSourceInitiated: i32 = 0i32;
pub const EcSubscriptionTypeCollectorInitiated: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct EC_VARIANT {
    pub Anonymous: EC_VARIANT_0,
    pub Count: u32,
    pub Type: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EC_VARIANT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EC_VARIANT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union EC_VARIANT_0 {
    pub BooleanVal: super::super::Foundation::BOOL,
    pub UInt32Val: u32,
    pub DateTimeVal: u64,
    pub StringVal: super::super::Foundation::PWSTR,
    pub BinaryVal: *mut u8,
    pub BooleanArr: *mut super::super::Foundation::BOOL,
    pub Int32Arr: *mut i32,
    pub StringArr: *mut super::super::Foundation::PWSTR,
    pub PropertyHandleVal: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EC_VARIANT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EC_VARIANT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const EcVarTypeNull: i32 = 0i32;
pub const EcVarTypeBoolean: i32 = 1i32;
pub const EcVarTypeUInt32: i32 = 2i32;
pub const EcVarTypeDateTime: i32 = 3i32;
pub const EcVarTypeString: i32 = 4i32;
pub const EcVarObjectArrayPropertyHandle: i32 = 5i32;
pub const EC_VARIANT_TYPE_ARRAY: u32 = 128u32;
pub const EC_VARIANT_TYPE_MASK: u32 = 127u32;
pub const EC_WRITE_ACCESS: u32 = 2u32;
