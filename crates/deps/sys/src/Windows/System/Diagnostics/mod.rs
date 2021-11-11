#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_Diagnostics_DevicePortal")]
pub mod DevicePortal;
#[cfg(feature = "System_Diagnostics_Telemetry")]
pub mod Telemetry;
#[cfg(feature = "System_Diagnostics_TraceReporting")]
pub mod TraceReporting;
#[link(name = "windows")]
extern "system" {
    fn DiagnosticActionResult();
    fn DiagnosticActionState();
    fn DiagnosticInvoker();
    fn IDiagnosticActionResult();
    fn IDiagnosticInvoker();
    fn IDiagnosticInvoker2();
    fn IDiagnosticInvokerStatics();
    fn IProcessCpuUsage();
    fn IProcessCpuUsageReport();
    fn IProcessDiagnosticInfo();
    fn IProcessDiagnosticInfo2();
    fn IProcessDiagnosticInfoStatics();
    fn IProcessDiagnosticInfoStatics2();
    fn IProcessDiskUsage();
    fn IProcessDiskUsageReport();
    fn IProcessMemoryUsage();
    fn IProcessMemoryUsageReport();
    fn ISystemCpuUsage();
    fn ISystemCpuUsageReport();
    fn ISystemDiagnosticInfo();
    fn ISystemDiagnosticInfoStatics();
    fn ISystemDiagnosticInfoStatics2();
    fn ISystemMemoryUsage();
    fn ISystemMemoryUsageReport();
    fn ProcessCpuUsage();
    fn ProcessCpuUsageReport();
    fn ProcessDiagnosticInfo();
    fn ProcessDiskUsage();
    fn ProcessDiskUsageReport();
    fn ProcessMemoryUsage();
    fn ProcessMemoryUsageReport();
    fn SystemCpuUsage();
    fn SystemCpuUsageReport();
    fn SystemDiagnosticInfo();
    fn SystemMemoryUsage();
    fn SystemMemoryUsageReport();
}
