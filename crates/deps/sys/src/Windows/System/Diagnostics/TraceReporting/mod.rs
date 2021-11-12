#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPlatformDiagnosticActionsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlatformDiagnosticTraceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlatformDiagnosticTraceRuntimeInfo(pub *mut ::core::ffi::c_void);
pub struct PlatformDiagnosticActionState(i32);
#[repr(transparent)]
pub struct PlatformDiagnosticActions(pub *mut ::core::ffi::c_void);
pub struct PlatformDiagnosticEscalationType(i32);
pub struct PlatformDiagnosticEventBufferLatencies(i32);
#[repr(transparent)]
pub struct PlatformDiagnosticTraceInfo(pub *mut ::core::ffi::c_void);
pub struct PlatformDiagnosticTracePriority(i32);
#[repr(transparent)]
pub struct PlatformDiagnosticTraceRuntimeInfo(pub *mut ::core::ffi::c_void);
pub struct PlatformDiagnosticTraceSlotState(i32);
pub struct PlatformDiagnosticTraceSlotType(i32);
