#[cfg(feature = "implement_exclusive")]
pub trait IDiagnosticActionResultImpl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Results(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiagnosticInvokerImpl: Sized {
    fn RunDiagnosticActionAsync(&self, context: &::core::option::Option<super::super::Data::Json::JsonObject>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiagnosticInvoker2Impl: Sized {
    fn RunDiagnosticActionFromStringAsync(&self, context: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DiagnosticActionResult, DiagnosticActionState>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDiagnosticInvokerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<DiagnosticInvoker>;
    fn GetForUser(&self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<DiagnosticInvoker>;
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessCpuUsageImpl: Sized {
    fn GetReport(&self) -> ::windows::core::Result<ProcessCpuUsageReport>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessCpuUsageReportImpl: Sized {
    fn KernelTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessDiagnosticInfoImpl: Sized {
    fn ProcessId(&self) -> ::windows::core::Result<u32>;
    fn ExecutableFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Parent(&self) -> ::windows::core::Result<ProcessDiagnosticInfo>;
    fn ProcessStartTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn DiskUsage(&self) -> ::windows::core::Result<ProcessDiskUsage>;
    fn MemoryUsage(&self) -> ::windows::core::Result<ProcessMemoryUsage>;
    fn CpuUsage(&self) -> ::windows::core::Result<ProcessCpuUsage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessDiagnosticInfo2Impl: Sized {
    fn GetAppDiagnosticInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::AppDiagnosticInfo>>;
    fn IsPackaged(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessDiagnosticInfoStaticsImpl: Sized {
    fn GetForProcesses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ProcessDiagnosticInfo>>;
    fn GetForCurrentProcess(&self) -> ::windows::core::Result<ProcessDiagnosticInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessDiagnosticInfoStatics2Impl: Sized {
    fn TryGetForProcessId(&self, processid: u32) -> ::windows::core::Result<ProcessDiagnosticInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessDiskUsageImpl: Sized {
    fn GetReport(&self) -> ::windows::core::Result<ProcessDiskUsageReport>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessDiskUsageReportImpl: Sized {
    fn ReadOperationCount(&self) -> ::windows::core::Result<i64>;
    fn WriteOperationCount(&self) -> ::windows::core::Result<i64>;
    fn OtherOperationCount(&self) -> ::windows::core::Result<i64>;
    fn BytesReadCount(&self) -> ::windows::core::Result<i64>;
    fn BytesWrittenCount(&self) -> ::windows::core::Result<i64>;
    fn OtherBytesCount(&self) -> ::windows::core::Result<i64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessMemoryUsageImpl: Sized {
    fn GetReport(&self) -> ::windows::core::Result<ProcessMemoryUsageReport>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessMemoryUsageReportImpl: Sized {
    fn NonPagedPoolSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn PageFaultCount(&self) -> ::windows::core::Result<u32>;
    fn PageFileSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn PagedPoolSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn PeakNonPagedPoolSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn PeakPageFileSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn PeakPagedPoolSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn PeakVirtualMemorySizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn PeakWorkingSetSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn PrivatePageCount(&self) -> ::windows::core::Result<u64>;
    fn VirtualMemorySizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn WorkingSetSizeInBytes(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemCpuUsageImpl: Sized {
    fn GetReport(&self) -> ::windows::core::Result<SystemCpuUsageReport>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemCpuUsageReportImpl: Sized {
    fn KernelTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn UserTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn IdleTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemDiagnosticInfoImpl: Sized {
    fn MemoryUsage(&self) -> ::windows::core::Result<SystemMemoryUsage>;
    fn CpuUsage(&self) -> ::windows::core::Result<SystemCpuUsage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemDiagnosticInfoStaticsImpl: Sized {
    fn GetForCurrentSystem(&self) -> ::windows::core::Result<SystemDiagnosticInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemDiagnosticInfoStatics2Impl: Sized {
    fn IsArchitectureSupported(&self, r#type: super::ProcessorArchitecture) -> ::windows::core::Result<bool>;
    fn PreferredArchitecture(&self) -> ::windows::core::Result<super::ProcessorArchitecture>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMemoryUsageImpl: Sized {
    fn GetReport(&self) -> ::windows::core::Result<SystemMemoryUsageReport>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemMemoryUsageReportImpl: Sized {
    fn TotalPhysicalSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn AvailableSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn CommittedSizeInBytes(&self) -> ::windows::core::Result<u64>;
}
