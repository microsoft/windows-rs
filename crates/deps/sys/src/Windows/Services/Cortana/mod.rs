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
    pub const BrowsingHistory: Self = Self(0i32);
    pub const Calendar: Self = Self(1i32);
    pub const CallHistory: Self = Self(2i32);
    pub const Contacts: Self = Self(3i32);
    pub const Email: Self = Self(4i32);
    pub const InputPersonalization: Self = Self(5i32);
    pub const Location: Self = Self(6i32);
    pub const Messaging: Self = Self(7i32);
    pub const Microphone: Self = Self(8i32);
    pub const Personalization: Self = Self(9i32);
    pub const PhoneCall: Self = Self(10i32);
}
#[repr(transparent)]
pub struct CortanaPermissionsChangeResult(pub i32);
impl CortanaPermissionsChangeResult {
    pub const Success: Self = Self(0i32);
    pub const Unavailable: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
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
