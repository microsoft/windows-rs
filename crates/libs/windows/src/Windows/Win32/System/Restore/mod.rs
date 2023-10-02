#[inline]
pub unsafe fn SRRemoveRestorePoint(dwrpnum: u32) -> u32 {
    ::windows_targets::link!("srclient.dll" "system" fn SRRemoveRestorePoint(dwrpnum : u32) -> u32);
    SRRemoveRestorePoint(dwrpnum)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SRSetRestorePointA(prestoreptspec: *const RESTOREPOINTINFOA, psmgrstatus: *mut STATEMGRSTATUS) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("sfc.dll" "system" fn SRSetRestorePointA(prestoreptspec : *const RESTOREPOINTINFOA, psmgrstatus : *mut STATEMGRSTATUS) -> super::super::Foundation:: BOOL);
    SRSetRestorePointA(prestoreptspec, psmgrstatus)
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SRSetRestorePointW(prestoreptspec: *const RESTOREPOINTINFOW, psmgrstatus: *mut STATEMGRSTATUS) -> super::super::Foundation::BOOL {
    ::windows_targets::link!("sfc.dll" "system" fn SRSetRestorePointW(prestoreptspec : *const RESTOREPOINTINFOW, psmgrstatus : *mut STATEMGRSTATUS) -> super::super::Foundation:: BOOL);
    SRSetRestorePointW(prestoreptspec, psmgrstatus)
}
pub const ACCESSIBILITY_SETTING: u32 = 3u32;
pub const APPLICATION_INSTALL: RESTOREPOINTINFO_TYPE = RESTOREPOINTINFO_TYPE(0u32);
pub const APPLICATION_RUN: u32 = 5u32;
pub const APPLICATION_UNINSTALL: RESTOREPOINTINFO_TYPE = RESTOREPOINTINFO_TYPE(1u32);
pub const BACKUP: u32 = 15u32;
pub const BACKUP_RECOVERY: u32 = 14u32;
pub const BEGIN_NESTED_SYSTEM_CHANGE: RESTOREPOINTINFO_EVENT_TYPE = RESTOREPOINTINFO_EVENT_TYPE(102u32);
pub const BEGIN_NESTED_SYSTEM_CHANGE_NORP: u32 = 104u32;
pub const BEGIN_SYSTEM_CHANGE: RESTOREPOINTINFO_EVENT_TYPE = RESTOREPOINTINFO_EVENT_TYPE(100u32);
pub const CANCELLED_OPERATION: RESTOREPOINTINFO_TYPE = RESTOREPOINTINFO_TYPE(13u32);
pub const CHECKPOINT: u32 = 7u32;
pub const CRITICAL_UPDATE: u32 = 18u32;
pub const DESKTOP_SETTING: u32 = 2u32;
pub const DEVICE_DRIVER_INSTALL: RESTOREPOINTINFO_TYPE = RESTOREPOINTINFO_TYPE(10u32);
pub const END_NESTED_SYSTEM_CHANGE: RESTOREPOINTINFO_EVENT_TYPE = RESTOREPOINTINFO_EVENT_TYPE(103u32);
pub const END_SYSTEM_CHANGE: RESTOREPOINTINFO_EVENT_TYPE = RESTOREPOINTINFO_EVENT_TYPE(101u32);
pub const FIRSTRUN: u32 = 11u32;
pub const MANUAL_CHECKPOINT: u32 = 16u32;
pub const MAX_DESC: u32 = 64u32;
pub const MAX_DESC_W: u32 = 256u32;
pub const MAX_EVENT: u32 = 104u32;
pub const MAX_RPT: u32 = 18u32;
pub const MIN_EVENT: u32 = 100u32;
pub const MIN_RPT: u32 = 0u32;
pub const MODIFY_SETTINGS: RESTOREPOINTINFO_TYPE = RESTOREPOINTINFO_TYPE(12u32);
pub const OE_SETTING: u32 = 4u32;
pub const RESTORE: u32 = 6u32;
pub const WINDOWS_BOOT: u32 = 9u32;
pub const WINDOWS_SHUTDOWN: u32 = 8u32;
pub const WINDOWS_UPDATE: u32 = 17u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RESTOREPOINTINFO_EVENT_TYPE(pub u32);
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
impl ::windows_core::TypeKind for RESTOREPOINTINFO_EVENT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RESTOREPOINTINFO_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RESTOREPOINTINFO_EVENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RESTOREPOINTINFO_TYPE(pub u32);
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
impl ::windows_core::TypeKind for RESTOREPOINTINFO_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RESTOREPOINTINFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RESTOREPOINTINFO_TYPE").field(&self.0).finish()
    }
}
#[repr(C, packed(1))]
pub struct RESTOREPOINTINFOA {
    pub dwEventType: RESTOREPOINTINFO_EVENT_TYPE,
    pub dwRestorePtType: RESTOREPOINTINFO_TYPE,
    pub llSequenceNumber: i64,
    pub szDescription: [u8; 64],
}
impl ::core::marker::Copy for RESTOREPOINTINFOA {}
impl ::core::clone::Clone for RESTOREPOINTINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for RESTOREPOINTINFOA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for RESTOREPOINTINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESTOREPOINTINFOEX {
    pub ftCreation: super::super::Foundation::FILETIME,
    pub dwEventType: u32,
    pub dwRestorePtType: u32,
    pub dwRPNum: u32,
    pub szDescription: [u16; 256],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESTOREPOINTINFOEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESTOREPOINTINFOEX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for RESTOREPOINTINFOEX {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESTOREPOINTINFOEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
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
impl ::windows_core::TypeKind for RESTOREPOINTINFOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for RESTOREPOINTINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct STATEMGRSTATUS {
    pub nStatus: super::super::Foundation::WIN32_ERROR,
    pub llSequenceNumber: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STATEMGRSTATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STATEMGRSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for STATEMGRSTATUS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STATEMGRSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
