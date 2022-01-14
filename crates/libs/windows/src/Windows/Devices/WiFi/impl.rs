#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWiFiAdapter_Impl: Sized {
    fn NetworkAdapter(&mut self) -> ::windows::core::Result<super::super::Networking::Connectivity::NetworkAdapter>;
    fn ScanAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn NetworkReport(&mut self) -> ::windows::core::Result<WiFiNetworkReport>;
    fn AvailableNetworksChanged(&mut self, args: &::core::option::Option<super::super::Foundation::TypedEventHandler<WiFiAdapter, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailableNetworksChanged(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectAsync(&mut self, availablenetwork: &::core::option::Option<WiFiAvailableNetwork>, reconnectionkind: WiFiReconnectionKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>;
    fn ConnectWithPasswordCredentialAsync(&mut self, availablenetwork: &::core::option::Option<WiFiAvailableNetwork>, reconnectionkind: WiFiReconnectionKind, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>;
    fn ConnectWithPasswordCredentialAndSsidAsync(&mut self, availablenetwork: &::core::option::Option<WiFiAvailableNetwork>, reconnectionkind: WiFiReconnectionKind, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>, ssid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>;
    fn Disconnect(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiAdapter {
    const NAME: &'static str = "Windows.Devices.WiFi.IWiFiAdapter";
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWiFiAdapter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiAdapter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiAdapter_Vtbl {
        unsafe extern "system" fn NetworkAdapter<Impl: IWiFiAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkAdapter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScanAsync<Impl: IWiFiAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScanAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkReport<Impl: IWiFiAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkReport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableNetworksChanged<Impl: IWiFiAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableNetworksChanged(&*(&args as *const <super::super::Foundation::TypedEventHandler<WiFiAdapter, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<WiFiAdapter, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAvailableNetworksChanged<Impl: IWiFiAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAvailableNetworksChanged(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConnectAsync<Impl: IWiFiAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availablenetwork: ::windows::core::RawPtr, reconnectionkind: WiFiReconnectionKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectAsync(&*(&availablenetwork as *const <WiFiAvailableNetwork as ::windows::core::Abi>::Abi as *const <WiFiAvailableNetwork as ::windows::core::DefaultType>::DefaultType), reconnectionkind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectWithPasswordCredentialAsync<Impl: IWiFiAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availablenetwork: ::windows::core::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectWithPasswordCredentialAsync(&*(&availablenetwork as *const <WiFiAvailableNetwork as ::windows::core::Abi>::Abi as *const <WiFiAvailableNetwork as ::windows::core::DefaultType>::DefaultType), reconnectionkind, &*(&passwordcredential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectWithPasswordCredentialAndSsidAsync<Impl: IWiFiAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availablenetwork: ::windows::core::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows::core::RawPtr, ssid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectWithPasswordCredentialAndSsidAsync(
                &*(&availablenetwork as *const <WiFiAvailableNetwork as ::windows::core::Abi>::Abi as *const <WiFiAvailableNetwork as ::windows::core::DefaultType>::DefaultType),
                reconnectionkind,
                &*(&passwordcredential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType),
                &*(&ssid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: IWiFiAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiAdapter, BASE_OFFSET>(),
            NetworkAdapter: NetworkAdapter::<Impl, IMPL_OFFSET>,
            ScanAsync: ScanAsync::<Impl, IMPL_OFFSET>,
            NetworkReport: NetworkReport::<Impl, IMPL_OFFSET>,
            AvailableNetworksChanged: AvailableNetworksChanged::<Impl, IMPL_OFFSET>,
            RemoveAvailableNetworksChanged: RemoveAvailableNetworksChanged::<Impl, IMPL_OFFSET>,
            ConnectAsync: ConnectAsync::<Impl, IMPL_OFFSET>,
            ConnectWithPasswordCredentialAsync: ConnectWithPasswordCredentialAsync::<Impl, IMPL_OFFSET>,
            ConnectWithPasswordCredentialAndSsidAsync: ConnectWithPasswordCredentialAndSsidAsync::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiAdapter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWiFiAdapter2_Impl: Sized {
    fn GetWpsConfigurationAsync(&mut self, availablenetwork: &::core::option::Option<WiFiAvailableNetwork>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiWpsConfigurationResult>>;
    fn ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync(&mut self, availablenetwork: &::core::option::Option<WiFiAvailableNetwork>, reconnectionkind: WiFiReconnectionKind, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>, ssid: &::windows::core::HSTRING, connectionmethod: WiFiConnectionMethod) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiAdapter2 {
    const NAME: &'static str = "Windows.Devices.WiFi.IWiFiAdapter2";
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWiFiAdapter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiAdapter2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiAdapter2_Vtbl {
        unsafe extern "system" fn GetWpsConfigurationAsync<Impl: IWiFiAdapter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availablenetwork: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWpsConfigurationAsync(&*(&availablenetwork as *const <WiFiAvailableNetwork as ::windows::core::Abi>::Abi as *const <WiFiAvailableNetwork as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync<Impl: IWiFiAdapter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availablenetwork: ::windows::core::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows::core::RawPtr, ssid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, connectionmethod: WiFiConnectionMethod, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync(
                &*(&availablenetwork as *const <WiFiAvailableNetwork as ::windows::core::Abi>::Abi as *const <WiFiAvailableNetwork as ::windows::core::DefaultType>::DefaultType),
                reconnectionkind,
                &*(&passwordcredential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType),
                &*(&ssid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                connectionmethod,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiAdapter2, BASE_OFFSET>(),
            GetWpsConfigurationAsync: GetWpsConfigurationAsync::<Impl, IMPL_OFFSET>,
            ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync: ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiAdapter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWiFiAdapterStatics_Impl: Sized {
    fn FindAllAdaptersAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WiFiAdapter>>>;
    fn GetDeviceSelector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiAdapter>>;
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiAccessStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiAdapterStatics {
    const NAME: &'static str = "Windows.Devices.WiFi.IWiFiAdapterStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWiFiAdapterStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiAdapterStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiAdapterStatics_Vtbl {
        unsafe extern "system" fn FindAllAdaptersAsync<Impl: IWiFiAdapterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllAdaptersAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceSelector<Impl: IWiFiAdapterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSelector() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IWiFiAdapterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIdAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessAsync<Impl: IWiFiAdapterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiAdapterStatics, BASE_OFFSET>(),
            FindAllAdaptersAsync: FindAllAdaptersAsync::<Impl, IMPL_OFFSET>,
            GetDeviceSelector: GetDeviceSelector::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiAdapterStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait IWiFiAvailableNetwork_Impl: Sized {
    fn Uptime(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Ssid(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Bssid(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ChannelCenterFrequencyInKilohertz(&mut self) -> ::windows::core::Result<i32>;
    fn NetworkRssiInDecibelMilliwatts(&mut self) -> ::windows::core::Result<f64>;
    fn SignalBars(&mut self) -> ::windows::core::Result<u8>;
    fn NetworkKind(&mut self) -> ::windows::core::Result<WiFiNetworkKind>;
    fn PhyKind(&mut self) -> ::windows::core::Result<WiFiPhyKind>;
    fn SecuritySettings(&mut self) -> ::windows::core::Result<super::super::Networking::Connectivity::NetworkSecuritySettings>;
    fn BeaconInterval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn IsWiFiDirect(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiAvailableNetwork {
    const NAME: &'static str = "Windows.Devices.WiFi.IWiFiAvailableNetwork";
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl IWiFiAvailableNetwork_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiAvailableNetwork_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiAvailableNetwork_Vtbl {
        unsafe extern "system" fn Uptime<Impl: IWiFiAvailableNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uptime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ssid<Impl: IWiFiAvailableNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ssid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Bssid<Impl: IWiFiAvailableNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bssid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChannelCenterFrequencyInKilohertz<Impl: IWiFiAvailableNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChannelCenterFrequencyInKilohertz() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkRssiInDecibelMilliwatts<Impl: IWiFiAvailableNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkRssiInDecibelMilliwatts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignalBars<Impl: IWiFiAvailableNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignalBars() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NetworkKind<Impl: IWiFiAvailableNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiNetworkKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NetworkKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhyKind<Impl: IWiFiAvailableNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiPhyKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhyKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SecuritySettings<Impl: IWiFiAvailableNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SecuritySettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeaconInterval<Impl: IWiFiAvailableNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeaconInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWiFiDirect<Impl: IWiFiAvailableNetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWiFiDirect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiAvailableNetwork, BASE_OFFSET>(),
            Uptime: Uptime::<Impl, IMPL_OFFSET>,
            Ssid: Ssid::<Impl, IMPL_OFFSET>,
            Bssid: Bssid::<Impl, IMPL_OFFSET>,
            ChannelCenterFrequencyInKilohertz: ChannelCenterFrequencyInKilohertz::<Impl, IMPL_OFFSET>,
            NetworkRssiInDecibelMilliwatts: NetworkRssiInDecibelMilliwatts::<Impl, IMPL_OFFSET>,
            SignalBars: SignalBars::<Impl, IMPL_OFFSET>,
            NetworkKind: NetworkKind::<Impl, IMPL_OFFSET>,
            PhyKind: PhyKind::<Impl, IMPL_OFFSET>,
            SecuritySettings: SecuritySettings::<Impl, IMPL_OFFSET>,
            BeaconInterval: BeaconInterval::<Impl, IMPL_OFFSET>,
            IsWiFiDirect: IsWiFiDirect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiAvailableNetwork as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiConnectionResult_Impl: Sized {
    fn ConnectionStatus(&mut self) -> ::windows::core::Result<WiFiConnectionStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiConnectionResult {
    const NAME: &'static str = "Windows.Devices.WiFi.IWiFiConnectionResult";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiConnectionResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiConnectionResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiConnectionResult_Vtbl {
        unsafe extern "system" fn ConnectionStatus<Impl: IWiFiConnectionResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiConnectionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiConnectionResult, BASE_OFFSET>(),
            ConnectionStatus: ConnectionStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiConnectionResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWiFiNetworkReport_Impl: Sized {
    fn Timestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn AvailableNetworks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WiFiAvailableNetwork>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiNetworkReport {
    const NAME: &'static str = "Windows.Devices.WiFi.IWiFiNetworkReport";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWiFiNetworkReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiNetworkReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiNetworkReport_Vtbl {
        unsafe extern "system" fn Timestamp<Impl: IWiFiNetworkReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AvailableNetworks<Impl: IWiFiNetworkReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AvailableNetworks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiNetworkReport, BASE_OFFSET>(),
            Timestamp: Timestamp::<Impl, IMPL_OFFSET>,
            AvailableNetworks: AvailableNetworks::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiNetworkReport as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWiFiWpsConfigurationResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<WiFiWpsConfigurationStatus>;
    fn SupportedWpsKinds(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WiFiWpsKind>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiWpsConfigurationResult {
    const NAME: &'static str = "Windows.Devices.WiFi.IWiFiWpsConfigurationResult";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWiFiWpsConfigurationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiWpsConfigurationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiWpsConfigurationResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IWiFiWpsConfigurationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiWpsConfigurationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedWpsKinds<Impl: IWiFiWpsConfigurationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedWpsKinds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiWpsConfigurationResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            SupportedWpsKinds: SupportedWpsKinds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiWpsConfigurationResult as ::windows::core::Interface>::IID
    }
}
