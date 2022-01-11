#[cfg(feature = "implement_exclusive")]
pub trait IDnssdRegistrationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<DnssdRegistrationStatus>;
    fn IPAddress(&self) -> ::windows::core::Result<super::super::HostName>;
    fn HasInstanceNameChanged(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDnssdRegistrationResult {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.IDnssdRegistrationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IDnssdRegistrationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDnssdRegistrationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDnssdRegistrationResultVtbl {
        unsafe extern "system" fn Status<Impl: IDnssdRegistrationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DnssdRegistrationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IPAddress<Impl: IDnssdRegistrationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IPAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasInstanceNameChanged<Impl: IDnssdRegistrationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasInstanceNameChanged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDnssdRegistrationResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>, IPAddress::<Impl, IMPL_OFFSET>, HasInstanceNameChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDnssdRegistrationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking_Connectivity", feature = "Networking_Sockets", feature = "implement_exclusive"))]
pub trait IDnssdServiceInstanceImpl: Sized {
    fn DnssdServiceInstanceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDnssdServiceInstanceName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HostName(&self) -> ::windows::core::Result<super::super::HostName>;
    fn SetHostName(&self, value: &::core::option::Option<super::super::HostName>) -> ::windows::core::Result<()>;
    fn Port(&self) -> ::windows::core::Result<u16>;
    fn SetPort(&self, value: u16) -> ::windows::core::Result<()>;
    fn Priority(&self) -> ::windows::core::Result<u16>;
    fn SetPriority(&self, value: u16) -> ::windows::core::Result<()>;
    fn Weight(&self) -> ::windows::core::Result<u16>;
    fn SetWeight(&self, value: u16) -> ::windows::core::Result<()>;
    fn TextAttributes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn RegisterStreamSocketListenerAsync1(&self, socket: &::core::option::Option<super::super::Sockets::StreamSocketListener>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>;
    fn RegisterStreamSocketListenerAsync2(&self, socket: &::core::option::Option<super::super::Sockets::StreamSocketListener>, adapter: &::core::option::Option<super::super::Connectivity::NetworkAdapter>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>;
    fn RegisterDatagramSocketAsync1(&self, socket: &::core::option::Option<super::super::Sockets::DatagramSocket>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>;
    fn RegisterDatagramSocketAsync2(&self, socket: &::core::option::Option<super::super::Sockets::DatagramSocket>, adapter: &::core::option::Option<super::super::Connectivity::NetworkAdapter>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DnssdRegistrationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking_Connectivity", feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDnssdServiceInstance {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceInstance";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking_Connectivity", feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl IDnssdServiceInstanceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDnssdServiceInstanceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDnssdServiceInstanceVtbl {
        unsafe extern "system" fn DnssdServiceInstanceName<Impl: IDnssdServiceInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DnssdServiceInstanceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDnssdServiceInstanceName<Impl: IDnssdServiceInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDnssdServiceInstanceName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HostName<Impl: IDnssdServiceInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HostName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHostName<Impl: IDnssdServiceInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHostName(&*(&value as *const <super::super::HostName as ::windows::core::Abi>::Abi as *const <super::super::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Port<Impl: IDnssdServiceInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Port() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Impl: IDnssdServiceInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPort(value).into()
        }
        unsafe extern "system" fn Priority<Impl: IDnssdServiceInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IDnssdServiceInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(value).into()
        }
        unsafe extern "system" fn Weight<Impl: IDnssdServiceInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Weight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeight<Impl: IDnssdServiceInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWeight(value).into()
        }
        unsafe extern "system" fn TextAttributes<Impl: IDnssdServiceInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TextAttributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterStreamSocketListenerAsync1<Impl: IDnssdServiceInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socket: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterStreamSocketListenerAsync1(&*(&socket as *const <super::super::Sockets::StreamSocketListener as ::windows::core::Abi>::Abi as *const <super::super::Sockets::StreamSocketListener as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterStreamSocketListenerAsync2<Impl: IDnssdServiceInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socket: ::windows::core::RawPtr, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterStreamSocketListenerAsync2(&*(&socket as *const <super::super::Sockets::StreamSocketListener as ::windows::core::Abi>::Abi as *const <super::super::Sockets::StreamSocketListener as ::windows::core::DefaultType>::DefaultType), &*(&adapter as *const <super::super::Connectivity::NetworkAdapter as ::windows::core::Abi>::Abi as *const <super::super::Connectivity::NetworkAdapter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterDatagramSocketAsync1<Impl: IDnssdServiceInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socket: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterDatagramSocketAsync1(&*(&socket as *const <super::super::Sockets::DatagramSocket as ::windows::core::Abi>::Abi as *const <super::super::Sockets::DatagramSocket as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterDatagramSocketAsync2<Impl: IDnssdServiceInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socket: ::windows::core::RawPtr, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterDatagramSocketAsync2(&*(&socket as *const <super::super::Sockets::DatagramSocket as ::windows::core::Abi>::Abi as *const <super::super::Sockets::DatagramSocket as ::windows::core::DefaultType>::DefaultType), &*(&adapter as *const <super::super::Connectivity::NetworkAdapter as ::windows::core::Abi>::Abi as *const <super::super::Connectivity::NetworkAdapter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDnssdServiceInstance>,
            ::windows::core::GetTrustLevel,
            DnssdServiceInstanceName::<Impl, IMPL_OFFSET>,
            SetDnssdServiceInstanceName::<Impl, IMPL_OFFSET>,
            HostName::<Impl, IMPL_OFFSET>,
            SetHostName::<Impl, IMPL_OFFSET>,
            Port::<Impl, IMPL_OFFSET>,
            SetPort::<Impl, IMPL_OFFSET>,
            Priority::<Impl, IMPL_OFFSET>,
            SetPriority::<Impl, IMPL_OFFSET>,
            Weight::<Impl, IMPL_OFFSET>,
            SetWeight::<Impl, IMPL_OFFSET>,
            TextAttributes::<Impl, IMPL_OFFSET>,
            RegisterStreamSocketListenerAsync1::<Impl, IMPL_OFFSET>,
            RegisterStreamSocketListenerAsync2::<Impl, IMPL_OFFSET>,
            RegisterDatagramSocketAsync1::<Impl, IMPL_OFFSET>,
            RegisterDatagramSocketAsync2::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDnssdServiceInstance as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDnssdServiceInstanceFactoryImpl: Sized {
    fn Create(&self, dnssdserviceinstancename: &::windows::core::HSTRING, hostname: &::core::option::Option<super::super::HostName>, port: u16) -> ::windows::core::Result<DnssdServiceInstance>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDnssdServiceInstanceFactory {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceInstanceFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDnssdServiceInstanceFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDnssdServiceInstanceFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDnssdServiceInstanceFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IDnssdServiceInstanceFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dnssdserviceinstancename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, hostname: ::windows::core::RawPtr, port: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&dnssdserviceinstancename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&hostname as *const <super::super::HostName as ::windows::core::Abi>::Abi as *const <super::super::HostName as ::windows::core::DefaultType>::DefaultType), port) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDnssdServiceInstanceFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDnssdServiceInstanceFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDnssdServiceWatcherImpl: Sized {
    fn Added(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, DnssdServiceInstance>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<DnssdServiceWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDnssdServiceWatcher {
    const NAME: &'static str = "Windows.Networking.ServiceDiscovery.Dnssd.IDnssdServiceWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDnssdServiceWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDnssdServiceWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDnssdServiceWatcherVtbl {
        unsafe extern "system" fn Added<Impl: IDnssdServiceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Added(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, DnssdServiceInstance> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, DnssdServiceInstance> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdded<Impl: IDnssdServiceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IDnssdServiceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IDnssdServiceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Stopped<Impl: IDnssdServiceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stopped(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<DnssdServiceWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStopped<Impl: IDnssdServiceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStopped(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IDnssdServiceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DnssdServiceWatcherStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IDnssdServiceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IDnssdServiceWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDnssdServiceWatcher>,
            ::windows::core::GetTrustLevel,
            Added::<Impl, IMPL_OFFSET>,
            RemoveAdded::<Impl, IMPL_OFFSET>,
            EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            Stopped::<Impl, IMPL_OFFSET>,
            RemoveStopped::<Impl, IMPL_OFFSET>,
            Status::<Impl, IMPL_OFFSET>,
            Start::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDnssdServiceWatcher as ::windows::core::Interface>::IID
    }
}
