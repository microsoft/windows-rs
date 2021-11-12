#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_Diagnostics_DevicePortal")]
pub mod DevicePortal;
#[cfg(feature = "System_Diagnostics_Telemetry")]
pub mod Telemetry;
#[cfg(feature = "System_Diagnostics_TraceReporting")]
pub mod TraceReporting;
#[link(name = "windows")]
extern "system" {}
pub struct DiagnosticActionResult(i32);
pub struct DiagnosticActionState(i32);
pub struct DiagnosticInvoker(i32);
pub struct IDiagnosticActionResult(pub *mut ::core::ffi::c_void);
pub struct IDiagnosticInvoker(pub *mut ::core::ffi::c_void);
pub struct IDiagnosticInvoker2(pub *mut ::core::ffi::c_void);
pub struct IDiagnosticInvokerStatics(pub *mut ::core::ffi::c_void);
pub struct IProcessCpuUsage(pub *mut ::core::ffi::c_void);
pub struct IProcessCpuUsageReport(pub *mut ::core::ffi::c_void);
pub struct IProcessDiagnosticInfo(pub *mut ::core::ffi::c_void);
pub struct IProcessDiagnosticInfo2(pub *mut ::core::ffi::c_void);
pub struct IProcessDiagnosticInfoStatics(pub *mut ::core::ffi::c_void);
pub struct IProcessDiagnosticInfoStatics2(pub *mut ::core::ffi::c_void);
pub struct IProcessDiskUsage(pub *mut ::core::ffi::c_void);
pub struct IProcessDiskUsageReport(pub *mut ::core::ffi::c_void);
pub struct IProcessMemoryUsage(pub *mut ::core::ffi::c_void);
pub struct IProcessMemoryUsageReport(pub *mut ::core::ffi::c_void);
pub struct ISystemCpuUsage(pub *mut ::core::ffi::c_void);
pub struct ISystemCpuUsageReport(pub *mut ::core::ffi::c_void);
pub struct ISystemDiagnosticInfo(pub *mut ::core::ffi::c_void);
pub struct ISystemDiagnosticInfoStatics(pub *mut ::core::ffi::c_void);
pub struct ISystemDiagnosticInfoStatics2(pub *mut ::core::ffi::c_void);
pub struct ISystemMemoryUsage(pub *mut ::core::ffi::c_void);
pub struct ISystemMemoryUsageReport(pub *mut ::core::ffi::c_void);
pub struct ProcessCpuUsage(i32);
pub struct ProcessCpuUsageReport(i32);
pub struct ProcessDiagnosticInfo(i32);
pub struct ProcessDiskUsage(i32);
pub struct ProcessDiskUsageReport(i32);
pub struct ProcessMemoryUsage(i32);
pub struct ProcessMemoryUsageReport(i32);
pub struct SystemCpuUsage(i32);
pub struct SystemCpuUsageReport(i32);
pub struct SystemDiagnosticInfo(i32);
pub struct SystemMemoryUsage(i32);
pub struct SystemMemoryUsageReport(i32);
