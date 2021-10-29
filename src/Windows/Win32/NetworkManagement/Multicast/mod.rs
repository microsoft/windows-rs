#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union IPNG_ADDRESS {
    pub IpAddrV4: u32,
    pub IpAddrV6: [u8; 16],
}
impl IPNG_ADDRESS {}
impl ::std::default::Default for IPNG_ADDRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for IPNG_ADDRESS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for IPNG_ADDRESS {}
unsafe impl ::windows::runtime::Abi for IPNG_ADDRESS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MCAST_API_CURRENT_VERSION: i32 = 1i32;
pub const MCAST_API_VERSION_0: i32 = 0i32;
pub const MCAST_API_VERSION_1: i32 = 1i32;
pub const MCAST_CLIENT_ID_LEN: u32 = 17u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MCAST_CLIENT_UID {
    pub ClientUID: *mut u8,
    pub ClientUIDLength: u32,
}
impl MCAST_CLIENT_UID {}
impl ::std::default::Default for MCAST_CLIENT_UID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MCAST_CLIENT_UID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MCAST_CLIENT_UID").field("ClientUID", &self.ClientUID).field("ClientUIDLength", &self.ClientUIDLength).finish()
    }
}
impl ::std::cmp::PartialEq for MCAST_CLIENT_UID {
    fn eq(&self, other: &Self) -> bool {
        self.ClientUID == other.ClientUID && self.ClientUIDLength == other.ClientUIDLength
    }
}
impl ::std::cmp::Eq for MCAST_CLIENT_UID {}
unsafe impl ::windows::runtime::Abi for MCAST_CLIENT_UID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MCAST_LEASE_REQUEST {
    pub LeaseStartTime: i32,
    pub MaxLeaseStartTime: i32,
    pub LeaseDuration: u32,
    pub MinLeaseDuration: u32,
    pub ServerAddress: IPNG_ADDRESS,
    pub MinAddrCount: u16,
    pub AddrCount: u16,
    pub pAddrBuf: *mut u8,
}
impl MCAST_LEASE_REQUEST {}
impl ::std::default::Default for MCAST_LEASE_REQUEST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MCAST_LEASE_REQUEST {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MCAST_LEASE_REQUEST {}
unsafe impl ::windows::runtime::Abi for MCAST_LEASE_REQUEST {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MCAST_LEASE_RESPONSE {
    pub LeaseStartTime: i32,
    pub LeaseEndTime: i32,
    pub ServerAddress: IPNG_ADDRESS,
    pub AddrCount: u16,
    pub pAddrBuf: *mut u8,
}
impl MCAST_LEASE_RESPONSE {}
impl ::std::default::Default for MCAST_LEASE_RESPONSE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MCAST_LEASE_RESPONSE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MCAST_LEASE_RESPONSE {}
unsafe impl ::windows::runtime::Abi for MCAST_LEASE_RESPONSE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MCAST_SCOPE_CTX {
    pub ScopeID: IPNG_ADDRESS,
    pub Interface: IPNG_ADDRESS,
    pub ServerID: IPNG_ADDRESS,
}
impl MCAST_SCOPE_CTX {}
impl ::std::default::Default for MCAST_SCOPE_CTX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for MCAST_SCOPE_CTX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for MCAST_SCOPE_CTX {}
unsafe impl ::windows::runtime::Abi for MCAST_SCOPE_CTX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MCAST_SCOPE_ENTRY {
    pub ScopeCtx: MCAST_SCOPE_CTX,
    pub LastAddr: IPNG_ADDRESS,
    pub TTL: u32,
    pub ScopeDesc: super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl MCAST_SCOPE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MCAST_SCOPE_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MCAST_SCOPE_ENTRY {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MCAST_SCOPE_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MCAST_SCOPE_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
#[inline]
pub unsafe fn McastApiCleanup() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn McastApiCleanup();
        }
        ::std::mem::transmute(McastApiCleanup())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn McastApiStartup(version: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn McastApiStartup(version: *mut u32) -> u32;
        }
        ::std::mem::transmute(McastApiStartup(::std::mem::transmute(version)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn McastEnumerateScopes<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(addrfamily: u16, requery: Param1, pscopelist: *mut MCAST_SCOPE_ENTRY, pscopelen: *mut u32, pscopecount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn McastEnumerateScopes(addrfamily: u16, requery: super::super::Foundation::BOOL, pscopelist: *mut MCAST_SCOPE_ENTRY, pscopelen: *mut u32, pscopecount: *mut u32) -> u32;
        }
        ::std::mem::transmute(McastEnumerateScopes(::std::mem::transmute(addrfamily), requery.into_param().abi(), ::std::mem::transmute(pscopelist), ::std::mem::transmute(pscopelen), ::std::mem::transmute(pscopecount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn McastGenUID(prequestid: *mut MCAST_CLIENT_UID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn McastGenUID(prequestid: *mut MCAST_CLIENT_UID) -> u32;
        }
        ::std::mem::transmute(McastGenUID(::std::mem::transmute(prequestid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn McastReleaseAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, preleaserequest: *mut MCAST_LEASE_REQUEST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn McastReleaseAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, preleaserequest: *mut MCAST_LEASE_REQUEST) -> u32;
        }
        ::std::mem::transmute(McastReleaseAddress(::std::mem::transmute(addrfamily), ::std::mem::transmute(prequestid), ::std::mem::transmute(preleaserequest)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn McastRenewAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, prenewrequest: *mut MCAST_LEASE_REQUEST, prenewresponse: *mut MCAST_LEASE_RESPONSE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn McastRenewAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, prenewrequest: *mut MCAST_LEASE_REQUEST, prenewresponse: *mut MCAST_LEASE_RESPONSE) -> u32;
        }
        ::std::mem::transmute(McastRenewAddress(::std::mem::transmute(addrfamily), ::std::mem::transmute(prequestid), ::std::mem::transmute(prenewrequest), ::std::mem::transmute(prenewresponse)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn McastRequestAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, pscopectx: *mut MCAST_SCOPE_CTX, paddrrequest: *mut MCAST_LEASE_REQUEST, paddrresponse: *mut MCAST_LEASE_RESPONSE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn McastRequestAddress(addrfamily: u16, prequestid: *mut MCAST_CLIENT_UID, pscopectx: *mut MCAST_SCOPE_CTX, paddrrequest: *mut MCAST_LEASE_REQUEST, paddrresponse: *mut MCAST_LEASE_RESPONSE) -> u32;
        }
        ::std::mem::transmute(McastRequestAddress(::std::mem::transmute(addrfamily), ::std::mem::transmute(prequestid), ::std::mem::transmute(pscopectx), ::std::mem::transmute(paddrrequest), ::std::mem::transmute(paddrresponse)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
