#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
pub const CDB_REPORT_BITS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
pub const CDB_REPORT_BYTES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
pub const COMDB_MAX_PORTS_ARBITRATED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
pub const COMDB_MIN_PORTS_ARBITRATED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
#[inline]
pub unsafe fn ComDBClaimNextFreePort<'a, P0>(hcomdb: P0, comnumber: *mut u32) -> i32
where
    P0: ::std::convert::Into<HCOMDB>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ComDBClaimNextFreePort(hcomdb: HCOMDB, comnumber: *mut u32) -> i32;
    }
    ComDBClaimNextFreePort(hcomdb.into(), ::core::mem::transmute(comnumber))
}
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ComDBClaimPort<'a, P0, P1>(hcomdb: P0, comnumber: u32, forceclaim: P1, forced: *mut super::super::Foundation::BOOL) -> i32
where
    P0: ::std::convert::Into<HCOMDB>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ComDBClaimPort(hcomdb: HCOMDB, comnumber: u32, forceclaim: super::super::Foundation::BOOL, forced: *mut super::super::Foundation::BOOL) -> i32;
    }
    ComDBClaimPort(hcomdb.into(), comnumber, forceclaim.into(), ::core::mem::transmute(forced))
}
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
#[inline]
pub unsafe fn ComDBClose<'a, P0>(hcomdb: P0) -> i32
where
    P0: ::std::convert::Into<HCOMDB>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ComDBClose(hcomdb: HCOMDB) -> i32;
    }
    ComDBClose(hcomdb.into())
}
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
#[inline]
pub unsafe fn ComDBGetCurrentPortUsage<'a, P0>(hcomdb: P0, buffer: *mut u8, buffersize: u32, reporttype: u32, maxportsreported: *mut u32) -> i32
where
    P0: ::std::convert::Into<HCOMDB>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ComDBGetCurrentPortUsage(hcomdb: HCOMDB, buffer: *mut u8, buffersize: u32, reporttype: u32, maxportsreported: *mut u32) -> i32;
    }
    ComDBGetCurrentPortUsage(hcomdb.into(), ::core::mem::transmute(buffer), buffersize, reporttype, ::core::mem::transmute(maxportsreported))
}
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
#[inline]
pub unsafe fn ComDBOpen(phcomdb: *mut isize) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ComDBOpen(phcomdb: *mut isize) -> i32;
    }
    ComDBOpen(::core::mem::transmute(phcomdb))
}
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
#[inline]
pub unsafe fn ComDBReleasePort<'a, P0>(hcomdb: P0, comnumber: u32) -> i32
where
    P0: ::std::convert::Into<HCOMDB>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ComDBReleasePort(hcomdb: HCOMDB, comnumber: u32) -> i32;
    }
    ComDBReleasePort(hcomdb.into(), comnumber)
}
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
#[inline]
pub unsafe fn ComDBResizeDatabase<'a, P0>(hcomdb: P0, newsize: u32) -> i32
where
    P0: ::std::convert::Into<HCOMDB>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ComDBResizeDatabase(hcomdb: HCOMDB, newsize: u32) -> i32;
    }
    ComDBResizeDatabase(hcomdb.into(), newsize)
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCOMDB(pub isize);
impl HCOMDB {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HCOMDB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCOMDB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCOMDB {}
impl ::core::fmt::Debug for HCOMDB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCOMDB").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HCOMDB>> for HCOMDB {
    fn from(optional: ::core::option::Option<HCOMDB>) -> HCOMDB {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HCOMDB {
    type Abi = Self;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
