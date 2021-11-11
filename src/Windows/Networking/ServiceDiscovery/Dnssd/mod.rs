#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DnssdRegistrationResult(pub ::windows::core::IInspectable);
impl DnssdRegistrationResult {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DnssdRegistrationResult, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn Status(&self) -> ::windows::core::Result<DnssdRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__: DnssdRegistrationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DnssdRegistrationStatus>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn IPAddress(&self) -> ::windows::core::Result<super::super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::HostName>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn HasInstanceNameChanged(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for DnssdRegistrationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationResult;{3d786ad2-e606-5350-73ea-7e97f066162f})");
}
unsafe impl ::windows::core::Interface for DnssdRegistrationResult {
    type Vtable = IDnssdRegistrationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d786ad2_e606_5350_73ea_7e97f066162f);
}
impl ::windows::core::RuntimeName for DnssdRegistrationResult {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationResult";
}
impl ::core::convert::From<DnssdRegistrationResult> for ::windows::core::IUnknown {
    fn from(value: DnssdRegistrationResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DnssdRegistrationResult> for ::windows::core::IUnknown {
    fn from(value: &DnssdRegistrationResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DnssdRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DnssdRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DnssdRegistrationResult> for ::windows::core::IInspectable {
    fn from(value: DnssdRegistrationResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DnssdRegistrationResult> for ::windows::core::IInspectable {
    fn from(value: &DnssdRegistrationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DnssdRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DnssdRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DnssdRegistrationResult> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: DnssdRegistrationResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DnssdRegistrationResult> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DnssdRegistrationResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for DnssdRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &DnssdRegistrationResult {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DnssdRegistrationResult {}
unsafe impl ::core::marker::Sync for DnssdRegistrationResult {}
#[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DnssdRegistrationStatus(pub i32);
impl DnssdRegistrationStatus {
    pub const Success: DnssdRegistrationStatus = DnssdRegistrationStatus(0i32);
    pub const InvalidServiceName: DnssdRegistrationStatus = DnssdRegistrationStatus(1i32);
    pub const ServerError: DnssdRegistrationStatus = DnssdRegistrationStatus(2i32);
    pub const SecurityError: DnssdRegistrationStatus = DnssdRegistrationStatus(3i32);
}
impl ::core::convert::From<i32> for DnssdRegistrationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DnssdRegistrationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DnssdRegistrationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationStatus;i4)");
}
impl ::windows::core::DefaultType for DnssdRegistrationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DnssdServiceInstance(pub ::windows::core::IInspectable);
impl DnssdServiceInstance {
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn DnssdServiceInstanceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn SetDnssdServiceInstanceName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn HostName(&self) -> ::windows::core::Result<super::super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::HostName>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn SetHostName<'a, Param0: ::windows::core::IntoParam<'a, super::super::HostName>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn Port(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn SetPort(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn Priority(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn SetPriority(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn Weight(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn SetWeight(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation_Collections`*"]
    pub fn TextAttributes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`, `Networking_Sockets`*"]
    pub fn RegisterStreamSocketListenerAsync1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Sockets::StreamSocketListener>>(&self, socket: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), socket.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`, `Networking_Connectivity`, `Networking_Sockets`*"]
    pub fn RegisterStreamSocketListenerAsync2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Sockets::StreamSocketListener>, Param1: ::windows::core::IntoParam<'a, super::super::Connectivity::NetworkAdapter>>(&self, socket: Param0, adapter: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), socket.into_param().abi(), adapter.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`, `Networking_Sockets`*"]
    pub fn RegisterDatagramSocketAsync1<'a, Param0: ::windows::core::IntoParam<'a, super::super::Sockets::DatagramSocket>>(&self, socket: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), socket.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`, `Networking_Connectivity`, `Networking_Sockets`*"]
    pub fn RegisterDatagramSocketAsync2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Sockets::DatagramSocket>, Param1: ::windows::core::IntoParam<'a, super::super::Connectivity::NetworkAdapter>>(&self, socket: Param0, adapter: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), socket.into_param().abi(), adapter.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::HostName>>(dnssdserviceinstancename: Param0, hostname: Param1, port: u16) -> ::windows::core::Result<DnssdServiceInstance> {
        Self::IDnssdServiceInstanceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), dnssdserviceinstancename.into_param().abi(), hostname.into_param().abi(), port, &mut result__).from_abi::<DnssdServiceInstance>(result__)
        })
    }
    pub fn IDnssdServiceInstanceFactory<R, F: FnOnce(&IDnssdServiceInstanceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DnssdServiceInstance, IDnssdServiceInstanceFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for DnssdServiceInstance {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance;{e246db7e-98a5-4ca1-b9e4-c253d33c35ff})");
}
unsafe impl ::windows::core::Interface for DnssdServiceInstance {
    type Vtable = IDnssdServiceInstance_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe246db7e_98a5_4ca1_b9e4_c253d33c35ff);
}
impl ::windows::core::RuntimeName for DnssdServiceInstance {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance";
}
impl ::core::convert::From<DnssdServiceInstance> for ::windows::core::IUnknown {
    fn from(value: DnssdServiceInstance) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DnssdServiceInstance> for ::windows::core::IUnknown {
    fn from(value: &DnssdServiceInstance) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DnssdServiceInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DnssdServiceInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DnssdServiceInstance> for ::windows::core::IInspectable {
    fn from(value: DnssdServiceInstance) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DnssdServiceInstance> for ::windows::core::IInspectable {
    fn from(value: &DnssdServiceInstance) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DnssdServiceInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DnssdServiceInstance {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<DnssdServiceInstance> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: DnssdServiceInstance) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&DnssdServiceInstance> for super::super::super::Foundation::IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &DnssdServiceInstance) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for DnssdServiceInstance {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IStringable> for &DnssdServiceInstance {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IStringable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for DnssdServiceInstance {}
unsafe impl ::core::marker::Sync for DnssdServiceInstance {}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DnssdServiceInstanceCollection(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl DnssdServiceInstanceCollection {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<DnssdServiceInstance> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<DnssdServiceInstance>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, DnssdServiceInstance>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<DnssdServiceInstance as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<DnssdServiceInstance>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<DnssdServiceInstance>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for DnssdServiceInstanceCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstanceCollection;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance;{e246db7e-98a5-4ca1-b9e4-c253d33c35ff})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for DnssdServiceInstanceCollection {
    type Vtable = super::super::super::Foundation::Collections::IVectorView_abi<DnssdServiceInstance>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for DnssdServiceInstanceCollection {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstanceCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DnssdServiceInstanceCollection> for ::windows::core::IUnknown {
    fn from(value: DnssdServiceInstanceCollection) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DnssdServiceInstanceCollection> for ::windows::core::IUnknown {
    fn from(value: &DnssdServiceInstanceCollection) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DnssdServiceInstanceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DnssdServiceInstanceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DnssdServiceInstanceCollection> for ::windows::core::IInspectable {
    fn from(value: DnssdServiceInstanceCollection) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DnssdServiceInstanceCollection> for ::windows::core::IInspectable {
    fn from(value: &DnssdServiceInstanceCollection) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DnssdServiceInstanceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DnssdServiceInstanceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DnssdServiceInstanceCollection> for super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance> {
    fn from(value: DnssdServiceInstanceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DnssdServiceInstanceCollection> for super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance> {
    fn from(value: &DnssdServiceInstanceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance>> for DnssdServiceInstanceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance>> for &DnssdServiceInstanceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<DnssdServiceInstanceCollection> for super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance> {
    type Error = ::windows::core::Error;
    fn try_from(value: DnssdServiceInstanceCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&DnssdServiceInstanceCollection> for super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DnssdServiceInstanceCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance>> for DnssdServiceInstanceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance>> for &DnssdServiceInstanceCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for DnssdServiceInstanceCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for DnssdServiceInstanceCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for DnssdServiceInstanceCollection {
    type Item = DnssdServiceInstance;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &DnssdServiceInstanceCollection {
    type Item = DnssdServiceInstance;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DnssdServiceWatcher(pub ::windows::core::IInspectable);
impl DnssdServiceWatcher {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`*"]
    pub fn Added<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, DnssdServiceInstance>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`*"]
    pub fn RemoveAdded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`*"]
    pub fn EnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`*"]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`*"]
    pub fn Stopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`*"]
    pub fn RemoveStopped<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn Status(&self) -> ::windows::core::Result<DnssdServiceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: DnssdServiceWatcherStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DnssdServiceWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DnssdServiceWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcher;{cc34d9c1-db7d-4b69-983d-c6f83f205682})");
}
unsafe impl ::windows::core::Interface for DnssdServiceWatcher {
    type Vtable = IDnssdServiceWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc34d9c1_db7d_4b69_983d_c6f83f205682);
}
impl ::windows::core::RuntimeName for DnssdServiceWatcher {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcher";
}
impl ::core::convert::From<DnssdServiceWatcher> for ::windows::core::IUnknown {
    fn from(value: DnssdServiceWatcher) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&DnssdServiceWatcher> for ::windows::core::IUnknown {
    fn from(value: &DnssdServiceWatcher) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DnssdServiceWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DnssdServiceWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<DnssdServiceWatcher> for ::windows::core::IInspectable {
    fn from(value: DnssdServiceWatcher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&DnssdServiceWatcher> for ::windows::core::IInspectable {
    fn from(value: &DnssdServiceWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DnssdServiceWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DnssdServiceWatcher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for DnssdServiceWatcher {}
unsafe impl ::core::marker::Sync for DnssdServiceWatcher {}
#[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DnssdServiceWatcherStatus(pub i32);
impl DnssdServiceWatcherStatus {
    pub const Created: DnssdServiceWatcherStatus = DnssdServiceWatcherStatus(0i32);
    pub const Started: DnssdServiceWatcherStatus = DnssdServiceWatcherStatus(1i32);
    pub const EnumerationCompleted: DnssdServiceWatcherStatus = DnssdServiceWatcherStatus(2i32);
    pub const Stopping: DnssdServiceWatcherStatus = DnssdServiceWatcherStatus(3i32);
    pub const Stopped: DnssdServiceWatcherStatus = DnssdServiceWatcherStatus(4i32);
    pub const Aborted: DnssdServiceWatcherStatus = DnssdServiceWatcherStatus(5i32);
}
impl ::core::convert::From<i32> for DnssdServiceWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DnssdServiceWatcherStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DnssdServiceWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcherStatus;i4)");
}
impl ::windows::core::DefaultType for DnssdServiceWatcherStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDnssdRegistrationResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDnssdRegistrationResult {
    type Vtable = IDnssdRegistrationResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d786ad2_e606_5350_73ea_7e97f066162f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDnssdRegistrationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DnssdRegistrationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDnssdServiceInstance(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDnssdServiceInstance {
    type Vtable = IDnssdServiceInstance_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe246db7e_98a5_4ca1_b9e4_c253d33c35ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDnssdServiceInstance_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, socket: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, socket: ::windows::core::RawPtr, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, socket: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, socket: ::windows::core::RawPtr, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDnssdServiceInstanceFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDnssdServiceInstanceFactory {
    type Vtable = IDnssdServiceInstanceFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cb061a1_c478_4331_9684_4af2186c0a2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDnssdServiceInstanceFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, dnssdserviceinstancename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, hostname: ::windows::core::RawPtr, port: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDnssdServiceWatcher(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDnssdServiceWatcher {
    type Vtable = IDnssdServiceWatcher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc34d9c1_db7d_4b69_983d_c6f83f205682);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDnssdServiceWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DnssdServiceWatcherStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
