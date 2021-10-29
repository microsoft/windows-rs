#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct APPLY_SNAPSHOT_VHDSET_FLAG(pub u32);
pub const APPLY_SNAPSHOT_VHDSET_FLAG_NONE: APPLY_SNAPSHOT_VHDSET_FLAG =
    APPLY_SNAPSHOT_VHDSET_FLAG(0u32);
pub const APPLY_SNAPSHOT_VHDSET_FLAG_WRITEABLE: APPLY_SNAPSHOT_VHDSET_FLAG =
    APPLY_SNAPSHOT_VHDSET_FLAG(1u32);
impl ::std::convert::From<u32> for APPLY_SNAPSHOT_VHDSET_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APPLY_SNAPSHOT_VHDSET_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for APPLY_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for APPLY_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for APPLY_SNAPSHOT_VHDSET_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for APPLY_SNAPSHOT_VHDSET_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for APPLY_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct APPLY_SNAPSHOT_VHDSET_PARAMETERS {
    pub Version: APPLY_SNAPSHOT_VHDSET_VERSION,
    pub Anonymous: APPLY_SNAPSHOT_VHDSET_PARAMETERS_0,
}
impl APPLY_SNAPSHOT_VHDSET_PARAMETERS {}
impl ::std::default::Default for APPLY_SNAPSHOT_VHDSET_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for APPLY_SNAPSHOT_VHDSET_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for APPLY_SNAPSHOT_VHDSET_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for APPLY_SNAPSHOT_VHDSET_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union APPLY_SNAPSHOT_VHDSET_PARAMETERS_0 {
    pub Version1: APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0,
}
impl APPLY_SNAPSHOT_VHDSET_PARAMETERS_0 {}
impl ::std::default::Default for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0 {}
unsafe impl ::windows::runtime::Abi for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    pub SnapshotId: ::windows::runtime::GUID,
    pub LeafSnapshotId: ::windows::runtime::GUID,
}
impl APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0 {}
impl ::std::default::Default for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version1_e__Struct")
            .field("SnapshotId", &self.SnapshotId)
            .field("LeafSnapshotId", &self.LeafSnapshotId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.SnapshotId == other.SnapshotId && self.LeafSnapshotId == other.LeafSnapshotId
    }
}
impl ::std::cmp::Eq for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0 {}
unsafe impl ::windows::runtime::Abi for APPLY_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct APPLY_SNAPSHOT_VHDSET_VERSION(pub i32);
pub const APPLY_SNAPSHOT_VHDSET_VERSION_UNSPECIFIED: APPLY_SNAPSHOT_VHDSET_VERSION =
    APPLY_SNAPSHOT_VHDSET_VERSION(0i32);
pub const APPLY_SNAPSHOT_VHDSET_VERSION_1: APPLY_SNAPSHOT_VHDSET_VERSION =
    APPLY_SNAPSHOT_VHDSET_VERSION(1i32);
impl ::std::convert::From<i32> for APPLY_SNAPSHOT_VHDSET_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for APPLY_SNAPSHOT_VHDSET_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ATTACH_VIRTUAL_DISK_FLAG(pub u32);
pub const ATTACH_VIRTUAL_DISK_FLAG_NONE: ATTACH_VIRTUAL_DISK_FLAG = ATTACH_VIRTUAL_DISK_FLAG(0u32);
pub const ATTACH_VIRTUAL_DISK_FLAG_READ_ONLY: ATTACH_VIRTUAL_DISK_FLAG =
    ATTACH_VIRTUAL_DISK_FLAG(1u32);
pub const ATTACH_VIRTUAL_DISK_FLAG_NO_DRIVE_LETTER: ATTACH_VIRTUAL_DISK_FLAG =
    ATTACH_VIRTUAL_DISK_FLAG(2u32);
pub const ATTACH_VIRTUAL_DISK_FLAG_PERMANENT_LIFETIME: ATTACH_VIRTUAL_DISK_FLAG =
    ATTACH_VIRTUAL_DISK_FLAG(4u32);
pub const ATTACH_VIRTUAL_DISK_FLAG_NO_LOCAL_HOST: ATTACH_VIRTUAL_DISK_FLAG =
    ATTACH_VIRTUAL_DISK_FLAG(8u32);
pub const ATTACH_VIRTUAL_DISK_FLAG_NO_SECURITY_DESCRIPTOR: ATTACH_VIRTUAL_DISK_FLAG =
    ATTACH_VIRTUAL_DISK_FLAG(16u32);
pub const ATTACH_VIRTUAL_DISK_FLAG_BYPASS_DEFAULT_ENCRYPTION_POLICY: ATTACH_VIRTUAL_DISK_FLAG =
    ATTACH_VIRTUAL_DISK_FLAG(32u32);
pub const ATTACH_VIRTUAL_DISK_FLAG_NON_PNP: ATTACH_VIRTUAL_DISK_FLAG =
    ATTACH_VIRTUAL_DISK_FLAG(64u32);
pub const ATTACH_VIRTUAL_DISK_FLAG_RESTRICTED_RANGE: ATTACH_VIRTUAL_DISK_FLAG =
    ATTACH_VIRTUAL_DISK_FLAG(128u32);
pub const ATTACH_VIRTUAL_DISK_FLAG_SINGLE_PARTITION: ATTACH_VIRTUAL_DISK_FLAG =
    ATTACH_VIRTUAL_DISK_FLAG(256u32);
pub const ATTACH_VIRTUAL_DISK_FLAG_REGISTER_VOLUME: ATTACH_VIRTUAL_DISK_FLAG =
    ATTACH_VIRTUAL_DISK_FLAG(512u32);
impl ::std::convert::From<u32> for ATTACH_VIRTUAL_DISK_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ATTACH_VIRTUAL_DISK_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for ATTACH_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ATTACH_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ATTACH_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ATTACH_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ATTACH_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ATTACH_VIRTUAL_DISK_PARAMETERS {
    pub Version: ATTACH_VIRTUAL_DISK_VERSION,
    pub Anonymous: ATTACH_VIRTUAL_DISK_PARAMETERS_0,
}
impl ATTACH_VIRTUAL_DISK_PARAMETERS {}
impl ::std::default::Default for ATTACH_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ATTACH_VIRTUAL_DISK_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ATTACH_VIRTUAL_DISK_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for ATTACH_VIRTUAL_DISK_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union ATTACH_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: ATTACH_VIRTUAL_DISK_PARAMETERS_0_0,
    pub Version2: ATTACH_VIRTUAL_DISK_PARAMETERS_0_1,
}
impl ATTACH_VIRTUAL_DISK_PARAMETERS_0 {}
impl ::std::default::Default for ATTACH_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ATTACH_VIRTUAL_DISK_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for ATTACH_VIRTUAL_DISK_PARAMETERS_0 {}
unsafe impl ::windows::runtime::Abi for ATTACH_VIRTUAL_DISK_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ATTACH_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub Reserved: u32,
}
impl ATTACH_VIRTUAL_DISK_PARAMETERS_0_0 {}
impl ::std::default::Default for ATTACH_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ATTACH_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version1_e__Struct")
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ATTACH_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for ATTACH_VIRTUAL_DISK_PARAMETERS_0_0 {}
unsafe impl ::windows::runtime::Abi for ATTACH_VIRTUAL_DISK_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ATTACH_VIRTUAL_DISK_PARAMETERS_0_1 {
    pub RestrictedOffset: u64,
    pub RestrictedLength: u64,
}
impl ATTACH_VIRTUAL_DISK_PARAMETERS_0_1 {}
impl ::std::default::Default for ATTACH_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ATTACH_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version2_e__Struct")
            .field("RestrictedOffset", &self.RestrictedOffset)
            .field("RestrictedLength", &self.RestrictedLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ATTACH_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.RestrictedOffset == other.RestrictedOffset
            && self.RestrictedLength == other.RestrictedLength
    }
}
impl ::std::cmp::Eq for ATTACH_VIRTUAL_DISK_PARAMETERS_0_1 {}
unsafe impl ::windows::runtime::Abi for ATTACH_VIRTUAL_DISK_PARAMETERS_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ATTACH_VIRTUAL_DISK_VERSION(pub i32);
pub const ATTACH_VIRTUAL_DISK_VERSION_UNSPECIFIED: ATTACH_VIRTUAL_DISK_VERSION =
    ATTACH_VIRTUAL_DISK_VERSION(0i32);
pub const ATTACH_VIRTUAL_DISK_VERSION_1: ATTACH_VIRTUAL_DISK_VERSION =
    ATTACH_VIRTUAL_DISK_VERSION(1i32);
pub const ATTACH_VIRTUAL_DISK_VERSION_2: ATTACH_VIRTUAL_DISK_VERSION =
    ATTACH_VIRTUAL_DISK_VERSION(2i32);
impl ::std::convert::From<i32> for ATTACH_VIRTUAL_DISK_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ATTACH_VIRTUAL_DISK_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddVirtualDiskParent<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    virtualdiskhandle: Param0,
    parentpath: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddVirtualDiskParent(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                parentpath: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(AddVirtualDiskParent(
            virtualdiskhandle.into_param().abi(),
            parentpath.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ApplySnapshotVhdSet<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    parameters: *const APPLY_SNAPSHOT_VHDSET_PARAMETERS,
    flags: APPLY_SNAPSHOT_VHDSET_FLAG,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ApplySnapshotVhdSet(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                parameters: *const APPLY_SNAPSHOT_VHDSET_PARAMETERS,
                flags: APPLY_SNAPSHOT_VHDSET_FLAG,
            ) -> u32;
        }
        ::std::mem::transmute(ApplySnapshotVhdSet(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(parameters),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security",
    feature = "Win32_System_SystemServices"
))]
#[inline]
pub unsafe fn AttachVirtualDisk<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR,
    flags: ATTACH_VIRTUAL_DISK_FLAG,
    providerspecificflags: u32,
    parameters: *const ATTACH_VIRTUAL_DISK_PARAMETERS,
    overlapped: *const super::super::System::SystemServices::OVERLAPPED,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AttachVirtualDisk(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR,
                flags: ATTACH_VIRTUAL_DISK_FLAG,
                providerspecificflags: u32,
                parameters: *const ATTACH_VIRTUAL_DISK_PARAMETERS,
                overlapped: *const super::super::System::SystemServices::OVERLAPPED,
            ) -> u32;
        }
        ::std::mem::transmute(AttachVirtualDisk(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(securitydescriptor),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(providerspecificflags),
            ::std::mem::transmute(parameters),
            ::std::mem::transmute(overlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BreakMirrorVirtualDisk<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BreakMirrorVirtualDisk(virtualdiskhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(BreakMirrorVirtualDisk(virtualdiskhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct COMPACT_VIRTUAL_DISK_FLAG(pub u32);
pub const COMPACT_VIRTUAL_DISK_FLAG_NONE: COMPACT_VIRTUAL_DISK_FLAG =
    COMPACT_VIRTUAL_DISK_FLAG(0u32);
pub const COMPACT_VIRTUAL_DISK_FLAG_NO_ZERO_SCAN: COMPACT_VIRTUAL_DISK_FLAG =
    COMPACT_VIRTUAL_DISK_FLAG(1u32);
pub const COMPACT_VIRTUAL_DISK_FLAG_NO_BLOCK_MOVES: COMPACT_VIRTUAL_DISK_FLAG =
    COMPACT_VIRTUAL_DISK_FLAG(2u32);
impl ::std::convert::From<u32> for COMPACT_VIRTUAL_DISK_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMPACT_VIRTUAL_DISK_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for COMPACT_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for COMPACT_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for COMPACT_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for COMPACT_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for COMPACT_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct COMPACT_VIRTUAL_DISK_PARAMETERS {
    pub Version: COMPACT_VIRTUAL_DISK_VERSION,
    pub Anonymous: COMPACT_VIRTUAL_DISK_PARAMETERS_0,
}
impl COMPACT_VIRTUAL_DISK_PARAMETERS {}
impl ::std::default::Default for COMPACT_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for COMPACT_VIRTUAL_DISK_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for COMPACT_VIRTUAL_DISK_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for COMPACT_VIRTUAL_DISK_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union COMPACT_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: COMPACT_VIRTUAL_DISK_PARAMETERS_0_0,
}
impl COMPACT_VIRTUAL_DISK_PARAMETERS_0 {}
impl ::std::default::Default for COMPACT_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for COMPACT_VIRTUAL_DISK_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for COMPACT_VIRTUAL_DISK_PARAMETERS_0 {}
unsafe impl ::windows::runtime::Abi for COMPACT_VIRTUAL_DISK_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct COMPACT_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub Reserved: u32,
}
impl COMPACT_VIRTUAL_DISK_PARAMETERS_0_0 {}
impl ::std::default::Default for COMPACT_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COMPACT_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version1_e__Struct")
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for COMPACT_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for COMPACT_VIRTUAL_DISK_PARAMETERS_0_0 {}
unsafe impl ::windows::runtime::Abi for COMPACT_VIRTUAL_DISK_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct COMPACT_VIRTUAL_DISK_VERSION(pub i32);
pub const COMPACT_VIRTUAL_DISK_VERSION_UNSPECIFIED: COMPACT_VIRTUAL_DISK_VERSION =
    COMPACT_VIRTUAL_DISK_VERSION(0i32);
pub const COMPACT_VIRTUAL_DISK_VERSION_1: COMPACT_VIRTUAL_DISK_VERSION =
    COMPACT_VIRTUAL_DISK_VERSION(1i32);
impl ::std::convert::From<i32> for COMPACT_VIRTUAL_DISK_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMPACT_VIRTUAL_DISK_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CREATE_VIRTUAL_DISK_FLAG(pub u32);
pub const CREATE_VIRTUAL_DISK_FLAG_NONE: CREATE_VIRTUAL_DISK_FLAG = CREATE_VIRTUAL_DISK_FLAG(0u32);
pub const CREATE_VIRTUAL_DISK_FLAG_FULL_PHYSICAL_ALLOCATION: CREATE_VIRTUAL_DISK_FLAG =
    CREATE_VIRTUAL_DISK_FLAG(1u32);
pub const CREATE_VIRTUAL_DISK_FLAG_PREVENT_WRITES_TO_SOURCE_DISK: CREATE_VIRTUAL_DISK_FLAG =
    CREATE_VIRTUAL_DISK_FLAG(2u32);
pub const CREATE_VIRTUAL_DISK_FLAG_DO_NOT_COPY_METADATA_FROM_PARENT: CREATE_VIRTUAL_DISK_FLAG =
    CREATE_VIRTUAL_DISK_FLAG(4u32);
pub const CREATE_VIRTUAL_DISK_FLAG_CREATE_BACKING_STORAGE: CREATE_VIRTUAL_DISK_FLAG =
    CREATE_VIRTUAL_DISK_FLAG(8u32);
pub const CREATE_VIRTUAL_DISK_FLAG_USE_CHANGE_TRACKING_SOURCE_LIMIT: CREATE_VIRTUAL_DISK_FLAG =
    CREATE_VIRTUAL_DISK_FLAG(16u32);
pub const CREATE_VIRTUAL_DISK_FLAG_PRESERVE_PARENT_CHANGE_TRACKING_STATE: CREATE_VIRTUAL_DISK_FLAG =
    CREATE_VIRTUAL_DISK_FLAG(32u32);
pub const CREATE_VIRTUAL_DISK_FLAG_VHD_SET_USE_ORIGINAL_BACKING_STORAGE: CREATE_VIRTUAL_DISK_FLAG =
    CREATE_VIRTUAL_DISK_FLAG(64u32);
pub const CREATE_VIRTUAL_DISK_FLAG_SPARSE_FILE: CREATE_VIRTUAL_DISK_FLAG =
    CREATE_VIRTUAL_DISK_FLAG(128u32);
pub const CREATE_VIRTUAL_DISK_FLAG_PMEM_COMPATIBLE: CREATE_VIRTUAL_DISK_FLAG =
    CREATE_VIRTUAL_DISK_FLAG(256u32);
pub const CREATE_VIRTUAL_DISK_FLAG_SUPPORT_COMPRESSED_VOLUMES: CREATE_VIRTUAL_DISK_FLAG =
    CREATE_VIRTUAL_DISK_FLAG(512u32);
pub const CREATE_VIRTUAL_DISK_FLAG_SUPPORT_SPARSE_FILES_ANY_FS: CREATE_VIRTUAL_DISK_FLAG =
    CREATE_VIRTUAL_DISK_FLAG(1024u32);
impl ::std::convert::From<u32> for CREATE_VIRTUAL_DISK_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CREATE_VIRTUAL_DISK_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CREATE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CREATE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CREATE_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CREATE_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CREATE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS {
    pub Version: CREATE_VIRTUAL_DISK_VERSION,
    pub Anonymous: CREATE_VIRTUAL_DISK_PARAMETERS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl CREATE_VIRTUAL_DISK_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CREATE_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CREATE_VIRTUAL_DISK_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CREATE_VIRTUAL_DISK_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CREATE_VIRTUAL_DISK_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union CREATE_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: CREATE_VIRTUAL_DISK_PARAMETERS_0_0,
    pub Version2: CREATE_VIRTUAL_DISK_PARAMETERS_0_1,
    pub Version3: CREATE_VIRTUAL_DISK_PARAMETERS_0_2,
    pub Version4: CREATE_VIRTUAL_DISK_PARAMETERS_0_3,
}
#[cfg(feature = "Win32_Foundation")]
impl CREATE_VIRTUAL_DISK_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CREATE_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CREATE_VIRTUAL_DISK_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CREATE_VIRTUAL_DISK_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CREATE_VIRTUAL_DISK_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub UniqueId: ::windows::runtime::GUID,
    pub MaximumSize: u64,
    pub BlockSizeInBytes: u32,
    pub SectorSizeInBytes: u32,
    pub ParentPath: super::super::Foundation::PWSTR,
    pub SourcePath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl CREATE_VIRTUAL_DISK_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CREATE_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CREATE_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version1_e__Struct")
            .field("UniqueId", &self.UniqueId)
            .field("MaximumSize", &self.MaximumSize)
            .field("BlockSizeInBytes", &self.BlockSizeInBytes)
            .field("SectorSizeInBytes", &self.SectorSizeInBytes)
            .field("ParentPath", &self.ParentPath)
            .field("SourcePath", &self.SourcePath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CREATE_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.UniqueId == other.UniqueId
            && self.MaximumSize == other.MaximumSize
            && self.BlockSizeInBytes == other.BlockSizeInBytes
            && self.SectorSizeInBytes == other.SectorSizeInBytes
            && self.ParentPath == other.ParentPath
            && self.SourcePath == other.SourcePath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CREATE_VIRTUAL_DISK_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CREATE_VIRTUAL_DISK_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS_0_1 {
    pub UniqueId: ::windows::runtime::GUID,
    pub MaximumSize: u64,
    pub BlockSizeInBytes: u32,
    pub SectorSizeInBytes: u32,
    pub PhysicalSectorSizeInBytes: u32,
    pub ParentPath: super::super::Foundation::PWSTR,
    pub SourcePath: super::super::Foundation::PWSTR,
    pub OpenFlags: OPEN_VIRTUAL_DISK_FLAG,
    pub ParentVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub SourceVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub ResiliencyGuid: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl CREATE_VIRTUAL_DISK_PARAMETERS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CREATE_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CREATE_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version2_e__Struct")
            .field("UniqueId", &self.UniqueId)
            .field("MaximumSize", &self.MaximumSize)
            .field("BlockSizeInBytes", &self.BlockSizeInBytes)
            .field("SectorSizeInBytes", &self.SectorSizeInBytes)
            .field("PhysicalSectorSizeInBytes", &self.PhysicalSectorSizeInBytes)
            .field("ParentPath", &self.ParentPath)
            .field("SourcePath", &self.SourcePath)
            .field("OpenFlags", &self.OpenFlags)
            .field("ParentVirtualStorageType", &self.ParentVirtualStorageType)
            .field("SourceVirtualStorageType", &self.SourceVirtualStorageType)
            .field("ResiliencyGuid", &self.ResiliencyGuid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CREATE_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.UniqueId == other.UniqueId
            && self.MaximumSize == other.MaximumSize
            && self.BlockSizeInBytes == other.BlockSizeInBytes
            && self.SectorSizeInBytes == other.SectorSizeInBytes
            && self.PhysicalSectorSizeInBytes == other.PhysicalSectorSizeInBytes
            && self.ParentPath == other.ParentPath
            && self.SourcePath == other.SourcePath
            && self.OpenFlags == other.OpenFlags
            && self.ParentVirtualStorageType == other.ParentVirtualStorageType
            && self.SourceVirtualStorageType == other.SourceVirtualStorageType
            && self.ResiliencyGuid == other.ResiliencyGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CREATE_VIRTUAL_DISK_PARAMETERS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CREATE_VIRTUAL_DISK_PARAMETERS_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS_0_2 {
    pub UniqueId: ::windows::runtime::GUID,
    pub MaximumSize: u64,
    pub BlockSizeInBytes: u32,
    pub SectorSizeInBytes: u32,
    pub PhysicalSectorSizeInBytes: u32,
    pub ParentPath: super::super::Foundation::PWSTR,
    pub SourcePath: super::super::Foundation::PWSTR,
    pub OpenFlags: OPEN_VIRTUAL_DISK_FLAG,
    pub ParentVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub SourceVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub ResiliencyGuid: ::windows::runtime::GUID,
    pub SourceLimitPath: super::super::Foundation::PWSTR,
    pub BackingStorageType: VIRTUAL_STORAGE_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl CREATE_VIRTUAL_DISK_PARAMETERS_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CREATE_VIRTUAL_DISK_PARAMETERS_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CREATE_VIRTUAL_DISK_PARAMETERS_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version3_e__Struct")
            .field("UniqueId", &self.UniqueId)
            .field("MaximumSize", &self.MaximumSize)
            .field("BlockSizeInBytes", &self.BlockSizeInBytes)
            .field("SectorSizeInBytes", &self.SectorSizeInBytes)
            .field("PhysicalSectorSizeInBytes", &self.PhysicalSectorSizeInBytes)
            .field("ParentPath", &self.ParentPath)
            .field("SourcePath", &self.SourcePath)
            .field("OpenFlags", &self.OpenFlags)
            .field("ParentVirtualStorageType", &self.ParentVirtualStorageType)
            .field("SourceVirtualStorageType", &self.SourceVirtualStorageType)
            .field("ResiliencyGuid", &self.ResiliencyGuid)
            .field("SourceLimitPath", &self.SourceLimitPath)
            .field("BackingStorageType", &self.BackingStorageType)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CREATE_VIRTUAL_DISK_PARAMETERS_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.UniqueId == other.UniqueId
            && self.MaximumSize == other.MaximumSize
            && self.BlockSizeInBytes == other.BlockSizeInBytes
            && self.SectorSizeInBytes == other.SectorSizeInBytes
            && self.PhysicalSectorSizeInBytes == other.PhysicalSectorSizeInBytes
            && self.ParentPath == other.ParentPath
            && self.SourcePath == other.SourcePath
            && self.OpenFlags == other.OpenFlags
            && self.ParentVirtualStorageType == other.ParentVirtualStorageType
            && self.SourceVirtualStorageType == other.SourceVirtualStorageType
            && self.ResiliencyGuid == other.ResiliencyGuid
            && self.SourceLimitPath == other.SourceLimitPath
            && self.BackingStorageType == other.BackingStorageType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CREATE_VIRTUAL_DISK_PARAMETERS_0_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CREATE_VIRTUAL_DISK_PARAMETERS_0_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CREATE_VIRTUAL_DISK_PARAMETERS_0_3 {
    pub UniqueId: ::windows::runtime::GUID,
    pub MaximumSize: u64,
    pub BlockSizeInBytes: u32,
    pub SectorSizeInBytes: u32,
    pub PhysicalSectorSizeInBytes: u32,
    pub ParentPath: super::super::Foundation::PWSTR,
    pub SourcePath: super::super::Foundation::PWSTR,
    pub OpenFlags: OPEN_VIRTUAL_DISK_FLAG,
    pub ParentVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub SourceVirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub ResiliencyGuid: ::windows::runtime::GUID,
    pub SourceLimitPath: super::super::Foundation::PWSTR,
    pub BackingStorageType: VIRTUAL_STORAGE_TYPE,
    pub PmemAddressAbstractionType: ::windows::runtime::GUID,
    pub DataAlignment: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl CREATE_VIRTUAL_DISK_PARAMETERS_0_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CREATE_VIRTUAL_DISK_PARAMETERS_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CREATE_VIRTUAL_DISK_PARAMETERS_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version4_e__Struct")
            .field("UniqueId", &self.UniqueId)
            .field("MaximumSize", &self.MaximumSize)
            .field("BlockSizeInBytes", &self.BlockSizeInBytes)
            .field("SectorSizeInBytes", &self.SectorSizeInBytes)
            .field("PhysicalSectorSizeInBytes", &self.PhysicalSectorSizeInBytes)
            .field("ParentPath", &self.ParentPath)
            .field("SourcePath", &self.SourcePath)
            .field("OpenFlags", &self.OpenFlags)
            .field("ParentVirtualStorageType", &self.ParentVirtualStorageType)
            .field("SourceVirtualStorageType", &self.SourceVirtualStorageType)
            .field("ResiliencyGuid", &self.ResiliencyGuid)
            .field("SourceLimitPath", &self.SourceLimitPath)
            .field("BackingStorageType", &self.BackingStorageType)
            .field(
                "PmemAddressAbstractionType",
                &self.PmemAddressAbstractionType,
            )
            .field("DataAlignment", &self.DataAlignment)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CREATE_VIRTUAL_DISK_PARAMETERS_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.UniqueId == other.UniqueId
            && self.MaximumSize == other.MaximumSize
            && self.BlockSizeInBytes == other.BlockSizeInBytes
            && self.SectorSizeInBytes == other.SectorSizeInBytes
            && self.PhysicalSectorSizeInBytes == other.PhysicalSectorSizeInBytes
            && self.ParentPath == other.ParentPath
            && self.SourcePath == other.SourcePath
            && self.OpenFlags == other.OpenFlags
            && self.ParentVirtualStorageType == other.ParentVirtualStorageType
            && self.SourceVirtualStorageType == other.SourceVirtualStorageType
            && self.ResiliencyGuid == other.ResiliencyGuid
            && self.SourceLimitPath == other.SourceLimitPath
            && self.BackingStorageType == other.BackingStorageType
            && self.PmemAddressAbstractionType == other.PmemAddressAbstractionType
            && self.DataAlignment == other.DataAlignment
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CREATE_VIRTUAL_DISK_PARAMETERS_0_3 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CREATE_VIRTUAL_DISK_PARAMETERS_0_3 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const CREATE_VIRTUAL_DISK_PARAMETERS_DEFAULT_BLOCK_SIZE: u32 = 0u32;
pub const CREATE_VIRTUAL_DISK_PARAMETERS_DEFAULT_SECTOR_SIZE: u32 = 0u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct CREATE_VIRTUAL_DISK_VERSION(pub i32);
pub const CREATE_VIRTUAL_DISK_VERSION_UNSPECIFIED: CREATE_VIRTUAL_DISK_VERSION =
    CREATE_VIRTUAL_DISK_VERSION(0i32);
pub const CREATE_VIRTUAL_DISK_VERSION_1: CREATE_VIRTUAL_DISK_VERSION =
    CREATE_VIRTUAL_DISK_VERSION(1i32);
pub const CREATE_VIRTUAL_DISK_VERSION_2: CREATE_VIRTUAL_DISK_VERSION =
    CREATE_VIRTUAL_DISK_VERSION(2i32);
pub const CREATE_VIRTUAL_DISK_VERSION_3: CREATE_VIRTUAL_DISK_VERSION =
    CREATE_VIRTUAL_DISK_VERSION(3i32);
pub const CREATE_VIRTUAL_DISK_VERSION_4: CREATE_VIRTUAL_DISK_VERSION =
    CREATE_VIRTUAL_DISK_VERSION(4i32);
impl ::std::convert::From<i32> for CREATE_VIRTUAL_DISK_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CREATE_VIRTUAL_DISK_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn CompactVirtualDisk<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    flags: COMPACT_VIRTUAL_DISK_FLAG,
    parameters: *const COMPACT_VIRTUAL_DISK_PARAMETERS,
    overlapped: *const super::super::System::SystemServices::OVERLAPPED,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CompactVirtualDisk(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                flags: COMPACT_VIRTUAL_DISK_FLAG,
                parameters: *const COMPACT_VIRTUAL_DISK_PARAMETERS,
                overlapped: *const super::super::System::SystemServices::OVERLAPPED,
            ) -> u32;
        }
        ::std::mem::transmute(CompactVirtualDisk(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(parameters),
            ::std::mem::transmute(overlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CompleteForkVirtualDisk<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CompleteForkVirtualDisk(virtualdiskhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(CompleteForkVirtualDisk(
            virtualdiskhandle.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Foundation",
    feature = "Win32_Security",
    feature = "Win32_System_SystemServices"
))]
#[inline]
pub unsafe fn CreateVirtualDisk<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    virtualstoragetype: *const VIRTUAL_STORAGE_TYPE,
    path: Param1,
    virtualdiskaccessmask: VIRTUAL_DISK_ACCESS_MASK,
    securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR,
    flags: CREATE_VIRTUAL_DISK_FLAG,
    providerspecificflags: u32,
    parameters: *const CREATE_VIRTUAL_DISK_PARAMETERS,
    overlapped: *const super::super::System::SystemServices::OVERLAPPED,
    handle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateVirtualDisk(
                virtualstoragetype: *const VIRTUAL_STORAGE_TYPE,
                path: super::super::Foundation::PWSTR,
                virtualdiskaccessmask: VIRTUAL_DISK_ACCESS_MASK,
                securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR,
                flags: CREATE_VIRTUAL_DISK_FLAG,
                providerspecificflags: u32,
                parameters: *const CREATE_VIRTUAL_DISK_PARAMETERS,
                overlapped: *const super::super::System::SystemServices::OVERLAPPED,
                handle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(CreateVirtualDisk(
            ::std::mem::transmute(virtualstoragetype),
            path.into_param().abi(),
            ::std::mem::transmute(virtualdiskaccessmask),
            ::std::mem::transmute(securitydescriptor),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(providerspecificflags),
            ::std::mem::transmute(parameters),
            ::std::mem::transmute(overlapped),
            ::std::mem::transmute(handle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DELETE_SNAPSHOT_VHDSET_FLAG(pub u32);
pub const DELETE_SNAPSHOT_VHDSET_FLAG_NONE: DELETE_SNAPSHOT_VHDSET_FLAG =
    DELETE_SNAPSHOT_VHDSET_FLAG(0u32);
pub const DELETE_SNAPSHOT_VHDSET_FLAG_PERSIST_RCT: DELETE_SNAPSHOT_VHDSET_FLAG =
    DELETE_SNAPSHOT_VHDSET_FLAG(1u32);
impl ::std::convert::From<u32> for DELETE_SNAPSHOT_VHDSET_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DELETE_SNAPSHOT_VHDSET_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DELETE_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DELETE_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DELETE_SNAPSHOT_VHDSET_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DELETE_SNAPSHOT_VHDSET_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DELETE_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DELETE_SNAPSHOT_VHDSET_PARAMETERS {
    pub Version: DELETE_SNAPSHOT_VHDSET_VERSION,
    pub Anonymous: DELETE_SNAPSHOT_VHDSET_PARAMETERS_0,
}
impl DELETE_SNAPSHOT_VHDSET_PARAMETERS {}
impl ::std::default::Default for DELETE_SNAPSHOT_VHDSET_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DELETE_SNAPSHOT_VHDSET_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DELETE_SNAPSHOT_VHDSET_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for DELETE_SNAPSHOT_VHDSET_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union DELETE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    pub Version1: DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0,
}
impl DELETE_SNAPSHOT_VHDSET_PARAMETERS_0 {}
impl ::std::default::Default for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0 {}
unsafe impl ::windows::runtime::Abi for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    pub SnapshotId: ::windows::runtime::GUID,
}
impl DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {}
impl ::std::default::Default for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version1_e__Struct")
            .field("SnapshotId", &self.SnapshotId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.SnapshotId == other.SnapshotId
    }
}
impl ::std::cmp::Eq for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {}
unsafe impl ::windows::runtime::Abi for DELETE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DELETE_SNAPSHOT_VHDSET_VERSION(pub i32);
pub const DELETE_SNAPSHOT_VHDSET_VERSION_UNSPECIFIED: DELETE_SNAPSHOT_VHDSET_VERSION =
    DELETE_SNAPSHOT_VHDSET_VERSION(0i32);
pub const DELETE_SNAPSHOT_VHDSET_VERSION_1: DELETE_SNAPSHOT_VHDSET_VERSION =
    DELETE_SNAPSHOT_VHDSET_VERSION(1i32);
impl ::std::convert::From<i32> for DELETE_SNAPSHOT_VHDSET_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DELETE_SNAPSHOT_VHDSET_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DEPENDENT_DISK_FLAG(pub u32);
pub const DEPENDENT_DISK_FLAG_NONE: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(0u32);
pub const DEPENDENT_DISK_FLAG_MULT_BACKING_FILES: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(1u32);
pub const DEPENDENT_DISK_FLAG_FULLY_ALLOCATED: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(2u32);
pub const DEPENDENT_DISK_FLAG_READ_ONLY: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(4u32);
pub const DEPENDENT_DISK_FLAG_REMOTE: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(8u32);
pub const DEPENDENT_DISK_FLAG_SYSTEM_VOLUME: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(16u32);
pub const DEPENDENT_DISK_FLAG_SYSTEM_VOLUME_PARENT: DEPENDENT_DISK_FLAG =
    DEPENDENT_DISK_FLAG(32u32);
pub const DEPENDENT_DISK_FLAG_REMOVABLE: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(64u32);
pub const DEPENDENT_DISK_FLAG_NO_DRIVE_LETTER: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(128u32);
pub const DEPENDENT_DISK_FLAG_PARENT: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(256u32);
pub const DEPENDENT_DISK_FLAG_NO_HOST_DISK: DEPENDENT_DISK_FLAG = DEPENDENT_DISK_FLAG(512u32);
pub const DEPENDENT_DISK_FLAG_PERMANENT_LIFETIME: DEPENDENT_DISK_FLAG =
    DEPENDENT_DISK_FLAG(1024u32);
pub const DEPENDENT_DISK_FLAG_SUPPORT_COMPRESSED_VOLUMES: DEPENDENT_DISK_FLAG =
    DEPENDENT_DISK_FLAG(2048u32);
pub const DEPENDENT_DISK_FLAG_ALWAYS_ALLOW_SPARSE: DEPENDENT_DISK_FLAG =
    DEPENDENT_DISK_FLAG(4096u32);
pub const DEPENDENT_DISK_FLAG_SUPPORT_ENCRYPTED_FILES: DEPENDENT_DISK_FLAG =
    DEPENDENT_DISK_FLAG(8192u32);
impl ::std::convert::From<u32> for DEPENDENT_DISK_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DEPENDENT_DISK_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DEPENDENT_DISK_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DEPENDENT_DISK_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DEPENDENT_DISK_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DEPENDENT_DISK_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DEPENDENT_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DETACH_VIRTUAL_DISK_FLAG(pub u32);
pub const DETACH_VIRTUAL_DISK_FLAG_NONE: DETACH_VIRTUAL_DISK_FLAG = DETACH_VIRTUAL_DISK_FLAG(0u32);
impl ::std::convert::From<u32> for DETACH_VIRTUAL_DISK_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DETACH_VIRTUAL_DISK_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DETACH_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DETACH_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DETACH_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DETACH_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DETACH_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteSnapshotVhdSet<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    parameters: *const DELETE_SNAPSHOT_VHDSET_PARAMETERS,
    flags: DELETE_SNAPSHOT_VHDSET_FLAG,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteSnapshotVhdSet(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                parameters: *const DELETE_SNAPSHOT_VHDSET_PARAMETERS,
                flags: DELETE_SNAPSHOT_VHDSET_FLAG,
            ) -> u32;
        }
        ::std::mem::transmute(DeleteSnapshotVhdSet(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(parameters),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteVirtualDiskMetadata<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    item: *const ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteVirtualDiskMetadata(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                item: *const ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(DeleteVirtualDiskMetadata(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(item),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DetachVirtualDisk<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    flags: DETACH_VIRTUAL_DISK_FLAG,
    providerspecificflags: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DetachVirtualDisk(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                flags: DETACH_VIRTUAL_DISK_FLAG,
                providerspecificflags: u32,
            ) -> u32;
        }
        ::std::mem::transmute(DetachVirtualDisk(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(providerspecificflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EXPAND_VIRTUAL_DISK_FLAG(pub u32);
pub const EXPAND_VIRTUAL_DISK_FLAG_NONE: EXPAND_VIRTUAL_DISK_FLAG = EXPAND_VIRTUAL_DISK_FLAG(0u32);
pub const EXPAND_VIRTUAL_DISK_FLAG_NOTIFY_CHANGE: EXPAND_VIRTUAL_DISK_FLAG =
    EXPAND_VIRTUAL_DISK_FLAG(1u32);
impl ::std::convert::From<u32> for EXPAND_VIRTUAL_DISK_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EXPAND_VIRTUAL_DISK_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for EXPAND_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for EXPAND_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for EXPAND_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for EXPAND_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for EXPAND_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EXPAND_VIRTUAL_DISK_PARAMETERS {
    pub Version: EXPAND_VIRTUAL_DISK_VERSION,
    pub Anonymous: EXPAND_VIRTUAL_DISK_PARAMETERS_0,
}
impl EXPAND_VIRTUAL_DISK_PARAMETERS {}
impl ::std::default::Default for EXPAND_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EXPAND_VIRTUAL_DISK_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EXPAND_VIRTUAL_DISK_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for EXPAND_VIRTUAL_DISK_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union EXPAND_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: EXPAND_VIRTUAL_DISK_PARAMETERS_0_0,
}
impl EXPAND_VIRTUAL_DISK_PARAMETERS_0 {}
impl ::std::default::Default for EXPAND_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for EXPAND_VIRTUAL_DISK_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for EXPAND_VIRTUAL_DISK_PARAMETERS_0 {}
unsafe impl ::windows::runtime::Abi for EXPAND_VIRTUAL_DISK_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct EXPAND_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub NewSize: u64,
}
impl EXPAND_VIRTUAL_DISK_PARAMETERS_0_0 {}
impl ::std::default::Default for EXPAND_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EXPAND_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version1_e__Struct")
            .field("NewSize", &self.NewSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for EXPAND_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.NewSize == other.NewSize
    }
}
impl ::std::cmp::Eq for EXPAND_VIRTUAL_DISK_PARAMETERS_0_0 {}
unsafe impl ::windows::runtime::Abi for EXPAND_VIRTUAL_DISK_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct EXPAND_VIRTUAL_DISK_VERSION(pub i32);
pub const EXPAND_VIRTUAL_DISK_VERSION_UNSPECIFIED: EXPAND_VIRTUAL_DISK_VERSION =
    EXPAND_VIRTUAL_DISK_VERSION(0i32);
pub const EXPAND_VIRTUAL_DISK_VERSION_1: EXPAND_VIRTUAL_DISK_VERSION =
    EXPAND_VIRTUAL_DISK_VERSION(1i32);
impl ::std::convert::From<i32> for EXPAND_VIRTUAL_DISK_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EXPAND_VIRTUAL_DISK_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnumerateVirtualDiskMetadata<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    numberofitems: *mut u32,
    items: *mut ::windows::runtime::GUID,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumerateVirtualDiskMetadata(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                numberofitems: *mut u32,
                items: *mut ::windows::runtime::GUID,
            ) -> u32;
        }
        ::std::mem::transmute(EnumerateVirtualDiskMetadata(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(numberofitems),
            ::std::mem::transmute(items),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn ExpandVirtualDisk<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    flags: EXPAND_VIRTUAL_DISK_FLAG,
    parameters: *const EXPAND_VIRTUAL_DISK_PARAMETERS,
    overlapped: *const super::super::System::SystemServices::OVERLAPPED,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExpandVirtualDisk(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                flags: EXPAND_VIRTUAL_DISK_FLAG,
                parameters: *const EXPAND_VIRTUAL_DISK_PARAMETERS,
                overlapped: *const super::super::System::SystemServices::OVERLAPPED,
            ) -> u32;
        }
        ::std::mem::transmute(ExpandVirtualDisk(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(parameters),
            ::std::mem::transmute(overlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FORK_VIRTUAL_DISK_FLAG(pub u32);
pub const FORK_VIRTUAL_DISK_FLAG_NONE: FORK_VIRTUAL_DISK_FLAG = FORK_VIRTUAL_DISK_FLAG(0u32);
pub const FORK_VIRTUAL_DISK_FLAG_EXISTING_FILE: FORK_VIRTUAL_DISK_FLAG =
    FORK_VIRTUAL_DISK_FLAG(1u32);
impl ::std::convert::From<u32> for FORK_VIRTUAL_DISK_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FORK_VIRTUAL_DISK_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for FORK_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for FORK_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for FORK_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for FORK_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for FORK_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FORK_VIRTUAL_DISK_PARAMETERS {
    pub Version: FORK_VIRTUAL_DISK_VERSION,
    pub Anonymous: FORK_VIRTUAL_DISK_PARAMETERS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl FORK_VIRTUAL_DISK_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FORK_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FORK_VIRTUAL_DISK_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FORK_VIRTUAL_DISK_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FORK_VIRTUAL_DISK_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union FORK_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: FORK_VIRTUAL_DISK_PARAMETERS_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl FORK_VIRTUAL_DISK_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FORK_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FORK_VIRTUAL_DISK_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FORK_VIRTUAL_DISK_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FORK_VIRTUAL_DISK_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FORK_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub ForkedVirtualDiskPath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl FORK_VIRTUAL_DISK_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for FORK_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for FORK_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version1_e__Struct")
            .field("ForkedVirtualDiskPath", &self.ForkedVirtualDiskPath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for FORK_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ForkedVirtualDiskPath == other.ForkedVirtualDiskPath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for FORK_VIRTUAL_DISK_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for FORK_VIRTUAL_DISK_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct FORK_VIRTUAL_DISK_VERSION(pub i32);
pub const FORK_VIRTUAL_DISK_VERSION_UNSPECIFIED: FORK_VIRTUAL_DISK_VERSION =
    FORK_VIRTUAL_DISK_VERSION(0i32);
pub const FORK_VIRTUAL_DISK_VERSION_1: FORK_VIRTUAL_DISK_VERSION = FORK_VIRTUAL_DISK_VERSION(1i32);
impl ::std::convert::From<i32> for FORK_VIRTUAL_DISK_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FORK_VIRTUAL_DISK_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn ForkVirtualDisk<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    flags: FORK_VIRTUAL_DISK_FLAG,
    parameters: *const FORK_VIRTUAL_DISK_PARAMETERS,
    overlapped: *mut super::super::System::SystemServices::OVERLAPPED,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ForkVirtualDisk(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                flags: FORK_VIRTUAL_DISK_FLAG,
                parameters: *const FORK_VIRTUAL_DISK_PARAMETERS,
                overlapped: *mut super::super::System::SystemServices::OVERLAPPED,
            ) -> u32;
        }
        ::std::mem::transmute(ForkVirtualDisk(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(parameters),
            ::std::mem::transmute(overlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GET_STORAGE_DEPENDENCY_FLAG(pub u32);
pub const GET_STORAGE_DEPENDENCY_FLAG_NONE: GET_STORAGE_DEPENDENCY_FLAG =
    GET_STORAGE_DEPENDENCY_FLAG(0u32);
pub const GET_STORAGE_DEPENDENCY_FLAG_HOST_VOLUMES: GET_STORAGE_DEPENDENCY_FLAG =
    GET_STORAGE_DEPENDENCY_FLAG(1u32);
pub const GET_STORAGE_DEPENDENCY_FLAG_DISK_HANDLE: GET_STORAGE_DEPENDENCY_FLAG =
    GET_STORAGE_DEPENDENCY_FLAG(2u32);
impl ::std::convert::From<u32> for GET_STORAGE_DEPENDENCY_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GET_STORAGE_DEPENDENCY_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for GET_STORAGE_DEPENDENCY_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for GET_STORAGE_DEPENDENCY_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for GET_STORAGE_DEPENDENCY_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for GET_STORAGE_DEPENDENCY_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for GET_STORAGE_DEPENDENCY_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GET_VIRTUAL_DISK_INFO {
    pub Version: GET_VIRTUAL_DISK_INFO_VERSION,
    pub Anonymous: GET_VIRTUAL_DISK_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl GET_VIRTUAL_DISK_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GET_VIRTUAL_DISK_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GET_VIRTUAL_DISK_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GET_VIRTUAL_DISK_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GET_VIRTUAL_DISK_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union GET_VIRTUAL_DISK_INFO_0 {
    pub Size: GET_VIRTUAL_DISK_INFO_0_3,
    pub Identifier: ::windows::runtime::GUID,
    pub ParentLocation: GET_VIRTUAL_DISK_INFO_0_1,
    pub ParentIdentifier: ::windows::runtime::GUID,
    pub ParentTimestamp: u32,
    pub VirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub ProviderSubtype: u32,
    pub Is4kAligned: super::super::Foundation::BOOL,
    pub IsLoaded: super::super::Foundation::BOOL,
    pub PhysicalDisk: GET_VIRTUAL_DISK_INFO_0_2,
    pub VhdPhysicalSectorSize: u32,
    pub SmallestSafeVirtualSize: u64,
    pub FragmentationPercentage: u32,
    pub VirtualDiskId: ::windows::runtime::GUID,
    pub ChangeTrackingState: GET_VIRTUAL_DISK_INFO_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl GET_VIRTUAL_DISK_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GET_VIRTUAL_DISK_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GET_VIRTUAL_DISK_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GET_VIRTUAL_DISK_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GET_VIRTUAL_DISK_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GET_VIRTUAL_DISK_INFO_0_0 {
    pub Enabled: super::super::Foundation::BOOL,
    pub NewerChanges: super::super::Foundation::BOOL,
    pub MostRecentId: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl GET_VIRTUAL_DISK_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GET_VIRTUAL_DISK_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GET_VIRTUAL_DISK_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ChangeTrackingState_e__Struct")
            .field("Enabled", &self.Enabled)
            .field("NewerChanges", &self.NewerChanges)
            .field("MostRecentId", &self.MostRecentId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GET_VIRTUAL_DISK_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Enabled == other.Enabled
            && self.NewerChanges == other.NewerChanges
            && self.MostRecentId == other.MostRecentId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GET_VIRTUAL_DISK_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GET_VIRTUAL_DISK_INFO_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GET_VIRTUAL_DISK_INFO_0_1 {
    pub ParentResolved: super::super::Foundation::BOOL,
    pub ParentLocationBuffer: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl GET_VIRTUAL_DISK_INFO_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GET_VIRTUAL_DISK_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GET_VIRTUAL_DISK_INFO_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ParentLocation_e__Struct")
            .field("ParentResolved", &self.ParentResolved)
            .field("ParentLocationBuffer", &self.ParentLocationBuffer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GET_VIRTUAL_DISK_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.ParentResolved == other.ParentResolved
            && self.ParentLocationBuffer == other.ParentLocationBuffer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GET_VIRTUAL_DISK_INFO_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GET_VIRTUAL_DISK_INFO_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct GET_VIRTUAL_DISK_INFO_0_2 {
    pub LogicalSectorSize: u32,
    pub PhysicalSectorSize: u32,
    pub IsRemote: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl GET_VIRTUAL_DISK_INFO_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for GET_VIRTUAL_DISK_INFO_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for GET_VIRTUAL_DISK_INFO_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_PhysicalDisk_e__Struct")
            .field("LogicalSectorSize", &self.LogicalSectorSize)
            .field("PhysicalSectorSize", &self.PhysicalSectorSize)
            .field("IsRemote", &self.IsRemote)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for GET_VIRTUAL_DISK_INFO_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.LogicalSectorSize == other.LogicalSectorSize
            && self.PhysicalSectorSize == other.PhysicalSectorSize
            && self.IsRemote == other.IsRemote
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for GET_VIRTUAL_DISK_INFO_0_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for GET_VIRTUAL_DISK_INFO_0_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GET_VIRTUAL_DISK_INFO_0_3 {
    pub VirtualSize: u64,
    pub PhysicalSize: u64,
    pub BlockSize: u32,
    pub SectorSize: u32,
}
impl GET_VIRTUAL_DISK_INFO_0_3 {}
impl ::std::default::Default for GET_VIRTUAL_DISK_INFO_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GET_VIRTUAL_DISK_INFO_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Size_e__Struct")
            .field("VirtualSize", &self.VirtualSize)
            .field("PhysicalSize", &self.PhysicalSize)
            .field("BlockSize", &self.BlockSize)
            .field("SectorSize", &self.SectorSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for GET_VIRTUAL_DISK_INFO_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualSize == other.VirtualSize
            && self.PhysicalSize == other.PhysicalSize
            && self.BlockSize == other.BlockSize
            && self.SectorSize == other.SectorSize
    }
}
impl ::std::cmp::Eq for GET_VIRTUAL_DISK_INFO_0_3 {}
unsafe impl ::windows::runtime::Abi for GET_VIRTUAL_DISK_INFO_0_3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GET_VIRTUAL_DISK_INFO_VERSION(pub i32);
pub const GET_VIRTUAL_DISK_INFO_UNSPECIFIED: GET_VIRTUAL_DISK_INFO_VERSION =
    GET_VIRTUAL_DISK_INFO_VERSION(0i32);
pub const GET_VIRTUAL_DISK_INFO_SIZE: GET_VIRTUAL_DISK_INFO_VERSION =
    GET_VIRTUAL_DISK_INFO_VERSION(1i32);
pub const GET_VIRTUAL_DISK_INFO_IDENTIFIER: GET_VIRTUAL_DISK_INFO_VERSION =
    GET_VIRTUAL_DISK_INFO_VERSION(2i32);
pub const GET_VIRTUAL_DISK_INFO_PARENT_LOCATION: GET_VIRTUAL_DISK_INFO_VERSION =
    GET_VIRTUAL_DISK_INFO_VERSION(3i32);
pub const GET_VIRTUAL_DISK_INFO_PARENT_IDENTIFIER: GET_VIRTUAL_DISK_INFO_VERSION =
    GET_VIRTUAL_DISK_INFO_VERSION(4i32);
pub const GET_VIRTUAL_DISK_INFO_PARENT_TIMESTAMP: GET_VIRTUAL_DISK_INFO_VERSION =
    GET_VIRTUAL_DISK_INFO_VERSION(5i32);
pub const GET_VIRTUAL_DISK_INFO_VIRTUAL_STORAGE_TYPE: GET_VIRTUAL_DISK_INFO_VERSION =
    GET_VIRTUAL_DISK_INFO_VERSION(6i32);
pub const GET_VIRTUAL_DISK_INFO_PROVIDER_SUBTYPE: GET_VIRTUAL_DISK_INFO_VERSION =
    GET_VIRTUAL_DISK_INFO_VERSION(7i32);
pub const GET_VIRTUAL_DISK_INFO_IS_4K_ALIGNED: GET_VIRTUAL_DISK_INFO_VERSION =
    GET_VIRTUAL_DISK_INFO_VERSION(8i32);
pub const GET_VIRTUAL_DISK_INFO_PHYSICAL_DISK: GET_VIRTUAL_DISK_INFO_VERSION =
    GET_VIRTUAL_DISK_INFO_VERSION(9i32);
pub const GET_VIRTUAL_DISK_INFO_VHD_PHYSICAL_SECTOR_SIZE: GET_VIRTUAL_DISK_INFO_VERSION =
    GET_VIRTUAL_DISK_INFO_VERSION(10i32);
pub const GET_VIRTUAL_DISK_INFO_SMALLEST_SAFE_VIRTUAL_SIZE: GET_VIRTUAL_DISK_INFO_VERSION =
    GET_VIRTUAL_DISK_INFO_VERSION(11i32);
pub const GET_VIRTUAL_DISK_INFO_FRAGMENTATION: GET_VIRTUAL_DISK_INFO_VERSION =
    GET_VIRTUAL_DISK_INFO_VERSION(12i32);
pub const GET_VIRTUAL_DISK_INFO_IS_LOADED: GET_VIRTUAL_DISK_INFO_VERSION =
    GET_VIRTUAL_DISK_INFO_VERSION(13i32);
pub const GET_VIRTUAL_DISK_INFO_VIRTUAL_DISK_ID: GET_VIRTUAL_DISK_INFO_VERSION =
    GET_VIRTUAL_DISK_INFO_VERSION(14i32);
pub const GET_VIRTUAL_DISK_INFO_CHANGE_TRACKING_STATE: GET_VIRTUAL_DISK_INFO_VERSION =
    GET_VIRTUAL_DISK_INFO_VERSION(15i32);
impl ::std::convert::From<i32> for GET_VIRTUAL_DISK_INFO_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GET_VIRTUAL_DISK_INFO_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAllAttachedVirtualDiskPhysicalPaths(
    pathsbuffersizeinbytes: *mut u32,
    pathsbuffer: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAllAttachedVirtualDiskPhysicalPaths(
                pathsbuffersizeinbytes: *mut u32,
                pathsbuffer: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetAllAttachedVirtualDiskPhysicalPaths(
            ::std::mem::transmute(pathsbuffersizeinbytes),
            ::std::mem::transmute(pathsbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStorageDependencyInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    objecthandle: Param0,
    flags: GET_STORAGE_DEPENDENCY_FLAG,
    storagedependencyinfosize: u32,
    storagedependencyinfo: *mut STORAGE_DEPENDENCY_INFO,
    sizeused: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStorageDependencyInformation(
                objecthandle: super::super::Foundation::HANDLE,
                flags: GET_STORAGE_DEPENDENCY_FLAG,
                storagedependencyinfosize: u32,
                storagedependencyinfo: *mut STORAGE_DEPENDENCY_INFO,
                sizeused: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetStorageDependencyInformation(
            objecthandle.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(storagedependencyinfosize),
            ::std::mem::transmute(storagedependencyinfo),
            ::std::mem::transmute(sizeused),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVirtualDiskInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    virtualdiskinfosize: *mut u32,
    virtualdiskinfo: *mut GET_VIRTUAL_DISK_INFO,
    sizeused: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVirtualDiskInformation(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                virtualdiskinfosize: *mut u32,
                virtualdiskinfo: *mut GET_VIRTUAL_DISK_INFO,
                sizeused: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetVirtualDiskInformation(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(virtualdiskinfosize),
            ::std::mem::transmute(virtualdiskinfo),
            ::std::mem::transmute(sizeused),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVirtualDiskMetadata<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    item: *const ::windows::runtime::GUID,
    metadatasize: *mut u32,
    metadata: *mut ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVirtualDiskMetadata(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                item: *const ::windows::runtime::GUID,
                metadatasize: *mut u32,
                metadata: *mut ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(GetVirtualDiskMetadata(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(item),
            ::std::mem::transmute(metadatasize),
            ::std::mem::transmute(metadata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn GetVirtualDiskOperationProgress<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    overlapped: *const super::super::System::SystemServices::OVERLAPPED,
    progress: *mut VIRTUAL_DISK_PROGRESS,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVirtualDiskOperationProgress(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                overlapped: *const super::super::System::SystemServices::OVERLAPPED,
                progress: *mut VIRTUAL_DISK_PROGRESS,
            ) -> u32;
        }
        ::std::mem::transmute(GetVirtualDiskOperationProgress(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(overlapped),
            ::std::mem::transmute(progress),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetVirtualDiskPhysicalPath<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    diskpathsizeinbytes: *mut u32,
    diskpath: super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetVirtualDiskPhysicalPath(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                diskpathsizeinbytes: *mut u32,
                diskpath: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(GetVirtualDiskPhysicalPath(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(diskpathsizeinbytes),
            ::std::mem::transmute(diskpath),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MERGE_VIRTUAL_DISK_DEFAULT_MERGE_DEPTH: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MERGE_VIRTUAL_DISK_FLAG(pub u32);
pub const MERGE_VIRTUAL_DISK_FLAG_NONE: MERGE_VIRTUAL_DISK_FLAG = MERGE_VIRTUAL_DISK_FLAG(0u32);
impl ::std::convert::From<u32> for MERGE_VIRTUAL_DISK_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MERGE_VIRTUAL_DISK_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MERGE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MERGE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MERGE_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MERGE_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MERGE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MERGE_VIRTUAL_DISK_PARAMETERS {
    pub Version: MERGE_VIRTUAL_DISK_VERSION,
    pub Anonymous: MERGE_VIRTUAL_DISK_PARAMETERS_0,
}
impl MERGE_VIRTUAL_DISK_PARAMETERS {}
impl ::std::default::Default for MERGE_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MERGE_VIRTUAL_DISK_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MERGE_VIRTUAL_DISK_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for MERGE_VIRTUAL_DISK_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union MERGE_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: MERGE_VIRTUAL_DISK_PARAMETERS_0_0,
    pub Version2: MERGE_VIRTUAL_DISK_PARAMETERS_0_1,
}
impl MERGE_VIRTUAL_DISK_PARAMETERS_0 {}
impl ::std::default::Default for MERGE_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MERGE_VIRTUAL_DISK_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MERGE_VIRTUAL_DISK_PARAMETERS_0 {}
unsafe impl ::windows::runtime::Abi for MERGE_VIRTUAL_DISK_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MERGE_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub MergeDepth: u32,
}
impl MERGE_VIRTUAL_DISK_PARAMETERS_0_0 {}
impl ::std::default::Default for MERGE_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MERGE_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version1_e__Struct")
            .field("MergeDepth", &self.MergeDepth)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MERGE_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.MergeDepth == other.MergeDepth
    }
}
impl ::std::cmp::Eq for MERGE_VIRTUAL_DISK_PARAMETERS_0_0 {}
unsafe impl ::windows::runtime::Abi for MERGE_VIRTUAL_DISK_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MERGE_VIRTUAL_DISK_PARAMETERS_0_1 {
    pub MergeSourceDepth: u32,
    pub MergeTargetDepth: u32,
}
impl MERGE_VIRTUAL_DISK_PARAMETERS_0_1 {}
impl ::std::default::Default for MERGE_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MERGE_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version2_e__Struct")
            .field("MergeSourceDepth", &self.MergeSourceDepth)
            .field("MergeTargetDepth", &self.MergeTargetDepth)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MERGE_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.MergeSourceDepth == other.MergeSourceDepth
            && self.MergeTargetDepth == other.MergeTargetDepth
    }
}
impl ::std::cmp::Eq for MERGE_VIRTUAL_DISK_PARAMETERS_0_1 {}
unsafe impl ::windows::runtime::Abi for MERGE_VIRTUAL_DISK_PARAMETERS_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MERGE_VIRTUAL_DISK_VERSION(pub i32);
pub const MERGE_VIRTUAL_DISK_VERSION_UNSPECIFIED: MERGE_VIRTUAL_DISK_VERSION =
    MERGE_VIRTUAL_DISK_VERSION(0i32);
pub const MERGE_VIRTUAL_DISK_VERSION_1: MERGE_VIRTUAL_DISK_VERSION =
    MERGE_VIRTUAL_DISK_VERSION(1i32);
pub const MERGE_VIRTUAL_DISK_VERSION_2: MERGE_VIRTUAL_DISK_VERSION =
    MERGE_VIRTUAL_DISK_VERSION(2i32);
impl ::std::convert::From<i32> for MERGE_VIRTUAL_DISK_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MERGE_VIRTUAL_DISK_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MIRROR_VIRTUAL_DISK_FLAG(pub u32);
pub const MIRROR_VIRTUAL_DISK_FLAG_NONE: MIRROR_VIRTUAL_DISK_FLAG = MIRROR_VIRTUAL_DISK_FLAG(0u32);
pub const MIRROR_VIRTUAL_DISK_FLAG_EXISTING_FILE: MIRROR_VIRTUAL_DISK_FLAG =
    MIRROR_VIRTUAL_DISK_FLAG(1u32);
pub const MIRROR_VIRTUAL_DISK_FLAG_SKIP_MIRROR_ACTIVATION: MIRROR_VIRTUAL_DISK_FLAG =
    MIRROR_VIRTUAL_DISK_FLAG(2u32);
pub const MIRROR_VIRTUAL_DISK_FLAG_ENABLE_SMB_COMPRESSION: MIRROR_VIRTUAL_DISK_FLAG =
    MIRROR_VIRTUAL_DISK_FLAG(4u32);
pub const MIRROR_VIRTUAL_DISK_FLAG_IS_LIVE_MIGRATION: MIRROR_VIRTUAL_DISK_FLAG =
    MIRROR_VIRTUAL_DISK_FLAG(8u32);
impl ::std::convert::From<u32> for MIRROR_VIRTUAL_DISK_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MIRROR_VIRTUAL_DISK_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MIRROR_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MIRROR_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MIRROR_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MIRROR_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MIRROR_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MIRROR_VIRTUAL_DISK_PARAMETERS {
    pub Version: MIRROR_VIRTUAL_DISK_VERSION,
    pub Anonymous: MIRROR_VIRTUAL_DISK_PARAMETERS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl MIRROR_VIRTUAL_DISK_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIRROR_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIRROR_VIRTUAL_DISK_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIRROR_VIRTUAL_DISK_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIRROR_VIRTUAL_DISK_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union MIRROR_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: MIRROR_VIRTUAL_DISK_PARAMETERS_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl MIRROR_VIRTUAL_DISK_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIRROR_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIRROR_VIRTUAL_DISK_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIRROR_VIRTUAL_DISK_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIRROR_VIRTUAL_DISK_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MIRROR_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub MirrorVirtualDiskPath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MIRROR_VIRTUAL_DISK_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MIRROR_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MIRROR_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version1_e__Struct")
            .field("MirrorVirtualDiskPath", &self.MirrorVirtualDiskPath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MIRROR_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.MirrorVirtualDiskPath == other.MirrorVirtualDiskPath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MIRROR_VIRTUAL_DISK_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MIRROR_VIRTUAL_DISK_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MIRROR_VIRTUAL_DISK_VERSION(pub i32);
pub const MIRROR_VIRTUAL_DISK_VERSION_UNSPECIFIED: MIRROR_VIRTUAL_DISK_VERSION =
    MIRROR_VIRTUAL_DISK_VERSION(0i32);
pub const MIRROR_VIRTUAL_DISK_VERSION_1: MIRROR_VIRTUAL_DISK_VERSION =
    MIRROR_VIRTUAL_DISK_VERSION(1i32);
impl ::std::convert::From<i32> for MIRROR_VIRTUAL_DISK_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MIRROR_VIRTUAL_DISK_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MODIFY_VHDSET_FLAG(pub u32);
pub const MODIFY_VHDSET_FLAG_NONE: MODIFY_VHDSET_FLAG = MODIFY_VHDSET_FLAG(0u32);
pub const MODIFY_VHDSET_FLAG_WRITEABLE_SNAPSHOT: MODIFY_VHDSET_FLAG = MODIFY_VHDSET_FLAG(1u32);
impl ::std::convert::From<u32> for MODIFY_VHDSET_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MODIFY_VHDSET_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MODIFY_VHDSET_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MODIFY_VHDSET_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MODIFY_VHDSET_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MODIFY_VHDSET_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MODIFY_VHDSET_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MODIFY_VHDSET_PARAMETERS {
    pub Version: MODIFY_VHDSET_VERSION,
    pub Anonymous: MODIFY_VHDSET_PARAMETERS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl MODIFY_VHDSET_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MODIFY_VHDSET_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MODIFY_VHDSET_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MODIFY_VHDSET_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MODIFY_VHDSET_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union MODIFY_VHDSET_PARAMETERS_0 {
    pub SnapshotPath: MODIFY_VHDSET_PARAMETERS_0_0,
    pub SnapshotId: ::windows::runtime::GUID,
    pub DefaultFilePath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MODIFY_VHDSET_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MODIFY_VHDSET_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MODIFY_VHDSET_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MODIFY_VHDSET_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MODIFY_VHDSET_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MODIFY_VHDSET_PARAMETERS_0_0 {
    pub SnapshotId: ::windows::runtime::GUID,
    pub SnapshotFilePath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MODIFY_VHDSET_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MODIFY_VHDSET_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MODIFY_VHDSET_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_SnapshotPath_e__Struct")
            .field("SnapshotId", &self.SnapshotId)
            .field("SnapshotFilePath", &self.SnapshotFilePath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MODIFY_VHDSET_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.SnapshotId == other.SnapshotId && self.SnapshotFilePath == other.SnapshotFilePath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MODIFY_VHDSET_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MODIFY_VHDSET_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct MODIFY_VHDSET_VERSION(pub i32);
pub const MODIFY_VHDSET_UNSPECIFIED: MODIFY_VHDSET_VERSION = MODIFY_VHDSET_VERSION(0i32);
pub const MODIFY_VHDSET_SNAPSHOT_PATH: MODIFY_VHDSET_VERSION = MODIFY_VHDSET_VERSION(1i32);
pub const MODIFY_VHDSET_REMOVE_SNAPSHOT: MODIFY_VHDSET_VERSION = MODIFY_VHDSET_VERSION(2i32);
pub const MODIFY_VHDSET_DEFAULT_SNAPSHOT_PATH: MODIFY_VHDSET_VERSION = MODIFY_VHDSET_VERSION(3i32);
impl ::std::convert::From<i32> for MODIFY_VHDSET_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MODIFY_VHDSET_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn MergeVirtualDisk<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    flags: MERGE_VIRTUAL_DISK_FLAG,
    parameters: *const MERGE_VIRTUAL_DISK_PARAMETERS,
    overlapped: *const super::super::System::SystemServices::OVERLAPPED,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MergeVirtualDisk(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                flags: MERGE_VIRTUAL_DISK_FLAG,
                parameters: *const MERGE_VIRTUAL_DISK_PARAMETERS,
                overlapped: *const super::super::System::SystemServices::OVERLAPPED,
            ) -> u32;
        }
        ::std::mem::transmute(MergeVirtualDisk(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(parameters),
            ::std::mem::transmute(overlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn MirrorVirtualDisk<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    flags: MIRROR_VIRTUAL_DISK_FLAG,
    parameters: *const MIRROR_VIRTUAL_DISK_PARAMETERS,
    overlapped: *const super::super::System::SystemServices::OVERLAPPED,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MirrorVirtualDisk(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                flags: MIRROR_VIRTUAL_DISK_FLAG,
                parameters: *const MIRROR_VIRTUAL_DISK_PARAMETERS,
                overlapped: *const super::super::System::SystemServices::OVERLAPPED,
            ) -> u32;
        }
        ::std::mem::transmute(MirrorVirtualDisk(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(parameters),
            ::std::mem::transmute(overlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ModifyVhdSet<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    parameters: *const MODIFY_VHDSET_PARAMETERS,
    flags: MODIFY_VHDSET_FLAG,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ModifyVhdSet(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                parameters: *const MODIFY_VHDSET_PARAMETERS,
                flags: MODIFY_VHDSET_FLAG,
            ) -> u32;
        }
        ::std::mem::transmute(ModifyVhdSet(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(parameters),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct OPEN_VIRTUAL_DISK_FLAG(pub u32);
pub const OPEN_VIRTUAL_DISK_FLAG_NONE: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(0u32);
pub const OPEN_VIRTUAL_DISK_FLAG_NO_PARENTS: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(1u32);
pub const OPEN_VIRTUAL_DISK_FLAG_BLANK_FILE: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(2u32);
pub const OPEN_VIRTUAL_DISK_FLAG_BOOT_DRIVE: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(4u32);
pub const OPEN_VIRTUAL_DISK_FLAG_CACHED_IO: OPEN_VIRTUAL_DISK_FLAG = OPEN_VIRTUAL_DISK_FLAG(8u32);
pub const OPEN_VIRTUAL_DISK_FLAG_CUSTOM_DIFF_CHAIN: OPEN_VIRTUAL_DISK_FLAG =
    OPEN_VIRTUAL_DISK_FLAG(16u32);
pub const OPEN_VIRTUAL_DISK_FLAG_PARENT_CACHED_IO: OPEN_VIRTUAL_DISK_FLAG =
    OPEN_VIRTUAL_DISK_FLAG(32u32);
pub const OPEN_VIRTUAL_DISK_FLAG_VHDSET_FILE_ONLY: OPEN_VIRTUAL_DISK_FLAG =
    OPEN_VIRTUAL_DISK_FLAG(64u32);
pub const OPEN_VIRTUAL_DISK_FLAG_IGNORE_RELATIVE_PARENT_LOCATOR: OPEN_VIRTUAL_DISK_FLAG =
    OPEN_VIRTUAL_DISK_FLAG(128u32);
pub const OPEN_VIRTUAL_DISK_FLAG_NO_WRITE_HARDENING: OPEN_VIRTUAL_DISK_FLAG =
    OPEN_VIRTUAL_DISK_FLAG(256u32);
pub const OPEN_VIRTUAL_DISK_FLAG_SUPPORT_COMPRESSED_VOLUMES: OPEN_VIRTUAL_DISK_FLAG =
    OPEN_VIRTUAL_DISK_FLAG(512u32);
pub const OPEN_VIRTUAL_DISK_FLAG_SUPPORT_SPARSE_FILES_ANY_FS: OPEN_VIRTUAL_DISK_FLAG =
    OPEN_VIRTUAL_DISK_FLAG(1024u32);
pub const OPEN_VIRTUAL_DISK_FLAG_SUPPORT_ENCRYPTED_FILES: OPEN_VIRTUAL_DISK_FLAG =
    OPEN_VIRTUAL_DISK_FLAG(2048u32);
impl ::std::convert::From<u32> for OPEN_VIRTUAL_DISK_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPEN_VIRTUAL_DISK_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for OPEN_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for OPEN_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for OPEN_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for OPEN_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for OPEN_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OPEN_VIRTUAL_DISK_PARAMETERS {
    pub Version: OPEN_VIRTUAL_DISK_VERSION,
    pub Anonymous: OPEN_VIRTUAL_DISK_PARAMETERS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl OPEN_VIRTUAL_DISK_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OPEN_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OPEN_VIRTUAL_DISK_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OPEN_VIRTUAL_DISK_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OPEN_VIRTUAL_DISK_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union OPEN_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: OPEN_VIRTUAL_DISK_PARAMETERS_0_0,
    pub Version2: OPEN_VIRTUAL_DISK_PARAMETERS_0_1,
    pub Version3: OPEN_VIRTUAL_DISK_PARAMETERS_0_2,
}
#[cfg(feature = "Win32_Foundation")]
impl OPEN_VIRTUAL_DISK_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OPEN_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OPEN_VIRTUAL_DISK_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OPEN_VIRTUAL_DISK_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OPEN_VIRTUAL_DISK_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct OPEN_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub RWDepth: u32,
}
impl OPEN_VIRTUAL_DISK_PARAMETERS_0_0 {}
impl ::std::default::Default for OPEN_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for OPEN_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version1_e__Struct")
            .field("RWDepth", &self.RWDepth)
            .finish()
    }
}
impl ::std::cmp::PartialEq for OPEN_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.RWDepth == other.RWDepth
    }
}
impl ::std::cmp::Eq for OPEN_VIRTUAL_DISK_PARAMETERS_0_0 {}
unsafe impl ::windows::runtime::Abi for OPEN_VIRTUAL_DISK_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OPEN_VIRTUAL_DISK_PARAMETERS_0_1 {
    pub GetInfoOnly: super::super::Foundation::BOOL,
    pub ReadOnly: super::super::Foundation::BOOL,
    pub ResiliencyGuid: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl OPEN_VIRTUAL_DISK_PARAMETERS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OPEN_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OPEN_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version2_e__Struct")
            .field("GetInfoOnly", &self.GetInfoOnly)
            .field("ReadOnly", &self.ReadOnly)
            .field("ResiliencyGuid", &self.ResiliencyGuid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OPEN_VIRTUAL_DISK_PARAMETERS_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.GetInfoOnly == other.GetInfoOnly
            && self.ReadOnly == other.ReadOnly
            && self.ResiliencyGuid == other.ResiliencyGuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OPEN_VIRTUAL_DISK_PARAMETERS_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OPEN_VIRTUAL_DISK_PARAMETERS_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct OPEN_VIRTUAL_DISK_PARAMETERS_0_2 {
    pub GetInfoOnly: super::super::Foundation::BOOL,
    pub ReadOnly: super::super::Foundation::BOOL,
    pub ResiliencyGuid: ::windows::runtime::GUID,
    pub SnapshotId: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl OPEN_VIRTUAL_DISK_PARAMETERS_0_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for OPEN_VIRTUAL_DISK_PARAMETERS_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for OPEN_VIRTUAL_DISK_PARAMETERS_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version3_e__Struct")
            .field("GetInfoOnly", &self.GetInfoOnly)
            .field("ReadOnly", &self.ReadOnly)
            .field("ResiliencyGuid", &self.ResiliencyGuid)
            .field("SnapshotId", &self.SnapshotId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for OPEN_VIRTUAL_DISK_PARAMETERS_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.GetInfoOnly == other.GetInfoOnly
            && self.ReadOnly == other.ReadOnly
            && self.ResiliencyGuid == other.ResiliencyGuid
            && self.SnapshotId == other.SnapshotId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for OPEN_VIRTUAL_DISK_PARAMETERS_0_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for OPEN_VIRTUAL_DISK_PARAMETERS_0_2 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const OPEN_VIRTUAL_DISK_RW_DEPTH_DEFAULT: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct OPEN_VIRTUAL_DISK_VERSION(pub i32);
pub const OPEN_VIRTUAL_DISK_VERSION_UNSPECIFIED: OPEN_VIRTUAL_DISK_VERSION =
    OPEN_VIRTUAL_DISK_VERSION(0i32);
pub const OPEN_VIRTUAL_DISK_VERSION_1: OPEN_VIRTUAL_DISK_VERSION = OPEN_VIRTUAL_DISK_VERSION(1i32);
pub const OPEN_VIRTUAL_DISK_VERSION_2: OPEN_VIRTUAL_DISK_VERSION = OPEN_VIRTUAL_DISK_VERSION(2i32);
pub const OPEN_VIRTUAL_DISK_VERSION_3: OPEN_VIRTUAL_DISK_VERSION = OPEN_VIRTUAL_DISK_VERSION(3i32);
impl ::std::convert::From<i32> for OPEN_VIRTUAL_DISK_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for OPEN_VIRTUAL_DISK_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenVirtualDisk<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    virtualstoragetype: *const VIRTUAL_STORAGE_TYPE,
    path: Param1,
    virtualdiskaccessmask: VIRTUAL_DISK_ACCESS_MASK,
    flags: OPEN_VIRTUAL_DISK_FLAG,
    parameters: *const OPEN_VIRTUAL_DISK_PARAMETERS,
    handle: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenVirtualDisk(
                virtualstoragetype: *const VIRTUAL_STORAGE_TYPE,
                path: super::super::Foundation::PWSTR,
                virtualdiskaccessmask: VIRTUAL_DISK_ACCESS_MASK,
                flags: OPEN_VIRTUAL_DISK_FLAG,
                parameters: *const OPEN_VIRTUAL_DISK_PARAMETERS,
                handle: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(OpenVirtualDisk(
            ::std::mem::transmute(virtualstoragetype),
            path.into_param().abi(),
            ::std::mem::transmute(virtualdiskaccessmask),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(parameters),
            ::std::mem::transmute(handle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct QUERY_CHANGES_VIRTUAL_DISK_FLAG(pub u32);
pub const QUERY_CHANGES_VIRTUAL_DISK_FLAG_NONE: QUERY_CHANGES_VIRTUAL_DISK_FLAG =
    QUERY_CHANGES_VIRTUAL_DISK_FLAG(0u32);
impl ::std::convert::From<u32> for QUERY_CHANGES_VIRTUAL_DISK_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for QUERY_CHANGES_VIRTUAL_DISK_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for QUERY_CHANGES_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for QUERY_CHANGES_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for QUERY_CHANGES_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for QUERY_CHANGES_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for QUERY_CHANGES_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct QUERY_CHANGES_VIRTUAL_DISK_RANGE {
    pub ByteOffset: u64,
    pub ByteLength: u64,
    pub Reserved: u64,
}
impl QUERY_CHANGES_VIRTUAL_DISK_RANGE {}
impl ::std::default::Default for QUERY_CHANGES_VIRTUAL_DISK_RANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for QUERY_CHANGES_VIRTUAL_DISK_RANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("QUERY_CHANGES_VIRTUAL_DISK_RANGE")
            .field("ByteOffset", &self.ByteOffset)
            .field("ByteLength", &self.ByteLength)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for QUERY_CHANGES_VIRTUAL_DISK_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.ByteOffset == other.ByteOffset
            && self.ByteLength == other.ByteLength
            && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for QUERY_CHANGES_VIRTUAL_DISK_RANGE {}
unsafe impl ::windows::runtime::Abi for QUERY_CHANGES_VIRTUAL_DISK_RANGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryChangesVirtualDisk<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    virtualdiskhandle: Param0,
    changetrackingid: Param1,
    byteoffset: u64,
    bytelength: u64,
    flags: QUERY_CHANGES_VIRTUAL_DISK_FLAG,
    ranges: *mut QUERY_CHANGES_VIRTUAL_DISK_RANGE,
    rangecount: *mut u32,
    processedlength: *mut u64,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryChangesVirtualDisk(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                changetrackingid: super::super::Foundation::PWSTR,
                byteoffset: u64,
                bytelength: u64,
                flags: QUERY_CHANGES_VIRTUAL_DISK_FLAG,
                ranges: *mut QUERY_CHANGES_VIRTUAL_DISK_RANGE,
                rangecount: *mut u32,
                processedlength: *mut u64,
            ) -> u32;
        }
        ::std::mem::transmute(QueryChangesVirtualDisk(
            virtualdiskhandle.into_param().abi(),
            changetrackingid.into_param().abi(),
            ::std::mem::transmute(byteoffset),
            ::std::mem::transmute(bytelength),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(ranges),
            ::std::mem::transmute(rangecount),
            ::std::mem::transmute(processedlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RAW_SCSI_VIRTUAL_DISK_FLAG(pub u32);
pub const RAW_SCSI_VIRTUAL_DISK_FLAG_NONE: RAW_SCSI_VIRTUAL_DISK_FLAG =
    RAW_SCSI_VIRTUAL_DISK_FLAG(0u32);
impl ::std::convert::From<u32> for RAW_SCSI_VIRTUAL_DISK_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RAW_SCSI_VIRTUAL_DISK_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for RAW_SCSI_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RAW_SCSI_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RAW_SCSI_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RAW_SCSI_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RAW_SCSI_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RAW_SCSI_VIRTUAL_DISK_PARAMETERS {
    pub Version: RAW_SCSI_VIRTUAL_DISK_VERSION,
    pub Anonymous: RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0,
}
#[cfg(feature = "Win32_Foundation")]
impl RAW_SCSI_VIRTUAL_DISK_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAW_SCSI_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAW_SCSI_VIRTUAL_DISK_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAW_SCSI_VIRTUAL_DISK_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAW_SCSI_VIRTUAL_DISK_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub RSVDHandle: super::super::Foundation::BOOL,
    pub DataIn: u8,
    pub CdbLength: u8,
    pub SenseInfoLength: u8,
    pub SrbFlags: u32,
    pub DataTransferLength: u32,
    pub DataBuffer: *mut ::std::ffi::c_void,
    pub SenseInfo: *mut u8,
    pub Cdb: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version1_e__Struct")
            .field("RSVDHandle", &self.RSVDHandle)
            .field("DataIn", &self.DataIn)
            .field("CdbLength", &self.CdbLength)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("SrbFlags", &self.SrbFlags)
            .field("DataTransferLength", &self.DataTransferLength)
            .field("DataBuffer", &self.DataBuffer)
            .field("SenseInfo", &self.SenseInfo)
            .field("Cdb", &self.Cdb)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.RSVDHandle == other.RSVDHandle
            && self.DataIn == other.DataIn
            && self.CdbLength == other.CdbLength
            && self.SenseInfoLength == other.SenseInfoLength
            && self.SrbFlags == other.SrbFlags
            && self.DataTransferLength == other.DataTransferLength
            && self.DataBuffer == other.DataBuffer
            && self.SenseInfo == other.SenseInfo
            && self.Cdb == other.Cdb
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAW_SCSI_VIRTUAL_DISK_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RAW_SCSI_VIRTUAL_DISK_RESPONSE {
    pub Version: RAW_SCSI_VIRTUAL_DISK_VERSION,
    pub Anonymous: RAW_SCSI_VIRTUAL_DISK_RESPONSE_0,
}
impl RAW_SCSI_VIRTUAL_DISK_RESPONSE {}
impl ::std::default::Default for RAW_SCSI_VIRTUAL_DISK_RESPONSE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RAW_SCSI_VIRTUAL_DISK_RESPONSE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RAW_SCSI_VIRTUAL_DISK_RESPONSE {}
unsafe impl ::windows::runtime::Abi for RAW_SCSI_VIRTUAL_DISK_RESPONSE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union RAW_SCSI_VIRTUAL_DISK_RESPONSE_0 {
    pub Version1: RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0,
}
impl RAW_SCSI_VIRTUAL_DISK_RESPONSE_0 {}
impl ::std::default::Default for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0 {}
unsafe impl ::windows::runtime::Abi for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0 {
    pub ScsiStatus: u8,
    pub SenseInfoLength: u8,
    pub DataTransferLength: u32,
}
impl RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0 {}
impl ::std::default::Default for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version1_e__Struct")
            .field("ScsiStatus", &self.ScsiStatus)
            .field("SenseInfoLength", &self.SenseInfoLength)
            .field("DataTransferLength", &self.DataTransferLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ScsiStatus == other.ScsiStatus
            && self.SenseInfoLength == other.SenseInfoLength
            && self.DataTransferLength == other.DataTransferLength
    }
}
impl ::std::cmp::Eq for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0 {}
unsafe impl ::windows::runtime::Abi for RAW_SCSI_VIRTUAL_DISK_RESPONSE_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RAW_SCSI_VIRTUAL_DISK_VERSION(pub i32);
pub const RAW_SCSI_VIRTUAL_DISK_VERSION_UNSPECIFIED: RAW_SCSI_VIRTUAL_DISK_VERSION =
    RAW_SCSI_VIRTUAL_DISK_VERSION(0i32);
pub const RAW_SCSI_VIRTUAL_DISK_VERSION_1: RAW_SCSI_VIRTUAL_DISK_VERSION =
    RAW_SCSI_VIRTUAL_DISK_VERSION(1i32);
impl ::std::convert::From<i32> for RAW_SCSI_VIRTUAL_DISK_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RAW_SCSI_VIRTUAL_DISK_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RESIZE_VIRTUAL_DISK_FLAG(pub u32);
pub const RESIZE_VIRTUAL_DISK_FLAG_NONE: RESIZE_VIRTUAL_DISK_FLAG = RESIZE_VIRTUAL_DISK_FLAG(0u32);
pub const RESIZE_VIRTUAL_DISK_FLAG_ALLOW_UNSAFE_VIRTUAL_SIZE: RESIZE_VIRTUAL_DISK_FLAG =
    RESIZE_VIRTUAL_DISK_FLAG(1u32);
pub const RESIZE_VIRTUAL_DISK_FLAG_RESIZE_TO_SMALLEST_SAFE_VIRTUAL_SIZE: RESIZE_VIRTUAL_DISK_FLAG =
    RESIZE_VIRTUAL_DISK_FLAG(2u32);
impl ::std::convert::From<u32> for RESIZE_VIRTUAL_DISK_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RESIZE_VIRTUAL_DISK_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for RESIZE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RESIZE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RESIZE_VIRTUAL_DISK_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RESIZE_VIRTUAL_DISK_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RESIZE_VIRTUAL_DISK_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RESIZE_VIRTUAL_DISK_PARAMETERS {
    pub Version: RESIZE_VIRTUAL_DISK_VERSION,
    pub Anonymous: RESIZE_VIRTUAL_DISK_PARAMETERS_0,
}
impl RESIZE_VIRTUAL_DISK_PARAMETERS {}
impl ::std::default::Default for RESIZE_VIRTUAL_DISK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RESIZE_VIRTUAL_DISK_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RESIZE_VIRTUAL_DISK_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for RESIZE_VIRTUAL_DISK_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union RESIZE_VIRTUAL_DISK_PARAMETERS_0 {
    pub Version1: RESIZE_VIRTUAL_DISK_PARAMETERS_0_0,
}
impl RESIZE_VIRTUAL_DISK_PARAMETERS_0 {}
impl ::std::default::Default for RESIZE_VIRTUAL_DISK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RESIZE_VIRTUAL_DISK_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RESIZE_VIRTUAL_DISK_PARAMETERS_0 {}
unsafe impl ::windows::runtime::Abi for RESIZE_VIRTUAL_DISK_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RESIZE_VIRTUAL_DISK_PARAMETERS_0_0 {
    pub NewSize: u64,
}
impl RESIZE_VIRTUAL_DISK_PARAMETERS_0_0 {}
impl ::std::default::Default for RESIZE_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RESIZE_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version1_e__Struct")
            .field("NewSize", &self.NewSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RESIZE_VIRTUAL_DISK_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.NewSize == other.NewSize
    }
}
impl ::std::cmp::Eq for RESIZE_VIRTUAL_DISK_PARAMETERS_0_0 {}
unsafe impl ::windows::runtime::Abi for RESIZE_VIRTUAL_DISK_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RESIZE_VIRTUAL_DISK_VERSION(pub i32);
pub const RESIZE_VIRTUAL_DISK_VERSION_UNSPECIFIED: RESIZE_VIRTUAL_DISK_VERSION =
    RESIZE_VIRTUAL_DISK_VERSION(0i32);
pub const RESIZE_VIRTUAL_DISK_VERSION_1: RESIZE_VIRTUAL_DISK_VERSION =
    RESIZE_VIRTUAL_DISK_VERSION(1i32);
impl ::std::convert::From<i32> for RESIZE_VIRTUAL_DISK_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RESIZE_VIRTUAL_DISK_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RawSCSIVirtualDisk<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    parameters: *const RAW_SCSI_VIRTUAL_DISK_PARAMETERS,
    flags: RAW_SCSI_VIRTUAL_DISK_FLAG,
    response: *mut RAW_SCSI_VIRTUAL_DISK_RESPONSE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RawSCSIVirtualDisk(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                parameters: *const RAW_SCSI_VIRTUAL_DISK_PARAMETERS,
                flags: RAW_SCSI_VIRTUAL_DISK_FLAG,
                response: *mut RAW_SCSI_VIRTUAL_DISK_RESPONSE,
            ) -> u32;
        }
        ::std::mem::transmute(RawSCSIVirtualDisk(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(parameters),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(response),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn ResizeVirtualDisk<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    flags: RESIZE_VIRTUAL_DISK_FLAG,
    parameters: *const RESIZE_VIRTUAL_DISK_PARAMETERS,
    overlapped: *const super::super::System::SystemServices::OVERLAPPED,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResizeVirtualDisk(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                flags: RESIZE_VIRTUAL_DISK_FLAG,
                parameters: *const RESIZE_VIRTUAL_DISK_PARAMETERS,
                overlapped: *const super::super::System::SystemServices::OVERLAPPED,
            ) -> u32;
        }
        ::std::mem::transmute(ResizeVirtualDisk(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(flags),
            ::std::mem::transmute(parameters),
            ::std::mem::transmute(overlapped),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SET_VIRTUAL_DISK_INFO {
    pub Version: SET_VIRTUAL_DISK_INFO_VERSION,
    pub Anonymous: SET_VIRTUAL_DISK_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl SET_VIRTUAL_DISK_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SET_VIRTUAL_DISK_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SET_VIRTUAL_DISK_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SET_VIRTUAL_DISK_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SET_VIRTUAL_DISK_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union SET_VIRTUAL_DISK_INFO_0 {
    pub ParentFilePath: super::super::Foundation::PWSTR,
    pub UniqueIdentifier: ::windows::runtime::GUID,
    pub ParentPathWithDepthInfo: SET_VIRTUAL_DISK_INFO_0_1,
    pub VhdPhysicalSectorSize: u32,
    pub VirtualDiskId: ::windows::runtime::GUID,
    pub ChangeTrackingEnabled: super::super::Foundation::BOOL,
    pub ParentLocator: SET_VIRTUAL_DISK_INFO_0_0,
}
#[cfg(feature = "Win32_Foundation")]
impl SET_VIRTUAL_DISK_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SET_VIRTUAL_DISK_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SET_VIRTUAL_DISK_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SET_VIRTUAL_DISK_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SET_VIRTUAL_DISK_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SET_VIRTUAL_DISK_INFO_0_0 {
    pub LinkageId: ::windows::runtime::GUID,
    pub ParentFilePath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SET_VIRTUAL_DISK_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SET_VIRTUAL_DISK_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SET_VIRTUAL_DISK_INFO_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ParentLocator_e__Struct")
            .field("LinkageId", &self.LinkageId)
            .field("ParentFilePath", &self.ParentFilePath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SET_VIRTUAL_DISK_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.LinkageId == other.LinkageId && self.ParentFilePath == other.ParentFilePath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SET_VIRTUAL_DISK_INFO_0_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SET_VIRTUAL_DISK_INFO_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SET_VIRTUAL_DISK_INFO_0_1 {
    pub ChildDepth: u32,
    pub ParentFilePath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SET_VIRTUAL_DISK_INFO_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SET_VIRTUAL_DISK_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SET_VIRTUAL_DISK_INFO_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ParentPathWithDepthInfo_e__Struct")
            .field("ChildDepth", &self.ChildDepth)
            .field("ParentFilePath", &self.ParentFilePath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SET_VIRTUAL_DISK_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.ChildDepth == other.ChildDepth && self.ParentFilePath == other.ParentFilePath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SET_VIRTUAL_DISK_INFO_0_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SET_VIRTUAL_DISK_INFO_0_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct SET_VIRTUAL_DISK_INFO_VERSION(pub i32);
pub const SET_VIRTUAL_DISK_INFO_UNSPECIFIED: SET_VIRTUAL_DISK_INFO_VERSION =
    SET_VIRTUAL_DISK_INFO_VERSION(0i32);
pub const SET_VIRTUAL_DISK_INFO_PARENT_PATH: SET_VIRTUAL_DISK_INFO_VERSION =
    SET_VIRTUAL_DISK_INFO_VERSION(1i32);
pub const SET_VIRTUAL_DISK_INFO_IDENTIFIER: SET_VIRTUAL_DISK_INFO_VERSION =
    SET_VIRTUAL_DISK_INFO_VERSION(2i32);
pub const SET_VIRTUAL_DISK_INFO_PARENT_PATH_WITH_DEPTH: SET_VIRTUAL_DISK_INFO_VERSION =
    SET_VIRTUAL_DISK_INFO_VERSION(3i32);
pub const SET_VIRTUAL_DISK_INFO_PHYSICAL_SECTOR_SIZE: SET_VIRTUAL_DISK_INFO_VERSION =
    SET_VIRTUAL_DISK_INFO_VERSION(4i32);
pub const SET_VIRTUAL_DISK_INFO_VIRTUAL_DISK_ID: SET_VIRTUAL_DISK_INFO_VERSION =
    SET_VIRTUAL_DISK_INFO_VERSION(5i32);
pub const SET_VIRTUAL_DISK_INFO_CHANGE_TRACKING_STATE: SET_VIRTUAL_DISK_INFO_VERSION =
    SET_VIRTUAL_DISK_INFO_VERSION(6i32);
pub const SET_VIRTUAL_DISK_INFO_PARENT_LOCATOR: SET_VIRTUAL_DISK_INFO_VERSION =
    SET_VIRTUAL_DISK_INFO_VERSION(7i32);
impl ::std::convert::From<i32> for SET_VIRTUAL_DISK_INFO_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SET_VIRTUAL_DISK_INFO_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_DEPENDENCY_INFO {
    pub Version: STORAGE_DEPENDENCY_INFO_VERSION,
    pub NumberEntries: u32,
    pub Anonymous: STORAGE_DEPENDENCY_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl STORAGE_DEPENDENCY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STORAGE_DEPENDENCY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STORAGE_DEPENDENCY_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STORAGE_DEPENDENCY_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STORAGE_DEPENDENCY_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union STORAGE_DEPENDENCY_INFO_0 {
    pub Version1Entries: [STORAGE_DEPENDENCY_INFO_TYPE_1; 1],
    pub Version2Entries: [STORAGE_DEPENDENCY_INFO_TYPE_2; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl STORAGE_DEPENDENCY_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STORAGE_DEPENDENCY_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STORAGE_DEPENDENCY_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STORAGE_DEPENDENCY_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STORAGE_DEPENDENCY_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct STORAGE_DEPENDENCY_INFO_TYPE_1 {
    pub DependencyTypeFlags: DEPENDENT_DISK_FLAG,
    pub ProviderSpecificFlags: u32,
    pub VirtualStorageType: VIRTUAL_STORAGE_TYPE,
}
impl STORAGE_DEPENDENCY_INFO_TYPE_1 {}
impl ::std::default::Default for STORAGE_DEPENDENCY_INFO_TYPE_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STORAGE_DEPENDENCY_INFO_TYPE_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STORAGE_DEPENDENCY_INFO_TYPE_1")
            .field("DependencyTypeFlags", &self.DependencyTypeFlags)
            .field("ProviderSpecificFlags", &self.ProviderSpecificFlags)
            .field("VirtualStorageType", &self.VirtualStorageType)
            .finish()
    }
}
impl ::std::cmp::PartialEq for STORAGE_DEPENDENCY_INFO_TYPE_1 {
    fn eq(&self, other: &Self) -> bool {
        self.DependencyTypeFlags == other.DependencyTypeFlags
            && self.ProviderSpecificFlags == other.ProviderSpecificFlags
            && self.VirtualStorageType == other.VirtualStorageType
    }
}
impl ::std::cmp::Eq for STORAGE_DEPENDENCY_INFO_TYPE_1 {}
unsafe impl ::windows::runtime::Abi for STORAGE_DEPENDENCY_INFO_TYPE_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct STORAGE_DEPENDENCY_INFO_TYPE_2 {
    pub DependencyTypeFlags: DEPENDENT_DISK_FLAG,
    pub ProviderSpecificFlags: u32,
    pub VirtualStorageType: VIRTUAL_STORAGE_TYPE,
    pub AncestorLevel: u32,
    pub DependencyDeviceName: super::super::Foundation::PWSTR,
    pub HostVolumeName: super::super::Foundation::PWSTR,
    pub DependentVolumeName: super::super::Foundation::PWSTR,
    pub DependentVolumeRelativePath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl STORAGE_DEPENDENCY_INFO_TYPE_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for STORAGE_DEPENDENCY_INFO_TYPE_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for STORAGE_DEPENDENCY_INFO_TYPE_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STORAGE_DEPENDENCY_INFO_TYPE_2")
            .field("DependencyTypeFlags", &self.DependencyTypeFlags)
            .field("ProviderSpecificFlags", &self.ProviderSpecificFlags)
            .field("VirtualStorageType", &self.VirtualStorageType)
            .field("AncestorLevel", &self.AncestorLevel)
            .field("DependencyDeviceName", &self.DependencyDeviceName)
            .field("HostVolumeName", &self.HostVolumeName)
            .field("DependentVolumeName", &self.DependentVolumeName)
            .field(
                "DependentVolumeRelativePath",
                &self.DependentVolumeRelativePath,
            )
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for STORAGE_DEPENDENCY_INFO_TYPE_2 {
    fn eq(&self, other: &Self) -> bool {
        self.DependencyTypeFlags == other.DependencyTypeFlags
            && self.ProviderSpecificFlags == other.ProviderSpecificFlags
            && self.VirtualStorageType == other.VirtualStorageType
            && self.AncestorLevel == other.AncestorLevel
            && self.DependencyDeviceName == other.DependencyDeviceName
            && self.HostVolumeName == other.HostVolumeName
            && self.DependentVolumeName == other.DependentVolumeName
            && self.DependentVolumeRelativePath == other.DependentVolumeRelativePath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for STORAGE_DEPENDENCY_INFO_TYPE_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for STORAGE_DEPENDENCY_INFO_TYPE_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct STORAGE_DEPENDENCY_INFO_VERSION(pub i32);
pub const STORAGE_DEPENDENCY_INFO_VERSION_UNSPECIFIED: STORAGE_DEPENDENCY_INFO_VERSION =
    STORAGE_DEPENDENCY_INFO_VERSION(0i32);
pub const STORAGE_DEPENDENCY_INFO_VERSION_1: STORAGE_DEPENDENCY_INFO_VERSION =
    STORAGE_DEPENDENCY_INFO_VERSION(1i32);
pub const STORAGE_DEPENDENCY_INFO_VERSION_2: STORAGE_DEPENDENCY_INFO_VERSION =
    STORAGE_DEPENDENCY_INFO_VERSION(2i32);
impl ::std::convert::From<i32> for STORAGE_DEPENDENCY_INFO_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STORAGE_DEPENDENCY_INFO_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetVirtualDiskInformation<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    virtualdiskinfo: *const SET_VIRTUAL_DISK_INFO,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetVirtualDiskInformation(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                virtualdiskinfo: *const SET_VIRTUAL_DISK_INFO,
            ) -> u32;
        }
        ::std::mem::transmute(SetVirtualDiskInformation(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(virtualdiskinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetVirtualDiskMetadata<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    item: *const ::windows::runtime::GUID,
    metadatasize: u32,
    metadata: *const ::std::ffi::c_void,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetVirtualDiskMetadata(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                item: *const ::windows::runtime::GUID,
                metadatasize: u32,
                metadata: *const ::std::ffi::c_void,
            ) -> u32;
        }
        ::std::mem::transmute(SetVirtualDiskMetadata(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(item),
            ::std::mem::transmute(metadatasize),
            ::std::mem::transmute(metadata),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TAKE_SNAPSHOT_VHDSET_FLAG(pub u32);
pub const TAKE_SNAPSHOT_VHDSET_FLAG_NONE: TAKE_SNAPSHOT_VHDSET_FLAG =
    TAKE_SNAPSHOT_VHDSET_FLAG(0u32);
pub const TAKE_SNAPSHOT_VHDSET_FLAG_WRITEABLE: TAKE_SNAPSHOT_VHDSET_FLAG =
    TAKE_SNAPSHOT_VHDSET_FLAG(1u32);
impl ::std::convert::From<u32> for TAKE_SNAPSHOT_VHDSET_FLAG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TAKE_SNAPSHOT_VHDSET_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for TAKE_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TAKE_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TAKE_SNAPSHOT_VHDSET_FLAG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TAKE_SNAPSHOT_VHDSET_FLAG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TAKE_SNAPSHOT_VHDSET_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TAKE_SNAPSHOT_VHDSET_PARAMETERS {
    pub Version: TAKE_SNAPSHOT_VHDSET_VERSION,
    pub Anonymous: TAKE_SNAPSHOT_VHDSET_PARAMETERS_0,
}
impl TAKE_SNAPSHOT_VHDSET_PARAMETERS {}
impl ::std::default::Default for TAKE_SNAPSHOT_VHDSET_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TAKE_SNAPSHOT_VHDSET_PARAMETERS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TAKE_SNAPSHOT_VHDSET_PARAMETERS {}
unsafe impl ::windows::runtime::Abi for TAKE_SNAPSHOT_VHDSET_PARAMETERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union TAKE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    pub Version1: TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0,
}
impl TAKE_SNAPSHOT_VHDSET_PARAMETERS_0 {}
impl ::std::default::Default for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0 {}
unsafe impl ::windows::runtime::Abi for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    pub SnapshotId: ::windows::runtime::GUID,
}
impl TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {}
impl ::std::default::Default for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Version1_e__Struct")
            .field("SnapshotId", &self.SnapshotId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.SnapshotId == other.SnapshotId
    }
}
impl ::std::cmp::Eq for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {}
unsafe impl ::windows::runtime::Abi for TAKE_SNAPSHOT_VHDSET_PARAMETERS_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct TAKE_SNAPSHOT_VHDSET_VERSION(pub i32);
pub const TAKE_SNAPSHOT_VHDSET_VERSION_UNSPECIFIED: TAKE_SNAPSHOT_VHDSET_VERSION =
    TAKE_SNAPSHOT_VHDSET_VERSION(0i32);
pub const TAKE_SNAPSHOT_VHDSET_VERSION_1: TAKE_SNAPSHOT_VHDSET_VERSION =
    TAKE_SNAPSHOT_VHDSET_VERSION(1i32);
impl ::std::convert::From<i32> for TAKE_SNAPSHOT_VHDSET_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TAKE_SNAPSHOT_VHDSET_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TakeSnapshotVhdSet<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    virtualdiskhandle: Param0,
    parameters: *const TAKE_SNAPSHOT_VHDSET_PARAMETERS,
    flags: TAKE_SNAPSHOT_VHDSET_FLAG,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TakeSnapshotVhdSet(
                virtualdiskhandle: super::super::Foundation::HANDLE,
                parameters: *const TAKE_SNAPSHOT_VHDSET_PARAMETERS,
                flags: TAKE_SNAPSHOT_VHDSET_FLAG,
            ) -> u32;
        }
        ::std::mem::transmute(TakeSnapshotVhdSet(
            virtualdiskhandle.into_param().abi(),
            ::std::mem::transmute(parameters),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct VIRTUAL_DISK_ACCESS_MASK(pub u32);
pub const VIRTUAL_DISK_ACCESS_NONE: VIRTUAL_DISK_ACCESS_MASK = VIRTUAL_DISK_ACCESS_MASK(0u32);
pub const VIRTUAL_DISK_ACCESS_ATTACH_RO: VIRTUAL_DISK_ACCESS_MASK =
    VIRTUAL_DISK_ACCESS_MASK(65536u32);
pub const VIRTUAL_DISK_ACCESS_ATTACH_RW: VIRTUAL_DISK_ACCESS_MASK =
    VIRTUAL_DISK_ACCESS_MASK(131072u32);
pub const VIRTUAL_DISK_ACCESS_DETACH: VIRTUAL_DISK_ACCESS_MASK =
    VIRTUAL_DISK_ACCESS_MASK(262144u32);
pub const VIRTUAL_DISK_ACCESS_GET_INFO: VIRTUAL_DISK_ACCESS_MASK =
    VIRTUAL_DISK_ACCESS_MASK(524288u32);
pub const VIRTUAL_DISK_ACCESS_CREATE: VIRTUAL_DISK_ACCESS_MASK =
    VIRTUAL_DISK_ACCESS_MASK(1048576u32);
pub const VIRTUAL_DISK_ACCESS_METAOPS: VIRTUAL_DISK_ACCESS_MASK =
    VIRTUAL_DISK_ACCESS_MASK(2097152u32);
pub const VIRTUAL_DISK_ACCESS_READ: VIRTUAL_DISK_ACCESS_MASK = VIRTUAL_DISK_ACCESS_MASK(851968u32);
pub const VIRTUAL_DISK_ACCESS_ALL: VIRTUAL_DISK_ACCESS_MASK = VIRTUAL_DISK_ACCESS_MASK(4128768u32);
pub const VIRTUAL_DISK_ACCESS_WRITABLE: VIRTUAL_DISK_ACCESS_MASK =
    VIRTUAL_DISK_ACCESS_MASK(3276800u32);
impl ::std::convert::From<u32> for VIRTUAL_DISK_ACCESS_MASK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VIRTUAL_DISK_ACCESS_MASK {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for VIRTUAL_DISK_ACCESS_MASK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VIRTUAL_DISK_ACCESS_MASK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VIRTUAL_DISK_ACCESS_MASK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VIRTUAL_DISK_ACCESS_MASK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VIRTUAL_DISK_ACCESS_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const VIRTUAL_DISK_MAXIMUM_CHANGE_TRACKING_ID_LENGTH: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VIRTUAL_DISK_PROGRESS {
    pub OperationStatus: u32,
    pub CurrentValue: u64,
    pub CompletionValue: u64,
}
impl VIRTUAL_DISK_PROGRESS {}
impl ::std::default::Default for VIRTUAL_DISK_PROGRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIRTUAL_DISK_PROGRESS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIRTUAL_DISK_PROGRESS")
            .field("OperationStatus", &self.OperationStatus)
            .field("CurrentValue", &self.CurrentValue)
            .field("CompletionValue", &self.CompletionValue)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VIRTUAL_DISK_PROGRESS {
    fn eq(&self, other: &Self) -> bool {
        self.OperationStatus == other.OperationStatus
            && self.CurrentValue == other.CurrentValue
            && self.CompletionValue == other.CompletionValue
    }
}
impl ::std::cmp::Eq for VIRTUAL_DISK_PROGRESS {}
unsafe impl ::windows::runtime::Abi for VIRTUAL_DISK_PROGRESS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VIRTUAL_STORAGE_TYPE {
    pub DeviceId: u32,
    pub VendorId: ::windows::runtime::GUID,
}
impl VIRTUAL_STORAGE_TYPE {}
impl ::std::default::Default for VIRTUAL_STORAGE_TYPE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VIRTUAL_STORAGE_TYPE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VIRTUAL_STORAGE_TYPE")
            .field("DeviceId", &self.DeviceId)
            .field("VendorId", &self.VendorId)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VIRTUAL_STORAGE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceId == other.DeviceId && self.VendorId == other.VendorId
    }
}
impl ::std::cmp::Eq for VIRTUAL_STORAGE_TYPE {}
unsafe impl ::windows::runtime::Abi for VIRTUAL_STORAGE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VIRTUAL_STORAGE_TYPE_DEVICE_ISO: u32 = 1u32;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_UNKNOWN: u32 = 0u32;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_VHD: u32 = 2u32;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_VHDSET: u32 = 4u32;
pub const VIRTUAL_STORAGE_TYPE_DEVICE_VHDX: u32 = 3u32;
pub const VIRTUAL_STORAGE_TYPE_VENDOR_MICROSOFT: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        3969403628,
        41209,
        18409,
        [144, 31, 113, 65, 90, 102, 52, 91],
    );
pub const VIRTUAL_STORAGE_TYPE_VENDOR_UNKNOWN: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(0, 0, 0, [0, 0, 0, 0, 0, 0, 0, 0]);
