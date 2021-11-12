#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const WNV_API_MAJOR_VERSION_1: u32 = 1u32;
pub const WNV_API_MINOR_VERSION_0: u32 = 0u32;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WNV_CA_NOTIFICATION_TYPE(pub i32);
pub const WnvCustomerAddressAdded: WNV_CA_NOTIFICATION_TYPE = WNV_CA_NOTIFICATION_TYPE(0i32);
pub const WnvCustomerAddressDeleted: WNV_CA_NOTIFICATION_TYPE = WNV_CA_NOTIFICATION_TYPE(1i32);
pub const WnvCustomerAddressMoved: WNV_CA_NOTIFICATION_TYPE = WNV_CA_NOTIFICATION_TYPE(2i32);
pub const WnvCustomerAddressMax: WNV_CA_NOTIFICATION_TYPE = WNV_CA_NOTIFICATION_TYPE(3i32);
impl ::core::convert::From<i32> for WNV_CA_NOTIFICATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WNV_CA_NOTIFICATION_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
impl WNV_CUSTOMER_ADDRESS_CHANGE_PARAM {}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for WNV_CUSTOMER_ADDRESS_CHANGE_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for WNV_CUSTOMER_ADDRESS_CHANGE_PARAM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for WNV_CUSTOMER_ADDRESS_CHANGE_PARAM {}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for WNV_CUSTOMER_ADDRESS_CHANGE_PARAM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct WNV_IP_ADDRESS {
    pub IP: WNV_IP_ADDRESS_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl WNV_IP_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for WNV_IP_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for WNV_IP_ADDRESS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for WNV_IP_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for WNV_IP_ADDRESS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union WNV_IP_ADDRESS_0 {
    pub v4: super::super::Networking::WinSock::IN_ADDR,
    pub v6: super::super::Networking::WinSock::IN6_ADDR,
    pub Addr: [u8; 16],
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl WNV_IP_ADDRESS_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for WNV_IP_ADDRESS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for WNV_IP_ADDRESS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for WNV_IP_ADDRESS_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for WNV_IP_ADDRESS_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct WNV_NOTIFICATION_PARAM {
    pub Header: WNV_OBJECT_HEADER,
    pub NotificationType: WNV_NOTIFICATION_TYPE,
    pub PendingNotifications: u32,
    pub Buffer: *mut u8,
}
impl WNV_NOTIFICATION_PARAM {}
impl ::core::default::Default for WNV_NOTIFICATION_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WNV_NOTIFICATION_PARAM {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WNV_NOTIFICATION_PARAM").field("Header", &self.Header).field("NotificationType", &self.NotificationType).field("PendingNotifications", &self.PendingNotifications).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::cmp::PartialEq for WNV_NOTIFICATION_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.NotificationType == other.NotificationType && self.PendingNotifications == other.PendingNotifications && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for WNV_NOTIFICATION_PARAM {}
unsafe impl ::windows::core::Abi for WNV_NOTIFICATION_PARAM {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WNV_NOTIFICATION_TYPE(pub i32);
pub const WnvPolicyMismatchType: WNV_NOTIFICATION_TYPE = WNV_NOTIFICATION_TYPE(0i32);
pub const WnvRedirectType: WNV_NOTIFICATION_TYPE = WNV_NOTIFICATION_TYPE(1i32);
pub const WnvObjectChangeType: WNV_NOTIFICATION_TYPE = WNV_NOTIFICATION_TYPE(2i32);
pub const WnvNotificationTypeMax: WNV_NOTIFICATION_TYPE = WNV_NOTIFICATION_TYPE(3i32);
impl ::core::convert::From<i32> for WNV_NOTIFICATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WNV_NOTIFICATION_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
pub struct WNV_OBJECT_CHANGE_PARAM {
    pub ObjectType: WNV_OBJECT_TYPE,
    pub ObjectParam: WNV_OBJECT_CHANGE_PARAM_0,
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl WNV_OBJECT_CHANGE_PARAM {}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for WNV_OBJECT_CHANGE_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for WNV_OBJECT_CHANGE_PARAM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for WNV_OBJECT_CHANGE_PARAM {}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for WNV_OBJECT_CHANGE_PARAM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
pub union WNV_OBJECT_CHANGE_PARAM_0 {
    pub ProviderAddressChange: WNV_PROVIDER_ADDRESS_CHANGE_PARAM,
    pub CustomerAddressChange: WNV_CUSTOMER_ADDRESS_CHANGE_PARAM,
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl WNV_OBJECT_CHANGE_PARAM_0 {}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for WNV_OBJECT_CHANGE_PARAM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for WNV_OBJECT_CHANGE_PARAM_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for WNV_OBJECT_CHANGE_PARAM_0 {}
#[cfg(all(feature = "Win32_NetworkManagement_WindowsFilteringPlatform", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for WNV_OBJECT_CHANGE_PARAM_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct WNV_OBJECT_HEADER {
    pub MajorVersion: u8,
    pub MinorVersion: u8,
    pub Size: u32,
}
impl WNV_OBJECT_HEADER {}
impl ::core::default::Default for WNV_OBJECT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for WNV_OBJECT_HEADER {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("WNV_OBJECT_HEADER").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("Size", &self.Size).finish()
    }
}
impl ::core::cmp::PartialEq for WNV_OBJECT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for WNV_OBJECT_HEADER {}
unsafe impl ::windows::core::Abi for WNV_OBJECT_HEADER {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WNV_OBJECT_TYPE(pub i32);
pub const WnvProviderAddressType: WNV_OBJECT_TYPE = WNV_OBJECT_TYPE(0i32);
pub const WnvCustomerAddressType: WNV_OBJECT_TYPE = WNV_OBJECT_TYPE(1i32);
pub const WnvObjectTypeMax: WNV_OBJECT_TYPE = WNV_OBJECT_TYPE(2i32);
impl ::core::convert::From<i32> for WNV_OBJECT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WNV_OBJECT_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct WNV_POLICY_MISMATCH_PARAM {
    pub CAFamily: u16,
    pub PAFamily: u16,
    pub VirtualSubnetId: u32,
    pub CA: WNV_IP_ADDRESS,
    pub PA: WNV_IP_ADDRESS,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl WNV_POLICY_MISMATCH_PARAM {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for WNV_POLICY_MISMATCH_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for WNV_POLICY_MISMATCH_PARAM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for WNV_POLICY_MISMATCH_PARAM {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for WNV_POLICY_MISMATCH_PARAM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct WNV_PROVIDER_ADDRESS_CHANGE_PARAM {
    pub PAFamily: u16,
    pub PA: WNV_IP_ADDRESS,
    pub AddressState: super::super::Networking::WinSock::NL_DAD_STATE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl WNV_PROVIDER_ADDRESS_CHANGE_PARAM {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for WNV_PROVIDER_ADDRESS_CHANGE_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for WNV_PROVIDER_ADDRESS_CHANGE_PARAM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for WNV_PROVIDER_ADDRESS_CHANGE_PARAM {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for WNV_PROVIDER_ADDRESS_CHANGE_PARAM {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
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
impl WNV_REDIRECT_PARAM {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for WNV_REDIRECT_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for WNV_REDIRECT_PARAM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for WNV_REDIRECT_PARAM {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for WNV_REDIRECT_PARAM {
    type Abi = Self;
}
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
