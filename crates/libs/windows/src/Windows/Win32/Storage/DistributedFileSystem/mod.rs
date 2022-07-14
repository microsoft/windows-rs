#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_ADD_VOLUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_FORCE_REMOVE: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_GET_PKT_ENTRY_STATE_ARG {
    pub DfsEntryPathLen: u16,
    pub ServerNameLen: u16,
    pub ShareNameLen: u16,
    pub Level: u32,
    pub Buffer: [u16; 1],
}
impl ::core::marker::Copy for DFS_GET_PKT_ENTRY_STATE_ARG {}
impl ::core::clone::Clone for DFS_GET_PKT_ENTRY_STATE_ARG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_GET_PKT_ENTRY_STATE_ARG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_GET_PKT_ENTRY_STATE_ARG").field("DfsEntryPathLen", &self.DfsEntryPathLen).field("ServerNameLen", &self.ServerNameLen).field("ShareNameLen", &self.ShareNameLen).field("Level", &self.Level).field("Buffer", &self.Buffer).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_GET_PKT_ENTRY_STATE_ARG {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_GET_PKT_ENTRY_STATE_ARG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_GET_PKT_ENTRY_STATE_ARG>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_GET_PKT_ENTRY_STATE_ARG {}
impl ::core::default::Default for DFS_GET_PKT_ENTRY_STATE_ARG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_1 {
    pub EntryPath: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DFS_INFO_1 {}
impl ::core::clone::Clone for DFS_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_1").field("EntryPath", &self.EntryPath).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_1 {}
impl ::core::default::Default for DFS_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_100 {
    pub Comment: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DFS_INFO_100 {}
impl ::core::clone::Clone for DFS_INFO_100 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_100 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_100").field("Comment", &self.Comment).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_100 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_100 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_100>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_100 {}
impl ::core::default::Default for DFS_INFO_100 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_101 {
    pub State: u32,
}
impl ::core::marker::Copy for DFS_INFO_101 {}
impl ::core::clone::Clone for DFS_INFO_101 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_101 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_101").field("State", &self.State).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_101 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_101 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_101>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_101 {}
impl ::core::default::Default for DFS_INFO_101 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_102 {
    pub Timeout: u32,
}
impl ::core::marker::Copy for DFS_INFO_102 {}
impl ::core::clone::Clone for DFS_INFO_102 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_102 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_102").field("Timeout", &self.Timeout).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_102 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_102 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_102>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_102 {}
impl ::core::default::Default for DFS_INFO_102 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_103 {
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
}
impl ::core::marker::Copy for DFS_INFO_103 {}
impl ::core::clone::Clone for DFS_INFO_103 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_103 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_103").field("PropertyFlagMask", &self.PropertyFlagMask).field("PropertyFlags", &self.PropertyFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_103 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_103 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_103>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_103 {}
impl ::core::default::Default for DFS_INFO_103 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_104 {
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
impl ::core::marker::Copy for DFS_INFO_104 {}
impl ::core::clone::Clone for DFS_INFO_104 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_104 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_104").field("TargetPriority", &self.TargetPriority).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_104 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_104 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_104>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_104 {}
impl ::core::default::Default for DFS_INFO_104 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_105 {
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
}
impl ::core::marker::Copy for DFS_INFO_105 {}
impl ::core::clone::Clone for DFS_INFO_105 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_105 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_105").field("Comment", &self.Comment).field("State", &self.State).field("Timeout", &self.Timeout).field("PropertyFlagMask", &self.PropertyFlagMask).field("PropertyFlags", &self.PropertyFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_105 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_105 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_105>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_105 {}
impl ::core::default::Default for DFS_INFO_105 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_106 {
    pub State: u32,
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
impl ::core::marker::Copy for DFS_INFO_106 {}
impl ::core::clone::Clone for DFS_INFO_106 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_106 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_106").field("State", &self.State).field("TargetPriority", &self.TargetPriority).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_106 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_106 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_106>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_106 {}
impl ::core::default::Default for DFS_INFO_106 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct DFS_INFO_107 {
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for DFS_INFO_107 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for DFS_INFO_107 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for DFS_INFO_107 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_107").field("Comment", &self.Comment).field("State", &self.State).field("Timeout", &self.Timeout).field("PropertyFlagMask", &self.PropertyFlagMask).field("PropertyFlags", &self.PropertyFlags).field("SdLengthReserved", &self.SdLengthReserved).field("pSecurityDescriptor", &self.pSecurityDescriptor).finish()
    }
}
#[cfg(feature = "Win32_Security")]
unsafe impl ::windows::core::Abi for DFS_INFO_107 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for DFS_INFO_107 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_107>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for DFS_INFO_107 {}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for DFS_INFO_107 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct DFS_INFO_150 {
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for DFS_INFO_150 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for DFS_INFO_150 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for DFS_INFO_150 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_150").field("SdLengthReserved", &self.SdLengthReserved).field("pSecurityDescriptor", &self.pSecurityDescriptor).finish()
    }
}
#[cfg(feature = "Win32_Security")]
unsafe impl ::windows::core::Abi for DFS_INFO_150 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for DFS_INFO_150 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_150>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for DFS_INFO_150 {}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for DFS_INFO_150 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DFS_INFO_1_32 {
    pub EntryPath: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DFS_INFO_1_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DFS_INFO_1_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for DFS_INFO_1_32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_1_32").field("EntryPath", &self.EntryPath).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for DFS_INFO_1_32 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for DFS_INFO_1_32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_1_32>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for DFS_INFO_1_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DFS_INFO_1_32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_2 {
    pub EntryPath: ::windows::core::PWSTR,
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub NumberOfStorages: u32,
}
impl ::core::marker::Copy for DFS_INFO_2 {}
impl ::core::clone::Clone for DFS_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_2").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("NumberOfStorages", &self.NumberOfStorages).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_2 {}
impl ::core::default::Default for DFS_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_200 {
    pub FtDfsName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DFS_INFO_200 {}
impl ::core::clone::Clone for DFS_INFO_200 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_200 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_200").field("FtDfsName", &self.FtDfsName).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_200 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_200 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_200>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_200 {}
impl ::core::default::Default for DFS_INFO_200 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DFS_INFO_2_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub NumberOfStorages: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DFS_INFO_2_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DFS_INFO_2_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for DFS_INFO_2_32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_2_32").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("NumberOfStorages", &self.NumberOfStorages).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for DFS_INFO_2_32 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for DFS_INFO_2_32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_2_32>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for DFS_INFO_2_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DFS_INFO_2_32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_3 {
    pub EntryPath: ::windows::core::PWSTR,
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO,
}
impl ::core::marker::Copy for DFS_INFO_3 {}
impl ::core::clone::Clone for DFS_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_3").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("NumberOfStorages", &self.NumberOfStorages).field("Storage", &self.Storage).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_3>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_3 {}
impl ::core::default::Default for DFS_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_300 {
    pub Flags: u32,
    pub DfsName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DFS_INFO_300 {}
impl ::core::clone::Clone for DFS_INFO_300 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_300 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_300").field("Flags", &self.Flags).field("DfsName", &self.DfsName).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_300 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_300 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_300>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_300 {}
impl ::core::default::Default for DFS_INFO_300 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DFS_INFO_3_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub NumberOfStorages: u32,
    pub Storage: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DFS_INFO_3_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DFS_INFO_3_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for DFS_INFO_3_32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_3_32").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("NumberOfStorages", &self.NumberOfStorages).field("Storage", &self.Storage).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for DFS_INFO_3_32 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for DFS_INFO_3_32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_3_32>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for DFS_INFO_3_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DFS_INFO_3_32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_4 {
    pub EntryPath: ::windows::core::PWSTR,
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO,
}
impl ::core::marker::Copy for DFS_INFO_4 {}
impl ::core::clone::Clone for DFS_INFO_4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_4").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("Timeout", &self.Timeout).field("Guid", &self.Guid).field("NumberOfStorages", &self.NumberOfStorages).field("Storage", &self.Storage).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_4 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_4>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_4 {}
impl ::core::default::Default for DFS_INFO_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DFS_INFO_4_32 {
    pub EntryPath: u32,
    pub Comment: u32,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub NumberOfStorages: u32,
    pub Storage: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DFS_INFO_4_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DFS_INFO_4_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for DFS_INFO_4_32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_4_32").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("Timeout", &self.Timeout).field("Guid", &self.Guid).field("NumberOfStorages", &self.NumberOfStorages).field("Storage", &self.Storage).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for DFS_INFO_4_32 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for DFS_INFO_4_32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_4_32>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for DFS_INFO_4_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DFS_INFO_4_32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_5 {
    pub EntryPath: ::windows::core::PWSTR,
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub NumberOfStorages: u32,
}
impl ::core::marker::Copy for DFS_INFO_5 {}
impl ::core::clone::Clone for DFS_INFO_5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_5").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("Timeout", &self.Timeout).field("Guid", &self.Guid).field("PropertyFlags", &self.PropertyFlags).field("MetadataSize", &self.MetadataSize).field("NumberOfStorages", &self.NumberOfStorages).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_5 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_5 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_5>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_5 {}
impl ::core::default::Default for DFS_INFO_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_50 {
    pub NamespaceMajorVersion: u32,
    pub NamespaceMinorVersion: u32,
    pub NamespaceCapabilities: u64,
}
impl ::core::marker::Copy for DFS_INFO_50 {}
impl ::core::clone::Clone for DFS_INFO_50 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_50 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_50").field("NamespaceMajorVersion", &self.NamespaceMajorVersion).field("NamespaceMinorVersion", &self.NamespaceMinorVersion).field("NamespaceCapabilities", &self.NamespaceCapabilities).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_50 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_50 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_50>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_50 {}
impl ::core::default::Default for DFS_INFO_50 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_6 {
    pub EntryPath: ::windows::core::PWSTR,
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO_1,
}
impl ::core::marker::Copy for DFS_INFO_6 {}
impl ::core::clone::Clone for DFS_INFO_6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_6").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("Timeout", &self.Timeout).field("Guid", &self.Guid).field("PropertyFlags", &self.PropertyFlags).field("MetadataSize", &self.MetadataSize).field("NumberOfStorages", &self.NumberOfStorages).field("Storage", &self.Storage).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_6 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_6>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_6 {}
impl ::core::default::Default for DFS_INFO_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_INFO_7 {
    pub GenerationGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for DFS_INFO_7 {}
impl ::core::clone::Clone for DFS_INFO_7 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_INFO_7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_7").field("GenerationGuid", &self.GenerationGuid).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_INFO_7 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_INFO_7 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_7>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_INFO_7 {}
impl ::core::default::Default for DFS_INFO_7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct DFS_INFO_8 {
    pub EntryPath: ::windows::core::PWSTR,
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub NumberOfStorages: u32,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for DFS_INFO_8 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for DFS_INFO_8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for DFS_INFO_8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_8").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("Timeout", &self.Timeout).field("Guid", &self.Guid).field("PropertyFlags", &self.PropertyFlags).field("MetadataSize", &self.MetadataSize).field("SdLengthReserved", &self.SdLengthReserved).field("pSecurityDescriptor", &self.pSecurityDescriptor).field("NumberOfStorages", &self.NumberOfStorages).finish()
    }
}
#[cfg(feature = "Win32_Security")]
unsafe impl ::windows::core::Abi for DFS_INFO_8 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for DFS_INFO_8 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_8>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for DFS_INFO_8 {}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for DFS_INFO_8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
pub struct DFS_INFO_9 {
    pub EntryPath: ::windows::core::PWSTR,
    pub Comment: ::windows::core::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: super::super::Security::PSECURITY_DESCRIPTOR,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO_1,
}
#[cfg(feature = "Win32_Security")]
impl ::core::marker::Copy for DFS_INFO_9 {}
#[cfg(feature = "Win32_Security")]
impl ::core::clone::Clone for DFS_INFO_9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for DFS_INFO_9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_INFO_9").field("EntryPath", &self.EntryPath).field("Comment", &self.Comment).field("State", &self.State).field("Timeout", &self.Timeout).field("Guid", &self.Guid).field("PropertyFlags", &self.PropertyFlags).field("MetadataSize", &self.MetadataSize).field("SdLengthReserved", &self.SdLengthReserved).field("pSecurityDescriptor", &self.pSecurityDescriptor).field("NumberOfStorages", &self.NumberOfStorages).field("Storage", &self.Storage).finish()
    }
}
#[cfg(feature = "Win32_Security")]
unsafe impl ::windows::core::Abi for DFS_INFO_9 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for DFS_INFO_9 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_9>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for DFS_INFO_9 {}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for DFS_INFO_9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_MOVE_FLAG_REPLACE_IF_EXISTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DFS_NAMESPACE_VERSION_ORIGIN(pub i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_NAMESPACE_VERSION_ORIGIN_COMBINED: DFS_NAMESPACE_VERSION_ORIGIN = DFS_NAMESPACE_VERSION_ORIGIN(0i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_NAMESPACE_VERSION_ORIGIN_SERVER: DFS_NAMESPACE_VERSION_ORIGIN = DFS_NAMESPACE_VERSION_ORIGIN(1i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_NAMESPACE_VERSION_ORIGIN_DOMAIN: DFS_NAMESPACE_VERSION_ORIGIN = DFS_NAMESPACE_VERSION_ORIGIN(2i32);
impl ::core::marker::Copy for DFS_NAMESPACE_VERSION_ORIGIN {}
impl ::core::clone::Clone for DFS_NAMESPACE_VERSION_ORIGIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DFS_NAMESPACE_VERSION_ORIGIN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DFS_NAMESPACE_VERSION_ORIGIN {
    type Abi = Self;
}
impl ::core::fmt::Debug for DFS_NAMESPACE_VERSION_ORIGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DFS_NAMESPACE_VERSION_ORIGIN").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_PROPERTY_FLAG_ABDE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_PROPERTY_FLAG_CLUSTER_ENABLED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_PROPERTY_FLAG_INSITE_REFERRALS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_PROPERTY_FLAG_ROOT_SCALABILITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_PROPERTY_FLAG_SITE_COSTING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_PROPERTY_FLAG_TARGET_FAILBACK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_RESTORE_VOLUME: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_SITELIST_INFO {
    pub cSites: u32,
    pub Site: [DFS_SITENAME_INFO; 1],
}
impl ::core::marker::Copy for DFS_SITELIST_INFO {}
impl ::core::clone::Clone for DFS_SITELIST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_SITELIST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_SITELIST_INFO").field("cSites", &self.cSites).field("Site", &self.Site).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_SITELIST_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_SITELIST_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_SITELIST_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_SITELIST_INFO {}
impl ::core::default::Default for DFS_SITELIST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_SITENAME_INFO {
    pub SiteFlags: u32,
    pub SiteName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DFS_SITENAME_INFO {}
impl ::core::clone::Clone for DFS_SITENAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_SITENAME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_SITENAME_INFO").field("SiteFlags", &self.SiteFlags).field("SiteName", &self.SiteName).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_SITENAME_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_SITENAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_SITENAME_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_SITENAME_INFO {}
impl ::core::default::Default for DFS_SITENAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_SITE_PRIMARY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_STORAGE_FLAVOR_UNUSED2: u32 = 768u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_STORAGE_INFO {
    pub State: u32,
    pub ServerName: ::windows::core::PWSTR,
    pub ShareName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DFS_STORAGE_INFO {}
impl ::core::clone::Clone for DFS_STORAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_STORAGE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_STORAGE_INFO").field("State", &self.State).field("ServerName", &self.ServerName).field("ShareName", &self.ShareName).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_STORAGE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_STORAGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_STORAGE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_STORAGE_INFO {}
impl ::core::default::Default for DFS_STORAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DFS_STORAGE_INFO_0_32 {
    pub State: u32,
    pub ServerName: u32,
    pub ShareName: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DFS_STORAGE_INFO_0_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DFS_STORAGE_INFO_0_32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for DFS_STORAGE_INFO_0_32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_STORAGE_INFO_0_32").field("State", &self.State).field("ServerName", &self.ServerName).field("ShareName", &self.ShareName).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
unsafe impl ::windows::core::Abi for DFS_STORAGE_INFO_0_32 {
    type Abi = Self;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for DFS_STORAGE_INFO_0_32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_STORAGE_INFO_0_32>()) == 0 }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for DFS_STORAGE_INFO_0_32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DFS_STORAGE_INFO_0_32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_STORAGE_INFO_1 {
    pub State: u32,
    pub ServerName: ::windows::core::PWSTR,
    pub ShareName: ::windows::core::PWSTR,
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
impl ::core::marker::Copy for DFS_STORAGE_INFO_1 {}
impl ::core::clone::Clone for DFS_STORAGE_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_STORAGE_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_STORAGE_INFO_1").field("State", &self.State).field("ServerName", &self.ServerName).field("ShareName", &self.ShareName).field("TargetPriority", &self.TargetPriority).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_STORAGE_INFO_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_STORAGE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_STORAGE_INFO_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_STORAGE_INFO_1 {}
impl ::core::default::Default for DFS_STORAGE_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_STORAGE_STATES: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_STORAGE_STATE_ACTIVE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_STORAGE_STATE_OFFLINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_STORAGE_STATE_ONLINE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    pub DomainDfsMajorVersion: u32,
    pub DomainDfsMinorVersion: u32,
    pub DomainDfsCapabilities: u64,
    pub StandaloneDfsMajorVersion: u32,
    pub StandaloneDfsMinorVersion: u32,
    pub StandaloneDfsCapabilities: u64,
}
impl ::core::marker::Copy for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {}
impl ::core::clone::Clone for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_SUPPORTED_NAMESPACE_VERSION_INFO").field("DomainDfsMajorVersion", &self.DomainDfsMajorVersion).field("DomainDfsMinorVersion", &self.DomainDfsMinorVersion).field("DomainDfsCapabilities", &self.DomainDfsCapabilities).field("StandaloneDfsMajorVersion", &self.StandaloneDfsMajorVersion).field("StandaloneDfsMinorVersion", &self.StandaloneDfsMinorVersion).field("StandaloneDfsCapabilities", &self.StandaloneDfsCapabilities).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_SUPPORTED_NAMESPACE_VERSION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {}
impl ::core::default::Default for DFS_SUPPORTED_NAMESPACE_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub struct DFS_TARGET_PRIORITY {
    pub TargetPriorityClass: DFS_TARGET_PRIORITY_CLASS,
    pub TargetPriorityRank: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for DFS_TARGET_PRIORITY {}
impl ::core::clone::Clone for DFS_TARGET_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DFS_TARGET_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DFS_TARGET_PRIORITY").field("TargetPriorityClass", &self.TargetPriorityClass).field("TargetPriorityRank", &self.TargetPriorityRank).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for DFS_TARGET_PRIORITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DFS_TARGET_PRIORITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_TARGET_PRIORITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DFS_TARGET_PRIORITY {}
impl ::core::default::Default for DFS_TARGET_PRIORITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DFS_TARGET_PRIORITY_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DfsInvalidPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(-1i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DfsSiteCostNormalPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DfsGlobalHighPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DfsSiteCostHighPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DfsSiteCostLowPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(3i32);
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DfsGlobalLowPriorityClass: DFS_TARGET_PRIORITY_CLASS = DFS_TARGET_PRIORITY_CLASS(4i32);
impl ::core::marker::Copy for DFS_TARGET_PRIORITY_CLASS {}
impl ::core::clone::Clone for DFS_TARGET_PRIORITY_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DFS_TARGET_PRIORITY_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DFS_TARGET_PRIORITY_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DFS_TARGET_PRIORITY_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DFS_TARGET_PRIORITY_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_FLAVORS: u32 = 768u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_FLAVOR_AD_BLOB: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_FLAVOR_STANDALONE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_FLAVOR_UNUSED1: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_STATES: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_STATE_FORCE_SYNC: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_STATE_INCONSISTENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_STATE_OFFLINE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_STATE_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_STATE_ONLINE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_STATE_RESYNCHRONIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const DFS_VOLUME_STATE_STANDBY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const FSCTL_DFS_BASE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const FSCTL_DFS_GET_PKT_ENTRY_STATE: u32 = 401340u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const NET_DFS_SETDC_FLAGS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const NET_DFS_SETDC_INITPKT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
pub const NET_DFS_SETDC_TIMEOUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsAdd<'a, P0, P1, P2, P3>(dfsentrypath: P0, servername: P1, sharename: P2, comment: P3, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsAdd(dfsentrypath: ::windows::core::PCWSTR, servername: ::windows::core::PCWSTR, sharename: ::windows::core::PCWSTR, comment: ::windows::core::PCWSTR, flags: u32) -> u32;
    }
    NetDfsAdd(dfsentrypath.into(), servername.into(), sharename.into(), comment.into(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsAddFtRoot<'a, P0, P1, P2, P3>(servername: P0, rootshare: P1, ftdfsname: P2, comment: P3, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsAddFtRoot(servername: ::windows::core::PCWSTR, rootshare: ::windows::core::PCWSTR, ftdfsname: ::windows::core::PCWSTR, comment: ::windows::core::PCWSTR, flags: u32) -> u32;
    }
    NetDfsAddFtRoot(servername.into(), rootshare.into(), ftdfsname.into(), comment.into(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsAddRootTarget<'a, P0, P1, P2>(pdfspath: P0, ptargetpath: P1, majorversion: u32, pcomment: P2, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsAddRootTarget(pdfspath: ::windows::core::PCWSTR, ptargetpath: ::windows::core::PCWSTR, majorversion: u32, pcomment: ::windows::core::PCWSTR, flags: u32) -> u32;
    }
    NetDfsAddRootTarget(pdfspath.into(), ptargetpath.into(), majorversion, pcomment.into(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsAddStdRoot<'a, P0, P1, P2>(servername: P0, rootshare: P1, comment: P2, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsAddStdRoot(servername: ::windows::core::PCWSTR, rootshare: ::windows::core::PCWSTR, comment: ::windows::core::PCWSTR, flags: u32) -> u32;
    }
    NetDfsAddStdRoot(servername.into(), rootshare.into(), comment.into(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsEnum<'a, P0>(dfsname: P0, level: u32, prefmaxlen: u32, buffer: *mut *mut u8, entriesread: *mut u32, resumehandle: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsEnum(dfsname: ::windows::core::PCWSTR, level: u32, prefmaxlen: u32, buffer: *mut *mut u8, entriesread: *mut u32, resumehandle: *mut u32) -> u32;
    }
    NetDfsEnum(dfsname.into(), level, prefmaxlen, ::core::mem::transmute(buffer), ::core::mem::transmute(entriesread), ::core::mem::transmute(resumehandle))
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsGetClientInfo<'a, P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2, level: u32, buffer: *mut *mut u8) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsGetClientInfo(dfsentrypath: ::windows::core::PCWSTR, servername: ::windows::core::PCWSTR, sharename: ::windows::core::PCWSTR, level: u32, buffer: *mut *mut u8) -> u32;
    }
    NetDfsGetClientInfo(dfsentrypath.into(), servername.into(), sharename.into(), level, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsGetFtContainerSecurity<'a, P0>(domainname: P0, securityinformation: u32, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsGetFtContainerSecurity(domainname: ::windows::core::PCWSTR, securityinformation: u32, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32;
    }
    NetDfsGetFtContainerSecurity(domainname.into(), securityinformation, ::core::mem::transmute(ppsecuritydescriptor), ::core::mem::transmute(lpcbsecuritydescriptor))
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsGetInfo<'a, P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2, level: u32, buffer: *mut *mut u8) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsGetInfo(dfsentrypath: ::windows::core::PCWSTR, servername: ::windows::core::PCWSTR, sharename: ::windows::core::PCWSTR, level: u32, buffer: *mut *mut u8) -> u32;
    }
    NetDfsGetInfo(dfsentrypath.into(), servername.into(), sharename.into(), level, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsGetSecurity<'a, P0>(dfsentrypath: P0, securityinformation: u32, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsGetSecurity(dfsentrypath: ::windows::core::PCWSTR, securityinformation: u32, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32;
    }
    NetDfsGetSecurity(dfsentrypath.into(), securityinformation, ::core::mem::transmute(ppsecuritydescriptor), ::core::mem::transmute(lpcbsecuritydescriptor))
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsGetStdContainerSecurity<'a, P0>(machinename: P0, securityinformation: u32, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsGetStdContainerSecurity(machinename: ::windows::core::PCWSTR, securityinformation: u32, ppsecuritydescriptor: *mut super::super::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32;
    }
    NetDfsGetStdContainerSecurity(machinename.into(), securityinformation, ::core::mem::transmute(ppsecuritydescriptor), ::core::mem::transmute(lpcbsecuritydescriptor))
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsGetSupportedNamespaceVersion<'a, P0>(origin: DFS_NAMESPACE_VERSION_ORIGIN, pname: P0, ppversioninfo: *mut *mut DFS_SUPPORTED_NAMESPACE_VERSION_INFO) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsGetSupportedNamespaceVersion(origin: DFS_NAMESPACE_VERSION_ORIGIN, pname: ::windows::core::PCWSTR, ppversioninfo: *mut *mut DFS_SUPPORTED_NAMESPACE_VERSION_INFO) -> u32;
    }
    NetDfsGetSupportedNamespaceVersion(origin, pname.into(), ::core::mem::transmute(ppversioninfo))
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsMove<'a, P0, P1>(olddfsentrypath: P0, newdfsentrypath: P1, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsMove(olddfsentrypath: ::windows::core::PCWSTR, newdfsentrypath: ::windows::core::PCWSTR, flags: u32) -> u32;
    }
    NetDfsMove(olddfsentrypath.into(), newdfsentrypath.into(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsRemove<'a, P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsRemove(dfsentrypath: ::windows::core::PCWSTR, servername: ::windows::core::PCWSTR, sharename: ::windows::core::PCWSTR) -> u32;
    }
    NetDfsRemove(dfsentrypath.into(), servername.into(), sharename.into())
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsRemoveFtRoot<'a, P0, P1, P2>(servername: P0, rootshare: P1, ftdfsname: P2, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsRemoveFtRoot(servername: ::windows::core::PCWSTR, rootshare: ::windows::core::PCWSTR, ftdfsname: ::windows::core::PCWSTR, flags: u32) -> u32;
    }
    NetDfsRemoveFtRoot(servername.into(), rootshare.into(), ftdfsname.into(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsRemoveFtRootForced<'a, P0, P1, P2, P3>(domainname: P0, servername: P1, rootshare: P2, ftdfsname: P3, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsRemoveFtRootForced(domainname: ::windows::core::PCWSTR, servername: ::windows::core::PCWSTR, rootshare: ::windows::core::PCWSTR, ftdfsname: ::windows::core::PCWSTR, flags: u32) -> u32;
    }
    NetDfsRemoveFtRootForced(domainname.into(), servername.into(), rootshare.into(), ftdfsname.into(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsRemoveRootTarget<'a, P0, P1>(pdfspath: P0, ptargetpath: P1, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsRemoveRootTarget(pdfspath: ::windows::core::PCWSTR, ptargetpath: ::windows::core::PCWSTR, flags: u32) -> u32;
    }
    NetDfsRemoveRootTarget(pdfspath.into(), ptargetpath.into(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsRemoveStdRoot<'a, P0, P1>(servername: P0, rootshare: P1, flags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsRemoveStdRoot(servername: ::windows::core::PCWSTR, rootshare: ::windows::core::PCWSTR, flags: u32) -> u32;
    }
    NetDfsRemoveStdRoot(servername.into(), rootshare.into(), flags)
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsSetClientInfo<'a, P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2, level: u32, buffer: *const u8) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsSetClientInfo(dfsentrypath: ::windows::core::PCWSTR, servername: ::windows::core::PCWSTR, sharename: ::windows::core::PCWSTR, level: u32, buffer: *const u8) -> u32;
    }
    NetDfsSetClientInfo(dfsentrypath.into(), servername.into(), sharename.into(), level, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsSetFtContainerSecurity<'a, P0, P1>(domainname: P0, securityinformation: u32, psecuritydescriptor: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsSetFtContainerSecurity(domainname: ::windows::core::PCWSTR, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> u32;
    }
    NetDfsSetFtContainerSecurity(domainname.into(), securityinformation, psecuritydescriptor.into())
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`*"]
#[inline]
pub unsafe fn NetDfsSetInfo<'a, P0, P1, P2>(dfsentrypath: P0, servername: P1, sharename: P2, level: u32, buffer: *const u8) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsSetInfo(dfsentrypath: ::windows::core::PCWSTR, servername: ::windows::core::PCWSTR, sharename: ::windows::core::PCWSTR, level: u32, buffer: *const u8) -> u32;
    }
    NetDfsSetInfo(dfsentrypath.into(), servername.into(), sharename.into(), level, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsSetSecurity<'a, P0, P1>(dfsentrypath: P0, securityinformation: u32, psecuritydescriptor: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsSetSecurity(dfsentrypath: ::windows::core::PCWSTR, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> u32;
    }
    NetDfsSetSecurity(dfsentrypath.into(), securityinformation, psecuritydescriptor.into())
}
#[doc = "*Required features: `\"Win32_Storage_DistributedFileSystem\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NetDfsSetStdContainerSecurity<'a, P0, P1>(machinename: P0, securityinformation: u32, psecuritydescriptor: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NetDfsSetStdContainerSecurity(machinename: ::windows::core::PCWSTR, securityinformation: u32, psecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> u32;
    }
    NetDfsSetStdContainerSecurity(machinename.into(), securityinformation, psecuritydescriptor.into())
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
