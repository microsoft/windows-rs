#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CompositionDebugHeatMaps(pub *mut ::core::ffi::c_void);
pub struct CompositionDebugOverdrawContentKinds(i32);
#[repr(transparent)]
pub struct CompositionDebugSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionDebugHeatMaps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionDebugSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionDebugSettingsStatics(pub *mut ::core::ffi::c_void);
