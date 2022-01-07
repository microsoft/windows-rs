#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceImpl: Sized {
    fn RemoteServiceInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SupportedConfigurationMethods(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<WiFiDirectServiceConfigurationMethod>>;
    fn PreferGroupOwnerMode(&self) -> ::windows::core::Result<bool>;
    fn SetPreferGroupOwnerMode(&self, value: bool) -> ::windows::core::Result<()>;
    fn SessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetSessionInfo(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn ServiceError(&self) -> ::windows::core::Result<WiFiDirectServiceError>;
    fn SessionDeferred(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectService, WiFiDirectServiceSessionDeferredEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionDeferred(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetProvisioningInfoAsync(&self, selectedconfigurationmethod: WiFiDirectServiceConfigurationMethod) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceProvisioningInfo>>;
    fn ConnectAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>;
    fn ConnectAsyncWithPin(&self, pin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectService {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectService";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceImpl, const OFFSET: isize>() -> IWiFiDirectServiceVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWiFiDirectService>,
            ::windows::core::GetTrustLevel,
            RemoteServiceInfo::<Impl, OFFSET>,
            SupportedConfigurationMethods::<Impl, OFFSET>,
            PreferGroupOwnerMode::<Impl, OFFSET>,
            SetPreferGroupOwnerMode::<Impl, OFFSET>,
            SessionInfo::<Impl, OFFSET>,
            SetSessionInfo::<Impl, OFFSET>,
            ServiceError::<Impl, OFFSET>,
            SessionDeferred::<Impl, OFFSET>,
            RemoveSessionDeferred::<Impl, OFFSET>,
            GetProvisioningInfoAsync::<Impl, OFFSET>,
            ConnectAsync::<Impl, OFFSET>,
            ConnectAsyncWithPin::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceAdvertiserImpl: Sized {
    fn ServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceNamePrefixes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ServiceInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetServiceInfo(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn AutoAcceptSession(&self) -> ::windows::core::Result<bool>;
    fn SetAutoAcceptSession(&self, value: bool) -> ::windows::core::Result<()>;
    fn PreferGroupOwnerMode(&self) -> ::windows::core::Result<bool>;
    fn SetPreferGroupOwnerMode(&self, value: bool) -> ::windows::core::Result<()>;
    fn PreferredConfigurationMethods(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<WiFiDirectServiceConfigurationMethod>>;
    fn ServiceStatus(&self) -> ::windows::core::Result<WiFiDirectServiceStatus>;
    fn SetServiceStatus(&self, value: WiFiDirectServiceStatus) -> ::windows::core::Result<()>;
    fn CustomServiceStatusCode(&self) -> ::windows::core::Result<u32>;
    fn SetCustomServiceStatusCode(&self, value: u32) -> ::windows::core::Result<()>;
    fn DeferredSessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetDeferredSessionInfo(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn AdvertisementStatus(&self) -> ::windows::core::Result<WiFiDirectServiceAdvertisementStatus>;
    fn ServiceError(&self) -> ::windows::core::Result<WiFiDirectServiceError>;
    fn SessionRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, WiFiDirectServiceSessionRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AutoAcceptSessionConnected(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, WiFiDirectServiceAutoAcceptSessionConnectedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAutoAcceptSessionConnected(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AdvertisementStatusChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceAdvertiser, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdvertisementStatusChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ConnectAsync(&self, deviceinfo: &::core::option::Option<super::super::Enumeration::DeviceInformation>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>;
    fn ConnectAsyncWithPin(&self, deviceinfo: &::core::option::Option<super::super::Enumeration::DeviceInformation>, pin: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectServiceSession>>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectServiceAdvertiser {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAdvertiser";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectServiceAdvertiserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceAdvertiserImpl, const OFFSET: isize>() -> IWiFiDirectServiceAdvertiserVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWiFiDirectServiceAdvertiser>,
            ::windows::core::GetTrustLevel,
            ServiceName::<Impl, OFFSET>,
            ServiceNamePrefixes::<Impl, OFFSET>,
            ServiceInfo::<Impl, OFFSET>,
            SetServiceInfo::<Impl, OFFSET>,
            AutoAcceptSession::<Impl, OFFSET>,
            SetAutoAcceptSession::<Impl, OFFSET>,
            PreferGroupOwnerMode::<Impl, OFFSET>,
            SetPreferGroupOwnerMode::<Impl, OFFSET>,
            PreferredConfigurationMethods::<Impl, OFFSET>,
            ServiceStatus::<Impl, OFFSET>,
            SetServiceStatus::<Impl, OFFSET>,
            CustomServiceStatusCode::<Impl, OFFSET>,
            SetCustomServiceStatusCode::<Impl, OFFSET>,
            DeferredSessionInfo::<Impl, OFFSET>,
            SetDeferredSessionInfo::<Impl, OFFSET>,
            AdvertisementStatus::<Impl, OFFSET>,
            ServiceError::<Impl, OFFSET>,
            SessionRequested::<Impl, OFFSET>,
            RemoveSessionRequested::<Impl, OFFSET>,
            AutoAcceptSessionConnected::<Impl, OFFSET>,
            RemoveAutoAcceptSessionConnected::<Impl, OFFSET>,
            AdvertisementStatusChanged::<Impl, OFFSET>,
            RemoveAdvertisementStatusChanged::<Impl, OFFSET>,
            ConnectAsync::<Impl, OFFSET>,
            ConnectAsyncWithPin::<Impl, OFFSET>,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceAdvertiserFactoryImpl: Sized {
    fn CreateWiFiDirectServiceAdvertiser(&self, servicename: &::windows::core::HSTRING) -> ::windows::core::Result<WiFiDirectServiceAdvertiser>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectServiceAdvertiserFactory {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAdvertiserFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectServiceAdvertiserFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceAdvertiserFactoryImpl, const OFFSET: isize>() -> IWiFiDirectServiceAdvertiserFactoryVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWiFiDirectServiceAdvertiserFactory>, ::windows::core::GetTrustLevel, CreateWiFiDirectServiceAdvertiser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceAutoAcceptSessionConnectedEventArgsImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<WiFiDirectServiceSession>;
    fn SessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectServiceAutoAcceptSessionConnectedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceAutoAcceptSessionConnectedEventArgsImpl, const OFFSET: isize>() -> IWiFiDirectServiceAutoAcceptSessionConnectedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWiFiDirectServiceAutoAcceptSessionConnectedEventArgs>, ::windows::core::GetTrustLevel, Session::<Impl, OFFSET>, SessionInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceProvisioningInfoImpl: Sized {
    fn SelectedConfigurationMethod(&self) -> ::windows::core::Result<WiFiDirectServiceConfigurationMethod>;
    fn IsGroupFormationNeeded(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectServiceProvisioningInfo {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceProvisioningInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectServiceProvisioningInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceProvisioningInfoImpl, const OFFSET: isize>() -> IWiFiDirectServiceProvisioningInfoVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWiFiDirectServiceProvisioningInfo>, ::windows::core::GetTrustLevel, SelectedConfigurationMethod::<Impl, OFFSET>, IsGroupFormationNeeded::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceRemotePortAddedEventArgsImpl: Sized {
    fn EndpointPairs(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Networking::EndpointPair>>;
    fn Protocol(&self) -> ::windows::core::Result<WiFiDirectServiceIPProtocol>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectServiceRemotePortAddedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceRemotePortAddedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectServiceRemotePortAddedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceRemotePortAddedEventArgsImpl, const OFFSET: isize>() -> IWiFiDirectServiceRemotePortAddedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWiFiDirectServiceRemotePortAddedEventArgs>, ::windows::core::GetTrustLevel, EndpointPairs::<Impl, OFFSET>, Protocol::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWiFiDirectServiceSessionImpl: Sized + IClosableImpl {
    fn ServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Status(&self) -> ::windows::core::Result<WiFiDirectServiceSessionStatus>;
    fn ErrorStatus(&self) -> ::windows::core::Result<WiFiDirectServiceSessionErrorStatus>;
    fn SessionId(&self) -> ::windows::core::Result<u32>;
    fn AdvertisementId(&self) -> ::windows::core::Result<u32>;
    fn ServiceAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SessionAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetConnectionEndpointPairs(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Networking::EndpointPair>>;
    fn SessionStatusChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionStatusChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AddStreamSocketListenerAsync(&self, value: &::core::option::Option<super::super::super::Networking::Sockets::StreamSocketListener>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn AddDatagramSocketAsync(&self, value: &::core::option::Option<super::super::super::Networking::Sockets::DatagramSocket>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn RemotePortAdded(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<WiFiDirectServiceSession, WiFiDirectServiceRemotePortAddedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemotePortAdded(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectServiceSession {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWiFiDirectServiceSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceSessionImpl, const OFFSET: isize>() -> IWiFiDirectServiceSessionVtbl {
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
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWiFiDirectServiceSession>,
            ::windows::core::GetTrustLevel,
            ServiceName::<Impl, OFFSET>,
            Status::<Impl, OFFSET>,
            ErrorStatus::<Impl, OFFSET>,
            SessionId::<Impl, OFFSET>,
            AdvertisementId::<Impl, OFFSET>,
            ServiceAddress::<Impl, OFFSET>,
            SessionAddress::<Impl, OFFSET>,
            GetConnectionEndpointPairs::<Impl, OFFSET>,
            SessionStatusChanged::<Impl, OFFSET>,
            RemoveSessionStatusChanged::<Impl, OFFSET>,
            AddStreamSocketListenerAsync::<Impl, OFFSET>,
            AddDatagramSocketAsync::<Impl, OFFSET>,
            RemotePortAdded::<Impl, OFFSET>,
            RemoveRemotePortAdded::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceSessionDeferredEventArgsImpl: Sized {
    fn DeferredSessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectServiceSessionDeferredEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionDeferredEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectServiceSessionDeferredEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceSessionDeferredEventArgsImpl, const OFFSET: isize>() -> IWiFiDirectServiceSessionDeferredEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWiFiDirectServiceSessionDeferredEventArgs>, ::windows::core::GetTrustLevel, DeferredSessionInfo::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IWiFiDirectServiceSessionRequestImpl: Sized + IClosableImpl {
    fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Enumeration::DeviceInformation>;
    fn ProvisioningInfo(&self) -> ::windows::core::Result<WiFiDirectServiceProvisioningInfo>;
    fn SessionInfo(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWiFiDirectServiceSessionRequest {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionRequest";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IWiFiDirectServiceSessionRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceSessionRequestImpl, const OFFSET: isize>() -> IWiFiDirectServiceSessionRequestVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWiFiDirectServiceSessionRequest>, ::windows::core::GetTrustLevel, DeviceInformation::<Impl, OFFSET>, ProvisioningInfo::<Impl, OFFSET>, SessionInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceSessionRequestedEventArgsImpl: Sized {
    fn GetSessionRequest(&self) -> ::windows::core::Result<WiFiDirectServiceSessionRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectServiceSessionRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceSessionRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectServiceSessionRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceSessionRequestedEventArgsImpl, const OFFSET: isize>() -> IWiFiDirectServiceSessionRequestedEventArgsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWiFiDirectServiceSessionRequestedEventArgs>, ::windows::core::GetTrustLevel, GetSessionRequest::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWiFiDirectServiceStaticsImpl: Sized {
    fn GetSelector(&self, servicename: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetSelectorWithFilter(&self, servicename: &::windows::core::HSTRING, serviceinfofilter: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WiFiDirectService>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWiFiDirectServiceStatics {
    const NAME: &'static str = "Windows.Devices.WiFiDirect.Services.IWiFiDirectServiceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWiFiDirectServiceStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWiFiDirectServiceStaticsImpl, const OFFSET: isize>() -> IWiFiDirectServiceStaticsVtbl {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWiFiDirectServiceStatics>, ::windows::core::GetTrustLevel, GetSelector::<Impl, OFFSET>, GetSelectorWithFilter::<Impl, OFFSET>, FromIdAsync::<Impl, OFFSET>)
    }
}
