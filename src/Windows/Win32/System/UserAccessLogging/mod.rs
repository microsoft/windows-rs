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
#[cfg(all(
    feature = "Win32_Networking_WinSock",
    feature = "Win32_System_SystemServices"
))]
pub struct UAL_DATA_BLOB {
    pub Size: u32,
    pub RoleGuid: ::windows::runtime::GUID,
    pub TenantId: ::windows::runtime::GUID,
    pub Address: super::super::Networking::WinSock::SOCKADDR_STORAGE,
    pub UserName: [u16; 260],
}
#[cfg(all(
    feature = "Win32_Networking_WinSock",
    feature = "Win32_System_SystemServices"
))]
impl UAL_DATA_BLOB {}
#[cfg(all(
    feature = "Win32_Networking_WinSock",
    feature = "Win32_System_SystemServices"
))]
impl ::std::default::Default for UAL_DATA_BLOB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(
    feature = "Win32_Networking_WinSock",
    feature = "Win32_System_SystemServices"
))]
impl ::std::fmt::Debug for UAL_DATA_BLOB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("UAL_DATA_BLOB")
            .field("Size", &self.Size)
            .field("RoleGuid", &self.RoleGuid)
            .field("TenantId", &self.TenantId)
            .field("Address", &self.Address)
            .field("UserName", &self.UserName)
            .finish()
    }
}
#[cfg(all(
    feature = "Win32_Networking_WinSock",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::PartialEq for UAL_DATA_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.RoleGuid == other.RoleGuid
            && self.TenantId == other.TenantId
            && self.Address == other.Address
            && self.UserName == other.UserName
    }
}
#[cfg(all(
    feature = "Win32_Networking_WinSock",
    feature = "Win32_System_SystemServices"
))]
impl ::std::cmp::Eq for UAL_DATA_BLOB {}
#[cfg(all(
    feature = "Win32_Networking_WinSock",
    feature = "Win32_System_SystemServices"
))]
unsafe impl ::windows::runtime::Abi for UAL_DATA_BLOB {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(
    feature = "Win32_Networking_WinSock",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn UalInstrument(data: *const UAL_DATA_BLOB) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ualapi")]
        extern "system" {
            fn UalInstrument(data: *const UAL_DATA_BLOB) -> ::windows::runtime::HRESULT;
        }
        UalInstrument(::std::mem::transmute(data)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn UalRegisterProduct<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    wszproductname: Param0,
    wszrolename: Param1,
    wszguid: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ualapi")]
        extern "system" {
            fn UalRegisterProduct(
                wszproductname: super::super::Foundation::PWSTR,
                wszrolename: super::super::Foundation::PWSTR,
                wszguid: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        UalRegisterProduct(
            wszproductname.into_param().abi(),
            wszrolename.into_param().abi(),
            wszguid.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Networking_WinSock",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn UalStart(data: *const UAL_DATA_BLOB) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ualapi")]
        extern "system" {
            fn UalStart(data: *const UAL_DATA_BLOB) -> ::windows::runtime::HRESULT;
        }
        UalStart(::std::mem::transmute(data)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_Networking_WinSock",
    feature = "Win32_System_SystemServices"
))]
pub unsafe fn UalStop(data: *const UAL_DATA_BLOB) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "ualapi")]
        extern "system" {
            fn UalStop(data: *const UAL_DATA_BLOB) -> ::windows::runtime::HRESULT;
        }
        UalStop(::std::mem::transmute(data)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
