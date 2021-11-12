#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ConditionForceEffect(i32);
pub struct ConditionForceEffectKind(i32);
pub struct ConstantForceEffect(i32);
pub struct ForceFeedbackEffectAxes(i32);
pub struct ForceFeedbackEffectState(i32);
pub struct ForceFeedbackLoadEffectResult(i32);
pub struct ForceFeedbackMotor(i32);
pub struct IConditionForceEffect(pub *mut ::core::ffi::c_void);
pub struct IConditionForceEffectFactory(pub *mut ::core::ffi::c_void);
pub struct IConstantForceEffect(pub *mut ::core::ffi::c_void);
pub struct IForceFeedbackEffect(pub *mut ::core::ffi::c_void);
pub struct IForceFeedbackMotor(pub *mut ::core::ffi::c_void);
pub struct IPeriodicForceEffect(pub *mut ::core::ffi::c_void);
pub struct IPeriodicForceEffectFactory(pub *mut ::core::ffi::c_void);
pub struct IRampForceEffect(pub *mut ::core::ffi::c_void);
pub struct PeriodicForceEffect(i32);
pub struct PeriodicForceEffectKind(i32);
pub struct RampForceEffect(i32);
