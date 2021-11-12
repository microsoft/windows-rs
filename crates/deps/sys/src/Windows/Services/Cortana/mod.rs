#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CortanaActionableInsights(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CortanaActionableInsightsOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CortanaPermission(pub i32);
impl CortanaPermission {
    pub const BrowsingHistory: CortanaPermission = CortanaPermission(0i32);
    pub const Calendar: CortanaPermission = CortanaPermission(1i32);
    pub const CallHistory: CortanaPermission = CortanaPermission(2i32);
    pub const Contacts: CortanaPermission = CortanaPermission(3i32);
    pub const Email: CortanaPermission = CortanaPermission(4i32);
    pub const InputPersonalization: CortanaPermission = CortanaPermission(5i32);
    pub const Location: CortanaPermission = CortanaPermission(6i32);
    pub const Messaging: CortanaPermission = CortanaPermission(7i32);
    pub const Microphone: CortanaPermission = CortanaPermission(8i32);
    pub const Personalization: CortanaPermission = CortanaPermission(9i32);
    pub const PhoneCall: CortanaPermission = CortanaPermission(10i32);
}
#[repr(transparent)]
pub struct CortanaPermissionsChangeResult(pub i32);
impl CortanaPermissionsChangeResult {
    pub const Success: CortanaPermissionsChangeResult = CortanaPermissionsChangeResult(0i32);
    pub const Unavailable: CortanaPermissionsChangeResult = CortanaPermissionsChangeResult(1i32);
    pub const DisabledByPolicy: CortanaPermissionsChangeResult = CortanaPermissionsChangeResult(2i32);
}
#[repr(transparent)]
pub struct CortanaPermissionsManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CortanaSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICortanaActionableInsights(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICortanaActionableInsightsOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICortanaActionableInsightsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICortanaPermissionsManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICortanaPermissionsManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICortanaSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICortanaSettingsStatics(pub *mut ::core::ffi::c_void);
