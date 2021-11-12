#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ConditionForceEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ConditionForceEffect {}
impl ::core::clone::Clone for ConditionForceEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ConditionForceEffectKind(pub i32);
impl ConditionForceEffectKind {
    pub const Spring: Self = Self(0i32);
    pub const Damper: Self = Self(1i32);
    pub const Inertia: Self = Self(2i32);
    pub const Friction: Self = Self(3i32);
}
impl ::core::marker::Copy for ConditionForceEffectKind {}
impl ::core::clone::Clone for ConditionForceEffectKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ConstantForceEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ConstantForceEffect {}
impl ::core::clone::Clone for ConstantForceEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ForceFeedbackEffectAxes(pub u32);
impl ForceFeedbackEffectAxes {
    pub const None: Self = Self(0u32);
    pub const X: Self = Self(1u32);
    pub const Y: Self = Self(2u32);
    pub const Z: Self = Self(4u32);
}
impl ::core::marker::Copy for ForceFeedbackEffectAxes {}
impl ::core::clone::Clone for ForceFeedbackEffectAxes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ForceFeedbackEffectState(pub i32);
impl ForceFeedbackEffectState {
    pub const Stopped: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Paused: Self = Self(2i32);
    pub const Faulted: Self = Self(3i32);
}
impl ::core::marker::Copy for ForceFeedbackEffectState {}
impl ::core::clone::Clone for ForceFeedbackEffectState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ForceFeedbackLoadEffectResult(pub i32);
impl ForceFeedbackLoadEffectResult {
    pub const Succeeded: Self = Self(0i32);
    pub const EffectStorageFull: Self = Self(1i32);
    pub const EffectNotSupported: Self = Self(2i32);
}
impl ::core::marker::Copy for ForceFeedbackLoadEffectResult {}
impl ::core::clone::Clone for ForceFeedbackLoadEffectResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ForceFeedbackMotor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ForceFeedbackMotor {}
impl ::core::clone::Clone for ForceFeedbackMotor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConditionForceEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConditionForceEffect {}
impl ::core::clone::Clone for IConditionForceEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConditionForceEffectFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConditionForceEffectFactory {}
impl ::core::clone::Clone for IConditionForceEffectFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IConstantForceEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IConstantForceEffect {}
impl ::core::clone::Clone for IConstantForceEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IForceFeedbackEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IForceFeedbackEffect {}
impl ::core::clone::Clone for IForceFeedbackEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IForceFeedbackMotor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IForceFeedbackMotor {}
impl ::core::clone::Clone for IForceFeedbackMotor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPeriodicForceEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPeriodicForceEffect {}
impl ::core::clone::Clone for IPeriodicForceEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPeriodicForceEffectFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPeriodicForceEffectFactory {}
impl ::core::clone::Clone for IPeriodicForceEffectFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRampForceEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRampForceEffect {}
impl ::core::clone::Clone for IRampForceEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PeriodicForceEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PeriodicForceEffect {}
impl ::core::clone::Clone for PeriodicForceEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PeriodicForceEffectKind(pub i32);
impl PeriodicForceEffectKind {
    pub const SquareWave: Self = Self(0i32);
    pub const SineWave: Self = Self(1i32);
    pub const TriangleWave: Self = Self(2i32);
    pub const SawtoothWaveUp: Self = Self(3i32);
    pub const SawtoothWaveDown: Self = Self(4i32);
}
impl ::core::marker::Copy for PeriodicForceEffectKind {}
impl ::core::clone::Clone for PeriodicForceEffectKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RampForceEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RampForceEffect {}
impl ::core::clone::Clone for RampForceEffect {
    fn clone(&self) -> Self {
        *self
    }
}
