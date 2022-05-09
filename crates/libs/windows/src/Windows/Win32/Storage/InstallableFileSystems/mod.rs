#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub struct FILTER_AGGREGATE_BASIC_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u32,
    pub Type: FILTER_AGGREGATE_BASIC_INFORMATION_0,
}
impl ::core::marker::Copy for FILTER_AGGREGATE_BASIC_INFORMATION {}
impl ::core::clone::Clone for FILTER_AGGREGATE_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FILTER_AGGREGATE_BASIC_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILTER_AGGREGATE_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILTER_AGGREGATE_BASIC_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILTER_AGGREGATE_BASIC_INFORMATION {}
impl ::core::default::Default for FILTER_AGGREGATE_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub union FILTER_AGGREGATE_BASIC_INFORMATION_0 {
    pub MiniFilter: FILTER_AGGREGATE_BASIC_INFORMATION_0_1,
    pub LegacyFilter: FILTER_AGGREGATE_BASIC_INFORMATION_0_0,
}
impl ::core::marker::Copy for FILTER_AGGREGATE_BASIC_INFORMATION_0 {}
impl ::core::clone::Clone for FILTER_AGGREGATE_BASIC_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FILTER_AGGREGATE_BASIC_INFORMATION_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILTER_AGGREGATE_BASIC_INFORMATION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILTER_AGGREGATE_BASIC_INFORMATION_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILTER_AGGREGATE_BASIC_INFORMATION_0 {}
impl ::core::default::Default for FILTER_AGGREGATE_BASIC_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub struct FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
}
impl ::core::marker::Copy for FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {}
impl ::core::clone::Clone for FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTER_AGGREGATE_BASIC_INFORMATION_0_0").field("FilterNameLength", &self.FilterNameLength).field("FilterNameBufferOffset", &self.FilterNameBufferOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILTER_AGGREGATE_BASIC_INFORMATION_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {}
impl ::core::default::Default for FILTER_AGGREGATE_BASIC_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub struct FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {
    pub FrameID: u32,
    pub NumberOfInstances: u32,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub FilterAltitudeLength: u16,
    pub FilterAltitudeBufferOffset: u16,
}
impl ::core::marker::Copy for FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {}
impl ::core::clone::Clone for FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTER_AGGREGATE_BASIC_INFORMATION_0_1").field("FrameID", &self.FrameID).field("NumberOfInstances", &self.NumberOfInstances).field("FilterNameLength", &self.FilterNameLength).field("FilterNameBufferOffset", &self.FilterNameBufferOffset).field("FilterAltitudeLength", &self.FilterAltitudeLength).field("FilterAltitudeBufferOffset", &self.FilterAltitudeBufferOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILTER_AGGREGATE_BASIC_INFORMATION_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {}
impl ::core::default::Default for FILTER_AGGREGATE_BASIC_INFORMATION_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub struct FILTER_AGGREGATE_STANDARD_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u32,
    pub Type: FILTER_AGGREGATE_STANDARD_INFORMATION_0,
}
impl ::core::marker::Copy for FILTER_AGGREGATE_STANDARD_INFORMATION {}
impl ::core::clone::Clone for FILTER_AGGREGATE_STANDARD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FILTER_AGGREGATE_STANDARD_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILTER_AGGREGATE_STANDARD_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILTER_AGGREGATE_STANDARD_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILTER_AGGREGATE_STANDARD_INFORMATION {}
impl ::core::default::Default for FILTER_AGGREGATE_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub union FILTER_AGGREGATE_STANDARD_INFORMATION_0 {
    pub MiniFilter: FILTER_AGGREGATE_STANDARD_INFORMATION_0_1,
    pub LegacyFilter: FILTER_AGGREGATE_STANDARD_INFORMATION_0_0,
}
impl ::core::marker::Copy for FILTER_AGGREGATE_STANDARD_INFORMATION_0 {}
impl ::core::clone::Clone for FILTER_AGGREGATE_STANDARD_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for FILTER_AGGREGATE_STANDARD_INFORMATION_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILTER_AGGREGATE_STANDARD_INFORMATION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILTER_AGGREGATE_STANDARD_INFORMATION_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILTER_AGGREGATE_STANDARD_INFORMATION_0 {}
impl ::core::default::Default for FILTER_AGGREGATE_STANDARD_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub struct FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {
    pub Flags: u32,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub FilterAltitudeLength: u16,
    pub FilterAltitudeBufferOffset: u16,
}
impl ::core::marker::Copy for FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {}
impl ::core::clone::Clone for FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTER_AGGREGATE_STANDARD_INFORMATION_0_0").field("Flags", &self.Flags).field("FilterNameLength", &self.FilterNameLength).field("FilterNameBufferOffset", &self.FilterNameBufferOffset).field("FilterAltitudeLength", &self.FilterAltitudeLength).field("FilterAltitudeBufferOffset", &self.FilterAltitudeBufferOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILTER_AGGREGATE_STANDARD_INFORMATION_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {}
impl ::core::default::Default for FILTER_AGGREGATE_STANDARD_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub struct FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {
    pub Flags: u32,
    pub FrameID: u32,
    pub NumberOfInstances: u32,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub FilterAltitudeLength: u16,
    pub FilterAltitudeBufferOffset: u16,
}
impl ::core::marker::Copy for FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {}
impl ::core::clone::Clone for FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTER_AGGREGATE_STANDARD_INFORMATION_0_1").field("Flags", &self.Flags).field("FrameID", &self.FrameID).field("NumberOfInstances", &self.NumberOfInstances).field("FilterNameLength", &self.FilterNameLength).field("FilterNameBufferOffset", &self.FilterNameBufferOffset).field("FilterAltitudeLength", &self.FilterAltitudeLength).field("FilterAltitudeBufferOffset", &self.FilterAltitudeBufferOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILTER_AGGREGATE_STANDARD_INFORMATION_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {}
impl ::core::default::Default for FILTER_AGGREGATE_STANDARD_INFORMATION_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub struct FILTER_FULL_INFORMATION {
    pub NextEntryOffset: u32,
    pub FrameID: u32,
    pub NumberOfInstances: u32,
    pub FilterNameLength: u16,
    pub FilterNameBuffer: [u16; 1],
}
impl ::core::marker::Copy for FILTER_FULL_INFORMATION {}
impl ::core::clone::Clone for FILTER_FULL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILTER_FULL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTER_FULL_INFORMATION").field("NextEntryOffset", &self.NextEntryOffset).field("FrameID", &self.FrameID).field("NumberOfInstances", &self.NumberOfInstances).field("FilterNameLength", &self.FilterNameLength).field("FilterNameBuffer", &self.FilterNameBuffer).finish()
    }
}
unsafe impl ::windows::core::Abi for FILTER_FULL_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILTER_FULL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILTER_FULL_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILTER_FULL_INFORMATION {}
impl ::core::default::Default for FILTER_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILTER_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FilterFullInformation: FILTER_INFORMATION_CLASS = FILTER_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FilterAggregateBasicInformation: FILTER_INFORMATION_CLASS = FILTER_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FilterAggregateStandardInformation: FILTER_INFORMATION_CLASS = FILTER_INFORMATION_CLASS(2i32);
impl ::core::marker::Copy for FILTER_INFORMATION_CLASS {}
impl ::core::clone::Clone for FILTER_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILTER_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FILTER_INFORMATION_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FILTER_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILTER_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub struct FILTER_MESSAGE_HEADER {
    pub ReplyLength: u32,
    pub MessageId: u64,
}
impl ::core::marker::Copy for FILTER_MESSAGE_HEADER {}
impl ::core::clone::Clone for FILTER_MESSAGE_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILTER_MESSAGE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTER_MESSAGE_HEADER").field("ReplyLength", &self.ReplyLength).field("MessageId", &self.MessageId).finish()
    }
}
unsafe impl ::windows::core::Abi for FILTER_MESSAGE_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILTER_MESSAGE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILTER_MESSAGE_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILTER_MESSAGE_HEADER {}
impl ::core::default::Default for FILTER_MESSAGE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FILTER_NAME_MAX_CHARS: u32 = 255u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct FILTER_REPLY_HEADER {
    pub Status: super::super::Foundation::NTSTATUS,
    pub MessageId: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FILTER_REPLY_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FILTER_REPLY_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILTER_REPLY_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTER_REPLY_HEADER").field("Status", &self.Status).field("MessageId", &self.MessageId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for FILTER_REPLY_HEADER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILTER_REPLY_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILTER_REPLY_HEADER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILTER_REPLY_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILTER_REPLY_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub struct FILTER_VOLUME_BASIC_INFORMATION {
    pub FilterVolumeNameLength: u16,
    pub FilterVolumeName: [u16; 1],
}
impl ::core::marker::Copy for FILTER_VOLUME_BASIC_INFORMATION {}
impl ::core::clone::Clone for FILTER_VOLUME_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILTER_VOLUME_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTER_VOLUME_BASIC_INFORMATION").field("FilterVolumeNameLength", &self.FilterVolumeNameLength).field("FilterVolumeName", &self.FilterVolumeName).finish()
    }
}
unsafe impl ::windows::core::Abi for FILTER_VOLUME_BASIC_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILTER_VOLUME_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILTER_VOLUME_BASIC_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILTER_VOLUME_BASIC_INFORMATION {}
impl ::core::default::Default for FILTER_VOLUME_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FILTER_VOLUME_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FilterVolumeBasicInformation: FILTER_VOLUME_INFORMATION_CLASS = FILTER_VOLUME_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FilterVolumeStandardInformation: FILTER_VOLUME_INFORMATION_CLASS = FILTER_VOLUME_INFORMATION_CLASS(1i32);
impl ::core::marker::Copy for FILTER_VOLUME_INFORMATION_CLASS {}
impl ::core::clone::Clone for FILTER_VOLUME_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILTER_VOLUME_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FILTER_VOLUME_INFORMATION_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for FILTER_VOLUME_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILTER_VOLUME_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub struct FILTER_VOLUME_STANDARD_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u32,
    pub FrameID: u32,
    pub FileSystemType: FLT_FILESYSTEM_TYPE,
    pub FilterVolumeNameLength: u16,
    pub FilterVolumeName: [u16; 1],
}
impl ::core::marker::Copy for FILTER_VOLUME_STANDARD_INFORMATION {}
impl ::core::clone::Clone for FILTER_VOLUME_STANDARD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FILTER_VOLUME_STANDARD_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTER_VOLUME_STANDARD_INFORMATION").field("NextEntryOffset", &self.NextEntryOffset).field("Flags", &self.Flags).field("FrameID", &self.FrameID).field("FileSystemType", &self.FileSystemType).field("FilterVolumeNameLength", &self.FilterVolumeNameLength).field("FilterVolumeName", &self.FilterVolumeName).finish()
    }
}
unsafe impl ::windows::core::Abi for FILTER_VOLUME_STANDARD_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FILTER_VOLUME_STANDARD_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FILTER_VOLUME_STANDARD_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for FILTER_VOLUME_STANDARD_INFORMATION {}
impl ::core::default::Default for FILTER_VOLUME_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLTFL_AGGREGATE_INFO_IS_LEGACYFILTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLTFL_AGGREGATE_INFO_IS_MINIFILTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLTFL_ASI_IS_LEGACYFILTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLTFL_ASI_IS_MINIFILTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLTFL_IASIL_DETACHED_VOLUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLTFL_IASIM_DETACHED_VOLUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLTFL_IASI_IS_LEGACYFILTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLTFL_IASI_IS_MINIFILTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLTFL_VSI_DETACHED_VOLUME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FLT_FILESYSTEM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_UNKNOWN: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_RAW: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_NTFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_FAT: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_CDFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_UDFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_LANMAN: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_WEBDAV: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_RDPDR: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_NFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_MS_NETWARE: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(10i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_NETWARE: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(11i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_BSUDF: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(12i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_MUP: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(13i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_RSFX: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(14i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_ROXIO_UDF1: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_ROXIO_UDF2: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(16i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_ROXIO_UDF3: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(17i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_TACIT: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(18i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_FS_REC: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(19i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_INCD: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(20i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_INCD_FAT: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(21i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_EXFAT: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(22i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_PSFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(23i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_GPFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(24i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_NPFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(25i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_MSFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(26i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_CSVFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(27i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_REFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(28i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_OPENAFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(29i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_FSTYPE_CIMFS: FLT_FILESYSTEM_TYPE = FLT_FILESYSTEM_TYPE(30i32);
impl ::core::marker::Copy for FLT_FILESYSTEM_TYPE {}
impl ::core::clone::Clone for FLT_FILESYSTEM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FLT_FILESYSTEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FLT_FILESYSTEM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FLT_FILESYSTEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FLT_FILESYSTEM_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const FLT_PORT_FLAG_SYNC_HANDLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[inline]
pub unsafe fn FilterAttach<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpfiltername: Param0, lpvolumename: Param1, lpinstancename: Param2, dwcreatedinstancenamelength: u32, lpcreatedinstancename: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterAttach(lpfiltername: ::windows::core::PCWSTR, lpvolumename: ::windows::core::PCWSTR, lpinstancename: ::windows::core::PCWSTR, dwcreatedinstancenamelength: u32, lpcreatedinstancename: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        FilterAttach(lpfiltername.into_param().abi(), lpvolumename.into_param().abi(), lpinstancename.into_param().abi(), ::core::mem::transmute(dwcreatedinstancenamelength), ::core::mem::transmute(lpcreatedinstancename)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[inline]
pub unsafe fn FilterAttachAtAltitude<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpfiltername: Param0, lpvolumename: Param1, lpaltitude: Param2, lpinstancename: Param3, dwcreatedinstancenamelength: u32, lpcreatedinstancename: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterAttachAtAltitude(lpfiltername: ::windows::core::PCWSTR, lpvolumename: ::windows::core::PCWSTR, lpaltitude: ::windows::core::PCWSTR, lpinstancename: ::windows::core::PCWSTR, dwcreatedinstancenamelength: u32, lpcreatedinstancename: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        FilterAttachAtAltitude(lpfiltername.into_param().abi(), lpvolumename.into_param().abi(), lpaltitude.into_param().abi(), lpinstancename.into_param().abi(), ::core::mem::transmute(dwcreatedinstancenamelength), ::core::mem::transmute(lpcreatedinstancename)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[inline]
pub unsafe fn FilterClose<'a, Param0: ::windows::core::IntoParam<'a, HFILTER>>(hfilter: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterClose(hfilter: HFILTER) -> ::windows::core::HRESULT;
        }
        FilterClose(hfilter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FilterConnectCommunicationPort<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpportname: Param0, dwoptions: u32, lpcontext: *const ::core::ffi::c_void, wsizeofcontext: u16, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterConnectCommunicationPort(lpportname: ::windows::core::PCWSTR, dwoptions: u32, lpcontext: *const ::core::ffi::c_void, wsizeofcontext: u16, lpsecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, hport: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::Foundation::HANDLE>::zeroed();
        FilterConnectCommunicationPort(lpportname.into_param().abi(), ::core::mem::transmute(dwoptions), ::core::mem::transmute(lpcontext), ::core::mem::transmute(wsizeofcontext), ::core::mem::transmute(lpsecurityattributes), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[inline]
pub unsafe fn FilterCreate<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpfiltername: Param0) -> ::windows::core::Result<HFILTER> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterCreate(lpfiltername: ::windows::core::PCWSTR, hfilter: *mut HFILTER) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<HFILTER>::zeroed();
        FilterCreate(lpfiltername.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<HFILTER>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[inline]
pub unsafe fn FilterDetach<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpfiltername: Param0, lpvolumename: Param1, lpinstancename: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterDetach(lpfiltername: ::windows::core::PCWSTR, lpvolumename: ::windows::core::PCWSTR, lpinstancename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        FilterDetach(lpfiltername.into_param().abi(), lpvolumename.into_param().abi(), lpinstancename.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FilterFindClose<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfilterfind: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterFindClose(hfilterfind: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        FilterFindClose(hfilterfind.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[inline]
pub unsafe fn FilterFindFirst(dwinformationclass: FILTER_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpfilterfind: *mut FilterFindHandle) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterFindFirst(dwinformationclass: FILTER_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpfilterfind: *mut FilterFindHandle) -> ::windows::core::HRESULT;
        }
        FilterFindFirst(::core::mem::transmute(dwinformationclass), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbuffersize), ::core::mem::transmute(lpbytesreturned), ::core::mem::transmute(lpfilterfind)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FilterFindHandle(pub isize);
impl FilterFindHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for FilterFindHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FilterFindHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FilterFindHandle {}
impl ::core::fmt::Debug for FilterFindHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FilterFindHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for FilterFindHandle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FilterFindNext<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfilterfind: Param0, dwinformationclass: FILTER_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterFindNext(hfilterfind: super::super::Foundation::HANDLE, dwinformationclass: FILTER_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::core::HRESULT;
        }
        FilterFindNext(hfilterfind.into_param().abi(), ::core::mem::transmute(dwinformationclass), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbuffersize), ::core::mem::transmute(lpbytesreturned)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[inline]
pub unsafe fn FilterGetDosName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpvolumename: Param0, lpdosname: &mut [u16]) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterGetDosName(lpvolumename: ::windows::core::PCWSTR, lpdosname: ::windows::core::PWSTR, dwdosnamebuffersize: u32) -> ::windows::core::HRESULT;
        }
        FilterGetDosName(lpvolumename.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpdosname)), lpdosname.len() as _).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[inline]
pub unsafe fn FilterGetInformation<'a, Param0: ::windows::core::IntoParam<'a, HFILTER>>(hfilter: Param0, dwinformationclass: FILTER_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterGetInformation(hfilter: HFILTER, dwinformationclass: FILTER_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::core::HRESULT;
        }
        FilterGetInformation(hfilter.into_param().abi(), ::core::mem::transmute(dwinformationclass), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbuffersize), ::core::mem::transmute(lpbytesreturned)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn FilterGetMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hport: Param0, lpmessagebuffer: *mut FILTER_MESSAGE_HEADER, dwmessagebuffersize: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterGetMessage(hport: super::super::Foundation::HANDLE, lpmessagebuffer: *mut FILTER_MESSAGE_HEADER, dwmessagebuffersize: u32, lpoverlapped: *mut super::super::System::IO::OVERLAPPED) -> ::windows::core::HRESULT;
        }
        FilterGetMessage(hport.into_param().abi(), ::core::mem::transmute(lpmessagebuffer), ::core::mem::transmute(dwmessagebuffersize), ::core::mem::transmute(lpoverlapped)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[inline]
pub unsafe fn FilterInstanceClose<'a, Param0: ::windows::core::IntoParam<'a, HFILTER_INSTANCE>>(hinstance: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterInstanceClose(hinstance: HFILTER_INSTANCE) -> ::windows::core::HRESULT;
        }
        FilterInstanceClose(hinstance.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[inline]
pub unsafe fn FilterInstanceCreate<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpfiltername: Param0, lpvolumename: Param1, lpinstancename: Param2) -> ::windows::core::Result<HFILTER_INSTANCE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterInstanceCreate(lpfiltername: ::windows::core::PCWSTR, lpvolumename: ::windows::core::PCWSTR, lpinstancename: ::windows::core::PCWSTR, hinstance: *mut HFILTER_INSTANCE) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<HFILTER_INSTANCE>::zeroed();
        FilterInstanceCreate(lpfiltername.into_param().abi(), lpvolumename.into_param().abi(), lpinstancename.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<HFILTER_INSTANCE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FilterInstanceFindClose<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfilterinstancefind: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterInstanceFindClose(hfilterinstancefind: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        FilterInstanceFindClose(hfilterinstancefind.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[inline]
pub unsafe fn FilterInstanceFindFirst<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpfiltername: Param0, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpfilterinstancefind: *mut FilterInstanceFindHandle) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterInstanceFindFirst(lpfiltername: ::windows::core::PCWSTR, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpfilterinstancefind: *mut FilterInstanceFindHandle) -> ::windows::core::HRESULT;
        }
        FilterInstanceFindFirst(lpfiltername.into_param().abi(), ::core::mem::transmute(dwinformationclass), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbuffersize), ::core::mem::transmute(lpbytesreturned), ::core::mem::transmute(lpfilterinstancefind)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FilterInstanceFindHandle(pub isize);
impl FilterInstanceFindHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for FilterInstanceFindHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FilterInstanceFindHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FilterInstanceFindHandle {}
impl ::core::fmt::Debug for FilterInstanceFindHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FilterInstanceFindHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for FilterInstanceFindHandle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FilterInstanceFindNext<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfilterinstancefind: Param0, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterInstanceFindNext(hfilterinstancefind: super::super::Foundation::HANDLE, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::core::HRESULT;
        }
        FilterInstanceFindNext(hfilterinstancefind.into_param().abi(), ::core::mem::transmute(dwinformationclass), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbuffersize), ::core::mem::transmute(lpbytesreturned)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[inline]
pub unsafe fn FilterInstanceGetInformation<'a, Param0: ::windows::core::IntoParam<'a, HFILTER_INSTANCE>>(hinstance: Param0, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterInstanceGetInformation(hinstance: HFILTER_INSTANCE, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::core::HRESULT;
        }
        FilterInstanceGetInformation(hinstance.into_param().abi(), ::core::mem::transmute(dwinformationclass), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbuffersize), ::core::mem::transmute(lpbytesreturned)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[inline]
pub unsafe fn FilterLoad<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpfiltername: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterLoad(lpfiltername: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        FilterLoad(lpfiltername.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FilterReplyMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hport: Param0, lpreplybuffer: *const FILTER_REPLY_HEADER, dwreplybuffersize: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterReplyMessage(hport: super::super::Foundation::HANDLE, lpreplybuffer: *const FILTER_REPLY_HEADER, dwreplybuffersize: u32) -> ::windows::core::HRESULT;
        }
        FilterReplyMessage(hport.into_param().abi(), ::core::mem::transmute(lpreplybuffer), ::core::mem::transmute(dwreplybuffersize)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FilterSendMessage<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hport: Param0, lpinbuffer: *const ::core::ffi::c_void, dwinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, dwoutbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterSendMessage(hport: super::super::Foundation::HANDLE, lpinbuffer: *const ::core::ffi::c_void, dwinbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, dwoutbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::core::HRESULT;
        }
        FilterSendMessage(hport.into_param().abi(), ::core::mem::transmute(lpinbuffer), ::core::mem::transmute(dwinbuffersize), ::core::mem::transmute(lpoutbuffer), ::core::mem::transmute(dwoutbuffersize), ::core::mem::transmute(lpbytesreturned)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[inline]
pub unsafe fn FilterUnload<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpfiltername: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterUnload(lpfiltername: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        FilterUnload(lpfiltername.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FilterVolumeFindClose<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hvolumefind: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterVolumeFindClose(hvolumefind: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        FilterVolumeFindClose(hvolumefind.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[inline]
pub unsafe fn FilterVolumeFindFirst(dwinformationclass: FILTER_VOLUME_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpvolumefind: *mut FilterVolumeFindHandle) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterVolumeFindFirst(dwinformationclass: FILTER_VOLUME_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpvolumefind: *mut FilterVolumeFindHandle) -> ::windows::core::HRESULT;
        }
        FilterVolumeFindFirst(::core::mem::transmute(dwinformationclass), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbuffersize), ::core::mem::transmute(lpbytesreturned), ::core::mem::transmute(lpvolumefind)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FilterVolumeFindHandle(pub isize);
impl FilterVolumeFindHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for FilterVolumeFindHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FilterVolumeFindHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FilterVolumeFindHandle {}
impl ::core::fmt::Debug for FilterVolumeFindHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FilterVolumeFindHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for FilterVolumeFindHandle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FilterVolumeFindNext<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hvolumefind: Param0, dwinformationclass: FILTER_VOLUME_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterVolumeFindNext(hvolumefind: super::super::Foundation::HANDLE, dwinformationclass: FILTER_VOLUME_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::core::HRESULT;
        }
        FilterVolumeFindNext(hvolumefind.into_param().abi(), ::core::mem::transmute(dwinformationclass), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbuffersize), ::core::mem::transmute(lpbytesreturned)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FilterVolumeInstanceFindClose<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hvolumeinstancefind: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterVolumeInstanceFindClose(hvolumeinstancefind: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        FilterVolumeInstanceFindClose(hvolumeinstancefind.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[inline]
pub unsafe fn FilterVolumeInstanceFindFirst<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpvolumename: Param0, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpvolumeinstancefind: *mut FilterVolumeInstanceFindHandle) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterVolumeInstanceFindFirst(lpvolumename: ::windows::core::PCWSTR, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32, lpvolumeinstancefind: *mut FilterVolumeInstanceFindHandle) -> ::windows::core::HRESULT;
        }
        FilterVolumeInstanceFindFirst(lpvolumename.into_param().abi(), ::core::mem::transmute(dwinformationclass), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbuffersize), ::core::mem::transmute(lpbytesreturned), ::core::mem::transmute(lpvolumeinstancefind)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FilterVolumeInstanceFindHandle(pub isize);
impl FilterVolumeInstanceFindHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for FilterVolumeInstanceFindHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for FilterVolumeInstanceFindHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for FilterVolumeInstanceFindHandle {}
impl ::core::fmt::Debug for FilterVolumeInstanceFindHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FilterVolumeInstanceFindHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for FilterVolumeInstanceFindHandle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FilterVolumeInstanceFindNext<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hvolumeinstancefind: Param0, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FilterVolumeInstanceFindNext(hvolumeinstancefind: super::super::Foundation::HANDLE, dwinformationclass: INSTANCE_INFORMATION_CLASS, lpbuffer: *mut ::core::ffi::c_void, dwbuffersize: u32, lpbytesreturned: *mut u32) -> ::windows::core::HRESULT;
        }
        FilterVolumeInstanceFindNext(hvolumeinstancefind.into_param().abi(), ::core::mem::transmute(dwinformationclass), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwbuffersize), ::core::mem::transmute(lpbytesreturned)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HFILTER(pub isize);
impl HFILTER {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HFILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HFILTER {}
impl ::core::fmt::Debug for HFILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HFILTER").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HFILTER {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HFILTER_INSTANCE(pub isize);
impl HFILTER_INSTANCE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HFILTER_INSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HFILTER_INSTANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HFILTER_INSTANCE {}
impl ::core::fmt::Debug for HFILTER_INSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HFILTER_INSTANCE").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HFILTER_INSTANCE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub struct INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u32,
    pub Type: INSTANCE_AGGREGATE_STANDARD_INFORMATION_0,
}
impl ::core::marker::Copy for INSTANCE_AGGREGATE_STANDARD_INFORMATION {}
impl ::core::clone::Clone for INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INSTANCE_AGGREGATE_STANDARD_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for INSTANCE_AGGREGATE_STANDARD_INFORMATION {}
impl ::core::default::Default for INSTANCE_AGGREGATE_STANDARD_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub union INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {
    pub MiniFilter: INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1,
    pub LegacyFilter: INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0,
}
impl ::core::marker::Copy for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {}
impl ::core::clone::Clone for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INSTANCE_AGGREGATE_STANDARD_INFORMATION_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {}
impl ::core::default::Default for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub struct INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {
    pub Flags: u32,
    pub AltitudeLength: u16,
    pub AltitudeBufferOffset: u16,
    pub VolumeNameLength: u16,
    pub VolumeNameBufferOffset: u16,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub SupportedFeatures: u32,
}
impl ::core::marker::Copy for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {}
impl ::core::clone::Clone for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0").field("Flags", &self.Flags).field("AltitudeLength", &self.AltitudeLength).field("AltitudeBufferOffset", &self.AltitudeBufferOffset).field("VolumeNameLength", &self.VolumeNameLength).field("VolumeNameBufferOffset", &self.VolumeNameBufferOffset).field("FilterNameLength", &self.FilterNameLength).field("FilterNameBufferOffset", &self.FilterNameBufferOffset).field("SupportedFeatures", &self.SupportedFeatures).finish()
    }
}
unsafe impl ::windows::core::Abi for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {}
impl ::core::default::Default for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub struct INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {
    pub Flags: u32,
    pub FrameID: u32,
    pub VolumeFileSystemType: FLT_FILESYSTEM_TYPE,
    pub InstanceNameLength: u16,
    pub InstanceNameBufferOffset: u16,
    pub AltitudeLength: u16,
    pub AltitudeBufferOffset: u16,
    pub VolumeNameLength: u16,
    pub VolumeNameBufferOffset: u16,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
    pub SupportedFeatures: u32,
}
impl ::core::marker::Copy for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {}
impl ::core::clone::Clone for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1")
            .field("Flags", &self.Flags)
            .field("FrameID", &self.FrameID)
            .field("VolumeFileSystemType", &self.VolumeFileSystemType)
            .field("InstanceNameLength", &self.InstanceNameLength)
            .field("InstanceNameBufferOffset", &self.InstanceNameBufferOffset)
            .field("AltitudeLength", &self.AltitudeLength)
            .field("AltitudeBufferOffset", &self.AltitudeBufferOffset)
            .field("VolumeNameLength", &self.VolumeNameLength)
            .field("VolumeNameBufferOffset", &self.VolumeNameBufferOffset)
            .field("FilterNameLength", &self.FilterNameLength)
            .field("FilterNameBufferOffset", &self.FilterNameBufferOffset)
            .field("SupportedFeatures", &self.SupportedFeatures)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {}
impl ::core::default::Default for INSTANCE_AGGREGATE_STANDARD_INFORMATION_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub struct INSTANCE_BASIC_INFORMATION {
    pub NextEntryOffset: u32,
    pub InstanceNameLength: u16,
    pub InstanceNameBufferOffset: u16,
}
impl ::core::marker::Copy for INSTANCE_BASIC_INFORMATION {}
impl ::core::clone::Clone for INSTANCE_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INSTANCE_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSTANCE_BASIC_INFORMATION").field("NextEntryOffset", &self.NextEntryOffset).field("InstanceNameLength", &self.InstanceNameLength).field("InstanceNameBufferOffset", &self.InstanceNameBufferOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for INSTANCE_BASIC_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INSTANCE_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INSTANCE_BASIC_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for INSTANCE_BASIC_INFORMATION {}
impl ::core::default::Default for INSTANCE_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub struct INSTANCE_FULL_INFORMATION {
    pub NextEntryOffset: u32,
    pub InstanceNameLength: u16,
    pub InstanceNameBufferOffset: u16,
    pub AltitudeLength: u16,
    pub AltitudeBufferOffset: u16,
    pub VolumeNameLength: u16,
    pub VolumeNameBufferOffset: u16,
    pub FilterNameLength: u16,
    pub FilterNameBufferOffset: u16,
}
impl ::core::marker::Copy for INSTANCE_FULL_INFORMATION {}
impl ::core::clone::Clone for INSTANCE_FULL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INSTANCE_FULL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSTANCE_FULL_INFORMATION")
            .field("NextEntryOffset", &self.NextEntryOffset)
            .field("InstanceNameLength", &self.InstanceNameLength)
            .field("InstanceNameBufferOffset", &self.InstanceNameBufferOffset)
            .field("AltitudeLength", &self.AltitudeLength)
            .field("AltitudeBufferOffset", &self.AltitudeBufferOffset)
            .field("VolumeNameLength", &self.VolumeNameLength)
            .field("VolumeNameBufferOffset", &self.VolumeNameBufferOffset)
            .field("FilterNameLength", &self.FilterNameLength)
            .field("FilterNameBufferOffset", &self.FilterNameBufferOffset)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for INSTANCE_FULL_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INSTANCE_FULL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INSTANCE_FULL_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for INSTANCE_FULL_INFORMATION {}
impl ::core::default::Default for INSTANCE_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct INSTANCE_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const InstanceBasicInformation: INSTANCE_INFORMATION_CLASS = INSTANCE_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const InstancePartialInformation: INSTANCE_INFORMATION_CLASS = INSTANCE_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const InstanceFullInformation: INSTANCE_INFORMATION_CLASS = INSTANCE_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const InstanceAggregateStandardInformation: INSTANCE_INFORMATION_CLASS = INSTANCE_INFORMATION_CLASS(3i32);
impl ::core::marker::Copy for INSTANCE_INFORMATION_CLASS {}
impl ::core::clone::Clone for INSTANCE_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for INSTANCE_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for INSTANCE_INFORMATION_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for INSTANCE_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSTANCE_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const INSTANCE_NAME_MAX_CHARS: u32 = 255u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub struct INSTANCE_PARTIAL_INFORMATION {
    pub NextEntryOffset: u32,
    pub InstanceNameLength: u16,
    pub InstanceNameBufferOffset: u16,
    pub AltitudeLength: u16,
    pub AltitudeBufferOffset: u16,
}
impl ::core::marker::Copy for INSTANCE_PARTIAL_INFORMATION {}
impl ::core::clone::Clone for INSTANCE_PARTIAL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for INSTANCE_PARTIAL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INSTANCE_PARTIAL_INFORMATION").field("NextEntryOffset", &self.NextEntryOffset).field("InstanceNameLength", &self.InstanceNameLength).field("InstanceNameBufferOffset", &self.InstanceNameBufferOffset).field("AltitudeLength", &self.AltitudeLength).field("AltitudeBufferOffset", &self.AltitudeBufferOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for INSTANCE_PARTIAL_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for INSTANCE_PARTIAL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<INSTANCE_PARTIAL_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for INSTANCE_PARTIAL_INFORMATION {}
impl ::core::default::Default for INSTANCE_PARTIAL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const VOLUME_NAME_MAX_CHARS: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_CRED_MANAGER: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_10NET: u32 = 327680u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_3IN1: u32 = 2555904u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_9P: u32 = 4718592u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_9TILES: u32 = 589824u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_APPLETALK: u32 = 1245184u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_AS400: u32 = 720896u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_AURISTOR_FS: u32 = 4587520u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_AVID: u32 = 1703936u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_AVID1: u32 = 3801088u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_BMC: u32 = 1572864u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_BWNFS: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_CLEARCASE: u32 = 1441792u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_COGENT: u32 = 1114112u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_CSC: u32 = 2490368u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_DAV: u32 = 3014656u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_DCE: u32 = 1638400u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_DECORB: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_DFS: u32 = 3866624u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_DISTINCT: u32 = 2293760u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_DOCUSHARE: u32 = 4521984u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_DOCUSPACE: u32 = 1769472u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_DRIVEONWEB: u32 = 4063232u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_EXIFS: u32 = 2949120u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_EXTENDNET: u32 = 2686976u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_FARALLON: u32 = 1179648u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_FJ_REDIR: u32 = 2228224u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_FOXBAT: u32 = 2818048u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_FRONTIER: u32 = 1507328u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_FTP_NFS: u32 = 786432u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_GOOGLE: u32 = 4390912u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_HOB_NFS: u32 = 3276800u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_IBMAL: u32 = 3407872u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_INTERGRAPH: u32 = 1310720u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_KNOWARE: u32 = 3080192u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_KWNP: u32 = 3932160u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_LANMAN: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_LANSTEP: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_LANTASTIC: u32 = 655360u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_LIFENET: u32 = 917504u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_LOCK: u32 = 3473408u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_LOCUS: u32 = 393216u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_MANGOSOFT: u32 = 1835008u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_MASFAX: u32 = 3211264u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_MFILES: u32 = 4259840u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_MSNET: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_MS_NFS: u32 = 4325376u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_NDFS: u32 = 4456448u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_NETWARE: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_OBJECT_DIRE: u32 = 3145728u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_OPENAFS: u32 = 3735552u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_PATHWORKS: u32 = 851968u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_POWERLAN: u32 = 983040u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_PROTSTOR: u32 = 2162688u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_QUINCY: u32 = 3670016u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_RDR2SAMPLE: u32 = 2424832u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_RIVERFRONT1: u32 = 1966080u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_RIVERFRONT2: u32 = 2031616u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_RSFX: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_SECUREAGENT: u32 = 4653056u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_SERNET: u32 = 1900544u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_SHIVA: u32 = 3342336u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_SMB: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_SRT: u32 = 3604480u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_STAC: u32 = 2752512u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_SUN_PC_NFS: u32 = 458752u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_SYMFONET: u32 = 1376256u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_TERMSRV: u32 = 3538944u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_TWINS: u32 = 2359296u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_VINES: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_VMWARE: u32 = 4128768u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_YAHOO: u32 = 2883584u32;
#[doc = "*Required features: `\"Win32_Storage_InstallableFileSystems\"`*"]
pub const WNNC_NET_ZENWORKS: u32 = 3997696u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
