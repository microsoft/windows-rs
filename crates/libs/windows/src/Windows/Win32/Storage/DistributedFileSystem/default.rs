impl ::core::default::Default for DFS_GET_PKT_ENTRY_STATE_ARG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_GET_PKT_ENTRY_STATE_ARG {
    fn eq(&self, other: &Self) -> bool {
        self.DfsEntryPathLen == other.DfsEntryPathLen && self.ServerNameLen == other.ServerNameLen && self.ShareNameLen == other.ShareNameLen && self.Level == other.Level && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for DFS_GET_PKT_ENTRY_STATE_ARG {}
impl ::core::fmt::Debug for DFS_GET_PKT_ENTRY_STATE_ARG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_GET_PKT_ENTRY_STATE_ARG").field("DfsEntryPathLen", &self.DfsEntryPathLen).field("ServerNameLen", &self.ServerNameLen).field("ShareNameLen", &self.ShareNameLen).field("Level", &self.Level).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for DFS_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.EntryPath == other.EntryPath
    }
}
impl ::core::cmp::Eq for DFS_INFO_1 {}
impl ::core::fmt::Debug for DFS_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_1").field("EntryPath", &self.EntryPath).finish()
    }
}
impl ::core::default::Default for DFS_INFO_100 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_100 {
    fn eq(&self, other: &Self) -> bool {
        self.Comment == other.Comment
    }
}
impl ::core::cmp::Eq for DFS_INFO_100 {}
impl ::core::fmt::Debug for DFS_INFO_100 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_100").field("Comment", &self.Comment).finish()
    }
}
impl ::core::default::Default for DFS_INFO_101 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_101 {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State
    }
}
impl ::core::cmp::Eq for DFS_INFO_101 {}
impl ::core::fmt::Debug for DFS_INFO_101 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_101").field("State", &self.State).finish()
    }
}
impl ::core::default::Default for DFS_INFO_102 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_102 {
    fn eq(&self, other: &Self) -> bool {
        self.Timeout == other.Timeout
    }
}
impl ::core::cmp::Eq for DFS_INFO_102 {}
impl ::core::fmt::Debug for DFS_INFO_102 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_102").field("Timeout", &self.Timeout).finish()
    }
}
impl ::core::default::Default for DFS_INFO_103 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_103 {
    fn eq(&self, other: &Self) -> bool {
        self.PropertyFlagMask == other.PropertyFlagMask && self.PropertyFlags == other.PropertyFlags
    }
}
impl ::core::cmp::Eq for DFS_INFO_103 {}
impl ::core::fmt::Debug for DFS_INFO_103 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_103").field("PropertyFlagMask", &self.PropertyFlagMask).field("PropertyFlags", &self.PropertyFlags).finish()
    }
}
impl ::core::default::Default for DFS_INFO_104 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_104 {
    fn eq(&self, other: &Self) -> bool {
        self.TargetPriority == other.TargetPriority
    }
}
impl ::core::cmp::Eq for DFS_INFO_104 {}
impl ::core::fmt::Debug for DFS_INFO_104 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_104").field("TargetPriority", &self.TargetPriority).finish()
    }
}
impl ::core::default::Default for DFS_INFO_105 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_105 {
    fn eq(&self, other: &Self) -> bool {
        self.Comment == other.Comment && self.State == other.State && self.Timeout == other.Timeout && self.PropertyFlagMask == other.PropertyFlagMask && self.PropertyFlags == other.PropertyFlags
    }
}
impl ::core::cmp::Eq for DFS_INFO_105 {}
impl ::core::fmt::Debug for DFS_INFO_105 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_105").field("Comment", &self.Comment).field("State", &self.State).field("Timeout", &self.Timeout).field("PropertyFlagMask", &self.PropertyFlagMask).field("PropertyFlags", &self.PropertyFlags).finish()
    }
}
impl ::core::default::Default for DFS_INFO_106 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_106 {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State && self.TargetPriority == other.TargetPriority
    }
}
impl ::core::cmp::Eq for DFS_INFO_106 {}
impl ::core::fmt::Debug for DFS_INFO_106 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_106").field("State", &self.State).field("TargetPriority", &self.TargetPriority).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for DFS_INFO_107 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for DFS_INFO_107 {
    fn eq(&self, other: &Self) -> bool {
        self.Comment == other.Comment && self.State == other.State && self.Timeout == other.Timeout && self.PropertyFlagMask == other.PropertyFlagMask && self.PropertyFlags == other.PropertyFlags && self.SdLengthReserved == other.SdLengthReserved && self.pSecurityDescriptor == other.pSecurityDescriptor
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for DFS_INFO_107 {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for DFS_INFO_107 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_107").field("Comment", &self.Comment).field("State", &self.State).field("Timeout", &self.Timeout).field("PropertyFlagMask", &self.PropertyFlagMask).field("PropertyFlags", &self.PropertyFlags).field("SdLengthReserved", &self.SdLengthReserved).field("pSecurityDescriptor", &self.pSecurityDescriptor).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for DFS_INFO_150 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for DFS_INFO_150 {
    fn eq(&self, other: &Self) -> bool {
        self.SdLengthReserved == other.SdLengthReserved && self.pSecurityDescriptor == other.pSecurityDescriptor
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for DFS_INFO_150 {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for DFS_INFO_150 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_150").field("SdLengthReserved", &self.SdLengthReserved).field("pSecurityDescriptor", &self.pSecurityDescriptor).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DFS_INFO_1_32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for DFS_INFO_1_32 {
    fn eq(&self, other: &Self) -> bool {
        self.EntryPath == other.EntryPath
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for DFS_INFO_1_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for DFS_INFO_1_32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_1_32").field("EntryPath", &self.EntryPath).finish()
    }
}
impl ::core::default::Default for DFS_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.EntryPath == other.EntryPath && self.Comment == other.Comment && self.State == other.State && self.NumberOfStorages == other.NumberOfStorages
    }
}
impl ::core::cmp::Eq for DFS_INFO_2 {}
impl ::core::fmt::Debug for DFS_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_2").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("NumberOfStorages", &self.NumberOfStorages).finish()
    }
}
impl ::core::default::Default for DFS_INFO_200 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_200 {
    fn eq(&self, other: &Self) -> bool {
        self.FtDfsName == other.FtDfsName
    }
}
impl ::core::cmp::Eq for DFS_INFO_200 {}
impl ::core::fmt::Debug for DFS_INFO_200 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_200").field("FtDfsName", &self.FtDfsName).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DFS_INFO_2_32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for DFS_INFO_2_32 {
    fn eq(&self, other: &Self) -> bool {
        self.EntryPath == other.EntryPath && self.Comment == other.Comment && self.State == other.State && self.NumberOfStorages == other.NumberOfStorages
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for DFS_INFO_2_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for DFS_INFO_2_32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_2_32").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("NumberOfStorages", &self.NumberOfStorages).finish()
    }
}
impl ::core::default::Default for DFS_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.EntryPath == other.EntryPath && self.Comment == other.Comment && self.State == other.State && self.NumberOfStorages == other.NumberOfStorages && self.Storage == other.Storage
    }
}
impl ::core::cmp::Eq for DFS_INFO_3 {}
impl ::core::fmt::Debug for DFS_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_3").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("NumberOfStorages", &self.NumberOfStorages).field("Storage", &self.Storage).finish()
    }
}
impl ::core::default::Default for DFS_INFO_300 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_300 {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.DfsName == other.DfsName
    }
}
impl ::core::cmp::Eq for DFS_INFO_300 {}
impl ::core::fmt::Debug for DFS_INFO_300 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_300").field("Flags", &self.Flags).field("DfsName", &self.DfsName).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DFS_INFO_3_32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for DFS_INFO_3_32 {
    fn eq(&self, other: &Self) -> bool {
        self.EntryPath == other.EntryPath && self.Comment == other.Comment && self.State == other.State && self.NumberOfStorages == other.NumberOfStorages && self.Storage == other.Storage
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for DFS_INFO_3_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for DFS_INFO_3_32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_3_32").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("NumberOfStorages", &self.NumberOfStorages).field("Storage", &self.Storage).finish()
    }
}
impl ::core::default::Default for DFS_INFO_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_4 {
    fn eq(&self, other: &Self) -> bool {
        self.EntryPath == other.EntryPath && self.Comment == other.Comment && self.State == other.State && self.Timeout == other.Timeout && self.Guid == other.Guid && self.NumberOfStorages == other.NumberOfStorages && self.Storage == other.Storage
    }
}
impl ::core::cmp::Eq for DFS_INFO_4 {}
impl ::core::fmt::Debug for DFS_INFO_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_4").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("Timeout", &self.Timeout).field("Guid", &self.Guid).field("NumberOfStorages", &self.NumberOfStorages).field("Storage", &self.Storage).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DFS_INFO_4_32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for DFS_INFO_4_32 {
    fn eq(&self, other: &Self) -> bool {
        self.EntryPath == other.EntryPath && self.Comment == other.Comment && self.State == other.State && self.Timeout == other.Timeout && self.Guid == other.Guid && self.NumberOfStorages == other.NumberOfStorages && self.Storage == other.Storage
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for DFS_INFO_4_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for DFS_INFO_4_32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_4_32").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("Timeout", &self.Timeout).field("Guid", &self.Guid).field("NumberOfStorages", &self.NumberOfStorages).field("Storage", &self.Storage).finish()
    }
}
impl ::core::default::Default for DFS_INFO_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_5 {
    fn eq(&self, other: &Self) -> bool {
        self.EntryPath == other.EntryPath && self.Comment == other.Comment && self.State == other.State && self.Timeout == other.Timeout && self.Guid == other.Guid && self.PropertyFlags == other.PropertyFlags && self.MetadataSize == other.MetadataSize && self.NumberOfStorages == other.NumberOfStorages
    }
}
impl ::core::cmp::Eq for DFS_INFO_5 {}
impl ::core::fmt::Debug for DFS_INFO_5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_5").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("Timeout", &self.Timeout).field("Guid", &self.Guid).field("PropertyFlags", &self.PropertyFlags).field("MetadataSize", &self.MetadataSize).field("NumberOfStorages", &self.NumberOfStorages).finish()
    }
}
impl ::core::default::Default for DFS_INFO_50 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_50 {
    fn eq(&self, other: &Self) -> bool {
        self.NamespaceMajorVersion == other.NamespaceMajorVersion && self.NamespaceMinorVersion == other.NamespaceMinorVersion && self.NamespaceCapabilities == other.NamespaceCapabilities
    }
}
impl ::core::cmp::Eq for DFS_INFO_50 {}
impl ::core::fmt::Debug for DFS_INFO_50 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_50").field("NamespaceMajorVersion", &self.NamespaceMajorVersion).field("NamespaceMinorVersion", &self.NamespaceMinorVersion).field("NamespaceCapabilities", &self.NamespaceCapabilities).finish()
    }
}
impl ::core::default::Default for DFS_INFO_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_6 {
    fn eq(&self, other: &Self) -> bool {
        self.EntryPath == other.EntryPath && self.Comment == other.Comment && self.State == other.State && self.Timeout == other.Timeout && self.Guid == other.Guid && self.PropertyFlags == other.PropertyFlags && self.MetadataSize == other.MetadataSize && self.NumberOfStorages == other.NumberOfStorages && self.Storage == other.Storage
    }
}
impl ::core::cmp::Eq for DFS_INFO_6 {}
impl ::core::fmt::Debug for DFS_INFO_6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_6").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("Timeout", &self.Timeout).field("Guid", &self.Guid).field("PropertyFlags", &self.PropertyFlags).field("MetadataSize", &self.MetadataSize).field("NumberOfStorages", &self.NumberOfStorages).field("Storage", &self.Storage).finish()
    }
}
impl ::core::default::Default for DFS_INFO_7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_INFO_7 {
    fn eq(&self, other: &Self) -> bool {
        self.GenerationGuid == other.GenerationGuid
    }
}
impl ::core::cmp::Eq for DFS_INFO_7 {}
impl ::core::fmt::Debug for DFS_INFO_7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_7").field("GenerationGuid", &self.GenerationGuid).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for DFS_INFO_8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for DFS_INFO_8 {
    fn eq(&self, other: &Self) -> bool {
        self.EntryPath == other.EntryPath && self.Comment == other.Comment && self.State == other.State && self.Timeout == other.Timeout && self.Guid == other.Guid && self.PropertyFlags == other.PropertyFlags && self.MetadataSize == other.MetadataSize && self.SdLengthReserved == other.SdLengthReserved && self.pSecurityDescriptor == other.pSecurityDescriptor && self.NumberOfStorages == other.NumberOfStorages
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for DFS_INFO_8 {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for DFS_INFO_8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_8").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("Timeout", &self.Timeout).field("Guid", &self.Guid).field("PropertyFlags", &self.PropertyFlags).field("MetadataSize", &self.MetadataSize).field("SdLengthReserved", &self.SdLengthReserved).field("pSecurityDescriptor", &self.pSecurityDescriptor).field("NumberOfStorages", &self.NumberOfStorages).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for DFS_INFO_9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for DFS_INFO_9 {
    fn eq(&self, other: &Self) -> bool {
        self.EntryPath == other.EntryPath && self.Comment == other.Comment && self.State == other.State && self.Timeout == other.Timeout && self.Guid == other.Guid && self.PropertyFlags == other.PropertyFlags && self.MetadataSize == other.MetadataSize && self.SdLengthReserved == other.SdLengthReserved && self.pSecurityDescriptor == other.pSecurityDescriptor && self.NumberOfStorages == other.NumberOfStorages && self.Storage == other.Storage
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for DFS_INFO_9 {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for DFS_INFO_9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_9").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("Timeout", &self.Timeout).field("Guid", &self.Guid).field("PropertyFlags", &self.PropertyFlags).field("MetadataSize", &self.MetadataSize).field("SdLengthReserved", &self.SdLengthReserved).field("pSecurityDescriptor", &self.pSecurityDescriptor).field("NumberOfStorages", &self.NumberOfStorages).field("Storage", &self.Storage).finish()
    }
}
impl ::core::default::Default for DFS_NAMESPACE_VERSION_ORIGIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DFS_NAMESPACE_VERSION_ORIGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DFS_NAMESPACE_VERSION_ORIGIN").field(&self.0).finish()
    }
}
impl ::core::default::Default for DFS_SITELIST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_SITELIST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cSites == other.cSites && self.Site == other.Site
    }
}
impl ::core::cmp::Eq for DFS_SITELIST_INFO {}
impl ::core::fmt::Debug for DFS_SITELIST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_SITELIST_INFO").field("cSites", &self.cSites).field("Site", &self.Site).finish()
    }
}
impl ::core::default::Default for DFS_SITENAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_SITENAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SiteFlags == other.SiteFlags && self.SiteName == other.SiteName
    }
}
impl ::core::cmp::Eq for DFS_SITENAME_INFO {}
impl ::core::fmt::Debug for DFS_SITENAME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_SITENAME_INFO").field("SiteFlags", &self.SiteFlags).field("SiteName", &self.SiteName).finish()
    }
}
impl ::core::default::Default for DFS_STORAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_STORAGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State && self.ServerName == other.ServerName && self.ShareName == other.ShareName
    }
}
impl ::core::cmp::Eq for DFS_STORAGE_INFO {}
impl ::core::fmt::Debug for DFS_STORAGE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_STORAGE_INFO").field("State", &self.State).field("ServerName", &self.ServerName).field("ShareName", &self.ShareName).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DFS_STORAGE_INFO_0_32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for DFS_STORAGE_INFO_0_32 {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State && self.ServerName == other.ServerName && self.ShareName == other.ShareName
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for DFS_STORAGE_INFO_0_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for DFS_STORAGE_INFO_0_32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_STORAGE_INFO_0_32").field("State", &self.State).field("ServerName", &self.ServerName).field("ShareName", &self.ShareName).finish()
    }
}
impl ::core::default::Default for DFS_STORAGE_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_STORAGE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.State == other.State && self.ServerName == other.ServerName && self.ShareName == other.ShareName && self.TargetPriority == other.TargetPriority
    }
}
impl ::core::cmp::Eq for DFS_STORAGE_INFO_1 {}
impl ::core::fmt::Debug for DFS_STORAGE_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_STORAGE_INFO_1").field("State", &self.State).field("ServerName", &self.ServerName).field("ShareName", &self.ShareName).field("TargetPriority", &self.TargetPriority).finish()
    }
}
impl ::core::default::Default for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DomainDfsMajorVersion == other.DomainDfsMajorVersion && self.DomainDfsMinorVersion == other.DomainDfsMinorVersion && self.DomainDfsCapabilities == other.DomainDfsCapabilities && self.StandaloneDfsMajorVersion == other.StandaloneDfsMajorVersion && self.StandaloneDfsMinorVersion == other.StandaloneDfsMinorVersion && self.StandaloneDfsCapabilities == other.StandaloneDfsCapabilities
    }
}
impl ::core::cmp::Eq for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {}
impl ::core::fmt::Debug for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_SUPPORTED_NAMESPACE_VERSION_INFO").field("DomainDfsMajorVersion", &self.DomainDfsMajorVersion).field("DomainDfsMinorVersion", &self.DomainDfsMinorVersion).field("DomainDfsCapabilities", &self.DomainDfsCapabilities).field("StandaloneDfsMajorVersion", &self.StandaloneDfsMajorVersion).field("StandaloneDfsMinorVersion", &self.StandaloneDfsMinorVersion).field("StandaloneDfsCapabilities", &self.StandaloneDfsCapabilities).finish()
    }
}
impl ::core::default::Default for DFS_TARGET_PRIORITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DFS_TARGET_PRIORITY {
    fn eq(&self, other: &Self) -> bool {
        self.TargetPriorityClass == other.TargetPriorityClass && self.TargetPriorityRank == other.TargetPriorityRank && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for DFS_TARGET_PRIORITY {}
impl ::core::fmt::Debug for DFS_TARGET_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_TARGET_PRIORITY").field("TargetPriorityClass", &self.TargetPriorityClass).field("TargetPriorityRank", &self.TargetPriorityRank).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for DFS_TARGET_PRIORITY_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DFS_TARGET_PRIORITY_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DFS_TARGET_PRIORITY_CLASS").field(&self.0).finish()
    }
}
