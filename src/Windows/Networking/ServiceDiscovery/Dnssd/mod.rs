#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct DnssdRegistrationResult(::windows::runtime::IInspectable);
impl DnssdRegistrationResult {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DnssdRegistrationResult, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<DnssdRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__: DnssdRegistrationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DnssdRegistrationStatus>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn IPAddress(&self) -> ::windows::runtime::Result<super::super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::HostName>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn HasInstanceNameChanged(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DnssdRegistrationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationResult;{3d786ad2-e606-5350-73ea-7e97f066162f})");
}
unsafe impl ::windows::runtime::Interface for DnssdRegistrationResult {
    type Vtable = IDnssdRegistrationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1031301842, 58886, 21328, [115, 234, 126, 151, 240, 102, 22, 47]);
}
impl ::windows::runtime::RuntimeName for DnssdRegistrationResult {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationResult";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<DnssdRegistrationResult> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DnssdRegistrationResult) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&DnssdRegistrationResult> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DnssdRegistrationResult) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for DnssdRegistrationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &DnssdRegistrationResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for DnssdRegistrationResult {}
unsafe impl ::std::marker::Sync for DnssdRegistrationResult {}
#[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DnssdRegistrationStatus(pub i32);
impl DnssdRegistrationStatus {
    pub const Success: DnssdRegistrationStatus = DnssdRegistrationStatus(0i32);
    pub const InvalidServiceName: DnssdRegistrationStatus = DnssdRegistrationStatus(1i32);
    pub const ServerError: DnssdRegistrationStatus = DnssdRegistrationStatus(2i32);
    pub const SecurityError: DnssdRegistrationStatus = DnssdRegistrationStatus(3i32);
}
impl ::std::convert::From<i32> for DnssdRegistrationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DnssdRegistrationStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DnssdRegistrationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationStatus;i4)");
}
#[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct DnssdServiceInstance(::windows::runtime::IInspectable);
impl DnssdServiceInstance {
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn DnssdServiceInstanceName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn SetDnssdServiceInstanceName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn HostName(&self) -> ::windows::runtime::Result<super::super::HostName> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::HostName>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn SetHostName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::HostName>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn Port(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn SetPort(&self, value: u16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn Priority(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn SetPriority(&self, value: u16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn Weight(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn SetWeight(&self, value: u16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation_Collections`*"]
    pub fn TextAttributes(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`, `Networking_Sockets`*"]
    pub fn RegisterStreamSocketListenerAsync1<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Sockets::StreamSocketListener>>(&self, socket: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), socket.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`, `Networking_Connectivity`, `Networking_Sockets`*"]
    pub fn RegisterStreamSocketListenerAsync2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Sockets::StreamSocketListener>, Param1: ::windows::runtime::IntoParam<'a, super::super::Connectivity::NetworkAdapter>>(&self, socket: Param0, adapter: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), socket.into_param().abi(), adapter.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`, `Networking_Sockets`*"]
    pub fn RegisterDatagramSocketAsync1<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Sockets::DatagramSocket>>(&self, socket: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), socket.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`, `Networking_Connectivity`, `Networking_Sockets`*"]
    pub fn RegisterDatagramSocketAsync2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Sockets::DatagramSocket>, Param1: ::windows::runtime::IntoParam<'a, super::super::Connectivity::NetworkAdapter>>(&self, socket: Param0, adapter: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), socket.into_param().abi(), adapter.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::HostName>>(dnssdserviceinstancename: Param0, hostname: Param1, port: u16) -> ::windows::runtime::Result<DnssdServiceInstance> {
        Self::IDnssdServiceInstanceFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), dnssdserviceinstancename.into_param().abi(), hostname.into_param().abi(), port, &mut result__).from_abi::<DnssdServiceInstance>(result__)
        })
    }
    pub fn IDnssdServiceInstanceFactory<R, F: FnOnce(&IDnssdServiceInstanceFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DnssdServiceInstance, IDnssdServiceInstanceFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DnssdServiceInstance {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance;{e246db7e-98a5-4ca1-b9e4-c253d33c35ff})");
}
unsafe impl ::windows::runtime::Interface for DnssdServiceInstance {
    type Vtable = IDnssdServiceInstance_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3796294526, 39077, 19617, [185, 228, 194, 83, 211, 60, 53, 255]);
}
impl ::windows::runtime::RuntimeName for DnssdServiceInstance {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<DnssdServiceInstance> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DnssdServiceInstance) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&DnssdServiceInstance> for super::super::super::Foundation::IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DnssdServiceInstance) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for DnssdServiceInstance {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IStringable> for &DnssdServiceInstance {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IStringable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for DnssdServiceInstance {}
unsafe impl ::std::marker::Sync for DnssdServiceInstance {}
#[cfg(feature = "Foundation_Collections")]
#[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation_Collections`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct DnssdServiceInstanceCollection(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl DnssdServiceInstanceCollection {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<DnssdServiceInstance> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<DnssdServiceInstance>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, DnssdServiceInstance>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<DnssdServiceInstance as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IIterator<DnssdServiceInstance>> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<DnssdServiceInstance>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::RuntimeType for DnssdServiceInstanceCollection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstanceCollection;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance;{e246db7e-98a5-4ca1-b9e4-c253d33c35ff})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::runtime::Interface for DnssdServiceInstanceCollection {
    type Vtable = super::super::super::Foundation::Collections::IVectorView_abi<DnssdServiceInstance>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::runtime::RuntimeName for DnssdServiceInstanceCollection {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstanceCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<DnssdServiceInstanceCollection> for super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance> {
    fn from(value: DnssdServiceInstanceCollection) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::From<&DnssdServiceInstanceCollection> for super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance> {
    fn from(value: &DnssdServiceInstanceCollection) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance>> for DnssdServiceInstanceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance>>::into(self))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance>> for &DnssdServiceInstanceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance>>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<DnssdServiceInstanceCollection> for super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DnssdServiceInstanceCollection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&DnssdServiceInstanceCollection> for super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DnssdServiceInstanceCollection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance>> for DnssdServiceInstanceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance>> for &DnssdServiceInstanceCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance>> {
        ::std::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::std::marker::Send for DnssdServiceInstanceCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::std::marker::Sync for DnssdServiceInstanceCollection {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for DnssdServiceInstanceCollection {
    type Item = DnssdServiceInstance;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &DnssdServiceInstanceCollection {
    type Item = DnssdServiceInstance;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct DnssdServiceWatcher(::windows::runtime::IInspectable);
impl DnssdServiceWatcher {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`*"]
    pub fn Added<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, DnssdServiceInstance>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`*"]
    pub fn RemoveAdded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`*"]
    pub fn EnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`*"]
    pub fn RemoveEnumerationCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`*"]
    pub fn Stopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`, `Foundation`*"]
    pub fn RemoveStopped<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<DnssdServiceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: DnssdServiceWatcherStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DnssdServiceWatcherStatus>(result__)
        }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DnssdServiceWatcher {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcher;{cc34d9c1-db7d-4b69-983d-c6f83f205682})");
}
unsafe impl ::windows::runtime::Interface for DnssdServiceWatcher {
    type Vtable = IDnssdServiceWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3426015681, 56189, 19305, [152, 61, 198, 248, 63, 32, 86, 130]);
}
impl ::windows::runtime::RuntimeName for DnssdServiceWatcher {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcher";
}
unsafe impl ::std::marker::Send for DnssdServiceWatcher {}
unsafe impl ::std::marker::Sync for DnssdServiceWatcher {}
#[doc = "*Required features: `Networking_ServiceDiscovery_Dnssd`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DnssdServiceWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DnssdServiceWatcherStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DnssdServiceWatcherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcherStatus;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDnssdRegistrationResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDnssdRegistrationResult {
    type Vtable = IDnssdRegistrationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1031301842, 58886, 21328, [115, 234, 126, 151, 240, 102, 22, 47]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDnssdRegistrationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DnssdRegistrationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDnssdServiceInstance(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDnssdServiceInstance {
    type Vtable = IDnssdServiceInstance_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3796294526, 39077, 19617, [185, 228, 194, 83, 211, 60, 53, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDnssdServiceInstance_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, socket: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, socket: ::windows::runtime::RawPtr, adapter: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, socket: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, socket: ::windows::runtime::RawPtr, adapter: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDnssdServiceInstanceFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDnssdServiceInstanceFactory {
    type Vtable = IDnssdServiceInstanceFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1823498657, 50296, 17201, [150, 132, 74, 242, 24, 108, 10, 43]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDnssdServiceInstanceFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dnssdserviceinstancename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, hostname: ::windows::runtime::RawPtr, port: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IDnssdServiceWatcher(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDnssdServiceWatcher {
    type Vtable = IDnssdServiceWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3426015681, 56189, 19305, [152, 61, 198, 248, 63, 32, 86, 130]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDnssdServiceWatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DnssdServiceWatcherStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
