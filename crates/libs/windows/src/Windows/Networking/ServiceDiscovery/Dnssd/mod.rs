#[doc = "*Required features: `\"Networking_ServiceDiscovery_Dnssd\"`*"]
#[repr(transparent)]
pub struct DnssdRegistrationResult(::windows::core::IUnknown);
impl DnssdRegistrationResult {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DnssdRegistrationResult, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Status(&self) -> ::windows::core::Result<DnssdRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DnssdRegistrationStatus>(result__)
        }
    }
    pub fn IPAddress(&self) -> ::windows::core::Result<super::super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IPAddress)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::HostName>(result__)
        }
    }
    pub fn HasInstanceNameChanged(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasInstanceNameChanged)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for DnssdRegistrationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DnssdRegistrationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DnssdRegistrationResult {}
impl ::core::fmt::Debug for DnssdRegistrationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DnssdRegistrationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DnssdRegistrationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationResult;{3d786ad2-e606-5350-73ea-7e97f066162f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DnssdRegistrationResult {
    type Vtable = IDnssdRegistrationResult_Vtbl;
    const IID: ::windows::core::GUID = <IDnssdRegistrationResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DnssdRegistrationResult {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationResult";
}
impl ::core::convert::From<DnssdRegistrationResult> for ::windows::core::IUnknown {
    fn from(value: DnssdRegistrationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DnssdRegistrationResult> for ::windows::core::IUnknown {
    fn from(value: &DnssdRegistrationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DnssdRegistrationResult> for &::windows::core::IUnknown {
    fn from(value: &DnssdRegistrationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DnssdRegistrationResult> for ::windows::core::IInspectable {
    fn from(value: DnssdRegistrationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DnssdRegistrationResult> for ::windows::core::IInspectable {
    fn from(value: &DnssdRegistrationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DnssdRegistrationResult> for &::windows::core::IInspectable {
    fn from(value: &DnssdRegistrationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&DnssdRegistrationResult> for ::windows::core::InParam<'a, super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DnssdRegistrationResult) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for DnssdRegistrationResult {}
unsafe impl ::core::marker::Sync for DnssdRegistrationResult {}
#[doc = "*Required features: `\"Networking_ServiceDiscovery_Dnssd\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DnssdRegistrationStatus(pub i32);
impl DnssdRegistrationStatus {
    pub const Success: Self = Self(0i32);
    pub const InvalidServiceName: Self = Self(1i32);
    pub const ServerError: Self = Self(2i32);
    pub const SecurityError: Self = Self(3i32);
}
impl ::core::marker::Copy for DnssdRegistrationStatus {}
impl ::core::clone::Clone for DnssdRegistrationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DnssdRegistrationStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DnssdRegistrationStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DnssdRegistrationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DnssdRegistrationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DnssdRegistrationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Networking_ServiceDiscovery_Dnssd\"`*"]
#[repr(transparent)]
pub struct DnssdServiceInstance(::windows::core::IUnknown);
impl DnssdServiceInstance {
    pub fn DnssdServiceInstanceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DnssdServiceInstanceName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDnssdServiceInstanceName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDnssdServiceInstanceName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn HostName(&self) -> ::windows::core::Result<super::super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HostName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::HostName>(result__)
        }
    }
    pub fn SetHostName<'a, P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::HostName>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHostName)(::windows::core::Interface::as_raw(this), value.into().abi()).ok() }
    }
    pub fn Port(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Port)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn SetPort(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPort)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Priority(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Priority)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn SetPriority(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPriority)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Weight(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Weight)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u16>(result__)
        }
    }
    pub fn SetWeight(&self, value: u16) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetWeight)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TextAttributes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TextAttributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Networking_Sockets\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub fn RegisterStreamSocketListenerAsync1<'a, P0>(&self, socket: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Sockets::StreamSocketListener>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RegisterStreamSocketListenerAsync1)(::windows::core::Interface::as_raw(this), socket.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Networking_Connectivity\"`, `\"Networking_Sockets\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))]
    pub fn RegisterStreamSocketListenerAsync2<'a, P0, P1>(&self, socket: P0, adapter: P1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Sockets::StreamSocketListener>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Connectivity::NetworkAdapter>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RegisterStreamSocketListenerAsync2)(::windows::core::Interface::as_raw(this), socket.into().abi(), adapter.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Networking_Sockets\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub fn RegisterDatagramSocketAsync1<'a, P0>(&self, socket: P0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Sockets::DatagramSocket>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RegisterDatagramSocketAsync1)(::windows::core::Interface::as_raw(this), socket.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Networking_Connectivity\"`, `\"Networking_Sockets\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))]
    pub fn RegisterDatagramSocketAsync2<'a, P0, P1>(&self, socket: P0, adapter: P1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Sockets::DatagramSocket>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::Connectivity::NetworkAdapter>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RegisterDatagramSocketAsync2)(::windows::core::Interface::as_raw(this), socket.into().abi(), adapter.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>(result__)
        }
    }
    pub fn Create<'a, P0>(dnssdserviceinstancename: &::windows::core::HSTRING, hostname: P0, port: u16) -> ::windows::core::Result<DnssdServiceInstance>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::HostName>>,
    {
        Self::IDnssdServiceInstanceFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(dnssdserviceinstancename), hostname.into().abi(), port, result__.as_mut_ptr()).from_abi::<DnssdServiceInstance>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ToString)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IDnssdServiceInstanceFactory<R, F: FnOnce(&IDnssdServiceInstanceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<DnssdServiceInstance, IDnssdServiceInstanceFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for DnssdServiceInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DnssdServiceInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DnssdServiceInstance {}
impl ::core::fmt::Debug for DnssdServiceInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DnssdServiceInstance").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DnssdServiceInstance {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance;{e246db7e-98a5-4ca1-b9e4-c253d33c35ff})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DnssdServiceInstance {
    type Vtable = IDnssdServiceInstance_Vtbl;
    const IID: ::windows::core::GUID = <IDnssdServiceInstance as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DnssdServiceInstance {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance";
}
impl ::core::convert::From<DnssdServiceInstance> for ::windows::core::IUnknown {
    fn from(value: DnssdServiceInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DnssdServiceInstance> for ::windows::core::IUnknown {
    fn from(value: &DnssdServiceInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DnssdServiceInstance> for &::windows::core::IUnknown {
    fn from(value: &DnssdServiceInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DnssdServiceInstance> for ::windows::core::IInspectable {
    fn from(value: DnssdServiceInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DnssdServiceInstance> for ::windows::core::IInspectable {
    fn from(value: &DnssdServiceInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DnssdServiceInstance> for &::windows::core::IInspectable {
    fn from(value: &DnssdServiceInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&DnssdServiceInstance> for ::windows::core::InParam<'a, super::super::super::Foundation::IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DnssdServiceInstance) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for DnssdServiceInstance {}
unsafe impl ::core::marker::Sync for DnssdServiceInstance {}
#[doc = "*Required features: `\"Networking_ServiceDiscovery_Dnssd\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct DnssdServiceInstanceCollection(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl DnssdServiceInstanceCollection {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<DnssdServiceInstance>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).First)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::Collections::IIterator<DnssdServiceInstance>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<DnssdServiceInstance> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(::windows::core::Interface::as_raw(this), index, result__.as_mut_ptr()).from_abi::<DnssdServiceInstance>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, P0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, DnssdServiceInstance>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(::windows::core::Interface::as_raw(this), value.into().abi(), index, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<DnssdServiceInstance>]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetMany)(::windows::core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for DnssdServiceInstanceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for DnssdServiceInstanceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for DnssdServiceInstanceCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for DnssdServiceInstanceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DnssdServiceInstanceCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for DnssdServiceInstanceCollection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstanceCollection;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance;{e246db7e-98a5-4ca1-b9e4-c253d33c35ff})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for DnssdServiceInstanceCollection {
    type Vtable = super::super::super::Foundation::Collections::IVectorView_Vtbl<DnssdServiceInstance>;
    const IID: ::windows::core::GUID = <super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance> as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for DnssdServiceInstanceCollection {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstanceCollection";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for DnssdServiceInstanceCollection {
    type Item = DnssdServiceInstance;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &DnssdServiceInstanceCollection {
    type Item = DnssdServiceInstance;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DnssdServiceInstanceCollection> for ::windows::core::IUnknown {
    fn from(value: DnssdServiceInstanceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DnssdServiceInstanceCollection> for ::windows::core::IUnknown {
    fn from(value: &DnssdServiceInstanceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DnssdServiceInstanceCollection> for &::windows::core::IUnknown {
    fn from(value: &DnssdServiceInstanceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<DnssdServiceInstanceCollection> for ::windows::core::IInspectable {
    fn from(value: DnssdServiceInstanceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DnssdServiceInstanceCollection> for ::windows::core::IInspectable {
    fn from(value: &DnssdServiceInstanceCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&DnssdServiceInstanceCollection> for &::windows::core::IInspectable {
    fn from(value: &DnssdServiceInstanceCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
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
impl<'a> ::core::convert::TryFrom<&DnssdServiceInstanceCollection> for ::windows::core::InParam<'a, super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DnssdServiceInstanceCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<DnssdServiceInstanceCollection> for super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance> {
    type Error = ::windows::core::Error;
    fn try_from(value: DnssdServiceInstanceCollection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&DnssdServiceInstanceCollection> for super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DnssdServiceInstanceCollection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::core::convert::TryFrom<&DnssdServiceInstanceCollection> for ::windows::core::InParam<'a, super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &DnssdServiceInstanceCollection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for DnssdServiceInstanceCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for DnssdServiceInstanceCollection {}
#[doc = "*Required features: `\"Networking_ServiceDiscovery_Dnssd\"`*"]
#[repr(transparent)]
pub struct DnssdServiceWatcher(::windows::core::IUnknown);
impl DnssdServiceWatcher {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, DnssdServiceInstance>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Added)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAdded)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EnumerationCompleted)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Stopped)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStopped)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> ::windows::core::Result<DnssdServiceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<DnssdServiceWatcherStatus>(result__)
        }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Start)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for DnssdServiceWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DnssdServiceWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DnssdServiceWatcher {}
impl ::core::fmt::Debug for DnssdServiceWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DnssdServiceWatcher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DnssdServiceWatcher {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcher;{cc34d9c1-db7d-4b69-983d-c6f83f205682})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DnssdServiceWatcher {
    type Vtable = IDnssdServiceWatcher_Vtbl;
    const IID: ::windows::core::GUID = <IDnssdServiceWatcher as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DnssdServiceWatcher {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcher";
}
impl ::core::convert::From<DnssdServiceWatcher> for ::windows::core::IUnknown {
    fn from(value: DnssdServiceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DnssdServiceWatcher> for ::windows::core::IUnknown {
    fn from(value: &DnssdServiceWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DnssdServiceWatcher> for &::windows::core::IUnknown {
    fn from(value: &DnssdServiceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<DnssdServiceWatcher> for ::windows::core::IInspectable {
    fn from(value: DnssdServiceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DnssdServiceWatcher> for ::windows::core::IInspectable {
    fn from(value: &DnssdServiceWatcher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&DnssdServiceWatcher> for &::windows::core::IInspectable {
    fn from(value: &DnssdServiceWatcher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for DnssdServiceWatcher {}
unsafe impl ::core::marker::Sync for DnssdServiceWatcher {}
#[doc = "*Required features: `\"Networking_ServiceDiscovery_Dnssd\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DnssdServiceWatcherStatus(pub i32);
impl DnssdServiceWatcherStatus {
    pub const Created: Self = Self(0i32);
    pub const Started: Self = Self(1i32);
    pub const EnumerationCompleted: Self = Self(2i32);
    pub const Stopping: Self = Self(3i32);
    pub const Stopped: Self = Self(4i32);
    pub const Aborted: Self = Self(5i32);
}
impl ::core::marker::Copy for DnssdServiceWatcherStatus {}
impl ::core::clone::Clone for DnssdServiceWatcherStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DnssdServiceWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DnssdServiceWatcherStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for DnssdServiceWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DnssdServiceWatcherStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DnssdServiceWatcherStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcherStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDnssdRegistrationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDnssdRegistrationResult {
    type Vtable = IDnssdRegistrationResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d786ad2_e606_5350_73ea_7e97f066162f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDnssdRegistrationResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DnssdRegistrationStatus) -> ::windows::core::HRESULT,
    pub IPAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HasInstanceNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDnssdServiceInstance(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDnssdServiceInstance {
    type Vtable = IDnssdServiceInstance_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe246db7e_98a5_4ca1_b9e4_c253d33c35ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDnssdServiceInstance_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DnssdServiceInstanceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDnssdServiceInstanceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Port: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub SetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT,
    pub Weight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TextAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TextAttributes: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub RegisterStreamSocketListenerAsync1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))]
    RegisterStreamSocketListenerAsync1: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))]
    pub RegisterStreamSocketListenerAsync2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socket: *mut ::core::ffi::c_void, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets")))]
    RegisterStreamSocketListenerAsync2: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub RegisterDatagramSocketAsync1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))]
    RegisterDatagramSocketAsync1: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))]
    pub RegisterDatagramSocketAsync2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socket: *mut ::core::ffi::c_void, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets")))]
    RegisterDatagramSocketAsync2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDnssdServiceInstanceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDnssdServiceInstanceFactory {
    type Vtable = IDnssdServiceInstanceFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cb061a1_c478_4331_9684_4af2186c0a2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDnssdServiceInstanceFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dnssdserviceinstancename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, hostname: *mut ::core::ffi::c_void, port: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDnssdServiceWatcher(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDnssdServiceWatcher {
    type Vtable = IDnssdServiceWatcher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc34d9c1_db7d_4b69_983d_c6f83f205682);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDnssdServiceWatcher_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DnssdServiceWatcherStatus) -> ::windows::core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
