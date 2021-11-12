#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISceneLightingEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISceneLightingEffect2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneLightingEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SceneLightingEffectReflectanceModel(pub i32);
impl SceneLightingEffectReflectanceModel {
    pub const BlinnPhong: Self = Self(0i32);
    pub const PhysicallyBasedBlinnPhong: Self = Self(1i32);
}
