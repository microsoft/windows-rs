#[doc = "*Required features: `\"Win32_System_UserAccessLogging\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn UalInstrument(data: *const UAL_DATA_BLOB) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ualapi.dll""system" fn UalInstrument ( data : *const UAL_DATA_BLOB ) -> :: windows::core::HRESULT );
    UalInstrument(data).ok()
}
#[doc = "*Required features: `\"Win32_System_UserAccessLogging\"`*"]
#[inline]
pub unsafe fn UalRegisterProduct<P0, P1, P2>(wszproductname: P0, wszrolename: P1, wszguid: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "ualapi.dll""system" fn UalRegisterProduct ( wszproductname : :: windows::core::PCWSTR , wszrolename : :: windows::core::PCWSTR , wszguid : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    UalRegisterProduct(wszproductname.into().abi(), wszrolename.into().abi(), wszguid.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_UserAccessLogging\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn UalStart(data: *const UAL_DATA_BLOB) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ualapi.dll""system" fn UalStart ( data : *const UAL_DATA_BLOB ) -> :: windows::core::HRESULT );
    UalStart(data).ok()
}
#[doc = "*Required features: `\"Win32_System_UserAccessLogging\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn UalStop(data: *const UAL_DATA_BLOB) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "ualapi.dll""system" fn UalStop ( data : *const UAL_DATA_BLOB ) -> :: windows::core::HRESULT );
    UalStop(data).ok()
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_UserAccessLogging\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct UAL_DATA_BLOB {
    pub Size: u32,
    pub RoleGuid: ::windows::core::GUID,
    pub TenantId: ::windows::core::GUID,
    pub Address: super::super::Networking::WinSock::SOCKADDR_STORAGE,
    pub UserName: [u16; 260],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for UAL_DATA_BLOB {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for UAL_DATA_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for UAL_DATA_BLOB {
    type Abi = Self;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
