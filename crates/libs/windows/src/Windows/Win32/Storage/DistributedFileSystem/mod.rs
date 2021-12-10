#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_ADD_VOLUME: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_FORCE_REMOVE: u32 = 2147483648u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_1 {
    pub EntryPath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DFS_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DFS_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DFS_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DFS_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_100 {
    pub Comment: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_100 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_100 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DFS_INFO_100 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DFS_INFO_100 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_100>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DFS_INFO_100 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DFS_INFO_100 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub struct DFS_INFO_101 {
    pub State: u32,
}
impl ::core::marker::Copy for DFS_INFO_101 {}
impl ::core::clone::Clone for DFS_INFO_101 {
    fn clone(&self) -> Self {
        *self
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
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub struct DFS_INFO_102 {
    pub Timeout: u32,
}
impl ::core::marker::Copy for DFS_INFO_102 {}
impl ::core::clone::Clone for DFS_INFO_102 {
    fn clone(&self) -> Self {
        *self
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
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub struct DFS_INFO_104 {
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
impl ::core::marker::Copy for DFS_INFO_104 {}
impl ::core::clone::Clone for DFS_INFO_104 {
    fn clone(&self) -> Self {
        *self
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
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_105 {
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_105 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_105 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DFS_INFO_105 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DFS_INFO_105 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_105>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DFS_INFO_105 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DFS_INFO_105 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct DFS_INFO_107 {
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub PropertyFlagMask: u32,
    pub PropertyFlags: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for DFS_INFO_107 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for DFS_INFO_107 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for DFS_INFO_107 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for DFS_INFO_107 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_107>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for DFS_INFO_107 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for DFS_INFO_107 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct DFS_INFO_150 {
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for DFS_INFO_150 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for DFS_INFO_150 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for DFS_INFO_150 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for DFS_INFO_150 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_150>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for DFS_INFO_150 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for DFS_INFO_150 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_2 {
    pub EntryPath: super::super::Foundation::PWSTR,
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub NumberOfStorages: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DFS_INFO_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DFS_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DFS_INFO_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DFS_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_200 {
    pub FtDfsName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_200 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_200 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DFS_INFO_200 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DFS_INFO_200 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_200>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DFS_INFO_200 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DFS_INFO_200 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_3 {
    pub EntryPath: super::super::Foundation::PWSTR,
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DFS_INFO_3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DFS_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DFS_INFO_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DFS_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_300 {
    pub Flags: u32,
    pub DfsName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_300 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_300 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DFS_INFO_300 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DFS_INFO_300 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_300>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DFS_INFO_300 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DFS_INFO_300 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_4 {
    pub EntryPath: super::super::Foundation::PWSTR,
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DFS_INFO_4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DFS_INFO_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DFS_INFO_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DFS_INFO_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_5 {
    pub EntryPath: super::super::Foundation::PWSTR,
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub NumberOfStorages: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DFS_INFO_5 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DFS_INFO_5 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_5>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DFS_INFO_5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DFS_INFO_5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_INFO_6 {
    pub EntryPath: super::super::Foundation::PWSTR,
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_INFO_6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_INFO_6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DFS_INFO_6 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DFS_INFO_6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_6>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DFS_INFO_6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DFS_INFO_6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub struct DFS_INFO_7 {
    pub GenerationGuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for DFS_INFO_7 {}
impl ::core::clone::Clone for DFS_INFO_7 {
    fn clone(&self) -> Self {
        *self
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
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct DFS_INFO_8 {
    pub EntryPath: super::super::Foundation::PWSTR,
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
    pub NumberOfStorages: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for DFS_INFO_8 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for DFS_INFO_8 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for DFS_INFO_8 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for DFS_INFO_8 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_8>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for DFS_INFO_8 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for DFS_INFO_8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct DFS_INFO_9 {
    pub EntryPath: super::super::Foundation::PWSTR,
    pub Comment: super::super::Foundation::PWSTR,
    pub State: u32,
    pub Timeout: u32,
    pub Guid: ::windows::core::GUID,
    pub PropertyFlags: u32,
    pub MetadataSize: u32,
    pub SdLengthReserved: u32,
    pub pSecurityDescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
    pub NumberOfStorages: u32,
    pub Storage: *mut DFS_STORAGE_INFO_1,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for DFS_INFO_9 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for DFS_INFO_9 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for DFS_INFO_9 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for DFS_INFO_9 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_INFO_9>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for DFS_INFO_9 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for DFS_INFO_9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_MOVE_FLAG_REPLACE_IF_EXISTS: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub type DFS_NAMESPACE_VERSION_ORIGIN = i32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_NAMESPACE_VERSION_ORIGIN_COMBINED: DFS_NAMESPACE_VERSION_ORIGIN = 0i32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_NAMESPACE_VERSION_ORIGIN_SERVER: DFS_NAMESPACE_VERSION_ORIGIN = 1i32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_NAMESPACE_VERSION_ORIGIN_DOMAIN: DFS_NAMESPACE_VERSION_ORIGIN = 2i32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_PROPERTY_FLAG_ABDE: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_PROPERTY_FLAG_CLUSTER_ENABLED: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_PROPERTY_FLAG_INSITE_REFERRALS: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_PROPERTY_FLAG_ROOT_SCALABILITY: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_PROPERTY_FLAG_SITE_COSTING: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_PROPERTY_FLAG_TARGET_FAILBACK: u32 = 8u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_RESTORE_VOLUME: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_SITELIST_INFO {
    pub cSites: u32,
    pub Site: [DFS_SITENAME_INFO; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_SITELIST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_SITELIST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DFS_SITELIST_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DFS_SITELIST_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_SITELIST_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DFS_SITELIST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DFS_SITELIST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_SITENAME_INFO {
    pub SiteFlags: u32,
    pub SiteName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_SITENAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_SITENAME_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DFS_SITENAME_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DFS_SITENAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_SITENAME_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DFS_SITENAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DFS_SITENAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_SITE_PRIMARY: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_STORAGE_FLAVOR_UNUSED2: u32 = 768u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_STORAGE_INFO {
    pub State: u32,
    pub ServerName: super::super::Foundation::PWSTR,
    pub ShareName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_STORAGE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_STORAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DFS_STORAGE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DFS_STORAGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_STORAGE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DFS_STORAGE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DFS_STORAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DFS_STORAGE_INFO_1 {
    pub State: u32,
    pub ServerName: super::super::Foundation::PWSTR,
    pub ShareName: super::super::Foundation::PWSTR,
    pub TargetPriority: DFS_TARGET_PRIORITY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DFS_STORAGE_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DFS_STORAGE_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DFS_STORAGE_INFO_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DFS_STORAGE_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DFS_STORAGE_INFO_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DFS_STORAGE_INFO_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DFS_STORAGE_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_STORAGE_STATES: u32 = 15u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_STORAGE_STATE_ACTIVE: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_STORAGE_STATE_OFFLINE: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_STORAGE_STATE_ONLINE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
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
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub type DFS_TARGET_PRIORITY_CLASS = i32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DfsInvalidPriorityClass: DFS_TARGET_PRIORITY_CLASS = -1i32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DfsSiteCostNormalPriorityClass: DFS_TARGET_PRIORITY_CLASS = 0i32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DfsGlobalHighPriorityClass: DFS_TARGET_PRIORITY_CLASS = 1i32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DfsSiteCostHighPriorityClass: DFS_TARGET_PRIORITY_CLASS = 2i32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DfsSiteCostLowPriorityClass: DFS_TARGET_PRIORITY_CLASS = 3i32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DfsGlobalLowPriorityClass: DFS_TARGET_PRIORITY_CLASS = 4i32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_VOLUME_FLAVORS: u32 = 768u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_VOLUME_FLAVOR_AD_BLOB: u32 = 512u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_VOLUME_FLAVOR_STANDALONE: u32 = 256u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_VOLUME_FLAVOR_UNUSED1: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_VOLUME_STATES: u32 = 15u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_VOLUME_STATE_FORCE_SYNC: u32 = 64u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_VOLUME_STATE_INCONSISTENT: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_VOLUME_STATE_OFFLINE: u32 = 3u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_VOLUME_STATE_OK: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_VOLUME_STATE_ONLINE: u32 = 4u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_VOLUME_STATE_RESYNCHRONIZE: u32 = 16u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const DFS_VOLUME_STATE_STANDBY: u32 = 32u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const FSCTL_DFS_BASE: u32 = 6u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const FSCTL_DFS_GET_PKT_ENTRY_STATE: u32 = 401340u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const NET_DFS_SETDC_FLAGS: u32 = 0u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const NET_DFS_SETDC_INITPKT: u32 = 2u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem'*"]
pub const NET_DFS_SETDC_TIMEOUT: u32 = 1u32;
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetDfsAdd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dfsentrypath: Param0, servername: Param1, sharename: Param2, comment: Param3, flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsAdd(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR, comment: super::super::Foundation::PWSTR, flags: u32) -> u32;
        }
        ::core::mem::transmute(NetDfsAdd(dfsentrypath.into_param().abi(), servername.into_param().abi(), sharename.into_param().abi(), comment.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetDfsAddFtRoot<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, rootshare: Param1, ftdfsname: Param2, comment: Param3, flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsAddFtRoot(servername: super::super::Foundation::PWSTR, rootshare: super::super::Foundation::PWSTR, ftdfsname: super::super::Foundation::PWSTR, comment: super::super::Foundation::PWSTR, flags: u32) -> u32;
        }
        ::core::mem::transmute(NetDfsAddFtRoot(servername.into_param().abi(), rootshare.into_param().abi(), ftdfsname.into_param().abi(), comment.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetDfsAddRootTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pdfspath: Param0, ptargetpath: Param1, majorversion: u32, pcomment: Param3, flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsAddRootTarget(pdfspath: super::super::Foundation::PWSTR, ptargetpath: super::super::Foundation::PWSTR, majorversion: u32, pcomment: super::super::Foundation::PWSTR, flags: u32) -> u32;
        }
        ::core::mem::transmute(NetDfsAddRootTarget(pdfspath.into_param().abi(), ptargetpath.into_param().abi(), ::core::mem::transmute(majorversion), pcomment.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetDfsAddStdRoot<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, rootshare: Param1, comment: Param2, flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsAddStdRoot(servername: super::super::Foundation::PWSTR, rootshare: super::super::Foundation::PWSTR, comment: super::super::Foundation::PWSTR, flags: u32) -> u32;
        }
        ::core::mem::transmute(NetDfsAddStdRoot(servername.into_param().abi(), rootshare.into_param().abi(), comment.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetDfsEnum<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dfsname: Param0, level: u32, prefmaxlen: u32, buffer: *mut *mut u8, entriesread: *mut u32, resumehandle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsEnum(dfsname: super::super::Foundation::PWSTR, level: u32, prefmaxlen: u32, buffer: *mut *mut u8, entriesread: *mut u32, resumehandle: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetDfsEnum(dfsname.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(prefmaxlen), ::core::mem::transmute(buffer), ::core::mem::transmute(entriesread), ::core::mem::transmute(resumehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetDfsGetClientInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dfsentrypath: Param0, servername: Param1, sharename: Param2, level: u32, buffer: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsGetClientInfo(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR, level: u32, buffer: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetDfsGetClientInfo(dfsentrypath.into_param().abi(), servername.into_param().abi(), sharename.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NetDfsGetFtContainerSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(domainname: Param0, securityinformation: u32, ppsecuritydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsGetFtContainerSecurity(domainname: super::super::Foundation::PWSTR, securityinformation: u32, ppsecuritydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetDfsGetFtContainerSecurity(domainname.into_param().abi(), ::core::mem::transmute(securityinformation), ::core::mem::transmute(ppsecuritydescriptor), ::core::mem::transmute(lpcbsecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetDfsGetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dfsentrypath: Param0, servername: Param1, sharename: Param2, level: u32, buffer: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsGetInfo(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR, level: u32, buffer: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(NetDfsGetInfo(dfsentrypath.into_param().abi(), servername.into_param().abi(), sharename.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NetDfsGetSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dfsentrypath: Param0, securityinformation: u32, ppsecuritydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsGetSecurity(dfsentrypath: super::super::Foundation::PWSTR, securityinformation: u32, ppsecuritydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetDfsGetSecurity(dfsentrypath.into_param().abi(), ::core::mem::transmute(securityinformation), ::core::mem::transmute(ppsecuritydescriptor), ::core::mem::transmute(lpcbsecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NetDfsGetStdContainerSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(machinename: Param0, securityinformation: u32, ppsecuritydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsGetStdContainerSecurity(machinename: super::super::Foundation::PWSTR, securityinformation: u32, ppsecuritydescriptor: *mut *mut super::super::Security::SECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> u32;
        }
        ::core::mem::transmute(NetDfsGetStdContainerSecurity(machinename.into_param().abi(), ::core::mem::transmute(securityinformation), ::core::mem::transmute(ppsecuritydescriptor), ::core::mem::transmute(lpcbsecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetDfsGetSupportedNamespaceVersion<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(origin: DFS_NAMESPACE_VERSION_ORIGIN, pname: Param1, ppversioninfo: *mut *mut DFS_SUPPORTED_NAMESPACE_VERSION_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsGetSupportedNamespaceVersion(origin: DFS_NAMESPACE_VERSION_ORIGIN, pname: super::super::Foundation::PWSTR, ppversioninfo: *mut *mut DFS_SUPPORTED_NAMESPACE_VERSION_INFO) -> u32;
        }
        ::core::mem::transmute(NetDfsGetSupportedNamespaceVersion(::core::mem::transmute(origin), pname.into_param().abi(), ::core::mem::transmute(ppversioninfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetDfsMove<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(olddfsentrypath: Param0, newdfsentrypath: Param1, flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsMove(olddfsentrypath: super::super::Foundation::PWSTR, newdfsentrypath: super::super::Foundation::PWSTR, flags: u32) -> u32;
        }
        ::core::mem::transmute(NetDfsMove(olddfsentrypath.into_param().abi(), newdfsentrypath.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetDfsRemove<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dfsentrypath: Param0, servername: Param1, sharename: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsRemove(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(NetDfsRemove(dfsentrypath.into_param().abi(), servername.into_param().abi(), sharename.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetDfsRemoveFtRoot<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, rootshare: Param1, ftdfsname: Param2, flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsRemoveFtRoot(servername: super::super::Foundation::PWSTR, rootshare: super::super::Foundation::PWSTR, ftdfsname: super::super::Foundation::PWSTR, flags: u32) -> u32;
        }
        ::core::mem::transmute(NetDfsRemoveFtRoot(servername.into_param().abi(), rootshare.into_param().abi(), ftdfsname.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetDfsRemoveFtRootForced<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(domainname: Param0, servername: Param1, rootshare: Param2, ftdfsname: Param3, flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsRemoveFtRootForced(domainname: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, rootshare: super::super::Foundation::PWSTR, ftdfsname: super::super::Foundation::PWSTR, flags: u32) -> u32;
        }
        ::core::mem::transmute(NetDfsRemoveFtRootForced(domainname.into_param().abi(), servername.into_param().abi(), rootshare.into_param().abi(), ftdfsname.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetDfsRemoveRootTarget<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pdfspath: Param0, ptargetpath: Param1, flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsRemoveRootTarget(pdfspath: super::super::Foundation::PWSTR, ptargetpath: super::super::Foundation::PWSTR, flags: u32) -> u32;
        }
        ::core::mem::transmute(NetDfsRemoveRootTarget(pdfspath.into_param().abi(), ptargetpath.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetDfsRemoveStdRoot<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(servername: Param0, rootshare: Param1, flags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsRemoveStdRoot(servername: super::super::Foundation::PWSTR, rootshare: super::super::Foundation::PWSTR, flags: u32) -> u32;
        }
        ::core::mem::transmute(NetDfsRemoveStdRoot(servername.into_param().abi(), rootshare.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetDfsSetClientInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dfsentrypath: Param0, servername: Param1, sharename: Param2, level: u32, buffer: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsSetClientInfo(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR, level: u32, buffer: *const u8) -> u32;
        }
        ::core::mem::transmute(NetDfsSetClientInfo(dfsentrypath.into_param().abi(), servername.into_param().abi(), sharename.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NetDfsSetFtContainerSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(domainname: Param0, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsSetFtContainerSecurity(domainname: super::super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(NetDfsSetFtContainerSecurity(domainname.into_param().abi(), ::core::mem::transmute(securityinformation), ::core::mem::transmute(psecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NetDfsSetInfo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dfsentrypath: Param0, servername: Param1, sharename: Param2, level: u32, buffer: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsSetInfo(dfsentrypath: super::super::Foundation::PWSTR, servername: super::super::Foundation::PWSTR, sharename: super::super::Foundation::PWSTR, level: u32, buffer: *const u8) -> u32;
        }
        ::core::mem::transmute(NetDfsSetInfo(dfsentrypath.into_param().abi(), servername.into_param().abi(), sharename.into_param().abi(), ::core::mem::transmute(level), ::core::mem::transmute(buffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NetDfsSetSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dfsentrypath: Param0, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsSetSecurity(dfsentrypath: super::super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(NetDfsSetSecurity(dfsentrypath.into_param().abi(), ::core::mem::transmute(securityinformation), ::core::mem::transmute(psecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Storage_DistributedFileSystem', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NetDfsSetStdContainerSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(machinename: Param0, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NetDfsSetStdContainerSecurity(machinename: super::super::Foundation::PWSTR, securityinformation: u32, psecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> u32;
        }
        ::core::mem::transmute(NetDfsSetStdContainerSecurity(machinename.into_param().abi(), ::core::mem::transmute(securityinformation), ::core::mem::transmute(psecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
