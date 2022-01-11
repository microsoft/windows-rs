#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorAuthenticationImpl: Sized {
    fn ServiceAuthenticationHmac(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
    fn SessionNonce(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
    fn DeviceNonce(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
    fn DeviceConfigurationData(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
    fn FinishAuthenticationAsync(&self, devicehmac: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>, sessionhmac: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorFinishAuthenticationStatus>>;
    fn AbortAuthenticationAsync(&self, errorlogmessage: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorAuthentication {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthentication";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorAuthenticationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorAuthenticationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorAuthenticationVtbl {
        unsafe extern "system" fn ServiceAuthenticationHmac<Impl: ISecondaryAuthenticationFactorAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SessionNonce<Impl: ISecondaryAuthenticationFactorAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceNonce<Impl: ISecondaryAuthenticationFactorAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceConfigurationData<Impl: ISecondaryAuthenticationFactorAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FinishAuthenticationAsync<Impl: ISecondaryAuthenticationFactorAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicehmac: ::windows::core::RawPtr, sessionhmac: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AbortAuthenticationAsync<Impl: ISecondaryAuthenticationFactorAuthenticationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorlogmessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISecondaryAuthenticationFactorAuthenticationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationStatus>;
    fn Authentication(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthentication>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorAuthenticationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationResult";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorAuthenticationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorAuthenticationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorAuthenticationResultVtbl {
        unsafe extern "system" fn Status<Impl: ISecondaryAuthenticationFactorAuthenticationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorAuthenticationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Authentication<Impl: ISecondaryAuthenticationFactorAuthenticationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgsImpl: Sized {
    fn StageInfo(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationStageInfo>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgsVtbl {
        unsafe extern "system" fn StageInfo<Impl: ISecondaryAuthenticationFactorAuthenticationStageChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISecondaryAuthenticationFactorAuthenticationStageInfoImpl: Sized {
    fn Stage(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationStage>;
    fn Scenario(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorAuthenticationScenario>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorAuthenticationStageInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStageInfo";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorAuthenticationStageInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorAuthenticationStageInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorAuthenticationStageInfoVtbl {
        unsafe extern "system" fn Stage<Impl: ISecondaryAuthenticationFactorAuthenticationStageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorAuthenticationStage) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Scenario<Impl: ISecondaryAuthenticationFactorAuthenticationStageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorAuthenticationScenario) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceId<Impl: ISecondaryAuthenticationFactorAuthenticationStageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait ISecondaryAuthenticationFactorAuthenticationStaticsImpl: Sized {
    fn ShowNotificationMessageAsync(&self, devicename: &::windows::core::HSTRING, message: SecondaryAuthenticationFactorAuthenticationMessage) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn StartAuthenticationAsync(&self, deviceid: &::windows::core::HSTRING, serviceauthenticationnonce: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationResult>>;
    fn AuthenticationStageChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<SecondaryAuthenticationFactorAuthenticationStageChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAuthenticationStageChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetAuthenticationStageInfoAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorAuthenticationStageInfo>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorAuthenticationStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorAuthenticationStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorAuthenticationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorAuthenticationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorAuthenticationStaticsVtbl {
        unsafe extern "system" fn ShowNotificationMessageAsync<Impl: ISecondaryAuthenticationFactorAuthenticationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, devicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: SecondaryAuthenticationFactorAuthenticationMessage, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartAuthenticationAsync<Impl: ISecondaryAuthenticationFactorAuthenticationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, serviceauthenticationnonce: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AuthenticationStageChanged<Impl: ISecondaryAuthenticationFactorAuthenticationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAuthenticationStageChanged<Impl: ISecondaryAuthenticationFactorAuthenticationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAuthenticationStageChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetAuthenticationStageInfoAsync<Impl: ISecondaryAuthenticationFactorAuthenticationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStaticsImpl: Sized {
    fn RegisterDevicePresenceMonitoringAsync(&self, deviceid: &::windows::core::HSTRING, deviceinstancepath: &::windows::core::HSTRING, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>>;
    fn RegisterDevicePresenceMonitoringWithNewDeviceAsync(&self, deviceid: &::windows::core::HSTRING, deviceinstancepath: &::windows::core::HSTRING, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, devicefriendlyname: &::windows::core::HSTRING, devicemodelnumber: &::windows::core::HSTRING, deviceconfigurationdata: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatus>>;
    fn UnregisterDevicePresenceMonitoringAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn IsDevicePresenceMonitoringSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStaticsVtbl {
        unsafe extern "system" fn RegisterDevicePresenceMonitoringAsync<Impl: ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deviceinstancepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RegisterDevicePresenceMonitoringWithNewDeviceAsync<Impl: ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deviceinstancepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, monitoringmode: SecondaryAuthenticationFactorDevicePresenceMonitoringMode, devicefriendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, devicemodelnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deviceconfigurationdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UnregisterDevicePresenceMonitoringAsync<Impl: ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDevicePresenceMonitoringSupported<Impl: ISecondaryAuthenticationFactorDevicePresenceMonitoringRegistrationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait ISecondaryAuthenticationFactorInfoImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceFriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceConfigurationData(&self) -> ::windows::core::Result<super::super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo";
}
#[cfg(all(feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorInfoVtbl {
        unsafe extern "system" fn DeviceId<Impl: ISecondaryAuthenticationFactorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceFriendlyName<Impl: ISecondaryAuthenticationFactorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceModelNumber<Impl: ISecondaryAuthenticationFactorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeviceConfigurationData<Impl: ISecondaryAuthenticationFactorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISecondaryAuthenticationFactorInfo2Impl: Sized + ISecondaryAuthenticationFactorInfoImpl {
    fn PresenceMonitoringMode(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorDevicePresenceMonitoringMode>;
    fn UpdateDevicePresenceAsync(&self, presencestate: SecondaryAuthenticationFactorDevicePresence) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn IsAuthenticationSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorInfo2 {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorInfo2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorInfo2Vtbl {
        unsafe extern "system" fn PresenceMonitoringMode<Impl: ISecondaryAuthenticationFactorInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorDevicePresenceMonitoringMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UpdateDevicePresenceAsync<Impl: ISecondaryAuthenticationFactorInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presencestate: SecondaryAuthenticationFactorDevicePresence, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsAuthenticationSupported<Impl: ISecondaryAuthenticationFactorInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait ISecondaryAuthenticationFactorRegistrationImpl: Sized {
    fn FinishRegisteringDeviceAsync(&self, deviceconfigurationdata: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn AbortRegisteringDeviceAsync(&self, errorlogmessage: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorRegistration {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistration";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorRegistrationVtbl {
        unsafe extern "system" fn FinishRegisteringDeviceAsync<Impl: ISecondaryAuthenticationFactorRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceconfigurationdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AbortRegisteringDeviceAsync<Impl: ISecondaryAuthenticationFactorRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorlogmessage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISecondaryAuthenticationFactorRegistrationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorRegistrationStatus>;
    fn Registration(&self) -> ::windows::core::Result<SecondaryAuthenticationFactorRegistration>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorRegistrationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistrationResult";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorRegistrationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorRegistrationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorRegistrationResultVtbl {
        unsafe extern "system" fn Status<Impl: ISecondaryAuthenticationFactorRegistrationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SecondaryAuthenticationFactorRegistrationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Registration<Impl: ISecondaryAuthenticationFactorRegistrationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISecondaryAuthenticationFactorRegistrationStaticsImpl: Sized {
    fn RequestStartRegisteringDeviceAsync(&self, deviceid: &::windows::core::HSTRING, capabilities: SecondaryAuthenticationFactorDeviceCapabilities, devicefriendlyname: &::windows::core::HSTRING, devicemodelnumber: &::windows::core::HSTRING, devicekey: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>, mutualauthenticationkey: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<SecondaryAuthenticationFactorRegistrationResult>>;
    fn FindAllRegisteredDeviceInfoAsync(&self, querytype: SecondaryAuthenticationFactorDeviceFindScope) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<SecondaryAuthenticationFactorInfo>>>;
    fn UnregisterDeviceAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn UpdateDeviceConfigurationDataAsync(&self, deviceid: &::windows::core::HSTRING, deviceconfigurationdata: &::core::option::Option<super::super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISecondaryAuthenticationFactorRegistrationStatics {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Provider.ISecondaryAuthenticationFactorRegistrationStatics";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "deprecated", feature = "implement_exclusive"))]
impl ISecondaryAuthenticationFactorRegistrationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecondaryAuthenticationFactorRegistrationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecondaryAuthenticationFactorRegistrationStaticsVtbl {
        unsafe extern "system" fn RequestStartRegisteringDeviceAsync<Impl: ISecondaryAuthenticationFactorRegistrationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, capabilities: SecondaryAuthenticationFactorDeviceCapabilities, devicefriendlyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, devicemodelnumber: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, devicekey: ::windows::core::RawPtr, mutualauthenticationkey: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindAllRegisteredDeviceInfoAsync<Impl: ISecondaryAuthenticationFactorRegistrationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, querytype: SecondaryAuthenticationFactorDeviceFindScope, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UnregisterDeviceAsync<Impl: ISecondaryAuthenticationFactorRegistrationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UpdateDeviceConfigurationDataAsync<Impl: ISecondaryAuthenticationFactorRegistrationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, deviceconfigurationdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
