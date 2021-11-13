#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CortanaActionableInsights(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CortanaActionableInsights {}
impl ::core::clone::Clone for CortanaActionableInsights {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CortanaActionableInsightsOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CortanaActionableInsightsOptions {}
impl ::core::clone::Clone for CortanaActionableInsightsOptions {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for CortanaPermission {}
impl ::core::clone::Clone for CortanaPermission {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CortanaPermissionsChangeResult(pub i32);
impl CortanaPermissionsChangeResult {
    pub const Success: Self = Self(0i32);
    pub const Unavailable: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
}
impl ::core::marker::Copy for CortanaPermissionsChangeResult {}
impl ::core::clone::Clone for CortanaPermissionsChangeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CortanaPermissionsManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CortanaPermissionsManager {}
impl ::core::clone::Clone for CortanaPermissionsManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CortanaSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CortanaSettings {}
impl ::core::clone::Clone for CortanaSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICortanaActionableInsights(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICortanaActionableInsights {}
impl ::core::clone::Clone for ICortanaActionableInsights {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICortanaActionableInsightsOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICortanaActionableInsightsOptions {}
impl ::core::clone::Clone for ICortanaActionableInsightsOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICortanaActionableInsightsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICortanaActionableInsightsStatics {}
impl ::core::clone::Clone for ICortanaActionableInsightsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICortanaPermissionsManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICortanaPermissionsManager {}
impl ::core::clone::Clone for ICortanaPermissionsManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICortanaPermissionsManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICortanaPermissionsManagerStatics {}
impl ::core::clone::Clone for ICortanaPermissionsManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICortanaSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICortanaSettings {}
impl ::core::clone::Clone for ICortanaSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICortanaSettingsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICortanaSettingsStatics {}
impl ::core::clone::Clone for ICortanaSettingsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
