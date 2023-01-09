impl ::core::default::Default for JOBOBJECTINFOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JOBOBJECTINFOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOBOBJECTINFOCLASS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    fn eq(&self, other: &Self) -> bool {
        self.CompletionKey == other.CompletionKey && self.CompletionPort == other.CompletionPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_ASSOCIATE_COMPLETION_PORT").field("CompletionKey", &self.CompletionKey).field("CompletionPort", &self.CompletionPort).finish()
    }
}
impl ::core::default::Default for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.TotalUserTime == other.TotalUserTime && self.TotalKernelTime == other.TotalKernelTime && self.ThisPeriodTotalUserTime == other.ThisPeriodTotalUserTime && self.ThisPeriodTotalKernelTime == other.ThisPeriodTotalKernelTime && self.TotalPageFaultCount == other.TotalPageFaultCount && self.TotalProcesses == other.TotalProcesses && self.ActiveProcesses == other.ActiveProcesses && self.TotalTerminatedProcesses == other.TotalTerminatedProcesses
    }
}
impl ::core::cmp::Eq for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {}
impl ::core::fmt::Debug for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_BASIC_ACCOUNTING_INFORMATION")
            .field("TotalUserTime", &self.TotalUserTime)
            .field("TotalKernelTime", &self.TotalKernelTime)
            .field("ThisPeriodTotalUserTime", &self.ThisPeriodTotalUserTime)
            .field("ThisPeriodTotalKernelTime", &self.ThisPeriodTotalKernelTime)
            .field("TotalPageFaultCount", &self.TotalPageFaultCount)
            .field("TotalProcesses", &self.TotalProcesses)
            .field("ActiveProcesses", &self.ActiveProcesses)
            .field("TotalTerminatedProcesses", &self.TotalTerminatedProcesses)
            .finish()
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::default::Default for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::cmp::PartialEq for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BasicInfo == other.BasicInfo && self.IoInfo == other.IoInfo
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::cmp::Eq for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::fmt::Debug for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION").field("BasicInfo", &self.BasicInfo).field("IoInfo", &self.IoInfo).finish()
    }
}
impl ::core::default::Default for JOBOBJECT_BASIC_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_BASIC_LIMIT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PerProcessUserTimeLimit == other.PerProcessUserTimeLimit && self.PerJobUserTimeLimit == other.PerJobUserTimeLimit && self.LimitFlags == other.LimitFlags && self.MinimumWorkingSetSize == other.MinimumWorkingSetSize && self.MaximumWorkingSetSize == other.MaximumWorkingSetSize && self.ActiveProcessLimit == other.ActiveProcessLimit && self.Affinity == other.Affinity && self.PriorityClass == other.PriorityClass && self.SchedulingClass == other.SchedulingClass
    }
}
impl ::core::cmp::Eq for JOBOBJECT_BASIC_LIMIT_INFORMATION {}
impl ::core::fmt::Debug for JOBOBJECT_BASIC_LIMIT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_BASIC_LIMIT_INFORMATION")
            .field("PerProcessUserTimeLimit", &self.PerProcessUserTimeLimit)
            .field("PerJobUserTimeLimit", &self.PerJobUserTimeLimit)
            .field("LimitFlags", &self.LimitFlags)
            .field("MinimumWorkingSetSize", &self.MinimumWorkingSetSize)
            .field("MaximumWorkingSetSize", &self.MaximumWorkingSetSize)
            .field("ActiveProcessLimit", &self.ActiveProcessLimit)
            .field("Affinity", &self.Affinity)
            .field("PriorityClass", &self.PriorityClass)
            .field("SchedulingClass", &self.SchedulingClass)
            .finish()
    }
}
impl ::core::default::Default for JOBOBJECT_BASIC_PROCESS_ID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_BASIC_PROCESS_ID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfAssignedProcesses == other.NumberOfAssignedProcesses && self.NumberOfProcessIdsInList == other.NumberOfProcessIdsInList && self.ProcessIdList == other.ProcessIdList
    }
}
impl ::core::cmp::Eq for JOBOBJECT_BASIC_PROCESS_ID_LIST {}
impl ::core::fmt::Debug for JOBOBJECT_BASIC_PROCESS_ID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_BASIC_PROCESS_ID_LIST").field("NumberOfAssignedProcesses", &self.NumberOfAssignedProcesses).field("NumberOfProcessIdsInList", &self.NumberOfProcessIdsInList).field("ProcessIdList", &self.ProcessIdList).finish()
    }
}
impl ::core::default::Default for JOBOBJECT_BASIC_UI_RESTRICTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_BASIC_UI_RESTRICTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.UIRestrictionsClass == other.UIRestrictionsClass
    }
}
impl ::core::cmp::Eq for JOBOBJECT_BASIC_UI_RESTRICTIONS {}
impl ::core::fmt::Debug for JOBOBJECT_BASIC_UI_RESTRICTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_BASIC_UI_RESTRICTIONS").field("UIRestrictionsClass", &self.UIRestrictionsClass).finish()
    }
}
impl ::core::default::Default for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.EndOfJobTimeAction == other.EndOfJobTimeAction
    }
}
impl ::core::cmp::Eq for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {}
impl ::core::fmt::Debug for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_END_OF_JOB_TIME_INFORMATION").field("EndOfJobTimeAction", &self.EndOfJobTimeAction).finish()
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::default::Default for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::cmp::PartialEq for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BasicLimitInformation == other.BasicLimitInformation && self.IoInfo == other.IoInfo && self.ProcessMemoryLimit == other.ProcessMemoryLimit && self.JobMemoryLimit == other.JobMemoryLimit && self.PeakProcessMemoryUsed == other.PeakProcessMemoryUsed && self.PeakJobMemoryUsed == other.PeakJobMemoryUsed
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::cmp::Eq for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::fmt::Debug for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_EXTENDED_LIMIT_INFORMATION").field("BasicLimitInformation", &self.BasicLimitInformation).field("IoInfo", &self.IoInfo).field("ProcessMemoryLimit", &self.ProcessMemoryLimit).field("JobMemoryLimit", &self.JobMemoryLimit).field("PeakProcessMemoryUsed", &self.PeakProcessMemoryUsed).field("PeakJobMemoryUsed", &self.PeakJobMemoryUsed).finish()
    }
}
impl ::core::default::Default for JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ControlFlags == other.ControlFlags && self.ReadStats == other.ReadStats && self.WriteStats == other.WriteStats
    }
}
impl ::core::cmp::Eq for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {}
impl ::core::fmt::Debug for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_IO_ATTRIBUTION_INFORMATION").field("ControlFlags", &self.ControlFlags).field("ReadStats", &self.ReadStats).field("WriteStats", &self.WriteStats).finish()
    }
}
impl ::core::default::Default for JOBOBJECT_IO_ATTRIBUTION_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_IO_ATTRIBUTION_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.IoCount == other.IoCount && self.TotalNonOverlappedQueueTime == other.TotalNonOverlappedQueueTime && self.TotalNonOverlappedServiceTime == other.TotalNonOverlappedServiceTime && self.TotalSize == other.TotalSize
    }
}
impl ::core::cmp::Eq for JOBOBJECT_IO_ATTRIBUTION_STATS {}
impl ::core::fmt::Debug for JOBOBJECT_IO_ATTRIBUTION_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_IO_ATTRIBUTION_STATS").field("IoCount", &self.IoCount).field("TotalNonOverlappedQueueTime", &self.TotalNonOverlappedQueueTime).field("TotalNonOverlappedServiceTime", &self.TotalNonOverlappedServiceTime).field("TotalSize", &self.TotalSize).finish()
    }
}
impl ::core::default::Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MaxIops == other.MaxIops && self.MaxBandwidth == other.MaxBandwidth && self.ReservationIops == other.ReservationIops && self.VolumeName == other.VolumeName && self.BaseIoSize == other.BaseIoSize && self.ControlFlags == other.ControlFlags
    }
}
impl ::core::cmp::Eq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {}
impl ::core::fmt::Debug for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_IO_RATE_CONTROL_INFORMATION").field("MaxIops", &self.MaxIops).field("MaxBandwidth", &self.MaxBandwidth).field("ReservationIops", &self.ReservationIops).field("VolumeName", &self.VolumeName).field("BaseIoSize", &self.BaseIoSize).field("ControlFlags", &self.ControlFlags).finish()
    }
}
impl ::core::default::Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.MaxIops == other.MaxIops && self.MaxBandwidth == other.MaxBandwidth && self.ReservationIops == other.ReservationIops && self.VolumeName == other.VolumeName && self.BaseIoSize == other.BaseIoSize && self.ControlFlags == other.ControlFlags && self.VolumeNameLength == other.VolumeNameLength && self.CriticalReservationIops == other.CriticalReservationIops && self.ReservationBandwidth == other.ReservationBandwidth && self.CriticalReservationBandwidth == other.CriticalReservationBandwidth && self.MaxTimePercent == other.MaxTimePercent && self.ReservationTimePercent == other.ReservationTimePercent && self.CriticalReservationTimePercent == other.CriticalReservationTimePercent
    }
}
impl ::core::cmp::Eq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {}
impl ::core::fmt::Debug for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2")
            .field("MaxIops", &self.MaxIops)
            .field("MaxBandwidth", &self.MaxBandwidth)
            .field("ReservationIops", &self.ReservationIops)
            .field("VolumeName", &self.VolumeName)
            .field("BaseIoSize", &self.BaseIoSize)
            .field("ControlFlags", &self.ControlFlags)
            .field("VolumeNameLength", &self.VolumeNameLength)
            .field("CriticalReservationIops", &self.CriticalReservationIops)
            .field("ReservationBandwidth", &self.ReservationBandwidth)
            .field("CriticalReservationBandwidth", &self.CriticalReservationBandwidth)
            .field("MaxTimePercent", &self.MaxTimePercent)
            .field("ReservationTimePercent", &self.ReservationTimePercent)
            .field("CriticalReservationTimePercent", &self.CriticalReservationTimePercent)
            .finish()
    }
}
impl ::core::default::Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.MaxIops == other.MaxIops
            && self.MaxBandwidth == other.MaxBandwidth
            && self.ReservationIops == other.ReservationIops
            && self.VolumeName == other.VolumeName
            && self.BaseIoSize == other.BaseIoSize
            && self.ControlFlags == other.ControlFlags
            && self.VolumeNameLength == other.VolumeNameLength
            && self.CriticalReservationIops == other.CriticalReservationIops
            && self.ReservationBandwidth == other.ReservationBandwidth
            && self.CriticalReservationBandwidth == other.CriticalReservationBandwidth
            && self.MaxTimePercent == other.MaxTimePercent
            && self.ReservationTimePercent == other.ReservationTimePercent
            && self.CriticalReservationTimePercent == other.CriticalReservationTimePercent
            && self.SoftMaxIops == other.SoftMaxIops
            && self.SoftMaxBandwidth == other.SoftMaxBandwidth
            && self.SoftMaxTimePercent == other.SoftMaxTimePercent
            && self.LimitExcessNotifyIops == other.LimitExcessNotifyIops
            && self.LimitExcessNotifyBandwidth == other.LimitExcessNotifyBandwidth
            && self.LimitExcessNotifyTimePercent == other.LimitExcessNotifyTimePercent
    }
}
impl ::core::cmp::Eq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {}
impl ::core::fmt::Debug for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3")
            .field("MaxIops", &self.MaxIops)
            .field("MaxBandwidth", &self.MaxBandwidth)
            .field("ReservationIops", &self.ReservationIops)
            .field("VolumeName", &self.VolumeName)
            .field("BaseIoSize", &self.BaseIoSize)
            .field("ControlFlags", &self.ControlFlags)
            .field("VolumeNameLength", &self.VolumeNameLength)
            .field("CriticalReservationIops", &self.CriticalReservationIops)
            .field("ReservationBandwidth", &self.ReservationBandwidth)
            .field("CriticalReservationBandwidth", &self.CriticalReservationBandwidth)
            .field("MaxTimePercent", &self.MaxTimePercent)
            .field("ReservationTimePercent", &self.ReservationTimePercent)
            .field("CriticalReservationTimePercent", &self.CriticalReservationTimePercent)
            .field("SoftMaxIops", &self.SoftMaxIops)
            .field("SoftMaxBandwidth", &self.SoftMaxBandwidth)
            .field("SoftMaxTimePercent", &self.SoftMaxTimePercent)
            .field("LimitExcessNotifyIops", &self.LimitExcessNotifyIops)
            .field("LimitExcessNotifyBandwidth", &self.LimitExcessNotifyBandwidth)
            .field("LimitExcessNotifyTimePercent", &self.LimitExcessNotifyTimePercent)
            .finish()
    }
}
impl ::core::default::Default for JOBOBJECT_JOBSET_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_JOBSET_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MemberLevel == other.MemberLevel
    }
}
impl ::core::cmp::Eq for JOBOBJECT_JOBSET_INFORMATION {}
impl ::core::fmt::Debug for JOBOBJECT_JOBSET_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_JOBSET_INFORMATION").field("MemberLevel", &self.MemberLevel).finish()
    }
}
impl ::core::default::Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.LimitFlags == other.LimitFlags && self.ViolationLimitFlags == other.ViolationLimitFlags && self.IoReadBytes == other.IoReadBytes && self.IoReadBytesLimit == other.IoReadBytesLimit && self.IoWriteBytes == other.IoWriteBytes && self.IoWriteBytesLimit == other.IoWriteBytesLimit && self.PerJobUserTime == other.PerJobUserTime && self.PerJobUserTimeLimit == other.PerJobUserTimeLimit && self.JobMemory == other.JobMemory && self.JobMemoryLimit == other.JobMemoryLimit && self.RateControlTolerance == other.RateControlTolerance && self.RateControlToleranceLimit == other.RateControlToleranceLimit
    }
}
impl ::core::cmp::Eq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {}
impl ::core::fmt::Debug for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_LIMIT_VIOLATION_INFORMATION")
            .field("LimitFlags", &self.LimitFlags)
            .field("ViolationLimitFlags", &self.ViolationLimitFlags)
            .field("IoReadBytes", &self.IoReadBytes)
            .field("IoReadBytesLimit", &self.IoReadBytesLimit)
            .field("IoWriteBytes", &self.IoWriteBytes)
            .field("IoWriteBytesLimit", &self.IoWriteBytesLimit)
            .field("PerJobUserTime", &self.PerJobUserTime)
            .field("PerJobUserTimeLimit", &self.PerJobUserTimeLimit)
            .field("JobMemory", &self.JobMemory)
            .field("JobMemoryLimit", &self.JobMemoryLimit)
            .field("RateControlTolerance", &self.RateControlTolerance)
            .field("RateControlToleranceLimit", &self.RateControlToleranceLimit)
            .finish()
    }
}
impl ::core::default::Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MaxBandwidth == other.MaxBandwidth && self.ControlFlags == other.ControlFlags && self.DscpTag == other.DscpTag
    }
}
impl ::core::cmp::Eq for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {}
impl ::core::fmt::Debug for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_NET_RATE_CONTROL_INFORMATION").field("MaxBandwidth", &self.MaxBandwidth).field("ControlFlags", &self.ControlFlags).field("DscpTag", &self.DscpTag).finish()
    }
}
impl ::core::default::Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.IoReadBytesLimit == other.IoReadBytesLimit && self.IoWriteBytesLimit == other.IoWriteBytesLimit && self.PerJobUserTimeLimit == other.PerJobUserTimeLimit && self.JobMemoryLimit == other.JobMemoryLimit && self.RateControlTolerance == other.RateControlTolerance && self.RateControlToleranceInterval == other.RateControlToleranceInterval && self.LimitFlags == other.LimitFlags
    }
}
impl ::core::cmp::Eq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {}
impl ::core::fmt::Debug for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION").field("IoReadBytesLimit", &self.IoReadBytesLimit).field("IoWriteBytesLimit", &self.IoWriteBytesLimit).field("PerJobUserTimeLimit", &self.PerJobUserTimeLimit).field("JobMemoryLimit", &self.JobMemoryLimit).field("RateControlTolerance", &self.RateControlTolerance).field("RateControlToleranceInterval", &self.RateControlToleranceInterval).field("LimitFlags", &self.LimitFlags).finish()
    }
}
impl ::core::default::Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JOBOBJECT_RATE_CONTROL_TOLERANCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JOBOBJECT_RATE_CONTROL_TOLERANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOBOBJECT_RATE_CONTROL_TOLERANCE").field(&self.0).finish()
    }
}
impl ::core::default::Default for JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.SecurityLimitFlags == other.SecurityLimitFlags && self.JobToken == other.JobToken && self.SidsToDisable == other.SidsToDisable && self.PrivilegesToDelete == other.PrivilegesToDelete && self.RestrictedSids == other.RestrictedSids
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for JOBOBJECT_SECURITY_LIMIT_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_SECURITY_LIMIT_INFORMATION").field("SecurityLimitFlags", &self.SecurityLimitFlags).field("JobToken", &self.JobToken).field("SidsToDisable", &self.SidsToDisable).field("PrivilegesToDelete", &self.PrivilegesToDelete).field("RestrictedSids", &self.RestrictedSids).finish()
    }
}
impl ::core::default::Default for JOB_OBJECT_CPU_RATE_CONTROL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JOB_OBJECT_CPU_RATE_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_CPU_RATE_CONTROL").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for JOB_OBJECT_CPU_RATE_CONTROL {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_CPU_RATE_CONTROL {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_CPU_RATE_CONTROL {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_CPU_RATE_CONTROL {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_CPU_RATE_CONTROL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_IO_RATE_CONTROL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for JOB_OBJECT_LIMIT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JOB_OBJECT_LIMIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_LIMIT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for JOB_OBJECT_LIMIT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_LIMIT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_LIMIT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_LIMIT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_LIMIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_NET_RATE_CONTROL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for JOB_OBJECT_SECURITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JOB_OBJECT_SECURITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_SECURITY").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for JOB_OBJECT_SECURITY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_SECURITY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_SECURITY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_SECURITY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_SECURITY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for JOB_OBJECT_TERMINATE_AT_END_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JOB_OBJECT_TERMINATE_AT_END_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_TERMINATE_AT_END_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for JOB_OBJECT_UILIMIT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JOB_OBJECT_UILIMIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_UILIMIT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for JOB_OBJECT_UILIMIT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_UILIMIT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_UILIMIT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_UILIMIT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_UILIMIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JOB_SET_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JOB_SET_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.JobHandle == other.JobHandle && self.MemberLevel == other.MemberLevel && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JOB_SET_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JOB_SET_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOB_SET_ARRAY").field("JobHandle", &self.JobHandle).field("MemberLevel", &self.MemberLevel).field("Flags", &self.Flags).finish()
    }
}
