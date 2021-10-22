#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
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
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DomainNameType(pub i32);
impl DomainNameType {
    pub const Suffix: DomainNameType = DomainNameType(0i32);
    pub const FullyQualified: DomainNameType = DomainNameType(1i32);
}
impl ::std::convert::From<i32> for DomainNameType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DomainNameType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DomainNameType {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.DomainNameType;i4)");
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct EndpointPair(::windows::runtime::IInspectable);
impl EndpointPair {
    pub fn LocalHostName(&self) -> ::windows::runtime::Result<HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<HostName>(result__)
        }
    }
    pub fn SetLocalHostName<'a, Param0: ::windows::runtime::IntoParam<'a, HostName>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn LocalServiceName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetLocalServiceName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoteHostName(&self) -> ::windows::runtime::Result<HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<HostName>(result__)
        }
    }
    pub fn SetRemoteHostName<'a, Param0: ::windows::runtime::IntoParam<'a, HostName>>(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn RemoteServiceName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetRemoteServiceName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn CreateEndpointPair<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, HostName>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, HostName>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        localhostname: Param0,
        localservicename: Param1,
        remotehostname: Param2,
        remoteservicename: Param3,
    ) -> ::windows::runtime::Result<EndpointPair> {
        Self::IEndpointPairFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                localhostname.into_param().abi(),
                localservicename.into_param().abi(),
                remotehostname.into_param().abi(),
                remoteservicename.into_param().abi(),
                &mut result__,
            )
            .from_abi::<EndpointPair>(result__)
        })
    }
    pub fn IEndpointPairFactory<
        R,
        F: FnOnce(&IEndpointPairFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<EndpointPair, IEndpointPairFactory> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for EndpointPair {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Networking.EndpointPair;{33a0aa36-f8fa-4b30-b856-76517c3bd06d})",
    );
}
unsafe impl ::windows::runtime::Interface for EndpointPair {
    type Vtable = IEndpointPair_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        866167350,
        63738,
        19248,
        [184, 86, 118, 81, 124, 59, 208, 109],
    );
}
impl ::windows::runtime::RuntimeName for EndpointPair {
    const NAME: &'static str = "Windows.Networking.EndpointPair";
}
impl ::std::convert::From<EndpointPair> for ::windows::runtime::IUnknown {
    fn from(value: EndpointPair) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&EndpointPair> for ::windows::runtime::IUnknown {
    fn from(value: &EndpointPair) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for EndpointPair {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &EndpointPair {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<EndpointPair> for ::windows::runtime::IInspectable {
    fn from(value: EndpointPair) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EndpointPair> for ::windows::runtime::IInspectable {
    fn from(value: &EndpointPair) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for EndpointPair {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a EndpointPair {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for EndpointPair {}
unsafe impl ::std::marker::Sync for EndpointPair {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct HostName(::windows::runtime::IInspectable);
impl HostName {
    #[cfg(feature = "Networking_Connectivity")]
    pub fn IPInformation(&self) -> ::windows::runtime::Result<Connectivity::IPInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<Connectivity::IPInformation>(result__)
        }
    }
    pub fn RawName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn CanonicalName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::runtime::Result<HostNameType> {
        let this = self;
        unsafe {
            let mut result__: HostNameType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<HostNameType>(result__)
        }
    }
    pub fn IsEqual<'a, Param0: ::windows::runtime::IntoParam<'a, HostName>>(
        &self,
        hostname: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                hostname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn CreateHostName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        hostname: Param0,
    ) -> ::windows::runtime::Result<HostName> {
        Self::IHostNameFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                hostname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<HostName>(result__)
        })
    }
    pub fn Compare<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        value1: Param0,
        value2: Param1,
    ) -> ::windows::runtime::Result<i32> {
        Self::IHostNameStatics(|this| unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                value1.into_param().abi(),
                value2.into_param().abi(),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        })
    }
    pub fn IHostNameFactory<R, F: FnOnce(&IHostNameFactory) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HostName, IHostNameFactory> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IHostNameStatics<R, F: FnOnce(&IHostNameStatics) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<HostName, IHostNameStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HostName {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.Networking.HostName;{bf8ecaad-ed96-49a7-9084-d416cae88dcb})",
    );
}
unsafe impl ::windows::runtime::Interface for HostName {
    type Vtable = IHostName_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3213806253,
        60822,
        18855,
        [144, 132, 212, 22, 202, 232, 141, 203],
    );
}
impl ::windows::runtime::RuntimeName for HostName {
    const NAME: &'static str = "Windows.Networking.HostName";
}
impl ::std::convert::From<HostName> for ::windows::runtime::IUnknown {
    fn from(value: HostName) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HostName> for ::windows::runtime::IUnknown {
    fn from(value: &HostName) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for HostName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &HostName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<HostName> for ::windows::runtime::IInspectable {
    fn from(value: HostName) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HostName> for ::windows::runtime::IInspectable {
    fn from(value: &HostName) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for HostName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a HostName {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HostName> for super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HostName) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HostName> for super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HostName) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Foundation::IStringable> for HostName {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::Foundation::IStringable> for &HostName {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::Foundation::IStringable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HostName {}
unsafe impl ::std::marker::Sync for HostName {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct HostNameSortOptions(pub u32);
impl HostNameSortOptions {
    pub const None: HostNameSortOptions = HostNameSortOptions(0u32);
    pub const OptimizeForLongConnections: HostNameSortOptions = HostNameSortOptions(2u32);
}
impl ::std::convert::From<u32> for HostNameSortOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HostNameSortOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HostNameSortOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.Networking.HostNameSortOptions;u4)",
    );
}
impl ::std::ops::BitOr for HostNameSortOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for HostNameSortOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for HostNameSortOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for HostNameSortOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for HostNameSortOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct HostNameType(pub i32);
impl HostNameType {
    pub const DomainName: HostNameType = HostNameType(0i32);
    pub const Ipv4: HostNameType = HostNameType(1i32);
    pub const Ipv6: HostNameType = HostNameType(2i32);
    pub const Bluetooth: HostNameType = HostNameType(3i32);
}
impl ::std::convert::From<i32> for HostNameType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HostNameType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HostNameType {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.HostNameType;i4)");
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IEndpointPair(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEndpointPair {
    type Vtable = IEndpointPair_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        866167350,
        63738,
        19248,
        [184, 86, 118, 81, 124, 59, 208, 109],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IEndpointPair_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IEndpointPairFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IEndpointPairFactory {
    type Vtable = IEndpointPairFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3054098801,
        25824,
        17451,
        [170, 111, 204, 140, 143, 24, 31, 120],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IEndpointPairFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        localhostname: ::windows::runtime::RawPtr,
        localservicename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        remotehostname: ::windows::runtime::RawPtr,
        remoteservicename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IHostName(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHostName {
    type Vtable = IHostName_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3213806253,
        60822,
        18855,
        [144, 132, 212, 22, 202, 232, 141, 203],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostName_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Networking_Connectivity")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Networking_Connectivity"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut HostNameType,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hostname: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IHostNameFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHostNameFactory {
    type Vtable = IHostNameFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1166812141,
        28975,
        17782,
        [173, 241, 194, 11, 44, 100, 53, 88],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostNameFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hostname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IHostNameStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHostNameStatics {
    type Vtable = IHostNameStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4136424639,
        41864,
        20107,
        [145, 234, 84, 221, 109, 217, 1, 192],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHostNameStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value1: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        value2: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
);
