#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPlatformDiagnosticActionsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlatformDiagnosticTraceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlatformDiagnosticTraceRuntimeInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlatformDiagnosticActionState(pub i32);
impl PlatformDiagnosticActionState {
    pub const Success: Self = Self(0i32);
    pub const FreeNetworkNotAvailable: Self = Self(1i32);
    pub const ACPowerNotAvailable: Self = Self(2i32);
}
#[repr(transparent)]
pub struct PlatformDiagnosticEscalationType(pub i32);
impl PlatformDiagnosticEscalationType {
    pub const OnCompletion: Self = Self(0i32);
    pub const OnFailure: Self = Self(1i32);
}
#[repr(transparent)]
pub struct PlatformDiagnosticEventBufferLatencies(pub u32);
impl PlatformDiagnosticEventBufferLatencies {
    pub const Normal: Self = Self(1u32);
    pub const CostDeferred: Self = Self(2u32);
    pub const Realtime: Self = Self(4u32);
}
#[repr(transparent)]
pub struct PlatformDiagnosticTraceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlatformDiagnosticTracePriority(pub i32);
impl PlatformDiagnosticTracePriority {
    pub const Normal: Self = Self(0i32);
    pub const UserElevated: Self = Self(1i32);
}
#[repr(transparent)]
pub struct PlatformDiagnosticTraceRuntimeInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlatformDiagnosticTraceSlotState(pub i32);
impl PlatformDiagnosticTraceSlotState {
    pub const NotRunning: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const Throttled: Self = Self(2i32);
}
#[repr(transparent)]
pub struct PlatformDiagnosticTraceSlotType(pub i32);
impl PlatformDiagnosticTraceSlotType {
    pub const Alternative: Self = Self(0i32);
    pub const AlwaysOn: Self = Self(1i32);
    pub const Mini: Self = Self(2i32);
}
