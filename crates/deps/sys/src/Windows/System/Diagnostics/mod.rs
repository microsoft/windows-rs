#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
impl ::core::marker::Copy for DiagnosticActionResult {}
impl ::core::clone::Clone for DiagnosticActionResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DiagnosticActionState {}
impl ::core::clone::Clone for DiagnosticActionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DiagnosticInvoker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DiagnosticInvoker {}
impl ::core::clone::Clone for DiagnosticInvoker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiagnosticActionResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiagnosticActionResult {}
impl ::core::clone::Clone for IDiagnosticActionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiagnosticInvoker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiagnosticInvoker {}
impl ::core::clone::Clone for IDiagnosticInvoker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiagnosticInvoker2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiagnosticInvoker2 {}
impl ::core::clone::Clone for IDiagnosticInvoker2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDiagnosticInvokerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDiagnosticInvokerStatics {}
impl ::core::clone::Clone for IDiagnosticInvokerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessCpuUsage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessCpuUsage {}
impl ::core::clone::Clone for IProcessCpuUsage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessCpuUsageReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessCpuUsageReport {}
impl ::core::clone::Clone for IProcessCpuUsageReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessDiagnosticInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessDiagnosticInfo {}
impl ::core::clone::Clone for IProcessDiagnosticInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessDiagnosticInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessDiagnosticInfo2 {}
impl ::core::clone::Clone for IProcessDiagnosticInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessDiagnosticInfoStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessDiagnosticInfoStatics {}
impl ::core::clone::Clone for IProcessDiagnosticInfoStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessDiagnosticInfoStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessDiagnosticInfoStatics2 {}
impl ::core::clone::Clone for IProcessDiagnosticInfoStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessDiskUsage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessDiskUsage {}
impl ::core::clone::Clone for IProcessDiskUsage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessDiskUsageReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessDiskUsageReport {}
impl ::core::clone::Clone for IProcessDiskUsageReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessMemoryUsage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessMemoryUsage {}
impl ::core::clone::Clone for IProcessMemoryUsage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProcessMemoryUsageReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProcessMemoryUsageReport {}
impl ::core::clone::Clone for IProcessMemoryUsageReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemCpuUsage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemCpuUsage {}
impl ::core::clone::Clone for ISystemCpuUsage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemCpuUsageReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemCpuUsageReport {}
impl ::core::clone::Clone for ISystemCpuUsageReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemDiagnosticInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemDiagnosticInfo {}
impl ::core::clone::Clone for ISystemDiagnosticInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemDiagnosticInfoStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemDiagnosticInfoStatics {}
impl ::core::clone::Clone for ISystemDiagnosticInfoStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemDiagnosticInfoStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemDiagnosticInfoStatics2 {}
impl ::core::clone::Clone for ISystemDiagnosticInfoStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemMemoryUsage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemMemoryUsage {}
impl ::core::clone::Clone for ISystemMemoryUsage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemMemoryUsageReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemMemoryUsageReport {}
impl ::core::clone::Clone for ISystemMemoryUsageReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProcessCpuUsage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProcessCpuUsage {}
impl ::core::clone::Clone for ProcessCpuUsage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProcessCpuUsageReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProcessCpuUsageReport {}
impl ::core::clone::Clone for ProcessCpuUsageReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProcessDiagnosticInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProcessDiagnosticInfo {}
impl ::core::clone::Clone for ProcessDiagnosticInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProcessDiskUsage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProcessDiskUsage {}
impl ::core::clone::Clone for ProcessDiskUsage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProcessDiskUsageReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProcessDiskUsageReport {}
impl ::core::clone::Clone for ProcessDiskUsageReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProcessMemoryUsage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProcessMemoryUsage {}
impl ::core::clone::Clone for ProcessMemoryUsage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ProcessMemoryUsageReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ProcessMemoryUsageReport {}
impl ::core::clone::Clone for ProcessMemoryUsageReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemCpuUsage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemCpuUsage {}
impl ::core::clone::Clone for SystemCpuUsage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemCpuUsageReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemCpuUsageReport {}
impl ::core::clone::Clone for SystemCpuUsageReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemDiagnosticInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemDiagnosticInfo {}
impl ::core::clone::Clone for SystemDiagnosticInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemMemoryUsage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemMemoryUsage {}
impl ::core::clone::Clone for SystemMemoryUsage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemMemoryUsageReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemMemoryUsageReport {}
impl ::core::clone::Clone for SystemMemoryUsageReport {
    fn clone(&self) -> Self {
        *self
    }
}
