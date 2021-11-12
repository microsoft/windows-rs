#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CompositionDebugHeatMaps(i32);
pub struct CompositionDebugOverdrawContentKinds(i32);
pub struct CompositionDebugSettings(i32);
pub struct ICompositionDebugHeatMaps(pub *mut ::core::ffi::c_void);
pub struct ICompositionDebugSettings(pub *mut ::core::ffi::c_void);
pub struct ICompositionDebugSettingsStatics(pub *mut ::core::ffi::c_void);
