#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Networking_BackgroundTransfer")]
pub mod BackgroundTransfer;
#[cfg(feature = "Networking_Connectivity")]
pub mod Connectivity;
#[cfg(feature = "Networking_NetworkOperators")]
pub mod NetworkOperators;
#[cfg(feature = "Networking_Proximity")]
pub mod Proximity;
#[cfg(feature = "Networking_PushNotifications")]
pub mod PushNotifications;
#[cfg(feature = "Networking_ServiceDiscovery")]
pub mod ServiceDiscovery;
#[cfg(feature = "Networking_Sockets")]
pub mod Sockets;
#[cfg(feature = "Networking_Vpn")]
pub mod Vpn;
#[cfg(feature = "Networking_XboxLive")]
pub mod XboxLive;
#[doc = "*Required features: `Networking`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DomainNameType(pub i32);
impl DomainNameType {
    pub const Suffix: DomainNameType = DomainNameType(0i32);
    pub const FullyQualified: DomainNameType = DomainNameType(1i32);
}
impl ::core::convert::From<i32> for DomainNameType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DomainNameType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DomainNameType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.DomainNameType;i4)");
}
impl ::windows::core::DefaultType for DomainNameType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Networking`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EndpointPair(pub ::windows::core::IInspectable);
impl EndpointPair {
    #[doc = "*Required features: `Networking`*"]
    pub fn LocalHostName(&self) -> ::windows::core::Result<HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HostName>(result__)
        }
    }
    #[doc = "*Required features: `Networking`*"]
    pub fn SetLocalHostName<'a, Param0: ::windows::core::IntoParam<'a, HostName>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking`*"]
    pub fn LocalServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking`*"]
    pub fn SetLocalServiceName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking`*"]
    pub fn RemoteHostName(&self) -> ::windows::core::Result<HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HostName>(result__)
        }
    }
    #[doc = "*Required features: `Networking`*"]
    pub fn SetRemoteHostName<'a, Param0: ::windows::core::IntoParam<'a, HostName>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking`*"]
    pub fn RemoteServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking`*"]
    pub fn SetRemoteServiceName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking`*"]
    pub fn CreateEndpointPair<'a, Param0: ::windows::core::IntoParam<'a, HostName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, HostName>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(localhostname: Param0, localservicename: Param1, remotehostname: Param2, remoteservicename: Param3) -> ::windows::core::Result<EndpointPair> {
        Self::IEndpointPairFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), localhostname.into_param().abi(), localservicename.into_param().abi(), remotehostname.into_param().abi(), remoteservicename.into_param().abi(), &mut result__).from_abi::<EndpointPair>(result__)
        })
    }
    pub fn IEndpointPairFactory<R, F: FnOnce(&IEndpointPairFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EndpointPair, IEndpointPairFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for EndpointPair {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.EndpointPair;{33a0aa36-f8fa-4b30-b856-76517c3bd06d})");
}
unsafe impl ::windows::core::Interface for EndpointPair {
    type Vtable = IEndpointPair_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33a0aa36_f8fa_4b30_b856_76517c3bd06d);
}
impl ::windows::core::RuntimeName for EndpointPair {
    const NAME: &'static str = "Windows.Networking.EndpointPair";
}
impl ::core::convert::From<EndpointPair> for ::windows::core::IUnknown {
    fn from(value: EndpointPair) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&EndpointPair> for ::windows::core::IUnknown {
    fn from(value: &EndpointPair) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EndpointPair {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EndpointPair {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<EndpointPair> for ::windows::core::IInspectable {
    fn from(value: EndpointPair) -> Self {
        value.0
    }
}
impl ::core::convert::From<&EndpointPair> for ::windows::core::IInspectable {
    fn from(value: &EndpointPair) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EndpointPair {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EndpointPair {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for EndpointPair {}
unsafe impl ::core::marker::Sync for EndpointPair {}
#[doc = "*Required features: `Networking`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct HostName(pub ::windows::core::IInspectable);
impl HostName {
    #[cfg(feature = "Networking_Connectivity")]
    #[doc = "*Required features: `Networking`, `Networking_Connectivity`*"]
    pub fn IPInformation(&self) -> ::windows::core::Result<Connectivity::IPInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Connectivity::IPInformation>(result__)
        }
    }
    #[doc = "*Required features: `Networking`*"]
    pub fn RawName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking`*"]
    pub fn CanonicalName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking`*"]
    pub fn Type(&self) -> ::windows::core::Result<HostNameType> {
        let this = self;
        unsafe {
            let mut result__: HostNameType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HostNameType>(result__)
        }
    }
    #[doc = "*Required features: `Networking`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, HostName>>(&self, hostname: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), hostname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking`*"]
    pub fn CreateHostName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(hostname: Param0) -> ::windows::core::Result<HostName> {
        Self::IHostNameFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), hostname.into_param().abi(), &mut result__).from_abi::<HostName>(result__)
        })
    }
    #[doc = "*Required features: `Networking`*"]
    pub fn Compare<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value1: Param0, value2: Param1) -> ::windows::core::Result<i32> {
        Self::IHostNameStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value1.into_param().abi(), value2.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    pub fn IHostNameFactory<R, F: FnOnce(&IHostNameFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HostName, IHostNameFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHostNameStatics<R, F: FnOnce(&IHostNameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HostName, IHostNameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for HostName {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.HostName;{bf8ecaad-ed96-49a7-9084-d416cae88dcb})");
}
unsafe impl ::windows::core::Interface for HostName {
    type Vtable = IHostName_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf8ecaad_ed96_49a7_9084_d416cae88dcb);
}
impl ::windows::core::RuntimeName for HostName {
    const NAME: &'static str = "Windows.Networking.HostName";
}
impl ::core::convert::From<HostName> for ::windows::core::IUnknown {
    fn from(value: HostName) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&HostName> for ::windows::core::IUnknown {
    fn from(value: &HostName) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HostName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HostName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<HostName> for ::windows::core::IInspectable {
    fn from(value: HostName) -> Self {
        value.0
    }
}
impl ::core::convert::From<&HostName> for ::windows::core::IInspectable {
    fn from(value: &HostName) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HostName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HostName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HostName> for super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: HostName) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HostName> for super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HostName) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IStringable> for HostName {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::Foundation::IStringable> for &HostName {
    fn into_param(self) -> ::windows::core::Param<'a, super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for HostName {}
unsafe impl ::core::marker::Sync for HostName {}
#[doc = "*Required features: `Networking`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HostNameSortOptions(pub u32);
impl HostNameSortOptions {
    pub const None: HostNameSortOptions = HostNameSortOptions(0u32);
    pub const OptimizeForLongConnections: HostNameSortOptions = HostNameSortOptions(2u32);
}
impl ::core::convert::From<u32> for HostNameSortOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HostNameSortOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for HostNameSortOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.HostNameSortOptions;u4)");
}
impl ::windows::core::DefaultType for HostNameSortOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for HostNameSortOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for HostNameSortOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for HostNameSortOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for HostNameSortOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for HostNameSortOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Networking`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HostNameType(pub i32);
impl HostNameType {
    pub const DomainName: HostNameType = HostNameType(0i32);
    pub const Ipv4: HostNameType = HostNameType(1i32);
    pub const Ipv6: HostNameType = HostNameType(2i32);
    pub const Bluetooth: HostNameType = HostNameType(3i32);
}
impl ::core::convert::From<i32> for HostNameType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HostNameType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for HostNameType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.HostNameType;i4)");
}
impl ::windows::core::DefaultType for HostNameType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IEndpointPair(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEndpointPair {
    type Vtable = IEndpointPair_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33a0aa36_f8fa_4b30_b856_76517c3bd06d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEndpointPair_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IEndpointPairFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IEndpointPairFactory {
    type Vtable = IEndpointPairFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb609d971_64e0_442b_aa6f_cc8c8f181f78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEndpointPairFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, localhostname: ::windows::core::RawPtr, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHostName(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHostName {
    type Vtable = IHostName_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf8ecaad_ed96_49a7_9084_d416cae88dcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostName_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Networking_Connectivity")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut HostNameType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hostname: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHostNameFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHostNameFactory {
    type Vtable = IHostNameFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x458c23ed_712f_4576_adf1_c20b2c643558);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostNameFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hostname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IHostNameStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IHostNameStatics {
    type Vtable = IHostNameStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf68cd4bf_a388_4e8b_91ea_54dd6dd901c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostNameStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut i32) -> ::windows::core::HRESULT,
);
