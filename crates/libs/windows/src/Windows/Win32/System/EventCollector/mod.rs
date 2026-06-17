#[inline]
pub unsafe fn EcClose(object: isize) -> windows_core::BOOL {
    windows_core::link!("wecapi.dll" "system" fn EcClose(object : isize) -> windows_core::BOOL);
    unsafe { EcClose(object) }
}
#[inline]
pub unsafe fn EcDeleteSubscription<P0>(subscriptionname: P0, flags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wecapi.dll" "system" fn EcDeleteSubscription(subscriptionname : windows_core::PCWSTR, flags : u32) -> windows_core::BOOL);
    unsafe { EcDeleteSubscription(subscriptionname.param().abi(), flags) }
}
#[inline]
pub unsafe fn EcEnumNextSubscription(subscriptionenum: isize, subscriptionnamebuffer: Option<&mut [u16]>, subscriptionnamebufferused: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wecapi.dll" "system" fn EcEnumNextSubscription(subscriptionenum : isize, subscriptionnamebuffersize : u32, subscriptionnamebuffer : windows_core::PWSTR, subscriptionnamebufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EcEnumNextSubscription(subscriptionenum, subscriptionnamebuffer.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(subscriptionnamebuffer.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), subscriptionnamebufferused as _) }
}
#[inline]
pub unsafe fn EcGetObjectArrayProperty(objectarray: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, arrayindex: u32, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EC_VARIANT, propertyvaluebufferused: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wecapi.dll" "system" fn EcGetObjectArrayProperty(objectarray : isize, propertyid : EC_SUBSCRIPTION_PROPERTY_ID, arrayindex : u32, flags : u32, propertyvaluebuffersize : u32, propertyvaluebuffer : *mut EC_VARIANT, propertyvaluebufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EcGetObjectArrayProperty(objectarray, propertyid, arrayindex, flags, propertyvaluebuffersize, propertyvaluebuffer as _, propertyvaluebufferused as _) }
}
#[inline]
pub unsafe fn EcGetObjectArraySize(objectarray: isize, objectarraysize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wecapi.dll" "system" fn EcGetObjectArraySize(objectarray : isize, objectarraysize : *mut u32) -> windows_core::BOOL);
    unsafe { EcGetObjectArraySize(objectarray, objectarraysize as _) }
}
#[inline]
pub unsafe fn EcGetSubscriptionProperty(subscription: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, flags: u32, propertyvaluebuffersize: u32, propertyvaluebuffer: *mut EC_VARIANT, propertyvaluebufferused: *mut u32) -> windows_core::BOOL {
    windows_core::link!("wecapi.dll" "system" fn EcGetSubscriptionProperty(subscription : isize, propertyid : EC_SUBSCRIPTION_PROPERTY_ID, flags : u32, propertyvaluebuffersize : u32, propertyvaluebuffer : *mut EC_VARIANT, propertyvaluebufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EcGetSubscriptionProperty(subscription, propertyid, flags, propertyvaluebuffersize, propertyvaluebuffer as _, propertyvaluebufferused as _) }
}
#[inline]
pub unsafe fn EcGetSubscriptionRunTimeStatus<P0, P2>(subscriptionname: P0, statusinfoid: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID, eventsourcename: P2, flags: u32, statusvaluebuffersize: u32, statusvaluebuffer: *mut EC_VARIANT, statusvaluebufferused: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wecapi.dll" "system" fn EcGetSubscriptionRunTimeStatus(subscriptionname : windows_core::PCWSTR, statusinfoid : EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID, eventsourcename : windows_core::PCWSTR, flags : u32, statusvaluebuffersize : u32, statusvaluebuffer : *mut EC_VARIANT, statusvaluebufferused : *mut u32) -> windows_core::BOOL);
    unsafe { EcGetSubscriptionRunTimeStatus(subscriptionname.param().abi(), statusinfoid, eventsourcename.param().abi(), flags, statusvaluebuffersize, statusvaluebuffer as _, statusvaluebufferused as _) }
}
#[inline]
pub unsafe fn EcInsertObjectArrayElement(objectarray: isize, arrayindex: u32) -> windows_core::BOOL {
    windows_core::link!("wecapi.dll" "system" fn EcInsertObjectArrayElement(objectarray : isize, arrayindex : u32) -> windows_core::BOOL);
    unsafe { EcInsertObjectArrayElement(objectarray, arrayindex) }
}
#[inline]
pub unsafe fn EcOpenSubscription<P0>(subscriptionname: P0, accessmask: u32, flags: u32) -> isize
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wecapi.dll" "system" fn EcOpenSubscription(subscriptionname : windows_core::PCWSTR, accessmask : u32, flags : u32) -> isize);
    unsafe { EcOpenSubscription(subscriptionname.param().abi(), accessmask, flags) }
}
#[inline]
pub unsafe fn EcOpenSubscriptionEnum(flags: u32) -> isize {
    windows_core::link!("wecapi.dll" "system" fn EcOpenSubscriptionEnum(flags : u32) -> isize);
    unsafe { EcOpenSubscriptionEnum(flags) }
}
#[inline]
pub unsafe fn EcRemoveObjectArrayElement(objectarray: isize, arrayindex: u32) -> windows_core::BOOL {
    windows_core::link!("wecapi.dll" "system" fn EcRemoveObjectArrayElement(objectarray : isize, arrayindex : u32) -> windows_core::BOOL);
    unsafe { EcRemoveObjectArrayElement(objectarray, arrayindex) }
}
#[inline]
pub unsafe fn EcRetrySubscription<P0, P1>(subscriptionname: P0, eventsourcename: P1, flags: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wecapi.dll" "system" fn EcRetrySubscription(subscriptionname : windows_core::PCWSTR, eventsourcename : windows_core::PCWSTR, flags : u32) -> windows_core::BOOL);
    unsafe { EcRetrySubscription(subscriptionname.param().abi(), eventsourcename.param().abi(), flags) }
}
#[inline]
pub unsafe fn EcSaveSubscription(subscription: isize, flags: u32) -> windows_core::BOOL {
    windows_core::link!("wecapi.dll" "system" fn EcSaveSubscription(subscription : isize, flags : u32) -> windows_core::BOOL);
    unsafe { EcSaveSubscription(subscription, flags) }
}
#[inline]
pub unsafe fn EcSetObjectArrayProperty(objectarray: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, arrayindex: u32, flags: u32, propertyvalue: *mut EC_VARIANT) -> windows_core::BOOL {
    windows_core::link!("wecapi.dll" "system" fn EcSetObjectArrayProperty(objectarray : isize, propertyid : EC_SUBSCRIPTION_PROPERTY_ID, arrayindex : u32, flags : u32, propertyvalue : *mut EC_VARIANT) -> windows_core::BOOL);
    unsafe { EcSetObjectArrayProperty(objectarray, propertyid, arrayindex, flags, propertyvalue as _) }
}
#[inline]
pub unsafe fn EcSetSubscriptionProperty(subscription: isize, propertyid: EC_SUBSCRIPTION_PROPERTY_ID, flags: u32, propertyvalue: *mut EC_VARIANT) -> windows_core::BOOL {
    windows_core::link!("wecapi.dll" "system" fn EcSetSubscriptionProperty(subscription : isize, propertyid : EC_SUBSCRIPTION_PROPERTY_ID, flags : u32, propertyvalue : *mut EC_VARIANT) -> windows_core::BOOL);
    unsafe { EcSetSubscriptionProperty(subscription, propertyid, flags, propertyvalue as _) }
}
pub const EC_CREATE_NEW: u32 = 1;
pub const EC_OPEN_ALWAYS: u32 = 0;
pub const EC_OPEN_EXISTING: u32 = 2;
pub const EC_READ_ACCESS: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EC_SUBSCRIPTION_CONFIGURATION_MODE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EC_SUBSCRIPTION_CONTENT_FORMAT(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EC_SUBSCRIPTION_CREDENTIALS_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EC_SUBSCRIPTION_DELIVERY_MODE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EC_SUBSCRIPTION_PROPERTY_ID(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EC_SUBSCRIPTION_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EC_VARIANT {
    pub Anonymous: EC_VARIANT_0,
    pub Count: u32,
    pub Type: u32,
}
impl Default for EC_VARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EC_VARIANT_0 {
    pub BooleanVal: windows_core::BOOL,
    pub UInt32Val: u32,
    pub DateTimeVal: u64,
    pub StringVal: windows_core::PCWSTR,
    pub BinaryVal: *mut u8,
    pub BooleanArr: *mut windows_core::BOOL,
    pub Int32Arr: *mut i32,
    pub StringArr: *mut windows_core::PWSTR,
    pub PropertyHandleVal: isize,
}
impl Default for EC_VARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EC_VARIANT_TYPE(pub i32);
pub const EC_VARIANT_TYPE_ARRAY: u32 = 128;
pub const EC_VARIANT_TYPE_MASK: u32 = 127;
pub const EC_WRITE_ACCESS: u32 = 2;
pub const EcConfigurationModeCustom: EC_SUBSCRIPTION_CONFIGURATION_MODE = EC_SUBSCRIPTION_CONFIGURATION_MODE(1);
pub const EcConfigurationModeMinBandwidth: EC_SUBSCRIPTION_CONFIGURATION_MODE = EC_SUBSCRIPTION_CONFIGURATION_MODE(3);
pub const EcConfigurationModeMinLatency: EC_SUBSCRIPTION_CONFIGURATION_MODE = EC_SUBSCRIPTION_CONFIGURATION_MODE(2);
pub const EcConfigurationModeNormal: EC_SUBSCRIPTION_CONFIGURATION_MODE = EC_SUBSCRIPTION_CONFIGURATION_MODE(0);
pub const EcContentFormatEvents: EC_SUBSCRIPTION_CONTENT_FORMAT = EC_SUBSCRIPTION_CONTENT_FORMAT(1);
pub const EcContentFormatRenderedText: EC_SUBSCRIPTION_CONTENT_FORMAT = EC_SUBSCRIPTION_CONTENT_FORMAT(2);
pub const EcDeliveryModePull: EC_SUBSCRIPTION_DELIVERY_MODE = EC_SUBSCRIPTION_DELIVERY_MODE(1);
pub const EcDeliveryModePush: EC_SUBSCRIPTION_DELIVERY_MODE = EC_SUBSCRIPTION_DELIVERY_MODE(2);
pub const EcRuntimeStatusActiveStatusActive: EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS = EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS(2);
pub const EcRuntimeStatusActiveStatusDisabled: EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS = EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS(1);
pub const EcRuntimeStatusActiveStatusInactive: EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS = EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS(3);
pub const EcRuntimeStatusActiveStatusTrying: EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS = EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS(4);
pub const EcSubscriptionAllowedIssuerCAs: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(28);
pub const EcSubscriptionAllowedSourceDomainComputers: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(31);
pub const EcSubscriptionAllowedSubjects: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(29);
pub const EcSubscriptionCommonPassword: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(23);
pub const EcSubscriptionCommonUserName: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(22);
pub const EcSubscriptionConfigurationMode: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(8);
pub const EcSubscriptionContentFormat: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(18);
pub const EcSubscriptionCredBasic: EC_SUBSCRIPTION_CREDENTIALS_TYPE = EC_SUBSCRIPTION_CREDENTIALS_TYPE(3);
pub const EcSubscriptionCredDefault: EC_SUBSCRIPTION_CREDENTIALS_TYPE = EC_SUBSCRIPTION_CREDENTIALS_TYPE(0);
pub const EcSubscriptionCredDigest: EC_SUBSCRIPTION_CREDENTIALS_TYPE = EC_SUBSCRIPTION_CREDENTIALS_TYPE(2);
pub const EcSubscriptionCredLocalMachine: EC_SUBSCRIPTION_CREDENTIALS_TYPE = EC_SUBSCRIPTION_CREDENTIALS_TYPE(4);
pub const EcSubscriptionCredNegotiate: EC_SUBSCRIPTION_CREDENTIALS_TYPE = EC_SUBSCRIPTION_CREDENTIALS_TYPE(1);
pub const EcSubscriptionCredentialsType: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(21);
pub const EcSubscriptionDeliveryMaxItems: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(14);
pub const EcSubscriptionDeliveryMaxLatencyTime: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(15);
pub const EcSubscriptionDeliveryMode: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(13);
pub const EcSubscriptionDeniedSubjects: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(30);
pub const EcSubscriptionDescription: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(6);
pub const EcSubscriptionDialect: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(26);
pub const EcSubscriptionEnabled: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(0);
pub const EcSubscriptionEventSourceAddress: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(2);
pub const EcSubscriptionEventSourceEnabled: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(3);
pub const EcSubscriptionEventSourcePassword: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(5);
pub const EcSubscriptionEventSourceUserName: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(4);
pub const EcSubscriptionEventSources: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(1);
pub const EcSubscriptionExpires: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(9);
pub const EcSubscriptionHeartbeatInterval: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(16);
pub const EcSubscriptionHostName: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(24);
pub const EcSubscriptionLocale: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(17);
pub const EcSubscriptionLogFile: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(19);
pub const EcSubscriptionPropertyIdEND: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(32);
pub const EcSubscriptionPublisherName: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(20);
pub const EcSubscriptionQuery: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(10);
pub const EcSubscriptionReadExistingEvents: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(25);
pub const EcSubscriptionRunTimeStatusActive: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(0);
pub const EcSubscriptionRunTimeStatusEventSources: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(5);
pub const EcSubscriptionRunTimeStatusInfoIdEND: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(7);
pub const EcSubscriptionRunTimeStatusLastError: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(1);
pub const EcSubscriptionRunTimeStatusLastErrorMessage: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(2);
pub const EcSubscriptionRunTimeStatusLastErrorTime: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(3);
pub const EcSubscriptionRunTimeStatusLastHeartbeatTime: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(6);
pub const EcSubscriptionRunTimeStatusNextRetryTime: EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID = EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID(4);
pub const EcSubscriptionTransportName: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(11);
pub const EcSubscriptionTransportPort: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(12);
pub const EcSubscriptionType: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(27);
pub const EcSubscriptionTypeCollectorInitiated: EC_SUBSCRIPTION_TYPE = EC_SUBSCRIPTION_TYPE(1);
pub const EcSubscriptionTypeSourceInitiated: EC_SUBSCRIPTION_TYPE = EC_SUBSCRIPTION_TYPE(0);
pub const EcSubscriptionURI: EC_SUBSCRIPTION_PROPERTY_ID = EC_SUBSCRIPTION_PROPERTY_ID(7);
pub const EcVarObjectArrayPropertyHandle: EC_VARIANT_TYPE = EC_VARIANT_TYPE(5);
pub const EcVarTypeBoolean: EC_VARIANT_TYPE = EC_VARIANT_TYPE(1);
pub const EcVarTypeDateTime: EC_VARIANT_TYPE = EC_VARIANT_TYPE(3);
pub const EcVarTypeNull: EC_VARIANT_TYPE = EC_VARIANT_TYPE(0);
pub const EcVarTypeString: EC_VARIANT_TYPE = EC_VARIANT_TYPE(4);
pub const EcVarTypeUInt32: EC_VARIANT_TYPE = EC_VARIANT_TYPE(2);
