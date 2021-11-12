#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPlatformDiagnosticActionsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlatformDiagnosticTraceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlatformDiagnosticTraceRuntimeInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PlatformDiagnosticActionState(i32);
#[repr(C)]
pub struct PlatformDiagnosticEscalationType(i32);
#[repr(C)]
pub struct PlatformDiagnosticEventBufferLatencies(i32);
#[repr(transparent)]
pub struct PlatformDiagnosticTraceInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PlatformDiagnosticTracePriority(i32);
#[repr(transparent)]
pub struct PlatformDiagnosticTraceRuntimeInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PlatformDiagnosticTraceSlotState(i32);
#[repr(C)]
pub struct PlatformDiagnosticTraceSlotType(i32);
