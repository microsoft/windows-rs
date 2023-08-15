#[doc(hidden)]
#[repr(transparent)]
pub struct IDnssdRegistrationResult(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDnssdRegistrationResult {
    type Vtable = IDnssdRegistrationResult_Vtbl;
}
impl ::core::clone::Clone for IDnssdRegistrationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDnssdRegistrationResult {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d786ad2_e606_5350_73ea_7e97f066162f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDnssdRegistrationResult_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DnssdRegistrationStatus) -> ::windows_core::HRESULT,
    pub IPAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub HasInstanceNameChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDnssdServiceInstance(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDnssdServiceInstance {
    type Vtable = IDnssdServiceInstance_Vtbl;
}
impl ::core::clone::Clone for IDnssdServiceInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDnssdServiceInstance {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe246db7e_98a5_4ca1_b9e4_c253d33c35ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDnssdServiceInstance_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DnssdServiceInstanceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub SetDnssdServiceInstanceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub HostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetHostName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Port: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub SetPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows_core::HRESULT,
    pub Priority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows_core::HRESULT,
    pub Weight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT,
    pub SetWeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TextAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TextAttributes: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub RegisterStreamSocketListenerAsync1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))]
    RegisterStreamSocketListenerAsync1: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))]
    pub RegisterStreamSocketListenerAsync2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socket: *mut ::core::ffi::c_void, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets")))]
    RegisterStreamSocketListenerAsync2: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub RegisterDatagramSocketAsync1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socket: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Sockets")))]
    RegisterDatagramSocketAsync1: usize,
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))]
    pub RegisterDatagramSocketAsync2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, socket: *mut ::core::ffi::c_void, adapter: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets")))]
    RegisterDatagramSocketAsync2: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDnssdServiceInstanceFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDnssdServiceInstanceFactory {
    type Vtable = IDnssdServiceInstanceFactory_Vtbl;
}
impl ::core::clone::Clone for IDnssdServiceInstanceFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDnssdServiceInstanceFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6cb061a1_c478_4331_9684_4af2186c0a2b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDnssdServiceInstanceFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dnssdserviceinstancename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, hostname: *mut ::core::ffi::c_void, port: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDnssdServiceWatcher(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IDnssdServiceWatcher {
    type Vtable = IDnssdServiceWatcher_Vtbl;
}
impl ::core::clone::Clone for IDnssdServiceWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IDnssdServiceWatcher {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcc34d9c1_db7d_4b69_983d_c6f83f205682);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDnssdServiceWatcher_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Added: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Added: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAdded: usize,
    #[cfg(feature = "Foundation")]
    pub EnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnumerationCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnumerationCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DnssdServiceWatcherStatus) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Networking_ServiceDiscovery_Dnssd\"`*"]
#[repr(transparent)]
pub struct DnssdRegistrationResult(::windows_core::IUnknown);
impl DnssdRegistrationResult {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DnssdRegistrationResult, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Status(&self) -> ::windows_core::Result<DnssdRegistrationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IPAddress(&self) -> ::windows_core::Result<super::super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IPAddress)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn HasInstanceNameChanged(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasInstanceNameChanged)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for DnssdRegistrationResult {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationResult;{3d786ad2-e606-5350-73ea-7e97f066162f})");
}
impl ::core::clone::Clone for DnssdRegistrationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DnssdRegistrationResult {
    type Vtable = IDnssdRegistrationResult_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DnssdRegistrationResult {
    const IID: ::windows_core::GUID = <IDnssdRegistrationResult as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DnssdRegistrationResult {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationResult";
}
::windows_core::imp::interface_hierarchy!(DnssdRegistrationResult, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IStringable> for DnssdRegistrationResult {}
unsafe impl ::core::marker::Send for DnssdRegistrationResult {}
unsafe impl ::core::marker::Sync for DnssdRegistrationResult {}
#[doc = "*Required features: `\"Networking_ServiceDiscovery_Dnssd\"`*"]
#[repr(transparent)]
pub struct DnssdServiceInstance(::windows_core::IUnknown);
impl DnssdServiceInstance {
    pub fn DnssdServiceInstanceName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DnssdServiceInstanceName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDnssdServiceInstanceName(&self, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDnssdServiceInstanceName)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn HostName(&self) -> ::windows_core::Result<super::super::HostName> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HostName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetHostName<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::HostName>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetHostName)(::windows_core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Port(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Port)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPort(&self, value: u16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPort)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Priority(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Priority)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPriority(&self, value: u16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPriority)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Weight(&self) -> ::windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Weight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetWeight(&self, value: u16) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetWeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TextAttributes(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TextAttributes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Networking_Sockets\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub fn RegisterStreamSocketListenerAsync1<P0>(&self, socket: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Sockets::StreamSocketListener>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegisterStreamSocketListenerAsync1)(::windows_core::Interface::as_raw(this), socket.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Networking_Connectivity\"`, `\"Networking_Sockets\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))]
    pub fn RegisterStreamSocketListenerAsync2<P0, P1>(&self, socket: P0, adapter: P1) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Sockets::StreamSocketListener>,
        P1: ::windows_core::IntoParam<super::super::Connectivity::NetworkAdapter>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegisterStreamSocketListenerAsync2)(::windows_core::Interface::as_raw(this), socket.into_param().abi(), adapter.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Networking_Sockets\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Sockets"))]
    pub fn RegisterDatagramSocketAsync1<P0>(&self, socket: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Sockets::DatagramSocket>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegisterDatagramSocketAsync1)(::windows_core::Interface::as_raw(this), socket.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Networking_Connectivity\"`, `\"Networking_Sockets\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Networking_Sockets"))]
    pub fn RegisterDatagramSocketAsync2<P0, P1>(&self, socket: P0, adapter: P1) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>
    where
        P0: ::windows_core::IntoParam<super::super::Sockets::DatagramSocket>,
        P1: ::windows_core::IntoParam<super::super::Connectivity::NetworkAdapter>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RegisterDatagramSocketAsync2)(::windows_core::Interface::as_raw(this), socket.into_param().abi(), adapter.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Create<P0>(dnssdserviceinstancename: &::windows_core::HSTRING, hostname: P0, port: u16) -> ::windows_core::Result<DnssdServiceInstance>
    where
        P0: ::windows_core::IntoParam<super::super::HostName>,
    {
        Self::IDnssdServiceInstanceFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(dnssdserviceinstancename), hostname.into_param().abi(), port, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IDnssdServiceInstanceFactory<R, F: FnOnce(&IDnssdServiceInstanceFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<DnssdServiceInstance, IDnssdServiceInstanceFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for DnssdServiceInstance {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance;{e246db7e-98a5-4ca1-b9e4-c253d33c35ff})");
}
impl ::core::clone::Clone for DnssdServiceInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DnssdServiceInstance {
    type Vtable = IDnssdServiceInstance_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DnssdServiceInstance {
    const IID: ::windows_core::GUID = <IDnssdServiceInstance as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DnssdServiceInstance {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance";
}
::windows_core::imp::interface_hierarchy!(DnssdServiceInstance, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IStringable> for DnssdServiceInstance {}
unsafe impl ::core::marker::Send for DnssdServiceInstance {}
unsafe impl ::core::marker::Sync for DnssdServiceInstance {}
#[doc = "*Required features: `\"Networking_ServiceDiscovery_Dnssd\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct DnssdServiceInstanceCollection(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl DnssdServiceInstanceCollection {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<DnssdServiceInstance>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<DnssdServiceInstance> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<DnssdServiceInstance>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<DnssdServiceInstance>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
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
impl ::windows_core::RuntimeType for DnssdServiceInstanceCollection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstanceCollection;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceInstance;{e246db7e-98a5-4ca1-b9e4-c253d33c35ff})))");
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for DnssdServiceInstanceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for DnssdServiceInstanceCollection {
    type Vtable = super::super::super::Foundation::Collections::IVectorView_Vtbl<DnssdServiceInstance>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for DnssdServiceInstanceCollection {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for DnssdServiceInstanceCollection {
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
        super::super::super::Foundation::Collections::VectorViewIterator::new(::windows_core::ComInterface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(DnssdServiceInstanceCollection, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<DnssdServiceInstance>> for DnssdServiceInstanceCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IVectorView<DnssdServiceInstance>> for DnssdServiceInstanceCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for DnssdServiceInstanceCollection {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for DnssdServiceInstanceCollection {}
#[doc = "*Required features: `\"Networking_ServiceDiscovery_Dnssd\"`*"]
#[repr(transparent)]
pub struct DnssdServiceWatcher(::windows_core::IUnknown);
impl DnssdServiceWatcher {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Added<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, DnssdServiceInstance>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Added)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAdded(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveAdded)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EnumerationCompleted)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveEnumerationCompleted)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Status(&self) -> ::windows_core::Result<DnssdServiceWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Status)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
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
impl ::windows_core::RuntimeType for DnssdServiceWatcher {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcher;{cc34d9c1-db7d-4b69-983d-c6f83f205682})");
}
impl ::core::clone::Clone for DnssdServiceWatcher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for DnssdServiceWatcher {
    type Vtable = IDnssdServiceWatcher_Vtbl;
}
unsafe impl ::windows_core::ComInterface for DnssdServiceWatcher {
    const IID: ::windows_core::GUID = <IDnssdServiceWatcher as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for DnssdServiceWatcher {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcher";
}
::windows_core::imp::interface_hierarchy!(DnssdServiceWatcher, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for DnssdServiceWatcher {}
unsafe impl ::core::marker::Sync for DnssdServiceWatcher {}
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
impl ::windows_core::TypeKind for DnssdRegistrationStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DnssdRegistrationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DnssdRegistrationStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DnssdRegistrationStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.ServiceDiscovery.Dnssd.DnssdRegistrationStatus;i4)");
}
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
impl ::windows_core::TypeKind for DnssdServiceWatcherStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for DnssdServiceWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DnssdServiceWatcherStatus").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for DnssdServiceWatcherStatus {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Networking.ServiceDiscovery.Dnssd.DnssdServiceWatcherStatus;i4)");
}
