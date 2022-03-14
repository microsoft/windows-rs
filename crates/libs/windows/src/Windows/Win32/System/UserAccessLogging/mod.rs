#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
impl ::core::fmt::Debug for UAL_DATA_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UAL_DATA_BLOB").field("Size", &self.Size).field("RoleGuid", &self.RoleGuid).field("TenantId", &self.TenantId).field("Address", &self.Address).field("UserName", &self.UserName).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for UAL_DATA_BLOB {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for UAL_DATA_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UAL_DATA_BLOB>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for UAL_DATA_BLOB {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for UAL_DATA_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_UserAccessLogging\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn UalInstrument(data: *const UAL_DATA_BLOB) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UalInstrument(data: *const UAL_DATA_BLOB) -> ::windows::core::HRESULT;
        }
        UalInstrument(::core::mem::transmute(data)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_UserAccessLogging\"`*"]
#[inline]
pub unsafe fn UalRegisterProduct<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(wszproductname: Param0, wszrolename: Param1, wszguid: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UalRegisterProduct(wszproductname: ::windows::core::PCWSTR, wszrolename: ::windows::core::PCWSTR, wszguid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        UalRegisterProduct(wszproductname.into_param().abi(), wszrolename.into_param().abi(), wszguid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_UserAccessLogging\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn UalStart(data: *const UAL_DATA_BLOB) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UalStart(data: *const UAL_DATA_BLOB) -> ::windows::core::HRESULT;
        }
        UalStart(::core::mem::transmute(data)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_UserAccessLogging\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn UalStop(data: *const UAL_DATA_BLOB) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UalStop(data: *const UAL_DATA_BLOB) -> ::windows::core::HRESULT;
        }
        UalStop(::core::mem::transmute(data)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
