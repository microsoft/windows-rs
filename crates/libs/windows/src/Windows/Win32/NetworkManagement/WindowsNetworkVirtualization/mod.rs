#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
pub const WNV_API_MAJOR_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
pub const WNV_API_MINOR_VERSION_0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WNV_CA_NOTIFICATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
pub const WnvCustomerAddressAdded: WNV_CA_NOTIFICATION_TYPE = WNV_CA_NOTIFICATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
pub const WnvCustomerAddressDeleted: WNV_CA_NOTIFICATION_TYPE = WNV_CA_NOTIFICATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
pub const WnvCustomerAddressMoved: WNV_CA_NOTIFICATION_TYPE = WNV_CA_NOTIFICATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
pub const WnvCustomerAddressMax: WNV_CA_NOTIFICATION_TYPE = WNV_CA_NOTIFICATION_TYPE(3i32);
impl ::core::marker::Copy for WNV_CA_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for WNV_CA_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WNV_CA_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WNV_CA_NOTIFICATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WNV_CA_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WNV_CA_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`, `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
pub struct WNV_CUSTOMER_ADDRESS_CHANGE_PARAM {
    pub MACAddress: super::WindowsFilteringPlatform::DL_EUI48,
    pub CAFamily: u16,
    pub CA: WNV_IP_ADDRESS,
    pub VirtualSubnetId: u32,
    pub PAFamily: u16,
    pub PA: WNV_IP_ADDRESS,
    pub NotificationReason: WNV_CA_NOTIFICATION_TYPE,
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for WNV_CUSTOMER_ADDRESS_CHANGE_PARAM {}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for WNV_CUSTOMER_ADDRESS_CHANGE_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for WNV_CUSTOMER_ADDRESS_CHANGE_PARAM {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for WNV_CUSTOMER_ADDRESS_CHANGE_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNV_CUSTOMER_ADDRESS_CHANGE_PARAM>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for WNV_CUSTOMER_ADDRESS_CHANGE_PARAM {}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for WNV_CUSTOMER_ADDRESS_CHANGE_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct WNV_IP_ADDRESS {
    pub IP: WNV_IP_ADDRESS_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for WNV_IP_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for WNV_IP_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for WNV_IP_ADDRESS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for WNV_IP_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNV_IP_ADDRESS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for WNV_IP_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for WNV_IP_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union WNV_IP_ADDRESS_0 {
    pub v4: super::super::Networking::WinSock::IN_ADDR,
    pub v6: super::super::Networking::WinSock::IN6_ADDR,
    pub Addr: [u8; 16],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for WNV_IP_ADDRESS_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for WNV_IP_ADDRESS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for WNV_IP_ADDRESS_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for WNV_IP_ADDRESS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNV_IP_ADDRESS_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for WNV_IP_ADDRESS_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for WNV_IP_ADDRESS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
pub struct WNV_NOTIFICATION_PARAM {
    pub Header: WNV_OBJECT_HEADER,
    pub NotificationType: WNV_NOTIFICATION_TYPE,
    pub PendingNotifications: u32,
    pub Buffer: *mut u8,
}
impl ::core::marker::Copy for WNV_NOTIFICATION_PARAM {}
impl ::core::clone::Clone for WNV_NOTIFICATION_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WNV_NOTIFICATION_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNV_NOTIFICATION_PARAM").field("Header", &self.Header).field("NotificationType", &self.NotificationType).field("PendingNotifications", &self.PendingNotifications).field("Buffer", &self.Buffer).finish()
    }
}
unsafe impl ::windows::core::Abi for WNV_NOTIFICATION_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WNV_NOTIFICATION_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNV_NOTIFICATION_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for WNV_NOTIFICATION_PARAM {}
impl ::core::default::Default for WNV_NOTIFICATION_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WNV_NOTIFICATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
pub const WnvPolicyMismatchType: WNV_NOTIFICATION_TYPE = WNV_NOTIFICATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
pub const WnvRedirectType: WNV_NOTIFICATION_TYPE = WNV_NOTIFICATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
pub const WnvObjectChangeType: WNV_NOTIFICATION_TYPE = WNV_NOTIFICATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
pub const WnvNotificationTypeMax: WNV_NOTIFICATION_TYPE = WNV_NOTIFICATION_TYPE(3i32);
impl ::core::marker::Copy for WNV_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for WNV_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WNV_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WNV_NOTIFICATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WNV_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WNV_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`, `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
pub struct WNV_OBJECT_CHANGE_PARAM {
    pub ObjectType: WNV_OBJECT_TYPE,
    pub ObjectParam: WNV_OBJECT_CHANGE_PARAM_0,
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for WNV_OBJECT_CHANGE_PARAM {}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for WNV_OBJECT_CHANGE_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for WNV_OBJECT_CHANGE_PARAM {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for WNV_OBJECT_CHANGE_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNV_OBJECT_CHANGE_PARAM>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for WNV_OBJECT_CHANGE_PARAM {}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for WNV_OBJECT_CHANGE_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`, `\"Win32_NetworkManagement_WindowsFilteringPlatform\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
pub union WNV_OBJECT_CHANGE_PARAM_0 {
    pub ProviderAddressChange: WNV_PROVIDER_ADDRESS_CHANGE_PARAM,
    pub CustomerAddressChange: WNV_CUSTOMER_ADDRESS_CHANGE_PARAM,
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for WNV_OBJECT_CHANGE_PARAM_0 {}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for WNV_OBJECT_CHANGE_PARAM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for WNV_OBJECT_CHANGE_PARAM_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for WNV_OBJECT_CHANGE_PARAM_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNV_OBJECT_CHANGE_PARAM_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for WNV_OBJECT_CHANGE_PARAM_0 {}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for WNV_OBJECT_CHANGE_PARAM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
pub struct WNV_OBJECT_HEADER {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub Size: u32,
}
impl ::core::marker::Copy for WNV_OBJECT_HEADER {}
impl ::core::clone::Clone for WNV_OBJECT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WNV_OBJECT_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WNV_OBJECT_HEADER").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("Size", &self.Size).finish()
    }
}
unsafe impl ::windows::core::Abi for WNV_OBJECT_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WNV_OBJECT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNV_OBJECT_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for WNV_OBJECT_HEADER {}
impl ::core::default::Default for WNV_OBJECT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WNV_OBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
pub const WnvProviderAddressType: WNV_OBJECT_TYPE = WNV_OBJECT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
pub const WnvCustomerAddressType: WNV_OBJECT_TYPE = WNV_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`*"]
pub const WnvObjectTypeMax: WNV_OBJECT_TYPE = WNV_OBJECT_TYPE(2i32);
impl ::core::marker::Copy for WNV_OBJECT_TYPE {}
impl ::core::clone::Clone for WNV_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WNV_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WNV_OBJECT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WNV_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WNV_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct WNV_POLICY_MISMATCH_PARAM {
    pub CAFamily: u16,
    pub PAFamily: u16,
    pub VirtualSubnetId: u32,
    pub CA: WNV_IP_ADDRESS,
    pub PA: WNV_IP_ADDRESS,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for WNV_POLICY_MISMATCH_PARAM {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for WNV_POLICY_MISMATCH_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for WNV_POLICY_MISMATCH_PARAM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for WNV_POLICY_MISMATCH_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNV_POLICY_MISMATCH_PARAM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for WNV_POLICY_MISMATCH_PARAM {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for WNV_POLICY_MISMATCH_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct WNV_PROVIDER_ADDRESS_CHANGE_PARAM {
    pub PAFamily: u16,
    pub PA: WNV_IP_ADDRESS,
    pub AddressState: super::super::Networking::WinSock::NL_DAD_STATE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for WNV_PROVIDER_ADDRESS_CHANGE_PARAM {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for WNV_PROVIDER_ADDRESS_CHANGE_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for WNV_PROVIDER_ADDRESS_CHANGE_PARAM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for WNV_PROVIDER_ADDRESS_CHANGE_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNV_PROVIDER_ADDRESS_CHANGE_PARAM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for WNV_PROVIDER_ADDRESS_CHANGE_PARAM {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for WNV_PROVIDER_ADDRESS_CHANGE_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct WNV_REDIRECT_PARAM {
    pub CAFamily: u16,
    pub PAFamily: u16,
    pub NewPAFamily: u16,
    pub VirtualSubnetId: u32,
    pub CA: WNV_IP_ADDRESS,
    pub PA: WNV_IP_ADDRESS,
    pub NewPA: WNV_IP_ADDRESS,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for WNV_REDIRECT_PARAM {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for WNV_REDIRECT_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for WNV_REDIRECT_PARAM {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for WNV_REDIRECT_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WNV_REDIRECT_PARAM>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for WNV_REDIRECT_PARAM {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for WNV_REDIRECT_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WnvOpen() -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WnvOpen() -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(WnvOpen())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsNetworkVirtualization\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn WnvRequestNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(wnvhandle: Param0, notificationparam: *mut WNV_NOTIFICATION_PARAM, overlapped: *mut super::super::System::IO::OVERLAPPED, bytestransferred: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WnvRequestNotification(wnvhandle: super::super::Foundation::HANDLE, notificationparam: *mut WNV_NOTIFICATION_PARAM, overlapped: *mut super::super::System::IO::OVERLAPPED, bytestransferred: *mut u32) -> u32;
        }
        ::core::mem::transmute(WnvRequestNotification(wnvhandle.into_param().abi(), ::core::mem::transmute(notificationparam), ::core::mem::transmute(overlapped), ::core::mem::transmute(bytestransferred)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
