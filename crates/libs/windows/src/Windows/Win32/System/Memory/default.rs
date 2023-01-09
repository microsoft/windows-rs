impl ::core::default::Default for CFG_CALL_TARGET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CFG_CALL_TARGET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for CFG_CALL_TARGET_INFO {}
impl ::core::fmt::Debug for CFG_CALL_TARGET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CFG_CALL_TARGET_INFO").field("Offset", &self.Offset).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for FILE_MAP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_MAP").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FILE_MAP {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_MAP {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_MAP {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_MAP {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_MAP {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for GLOBAL_ALLOC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GLOBAL_ALLOC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GLOBAL_ALLOC_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GLOBAL_ALLOC_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GLOBAL_ALLOC_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for HEAP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEAP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEAP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HEAP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HEAP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HEAP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HEAP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HEAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for HEAP_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEAP_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEAP_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEAP_SUMMARY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HEAP_SUMMARY {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.cbAllocated == other.cbAllocated && self.cbCommitted == other.cbCommitted && self.cbReserved == other.cbReserved && self.cbMaxReserve == other.cbMaxReserve
    }
}
impl ::core::cmp::Eq for HEAP_SUMMARY {}
impl ::core::fmt::Debug for HEAP_SUMMARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HEAP_SUMMARY").field("cb", &self.cb).field("cbAllocated", &self.cbAllocated).field("cbCommitted", &self.cbCommitted).field("cbReserved", &self.cbReserved).field("cbMaxReserve", &self.cbMaxReserve).finish()
    }
}
impl ::core::default::Default for LOCAL_ALLOC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LOCAL_ALLOC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOCAL_ALLOC_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for LOCAL_ALLOC_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for LOCAL_ALLOC_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for MEMORY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for MEMORY_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress && self.AllocationBase == other.AllocationBase && self.AllocationProtect == other.AllocationProtect && self.PartitionId == other.PartitionId && self.RegionSize == other.RegionSize && self.State == other.State && self.Protect == other.Protect && self.Type == other.Type
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for MEMORY_BASIC_INFORMATION {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for MEMORY_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_BASIC_INFORMATION").field("BaseAddress", &self.BaseAddress).field("AllocationBase", &self.AllocationBase).field("AllocationProtect", &self.AllocationProtect).field("PartitionId", &self.PartitionId).field("RegionSize", &self.RegionSize).field("State", &self.State).field("Protect", &self.Protect).field("Type", &self.Type).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for MEMORY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for MEMORY_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress && self.AllocationBase == other.AllocationBase && self.AllocationProtect == other.AllocationProtect && self.RegionSize == other.RegionSize && self.State == other.State && self.Protect == other.Protect && self.Type == other.Type
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for MEMORY_BASIC_INFORMATION {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for MEMORY_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_BASIC_INFORMATION").field("BaseAddress", &self.BaseAddress).field("AllocationBase", &self.AllocationBase).field("AllocationProtect", &self.AllocationProtect).field("RegionSize", &self.RegionSize).field("State", &self.State).field("Protect", &self.Protect).field("Type", &self.Type).finish()
    }
}
impl ::core::default::Default for MEMORY_BASIC_INFORMATION32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MEMORY_BASIC_INFORMATION32 {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress && self.AllocationBase == other.AllocationBase && self.AllocationProtect == other.AllocationProtect && self.RegionSize == other.RegionSize && self.State == other.State && self.Protect == other.Protect && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for MEMORY_BASIC_INFORMATION32 {}
impl ::core::fmt::Debug for MEMORY_BASIC_INFORMATION32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_BASIC_INFORMATION32").field("BaseAddress", &self.BaseAddress).field("AllocationBase", &self.AllocationBase).field("AllocationProtect", &self.AllocationProtect).field("RegionSize", &self.RegionSize).field("State", &self.State).field("Protect", &self.Protect).field("Type", &self.Type).finish()
    }
}
impl ::core::default::Default for MEMORY_BASIC_INFORMATION64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MEMORY_BASIC_INFORMATION64 {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress && self.AllocationBase == other.AllocationBase && self.AllocationProtect == other.AllocationProtect && self.__alignment1 == other.__alignment1 && self.RegionSize == other.RegionSize && self.State == other.State && self.Protect == other.Protect && self.Type == other.Type && self.__alignment2 == other.__alignment2
    }
}
impl ::core::cmp::Eq for MEMORY_BASIC_INFORMATION64 {}
impl ::core::fmt::Debug for MEMORY_BASIC_INFORMATION64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_BASIC_INFORMATION64").field("BaseAddress", &self.BaseAddress).field("AllocationBase", &self.AllocationBase).field("AllocationProtect", &self.AllocationProtect).field("__alignment1", &self.__alignment1).field("RegionSize", &self.RegionSize).field("State", &self.State).field("Protect", &self.Protect).field("Type", &self.Type).field("__alignment2", &self.__alignment2).finish()
    }
}
impl ::core::default::Default for MEMORY_RESOURCE_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MEMORY_RESOURCE_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEMORY_RESOURCE_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MEM_ADDRESS_REQUIREMENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MEM_ADDRESS_REQUIREMENTS {
    fn eq(&self, other: &Self) -> bool {
        self.LowestStartingAddress == other.LowestStartingAddress && self.HighestEndingAddress == other.HighestEndingAddress && self.Alignment == other.Alignment
    }
}
impl ::core::cmp::Eq for MEM_ADDRESS_REQUIREMENTS {}
impl ::core::fmt::Debug for MEM_ADDRESS_REQUIREMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEM_ADDRESS_REQUIREMENTS").field("LowestStartingAddress", &self.LowestStartingAddress).field("HighestEndingAddress", &self.HighestEndingAddress).field("Alignment", &self.Alignment).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MEM_EXTENDED_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MEM_EXTENDED_PARAMETER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MEM_EXTENDED_PARAMETER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEM_EXTENDED_PARAMETER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for OFFER_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OFFER_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFER_PRIORITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for PAGE_PROTECTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAGE_PROTECTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAGE_PROTECTION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PAGE_PROTECTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PAGE_PROTECTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PAGE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PAGE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PAGE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PAGE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PAGE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROCESS_HEAP_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for UNMAP_VIEW_OF_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UNMAP_VIEW_OF_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNMAP_VIEW_OF_FILE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for VIRTUAL_ALLOCATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VIRTUAL_ALLOCATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIRTUAL_ALLOCATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VIRTUAL_ALLOCATION_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VIRTUAL_ALLOCATION_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for VIRTUAL_FREE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VIRTUAL_FREE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIRTUAL_FREE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WIN32_MEMORY_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WIN32_MEMORY_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIN32_MEMORY_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WIN32_MEMORY_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIN32_MEMORY_PARTITION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.NumaNode == other.NumaNode && self.Channel == other.Channel && self.NumberOfNumaNodes == other.NumberOfNumaNodes && self.ResidentAvailablePages == other.ResidentAvailablePages && self.CommittedPages == other.CommittedPages && self.CommitLimit == other.CommitLimit && self.PeakCommitment == other.PeakCommitment && self.TotalNumberOfPages == other.TotalNumberOfPages && self.AvailablePages == other.AvailablePages && self.ZeroPages == other.ZeroPages && self.FreePages == other.FreePages && self.StandbyPages == other.StandbyPages && self.Reserved == other.Reserved && self.MaximumCommitLimit == other.MaximumCommitLimit && self.Reserved2 == other.Reserved2 && self.PartitionId == other.PartitionId
    }
}
impl ::core::cmp::Eq for WIN32_MEMORY_PARTITION_INFORMATION {}
impl ::core::fmt::Debug for WIN32_MEMORY_PARTITION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_MEMORY_PARTITION_INFORMATION")
            .field("Flags", &self.Flags)
            .field("NumaNode", &self.NumaNode)
            .field("Channel", &self.Channel)
            .field("NumberOfNumaNodes", &self.NumberOfNumaNodes)
            .field("ResidentAvailablePages", &self.ResidentAvailablePages)
            .field("CommittedPages", &self.CommittedPages)
            .field("CommitLimit", &self.CommitLimit)
            .field("PeakCommitment", &self.PeakCommitment)
            .field("TotalNumberOfPages", &self.TotalNumberOfPages)
            .field("AvailablePages", &self.AvailablePages)
            .field("ZeroPages", &self.ZeroPages)
            .field("FreePages", &self.FreePages)
            .field("StandbyPages", &self.StandbyPages)
            .field("Reserved", &self.Reserved)
            .field("MaximumCommitLimit", &self.MaximumCommitLimit)
            .field("Reserved2", &self.Reserved2)
            .field("PartitionId", &self.PartitionId)
            .finish()
    }
}
impl ::core::default::Default for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WIN32_MEMORY_PARTITION_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIN32_MEMORY_PARTITION_INFORMATION_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WIN32_MEMORY_RANGE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIN32_MEMORY_RANGE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualAddress == other.VirtualAddress && self.NumberOfBytes == other.NumberOfBytes
    }
}
impl ::core::cmp::Eq for WIN32_MEMORY_RANGE_ENTRY {}
impl ::core::fmt::Debug for WIN32_MEMORY_RANGE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIN32_MEMORY_RANGE_ENTRY").field("VirtualAddress", &self.VirtualAddress).field("NumberOfBytes", &self.NumberOfBytes).finish()
    }
}
impl ::core::default::Default for WIN32_MEMORY_REGION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
