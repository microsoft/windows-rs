#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ConditionForceEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConditionForceEffectKind(pub i32);
impl ConditionForceEffectKind {
    pub const Spring: ConditionForceEffectKind = ConditionForceEffectKind(0i32);
    pub const Damper: ConditionForceEffectKind = ConditionForceEffectKind(1i32);
    pub const Inertia: ConditionForceEffectKind = ConditionForceEffectKind(2i32);
    pub const Friction: ConditionForceEffectKind = ConditionForceEffectKind(3i32);
}
#[repr(transparent)]
pub struct ConstantForceEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ForceFeedbackEffectAxes(pub u32);
impl ForceFeedbackEffectAxes {
    pub const None: ForceFeedbackEffectAxes = ForceFeedbackEffectAxes(0u32);
    pub const X: ForceFeedbackEffectAxes = ForceFeedbackEffectAxes(1u32);
    pub const Y: ForceFeedbackEffectAxes = ForceFeedbackEffectAxes(2u32);
    pub const Z: ForceFeedbackEffectAxes = ForceFeedbackEffectAxes(4u32);
}
#[repr(transparent)]
pub struct ForceFeedbackEffectState(pub i32);
impl ForceFeedbackEffectState {
    pub const Stopped: ForceFeedbackEffectState = ForceFeedbackEffectState(0i32);
    pub const Running: ForceFeedbackEffectState = ForceFeedbackEffectState(1i32);
    pub const Paused: ForceFeedbackEffectState = ForceFeedbackEffectState(2i32);
    pub const Faulted: ForceFeedbackEffectState = ForceFeedbackEffectState(3i32);
}
#[repr(transparent)]
pub struct ForceFeedbackLoadEffectResult(pub i32);
impl ForceFeedbackLoadEffectResult {
    pub const Succeeded: ForceFeedbackLoadEffectResult = ForceFeedbackLoadEffectResult(0i32);
    pub const EffectStorageFull: ForceFeedbackLoadEffectResult = ForceFeedbackLoadEffectResult(1i32);
    pub const EffectNotSupported: ForceFeedbackLoadEffectResult = ForceFeedbackLoadEffectResult(2i32);
}
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
#[repr(transparent)]
pub struct PeriodicForceEffectKind(pub i32);
impl PeriodicForceEffectKind {
    pub const SquareWave: PeriodicForceEffectKind = PeriodicForceEffectKind(0i32);
    pub const SineWave: PeriodicForceEffectKind = PeriodicForceEffectKind(1i32);
    pub const TriangleWave: PeriodicForceEffectKind = PeriodicForceEffectKind(2i32);
    pub const SawtoothWaveUp: PeriodicForceEffectKind = PeriodicForceEffectKind(3i32);
    pub const SawtoothWaveDown: PeriodicForceEffectKind = PeriodicForceEffectKind(4i32);
}
#[repr(transparent)]
pub struct RampForceEffect(pub *mut ::core::ffi::c_void);
