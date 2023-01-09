impl ::core::default::Default for CACHE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CACHE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Level == other.Level && self.Associativity == other.Associativity && self.LineSize == other.LineSize && self.Size == other.Size && self.Type == other.Type
    }
}
impl ::core::cmp::Eq for CACHE_DESCRIPTOR {}
impl ::core::fmt::Debug for CACHE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CACHE_DESCRIPTOR").field("Level", &self.Level).field("Associativity", &self.Associativity).field("LineSize", &self.LineSize).field("Size", &self.Size).field("Type", &self.Type).finish()
    }
}
impl ::core::default::Default for CACHE_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for COMPUTER_NAME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COMPUTER_NAME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPUTER_NAME_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for CPU_SET_INFORMATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CPU_SET_INFORMATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CPU_SET_INFORMATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEP_SYSTEM_POLICY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEP_SYSTEM_POLICY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEP_SYSTEM_POLICY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEVICEFAMILYDEVICEFORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVICEFAMILYDEVICEFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICEFAMILYDEVICEFORM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEVICEFAMILYINFOENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVICEFAMILYINFOENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICEFAMILYINFOENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for FIRMWARE_TABLE_PROVIDER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FIRMWARE_TABLE_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FIRMWARE_TABLE_PROVIDER").field(&self.0).finish()
    }
}
impl ::core::default::Default for FIRMWARE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FIRMWARE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FIRMWARE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for GROUP_AFFINITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GROUP_AFFINITY {
    fn eq(&self, other: &Self) -> bool {
        self.Mask == other.Mask && self.Group == other.Group && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for GROUP_AFFINITY {}
impl ::core::fmt::Debug for GROUP_AFFINITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_AFFINITY").field("Mask", &self.Mask).field("Group", &self.Group).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for GROUP_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GROUP_RELATIONSHIP {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumGroupCount == other.MaximumGroupCount && self.ActiveGroupCount == other.ActiveGroupCount && self.Reserved == other.Reserved && self.GroupInfo == other.GroupInfo
    }
}
impl ::core::cmp::Eq for GROUP_RELATIONSHIP {}
impl ::core::fmt::Debug for GROUP_RELATIONSHIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GROUP_RELATIONSHIP").field("MaximumGroupCount", &self.MaximumGroupCount).field("ActiveGroupCount", &self.ActiveGroupCount).field("Reserved", &self.Reserved).field("GroupInfo", &self.GroupInfo).finish()
    }
}
impl ::core::default::Default for IMAGE_FILE_MACHINE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IMAGE_FILE_MACHINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMAGE_FILE_MACHINE").field(&self.0).finish()
    }
}
impl ::core::default::Default for LOGICAL_PROCESSOR_RELATIONSHIP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LOGICAL_PROCESSOR_RELATIONSHIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOGICAL_PROCESSOR_RELATIONSHIP").field(&self.0).finish()
    }
}
impl ::core::default::Default for MEMORYSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MEMORYSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.dwMemoryLoad == other.dwMemoryLoad && self.dwTotalPhys == other.dwTotalPhys && self.dwAvailPhys == other.dwAvailPhys && self.dwTotalPageFile == other.dwTotalPageFile && self.dwAvailPageFile == other.dwAvailPageFile && self.dwTotalVirtual == other.dwTotalVirtual && self.dwAvailVirtual == other.dwAvailVirtual
    }
}
impl ::core::cmp::Eq for MEMORYSTATUS {}
impl ::core::fmt::Debug for MEMORYSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORYSTATUS").field("dwLength", &self.dwLength).field("dwMemoryLoad", &self.dwMemoryLoad).field("dwTotalPhys", &self.dwTotalPhys).field("dwAvailPhys", &self.dwAvailPhys).field("dwTotalPageFile", &self.dwTotalPageFile).field("dwAvailPageFile", &self.dwAvailPageFile).field("dwTotalVirtual", &self.dwTotalVirtual).field("dwAvailVirtual", &self.dwAvailVirtual).finish()
    }
}
impl ::core::default::Default for MEMORYSTATUSEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MEMORYSTATUSEX {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.dwMemoryLoad == other.dwMemoryLoad && self.ullTotalPhys == other.ullTotalPhys && self.ullAvailPhys == other.ullAvailPhys && self.ullTotalPageFile == other.ullTotalPageFile && self.ullAvailPageFile == other.ullAvailPageFile && self.ullTotalVirtual == other.ullTotalVirtual && self.ullAvailVirtual == other.ullAvailVirtual && self.ullAvailExtendedVirtual == other.ullAvailExtendedVirtual
    }
}
impl ::core::cmp::Eq for MEMORYSTATUSEX {}
impl ::core::fmt::Debug for MEMORYSTATUSEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORYSTATUSEX").field("dwLength", &self.dwLength).field("dwMemoryLoad", &self.dwMemoryLoad).field("ullTotalPhys", &self.ullTotalPhys).field("ullAvailPhys", &self.ullAvailPhys).field("ullTotalPageFile", &self.ullTotalPageFile).field("ullAvailPageFile", &self.ullAvailPageFile).field("ullTotalVirtual", &self.ullTotalVirtual).field("ullAvailVirtual", &self.ullAvailVirtual).field("ullAvailExtendedVirtual", &self.ullAvailExtendedVirtual).finish()
    }
}
impl ::core::default::Default for NUMA_NODE_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OSVERSIONINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OSVERSIONINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize && self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion && self.dwBuildNumber == other.dwBuildNumber && self.dwPlatformId == other.dwPlatformId && self.szCSDVersion == other.szCSDVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OSVERSIONINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OSVERSIONINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OSVERSIONINFOA").field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize).field("dwMajorVersion", &self.dwMajorVersion).field("dwMinorVersion", &self.dwMinorVersion).field("dwBuildNumber", &self.dwBuildNumber).field("dwPlatformId", &self.dwPlatformId).field("szCSDVersion", &self.szCSDVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OSVERSIONINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OSVERSIONINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize && self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion && self.dwBuildNumber == other.dwBuildNumber && self.dwPlatformId == other.dwPlatformId && self.szCSDVersion == other.szCSDVersion && self.wServicePackMajor == other.wServicePackMajor && self.wServicePackMinor == other.wServicePackMinor && self.wSuiteMask == other.wSuiteMask && self.wProductType == other.wProductType && self.wReserved == other.wReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OSVERSIONINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OSVERSIONINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OSVERSIONINFOEXA")
            .field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("dwBuildNumber", &self.dwBuildNumber)
            .field("dwPlatformId", &self.dwPlatformId)
            .field("szCSDVersion", &self.szCSDVersion)
            .field("wServicePackMajor", &self.wServicePackMajor)
            .field("wServicePackMinor", &self.wServicePackMinor)
            .field("wSuiteMask", &self.wSuiteMask)
            .field("wProductType", &self.wProductType)
            .field("wReserved", &self.wReserved)
            .finish()
    }
}
impl ::core::default::Default for OSVERSIONINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OSVERSIONINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize && self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion && self.dwBuildNumber == other.dwBuildNumber && self.dwPlatformId == other.dwPlatformId && self.szCSDVersion == other.szCSDVersion && self.wServicePackMajor == other.wServicePackMajor && self.wServicePackMinor == other.wServicePackMinor && self.wSuiteMask == other.wSuiteMask && self.wProductType == other.wProductType && self.wReserved == other.wReserved
    }
}
impl ::core::cmp::Eq for OSVERSIONINFOEXW {}
impl ::core::fmt::Debug for OSVERSIONINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OSVERSIONINFOEXW")
            .field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize)
            .field("dwMajorVersion", &self.dwMajorVersion)
            .field("dwMinorVersion", &self.dwMinorVersion)
            .field("dwBuildNumber", &self.dwBuildNumber)
            .field("dwPlatformId", &self.dwPlatformId)
            .field("szCSDVersion", &self.szCSDVersion)
            .field("wServicePackMajor", &self.wServicePackMajor)
            .field("wServicePackMinor", &self.wServicePackMinor)
            .field("wSuiteMask", &self.wSuiteMask)
            .field("wProductType", &self.wProductType)
            .field("wReserved", &self.wReserved)
            .finish()
    }
}
impl ::core::default::Default for OSVERSIONINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OSVERSIONINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwOSVersionInfoSize == other.dwOSVersionInfoSize && self.dwMajorVersion == other.dwMajorVersion && self.dwMinorVersion == other.dwMinorVersion && self.dwBuildNumber == other.dwBuildNumber && self.dwPlatformId == other.dwPlatformId && self.szCSDVersion == other.szCSDVersion
    }
}
impl ::core::cmp::Eq for OSVERSIONINFOW {}
impl ::core::fmt::Debug for OSVERSIONINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OSVERSIONINFOW").field("dwOSVersionInfoSize", &self.dwOSVersionInfoSize).field("dwMajorVersion", &self.dwMajorVersion).field("dwMinorVersion", &self.dwMinorVersion).field("dwBuildNumber", &self.dwBuildNumber).field("dwPlatformId", &self.dwPlatformId).field("szCSDVersion", &self.szCSDVersion).finish()
    }
}
impl ::core::default::Default for OS_DEPLOYEMENT_STATE_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OS_DEPLOYEMENT_STATE_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OS_DEPLOYEMENT_STATE_VALUES").field(&self.0).finish()
    }
}
impl ::core::default::Default for OS_PRODUCT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OS_PRODUCT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OS_PRODUCT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROCESSOR_CACHE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROCESSOR_CACHE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESSOR_CACHE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PROCESSOR_GROUP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_GROUP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumProcessorCount == other.MaximumProcessorCount && self.ActiveProcessorCount == other.ActiveProcessorCount && self.Reserved == other.Reserved && self.ActiveProcessorMask == other.ActiveProcessorMask
    }
}
impl ::core::cmp::Eq for PROCESSOR_GROUP_INFO {}
impl ::core::fmt::Debug for PROCESSOR_GROUP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSOR_GROUP_INFO").field("MaximumProcessorCount", &self.MaximumProcessorCount).field("ActiveProcessorCount", &self.ActiveProcessorCount).field("Reserved", &self.Reserved).field("ActiveProcessorMask", &self.ActiveProcessorMask).finish()
    }
}
impl ::core::default::Default for PROCESSOR_RELATIONSHIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESSOR_RELATIONSHIP {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.EfficiencyClass == other.EfficiencyClass && self.Reserved == other.Reserved && self.GroupCount == other.GroupCount && self.GroupMask == other.GroupMask
    }
}
impl ::core::cmp::Eq for PROCESSOR_RELATIONSHIP {}
impl ::core::fmt::Debug for PROCESSOR_RELATIONSHIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSOR_RELATIONSHIP").field("Flags", &self.Flags).field("EfficiencyClass", &self.EfficiencyClass).field("Reserved", &self.Reserved).field("GroupCount", &self.GroupCount).field("GroupMask", &self.GroupMask).finish()
    }
}
impl ::core::default::Default for RTL_SYSTEM_GLOBAL_DATA_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RTL_SYSTEM_GLOBAL_DATA_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTL_SYSTEM_GLOBAL_DATA_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYSTEM_CPU_SET_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
impl ::core::default::Default for SYSTEM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYSTEM_POOL_ZEROING_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYSTEM_POOL_ZEROING_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PoolZeroingSupportPresent == other.PoolZeroingSupportPresent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYSTEM_POOL_ZEROING_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYSTEM_POOL_ZEROING_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_POOL_ZEROING_INFORMATION").field("PoolZeroingSupportPresent", &self.PoolZeroingSupportPresent).finish()
    }
}
impl ::core::default::Default for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.CycleTime == other.CycleTime
    }
}
impl ::core::cmp::Eq for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {}
impl ::core::fmt::Debug for SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION").field("CycleTime", &self.CycleTime).finish()
    }
}
impl ::core::default::Default for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {}
impl ::core::fmt::Debug for SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for USER_CET_ENVIRONMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USER_CET_ENVIRONMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_CET_ENVIRONMENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for VER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for VER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for VER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for VER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for VER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for VER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
