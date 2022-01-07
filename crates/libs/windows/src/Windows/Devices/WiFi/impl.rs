#[cfg(feature = "implement_exclusive")]
pub trait IWiFiAdapterImpl: Sized {
    fn NetworkAdapter(&self) -> ::windows::core::Result<super::super::Networking::Connectivity::NetworkAdapter>;
    fn ScanAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn NetworkReport(&self) -> ::windows::core::Result<WiFiNetworkReport>;
    fn AvailableNetworksChanged(&self, args: &::core::option::Option<super::super::Foundation::TypedEventHandler<WiFiAdapter, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailableNetworksChanged(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectAsync(&self, availablenetwork: &::core::option::Option<WiFiAvailableNetwork>, reconnectionkind: WiFiReconnectionKind) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>;
    fn ConnectWithPasswordCredentialAsync(&self, availablenetwork: &::core::option::Option<WiFiAvailableNetwork>, reconnectionkind: WiFiReconnectionKind, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>;
    fn ConnectWithPasswordCredentialAndSsidAsync(&self, availablenetwork: &::core::option::Option<WiFiAvailableNetwork>, reconnectionkind: WiFiReconnectionKind, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>, ssid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>;
    fn Disconnect(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiAdapter {
    const NAME: &'static str = "Windows.Devices.WiFi.IWiFiAdapter";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiAdapterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiAdapterImpl, const OFFSET: isize>() -> IWiFiAdapterVtbl {
        unsafe extern "system" fn NetworkAdapter<Impl: IWiFiAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ScanAsync<Impl: IWiFiAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NetworkReport<Impl: IWiFiAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AvailableNetworksChanged<Impl: IWiFiAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAvailableNetworksChanged<Impl: IWiFiAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAvailableNetworksChanged(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConnectAsync<Impl: IWiFiAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availablenetwork: ::windows::core::RawPtr, reconnectionkind: WiFiReconnectionKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectWithPasswordCredentialAsync<Impl: IWiFiAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availablenetwork: ::windows::core::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectWithPasswordCredentialAndSsidAsync<Impl: IWiFiAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availablenetwork: ::windows::core::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows::core::RawPtr, ssid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Disconnect<Impl: IWiFiAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWiFiAdapter>,
            ::windows::core::GetTrustLevel,
            NetworkAdapter::<Impl, OFFSET>,
            ScanAsync::<Impl, OFFSET>,
            NetworkReport::<Impl, OFFSET>,
            AvailableNetworksChanged::<Impl, OFFSET>,
            RemoveAvailableNetworksChanged::<Impl, OFFSET>,
            ConnectAsync::<Impl, OFFSET>,
            ConnectWithPasswordCredentialAsync::<Impl, OFFSET>,
            ConnectWithPasswordCredentialAndSsidAsync::<Impl, OFFSET>,
            Disconnect::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiAdapter2Impl: Sized {
    fn GetWpsConfigurationAsync(&self, availablenetwork: &::core::option::Option<WiFiAvailableNetwork>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiWpsConfigurationResult>>;
    fn ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync(&self, availablenetwork: &::core::option::Option<WiFiAvailableNetwork>, reconnectionkind: WiFiReconnectionKind, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>, ssid: &::windows::core::HSTRING, connectionmethod: WiFiConnectionMethod) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiConnectionResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiAdapter2 {
    const NAME: &'static str = "Windows.Devices.WiFi.IWiFiAdapter2";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiAdapter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiAdapter2Impl, const OFFSET: isize>() -> IWiFiAdapter2Vtbl {
        unsafe extern "system" fn GetWpsConfigurationAsync<Impl: IWiFiAdapter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availablenetwork: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync<Impl: IWiFiAdapter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availablenetwork: ::windows::core::RawPtr, reconnectionkind: WiFiReconnectionKind, passwordcredential: ::windows::core::RawPtr, ssid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, connectionmethod: WiFiConnectionMethod, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWiFiAdapter2>, ::windows::core::GetTrustLevel, GetWpsConfigurationAsync::<Impl, OFFSET>, ConnectWithPasswordCredentialAndSsidAndConnectionMethodAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiAdapterStaticsImpl: Sized {
    fn FindAllAdaptersAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<WiFiAdapter>>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiAdapter>>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WiFiAccessStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiAdapterStatics {
    const NAME: &'static str = "Windows.Devices.WiFi.IWiFiAdapterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiAdapterStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiAdapterStaticsImpl, const OFFSET: isize>() -> IWiFiAdapterStaticsVtbl {
        unsafe extern "system" fn FindAllAdaptersAsync<Impl: IWiFiAdapterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceSelector<Impl: IWiFiAdapterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromIdAsync<Impl: IWiFiAdapterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestAccessAsync<Impl: IWiFiAdapterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWiFiAdapterStatics>, ::windows::core::GetTrustLevel, FindAllAdaptersAsync::<Impl, OFFSET>, GetDeviceSelector::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>, RequestAccessAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiAvailableNetworkImpl: Sized {
    fn Uptime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Ssid(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Bssid(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ChannelCenterFrequencyInKilohertz(&self) -> ::windows::core::Result<i32>;
    fn NetworkRssiInDecibelMilliwatts(&self) -> ::windows::core::Result<f64>;
    fn SignalBars(&self) -> ::windows::core::Result<u8>;
    fn NetworkKind(&self) -> ::windows::core::Result<WiFiNetworkKind>;
    fn PhyKind(&self) -> ::windows::core::Result<WiFiPhyKind>;
    fn SecuritySettings(&self) -> ::windows::core::Result<super::super::Networking::Connectivity::NetworkSecuritySettings>;
    fn BeaconInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn IsWiFiDirect(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiAvailableNetwork {
    const NAME: &'static str = "Windows.Devices.WiFi.IWiFiAvailableNetwork";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiAvailableNetworkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiAvailableNetworkImpl, const OFFSET: isize>() -> IWiFiAvailableNetworkVtbl {
        unsafe extern "system" fn Uptime<Impl: IWiFiAvailableNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Ssid<Impl: IWiFiAvailableNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Bssid<Impl: IWiFiAvailableNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChannelCenterFrequencyInKilohertz<Impl: IWiFiAvailableNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NetworkRssiInDecibelMilliwatts<Impl: IWiFiAvailableNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SignalBars<Impl: IWiFiAvailableNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NetworkKind<Impl: IWiFiAvailableNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiNetworkKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PhyKind<Impl: IWiFiAvailableNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiPhyKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SecuritySettings<Impl: IWiFiAvailableNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BeaconInterval<Impl: IWiFiAvailableNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsWiFiDirect<Impl: IWiFiAvailableNetworkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWiFiAvailableNetwork>,
            ::windows::core::GetTrustLevel,
            Uptime::<Impl, OFFSET>,
            Ssid::<Impl, OFFSET>,
            Bssid::<Impl, OFFSET>,
            ChannelCenterFrequencyInKilohertz::<Impl, OFFSET>,
            NetworkRssiInDecibelMilliwatts::<Impl, OFFSET>,
            SignalBars::<Impl, OFFSET>,
            NetworkKind::<Impl, OFFSET>,
            PhyKind::<Impl, OFFSET>,
            SecuritySettings::<Impl, OFFSET>,
            BeaconInterval::<Impl, OFFSET>,
            IsWiFiDirect::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiConnectionResultImpl: Sized {
    fn ConnectionStatus(&self) -> ::windows::core::Result<WiFiConnectionStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiConnectionResult {
    const NAME: &'static str = "Windows.Devices.WiFi.IWiFiConnectionResult";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiConnectionResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiConnectionResultImpl, const OFFSET: isize>() -> IWiFiConnectionResultVtbl {
        unsafe extern "system" fn ConnectionStatus<Impl: IWiFiConnectionResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiConnectionStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWiFiConnectionResult>, ::windows::core::GetTrustLevel, ConnectionStatus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiNetworkReportImpl: Sized {
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn AvailableNetworks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WiFiAvailableNetwork>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiNetworkReport {
    const NAME: &'static str = "Windows.Devices.WiFi.IWiFiNetworkReport";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiNetworkReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiNetworkReportImpl, const OFFSET: isize>() -> IWiFiNetworkReportVtbl {
        unsafe extern "system" fn Timestamp<Impl: IWiFiNetworkReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AvailableNetworks<Impl: IWiFiNetworkReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWiFiNetworkReport>, ::windows::core::GetTrustLevel, Timestamp::<Impl, OFFSET>, AvailableNetworks::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiWpsConfigurationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<WiFiWpsConfigurationStatus>;
    fn SupportedWpsKinds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<WiFiWpsKind>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiWpsConfigurationResult {
    const NAME: &'static str = "Windows.Devices.WiFi.IWiFiWpsConfigurationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiWpsConfigurationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiWpsConfigurationResultImpl, const OFFSET: isize>() -> IWiFiWpsConfigurationResultVtbl {
        unsafe extern "system" fn Status<Impl: IWiFiWpsConfigurationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiWpsConfigurationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SupportedWpsKinds<Impl: IWiFiWpsConfigurationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWiFiWpsConfigurationResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, SupportedWpsKinds::<Impl, OFFSET>)
    }
}
