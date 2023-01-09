impl ::core::default::Default for PSS_ALLOCATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PSS_ALLOCATOR {
    fn eq(&self, other: &Self) -> bool {
        self.Context == other.Context && self.AllocRoutine == other.AllocRoutine && self.FreeRoutine == other.FreeRoutine
    }
}
impl ::core::cmp::Eq for PSS_ALLOCATOR {}
impl ::core::fmt::Debug for PSS_ALLOCATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSS_ALLOCATOR").field("Context", &self.Context).field("AllocRoutine", &self.AllocRoutine).field("FreeRoutine", &self.FreeRoutine).finish()
    }
}
impl ::core::default::Default for PSS_AUXILIARY_PAGES_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PSS_AUXILIARY_PAGES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.AuxPagesCaptured == other.AuxPagesCaptured
    }
}
impl ::core::cmp::Eq for PSS_AUXILIARY_PAGES_INFORMATION {}
impl ::core::fmt::Debug for PSS_AUXILIARY_PAGES_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSS_AUXILIARY_PAGES_INFORMATION").field("AuxPagesCaptured", &self.AuxPagesCaptured).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::default::Default for PSS_AUXILIARY_PAGE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::cmp::PartialEq for PSS_AUXILIARY_PAGE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Address == other.Address && self.BasicInformation == other.BasicInformation && self.CaptureTime == other.CaptureTime && self.PageContents == other.PageContents && self.PageSize == other.PageSize
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::cmp::Eq for PSS_AUXILIARY_PAGE_ENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Memory"))]
impl ::core::fmt::Debug for PSS_AUXILIARY_PAGE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSS_AUXILIARY_PAGE_ENTRY").field("Address", &self.Address).field("BasicInformation", &self.BasicInformation).field("CaptureTime", &self.CaptureTime).field("PageContents", &self.PageContents).field("PageSize", &self.PageSize).finish()
    }
}
impl ::core::default::Default for PSS_CAPTURE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PSS_CAPTURE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSS_CAPTURE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PSS_CAPTURE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PSS_CAPTURE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PSS_CAPTURE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PSS_CAPTURE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PSS_CAPTURE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PSS_DUPLICATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PSS_DUPLICATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSS_DUPLICATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PSS_DUPLICATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PSS_DUPLICATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PSS_DUPLICATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PSS_DUPLICATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PSS_DUPLICATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PSS_HANDLE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PSS_HANDLE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PSS_HANDLE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSS_HANDLE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PSS_HANDLE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PSS_HANDLE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PSS_HANDLE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PSS_HANDLE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PSS_HANDLE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PSS_HANDLE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PSS_HANDLE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.HandlesCaptured == other.HandlesCaptured
    }
}
impl ::core::cmp::Eq for PSS_HANDLE_INFORMATION {}
impl ::core::fmt::Debug for PSS_HANDLE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSS_HANDLE_INFORMATION").field("HandlesCaptured", &self.HandlesCaptured).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PSS_HANDLE_TRACE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PSS_HANDLE_TRACE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.SectionHandle == other.SectionHandle && self.Size == other.Size
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PSS_HANDLE_TRACE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PSS_HANDLE_TRACE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSS_HANDLE_TRACE_INFORMATION").field("SectionHandle", &self.SectionHandle).field("Size", &self.Size).finish()
    }
}
impl ::core::default::Default for PSS_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PSS_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSS_OBJECT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PSS_PERFORMANCE_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PSS_PERFORMANCE_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.TotalCycleCount == other.TotalCycleCount && self.TotalWallClockPeriod == other.TotalWallClockPeriod && self.VaCloneCycleCount == other.VaCloneCycleCount && self.VaCloneWallClockPeriod == other.VaCloneWallClockPeriod && self.VaSpaceCycleCount == other.VaSpaceCycleCount && self.VaSpaceWallClockPeriod == other.VaSpaceWallClockPeriod && self.AuxPagesCycleCount == other.AuxPagesCycleCount && self.AuxPagesWallClockPeriod == other.AuxPagesWallClockPeriod && self.HandlesCycleCount == other.HandlesCycleCount && self.HandlesWallClockPeriod == other.HandlesWallClockPeriod && self.ThreadsCycleCount == other.ThreadsCycleCount && self.ThreadsWallClockPeriod == other.ThreadsWallClockPeriod
    }
}
impl ::core::cmp::Eq for PSS_PERFORMANCE_COUNTERS {}
impl ::core::fmt::Debug for PSS_PERFORMANCE_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSS_PERFORMANCE_COUNTERS")
            .field("TotalCycleCount", &self.TotalCycleCount)
            .field("TotalWallClockPeriod", &self.TotalWallClockPeriod)
            .field("VaCloneCycleCount", &self.VaCloneCycleCount)
            .field("VaCloneWallClockPeriod", &self.VaCloneWallClockPeriod)
            .field("VaSpaceCycleCount", &self.VaSpaceCycleCount)
            .field("VaSpaceWallClockPeriod", &self.VaSpaceWallClockPeriod)
            .field("AuxPagesCycleCount", &self.AuxPagesCycleCount)
            .field("AuxPagesWallClockPeriod", &self.AuxPagesWallClockPeriod)
            .field("HandlesCycleCount", &self.HandlesCycleCount)
            .field("HandlesWallClockPeriod", &self.HandlesWallClockPeriod)
            .field("ThreadsCycleCount", &self.ThreadsCycleCount)
            .field("ThreadsWallClockPeriod", &self.ThreadsWallClockPeriod)
            .finish()
    }
}
impl ::core::default::Default for PSS_PROCESS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PSS_PROCESS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSS_PROCESS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PSS_PROCESS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PSS_PROCESS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PSS_PROCESS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PSS_PROCESS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PSS_PROCESS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PSS_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PSS_PROCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ExitStatus == other.ExitStatus
            && self.PebBaseAddress == other.PebBaseAddress
            && self.AffinityMask == other.AffinityMask
            && self.BasePriority == other.BasePriority
            && self.ProcessId == other.ProcessId
            && self.ParentProcessId == other.ParentProcessId
            && self.Flags == other.Flags
            && self.CreateTime == other.CreateTime
            && self.ExitTime == other.ExitTime
            && self.KernelTime == other.KernelTime
            && self.UserTime == other.UserTime
            && self.PriorityClass == other.PriorityClass
            && self.PeakVirtualSize == other.PeakVirtualSize
            && self.VirtualSize == other.VirtualSize
            && self.PageFaultCount == other.PageFaultCount
            && self.PeakWorkingSetSize == other.PeakWorkingSetSize
            && self.WorkingSetSize == other.WorkingSetSize
            && self.QuotaPeakPagedPoolUsage == other.QuotaPeakPagedPoolUsage
            && self.QuotaPagedPoolUsage == other.QuotaPagedPoolUsage
            && self.QuotaPeakNonPagedPoolUsage == other.QuotaPeakNonPagedPoolUsage
            && self.QuotaNonPagedPoolUsage == other.QuotaNonPagedPoolUsage
            && self.PagefileUsage == other.PagefileUsage
            && self.PeakPagefileUsage == other.PeakPagefileUsage
            && self.PrivateUsage == other.PrivateUsage
            && self.ExecuteFlags == other.ExecuteFlags
            && self.ImageFileName == other.ImageFileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PSS_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PSS_PROCESS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSS_PROCESS_INFORMATION")
            .field("ExitStatus", &self.ExitStatus)
            .field("PebBaseAddress", &self.PebBaseAddress)
            .field("AffinityMask", &self.AffinityMask)
            .field("BasePriority", &self.BasePriority)
            .field("ProcessId", &self.ProcessId)
            .field("ParentProcessId", &self.ParentProcessId)
            .field("Flags", &self.Flags)
            .field("CreateTime", &self.CreateTime)
            .field("ExitTime", &self.ExitTime)
            .field("KernelTime", &self.KernelTime)
            .field("UserTime", &self.UserTime)
            .field("PriorityClass", &self.PriorityClass)
            .field("PeakVirtualSize", &self.PeakVirtualSize)
            .field("VirtualSize", &self.VirtualSize)
            .field("PageFaultCount", &self.PageFaultCount)
            .field("PeakWorkingSetSize", &self.PeakWorkingSetSize)
            .field("WorkingSetSize", &self.WorkingSetSize)
            .field("QuotaPeakPagedPoolUsage", &self.QuotaPeakPagedPoolUsage)
            .field("QuotaPagedPoolUsage", &self.QuotaPagedPoolUsage)
            .field("QuotaPeakNonPagedPoolUsage", &self.QuotaPeakNonPagedPoolUsage)
            .field("QuotaNonPagedPoolUsage", &self.QuotaNonPagedPoolUsage)
            .field("PagefileUsage", &self.PagefileUsage)
            .field("PeakPagefileUsage", &self.PeakPagefileUsage)
            .field("PrivateUsage", &self.PrivateUsage)
            .field("ExecuteFlags", &self.ExecuteFlags)
            .field("ImageFileName", &self.ImageFileName)
            .finish()
    }
}
impl ::core::default::Default for PSS_QUERY_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PSS_QUERY_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSS_QUERY_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for PSS_THREAD_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for PSS_THREAD_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ExitStatus == other.ExitStatus
            && self.TebBaseAddress == other.TebBaseAddress
            && self.ProcessId == other.ProcessId
            && self.ThreadId == other.ThreadId
            && self.AffinityMask == other.AffinityMask
            && self.Priority == other.Priority
            && self.BasePriority == other.BasePriority
            && self.LastSyscallFirstArgument == other.LastSyscallFirstArgument
            && self.LastSyscallNumber == other.LastSyscallNumber
            && self.CreateTime == other.CreateTime
            && self.ExitTime == other.ExitTime
            && self.KernelTime == other.KernelTime
            && self.UserTime == other.UserTime
            && self.Win32StartAddress == other.Win32StartAddress
            && self.CaptureTime == other.CaptureTime
            && self.Flags == other.Flags
            && self.SuspendCount == other.SuspendCount
            && self.SizeOfContextRecord == other.SizeOfContextRecord
            && self.ContextRecord == other.ContextRecord
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for PSS_THREAD_ENTRY {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for PSS_THREAD_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSS_THREAD_ENTRY")
            .field("ExitStatus", &self.ExitStatus)
            .field("TebBaseAddress", &self.TebBaseAddress)
            .field("ProcessId", &self.ProcessId)
            .field("ThreadId", &self.ThreadId)
            .field("AffinityMask", &self.AffinityMask)
            .field("Priority", &self.Priority)
            .field("BasePriority", &self.BasePriority)
            .field("LastSyscallFirstArgument", &self.LastSyscallFirstArgument)
            .field("LastSyscallNumber", &self.LastSyscallNumber)
            .field("CreateTime", &self.CreateTime)
            .field("ExitTime", &self.ExitTime)
            .field("KernelTime", &self.KernelTime)
            .field("UserTime", &self.UserTime)
            .field("Win32StartAddress", &self.Win32StartAddress)
            .field("CaptureTime", &self.CaptureTime)
            .field("Flags", &self.Flags)
            .field("SuspendCount", &self.SuspendCount)
            .field("SizeOfContextRecord", &self.SizeOfContextRecord)
            .field("ContextRecord", &self.ContextRecord)
            .finish()
    }
}
impl ::core::default::Default for PSS_THREAD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PSS_THREAD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSS_THREAD_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PSS_THREAD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PSS_THREAD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PSS_THREAD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PSS_THREAD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PSS_THREAD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PSS_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PSS_THREAD_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadsCaptured == other.ThreadsCaptured && self.ContextLength == other.ContextLength
    }
}
impl ::core::cmp::Eq for PSS_THREAD_INFORMATION {}
impl ::core::fmt::Debug for PSS_THREAD_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSS_THREAD_INFORMATION").field("ThreadsCaptured", &self.ThreadsCaptured).field("ContextLength", &self.ContextLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PSS_VA_CLONE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PSS_VA_CLONE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.VaCloneHandle == other.VaCloneHandle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PSS_VA_CLONE_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PSS_VA_CLONE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSS_VA_CLONE_INFORMATION").field("VaCloneHandle", &self.VaCloneHandle).finish()
    }
}
impl ::core::default::Default for PSS_VA_SPACE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PSS_VA_SPACE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress && self.AllocationBase == other.AllocationBase && self.AllocationProtect == other.AllocationProtect && self.RegionSize == other.RegionSize && self.State == other.State && self.Protect == other.Protect && self.Type == other.Type && self.TimeDateStamp == other.TimeDateStamp && self.SizeOfImage == other.SizeOfImage && self.ImageBase == other.ImageBase && self.CheckSum == other.CheckSum && self.MappedFileNameLength == other.MappedFileNameLength && self.MappedFileName == other.MappedFileName
    }
}
impl ::core::cmp::Eq for PSS_VA_SPACE_ENTRY {}
impl ::core::fmt::Debug for PSS_VA_SPACE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSS_VA_SPACE_ENTRY")
            .field("BaseAddress", &self.BaseAddress)
            .field("AllocationBase", &self.AllocationBase)
            .field("AllocationProtect", &self.AllocationProtect)
            .field("RegionSize", &self.RegionSize)
            .field("State", &self.State)
            .field("Protect", &self.Protect)
            .field("Type", &self.Type)
            .field("TimeDateStamp", &self.TimeDateStamp)
            .field("SizeOfImage", &self.SizeOfImage)
            .field("ImageBase", &self.ImageBase)
            .field("CheckSum", &self.CheckSum)
            .field("MappedFileNameLength", &self.MappedFileNameLength)
            .field("MappedFileName", &self.MappedFileName)
            .finish()
    }
}
impl ::core::default::Default for PSS_VA_SPACE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PSS_VA_SPACE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.RegionCount == other.RegionCount
    }
}
impl ::core::cmp::Eq for PSS_VA_SPACE_INFORMATION {}
impl ::core::fmt::Debug for PSS_VA_SPACE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSS_VA_SPACE_INFORMATION").field("RegionCount", &self.RegionCount).finish()
    }
}
impl ::core::default::Default for PSS_WALK_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PSS_WALK_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSS_WALK_INFORMATION_CLASS").field(&self.0).finish()
    }
}
