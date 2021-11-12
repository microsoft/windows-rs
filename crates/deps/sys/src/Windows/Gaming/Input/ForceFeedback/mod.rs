#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ConditionForceEffect(pub *mut ::core::ffi::c_void);
pub struct ConditionForceEffectKind(i32);
#[repr(transparent)]
pub struct ConstantForceEffect(pub *mut ::core::ffi::c_void);
pub struct ForceFeedbackEffectAxes(i32);
pub struct ForceFeedbackEffectState(i32);
pub struct ForceFeedbackLoadEffectResult(i32);
#[repr(transparent)]
pub struct ForceFeedbackMotor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConditionForceEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConditionForceEffectFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConstantForceEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IForceFeedbackEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IForceFeedbackMotor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPeriodicForceEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPeriodicForceEffectFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRampForceEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PeriodicForceEffect(pub *mut ::core::ffi::c_void);
pub struct PeriodicForceEffectKind(i32);
#[repr(transparent)]
pub struct RampForceEffect(pub *mut ::core::ffi::c_void);
