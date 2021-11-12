#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CortanaActionableInsights(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CortanaActionableInsightsOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CortanaPermission(i32);
#[repr(C)]
pub struct CortanaPermissionsChangeResult(i32);
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
