#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ConditionForceEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConditionForceEffectKind(pub i32);
impl ConditionForceEffectKind {
    pub const Spring: Self = Self(0i32);
    pub const Damper: Self = Self(1i32);
    pub const Inertia: Self = Self(2i32);
    pub const Friction: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ConstantForceEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ForceFeedbackEffectAxes(pub u32);
impl ForceFeedbackEffectAxes {
    pub const None: Self = Self(0u32);
    pub const X: Self = Self(1u32);
    pub const Y: Self = Self(2u32);
    pub const Z: Self = Self(4u32);
}
#[repr(transparent)]
pub struct ForceFeedbackEffectState(pub i32);
impl ForceFeedbackEffectState {
    pub const Stopped: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Paused: Self = Self(2i32);
    pub const Faulted: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ForceFeedbackLoadEffectResult(pub i32);
impl ForceFeedbackLoadEffectResult {
    pub const Succeeded: Self = Self(0i32);
    pub const EffectStorageFull: Self = Self(1i32);
    pub const EffectNotSupported: Self = Self(2i32);
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
    pub const SquareWave: Self = Self(0i32);
    pub const SineWave: Self = Self(1i32);
    pub const TriangleWave: Self = Self(2i32);
    pub const SawtoothWaveUp: Self = Self(3i32);
    pub const SawtoothWaveDown: Self = Self(4i32);
}
#[repr(transparent)]
pub struct RampForceEffect(pub *mut ::core::ffi::c_void);
