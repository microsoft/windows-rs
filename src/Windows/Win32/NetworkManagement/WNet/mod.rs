#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CONNECTDLGSTRUCTA {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpConnRes: *mut NETRESOURCEA,
    pub dwFlags: CONNECTDLGSTRUCT_FLAGS,
    pub dwDevNum: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CONNECTDLGSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CONNECTDLGSTRUCTA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CONNECTDLGSTRUCTA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONNECTDLGSTRUCTA")
            .field("cbStructure", &self.cbStructure)
            .field("hwndOwner", &self.hwndOwner)
            .field("lpConnRes", &self.lpConnRes)
            .field("dwFlags", &self.dwFlags)
            .field("dwDevNum", &self.dwDevNum)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CONNECTDLGSTRUCTA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStructure == other.cbStructure
            && self.hwndOwner == other.hwndOwner
            && self.lpConnRes == other.lpConnRes
            && self.dwFlags == other.dwFlags
            && self.dwDevNum == other.dwDevNum
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CONNECTDLGSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CONNECTDLGSTRUCTA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CONNECTDLGSTRUCTW {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpConnRes: *mut NETRESOURCEW,
    pub dwFlags: CONNECTDLGSTRUCT_FLAGS,
    pub dwDevNum: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl CONNECTDLGSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for CONNECTDLGSTRUCTW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for CONNECTDLGSTRUCTW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CONNECTDLGSTRUCTW")
            .field("cbStructure", &self.cbStructure)
            .field("hwndOwner", &self.hwndOwner)
            .field("lpConnRes", &self.lpConnRes)
            .field("dwFlags", &self.dwFlags)
            .field("dwDevNum", &self.dwDevNum)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for CONNECTDLGSTRUCTW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStructure == other.cbStructure
            && self.hwndOwner == other.hwndOwner
            && self.lpConnRes == other.lpConnRes
            && self.dwFlags == other.dwFlags
            && self.dwDevNum == other.dwDevNum
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for CONNECTDLGSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for CONNECTDLGSTRUCTW {
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
pub struct CONNECTDLGSTRUCT_FLAGS(pub u32);
pub const CONNDLG_RO_PATH: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(1u32);
pub const CONNDLG_CONN_POINT: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(2u32);
pub const CONNDLG_USE_MRU: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(4u32);
pub const CONNDLG_HIDE_BOX: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(8u32);
pub const CONNDLG_PERSIST: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(16u32);
pub const CONNDLG_NOT_PERSIST: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(32u32);
impl ::std::convert::From<u32> for CONNECTDLGSTRUCT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CONNECTDLGSTRUCT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for CONNECTDLGSTRUCT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for CONNECTDLGSTRUCT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for CONNECTDLGSTRUCT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for CONNECTDLGSTRUCT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for CONNECTDLGSTRUCT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const CONNECT_CRED_RESET: u32 = 8192u32;
pub const CONNECT_CURRENT_MEDIA: u32 = 512u32;
pub const CONNECT_GLOBAL_MAPPING: u32 = 262144u32;
pub const CONNECT_LOCALDRIVE: u32 = 256u32;
pub const CONNECT_NEED_DRIVE: u32 = 32u32;
pub const CONNECT_REFCOUNT: u32 = 64u32;
pub const CONNECT_REQUIRE_INTEGRITY: u32 = 16384u32;
pub const CONNECT_REQUIRE_PRIVACY: u32 = 32768u32;
pub const CONNECT_RESERVED: u32 = 4278190080u32;
pub const CONNECT_WRITE_THROUGH_SEMANTICS: u32 = 65536u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DISCDLGSTRUCTA {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpLocalName: super::super::Foundation::PSTR,
    pub lpRemoteName: super::super::Foundation::PSTR,
    pub dwFlags: DISCDLGSTRUCT_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl DISCDLGSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISCDLGSTRUCTA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DISCDLGSTRUCTA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DISCDLGSTRUCTA")
            .field("cbStructure", &self.cbStructure)
            .field("hwndOwner", &self.hwndOwner)
            .field("lpLocalName", &self.lpLocalName)
            .field("lpRemoteName", &self.lpRemoteName)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISCDLGSTRUCTA {
    fn eq(&self, other: &Self) -> bool {
        self.cbStructure == other.cbStructure
            && self.hwndOwner == other.hwndOwner
            && self.lpLocalName == other.lpLocalName
            && self.lpRemoteName == other.lpRemoteName
            && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISCDLGSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISCDLGSTRUCTA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DISCDLGSTRUCTW {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpLocalName: super::super::Foundation::PWSTR,
    pub lpRemoteName: super::super::Foundation::PWSTR,
    pub dwFlags: DISCDLGSTRUCT_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl DISCDLGSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for DISCDLGSTRUCTW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for DISCDLGSTRUCTW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DISCDLGSTRUCTW")
            .field("cbStructure", &self.cbStructure)
            .field("hwndOwner", &self.hwndOwner)
            .field("lpLocalName", &self.lpLocalName)
            .field("lpRemoteName", &self.lpRemoteName)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for DISCDLGSTRUCTW {
    fn eq(&self, other: &Self) -> bool {
        self.cbStructure == other.cbStructure
            && self.hwndOwner == other.hwndOwner
            && self.lpLocalName == other.lpLocalName
            && self.lpRemoteName == other.lpRemoteName
            && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for DISCDLGSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for DISCDLGSTRUCTW {
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
pub struct DISCDLGSTRUCT_FLAGS(pub u32);
pub const DISC_UPDATE_PROFILE: DISCDLGSTRUCT_FLAGS = DISCDLGSTRUCT_FLAGS(1u32);
pub const DISC_NO_FORCE: DISCDLGSTRUCT_FLAGS = DISCDLGSTRUCT_FLAGS(64u32);
impl ::std::convert::From<u32> for DISCDLGSTRUCT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISCDLGSTRUCT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DISCDLGSTRUCT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DISCDLGSTRUCT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DISCDLGSTRUCT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DISCDLGSTRUCT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DISCDLGSTRUCT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn MultinetGetConnectionPerformanceA(
    lpnetresource: *const NETRESOURCEA,
    lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MultinetGetConnectionPerformanceA(
                lpnetresource: *const NETRESOURCEA,
                lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT,
            ) -> u32;
        }
        ::std::mem::transmute(MultinetGetConnectionPerformanceA(
            ::std::mem::transmute(lpnetresource),
            ::std::mem::transmute(lpnetconnectinfostruct),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn MultinetGetConnectionPerformanceW(
    lpnetresource: *const NETRESOURCEW,
    lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MultinetGetConnectionPerformanceW(
                lpnetresource: *const NETRESOURCEW,
                lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT,
            ) -> u32;
        }
        ::std::mem::transmute(MultinetGetConnectionPerformanceW(
            ::std::mem::transmute(lpnetresource),
            ::std::mem::transmute(lpnetconnectinfostruct),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NETCONNECTINFOSTRUCT {
    pub cbStructure: u32,
    pub dwFlags: u32,
    pub dwSpeed: u32,
    pub dwDelay: u32,
    pub dwOptDataSize: u32,
}
impl NETCONNECTINFOSTRUCT {}
impl ::std::default::Default for NETCONNECTINFOSTRUCT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NETCONNECTINFOSTRUCT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NETCONNECTINFOSTRUCT")
            .field("cbStructure", &self.cbStructure)
            .field("dwFlags", &self.dwFlags)
            .field("dwSpeed", &self.dwSpeed)
            .field("dwDelay", &self.dwDelay)
            .field("dwOptDataSize", &self.dwOptDataSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NETCONNECTINFOSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStructure == other.cbStructure
            && self.dwFlags == other.dwFlags
            && self.dwSpeed == other.dwSpeed
            && self.dwDelay == other.dwDelay
            && self.dwOptDataSize == other.dwOptDataSize
    }
}
impl ::std::cmp::Eq for NETCONNECTINFOSTRUCT {}
unsafe impl ::windows::runtime::Abi for NETCONNECTINFOSTRUCT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NETINFOSTRUCT {
    pub cbStructure: u32,
    pub dwProviderVersion: u32,
    pub dwStatus: super::super::Foundation::WIN32_ERROR,
    pub dwCharacteristics: NETINFOSTRUCT_CHARACTERISTICS,
    pub dwHandle: usize,
    pub wNetType: u16,
    pub dwPrinters: u32,
    pub dwDrives: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl NETINFOSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NETINFOSTRUCT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NETINFOSTRUCT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NETINFOSTRUCT")
            .field("cbStructure", &self.cbStructure)
            .field("dwProviderVersion", &self.dwProviderVersion)
            .field("dwStatus", &self.dwStatus)
            .field("dwCharacteristics", &self.dwCharacteristics)
            .field("dwHandle", &self.dwHandle)
            .field("wNetType", &self.wNetType)
            .field("dwPrinters", &self.dwPrinters)
            .field("dwDrives", &self.dwDrives)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NETINFOSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.cbStructure == other.cbStructure
            && self.dwProviderVersion == other.dwProviderVersion
            && self.dwStatus == other.dwStatus
            && self.dwCharacteristics == other.dwCharacteristics
            && self.dwHandle == other.dwHandle
            && self.wNetType == other.wNetType
            && self.dwPrinters == other.dwPrinters
            && self.dwDrives == other.dwDrives
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NETINFOSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NETINFOSTRUCT {
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
pub struct NETINFOSTRUCT_CHARACTERISTICS(pub u32);
pub const NETINFO_DLL16: NETINFOSTRUCT_CHARACTERISTICS = NETINFOSTRUCT_CHARACTERISTICS(1u32);
pub const NETINFO_DISKRED: NETINFOSTRUCT_CHARACTERISTICS = NETINFOSTRUCT_CHARACTERISTICS(4u32);
pub const NETINFO_PRINTERRED: NETINFOSTRUCT_CHARACTERISTICS = NETINFOSTRUCT_CHARACTERISTICS(8u32);
impl ::std::convert::From<u32> for NETINFOSTRUCT_CHARACTERISTICS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NETINFOSTRUCT_CHARACTERISTICS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for NETINFOSTRUCT_CHARACTERISTICS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NETINFOSTRUCT_CHARACTERISTICS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NETINFOSTRUCT_CHARACTERISTICS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NETINFOSTRUCT_CHARACTERISTICS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NETINFOSTRUCT_CHARACTERISTICS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const NETPROPERTY_PERSISTENT: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NETRESOURCEA {
    pub dwScope: NET_RESOURCE_SCOPE,
    pub dwType: NET_RESOURCE_TYPE,
    pub dwDisplayType: u32,
    pub dwUsage: u32,
    pub lpLocalName: super::super::Foundation::PSTR,
    pub lpRemoteName: super::super::Foundation::PSTR,
    pub lpComment: super::super::Foundation::PSTR,
    pub lpProvider: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl NETRESOURCEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NETRESOURCEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NETRESOURCEA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NETRESOURCEA")
            .field("dwScope", &self.dwScope)
            .field("dwType", &self.dwType)
            .field("dwDisplayType", &self.dwDisplayType)
            .field("dwUsage", &self.dwUsage)
            .field("lpLocalName", &self.lpLocalName)
            .field("lpRemoteName", &self.lpRemoteName)
            .field("lpComment", &self.lpComment)
            .field("lpProvider", &self.lpProvider)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NETRESOURCEA {
    fn eq(&self, other: &Self) -> bool {
        self.dwScope == other.dwScope
            && self.dwType == other.dwType
            && self.dwDisplayType == other.dwDisplayType
            && self.dwUsage == other.dwUsage
            && self.lpLocalName == other.lpLocalName
            && self.lpRemoteName == other.lpRemoteName
            && self.lpComment == other.lpComment
            && self.lpProvider == other.lpProvider
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NETRESOURCEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NETRESOURCEA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NETRESOURCEW {
    pub dwScope: NET_RESOURCE_SCOPE,
    pub dwType: NET_RESOURCE_TYPE,
    pub dwDisplayType: u32,
    pub dwUsage: u32,
    pub lpLocalName: super::super::Foundation::PWSTR,
    pub lpRemoteName: super::super::Foundation::PWSTR,
    pub lpComment: super::super::Foundation::PWSTR,
    pub lpProvider: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl NETRESOURCEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NETRESOURCEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NETRESOURCEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NETRESOURCEW")
            .field("dwScope", &self.dwScope)
            .field("dwType", &self.dwType)
            .field("dwDisplayType", &self.dwDisplayType)
            .field("dwUsage", &self.dwUsage)
            .field("lpLocalName", &self.lpLocalName)
            .field("lpRemoteName", &self.lpRemoteName)
            .field("lpComment", &self.lpComment)
            .field("lpProvider", &self.lpProvider)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NETRESOURCEW {
    fn eq(&self, other: &Self) -> bool {
        self.dwScope == other.dwScope
            && self.dwType == other.dwType
            && self.dwDisplayType == other.dwDisplayType
            && self.dwUsage == other.dwUsage
            && self.lpLocalName == other.lpLocalName
            && self.lpRemoteName == other.lpRemoteName
            && self.lpComment == other.lpComment
            && self.lpProvider == other.lpProvider
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NETRESOURCEW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NETRESOURCEW {
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
pub struct NETWORK_NAME_FORMAT_FLAGS(pub u32);
pub const WNFMT_MULTILINE: NETWORK_NAME_FORMAT_FLAGS = NETWORK_NAME_FORMAT_FLAGS(1u32);
pub const WNFMT_ABBREVIATED: NETWORK_NAME_FORMAT_FLAGS = NETWORK_NAME_FORMAT_FLAGS(2u32);
impl ::std::convert::From<u32> for NETWORK_NAME_FORMAT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NETWORK_NAME_FORMAT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for NETWORK_NAME_FORMAT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NETWORK_NAME_FORMAT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NETWORK_NAME_FORMAT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NETWORK_NAME_FORMAT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NETWORK_NAME_FORMAT_FLAGS {
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
pub struct NET_RESOURCE_SCOPE(pub u32);
pub const RESOURCE_CONNECTED: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(1u32);
pub const RESOURCE_CONTEXT: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(5u32);
pub const RESOURCE_GLOBALNET: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(2u32);
pub const RESOURCE_REMEMBERED: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(3u32);
impl ::std::convert::From<u32> for NET_RESOURCE_SCOPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NET_RESOURCE_SCOPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for NET_RESOURCE_SCOPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NET_RESOURCE_SCOPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NET_RESOURCE_SCOPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NET_RESOURCE_SCOPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NET_RESOURCE_SCOPE {
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
pub struct NET_RESOURCE_TYPE(pub u32);
pub const RESOURCETYPE_ANY: NET_RESOURCE_TYPE = NET_RESOURCE_TYPE(0u32);
pub const RESOURCETYPE_DISK: NET_RESOURCE_TYPE = NET_RESOURCE_TYPE(1u32);
pub const RESOURCETYPE_PRINT: NET_RESOURCE_TYPE = NET_RESOURCE_TYPE(2u32);
impl ::std::convert::From<u32> for NET_RESOURCE_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NET_RESOURCE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for NET_RESOURCE_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NET_RESOURCE_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NET_RESOURCE_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NET_RESOURCE_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NET_RESOURCE_TYPE {
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
pub struct NET_USE_CONNECT_FLAGS(pub u32);
pub const CONNECT_INTERACTIVE: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(8u32);
pub const CONNECT_PROMPT: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(16u32);
pub const CONNECT_REDIRECT: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(128u32);
pub const CONNECT_UPDATE_PROFILE: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(1u32);
pub const CONNECT_COMMANDLINE: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(2048u32);
pub const CONNECT_CMD_SAVECRED: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(4096u32);
pub const CONNECT_TEMPORARY: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(4u32);
pub const CONNECT_DEFERRED: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(1024u32);
pub const CONNECT_UPDATE_RECENT: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(2u32);
impl ::std::convert::From<u32> for NET_USE_CONNECT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NET_USE_CONNECT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for NET_USE_CONNECT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NET_USE_CONNECT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NET_USE_CONNECT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NET_USE_CONNECT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NET_USE_CONNECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NOTIFYADD {
    pub hwndOwner: super::super::Foundation::HWND,
    pub NetResource: NETRESOURCEA,
    pub dwAddFlags: NET_USE_CONNECT_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl NOTIFYADD {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NOTIFYADD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NOTIFYADD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NOTIFYADD")
            .field("hwndOwner", &self.hwndOwner)
            .field("NetResource", &self.NetResource)
            .field("dwAddFlags", &self.dwAddFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NOTIFYADD {
    fn eq(&self, other: &Self) -> bool {
        self.hwndOwner == other.hwndOwner
            && self.NetResource == other.NetResource
            && self.dwAddFlags == other.dwAddFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NOTIFYADD {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NOTIFYADD {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NOTIFYCANCEL {
    pub lpName: super::super::Foundation::PWSTR,
    pub lpProvider: super::super::Foundation::PWSTR,
    pub dwFlags: u32,
    pub fForce: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl NOTIFYCANCEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for NOTIFYCANCEL {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for NOTIFYCANCEL {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NOTIFYCANCEL")
            .field("lpName", &self.lpName)
            .field("lpProvider", &self.lpProvider)
            .field("dwFlags", &self.dwFlags)
            .field("fForce", &self.fForce)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for NOTIFYCANCEL {
    fn eq(&self, other: &Self) -> bool {
        self.lpName == other.lpName
            && self.lpProvider == other.lpProvider
            && self.dwFlags == other.dwFlags
            && self.fForce == other.fForce
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for NOTIFYCANCEL {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for NOTIFYCANCEL {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct NOTIFYINFO {
    pub dwNotifyStatus: u32,
    pub dwOperationStatus: u32,
    pub lpContext: *mut ::std::ffi::c_void,
}
impl NOTIFYINFO {}
impl ::std::default::Default for NOTIFYINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NOTIFYINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NOTIFYINFO")
            .field("dwNotifyStatus", &self.dwNotifyStatus)
            .field("dwOperationStatus", &self.dwOperationStatus)
            .field("lpContext", &self.lpContext)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NOTIFYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwNotifyStatus == other.dwNotifyStatus
            && self.dwOperationStatus == other.dwOperationStatus
            && self.lpContext == other.lpContext
    }
}
impl ::std::cmp::Eq for NOTIFYINFO {}
unsafe impl ::windows::runtime::Abi for NOTIFYINFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const NOTIFY_POST: u32 = 2u32;
pub const NOTIFY_PRE: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPAddConnection<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpnetresource: *const NETRESOURCEW,
    lppassword: Param1,
    lpusername: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPAddConnection(
                lpnetresource: *const NETRESOURCEW,
                lppassword: super::super::Foundation::PWSTR,
                lpusername: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(NPAddConnection(
            ::std::mem::transmute(lpnetresource),
            lppassword.into_param().abi(),
            lpusername.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPAddConnection3<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hwndowner: Param0,
    lpnetresource: *const NETRESOURCEW,
    lppassword: Param2,
    lpusername: Param3,
    dwflags: NET_USE_CONNECT_FLAGS,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPAddConnection3(
                hwndowner: super::super::Foundation::HWND,
                lpnetresource: *const NETRESOURCEW,
                lppassword: super::super::Foundation::PWSTR,
                lpusername: super::super::Foundation::PWSTR,
                dwflags: NET_USE_CONNECT_FLAGS,
            ) -> u32;
        }
        ::std::mem::transmute(NPAddConnection3(
            hwndowner.into_param().abi(),
            ::std::mem::transmute(lpnetresource),
            lppassword.into_param().abi(),
            lpusername.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPAddConnection4<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwndowner: Param0,
    lpnetresource: *const NETRESOURCEW,
    lpauthbuffer: *const ::std::ffi::c_void,
    cbauthbuffer: u32,
    dwflags: u32,
    lpuseoptions: *const u8,
    cbuseoptions: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPAddConnection4(
                hwndowner: super::super::Foundation::HWND,
                lpnetresource: *const NETRESOURCEW,
                lpauthbuffer: *const ::std::ffi::c_void,
                cbauthbuffer: u32,
                dwflags: u32,
                lpuseoptions: *const u8,
                cbuseoptions: u32,
            ) -> u32;
        }
        ::std::mem::transmute(NPAddConnection4(
            hwndowner.into_param().abi(),
            ::std::mem::transmute(lpnetresource),
            ::std::mem::transmute(lpauthbuffer),
            ::std::mem::transmute(cbauthbuffer),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpuseoptions),
            ::std::mem::transmute(cbuseoptions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPCancelConnection<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lpname: Param0,
    fforce: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPCancelConnection(
                lpname: super::super::Foundation::PWSTR,
                fforce: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(NPCancelConnection(
            lpname.into_param().abi(),
            fforce.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPCancelConnection2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lpname: Param0,
    fforce: Param1,
    dwflags: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPCancelConnection2(
                lpname: super::super::Foundation::PWSTR,
                fforce: super::super::Foundation::BOOL,
                dwflags: u32,
            ) -> u32;
        }
        ::std::mem::transmute(NPCancelConnection2(
            lpname.into_param().abi(),
            fforce.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPCloseEnum<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    henum: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPCloseEnum(henum: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(NPCloseEnum(henum.into_param().abi()))
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
pub struct NPDIRECTORY_NOTIFY_OPERATION(pub u32);
pub const WNDN_MKDIR: NPDIRECTORY_NOTIFY_OPERATION = NPDIRECTORY_NOTIFY_OPERATION(1u32);
pub const WNDN_RMDIR: NPDIRECTORY_NOTIFY_OPERATION = NPDIRECTORY_NOTIFY_OPERATION(2u32);
pub const WNDN_MVDIR: NPDIRECTORY_NOTIFY_OPERATION = NPDIRECTORY_NOTIFY_OPERATION(3u32);
impl ::std::convert::From<u32> for NPDIRECTORY_NOTIFY_OPERATION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NPDIRECTORY_NOTIFY_OPERATION {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for NPDIRECTORY_NOTIFY_OPERATION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NPDIRECTORY_NOTIFY_OPERATION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NPDIRECTORY_NOTIFY_OPERATION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NPDIRECTORY_NOTIFY_OPERATION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NPDIRECTORY_NOTIFY_OPERATION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPEnumResource<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    henum: Param0,
    lpccount: *mut u32,
    lpbuffer: *mut ::std::ffi::c_void,
    lpbuffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPEnumResource(
                henum: super::super::Foundation::HANDLE,
                lpccount: *mut u32,
                lpbuffer: *mut ::std::ffi::c_void,
                lpbuffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(NPEnumResource(
            henum.into_param().abi(),
            ::std::mem::transmute(lpccount),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPFormatNetworkName<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpremotename: Param0,
    lpformattedname: super::super::Foundation::PWSTR,
    lpnlength: *mut u32,
    dwflags: NETWORK_NAME_FORMAT_FLAGS,
    dwavecharperline: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPFormatNetworkName(
                lpremotename: super::super::Foundation::PWSTR,
                lpformattedname: super::super::Foundation::PWSTR,
                lpnlength: *mut u32,
                dwflags: NETWORK_NAME_FORMAT_FLAGS,
                dwavecharperline: u32,
            ) -> u32;
        }
        ::std::mem::transmute(NPFormatNetworkName(
            lpremotename.into_param().abi(),
            ::std::mem::transmute(lpformattedname),
            ::std::mem::transmute(lpnlength),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwavecharperline),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn NPGetCaps(ndex: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPGetCaps(ndex: u32) -> u32;
        }
        ::std::mem::transmute(NPGetCaps(::std::mem::transmute(ndex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPGetConnection<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lplocalname: Param0,
    lpremotename: super::super::Foundation::PWSTR,
    lpnbufferlen: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPGetConnection(
                lplocalname: super::super::Foundation::PWSTR,
                lpremotename: super::super::Foundation::PWSTR,
                lpnbufferlen: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(NPGetConnection(
            lplocalname.into_param().abi(),
            ::std::mem::transmute(lpremotename),
            ::std::mem::transmute(lpnbufferlen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPGetConnection3<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lplocalname: Param0,
    dwlevel: u32,
    lpbuffer: *mut ::std::ffi::c_void,
    lpbuffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPGetConnection3(
                lplocalname: super::super::Foundation::PWSTR,
                dwlevel: u32,
                lpbuffer: *mut ::std::ffi::c_void,
                lpbuffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(NPGetConnection3(
            lplocalname.into_param().abi(),
            ::std::mem::transmute(dwlevel),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPGetConnectionPerformance<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpremotename: Param0,
    lpnetconnectinfo: *mut NETCONNECTINFOSTRUCT,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPGetConnectionPerformance(
                lpremotename: super::super::Foundation::PWSTR,
                lpnetconnectinfo: *mut NETCONNECTINFOSTRUCT,
            ) -> u32;
        }
        ::std::mem::transmute(NPGetConnectionPerformance(
            lpremotename.into_param().abi(),
            ::std::mem::transmute(lpnetconnectinfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPGetPersistentUseOptionsForConnection<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpremotepath: Param0,
    lpreaduseoptions: *const u8,
    cbreaduseoptions: u32,
    lpwriteuseoptions: *mut u8,
    lpsizewriteuseoptions: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPGetPersistentUseOptionsForConnection(
                lpremotepath: super::super::Foundation::PWSTR,
                lpreaduseoptions: *const u8,
                cbreaduseoptions: u32,
                lpwriteuseoptions: *mut u8,
                lpsizewriteuseoptions: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(NPGetPersistentUseOptionsForConnection(
            lpremotepath.into_param().abi(),
            ::std::mem::transmute(lpreaduseoptions),
            ::std::mem::transmute(cbreaduseoptions),
            ::std::mem::transmute(lpwriteuseoptions),
            ::std::mem::transmute(lpsizewriteuseoptions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPGetResourceInformation(
    lpnetresource: *const NETRESOURCEW,
    lpbuffer: *mut ::std::ffi::c_void,
    lpbuffersize: *mut u32,
    lplpsystem: *mut super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPGetResourceInformation(
                lpnetresource: *const NETRESOURCEW,
                lpbuffer: *mut ::std::ffi::c_void,
                lpbuffersize: *mut u32,
                lplpsystem: *mut super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(NPGetResourceInformation(
            ::std::mem::transmute(lpnetresource),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpbuffersize),
            ::std::mem::transmute(lplpsystem),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPGetResourceParent(
    lpnetresource: *const NETRESOURCEW,
    lpbuffer: *mut ::std::ffi::c_void,
    lpbuffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPGetResourceParent(
                lpnetresource: *const NETRESOURCEW,
                lpbuffer: *mut ::std::ffi::c_void,
                lpbuffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(NPGetResourceParent(
            ::std::mem::transmute(lpnetresource),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPGetUniversalName<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lplocalpath: Param0,
    dwinfolevel: UNC_INFO_LEVEL,
    lpbuffer: *mut ::std::ffi::c_void,
    lpbuffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPGetUniversalName(
                lplocalpath: super::super::Foundation::PWSTR,
                dwinfolevel: UNC_INFO_LEVEL,
                lpbuffer: *mut ::std::ffi::c_void,
                lpbuffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(NPGetUniversalName(
            lplocalpath.into_param().abi(),
            ::std::mem::transmute(dwinfolevel),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPGetUser<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpname: Param0,
    lpusername: super::super::Foundation::PWSTR,
    lpnbufferlen: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPGetUser(
                lpname: super::super::Foundation::PWSTR,
                lpusername: super::super::Foundation::PWSTR,
                lpnbufferlen: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(NPGetUser(
            lpname.into_param().abi(),
            ::std::mem::transmute(lpusername),
            ::std::mem::transmute(lpnbufferlen),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn NPOpenEnum(
    dwscope: u32,
    dwtype: u32,
    dwusage: u32,
    lpnetresource: *const NETRESOURCEW,
    lphenum: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NPOpenEnum(
                dwscope: u32,
                dwtype: u32,
                dwusage: u32,
                lpnetresource: *const NETRESOURCEW,
                lphenum: *mut super::super::Foundation::HANDLE,
            ) -> u32;
        }
        ::std::mem::transmute(NPOpenEnum(
            ::std::mem::transmute(dwscope),
            ::std::mem::transmute(dwtype),
            ::std::mem::transmute(dwusage),
            ::std::mem::transmute(lpnetresource),
            ::std::mem::transmute(lphenum),
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
pub struct NP_PROPERTY_DIALOG_SELECTION(pub u32);
pub const WNPS_FILE: NP_PROPERTY_DIALOG_SELECTION = NP_PROPERTY_DIALOG_SELECTION(0u32);
pub const WNPS_DIR: NP_PROPERTY_DIALOG_SELECTION = NP_PROPERTY_DIALOG_SELECTION(1u32);
pub const WNPS_MULT: NP_PROPERTY_DIALOG_SELECTION = NP_PROPERTY_DIALOG_SELECTION(2u32);
impl ::std::convert::From<u32> for NP_PROPERTY_DIALOG_SELECTION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for NP_PROPERTY_DIALOG_SELECTION {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for NP_PROPERTY_DIALOG_SELECTION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for NP_PROPERTY_DIALOG_SELECTION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for NP_PROPERTY_DIALOG_SELECTION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for NP_PROPERTY_DIALOG_SELECTION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for NP_PROPERTY_DIALOG_SELECTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct NetEnumHandle(pub isize);
impl ::std::default::Default for NetEnumHandle {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for NetEnumHandle {}
unsafe impl ::windows::runtime::Abi for NetEnumHandle {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type PF_AddConnectNotify =
    unsafe extern "system" fn(lpnotifyinfo: *mut NOTIFYINFO, lpaddinfo: *const NOTIFYADD) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_CancelConnectNotify = unsafe extern "system" fn(
    lpnotifyinfo: *mut NOTIFYINFO,
    lpcancelinfo: *const NOTIFYCANCEL,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPAddConnection = unsafe extern "system" fn(
    lpnetresource: *const NETRESOURCEW,
    lppassword: super::super::Foundation::PWSTR,
    lpusername: super::super::Foundation::PWSTR,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPAddConnection3 = unsafe extern "system" fn(
    hwndowner: super::super::Foundation::HWND,
    lpnetresource: *const NETRESOURCEW,
    lppassword: super::super::Foundation::PWSTR,
    lpusername: super::super::Foundation::PWSTR,
    dwflags: u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPAddConnection4 = unsafe extern "system" fn(
    hwndowner: super::super::Foundation::HWND,
    lpnetresource: *const NETRESOURCEW,
    lpauthbuffer: *const ::std::ffi::c_void,
    cbauthbuffer: u32,
    dwflags: u32,
    lpuseoptions: *const u8,
    cbuseoptions: u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPCancelConnection = unsafe extern "system" fn(
    lpname: super::super::Foundation::PWSTR,
    fforce: super::super::Foundation::BOOL,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPCancelConnection2 = unsafe extern "system" fn(
    lpname: super::super::Foundation::PWSTR,
    fforce: super::super::Foundation::BOOL,
    dwflags: u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPCloseEnum = unsafe extern "system" fn(henum: super::super::Foundation::HANDLE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPDeviceMode =
    unsafe extern "system" fn(hparent: super::super::Foundation::HWND) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPDirectoryNotify = unsafe extern "system" fn(
    hwnd: super::super::Foundation::HWND,
    lpdir: super::super::Foundation::PWSTR,
    dwoper: u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPEnumResource = unsafe extern "system" fn(
    henum: super::super::Foundation::HANDLE,
    lpccount: *mut u32,
    lpbuffer: *mut ::std::ffi::c_void,
    lpbuffersize: *mut u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPFMXEditPerm = unsafe extern "system" fn(
    lpdrivename: super::super::Foundation::PWSTR,
    hwndfmx: super::super::Foundation::HWND,
    ndialogtype: u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPFMXGetPermCaps =
    unsafe extern "system" fn(lpdrivename: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPFMXGetPermHelp = unsafe extern "system" fn(
    lpdrivename: super::super::Foundation::PWSTR,
    ndialogtype: u32,
    fdirectory: super::super::Foundation::BOOL,
    lpfilenamebuffer: *mut ::std::ffi::c_void,
    lpbuffersize: *mut u32,
    lpnhelpcontext: *mut u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPFormatNetworkName = unsafe extern "system" fn(
    lpremotename: super::super::Foundation::PWSTR,
    lpformattedname: super::super::Foundation::PWSTR,
    lpnlength: *mut u32,
    dwflags: u32,
    dwavecharperline: u32,
) -> u32;
pub type PF_NPGetCaps = unsafe extern "system" fn(ndex: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetConnection = unsafe extern "system" fn(
    lplocalname: super::super::Foundation::PWSTR,
    lpremotename: super::super::Foundation::PWSTR,
    lpnbufferlen: *mut u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetConnection3 = unsafe extern "system" fn(
    lplocalname: super::super::Foundation::PWSTR,
    dwlevel: u32,
    lpbuffer: *mut ::std::ffi::c_void,
    lpbuffersize: *mut u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetConnectionPerformance = unsafe extern "system" fn(
    lpremotename: super::super::Foundation::PWSTR,
    lpnetconnectinfo: *mut NETCONNECTINFOSTRUCT,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetDirectoryType = unsafe extern "system" fn(
    lpname: super::super::Foundation::PWSTR,
    lptype: *const i32,
    bflushcache: super::super::Foundation::BOOL,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetPersistentUseOptionsForConnection = unsafe extern "system" fn(
    lpremotepath: super::super::Foundation::PWSTR,
    lpreaduseoptions: *const u8,
    cbreaduseoptions: u32,
    lpwriteuseoptions: *mut u8,
    lpsizewriteuseoptions: *mut u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetPropertyText = unsafe extern "system" fn(
    ibutton: u32,
    npropsel: u32,
    lpname: super::super::Foundation::PWSTR,
    lpbuttonname: super::super::Foundation::PWSTR,
    nbuttonnamelen: u32,
    ntype: u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetResourceInformation = unsafe extern "system" fn(
    lpnetresource: *const NETRESOURCEW,
    lpbuffer: *mut ::std::ffi::c_void,
    lpbuffersize: *mut u32,
    lplpsystem: *mut super::super::Foundation::PWSTR,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetResourceParent = unsafe extern "system" fn(
    lpnetresource: *const NETRESOURCEW,
    lpbuffer: *mut ::std::ffi::c_void,
    lpbuffersize: *mut u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetUniversalName = unsafe extern "system" fn(
    lplocalpath: super::super::Foundation::PWSTR,
    dwinfolevel: u32,
    lpbuffer: *mut ::std::ffi::c_void,
    lpnbuffersize: *mut u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetUser = unsafe extern "system" fn(
    lpname: super::super::Foundation::PWSTR,
    lpusername: super::super::Foundation::PWSTR,
    lpnbufferlen: *mut u32,
) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub type PF_NPLogonNotify = unsafe extern "system" fn(
    lplogonid: *const super::super::System::SystemServices::LUID,
    lpauthentinfotype: super::super::Foundation::PWSTR,
    lpauthentinfo: *const ::std::ffi::c_void,
    lppreviousauthentinfotype: super::super::Foundation::PWSTR,
    lppreviousauthentinfo: *const ::std::ffi::c_void,
    lpstationname: super::super::Foundation::PWSTR,
    stationhandle: *const ::std::ffi::c_void,
    lplogonscript: *mut super::super::Foundation::PWSTR,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPOpenEnum = unsafe extern "system" fn(
    dwscope: u32,
    dwtype: u32,
    dwusage: u32,
    lpnetresource: *const NETRESOURCEW,
    lphenum: *mut super::super::Foundation::HANDLE,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPPasswordChangeNotify = unsafe extern "system" fn(
    lpauthentinfotype: super::super::Foundation::PWSTR,
    lpauthentinfo: *const ::std::ffi::c_void,
    lppreviousauthentinfotype: super::super::Foundation::PWSTR,
    lppreviousauthentinfo: *const ::std::ffi::c_void,
    lpstationname: super::super::Foundation::PWSTR,
    stationhandle: *const ::std::ffi::c_void,
    dwchangeinfo: u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPPropertyDialog = unsafe extern "system" fn(
    hwndparent: super::super::Foundation::HWND,
    ibuttondlg: u32,
    npropsel: u32,
    lpfilename: super::super::Foundation::PWSTR,
    ntype: u32,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPSearchDialog = unsafe extern "system" fn(
    hwndparent: super::super::Foundation::HWND,
    lpnetresource: *const NETRESOURCEW,
    lpbuffer: *mut ::std::ffi::c_void,
    cbbuffer: u32,
    lpnflags: *mut u32,
) -> u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct REMOTE_NAME_INFOA {
    pub lpUniversalName: super::super::Foundation::PSTR,
    pub lpConnectionName: super::super::Foundation::PSTR,
    pub lpRemainingPath: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl REMOTE_NAME_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for REMOTE_NAME_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for REMOTE_NAME_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("REMOTE_NAME_INFOA")
            .field("lpUniversalName", &self.lpUniversalName)
            .field("lpConnectionName", &self.lpConnectionName)
            .field("lpRemainingPath", &self.lpRemainingPath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for REMOTE_NAME_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.lpUniversalName == other.lpUniversalName
            && self.lpConnectionName == other.lpConnectionName
            && self.lpRemainingPath == other.lpRemainingPath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for REMOTE_NAME_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for REMOTE_NAME_INFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct REMOTE_NAME_INFOW {
    pub lpUniversalName: super::super::Foundation::PWSTR,
    pub lpConnectionName: super::super::Foundation::PWSTR,
    pub lpRemainingPath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl REMOTE_NAME_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for REMOTE_NAME_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for REMOTE_NAME_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("REMOTE_NAME_INFOW")
            .field("lpUniversalName", &self.lpUniversalName)
            .field("lpConnectionName", &self.lpConnectionName)
            .field("lpRemainingPath", &self.lpRemainingPath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for REMOTE_NAME_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.lpUniversalName == other.lpUniversalName
            && self.lpConnectionName == other.lpConnectionName
            && self.lpRemainingPath == other.lpRemainingPath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for REMOTE_NAME_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for REMOTE_NAME_INFOW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RESOURCEDISPLAYTYPE_DIRECTORY: u32 = 9u32;
pub const RESOURCEDISPLAYTYPE_NDSCONTAINER: u32 = 11u32;
pub const RESOURCEDISPLAYTYPE_NETWORK: u32 = 6u32;
pub const RESOURCEDISPLAYTYPE_ROOT: u32 = 7u32;
pub const RESOURCEDISPLAYTYPE_SHAREADMIN: u32 = 8u32;
pub const RESOURCETYPE_RESERVED: u32 = 8u32;
pub const RESOURCETYPE_UNKNOWN: u32 = 4294967295u32;
pub const RESOURCEUSAGE_NOLOCALDEVICE: u32 = 4u32;
pub const RESOURCEUSAGE_RESERVED: u32 = 2147483648u32;
pub const RESOURCEUSAGE_SIBLING: u32 = 8u32;
pub const RESOURCE_RECENT: u32 = 4u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct UNC_INFO_LEVEL(pub u32);
pub const UNIVERSAL_NAME_INFO_LEVEL: UNC_INFO_LEVEL = UNC_INFO_LEVEL(1u32);
pub const REMOTE_NAME_INFO_LEVEL: UNC_INFO_LEVEL = UNC_INFO_LEVEL(2u32);
impl ::std::convert::From<u32> for UNC_INFO_LEVEL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UNC_INFO_LEVEL {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for UNC_INFO_LEVEL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for UNC_INFO_LEVEL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for UNC_INFO_LEVEL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for UNC_INFO_LEVEL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for UNC_INFO_LEVEL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct UNIVERSAL_NAME_INFOA {
    pub lpUniversalName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl UNIVERSAL_NAME_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for UNIVERSAL_NAME_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for UNIVERSAL_NAME_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("UNIVERSAL_NAME_INFOA")
            .field("lpUniversalName", &self.lpUniversalName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for UNIVERSAL_NAME_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.lpUniversalName == other.lpUniversalName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for UNIVERSAL_NAME_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for UNIVERSAL_NAME_INFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct UNIVERSAL_NAME_INFOW {
    pub lpUniversalName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl UNIVERSAL_NAME_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for UNIVERSAL_NAME_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for UNIVERSAL_NAME_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("UNIVERSAL_NAME_INFOW")
            .field("lpUniversalName", &self.lpUniversalName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for UNIVERSAL_NAME_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.lpUniversalName == other.lpUniversalName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for UNIVERSAL_NAME_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for UNIVERSAL_NAME_INFOW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WNCON_DYNAMIC: u32 = 8u32;
pub const WNCON_FORNETCARD: u32 = 1u32;
pub const WNCON_NOTROUTED: u32 = 2u32;
pub const WNCON_SLOWLINK: u32 = 4u32;
pub const WNDT_NETWORK: u32 = 1u32;
pub const WNDT_NORMAL: u32 = 0u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WNET_OPEN_ENUM_USAGE(pub u32);
pub const RESOURCEUSAGE_NONE: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(0u32);
pub const RESOURCEUSAGE_CONNECTABLE: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(1u32);
pub const RESOURCEUSAGE_CONTAINER: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(2u32);
pub const RESOURCEUSAGE_ATTACHED: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(16u32);
pub const RESOURCEUSAGE_ALL: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(19u32);
impl ::std::convert::From<u32> for WNET_OPEN_ENUM_USAGE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WNET_OPEN_ENUM_USAGE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WNET_OPEN_ENUM_USAGE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WNET_OPEN_ENUM_USAGE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WNET_OPEN_ENUM_USAGE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WNET_OPEN_ENUM_USAGE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WNET_OPEN_ENUM_USAGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const WNFMT_CONNECTION: u32 = 32u32;
pub const WNFMT_INENUM: u32 = 16u32;
pub const WNGETCON_CONNECTED: u32 = 0u32;
pub const WNGETCON_DISCONNECTED: u32 = 1u32;
pub const WNNC_ADMIN: u32 = 9u32;
pub const WNNC_ADM_DIRECTORYNOTIFY: u32 = 2u32;
pub const WNNC_ADM_GETDIRECTORYTYPE: u32 = 1u32;
pub const WNNC_CONNECTION: u32 = 6u32;
pub const WNNC_CONNECTION_FLAGS: u32 = 13u32;
pub const WNNC_CON_ADDCONNECTION: u32 = 1u32;
pub const WNNC_CON_ADDCONNECTION3: u32 = 8u32;
pub const WNNC_CON_ADDCONNECTION4: u32 = 16u32;
pub const WNNC_CON_CANCELCONNECTION: u32 = 2u32;
pub const WNNC_CON_CANCELCONNECTION2: u32 = 32u32;
pub const WNNC_CON_DEFER: u32 = 128u32;
pub const WNNC_CON_GETCONNECTIONS: u32 = 4u32;
pub const WNNC_CON_GETPERFORMANCE: u32 = 64u32;
pub const WNNC_DIALOG: u32 = 8u32;
pub const WNNC_DLG_DEVICEMODE: u32 = 1u32;
pub const WNNC_DLG_FORMATNETWORKNAME: u32 = 128u32;
pub const WNNC_DLG_GETRESOURCEINFORMATION: u32 = 2048u32;
pub const WNNC_DLG_GETRESOURCEPARENT: u32 = 512u32;
pub const WNNC_DLG_PERMISSIONEDITOR: u32 = 256u32;
pub const WNNC_DLG_PROPERTYDIALOG: u32 = 32u32;
pub const WNNC_DLG_SEARCHDIALOG: u32 = 64u32;
pub const WNNC_DRIVER_VERSION: u32 = 3u32;
pub const WNNC_ENUMERATION: u32 = 11u32;
pub const WNNC_ENUM_CONTEXT: u32 = 4u32;
pub const WNNC_ENUM_GLOBAL: u32 = 1u32;
pub const WNNC_ENUM_LOCAL: u32 = 2u32;
pub const WNNC_ENUM_SHAREABLE: u32 = 8u32;
pub const WNNC_NET_NONE: u32 = 0u32;
pub const WNNC_NET_TYPE: u32 = 2u32;
pub const WNNC_SPEC_VERSION: u32 = 1u32;
pub const WNNC_SPEC_VERSION51: u32 = 327681u32;
pub const WNNC_START: u32 = 12u32;
pub const WNNC_USER: u32 = 4u32;
pub const WNNC_USR_GETUSER: u32 = 1u32;
pub const WNNC_WAIT_FOR_START: u32 = 1u32;
pub const WNPERMC_AUDIT: u32 = 2u32;
pub const WNPERMC_OWNER: u32 = 4u32;
pub const WNPERMC_PERM: u32 = 1u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WNPERM_DLG(pub u32);
pub const WNPERM_DLG_PERM: WNPERM_DLG = WNPERM_DLG(0u32);
pub const WNPERM_DLG_AUDIT: WNPERM_DLG = WNPERM_DLG(1u32);
pub const WNPERM_DLG_OWNER: WNPERM_DLG = WNPERM_DLG(2u32);
impl ::std::convert::From<u32> for WNPERM_DLG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WNPERM_DLG {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WNPERM_DLG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WNPERM_DLG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WNPERM_DLG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WNPERM_DLG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WNPERM_DLG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const WNSRCH_REFRESH_FIRST_LEVEL: u32 = 1u32;
pub const WNTYPE_COMM: u32 = 4u32;
pub const WNTYPE_DRIVE: u32 = 1u32;
pub const WNTYPE_FILE: u32 = 2u32;
pub const WNTYPE_PRINTER: u32 = 3u32;
pub const WN_CREDENTIAL_CLASS: u32 = 2u32;
pub const WN_NETWORK_CLASS: u32 = 1u32;
pub const WN_NT_PASSWORD_CHANGED: u32 = 2u32;
pub const WN_PRIMARY_AUTHENT_CLASS: u32 = 4u32;
pub const WN_SERVICE_CLASS: u32 = 8u32;
pub const WN_VALID_LOGON_ACCOUNT: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetAddConnection2A<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpnetresource: *const NETRESOURCEA,
    lppassword: Param1,
    lpusername: Param2,
    dwflags: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetAddConnection2A(
                lpnetresource: *const NETRESOURCEA,
                lppassword: super::super::Foundation::PSTR,
                lpusername: super::super::Foundation::PSTR,
                dwflags: u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetAddConnection2A(
            ::std::mem::transmute(lpnetresource),
            lppassword.into_param().abi(),
            lpusername.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetAddConnection2W<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpnetresource: *const NETRESOURCEW,
    lppassword: Param1,
    lpusername: Param2,
    dwflags: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetAddConnection2W(
                lpnetresource: *const NETRESOURCEW,
                lppassword: super::super::Foundation::PWSTR,
                lpusername: super::super::Foundation::PWSTR,
                dwflags: u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetAddConnection2W(
            ::std::mem::transmute(lpnetresource),
            lppassword.into_param().abi(),
            lpusername.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetAddConnection3A<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hwndowner: Param0,
    lpnetresource: *const NETRESOURCEA,
    lppassword: Param2,
    lpusername: Param3,
    dwflags: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetAddConnection3A(
                hwndowner: super::super::Foundation::HWND,
                lpnetresource: *const NETRESOURCEA,
                lppassword: super::super::Foundation::PSTR,
                lpusername: super::super::Foundation::PSTR,
                dwflags: u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetAddConnection3A(
            hwndowner.into_param().abi(),
            ::std::mem::transmute(lpnetresource),
            lppassword.into_param().abi(),
            lpusername.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetAddConnection3W<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hwndowner: Param0,
    lpnetresource: *const NETRESOURCEW,
    lppassword: Param2,
    lpusername: Param3,
    dwflags: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetAddConnection3W(
                hwndowner: super::super::Foundation::HWND,
                lpnetresource: *const NETRESOURCEW,
                lppassword: super::super::Foundation::PWSTR,
                lpusername: super::super::Foundation::PWSTR,
                dwflags: u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetAddConnection3W(
            hwndowner.into_param().abi(),
            ::std::mem::transmute(lpnetresource),
            lppassword.into_param().abi(),
            lpusername.into_param().abi(),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetAddConnection4A<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwndowner: Param0,
    lpnetresource: *const NETRESOURCEA,
    pauthbuffer: *const ::std::ffi::c_void,
    cbauthbuffer: u32,
    dwflags: u32,
    lpuseoptions: *const u8,
    cbuseoptions: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetAddConnection4A(
                hwndowner: super::super::Foundation::HWND,
                lpnetresource: *const NETRESOURCEA,
                pauthbuffer: *const ::std::ffi::c_void,
                cbauthbuffer: u32,
                dwflags: u32,
                lpuseoptions: *const u8,
                cbuseoptions: u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetAddConnection4A(
            hwndowner.into_param().abi(),
            ::std::mem::transmute(lpnetresource),
            ::std::mem::transmute(pauthbuffer),
            ::std::mem::transmute(cbauthbuffer),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpuseoptions),
            ::std::mem::transmute(cbuseoptions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetAddConnection4W<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwndowner: Param0,
    lpnetresource: *const NETRESOURCEW,
    pauthbuffer: *const ::std::ffi::c_void,
    cbauthbuffer: u32,
    dwflags: u32,
    lpuseoptions: *const u8,
    cbuseoptions: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetAddConnection4W(
                hwndowner: super::super::Foundation::HWND,
                lpnetresource: *const NETRESOURCEW,
                pauthbuffer: *const ::std::ffi::c_void,
                cbauthbuffer: u32,
                dwflags: u32,
                lpuseoptions: *const u8,
                cbuseoptions: u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetAddConnection4W(
            hwndowner.into_param().abi(),
            ::std::mem::transmute(lpnetresource),
            ::std::mem::transmute(pauthbuffer),
            ::std::mem::transmute(cbauthbuffer),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpuseoptions),
            ::std::mem::transmute(cbuseoptions),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetAddConnectionA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpremotename: Param0,
    lppassword: Param1,
    lplocalname: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetAddConnectionA(
                lpremotename: super::super::Foundation::PSTR,
                lppassword: super::super::Foundation::PSTR,
                lplocalname: super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(WNetAddConnectionA(
            lpremotename.into_param().abi(),
            lppassword.into_param().abi(),
            lplocalname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetAddConnectionW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpremotename: Param0,
    lppassword: Param1,
    lplocalname: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetAddConnectionW(
                lpremotename: super::super::Foundation::PWSTR,
                lppassword: super::super::Foundation::PWSTR,
                lplocalname: super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(WNetAddConnectionW(
            lpremotename.into_param().abi(),
            lppassword.into_param().abi(),
            lplocalname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetCancelConnection2A<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lpname: Param0,
    dwflags: u32,
    fforce: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetCancelConnection2A(
                lpname: super::super::Foundation::PSTR,
                dwflags: u32,
                fforce: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(WNetCancelConnection2A(
            lpname.into_param().abi(),
            ::std::mem::transmute(dwflags),
            fforce.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetCancelConnection2W<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lpname: Param0,
    dwflags: u32,
    fforce: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetCancelConnection2W(
                lpname: super::super::Foundation::PWSTR,
                dwflags: u32,
                fforce: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(WNetCancelConnection2W(
            lpname.into_param().abi(),
            ::std::mem::transmute(dwflags),
            fforce.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetCancelConnectionA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lpname: Param0,
    fforce: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetCancelConnectionA(
                lpname: super::super::Foundation::PSTR,
                fforce: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(WNetCancelConnectionA(
            lpname.into_param().abi(),
            fforce.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetCancelConnectionW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    lpname: Param0,
    fforce: Param1,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetCancelConnectionW(
                lpname: super::super::Foundation::PWSTR,
                fforce: super::super::Foundation::BOOL,
            ) -> u32;
        }
        ::std::mem::transmute(WNetCancelConnectionW(
            lpname.into_param().abi(),
            fforce.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetCloseEnum<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    henum: Param0,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetCloseEnum(henum: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(WNetCloseEnum(henum.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetConnectionDialog<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    dwtype: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetConnectionDialog(hwnd: super::super::Foundation::HWND, dwtype: u32) -> u32;
        }
        ::std::mem::transmute(WNetConnectionDialog(
            hwnd.into_param().abi(),
            ::std::mem::transmute(dwtype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetConnectionDialog1A(lpconndlgstruct: *mut CONNECTDLGSTRUCTA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetConnectionDialog1A(lpconndlgstruct: *mut CONNECTDLGSTRUCTA) -> u32;
        }
        ::std::mem::transmute(WNetConnectionDialog1A(::std::mem::transmute(
            lpconndlgstruct,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetConnectionDialog1W(lpconndlgstruct: *mut CONNECTDLGSTRUCTW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetConnectionDialog1W(lpconndlgstruct: *mut CONNECTDLGSTRUCTW) -> u32;
        }
        ::std::mem::transmute(WNetConnectionDialog1W(::std::mem::transmute(
            lpconndlgstruct,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetDisconnectDialog<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    dwtype: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetDisconnectDialog(hwnd: super::super::Foundation::HWND, dwtype: u32) -> u32;
        }
        ::std::mem::transmute(WNetDisconnectDialog(
            hwnd.into_param().abi(),
            ::std::mem::transmute(dwtype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetDisconnectDialog1A(lpconndlgstruct: *const DISCDLGSTRUCTA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetDisconnectDialog1A(lpconndlgstruct: *const DISCDLGSTRUCTA) -> u32;
        }
        ::std::mem::transmute(WNetDisconnectDialog1A(::std::mem::transmute(
            lpconndlgstruct,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetDisconnectDialog1W(lpconndlgstruct: *const DISCDLGSTRUCTW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetDisconnectDialog1W(lpconndlgstruct: *const DISCDLGSTRUCTW) -> u32;
        }
        ::std::mem::transmute(WNetDisconnectDialog1W(::std::mem::transmute(
            lpconndlgstruct,
        )))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetEnumResourceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    henum: Param0,
    lpccount: *mut u32,
    lpbuffer: *mut ::std::ffi::c_void,
    lpbuffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetEnumResourceA(
                henum: super::super::Foundation::HANDLE,
                lpccount: *mut u32,
                lpbuffer: *mut ::std::ffi::c_void,
                lpbuffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetEnumResourceA(
            henum.into_param().abi(),
            ::std::mem::transmute(lpccount),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetEnumResourceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    henum: Param0,
    lpccount: *mut u32,
    lpbuffer: *mut ::std::ffi::c_void,
    lpbuffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetEnumResourceW(
                henum: super::super::Foundation::HANDLE,
                lpccount: *mut u32,
                lpbuffer: *mut ::std::ffi::c_void,
                lpbuffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetEnumResourceW(
            henum.into_param().abi(),
            ::std::mem::transmute(lpccount),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetGetConnectionA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lplocalname: Param0,
    lpremotename: super::super::Foundation::PSTR,
    lpnlength: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetGetConnectionA(
                lplocalname: super::super::Foundation::PSTR,
                lpremotename: super::super::Foundation::PSTR,
                lpnlength: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetGetConnectionA(
            lplocalname.into_param().abi(),
            ::std::mem::transmute(lpremotename),
            ::std::mem::transmute(lpnlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetGetConnectionW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lplocalname: Param0,
    lpremotename: super::super::Foundation::PWSTR,
    lpnlength: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetGetConnectionW(
                lplocalname: super::super::Foundation::PWSTR,
                lpremotename: super::super::Foundation::PWSTR,
                lpnlength: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetGetConnectionW(
            lplocalname.into_param().abi(),
            ::std::mem::transmute(lpremotename),
            ::std::mem::transmute(lpnlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetGetLastErrorA(
    lperror: *mut u32,
    lperrorbuf: super::super::Foundation::PSTR,
    nerrorbufsize: u32,
    lpnamebuf: super::super::Foundation::PSTR,
    nnamebufsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetGetLastErrorA(
                lperror: *mut u32,
                lperrorbuf: super::super::Foundation::PSTR,
                nerrorbufsize: u32,
                lpnamebuf: super::super::Foundation::PSTR,
                nnamebufsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetGetLastErrorA(
            ::std::mem::transmute(lperror),
            ::std::mem::transmute(lperrorbuf),
            ::std::mem::transmute(nerrorbufsize),
            ::std::mem::transmute(lpnamebuf),
            ::std::mem::transmute(nnamebufsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetGetLastErrorW(
    lperror: *mut u32,
    lperrorbuf: super::super::Foundation::PWSTR,
    nerrorbufsize: u32,
    lpnamebuf: super::super::Foundation::PWSTR,
    nnamebufsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetGetLastErrorW(
                lperror: *mut u32,
                lperrorbuf: super::super::Foundation::PWSTR,
                nerrorbufsize: u32,
                lpnamebuf: super::super::Foundation::PWSTR,
                nnamebufsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetGetLastErrorW(
            ::std::mem::transmute(lperror),
            ::std::mem::transmute(lperrorbuf),
            ::std::mem::transmute(nerrorbufsize),
            ::std::mem::transmute(lpnamebuf),
            ::std::mem::transmute(nnamebufsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetGetNetworkInformationA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpprovider: Param0,
    lpnetinfostruct: *mut NETINFOSTRUCT,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetGetNetworkInformationA(
                lpprovider: super::super::Foundation::PSTR,
                lpnetinfostruct: *mut NETINFOSTRUCT,
            ) -> u32;
        }
        ::std::mem::transmute(WNetGetNetworkInformationA(
            lpprovider.into_param().abi(),
            ::std::mem::transmute(lpnetinfostruct),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetGetNetworkInformationW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpprovider: Param0,
    lpnetinfostruct: *mut NETINFOSTRUCT,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetGetNetworkInformationW(
                lpprovider: super::super::Foundation::PWSTR,
                lpnetinfostruct: *mut NETINFOSTRUCT,
            ) -> u32;
        }
        ::std::mem::transmute(WNetGetNetworkInformationW(
            lpprovider.into_param().abi(),
            ::std::mem::transmute(lpnetinfostruct),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetGetProviderNameA(
    dwnettype: u32,
    lpprovidername: super::super::Foundation::PSTR,
    lpbuffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetGetProviderNameA(
                dwnettype: u32,
                lpprovidername: super::super::Foundation::PSTR,
                lpbuffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetGetProviderNameA(
            ::std::mem::transmute(dwnettype),
            ::std::mem::transmute(lpprovidername),
            ::std::mem::transmute(lpbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetGetProviderNameW(
    dwnettype: u32,
    lpprovidername: super::super::Foundation::PWSTR,
    lpbuffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetGetProviderNameW(
                dwnettype: u32,
                lpprovidername: super::super::Foundation::PWSTR,
                lpbuffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetGetProviderNameW(
            ::std::mem::transmute(dwnettype),
            ::std::mem::transmute(lpprovidername),
            ::std::mem::transmute(lpbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetGetResourceInformationA(
    lpnetresource: *const NETRESOURCEA,
    lpbuffer: *mut ::std::ffi::c_void,
    lpcbbuffer: *mut u32,
    lplpsystem: *mut super::super::Foundation::PSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetGetResourceInformationA(
                lpnetresource: *const NETRESOURCEA,
                lpbuffer: *mut ::std::ffi::c_void,
                lpcbbuffer: *mut u32,
                lplpsystem: *mut super::super::Foundation::PSTR,
            ) -> u32;
        }
        ::std::mem::transmute(WNetGetResourceInformationA(
            ::std::mem::transmute(lpnetresource),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpcbbuffer),
            ::std::mem::transmute(lplpsystem),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetGetResourceInformationW(
    lpnetresource: *const NETRESOURCEW,
    lpbuffer: *mut ::std::ffi::c_void,
    lpcbbuffer: *mut u32,
    lplpsystem: *mut super::super::Foundation::PWSTR,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetGetResourceInformationW(
                lpnetresource: *const NETRESOURCEW,
                lpbuffer: *mut ::std::ffi::c_void,
                lpcbbuffer: *mut u32,
                lplpsystem: *mut super::super::Foundation::PWSTR,
            ) -> u32;
        }
        ::std::mem::transmute(WNetGetResourceInformationW(
            ::std::mem::transmute(lpnetresource),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpcbbuffer),
            ::std::mem::transmute(lplpsystem),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetGetResourceParentA(
    lpnetresource: *const NETRESOURCEA,
    lpbuffer: *mut ::std::ffi::c_void,
    lpcbbuffer: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetGetResourceParentA(
                lpnetresource: *const NETRESOURCEA,
                lpbuffer: *mut ::std::ffi::c_void,
                lpcbbuffer: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetGetResourceParentA(
            ::std::mem::transmute(lpnetresource),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpcbbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetGetResourceParentW(
    lpnetresource: *const NETRESOURCEW,
    lpbuffer: *mut ::std::ffi::c_void,
    lpcbbuffer: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetGetResourceParentW(
                lpnetresource: *const NETRESOURCEW,
                lpbuffer: *mut ::std::ffi::c_void,
                lpcbbuffer: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetGetResourceParentW(
            ::std::mem::transmute(lpnetresource),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpcbbuffer),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetGetUniversalNameA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lplocalpath: Param0,
    dwinfolevel: UNC_INFO_LEVEL,
    lpbuffer: *mut ::std::ffi::c_void,
    lpbuffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetGetUniversalNameA(
                lplocalpath: super::super::Foundation::PSTR,
                dwinfolevel: UNC_INFO_LEVEL,
                lpbuffer: *mut ::std::ffi::c_void,
                lpbuffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetGetUniversalNameA(
            lplocalpath.into_param().abi(),
            ::std::mem::transmute(dwinfolevel),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetGetUniversalNameW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lplocalpath: Param0,
    dwinfolevel: UNC_INFO_LEVEL,
    lpbuffer: *mut ::std::ffi::c_void,
    lpbuffersize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetGetUniversalNameW(
                lplocalpath: super::super::Foundation::PWSTR,
                dwinfolevel: UNC_INFO_LEVEL,
                lpbuffer: *mut ::std::ffi::c_void,
                lpbuffersize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetGetUniversalNameW(
            lplocalpath.into_param().abi(),
            ::std::mem::transmute(dwinfolevel),
            ::std::mem::transmute(lpbuffer),
            ::std::mem::transmute(lpbuffersize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetGetUserA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    lpname: Param0,
    lpusername: super::super::Foundation::PSTR,
    lpnlength: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetGetUserA(
                lpname: super::super::Foundation::PSTR,
                lpusername: super::super::Foundation::PSTR,
                lpnlength: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetGetUserA(
            lpname.into_param().abi(),
            ::std::mem::transmute(lpusername),
            ::std::mem::transmute(lpnlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetGetUserW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    lpname: Param0,
    lpusername: super::super::Foundation::PWSTR,
    lpnlength: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetGetUserW(
                lpname: super::super::Foundation::PWSTR,
                lpusername: super::super::Foundation::PWSTR,
                lpnlength: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetGetUserW(
            lpname.into_param().abi(),
            ::std::mem::transmute(lpusername),
            ::std::mem::transmute(lpnlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetOpenEnumA(
    dwscope: NET_RESOURCE_SCOPE,
    dwtype: NET_RESOURCE_TYPE,
    dwusage: WNET_OPEN_ENUM_USAGE,
    lpnetresource: *const NETRESOURCEA,
    lphenum: *mut NetEnumHandle,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetOpenEnumA(
                dwscope: NET_RESOURCE_SCOPE,
                dwtype: NET_RESOURCE_TYPE,
                dwusage: WNET_OPEN_ENUM_USAGE,
                lpnetresource: *const NETRESOURCEA,
                lphenum: *mut NetEnumHandle,
            ) -> u32;
        }
        ::std::mem::transmute(WNetOpenEnumA(
            ::std::mem::transmute(dwscope),
            ::std::mem::transmute(dwtype),
            ::std::mem::transmute(dwusage),
            ::std::mem::transmute(lpnetresource),
            ::std::mem::transmute(lphenum),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetOpenEnumW(
    dwscope: NET_RESOURCE_SCOPE,
    dwtype: NET_RESOURCE_TYPE,
    dwusage: WNET_OPEN_ENUM_USAGE,
    lpnetresource: *const NETRESOURCEW,
    lphenum: *mut NetEnumHandle,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetOpenEnumW(
                dwscope: NET_RESOURCE_SCOPE,
                dwtype: NET_RESOURCE_TYPE,
                dwusage: WNET_OPEN_ENUM_USAGE,
                lpnetresource: *const NETRESOURCEW,
                lphenum: *mut NetEnumHandle,
            ) -> u32;
        }
        ::std::mem::transmute(WNetOpenEnumW(
            ::std::mem::transmute(dwscope),
            ::std::mem::transmute(dwtype),
            ::std::mem::transmute(dwusage),
            ::std::mem::transmute(lpnetresource),
            ::std::mem::transmute(lphenum),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetSetLastErrorA<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    err: u32,
    lperror: Param1,
    lpproviders: Param2,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetSetLastErrorA(
                err: u32,
                lperror: super::super::Foundation::PSTR,
                lpproviders: super::super::Foundation::PSTR,
            );
        }
        ::std::mem::transmute(WNetSetLastErrorA(
            ::std::mem::transmute(err),
            lperror.into_param().abi(),
            lpproviders.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetSetLastErrorW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    err: u32,
    lperror: Param1,
    lpproviders: Param2,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetSetLastErrorW(
                err: u32,
                lperror: super::super::Foundation::PWSTR,
                lpproviders: super::super::Foundation::PWSTR,
            );
        }
        ::std::mem::transmute(WNetSetLastErrorW(
            ::std::mem::transmute(err),
            lperror.into_param().abi(),
            lpproviders.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetUseConnection4A<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwndowner: Param0,
    lpnetresource: *const NETRESOURCEA,
    pauthbuffer: *const ::std::ffi::c_void,
    cbauthbuffer: u32,
    dwflags: u32,
    lpuseoptions: *const u8,
    cbuseoptions: u32,
    lpaccessname: super::super::Foundation::PSTR,
    lpbuffersize: *mut u32,
    lpresult: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetUseConnection4A(
                hwndowner: super::super::Foundation::HWND,
                lpnetresource: *const NETRESOURCEA,
                pauthbuffer: *const ::std::ffi::c_void,
                cbauthbuffer: u32,
                dwflags: u32,
                lpuseoptions: *const u8,
                cbuseoptions: u32,
                lpaccessname: super::super::Foundation::PSTR,
                lpbuffersize: *mut u32,
                lpresult: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetUseConnection4A(
            hwndowner.into_param().abi(),
            ::std::mem::transmute(lpnetresource),
            ::std::mem::transmute(pauthbuffer),
            ::std::mem::transmute(cbauthbuffer),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpuseoptions),
            ::std::mem::transmute(cbuseoptions),
            ::std::mem::transmute(lpaccessname),
            ::std::mem::transmute(lpbuffersize),
            ::std::mem::transmute(lpresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetUseConnection4W<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwndowner: Param0,
    lpnetresource: *const NETRESOURCEW,
    pauthbuffer: *const ::std::ffi::c_void,
    cbauthbuffer: u32,
    dwflags: u32,
    lpuseoptions: *const u8,
    cbuseoptions: u32,
    lpaccessname: super::super::Foundation::PWSTR,
    lpbuffersize: *mut u32,
    lpresult: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetUseConnection4W(
                hwndowner: super::super::Foundation::HWND,
                lpnetresource: *const NETRESOURCEW,
                pauthbuffer: *const ::std::ffi::c_void,
                cbauthbuffer: u32,
                dwflags: u32,
                lpuseoptions: *const u8,
                cbuseoptions: u32,
                lpaccessname: super::super::Foundation::PWSTR,
                lpbuffersize: *mut u32,
                lpresult: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetUseConnection4W(
            hwndowner.into_param().abi(),
            ::std::mem::transmute(lpnetresource),
            ::std::mem::transmute(pauthbuffer),
            ::std::mem::transmute(cbauthbuffer),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpuseoptions),
            ::std::mem::transmute(cbuseoptions),
            ::std::mem::transmute(lpaccessname),
            ::std::mem::transmute(lpbuffersize),
            ::std::mem::transmute(lpresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetUseConnectionA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hwndowner: Param0,
    lpnetresource: *const NETRESOURCEA,
    lppassword: Param2,
    lpuserid: Param3,
    dwflags: NET_USE_CONNECT_FLAGS,
    lpaccessname: super::super::Foundation::PSTR,
    lpbuffersize: *mut u32,
    lpresult: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetUseConnectionA(
                hwndowner: super::super::Foundation::HWND,
                lpnetresource: *const NETRESOURCEA,
                lppassword: super::super::Foundation::PSTR,
                lpuserid: super::super::Foundation::PSTR,
                dwflags: NET_USE_CONNECT_FLAGS,
                lpaccessname: super::super::Foundation::PSTR,
                lpbuffersize: *mut u32,
                lpresult: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetUseConnectionA(
            hwndowner.into_param().abi(),
            ::std::mem::transmute(lpnetresource),
            lppassword.into_param().abi(),
            lpuserid.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpaccessname),
            ::std::mem::transmute(lpbuffersize),
            ::std::mem::transmute(lpresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn WNetUseConnectionW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hwndowner: Param0,
    lpnetresource: *const NETRESOURCEW,
    lppassword: Param2,
    lpuserid: Param3,
    dwflags: NET_USE_CONNECT_FLAGS,
    lpaccessname: super::super::Foundation::PWSTR,
    lpbuffersize: *mut u32,
    lpresult: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WNetUseConnectionW(
                hwndowner: super::super::Foundation::HWND,
                lpnetresource: *const NETRESOURCEW,
                lppassword: super::super::Foundation::PWSTR,
                lpuserid: super::super::Foundation::PWSTR,
                dwflags: NET_USE_CONNECT_FLAGS,
                lpaccessname: super::super::Foundation::PWSTR,
                lpbuffersize: *mut u32,
                lpresult: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(WNetUseConnectionW(
            hwndowner.into_param().abi(),
            ::std::mem::transmute(lpnetresource),
            lppassword.into_param().abi(),
            lpuserid.into_param().abi(),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(lpaccessname),
            ::std::mem::transmute(lpbuffersize),
            ::std::mem::transmute(lpresult),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
