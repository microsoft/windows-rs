#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISceneLightingEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneLightingEffect2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneLightingEffect(pub *mut ::core::ffi::c_void);
pub struct SceneLightingEffectReflectanceModel(i32);
