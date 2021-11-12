#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "System_Diagnostics_DevicePortal")]
pub mod DevicePortal;
#[cfg(feature = "System_Diagnostics_Telemetry")]
pub mod Telemetry;
#[cfg(feature = "System_Diagnostics_TraceReporting")]
pub mod TraceReporting;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DiagnosticActionResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DiagnosticActionState(pub i32);
impl DiagnosticActionState {
    pub const Initializing: Self = Self(0i32);
    pub const Downloading: Self = Self(1i32);
    pub const VerifyingTrust: Self = Self(2i32);
    pub const Detecting: Self = Self(3i32);
    pub const Resolving: Self = Self(4i32);
    pub const VerifyingResolution: Self = Self(5i32);
    pub const Executing: Self = Self(6i32);
}
#[repr(transparent)]
pub struct DiagnosticInvoker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiagnosticActionResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiagnosticInvoker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiagnosticInvoker2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDiagnosticInvokerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessCpuUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessCpuUsageReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessDiagnosticInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessDiagnosticInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessDiagnosticInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessDiagnosticInfoStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessDiskUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessDiskUsageReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessMemoryUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProcessMemoryUsageReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemCpuUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemCpuUsageReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemDiagnosticInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemDiagnosticInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemDiagnosticInfoStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemMemoryUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemMemoryUsageReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProcessCpuUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProcessCpuUsageReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProcessDiagnosticInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProcessDiskUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProcessDiskUsageReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProcessMemoryUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ProcessMemoryUsageReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemCpuUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemCpuUsageReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemDiagnosticInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemMemoryUsage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemMemoryUsageReport(pub *mut ::core::ffi::c_void);
