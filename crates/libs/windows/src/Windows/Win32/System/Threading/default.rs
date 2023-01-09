impl ::core::default::Default for APP_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for APP_MEMORY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.AvailableCommit == other.AvailableCommit && self.PrivateCommitUsage == other.PrivateCommitUsage && self.PeakPrivateCommitUsage == other.PeakPrivateCommitUsage && self.TotalCommitUsage == other.TotalCommitUsage
    }
}
impl ::core::cmp::Eq for APP_MEMORY_INFORMATION {}
impl ::core::fmt::Debug for APP_MEMORY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APP_MEMORY_INFORMATION").field("AvailableCommit", &self.AvailableCommit).field("PrivateCommitUsage", &self.PrivateCommitUsage).field("PeakPrivateCommitUsage", &self.PeakPrivateCommitUsage).field("TotalCommitUsage", &self.TotalCommitUsage).finish()
    }
}
impl ::core::default::Default for AVRT_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AVRT_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AVRT_PRIORITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for CREATE_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREATE_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_EVENT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CREATE_EVENT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CREATE_EVENT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CREATE_EVENT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CREATE_EVENT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CREATE_EVENT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CREATE_PROCESS_LOGON_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREATE_PROCESS_LOGON_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_PROCESS_LOGON_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for GET_GUI_RESOURCES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_GUI_RESOURCES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_GUI_RESOURCES_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for IO_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IO_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.ReadOperationCount == other.ReadOperationCount && self.WriteOperationCount == other.WriteOperationCount && self.OtherOperationCount == other.OtherOperationCount && self.ReadTransferCount == other.ReadTransferCount && self.WriteTransferCount == other.WriteTransferCount && self.OtherTransferCount == other.OtherTransferCount
    }
}
impl ::core::cmp::Eq for IO_COUNTERS {}
impl ::core::fmt::Debug for IO_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IO_COUNTERS").field("ReadOperationCount", &self.ReadOperationCount).field("WriteOperationCount", &self.WriteOperationCount).field("OtherOperationCount", &self.OtherOperationCount).field("ReadTransferCount", &self.ReadTransferCount).field("WriteTransferCount", &self.WriteTransferCount).field("OtherTransferCount", &self.OtherTransferCount).finish()
    }
}
impl ::core::default::Default for MACHINE_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MACHINE_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MACHINE_ATTRIBUTES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MACHINE_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MACHINE_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for MEMORY_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MEMORY_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEMORY_PRIORITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for MEMORY_PRIORITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MEMORY_PRIORITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MemoryPriority == other.MemoryPriority
    }
}
impl ::core::cmp::Eq for MEMORY_PRIORITY_INFORMATION {}
impl ::core::fmt::Debug for MEMORY_PRIORITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_PRIORITY_INFORMATION").field("MemoryPriority", &self.MemoryPriority).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for PEB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for PEB_LDR_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::PartialEq for PEB_LDR_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.InMemoryOrderModuleList == other.InMemoryOrderModuleList
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::Eq for PEB_LDR_DATA {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::fmt::Debug for PEB_LDR_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEB_LDR_DATA").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("InMemoryOrderModuleList", &self.InMemoryOrderModuleList).finish()
    }
}
impl ::core::default::Default for POWER_REQUEST_CONTEXT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for POWER_REQUEST_CONTEXT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_REQUEST_CONTEXT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROCESSINFOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESSINFOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESSINFOCLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROCESSOR_FEATURE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESSOR_FEATURE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESSOR_FEATURE_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROCESS_ACCESS_RIGHTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_ACCESS_RIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_ACCESS_RIGHTS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROCESS_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROCESS_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_AFFINITY_AUTO_UPDATE_FLAGS").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for PROCESS_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for PROCESS_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.PebBaseAddress == other.PebBaseAddress && self.Reserved2 == other.Reserved2 && self.UniqueProcessId == other.UniqueProcessId && self.Reserved3 == other.Reserved3
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for PROCESS_BASIC_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for PROCESS_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_BASIC_INFORMATION").field("Reserved1", &self.Reserved1).field("PebBaseAddress", &self.PebBaseAddress).field("Reserved2", &self.Reserved2).field("UniqueProcessId", &self.UniqueProcessId).field("Reserved3", &self.Reserved3).finish()
    }
}
impl ::core::default::Default for PROCESS_CREATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_CREATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_CREATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROCESS_CREATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROCESS_CREATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PROCESS_DEP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_DEP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_DEP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROCESS_DEP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROCESS_DEP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn eq(&self, other: &Self) -> bool {
        self.TargetAddress == other.TargetAddress && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {}
impl ::core::fmt::Debug for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_DYNAMIC_EH_CONTINUATION_TARGET").field("TargetAddress", &self.TargetAddress).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfTargets == other.NumberOfTargets && self.Reserved == other.Reserved && self.Reserved2 == other.Reserved2 && self.Targets == other.Targets
    }
}
impl ::core::cmp::Eq for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {}
impl ::core::fmt::Debug for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION").field("NumberOfTargets", &self.NumberOfTargets).field("Reserved", &self.Reserved).field("Reserved2", &self.Reserved2).field("Targets", &self.Targets).finish()
    }
}
impl ::core::default::Default for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress && self.Size == other.Size && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {}
impl ::core::fmt::Debug for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE").field("BaseAddress", &self.BaseAddress).field("Size", &self.Size).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfRanges == other.NumberOfRanges && self.Reserved == other.Reserved && self.Reserved2 == other.Reserved2 && self.Ranges == other.Ranges
    }
}
impl ::core::cmp::Eq for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {}
impl ::core::fmt::Debug for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION").field("NumberOfRanges", &self.NumberOfRanges).field("Reserved", &self.Reserved).field("Reserved2", &self.Reserved2).field("Ranges", &self.Ranges).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.hProcess == other.hProcess && self.hThread == other.hThread && self.dwProcessId == other.dwProcessId && self.dwThreadId == other.dwThreadId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PROCESS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_INFORMATION").field("hProcess", &self.hProcess).field("hThread", &self.hThread).field("dwProcessId", &self.dwProcessId).field("dwThreadId", &self.dwThreadId).finish()
    }
}
impl ::core::default::Default for PROCESS_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROCESS_LEAP_SECOND_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESS_LEAP_SECOND_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PROCESS_LEAP_SECOND_INFO {}
impl ::core::fmt::Debug for PROCESS_LEAP_SECOND_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_LEAP_SECOND_INFO").field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::default::Default for PROCESS_MACHINE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::cmp::PartialEq for PROCESS_MACHINE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessMachine == other.ProcessMachine && self.Res0 == other.Res0 && self.MachineAttributes == other.MachineAttributes
    }
}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::cmp::Eq for PROCESS_MACHINE_INFORMATION {}
#[cfg(feature = "Win32_System_SystemInformation")]
impl ::core::fmt::Debug for PROCESS_MACHINE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_MACHINE_INFORMATION").field("ProcessMachine", &self.ProcessMachine).field("Res0", &self.Res0).field("MachineAttributes", &self.MachineAttributes).finish()
    }
}
impl ::core::default::Default for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Reserved == other.Reserved && self.Type == other.Type && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for PROCESS_MEMORY_EXHAUSTION_INFO {}
impl ::core::fmt::Debug for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_MEMORY_EXHAUSTION_INFO").field("Version", &self.Version).field("Reserved", &self.Reserved).field("Type", &self.Type).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for PROCESS_MEMORY_EXHAUSTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_MEMORY_EXHAUSTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_MEMORY_EXHAUSTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_MITIGATION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_MITIGATION_POLICY").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROCESS_NAME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_NAME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_NAME_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROCESS_POWER_THROTTLING_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESS_POWER_THROTTLING_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ControlMask == other.ControlMask && self.StateMask == other.StateMask
    }
}
impl ::core::cmp::Eq for PROCESS_POWER_THROTTLING_STATE {}
impl ::core::fmt::Debug for PROCESS_POWER_THROTTLING_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_POWER_THROTTLING_STATE").field("Version", &self.Version).field("ControlMask", &self.ControlMask).field("StateMask", &self.StateMask).finish()
    }
}
impl ::core::default::Default for PROCESS_PROTECTION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESS_PROTECTION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_PROTECTION_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ProtectionLevel == other.ProtectionLevel
    }
}
impl ::core::cmp::Eq for PROCESS_PROTECTION_LEVEL_INFORMATION {}
impl ::core::fmt::Debug for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_PROTECTION_LEVEL_INFORMATION").field("ProtectionLevel", &self.ProtectionLevel).finish()
    }
}
impl ::core::default::Default for PROC_THREAD_ATTRIBUTE_NUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROC_THREAD_ATTRIBUTE_NUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROC_THREAD_ATTRIBUTE_NUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for QUEUE_USER_APC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QUEUE_USER_APC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUEUE_USER_APC_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REASON_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RTL_BARRIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RTL_BARRIER {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.Reserved3 == other.Reserved3 && self.Reserved4 == other.Reserved4 && self.Reserved5 == other.Reserved5
    }
}
impl ::core::cmp::Eq for RTL_BARRIER {}
impl ::core::fmt::Debug for RTL_BARRIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_BARRIER").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).field("Reserved4", &self.Reserved4).field("Reserved5", &self.Reserved5).finish()
    }
}
impl ::core::default::Default for RTL_CONDITION_VARIABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RTL_CONDITION_VARIABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Ptr == other.Ptr
    }
}
impl ::core::cmp::Eq for RTL_CONDITION_VARIABLE {}
impl ::core::fmt::Debug for RTL_CONDITION_VARIABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_CONDITION_VARIABLE").field("Ptr", &self.Ptr).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for RTL_CRITICAL_SECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for RTL_CRITICAL_SECTION {
    fn eq(&self, other: &Self) -> bool {
        self.DebugInfo == other.DebugInfo && self.LockCount == other.LockCount && self.RecursionCount == other.RecursionCount && self.OwningThread == other.OwningThread && self.LockSemaphore == other.LockSemaphore && self.SpinCount == other.SpinCount
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for RTL_CRITICAL_SECTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for RTL_CRITICAL_SECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_CRITICAL_SECTION").field("DebugInfo", &self.DebugInfo).field("LockCount", &self.LockCount).field("RecursionCount", &self.RecursionCount).field("OwningThread", &self.OwningThread).field("LockSemaphore", &self.LockSemaphore).field("SpinCount", &self.SpinCount).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for RTL_CRITICAL_SECTION_DEBUG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for RTL_CRITICAL_SECTION_DEBUG {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.CreatorBackTraceIndex == other.CreatorBackTraceIndex && self.CriticalSection == other.CriticalSection && self.ProcessLocksList == other.ProcessLocksList && self.EntryCount == other.EntryCount && self.ContentionCount == other.ContentionCount && self.Flags == other.Flags && self.CreatorBackTraceIndexHigh == other.CreatorBackTraceIndexHigh && self.SpareWORD == other.SpareWORD
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for RTL_CRITICAL_SECTION_DEBUG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for RTL_CRITICAL_SECTION_DEBUG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_CRITICAL_SECTION_DEBUG").field("Type", &self.Type).field("CreatorBackTraceIndex", &self.CreatorBackTraceIndex).field("CriticalSection", &self.CriticalSection).field("ProcessLocksList", &self.ProcessLocksList).field("EntryCount", &self.EntryCount).field("ContentionCount", &self.ContentionCount).field("Flags", &self.Flags).field("CreatorBackTraceIndexHigh", &self.CreatorBackTraceIndexHigh).field("SpareWORD", &self.SpareWORD).finish()
    }
}
impl ::core::default::Default for RTL_RUN_ONCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RTL_SRWLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RTL_SRWLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.Ptr == other.Ptr
    }
}
impl ::core::cmp::Eq for RTL_SRWLOCK {}
impl ::core::fmt::Debug for RTL_SRWLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_SRWLOCK").field("Ptr", &self.Ptr).finish()
    }
}
impl ::core::default::Default for RTL_UMS_THREAD_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTL_UMS_THREAD_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTL_UMS_THREAD_INFO_CLASS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RTL_USER_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RTL_USER_PROCESS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.ImagePathName == other.ImagePathName && self.CommandLine == other.CommandLine
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RTL_USER_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RTL_USER_PROCESS_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_USER_PROCESS_PARAMETERS").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("ImagePathName", &self.ImagePathName).field("CommandLine", &self.CommandLine).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STARTUPINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STARTUPINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.lpReserved == other.lpReserved && self.lpDesktop == other.lpDesktop && self.lpTitle == other.lpTitle && self.dwX == other.dwX && self.dwY == other.dwY && self.dwXSize == other.dwXSize && self.dwYSize == other.dwYSize && self.dwXCountChars == other.dwXCountChars && self.dwYCountChars == other.dwYCountChars && self.dwFillAttribute == other.dwFillAttribute && self.dwFlags == other.dwFlags && self.wShowWindow == other.wShowWindow && self.cbReserved2 == other.cbReserved2 && self.lpReserved2 == other.lpReserved2 && self.hStdInput == other.hStdInput && self.hStdOutput == other.hStdOutput && self.hStdError == other.hStdError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STARTUPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STARTUPINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTUPINFOA")
            .field("cb", &self.cb)
            .field("lpReserved", &self.lpReserved)
            .field("lpDesktop", &self.lpDesktop)
            .field("lpTitle", &self.lpTitle)
            .field("dwX", &self.dwX)
            .field("dwY", &self.dwY)
            .field("dwXSize", &self.dwXSize)
            .field("dwYSize", &self.dwYSize)
            .field("dwXCountChars", &self.dwXCountChars)
            .field("dwYCountChars", &self.dwYCountChars)
            .field("dwFillAttribute", &self.dwFillAttribute)
            .field("dwFlags", &self.dwFlags)
            .field("wShowWindow", &self.wShowWindow)
            .field("cbReserved2", &self.cbReserved2)
            .field("lpReserved2", &self.lpReserved2)
            .field("hStdInput", &self.hStdInput)
            .field("hStdOutput", &self.hStdOutput)
            .field("hStdError", &self.hStdError)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STARTUPINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STARTUPINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.StartupInfo == other.StartupInfo && self.lpAttributeList == other.lpAttributeList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STARTUPINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STARTUPINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTUPINFOEXA").field("StartupInfo", &self.StartupInfo).field("lpAttributeList", &self.lpAttributeList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STARTUPINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STARTUPINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.StartupInfo == other.StartupInfo && self.lpAttributeList == other.lpAttributeList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STARTUPINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STARTUPINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTUPINFOEXW").field("StartupInfo", &self.StartupInfo).field("lpAttributeList", &self.lpAttributeList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STARTUPINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STARTUPINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.lpReserved == other.lpReserved && self.lpDesktop == other.lpDesktop && self.lpTitle == other.lpTitle && self.dwX == other.dwX && self.dwY == other.dwY && self.dwXSize == other.dwXSize && self.dwYSize == other.dwYSize && self.dwXCountChars == other.dwXCountChars && self.dwYCountChars == other.dwYCountChars && self.dwFillAttribute == other.dwFillAttribute && self.dwFlags == other.dwFlags && self.wShowWindow == other.wShowWindow && self.cbReserved2 == other.cbReserved2 && self.lpReserved2 == other.lpReserved2 && self.hStdInput == other.hStdInput && self.hStdOutput == other.hStdOutput && self.hStdError == other.hStdError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STARTUPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STARTUPINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTUPINFOW")
            .field("cb", &self.cb)
            .field("lpReserved", &self.lpReserved)
            .field("lpDesktop", &self.lpDesktop)
            .field("lpTitle", &self.lpTitle)
            .field("dwX", &self.dwX)
            .field("dwY", &self.dwY)
            .field("dwXSize", &self.dwXSize)
            .field("dwYSize", &self.dwYSize)
            .field("dwXCountChars", &self.dwXCountChars)
            .field("dwYCountChars", &self.dwYCountChars)
            .field("dwFillAttribute", &self.dwFillAttribute)
            .field("dwFlags", &self.dwFlags)
            .field("wShowWindow", &self.wShowWindow)
            .field("cbReserved2", &self.cbReserved2)
            .field("lpReserved2", &self.lpReserved2)
            .field("hStdInput", &self.hStdInput)
            .field("hStdOutput", &self.hStdOutput)
            .field("hStdError", &self.hStdError)
            .finish()
    }
}
impl ::core::default::Default for STARTUPINFOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STARTUPINFOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STARTUPINFOW_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for STARTUPINFOW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for STARTUPINFOW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SYNCHRONIZATION_ACCESS_RIGHTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYNCHRONIZATION_ACCESS_RIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNCHRONIZATION_ACCESS_RIGHTS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SYNCHRONIZATION_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SYNCHRONIZATION_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SYNCHRONIZATION_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SYNCHRONIZATION_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SYNCHRONIZATION_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for THREADINFOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THREADINFOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREADINFOCLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for THREAD_ACCESS_RIGHTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THREAD_ACCESS_RIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_ACCESS_RIGHTS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for THREAD_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for THREAD_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for THREAD_CREATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THREAD_CREATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_CREATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for THREAD_CREATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for THREAD_CREATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for THREAD_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THREAD_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for THREAD_POWER_THROTTLING_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for THREAD_POWER_THROTTLING_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ControlMask == other.ControlMask && self.StateMask == other.StateMask
    }
}
impl ::core::cmp::Eq for THREAD_POWER_THROTTLING_STATE {}
impl ::core::fmt::Debug for THREAD_POWER_THROTTLING_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("THREAD_POWER_THROTTLING_STATE").field("Version", &self.Version).field("ControlMask", &self.ControlMask).field("StateMask", &self.StateMask).finish()
    }
}
impl ::core::default::Default for THREAD_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for THREAD_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_PRIORITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for TP_CALLBACK_ENVIRON_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TP_CALLBACK_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TP_CALLBACK_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TP_CALLBACK_PRIORITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for TP_POOL_STACK_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TP_POOL_STACK_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.StackReserve == other.StackReserve && self.StackCommit == other.StackCommit
    }
}
impl ::core::cmp::Eq for TP_POOL_STACK_INFORMATION {}
impl ::core::fmt::Debug for TP_POOL_STACK_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TP_POOL_STACK_INFORMATION").field("StackReserve", &self.StackReserve).field("StackCommit", &self.StackCommit).finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::default::Default for UMS_SCHEDULER_STARTUP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for UMS_SYSTEM_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WORKER_THREAD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WORKER_THREAD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WORKER_THREAD_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WORKER_THREAD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WORKER_THREAD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
