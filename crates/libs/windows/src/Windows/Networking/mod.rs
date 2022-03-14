#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
#[doc = "*Required features: `\"Networking\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DomainNameType(pub i32);
impl DomainNameType {
    pub const Suffix: Self = Self(0i32);
    pub const FullyQualified: Self = Self(1i32);
}
impl ::core::marker::Copy for DomainNameType {}
impl ::core::clone::Clone for DomainNameType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DomainNameType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DomainNameType {
    type Abi = Self;
}
impl ::core::fmt::Debug for DomainNameType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DomainNameType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DomainNameType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.DomainNameType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking\"`*"]
#[repr(transparent)]
pub struct EndpointPair(::windows::core::IUnknown);
impl EndpointPair {
    #[doc = "*Required features: `\"Networking\"`*"]
    pub fn LocalHostName(&self) -> ::windows::core::Result<HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalHostName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HostName>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    pub fn SetLocalHostName<'a, Param0: ::windows::core::IntoParam<'a, HostName>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLocalHostName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    pub fn LocalServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LocalServiceName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    pub fn SetLocalServiceName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLocalServiceName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    pub fn RemoteHostName(&self) -> ::windows::core::Result<HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoteHostName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HostName>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    pub fn SetRemoteHostName<'a, Param0: ::windows::core::IntoParam<'a, HostName>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRemoteHostName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    pub fn RemoteServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoteServiceName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    pub fn SetRemoteServiceName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRemoteServiceName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    pub fn CreateEndpointPair<'a, Param0: ::windows::core::IntoParam<'a, HostName>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, HostName>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(localhostname: Param0, localservicename: Param1, remotehostname: Param2, remoteservicename: Param3) -> ::windows::core::Result<EndpointPair> {
        Self::IEndpointPairFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateEndpointPair)(::core::mem::transmute_copy(this), localhostname.into_param().abi(), localservicename.into_param().abi(), remotehostname.into_param().abi(), remoteservicename.into_param().abi(), &mut result__).from_abi::<EndpointPair>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IEndpointPairFactory<R, F: FnOnce(&IEndpointPairFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<EndpointPair, IEndpointPairFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for EndpointPair {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for EndpointPair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EndpointPair {}
impl ::core::fmt::Debug for EndpointPair {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EndpointPair").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for EndpointPair {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.EndpointPair;{33a0aa36-f8fa-4b30-b856-76517c3bd06d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for EndpointPair {
    type Vtable = IEndpointPair_Vtbl;
    const IID: ::windows::core::GUID = <IEndpointPair as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for EndpointPair {
    const NAME: &'static str = "Windows.Networking.EndpointPair";
}
impl ::core::convert::From<EndpointPair> for ::windows::core::IUnknown {
    fn from(value: EndpointPair) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EndpointPair> for ::windows::core::IUnknown {
    fn from(value: &EndpointPair) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for EndpointPair {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a EndpointPair {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<EndpointPair> for ::windows::core::IInspectable {
    fn from(value: EndpointPair) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&EndpointPair> for ::windows::core::IInspectable {
    fn from(value: &EndpointPair) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for EndpointPair {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a EndpointPair {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for EndpointPair {}
unsafe impl ::core::marker::Sync for EndpointPair {}
#[doc = "*Required features: `\"Networking\"`*"]
#[repr(transparent)]
pub struct HostName(::windows::core::IUnknown);
impl HostName {
    #[doc = "*Required features: `\"Networking\"`, `\"Networking_Connectivity\"`*"]
    #[cfg(feature = "Networking_Connectivity")]
    pub fn IPInformation(&self) -> ::windows::core::Result<Connectivity::IPInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IPInformation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Connectivity::IPInformation>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    pub fn RawName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RawName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    pub fn CanonicalName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanonicalName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    pub fn Type(&self) -> ::windows::core::Result<HostNameType> {
        let this = self;
        unsafe {
            let mut result__: HostNameType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HostNameType>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    pub fn IsEqual<'a, Param0: ::windows::core::IntoParam<'a, HostName>>(&self, hostname: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEqual)(::core::mem::transmute_copy(this), hostname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    pub fn CreateHostName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(hostname: Param0) -> ::windows::core::Result<HostName> {
        Self::IHostNameFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateHostName)(::core::mem::transmute_copy(this), hostname.into_param().abi(), &mut result__).from_abi::<HostName>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking\"`*"]
    pub fn Compare<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value1: Param0, value2: Param1) -> ::windows::core::Result<i32> {
        Self::IHostNameStatics(|this| unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Compare)(::core::mem::transmute_copy(this), value1.into_param().abi(), value2.into_param().abi(), &mut result__).from_abi::<i32>(result__)
        })
    }
    #[doc = "*Required features: `\"Networking\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ToString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IHostNameFactory<R, F: FnOnce(&IHostNameFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HostName, IHostNameFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IHostNameStatics<R, F: FnOnce(&IHostNameStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<HostName, IHostNameStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for HostName {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HostName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HostName {}
impl ::core::fmt::Debug for HostName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostName").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HostName {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.HostName;{bf8ecaad-ed96-49a7-9084-d416cae88dcb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HostName {
    type Vtable = IHostName_Vtbl;
    const IID: ::windows::core::GUID = <IHostName as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HostName {
    const NAME: &'static str = "Windows.Networking.HostName";
}
impl ::core::convert::From<HostName> for ::windows::core::IUnknown {
    fn from(value: HostName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HostName> for ::windows::core::IUnknown {
    fn from(value: &HostName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for HostName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a HostName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<HostName> for ::windows::core::IInspectable {
    fn from(value: HostName) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HostName> for ::windows::core::IInspectable {
    fn from(value: &HostName) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for HostName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a HostName {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Networking\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HostNameSortOptions(pub u32);
impl HostNameSortOptions {
    pub const None: Self = Self(0u32);
    pub const OptimizeForLongConnections: Self = Self(2u32);
}
impl ::core::marker::Copy for HostNameSortOptions {}
impl ::core::clone::Clone for HostNameSortOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HostNameSortOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HostNameSortOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for HostNameSortOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostNameSortOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HostNameSortOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HostNameSortOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HostNameSortOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HostNameSortOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HostNameSortOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for HostNameSortOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.HostNameSortOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HostNameType(pub i32);
impl HostNameType {
    pub const DomainName: Self = Self(0i32);
    pub const Ipv4: Self = Self(1i32);
    pub const Ipv6: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
}
impl ::core::marker::Copy for HostNameType {}
impl ::core::clone::Clone for HostNameType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HostNameType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HostNameType {
    type Abi = Self;
}
impl ::core::fmt::Debug for HostNameType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HostNameType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HostNameType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.HostNameType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEndpointPair(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEndpointPair {
    type Vtable = IEndpointPair_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33a0aa36_f8fa_4b30_b856_76517c3bd06d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEndpointPair_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub LocalHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetLocalHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LocalServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLocalServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub RemoteHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetRemoteHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoteServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetRemoteServiceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IEndpointPairFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IEndpointPairFactory {
    type Vtable = IEndpointPairFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb609d971_64e0_442b_aa6f_cc8c8f181f78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEndpointPairFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateEndpointPair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, localhostname: ::windows::core::RawPtr, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHostName(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHostName {
    type Vtable = IHostName_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf8ecaad_ed96_49a7_9084_d416cae88dcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostName_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Networking_Connectivity")]
    pub IPInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))]
    IPInformation: usize,
    pub RawName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CanonicalName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HostNameType) -> ::windows::core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostname: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHostNameFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHostNameFactory {
    type Vtable = IHostNameFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x458c23ed_712f_4576_adf1_c20b2c643558);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostNameFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHostNameStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHostNameStatics {
    type Vtable = IHostNameStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf68cd4bf_a388_4e8b_91ea_54dd6dd901c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostNameStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
