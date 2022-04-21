#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const ACCESSIBILITY_SETTING: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const APPLICATION_RUN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const BACKUP: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const BACKUP_RECOVERY: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const BEGIN_NESTED_SYSTEM_CHANGE_NORP: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const CHECKPOINT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const CRITICAL_UPDATE: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const DESKTOP_SETTING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const FIRSTRUN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const MANUAL_CHECKPOINT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const MAX_DESC: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const MAX_DESC_W: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const MAX_EVENT: u32 = 104u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const MAX_RPT: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const MIN_EVENT: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const MIN_RPT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const OE_SETTING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const RESTORE: u32 = 6u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Restore\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESTOREPOINTINFOA {
    pub dwEventType: RESTOREPOINTINFO_EVENT_TYPE,
    pub dwRestorePtType: RESTOREPOINTINFO_TYPE,
    pub llSequenceNumber: i64,
    pub szDescription: [super::super::Foundation::CHAR; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESTOREPOINTINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESTOREPOINTINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RESTOREPOINTINFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESTOREPOINTINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESTOREPOINTINFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESTOREPOINTINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESTOREPOINTINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub struct RESTOREPOINTINFOW {
    pub dwEventType: RESTOREPOINTINFO_EVENT_TYPE,
    pub dwRestorePtType: RESTOREPOINTINFO_TYPE,
    pub llSequenceNumber: i64,
    pub szDescription: [u16; 256],
}
impl ::core::marker::Copy for RESTOREPOINTINFOW {}
impl ::core::clone::Clone for RESTOREPOINTINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RESTOREPOINTINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RESTOREPOINTINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RESTOREPOINTINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RESTOREPOINTINFOW {}
impl ::core::default::Default for RESTOREPOINTINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RESTOREPOINTINFO_EVENT_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const BEGIN_NESTED_SYSTEM_CHANGE: RESTOREPOINTINFO_EVENT_TYPE = RESTOREPOINTINFO_EVENT_TYPE(102u32);
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const BEGIN_SYSTEM_CHANGE: RESTOREPOINTINFO_EVENT_TYPE = RESTOREPOINTINFO_EVENT_TYPE(100u32);
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const END_NESTED_SYSTEM_CHANGE: RESTOREPOINTINFO_EVENT_TYPE = RESTOREPOINTINFO_EVENT_TYPE(103u32);
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const END_SYSTEM_CHANGE: RESTOREPOINTINFO_EVENT_TYPE = RESTOREPOINTINFO_EVENT_TYPE(101u32);
impl ::core::marker::Copy for RESTOREPOINTINFO_EVENT_TYPE {}
impl ::core::clone::Clone for RESTOREPOINTINFO_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RESTOREPOINTINFO_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RESTOREPOINTINFO_EVENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RESTOREPOINTINFO_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RESTOREPOINTINFO_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RESTOREPOINTINFO_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const APPLICATION_INSTALL: RESTOREPOINTINFO_TYPE = RESTOREPOINTINFO_TYPE(0u32);
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const APPLICATION_UNINSTALL: RESTOREPOINTINFO_TYPE = RESTOREPOINTINFO_TYPE(1u32);
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const DEVICE_DRIVER_INSTALL: RESTOREPOINTINFO_TYPE = RESTOREPOINTINFO_TYPE(10u32);
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const MODIFY_SETTINGS: RESTOREPOINTINFO_TYPE = RESTOREPOINTINFO_TYPE(12u32);
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const CANCELLED_OPERATION: RESTOREPOINTINFO_TYPE = RESTOREPOINTINFO_TYPE(13u32);
impl ::core::marker::Copy for RESTOREPOINTINFO_TYPE {}
impl ::core::clone::Clone for RESTOREPOINTINFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RESTOREPOINTINFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RESTOREPOINTINFO_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RESTOREPOINTINFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RESTOREPOINTINFO_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Restore\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SRSetRestorePointA(prestoreptspec: *const RESTOREPOINTINFOA, psmgrstatus: *mut STATEMGRSTATUS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SRSetRestorePointA(prestoreptspec: *const RESTOREPOINTINFOA, psmgrstatus: *mut STATEMGRSTATUS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SRSetRestorePointA(::core::mem::transmute(prestoreptspec), ::core::mem::transmute(psmgrstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Restore\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SRSetRestorePointW(prestoreptspec: *const RESTOREPOINTINFOW, psmgrstatus: *mut STATEMGRSTATUS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SRSetRestorePointW(prestoreptspec: *const RESTOREPOINTINFOW, psmgrstatus: *mut STATEMGRSTATUS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SRSetRestorePointW(::core::mem::transmute(prestoreptspec), ::core::mem::transmute(psmgrstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub struct STATEMGRSTATUS {
    pub nStatus: u32,
    pub llSequenceNumber: i64,
}
impl ::core::marker::Copy for STATEMGRSTATUS {}
impl ::core::clone::Clone for STATEMGRSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for STATEMGRSTATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for STATEMGRSTATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STATEMGRSTATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for STATEMGRSTATUS {}
impl ::core::default::Default for STATEMGRSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const WINDOWS_BOOT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const WINDOWS_SHUTDOWN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Restore\"`*"]
pub const WINDOWS_UPDATE: u32 = 17u32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_Restore\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct _RESTOREPTINFOEX {
    pub ftCreation: super::super::Foundation::FILETIME,
    pub dwEventType: u32,
    pub dwRestorePtType: u32,
    pub dwRPNum: u32,
    pub szDescription: [u16; 256],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _RESTOREPTINFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _RESTOREPTINFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for _RESTOREPTINFOEX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for _RESTOREPTINFOEX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_RESTOREPTINFOEX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for _RESTOREPTINFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for _RESTOREPTINFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
