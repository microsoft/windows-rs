#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorAuthentication_Impl: Sized {
    fn ServiceAuthenticationHmac(&mut self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
    fn SessionNonce(&mut self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
    fn DeviceNonce(&mut self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
    fn DeviceConfigurationData(&mut self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
    fn FinishAuthenticationAsync(&mut self, devicehmac: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>, sessionhmac: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorFinishAuthenticationStatus>>;
    fn AbortAuthenticationAsync(&mut self, errorlogmessage: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorAuthentication {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthentication";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorAuthentication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorAuthentication_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorAuthentication_Vtbl {
        unsafe extern "system" fn ServiceAuthenticationHmac<Impl: ISecondaryAuthenticationFactorAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceAuthenticationHmac() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionNonce<Impl: ISecondaryAuthenticationFactorAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionNonce() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceNonce<Impl: ISecondaryAuthenticationFactorAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceNonce() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceConfigurationData<Impl: ISecondaryAuthenticationFactorAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceConfigurationData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinishAuthenticationAsync<Impl: ISecondaryAuthenticationFactorAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicehmac: ::windows::core::RawPtr, sessionhmac: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinishAuthenticationAsync(&*(&devicehmac as *const <super::super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType), &*(&sessionhmac as *const <super::super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortAuthenticationAsync<Impl: ISecondaryAuthenticationFactorAuthentication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorlogmessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbortAuthenticationAsync(&*(&errorlogmessage as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryAuthenticationFactorAuthentication, BASE_OFFSET>(),
            ServiceAuthenticationHmac: ServiceAuthenticationHmac::<Impl, IMPL_OFFSET>,
            SessionNonce: SessionNonce::<Impl, IMPL_OFFSET>,
            DeviceNonce: DeviceNonce::<Impl, IMPL_OFFSET>,
            DeviceConfigurationData: DeviceConfigurationData::<Impl, IMPL_OFFSET>,
            FinishAuthenticationAsync: FinishAuthenticationAsync::<Impl, IMPL_OFFSET>,
            AbortAuthenticationAsync: AbortAuthenticationAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryAuthenticationFactorAuthentication as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorAuthenticationResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationStatus>;
    fn Authentication(&mut self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthentication>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorAuthenticationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationResult";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorAuthenticationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorAuthenticationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorAuthenticationResult_Vtbl {
        unsafe extern "system" fn Status<Impl: ISecondaryAuthenticationFactorAuthenticationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorAuthenticationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Authentication<Impl: ISecondaryAuthenticationFactorAuthenticationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authentication() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryAuthenticationFactorAuthenticationResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Authentication: Authentication::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryAuthenticationFactorAuthenticationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_Impl: Sized {
    fn StageInfo(&mut self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationStageInfo>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_Vtbl {
        unsafe extern "system" fn StageInfo<Impl: ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StageInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs, BASE_OFFSET>(),
            StageInfo: StageInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorAuthenticationStageInfo_Impl: Sized {
    fn Stage(&mut self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationStage>;
    fn Scenario(&mut self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationScenario>;
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorAuthenticationStageInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStageInfo";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorAuthenticationStageInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorAuthenticationStageInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorAuthenticationStageInfo_Vtbl {
        unsafe extern "system" fn Stage<Impl: ISecondaryAuthenticationFactorAuthenticationStageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorAuthenticationStage) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scenario<Impl: ISecondaryAuthenticationFactorAuthenticationStageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorAuthenticationScenario) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scenario() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceId<Impl: ISecondaryAuthenticationFactorAuthenticationStageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryAuthenticationFactorAuthenticationStageInfo, BASE_OFFSET>(),
            Stage: Stage::<Impl, IMPL_OFFSET>,
            Scenario: Scenario::<Impl, IMPL_OFFSET>,
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryAuthenticationFactorAuthenticationStageInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorAuthenticationStatics_Impl: Sized {
    fn ShowNotificationMessageAsync(&mut self, devicename: &::windows::core::HSTRING, message: SecondaryAuthenticationFactorAuthenticationMessage) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn StartAuthenticationAsync(&mut self, deviceid: &::windows::core::HSTRING, serviceauthenticationnonce: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationResult>>;
    fn AuthenticationStageChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAuthenticationStageChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetAuthenticationStageInfoAsync(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationStageInfo>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorAuthenticationStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorAuthenticationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorAuthenticationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorAuthenticationStatics_Vtbl {
        unsafe extern "system" fn ShowNotificationMessageAsync<Impl: ISecondaryAuthenticationFactorAuthenticationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: SecondaryAuthenticationFactorAuthenticationMessage, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowNotificationMessageAsync(&*(&devicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), message) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAuthenticationAsync<Impl: ISecondaryAuthenticationFactorAuthenticationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, serviceauthenticationnonce: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAuthenticationAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&serviceauthenticationnonce as *const <super::super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthenticationStageChanged<Impl: ISecondaryAuthenticationFactorAuthenticationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationStageChanged(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAuthenticationStageChanged<Impl: ISecondaryAuthenticationFactorAuthenticationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAuthenticationStageChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetAuthenticationStageInfoAsync<Impl: ISecondaryAuthenticationFactorAuthenticationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAuthenticationStageInfoAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryAuthenticationFactorAuthenticationStatics, BASE_OFFSET>(),
            ShowNotificationMessageAsync: ShowNotificationMessageAsync::<Impl, IMPL_OFFSET>,
            StartAuthenticationAsync: StartAuthenticationAsync::<Impl, IMPL_OFFSET>,
            AuthenticationStageChanged: AuthenticationStageChanged::<Impl, IMPL_OFFSET>,
            RemoveAuthenticationStageChanged: RemoveAuthenticationStageChanged::<Impl, IMPL_OFFSET>,
            GetAuthenticationStageInfoAsync: GetAuthenticationStageInfoAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryAuthenticationFactorAuthenticationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_Impl: Sized {
    fn RegisterDevicePresenceMonitoringAsync(&mut self, deviceid: &::windows::core::HSTRING, deviceinstancepath: &::windows::core::HSTRING, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>>;
    fn RegisterDevicePresenceMonitoringWithNewDeviceAsync(&mut self, deviceid: &::windows::core::HSTRING, deviceinstancepath: &::windows::core::HSTRING, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, devicefriendlyname: &::windows::core::HSTRING, devicemodelnumber: &::windows::core::HSTRING, deviceconfigurationdata: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>>;
    fn UnregisterDevicePresenceMonitoringAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn IsDevicePresenceMonitoringSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_Vtbl {
        unsafe extern "system" fn RegisterDevicePresenceMonitoringAsync<Impl: ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deviceinstancepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterDevicePresenceMonitoringAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&deviceinstancepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), monitoringmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterDevicePresenceMonitoringWithNewDeviceAsync<Impl: ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deviceinstancepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, devicefriendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, devicemodelnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deviceconfigurationdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterDevicePresenceMonitoringWithNewDeviceAsync(
                &*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&deviceinstancepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                monitoringmode,
                &*(&devicefriendlyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&devicemodelnumber as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&deviceconfigurationdata as *const <super::super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDevicePresenceMonitoringAsync<Impl: ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterDevicePresenceMonitoringAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDevicePresenceMonitoringSupported<Impl: ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDevicePresenceMonitoringSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics, BASE_OFFSET>(),
            RegisterDevicePresenceMonitoringAsync: RegisterDevicePresenceMonitoringAsync::<Impl, IMPL_OFFSET>,
            RegisterDevicePresenceMonitoringWithNewDeviceAsync: RegisterDevicePresenceMonitoringWithNewDeviceAsync::<Impl, IMPL_OFFSET>,
            UnregisterDevicePresenceMonitoringAsync: UnregisterDevicePresenceMonitoringAsync::<Impl, IMPL_OFFSET>,
            IsDevicePresenceMonitoringSupported: IsDevicePresenceMonitoringSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorInfo_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceFriendlyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceModelNumber(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceConfigurationData(&mut self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo";
}
#[cfg(all(feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorInfo_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: ISecondaryAuthenticationFactorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceFriendlyName<Impl: ISecondaryAuthenticationFactorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceFriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceModelNumber<Impl: ISecondaryAuthenticationFactorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceModelNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceConfigurationData<Impl: ISecondaryAuthenticationFactorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceConfigurationData() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryAuthenticationFactorInfo, BASE_OFFSET>(),
            DeviceId: DeviceId::<Impl, IMPL_OFFSET>,
            DeviceFriendlyName: DeviceFriendlyName::<Impl, IMPL_OFFSET>,
            DeviceModelNumber: DeviceModelNumber::<Impl, IMPL_OFFSET>,
            DeviceConfigurationData: DeviceConfigurationData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryAuthenticationFactorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorInfo2_Impl: Sized + ISecondaryAuthenticationFactorInfo_Impl {
    fn PresenceMonitoringMode(&mut self) -> ::windows::core::Result<SecondaryAuthenticationFactorDevicePresenceMonitoringMode>;
    fn UpdateDevicePresenceAsync(&mut self, presencestate: SecondaryAuthenticationFactorDevicePresence) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn IsAuthenticationSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorInfo2 {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorInfo2_Vtbl {
        unsafe extern "system" fn PresenceMonitoringMode<Impl: ISecondaryAuthenticationFactorInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresenceMonitoringMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateDevicePresenceAsync<Impl: ISecondaryAuthenticationFactorInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presencestate: SecondaryAuthenticationFactorDevicePresence, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateDevicePresenceAsync(presencestate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAuthenticationSupported<Impl: ISecondaryAuthenticationFactorInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthenticationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryAuthenticationFactorInfo2, BASE_OFFSET>(),
            PresenceMonitoringMode: PresenceMonitoringMode::<Impl, IMPL_OFFSET>,
            UpdateDevicePresenceAsync: UpdateDevicePresenceAsync::<Impl, IMPL_OFFSET>,
            IsAuthenticationSupported: IsAuthenticationSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryAuthenticationFactorInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorRegistration_Impl: Sized {
    fn FinishRegisteringDeviceAsync(&mut self, deviceconfigurationdata: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn AbortRegisteringDeviceAsync(&mut self, errorlogmessage: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorRegistration {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistration";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorRegistration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorRegistration_Vtbl {
        unsafe extern "system" fn FinishRegisteringDeviceAsync<Impl: ISecondaryAuthenticationFactorRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceconfigurationdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinishRegisteringDeviceAsync(&*(&deviceconfigurationdata as *const <super::super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortRegisteringDeviceAsync<Impl: ISecondaryAuthenticationFactorRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorlogmessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbortRegisteringDeviceAsync(&*(&errorlogmessage as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryAuthenticationFactorRegistration, BASE_OFFSET>(),
            FinishRegisteringDeviceAsync: FinishRegisteringDeviceAsync::<Impl, IMPL_OFFSET>,
            AbortRegisteringDeviceAsync: AbortRegisteringDeviceAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryAuthenticationFactorRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorRegistrationResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<SecondaryAuthenticationFactorRegistrationStatus>;
    fn Registration(&mut self) -> ::windows::core::Result<SecondaryAuthenticationFactorRegistration>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorRegistrationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistrationResult";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorRegistrationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorRegistrationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorRegistrationResult_Vtbl {
        unsafe extern "system" fn Status<Impl: ISecondaryAuthenticationFactorRegistrationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorRegistrationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Registration<Impl: ISecondaryAuthenticationFactorRegistrationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Registration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryAuthenticationFactorRegistrationResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Registration: Registration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryAuthenticationFactorRegistrationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorRegistrationStatics_Impl: Sized {
    fn RequestStartRegisteringDeviceAsync(&mut self, deviceid: &::windows::core::HSTRING, capabilities: SecondaryAuthenticationFactorDeviceCapabilities, devicefriendlyname: &::windows::core::HSTRING, devicemodelnumber: &::windows::core::HSTRING, devicekey: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>, mutualauthenticationkey: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorRegistrationResult>>;
    fn FindAllRegisteredDeviceInfoAsync(&mut self, querytype: SecondaryAuthenticationFactorDeviceFindScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<SecondaryAuthenticationFactorInfo>>>;
    fn UnregisterDeviceAsync(&mut self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn UpdateDeviceConfigurationDataAsync(&mut self, deviceid: &::windows::core::HSTRING, deviceconfigurationdata: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorRegistrationStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistrationStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorRegistrationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorRegistrationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorRegistrationStatics_Vtbl {
        unsafe extern "system" fn RequestStartRegisteringDeviceAsync<Impl: ISecondaryAuthenticationFactorRegistrationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, capabilities: SecondaryAuthenticationFactorDeviceCapabilities, devicefriendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, devicemodelnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, devicekey: ::windows::core::RawPtr, mutualauthenticationkey: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStartRegisteringDeviceAsync(
                &*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                capabilities,
                &*(&devicefriendlyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&devicemodelnumber as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&devicekey as *const <super::super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
                &*(&mutualauthenticationkey as *const <super::super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindAllRegisteredDeviceInfoAsync<Impl: ISecondaryAuthenticationFactorRegistrationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querytype: SecondaryAuthenticationFactorDeviceFindScope, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAllRegisteredDeviceInfoAsync(querytype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDeviceAsync<Impl: ISecondaryAuthenticationFactorRegistrationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterDeviceAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateDeviceConfigurationDataAsync<Impl: ISecondaryAuthenticationFactorRegistrationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deviceconfigurationdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateDeviceConfigurationDataAsync(&*(&deviceid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&deviceconfigurationdata as *const <super::super::super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISecondaryAuthenticationFactorRegistrationStatics, BASE_OFFSET>(),
            RequestStartRegisteringDeviceAsync: RequestStartRegisteringDeviceAsync::<Impl, IMPL_OFFSET>,
            FindAllRegisteredDeviceInfoAsync: FindAllRegisteredDeviceInfoAsync::<Impl, IMPL_OFFSET>,
            UnregisterDeviceAsync: UnregisterDeviceAsync::<Impl, IMPL_OFFSET>,
            UpdateDeviceConfigurationDataAsync: UpdateDeviceConfigurationDataAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecondaryAuthenticationFactorRegistrationStatics as ::windows::core::Interface>::IID
    }
}
