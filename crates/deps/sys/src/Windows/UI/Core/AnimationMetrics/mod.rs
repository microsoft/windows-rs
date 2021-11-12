#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AnimationDescription(i32);
pub struct AnimationEffect(i32);
pub struct AnimationEffectTarget(i32);
pub struct AnimationMetricsContract(i32);
pub struct IAnimationDescription(pub *mut ::core::ffi::c_void);
pub struct IAnimationDescriptionFactory(pub *mut ::core::ffi::c_void);
pub struct IOpacityAnimation(pub *mut ::core::ffi::c_void);
pub struct IPropertyAnimation(pub *mut ::core::ffi::c_void);
pub struct IScaleAnimation(pub *mut ::core::ffi::c_void);
pub struct OpacityAnimation(i32);
pub struct PropertyAnimation(i32);
pub struct PropertyAnimationType(i32);
pub struct ScaleAnimation(i32);
pub struct TranslationAnimation(i32);
