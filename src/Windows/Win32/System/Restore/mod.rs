#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const ACCESSIBILITY_SETTING: u32 = 3u32;
pub const APPLICATION_RUN: u32 = 5u32;
pub const BACKUP: u32 = 15u32;
pub const BACKUP_RECOVERY: u32 = 14u32;
pub const BEGIN_NESTED_SYSTEM_CHANGE_NORP: u32 = 104u32;
pub const CHECKPOINT: u32 = 7u32;
pub const CRITICAL_UPDATE: u32 = 18u32;
pub const DESKTOP_SETTING: u32 = 2u32;
pub const FIRSTRUN: u32 = 11u32;
pub const MANUAL_CHECKPOINT: u32 = 16u32;
pub const MAX_DESC: u32 = 64u32;
pub const MAX_DESC_W: u32 = 256u32;
pub const MAX_EVENT: u32 = 104u32;
pub const MAX_RPT: u32 = 18u32;
pub const MIN_EVENT: u32 = 100u32;
pub const MIN_RPT: u32 = 0u32;
pub const OE_SETTING: u32 = 4u32;
pub const RESTORE: u32 = 6u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct RESTOREPOINTINFOA {
    pub dwEventType: RESTOREPOINTINFO_EVENT_TYPE,
    pub dwRestorePtType: RESTOREPOINTINFO_TYPE,
    pub llSequenceNumber: i64,
    pub szDescription: [super::SystemServices::CHAR; 64],
}
#[cfg(feature = "Win32_System_SystemServices")]
impl RESTOREPOINTINFOA {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for RESTOREPOINTINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for RESTOREPOINTINFOA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for RESTOREPOINTINFOA {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for RESTOREPOINTINFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct RESTOREPOINTINFOW {
    pub dwEventType: RESTOREPOINTINFO_EVENT_TYPE,
    pub dwRestorePtType: RESTOREPOINTINFO_TYPE,
    pub llSequenceNumber: i64,
    pub szDescription: [u16; 256],
}
impl RESTOREPOINTINFOW {}
impl ::std::default::Default for RESTOREPOINTINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RESTOREPOINTINFOW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RESTOREPOINTINFOW {}
unsafe impl ::windows::runtime::Abi for RESTOREPOINTINFOW {
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
pub struct RESTOREPOINTINFO_EVENT_TYPE(pub u32);
pub const BEGIN_NESTED_SYSTEM_CHANGE: RESTOREPOINTINFO_EVENT_TYPE =
    RESTOREPOINTINFO_EVENT_TYPE(102u32);
pub const BEGIN_SYSTEM_CHANGE: RESTOREPOINTINFO_EVENT_TYPE = RESTOREPOINTINFO_EVENT_TYPE(100u32);
pub const END_NESTED_SYSTEM_CHANGE: RESTOREPOINTINFO_EVENT_TYPE =
    RESTOREPOINTINFO_EVENT_TYPE(103u32);
pub const END_SYSTEM_CHANGE: RESTOREPOINTINFO_EVENT_TYPE = RESTOREPOINTINFO_EVENT_TYPE(101u32);
impl ::std::convert::From<u32> for RESTOREPOINTINFO_EVENT_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RESTOREPOINTINFO_EVENT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for RESTOREPOINTINFO_EVENT_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RESTOREPOINTINFO_EVENT_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RESTOREPOINTINFO_EVENT_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RESTOREPOINTINFO_EVENT_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RESTOREPOINTINFO_EVENT_TYPE {
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
pub struct RESTOREPOINTINFO_TYPE(pub u32);
pub const APPLICATION_INSTALL: RESTOREPOINTINFO_TYPE = RESTOREPOINTINFO_TYPE(0u32);
pub const APPLICATION_UNINSTALL: RESTOREPOINTINFO_TYPE = RESTOREPOINTINFO_TYPE(1u32);
pub const DEVICE_DRIVER_INSTALL: RESTOREPOINTINFO_TYPE = RESTOREPOINTINFO_TYPE(10u32);
pub const MODIFY_SETTINGS: RESTOREPOINTINFO_TYPE = RESTOREPOINTINFO_TYPE(12u32);
pub const CANCELLED_OPERATION: RESTOREPOINTINFO_TYPE = RESTOREPOINTINFO_TYPE(13u32);
impl ::std::convert::From<u32> for RESTOREPOINTINFO_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RESTOREPOINTINFO_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for RESTOREPOINTINFO_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RESTOREPOINTINFO_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RESTOREPOINTINFO_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RESTOREPOINTINFO_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RESTOREPOINTINFO_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn SRSetRestorePointA(
    prestoreptspec: *const RESTOREPOINTINFOA,
    psmgrstatus: *mut STATEMGRSTATUS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SRSetRestorePointA(
                prestoreptspec: *const RESTOREPOINTINFOA,
                psmgrstatus: *mut STATEMGRSTATUS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SRSetRestorePointA(
            ::std::mem::transmute(prestoreptspec),
            ::std::mem::transmute(psmgrstatus),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SRSetRestorePointW(
    prestoreptspec: *const RESTOREPOINTINFOW,
    psmgrstatus: *mut STATEMGRSTATUS,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SRSetRestorePointW(
                prestoreptspec: *const RESTOREPOINTINFOW,
                psmgrstatus: *mut STATEMGRSTATUS,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SRSetRestorePointW(
            ::std::mem::transmute(prestoreptspec),
            ::std::mem::transmute(psmgrstatus),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
pub struct STATEMGRSTATUS {
    pub nStatus: u32,
    pub llSequenceNumber: i64,
}
impl STATEMGRSTATUS {}
impl ::std::default::Default for STATEMGRSTATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for STATEMGRSTATUS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for STATEMGRSTATUS {}
unsafe impl ::windows::runtime::Abi for STATEMGRSTATUS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WINDOWS_BOOT: u32 = 9u32;
pub const WINDOWS_SHUTDOWN: u32 = 8u32;
pub const WINDOWS_UPDATE: u32 = 17u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(1))]
#[cfg(feature = "Win32_Foundation")]
pub struct _RESTOREPTINFOEX {
    pub ftCreation: super::super::Foundation::FILETIME,
    pub dwEventType: u32,
    pub dwRestorePtType: u32,
    pub dwRPNum: u32,
    pub szDescription: [u16; 256],
}
#[cfg(feature = "Win32_Foundation")]
impl _RESTOREPTINFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for _RESTOREPTINFOEX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for _RESTOREPTINFOEX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for _RESTOREPTINFOEX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for _RESTOREPTINFOEX {
    type Abi = Self;
    type DefaultType = Self;
}
