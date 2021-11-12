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
    pub const Success: PlatformDiagnosticActionState = PlatformDiagnosticActionState(0i32);
    pub const FreeNetworkNotAvailable: PlatformDiagnosticActionState = PlatformDiagnosticActionState(1i32);
    pub const ACPowerNotAvailable: PlatformDiagnosticActionState = PlatformDiagnosticActionState(2i32);
}
#[repr(transparent)]
pub struct PlatformDiagnosticEscalationType(pub i32);
impl PlatformDiagnosticEscalationType {
    pub const OnCompletion: PlatformDiagnosticEscalationType = PlatformDiagnosticEscalationType(0i32);
    pub const OnFailure: PlatformDiagnosticEscalationType = PlatformDiagnosticEscalationType(1i32);
}
#[repr(transparent)]
pub struct PlatformDiagnosticEventBufferLatencies(pub u32);
impl PlatformDiagnosticEventBufferLatencies {
    pub const Normal: PlatformDiagnosticEventBufferLatencies = PlatformDiagnosticEventBufferLatencies(1u32);
    pub const CostDeferred: PlatformDiagnosticEventBufferLatencies = PlatformDiagnosticEventBufferLatencies(2u32);
    pub const Realtime: PlatformDiagnosticEventBufferLatencies = PlatformDiagnosticEventBufferLatencies(4u32);
}
#[repr(transparent)]
pub struct PlatformDiagnosticTraceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlatformDiagnosticTracePriority(pub i32);
impl PlatformDiagnosticTracePriority {
    pub const Normal: PlatformDiagnosticTracePriority = PlatformDiagnosticTracePriority(0i32);
    pub const UserElevated: PlatformDiagnosticTracePriority = PlatformDiagnosticTracePriority(1i32);
}
#[repr(transparent)]
pub struct PlatformDiagnosticTraceRuntimeInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlatformDiagnosticTraceSlotState(pub i32);
impl PlatformDiagnosticTraceSlotState {
    pub const NotRunning: PlatformDiagnosticTraceSlotState = PlatformDiagnosticTraceSlotState(0i32);
    pub const Running: PlatformDiagnosticTraceSlotState = PlatformDiagnosticTraceSlotState(1i32);
    pub const Throttled: PlatformDiagnosticTraceSlotState = PlatformDiagnosticTraceSlotState(2i32);
}
#[repr(transparent)]
pub struct PlatformDiagnosticTraceSlotType(pub i32);
impl PlatformDiagnosticTraceSlotType {
    pub const Alternative: PlatformDiagnosticTraceSlotType = PlatformDiagnosticTraceSlotType(0i32);
    pub const AlwaysOn: PlatformDiagnosticTraceSlotType = PlatformDiagnosticTraceSlotType(1i32);
    pub const Mini: PlatformDiagnosticTraceSlotType = PlatformDiagnosticTraceSlotType(2i32);
}
