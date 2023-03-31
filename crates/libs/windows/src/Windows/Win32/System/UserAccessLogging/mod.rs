#[doc = "*Required features: `\"Win32_System_UserAccessLogging\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn UalInstrument(data: *const UAL_DATA_BLOB) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ualapi.dll""system" fn UalInstrument ( data : *const UAL_DATA_BLOB ) -> ::windows::core::HRESULT );
    UalInstrument(data).ok()
}
#[doc = "*Required features: `\"Win32_System_UserAccessLogging\"`*"]
#[inline]
pub unsafe fn UalRegisterProduct<P0, P1, P2>(wszproductname: P0, wszrolename: P1, wszguid: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "ualapi.dll""system" fn UalRegisterProduct ( wszproductname : ::windows::core::PCWSTR , wszrolename : ::windows::core::PCWSTR , wszguid : ::windows::core::PCWSTR ) -> ::windows::core::HRESULT );
    UalRegisterProduct(wszproductname.into_param().abi(), wszrolename.into_param().abi(), wszguid.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_UserAccessLogging\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn UalStart(data: *const UAL_DATA_BLOB) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ualapi.dll""system" fn UalStart ( data : *const UAL_DATA_BLOB ) -> ::windows::core::HRESULT );
    UalStart(data).ok()
}
#[doc = "*Required features: `\"Win32_System_UserAccessLogging\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn UalStop(data: *const UAL_DATA_BLOB) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ualapi.dll""system" fn UalStop ( data : *const UAL_DATA_BLOB ) -> ::windows::core::HRESULT );
    UalStop(data).ok()
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_UserAccessLogging\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct UAL_DATA_BLOB {
    pub Size: u32,
    pub RoleGuid: ::windows::core::GUID,
    pub TenantId: ::windows::core::GUID,
    pub Address: super::super::Networking::WinSock::SOCKADDR_STORAGE,
    pub UserName: [u16; 260],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for UAL_DATA_BLOB {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for UAL_DATA_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for UAL_DATA_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UAL_DATA_BLOB").field("Size", &self.Size).field("RoleGuid", &self.RoleGuid).field("TenantId", &self.TenantId).field("Address", &self.Address).field("UserName", &self.UserName).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows::core::TypeKind for UAL_DATA_BLOB {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for UAL_DATA_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.RoleGuid == other.RoleGuid && self.TenantId == other.TenantId && self.Address == other.Address && self.UserName == other.UserName
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for UAL_DATA_BLOB {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for UAL_DATA_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
