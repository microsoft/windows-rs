#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IWiFiDirectServiceImpl: Sized {
    fn RemoteServiceInfo(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SupportedConfigurationMethods(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<WiFiDirectServiceConfigurationMethod>>;
    fn PreferGroupOwnerMode(&mut self) -> ::windows::core::Result<bool>;
    fn SetPreferGroupOwnerMode(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SessionInfo(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetSessionInfo(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn ServiceError(&mut self) -> ::windows::core::Result<WiFiDirectServiceError>;
    fn SessionDeferred(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectService, WiFiDirectServiceSessionDeferredEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionDeferred(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetProvisioningInfoAsync(&mut self, selectedconfigurationmethod: WiFiDirectServiceConfigurationMethod) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceProvisioningInfo>>;
    fn ConnectAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>;
    fn ConnectAsyncWithPin(&mut self, pin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectService {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectService";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWiFiDirectServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectServiceVtbl {
        unsafe extern "system" fn RemoteServiceInfo<Impl: IWiFiDirectServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteServiceInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedConfigurationMethods<Impl: IWiFiDirectServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedConfigurationMethods() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreferGroupOwnerMode<Impl: IWiFiDirectServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferGroupOwnerMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferGroupOwnerMode<Impl: IWiFiDirectServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferGroupOwnerMode(value).into()
        }
        unsafe extern "system" fn SessionInfo<Impl: IWiFiDirectServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionInfo<Impl: IWiFiDirectServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSessionInfo(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServiceError<Impl: IWiFiDirectServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectServiceError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionDeferred<Impl: IWiFiDirectServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionDeferred(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<WiFiDirectService, WiFiDirectServiceSessionDeferredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<WiFiDirectService, WiFiDirectServiceSessionDeferredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSessionDeferred<Impl: IWiFiDirectServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSessionDeferred(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetProvisioningInfoAsync<Impl: IWiFiDirectServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, selectedconfigurationmethod: WiFiDirectServiceConfigurationMethod, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProvisioningInfoAsync(selectedconfigurationmethod) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectAsync<Impl: IWiFiDirectServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectAsyncWithPin<Impl: IWiFiDirectServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectAsyncWithPin(&*(&pin as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectService, BASE_OFFSET>(),
            RemoteServiceInfo: RemoteServiceInfo::<Impl, IMPL_OFFSET>,
            SupportedConfigurationMethods: SupportedConfigurationMethods::<Impl, IMPL_OFFSET>,
            PreferGroupOwnerMode: PreferGroupOwnerMode::<Impl, IMPL_OFFSET>,
            SetPreferGroupOwnerMode: SetPreferGroupOwnerMode::<Impl, IMPL_OFFSET>,
            SessionInfo: SessionInfo::<Impl, IMPL_OFFSET>,
            SetSessionInfo: SetSessionInfo::<Impl, IMPL_OFFSET>,
            ServiceError: ServiceError::<Impl, IMPL_OFFSET>,
            SessionDeferred: SessionDeferred::<Impl, IMPL_OFFSET>,
            RemoveSessionDeferred: RemoveSessionDeferred::<Impl, IMPL_OFFSET>,
            GetProvisioningInfoAsync: GetProvisioningInfoAsync::<Impl, IMPL_OFFSET>,
            ConnectAsync: ConnectAsync::<Impl, IMPL_OFFSET>,
            ConnectAsyncWithPin: ConnectAsyncWithPin::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IWiFiDirectServiceAdvertiserImpl: Sized {
    fn ServiceName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceNamePrefixes(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ServiceInfo(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetServiceInfo(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn AutoAcceptSession(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutoAcceptSession(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PreferGroupOwnerMode(&mut self) -> ::windows::core::Result<bool>;
    fn SetPreferGroupOwnerMode(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PreferredConfigurationMethods(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<WiFiDirectServiceConfigurationMethod>>;
    fn ServiceStatus(&mut self) -> ::windows::core::Result<WiFiDirectServiceStatus>;
    fn SetServiceStatus(&mut self, value: WiFiDirectServiceStatus) -> ::windows::core::Result<()>;
    fn CustomServiceStatusCode(&mut self) -> ::windows::core::Result<u32>;
    fn SetCustomServiceStatusCode(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn DeferredSessionInfo(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetDeferredSessionInfo(&mut self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn AdvertisementStatus(&mut self) -> ::windows::core::Result<WiFiDirectServiceAdvertisementStatus>;
    fn ServiceError(&mut self) -> ::windows::core::Result<WiFiDirectServiceError>;
    fn SessionRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, WiFiDirectServiceSessionRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AutoAcceptSessionConnected(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, WiFiDirectServiceAutoAcceptSessionConnectedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAutoAcceptSessionConnected(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AdvertisementStatusChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdvertisementStatusChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectAsync(&mut self, deviceinfo: &::core::option::Option<super::super::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>;
    fn ConnectAsyncWithPin(&mut self, deviceinfo: &::core::option::Option<super::super::Enumeration::DeviceInformation>, pin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectServiceAdvertiser {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAdvertiser";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWiFiDirectServiceAdvertiserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceAdvertiserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectServiceAdvertiserVtbl {
        unsafe extern "system" fn ServiceName<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceNamePrefixes<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceNamePrefixes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceInfo<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceInfo<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceInfo(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AutoAcceptSession<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoAcceptSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoAcceptSession<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoAcceptSession(value).into()
        }
        unsafe extern "system" fn PreferGroupOwnerMode<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferGroupOwnerMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferGroupOwnerMode<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferGroupOwnerMode(value).into()
        }
        unsafe extern "system" fn PreferredConfigurationMethods<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreferredConfigurationMethods() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceStatus<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectServiceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceStatus<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: WiFiDirectServiceStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceStatus(value).into()
        }
        unsafe extern "system" fn CustomServiceStatusCode<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomServiceStatusCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomServiceStatusCode<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomServiceStatusCode(value).into()
        }
        unsafe extern "system" fn DeferredSessionInfo<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeferredSessionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeferredSessionInfo<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeferredSessionInfo(&*(&value as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AdvertisementStatus<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectServiceAdvertisementStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvertisementStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceError<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectServiceError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionRequested<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, WiFiDirectServiceSessionRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, WiFiDirectServiceSessionRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSessionRequested<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSessionRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AutoAcceptSessionConnected<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoAcceptSessionConnected(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, WiFiDirectServiceAutoAcceptSessionConnectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, WiFiDirectServiceAutoAcceptSessionConnectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAutoAcceptSessionConnected<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAutoAcceptSessionConnected(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AdvertisementStatusChanged<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvertisementStatusChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdvertisementStatusChanged<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdvertisementStatusChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConnectAsync<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectAsync(&*(&deviceinfo as *const <super::super::Enumeration::DeviceInformation as ::windows::core::Abi>::Abi as *const <super::super::Enumeration::DeviceInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectAsyncWithPin<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceinfo: ::windows::core::RawPtr, pin: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectAsyncWithPin(&*(&deviceinfo as *const <super::super::Enumeration::DeviceInformation as ::windows::core::Abi>::Abi as *const <super::super::Enumeration::DeviceInformation as ::windows::core::DefaultType>::DefaultType), &*(&pin as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectServiceAdvertiser, BASE_OFFSET>(),
            ServiceName: ServiceName::<Impl, IMPL_OFFSET>,
            ServiceNamePrefixes: ServiceNamePrefixes::<Impl, IMPL_OFFSET>,
            ServiceInfo: ServiceInfo::<Impl, IMPL_OFFSET>,
            SetServiceInfo: SetServiceInfo::<Impl, IMPL_OFFSET>,
            AutoAcceptSession: AutoAcceptSession::<Impl, IMPL_OFFSET>,
            SetAutoAcceptSession: SetAutoAcceptSession::<Impl, IMPL_OFFSET>,
            PreferGroupOwnerMode: PreferGroupOwnerMode::<Impl, IMPL_OFFSET>,
            SetPreferGroupOwnerMode: SetPreferGroupOwnerMode::<Impl, IMPL_OFFSET>,
            PreferredConfigurationMethods: PreferredConfigurationMethods::<Impl, IMPL_OFFSET>,
            ServiceStatus: ServiceStatus::<Impl, IMPL_OFFSET>,
            SetServiceStatus: SetServiceStatus::<Impl, IMPL_OFFSET>,
            CustomServiceStatusCode: CustomServiceStatusCode::<Impl, IMPL_OFFSET>,
            SetCustomServiceStatusCode: SetCustomServiceStatusCode::<Impl, IMPL_OFFSET>,
            DeferredSessionInfo: DeferredSessionInfo::<Impl, IMPL_OFFSET>,
            SetDeferredSessionInfo: SetDeferredSessionInfo::<Impl, IMPL_OFFSET>,
            AdvertisementStatus: AdvertisementStatus::<Impl, IMPL_OFFSET>,
            ServiceError: ServiceError::<Impl, IMPL_OFFSET>,
            SessionRequested: SessionRequested::<Impl, IMPL_OFFSET>,
            RemoveSessionRequested: RemoveSessionRequested::<Impl, IMPL_OFFSET>,
            AutoAcceptSessionConnected: AutoAcceptSessionConnected::<Impl, IMPL_OFFSET>,
            RemoveAutoAcceptSessionConnected: RemoveAutoAcceptSessionConnected::<Impl, IMPL_OFFSET>,
            AdvertisementStatusChanged: AdvertisementStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveAdvertisementStatusChanged: RemoveAdvertisementStatusChanged::<Impl, IMPL_OFFSET>,
            ConnectAsync: ConnectAsync::<Impl, IMPL_OFFSET>,
            ConnectAsyncWithPin: ConnectAsyncWithPin::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectServiceAdvertiser as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceAdvertiserFactoryImpl: Sized {
    fn CreateWiFiDirectServiceAdvertiser(&mut self, servicename: &::windows::core::HSTRING) -> ::windows::core::Result<WiFiDirectServiceAdvertiser>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectServiceAdvertiserFactory {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAdvertiserFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectServiceAdvertiserFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceAdvertiserFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectServiceAdvertiserFactoryVtbl {
        unsafe extern "system" fn CreateWiFiDirectServiceAdvertiser<Impl: IWiFiDirectServiceAdvertiserFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWiFiDirectServiceAdvertiser(&*(&servicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectServiceAdvertiserFactory, BASE_OFFSET>(),
            CreateWiFiDirectServiceAdvertiser: CreateWiFiDirectServiceAdvertiser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectServiceAdvertiserFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IWiFiDirectServiceAutoAcceptSessionConnectedEventArgsImpl: Sized {
    fn Session(&mut self) -> ::windows::core::Result<WiFiDirectServiceSession>;
    fn SessionInfo(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWiFiDirectServiceAutoAcceptSessionConnectedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceAutoAcceptSessionConnectedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectServiceAutoAcceptSessionConnectedEventArgsVtbl {
        unsafe extern "system" fn Session<Impl: IWiFiDirectServiceAutoAcceptSessionConnectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionInfo<Impl: IWiFiDirectServiceAutoAcceptSessionConnectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs, BASE_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            SessionInfo: SessionInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceProvisioningInfoImpl: Sized {
    fn SelectedConfigurationMethod(&mut self) -> ::windows::core::Result<WiFiDirectServiceConfigurationMethod>;
    fn IsGroupFormationNeeded(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectServiceProvisioningInfo {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceProvisioningInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectServiceProvisioningInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceProvisioningInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectServiceProvisioningInfoVtbl {
        unsafe extern "system" fn SelectedConfigurationMethod<Impl: IWiFiDirectServiceProvisioningInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectServiceConfigurationMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedConfigurationMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsGroupFormationNeeded<Impl: IWiFiDirectServiceProvisioningInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGroupFormationNeeded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectServiceProvisioningInfo, BASE_OFFSET>(),
            SelectedConfigurationMethod: SelectedConfigurationMethod::<Impl, IMPL_OFFSET>,
            IsGroupFormationNeeded: IsGroupFormationNeeded::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectServiceProvisioningInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking", feature = "implement_exclusive"))]
pub trait IWiFiDirectServiceRemotePortAddedEventArgsImpl: Sized {
    fn EndpointPairs(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Networking::EndpointPair>>;
    fn Protocol(&mut self) -> ::windows::core::Result<WiFiDirectServiceIPProtocol>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectServiceRemotePortAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceRemotePortAddedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking", feature = "implement_exclusive"))]
impl IWiFiDirectServiceRemotePortAddedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceRemotePortAddedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectServiceRemotePortAddedEventArgsVtbl {
        unsafe extern "system" fn EndpointPairs<Impl: IWiFiDirectServiceRemotePortAddedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndpointPairs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Impl: IWiFiDirectServiceRemotePortAddedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectServiceIPProtocol) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectServiceRemotePortAddedEventArgs, BASE_OFFSET>(),
            EndpointPairs: EndpointPairs::<Impl, IMPL_OFFSET>,
            Protocol: Protocol::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectServiceRemotePortAddedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Networking_Sockets", feature = "implement_exclusive"))]
pub trait IWiFiDirectServiceSessionImpl: Sized + IClosableImpl {
    fn ServiceName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Status(&mut self) -> ::windows::core::Result<WiFiDirectServiceSessionStatus>;
    fn ErrorStatus(&mut self) -> ::windows::core::Result<WiFiDirectServiceSessionErrorStatus>;
    fn SessionId(&mut self) -> ::windows::core::Result<u32>;
    fn AdvertisementId(&mut self) -> ::windows::core::Result<u32>;
    fn ServiceAddress(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SessionAddress(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetConnectionEndpointPairs(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Networking::EndpointPair>>;
    fn SessionStatusChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionStatusChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AddStreamSocketListenerAsync(&mut self, value: &::core::option::Option<super::super::super::Networking::Sockets::StreamSocketListener>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn AddDatagramSocketAsync(&mut self, value: &::core::option::Option<super::super::super::Networking::Sockets::DatagramSocket>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn RemotePortAdded(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceSession, WiFiDirectServiceRemotePortAddedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemotePortAdded(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectServiceSession {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSession";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl IWiFiDirectServiceSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectServiceSessionVtbl {
        unsafe extern "system" fn ServiceName<Impl: IWiFiDirectServiceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IWiFiDirectServiceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectServiceSessionStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ErrorStatus<Impl: IWiFiDirectServiceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WiFiDirectServiceSessionErrorStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionId<Impl: IWiFiDirectServiceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdvertisementId<Impl: IWiFiDirectServiceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvertisementId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceAddress<Impl: IWiFiDirectServiceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionAddress<Impl: IWiFiDirectServiceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionEndpointPairs<Impl: IWiFiDirectServiceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionEndpointPairs() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionStatusChanged<Impl: IWiFiDirectServiceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionStatusChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSessionStatusChanged<Impl: IWiFiDirectServiceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSessionStatusChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddStreamSocketListenerAsync<Impl: IWiFiDirectServiceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddStreamSocketListenerAsync(&*(&value as *const <super::super::super::Networking::Sockets::StreamSocketListener as ::windows::core::Abi>::Abi as *const <super::super::super::Networking::Sockets::StreamSocketListener as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDatagramSocketAsync<Impl: IWiFiDirectServiceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddDatagramSocketAsync(&*(&value as *const <super::super::super::Networking::Sockets::DatagramSocket as ::windows::core::Abi>::Abi as *const <super::super::super::Networking::Sockets::DatagramSocket as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemotePortAdded<Impl: IWiFiDirectServiceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemotePortAdded(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceSession, WiFiDirectServiceRemotePortAddedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceSession, WiFiDirectServiceRemotePortAddedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemotePortAdded<Impl: IWiFiDirectServiceSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemotePortAdded(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectServiceSession, BASE_OFFSET>(),
            ServiceName: ServiceName::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            ErrorStatus: ErrorStatus::<Impl, IMPL_OFFSET>,
            SessionId: SessionId::<Impl, IMPL_OFFSET>,
            AdvertisementId: AdvertisementId::<Impl, IMPL_OFFSET>,
            ServiceAddress: ServiceAddress::<Impl, IMPL_OFFSET>,
            SessionAddress: SessionAddress::<Impl, IMPL_OFFSET>,
            GetConnectionEndpointPairs: GetConnectionEndpointPairs::<Impl, IMPL_OFFSET>,
            SessionStatusChanged: SessionStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveSessionStatusChanged: RemoveSessionStatusChanged::<Impl, IMPL_OFFSET>,
            AddStreamSocketListenerAsync: AddStreamSocketListenerAsync::<Impl, IMPL_OFFSET>,
            AddDatagramSocketAsync: AddDatagramSocketAsync::<Impl, IMPL_OFFSET>,
            RemotePortAdded: RemotePortAdded::<Impl, IMPL_OFFSET>,
            RemoveRemotePortAdded: RemoveRemotePortAdded::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectServiceSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IWiFiDirectServiceSessionDeferredEventArgsImpl: Sized {
    fn DeferredSessionInfo(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectServiceSessionDeferredEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionDeferredEventArgs";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWiFiDirectServiceSessionDeferredEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceSessionDeferredEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectServiceSessionDeferredEventArgsVtbl {
        unsafe extern "system" fn DeferredSessionInfo<Impl: IWiFiDirectServiceSessionDeferredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeferredSessionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectServiceSessionDeferredEventArgs, BASE_OFFSET>(),
            DeferredSessionInfo: DeferredSessionInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectServiceSessionDeferredEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IWiFiDirectServiceSessionRequestImpl: Sized + IClosableImpl {
    fn DeviceInformation(&mut self) -> ::windows::core::Result<super::super::Enumeration::DeviceInformation>;
    fn ProvisioningInfo(&mut self) -> ::windows::core::Result<WiFiDirectServiceProvisioningInfo>;
    fn SessionInfo(&mut self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectServiceSessionRequest {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionRequest";
}
#[cfg(all(feature = "Devices_Enumeration", feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWiFiDirectServiceSessionRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceSessionRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectServiceSessionRequestVtbl {
        unsafe extern "system" fn DeviceInformation<Impl: IWiFiDirectServiceSessionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProvisioningInfo<Impl: IWiFiDirectServiceSessionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProvisioningInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionInfo<Impl: IWiFiDirectServiceSessionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectServiceSessionRequest, BASE_OFFSET>(),
            DeviceInformation: DeviceInformation::<Impl, IMPL_OFFSET>,
            ProvisioningInfo: ProvisioningInfo::<Impl, IMPL_OFFSET>,
            SessionInfo: SessionInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectServiceSessionRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceSessionRequestedEventArgsImpl: Sized {
    fn GetSessionRequest(&mut self) -> ::windows::core::Result<WiFiDirectServiceSessionRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectServiceSessionRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectServiceSessionRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceSessionRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectServiceSessionRequestedEventArgsVtbl {
        unsafe extern "system" fn GetSessionRequest<Impl: IWiFiDirectServiceSessionRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectServiceSessionRequestedEventArgs, BASE_OFFSET>(),
            GetSessionRequest: GetSessionRequest::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectServiceSessionRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IWiFiDirectServiceStaticsImpl: Sized {
    fn GetSelector(&mut self, servicename: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetSelectorWithFilter(&mut self, servicename: &::windows::core::HSTRING, serviceinfofilter: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectService>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectServiceStatics {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IWiFiDirectServiceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWiFiDirectServiceStaticsVtbl {
        unsafe extern "system" fn GetSelector<Impl: IWiFiDirectServiceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelector(&*(&servicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectorWithFilter<Impl: IWiFiDirectServiceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, servicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, serviceinfofilter: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelectorWithFilter(&*(&servicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&serviceinfofilter as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromIdAsync<Impl: IWiFiDirectServiceStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWiFiDirectServiceStatics, BASE_OFFSET>(),
            GetSelector: GetSelector::<Impl, IMPL_OFFSET>,
            GetSelectorWithFilter: GetSelectorWithFilter::<Impl, IMPL_OFFSET>,
            FromIdAsync: FromIdAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWiFiDirectServiceStatics as ::windows::core::Interface>::IID
    }
}
