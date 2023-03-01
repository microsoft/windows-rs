#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
#[inline]
pub unsafe fn ComDBClaimNextFreePort<P0>(hcomdb: P0, comnumber: *mut u32) -> i32
where
    P0: ::windows::core::IntoParam<HCOMDB>,
{
    ::windows::imp::link ! ( "msports.dll""system" fn ComDBClaimNextFreePort ( hcomdb : HCOMDB , comnumber : *mut u32 ) -> i32 );
    ComDBClaimNextFreePort(hcomdb.into_param().abi(), comnumber)
}
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ComDBClaimPort<P0, P1>(hcomdb: P0, comnumber: u32, forceclaim: P1, forced: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> i32
where
    P0: ::windows::core::IntoParam<HCOMDB>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows::imp::link ! ( "msports.dll""system" fn ComDBClaimPort ( hcomdb : HCOMDB , comnumber : u32 , forceclaim : super::super::Foundation:: BOOL , forced : *mut super::super::Foundation:: BOOL ) -> i32 );
    ComDBClaimPort(hcomdb.into_param().abi(), comnumber, forceclaim.into_param().abi(), ::core::mem::transmute(forced.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
#[inline]
pub unsafe fn ComDBClose<P0>(hcomdb: P0) -> i32
where
    P0: ::windows::core::IntoParam<HCOMDB>,
{
    ::windows::imp::link ! ( "msports.dll""system" fn ComDBClose ( hcomdb : HCOMDB ) -> i32 );
    ComDBClose(hcomdb.into_param().abi())
}
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
#[inline]
pub unsafe fn ComDBGetCurrentPortUsage<P0>(hcomdb: P0, buffer: ::core::option::Option<&mut [u8]>, reporttype: u32, maxportsreported: ::core::option::Option<*mut u32>) -> i32
where
    P0: ::windows::core::IntoParam<HCOMDB>,
{
    ::windows::imp::link ! ( "msports.dll""system" fn ComDBGetCurrentPortUsage ( hcomdb : HCOMDB , buffer : *mut u8 , buffersize : u32 , reporttype : u32 , maxportsreported : *mut u32 ) -> i32 );
    ComDBGetCurrentPortUsage(hcomdb.into_param().abi(), ::core::mem::transmute(buffer.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), buffer.as_deref().map_or(0, |slice| slice.len() as _), reporttype, ::core::mem::transmute(maxportsreported.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
#[inline]
pub unsafe fn ComDBOpen(phcomdb: *mut isize) -> i32 {
    ::windows::imp::link ! ( "msports.dll""system" fn ComDBOpen ( phcomdb : *mut isize ) -> i32 );
    ComDBOpen(phcomdb)
}
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
#[inline]
pub unsafe fn ComDBReleasePort<P0>(hcomdb: P0, comnumber: u32) -> i32
where
    P0: ::windows::core::IntoParam<HCOMDB>,
{
    ::windows::imp::link ! ( "msports.dll""system" fn ComDBReleasePort ( hcomdb : HCOMDB , comnumber : u32 ) -> i32 );
    ComDBReleasePort(hcomdb.into_param().abi(), comnumber)
}
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
#[inline]
pub unsafe fn ComDBResizeDatabase<P0>(hcomdb: P0, newsize: u32) -> i32
where
    P0: ::windows::core::IntoParam<HCOMDB>,
{
    ::windows::imp::link ! ( "msports.dll""system" fn ComDBResizeDatabase ( hcomdb : HCOMDB , newsize : u32 ) -> i32 );
    ComDBResizeDatabase(hcomdb.into_param().abi(), newsize)
}
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
pub const CDB_REPORT_BITS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
pub const CDB_REPORT_BYTES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
pub const COMDB_MAX_PORTS_ARBITRATED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Devices_SerialCommunication\"`*"]
pub const COMDB_MIN_PORTS_ARBITRATED: u32 = 256u32;
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
impl ::windows::core::TypeKind for HCOMDB {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
