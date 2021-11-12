#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CortanaActionableInsights(i32);
pub struct CortanaActionableInsightsOptions(i32);
pub struct CortanaPermission(i32);
pub struct CortanaPermissionsChangeResult(i32);
pub struct CortanaPermissionsManager(i32);
pub struct CortanaSettings(i32);
pub struct ICortanaActionableInsights(pub *mut ::core::ffi::c_void);
pub struct ICortanaActionableInsightsOptions(pub *mut ::core::ffi::c_void);
pub struct ICortanaActionableInsightsStatics(pub *mut ::core::ffi::c_void);
pub struct ICortanaPermissionsManager(pub *mut ::core::ffi::c_void);
pub struct ICortanaPermissionsManagerStatics(pub *mut ::core::ffi::c_void);
pub struct ICortanaSettings(pub *mut ::core::ffi::c_void);
pub struct ICortanaSettingsStatics(pub *mut ::core::ffi::c_void);
