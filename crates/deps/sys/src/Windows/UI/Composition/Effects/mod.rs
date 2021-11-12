#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ISceneLightingEffect(pub *mut ::core::ffi::c_void);
pub struct ISceneLightingEffect2(pub *mut ::core::ffi::c_void);
pub struct SceneLightingEffect(i32);
pub struct SceneLightingEffectReflectanceModel(i32);
