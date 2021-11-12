#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AnimationDescription(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AnimationEffect(i32);
#[repr(C)]
pub struct AnimationEffectTarget(i32);
#[repr(C)]
pub struct AnimationMetricsContract(i32);
#[repr(transparent)]
pub struct IAnimationDescription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnimationDescriptionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOpacityAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScaleAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OpacityAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PropertyAnimation(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PropertyAnimationType(i32);
#[repr(transparent)]
pub struct ScaleAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TranslationAnimation(pub *mut ::core::ffi::c_void);
